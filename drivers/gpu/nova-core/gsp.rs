// SPDX-License-Identifier: GPL-2.0

use kernel::alloc::flags::GFP_KERNEL;
use kernel::bindings;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::dma_write;
use kernel::pci;
use kernel::prelude::*;
use kernel::transmute::{AsBytes, FromBytes};

use crate::driver::Bar0;
use crate::fb::FbLayout;
use crate::firmware::Firmware;
use crate::gsp::cmdq::GspCmdq;
use crate::gsp::commands::{build_registry, set_system_info};
use crate::nvfw::r570_144 as fw;

pub(crate) mod cmdq;
pub(crate) mod commands;

pub(crate) mod sequencer;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_SHIFT: u64 = 1 << 20;

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::LibosMemoryRegionInitArgument {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for fw::LibosMemoryRegionInitArgument {}

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::GspFwWprMeta {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for fw::GspFwWprMeta {}

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::GSP_ARGUMENTS_CACHED {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for fw::GSP_ARGUMENTS_CACHED {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::MESSAGE_QUEUE_INIT_ARGUMENTS {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::GSP_SR_INIT_ARGUMENTS {}

#[allow(unused)]
pub(crate) struct GspMemObjects {
    libos: CoherentAllocation<fw::LibosMemoryRegionInitArgument>,
    pub loginit: CoherentAllocation<u8>,
    pub logintr: CoherentAllocation<u8>,
    pub logrm: CoherentAllocation<u8>,
    pub wpr_meta: CoherentAllocation<fw::GspFwWprMeta>,
    pub cmdq: GspCmdq,
    rmargs: CoherentAllocation<fw::GSP_ARGUMENTS_CACHED>,
}

// TODO: use dedicated type for coherent allocation of GspFwWprMeta? And make it part of the
// firmware struct to guarantee ownership?
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
            revision: u64::from(fw::GSP_FW_WPR_META_REVISION),
            sysmemAddrOfRadix3Elf: fw.gsp.lvl0_dma_handle(),
            sizeOfRadix3Elf: fw.gsp.size as u64,
            sysmemAddrOfBootloader: fw.bootloader.ucode.dma_handle(),
            sizeOfBootloader: fw.bootloader.ucode.size() as u64,
            bootloaderCodeOffset: u64::from(fw.bootloader.code_offset),
            bootloaderDataOffset: u64::from(fw.bootloader.data_offset),
            bootloaderManifestOffset: u64::from(fw.bootloader.manifest_offset),
            __bindgen_anon_1: fw::GspFwWprMeta__bindgen_ty_1 {
                __bindgen_anon_1: fw::GspFwWprMeta__bindgen_ty_1__bindgen_ty_1 {
                    sysmemAddrOfSignature: fw.gsp_sigs.dma_handle(),
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
            // TODO: magic number!
            gspFwWprEnd: fb_layout.vga_workspace.start & !(0x20000 - 1),
            gspFwHeapVfPartitionCount: fb_layout.vf_partition_count,
            fbSize: fb_layout.fb.end - fb_layout.fb.start,
            vgaWorkspaceOffset: fb_layout.vga_workspace.start,
            vgaWorkspaceSize: fb_layout.vga_workspace.end - fb_layout.vga_workspace.start,
            bootCount: 0,
            verified: 0,
            ..Default::default()
        }
    )?;

    Ok(wpr_meta)
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

    // SAFETY:
    //  - By the invariants of the CoherentAllocation ptr is non-NULL.
    //  - CoherentAllocation CPU addresses are always aligned to a
    //    page-boundary, satisfying the alignement requirements for
    //    from_raw_parts_mut()
    //  - The allocation size is at least as long as 8 * num_pages as
    //    GSP_PAGE_SIZE is larger than 8 bytes.
    let ptes = unsafe {
        let ptr = obj.start_ptr_mut().cast::<u64>().add(skip);
        core::slice::from_raw_parts_mut(ptr, num_pages)
    };

    for (i, pte) in ptes.iter_mut().enumerate() {
        *pte = handle + ((i as u64) << GSP_PAGE_SHIFT);
    }
}

/// Creates a new `CoherentAllocation<A>` with `name` of `size` elements, and
/// register it into the `libos` object at argument position `libos_arg_nr`.
fn create_coherent_dma_object<A: AsBytes + FromBytes>(
    dev: &device::Device<device::Bound>,
    name: &'static str,
    size: usize,
    libos: &mut CoherentAllocation<fw::LibosMemoryRegionInitArgument>,
    libos_arg_nr: usize,
) -> Result<CoherentAllocation<A>> {
    let obj = CoherentAllocation::<A>::alloc_coherent(dev, size, GFP_KERNEL | __GFP_ZERO)?;

    dma_write!(
        libos[libos_arg_nr] = fw::LibosMemoryRegionInitArgument {
            id8: id8(name),
            pa: obj.dma_handle(),
            size: obj.size() as u64,
            kind: fw::LibosMemoryRegionKind_LIBOS_MEMORY_REGION_CONTIGUOUS as u8,
            loc: fw::LibosMemoryRegionLoc_LIBOS_MEMORY_REGION_LOC_SYSMEM as u8,
        }
    )?;

    Ok(obj)
}

impl GspMemObjects {
    pub(crate) fn new(
        pdev: &pci::Device<device::Bound>,
        bar: &Bar0,
        fw: &Firmware,
        fb_layout: &FbLayout,
    ) -> Result<Self> {
        let dev = pdev.as_ref();
        let mut libos = CoherentAllocation::<fw::LibosMemoryRegionInitArgument>::alloc_coherent(
            dev,
            GSP_PAGE_SIZE / size_of::<fw::LibosMemoryRegionInitArgument>(),
            GFP_KERNEL | __GFP_ZERO,
        )?;
        let mut loginit = create_coherent_dma_object::<u8>(dev, "LOGINIT", 0x10000, &mut libos, 0)?;
        create_pte_array(&mut loginit, 1);
        let mut logintr = create_coherent_dma_object::<u8>(dev, "LOGINTR", 0x10000, &mut libos, 1)?;
        create_pte_array(&mut logintr, 1);
        let mut logrm = create_coherent_dma_object::<u8>(dev, "LOGRM", 0x10000, &mut libos, 2)?;
        create_pte_array(&mut logrm, 1);
        let wpr_meta = build_wpr_meta(dev, fw, fb_layout)?;

        // Creates its own PTE array
        let mut cmdq = GspCmdq::new(dev)?;
        let rmargs = create_coherent_dma_object::<fw::GSP_ARGUMENTS_CACHED>(
            dev, "RMARGS", 1, &mut libos, 3,
        )?;
        let (shared_mem_phys_addr, cmd_queue_offset, stat_queue_offset) = cmdq.get_cmdq_offsets();

        dma_write!(
            rmargs[0].messageQueueInitArguments = fw::MESSAGE_QUEUE_INIT_ARGUMENTS {
                sharedMemPhysAddr: shared_mem_phys_addr,
                pageTableEntryCount: cmdq.nr_ptes,
                cmdQueueOffset: cmd_queue_offset,
                statQueueOffset: stat_queue_offset,
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

        set_system_info(&mut cmdq, pdev, bar)?;
        build_registry(&mut cmdq, bar)?;

        Ok(GspMemObjects {
            libos,
            loginit,
            logintr,
            logrm,
            rmargs,
            wpr_meta,
            cmdq,
        })
    }

    pub(crate) fn libos_dma_handle(&self) -> bindings::dma_addr_t {
        self.libos.dma_handle()
    }
}
