// SPDX-License-Identifier: GPL-2.0
use core::mem::offset_of;
use core::sync::atomic::{fence, Ordering};

use kernel::alloc::flags::GFP_KERNEL;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::dma_write;
use kernel::prelude::*;
use kernel::sync::aref::ARef;
use kernel::time::Delta;
use kernel::transmute::{AsBytes, FromBytes};

use crate::driver::Bar0;
use crate::gsp::create_pte_array;
use crate::gsp::fw::{
    GspMsgElement, GspRpcHeader, NV_VGPU_MSG_EVENT_GSP_INIT_DONE,
    NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE, NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD,
    NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER, NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED,
    NV_VGPU_MSG_EVENT_OS_ERROR_LOG, NV_VGPU_MSG_EVENT_POST_EVENT, NV_VGPU_MSG_EVENT_RC_TRIGGERED,
    NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT, NV_VGPU_MSG_FUNCTION_ALLOC_CHANNEL_DMA,
    NV_VGPU_MSG_FUNCTION_ALLOC_CTX_DMA, NV_VGPU_MSG_FUNCTION_ALLOC_DEVICE,
    NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY, NV_VGPU_MSG_FUNCTION_ALLOC_OBJECT,
    NV_VGPU_MSG_FUNCTION_ALLOC_ROOT, NV_VGPU_MSG_FUNCTION_BIND_CTX_DMA, NV_VGPU_MSG_FUNCTION_FREE,
    NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO, NV_VGPU_MSG_FUNCTION_GET_STATIC_INFO,
    NV_VGPU_MSG_FUNCTION_GSP_INIT_POST_OBJGPU, NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL,
    NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO, NV_VGPU_MSG_FUNCTION_LOG,
    NV_VGPU_MSG_FUNCTION_MAP_MEMORY, NV_VGPU_MSG_FUNCTION_NOP,
    NV_VGPU_MSG_FUNCTION_SET_GUEST_SYSTEM_INFO, NV_VGPU_MSG_FUNCTION_SET_REGISTRY,
};
use crate::gsp::{GSP_PAGE_SHIFT, GSP_PAGE_SIZE};
use crate::regs::NV_PGSP_QUEUE_HEAD;
use crate::sbuffer::SBuffer;
use crate::util::wait_on;

pub(crate) trait GspCommandToGsp: Sized {
    const FUNCTION: u32;
}

pub(crate) trait GspMessageFromGsp: Sized {
    const FUNCTION: u32;
}

/// Number of GSP pages making the Msgq.
const MSGQ_NUM_PAGES: u32 = 0x3f;

#[repr(C, align(0x1000))]
#[derive(Debug)]
struct MsgqData {
    data: [[u8; GSP_PAGE_SIZE]; MSGQ_NUM_PAGES as usize],
}

// Annoyingly there is no real equivalent of #define so we're forced to use a
// literal to specify the alignment above. So check that against the actual GSP
// page size here.
static_assert!(align_of::<MsgqData>() == GSP_PAGE_SIZE);

/// TX header for setting up a command queue with the GSP.
///
/// # Invariants
///
/// [`Self::write_ptr`] is guaranteed to return a value in the range `0..NUM_PAGES`.
#[repr(transparent)]
#[derive(Debug)]
struct MsgqTxHeader(super::fw::MsgqTxHeader);

unsafe impl AsBytes for MsgqTxHeader {}

impl MsgqTxHeader {
    fn new(msgq_size: u32, rx_hdr_offset: u32) -> Self {
        Self(super::fw::MsgqTxHeader::new(
            msgq_size,
            MSGQ_NUM_PAGES,
            rx_hdr_offset,
        ))
    }

    fn write_ptr(&self) -> u32 {
        self.0.write_ptr()
    }

    /// Advance the write pointer by `elem_count` units, wrapping around the ring buffer if
    /// necessary.
    fn advance_write_ptr(&mut self, elem_count: u32) {
        let wptr = self.write_ptr().wrapping_add(elem_count) % MSGQ_NUM_PAGES;
        self.0.set_write_ptr(wptr);

        // Ensure all command data is visible before triggering the GSP read
        fence(Ordering::SeqCst);
    }
}

/// RX header for setting up a message queue with the GSP.
///
/// # Invariants
///
/// [`Self::read_ptr`] is guaranteed to return a value in the range `0..NUM_PAGES`.
#[repr(transparent)]
#[derive(Debug)]
struct MsgqRxHeader(super::fw::MsgqRxHeader);

