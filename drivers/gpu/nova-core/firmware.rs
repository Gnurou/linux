// SPDX-License-Identifier: GPL-2.0

//! Contains structures and functions dedicated to the parsing, building and patching of firmwares
//! to be loaded into a given execution unit.

use core::marker::PhantomData;
use core::mem::size_of;

use kernel::device;
use kernel::firmware;
use kernel::prelude::*;
use kernel::str::CString;
use kernel::transmute::FromBytes;

use crate::dma::DmaObject;
use crate::falcon::FalconFirmware;
use crate::gpu;

pub(crate) mod booter;
pub(crate) mod fsp;
pub(crate) mod fwsec;
pub(crate) mod gsp;
pub(crate) mod riscv;

pub(crate) const FIRMWARE_VERSION: &str = "570.144";

pub(crate) fn elf_section<'a, 'b>(elf: &'a [u8], section_name: &'b str) -> Option<&'a [u8]> {
    // Check ELF magic
    if elf.len() < 5 || &elf[0..4] != b"\x7fELF" {
        return None;
    }

    let class = elf[4];
    match class {
        1 => {
            // ELF32
            elf::elf32_section(elf, section_name)
        }
        2 => {
            // ELF64
            elf::elf64_section(elf, section_name)
        }
        _ => None,
    }
}

/// Ad-hoc and temporary module to extract sections from ELF images.
///
/// Some firmware images are currently packaged as ELF files, where sections names are used as keys
/// to specific and related bits of data. Future firmware versions are scheduled to move away from
/// that scheme before nova-core becomes stable, which means this module will eventually be
/// removed.
mod elf {
    use core::mem::{align_of, size_of};
    use kernel::bindings;
    use kernel::str::CStr;
    use kernel::transmute::FromBytes;

    /// Newtype to provide a [`FromBytes`] implementation.
    #[repr(transparent)]
    struct Elf32Hdr(bindings::elf32_hdr);

    // SAFETY: all bit patterns are valid for this type, and it doesn't use interior mutability.
    unsafe impl FromBytes for Elf32Hdr {}

    /// Newtype to provide a [`FromBytes`] implementation.
    #[repr(transparent)]
    struct Elf64Hdr(bindings::elf64_hdr);

    // SAFETY: all bit patterns are valid for this type, and it doesn't use interior mutability.
    unsafe impl FromBytes for Elf64Hdr {}

    /// Minimal trait to abstract over ELF header differences.
    trait ElfHeader {
        fn shnum(&self) -> u16;
        fn shoff(&self) -> u64;
        fn shstrndx(&self) -> u16;
    }

    impl ElfHeader for bindings::elf32_hdr {
        fn shnum(&self) -> u16 {
            self.e_shnum
        }

        fn shoff(&self) -> u64 {
            u64::from(self.e_shoff)
        }

        fn shstrndx(&self) -> u16 {
            self.e_shstrndx
        }
    }

    impl ElfHeader for bindings::elf64_hdr {
        fn shnum(&self) -> u16 {
            self.e_shnum
        }

        fn shoff(&self) -> u64 {
            self.e_shoff
        }

        fn shstrndx(&self) -> u16 {
            self.e_shstrndx
        }
    }

    /// Minimal trait to abstract over ELF section header differences.
    trait ElfSectionHeader {
        fn name(&self) -> u32;
        fn offset(&self) -> u64;
        fn size(&self) -> u64;
    }

    impl ElfSectionHeader for bindings::elf32_shdr {
        fn name(&self) -> u32 {
            self.sh_name
        }

        fn offset(&self) -> u64 {
            u64::from(self.sh_offset)
        }

        fn size(&self) -> u64 {
            u64::from(self.sh_size)
        }
    }

    impl ElfSectionHeader for bindings::elf64_shdr {
        fn name(&self) -> u32 {
            self.sh_name
        }

        fn offset(&self) -> u64 {
            self.sh_offset
        }

        fn size(&self) -> u64 {
            self.sh_size
        }
    }

