// SPDX-License-Identifier: GPL-2.0

mod boot;
mod fw;

pub(crate) mod cmdq;
pub(crate) mod commands;

use kernel::alloc::flags::GFP_KERNEL;
use kernel::bindings;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::dma_write;
use kernel::pci;
use kernel::prelude::*;
use kernel::ptr::Alignment;
use kernel::transmute::{AsBytes, FromBytes};

use crate::driver::Bar0;
use crate::gsp::cmdq::GspCmdq;
use crate::gsp::commands::{build_registry, set_system_info};

use fw::GspArgumentsCached;
use fw::LibosMemoryRegionInitArgument;

pub(crate) use fw::{GspFwWprMeta, LibosParams};

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_ALIGNMENT: Alignment = Alignment::new::<{ 1 << 20 }>();

/// GSP runtime data.
///
/// This is an empty pinned placeholder for now.
#[pin_data]
#[allow(unused)]
pub(crate) struct Gsp {
    libos: CoherentAllocation<LibosMemoryRegionInitArgument>,
    pub loginit: CoherentAllocation<u8>,
    pub logintr: CoherentAllocation<u8>,
    pub logrm: CoherentAllocation<u8>,
    pub cmdq: GspCmdq,
    rmargs: CoherentAllocation<GspArgumentsCached>,
}

/// Creates a self-mapping page table for `obj` at its beginning.
fn create_pte_array<T: AsBytes + FromBytes>(obj: &mut CoherentAllocation<T>, skip: usize) {
    let num_pages = obj.size().div_ceil(GSP_PAGE_SIZE);
    let handle = obj.dma_handle();

    // SAFETY:
    //  - By the invariants of the CoherentAllocation ptr is non-NULL.
    //  - CoherentAllocation CPU addresses are always aligned to a
    //    page-boundary, satisfying the alignment requirements for
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
    libos: &mut CoherentAllocation<LibosMemoryRegionInitArgument>,
    libos_arg_nr: usize,
) -> Result<CoherentAllocation<A>> {
    let obj = CoherentAllocation::<A>::alloc_coherent(dev, size, GFP_KERNEL | __GFP_ZERO)?;

    dma_write!(libos[libos_arg_nr] = LibosMemoryRegionInitArgument::new(name, &obj))?;

    Ok(obj)
}

impl Gsp {
    pub(crate) fn new(
        pdev: &pci::Device<device::Bound>,
        bar: &Bar0,
    ) -> Result<impl PinInit<Self, Error>> {
        let dev = pdev.as_ref();
        let mut libos = CoherentAllocation::<LibosMemoryRegionInitArgument>::alloc_coherent(
            dev,
            GSP_PAGE_SIZE / size_of::<LibosMemoryRegionInitArgument>(),
            GFP_KERNEL | __GFP_ZERO,
        )?;
        let mut loginit = create_coherent_dma_object::<u8>(dev, "LOGINIT", 0x10000, &mut libos, 0)?;
        create_pte_array(&mut loginit, 1);
        let mut logintr = create_coherent_dma_object::<u8>(dev, "LOGINTR", 0x10000, &mut libos, 1)?;
        create_pte_array(&mut logintr, 1);
        let mut logrm = create_coherent_dma_object::<u8>(dev, "LOGRM", 0x10000, &mut libos, 2)?;
        create_pte_array(&mut logrm, 1);

        // Creates its own PTE array
        let mut cmdq = GspCmdq::new(dev)?;
        let rmargs =
            create_coherent_dma_object::<GspArgumentsCached>(dev, "RMARGS", 1, &mut libos, 3)?;

        dma_write!(rmargs[0] = GspArgumentsCached::new(&cmdq))?;

        set_system_info(&mut cmdq, pdev, bar)?;
        build_registry(&mut cmdq, bar)?;

        Ok(try_pin_init!(Self {
            libos,
            loginit,
            logintr,
            logrm,
            rmargs,
            cmdq,
        }))
    }

    #[expect(unused)]
    pub(crate) fn libos_dma_handle(&self) -> bindings::dma_addr_t {
        self.libos.dma_handle()
    }
}