unsafe impl AsBytes for MsgqRxHeader {}

impl MsgqRxHeader {
    fn new() -> Self {
        Self(super::fw::MsgqRxHeader::new())
    }

    fn read_ptr(&self) -> u32 {
        self.0.read_ptr()
    }

    /// Advance the read pointer by `elem_count` units, wrapping around the ring buffer if
    /// necessary.
    fn advance_read_ptr(&mut self, elem_count: u32) {
        let rptr = self.read_ptr().wrapping_add(elem_count) % MSGQ_NUM_PAGES;

        // Ensure read pointer is properly ordered
        fence(Ordering::SeqCst);

        self.0.set_read_ptr(rptr);
    }
}

// There is no struct defined for this in the open-gpu-kernel-source headers.
// Instead it is defined by code in GspMsgQueuesInit().
#[repr(C)]
struct Msgq {
    tx: MsgqTxHeader,
    rx: MsgqRxHeader,
    msgq: MsgqData,
}

#[repr(C)]
struct GspMem {
    ptes: [u8; GSP_PAGE_SIZE],
    cpuq: Msgq,
    gspq: Msgq,
}

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
// that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for GspMem {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
// that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for GspMem {}

pub(crate) struct GspCmdq {
    dev: ARef<device::Device>,
    msg_count: u32,
    seq: u32,
    gsp_mem: CoherentAllocation<GspMem>,
    pub nr_ptes: u32,
}

// A reference to a message currently sitting in the GSP command queue. May
// contain two slices as the command queue is a circular buffer which may have
// wrapped.
//
// INVARIANT: The underlying message data cannot change because the struct holds
// a reference to the command queue which prevents command queue manipulation
// until the GspQueueMessage is dropped.
pub(crate) struct GspQueueMessage<'a> {
    cmdq: &'a mut GspCmdq,
    rpc_header: &'a GspRpcHeader,
    slice_1: &'a [u8],
    slice_2: Option<&'a [u8]>,
}

type GspQueueMessageData<'a, M> = (&'a M, Option<SBuffer<core::array::IntoIter<&'a [u8], 2>>>);

impl<'a> GspQueueMessage<'a> {
    pub(crate) fn try_as<M: GspMessageFromGsp>(&'a self) -> Result<GspQueueMessageData<'a, M>> {
        if self.rpc_header.function() != M::FUNCTION {
            return Err(ERANGE);
        }

        // SAFETY: The slice references the cmdq message memory which is
        // guaranteed to outlive the returned GspQueueMessageData by the
        // invariants of GspQueueMessage and the lifetime 'a.
        let msg = unsafe { &*(self.slice_1.as_ptr().cast::<M>()) };
        let data = &self.slice_1[size_of::<M>()..];
        let data_size =
            self.rpc_header.length() as usize - size_of::<GspRpcHeader>() - size_of::<M>();
        let sbuf = if data_size > 0 {
            Some(SBuffer::new_reader([data, self.slice_2.unwrap_or(&[])]))
        } else {
            None
        };

        Ok((msg, sbuf))
    }

    pub(crate) fn ack(self) -> Result {
        self.cmdq.ack_msg(self.rpc_header.length())?;

        Ok(())
    }
}

// The same as GspQueueMessage except the fields are mutable for constructing a
// message to the GSP.
pub(crate) struct GspQueueCommand<'a> {
    cmdq: &'a mut GspCmdq,
    msg_element: &'a mut GspMsgElement,
    slice_1: &'a mut [u8],
    slice_2: &'a mut [u8],
}

type GspQueueCommandData<'a, M> = (
    &'a mut M,
    Option<SBuffer<core::array::IntoIter<&'a mut [u8], 2>>>,
);

