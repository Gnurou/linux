// SPDX-License-Identifier: GPL-2.0

use core::mem::offset_of;
use core::sync::atomic::fence;
use core::sync::atomic::Ordering;

use kernel::alloc::flags::GFP_KERNEL;
use kernel::device;
use kernel::dma::{CoherentAllocation, DmaAddress};
use kernel::prelude::*;
use kernel::sync::aref::ARef;
use kernel::time::Delta;
use kernel::transmute::{AsBytes, FromBytes};
use kernel::{dma_read, dma_write};

use super::fw::{
    NV_VGPU_MSG_EVENT_GSP_INIT_DONE, NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE,
    NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD, NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER,
    NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED, NV_VGPU_MSG_EVENT_OS_ERROR_LOG,
    NV_VGPU_MSG_EVENT_POST_EVENT, NV_VGPU_MSG_EVENT_RC_TRIGGERED,
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
use crate::driver::Bar0;
use crate::gsp::fw::{GspMsgElement, MsgqRxHeader, MsgqTxHeader};
use crate::gsp::PteArray;
use crate::gsp::{GSP_PAGE_SHIFT, GSP_PAGE_SIZE};
use crate::regs::NV_PGSP_QUEUE_HEAD;
use crate::sbuffer::SBuffer;
use crate::util::wait_on;

pub(crate) trait GspCommandToGsp: Sized + AsBytes {
    const FUNCTION: u32;
}

pub(crate) trait GspCommandToGspWithPayload: GspCommandToGsp {}
pub(crate) trait GspCommandToGspWithoutPayload: GspCommandToGsp {}

pub(crate) trait GspMessageFromGsp: Sized + FromBytes + AsBytes {
    const FUNCTION: u32;
}

/// Number of GSP pages making the Msgq.
pub(crate) const MSGQ_NUM_PAGES: u32 = 0x3f;

#[repr(C, align(0x1000))]
#[derive(Debug)]
struct MsgqData {
    data: [[u8; GSP_PAGE_SIZE]; MSGQ_NUM_PAGES as usize],
}

// Annoyingly there is no real equivalent of #define so we're forced to use a
// literal to specify the alignment above. So check that against the actual GSP
// page size here.
static_assert!(align_of::<MsgqData>() == GSP_PAGE_SIZE);

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
    ptes: PteArray<{ GSP_PAGE_SIZE / size_of::<u64>() }>,
    cpuq: Msgq,
    gspq: Msgq,
}

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
// that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for GspMem {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
// that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for GspMem {}

/// `GspMem` struct that is shared with the GSP.
struct DmaGspMem(CoherentAllocation<GspMem>);

impl DmaGspMem {
    fn new(dev: &device::Device<device::Bound>) -> Result<Self> {
        const MSGQ_SIZE: u32 = size_of::<Msgq>() as u32;
        const RX_HDR_OFF: u32 = offset_of!(Msgq, rx) as u32;

        let gsp_mem =
            CoherentAllocation::<GspMem>::alloc_coherent(dev, 1, GFP_KERNEL | __GFP_ZERO)?;
        dma_write!(gsp_mem[0].ptes = PteArray::new(gsp_mem.dma_handle()))?;
        dma_write!(gsp_mem[0].cpuq.tx = MsgqTxHeader::new(MSGQ_SIZE, RX_HDR_OFF))?;
        dma_write!(gsp_mem[0].cpuq.rx = MsgqRxHeader::new())?;

        Ok(Self(gsp_mem))
    }

    /// # Safety
    ///
    /// The caller must ensure that the device doesn't access the parts of the [`GspMem`] it works
    /// with.
    unsafe fn access_mut(&mut self) -> &mut GspMem {
        // SAFETY:
        // - The [`CoherentAllocation`] contains exactly one object.
        // - Per the safety statement of the function, no concurrent access wil be performed.
        &mut unsafe { self.0.as_slice_mut(0, 1) }.unwrap()[0]
    }

    /// # Safety
    ///
    /// The caller must ensure that the device doesn't access the parts of the [`GspMem`] it works
    /// with.
    unsafe fn access(&self) -> &GspMem {
        // SAFETY:
        // - The [`CoherentAllocation`] contains exactly one object.
        // - Per the safety statement of the function, no concurrent access wil be performed.
        &unsafe { self.0.as_slice(0, 1) }.unwrap()[0]
    }

