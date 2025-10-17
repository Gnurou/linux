// SPDX-License-Identifier: GPL-2.0

mod r570_144;

// Alias to avoid repeating the version number with every use.
use r570_144 as bindings;

use core::fmt;
use core::ops::Range;

use kernel::dma::CoherentAllocation;
use kernel::prelude::*;
use kernel::ptr::{Alignable, Alignment};
use kernel::sizes::{SZ_128K, SZ_1M};
use kernel::transmute::{AsBytes, FromBytes};

use crate::firmware::gsp::GspFirmware;
use crate::gpu::Chipset;
use crate::gsp::cmdq::Cmdq;
use crate::gsp::FbLayout;
use crate::gsp::GSP_PAGE_SIZE;

/// Dummy type to group methods related to heap parameters for running the GSP firmware.
pub(crate) struct GspFwHeapParams(());

/// Minimum required alignment for the GSP heap.
const GSP_HEAP_ALIGNMENT: Alignment = Alignment::new::<{ 1 << 20 }>();

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
            .align_up(GSP_HEAP_ALIGNMENT)
            .unwrap_or(u64::MAX)
    }

    /// Returns the amount of memory to reserve for management purposes for a framebuffer of size
    /// `fb_size`.
    fn management_overhead(fb_size: u64) -> u64 {
        let fb_size_gb = fb_size.div_ceil(kernel::sizes::SZ_1G as u64);

        u64::from(bindings::GSP_FW_HEAP_PARAM_SIZE_PER_GB_FB)
            .saturating_mul(fb_size_gb)
            .align_up(GSP_HEAP_ALIGNMENT)
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
    pub(crate) fn new(gsp_firmware: &GspFirmware, fb_layout: &FbLayout) -> Result<Self> {
        Ok(Self(bindings::GspFwWprMeta {
            magic: r570_144::GSP_FW_WPR_META_MAGIC as u64,
            revision: u64::from(r570_144::GSP_FW_WPR_META_REVISION),
            sysmemAddrOfRadix3Elf: gsp_firmware.radix3_dma_handle(),
            sizeOfRadix3Elf: u64::try_from(gsp_firmware.size)?,
            sysmemAddrOfBootloader: gsp_firmware.bootloader.ucode.dma_handle(),
            sizeOfBootloader: u64::try_from(gsp_firmware.bootloader.ucode.size())?,
            bootloaderCodeOffset: u64::from(gsp_firmware.bootloader.code_offset),
            bootloaderDataOffset: u64::from(gsp_firmware.bootloader.data_offset),
            bootloaderManifestOffset: u64::from(gsp_firmware.bootloader.manifest_offset),
            __bindgen_anon_1: GspFwWprMetaBootResumeInfo {
                __bindgen_anon_1: GspFwWprMetaBootInfo {
                    sysmemAddrOfSignature: gsp_firmware.signatures.dma_handle(),
                    sizeOfSignature: u64::try_from(gsp_firmware.signatures.size())?,
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
        }))
    }
}

#[derive(PartialEq)]
pub(crate) enum MsgFunction {
    // Common function codes
    Nop = bindings::NV_VGPU_MSG_FUNCTION_NOP as isize,
    SetGuestSystemInfo = bindings::NV_VGPU_MSG_FUNCTION_SET_GUEST_SYSTEM_INFO as isize,
    AllocRoot = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_ROOT as isize,
    AllocDevice = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_DEVICE as isize,
    AllocMemory = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY as isize,
    AllocCtxDma = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_CTX_DMA as isize,
    AllocChannelDma = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_CHANNEL_DMA as isize,
    MapMemory = bindings::NV_VGPU_MSG_FUNCTION_MAP_MEMORY as isize,
    BindCtxDma = bindings::NV_VGPU_MSG_FUNCTION_BIND_CTX_DMA as isize,
    AllocObject = bindings::NV_VGPU_MSG_FUNCTION_ALLOC_OBJECT as isize,
    Free = bindings::NV_VGPU_MSG_FUNCTION_FREE as isize,
    Log = bindings::NV_VGPU_MSG_FUNCTION_LOG as isize,
    GetGspStaticInfo = bindings::NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO as isize,
    SetRegistry = bindings::NV_VGPU_MSG_FUNCTION_SET_REGISTRY as isize,
    GspSetSystemInfo = bindings::NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO as isize,
    GspInitPostObjGpu = bindings::NV_VGPU_MSG_FUNCTION_GSP_INIT_POST_OBJGPU as isize,
    GspRmControl = bindings::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL as isize,
    GetStaticInfo = bindings::NV_VGPU_MSG_FUNCTION_GET_STATIC_INFO as isize,

    // Event codes
    GspInitDone = bindings::NV_VGPU_MSG_EVENT_GSP_INIT_DONE as isize,
    GspRunCpuSequencer = bindings::NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER as isize,
    PostEvent = bindings::NV_VGPU_MSG_EVENT_POST_EVENT as isize,
    RcTriggered = bindings::NV_VGPU_MSG_EVENT_RC_TRIGGERED as isize,
    MmuFaultQueued = bindings::NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED as isize,
    OsErrorLog = bindings::NV_VGPU_MSG_EVENT_OS_ERROR_LOG as isize,
    GspPostNoCat = bindings::NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD as isize,
    GspLockdownNotice = bindings::NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE as isize,
    UcodeLibOsPrint = bindings::NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT as isize,
}

impl fmt::Display for MsgFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Common function codes
            MsgFunction::Nop => write!(f, "NOP"),
            MsgFunction::SetGuestSystemInfo => write!(f, "SET_GUEST_SYSTEM_INFO"),
            MsgFunction::AllocRoot => write!(f, "ALLOC_ROOT"),
            MsgFunction::AllocDevice => write!(f, "ALLOC_DEVICE"),
            MsgFunction::AllocMemory => write!(f, "ALLOC_MEMORY"),
            MsgFunction::AllocCtxDma => write!(f, "ALLOC_CTX_DMA"),
            MsgFunction::AllocChannelDma => write!(f, "ALLOC_CHANNEL_DMA"),
            MsgFunction::MapMemory => write!(f, "MAP_MEMORY"),
            MsgFunction::BindCtxDma => write!(f, "BIND_CTX_DMA"),
            MsgFunction::AllocObject => write!(f, "ALLOC_OBJECT"),
            MsgFunction::Free => write!(f, "FREE"),
            MsgFunction::Log => write!(f, "LOG"),
            MsgFunction::GetGspStaticInfo => write!(f, "GET_GSP_STATIC_INFO"),
            MsgFunction::SetRegistry => write!(f, "SET_REGISTRY"),
            MsgFunction::GspSetSystemInfo => write!(f, "GSP_SET_SYSTEM_INFO"),
            MsgFunction::GspInitPostObjGpu => write!(f, "GSP_INIT_POST_OBJGPU"),
            MsgFunction::GspRmControl => write!(f, "GSP_RM_CONTROL"),
            MsgFunction::GetStaticInfo => write!(f, "GET_STATIC_INFO"),

            // Event codes
            MsgFunction::GspInitDone => write!(f, "INIT_DONE"),
            MsgFunction::GspRunCpuSequencer => write!(f, "RUN_CPU_SEQUENCER"),
            MsgFunction::PostEvent => write!(f, "POST_EVENT"),
            MsgFunction::RcTriggered => write!(f, "RC_TRIGGERED"),
            MsgFunction::MmuFaultQueued => write!(f, "MMU_FAULT_QUEUED"),
            MsgFunction::OsErrorLog => write!(f, "OS_ERROR_LOG"),
            MsgFunction::GspPostNoCat => write!(f, "NOCAT"),
            MsgFunction::GspLockdownNotice => write!(f, "LOCKDOWN_NOTICE"),
            MsgFunction::UcodeLibOsPrint => write!(f, "LIBOS_PRINT"),
        }
    }
}

