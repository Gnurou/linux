// SPDX-License-Identifier: GPL-2.0
//
// Common RM API implementation for nova-core
// Provides generic infrastructure for RM control and RM alloc operations.

use crate::gsp::{GspCmdq, GspCommand, GspCommandElement, GspMessageElement, GspStaticConfigInfo};
use crate::sbuffer::SBuffer;
use crate::nvfw::r570_144 as fw;
use crate::util::wait_on_result;
use core::marker::PhantomData;
use kernel::device;
use kernel::prelude::*;
use kernel::time::Delta;
use kernel::{dev_err, dev_info};

/// Trait for RM headers common to all RM API operations
pub(crate) trait RmHeader: GspMessageElement {
    /// Set the client handle of a request
    fn set_client(&mut self, client: u32);

    /// Set the status code of a request
    fn set_status(&mut self, status: u32);

    /// Set the parameters size of a request
    fn set_params_size(&mut self, size: u32);

    /// Set flags of a request
    fn set_flags(&mut self, flags: u32);

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
struct RmMessage<'a, H: RmHeader> {
    header: H,
    params: Option<&'a [u8]>,
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

        // Write params if present
        if let Some(params) = self.params {
            sbuf.write_all(params)?;
        }

        Ok(())
    }

    fn size(&self) -> usize {
        core::mem::size_of::<H>() + self.params.map_or(0, |p| p.len())
    }
}

/// Wrapper for RM Control commands
struct RmControlCmd<'a, H: RmHeader>(RmMessage<'a, H>);

impl<'a, H: RmHeader> GspCommandElement for RmControlCmd<'a, H> {
    fn copy_to_sbuf<'b, I: Iterator<Item = &'b mut [u8]>>(&self, sbuf: &mut SBuffer<I>) -> Result {
        self.0.copy_to_sbuf(sbuf)
    }

    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<'a, H: RmHeader> GspCommand for RmControlCmd<'a, H> {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL;
}

/// Wrapper for RM Alloc commands
struct RmAllocCmd<'a, H: RmHeader>(RmMessage<'a, H>);

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

/// Trait for input parameters
pub(crate) trait RmParams {
    fn to_bytes(&self) -> &[u8];
}

/// Trait for RM response message elements
pub(crate) trait RmResponseElement: Sized {
    /// Parse response from bytes
    fn from_bytes(data: &[u8]) -> Result<Self>;
}

/// Generic RM API struct for all RM API operations
pub(crate) struct RmApi<'a, H: RmHeader> {
    cmdq: &'a mut GspCmdq,
    bar: &'a crate::driver::Bar0,
    gsp_info: &'a GspStaticConfigInfo,
    dev: &'a device::Device<device::Bound>,
    function: u32,
    _header: PhantomData<H>,
}

impl<'a, H: RmHeader> RmApi<'a, H> {
    /// Create new RM API instance
    pub(crate) fn new(
        cmdq: &'a mut GspCmdq,
        bar: &'a crate::driver::Bar0,
        gsp_info: &'a GspStaticConfigInfo,
        dev: &'a device::Device<device::Bound>,
        function: u32,
    ) -> Self {
        Self {
            cmdq,
            bar,
            gsp_info,
            dev,
            function,
            _header: PhantomData,
        }
    }

    /// Send an RM command with optional params and get response
    pub(crate) fn send<P: RmParams, T: RmResponseElement>(
        &mut self,
        mut header: H,
        params: Option<&P>,
    ) -> Result<T> {
        let params_size = params.map_or(0, |p| p.to_bytes().len());

        // Configure common header fields
        header.set_client(self.gsp_info.h_internal_client);
        header.set_status(0);
        header.set_params_size(params_size as u32);
        header.set_flags(0);

        // Create message wrapper
        let msg = RmMessage {
            header,
            params: params.map(|p| p.to_bytes()),
        };

        // Send the command using appropriate wrapper based on function
        match self.function {
            fw::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL => {
                let cmd = RmControlCmd(msg);
                self.cmdq.send(self.bar, &cmd)?;
            }
            fw::NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC => {
                let cmd = RmAllocCmd(msg);
                self.cmdq.send(self.bar, &cmd)?;
            }
            _ => {
                dev_err!(self.dev, "Unknown RM function: {:#x}\n", self.function);
                return Err(EINVAL);
            }
        }

        dev_info!(
            self.dev,
            "RM API: Sent function {:#x} with {} bytes params\n",
            self.function,
            params_size
        );

        // Wait for response
        // TODO: Should this be implemented as a receive(), similar to GSP RPC?
        // TODO: Should this be skipped in case usecase doesn't need a response?
        let response = wait_on_result(Delta::from_secs(5), || {
            match self.cmdq.receive::<RmGspResponse<H>>(self.function) {
                Ok(response) => Some(Ok(response)),
                Err(EAGAIN) => None,
                Err(e) => Some(Err(e)),
            }
        })?;

        // Check for RM errors
        if response.header.get_status() != 0 {
            dev_err!(
                self.dev,
                "RM API: Function {:#x} failed with status {:#x}\n",
                self.function,
                response.header.get_status()
            );
            return Err(EIO);
        }

        // Parse and return data of the expected type
        T::from_bytes(&response.data)
    }

    /// Get the internal subdevice handle (useful for control operations)
    pub(crate) fn subdevice_handle(&self) -> u32 {
        self.gsp_info.h_internal_subdevice
    }
}
