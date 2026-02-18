// SPDX-License-Identifier: GPL-2.0

//! MMU v2 page table types for Turing and Ampere GPUs.
//!
//! This module defines MMU version 2 specific types (Turing, Ampere and Ada GPUs).
//!
//! Bit field layouts derived from the NVIDIA OpenRM documentation:
//! `open-gpu-kernel-modules/src/common/inc/swref/published/turing/tu102/dev_mmu.h`

#![expect(dead_code)]

use super::{
    AperturePde,
    AperturePte,
    PageTableLevel, //
};
use crate::mm::{
    Pfn,
    VramAddress, //
};

/// PDE levels for MMU v2 (5-level hierarchy: PDB -> L1 -> L2 -> L3 -> L4).
pub(crate) const PDE_LEVELS: &[PageTableLevel] = &[
    PageTableLevel::Pdb,
    PageTableLevel::L1,
    PageTableLevel::L2,
    PageTableLevel::L3,
];

/// PTE level for MMU v2.
pub(crate) const PTE_LEVEL: PageTableLevel = PageTableLevel::L4;

/// Dual PDE level for MMU v2 (128-bit entries).
pub(crate) const DUAL_PDE_LEVEL: PageTableLevel = PageTableLevel::L3;

// Page Table Entry (PTE) for MMU v2 - 64-bit entry at level 4.
bitfield! {
    pub(crate) struct Pte(u64), "Page Table Entry for MMU v2" {
        0:0     valid               as bool, "Entry is valid";
        2:1     aperture            as u8 => AperturePte, "Memory aperture type";
        3:3     volatile            as bool, "Volatile (bypass L2 cache)";
        4:4     encrypted           as bool, "Encryption enabled (Confidential Computing)";
        5:5     privilege           as bool, "Privileged access only";
        6:6     read_only           as bool, "Write protection";
        7:7     atomic_disable      as bool, "Atomic operations disabled";
        53:8    frame_number_sys    as u64 => Pfn, "Frame number for system memory";
        32:8    frame_number_vid    as u64 => Pfn, "Frame number for video memory";
        35:33   peer_id             as u8, "Peer GPU ID for peer memory (0-7)";
        53:36   comptagline         as u32, "Compression tag line bits";
        63:56   kind                as u8, "Surface kind/format";
    }
}

impl Pte {
    /// Create a PTE from a `u64` value.
    pub(crate) fn new(val: u64) -> Self {
        Self(val)
    }

    /// Create a valid PTE for video memory.
    pub(crate) fn new_vram(pfn: Pfn, writable: bool) -> Self {
        Self::default()
            .set_valid(true)
            .set_aperture(AperturePte::VideoMemory)
            .set_frame_number_vid(pfn)
            .set_read_only(!writable)
    }

    /// Create an invalid PTE.
    pub(crate) fn invalid() -> Self {
        Self::default()
    }

    /// Get the frame number based on aperture type.
    pub(crate) fn frame_number(&self) -> Pfn {
        match self.aperture() {
            AperturePte::VideoMemory => self.frame_number_vid(),
            _ => self.frame_number_sys(),
        }
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        self.0
    }
}

// Page Directory Entry (PDE) for MMU v2 - 64-bit entry at levels 0-2.
bitfield! {
    pub(crate) struct Pde(u64), "Page Directory Entry for MMU v2" {
        0:0     valid_inverted      as bool, "Valid bit (inverted logic)";
        2:1     aperture            as u8 => AperturePde, "Memory aperture type";
        3:3     volatile            as bool, "Volatile (bypass L2 cache)";
        5:5     no_ats              as bool, "Disable Address Translation Services";
        53:8    table_frame_sys     as u64 => Pfn, "Table frame number for system memory";
        32:8    table_frame_vid     as u64 => Pfn, "Table frame number for video memory";
        35:33   peer_id             as u8, "Peer GPU ID (0-7)";
    }
}

impl Pde {
    /// Create a PDE from a `u64` value.
    pub(crate) fn new(val: u64) -> Self {
        Self(val)
    }

    /// Create a valid PDE pointing to a page table in video memory.
    pub(crate) fn new_vram(table_pfn: Pfn) -> Self {
        Self::default()
            .set_valid_inverted(false) // 0 = valid
            .set_aperture(AperturePde::VideoMemory)
            .set_table_frame_vid(table_pfn)
    }

    /// Create an invalid PDE.
    pub(crate) fn invalid() -> Self {
        Self::default()
            .set_valid_inverted(true)
            .set_aperture(AperturePde::Invalid)
    }

    /// Check if this PDE is valid.
    pub(crate) fn is_valid(&self) -> bool {
        !self.valid_inverted() && self.aperture() != AperturePde::Invalid
    }

    /// Get the table frame number based on aperture type.
    pub(crate) fn table_frame(&self) -> Pfn {
        match self.aperture() {
            AperturePde::VideoMemory => self.table_frame_vid(),
            _ => self.table_frame_sys(),
        }
    }

    /// Get the VRAM address of the page table.
    pub(crate) fn table_vram_address(&self) -> VramAddress {
        debug_assert!(
            self.aperture() == AperturePde::VideoMemory,
            "table_vram_address called on non-VRAM PDE (aperture: {:?})",
            self.aperture()
        );
        VramAddress::from(self.table_frame_vid())
    }

    /// Get the raw `u64` value of the PDE.
    pub(crate) fn raw_u64(&self) -> u64 {
        self.0
    }
}

/// Dual PDE at Level 3 - 128-bit entry of Large/Small Page Table pointers.
///
/// The dual PDE supports both large (64KB) and small (4KB) page tables.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct DualPde {
    /// Large/Big Page Table pointer (lower 64 bits).
    pub big: Pde,
    /// Small Page Table pointer (upper 64 bits).
    pub small: Pde,
}

impl DualPde {
    /// Create a dual PDE from raw 128-bit value (two `u64`s).
    pub(crate) fn new(big: u64, small: u64) -> Self {
        Self {
            big: Pde::new(big),
            small: Pde::new(small),
        }
    }

    /// Create a dual PDE with only the small page table pointer set.
    ///
    /// Note: The big (LPT) portion is set to 0, not `Pde::invalid()`.
    /// According to hardware documentation, clearing bit 0 of the 128-bit
    /// entry makes the PDE behave as a "normal" PDE. Using `Pde::invalid()`
    /// would set bit 0 (valid_inverted), which breaks page table walking.
    pub(crate) fn new_small(table_pfn: Pfn) -> Self {
        Self {
            big: Pde::new(0),
            small: Pde::new_vram(table_pfn),
        }
    }

    /// Check if the small page table pointer is valid.
    pub(crate) fn has_small(&self) -> bool {
        self.small.is_valid()
    }

    /// Check if the big page table pointer is valid.
    pub(crate) fn has_big(&self) -> bool {
        self.big.is_valid()
    }

    /// Get the small page table PFN.
    pub(crate) fn small_pfn(&self) -> Pfn {
        self.small.table_frame()
    }
}