impl TryFrom<u32> for MsgFunction {
    type Error = kernel::error::Error;

    fn try_from(value: u32) -> Result<MsgFunction> {
        match value {
            bindings::NV_VGPU_MSG_FUNCTION_NOP => Ok(MsgFunction::Nop),
            bindings::NV_VGPU_MSG_FUNCTION_SET_GUEST_SYSTEM_INFO => {
                Ok(MsgFunction::SetGuestSystemInfo)
            }
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_ROOT => Ok(MsgFunction::AllocRoot),
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_DEVICE => Ok(MsgFunction::AllocDevice),
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY => Ok(MsgFunction::AllocMemory),
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_CTX_DMA => Ok(MsgFunction::AllocCtxDma),
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_CHANNEL_DMA => Ok(MsgFunction::AllocChannelDma),
            bindings::NV_VGPU_MSG_FUNCTION_MAP_MEMORY => Ok(MsgFunction::MapMemory),
            bindings::NV_VGPU_MSG_FUNCTION_BIND_CTX_DMA => Ok(MsgFunction::BindCtxDma),
            bindings::NV_VGPU_MSG_FUNCTION_ALLOC_OBJECT => Ok(MsgFunction::AllocObject),
            bindings::NV_VGPU_MSG_FUNCTION_FREE => Ok(MsgFunction::Free),
            bindings::NV_VGPU_MSG_FUNCTION_LOG => Ok(MsgFunction::Log),
            bindings::NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO => Ok(MsgFunction::GetGspStaticInfo),
            bindings::NV_VGPU_MSG_FUNCTION_SET_REGISTRY => Ok(MsgFunction::SetRegistry),
            bindings::NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO => Ok(MsgFunction::GspSetSystemInfo),
            bindings::NV_VGPU_MSG_FUNCTION_GSP_INIT_POST_OBJGPU => {
                Ok(MsgFunction::GspInitPostObjGpu)
            }
            bindings::NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL => Ok(MsgFunction::GspRmControl),
            bindings::NV_VGPU_MSG_FUNCTION_GET_STATIC_INFO => Ok(MsgFunction::GetStaticInfo),
            bindings::NV_VGPU_MSG_EVENT_GSP_INIT_DONE => Ok(MsgFunction::GspInitDone),
            bindings::NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER => {
                Ok(MsgFunction::GspRunCpuSequencer)
            }
            bindings::NV_VGPU_MSG_EVENT_POST_EVENT => Ok(MsgFunction::PostEvent),
            bindings::NV_VGPU_MSG_EVENT_RC_TRIGGERED => Ok(MsgFunction::RcTriggered),
            bindings::NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED => Ok(MsgFunction::MmuFaultQueued),
            bindings::NV_VGPU_MSG_EVENT_OS_ERROR_LOG => Ok(MsgFunction::OsErrorLog),
            bindings::NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD => Ok(MsgFunction::GspPostNoCat),
            bindings::NV_VGPU_MSG_EVENT_GSP_LOCKDOWN_NOTICE => Ok(MsgFunction::GspLockdownNotice),
            bindings::NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT => Ok(MsgFunction::UcodeLibOsPrint),
            _ => Err(EINVAL),
        }
    }
}

