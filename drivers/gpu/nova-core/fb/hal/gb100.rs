// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! Blackwell GB10x framebuffer HAL.
//!
//! GB10x GPUs use HSHUB0 registers for the sysmem flush page. Both the primary and EG (egress)
//! register pairs must be programmed to the same address, as required by hardware.

use kernel::{
    io::Io,
    num::Bounded,
    prelude::*,
    ptr::{
        const_align_up,
        Alignment, //
    },
    sizes::*, //
};

use crate::{
    fb::{
        hal::FbHal,
        Bar0, //
    },
    num::usize_into_u32,
    regs, //
};

struct Gb100;

fn read_sysmem_flush_page_gb100(bar: &Bar0) -> u64 {
    let lo = u64::from(
        bar.read(regs::NV_PFB_HSHUB0_PCIE_FLUSH_SYSMEM_ADDR_LO)
            .adr(),
    );
    let hi = u64::from(
        bar.read(regs::NV_PFB_HSHUB0_PCIE_FLUSH_SYSMEM_ADDR_HI)
            .adr(),
    );

    lo | (hi << 32)
}

fn write_sysmem_flush_page_gb100(bar: &Bar0, addr: Bounded<u64, 52>) {
    // CAST: lower 32 bits. Hardware ignores bits 7:0.
    let addr_lo = *addr as u32;
    let addr_hi = addr.shr::<32, 20>().cast::<u32>();

    // Write HI first. The hardware will trigger the flush on the LO write.

    // Primary HSHUB pair.
    bar.write_reg(regs::NV_PFB_HSHUB0_PCIE_FLUSH_SYSMEM_ADDR_HI::zeroed().with_adr(addr_hi));
    bar.write_reg(regs::NV_PFB_HSHUB0_PCIE_FLUSH_SYSMEM_ADDR_LO::zeroed().with_adr(addr_lo));

    // EG (egress) pair -- must match the primary pair.
    bar.write_reg(regs::NV_PFB_HSHUB0_EG_PCIE_FLUSH_SYSMEM_ADDR_HI::zeroed().with_adr(addr_hi));
    bar.write_reg(regs::NV_PFB_HSHUB0_EG_PCIE_FLUSH_SYSMEM_ADDR_LO::zeroed().with_adr(addr_lo));
}

pub(super) const fn pmu_reserved_size_gb100() -> u32 {
    usize_into_u32::<{ const_align_up(SZ_8M + SZ_16M + SZ_4K, Alignment::new::<SZ_128K>()).unwrap() }>(
    )
}

impl FbHal for Gb100 {
    fn read_sysmem_flush_page(&self, bar: &Bar0) -> u64 {
        read_sysmem_flush_page_gb100(bar)
    }

    fn write_sysmem_flush_page(&self, bar: &Bar0, addr: u64) -> Result {
        let addr: Bounded<u64, 52> = Bounded::<u64, 64>::from(addr)
            .try_shrink::<52>()
            .ok_or(EINVAL)?;

        write_sysmem_flush_page_gb100(bar, addr);

        Ok(())
    }

    fn supports_display(&self, bar: &Bar0) -> bool {
        super::ga100::display_enabled_ga100(bar)
    }

    fn vidmem_size(&self, bar: &Bar0) -> u64 {
        super::ga102::vidmem_size_ga102(bar)
    }

    fn pmu_reserved_size(&self) -> u32 {
        pmu_reserved_size_gb100()
    }

    fn non_wpr_heap_size(&self) -> Option<u32> {
        Some(super::BLACKWELL_NON_WPR_HEAP_SIZE)
    }

    fn frts_size(&self) -> u64 {
        super::tu102::frts_size_tu102()
    }
}

const GB100: Gb100 = Gb100;
pub(super) const GB100_HAL: &dyn FbHal = &GB100;
