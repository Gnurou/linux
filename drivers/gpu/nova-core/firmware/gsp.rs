// SPDX-License-Identifier: GPL-2.0

use kernel::device;
use kernel::dma::DataDirection;
use kernel::dma::DmaAddress;
use kernel::prelude::*;
use kernel::scatterlist::Owned;
use kernel::scatterlist::SGTable;

use crate::dma::DmaObject;
use crate::gsp::GSP_PAGE_SIZE;

/// A device-mapped firmware with a set of (also device-mapped) pages tables mapping the firmware
/// to the start of their own address space, also known as a `Radix3` firmware.
#[pin_data]
pub(crate) struct GspFirmware {
    /// The GSP firmware inside a [`VVec`], device-mapped via a SG table.
    #[pin]
    fw: SGTable<Owned<VVec<u8>>>,
    /// The level 2 page table, mapping [`Self::fw`] at its beginning.
    #[pin]
    lvl2: SGTable<Owned<VVec<u8>>>,
    /// The level 1 page table, mapping [`Self::lvl2`] at its beginning.
    #[pin]
    lvl1: SGTable<Owned<VVec<u8>>>,
    /// The level 0 page table, mapping [`Self::lvl1`] at its beginning.
    lvl0: DmaObject,
    /// Size in bytes of the firmware contained in [`Self::fw`].
    pub size: usize,
}

impl GspFirmware {
    /// Maps the GSP firmware image `fw` into `dev`'s address-space, and creates the page tables
    /// expected by the GSP bootloader to load it.
    pub(crate) fn new<'a>(
        dev: &'a device::Device<device::Bound>,
        fw: &'a [u8],
    ) -> impl PinInit<Self, Error> + 'a {
        try_pin_init!(&this in Self {
            fw <- {
                // Move the firmware into a vmalloc'd vector and map it into the device address
                // space.
                VVec::with_capacity(fw.len(), GFP_KERNEL)
                .and_then(|mut v| {
                    v.extend_from_slice(fw, GFP_KERNEL)?;
                    Ok(v)
                })
                .map_err(|_| ENOMEM)
                .map(|v| SGTable::new(dev, v, DataDirection::ToDevice, GFP_KERNEL))?
            },
            lvl2 <- {
                // Allocate the level 2 page table, map the firmware onto it, and map it into the
                // device address space.
                // SAFETY: `this` is a valid pointer, and `fw` has been initialized.
                let fw_sg_table = unsafe { &(*this.as_ptr()).fw };
                VVec::<u8>::with_capacity(
                    fw_sg_table.iter().count() * core::mem::size_of::<u64>(),
                    GFP_KERNEL,
                )
                .map_err(|_| ENOMEM)
                .and_then(|lvl2| map_into_lvl(fw_sg_table, lvl2))
                .map(|lvl2| SGTable::new(dev, lvl2, DataDirection::ToDevice, GFP_KERNEL))?
            },
            lvl1 <- {
                // Allocate the level 1 page table, map the level 2 page table onto it, and map it
                // into the device address space.
                // SAFETY: `this` is a valid pointer, and `lvl2` has been initialized.
                let lvl2_sg_table = unsafe { &(*this.as_ptr()).lvl2 };
                VVec::<u8>::with_capacity(
                    lvl2_sg_table.iter().count() * core::mem::size_of::<u64>(),
                    GFP_KERNEL,
                )
                .map_err(|_| ENOMEM)
                .and_then(|lvl1| map_into_lvl(lvl2_sg_table, lvl1))
                .map(|lvl1| SGTable::new(dev, lvl1, DataDirection::ToDevice, GFP_KERNEL))?
            },
            lvl0: {
                // Allocate the level 0 page table as a device-visible DMA object, and map the
                // level 1 page table onto it.
                // SAFETY: `this` is a valid pointer, and `lvl1` has been initialized.
                let lvl1_sg_table = unsafe { &(*this.as_ptr()).lvl1 };
                let mut lvl0 = DmaObject::new(dev, GSP_PAGE_SIZE)?;
                // SAFETY: we are the only owner of this newly-created object, making races
                // impossible.
                let lvl0_slice = unsafe { lvl0.as_slice_mut(0, GSP_PAGE_SIZE) }?;
                lvl0_slice[0..core::mem::size_of::<u64>()].copy_from_slice(
                    #[allow(clippy::useless_conversion)]
                    &(u64::from(lvl1_sg_table.iter().next().unwrap().dma_address())).to_le_bytes(),
                );

                lvl0
            },
            size: fw.len(),
        })
    }

    #[expect(unused)]
    /// Returns the DMA handle of the level 0 page table.
    pub(crate) fn lvl0_dma_handle(&self) -> DmaAddress {
        self.lvl0.dma_handle()
    }
}

/// Create a linear mapping the device mapping of the buffer described by `sg_table` into `dst`.
fn map_into_lvl(sg_table: &SGTable<Owned<VVec<u8>>>, mut dst: VVec<u8>) -> Result<VVec<u8>> {
    for sg_entry in sg_table.iter() {
        // Number of pages we need to map.
        let num_pages = (sg_entry.dma_len() as usize).div_ceil(GSP_PAGE_SIZE);

        for i in 0..num_pages {
            let entry = sg_entry.dma_address() + (i as u64 * GSP_PAGE_SIZE as u64);
            dst.extend_from_slice(&entry.to_le_bytes(), GFP_KERNEL)?;
        }
    }

    Ok(dst)
}