impl<'a> GspQueueCommand<'a> {
    pub(crate) fn try_as<'b, M: GspCommandToGsp>(&'b mut self) -> GspQueueCommandData<'b, M> {
        // SAFETY: The slice references the cmdq message memory which is
        // guaranteed to outlive the returned GspQueueCommandData by the
        // invariants of GspQueueCommand and the lifetime 'a.
        let msg = unsafe { &mut *(self.slice_1.as_mut_ptr().cast::<M>()) };
        let data = &mut self.slice_1[size_of::<M>()..];
        let data_size = self.msg_element.rpc_header().length() as usize
            - size_of::<GspRpcHeader>()
            - size_of::<M>();
        let sbuf = if data_size > 0 {
            Some(SBuffer::new_writer([data, self.slice_2]))
        } else {
            None
        };
        self.msg_element.rpc_header_mut().set_function(M::FUNCTION);

        (msg, sbuf)
    }

    pub(crate) fn send_to_gsp(self, bar: &Bar0) -> Result {
        GspCmdq::send_cmd_to_gsp(self, bar)?;
        Ok(())
    }
}

impl GspCmdq {
    pub(crate) fn new(dev: &device::Device<device::Bound>) -> Result<GspCmdq> {
        let mut gsp_mem =
            CoherentAllocation::<GspMem>::alloc_coherent(dev, 1, GFP_KERNEL | __GFP_ZERO)?;

        let nr_ptes = size_of::<GspMem>() >> GSP_PAGE_SHIFT;
        build_assert!(nr_ptes * size_of::<u64>() <= GSP_PAGE_SIZE);

        create_pte_array(&mut gsp_mem, 0);

        const MSGQ_SIZE: u32 = size_of::<Msgq>() as u32;
        //TODO: this is equal to MSGQ_NUM_PAGES...
        const MSG_COUNT: u32 = ((MSGQ_SIZE as usize - GSP_PAGE_SIZE) / GSP_PAGE_SIZE) as u32;
        const RX_HDR_OFF: u32 = offset_of!(Msgq, rx) as u32;
        dma_write!(gsp_mem[0].cpuq.tx = MsgqTxHeader::new(MSGQ_SIZE, RX_HDR_OFF))?;
        dma_write!(gsp_mem[0].cpuq.rx = MsgqRxHeader::new())?;

        Ok(GspCmdq {
            dev: dev.into(),
            msg_count: MSG_COUNT,
            seq: 0,
            gsp_mem,
            nr_ptes: nr_ptes as u32,
        })
    }

    fn cpu_wptr(&self) -> u32 {
        // SAFETY: index `0` is valid as `gsp_mem` has been allocated accordingly, thus the access
        // cannot fail.
        let gsp_mem = unsafe { &self.gsp_mem.as_slice(0, 1).unwrap_unchecked()[0] };
        gsp_mem.cpuq.tx.write_ptr()
    }

    fn gsp_rptr(&self) -> u32 {
        // SAFETY: index `0` is valid as `gsp_mem` has been allocated accordingly, thus the access
        // cannot fail.
        let gsp_mem = unsafe { &self.gsp_mem.as_slice(0, 1).unwrap_unchecked()[0] };
        gsp_mem.gspq.rx.read_ptr()
    }

    fn cpu_rptr(&self) -> u32 {
        // SAFETY: index `0` is valid as `gsp_mem` has been allocated accordingly, thus the access
        // cannot fail.
        let gsp_mem = unsafe { &self.gsp_mem.as_slice(0, 1).unwrap_unchecked()[0] };
        gsp_mem.cpuq.rx.read_ptr()
    }

    fn gsp_wptr(&self) -> u32 {
        // SAFETY: index `0` is valid as `gsp_mem` has been allocated accordingly, thus the access
        // cannot fail.
        let gsp_mem = unsafe { &self.gsp_mem.as_slice(0, 1).unwrap_unchecked()[0] };
        gsp_mem.gspq.tx.write_ptr()
    }

    // Returns the numbers of pages free for sending an RPC to GSP.
    fn free_tx_pages(&self) -> u32 {
        let wptr = self.cpu_wptr();
        let rptr = self.gsp_rptr();
        let mut free = rptr + self.msg_count - wptr - 1;

        if free >= self.msg_count {
            free -= self.msg_count;
        }

        free
    }

    // Returns the number of pages the GSP has written to the queue.
    fn used_rx_pages(&self) -> u32 {
        let rptr = self.cpu_rptr();
        let wptr = self.gsp_wptr();
        let mut used = wptr + self.msg_count - rptr;
        if used >= self.msg_count {
            used -= self.msg_count;
        }

        used
    }

    fn calculate_checksum<T: Iterator<Item = u8>>(it: T) -> u32 {
        let sum64 = it
            .enumerate()
            .map(|(idx, byte)| (((idx % 8) * 8) as u32, byte))
            .fold(0, |acc, (rol, byte)| acc ^ u64::from(byte).rotate_left(rol));

        ((sum64 >> 32) as u32) ^ (sum64 as u32)
    }

