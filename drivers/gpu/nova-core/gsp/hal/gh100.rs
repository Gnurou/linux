// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

use kernel::prelude::*;

use kernel::{
    device,
    dma::Coherent,
    io::{
        poll::read_poll_timeout,
        register::WithBase,
        Io, //
    },
    time::Delta,
};

use crate::{
    driver::Bar0,
    falcon::{
        fsp::Fsp as FspEngine,
        gsp::Gsp as GspEngine,
        sec2::Sec2,
        Falcon, //
    },
    fb::FbLayout,
    firmware::{
        fsp::FspFirmware,
        FIRMWARE_VERSION, //
    },
    fsp::{
        FmcBootArgs,
        Fsp, //
    },
    gpu::Chipset,
    gsp::{
        boot::BootUnloadGuard,
        hal::GspHal,
        Gsp,
        GspFwWprMeta, //
    },
    regs,
};

/// GSP lockdown pattern written by firmware to mbox0 while RISC-V branch privilege
/// lockdown is active. The low byte varies, the upper 24 bits are fixed.
const GSP_LOCKDOWN_PATTERN: u32 = 0xbadf4100;
const GSP_LOCKDOWN_MASK: u32 = 0xffffff00;

/// GSP falcon mailbox state, used to track lockdown release status.
struct GspMbox {
    mbox0: u32,
    mbox1: u32,
}

impl GspMbox {
    /// Read both mailboxes from the GSP falcon.
    fn read(gsp_falcon: &Falcon<GspEngine>, bar: &Bar0) -> Self {
        Self {
            mbox0: gsp_falcon.read_mailbox0(bar),
            mbox1: gsp_falcon.read_mailbox1(bar),
        }
    }

    /// Returns true if the lockdown pattern is present in mbox0.
    fn is_locked_down(&self) -> bool {
        self.mbox0 != 0 && (self.mbox0 & GSP_LOCKDOWN_MASK) == GSP_LOCKDOWN_PATTERN
    }

    /// Combines mailbox0 and mailbox1 into a 64-bit address.
    fn combined_addr(&self) -> u64 {
        (u64::from(self.mbox1) << 32) | u64::from(self.mbox0)
    }

    /// Returns true if GSP lockdown has been released.
    ///
    /// Checks the lockdown pattern, validates the boot params address,
    /// and verifies the HWCFG2 lockdown bit is clear.
    fn lockdown_released(&self, bar: &Bar0, fmc_boot_params_addr: u64) -> bool {
        if self.is_locked_down() {
            return false;
        }

        if self.mbox0 != 0 && self.combined_addr() != fmc_boot_params_addr {
            return true;
        }

        let hwcfg2 = bar.read(regs::NV_PFALCON_FALCON_HWCFG2::of::<GspEngine>());
        !hwcfg2.riscv_br_priv_lockdown()
    }
}

/// Wait for GSP lockdown to be released after FSP Chain of Trust.
fn wait_for_gsp_lockdown_release(
    dev: &device::Device<device::Bound>,
    bar: &Bar0,
    gsp_falcon: &Falcon<GspEngine>,
    fmc_boot_params_addr: u64,
) -> Result {
    dev_dbg!(dev, "Waiting for GSP lockdown release\n");

    let mbox = read_poll_timeout(
        || Ok(GspMbox::read(gsp_falcon, bar)),
        |mbox| mbox.lockdown_released(bar, fmc_boot_params_addr),
        Delta::from_millis(10),
        Delta::from_secs(30),
    )
    .inspect_err(|_| {
        dev_err!(dev, "GSP lockdown release timeout\n");
    })?;

    if mbox.mbox0 != 0 {
        dev_err!(dev, "GSP-FMC boot failed (mbox: {:#x})\n", mbox.mbox0);
        return Err(EIO);
    }

    dev_dbg!(dev, "GSP lockdown released\n");
    Ok(())
}

struct Gh100;

impl GspHal for Gh100 {
    /// Boot GSP via FSP Chain of Trust (Hopper/Blackwell+ path).
    ///
    /// This path uses FSP to establish a chain of trust and boot GSP-FMC. FSP handles
    /// the GSP boot internally - no manual GSP reset/boot is needed.
    fn boot<'a>(
        &self,
        gsp: &'a Gsp,
        dev: &'a device::Device<device::Bound>,
        bar: &'a Bar0,
        chipset: Chipset,
        fb_layout: &FbLayout,
        wpr_meta: &Coherent<GspFwWprMeta>,
        gsp_falcon: &'a Falcon<GspEngine>,
        _sec2_falcon: &'a Falcon<Sec2>,
    ) -> Result<BootUnloadGuard<'a>> {
        let fsp_falcon = Falcon::<FspEngine>::new(dev, chipset)?;
        let fsp_fw = FspFirmware::new(dev, chipset, FIRMWARE_VERSION)?;

        Fsp::wait_secure_boot(dev, bar, chipset)?;

        let args = FmcBootArgs::new(
            dev,
            chipset,
            &fsp_fw,
            wpr_meta.dma_handle(),
            gsp.libos.dma_handle(),
            false,
        )?;

        Fsp::boot_fmc(dev, bar, fb_layout, &fsp_falcon, &args)?;

        let fmc_boot_params_addr = args.boot_params_dma_handle();
        wait_for_gsp_lockdown_release(dev, bar, gsp_falcon, fmc_boot_params_addr)?;

        Err(ENOTSUPP)
    }
}

const GH100: Gh100 = Gh100;
pub(super) const GH100_HAL: &dyn GspHal = &GH100;
