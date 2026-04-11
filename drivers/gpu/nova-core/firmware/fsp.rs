// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

//! FSP is a hardware unit that runs FMC firmware.

use kernel::{
    device,
    dma::Coherent,
    firmware::Firmware,
    prelude::*, //
};

use crate::{
    firmware::elf,
    gpu::Chipset, //
};

/// Size of the FSP SHA-384 hash, in bytes.
pub(crate) const FSP_HASH_SIZE: usize = 48;
/// Size of the RSA-3072 public key, in bytes.
pub(crate) const FSP_PKEY_SIZE: usize = 384;
/// Size of the RSA-3072 signature, in bytes.
pub(crate) const FSP_SIG_SIZE: usize = 384;

/// Structure to hold FMC signatures.
///
/// C representation is used because this type is used for communication with the FSP.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct FmcSignatures {
    pub(crate) hash384: [u8; FSP_HASH_SIZE],
    pub(crate) public_key: [u8; FSP_PKEY_SIZE],
    pub(crate) signature: [u8; FSP_SIG_SIZE],
}

pub(crate) struct FspFirmware {
    /// FMC firmware image data (only the "image" ELF section).
    #[expect(unused)]
    pub(crate) fmc_image: Coherent<[u8]>,
    /// FMC firmware signatures.
    #[expect(unused)]
    pub(crate) fmc_sigs: KBox<FmcSignatures>,
}

impl FspFirmware {
    pub(crate) fn new(
        dev: &device::Device<device::Bound>,
        chipset: Chipset,
        ver: &str,
    ) -> Result<Self> {
        let fw = super::request_firmware(dev, chipset, "fmc", ver)?;

        // FSP expects only the "image" section, not the entire ELF file.
        let fmc_image_data = elf::elf_section(fw.data(), "image").ok_or_else(|| {
            dev_err!(dev, "FMC ELF file missing 'image' section\n");
            EINVAL
        })?;
        let fmc_image = Coherent::from_slice(dev, fmc_image_data, GFP_KERNEL)?;

        Ok(Self {
            fmc_image,
            fmc_sigs: Self::extract_fmc_signatures(&fw, dev)?,
        })
    }

    /// Extract FMC firmware signatures for Chain of Trust verification.
    ///
    /// Extracts real cryptographic signatures from FMC ELF32 firmware sections.
    /// Returns signatures in a heap-allocated structure to prevent stack overflow.
    fn extract_fmc_signatures(
        fmc_fw: &Firmware,
        dev: &device::Device,
    ) -> Result<KBox<FmcSignatures>> {
        let hash_section = crate::firmware::elf_section(fmc_fw.data(), "hash")
            .ok_or(EINVAL)
            .inspect_err(|_| dev_err!(dev, "FMC firmware missing 'hash' section\n"))?;

        let pkey_section = crate::firmware::elf_section(fmc_fw.data(), "publickey")
            .ok_or(EINVAL)
            .inspect_err(|_| dev_err!(dev, "FMC firmware missing 'publickey' section\n"))?;

        let sig_section = crate::firmware::elf_section(fmc_fw.data(), "signature")
            .ok_or(EINVAL)
            .inspect_err(|_| dev_err!(dev, "FMC firmware missing 'signature' section\n"))?;

        if hash_section.len() != FSP_HASH_SIZE {
            dev_err!(
                dev,
                "FMC hash section size {} != expected {}\n",
                hash_section.len(),
                FSP_HASH_SIZE
            );
            return Err(EINVAL);
        }

        if pkey_section.len() > FSP_PKEY_SIZE {
            dev_err!(
                dev,
                "FMC publickey section size {} > maximum {}\n",
                pkey_section.len(),
                FSP_PKEY_SIZE
            );
            return Err(EINVAL);
        }

        if sig_section.len() > FSP_SIG_SIZE {
            dev_err!(
                dev,
                "FMC signature section size {} > maximum {}\n",
                sig_section.len(),
                FSP_SIG_SIZE
            );
            return Err(EINVAL);
        }

        let mut signatures = KBox::new(
            FmcSignatures {
                hash384: [0; _],
                public_key: [0; _],
                signature: [0; _],
            },
            GFP_KERNEL,
        )?;

        signatures.hash384.copy_from_slice(hash_section);
        signatures.public_key[..pkey_section.len()].copy_from_slice(pkey_section);
        signatures.signature[..sig_section.len()].copy_from_slice(sig_section);

        Ok(signatures)
    }
}
