// SPDX-License-Identifier: GPL-2.0

//! Page table walker implementation for NVIDIA GPUs.
//!
//! This module provides page table walking functionality for MMU v2 and v3.
//! The walker traverses the page table hierarchy to resolve virtual addresses
//! to physical addresses or to find PTE locations.
//!
//! # Page Table Hierarchy
//!
//! ## MMU v2 (Turing/Ampere/Ada) - 5 levels
//!
//! ```text
//!     +-------+     +-------+     +-------+     +---------+     +-------+
//!     | PDB   |---->|  L1   |---->|  L2   |---->| L3 Dual |---->|  L4   |
//!     | (L0)  |     |       |     |       |     | PDE     |     | (PTE) |
//!     +-------+     +-------+     +-------+     +---------+     +-------+
//!       64-bit        64-bit        64-bit        128-bit         64-bit
//!        PDE           PDE           PDE        (big+small)        PTE
//! ```
//!
//! ## MMU v3 (Hopper+) - 6 levels
//!
//! ```text
//!     +-------+     +-------+     +-------+     +-------+     +---------+     +-------+
//!     | PDB   |---->|  L1   |---->|  L2   |---->|  L3   |---->| L4 Dual |---->|  L5   |
//!     | (L0)  |     |       |     |       |     |       |     | PDE     |     | (PTE) |
//!     +-------+     +-------+     +-------+     +-------+     +---------+     +-------+
//!       64-bit        64-bit        64-bit        64-bit        128-bit         64-bit
//!        PDE           PDE           PDE           PDE        (big+small)        PTE
//! ```
//!
//! # Result of a page table walk
//!
//! The walker returns a [`WalkResult`] indicating the outcome.

use kernel::prelude::*;

use super::{
    DualPde,
    MmuVersion,
    PageTableLevel,
    Pde,
    Pte, //
};
use crate::mm::{
    pramin,
    GpuMm,
    Pfn,
    Vfn,
    VirtualAddress,
    VramAddress, //
};

/// Result of walking to a PTE.
#[derive(Debug, Clone, Copy)]
pub(crate) enum WalkResult {
    /// Intermediate page tables are missing (only returned in lookup mode).
    PageTableMissing,
    /// PTE exists but is invalid (page not mapped).
    Unmapped { pte_addr: VramAddress },
    /// PTE exists and is valid (page is mapped).
    Mapped { pte_addr: VramAddress, pfn: Pfn },
}

/// Result of walking PDE levels only.
///
/// Returned by [`PtWalk::walk_pde_levels()`] to indicate whether all PDE levels
/// resolved or a PDE is missing.
#[derive(Debug, Clone, Copy)]
pub(crate) enum WalkPdeResult {
    /// All PDE levels resolved -- returns PTE page table address.
    Complete {
        /// VRAM address of the PTE-level page table.
        pte_table: VramAddress,
    },
    /// A PDE is missing and no prepared page was provided by the closure.
    Missing {
        /// PDE slot address in the parent page table (where to install).
        install_addr: VramAddress,
        /// The page table level that is missing.
        level: PageTableLevel,
    },
}

/// Page table walker for NVIDIA GPUs.
///
/// Walks the page table hierarchy (5 levels for v2, 6 for v3) to find PTE
/// locations or resolve virtual addresses.
pub(crate) struct PtWalk {
    pdb_addr: VramAddress,
    mmu_version: MmuVersion,
}

impl PtWalk {
    /// Calculate the VRAM address of an entry within a page table.
    fn entry_addr(
        table: VramAddress,
        mmu_version: MmuVersion,
        level: PageTableLevel,
        index: u64,
    ) -> VramAddress {
        let entry_size = mmu_version.entry_size(level) as u64;
        VramAddress::new(table.raw_u64() + index * entry_size)
    }

    /// Create a new page table walker.
    pub(crate) fn new(pdb_addr: VramAddress, mmu_version: MmuVersion) -> Self {
        Self {
            pdb_addr,
            mmu_version,
        }
    }

