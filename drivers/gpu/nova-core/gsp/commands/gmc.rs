// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! GMC (GPU Management Controller) API commands.

use core::convert::Infallible;

use kernel::{
    device,
    pci,
    prelude::*,
    transmute::AsBytes, //
};

use crate::{
    gpu::Chipset,
    gsp::{
        cmdq::{
            gmc::{
                CommandToGsp,
                MessageFromGsp, //
            },
            NoReply, //
        },
        commands::GspStaticInfo,
        fw::{
            self,
            commands::{
                GspInitRequest,
                GspInitResponseSchema, //
            },
            GmcCommandId, //
        },
        nvkv::{
            Decoder,
            Encodable,
            Encoder,
            Schema,
            UnknownKeyPolicy, //
        },
    },
    sbuffer::SBufferIter,
    vgpu::VgpuState, //
};

/// The `GSP_INIT` command, which carries the system information and registry keys to GSP-RM.
///
/// GSP-RM answers it with the static GPU configuration once it has finished starting.
pub(crate) struct GspInit {
    /// The NVKV-encoded request.
    payload: KVVec<u64>,
}

impl GspInit {
    /// Builds the request for `pdev`.
    ///
    /// # Errors
    ///
    /// - `ENOMEM` if the request or the encoder buffer cannot be allocated.
    pub(crate) fn new(
        pdev: &pci::Device<device::Bound>,
        chipset: Chipset,
        vgpu_state: VgpuState,
    ) -> Result<Self> {
        let mut encoder = Encoder::new();
        GspInitRequest::new(pdev, chipset, vgpu_state)?.encode(&mut encoder)?;

        Ok(Self {
            payload: encoder.finish(),
        })
    }

    /// Returns the request as the bytes that the GSP reads.
    fn payload_bytes(&self) -> &[u8] {
        AsBytes::as_bytes(self.payload.as_slice())
    }
}

impl CommandToGsp for GspInit {
    const COMMAND: GmcCommandId = GmcCommandId::GspInit;
    /// Largest reply that the driver accepts, as in OpenRM's `kgspSendInitRpcs`.
    const MAX_REPLY_SIZE: u32 = 48 * 1024;
    type Command = ();
    type Reply = GspStaticInfo;
    type InitError = Infallible;

    fn init(&self) -> impl Init<Self::Command, Self::InitError> {
        pin_init::init_zeroed()
    }

    fn variable_payload_len(&self) -> usize {
        self.payload_bytes().len()
    }

    fn init_variable_payload(
        &self,
        dst: &mut SBufferIter<core::array::IntoIter<&mut [u8], 2>>,
    ) -> Result {
        dst.write_all(self.payload_bytes())
    }
}

pub(crate) use fw::commands::PowerStateLevel;

/// The `GSP_SUSPEND` command, which suspends GSP-RM.
///
/// GSP-RM sends no reply on the message queue. It reports the completed suspend in the GSP
/// falcon's `MAILBOX0` register instead.
pub(crate) struct GspSuspend {
    level: PowerStateLevel,
}

impl GspSuspend {
    /// Creates a `GSP_SUSPEND` command for the given [`PowerStateLevel`].
    pub(crate) fn new(level: PowerStateLevel) -> Self {
        Self { level }
    }
}

impl CommandToGsp for GspSuspend {
    const COMMAND: GmcCommandId = GmcCommandId::GspSuspend;
    type Command = fw::commands::GspSuspend;
    type Reply = NoReply;
    const MAX_REPLY_SIZE: u32 = 0;
    type InitError = Infallible;

    fn init(&self) -> impl Init<Self::Command, Self::InitError> {
        fw::commands::GspSuspend::new(self.level)
    }
}

/// Copies `src`, which GSP-RM sends with its NUL terminator, to the start of `dst`.
///
/// # Errors
///
/// - `EINVAL` if `src` is longer than `dst`.
fn copy_name<const N: usize>(dst: &mut [u8; N], src: &[u8]) -> Result {
    dst.get_mut(..src.len()).ok_or(EINVAL)?.copy_from_slice(src);

    Ok(())
}

impl MessageFromGsp for GspStaticInfo {
    const COMMAND: GmcCommandId = GmcCommandId::GspInit;
    type InitError = Error;

    /// Decodes the `GSP_INIT` reply, an NVKV stream, into the static configuration.
    ///
    /// # Errors
    ///
    /// - `EINVAL` if the payload is not a whole number of NVKV words, if the stream is malformed
    ///   or omits a required key, or if GSP-RM reported no framebuffer region.
    /// - `ENOMEM` if the words or the decoded regions cannot be allocated.
    fn read(payload: &mut SBufferIter<core::array::IntoIter<&[u8], 2>>) -> Result<Self> {
        let bytes = payload.flush_into_kvec(GFP_KERNEL)?;

        let chunks = bytes.chunks_exact(size_of::<u64>());
        if !chunks.remainder().is_empty() {
            return Err(EINVAL);
        }
        let mut words = KVVec::with_capacity(chunks.len(), GFP_KERNEL)?;
        for chunk in chunks {
            // `chunks_exact` yields slices of exactly `size_of::<u64>()` bytes.
            let word = chunk.try_into().map_err(|_| EINVAL)?;
            words.push(u64::from_le_bytes(word), GFP_KERNEL)?;
        }

        let decoder = Decoder::new(&words, UnknownKeyPolicy::Ignore);
        let mut schema = KBox::init(GspInitResponseSchema::init(), GFP_KERNEL)?;
        let reply = KBox::try_init(decoder.decode(&mut *schema)?, GFP_KERNEL)?;

        let mut gpu_name = [0u8; GspStaticInfo::MAX_GPU_NAME_LEN];
        copy_name(&mut gpu_name, reply.gpu_name())?;
        let mut gpu_short_name = [0u8; GspStaticInfo::MAX_GPU_NAME_LEN];
        copy_name(&mut gpu_short_name, reply.gpu_short_name())?;
        let mut gpu_gid = [0u8; GspStaticInfo::GID_LEN];
        copy_name(&mut gpu_gid, reply.gpu_gid())?;

        let mut usable_fb_regions = KVec::new();
        for region in reply.usable_fb_regions() {
            usable_fb_regions.push(region, GFP_KERNEL)?;
        }

        Ok(GspStaticInfo {
            gpu_name,
            gpu_short_name,
            gpu_gid,
            bar1_pde_base: reply.bar1_pde_base(),
            usable_fb_regions,
            total_fb_end: reply.total_fb_end().ok_or(EINVAL)?,
        })
    }
}