    fn driver_write_area(&mut self) -> (&mut [[u8; GSP_PAGE_SIZE]], &mut [[u8; GSP_PAGE_SIZE]]) {
        let tx = self.cpu_write_ptr() as usize;
        let rx = self.gsp_read_ptr() as usize;

        // SAFETY: we will only access the driver-owned part of the shared memory.
        let gsp_mem = unsafe { self.access_mut() };
        let (before_tx, after_tx) = gsp_mem.cpuq.msgq.data.split_at_mut(tx);

        if rx <= tx {
            // The area from `tx` up to the end of the ring, and from the beginning of the ring up
            // to `rx`, minus one unit, belongs to the driver.
            if rx == 0 {
                let last = after_tx.len() - 1;
                (&mut after_tx[..last], &mut before_tx[0..0])
            } else {
                (after_tx, &mut before_tx[..rx])
            }
        } else {
            // The area from `tx` to `rx`, minus one unit, belongs to the driver.
            (after_tx.split_at_mut(rx - tx).0, &mut before_tx[0..0])
        }
    }

    fn driver_read_area(&self) -> (&[[u8; GSP_PAGE_SIZE]], &[[u8; GSP_PAGE_SIZE]]) {
        let tx = self.gsp_write_ptr() as usize;
        let rx = self.cpu_read_ptr() as usize;

        // SAFETY: we will only access the driver-owned part of the shared memory.
        let gsp_mem = unsafe { self.access() };
        let (before_rx, after_rx) = gsp_mem.gspq.msgq.data.split_at(rx);

        if tx < rx {
            // The area from `rx` up to the end of the ring, and from the beginning of the ring up
            // to `tx` belongs to the driver.
            (after_rx, &before_rx[0..tx])
        } else {
            // The area from `rx` to `tx` belongs to the driver.
            (after_rx.split_at(tx - rx).0, &before_rx[0..0])
        }
    }

    /// Tries to allocate `size` bytes on the command queue circular buffer.
    ///
    /// If enough space was available, two slices are returned. The first one starts at the address
    /// of the CPU write pointer and ends either at `size` or at the end of the circular buffer,
    /// whichever came first. If the first slice does not cover the whole requested allocation, the
    /// second slice covers the remainder, starting at the beginning or the circular buffer.
    ///
    /// # Invariants
    ///
    /// The returned slices are both aligned to [`GSP_PAGE_SIZE`].
    fn allocate_command(&mut self, size: usize) -> Result<(&mut [u8], &mut [u8])> {
        let driver_area = self.driver_write_area();
        let free_tx_pages = driver_area.0.len() + driver_area.1.len();

        if free_tx_pages < size.div_ceil(GSP_PAGE_SIZE) {
            return Err(EAGAIN);
        }

        // Flatten the slices into byte arrays.
        let (slice_1, slice_2) = (
            driver_area.0.as_flattened_mut(),
            driver_area.1.as_flattened_mut(),
        );

        Ok(match size.checked_sub(slice_1.len()) {
            // We need the second slice, return the full first slice and the second one
            // truncated to the remainder of the requested size.
            Some(remainder) => (slice_1, &mut slice_2[..remainder]),
            // There is more space in the first slice than we need, truncate it and return an
            // empty second slice.
            None => (&mut slice_1[..size], &mut slice_2[0..0]),
        })
    }

    fn gsp_write_ptr(&self) -> u32 {
        let gsp_mem = &self.0;
        dma_read!(gsp_mem[0].gspq.tx.writePtr).unwrap() % MSGQ_NUM_PAGES
    }

    fn gsp_read_ptr(&self) -> u32 {
        let gsp_mem = &self.0;
        dma_read!(gsp_mem[0].gspq.rx.readPtr).unwrap() % MSGQ_NUM_PAGES
    }

    fn cpu_read_ptr(&self) -> u32 {
        let gsp_mem = &self.0;
        dma_read!(gsp_mem[0].cpuq.rx.readPtr).unwrap() % MSGQ_NUM_PAGES
    }

    /// Inform the GSP that it can send `elem_count` new pages into the message queue.
    fn advance_cpu_read_ptr(&mut self, elem_count: u32) {
        let gsp_mem = &self.0;
        let rptr = self.cpu_read_ptr().wrapping_add(elem_count) % MSGQ_NUM_PAGES;

        // Ensure read pointer is properly ordered
        fence(Ordering::SeqCst);

        dma_write!(gsp_mem[0].cpuq.rx.readPtr = rptr).unwrap();
    }

    fn cpu_write_ptr(&self) -> u32 {
        let gsp_mem = &self.0;
        dma_read!(gsp_mem[0].cpuq.tx.writePtr).unwrap() % MSGQ_NUM_PAGES
    }

