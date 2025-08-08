// SPDX-License-Identifier: GPL-2.0

//! FSP (Firmware System Processor) falcon engine for Hopper/Blackwell GPUs.
//!
//! The FSP falcon handles secure boot and Chain of Trust operations
//! on Hopper and Blackwell architectures, replacing SEC2's role.

use kernel::prelude::*;

use crate::driver::Bar0;
use crate::{
    falcon::{Falcon, FalconEngine, PFalcon2Base, PFalconBase},
    regs::{self, macros::RegisterBase},
};

/// Type specifying the `Fsp` falcon engine. Cannot be instantiated.
#[allow(dead_code)]
pub(crate) struct Fsp(());

impl RegisterBase<PFalconBase> for Fsp {
    // FSP falcon base address for Blackwell
    const BASE: usize = 0x8f2000;
}

impl RegisterBase<PFalcon2Base> for Fsp {
    const BASE: usize = 0x8f3000;
}

impl FalconEngine for Fsp {
    const ID: Self = Fsp(());
}

impl Falcon<Fsp> {
    /// Write data to FSP external memory using Falcon PIO (Programmed I/O).
    ///
    /// This function writes data to the FSP (Falcon Security Processor) external
    /// memory space using the Falcon's indirect memory access interface.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping for register access
    /// * `offset` - Byte offset within FSP external memory to start writing
    /// * `data` - Slice of bytes to write to memory (must be 4-byte aligned)
    ///
    /// # Returns
    /// `Ok(())` on successful write, or an error if register operations fail.
    ///
    /// # Note
    /// The data length must be 4-byte aligned as required by the falcon hardware.
    #[allow(dead_code)]
    pub(crate) fn write_emem(&self, bar: &Bar0, offset: u32, data: &[u8]) -> Result {
        // Use GP102 EMEM PIO registers for FSP
        // Initialize EMEM write: BIT(24) | emem_base
        regs::NV_PFALCON_FALCON_EMEM_CTL::default()
            .set_value((1 << 24) | offset)
            .write(bar, &Fsp::ID);

        // Write data in 4-byte chunks using GP102 EMEM data register
        for chunk in data.chunks(4) {
            let mut word = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                word |= (byte as u32) << (i * 8);
            }

            regs::NV_PFALCON_FALCON_EMEM_DATA::default()
                .set_data(word)
                .write(bar, &Fsp::ID);
        }

        Ok(())
    }

    /// Read data from FSP external memory using Falcon PIO (Programmed I/O).
    ///
    /// This function reads data from the FSP (Falcon Security Processor) external
    /// memory space using the Falcon's indirect memory access interface.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping for register access
    /// * `offset` - Byte offset within FSP external memory to start reading
    /// * `data` - Mutable slice to store the read data (must be 4-byte aligned)
    ///
    /// # Returns
    /// `Ok(())` on successful read, or an error if register operations fail.
    ///
    /// # Note
    /// The data length must be 4-byte aligned as required by the falcon hardware.
    #[allow(dead_code)]
    pub(crate) fn read_emem(&self, bar: &Bar0, offset: u32, data: &mut [u8]) -> Result {
        // Use GP102 EMEM PIO registers for FSP
        // Initialize EMEM read: BIT(25) | emem_base (different from write which uses BIT(24))
        regs::NV_PFALCON_FALCON_EMEM_CTL::default()
            .set_value((1 << 25) | offset)
            .write(bar, &Fsp::ID);

        // Read data in 4-byte chunks using GP102 EMEM data register
        for chunk in data.chunks_mut(4) {
            let word = regs::NV_PFALCON_FALCON_EMEM_DATA::read(bar, &Fsp::ID).data();

            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte = ((word >> (i * 8)) & 0xff) as u8;
            }
        }

        Ok(())
    }

    /// Poll FSP for incoming data.
    ///
    /// Returns the size of available data in bytes, or 0 if no data is available.
    #[allow(dead_code)]
    pub(crate) fn poll_msgq(&self, bar: &Bar0) -> u32 {
        let head = regs::NV_PFSP_MSGQ_HEAD::read(bar).address();
        let tail = regs::NV_PFSP_MSGQ_TAIL::read(bar).address();

        if head == tail {
            return 0;
        }

        // Ensure tail >= head (no wraparound in this simple implementation)
        if tail < head {
            return 0;
        }

        // TAIL points at last DWORD written, so add 4 to get total size
        tail.saturating_sub(head).saturating_add(4)
    }

    /// Send message to FSP.
    ///
    /// Writes a message to FSP EMEM and updates queue pointers to notify FSP.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping
    /// * `packet` - Message data (must be 4-byte aligned)
    ///
    /// # Returns
    /// `Ok(())` on success, `Err(EINVAL)` if packet size is invalid
    #[allow(dead_code)]
    pub(crate) fn send_msg(&self, bar: &Bar0, packet: &[u8]) -> Result {
        let packet_size = packet.len();

        if packet_size == 0 || packet_size % 4 != 0 {
            return Err(EINVAL);
        }

        // Write message to EMEM at offset 0
        self.write_emem(bar, 0, packet)?;

        // Update queue pointers - TAIL points at last DWORD written
        regs::NV_PFSP_QUEUE_TAIL::default()
            .set_address((packet_size - 4) as u32)
            .write(bar);
        regs::NV_PFSP_QUEUE_HEAD::default()
            .set_address(0)
            .write(bar);

        Ok(())
    }

    /// Receive message from FSP.
    ///
    /// Reads a message from FSP EMEM and resets queue pointers.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping
    /// * `buffer` - Buffer to receive message data
    /// * `packet_size` - Size of message to read (from poll_msgq)
    ///
    /// # Returns
    /// `Ok(bytes_read)` on success
    #[allow(dead_code)]
    pub(crate) fn recv_msg(
        &self,
        bar: &Bar0,
        buffer: &mut [u8],
        packet_size: u32,
    ) -> Result<usize> {
        let packet_size = packet_size as usize;

        if packet_size == 0 || packet_size % 4 != 0 || packet_size > buffer.len() {
            return Err(EINVAL);
        }

        // Read response from EMEM at offset 0
        self.read_emem(bar, 0, &mut buffer[..packet_size])?;

        // Reset message queue pointers after reading
        regs::NV_PFSP_MSGQ_TAIL::default().set_address(0).write(bar);
        regs::NV_PFSP_MSGQ_HEAD::default().set_address(0).write(bar);

        Ok(packet_size)
    }
}