    /// Generic implementation for extracting a section from an ELF image.
    fn elf_section_generic<'a, H, S>(
        elf: &'a [u8],
        name: &str,
        header_size: usize,
        shdr_size: usize,
        parse_header: impl Fn(&[u8]) -> Option<&H>,
    ) -> Option<&'a [u8]>
    where
        H: ElfHeader,
        S: ElfSectionHeader,
    {
        let hdr = parse_header(elf.get(0..header_size)?)?;

        let shdr_num = usize::from(hdr.shnum());
        let shdr_start = usize::try_from(hdr.shoff()).ok()?;
        let shdr_end = shdr_num
            .checked_mul(shdr_size)
            .and_then(|v| v.checked_add(shdr_start))?;

        // Get all the section headers.
        let shdr = elf
            .get(shdr_start..shdr_end)
            .map(|slice| slice.as_ptr())
            .filter(|ptr| ptr.align_offset(align_of::<S>()) == 0)
            // `FromBytes::from_bytes` does not support slices yet, so build it manually.
            //
            // SAFETY:
            // * `get` guarantees that the slice is within the bounds of `elf` and of size
            //   `shdr_size * shdr_num`.
            // * We checked that `ptr` had the correct alignment for `S`.
            .map(|ptr| unsafe { core::slice::from_raw_parts(ptr.cast::<S>(), shdr_num) })?;

        // Get the strings table.
        let strhdr = shdr.get(usize::from(hdr.shstrndx()))?;

        // Find the section which name matches `name` and return it.
        shdr.iter()
            .find(|sh| {
                let Some(name_idx) = strhdr
                    .offset()
                    .checked_add(u64::from(sh.name()))
                    .and_then(|idx| usize::try_from(idx).ok())
                else {
                    return false;
                };

                // Get the start of the name.
                elf.get(name_idx..)
                    // Stop at the first `0`.
                    .and_then(|nstr| nstr.get(0..=nstr.iter().position(|b| *b == 0)?))
                    // Convert into CStr. This should never fail because of the line above.
                    .and_then(|nstr| CStr::from_bytes_with_nul(nstr).ok())
                    // Convert into str.
                    .and_then(|c_str| c_str.to_str().ok())
                    // Check that the name matches.
                    .map(|str| str == name)
                    .unwrap_or(false)
            })
            // Return the slice containing the section.
            .and_then(|sh| {
                let start = usize::try_from(sh.offset()).ok()?;
                let end = usize::try_from(sh.size())
                    .ok()
                    .and_then(|sh_size| start.checked_add(sh_size))?;

                elf.get(start..end)
            })
    }

    /// Tries to extract section with name `name` from the ELF32 image `elf`, and returns it.
    pub(super) fn elf32_section<'a, 'b>(elf: &'a [u8], name: &'b str) -> Option<&'a [u8]> {
        elf_section_generic::<bindings::elf32_hdr, bindings::elf32_shdr>(
            elf,
            name,
            size_of::<bindings::elf32_hdr>(),
            size_of::<bindings::elf32_shdr>(),
            |data| Elf32Hdr::from_bytes(data).map(|h| &h.0),
        )
    }

    /// Tries to extract section with name `name` from the ELF64 image `elf`, and returns it.
    pub(super) fn elf64_section<'a, 'b>(elf: &'a [u8], name: &'b str) -> Option<&'a [u8]> {
        elf_section_generic::<bindings::elf64_hdr, bindings::elf64_shdr>(
            elf,
            name,
            size_of::<bindings::elf64_hdr>(),
            size_of::<bindings::elf64_shdr>(),
            |data| Elf64Hdr::from_bytes(data).map(|h| &h.0),
        )
    }
}

/// Requests the GPU firmware `name` suitable for `chipset`, with version `ver`.
fn request_firmware(
    dev: &device::Device,
    chipset: gpu::Chipset,
    name: &str,
    ver: &str,
) -> Result<firmware::Firmware> {
    let chip_name = chipset.name();

    CString::try_from_fmt(fmt!("nvidia/{chip_name}/gsp/{name}-{ver}.bin"))
        .and_then(|path| firmware::Firmware::request(&path, dev))
}

