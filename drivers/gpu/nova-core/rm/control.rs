// SPDX-License-Identifier: GPL-2.0
//
// RM Control implementation for nova-core
// RM control commands are used to query and configure various GPU resources.

use super::common::{RmApi, RmHeader, RmParams, RmResponseElement};
use crate::gsp::{GspCmdq, GspStaticConfigInfo};
use crate::nvfw::r570_144 as fw;
use kernel::device;
use kernel::prelude::*;

/// RM Control RPC header structure
#[repr(C)]
#[derive(Debug)]
pub(crate) struct RmControlHeader {
    h_client: u32,
    h_object: u32,
    cmd: u32,
    status: u32,
    params_size: u32,
    flags: u32,
}

impl GspMessageElement for RmControlHeader {}

impl RmHeader for RmControlHeader {
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

impl RmControlHeader {
    /// Create a new RM control header with the specified command and object
    pub(crate) fn new(cmd: u32, h_object: u32) -> Self {
        Self {
            h_client: 0, // Will be set by RmApi
            h_object,
            cmd,
            status: 0,      // Will be set by RmApi
            params_size: 0, // Will be set by RmApi
            flags: 0,       // Will be set by RmApi
        }
    }
}

/// Type alias for RM Control API
pub(crate) type RmControl<'a> = RmApi<'a, RmControlHeader>;

/// Extensions specific to RM Control operations
impl<'a> RmControl<'a> {
    /// Create new RM control instance
    pub(crate) fn new_control(
        cmdq: &'a mut GspCmdq<'a>,
        gsp_info: &'a GspStaticConfigInfo,
        dev: &'a device::Device<device::Bound>,
    ) -> Self {
        Self::new(
            cmdq,
            bar,
            gsp_info,
            dev,
            fw::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL,
        )
    }

    /// Send an RM control command with automatic object handle setup
    pub(crate) fn send_control<P: RmParams, T: RmResponseElement>(
        &mut self,
        cmd: u32,
        params: Option<&P>,
    ) -> Result<T> {
        let header = RmControlHeader::new(cmd, self.subdevice_handle());
        self.send(header, params)
    }
}
