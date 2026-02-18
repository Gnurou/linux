// SPDX-License-Identifier: GPL-2.0

//! Common page table types shared between MMU v2 and v3.
//!
//! This module provides foundational types used by both MMU versions:
//! - Page table level hierarchy
//! - Memory aperture types for PDEs and PTEs

#![expect(dead_code)]
pub(crate) mod ver2;
pub(crate) mod ver3;
pub(crate) mod walk;

use kernel::prelude::*;

use super::{
    pramin,
    Pfn,
    VramAddress,
    PAGE_SIZE, //
};
use crate::gpu::Architecture;

/// MMU version enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MmuVersion {
    /// MMU v2 for Turing/Ampere/Ada.
    V2,
    /// MMU v3 for Hopper and later.
    V3,
}

impl From<Architecture> for MmuVersion {
    fn from(arch: Architecture) -> Self {
        match arch {
            Architecture::Turing | Architecture::Ampere | Architecture::Ada => Self::V2,
            // In the future, uncomment:
            // _ => Self::V3,
        }
    }
}

/// Page Table Level hierarchy for MMU v2/v3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PageTableLevel {
    /// Level 0 - Page Directory Base (root).
    Pdb,
    /// Level 1 - Intermediate page directory.
    L1,
    /// Level 2 - Intermediate page directory.
    L2,
    /// Level 3 - Intermediate page directory or dual PDE (version-dependent).
    L3,
    /// Level 4 - PTE level for v2, intermediate page directory for v3.
    L4,
    /// Level 5 - PTE level used for MMU v3 only.
    L5,
}

impl PageTableLevel {
    /// Number of entries per page table (512 for 4KB pages).
    pub(crate) const ENTRIES_PER_TABLE: usize = 512;

    /// Get the next level in the hierarchy.
    pub(crate) const fn next(&self) -> Option<PageTableLevel> {
        match self {
            Self::Pdb => Some(Self::L1),
            Self::L1 => Some(Self::L2),
            Self::L2 => Some(Self::L3),
            Self::L3 => Some(Self::L4),
            Self::L4 => Some(Self::L5),
            Self::L5 => None,
        }
    }

    /// Convert level to index.
    pub(crate) const fn as_index(&self) -> u64 {
        match self {
            Self::Pdb => 0,
            Self::L1 => 1,
            Self::L2 => 2,
            Self::L3 => 3,
            Self::L4 => 4,
            Self::L5 => 5,
        }
    }
}

impl MmuVersion {
    /// Get the `PDE` levels (excluding PTE level) for page table walking.
    pub(crate) fn pde_levels(&self) -> &'static [PageTableLevel] {
        match self {
            Self::V2 => ver2::PDE_LEVELS,
            Self::V3 => ver3::PDE_LEVELS,
        }
    }

    /// Get the PTE level for this MMU version.
    pub(crate) fn pte_level(&self) -> PageTableLevel {
        match self {
            Self::V2 => ver2::PTE_LEVEL,
            Self::V3 => ver3::PTE_LEVEL,
        }
    }

    /// Get the dual PDE level (128-bit entries) for this MMU version.
    pub(crate) fn dual_pde_level(&self) -> PageTableLevel {
        match self {
            Self::V2 => ver2::DUAL_PDE_LEVEL,
            Self::V3 => ver3::DUAL_PDE_LEVEL,
        }
    }

    /// Get the number of PDE levels for this MMU version.
    pub(crate) fn pde_level_count(&self) -> usize {
        self.pde_levels().len()
    }

    /// Get the entry size in bytes for a given level.
    pub(crate) fn entry_size(&self, level: PageTableLevel) -> usize {
        if level == self.dual_pde_level() {
            16 // 128-bit dual PDE
        } else {
            8 // 64-bit PDE/PTE
        }
    }

    /// Get the number of entries per page table page for a given level.
    pub(crate) fn entries_per_page(&self, level: PageTableLevel) -> usize {
        PAGE_SIZE / self.entry_size(level)
    }

    /// Compute upper bound on page table pages needed for `num_virt_pages`.
    ///
    /// Walks from PTE level up through PDE levels, accumulating the tree.
    pub(crate) fn pt_pages_upper_bound(&self, num_virt_pages: usize) -> usize {
        let mut total = 0;

        // PTE pages at the leaf level.
        let pte_epp = self.entries_per_page(self.pte_level());
        // Ceiling of (num_virt_pages / entries_per_page).
        let mut pages_at_level = (num_virt_pages + pte_epp - 1) / pte_epp;
        total += pages_at_level;

        // Walk PDE levels bottom-up (reverse of pde_levels()).
        for &level in self.pde_levels().iter().rev() {
            let epp = self.entries_per_page(level);
            // How many pages at this level do we need to point to
            // the previous pages_at_level?
            // Calculated as ceiling of (pages_at_level / entries_per_page).
            pages_at_level = (pages_at_level + epp - 1) / epp;
            total += pages_at_level;
        }

        total
    }
}

