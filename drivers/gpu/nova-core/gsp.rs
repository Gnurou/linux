// SPDX-License-Identifier: GPL-2.0

use kernel::device;
use kernel::prelude::*;

use crate::dma::DmaObject;
use crate::fb::FbLayout;
use crate::firmware::Firmware;
use crate::nvfw::r570_133_07 as fw;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_SHIFT: u64 = 1 << 20;

pub(crate) fn build_wpr_meta(
    dev: &device::Device<device::Bound>,
    fw: &Firmware,
    fb_layout: &FbLayout,
) -> Result<DmaObject> {
    // TODO: Can we use dma_write!() with a properly defined structure instead?
    let mut wpr_meta = DmaObject::new(dev, GSP_PAGE_SIZE)?;
    fw::s_GspFwWprMeta::new(wpr_meta.start_ptr_mut())
        .magic(fw::GSP_FW_WPR_META_MAGIC)
        .revision(fw::GSP_FW_WPR_META_REVISION as u64)
        .sysmemAddrOfRadix3Elf(fw.gsp.lvl0_dma_handle() as u64)
        .sizeOfRadix3Elf(fw.gsp.size() as u64)
        .sysmemAddrOfBootloader(fw.bootloader.ucode.dma_handle())
        .sizeOfBootloader(fw.bootloader.ucode.size() as u64)
        .bootloaderCodeOffset(fw.bootloader.code_offset as u64)
        .bootloaderDataOffset(fw.bootloader.data_offset as u64)
        .bootloaderManifestOffset(fw.bootloader.manifest_offset as u64)
        .sysmemAddrOfSignature(fw.gsp_sigs.dma_handle() as u64)
        .sizeOfSignature(fw.gsp_sigs.size() as u64)
        .gspFwRsvdStart(fb_layout.heap.start)
        .nonWprHeapOffset(fb_layout.heap.start)
        .nonWprHeapSize(fb_layout.heap.end - fb_layout.heap.start)
        .gspFwWprStart(fb_layout.wpr2.start)
        .gspFwHeapOffset(fb_layout.wpr2_heap.start)
        .gspFwHeapSize(fb_layout.wpr2_heap.end - fb_layout.wpr2_heap.start)
        .gspFwOffset(fb_layout.elf.start)
        .bootBinOffset(fb_layout.boot.start)
        .frtsOffset(fb_layout.frts.start)
        .frtsSize(fb_layout.frts.end - fb_layout.frts.start)
        .gspFwWprEnd(fb_layout.vga_workspace.start & !(0x20000 - 1))
        .gspFwHeapVfPartitionCount(fb_layout.vf_partition_count)
        .fbSize(fb_layout.fb.end - fb_layout.fb.start)
        .vgaWorkspaceOffset(fb_layout.vga_workspace.start)
        .vgaWorkspaceSize(fb_layout.vga_workspace.end - fb_layout.vga_workspace.start)
        .bootCount(0)
        .partitionRpcAddr(0)
        .partitionRpcRequestOffset(0)
        .partitionRpcReplyOffset(0)
        .verified(0);

    Ok(wpr_meta)
}

#[allow(unused)]
pub(crate) struct GspSharedMemObjects {
    libos: DmaObject,
    loginit: DmaObject,
    logintr: DmaObject,
    logrm: DmaObject,
    rmargs: DmaObject,
    kern: Option<DmaObject>,
    shm: DmaObject,
    wpr_meta: DmaObject,
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
fn create_pte_array(obj: &mut DmaObject) {
    let num_pages = obj.size().div_ceil(GSP_PAGE_SIZE);
    let handle = obj.dma_handle();

    let ptes = unsafe {
        let ptr = obj
            .start_ptr_mut()
            .add(core::mem::size_of::<u64>())
            .cast::<u64>();
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
    create_pte_array(&mut obj);

    let arg_offset = libos_arg_nr * fw::s_LibosMemoryRegionInitArgument::str_size();
    let libos_start_ptr = unsafe { libos.start_ptr_mut().add(arg_offset) };

    fw::s_LibosMemoryRegionInitArgument::new(libos_start_ptr)
        .id8(id8(name))
        .pa(obj.dma_handle())
        .size(obj.size() as u64)
        .kind(fw::LIBOS_MEMORY_REGION_CONTIGUOUS as u8)
        .loc(fw::LIBOS_MEMORY_REGION_LOC_SYSMEM as u8);

    Ok(obj)
}

impl GspSharedMemObjects {
    pub(crate) fn new(dev: &device::Device<device::Bound>) -> Result<Self> {
        let mut libos = DmaObject::new(dev, GSP_PAGE_SIZE)?;

        let loginit = create_dma_object(dev, "LOGINIT", 0x10000, &mut libos, 0);
        let logintr = create_dma_object(dev, "LOGINTR", 0x10000, &mut libos, 1);
        let logrm = create_dma_object(dev, "LOGRM", 0x10000, &mut libos, 2);
        let rmargs = create_dma_object(dev, "RMARGS", 0x1000, &mut libos, 3);

        // TODO: initialize rmargs and shm as per r535_gsp_rmargs_init.
        // TODO: also kernel from Dave's branch?

        Err(EINVAL)
    }
}
