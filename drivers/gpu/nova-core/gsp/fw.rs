// SPDX-License-Identifier: GPL-2.0

mod r570_144;

// Alias to avoid repeating the version number with every use.
use r570_144 as bindings;

use core::ops::Range;

use kernel::dma::CoherentAllocation;
use kernel::ptr::{Alignable, Alignment};
use kernel::sizes::{SZ_128K, SZ_1M};
use kernel::transmute::{AsBytes, FromBytes};

use crate::firmware::gsp::GspFirmware;
use crate::gpu::Chipset;
use crate::gsp;
use crate::gsp::cmdq::MSGQ_NUM_PAGES;
use crate::gsp::FbLayout;
use crate::gsp::GSP_PAGE_SIZE;

/// Dummy type to group methods related to heap parameters for running the GSP firmware.
pub(crate) struct GspFwHeapParams(());

impl GspFwHeapParams {
    /// Returns the amount of GSP-RM heap memory used during GSP-RM boot and initialization (up to
    /// and including the first client subdevice allocation).
    fn base_rm_size(_chipset: Chipset) -> u64 {
        // TODO: this needs to be updated to return the correct value for Hopper+ once support for
        // them is added:
        // u64::from(bindings::GSP_FW_HEAP_PARAM_BASE_RM_SIZE_GH100)
        u64::from(bindings::GSP_FW_HEAP_PARAM_BASE_RM_SIZE_TU10X)
    }

    /// Returns the amount of heap memory required to support a single channel allocation.
    fn client_alloc_size() -> u64 {
        u64::from(bindings::GSP_FW_HEAP_PARAM_CLIENT_ALLOC_SIZE)
            .align_up(gsp::GSP_HEAP_ALIGNMENT)
            .unwrap_or(u64::MAX)
    }

    /// Returns the amount of memory to reserve for management purposes for a framebuffer of size
    /// `fb_size`.
    fn management_overhead(fb_size: u64) -> u64 {
        let fb_size_gb = fb_size.div_ceil(kernel::sizes::SZ_1G as u64);

        u64::from(bindings::GSP_FW_HEAP_PARAM_SIZE_PER_GB_FB)
            .saturating_mul(fb_size_gb)
            .align_up(gsp::GSP_HEAP_ALIGNMENT)
            .unwrap_or(u64::MAX)
    }
}

/// Heap memory requirements and constraints for a given version of the GSP LIBOS.
pub(crate) struct LibosParams {
    /// The base amount of heap required by the GSP operating system, in bytes.
    carveout_size: u64,
    /// The minimum and maximum sizes allowed for the GSP FW heap, in bytes.
    allowed_heap_size: Range<u64>,
}

impl LibosParams {
    /// Version 2 of the GSP LIBOS (Turing and GA100)
    const LIBOS2: LibosParams = LibosParams {
        carveout_size: bindings::GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS2 as u64,
        allowed_heap_size: bindings::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MIN_MB as u64 * SZ_1M as u64
            ..bindings::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MAX_MB as u64 * SZ_1M as u64,
    };

    /// Version 3 of the GSP LIBOS (GA102+)
    const LIBOS3: LibosParams = LibosParams {
        carveout_size: bindings::GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS3_BAREMETAL as u64,
        allowed_heap_size: bindings::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MIN_MB as u64
            * SZ_1M as u64
            ..bindings::GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MAX_MB as u64 * SZ_1M as u64,
    };

    /// Returns the libos parameters corresponding to `chipset`.
    pub(crate) fn from_chipset(chipset: Chipset) -> &'static LibosParams {
        if chipset < Chipset::GA102 {
            &Self::LIBOS2
        } else {
            &Self::LIBOS3
        }
    }

    /// Returns the amount of memory (in bytes) to allocate for the WPR heap for a framebuffer size
    /// of `fb_size` (in bytes) for `chipset`.
    pub(crate) fn wpr_heap_size(&self, chipset: Chipset, fb_size: u64) -> u64 {
        // The WPR heap will contain the following:
        // LIBOS carveout,
        self.carveout_size
            // RM boot working memory,
            .saturating_add(GspFwHeapParams::base_rm_size(chipset))
            // One RM client,
            .saturating_add(GspFwHeapParams::client_alloc_size())
            // Overhead for memory management.
            .saturating_add(GspFwHeapParams::management_overhead(fb_size))
            // Clamp to the supported heap sizes.
            .clamp(self.allowed_heap_size.start, self.allowed_heap_size.end - 1)
    }
}