/// Struct containing the arguments required to pass a memory buffer to the GSP
/// for use during initialisation.
///
/// The GSP only understands 4K pages (GSP_PAGE_SIZE), so even if the kernel is
/// configured for a larger page size (e.g. 64K pages), we need to give
/// the GSP an array of 4K pages. Since we only create physically contiguous
/// buffers the math to calculate the addresses is simple.
///
/// The buffers must be a multiple of GSP_PAGE_SIZE.  GSP-RM also currently
/// ignores the @kind field for LOGINIT, LOGINTR, and LOGRM, but expects the
/// buffers to be physically contiguous anyway.
///
/// The memory allocated for the arguments must remain until the GSP sends the
/// init_done RPC.
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
    ) -> Result<Self> {
        /// Generates the `ID8` identifier required for some GSP objects.
        fn id8(name: &str) -> u64 {
            let mut bytes = [0u8; core::mem::size_of::<u64>()];

            for (c, b) in name.bytes().rev().zip(&mut bytes) {
                *b = c;
            }

            u64::from_ne_bytes(bytes)
        }

        Ok(Self(bindings::LibosMemoryRegionInitArgument {
            id8: id8(name),
            pa: obj.dma_handle(),
            size: obj.size() as u64,
            kind: bindings::LibosMemoryRegionKind_LIBOS_MEMORY_REGION_CONTIGUOUS.try_into()?,
            loc: bindings::LibosMemoryRegionLoc_LIBOS_MEMORY_REGION_LOC_SYSMEM.try_into()?,
            ..Default::default()
        }))
    }
}

