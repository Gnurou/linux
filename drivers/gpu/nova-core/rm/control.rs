// SPDX-License-Identifier: GPL-2.0
//
// RM Control implementation for nova-core
// RM control commands are used to query and configure various GPU resources.

use super::common::{RmApi, RmCommand, RmHeader, RmMessage, RmResponseElement};
use crate::gsp::{GspCommand, GspCommandElement, GspMessageElement, GspStaticConfigInfo};
use crate::nvfw::r570_144 as fw;
use crate::sbuffer::SBuffer;
use kernel::prelude::*;
use kernel::transmute::AsBytes;

/// Wrapper for RM Control commands
pub(crate) struct RmControlCmd<'a>(pub(crate) RmMessage<'a, RmControlHeader>);

impl<'a> GspCommandElement for RmControlCmd<'a> {
    fn copy_to_sbuf<'b, I: Iterator<Item = &'b mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        self.0.copy_to_sbuf(sbuf)
    }

    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<'a> GspCommand for RmControlCmd<'a> {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL;
}

impl<'a> RmCommand<'a> for RmControlCmd<'a> {
    type Header = RmControlHeader;

    fn new(header: RmControlHeader, params: &'a [u8]) -> Self {
        Self(RmMessage { header, params })
    }
}

pub(crate) trait RmControl: AsBytes {
    // The control code corresponding to this parameter.
    const CONTROL: u32;

    // The expected response type.
    type Response: RmResponseElement;
}

/// RM Control header structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RmControlHeader {
    h_client: u32,    // IN
    h_object: u32,    // IN
    cmd: u32,         // IN
    status: u32,      // OUT
    params_size: u32, // IN
    flags: u32,       // IN
}

impl RmControlHeader {
    /// Creates a new control header with the specified object handle and command
    pub(crate) fn new<C: RmControl>(gsp_info: &GspStaticConfigInfo, control: &C) -> Self {
        Self {
            h_client: gsp_info.h_internal_client,
            h_object: gsp_info.h_internal_subdevice,
            cmd: C::CONTROL,
            status: 0,
            params_size: control.as_bytes().len() as u32,
            flags: 0,
        }
    }
}

/// Extensions specific to RM Control operations
impl<'a> RmApi<'a> {
    /// Send an RM control command with automatic object handle setup
    pub(crate) fn send_control<C: RmControl>(&mut self, params: &'a C) -> Result<C::Response> {
        let header = RmControlHeader::new(self.gsp_info(), params);
        self.send::<RmControlCmd<'a>, _>(header, params.as_bytes())
    }
}

impl GspMessageElement for RmControlHeader {
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self> {
        let mut bytes = [0u8; core::mem::size_of::<RmControlHeader>()];
        sbuf.read_exact(&mut bytes)?;

        // SAFETY: RmControlHeader is repr(C, packed) and we're reading
        // exactly size_of::<RmControlHeader>() bytes
        unsafe {
            let header_ptr = bytes.as_ptr() as *const RmControlHeader;
            Ok(*header_ptr)
        }
    }
}

impl RmHeader for RmControlHeader {
    fn get_status(&self) -> u32 {
        self.status
    }
}
