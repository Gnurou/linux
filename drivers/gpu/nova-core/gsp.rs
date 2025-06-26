// SPDX-License-Identifier: GPL-2.0

use core::alloc::Layout;
use core::mem::{offset_of, MaybeUninit};

use kernel::alloc::allocator::Kmalloc;
use kernel::alloc::Allocator;
use kernel::asm;
use kernel::bindings;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::pci;
use kernel::pr_info;
use kernel::prelude::*;
use kernel::time::Delta;
use kernel::transmute::{AsBytes, FromBytes, FromBytesSized};
use kernel::{dma_read, dma_write};

use crate::dma::DmaObject;
use crate::driver::Bar0;
use crate::fb::FbLayout;
use crate::firmware::Firmware;
use crate::nvfw::r570_144 as fw;
use crate::regs::NV_PGSP_QUEUE_HEAD;
use crate::sbuffer::SBuffer;
use crate::util::wait_on_result;

pub(crate) mod sequencer;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_SHIFT: u64 = 1 << 20;

unsafe impl FromBytesSized for fw::GSP_ARGUMENTS_CACHED {}
unsafe impl AsBytes for fw::GSP_ARGUMENTS_CACHED {}
unsafe impl AsBytes for fw::MESSAGE_QUEUE_INIT_ARGUMENTS {}
unsafe impl AsBytes for fw::GSP_SR_INIT_ARGUMENTS {}
unsafe impl FromBytesSized for fw::GspFwWprMeta {}
unsafe impl AsBytes for fw::GspFwWprMeta {}
unsafe impl FromBytesSized for fw::GspSystemInfo {}
unsafe impl AsBytes for fw::GspSystemInfo {}

// We provide this trait because not all our structs are Sized so therefore the
// AsBytes and FromBytes traits don't work. However we can provide default
// implementations for all structs that are Sized, which we do here.
//
// This also allows us to create a convenient internal representation of a
// message which is only converted to bytes when actually doing the call. See the
// registry for an example.
pub(crate) trait GspMessageElement: Sized {
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self>;
}

impl<T> GspMessageElement for T
where
    T: Sized + FromBytes,
{
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self> {
        return unsafe {
            let mut result = MaybeUninit::<Self>::uninit();
            let result_ptr = result.as_mut_ptr() as *mut u8;
            let result_slice = core::slice::from_raw_parts_mut(result_ptr, size_of::<Self>());
            sbuf.read_exact(result_slice)?;
            Ok(result.assume_init())
        };
    }
}

pub(crate) trait GspCommandElement {
    fn copy_to_sbuf<'a, I: Iterator<Item = &'a mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result;
    fn size(&self) -> usize;
}

impl<T> GspCommandElement for T
where
    T: Sized + AsBytes,
{
    fn copy_to_sbuf<'a, I: Iterator<Item = &'a mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        sbuf.write_all(self.as_bytes())
    }

    fn size(&self) -> usize {
        return size_of::<Self>();
    }
}

pub(crate) trait GspCommand: GspCommandElement {
    const FUNCTION: u32;
}

// This next section contains constants and structures hand-coded from the GSP
// headers We could replace these with bindgen versions, but that's a bit of a
// pain because they basically end up pulling in the world (ie. definitions for
// every rpc method). So for now the hand-coded ones are fine. They are just
// structs so we can easily move to bindgen generated ones if/when we want to.

// A GSP RPC header
#[repr(C)]
#[derive(Debug, Clone)]
struct GspRpcHeader {
    header_version: u32,
    signature: u32,
    length: u32,
    function: u32,
    rpc_result: u32,
    rpc_result_private: u32,
    sequence: u32,
    cpu_rm_gfid: u32,
}
unsafe impl FromBytesSized for GspRpcHeader {}
unsafe impl AsBytes for GspRpcHeader {}

// A GSP message element header
#[repr(C)]
#[derive(Debug, Clone)]
struct GspMsgHeader {
    auth_tag_buffer: [u8; 16],
    aad_buffer: [u8; 16],
    checksum: u32,
    sequence: u32,
    elem_count: u32,
    pad: u32,
}
unsafe impl FromBytesSized for GspMsgHeader {}
unsafe impl AsBytes for GspMsgHeader {}