/// Structure used to describe some firmwares, notably FWSEC-FRTS.
#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct FalconUCodeDescV2 {
    /// Header defined by 'NV_BIT_FALCON_UCODE_DESC_HEADER_VDESC*' in OpenRM.
    hdr: u32,
    /// Stored size of the ucode after the header, compressed or uncompressed
    stored_size: u32,
    /// Uncompressed size of the ucode.  If store_size == uncompressed_size, then the ucode
    /// is not compressed.
    pub(crate) uncompressed_size: u32,
    /// Code entry point
    pub(crate) virtual_entry: u32,
    /// Offset after the code segment at which the Application Interface Table headers are located.
    pub(crate) interface_offset: u32,
    /// Base address at which to load the code segment into 'IMEM'.
    pub(crate) imem_phys_base: u32,
    /// Size in bytes of the code to copy into 'IMEM'.
    pub(crate) imem_load_size: u32,
    /// Virtual 'IMEM' address (i.e. 'tag') at which the code should start.
    pub(crate) imem_virt_base: u32,
    /// Virtual address of secure IMEM segment.
    pub(crate) imem_sec_base: u32,
    /// Size of secure IMEM segment.
    pub(crate) imem_sec_size: u32,
    /// Offset into stored (uncompressed) image at which DMEM begins.
    pub(crate) dmem_offset: u32,
    /// Base address at which to load the data segment into 'DMEM'.
    pub(crate) dmem_phys_base: u32,
    /// Size in bytes of the data to copy into 'DMEM'.
    pub(crate) dmem_load_size: u32,
    /// "Alternate" Size of data to load into IMEM.
    pub(crate) alt_imem_load_size: u32,
    /// "Alternate" Size of data to load into DMEM.
    pub(crate) alt_dmem_load_size: u32,
}

/// Structure used to describe some firmwares, notably FWSEC-FRTS.
#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct FalconUCodeDescV3 {
    /// Header defined by `NV_BIT_FALCON_UCODE_DESC_HEADER_VDESC*` in OpenRM.
    hdr: u32,
    /// Stored size of the ucode after the header.
    stored_size: u32,
    /// Offset in `DMEM` at which the signature is expected to be found.
    pub(crate) pkc_data_offset: u32,
    /// Offset after the code segment at which the app headers are located.
    pub(crate) interface_offset: u32,
    /// Base address at which to load the code segment into `IMEM`.
    pub(crate) imem_phys_base: u32,
    /// Size in bytes of the code to copy into `IMEM`.
    pub(crate) imem_load_size: u32,
    /// Virtual `IMEM` address (i.e. `tag`) at which the code should start.
    pub(crate) imem_virt_base: u32,
    /// Base address at which to load the data segment into `DMEM`.
    pub(crate) dmem_phys_base: u32,
    /// Size in bytes of the data to copy into `DMEM`.
    pub(crate) dmem_load_size: u32,
    /// Mask of the falcon engines on which this firmware can run.
    pub(crate) engine_id_mask: u16,
    /// ID of the ucode used to infer a fuse register to validate the signature.
    pub(crate) ucode_id: u8,
    /// Number of signatures in this firmware.
    pub(crate) signature_count: u8,
    /// Versions of the signatures, used to infer a valid signature to use.
    pub(crate) signature_versions: u16,
    _reserved: u16,
}

#[derive(Debug, Clone)]
pub(crate) enum FalconUCodeDesc {
    V2(FalconUCodeDescV2),
    V3(FalconUCodeDescV3),
}

