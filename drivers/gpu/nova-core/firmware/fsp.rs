use kernel::prelude::*;
use kernel::{device, prelude::EINVAL};

use crate::{dma::DmaObject, gpu::Chipset};

#[expect(unused)]
pub(crate) struct FspFirmware {
    /// FMC firmware image data (only the .image section)
    fmc_image: DmaObject,
    /// Full FMC ELF data (for signature extraction)
    fmc_full: DmaObject,
}

impl FspFirmware {
    #[expect(unused)]
    pub(crate) fn new(
        dev: &device::Device<device::Bound>,
        chipset: Chipset,
        ver: &str,
    ) -> Result<Self> {
        let fw = super::request_firmware(dev, chipset, "fsp", ver)?;

        // FSP expects only the .image section, not the entire ELF file
        let fmc_image_data = super::elf_section(fw.data(), "image").ok_or_else(|| {
            dev_err!(dev, "FMC ELF file missing 'image' section\n");
            EINVAL
        })?;

        Ok(Self {
            fmc_image: DmaObject::from_data(dev, fmc_image_data)?,
            fmc_full: DmaObject::from_data(dev, fw.data())?,
        })
    }
}