// These next two structs come from msgq_priv.h. Hopefully the will never
// need updating once the ABI is stabalised.
#[repr(C)]
#[derive(Debug)]
struct MsgqTxHeader {
    version: u32,    // queue version
    size: u32,       // bytes, page aligned
    msg_size: u32,   // entry size, bytes, must be power-of-2, 16 is minimum
    msg_count: u32,  // number of entries in queue
    write_ptr: u32,  // message id of next slot
    flags: u32,      // if set it means "i want to swap RX"
    rx_hdr_off: u32, // Offset of msgqRxHeader from start of backing store
    entry_off: u32,  // Offset of entries from start of backing store
}
unsafe impl AsBytes for MsgqTxHeader {}

#[repr(C)]
#[derive(Debug)]
struct MsgqRxHeader {
    read_ptr: u32, // message id of last message read
}

/// Number of GSP pages making the Msgq.
const MSGQ_NUM_PAGES: usize = 0x3f;

#[repr(C, align(0x1000))]
#[derive(Debug)]
struct MsgqData {
    data: [[u8; GSP_PAGE_SIZE]; MSGQ_NUM_PAGES],
}

// Annoyingly there is no real equivalent of #define so we're forced to use a
// literal to specify the alignment above. So check that against the actual GSP
// page size here.
static_assert!(align_of::<MsgqData>() == GSP_PAGE_SIZE);

// There is no struct defined for this in the open-gpu-kernel-source headers.
// Instead it is defined by code in GspMsgQueuesInit().
#[repr(C)]
#[derive(Debug)]
struct Msgq {
    tx: MsgqTxHeader,
    rx: MsgqRxHeader,
    msgq: MsgqData,
}

#[repr(C)]
#[derive(Debug)]
struct GspMem {
    ptes: [u8; GSP_PAGE_SIZE],
    cpuq: Msgq,
    gspq: Msgq,
}

// Needed for CoherentAllocation
unsafe impl FromBytesSized for GspMem {}
unsafe impl AsBytes for GspMem {}

// SAFETY: this hack isn't :-) Only required until Nova core can boot GSP.
unsafe impl Send for GspCmdq {}

pub(crate) struct GspCmdq {
    msg_count: u32,
    seq: u32,
    gsp_mem: CoherentAllocation<GspMem>,
    nr_ptes: u32,
    libos_dma_handle: u64,
}

impl GspCmdq {
    // This is equivalent to gsp_shared_init()
    fn new(dev: &device::Device<device::Bound>, libos_dma_handle: u64) -> Result<GspCmdq> {
        let mut gsp_mem =
            CoherentAllocation::<GspMem>::alloc_coherent(dev, 1, GFP_KERNEL | __GFP_ZERO)?;

        let nr_ptes = size_of::<GspMem>() >> GSP_PAGE_SHIFT;
        build_assert!(nr_ptes * size_of::<u64>() <= GSP_PAGE_SIZE);

        create_pte_array(&mut gsp_mem, 0);

        const MSGQ_SIZE: u32 = size_of::<Msgq>() as u32;
        const MSG_COUNT: u32 = ((MSGQ_SIZE as usize - GSP_PAGE_SIZE) / GSP_PAGE_SIZE) as u32;
        const RX_HDR_OFF: u32 = offset_of!(Msgq, rx) as u32;
        dma_write!(
            gsp_mem[0].cpuq.tx = MsgqTxHeader {
                version: 0,
                size: MSGQ_SIZE,
                entry_off: GSP_PAGE_SIZE as u32,
                msg_size: GSP_PAGE_SIZE as u32,
                msg_count: MSG_COUNT,
                write_ptr: 0,
                flags: 1,
                rx_hdr_off: RX_HDR_OFF,
            }
        )?;

        Ok(GspCmdq {
            msg_count: MSG_COUNT,
            seq: 0,
            gsp_mem,
            nr_ptes: nr_ptes as u32,
            libos_dma_handle,
        })
    }