impl FalconUCodeDesc {
    /// Returns the size in bytes of the header.
    pub(crate) fn size(&self) -> usize {
        let hdr = match self {
            FalconUCodeDesc::V2(v2) => v2.hdr,
            FalconUCodeDesc::V3(v3) => v3.hdr,
        };

        const HDR_SIZE_SHIFT: u32 = 16;
        const HDR_SIZE_MASK: u32 = 0xffff0000;
        ((hdr & HDR_SIZE_MASK) >> HDR_SIZE_SHIFT) as usize
    }

    pub(crate) fn imem_load_size(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(v2) => v2.imem_load_size,
            FalconUCodeDesc::V3(v3) => v3.imem_load_size,
        }
    }

    pub(crate) fn interface_offset(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(v2) => v2.interface_offset,
            FalconUCodeDesc::V3(v3) => v3.interface_offset,
        }
    }

    pub(crate) fn dmem_load_size(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(v2) => v2.dmem_load_size,
            FalconUCodeDesc::V3(v3) => v3.dmem_load_size,
        }
    }

    pub(crate) fn pkc_data_offset(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(_v2) => 0,
            FalconUCodeDesc::V3(v3) => v3.pkc_data_offset,
        }
    }

    pub(crate) fn engine_id_mask(&self) -> u16 {
        match self {
            FalconUCodeDesc::V2(_v2) => 0,
            FalconUCodeDesc::V3(v3) => v3.engine_id_mask,
        }
    }

    pub(crate) fn ucode_id(&self) -> u8 {
        match self {
            FalconUCodeDesc::V2(_v2) => 0,
            FalconUCodeDesc::V3(v3) => v3.ucode_id,
        }
    }

    pub(crate) fn signature_count(&self) -> u8 {
        match self {
            FalconUCodeDesc::V2(_v2) => 0,
            FalconUCodeDesc::V3(v3) => v3.signature_count,
        }
    }

    pub(crate) fn signature_versions(&self) -> u16 {
        match self {
            FalconUCodeDesc::V2(_v2) => 0,
            FalconUCodeDesc::V3(v3) => v3.signature_versions,
        }
    }

    pub(crate) fn imem_phys_base(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(v2) => v2.imem_phys_base,
            FalconUCodeDesc::V3(v3) => v3.imem_phys_base,
        }
    }

    pub(crate) fn dmem_phys_base(&self) -> u32 {
        match self {
            FalconUCodeDesc::V2(v2) => v2.dmem_phys_base,
            FalconUCodeDesc::V3(v3) => v3.dmem_phys_base,
        }
    }
}

/// Trait implemented by types defining the signed state of a firmware.
trait SignedState {}

/// Type indicating that the firmware must be signed before it can be used.
struct Unsigned;
impl SignedState for Unsigned {}

/// Type indicating that the firmware is signed and ready to be loaded.
struct Signed;
impl SignedState for Signed {}

/// A [`DmaObject`] containing a specific microcode ready to be loaded into a falcon.
///
/// This is module-local and meant for sub-modules to use internally.
///
/// After construction, a firmware is [`Unsigned`], and must generally be patched with a signature
/// before it can be loaded (with an exception for development hardware). The
/// [`Self::patch_signature`] and [`Self::no_patch_signature`] methods are used to transition the
/// firmware to its [`Signed`] state.
struct FirmwareDmaObject<F: FalconFirmware, S: SignedState>(DmaObject, PhantomData<(F, S)>);

/// Trait for signatures to be patched directly into a given firmware.
///
/// This is module-local and meant for sub-modules to use internally.
trait FirmwareSignature<F: FalconFirmware>: AsRef<[u8]> {}

impl<F: FalconFirmware> FirmwareDmaObject<F, Unsigned> {
    /// Patches the firmware at offset `sig_base_img` with `signature`.
    fn patch_signature<S: FirmwareSignature<F>>(
        mut self,
        signature: &S,
        sig_base_img: usize,
    ) -> Result<FirmwareDmaObject<F, Signed>> {
        let signature_bytes = signature.as_ref();
        if sig_base_img + signature_bytes.len() > self.0.size() {
            return Err(EINVAL);
        }

        // SAFETY: We are the only user of this object, so there cannot be any race.
        let dst = unsafe { self.0.start_ptr_mut().add(sig_base_img) };

        // SAFETY: `signature` and `dst` are valid, properly aligned, and do not overlap.
        unsafe {
            core::ptr::copy_nonoverlapping(signature_bytes.as_ptr(), dst, signature_bytes.len())
        };

        Ok(FirmwareDmaObject(self.0, PhantomData))
    }