    /// Inform the GSP that it can process `elem_count` new pages from the command queue.
    fn advance_cpu_write_ptr(&mut self, elem_count: u32) {
        let gsp_mem = &self.0;
        let wptr = self.cpu_write_ptr().wrapping_add(elem_count) & MSGQ_NUM_PAGES;
        dma_write!(gsp_mem[0].cpuq.tx.writePtr = wptr).unwrap();

        // Ensure all command data is visible before triggering the GSP read
        fence(Ordering::SeqCst);
    }
}

pub(crate) struct GspCmdq {
    dev: ARef<device::Device>,
    seq: u32,
    gsp_mem: DmaGspMem,
}

// The page table entries for the GSP shared region must fit within a single page.
static_assert!(GspCmdq::NUM_PTES * size_of::<u64>() <= GSP_PAGE_SIZE);

impl GspCmdq {
    /// Offset of the data after the PTEs.
    const POST_PTE_OFFSET: usize = core::mem::offset_of!(GspMem, cpuq);

    /// Offset of command queue ring buffer.
    pub(crate) const CMDQ_OFFSET: usize = core::mem::offset_of!(GspMem, cpuq)
        + core::mem::offset_of!(Msgq, msgq)
        - Self::POST_PTE_OFFSET;

    /// Offset of message queue ring buffer.
    pub(crate) const STATQ_OFFSET: usize = core::mem::offset_of!(GspMem, gspq)
        + core::mem::offset_of!(Msgq, msgq)
        - Self::POST_PTE_OFFSET;

    /// Number of page table entries for the GSP shared region.
    pub(crate) const NUM_PTES: usize = size_of::<GspMem>() >> GSP_PAGE_SHIFT;

    pub(crate) fn new(dev: &device::Device<device::Bound>) -> Result<GspCmdq> {
        let gsp_mem = DmaGspMem::new(dev)?;

        Ok(GspCmdq {
            dev: dev.into(),
            seq: 0,
            gsp_mem,
        })
    }

    fn calculate_checksum<T: Iterator<Item = u8>>(it: T) -> u32 {
        let sum64 = it
            .enumerate()
            .map(|(idx, byte)| (((idx % 8) * 8) as u32, byte))
            .fold(0, |acc, (rol, byte)| acc ^ u64::from(byte).rotate_left(rol));

        ((sum64 >> 32) as u32) ^ (sum64 as u32)
    }

    fn send_gsp_command_internal<M: GspCommandToGsp>(
        &mut self,
        bar: &Bar0,
        init: impl Init<M>,
        payload_size: usize,
        init_payload: impl FnOnce(&mut SBuffer<core::array::IntoIter<&mut [u8], 2>>) -> Result,
    ) -> Result {
        #[repr(C)]
        struct FullCommand<M> {
            hdr: GspMsgElement,
            cmd: M,
        }

        // The command must fit within a single page, and within the first slice of the driver
        // area.
        build_assert!(size_of::<FullCommand<M>>() <= GSP_PAGE_SIZE);

        let (segment_1, segment_2) = self
            .gsp_mem
            .allocate_command(size_of::<FullCommand<M>>() + payload_size)?;

        // Per the `build_assert` above, we know that the header and command fit into the first
        // segment.
        let (cmd_slice, segment_1_payload) = segment_1.split_at_mut(size_of::<FullCommand<M>>());

        let seq = self.seq;
        let initializer = init!(FullCommand {
            hdr: GspMsgElement::new(seq, size_of::<M>() + payload_size, M::FUNCTION),
            cmd <- init,
        });

        // Fill the header and command in-place.
        unsafe {
            initializer.__init(cmd_slice.as_mut_ptr().cast())?;
        }

        // Fill the payload.
        let mut sbuffer = SBuffer::new_writer([&mut segment_1_payload[..], &mut segment_2[..]]);
        init_payload(&mut sbuffer)?;
        if !sbuffer.is_empty() {
            dev_warn!(
                &self.dev,
                "sent command did not fill the whole requested payload"
            );
            return Err(EINVAL);
        }
        drop(sbuffer);

        // Compute the checksum and update it on the initialized header.
        let checksum = GspCmdq::calculate_checksum(SBuffer::new_reader([&*segment_1, &*segment_2]));
        let msg_header =
            GspMsgElement::from_bytes_mut(segment_1.split_at_mut(size_of::<GspMsgElement>()).0)
                .ok_or(EINVAL)?;
        msg_header.checkSum = checksum;

        let rpc_header = &msg_header.rpc;
        // TODO: dev_dbg?
        dev_info!(
            &self.dev,
            "GSP RPC: send: seq# {}, function=0x{:x} ({}), length=0x{:x}\n",
            self.seq,
            rpc_header.function,
            decode_gsp_function(rpc_header.function),
            rpc_header.length,
        );

        // Advance write pointer, and signal availability of a new command to the GSP.
        let elem_count = msg_header.elemCount;
        self.seq += 1;
        self.gsp_mem.advance_cpu_write_ptr(elem_count);
        NV_PGSP_QUEUE_HEAD::default().set_address(0).write(bar);

        Ok(())
    }