    // We need the next four accessors because the dma_read macro is failable
    // and uses `?` which requires any calling function to return a Result<>.
    // However in the first instance a dma_read failure probably needs to be dealt with
    // by the function trying to do the read, so we need the accessors to permit that.
    //
    // Of course at the moment we "deal" with errors by panicing...
    //
    // I think we need to update the dma macro's to return a Result<u32>
    fn cpu_wptr(self: &Self) -> Result<u32> {
        dma_read!(self.gsp_mem[0].cpuq.tx.write_ptr)
    }

    fn gsp_rptr(self: &Self) -> Result<u32> {
        dma_read!(self.gsp_mem[0].gspq.rx.read_ptr)
    }

    fn cpu_rptr(self: &Self) -> Result<u32> {
        dma_read!(self.gsp_mem[0].cpuq.rx.read_ptr)
    }

    fn gsp_wptr(self: &Self) -> Result<u32> {
        dma_read!(self.gsp_mem[0].gspq.tx.write_ptr)
    }

    // Returns the numbers of pages free for sending an RPC to GSP.
    fn free_tx_pages(self: &Self) -> u32 {
        let wptr = self.cpu_wptr().unwrap();
        let rptr = self.gsp_rptr().unwrap();
        let mut free = rptr + self.msg_count - wptr - 1;

        if free >= self.msg_count {
            free -= self.msg_count;
        }

        free
    }

    // Returns the number of pages the GSP has written to the queue.
    fn used_rx_pages(self: &Self) -> u32 {
        let rptr = self.cpu_rptr().unwrap();
        let wptr = self.gsp_wptr().unwrap();
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
            .fold(0, |acc, (rol, byte)| acc ^ (byte as u64).rotate_left(rol));

        ((sum64 >> 32) as u32) ^ (sum64 as u32)
    }

