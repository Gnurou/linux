// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright (c) 2025-2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

use core::ops::Range;

use kernel::{
    alloc::ArrayVec,
    bitfield,
    device,
    pci,
    prelude::*,
    transmute::{
        AsBytes,
        FromBytes, //
    }, //
};

use crate::{
    gpu::Chipset,
    gsp::{
        commands::GspStaticInfo,
        nvkv::{
            nvkv_decode,
            nvkv_encode,
            Accumulated,
            Encodable,
            Encoder,
            Key,
            Required,
            TruncatingArray, //
        },
    },
    vgpu::VgpuState, //
};

use super::bindings;

/// Power level that a `GSP_SUSPEND` request names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[expect(unused)]
pub(crate) enum PowerStateLevel {
    /// Full unload.
    Level0 = bindings::NV2080_CTRL_GPU_SET_POWER_STATE_GPU_LEVEL_0,
    /// S3 (suspend to RAM).
    Level3 = bindings::NV2080_CTRL_GPU_SET_POWER_STATE_GPU_LEVEL_3,
    /// Hibernate (suspend to disk).
    Level7 = bindings::NV2080_CTRL_GPU_SET_POWER_STATE_GPU_LEVEL_7,
}

impl PowerStateLevel {
    /// Returns `true` if this state represents a power management transition, i.e. some GPU state
    /// must survive it (as opposed to a full unload).
    pub(crate) fn is_power_transition(self) -> bool {
        self != PowerStateLevel::Level0
    }
}

/// Set in [`GspSuspend::flags`] when the suspend is a power management transition (S3 or
/// hibernate) rather than a full unload.
const GMCAPI_GSP_SUSPEND_FLAGS_PM_TRANSITION: u64 = 1 << 0;

/// Payload of the `GSP_SUSPEND` GMC command.
#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable)]
pub(crate) struct GspSuspend {
    flags: u64,
}

impl GspSuspend {
    /// Creates a `GSP_SUSPEND` payload for the given [`PowerStateLevel`].
    pub(crate) fn new(level: PowerStateLevel) -> Self {
        Self {
            flags: if level.is_power_transition() {
                GMCAPI_GSP_SUSPEND_FLAGS_PM_TRANSITION
            } else {
                0
            },
        }
    }
}

// SAFETY: The single field is an integer type, and the struct has no padding.
unsafe impl AsBytes for GspSuspend {}

// SAFETY: The single field is an integer type for which all bit patterns are valid.
unsafe impl FromBytes for GspSuspend {}

/// The host CPU architecture.
#[derive(Clone, Copy)]
enum HostArch {
    None = 0,
    X86_64 = 1,
    Ppc64le = 2,
    Arm = 3,
    Aarch64 = 4,
    Riscv64 = 5,
}

// TODO[FPRI]: This is a temporary solution to be replaced with the corresponding derive macros once
// they land.
impl TryFrom<u32> for HostArch {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::X86_64),
            2 => Ok(Self::Ppc64le),
            3 => Ok(Self::Arm),
            4 => Ok(Self::Aarch64),
            5 => Ok(Self::Riscv64),
            _ => Err(EINVAL),
        }
    }
}

impl From<HostArch> for u32 {
    fn from(value: HostArch) -> Self {
        value as u32
    }
}

impl HostArch {
    /// Returns the variant that names the architecture for which this kernel is built.
    fn host() -> Self {
        if cfg!(target_arch = "x86_64") {
            Self::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Self::Aarch64
        } else if cfg!(target_arch = "powerpc64") {
            Self::Ppc64le
        } else if cfg!(target_arch = "arm") {
            Self::Arm
        } else if cfg!(target_arch = "riscv64") {
            Self::Riscv64
        } else {
            Self::None
        }
    }
}

nvkv_encode! {
    /// A GSP registry entry.
    struct RegKey {
        key_name: Key<&'static [u8], 0x3070>,
        key_value: Key<u32, 0x3071>,
    }
}

impl RegKey {
    /// Creates a registry entry.
    ///
    /// `key_name` must include its NUL terminator, which GSP-RM counts in the encoded name
    /// length.
    fn new(key_name: &'static [u8], key_value: u32) -> Self {
        Self {
            key_name: key_name.into(),
            key_value: key_value.into(),
        }
    }
}

