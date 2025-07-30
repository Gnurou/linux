// SPDX-License-Identifier: GPL-2.0
//
// Common RM API implementation for nova-core
// Provides generic infrastructure for RM control and RM alloc operations.

use crate::driver::Bar0;
use crate::gsp::{GspCmdq, GspCommand, GspCommandElement, GspMessageElement};
use crate::sbuffer::SBuffer;
use crate::util::wait_on_result;
use kernel::device;
use kernel::prelude::*;
use kernel::time::Delta;
use kernel::{dev_err, dev_info};

/// Trait for RM headers common to all RM API operations
pub(crate) trait RmHeader: GspMessageElement {
    /// Get the status code of a response
    fn get_status(&self) -> u32;
}

/// Generic response wrapper that holds both header and data. Only the header
/// differs between different RM API operations (e.g. control vs alloc).
/// TODO: Shall we combine this and RmMessage?
pub(crate) struct RmGspResponse<H: RmHeader> {
    pub header: H,
    pub data: KVec<u8>,
}

impl<H: RmHeader> GspMessageElement for RmGspResponse<H> {
    fn new_from_sbuf<'a, I: Iterator<Item = &'a [u8]>>(sbuf: &mut SBuffer<I>) -> Result<Self> {
        // RM API implementation specific: Read RM header
        let header = H::new_from_sbuf(sbuf)?;

        // Read variable-length data after header
        let data = sbuf.read_into_kvec(GFP_KERNEL)?;

        Ok(RmGspResponse { header, data })
    }
}

/// Generic message wrapper for sending (header + optional params)
/// TODO: Shall we combine this and RmGspResponse?
pub(crate) struct RmMessage<'a, H: RmHeader> {
    pub(crate) header: H,
    pub(crate) params: &'a [u8],
}

impl<'a, H: RmHeader> GspCommandElement for RmMessage<'a, H> {
    fn copy_to_sbuf<'b, I: Iterator<Item = &'b mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        // Write the header
        let header_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.header as *const H as *const u8,
                core::mem::size_of::<H>(),
            )
        };
        sbuf.write_all(header_bytes)?;
        sbuf.write_all(self.params)?;

        Ok(())
    }

    fn size(&self) -> usize {
        core::mem::size_of::<H>() + self.params.len()
    }
}

/// Trait for wrapping RmMessage into specific command types
pub(crate) trait RmCommand<'a>: GspCommand {
    type Header: RmHeader;
}

/// Trait for RM response message elements
pub(crate) trait RmResponseElement: Sized {
    /// Parse response from bytes
    fn from_bytes(data: &[u8]) -> Result<Self>;
}

impl GspCmdq {
    /// Send an RM command and get its response.
    pub(crate) fn send_rm_command<'a, CMD: RmCommand<'a>, T: RmResponseElement>(
        &mut self,
        // TODO: we should store an ARef of this in GspCmdq and remove this parameter. This is
        // possible as the device does not need to be bound to use `dev_*`.
        dev: &device::Device,
        bar: &Bar0,
        cmd: &CMD,
    ) -> Result<T> {
        self.send(bar, cmd)?;

        dev_info!(dev, "RM API: Sent function {:#x}\n", CMD::FUNCTION,);

        // Wait for response
        // TODO: Should this be implemented as a receive(), similar to GSP RPC?
        // TODO: Should this be skipped in case usecase doesn't need a response?
        let response = wait_on_result(Delta::from_secs(5), || {
            match self.receive::<RmGspResponse<CMD::Header>>(CMD::FUNCTION) {
                Ok(response) => Some(Ok(response)),
                Err(EAGAIN) => None,
                Err(e) => Some(Err(e)),
            }
        })?;

        // Check for RM errors
        if response.header.get_status() != 0 {
            dev_err!(
                dev,
                "RM API: Function {:#x} failed with status {:#x}\n",
                CMD::FUNCTION,
                response.header.get_status()
            );
            return Err(EIO);
        }

        // Parse and return data of the expected type
        T::from_bytes(&response.data)
    }
}