    fn alloc_cmd_sbuffer(self: &mut Self, cmd_size: usize) -> Result<(&mut [u8], &mut [u8])> {
        let msg_size = cmd_size.div_ceil(GSP_PAGE_SIZE);

        while self.free_tx_pages() < msg_size as u32 {}
        let wptr = self.cpu_wptr().unwrap() as usize;
        let ptr = unsafe {
            core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).cpuq.msgq.data[wptr])
        };

        // Simple case where the queue doesn't wrap
        if wptr + msg_size <= MSGQ_NUM_PAGES {
            let slice: &mut [u8] = unsafe {
                core::slice::from_raw_parts_mut(ptr as *mut u8, msg_size * GSP_PAGE_SIZE)
            };

            Ok((slice, &mut []))
        } else {
            // First slice contains the remaining free pages in the queue
            let slice_1: &mut [u8] = unsafe {
                core::slice::from_raw_parts_mut(
                    ptr as *mut u8,
                    (MSGQ_NUM_PAGES - wptr) * GSP_PAGE_SIZE,
                )
            };
            let ptr = unsafe {
                core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).cpuq.msgq.data[0])
            };
            pr_info!("msg_size {} wptr {}\n", msg_size, wptr);
            let slice_2: &mut [u8] = unsafe {
                core::slice::from_raw_parts_mut(
                    ptr as *mut u8,
                    (msg_size - MSGQ_NUM_PAGES + wptr) * GSP_PAGE_SIZE,
                )
            };
            Ok((slice_1, slice_2))
        }
    }

    pub(crate) fn send<A: GspCommand>(&mut self, bar: &Bar0, cmd: &A) -> Result<()> {
        let mut msg_header = GspMsgHeader {
            auth_tag_buffer: [0; 16],
            aad_buffer: [0; 16],
            checksum: 0,
            sequence: self.seq,
            elem_count: 1,
            pad: 0,
        };
        let rpc = GspRpcHeader {
            header_version: 0x03000000,
            signature: 0x43505256,
            length: (size_of::<GspRpcHeader>() + cmd.size()) as u32,
            function: A::FUNCTION,
            rpc_result: 0xffffffff,
            rpc_result_private: 0xffffffff,
            sequence: 0,
            cpu_rm_gfid: 0,
        };

        self.seq += 1;
        let cmd_len = size_of::<GspMsgHeader>() + rpc.length as usize;

        // `alloc_cmd_sbuffer` returns the two slices we need, and we build a SRead/Write buffer
        // from them.
        let (slice1, slice2) = self.alloc_cmd_sbuffer(cmd_len)?;

        let mut sbuf = SBuffer::new_writer([&mut slice1[..], &mut slice2[..]]);

        sbuf.write_all(msg_header.as_bytes())?;
        sbuf.write_all(rpc.as_bytes())?;
        cmd.copy_to_sbuf(&mut sbuf)?;
        drop(sbuf);

        msg_header.checksum = 0;
        msg_header.elem_count = cmd_len.div_ceil(GSP_PAGE_SIZE) as u32;

        // Calculate checksum over the entire message
        msg_header.checksum =
            GspCmdq::calculate_checksum(SBuffer::new_reader([&slice1[..], &slice2[..]]));

        // Re-write the message header with the updated element count and checksum
        let mut sbuf = SBuffer::new_writer([slice1, slice2]);
        sbuf.write_all(msg_header.as_bytes())?;
        drop(sbuf);

        let mut wptr = self.cpu_wptr().unwrap() as u32;
        wptr += msg_header.elem_count as u32;
        wptr %= MSGQ_NUM_PAGES as u32;

        // TODO: Figure out Rust barriers
        unsafe {
            asm!("sfence";);
            dma_write!(self.gsp_mem[0].cpuq.tx.write_ptr = wptr)?;
            asm!("mfence";);
        };

        NV_PGSP_QUEUE_HEAD::default()
            .set_address(0 as u32)
            .write(bar);

        Ok(())
    }

    pub(crate) fn receive<A: GspMessageElement>(self: &mut Self, function: u32) -> Result<A> {
        const HEADER_SIZE: u32 = (size_of::<GspMsgHeader>() + size_of::<GspRpcHeader>()) as u32;

        // Used pages contains the total number of pages available to consume
        let used_pages = self.used_rx_pages();
        if used_pages < HEADER_SIZE.div_ceil(GSP_PAGE_SIZE as u32) {
            return Err(EAGAIN);
        }

        let rptr = self.cpu_rptr().unwrap();

        // Remaining number of bytes left before we have to wrap
        let remaining = if rptr + used_pages > self.msg_count {
            (self.msg_count - rptr) << GSP_PAGE_SHIFT
        } else {
            used_pages << GSP_PAGE_SHIFT
        };

        let ptr = unsafe {
            core::ptr::addr_of_mut!((*self.gsp_mem.start_ptr_mut()).gspq.msgq.data[rptr as usize])
        };
        let msg_slice =
            unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, remaining as usize) };

        // TODO: Validating the checksum will read this
        let _msg = GspMsgHeader::from_bytes(&msg_slice[0..size_of::<GspMsgHeader>()])
            .ok_or(EINVAL)?
            .clone();
        let rpc = GspRpcHeader::from_bytes(
            &msg_slice
                [size_of::<GspMsgHeader>()..size_of::<GspMsgHeader>() + size_of::<GspRpcHeader>()],
        )
        .ok_or(EINVAL)?
        .clone();

        // rpc.length includes the size of the GspRpcHeader. Remove it to make
        // the rest of the code a bit easier to follow.
        let rpc_length = rpc.length - size_of::<GspRpcHeader>() as u32;

        // Not all pages of the message have made it to the queue so bail and let the caller retry.
        if used_pages << GSP_PAGE_SHIFT < HEADER_SIZE + rpc_length {
            return Err(EAGAIN);
        }

        let result = if rpc.length + HEADER_SIZE < remaining {
            let mut sbuf = SBuffer::new_reader([
                &msg_slice[(HEADER_SIZE as usize)..(HEADER_SIZE + rpc.length) as usize]
            ]);
            A::new_from_sbuf(&mut sbuf)
        } else {
            let slice_1 = &msg_slice[(HEADER_SIZE as usize)..(HEADER_SIZE + remaining) as usize];
            let ptr =
                unsafe { core::ptr::addr_of!((*self.gsp_mem.start_ptr_mut()).gspq.msgq.data[0]) };
            let slice_2 = unsafe {
                core::slice::from_raw_parts(ptr as *const u8, rpc.length as usize - slice_1.len())
            };

            let mut sbuf = SBuffer::new_reader([slice_1, slice_2]);

            A::new_from_sbuf(&mut sbuf)
        };

        let result = if rpc.function == function {
            result
        } else {
            Err(ERANGE)
        };

        let mut rptr = self.cpu_rptr()?;
        rptr = rptr + (HEADER_SIZE + rpc_length).div_ceil(GSP_PAGE_SIZE as u32);
        rptr %= MSGQ_NUM_PAGES as u32;

        // TODO: Figure out Rust barriers
        unsafe {
            asm!("mfence";);
            dma_write!(self.gsp_mem[0].cpuq.rx.read_ptr = rptr)?;
        };

        result
    }

    /// Same as the `receive_wait()` method but will consume and ingnore
    /// unexpected messages. Ie. messages with a different function to the passed
    /// `function` parameter.
    fn receive_wait_ignore<R: GspMessageElement>(
        &mut self,
        timeout: Delta,
        function: u32,
    ) -> Result<R> {
        wait_on_result(timeout, || match self.receive::<R>(function) {
            Ok(x) => Some(Ok(x)),
            Err(EAGAIN) => None,
            Err(ERANGE) => None,
            Err(e) => Some(Err(e)),
        })
    }

    pub(crate) fn gsp_init_done(&mut self, timeout: Delta) -> Result {
        self.receive_wait_ignore::<EmptyCmd>(timeout, fw::NV_VGPU_MSG_EVENT_GSP_INIT_DONE)
            .map(|_| ())
    }
}

