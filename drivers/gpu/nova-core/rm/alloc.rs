// SPDX-License-Identifier: GPL-2.0
//
// RM alloc/free operation implementation

use super::{RmCommand, RmHeader, RmMessage};
use crate::gsp::GspCommand;
use crate::nvfw::r570_144 as fw;
use kernel::transmute::FromBytesSized;

/// Wrapper for RM Alloc commands
#[allow(dead_code)]
pub(crate) type RmAllocCmd<'a> = RmMessage<'a, RmAllocHeader>;

impl<'a> GspCommand for RmAllocCmd<'a> {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC;
}

impl<'a> RmCommand<'a> for RmAllocCmd<'a> {
    type Header = RmAllocHeader;
}

/// RM Alloc header structure (32 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RmAllocHeader {
    /// Client handle
    h_client: u32,
    /// Parent object handle
    h_parent: u32,
    /// Object handle to allocate
    h_object: u32,
    /// Class ID to allocate
    h_class: u32,
    /// Operation status
    status: u32,
    /// Size of parameters
    params_size: u32,
    /// Flags
    flags: u32,
    /// Padding for 32-byte alignment
    _padding: u32,
}
unsafe impl FromBytesSized for RmAllocHeader {}

impl RmAllocHeader {
    /// Create a new alloc header
    #[expect(dead_code)]
    pub(crate) fn new(
        h_client: u32,
        h_parent: u32,
        h_object: u32,
        h_class: u32,
        params_size: u32,
    ) -> Self {
        Self {
            h_client,
            h_parent,
            h_object,
            h_class,
            status: 0,
            params_size,
            flags: 0,
            _padding: 0,
        }
    }
}

impl RmHeader for RmAllocHeader {
    fn get_status(&self) -> u32 {
        self.status
    }
}
