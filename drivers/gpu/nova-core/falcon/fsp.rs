// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! FSP (Firmware System Processor) falcon engine for Hopper/Blackwell GPUs.
//!
//! The FSP falcon handles secure boot and Chain of Trust operations
//! on Hopper and Blackwell architectures, replacing SEC2's role.

use kernel::{
    io::{
        register::{
            RegisterBase,
            WithBase, //
        },
        Io,
        IoCapable, //
    },
    num::Bounded,
    prelude::*,
    ptr::Alignment, //
};

use crate::{
    driver::Bar0,
    falcon::{
        Falcon,
        FalconEngine,
        PFalcon2Base,
        PFalconBase, //
    },
    regs,
};

/// Type specifying the `Fsp` falcon engine. Cannot be instantiated.
pub(crate) struct Fsp(());

impl RegisterBase<PFalconBase> for Fsp {
    const BASE: usize = 0x8f2000;
}

impl RegisterBase<PFalcon2Base> for Fsp {
    const BASE: usize = 0x8f3000;
}

impl FalconEngine for Fsp {}

/// Maximum addressable EMEM size, derived from the 24-bit offset field
/// in NV_PFALCON_FALCON_EMEM_CTL.
const EMEM_MAX_SIZE: Alignment = Alignment::new::<{ 1 << 24 }>();

/// I/O backend for the FSP falcon's external memory (EMEM).
///
/// Each 32-bit access programs a byte offset via the EMEM_CTL register,
/// then reads or writes through the EMEM_DATA register.
struct Emem<'a> {
    bar: &'a Bar0,
}

impl<'a> Emem<'a> {
    fn new(bar: &'a Bar0) -> Self {
        Self { bar }
    }
}

impl IoCapable<u32> for Emem<'_> {
    unsafe fn io_read(&self, address: usize) -> u32 {
        // PANIC: Per the `io_read` SAFETY comment, `address` is within the I/O bounds of `Self` and
        // thus less than `EMEM_MAX_SIZE`, meaning the `else` block is never taken.
        let Some(offset) =
            Bounded::<usize, { EMEM_MAX_SIZE.log2() }>::try_new(address).map(Bounded::cast::<u32>)
        else {
            unreachable!()
        };

        self.bar.write(
            WithBase::of::<Fsp>(),
            regs::NV_PFALCON_FALCON_EMEM_CTL::zeroed()
                .with_rd_mode(true)
                .with_offset(offset),
        );

        self.bar
            .read(regs::NV_PFALCON_FALCON_EMEM_DATA::of::<Fsp>())
            .data()
    }

    unsafe fn io_write(&self, value: u32, address: usize) {
        // PANIC: Per the `io_write` SAFETY comment, `address` is within the I/O bounds of `Self` and
        // thus less than `EMEM_MAX_SIZE`, meaning the `else` block is never taken.
        let Some(offset) =
            Bounded::<usize, { EMEM_MAX_SIZE.log2() }>::try_new(address).map(Bounded::cast::<u32>)
        else {
            unreachable!()
        };

        self.bar.write(
            WithBase::of::<Fsp>(),
            regs::NV_PFALCON_FALCON_EMEM_CTL::zeroed()
                .with_wr_mode(true)
                .with_offset(offset),
        );

        self.bar.write(
            WithBase::of::<Fsp>(),
            regs::NV_PFALCON_FALCON_EMEM_DATA::zeroed().with_data(value),
        );
    }
}

impl Io for Emem<'_> {
    fn addr(&self) -> usize {
        0
    }

    fn maxsize(&self) -> usize {
        EMEM_MAX_SIZE.as_usize()
    }
}