struct EmptyCmd {
    size: usize,
}

impl GspMessageElement for EmptyCmd {
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self> {
        Ok(Self { size: sbuf.count() })
    }
}

impl GspCommandElement for EmptyCmd {
    fn size(&self) -> usize {
        self.size
    }

    fn copy_to_sbuf<'a, I: Iterator<Item = &'a mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        for _i in 0..self.size {
            sbuf.write_all(&[0])?;
        }

        Ok(())
    }
}

pub(crate) fn build_wpr_meta(
    dev: &device::Device<device::Bound>,
    fw: &Firmware,
    fb_layout: &FbLayout,
) -> Result<CoherentAllocation<fw::GspFwWprMeta>> {
    let wpr_meta =
        CoherentAllocation::<fw::GspFwWprMeta>::alloc_coherent(dev, 1, GFP_KERNEL | __GFP_ZERO)?;
    dma_write!(
        wpr_meta[0] = fw::GspFwWprMeta {
            magic: fw::GSP_FW_WPR_META_MAGIC as u64,
            revision: fw::GSP_FW_WPR_META_REVISION as u64,
            sysmemAddrOfRadix3Elf: fw.gsp.lvl0_dma_handle() as u64,
            sizeOfRadix3Elf: fw.gsp.size() as u64,
            sysmemAddrOfBootloader: fw.bootloader.ucode.dma_handle(),
            sizeOfBootloader: fw.bootloader.ucode.size() as u64,
            bootloaderCodeOffset: fw.bootloader.code_offset as u64,
            bootloaderDataOffset: fw.bootloader.data_offset as u64,
            bootloaderManifestOffset: fw.bootloader.manifest_offset as u64,
            __bindgen_anon_1: fw::GspFwWprMeta__bindgen_ty_1 {
                __bindgen_anon_1: fw::GspFwWprMeta__bindgen_ty_1__bindgen_ty_1 {
                    sysmemAddrOfSignature: fw.gsp_sigs.dma_handle() as u64,
                    sizeOfSignature: fw.gsp_sigs.size() as u64,
                }
            },
            gspFwRsvdStart: fb_layout.heap.start,
            nonWprHeapOffset: fb_layout.heap.start,
            nonWprHeapSize: fb_layout.heap.end - fb_layout.heap.start,
            gspFwWprStart: fb_layout.wpr2.start,
            gspFwHeapOffset: fb_layout.wpr2_heap.start,
            gspFwHeapSize: fb_layout.wpr2_heap.end - fb_layout.wpr2_heap.start,
            gspFwOffset: fb_layout.elf.start,
            bootBinOffset: fb_layout.boot.start,
            frtsOffset: fb_layout.frts.start,
            frtsSize: fb_layout.frts.end - fb_layout.frts.start,
            gspFwWprEnd: fb_layout.vga_workspace.start & !(0x20000 - 1),
            gspFwHeapVfPartitionCount: fb_layout.vf_partition_count,
            fbSize: fb_layout.fb.end - fb_layout.fb.start,
            vgaWorkspaceOffset: fb_layout.vga_workspace.start,
            vgaWorkspaceSize: fb_layout.vga_workspace.end - fb_layout.vga_workspace.start,
            bootCount: 0,
            __bindgen_anon_2: fw::GspFwWprMeta__bindgen_ty_2 {
                __bindgen_anon_1: fw::GspFwWprMeta__bindgen_ty_2__bindgen_ty_1 {
                    partitionRpcAddr: 0,
                    partitionRpcRequestOffset: 0,
                    partitionRpcReplyOffset: 0,
                    ..Default::default()
                },
            },
            verified: 0,
            ..Default::default()
        }
    )?;

    Ok(wpr_meta)
}