/// Memory aperture for Page Table Entries (`PTE`s).
///
/// Determines which memory region the `PTE` points to.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AperturePte {
    /// Local video memory (VRAM).
    #[default]
    VideoMemory = 0,
    /// Peer GPU's video memory.
    PeerMemory = 1,
    /// System memory with cache coherence.
    SystemCoherent = 2,
    /// System memory without cache coherence.
    SystemNonCoherent = 3,
}

// TODO[FPRI]: Replace with `#[derive(FromPrimitive)]` when available.
impl From<u8> for AperturePte {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::VideoMemory,
            1 => Self::PeerMemory,
            2 => Self::SystemCoherent,
            3 => Self::SystemNonCoherent,
            _ => Self::VideoMemory,
        }
    }
}

// TODO[FPRI]: Replace with `#[derive(ToPrimitive)]` when available.
impl From<AperturePte> for u8 {
    fn from(val: AperturePte) -> Self {
        val as u8
    }
}

/// Memory aperture for Page Directory Entries (`PDE`s).
///
/// Note: For `PDE`s, `Invalid` (0) means the entry is not valid.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AperturePde {
    /// Invalid/unused entry.
    #[default]
    Invalid = 0,
    /// Page table is in video memory.
    VideoMemory = 1,
    /// Page table is in system memory with coherence.
    SystemCoherent = 2,
    /// Page table is in system memory without coherence.
    SystemNonCoherent = 3,
}

// TODO[FPRI]: Replace with `#[derive(FromPrimitive)]` when available.
impl From<u8> for AperturePde {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::VideoMemory,
            2 => Self::SystemCoherent,
            3 => Self::SystemNonCoherent,
            _ => Self::Invalid,
        }
    }
}

// TODO[FPRI]: Replace with `#[derive(ToPrimitive)]` when available.
impl From<AperturePde> for u8 {
    fn from(val: AperturePde) -> Self {
        val as u8
    }
}

/// Unified Page Table Entry wrapper for both MMU v2 and v3 `PTE`
/// types, allowing the walker to work with either format.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Pte {
    /// MMU v2 `PTE` (Turing/Ampere/Ada).
    V2(ver2::Pte),
    /// MMU v3 `PTE` (Hopper+).
    V3(ver3::Pte),
}

impl Pte {
    /// Create a `PTE` from a raw `u64` value for the given MMU version.
    pub(crate) fn new(version: MmuVersion, val: u64) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pte::new(val)),
            MmuVersion::V3 => Self::V3(ver3::Pte::new(val)),
        }
    }

    /// Create an invalid `PTE` for the given MMU version.
    pub(crate) fn invalid(version: MmuVersion) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pte::invalid()),
            MmuVersion::V3 => Self::V3(ver3::Pte::invalid()),
        }
    }

    /// Create a valid `PTE` for video memory.
    pub(crate) fn new_vram(version: MmuVersion, pfn: Pfn, writable: bool) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pte::new_vram(pfn, writable)),
            MmuVersion::V3 => Self::V3(ver3::Pte::new_vram(pfn, writable)),
        }
    }

    /// Check if this `PTE` is valid.
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Self::V2(p) => p.valid(),
            Self::V3(p) => p.valid(),
        }
    }

    /// Get the physical frame number.
    pub(crate) fn frame_number(&self) -> Pfn {
        match self {
            Self::V2(p) => p.frame_number(),
            Self::V3(p) => p.frame_number(),
        }
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        match self {
            Self::V2(p) => p.raw_u64(),
            Self::V3(p) => p.raw_u64(),
        }
    }

    /// Read a `PTE` from VRAM.
    pub(crate) fn read(
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
        mmu_version: MmuVersion,
    ) -> Result<Self> {
        let val = window.try_read64(addr.raw())?;
        Ok(Self::new(mmu_version, val))
    }

    /// Write this `PTE` to VRAM.
    pub(crate) fn write(
        &self,
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
    ) -> Result {
        window.try_write64(addr.raw(), self.raw_u64())
    }
}

impl Default for Pte {
    fn default() -> Self {
        Self::V2(ver2::Pte::default())
    }
}

/// Unified Page Directory Entry wrapper for both MMU v2 and v3 `PDE`.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Pde {
    /// MMU v2 `PDE` (Turing/Ampere/Ada).
    V2(ver2::Pde),
    /// MMU v3 `PDE` (Hopper+).
    V3(ver3::Pde),
}