impl Encodable for KVVec<RegKey> {
    fn encode(&self, encoder: &mut Encoder) -> Result {
        for regkey in self {
            regkey.encode(encoder)?;
        }
        Ok(())
    }
}

nvkv_encode! {
    /// SR-IOV virtual function information.
    struct VfInfo {
        total_vfs: Key<u32, 0x0080>,
        first_vf_offset: Key<u32, 0x0081>,
        flags: Key<u64, 0x1003>,
        first_bar0_address: Key<u64, 0x1050>,
        first_bar1_address: Key<u64, 0x1051>,
        first_bar2_address: Key<u64, 0x1052>,
    }
}

nvkv_encode! {
    /// Payload of the `GSP_INIT` command.
    pub(crate) struct GspInitRequest {
        pci_device_id: Key<u32, 0x0001>,
        pci_sub_device_id: Key<u32, 0x0002>,
        pci_revision_id: Key<u32, 0x0003>,
        pci_config_mirror_base: Key<u32, 0x0010>,
        pci_config_mirror_size: Key<u32, 0x0011>,
        host_arch: Key<HostArch, 0x0070, u32>,
        bus_device_func: Key<u64, 0x1020>,
        regkeys: KVVec<RegKey>,
        vf_info: Option<VfInfo>,
    }
}

bitfield! {
    /// A PCI device id as `PCI_DEVID` packs it: the bus number and the device and function
    /// numbers.
    struct PciDevId(u16) {
        15:8 bus;
        7:3 device;
        2:0 function;
    }
}

bitfield! {
    /// A GPU's PCI location, encoded as GSP-RM decodes it (`gpuEncodeDomainBusDevice` in OpenRM).
    /// Despite the name that GSP-RM gives the key, the function number is not part of the value.
    struct DomainBusDevice(u64) {
        63:32 domain;
        15:8 bus;
        7:0 device;
    }
}

/// Registry entries that the driver sends to GSP-RM on every boot.
///
/// `RMSecBusResetEnable` enables PCI secondary bus reset. `RMForcePcieConfigSave` makes GSP-RM
/// preserve PCI configuration registers across any PCI reset. `RMDevidCheckIgnore` lets GSP-RM
/// boot when the PCI device id is absent from its product name database.
const REGISTRY_ENTRIES: &[(&[u8], u32)] = &[
    (b"RMSecBusResetEnable\0", 1),
    (b"RMForcePcieConfigSave\0", 1),
    (b"RMDevidCheckIgnore\0", 1),
];

impl GspInitRequest {
    /// Creates the request for `dev`.
    ///
    /// The registry keys are [`REGISTRY_ENTRIES`], plus `RMSetSriovMode` when `vgpu_state` reports
    /// that vGPU is enabled.
    ///
    /// # Errors
    ///
    /// - `ENOMEM` if the registry list cannot be allocated.
    pub(crate) fn new(
        dev: &pci::Device<device::Bound>,
        chipset: Chipset,
        vgpu_state: VgpuState,
    ) -> Result<Self> {
        let mut regkeys = KVVec::new();
        for &(name, value) in REGISTRY_ENTRIES {
            regkeys.push(RegKey::new(name, value), GFP_KERNEL)?;
        }
        if matches!(vgpu_state, VgpuState::Enabled { .. }) {
            regkeys.push(RegKey::new(b"RMSetSriovMode\0", 1), GFP_KERNEL)?;
        }

        let mirror = chipset.pci_config_mirror_range();
        let dev_id = PciDevId::from(dev.dev_id());
        let bus_device_func = DomainBusDevice::zeroed()
            .with_domain(dev.domain_nr())
            .with_bus(u8::from(dev_id.bus()))
            .with_device(u8::from(dev_id.device()));
        let device_id = (u32::from(dev.device_id()) << 16) | u32::from(dev.vendor_id().as_raw());
        let sub_device_id =
            (u32::from(dev.subsystem_device_id()) << 16) | u32::from(dev.subsystem_vendor_id());

        Ok(Self {
            pci_device_id: device_id.into(),
            pci_sub_device_id: sub_device_id.into(),
            pci_revision_id: u32::from(dev.revision_id()).into(),
            pci_config_mirror_base: mirror.start.into(),
            pci_config_mirror_size: (mirror.end - mirror.start).into(),
            host_arch: HostArch::host().into(),
            bus_device_func: u64::from(bus_device_func).into(),
            regkeys,
            vf_info: None,
        })
    }
}

