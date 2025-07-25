// SPDX-License-Identifier: GPL-2.0
//
// RM Alloc implementation for nova-core
// Handles RM allocation operations using the generic RM API infrastructure.

use super::common::{RmApi, RmHeader};
use crate::gsp::{GspCmdq, GspStaticConfigInfo};
use crate::nvfw::r570_144 as fw;
use kernel::{device, prelude::*};

/// RM Alloc header structure (32 bytes total)
/// Wire format for RM allocation operations
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub(crate) struct RmAllocHeader {
    /// Client handle
    pub(crate) h_client: u32,
    /// Parent object handle
    pub(crate) h_parent: u32,
    /// Object handle to allocate
    pub(crate) h_object: u32,
    /// Class ID to allocate
    pub(crate) h_class: u32,
    /// Operation status
    pub(crate) status: u32,
    /// Size of parameters
    pub(crate) params_size: u32,
    /// Flags
    pub(crate) flags: u32,
    /// Padding for 32-byte alignment
    _padding: u32,
}

// SAFETY: This struct is used in FFI with the GSP firmware
unsafe impl Zeroable for RmAllocHeader {}

impl GspMessageElement for RmAllocHeader {}

impl RmHeader for RmAllocHeader {
    fn set_client(&mut self, client: u32) {
        self.h_client = client;
    }

    fn set_status(&mut self, status: u32) {
        self.status = status;
    }

    fn set_params_size(&mut self, size: u32) {
        self.params_size = size;
    }

    fn get_status(&self) -> u32 {
        self.status
    }

    fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }
}

impl RmAllocHeader {
    /// Create a new RM alloc header
    #[allow(dead_code)]
    pub(crate) fn new(h_parent: u32, h_object: u32, h_class: u32) -> Self {
        Self {
            h_client: 0, // Will be set by RmApi
            h_parent,
            h_object,
            h_class,
            status: 0,      // Will be set by RmApi
            params_size: 0, // Will be set by RmApi
            flags: 0,       // Will be set by RmApi
            _padding: 0,
        }
    }
}

/// Type alias for RM Alloc API
#[allow(dead_code)]
pub(crate) type RmAlloc<'a> = RmApi<'a, RmAllocHeader>;

/// Extensions specific to RM Alloc operations
impl<'a> RmAlloc<'a> {
    /// Create new RM alloc instance
    #[allow(dead_code)]
    pub(crate) fn new_alloc(
        cmdq: &'a mut GspCmdq<'a>,
        gsp_info: &'a GspStaticConfigInfo,
        dev: &'a device::Device<device::Bound>,
    ) -> Self {
        // TODO: Verify the correct RPC function number for RM alloc
        // This is likely NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC
        Self::new(
            cmdq,
            bar,
            gsp_info,
            dev,
            fw::NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC,
        )
    }

    /// Send an RM alloc command
    #[allow(dead_code)]
    pub(crate) fn send_alloc<P: super::common::RmParams, T: super::common::RmResponseElement>(
        &mut self,
        h_parent: u32,
        h_object: u32,
        h_class: u32,
        params: Option<&P>,
    ) -> Result<T> {
        let header = RmAllocHeader::new(h_parent, h_object, h_class);
        self.send(header, params)
    }
}