impl Pde {
    /// Create a `PDE` from a raw `u64` value for the given MMU version.
    pub(crate) fn new(version: MmuVersion, val: u64) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pde::new(val)),
            MmuVersion::V3 => Self::V3(ver3::Pde::new(val)),
        }
    }

    /// Create a valid `PDE` pointing to a page table in video memory.
    pub(crate) fn new_vram(version: MmuVersion, table_pfn: Pfn) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pde::new_vram(table_pfn)),
            MmuVersion::V3 => Self::V3(ver3::Pde::new_vram(table_pfn)),
        }
    }

    /// Create an invalid `PDE` for the given MMU version.
    pub(crate) fn invalid(version: MmuVersion) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::Pde::invalid()),
            MmuVersion::V3 => Self::V3(ver3::Pde::invalid()),
        }
    }

    /// Check if this `PDE` is valid.
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Self::V2(p) => p.is_valid(),
            Self::V3(p) => p.is_valid(),
        }
    }

    /// Get the VRAM address of the page table.
    pub(crate) fn table_vram_address(&self) -> VramAddress {
        match self {
            Self::V2(p) => p.table_vram_address(),
            Self::V3(p) => p.table_vram_address(),
        }
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        match self {
            Self::V2(p) => p.raw_u64(),
            Self::V3(p) => p.raw_u64(),
        }
    }

    /// Read a `PDE` from VRAM.
    pub(crate) fn read(
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
        mmu_version: MmuVersion,
    ) -> Result<Self> {
        let val = window.try_read64(addr.raw())?;
        Ok(Self::new(mmu_version, val))
    }

    /// Write this `PDE` to VRAM.
    pub(crate) fn write(
        &self,
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
    ) -> Result {
        window.try_write64(addr.raw(), self.raw_u64())
    }
}

impl Default for Pde {
    fn default() -> Self {
        Self::V2(ver2::Pde::default())
    }
}

/// Unified Dual Page Directory Entry wrapper for both MMU v2 and v3 [`DualPde`].
#[derive(Debug, Clone, Copy)]
pub(crate) enum DualPde {
    /// MMU v2 [`DualPde`] (Turing/Ampere/Ada).
    V2(ver2::DualPde),
    /// MMU v3 [`DualPde`] (Hopper+).
    V3(ver3::DualPde),
}

impl DualPde {
    /// Create a [`DualPde`] from raw 128-bit value (two `u64`s) for the given MMU version.
    pub(crate) fn new(version: MmuVersion, big: u64, small: u64) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::DualPde::new(big, small)),
            MmuVersion::V3 => Self::V3(ver3::DualPde::new(big, small)),
        }
    }

    /// Create a [`DualPde`] with only the small page table pointer set.
    pub(crate) fn new_small(version: MmuVersion, table_pfn: Pfn) -> Self {
        match version {
            MmuVersion::V2 => Self::V2(ver2::DualPde::new_small(table_pfn)),
            MmuVersion::V3 => Self::V3(ver3::DualPde::new_small(table_pfn)),
        }
    }

    /// Check if the small page table pointer is valid.
    pub(crate) fn has_small(&self) -> bool {
        match self {
            Self::V2(d) => d.has_small(),
            Self::V3(d) => d.has_small(),
        }
    }

    /// Get the small page table VRAM address.
    pub(crate) fn small_vram_address(&self) -> VramAddress {
        match self {
            Self::V2(d) => d.small.table_vram_address(),
            Self::V3(d) => d.small.table_vram_address(),
        }
    }

    /// Get the raw `u64` value of the big PDE.
    pub(crate) fn big_raw_u64(&self) -> u64 {
        match self {
            Self::V2(d) => d.big.raw_u64(),
            Self::V3(d) => d.big.raw_u64(),
        }
    }

    /// Get the raw `u64` value of the small PDE.
    pub(crate) fn small_raw_u64(&self) -> u64 {
        match self {
            Self::V2(d) => d.small.raw_u64(),
            Self::V3(d) => d.small.raw_u64(),
        }
    }

    /// Read a dual PDE (128-bit) from VRAM.
    pub(crate) fn read(
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
        mmu_version: MmuVersion,
    ) -> Result<Self> {
        let lo = window.try_read64(addr.raw())?;
        let hi = window.try_read64(addr.raw() + 8)?;
        Ok(Self::new(mmu_version, lo, hi))
    }

    /// Write this dual PDE (128-bit) to VRAM.
    pub(crate) fn write(
        &self,
        window: &mut pramin::PraminWindow<'_>,
        addr: VramAddress,
    ) -> Result {
        window.try_write64(addr.raw(), self.big_raw_u64())?;
        window.try_write64(addr.raw() + 8, self.small_raw_u64())
    }
}
