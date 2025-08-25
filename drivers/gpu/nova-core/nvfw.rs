// SPDX-License-Identifier: GPL-2.0

mod r570_144;

use core::ops::Range;

use kernel::sizes::SZ_1M;

/// Heap memory requirements and constraints for a given version of the GSP LIBOS.
pub(crate) struct LibosParams {
    /// The base amount of heap required by the GSP operating system, in bytes.
    pub(crate) carveout_size: u64,
    /// The minimum and maximum sizes allowed for the GSP FW heap, in bytes.
    pub(crate) allowed_heap_size: Range<u64>,
}

/// Version 2 of the GSP LIBOS (Turing and GA100)
pub(crate) const LIBOS2_PARAMS: LibosParams = LibosParams {
    carveout_size: r570_144::GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS2 as u64,
    allowed_heap_size: r570_144::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MIN_MB as u64 * SZ_1M as u64
        ..r570_144::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MAX_MB as u64 * SZ_1M as u64,
};

/// Version 3 of the GSP LIBOS (GA102+)
pub(crate) const LIBOS3_PARAMS: LibosParams = LibosParams {
    carveout_size: r570_144::GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS3_BAREMETAL as u64,
    allowed_heap_size: r570_144::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MIN_MB as u64
        * SZ_1M as u64
        ..r570_144::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MAX_MB as u64 * SZ_1M as u64,
};

/// Amount of GSP-RM heap memory used during GSP-RM boot and initialization (up to and including
/// the first client subdevice allocation) on Turing/Ampere/Ada.
pub(crate) use r570_144::GSP_FW_HEAP_PARAM_BASE_RM_SIZE_TU10X;
/// WPR heap usage of a single client channel allocation.
pub(crate) use r570_144::GSP_FW_HEAP_PARAM_CLIENT_ALLOC_SIZE;
/// Amount of extra WPR heap to reserve per GB of framebuffer memory, in bytes.
pub(crate) use r570_144::GSP_FW_HEAP_PARAM_SIZE_PER_GB_FB;

/// Structure passed to the GSP bootloader, containing the framebuffer layout as well as the DMA
/// addresses of the GSP bootloader and firmware.
pub(crate) use r570_144::GspFwWprMeta;
