// SPDX-License-Identifier: GPL-2.0

//! GSP Sequencer implementation for Pre-hopper GSP boot sequence.

use core::mem::size_of;
use kernel::alloc::flags::GFP_KERNEL;
use kernel::bindings;
use kernel::device;
use kernel::io::poll::read_poll_timeout;
use kernel::prelude::*;
use kernel::time::Delta;

use crate::driver::Bar0;
use crate::falcon::{gsp::Gsp, sec2::Sec2, Falcon};
use crate::firmware::gsp::GspFirmware;
use crate::gsp::cmdq::{GspCmdq, GspMessageFromGsp};
use crate::gsp::fw;

use kernel::transmute::FromBytes;
use kernel::{dev_dbg, dev_err};

unsafe impl FromBytes for fw::GSP_SEQUENCER_BUFFER_CMD {}
unsafe impl FromBytes for fw::rpc_run_cpu_sequencer_v17_00 {}
impl GspMessageFromGsp for fw::rpc_run_cpu_sequencer_v17_00 {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER;
}

const CMD_SIZE: usize = size_of::<fw::GSP_SEQUENCER_BUFFER_CMD>();

struct GspSequencerInfo<'a> {
    info: &'a fw::rpc_run_cpu_sequencer_v17_00,
    cmd_data: KVec<u8>,
}

/// GSP Sequencer Command types with payload data
/// Commands have an opcode and a opcode-dependent struct.
pub(crate) enum GspSeqCmd {
    RegWrite(fw::GSP_SEQ_BUF_PAYLOAD_REG_WRITE),
    RegModify(fw::GSP_SEQ_BUF_PAYLOAD_REG_MODIFY),
    RegPoll(fw::GSP_SEQ_BUF_PAYLOAD_REG_POLL),
    DelayUs(fw::GSP_SEQ_BUF_PAYLOAD_DELAY_US),
    RegStore(fw::GSP_SEQ_BUF_PAYLOAD_REG_STORE),
    CoreReset,
    CoreStart,
    CoreWaitForHalt,
}

impl GspSeqCmd {
    /// Creates a new GspSeqCmd from a firmware GSP_SEQUENCER_BUFFER_CMD
    pub(crate) fn from_fw_cmd(cmd: &fw::GSP_SEQUENCER_BUFFER_CMD) -> Result<Self> {
        match cmd.opCode {
            // SAFETY: In the below unsafe accesses, we're using the union field
            //         that corresponds to the opCode
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_WRITE => {
                Ok(GspSeqCmd::RegWrite(unsafe { cmd.payload.regWrite }))
            }
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_MODIFY => {
                Ok(GspSeqCmd::RegModify(unsafe { cmd.payload.regModify }))
            }
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_POLL => {
                Ok(GspSeqCmd::RegPoll(unsafe { cmd.payload.regPoll }))
            }
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_DELAY_US => {
                Ok(GspSeqCmd::DelayUs(unsafe { cmd.payload.delayUs }))
            }
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_STORE => {
                Ok(GspSeqCmd::RegStore(unsafe { cmd.payload.regStore }))
            }
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_RESET => Ok(GspSeqCmd::CoreReset),
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_START => Ok(GspSeqCmd::CoreStart),
            fw::GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_WAIT_FOR_HALT => {
                Ok(GspSeqCmd::CoreWaitForHalt)
            }
            _ => Err(EINVAL),
        }
    }

    pub(crate) fn new(data: &[u8], dev: &device::Device<device::Bound>) -> Result<Self> {
        let fw_cmd = fw::GSP_SEQUENCER_BUFFER_CMD::from_bytes(data).ok_or(EINVAL)?;
        let cmd = Self::from_fw_cmd(fw_cmd)?;

        if data.len() < cmd.size_bytes() {
            dev_err!(dev, "data is not enough for command.\n");
            return Err(EINVAL);
        }

        Ok(cmd)
    }

    /// Get the size of this command in bytes, the command consists of
    /// a 4-byte opcode, and a variable-sized payload.
    pub(crate) fn size_bytes(&self) -> usize {
        let opcode_size = size_of::<fw::GSP_SEQ_BUF_OPCODE>();
        match self {
            // Each simple command type just adds 4 bytes (opcode_size) for the header
            GspSeqCmd::CoreReset | GspSeqCmd::CoreStart | GspSeqCmd::CoreWaitForHalt => opcode_size,

            // For commands with payloads, add the payload size in bytes
            GspSeqCmd::RegWrite(_) => opcode_size + size_of::<fw::GSP_SEQ_BUF_PAYLOAD_REG_WRITE>(),
            GspSeqCmd::RegModify(_) => {
                opcode_size + size_of::<fw::GSP_SEQ_BUF_PAYLOAD_REG_MODIFY>()
            }
            GspSeqCmd::RegPoll(_) => opcode_size + size_of::<fw::GSP_SEQ_BUF_PAYLOAD_REG_POLL>(),
            GspSeqCmd::DelayUs(_) => opcode_size + size_of::<fw::GSP_SEQ_BUF_PAYLOAD_DELAY_US>(),
            GspSeqCmd::RegStore(_) => opcode_size + size_of::<fw::GSP_SEQ_BUF_PAYLOAD_REG_STORE>(),
        }
    }
}

