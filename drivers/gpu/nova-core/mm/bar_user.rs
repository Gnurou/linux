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

/// Check if the PDB has valid, VRAM-backed page tables.
///
/// Returns `Err(ENOENT)` if page tables are missing or not in VRAM.
#[cfg(CONFIG_NOVA_MM_SELFTESTS)]
fn check_valid_page_tables(mm: &GpuMm, pdb_addr: VramAddress) -> Result {
    use crate::mm::pagetable::ver2::Pde;
    use crate::mm::pagetable::AperturePde;

    let mut window = mm.pramin().window()?;
    let pdb_entry_raw = window.try_read64(pdb_addr.raw())?;
    let pdb_entry = Pde::new(pdb_entry_raw);

    if !pdb_entry.is_valid() {
        return Err(ENOENT);
    }

    if pdb_entry.aperture() != AperturePde::VideoMemory {
        return Err(ENOENT);
    }

    Ok(())
}

/// Run MM subsystem self-tests during probe.
///
/// Tests page table infrastructure and `BAR1` MMIO access using the `BAR1`
/// address space. Uses the `GpuMm`'s buddy allocator / to allocate page tables
/// and test pages as needed.
#[cfg(CONFIG_NOVA_MM_SELFTESTS)]
pub(crate) fn run_self_test(
    dev: &kernel::device::Device,
    mm: &GpuMm,
    bar1: &crate::driver::Bar1,
    bar1_pdb: u64,
    mmu_version: MmuVersion,
) -> Result {
    use crate::mm::vmm::Vmm;
    use crate::mm::PAGE_SIZE;
    use kernel::gpu::buddy::BuddyFlags;
    use kernel::gpu::buddy::GpuBuddyAllocParams;
    use kernel::sizes::{
        SZ_4K,
        SZ_64K, //
    };

    // Self-tests only support MMU v2 for now.
    if mmu_version != MmuVersion::V2 {
        dev_info!(
            dev,
            "MM: Skipping self-tests for MMU {:?} (only V2 supported)\n",
            mmu_version
        );
        return Ok(());
    }

    // Test patterns.
    const PATTERN_PRAMIN: u32 = 0xDEAD_BEEF;
    const PATTERN_BAR1: u32 = 0xCAFE_BABE;

    dev_info!(dev, "MM: Starting self-test...\n");

    let pdb_addr = VramAddress::new(bar1_pdb);

    // Check if initial page tables are in VRAM.
    if check_valid_page_tables(mm, pdb_addr).is_err() {
        dev_info!(dev, "MM: Self-test SKIPPED - no valid VRAM page tables\n");
        return Ok(());
    }

    // Setup a test page from the buddy allocator.
    let alloc_params = GpuBuddyAllocParams {
        start_range_address: 0,
        end_range_address: 0,
        size_bytes: SZ_4K as u64,
        min_block_size_bytes: SZ_4K as u64,
        buddy_flags: BuddyFlags::try_new(0)?,
    };

    let test_page_blocks = KBox::pin_init(mm.buddy().alloc_blocks(&alloc_params), GFP_KERNEL)?;
    let test_vram_offset = test_page_blocks.iter().next().ok_or(ENOMEM)?.offset();
    let test_vram = VramAddress::new(test_vram_offset);
    let test_pfn = Pfn::from(test_vram);

    // Create a VMM of size 64K to track virtual memory mappings.
    let mut vmm = Vmm::new(pdb_addr, MmuVersion::V2, SZ_64K as u64)?;

    // Create a test mapping.
    let mapped = vmm.map_pages(mm, &[test_pfn], None, true)?;
    let test_vfn = mapped.vfn_start;

    // Pre-compute test addresses for each access path.
    // Use distinct offsets within the page for read (0x100) and write (0x200) tests.
    let bar1_base_offset = test_vfn.raw() as usize * PAGE_SIZE;
    let bar1_read_offset: usize = bar1_base_offset + 0x100;
    let bar1_write_offset: usize = bar1_base_offset + 0x200;
    let vram_read_addr: usize = test_vram.raw() + 0x100;
    let vram_write_addr: usize = test_vram.raw() + 0x200;

    // Test 1: Write via PRAMIN, read via BAR1.
    {
        let mut window = mm.pramin().window()?;
        window.try_write32(vram_read_addr, PATTERN_PRAMIN)?;
    }

    // Read back via BAR1 aperture.
    let bar1_value = bar1.try_read32(bar1_read_offset)?;

    let test1_passed = if bar1_value == PATTERN_PRAMIN {
        true
    } else {
        dev_err!(
            dev,
            "MM: Test 1 FAILED - Expected {:#010x}, got {:#010x}\n",
            PATTERN_PRAMIN,
            bar1_value
        );
        false
    };

    // Test 2: Write via BAR1, read via PRAMIN.
    bar1.try_write32(PATTERN_BAR1, bar1_write_offset)?;

    // Read back via PRAMIN.
    let pramin_value = {
        let mut window = mm.pramin().window()?;
        window.try_read32(vram_write_addr)?
    };

    let test2_passed = if pramin_value == PATTERN_BAR1 {
        true
    } else {
        dev_err!(
            dev,
            "MM: Test 2 FAILED - Expected {:#010x}, got {:#010x}\n",
            PATTERN_BAR1,
            pramin_value
        );
        false
    };

    // Cleanup - invalidate PTE.
    vmm.unmap_pages(mm, mapped)?;

    // Test 3: Two-phase prepare/execute API.
    let prepared = vmm.prepare_map(mm, 1, None)?;
    let mapped2 = vmm.execute_map(mm, prepared, &[test_pfn], true)?;
    let readback = vmm.read_mapping(mm, mapped2.vfn_start)?;
    let test3_passed = if readback == Some(test_pfn) {
        true
    } else {
        dev_err!(dev, "MM: Test 3 FAILED - Two-phase map readback mismatch\n");
        false
    };
    vmm.unmap_pages(mm, mapped2)?;

    if test1_passed && test2_passed && test3_passed {
        dev_info!(dev, "MM: All self-tests PASSED\n");
        Ok(())
    } else {
        dev_err!(dev, "MM: Self-tests FAILED\n");
        Err(EIO)
    }
}
