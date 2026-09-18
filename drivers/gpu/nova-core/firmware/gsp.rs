// SPDX-License-Identifier: GPL-2.0

use kernel::{
    device,
    dma::{
        Coherent,
        DmaAddress, //
    },
    prelude::*, //
};

use crate::{
    firmware::{
        radix3::Radix3,
        riscv::RiscvFirmware, //
        tlv::{
            request_tlv, //
            Tlv,
        },
    },
    gpu::Chipset, //
};

/// The GSP firmware image, its signatures, and the GSP bootloader.
#[pin_data]
pub(crate) struct GspFirmware<'a> {
    /// The firmware image and the radix3 table that maps it.
    #[pin]
    radix3: Radix3<'a>,
    /// Device-mapped GSP signatures matching the GPU's [`Chipset`].
    pub(crate) signatures: Coherent<'a, [u8]>,
    /// GSP bootloader, verifies the GSP firmware before loading and running it.
    pub(crate) bootloader: RiscvFirmware<'a>,
}

impl<'a> GspFirmware<'a> {
    /// Loads the GSP firmware binaries, map them into `dev`'s address-space, and creates the page
    /// tables expected by the GSP bootloader to load it.
    pub(crate) fn new(
        dev: &'a device::Device<device::Bound>,
        chipset: Chipset,
    ) -> impl PinInit<Self, Error> + 'a {
        pin_init::pin_init_scope(move || {
            let firmware = request_tlv(dev, chipset, "gsp")?;
            let tlv = Tlv::new(firmware.data())?;
            dev_dbg!(dev, "loaded gsp firmware v{}\n", tlv.get_string(b"VERS")?);

            let fw_vvec = tlv.load_file(dev, chipset)?;

            let signatures = Coherent::from_slice(dev, tlv.get_bytes(b"SIGN")?, GFP_KERNEL)?;

            Ok(try_pin_init!(Self {
                radix3 <- Radix3::new(dev, fw_vvec),
                signatures,
                bootloader: {
                    let bl = request_tlv(dev, chipset, "gsp_bootloader")?;

                    RiscvFirmware::new(dev, &bl)?
                },
            }))
        })
    }

    /// Returns the size of the firmware image, in bytes.
    pub(crate) fn size(&self) -> usize {
        self.radix3.size()
    }

    /// Returns the DMA address of the radix3 table that maps the firmware image.
    pub(crate) fn radix3_dma_address(&self) -> DmaAddress {
        self.radix3.dma_address()
    }
}