#[expect(dead_code)]
pub(crate) struct GspSequencer<'a> {
    seq_info: GspSequencerInfo<'a>,
    bar: &'a Bar0,
    sec2_falcon: &'a Falcon<Sec2>,
    gsp_falcon: &'a Falcon<Gsp>,
    libos_dma_handle: u64,
    gsp_fw: &'a GspFirmware,
    dev: &'a device::Device<device::Bound>,
}

pub(crate) trait GspSeqCmdRunner {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result;
}

impl GspSeqCmdRunner for fw::GSP_SEQ_BUF_PAYLOAD_REG_WRITE {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result {
        dev_dbg!(
            sequencer.dev,
            "RegWrite: addr=0x{:x}, val=0x{:x}\n",
            self.addr,
            self.val
        );
        let addr = self.addr as usize;
        let val = self.val;
        let _ = sequencer.bar.try_write32(val, addr);
        Ok(())
    }
}

impl GspSeqCmdRunner for fw::GSP_SEQ_BUF_PAYLOAD_REG_MODIFY {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result {
        dev_dbg!(
            sequencer.dev,
            "RegModify: addr=0x{:x}, mask=0x{:x}, val=0x{:x}\n",
            self.addr,
            self.mask,
            self.val
        );

        let addr = self.addr as usize;
        if let Ok(temp) = sequencer.bar.try_read32(addr) {
            let _ = sequencer
                .bar
                .try_write32((temp & !self.mask) | self.val, addr);
        }
        Ok(())
    }
}

impl GspSeqCmdRunner for fw::GSP_SEQ_BUF_PAYLOAD_REG_POLL {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result {
        dev_dbg!(
            sequencer.dev,
            "RegPoll: addr=0x{:x}, mask=0x{:x}, val=0x{:x}, timeout=0x{:x}, error=0x{:x}\n",
            self.addr,
            self.mask,
            self.val,
            self.timeout,
            self.error
        );

        let addr = self.addr as usize;
        let mut timeout_us = self.timeout as i64;

        // Default timeout to 4 seconds
        timeout_us = if timeout_us == 0 { 4000000 } else { timeout_us };

        // First read
        sequencer.bar.try_read32(addr)?;

        // Poll the requested register with requested timeout.
        read_poll_timeout(
            || sequencer.bar.try_read32(addr),
            |current| (current & self.mask) == self.val,
            Delta::ZERO,
            Delta::from_micros(timeout_us),
        )
        .map(|_| ())
    }
}

impl GspSeqCmdRunner for fw::GSP_SEQ_BUF_PAYLOAD_DELAY_US {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result {
        dev_dbg!(sequencer.dev, "DelayUs: val=0x{:x}\n", self.val);
        // SAFETY: `usleep_range_state` is safe to call with any parameter.
        unsafe {
            bindings::usleep_range_state(
                self.val as usize,
                self.val as usize,
                bindings::TASK_UNINTERRUPTIBLE as u32,
            )
        };
        Ok(())
    }
}

impl GspSeqCmdRunner for fw::GSP_SEQ_BUF_PAYLOAD_REG_STORE {
    fn run(&self, sequencer: &GspSequencer<'_>) -> Result {
        let addr = self.addr as usize;
        let _index = self.index;

        let val = sequencer.bar.try_read32(addr)?;

        dev_dbg!(
            sequencer.dev,
            "RegStore: addr=0x{:x}, index=0x{:x}, value={:?}\n",
            self.addr,
            self.index,
            val
        );

        Ok(())
    }
}