// Decode:

// Should decode with UnknownKeyPolicy::Ignore.
nvkv_decode! {
    /// Schema for the `GSP_INIT` response.
    pub(crate) struct GspInitResponseSchema => GspInitResponse {
        gpu_name: TruncatingArray<u8, { GspStaticInfo::MAX_GPU_NAME_LEN }, 0x2000>,
        gpu_short_name: TruncatingArray<u8, { GspStaticInfo::MAX_GPU_NAME_LEN }, 0x2001>,
        gpu_gid: TruncatingArray<u8, { GspStaticInfo::GID_LEN }, 0x2002>,
        fb_regions: Accumulated<FbRegionSchema>,
        bar1_pde_base: Required<Key<u64, 0x1020>>,
        vmmu_segment_size: Key<u64, 0x1050>,
    }
}

/// Payload of the `GSP_INIT` response.
pub(crate) struct GspInitResponse {
    gpu_name: ArrayVec<u8, { GspStaticInfo::MAX_GPU_NAME_LEN }>,
    gpu_short_name: ArrayVec<u8, { GspStaticInfo::MAX_GPU_NAME_LEN }>,
    gpu_gid: ArrayVec<u8, { GspStaticInfo::GID_LEN }>,
    fb_regions: KVVec<FbRegion>,
    bar1_pde_base: u64,
    vmmu_segment_size: u64,
}

impl GspInitResponse {
    /// Tag of a general-purpose region. Any other tag marks a region that GSP-RM reserves for the
    /// use that the tag names.
    const FB_REGION_TAG_NONE: u32 = 0;

    /// Returns the GPU name, which GSP-RM sends with its NUL terminator.
    pub(crate) fn gpu_name(&self) -> &[u8] {
        self.gpu_name.as_slice()
    }

    /// Returns the short GPU name, which GSP-RM sends with its NUL terminator.
    pub(crate) fn gpu_short_name(&self) -> &[u8] {
        self.gpu_short_name.as_slice()
    }

    /// Returns the SHA-1 based GPU identifier (GID).
    pub(crate) fn gpu_gid(&self) -> &[u8] {
        self.gpu_gid.as_slice()
    }

    /// Returns an iterator over the FB regions from which the driver may allocate: the
    /// general-purpose regions that are not protected and that support both compression and
    /// isochronous access.
    pub(crate) fn usable_fb_regions(&self) -> impl Iterator<Item = Range<u64>> + '_ {
        self.fb_regions.iter().filter_map(|region| {
            if region.limit >= region.base
                && region.tag == Self::FB_REGION_TAG_NONE
                && !region.flags.protected()
                && region.flags.support_compressed()
                && region.flags.support_iso()
            {
                region.limit.checked_add(1).map(|end| region.base..end)
            } else {
                None
            }
        })
    }

    /// Returns the exclusive end of the FB physical address space, which spans every region
    /// including the ones that [`Self::usable_fb_regions`] leaves out.
    ///
    /// Returns `None` if no region that GSP-RM reported has a limit at or above its base.
    pub(crate) fn total_fb_end(&self) -> Option<u64> {
        self.fb_regions
            .iter()
            .filter(|region| region.limit >= region.base)
            .map(|region| region.limit)
            .max()?
            .checked_add(1)
    }

    /// Returns the BAR1 page directory entry base address.
    pub(crate) fn bar1_pde_base(&self) -> u64 {
        self.bar1_pde_base
    }
}

nvkv_decode! {
    /// Schema for one FB region of the `GSP_INIT` response.
    struct FbRegionSchema => FbRegion {
        base: Required<Key<u64, 0x1011>>,
        limit: Required<Key<u64, 0x1012>>,
        flags: Required<Key<FbRegionFlags, 0x0012, u32>>,
        tag: Required<Key<u32, 0x0013>>,
    }
}

bitfield! {
    /// FB region attribute flags.
    #[derive(Default)]
    struct FbRegionFlags(u32) {
        0:0 support_compressed => bool;
        1:1 support_iso => bool;
        2:2 protected => bool;
    }
}

/// One FB memory region.
struct FbRegion {
    base: u64,
    limit: u64,
    flags: FbRegionFlags,
    tag: u32,
}
