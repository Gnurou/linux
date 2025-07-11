// SPDX-License-Identifier: GPL-2.0

#![allow(dead_code, unused_variables)]

use kernel::bindings;
use kernel::device;
use kernel::dma::DmaDataDirection;
use kernel::prelude::*;
use kernel::scatterlist::ManagedMapping;
use kernel::scatterlist::OwnedSgt;
use kernel::scatterlist::SGTable;
use kernel::scatterlist::SGTablePages;
use kernel::types::ARef;

use crate::dma::DmaObject;
use crate::gsp::GSP_PAGE_SIZE;

pub(crate) struct Radix3 {
    lvl0: DmaObject,
    lvl1: DmaObject,
}

impl Radix3 {
    pub(crate) fn new(dev: &device::Device, size: usize) -> Result<Self> {
        Err(ENOTSUPP)
    }
}

pub(crate) struct RadixFirmware {
    // pub radix3: Radix3,
    dev: ARef<device::Device>,
    fw_sg_table: SGTable<OwnedSgt<VVec<u8>>, ManagedMapping>,
    lvl2_sg_table: SGTable<OwnedSgt<VVec<u8>>, ManagedMapping>,
    lvl1_sg_table: SGTable<OwnedSgt<VVec<u8>>, ManagedMapping>,
    lvl0: DmaObject,
    size: usize,
}

impl RadixFirmware {
    pub(crate) fn new(
        dev: &device::Device<device::Bound>,
        name: &'static str,
        fw: &[u8],
    ) -> Result<Self> {
        pr_info!("GSP firmware has size {:#x}\n", fw.len());

        // Move the firmware into a vmalloc'd vector.
        let mut fw_vvec = VVec::with_capacity(fw.len(), GFP_KERNEL)?;
        fw_vvec.extend_from_slice(fw, GFP_KERNEL)?;

        let fw_sg_table = SGTable::new_owned(fw_vvec, GFP_KERNEL)
            .and_then(|sg| sg.dma_map(dev, DmaDataDirection::DmaToDevice))?;
        pr_info!(
            "FW SG table has {} (from {}) entries\n",
            fw_sg_table.as_ref().nents,
            fw_sg_table.as_ref().orig_nents
        );

        let mut lvl2 = VVec::<u8>::with_capacity(
            fw_sg_table.borrow().num_pages() * core::mem::size_of::<u64>(),
            GFP_KERNEL,
        )?;

        pr_info!(
            "lvl2 allocated with capacity {} size {}\n",
            lvl2.capacity(),
            lvl2.len()
        );

        map_into_lvl(&fw_sg_table, &mut lvl2)?;

        pr_info!(
            "lvl2 filled with capacity {} size {} or {} entries\n",
            lvl2.capacity(),
            lvl2.len(),
            lvl2.len() / core::mem::size_of::<u64>(),
        );

        let lvl2_sg_table = SGTable::new_owned(lvl2, GFP_KERNEL)
            .and_then(|sg| sg.dma_map(dev, DmaDataDirection::DmaToDevice))?;
        pr_info!(
            "LVL2 SG table has {} (from {}) entries\n",
            lvl2_sg_table.as_ref().nents,
            lvl2_sg_table.as_ref().orig_nents
        );

        let mut lvl1 = VVec::<u8>::with_capacity(
            lvl2_sg_table.borrow().num_pages() * core::mem::size_of::<u64>(),
            GFP_KERNEL,
        )?;

        pr_info!(
            "lvl1 allocated with capacity {} size {}\n",
            lvl1.capacity(),
            lvl1.len()
        );

        map_into_lvl(&lvl2_sg_table, &mut lvl1)?;

        pr_info!(
            "lvl1 filled with capacity {} size {} or {} entries\n",
            lvl1.capacity(),
            lvl1.len(),
            lvl1.len() / core::mem::size_of::<u64>(),
        );

        let lvl1_sg_table = SGTable::new_owned(lvl1, GFP_KERNEL)
            .and_then(|sg| sg.dma_map(dev, DmaDataDirection::DmaToDevice))?;
        pr_info!(
            "LVL1 SG table has {} (from {}) entries\n",
            lvl1_sg_table.as_ref().nents,
            lvl1_sg_table.as_ref().orig_nents
        );

        let mut lvl0 = DmaObject::new(dev, GSP_PAGE_SIZE)?;
        let lvl0_slice =
            unsafe { core::slice::from_raw_parts_mut(lvl0.start_ptr_mut(), lvl0.size()) };
        lvl0_slice[0..core::mem::size_of::<u64>()].copy_from_slice(
            &(lvl1_sg_table.iter().next().unwrap().dma_address() as u64).to_le_bytes(),
        );

        pr_info!("LVL0 has DMA address {:#x}\n", lvl0.dma_handle());
        pr_info!(
            "First entry of LVL0: {:x}\n",
            u64::from_le_bytes(unsafe { *(lvl0.start_ptr().cast::<[u8; 8]>()) })
        );
        pr_info!(
            "LVL1 has DMA address {:#x}\n",
            lvl1_sg_table.iter().next().unwrap().dma_address()
        );
        pr_info!(
            "First entry of LVL1: {:x}\n",
            u64::from_le_bytes((&lvl1_sg_table.borrow()[0..8]).try_into().unwrap())
        );
        pr_info!(
            "LVL2 has DMA address {:#x}\n",
            lvl2_sg_table.iter().next().unwrap().dma_address()
        );
        pr_info!(
            "First entry of LVL2: {:x}\n",
            u64::from_le_bytes((&lvl2_sg_table.borrow()[0..8]).try_into().unwrap())
        );

        Ok(Self {
            dev: dev.into(),
            fw_sg_table,
            lvl2_sg_table,
            lvl1_sg_table,
            lvl0,
            size: fw.len(),
        })
    }

    pub(crate) fn lvl0_dma_handle(&self) -> bindings::dma_addr_t {
        self.lvl0.dma_handle()
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }
}

fn map_into_lvl(
    sg_table: &SGTable<OwnedSgt<VVec<u8>>, ManagedMapping>,
    dst: &mut VVec<u8>,
) -> Result {
    for sg_entry in sg_table.iter() {
        pr_debug!(
            "sl: {:#x} {:#x}\n",
            sg_entry.dma_address(),
            sg_entry.dma_len()
        );
        // Round the size up to the next full page, if needed.
        // TODO: handle the case if GSP_PAGE_SIZE != PAGE_SIZE!
        let rounded_up_length = (sg_entry.dma_len() as usize)
            .checked_next_multiple_of(GSP_PAGE_SIZE)
            .ok_or(EINVAL)?;
        for i in 0..(rounded_up_length / GSP_PAGE_SIZE) {
            let entry = sg_entry.dma_address() + (GSP_PAGE_SIZE as u64 * i as u64);
            let entry_bytes = entry.to_le_bytes();
            dst.extend_from_slice(&entry_bytes, GFP_KERNEL)?;
        }
    }

    Ok(())
}
