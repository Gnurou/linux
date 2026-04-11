// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! FSP (Firmware System Processor) interface for Hopper/Blackwell GPUs.
//!
//! Hopper/Blackwell use a simplified firmware boot sequence: FMC --> FSP --> GSP.
//! Unlike Turing/Ampere/Ada, there is NO SEC2 (Security Engine 2) usage.
//! FSP handles secure boot directly using FMC firmware + Chain of Trust.

use kernel::{
    device,
    dma::Coherent,
    io::poll::read_poll_timeout,
    prelude::*,
    ptr::{
        Alignable,
        Alignment, //
    },
    sizes::SZ_2M,
    time::Delta,
    transmute::{
        AsBytes,
        FromBytes, //
    },
};

use crate::{
    fb::FbLayout,
    firmware::fsp::{
        FmcSignatures,
        FspFirmware, //
    },
    gpu::Chipset,
    gsp::GspFmcBootParams,
    mctp::{
        MctpHeader,
        NvdmHeader,
        NvdmType, //
    },
    num,
    regs, //
};

/// FSP Chain of Trust protocol version.
///
/// Hopper (GH100) uses version 1, Blackwell uses version 2.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FspCotVersion(u16);

impl FspCotVersion {
    /// Create a new FSP COT version.
    pub(crate) const fn new(version: u16) -> Self {
        Self(version)
    }

    /// Return the raw protocol version number for the wire format.
    pub(crate) const fn raw(self) -> u16 {
        self.0
    }
}

/// FSP message timeout in milliseconds.
const FSP_MSG_TIMEOUT_MS: i64 = 2000;

/// FSP Command Response payload structure.
/// NVDM_PAYLOAD_COMMAND_RESPONSE structure.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct NvdmPayloadCommandResponse {
    task_id: u32,
    command_nvdm_type: u32,
    error_code: u32,
}

/// NVDM (NVIDIA Device Management) COT (Chain of Trust) payload structure.
/// This is the main message payload sent to FSP for Chain of Trust.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct NvdmPayloadCot {
    version: u16,
    size: u16,
    gsp_fmc_sysmem_offset: u64,
    frts_sysmem_offset: u64,
    frts_sysmem_size: u32,
    frts_vidmem_offset: u64,
    frts_vidmem_size: u32,
    sigs: FmcSignatures,
    gsp_boot_args_sysmem_offset: u64,
}

/// Complete FSP message structure with MCTP and NVDM headers.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct FspMessage {
    mctp_header: u32,
    nvdm_header: u32,
    cot: NvdmPayloadCot,
}

// SAFETY: FspMessage is a packed C struct with only integral fields.
unsafe impl AsBytes for FspMessage {}
/// Complete FSP response structure with MCTP and NVDM headers.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct FspResponse {
    mctp_header: MctpHeader,
    nvdm_header: NvdmHeader,
    response: NvdmPayloadCommandResponse,
}

// SAFETY: FspResponse is a packed C struct with only integral fields.
unsafe impl FromBytes for FspResponse {}

/// Trait implemented by types representing a message to send to FSP.
///
/// This provides [`Fsp::send_sync_fsp`] with the information it needs to send
/// a given message, following the same pattern as GSP's `CommandToGsp`.
pub(crate) trait MessageToFsp: AsBytes {
    /// NVDM type identifying this message to FSP.
    const NVDM_TYPE: u32;
}

impl MessageToFsp for FspMessage {
    const NVDM_TYPE: u32 = NvdmType::Cot as u32;
}

/// Bundled arguments for FMC boot via FSP Chain of Trust.
pub(crate) struct FmcBootArgs<'a> {
    chipset: crate::gpu::Chipset,
    fsp_fw: &'a FspFirmware,
    fmc_boot_params: Coherent<GspFmcBootParams>,
    resume: bool,
}

impl<'a> FmcBootArgs<'a> {
    /// Build FMC boot arguments, allocating the DMA-coherent boot parameter
    /// structure that FSP will read.
    pub(crate) fn new(
        dev: &device::Device<device::Bound>,
        chipset: crate::gpu::Chipset,
        fsp_fw: &'a FspFirmware,
        wpr_meta_addr: u64,
        libos_addr: u64,
        resume: bool,
    ) -> Result<Self> {
        let init = GspFmcBootParams::new(wpr_meta_addr, libos_addr);

        Ok(Self {
            chipset,
            fsp_fw,
            fmc_boot_params: Coherent::<GspFmcBootParams>::init(dev, GFP_KERNEL, init)?,
            resume,
        })
    }

    /// DMA address of the FMC boot parameters, needed after boot for lockdown
    /// release polling.
    pub(crate) fn boot_params_dma_handle(&self) -> u64 {
        self.fmc_boot_params.dma_handle()
    }
}
/// FSP interface for Hopper/Blackwell GPUs.
pub(crate) struct Fsp;

mod hal;

impl Fsp {
    /// Wait for FSP secure boot completion.
    ///
    /// Polls the thermal scratch register until FSP signals boot completion
    /// or timeout occurs.
    pub(crate) fn wait_secure_boot(
        dev: &device::Device,
        bar: &crate::driver::Bar0,
        chipset: Chipset,
    ) -> Result {
        /// FSP secure boot completion timeout in milliseconds.
        const FSP_SECURE_BOOT_TIMEOUT_MS: i64 = 5000;

        let hal = hal::fsp_hal(chipset).ok_or(ENOTSUPP)?;

        read_poll_timeout(
            || Ok(hal.fsp_boot_status(bar)),
            |&status| status == regs::NV_THERM_I2CS_SCRATCH_FSP_BOOT_COMPLETE_STATUS_SUCCESS,
            Delta::from_millis(10),
            Delta::from_millis(FSP_SECURE_BOOT_TIMEOUT_MS),
        )
        .map_err(|_| {
            dev_err!(dev, "FSP secure boot completion timeout\n");
            ETIMEDOUT
        })
        .map(|_| ())
    }