impl Falcon<Fsp> {
    /// Writes `data` to FSP external memory at byte `offset`.
    ///
    /// Data is interpreted as little-endian 32-bit words.
    /// Returns `EINVAL` if offset or data length is not 4-byte aligned.
    fn write_emem(&self, bar: &Bar0, offset: u32, data: &[u8]) -> Result {
        if offset % 4 != 0 || data.len() % 4 != 0 {
            return Err(EINVAL);
        }

        let emem = Emem::new(bar);
        let mut off = offset as usize;
        for chunk in data.chunks_exact(4) {
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            emem.try_write32(word, off)?;
            off += 4;
        }

        Ok(())
    }

    /// Reads FSP external memory at byte `offset` into `data`.
    ///
    /// Data is stored as little-endian 32-bit words.
    /// Returns `EINVAL` if offset or data length is not 4-byte aligned.
    fn read_emem(&self, bar: &Bar0, offset: u32, data: &mut [u8]) -> Result {
        if offset % 4 != 0 || data.len() % 4 != 0 {
            return Err(EINVAL);
        }

        let emem = Emem::new(bar);
        let mut off = offset as usize;
        for chunk in data.chunks_exact_mut(4) {
            let word = emem.try_read32(off)?;
            chunk.copy_from_slice(&word.to_le_bytes());
            off += 4;
        }

        Ok(())
    }

    /// Poll FSP for incoming data.
    ///
    /// Returns the size of available data in bytes, or 0 if no data is available.
    ///
    /// The FSP message queue is not circular - pointers are reset to 0 after each
    /// message exchange, so `tail >= head` is always true when data is present.
    pub(crate) fn poll_msgq(&self, bar: &Bar0) -> u32 {
        let head = bar.read(regs::NV_PFSP_MSGQ_HEAD).address();
        let tail = bar.read(regs::NV_PFSP_MSGQ_TAIL).address();

        if head == tail {
            return 0;
        }

        // TAIL points at last DWORD written, so add 4 to get total size
        tail.saturating_sub(head) + 4
    }

    /// Send message to FSP.
    ///
    /// Writes a message to FSP EMEM and updates queue pointers to notify FSP.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping
    /// * `packet` - Message data (must be 4-byte aligned in length)
    ///
    /// # Returns
    /// `Ok(())` on success, `Err(EINVAL)` if packet is empty or not 4-byte aligned
    pub(crate) fn send_msg(&self, bar: &Bar0, packet: &[u8]) -> Result {
        if packet.is_empty() {
            return Err(EINVAL);
        }

        // Write message to EMEM at offset 0 (validates 4-byte alignment)
        self.write_emem(bar, 0, packet)?;

        // Update queue pointers - TAIL points at last DWORD written
        let tail_offset = u32::try_from(packet.len() - 4).map_err(|_| EINVAL)?;
        bar.write_reg(regs::NV_PFSP_QUEUE_TAIL::zeroed().with_address(tail_offset));
        bar.write_reg(regs::NV_PFSP_QUEUE_HEAD::zeroed().with_address(0));

        Ok(())
    }

    /// Receive message from FSP.
    ///
    /// Reads a message from FSP EMEM and resets queue pointers.
    ///
    /// # Arguments
    /// * `bar` - BAR0 memory mapping
    /// * `buffer` - Buffer to receive message data
    /// * `size` - Size of message to read in bytes (from `poll_msgq`)
    ///
    /// # Returns
    /// `Ok(bytes_read)` on success, `Err(EINVAL)` if size is 0, exceeds buffer, or not aligned
    pub(crate) fn recv_msg(&self, bar: &Bar0, buffer: &mut [u8], size: usize) -> Result<usize> {
        if size == 0 || size > buffer.len() {
            return Err(EINVAL);
        }

        // Read response from EMEM at offset 0 (validates 4-byte alignment)
        self.read_emem(bar, 0, &mut buffer[..size])?;

        // Reset message queue pointers after reading
        bar.write_reg(regs::NV_PFSP_MSGQ_TAIL::zeroed().with_address(0));
        bar.write_reg(regs::NV_PFSP_MSGQ_HEAD::zeroed().with_address(0));

        Ok(size)
    }
}
