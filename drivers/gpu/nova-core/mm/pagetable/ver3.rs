// SPDX-License-Identifier: GPL-2.0

//! MMU v3 page table types for Hopper and later GPUs.
//!
//! This module defines MMU version 3 specific types (Hopper and later GPUs).
//!
//! Key differences from MMU v2:
//! - Unified 40-bit address field for all apertures (v2 had separate sys/vid fields).
//! - PCF (Page Classification Field) replaces separate privilege/RO/atomic/cache bits.
//! - KIND field is 4 bits (not 8).
//! - IS_PTE bit in PDE to support large pages directly.
//! - No COMPTAGLINE field (compression handled differently in v3).
//! - No separate ENCRYPTED bit.
//!
//! Bit field layouts derived from the NVIDIA OpenRM documentation:
//! `open-gpu-kernel-modules/src/common/inc/swref/published/hopper/gh100/dev_mmu.h`

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
use kernel::prelude::*;

/// PDE levels for MMU v3 (6-level hierarchy).
pub(crate) const PDE_LEVELS: &[PageTableLevel] = &[
    PageTableLevel::Pdb,
    PageTableLevel::L1,
    PageTableLevel::L2,
    PageTableLevel::L3,
    PageTableLevel::L4,
];

/// PTE level for MMU v3.
pub(crate) const PTE_LEVEL: PageTableLevel = PageTableLevel::L5;

/// Dual PDE level for MMU v3 (128-bit entries).
pub(crate) const DUAL_PDE_LEVEL: PageTableLevel = PageTableLevel::L4;

// Page Classification Field (PCF) - 5 bits for PTEs in MMU v3.
bitfield! {
    pub(crate) struct PtePcf(u8), "Page Classification Field for PTEs" {
        0:0     uncached    as bool, "Bypass L2 cache (0=cached, 1=bypass)";
        1:1     acd         as bool, "Access counting disabled (0=enabled, 1=disabled)";
        2:2     read_only   as bool, "Read-only access (0=read-write, 1=read-only)";
        3:3     no_atomic   as bool, "Atomics disabled (0=enabled, 1=disabled)";
        4:4     privileged  as bool, "Privileged access only (0=regular, 1=privileged)";
    }
}

impl PtePcf {
    /// Create PCF for read-write mapping (cached, no atomics, regular mode).
    pub(crate) fn rw() -> Self {
        Self::default().set_no_atomic(true)
    }

    /// Create PCF for read-only mapping (cached, no atomics, regular mode).
    pub(crate) fn ro() -> Self {
        Self::default().set_read_only(true).set_no_atomic(true)
    }