#[repr(transparent)]
pub(crate) struct MsgqTxHeader(bindings::msgqTxHeader);

impl MsgqTxHeader {
    pub(crate) fn new(msgq_size: u32, rx_hdr_offset: u32, msg_count: u32) -> Self {
        Self(bindings::msgqTxHeader {
            version: 0,
            size: msgq_size,
            msgSize: GSP_PAGE_SIZE as u32,
            msgCount: msg_count,
            writePtr: 0,
            flags: 1,
            rxHdrOff: rx_hdr_offset,
            entryOff: GSP_PAGE_SIZE as u32,
        })
    }

    pub(crate) fn write_ptr(&self) -> u32 {
        let ptr = (&self.0.writePtr) as *const u32;

        // SAFETY: This is part of a CoherentAllocation and implements the
        // equivalent as what the dma_read! macro would and is therefore safe
        // for the same reasons.
        unsafe { ptr.read_volatile() }
    }

    pub(crate) fn set_write_ptr(&mut self, val: u32) {
        let ptr = (&mut self.0.writePtr) as *mut u32;

        // SAFETY: This is part of a CoherentAllocation and implements the
        // equivalent as what the dma_write! macro would and is therefore safe
        // for the same reasons.
        unsafe { ptr.write_volatile(val) }
    }
}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for MsgqTxHeader {}

/// RX header for setting up a message queue with the GSP.
#[repr(transparent)]
pub(crate) struct MsgqRxHeader(bindings::msgqRxHeader);

impl MsgqRxHeader {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }

    pub(crate) fn read_ptr(&self) -> u32 {
        let ptr = (&self.0.readPtr) as *const u32;

        // SAFETY: This is part of a CoherentAllocation and implements the
        // equivalent as what the dma_read! macro would and is therefore safe
        // for the same reasons.
        unsafe { ptr.read_volatile() }
    }

    pub(crate) fn set_read_ptr(&mut self, val: u32) {
        let ptr = (&mut self.0.readPtr) as *mut u32;

        // SAFETY: This is part of a CoherentAllocation and implements the
        // equivalent as what the dma_write! macro would and is therefore safe
        // for the same reasons.
        unsafe { ptr.write_volatile(val) }
    }
}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for MsgqRxHeader {}

impl bindings::rpc_message_header_v {
    pub(crate) fn init(cmd_size: u32, function: MsgFunction) -> impl Init<Self, Error> {
        type RpcMessageHeader = bindings::rpc_message_header_v;
        try_init!(RpcMessageHeader {
            // TODO: magic number
            header_version: 0x03000000,
            signature: bindings::NV_VGPU_MSG_SIGNATURE_VALID,
            function: function as u32,
            length: (size_of::<Self>() as u32)
                .checked_add(cmd_size)
                .ok_or(EOVERFLOW)?,
            rpc_result: 0xffffffff,
            rpc_result_private: 0xffffffff,
            ..Zeroable::init_zeroed()
        })
    }
}

// SAFETY: We can't derive the Zeroable trait for this binding because the
// procedural macro doesn't support the syntax used by bindgen to create the
// __IncompleteArrayField types. So instead we implement it here, which is safe
// because these are explicitly padded structures only containing types for
// which any bit pattern, including all zeros, is valid.
unsafe impl Zeroable for bindings::rpc_message_header_v {}

