// SPDX-License-Identifier: GPL-2.0

//! Virtual Memory Manager for NVIDIA GPU page table management.
//!
//! The [`Vmm`] provides high-level page mapping and unmapping operations for GPU
//! virtual address spaces (Channels, BAR1, BAR2). It wraps the page table walker
//! and handles TLB flushing after modifications.

#![allow(dead_code)]

use kernel::{
    gpu::buddy::{
        AllocatedBlocks,
        BuddyFlags,
        GpuBuddy,
        GpuBuddyAllocParams,
        GpuBuddyParams, //
    },
    prelude::*,
    sizes::SZ_4K, //
};

use core::ops::Range;

use crate::mm::{
    pagetable::{
        walk::{PtWalk, WalkResult},
        MmuVersion, //
    },
    GpuMm,
    Pfn,
    Vfn,
    VramAddress,
    PAGE_SIZE, //
};

/// Virtual Memory Manager for a GPU address space.
///
/// Each [`Vmm`] instance manages a single address space identified by its Page
/// Directory Base (`PDB`) address. The [`Vmm`] is used for Channel, BAR1 and
/// BAR2 mappings.
pub(crate) struct Vmm {
    pub(crate) pdb_addr: VramAddress,
    pub(crate) mmu_version: MmuVersion,
    /// Page table allocations required for mappings.
    page_table_allocs: KVec<Pin<KBox<AllocatedBlocks>>>,
    /// Buddy allocator for virtual address range tracking.
    virt_buddy: GpuBuddy,
}

impl Vmm {
    /// Create a new [`Vmm`] for the given Page Directory Base address.
    /// The [`Vmm`] will manage a virtual address space of `va_size` bytes.
    pub(crate) fn new(
        pdb_addr: VramAddress,
        mmu_version: MmuVersion,
        va_size: u64,
    ) -> Result<Self> {
        // Only MMU v2 is supported for now.
        if mmu_version != MmuVersion::V2 {
            return Err(ENOTSUPP);
        }

        let virt_buddy = GpuBuddy::new(GpuBuddyParams {
            base_offset_bytes: 0,
            physical_memory_size_bytes: va_size,
            chunk_size_bytes: SZ_4K as u64,
        })?;

        Ok(Self {
            pdb_addr,
            mmu_version,
            page_table_allocs: KVec::new(),
            virt_buddy,
        })
    }

    /// Allocate a contiguous virtual frame number range.
    ///
    /// # Arguments
    /// - `num_pages`: Number of pages to allocate.
    /// - `va_range`: `None` = allocate anywhere,
    ///               `Some(range)` = constrain allocation to the given range.
    pub(crate) fn alloc_vfn_range(
        &self,
        num_pages: usize,
        va_range: Option<Range<u64>>,
    ) -> Result<(Vfn, Pin<KBox<AllocatedBlocks>>)> {
        let size_bytes = (num_pages as u64)
            .checked_mul(PAGE_SIZE as u64)
            .ok_or(EOVERFLOW)?;

        let (start, end) = match va_range {
            Some(r) => {
                let range_size = r.end.checked_sub(r.start).ok_or(EOVERFLOW)?;
                if range_size != size_bytes {
                    return Err(EINVAL);
                }
                (r.start, r.end)
            }
            None => (0, 0),
        };

        let params = GpuBuddyAllocParams {
            start_range_address: start,
            end_range_address: end,
            size_bytes,
            min_block_size_bytes: SZ_4K as u64,
            buddy_flags: BuddyFlags::try_new(BuddyFlags::CONTIGUOUS_ALLOCATION)?,
        };

        let alloc = KBox::pin_init(self.virt_buddy.alloc_blocks(&params), GFP_KERNEL)?;

        // Get the starting offset of the first block (only block as range is contiguous).
        let offset = alloc.iter().next().ok_or(ENOMEM)?.offset();
        let vfn = Vfn::new(offset / PAGE_SIZE as u64);

        Ok((vfn, alloc))
    }

    /// Read the PFN for a mapped VFN if one is mapped.
    pub(crate) fn read_mapping(&self, mm: &GpuMm, vfn: Vfn) -> Result<Option<Pfn>> {
        let walker = PtWalk::new(self.pdb_addr, self.mmu_version);

        match walker.walk_to_pte_lookup(mm, vfn)? {
            WalkResult::Mapped { pfn, .. } => Ok(Some(pfn)),
            WalkResult::Unmapped { .. } | WalkResult::PageTableMissing => Ok(None),
        }
    }
}