    /// Walk PDE levels with closure-based resolution for missing PDEs.
    ///
    /// Traverses all PDE levels for the MMU version. At each level, reads the PDE.
    /// If valid, extracts the child table address and continues. If missing, calls
    /// `resolve_prepared(install_addr)` to resolve the missing PDE.
    pub(crate) fn walk_pde_levels(
        &self,
        window: &mut pramin::PraminWindow<'_>,
        vfn: Vfn,
        resolve_prepared: impl Fn(VramAddress) -> Option<VramAddress>,
    ) -> Result<WalkPdeResult> {
        let va = VirtualAddress::from(vfn);
        let mut cur_table = self.pdb_addr;

        for &level in self.mmu_version.pde_levels() {
            let idx = va.level_index(level.as_index());
            let install_addr = Self::entry_addr(cur_table, self.mmu_version, level, idx);

            if level == self.mmu_version.dual_pde_level() {
                // 128-bit dual PDE with big+small page table pointers.
                let dpde = DualPde::read(window, install_addr, self.mmu_version)?;
                if dpde.has_small() {
                    cur_table = dpde.small_vram_address();
                    continue;
                }
            } else {
                // Regular 64-bit PDE.
                let pde = Pde::read(window, install_addr, self.mmu_version)?;
                if pde.is_valid() {
                    cur_table = pde.table_vram_address();
                    continue;
                }
            }

            // PDE missing in HW. Ask caller for resolution.
            if let Some(prepared_addr) = resolve_prepared(install_addr) {
                cur_table = prepared_addr;
                continue;
            }

            return Ok(WalkPdeResult::Missing {
                install_addr,
                level,
            });
        }

        Ok(WalkPdeResult::Complete {
            pte_table: cur_table,
        })
    }

    /// Walk to PTE for lookup only (no allocation).
    ///
    /// Returns [`WalkResult::PageTableMissing`] if intermediate tables don't exist.
    pub(crate) fn walk_to_pte_lookup(&self, mm: &GpuMm, vfn: Vfn) -> Result<WalkResult> {
        let mut window = mm.pramin().window()?;
        self.walk_to_pte_lookup_with_window(&mut window, vfn)
    }

    /// Walk to PTE using a caller-provided PRAMIN window (lookup only).
    ///
    /// Uses [`PtWalk::walk_pde_levels()`] for the PDE traversal, then reads the PTE at
    /// the leaf level. Useful when called for multiple VFNs with single PRAMIN window
    /// acquisition. Used by [`Vmm::execute_map()`] and [`Vmm::unmap_pages()`].
    pub(crate) fn walk_to_pte_lookup_with_window(
        &self,
        window: &mut pramin::PraminWindow<'_>,
        vfn: Vfn,
    ) -> Result<WalkResult> {
        match self.walk_pde_levels(window, vfn, |_| None)? {
            WalkPdeResult::Complete { pte_table } => {
                Self::read_pte_at_level(window, vfn, pte_table, self.mmu_version)
            }
            WalkPdeResult::Missing { .. } => Ok(WalkResult::PageTableMissing),
        }
    }

    /// Read the PTE at the PTE level given the PTE table address.
    fn read_pte_at_level(
        window: &mut pramin::PraminWindow<'_>,
        vfn: Vfn,
        pte_table: VramAddress,
        mmu_version: MmuVersion,
    ) -> Result<WalkResult> {
        let va = VirtualAddress::from(vfn);
        let pte_level = mmu_version.pte_level();
        let pte_idx = va.level_index(pte_level.as_index());
        let pte_addr = Self::entry_addr(pte_table, mmu_version, pte_level, pte_idx);
        let pte = Pte::read(window, pte_addr, mmu_version)?;

        if pte.is_valid() {
            return Ok(WalkResult::Mapped {
                pte_addr,
                pfn: pte.frame_number(),
            });
        }
        Ok(WalkResult::Unmapped { pte_addr })
    }
}