impl GspSeqCmdRunner for GspSeqCmd {
    fn run(&self, seq: &GspSequencer<'_>) -> Result {
        match self {
            GspSeqCmd::RegWrite(cmd) => cmd.run(seq),
            GspSeqCmd::RegModify(cmd) => cmd.run(seq),
            GspSeqCmd::RegPoll(cmd) => cmd.run(seq),
            GspSeqCmd::DelayUs(cmd) => cmd.run(seq),
            GspSeqCmd::RegStore(cmd) => cmd.run(seq),
            GspSeqCmd::CoreReset => {
                dev_dbg!(seq.dev, "CoreReset\n");
                seq.gsp_falcon.reset(seq.bar)?;
                seq.gsp_falcon.dma_reset(seq.bar);
                Ok(())
            }
            GspSeqCmd::CoreStart => {
                dev_dbg!(seq.dev, "CoreStart\n");
                seq.gsp_falcon.start(seq.bar)?;
                Ok(())
            }
            GspSeqCmd::CoreWaitForHalt => {
                dev_dbg!(seq.dev, "CoreWaitForHalt\n");
                seq.gsp_falcon.wait_till_halted(seq.bar)?;
                Ok(())
            }
        }
    }
}

pub(crate) struct GspSeqIter<'a> {
    cmd_data: &'a [u8],
    current_offset: usize, // Tracking the current position
    total_cmds: u32,
    cmds_processed: u32,
    dev: &'a device::Device<device::Bound>,
}

impl<'a> Iterator for GspSeqIter<'a> {
    type Item = Result<GspSeqCmd>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if we've processed all commands or reached the end of data
        if self.cmds_processed >= self.total_cmds || self.current_offset >= self.cmd_data.len() {
            return None;
        }

        // Check if we have enough data for opcode
        let opcode_size = size_of::<fw::GSP_SEQ_BUF_OPCODE>();
        if self.current_offset + opcode_size > self.cmd_data.len() {
            return Some(Err(EINVAL));
        }

        let offset = self.current_offset;

        // Handle command creation based on available data,
        // zero-pad if necessary (since last command may not be full size).
        let mut buffer = [0u8; CMD_SIZE];
        let copy_len = if offset + CMD_SIZE <= self.cmd_data.len() {
            CMD_SIZE
        } else {
            self.cmd_data.len() - offset
        };
        buffer[..copy_len].copy_from_slice(&self.cmd_data[offset..offset + copy_len]);
        let cmd_result = GspSeqCmd::new(&buffer, self.dev);

        cmd_result.map_or_else(
            |_err| {
                dev_err!(self.dev, "Error parsing command at offset {}", offset);
                None
            },
            |cmd| {
                self.current_offset += cmd.size_bytes();
                self.cmds_processed += 1;
                Some(Ok(cmd))
            },
        )
    }
}

impl<'a, 'b> IntoIterator for &'b GspSequencer<'a> {
    type Item = Result<GspSeqCmd>;
    type IntoIter = GspSeqIter<'b>;

    fn into_iter(self) -> Self::IntoIter {
        let cmd_data = &self.seq_info.cmd_data[..];

        GspSeqIter {
            cmd_data,
            current_offset: 0,
            total_cmds: self.seq_info.info.cmdIndex,
            cmds_processed: 0,
            dev: self.dev,
        }
    }
}

impl<'a> GspSequencer<'a> {
    pub(crate) fn run(
        cmdq: &mut GspCmdq,
        gsp_fw: &GspFirmware,
        libos_dma_handle: u64,
        gsp_falcon: &'a Falcon<Gsp>,
        sec2_falcon: &'a Falcon<Sec2>,
        dev: &'a device::Device<device::Bound>,
        bar: &'a Bar0,
        timeout: Delta,
    ) -> Result {
        cmdq.wait_for_msg_from_gsp(timeout)?;
        let msg = cmdq.receive_msg_from_gsp()?;

        let (info, mut sbuf) = msg.try_as::<fw::rpc_run_cpu_sequencer_v17_00>()?;
        let cmd_data = match sbuf {
            Some(ref mut sbuf) => sbuf.read_into_kvec(GFP_KERNEL),
            _ => Err(EINVAL),
        }?;
        let seq_info = GspSequencerInfo { info, cmd_data };

        let sequencer = GspSequencer {
            seq_info,
            bar,
            sec2_falcon,
            gsp_falcon,
            libos_dma_handle,
            gsp_fw,
            dev,
        };

        dev_dbg!(dev, "Running CPU Sequencer commands\n");

        for cmd_result in &sequencer {
            match cmd_result {
                Ok(cmd) => cmd.run(&sequencer)?,
                Err(e) => {
                    dev_err!(
                        dev,
                        "Error running command at index {}\n",
                        sequencer.seq_info.info.cmdIndex
                    );
                    return Err(e);
                }
            }
        }

        dev_dbg!(dev, "CPU Sequencer commands completed successfully\n");

        drop(sbuf);
        msg.ack()?;

        Ok(())
    }
}
