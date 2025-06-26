// SPDX-License-Identifier: GPL-2.0
use kernel::bindings;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::dma_write;
use kernel::pci;
use kernel::prelude::*;
use kernel::ptr::Alignment;
use kernel::transmute::{AsBytes, FromBytes};

use crate::nvfw::r570_144 as fw;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_ALIGNMENT: Alignment = Alignment::new(1 << 20);

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for fw::LibosMemoryRegionInitArgument {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for fw::LibosMemoryRegionInitArgument {}

#[allow(unused)]
pub(crate) struct GspMemObjects {
    libos: CoherentAllocation<fw::LibosMemoryRegionInitArgument>,
    pub loginit: CoherentAllocation<u8>,
    pub logintr: CoherentAllocation<u8>,
    pub logrm: CoherentAllocation<u8>,
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
fn create_pte_array(obj: &mut CoherentAllocation<u8>) {
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
        let ptr = obj.start_ptr_mut().cast::<u64>().add(1);
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
            ..Default::default()
        }
    )?;

    Ok(obj)
}

impl GspMemObjects {
    pub(crate) fn new(pdev: &pci::Device<device::Bound>) -> Result<Self> {
        let dev = pdev.as_ref();
        let mut libos = CoherentAllocation::<fw::LibosMemoryRegionInitArgument>::alloc_coherent(
            dev,
            GSP_PAGE_SIZE / size_of::<fw::LibosMemoryRegionInitArgument>(),
            GFP_KERNEL | __GFP_ZERO,
        )?;
        let mut loginit = create_coherent_dma_object::<u8>(dev, "LOGINIT", 0x10000, &mut libos, 0)?;
        create_pte_array(&mut loginit);
        let mut logintr = create_coherent_dma_object::<u8>(dev, "LOGINTR", 0x10000, &mut libos, 1)?;
        create_pte_array(&mut logintr);
        let mut logrm = create_coherent_dma_object::<u8>(dev, "LOGRM", 0x10000, &mut libos, 2)?;
        create_pte_array(&mut logrm);

        Ok(GspMemObjects {
            libos,
            loginit,
            logintr,
            logrm,
        })
    }

    pub(crate) fn libos_dma_handle(&self) -> bindings::dma_addr_t {
        self.libos.dma_handle()
    }
}
