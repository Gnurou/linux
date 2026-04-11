// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! FSP (Firmware System Processor) interface for Hopper/Blackwell GPUs.
//!
//! Hopper/Blackwell use a simplified firmware boot sequence: FMC --> FSP --> GSP.
//! Unlike Turing/Ampere/Ada, there is NO SEC2 (Security Engine 2) usage.
//! FSP handles secure boot directly using FMC firmware + Chain of Trust.

use kernel::{
    device,
    io::poll::read_poll_timeout,
    prelude::*,
    time::Delta,
    transmute::{
        AsBytes,
        FromBytes, //
    },
};

use crate::{
    gpu::Chipset,
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
    #[expect(dead_code)]
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

    /// Send message to FSP and wait for response.
    #[expect(dead_code)]
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