    pub(crate) fn alloc_gsp_queue_command<'a>(
        &'a mut self,
        cmd_size: usize,
    ) -> Result<GspQueueCommand<'a>> {
        const HEADER_SIZE: usize = size_of::<GspMsgElement>();
        let msg_size = HEADER_SIZE + cmd_size;
        if self.free_tx_pages() < msg_size.div_ceil(GSP_PAGE_SIZE) as u32 {
            return Err(EAGAIN);
        }
        let wptr = self.cpu_wptr() as usize;

        // SAFETY: By the invariants of CoherentAllocation gsp_mem.start_ptr_mut() is valid.
        let ptr = unsafe {
            core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).cpuq.msgq.data[wptr])
        };

        // SAFETY: ptr points to at least one GSP_PAGE_SIZE bytes of contiguous
        // memory which is larger than GspMsgHeader.
        let msg_element_slice: &mut [u8] = unsafe {
            core::slice::from_raw_parts_mut(ptr.cast::<u8>(), size_of::<GspMsgElement>())
        };
        let msg_element = GspMsgElement::from_bytes_mut(msg_element_slice).ok_or(EINVAL)?;
        *msg_element = GspMsgElement::new(self.seq, cmd_size);
        self.seq += 1;

        // Number of bytes left before we have to wrap the buffer
        let remaining = ((self.msg_count as usize - wptr) << GSP_PAGE_SHIFT) - HEADER_SIZE;

        let (slice_1, slice_2) = if cmd_size <= remaining {
            // SAFETY: ptr points to a region of contiguous memory at least
            // cmd_size + HEADER_SIZE long.
            let slice_1: &mut [u8] = unsafe {
                core::slice::from_raw_parts_mut(ptr.cast::<u8>().add(HEADER_SIZE), cmd_size)
            };
            slice_1.fill(0);
            (slice_1, &mut [] as &mut [u8])
        } else {
            // SAFETY: ptr points to a region of contiguous memory remaining +
            // HEADER_SIZE bytes long.
            let slice_1: &mut [u8] = unsafe {
                core::slice::from_raw_parts_mut(ptr.cast::<u8>().add(HEADER_SIZE), remaining)
            };
            // SAFETY: By the invariants of CoherentAllocation gsp_mem.start_ptr_mut() is valid.
            let ptr = unsafe {
                core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).cpuq.msgq.data[0])
            };
            // SAFETY: ptr points to a region of contiguous memory
            // self.msg_count GSP_PAGE_SIZE pages long.
            let slice_2: &mut [u8] =
                unsafe { core::slice::from_raw_parts_mut(ptr.cast::<u8>(), remaining - cmd_size) };
            slice_2.fill(0);
            (slice_1, slice_2)
        };

        Ok(GspQueueCommand {
            cmdq: self,
            msg_element,
            slice_1,
            slice_2,
        })
    }

    pub(crate) fn send_cmd_to_gsp(cmd: GspQueueCommand<'_>, bar: &Bar0) -> Result {
        let rpc_header = cmd.msg_element.rpc_header();
        dev_info!(
            &cmd.cmdq.dev,
            "GSP RPC: send: seq# {}, function=0x{:x} ({}), length=0x{:x}\n",
            cmd.cmdq.seq - 1,
            rpc_header.function(),
            decode_gsp_function(rpc_header.function()),
            rpc_header.length(),
        );

        // Calculate checksum over the entire message
        cmd.msg_element
            .set_checksum(GspCmdq::calculate_checksum(SBuffer::new_reader([
                cmd.msg_element.as_bytes(),
                &cmd.slice_1[..],
                &cmd.slice_2[..],
            ])));

        let gsp_mem = unsafe { &mut cmd.cmdq.gsp_mem.as_slice_mut(0, 1).unwrap_unchecked()[0] };
        gsp_mem
            .cpuq
            .tx
            .advance_write_ptr(cmd.msg_element.elem_count());

        NV_PGSP_QUEUE_HEAD::default().set_address(0).write(bar);

        Ok(())
    }

    pub(crate) fn msg_from_gsp_available(&self) -> bool {
        const HEADER_SIZE: u32 = size_of::<GspMsgElement>() as u32;

        // Used pages contains the total number of pages available to consume
        let used_pages = self.used_rx_pages();
        if used_pages < HEADER_SIZE.div_ceil(GSP_PAGE_SIZE as u32) {
            return false;
        }

        let rptr = self.cpu_rptr();
        // SAFETY: By the invariants of CoherentAllocation gsp_mem.start_ptr() is valid.
        let ptr = unsafe {
            core::ptr::addr_of!((*self.gsp_mem.start_ptr()).gspq.msgq.data[rptr as usize])
        };

        // SAFETY: ptr points to at least GSP_PAGE_SIZE bytes of memory which is
        // larger than GspRpcHeader.
        let msg_element = unsafe { &*(ptr.cast::<u8>().cast::<GspMsgElement>()) };

        // Not all pages of the message have made it to the queue so bail and
        // let the caller retry. Note rpc.length includes the rpc header size
        // but not the message header size.
        if (used_pages as usize) << GSP_PAGE_SHIFT < msg_element.length() {
            return false;
        }

        true
    }

    pub(crate) fn wait_for_msg_from_gsp(&self, timeout: Delta) -> Result {
        wait_on(timeout, || {
            if self.msg_from_gsp_available() {
                Some(())
            } else {
                None
            }
        })
    }

    pub(crate) fn receive_msg_from_gsp<'a>(&'a mut self) -> Result<GspQueueMessage<'a>> {
        const HEADER_SIZE: u32 = size_of::<GspMsgElement>() as u32;

        // Used pages contains the total number of pages available to consume
        let used_pages = self.used_rx_pages();
        if used_pages < HEADER_SIZE.div_ceil(GSP_PAGE_SIZE as u32) {
            return Err(EAGAIN);
        }

        let rptr = self.cpu_rptr();

        // Remaining number of bytes left before we have to wrap
        let remaining = if rptr + used_pages > self.msg_count {
            (self.msg_count - rptr) << GSP_PAGE_SHIFT
        } else {
            used_pages << GSP_PAGE_SHIFT
        };

        // SAFETY: By the invariants of CoherentAllocation gsp_mem.start_ptr_mut() is valid.
        let ptr = unsafe {
            core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).gspq.msgq.data[rptr as usize])
        };

        // SAFETY: ptr points to a region of memory remaining bytes long.
        let msg_slice =
            unsafe { core::slice::from_raw_parts(ptr as *const u8, remaining as usize) };

        let msg_element =
            GspMsgElement::from_bytes(&msg_slice[0..size_of::<GspMsgElement>()]).ok_or(EINVAL)?;
        let rpc_header = msg_element.rpc_header();

        if rpc_header.length() >= self.msg_count << GSP_PAGE_SHIFT {
            return Err(E2BIG);
        }

        // rpc.length includes the size of the GspRpcHeader. Remove it to make
        // the rest of the code a bit easier to follow.
        let rpc_data_length = rpc_header.length() - size_of::<GspRpcHeader>() as u32;

        // Log RPC receive with message type decoding
        dev_info!(
            self.dev,
            "GSP RPC: receive: seq# {}, function=0x{:x} ({}), length=0x{:x}\n",
            rpc_header.sequence(),
            rpc_header.function(),
            decode_gsp_function(rpc_header.function()),
            rpc_header.length(),
        );

        // Should never happen if `wait_on_message()` has been called but we need to check.
        if used_pages << GSP_PAGE_SHIFT < HEADER_SIZE + rpc_data_length {
            return Err(EAGAIN);
        }

        let (slice_1, slice_2) = if rpc_data_length + HEADER_SIZE < remaining {
            (
                &msg_slice[(HEADER_SIZE as usize)..(HEADER_SIZE + rpc_data_length) as usize],
                None,
            )
        } else {
            let slice_1 = &msg_slice[(HEADER_SIZE as usize)..(HEADER_SIZE + remaining) as usize];
            // SAFETY: By the invariants of CoherentAllocation gsp_mem.start_ptr_mut() is valid and
            // large enough to hold gsp_mem.
            let ptr =
                unsafe { core::ptr::addr_of!((*self.gsp_mem.start_ptr_mut()).gspq.msgq.data[0]) };
            // SAFETY: ptr pointers to self.msg_count GSP_PAGE_SIZE bytes of memory which by the
            // earlier check is greater than rpc_data_length.
            let slice_2 = unsafe {
                core::slice::from_raw_parts(
                    ptr.cast::<u8>(),
                    rpc_data_length as usize - slice_1.len(),
                )
            };
            (slice_1, Some(slice_2))
        };

        if GspCmdq::calculate_checksum(SBuffer::new_reader([
            msg_element.as_bytes(),
            slice_1,
            slice_2.unwrap_or(&[]),
        ])) != 0
        {
            dev_err!(
                self.dev,
                "GSP RPC: receive: Call {} - bad checksum",
                rpc_header.sequence()
            );
            return Err(EIO);
        }

        let gspq_msg = GspQueueMessage {
            cmdq: self,
            slice_1,
            slice_2,
            rpc_header,
        };

        Ok(gspq_msg)
    }

    pub(crate) fn get_cmdq_offsets(&self) -> (u64, u64, u64) {
        (
            self.gsp_mem.dma_handle(),
            core::mem::offset_of!(Msgq, msgq) as u64,
            (core::mem::offset_of!(GspMem, gspq) - core::mem::offset_of!(GspMem, cpuq)
                + core::mem::offset_of!(Msgq, msgq)) as u64,
        )
    }

    fn ack_msg(&mut self, length: u32) -> Result {
        const HEADER_SIZE: u32 = size_of::<GspMsgElement>() as u32;
        let num_elems = (HEADER_SIZE + length).div_ceil(GSP_PAGE_SIZE as u32);
        let gsp_mem = unsafe { &mut self.gsp_mem.as_slice_mut(0, 1).unwrap_unchecked()[0] };
        gsp_mem.cpuq.rx.advance_read_ptr(num_elems);

        Ok(())
    }
}