#[repr(transparent)]
pub(crate) struct GspMsgElement {
    inner: bindings::GSP_MSG_QUEUE_ELEMENT,
}

impl GspMsgElement {
    #[allow(non_snake_case)]
    pub(crate) fn init(
        sequence: u32,
        cmd_size: usize,
        function: MsgFunction,
    ) -> impl Init<Self, Error> {
        type RpcMessageHeader = bindings::rpc_message_header_v;
        type InnerGspMsgElement = bindings::GSP_MSG_QUEUE_ELEMENT;
        let init_inner = try_init!(InnerGspMsgElement {
            seqNum: sequence,
            elemCount: size_of::<Self>()
                .checked_add(cmd_size)
                .ok_or(EOVERFLOW)?
                .div_ceil(GSP_PAGE_SIZE) as u32,
            rpc <- RpcMessageHeader::init(cmd_size as u32, function),
            ..Zeroable::init_zeroed()
        });

        try_init!(GspMsgElement {
            inner <- init_inner,
        })
    }

    pub(crate) fn set_checksum(&mut self, checksum: u32) {
        self.inner.checkSum = checksum;
    }

    // Return the total length of the message, noting that rpc.length includes
    // the length of the GspRpcHeader but not the message header.
    pub(crate) fn length(&self) -> u32 {
        size_of::<Self>() as u32 - size_of::<bindings::rpc_message_header_v>() as u32
            + self.inner.rpc.length
    }

    pub(crate) fn sequence(&self) -> u32 {
        self.inner.rpc.sequence
    }

    pub(crate) fn function_number(&self) -> u32 {
        self.inner.rpc.function
    }

    pub(crate) fn function(&self) -> Result<MsgFunction> {
        self.inner.rpc.function.try_into()
    }

    pub(crate) fn element_count(&self) -> u32 {
        self.inner.elemCount
    }
}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GspMsgElement {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GspMsgElement {}

#[repr(transparent)]
pub(crate) struct GspArgumentsCached(bindings::GSP_ARGUMENTS_CACHED);

impl GspArgumentsCached {
    pub(crate) fn new(
        queue_arguments: MessageQueueInitArguments,
        sr_arguments: GspSrInitArguments,
    ) -> Self {
        Self(bindings::GSP_ARGUMENTS_CACHED {
            messageQueueInitArguments: queue_arguments.0,
            srInitArguments: sr_arguments.0,
            bDmemStack: 1,
            ..Default::default()
        })
    }
}

// SAFETY: Padding is explicit and will not contain uninitialized data.
unsafe impl AsBytes for GspArgumentsCached {}

// SAFETY: This struct only contains integer types for which all bit patterns
// are valid.
unsafe impl FromBytes for GspArgumentsCached {}

#[repr(transparent)]
pub(crate) struct MessageQueueInitArguments(bindings::MESSAGE_QUEUE_INIT_ARGUMENTS);

impl MessageQueueInitArguments {
    pub(crate) fn new(cmdq: &Cmdq) -> Self {
        Self(bindings::MESSAGE_QUEUE_INIT_ARGUMENTS {
            sharedMemPhysAddr: cmdq.dma_handle(),
            pageTableEntryCount: Cmdq::NUM_PTES as u32,
            cmdQueueOffset: Cmdq::CMDQ_OFFSET as u64,
            statQueueOffset: Cmdq::STATQ_OFFSET as u64,
            ..Default::default()
        })
    }
}

#[repr(transparent)]
pub(crate) struct GspSrInitArguments(bindings::GSP_SR_INIT_ARGUMENTS);

impl GspSrInitArguments {
    pub(crate) fn new() -> Self {
        Self(bindings::GSP_SR_INIT_ARGUMENTS {
            oldLevel: 0,
            flags: 0,
            bInPMTransition: 0,
            ..Default::default()
        })
    }
}