/// Structure passed to the GSP bootloader, containing the framebuffer layout as well as the DMA
/// addresses of the GSP bootloader and firmware.
#[repr(transparent)]
pub(crate) struct GspFwWprMeta(bindings::GspFwWprMeta);

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GspFwWprMeta {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GspFwWprMeta {}

type GspFwWprMetaBootResumeInfo = r570_144::GspFwWprMeta__bindgen_ty_1;
type GspFwWprMetaBootInfo = r570_144::GspFwWprMeta__bindgen_ty_1__bindgen_ty_1;

impl GspFwWprMeta {
    pub(crate) fn new(gsp_firmware: &GspFirmware, fb_layout: &FbLayout) -> Self {
        Self(bindings::GspFwWprMeta {
            magic: GSP_FW_WPR_META_MAGIC as u64,
            revision: u64::from(GSP_FW_WPR_META_REVISION),
            sysmemAddrOfRadix3Elf: gsp_firmware.radix3_dma_handle(),
            sizeOfRadix3Elf: gsp_firmware.size as u64,
            sysmemAddrOfBootloader: gsp_firmware.bootloader.ucode.dma_handle(),
            sizeOfBootloader: gsp_firmware.bootloader.ucode.size() as u64,
            bootloaderCodeOffset: u64::from(gsp_firmware.bootloader.code_offset),
            bootloaderDataOffset: u64::from(gsp_firmware.bootloader.data_offset),
            bootloaderManifestOffset: u64::from(gsp_firmware.bootloader.manifest_offset),
            __bindgen_anon_1: GspFwWprMetaBootResumeInfo {
                __bindgen_anon_1: GspFwWprMetaBootInfo {
                    sysmemAddrOfSignature: gsp_firmware.signatures.dma_handle(),
                    sizeOfSignature: gsp_firmware.signatures.size() as u64,
                },
            },
            gspFwRsvdStart: fb_layout.heap.start,
            nonWprHeapOffset: fb_layout.heap.start,
            nonWprHeapSize: fb_layout.heap.end - fb_layout.heap.start,
            gspFwWprStart: fb_layout.wpr2.start,
            gspFwHeapOffset: fb_layout.wpr2_heap.start,
            gspFwHeapSize: fb_layout.wpr2_heap.end - fb_layout.wpr2_heap.start,
            gspFwOffset: fb_layout.elf.start,
            bootBinOffset: fb_layout.boot.start,
            frtsOffset: fb_layout.frts.start,
            frtsSize: fb_layout.frts.end - fb_layout.frts.start,
            gspFwWprEnd: fb_layout
                .vga_workspace
                .start
                .align_down(Alignment::new::<SZ_128K>()),
            gspFwHeapVfPartitionCount: fb_layout.vf_partition_count,
            fbSize: fb_layout.fb.end - fb_layout.fb.start,
            vgaWorkspaceOffset: fb_layout.vga_workspace.start,
            vgaWorkspaceSize: fb_layout.vga_workspace.end - fb_layout.vga_workspace.start,
            ..Default::default()
        })
    }
}

pub(crate) use r570_144::{
    rpc_run_cpu_sequencer_v17_00,
    // Core GSP structures
    GspSystemInfo,

    GSP_ARGUMENTS_CACHED,

    // GSP firmware constants
    GSP_FW_WPR_META_MAGIC,
    GSP_FW_WPR_META_REVISION,

    // GSP sequencer structures
    GSP_SEQUENCER_BUFFER_CMD,
    GSP_SEQ_BUF_OPCODE,

    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_RESET,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_RESUME,

    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_START,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_CORE_WAIT_FOR_HALT,

    // GSP sequencer opcode constants
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_DELAY_US,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_MODIFY,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_POLL,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_STORE,
    GSP_SEQ_BUF_OPCODE_GSP_SEQ_BUF_OPCODE_REG_WRITE,

    // GSP sequencer payload structures
    GSP_SEQ_BUF_PAYLOAD_DELAY_US,
    GSP_SEQ_BUF_PAYLOAD_REG_MODIFY,
    GSP_SEQ_BUF_PAYLOAD_REG_POLL,
    GSP_SEQ_BUF_PAYLOAD_REG_STORE,
    GSP_SEQ_BUF_PAYLOAD_REG_WRITE,

    GSP_SR_INIT_ARGUMENTS,

    // RM message queue parameters
    MESSAGE_QUEUE_INIT_ARGUMENTS,

    // GSP events
    NV_VGPU_MSG_EVENT_GSP_INIT_DONE,
    NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE,
    NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD,
    NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER,
    NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED,
    NV_VGPU_MSG_EVENT_OS_ERROR_LOG,
    NV_VGPU_MSG_EVENT_POST_EVENT,
    NV_VGPU_MSG_EVENT_RC_TRIGGERED,
    NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT,

    // GSP function calls
    NV_VGPU_MSG_FUNCTION_ALLOC_CHANNEL_DMA,
    NV_VGPU_MSG_FUNCTION_ALLOC_CTX_DMA,
    NV_VGPU_MSG_FUNCTION_ALLOC_DEVICE,
    NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY,
    NV_VGPU_MSG_FUNCTION_ALLOC_OBJECT,
    NV_VGPU_MSG_FUNCTION_ALLOC_ROOT,
    NV_VGPU_MSG_FUNCTION_BIND_CTX_DMA,
    NV_VGPU_MSG_FUNCTION_FREE,
    NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO,
    NV_VGPU_MSG_FUNCTION_GET_STATIC_INFO,
    NV_VGPU_MSG_FUNCTION_GSP_INIT_POST_OBJGPU,
    NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL,
    NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO,
    NV_VGPU_MSG_FUNCTION_LOG,
    NV_VGPU_MSG_FUNCTION_MAP_MEMORY,
    NV_VGPU_MSG_FUNCTION_NOP,
    NV_VGPU_MSG_FUNCTION_SET_GUEST_SYSTEM_INFO,
    NV_VGPU_MSG_FUNCTION_SET_REGISTRY,

    // RM registry structures
    PACKED_REGISTRY_ENTRY,
    PACKED_REGISTRY_TABLE,
    REGISTRY_TABLE_ENTRY_TYPE_DWORD,
};

#[repr(transparent)]
pub(crate) struct LibosMemoryRegionInitArgument(bindings::LibosMemoryRegionInitArgument);

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for LibosMemoryRegionInitArgument {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for LibosMemoryRegionInitArgument {}

impl LibosMemoryRegionInitArgument {
    pub(crate) fn new<A: AsBytes + FromBytes>(
        name: &'static str,
        obj: &CoherentAllocation<A>,
    ) -> Self {
        /// Generates the `ID8` identifier required for some GSP objects.
        fn id8(name: &str) -> u64 {
            let mut bytes = [0u8; core::mem::size_of::<u64>()];

            for (c, b) in name.bytes().rev().zip(&mut bytes) {
                *b = c;
            }

            u64::from_ne_bytes(bytes)
        }

        Self(bindings::LibosMemoryRegionInitArgument {
            id8: id8(name),
            pa: obj.dma_handle(),
            size: obj.size() as u64,
            kind: bindings::LibosMemoryRegionKind_LIBOS_MEMORY_REGION_CONTIGUOUS as u8,
            loc: bindings::LibosMemoryRegionLoc_LIBOS_MEMORY_REGION_LOC_SYSMEM as u8,
            ..Default::default()
        })
    }
}

pub(crate) type MsgqTxHeader = bindings::msgqTxHeader;

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for MsgqTxHeader {}

impl MsgqTxHeader {
    pub(crate) fn new(msgq_size: u32, rx_hdr_offset: u32) -> Self {
        Self {
            version: 0,
            size: msgq_size,
            msgSize: GSP_PAGE_SIZE as u32,
            msgCount: MSGQ_NUM_PAGES,
            writePtr: 0,
            flags: 1,
            rxHdrOff: rx_hdr_offset,
            entryOff: GSP_PAGE_SIZE as u32,
        }
    }
}

/// RX header for setting up a message queue with the GSP.
///
/// # Invariants
///
/// [`Self::read_ptr`] is guaranteed to return a value in the range `0..NUM_PAGES`.
pub(crate) type MsgqRxHeader = bindings::msgqRxHeader;

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for MsgqRxHeader {}

impl MsgqRxHeader {
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

pub(crate) type GspRpcHeader = bindings::rpc_message_header_v;

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GspRpcHeader {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GspRpcHeader {}

impl GspRpcHeader {
    pub(crate) fn new(cmd_size: u32, function: u32) -> Self {
        Self {
            // TODO: magic number
            header_version: 0x03000000,
            signature: bindings::NV_VGPU_MSG_SIGNATURE_VALID,
            function,
            // TODO: overflow check?
            length: size_of::<Self>() as u32 + cmd_size,
            rpc_result: 0xffffffff,
            rpc_result_private: 0xffffffff,
            ..Default::default()
        }
    }
}

pub(crate) type GspMsgElement = bindings::GSP_MSG_QUEUE_ELEMENT;

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GspMsgElement {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GspMsgElement {}

impl GspMsgElement {
    pub(crate) fn new(sequence: u32, cmd_size: usize, function: u32) -> Self {
        Self {
            seqNum: sequence,
            // TODO: overflow check and fallible div?
            elemCount: (size_of::<Self>() + cmd_size).div_ceil(GSP_PAGE_SIZE) as u32,
            // TODO: fallible conversion.
            rpc: GspRpcHeader::new(cmd_size as u32, function),
            ..Default::default()
        }
    }
}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GSP_ARGUMENTS_CACHED {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GSP_ARGUMENTS_CACHED {}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for MESSAGE_QUEUE_INIT_ARGUMENTS {}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GSP_SR_INIT_ARGUMENTS {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for PACKED_REGISTRY_TABLE {}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for PACKED_REGISTRY_TABLE {}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for PACKED_REGISTRY_ENTRY {}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for r570_144::rpc_run_cpu_sequencer_v17_00 {}
