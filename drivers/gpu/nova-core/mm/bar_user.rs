// SPDX-License-Identifier: GPL-2.0

//! BAR1 user interface for CPU access to GPU virtual memory. Used for USERD
//! for GPU work submission, and applications to access GPU buffers via mmap().

use kernel::io::Io;
use kernel::prelude::*;

use crate::{
    driver::Bar1,
    mm::{
        pagetable::MmuVersion,
        vmm::{
            MappedRange,
            Vmm, //
        },
        GpuMm,
        Pfn,
        Vfn,
        VirtualAddress,
        VramAddress,
        PAGE_SIZE, //
    },
};

/// BAR1 user interface for virtual memory mappings.
///
/// Owns a VMM instance with virtual address tracking and provides
/// BAR1-specific mapping and cleanup operations.
pub(crate) struct BarUser {
    vmm: Vmm,
}

impl BarUser {
    /// Create a new [`BarUser`] with virtual address tracking.
    pub(crate) fn new(
        pdb_addr: VramAddress,
        mmu_version: MmuVersion,
        va_size: u64,
    ) -> Result<Self> {
        Ok(Self {
            vmm: Vmm::new(pdb_addr, mmu_version, va_size)?,
        })
    }

    /// Map physical pages to a contiguous BAR1 virtual range.
    pub(crate) fn map<'a>(
        &'a mut self,
        mm: &'a GpuMm,
        bar: &'a Bar1,
        pfns: &[Pfn],
        writable: bool,
    ) -> Result<BarAccess<'a>> {
        if pfns.is_empty() {
            return Err(EINVAL);
        }

        let mapped = self.vmm.map_pages(mm, pfns, None, writable)?;

        Ok(BarAccess {
            vmm: &mut self.vmm,
            mm,
            bar,
            mapped: Some(mapped),
        })
    }
}

/// Access object for a mapped BAR1 region.
///
/// Wraps a [`MappedRange`] and provides BAR1 access. When dropped,
/// unmaps pages and releases the VA range (by passing the range to
/// [`Vmm::unmap_pages()`], which consumes it).
pub(crate) struct BarAccess<'a> {
    vmm: &'a mut Vmm,
    mm: &'a GpuMm,
    bar: &'a Bar1,
    /// Needs to be an `Option` so that we can `take()` it and call `Drop`
    /// on it in [`Vmm::unmap_pages()`].
    mapped: Option<MappedRange>,
}

impl<'a> BarAccess<'a> {
    /// Returns the active mapping.
    fn mapped(&self) -> &MappedRange {
        // SAFETY: unwrap() will never panic here because `mapped` is only
        // `None` after `take()` in `Drop`, accessors are never called in `Drop`.
        self.mapped.as_ref().unwrap()
    }

    /// Get the base virtual address of this mapping.
    pub(crate) fn base(&self) -> VirtualAddress {
        VirtualAddress::from(self.mapped().vfn_start)
    }

    /// Get the total size of the mapped region in bytes.
    pub(crate) fn size(&self) -> usize {
        self.mapped().num_pages * PAGE_SIZE
    }

    /// Get the starting virtual frame number.
    pub(crate) fn vfn_start(&self) -> Vfn {
        self.mapped().vfn_start
    }

    /// Get the number of pages in this mapping.
    pub(crate) fn num_pages(&self) -> usize {
        self.mapped().num_pages
    }

    /// Translate an offset within this mapping to a BAR1 aperture offset.
    fn bar_offset(&self, offset: usize) -> Result<usize> {
        if offset >= self.size() {
            return Err(EINVAL);
        }

        let base = (self.mapped().vfn_start.raw() as usize)
            .checked_mul(PAGE_SIZE)
            .ok_or(EOVERFLOW)?;
        base.checked_add(offset).ok_or(EOVERFLOW)
    }

    // Fallible accessors with runtime bounds checking.

    /// Read a 32-bit value at the given offset.
    pub(crate) fn try_read32(&self, offset: usize) -> Result<u32> {
        self.bar.try_read32(self.bar_offset(offset)?)
    }

    /// Write a 32-bit value at the given offset.
    pub(crate) fn try_write32(&self, value: u32, offset: usize) -> Result {
        self.bar.try_write32(value, self.bar_offset(offset)?)
    }

    /// Read a 64-bit value at the given offset.
    pub(crate) fn try_read64(&self, offset: usize) -> Result<u64> {
        self.bar.try_read64(self.bar_offset(offset)?)
    }

    /// Write a 64-bit value at the given offset.
    pub(crate) fn try_write64(&self, value: u64, offset: usize) -> Result {
        self.bar.try_write64(value, self.bar_offset(offset)?)
    }
}

impl Drop for BarAccess<'_> {
    fn drop(&mut self) {
        if let Some(mapped) = self.mapped.take() {
            if self.vmm.unmap_pages(self.mm, mapped).is_err() {
                kernel::pr_warn_once!("BarAccess: unmap_pages failed.\n");
            }
        }
    }
}