#[allow(unused)]
pub(crate) struct GspMemObjects {
    libos: DmaObject,
    pub loginit: DmaObject,
    pub logintr: DmaObject,
    pub logrm: DmaObject,
    rmargs: CoherentAllocation<fw::GSP_ARGUMENTS_CACHED>,
    pub cmdq: GspCmdq,
}

/// Generates the `ID8` identifier required for some GSP objects.
fn id8(name: &str) -> u64 {
    let mut bytes = [0u8; core::mem::size_of::<u64>()];

    for (c, b) in name.bytes().rev().zip(&mut bytes) {
        *b = c;
    }

    u64::from_ne_bytes(bytes)
}

/// Creates a self-mapping page table for `obj` at its beginning.
fn create_pte_array<T: AsBytes + FromBytes>(obj: &mut CoherentAllocation<T>, skip: usize) {
    let num_pages = obj.size().div_ceil(GSP_PAGE_SIZE);
    let handle = obj.dma_handle();

    let ptes = unsafe {
        let ptr = obj.start_ptr_mut().cast::<u64>().add(skip);
        core::slice::from_raw_parts_mut(ptr, num_pages)
    };

    for (i, pte) in ptes.iter_mut().enumerate() {
        *pte = handle as u64 + ((i as u64) << GSP_PAGE_SHIFT);
    }
}

/// Creates a new `DmaObject` with `name` of `size`, and register it into the `libos` object at
/// argument position `libos_arg_nr`.
fn create_dma_object(
    dev: &device::Device<device::Bound>,
    name: &'static str,
    size: usize,
    libos: &mut DmaObject,
    libos_arg_nr: usize,
) -> Result<DmaObject> {
    let mut obj = DmaObject::new(dev, size)?;
    create_pte_array(&mut obj, 1);

    let arg_offset = libos_arg_nr * size_of::<fw::LibosMemoryRegionInitArgument>();
    let libos_start_ptr = unsafe { libos.start_ptr_mut().add(arg_offset) };

    let libos_mem_init_args = fw::LibosMemoryRegionInitArgument {
        id8: id8(name),
        pa: obj.dma_handle(),
        size: obj.size() as u64,
        kind: fw::LibosMemoryRegionKind_LIBOS_MEMORY_REGION_CONTIGUOUS as u8,
        loc: fw::LibosMemoryRegionLoc_LIBOS_MEMORY_REGION_LOC_SYSMEM as u8,
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            &libos_mem_init_args as *const fw::LibosMemoryRegionInitArgument,
            libos_start_ptr as *mut fw::LibosMemoryRegionInitArgument,
            1,
        );
    };

    Ok(obj)
}

const GSP_REGISTRY_NUM_ENTRIES: usize = 2;
struct RegistryEntry {
    key: &'static str,
    value: u32,
}

struct RegistryTable {
    entries: [RegistryEntry; GSP_REGISTRY_NUM_ENTRIES],
}

