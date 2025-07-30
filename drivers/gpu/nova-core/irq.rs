// SPDX-License-Identifier: GPL-2.0

use crate::gsp::{GspCmdq, GspStaticConfigInfo};
use crate::nvfw::r570_144 as fw;
use crate::rm::control::RmControl;
use crate::rm::RmResponseElement as RmControlMessageElement;
use kernel::alloc::KVec;
use kernel::prelude::*;
use kernel::transmute::AsBytes;
use kernel::{dev_info, device};

// Category subtree map structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SubtreeMap {
    pub subtree_start: u8,
    pub subtree_end: u8,
}

// Interrupt table entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct IrqTableEntry {
    pub engine_idx: u16,
    pub pmc_intr_mask: u32,
    pub vector_stall: u32,
    pub vector_nonstall: u32,
}

// Parameters structure for NV2080_CTRL_CMD_INTERNAL_INTR_GET_KERNEL_TABLE
#[repr(C)]
struct IrqTableParams {
    pub table_len: u32,
    pub table: [IrqTableEntry; fw::NV2080_CTRL_INTERNAL_INTR_MAX_TABLE_SIZE as usize],
    pub subtree_map: [SubtreeMap; fw::NV2080_INTR_CATEGORY_ENUM_COUNT as usize],
}
unsafe impl AsBytes for IrqTableParams {}

impl_from_bytes!(IrqTableParams);

impl RmControl for IrqTableParams {
    const CONTROL: u32 = fw::NV2080_CTRL_CMD_INTERNAL_INTR_GET_KERNEL_TABLE;

    type Response = IrqTable;
}

// Parsed interrupt table structure
#[derive(Debug)]
struct IrqTable {
    pub table_len: u32,
    pub entries: KVec<IrqTableEntry>,
}

impl RmControlMessageElement for IrqTable {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        let params = IrqTableParams::from_bytes(data)?;
        let mut entries = KVec::new();
        let table_len = params.table_len as usize;

        if table_len > fw::NV2080_CTRL_INTERNAL_INTR_MAX_TABLE_SIZE as usize {
            return Err(EINVAL);
        }

        for i in 0..table_len {
            entries.push(params.table[i], GFP_KERNEL)?;
        }

        Ok(Self {
            table_len: params.table_len,
            entries,
        })
    }
}

pub(crate) fn dump_table<'a>(
    cmdq: &mut GspCmdq,
    bar: &'a crate::driver::Bar0,
    gsp_info: &'a GspStaticConfigInfo,
    dev: &'a device::Device<device::Bound>,
) -> Result {
    let params = IrqTableParams {
        table_len: 0,
        table: [IrqTableEntry {
            engine_idx: 0,
            pmc_intr_mask: 0,
            vector_stall: 0,
            vector_nonstall: 0,
        }; fw::NV2080_CTRL_INTERNAL_INTR_MAX_TABLE_SIZE as usize],
        subtree_map: [SubtreeMap {
            subtree_start: 0,
            subtree_end: 0,
        }; fw::NV2080_INTR_CATEGORY_ENUM_COUNT as usize],
    };

    let table: IrqTable = cmdq.send_rm_control(dev, bar, gsp_info, &params)?;

    dev_info!(dev, "Interrupt table: {} entries\n", table.table_len);
    for (i, entry) in table.entries.iter().enumerate() {
        dev_info!(
            dev,
            "  [{}]: engine_idx={} pmc_mask={:#x} stall={:#x} nonstall={:#x}\n",
            i,
            entry.engine_idx,
            entry.pmc_intr_mask,
            entry.vector_stall,
            entry.vector_nonstall
        );
    }

    Ok(())
}
