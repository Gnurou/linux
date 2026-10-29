// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! Loading of the ucodes image, the bindata blob of microcode that GSP-RM loads at run time.

use kernel::{
    device,
    dma::DmaAddress,
    prelude::*, //
};

use crate::{
    firmware::{
        radix3::Radix3,
        tlv::{
            request_tlv,
            Tlv, //
        },
    },
    gpu::Chipset,
};

/// The ucodes image, mapped for GSP-RM through a radix3 page table.
pub(crate) struct UcodesImage<'a> {
    /// The image and the page table that maps it.
    radix3: Pin<KBox<Radix3<'a>>>,
}

impl<'a> UcodesImage<'a> {
    /// Loads the ucodes image that the `ucodes` metadata file names, and maps it for `dev`.
    ///
    /// # Errors
    ///
    /// - `ENOENT` if the metadata file is not installed.
    /// - `EINVAL` if the metadata is malformed.
    /// - `ENOMEM` if the page table cannot be allocated.
    ///
    /// Errors from [`Tlv::load_file`] are propagated as-is.
    pub(crate) fn new(dev: &'a device::Device<device::Bound>, chipset: Chipset) -> Result<Self> {
        let firmware = request_tlv(dev, chipset, "ucodes")?;
        let tlv = Tlv::new(firmware.data())?;
        let image = tlv.load_file(dev, chipset)?;

        Ok(Self {
            radix3: KBox::pin_init(Radix3::new(dev, image), GFP_KERNEL)?,
        })
    }

    /// Returns the DMA address of the level 0 page of the page table that maps the image.
    pub(crate) fn radix3_dma_address(&self) -> DmaAddress {
        self.radix3.dma_address()
    }

    /// Returns the size of the image in bytes.
    pub(crate) fn size(&self) -> usize {
        self.radix3.size()
    }
}