impl GspCommandElement for RegistryTable {
    fn copy_to_sbuf<'a, I: Iterator<Item = &'a mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        let total_size = self.size();
        let align = core::mem::align_of::<fw::PACKED_REGISTRY_TABLE>();
        let layout = Layout::from_size_align(total_size, align)
            .map_err(|_| ENOMEM)
            .unwrap();
        let cmd_slice = unsafe {
            // Use the kernel allocator which respects alignment
            let allocation = Kmalloc::alloc(layout, GFP_KERNEL | __GFP_ZERO).unwrap();
            let ptr = allocation.as_ptr() as *mut u8;

            // Verify alignment (debug only)
            debug_assert_eq!(ptr as usize % align, 0);

            // Serialize the data into the allocated memory
            let table = ptr as *mut fw::PACKED_REGISTRY_TABLE;
            let mut table_data = ptr.add(
                size_of::<fw::PACKED_REGISTRY_TABLE>()
                    + GSP_REGISTRY_NUM_ENTRIES * size_of::<fw::PACKED_REGISTRY_ENTRY>(),
            );

            (*table).numEntries = GSP_REGISTRY_NUM_ENTRIES as u32;
            (*table).size = total_size as u32;

            for i in 0..GSP_REGISTRY_NUM_ENTRIES {
                let entry_ptr = ptr.add(
                    size_of::<fw::PACKED_REGISTRY_TABLE>()
                        + i * size_of::<fw::PACKED_REGISTRY_ENTRY>(),
                ) as *mut fw::PACKED_REGISTRY_ENTRY;

                (*entry_ptr).nameOffset = table_data.offset_from(table as *const u8) as u32;
                (*entry_ptr).type_ = fw::REGISTRY_TABLE_ENTRY_TYPE_DWORD as u8;
                (*entry_ptr).data = self.entries[i].value;
                (*entry_ptr).length = 0;

                // Copy the key string to table_data and null terminate it
                let key_bytes = self.entries[i].key.as_bytes();
                core::ptr::copy_nonoverlapping(key_bytes.as_ptr(), table_data, key_bytes.len());
                table_data = table_data.add(key_bytes.len());
                *table_data = 0; // Add null terminator
                table_data = table_data.add(1); // Move past null terminator
            }

            core::slice::from_raw_parts(ptr as *const u8, layout.size())
        };

        sbuf.write_all(cmd_slice)?;

        // Free the allocated memory by converting slice back to pointer.
        unsafe {
            use core::ptr::NonNull;
            let ptr = cmd_slice.as_ptr() as *mut u8;
            let ptr_nn = NonNull::new_unchecked(ptr);
            Kmalloc::free(ptr_nn, layout);
        }

        Ok(())
    }

    fn size(&self) -> usize {
        let mut key_size = 0;
        for i in 0..GSP_REGISTRY_NUM_ENTRIES {
            key_size += self.entries[i].key.len() + 1; // +1 for NULL terminator
        }
        size_of::<fw::PACKED_REGISTRY_TABLE>()
            + GSP_REGISTRY_NUM_ENTRIES * size_of::<fw::PACKED_REGISTRY_ENTRY>()
            + key_size
    }
}

impl GspCommand for RegistryTable {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_SET_REGISTRY;
}

fn build_registry(bar: &Bar0, cmdq: &mut GspCmdq) {
    let registry = RegistryTable {
        entries: [
            RegistryEntry {
                key: "RMSecBusResetEnable",
                value: 1,
            },
            RegistryEntry {
                key: "RMForcePcieConfigSave",
                value: 1,
            },
        ],
    };

    cmdq.send(bar, &registry).unwrap();
}

impl GspCommand for fw::GspSystemInfo {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO;
}