    /// Mark the firmware as signed without patching it.
    ///
    /// This method is used to explicitly confirm that we do not need to sign the firmware, while
    /// allowing us to continue as if it was. This is typically only needed for development
    /// hardware.
    fn no_patch_signature(self) -> FirmwareDmaObject<F, Signed> {
        FirmwareDmaObject(self.0, PhantomData)
    }
}

/// Header common to most firmware files.
#[repr(C)]
#[derive(Debug, Clone)]
struct BinHdr {
    /// Magic number, must be `0x10de`.
    bin_magic: u32,
    /// Version of the header.
    bin_ver: u32,
    /// Size in bytes of the binary (to be ignored).
    bin_size: u32,
    /// Offset of the start of the application-specific header.
    header_offset: u32,
    /// Offset of the start of the data payload.
    data_offset: u32,
    /// Size in bytes of the data payload.
    data_size: u32,
}

// SAFETY: all bit patterns are valid for this type, and it doesn't use interior mutability.
unsafe impl FromBytes for BinHdr {}

// A firmware blob starting with a `BinHdr`.
struct BinFirmware<'a> {
    hdr: BinHdr,
    fw: &'a [u8],
}

impl<'a> BinFirmware<'a> {
    /// Interpret `fw` as a firmware image starting with a [`BinHdr`], and returns the
    /// corresponding [`BinFirmware`] that can be used to extract its payload.
    fn new(fw: &'a firmware::Firmware) -> Result<Self> {
        const BIN_MAGIC: u32 = 0x10de;
        let fw = fw.data();

        fw.get(0..size_of::<BinHdr>())
            // Extract header.
            .and_then(BinHdr::from_bytes_copy)
            // Validate header.
            .and_then(|hdr| {
                if hdr.bin_magic == BIN_MAGIC {
                    Some(hdr)
                } else {
                    None
                }
            })
            .map(|hdr| Self { hdr, fw })
            .ok_or(EINVAL)
    }

    /// Returns the data payload of the firmware, or `None` if the data range is out of bounds of
    /// the firmware image.
    fn data(&self) -> Option<&[u8]> {
        let fw_start = self.hdr.data_offset as usize;
        let fw_size = self.hdr.data_size as usize;

        self.fw.get(fw_start..fw_start + fw_size)
    }
}

pub(crate) struct ModInfoBuilder<const N: usize>(firmware::ModInfoBuilder<N>);

impl<const N: usize> ModInfoBuilder<N> {
    const fn make_entry_file(self, chipset: &str, fw: &str) -> Self {
        ModInfoBuilder(
            self.0
                .new_entry()
                .push("nvidia/")
                .push(chipset)
                .push("/gsp/")
                .push(fw)
                .push("-")
                .push(FIRMWARE_VERSION)
                .push(".bin"),
        )
    }

    const fn make_entry_chipset(self, chipset: &str) -> Self {
        self.make_entry_file(chipset, "booter_load")
            .make_entry_file(chipset, "booter_unload")
            .make_entry_file(chipset, "bootloader")
            .make_entry_file(chipset, "gsp")
    }

    pub(crate) const fn create(
        module_name: &'static kernel::str::CStr,
    ) -> firmware::ModInfoBuilder<N> {
        let mut this = Self(firmware::ModInfoBuilder::new(module_name));
        let mut i = 0;

        while i < gpu::Chipset::ALL.len() {
            this = this.make_entry_chipset(gpu::Chipset::ALL[i].name());
            i += 1;
        }

        this.0
    }
}
