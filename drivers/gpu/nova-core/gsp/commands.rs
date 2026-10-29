// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2025-2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub(crate) mod gmc;

use core::{
    ffi::FromBytesUntilNulError,
    ops::Range,
    str::Utf8Error, //
};

use kernel::prelude::*;

/// The static GPU configuration, which GSP-RM reports in reply to [`GspInit`](gmc::GspInit).
pub struct GspStaticInfo {
    gpu_name: [u8; Self::MAX_GPU_NAME_LEN],
    gpu_short_name: [u8; Self::MAX_GPU_NAME_LEN],
    /// The 16-byte SHA-1 based GPU identifier (GID) reported by GSP-RM.
    pub gpu_gid: [u8; Self::GID_LEN],
    /// BAR1 Page Directory Entry base address.
    pub(crate) bar1_pde_base: u64,
    /// Usable FB (VRAM) regions for driver memory allocation.
    pub(crate) usable_fb_regions: KVec<Range<u64>>,
    /// Exclusive end of the FB physical address space.
    pub(crate) total_fb_end: u64,
}

/// Error type for [`GspStaticInfo::gpu_name`].
#[derive(Debug)]
pub enum GpuNameError {
    /// The GPU name string does not contain a null terminator.
    NoNullTerminator(FromBytesUntilNulError),

    /// The GPU name string contains invalid UTF-8.
    InvalidUtf8(Utf8Error),
}

impl GspStaticInfo {
    /// Length of the buffers holding the GPU names, NUL terminator included.
    pub(crate) const MAX_GPU_NAME_LEN: usize = 64;
    /// Length of the GPU identifier.
    pub(crate) const GID_LEN: usize = 16;

    /// Returns the name of the GPU as a string.
    ///
    /// Returns an error if the string given by the GSP does not contain a null terminator or
    /// contains invalid UTF-8.
    pub fn gpu_name(&self) -> Result<&str, GpuNameError> {
        CStr::from_bytes_until_nul(&self.gpu_name)
            .map_err(GpuNameError::NoNullTerminator)?
            .to_str()
            .map_err(GpuNameError::InvalidUtf8)
    }

    /// Returns the short name of the GPU as a string.
    ///
    /// Returns an error if the string given by the GSP does not contain a null terminator or
    /// contains invalid UTF-8.
    pub fn gpu_short_name(&self) -> core::result::Result<&str, GpuNameError> {
        CStr::from_bytes_until_nul(&self.gpu_short_name)
            .map_err(GpuNameError::NoNullTerminator)?
            .to_str()
            .map_err(GpuNameError::InvalidUtf8)
    }

    /// Returns the total usable VRAM size in bytes, i.e. the summed lengths of all usable FB
    /// regions.
    pub fn vram_size(&self) -> u64 {
        self.usable_fb_regions.iter().fold(0, |size, region| {
            size.saturating_add(region.end - region.start)
        })
    }
}