fn set_system_info(dev: &pci::Device<device::Bound>, bar: &Bar0, cmdq: &mut GspCmdq) -> Result {
    let mut info = unsafe { MaybeUninit::<fw::GspSystemInfo>::zeroed().assume_init() };

    info.gpuPhysAddr = dev.resource_start(0)?;
    info.gpuPhysFbAddr = dev.resource_start(1)?;
    info.gpuPhysInstAddr = dev.resource_start(3)?;
    info.nvDomainBusDeviceFunc = dev.dev_id() as u64;

    // Using TASK_SIZE in r535_gsp_rpc_set_system_info() seems wrong because
    // TASK_SIZE is per-task. That's probably a design issue in GSP-RM though.
    info.maxUserVa = (1 << 47) - 4096;
    info.pciConfigMirrorBase = 0x088000;
    info.pciConfigMirrorSize = 0x001000;

    info.PCIDeviceID = ((dev.device_id() as u32) << 16) | dev.vendor_id() as u32;
    info.PCISubDeviceID =
        ((dev.subsystem_device_id() as u32) << 16) | dev.subsystem_vendor_id() as u32;
    info.PCIRevisionID = dev.revision_id() as u32;
    info.bIsPrimary = 0;
    info.bPreserveVideoMemoryAllocations = 0;

    cmdq.send(bar, &info)?;
    Ok(())
}

fn create_coherent_dma_object<A: AsBytes + FromBytes>(
    dev: &device::Device<device::Bound>,
    name: &'static str,
    libos: &mut DmaObject,
    libos_arg_nr: usize,
) -> Result<CoherentAllocation<A>> {
    let obj = CoherentAllocation::<A>::alloc_coherent(dev, 1, GFP_KERNEL | __GFP_ZERO)?;

    let arg_offset = libos_arg_nr * size_of::<fw::LibosMemoryRegionInitArgument>();
    let libos_start_ptr = unsafe { libos.start_ptr_mut().add(arg_offset) };

    let libos_mem_init_args = fw::LibosMemoryRegionInitArgument {
        id8: id8(name),
        pa: obj.dma_handle(),
        size: obj.size() as u64,
        kind: fw::LibosMemoryRegionKind_LIBOS_MEMORY_REGION_CONTIGUOUS as u8,
        loc: fw::LibosMemoryRegionLoc_LIBOS_MEMORY_REGION_LOC_SYSMEM as u8,
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            &libos_mem_init_args as *const fw::LibosMemoryRegionInitArgument,
            libos_start_ptr as *mut fw::LibosMemoryRegionInitArgument,
            1,
        );
    };

    Ok(obj)
}

impl GspMemObjects {
    pub(crate) fn new(pdev: &pci::Device<device::Bound>, bar: &Bar0) -> Result<Self> {
        let dev = pdev.as_ref();
        let mut libos = DmaObject::new(dev, GSP_PAGE_SIZE)?;
        let loginit = create_dma_object(dev, "LOGINIT", 0x10000, &mut libos, 0)?;
        let logintr = create_dma_object(dev, "LOGINTR", 0x10000, &mut libos, 1)?;
        let logrm = create_dma_object(dev, "LOGRM", 0x10000, &mut libos, 2)?;

        // Creates its own PTE array
        let mut cmdq = GspCmdq::new(dev, libos.dma_handle())?;
        let rmargs =
            create_coherent_dma_object::<fw::GSP_ARGUMENTS_CACHED>(dev, "RMARGS", &mut libos, 3)?;
        dma_write!(
            rmargs[0].messageQueueInitArguments = fw::MESSAGE_QUEUE_INIT_ARGUMENTS {
                sharedMemPhysAddr: cmdq.gsp_mem.dma_handle(),
                pageTableEntryCount: cmdq.nr_ptes,
                cmdQueueOffset: core::mem::offset_of!(Msgq, msgq) as u64,
                statQueueOffset: (core::mem::offset_of!(GspMem, gspq)
                    - core::mem::offset_of!(GspMem, cpuq)
                    + core::mem::offset_of!(Msgq, msgq)) as u64,
            }
        )?;
        dma_write!(
            rmargs[0].srInitArguments = fw::GSP_SR_INIT_ARGUMENTS {
                oldLevel: 0,
                flags: 0,
                bInPMTransition: 0,
            }
        )?;
        dma_write!(rmargs[0].bDmemStack = 1)?;

        set_system_info(pdev, bar, &mut cmdq)?;
        build_registry(bar, &mut cmdq);

        Ok(GspMemObjects {
            libos,
            loginit,
            logintr,
            logrm,
            rmargs,
            cmdq,
        })
    }

    pub(crate) fn libos_dma_handle(&self) -> bindings::dma_addr_t {
        self.libos.dma_handle()
    }
}