    pub(crate) fn send_gsp_command_with_payload<M: GspCommandToGspWithPayload>(
        &mut self,
        bar: &Bar0,
        init: impl Init<M>,
        payload_size: usize,
        init_payload: impl FnOnce(&mut SBuffer<core::array::IntoIter<&mut [u8], 2>>) -> Result,
    ) -> Result {
        self.send_gsp_command_internal(bar, init, payload_size, init_payload)
    }

    pub(crate) fn send_gsp_command<M: GspCommandToGspWithoutPayload>(
        &mut self,
        bar: &Bar0,
        init: impl Init<M>,
    ) -> Result {
        self.send_gsp_command_internal(bar, init, 0, |_| Ok(()))
    }

    pub(crate) fn receive_msg_from_gsp<M: GspMessageFromGsp, R>(
        &mut self,
        timeout: Delta,
        init: impl FnOnce(&M, SBuffer<core::array::IntoIter<&[u8], 2>>) -> Result<R>,
    ) -> Result<R> {
        let (driver_area, msg_header, slice_1) = wait_on(timeout, || {
            let driver_area = self.gsp_mem.driver_read_area();
            if driver_area.0.as_flattened().len() < size_of::<GspMsgElement>() {
                return None;
            }
            // TODO: find an alternative to as_flattened()
            #[allow(clippy::incompatible_msrv)]
            let (msg_header_slice, slice_1) = driver_area
                .0
                .as_flattened()
                .split_at(size_of::<GspMsgElement>());

            // Can't fail because msg_slice will always be
            // size_of::<GspMsgElement>() bytes long by the above split.
            let msg_header = GspMsgElement::from_bytes(msg_header_slice).unwrap();
            if msg_header.rpc.length < size_of::<M>() as u32 {
                return None;
            }

            Some((driver_area, msg_header, slice_1))
        })?;

        // Log RPC receive with message type decoding
        dev_info!(
            self.dev,
            "GSP RPC: receive: seq# {}, function=0x{:x} ({}), length=0x{:x}\n",
            msg_header.rpc.sequence,
            msg_header.rpc.function,
            decode_gsp_function(msg_header.rpc.function),
            msg_header.rpc.length,
        );

        if msg_header.rpc.function != M::FUNCTION {
            self.gsp_mem.advance_cpu_read_ptr(
                (size_of_val(msg_header) as u32 - size_of_val(&msg_header.rpc) as u32
                    + msg_header.rpc.length)
                    .div_ceil(GSP_PAGE_SIZE as u32),
            );
            return Err(ERANGE);
        }

        let (cmd_slice, payload_1) = slice_1.split_at(size_of::<M>());
        let cmd = M::from_bytes(cmd_slice).ok_or(EINVAL)?;
        // TODO: find an alternative to as_flattened()
        #[allow(clippy::incompatible_msrv)]
        let payload_2 = driver_area.1.as_flattened();

        if GspCmdq::calculate_checksum(SBuffer::new_reader([
            msg_header.as_bytes(),
            cmd.as_bytes(),
            payload_1,
            payload_2,
        ])) != 0
        {
            dev_err!(
                self.dev,
                "GSP RPC: receive: Call {} - bad checksum",
                msg_header.rpc.sequence
            );
            return Err(EIO);
        }

        let sbuffer = SBuffer::new_reader([payload_1, payload_2]);
        let result = init(cmd, sbuffer);

        // Err should we also consider the size of the msg header??
        // TODO: make this a method of msg_header!
        self.gsp_mem.advance_cpu_read_ptr(
            (size_of_val(msg_header) as u32 - size_of_val(&msg_header.rpc) as u32
                + msg_header.rpc.length)
                .div_ceil(GSP_PAGE_SIZE as u32),
        );
        result
    }

    pub(crate) fn dma_handle(&self) -> DmaAddress {
        self.gsp_mem.0.dma_handle()
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