    /// Boot GSP FMC via FSP Chain of Trust.
    ///
    /// Builds the COT message from the pre-configured [`FmcBootArgs`], sends it
    /// to FSP, and waits for the response.
    pub(crate) fn boot_fmc(
        dev: &device::Device<device::Bound>,
        bar: &crate::driver::Bar0,
        fb_layout: &FbLayout,
        fsp_falcon: &crate::falcon::Falcon<crate::falcon::fsp::Fsp>,
        args: &FmcBootArgs<'_>,
    ) -> Result {
        dev_dbg!(dev, "Starting FSP boot sequence for {}\n", args.chipset);

        let fmc_addr = args.fsp_fw.fmc_image.dma_handle();
        let fmc_boot_params_addr = args.fmc_boot_params.dma_handle();

        // frts_offset is relative to FB end: FRTS_location = FB_END - frts_offset
        let frts_offset = if !args.resume {
            let frts_reserved_size = fb_layout.heap.len() + u64::from(fb_layout.pmu_reserved_size);

            frts_reserved_size
                .align_up(Alignment::new::<SZ_2M>())
                .ok_or(EINVAL)?
        } else {
            0
        };
        let frts_size: u32 = if !args.resume {
            fb_layout.frts.len().try_into()?
        } else {
            0
        };

        let msg = KBox::new(
            FspMessage {
                mctp_header: MctpHeader::single_packet().into(),
                nvdm_header: NvdmHeader::new(NvdmType::Cot).into(),

                cot: NvdmPayloadCot {
                    version: args.chipset.fsp_cot_version().ok_or(ENOTSUPP)?.raw(),
                    size: u16::try_from(core::mem::size_of::<NvdmPayloadCot>())
                        .map_err(|_| EINVAL)?,
                    gsp_fmc_sysmem_offset: fmc_addr,
                    frts_sysmem_offset: 0,
                    frts_sysmem_size: 0,
                    frts_vidmem_offset: frts_offset,
                    frts_vidmem_size: frts_size,
                    sigs: *args.fsp_fw.fmc_sigs,
                    gsp_boot_args_sysmem_offset: fmc_boot_params_addr,
                },
            },
            GFP_KERNEL,
        )?;

        Self::send_sync_fsp(dev, bar, fsp_falcon, &*msg)?;

        dev_dbg!(dev, "FSP Chain of Trust completed successfully\n");
        Ok(())
    }

    /// Send message to FSP and wait for response.
    fn send_sync_fsp<M>(
        dev: &device::Device,
        bar: &crate::driver::Bar0,
        fsp_falcon: &crate::falcon::Falcon<crate::falcon::fsp::Fsp>,
        msg: &M,
    ) -> Result
    where
        M: MessageToFsp,
    {
        fsp_falcon.send_msg(bar, msg.as_bytes())?;

        let packet_size = read_poll_timeout(
            || Ok(fsp_falcon.poll_msgq(bar)),
            |&size| size > 0,
            Delta::from_millis(10),
            Delta::from_millis(FSP_MSG_TIMEOUT_MS),
        )
        .map_err(|_| {
            dev_err!(dev, "FSP response timeout\n");
            ETIMEDOUT
        })?;

        let packet_size = num::u32_as_usize(packet_size);
        let mut response_buf = KVec::<u8>::new();
        response_buf.resize(packet_size, 0, GFP_KERNEL)?;
        fsp_falcon.recv_msg(bar, &mut response_buf, packet_size)?;

        if response_buf.len() < core::mem::size_of::<FspResponse>() {
            dev_err!(dev, "FSP response too small: {}\n", response_buf.len());
            return Err(EIO);
        }

        let response = FspResponse::from_bytes(&response_buf[..]).ok_or(EIO)?;

        let mctp_header: MctpHeader = response.mctp_header;
        let nvdm_header: NvdmHeader = response.nvdm_header;
        let command_nvdm_type = response.response.command_nvdm_type;
        let error_code = response.response.error_code;

        if !mctp_header.is_single_packet() {
            dev_err!(
                dev,
                "Unexpected MCTP header in FSP reply: {:x?}\n",
                mctp_header,
            );
            return Err(EIO);
        }

        if !nvdm_header.validate(NvdmType::FspResponse) {
            dev_err!(
                dev,
                "Unexpected NVDM header in FSP reply: {:x?}\n",
                nvdm_header,
            );
            return Err(EIO);
        }

        if command_nvdm_type != M::NVDM_TYPE {
            dev_err!(
                dev,
                "Expected NVDM type {:#x} in reply, got {:#x}\n",
                M::NVDM_TYPE,
                command_nvdm_type
            );
            return Err(EIO);
        }

        if error_code != 0 {
            dev_err!(
                dev,
                "NVDM command {:#x} failed with error {:#x}\n",
                M::NVDM_TYPE,
                error_code
            );
            return Err(EIO);
        }

        Ok(())
    }
}