fn decode_gsp_function(function: u32) -> &'static str {
    match function {
        // Common function codes
        NV_VGPU_MSG_FUNCTION_NOP => "NOP",
        NV_VGPU_MSG_FUNCTION_SET_GUEST_SYSTEM_INFO => "SET_GUEST_SYSTEM_INFO",
        NV_VGPU_MSG_FUNCTION_ALLOC_ROOT => "ALLOC_ROOT",
        NV_VGPU_MSG_FUNCTION_ALLOC_DEVICE => "ALLOC_DEVICE",
        NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY => "ALLOC_MEMORY",
        NV_VGPU_MSG_FUNCTION_ALLOC_CTX_DMA => "ALLOC_CTX_DMA",
        NV_VGPU_MSG_FUNCTION_ALLOC_CHANNEL_DMA => "ALLOC_CHANNEL_DMA",
        NV_VGPU_MSG_FUNCTION_MAP_MEMORY => "MAP_MEMORY",
        NV_VGPU_MSG_FUNCTION_BIND_CTX_DMA => "BIND_CTX_DMA",
        NV_VGPU_MSG_FUNCTION_ALLOC_OBJECT => "ALLOC_OBJECT",
        NV_VGPU_MSG_FUNCTION_FREE => "FREE",
        NV_VGPU_MSG_FUNCTION_LOG => "LOG",
        NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO => "GET_GSP_STATIC_INFO",
        NV_VGPU_MSG_FUNCTION_SET_REGISTRY => "SET_REGISTRY",
        NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO => "GSP_SET_SYSTEM_INFO",
        NV_VGPU_MSG_FUNCTION_GSP_INIT_POST_OBJGPU => "GSP_INIT_POST_OBJGPU",
        NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL => "GSP_RM_CONTROL",
        NV_VGPU_MSG_FUNCTION_GET_STATIC_INFO => "GET_STATIC_INFO",

        // Event codes
        NV_VGPU_MSG_EVENT_GSP_INIT_DONE => "INIT_DONE",
        NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER => "RUN_CPU_SEQUENCER",
        NV_VGPU_MSG_EVENT_POST_EVENT => "POST_EVENT",
        NV_VGPU_MSG_EVENT_RC_TRIGGERED => "RC_TRIGGERED",
        NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED => "MMU_FAULT_QUEUED",
        NV_VGPU_MSG_EVENT_OS_ERROR_LOG => "OS_ERROR_LOG",
        NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD => "NOCAT",
        NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE => "LOCKDOWN_NOTICE",
        NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT => "LIBOS_PRINT",

        // Default for unknown codes
        _ => "UNKNOWN",
    }
}