    /// Get the raw `u8` value.
    pub(crate) fn raw_u8(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PtePcf {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

// Page Classification Field (PCF) - 3 bits for PDEs in MMU v3.
// Controls Address Translation Services (ATS) and caching.
bitfield! {
    pub(crate) struct PdePcf(u8), "Page Classification Field for PDEs" {
        0:0     uncached    as bool, "Bypass L2 cache (0=cached, 1=bypass)";
        1:1     no_ats      as bool, "Address Translation Services disabled (0=enabled, 1=disabled)";
    }
}

impl PdePcf {
    /// Create PCF for cached mapping with ATS enabled (default).
    pub(crate) fn cached() -> Self {
        Self::default()
    }

    /// Get the raw `u8` value.
    pub(crate) fn raw_u8(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PdePcf {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

// Page Table Entry (PTE) for MMU v3.
bitfield! {
    pub(crate) struct Pte(u64), "Page Table Entry for MMU v3" {
        0:0     valid           as bool, "Entry is valid";
        2:1     aperture        as u8 => AperturePte, "Memory aperture type";
        7:3     pcf             as u8 => PtePcf, "Page Classification Field";
        11:8    kind            as u8, "Surface kind (4 bits, 0x0=pitch, 0xF=invalid)";
        51:12   frame_number    as u64 => Pfn, "Physical frame number (for all apertures)";
        63:61   peer_id         as u8, "Peer GPU ID for peer memory (0-7)";
    }
}

impl Pte {
    /// Create a PTE from a `u64` value.
    pub(crate) fn new(val: u64) -> Self {
        Self(val)
    }

    /// Create a valid PTE for video memory.
    pub(crate) fn new_vram(frame: Pfn, writable: bool) -> Self {
        let pcf = if writable { PtePcf::rw() } else { PtePcf::ro() };
        Self::default()
            .set_valid(true)
            .set_aperture(AperturePte::VideoMemory)
            .set_pcf(pcf)
            .set_frame_number(frame)
    }

    /// Create an invalid PTE.
    pub(crate) fn invalid() -> Self {
        Self::default()
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        self.0
    }
}

// Page Directory Entry (PDE) for MMU v3.
//
// Note: v3 uses a unified 40-bit address field (v2 had separate sys/vid address fields).
bitfield! {
    pub(crate) struct Pde(u64), "Page Directory Entry for MMU v3 (Hopper+)" {
        0:0     is_pte      as bool, "Entry is a PTE (0=PDE, 1=large page PTE)";
        2:1     aperture    as u8 => AperturePde, "Memory aperture (0=invalid, 1=vidmem, 2=coherent, 3=non-coherent)";
        5:3     pcf         as u8 => PdePcf, "Page Classification Field (3 bits for PDE)";
        51:12   table_frame as u64 => Pfn, "Table frame number (40-bit unified address)";
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
            .set_is_pte(false)
            .set_aperture(AperturePde::VideoMemory)
            .set_table_frame(table_pfn)
    }

    /// Create an invalid PDE.
    pub(crate) fn invalid() -> Self {
        Self::default().set_aperture(AperturePde::Invalid)
    }

    /// Check if this PDE is valid.
    pub(crate) fn is_valid(&self) -> bool {
        self.aperture() != AperturePde::Invalid
    }

    /// Get the VRAM address of the page table.
    pub(crate) fn table_vram_address(&self) -> VramAddress {
        debug_assert!(
            self.aperture() == AperturePde::VideoMemory,
            "table_vram_address called on non-VRAM PDE (aperture: {:?})",
            self.aperture()
        );
        VramAddress::from(self.table_frame())
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        self.0
    }
}

// Big Page Table pointer for Dual PDE - 64-bit lower word of the 128-bit Dual PDE.
bitfield! {
    pub(crate) struct DualPdeBig(u64), "Big Page Table pointer in Dual PDE (MMU v3)" {
        0:0     is_pte      as bool, "Entry is a PTE (for large pages)";
        2:1     aperture    as u8 => AperturePde, "Memory aperture type";
        5:3     pcf         as u8 => PdePcf, "Page Classification Field";
        51:8    table_frame as u64, "Table frame (table address 256-byte aligned)";
    }
}

impl DualPdeBig {
    /// Create a big page table pointer from a `u64` value.
    pub(crate) fn new(val: u64) -> Self {
        Self(val)
    }

    /// Create an invalid big page table pointer.
    pub(crate) fn invalid() -> Self {
        Self::default().set_aperture(AperturePde::Invalid)
    }

    /// Create a valid big PDE pointing to a page table in video memory.
    pub(crate) fn new_vram(table_addr: VramAddress) -> Result<Self> {
        // Big page table addresses must be 256-byte aligned (shift 8).
        if table_addr.raw_u64() & 0xFF != 0 {
            return Err(EINVAL);
        }

        let table_frame = table_addr.raw_u64() >> 8;
        Ok(Self::default()
            .set_is_pte(false)
            .set_aperture(AperturePde::VideoMemory)
            .set_table_frame(table_frame))
    }

    /// Check if this big PDE is valid.
    pub(crate) fn is_valid(&self) -> bool {
        self.aperture() != AperturePde::Invalid
    }

    /// Get the VRAM address of the big page table.
    pub(crate) fn table_vram_address(&self) -> VramAddress {
        debug_assert!(
            self.aperture() == AperturePde::VideoMemory,
            "table_vram_address called on non-VRAM DualPdeBig (aperture: {:?})",
            self.aperture()
        );
        VramAddress::new(self.table_frame() << 8)
    }

    /// Get the raw `u64` value.
    pub(crate) fn raw_u64(&self) -> u64 {
        self.0
    }
}

/// Dual PDE at Level 4 for MMU v3 - 128-bit entry.
///
/// Contains both big (64KB) and small (4KB) page table pointers:
/// - Lower 64 bits: Big Page Table pointer.
/// - Upper 64 bits: Small Page Table pointer.
///
/// ## Note
///
/// The big and small page table pointers have different address layouts:
/// - Big address = field value << 8 (256-byte alignment).
/// - Small address = field value << 12 (4KB alignment).
///
/// This is why `DualPdeBig` is a separate type from `Pde`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct DualPde {
    /// Big Page Table pointer.
    pub big: DualPdeBig,
    /// Small Page Table pointer.
    pub small: Pde,
}

impl DualPde {
    /// Create a dual PDE from raw 128-bit value (two `u64`s).
    pub(crate) fn new(big: u64, small: u64) -> Self {
        Self {
            big: DualPdeBig::new(big),
            small: Pde::new(small),
        }
    }

    /// Create a dual PDE with only the small page table pointer set.
    pub(crate) fn new_small(table_pfn: Pfn) -> Self {
        Self {
            big: DualPdeBig::invalid(),
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
}
