// SPDX-License-Identifier: GPL-2.0
//
// RM alloc/free operation implementation

use super::common::{RmCommand, RmHeader, RmMessage};
use crate::gsp::{GspCommand, GspCommandElement, GspMessageElement};
use crate::nvfw::r570_144 as fw;
use crate::sbuffer::SBuffer;
use kernel::prelude::*;

/// Wrapper for RM Alloc commands
pub(crate) struct RmAllocCmd<'a, H: RmHeader>(pub(crate) RmMessage<'a, H>);

impl<'a, H: RmHeader> GspCommandElement for RmAllocCmd<'a, H> {
    fn copy_to_sbuf<'b, I: Iterator<Item = &'b mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        self.0.copy_to_sbuf(sbuf)
    }

    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<'a, H: RmHeader> GspCommand for RmAllocCmd<'a, H> {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC;
}

/// Alloc command wrapper implementation
#[allow(dead_code)]
pub(crate) struct AllocCommandWrapper;

impl<H: RmHeader> RmCommand<H> for AllocCommandWrapper {
    type Command<'a>
        = RmAllocCmd<'a, H>
    where
        H: 'a;

    fn from_message<'a>(msg: RmMessage<'a, H>) -> Self::Command<'a>
    where
        H: 'a,
    {
        RmAllocCmd(msg)
    }
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

impl RmAllocHeader {
    /// Create a new alloc header
    #[expect(dead_code)]
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

impl GspMessageElement for RmAllocHeader {
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self> {
        let mut bytes = [0u8; core::mem::size_of::<RmAllocHeader>()];
        sbuf.read_exact(&mut bytes)?;

        // SAFETY: RmAllocHeader is repr(C, packed) and we're reading
        // exactly size_of::<RmAllocHeader>() bytes
        unsafe {
            let header_ptr = bytes.as_ptr() as *const RmAllocHeader;
            Ok(*header_ptr)
        }
    }
}

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

    fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    fn get_status(&self) -> u32 {
        self.status
    }
}
