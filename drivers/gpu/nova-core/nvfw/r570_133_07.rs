// AUTO GENERATED
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_upper_case_globals)]

pub(crate) const NV_VGPU_MSG_FUNCTION_UPDATE_BAR_PDE: u32 = 70;
pub(crate) const NV_VGPU_MSG_FUNCTION_ALLOC_MEMORY: u32 = 4;
pub(crate) const NV_VGPU_MSG_FUNCTION_FREE: u32 = 10;
pub(crate) const NV_VGPU_MSG_FUNCTION_CTRL_VASPACE_COPY_SERVER_RESERVED_PDES: u32 = 116;
pub(crate) const NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO: u32 = 65;
pub(crate) const NV_VGPU_MSG_FUNCTION_SET_REGISTRY: u32 = 73;
pub(crate) const NV_VGPU_MSG_FUNCTION_GSP_RM_ALLOC: u32 = 103;
pub(crate) const NV_VGPU_MSG_FUNCTION_GSP_RM_CONTROL: u32 = 76;
pub(crate) const NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO: u32 = 72;
pub(crate) const NV_VGPU_MSG_FUNCTION_CONTINUATION_RECORD: u32 = 71;
pub(crate) const NV_VGPU_MSG_FUNCTION_UNLOADING_GUEST_DRIVER: u32 = 47;
pub(crate) const NV_VGPU_MSG_EVENT_GSP_INIT_DONE: u32 = 4097;
pub(crate) const NV_VGPU_MSG_EVENT_GSP_RUN_CPU_SEQUENCER: u32 = 4098;
pub(crate) const NV_VGPU_MSG_EVENT_POST_EVENT: u32 = 4099;
pub(crate) const NV_VGPU_MSG_EVENT_RC_TRIGGERED: u32 = 4100;
pub(crate) const NV_VGPU_MSG_EVENT_MMU_FAULT_QUEUED: u32 = 4101;
pub(crate) const NV_VGPU_MSG_EVENT_OS_ERROR_LOG: u32 = 4102;
pub(crate) const NV_VGPU_MSG_EVENT_GPUACCT_PERFMON_UTIL_SAMPLES: u32 = 4104;
pub(crate) const NV_VGPU_MSG_EVENT_PERF_BRIDGELESS_INFO_UPDATE: u32 = 4111;
pub(crate) const NV_VGPU_MSG_EVENT_UCODE_LIBOS_PRINT: u32 = 4108;
pub(crate) const NV_VGPU_MSG_EVENT_GSP_POST_NOCAT_RECORD: u32 = 4128;
pub(crate) const MC_ENGINE_IDX_ACCESS_CNTR: u32 = 60;
pub(crate) const MC_ENGINE_IDX_BIF: u32 = 11;
pub(crate) const MC_ENGINE_IDX_BLG: u32 = 76;
pub(crate) const MC_ENGINE_IDX_BSP: u32 = 65;
pub(crate) const MC_ENGINE_IDX_BUF_RESET: u32 = 78;
pub(crate) const MC_ENGINE_IDX_BUS: u32 = 7;
pub(crate) const MC_ENGINE_IDX_C2C: u32 = 42;
pub(crate) const MC_ENGINE_IDX_CE0: u32 = 15;
pub(crate) const MC_ENGINE_IDX_CE1: u32 = 16;
pub(crate) const MC_ENGINE_IDX_CE10: u32 = 25;
pub(crate) const MC_ENGINE_IDX_CE11: u32 = 26;
pub(crate) const MC_ENGINE_IDX_CE12: u32 = 27;
pub(crate) const MC_ENGINE_IDX_CE13: u32 = 28;
pub(crate) const MC_ENGINE_IDX_CE14: u32 = 29;
pub(crate) const MC_ENGINE_IDX_CE15: u32 = 30;
pub(crate) const MC_ENGINE_IDX_CE16: u32 = 31;
pub(crate) const MC_ENGINE_IDX_CE17: u32 = 32;
pub(crate) const MC_ENGINE_IDX_CE18: u32 = 33;
pub(crate) const MC_ENGINE_IDX_CE19: u32 = 34;
pub(crate) const MC_ENGINE_IDX_CE2: u32 = 17;
pub(crate) const MC_ENGINE_IDX_CE3: u32 = 18;
pub(crate) const MC_ENGINE_IDX_CE4: u32 = 19;
pub(crate) const MC_ENGINE_IDX_CE5: u32 = 20;
pub(crate) const MC_ENGINE_IDX_CE6: u32 = 21;
pub(crate) const MC_ENGINE_IDX_CE7: u32 = 22;
pub(crate) const MC_ENGINE_IDX_CE8: u32 = 23;
pub(crate) const MC_ENGINE_IDX_CE9: u32 = 24;
pub(crate) const MC_ENGINE_IDX_CE_MAX: u32 = MC_ENGINE_IDX_CE19;
pub(crate) const MC_ENGINE_IDX_CIPHER: u32 = 10;
pub(crate) const MC_ENGINE_IDX_CPU_DOORBELL: u32 = 73;
pub(crate) const MC_ENGINE_IDX_DISP: u32 = 2;
pub(crate) const MC_ENGINE_IDX_DISP_GSP: u32 = 165;
pub(crate) const MC_ENGINE_IDX_DISP_LOW: u32 = 176;
pub(crate) const MC_ENGINE_IDX_DPAUX: u32 = 175;
pub(crate) const MC_ENGINE_IDX_ESCHED: u32 = 92;
pub(crate) const MC_ENGINE_IDX_ESCHED__SIZE: u32 = 64;
pub(crate) const MC_ENGINE_IDX_FB: u32 = 3;
pub(crate) const MC_ENGINE_IDX_FBHUB: u32 = 44;
pub(crate) const MC_ENGINE_IDX_FIFO: u32 = 4;
pub(crate) const MC_ENGINE_IDX_FSP: u32 = 48;
pub(crate) const MC_ENGINE_IDX_GMMU: u32 = 46;
pub(crate) const MC_ENGINE_IDX_GR: u32 = 84;
pub(crate) const MC_ENGINE_IDX_GR0: u32 = MC_ENGINE_IDX_GR;
pub(crate) const MC_ENGINE_IDX_GR0_FECS_LOG: u32 = MC_ENGINE_IDX_GR_FECS_LOG;
pub(crate) const MC_ENGINE_IDX_GR1: u32 = 85;
pub(crate) const MC_ENGINE_IDX_GR1_FECS_LOG: u32 = 157;
pub(crate) const MC_ENGINE_IDX_GR2: u32 = 86;
pub(crate) const MC_ENGINE_IDX_GR2_FECS_LOG: u32 = 158;
pub(crate) const MC_ENGINE_IDX_GR3: u32 = 87;
pub(crate) const MC_ENGINE_IDX_GR3_FECS_LOG: u32 = 159;
pub(crate) const MC_ENGINE_IDX_GR4: u32 = 88;
pub(crate) const MC_ENGINE_IDX_GR4_FECS_LOG: u32 = 160;
pub(crate) const MC_ENGINE_IDX_GR5: u32 = 89;
pub(crate) const MC_ENGINE_IDX_GR5_FECS_LOG: u32 = 161;
pub(crate) const MC_ENGINE_IDX_GR6: u32 = 90;
pub(crate) const MC_ENGINE_IDX_GR6_FECS_LOG: u32 = 162;
pub(crate) const MC_ENGINE_IDX_GR7: u32 = 91;
pub(crate) const MC_ENGINE_IDX_GR7_FECS_LOG: u32 = 163;
pub(crate) const MC_ENGINE_IDX_GR_FECS_LOG: u32 = 156;
pub(crate) const MC_ENGINE_IDX_GSP: u32 = 50;
pub(crate) const MC_ENGINE_IDX_GSPLITE: u32 = 171;
pub(crate) const MC_ENGINE_IDX_GSPLITE0: u32 = MC_ENGINE_IDX_GSPLITE;
pub(crate) const MC_ENGINE_IDX_GSPLITE1: u32 = 172;
pub(crate) const MC_ENGINE_IDX_GSPLITE2: u32 = 173;
pub(crate) const MC_ENGINE_IDX_GSPLITE3: u32 = 174;
pub(crate) const MC_ENGINE_IDX_GSPLITE_MAX: u32 = MC_ENGINE_IDX_GSPLITE3;
pub(crate) const MC_ENGINE_IDX_HDACODEC: u32 = 45;
pub(crate) const MC_ENGINE_IDX_INFO_FAULT: u32 = 64;
pub(crate) const MC_ENGINE_IDX_ISOHUB: u32 = 36;
pub(crate) const MC_ENGINE_IDX_LRCC: u32 = 170;
pub(crate) const MC_ENGINE_IDX_LTC: u32 = 43;
pub(crate) const MC_ENGINE_IDX_MAX: u32 = 177;
pub(crate) const MC_ENGINE_IDX_MD: u32 = 6;
pub(crate) const MC_ENGINE_IDX_MMU_ECC_ERROR: u32 = 75;
pub(crate) const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT: u32 = 61;
pub(crate) const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT_CPU: u32 = 167;
pub(crate) const MC_ENGINE_IDX_NON_REPLAYABLE_FAULT_ERROR: u32 = 63;
pub(crate) const MC_ENGINE_IDX_NULL: u32 = 0;
pub(crate) const MC_ENGINE_IDX_NVDEC: u32 = MC_ENGINE_IDX_BSP;
pub(crate) const MC_ENGINE_IDX_NVDEC0: u32 = MC_ENGINE_IDX_NVDEC;
pub(crate) const MC_ENGINE_IDX_NVDEC1: u32 = 66;
pub(crate) const MC_ENGINE_IDX_NVDEC2: u32 = 67;
pub(crate) const MC_ENGINE_IDX_NVDEC3: u32 = 68;
pub(crate) const MC_ENGINE_IDX_NVDEC4: u32 = 69;
pub(crate) const MC_ENGINE_IDX_NVDEC5: u32 = 70;
pub(crate) const MC_ENGINE_IDX_NVDEC6: u32 = 71;
pub(crate) const MC_ENGINE_IDX_NVDEC7: u32 = 72;
pub(crate) const MC_ENGINE_IDX_NVENC: u32 = 38;
pub(crate) const MC_ENGINE_IDX_NVENC1: u32 = 39;
pub(crate) const MC_ENGINE_IDX_NVENC2: u32 = 40;
pub(crate) const MC_ENGINE_IDX_NVENC3: u32 = 41;
pub(crate) const MC_ENGINE_IDX_NVJPEG: u32 = MC_ENGINE_IDX_NVJPG;
pub(crate) const MC_ENGINE_IDX_NVJPEG0: u32 = MC_ENGINE_IDX_NVJPEG;
pub(crate) const MC_ENGINE_IDX_NVJPEG1: u32 = 52;
pub(crate) const MC_ENGINE_IDX_NVJPEG2: u32 = 53;
pub(crate) const MC_ENGINE_IDX_NVJPEG3: u32 = 54;
pub(crate) const MC_ENGINE_IDX_NVJPEG4: u32 = 55;
pub(crate) const MC_ENGINE_IDX_NVJPEG5: u32 = 56;
pub(crate) const MC_ENGINE_IDX_NVJPEG6: u32 = 57;
pub(crate) const MC_ENGINE_IDX_NVJPEG7: u32 = 58;
pub(crate) const MC_ENGINE_IDX_NVJPG: u32 = 51;
pub(crate) const MC_ENGINE_IDX_NVLINK: u32 = 49;
pub(crate) const MC_ENGINE_IDX_OFA0: u32 = 81;
pub(crate) const MC_ENGINE_IDX_OFA1: u32 = 82;
pub(crate) const MC_ENGINE_IDX_PERFMON: u32 = 77;
pub(crate) const MC_ENGINE_IDX_PMGR: u32 = 8;
pub(crate) const MC_ENGINE_IDX_PMU: u32 = 14;
pub(crate) const MC_ENGINE_IDX_PPP: u32 = 12;
pub(crate) const MC_ENGINE_IDX_PRIVRING: u32 = 13;
pub(crate) const MC_ENGINE_IDX_PRIV_DOORBELL: u32 = 74;
pub(crate) const MC_ENGINE_IDX_PXUC: u32 = 168;
pub(crate) const MC_ENGINE_IDX_REPLAYABLE_FAULT: u32 = 59;
pub(crate) const MC_ENGINE_IDX_REPLAYABLE_FAULT_CPU: u32 = 166;
pub(crate) const MC_ENGINE_IDX_REPLAYABLE_FAULT_ERROR: u32 = 62;
pub(crate) const MC_ENGINE_IDX_SEC2: u32 = 47;
pub(crate) const MC_ENGINE_IDX_SYSLTC: u32 = 169;
pub(crate) const MC_ENGINE_IDX_TEGRA: u32 = 83;
pub(crate) const MC_ENGINE_IDX_TMR: u32 = 1;
pub(crate) const MC_ENGINE_IDX_TMR_SWRL: u32 = 164;
pub(crate) const MC_ENGINE_IDX_VGPU: u32 = 37;
pub(crate) const MC_ENGINE_IDX_VIC: u32 = 35;
pub(crate) const MC_ENGINE_IDX_VIDEO: u32 = 5;
pub(crate) const MC_ENGINE_IDX_VP2: u32 = 9;
pub(crate) const MC_ENGINE_IDX_XBAR: u32 = 79;
pub(crate) const MC_ENGINE_IDX_ZPW: u32 = 80;
pub(crate) const ENGINE_INFO_TYPE_DEV_TYPE_ENUM: u32 = 9;
pub(crate) const ENGINE_INFO_TYPE_RM_ENGINE_TYPE: u32 = 2;
pub(crate) const ENGINE_INFO_TYPE_RUNLIST: u32 = 3;
pub(crate) const ENGINE_INFO_TYPE_RUNLIST_ENGINE_ID: u32 = 13;
pub(crate) const ENGINE_INFO_TYPE_RUNLIST_PRI_BASE: u32 = 11;
pub(crate) const ENGINE_INFO_TYPE_ENG_DESC: u32 = 0;
pub(crate) const GSP_FW_WPR_META_MAGIC: u64 = 0xdc3aae21371a60b3_u64;
pub(crate) const GSP_FW_WPR_META_REVISION: u32 = 1;
pub(crate) const GSP_FW_SR_META_MAGIC: u64 = 0x8a3bb9e6c6c39d93_u64;
pub(crate) const GSP_FW_SR_META_REVISION: u32 = 2;
pub(crate) const LIBOS_MEMORY_REGION_CONTIGUOUS: u32 = 1;
pub(crate) const LIBOS_MEMORY_REGION_INIT_ARGUMENTS_MAX: u32 = 4096;
pub(crate) const LIBOS_MEMORY_REGION_LOC_FB: u32 = 2;
pub(crate) const LIBOS_MEMORY_REGION_LOC_NONE: u32 = 0;
pub(crate) const LIBOS_MEMORY_REGION_LOC_SYSMEM: u32 = 1;
pub(crate) const LIBOS_MEMORY_REGION_NONE: u32 = 0;
pub(crate) const LIBOS_MEMORY_REGION_RADIX3: u32 = 2;
pub(crate) const LIBOS_MEMORY_REGION_RADIX_PAGE_LOG2: u32 = 12;
pub(crate) const LIBOS_MEMORY_REGION_RADIX_PAGE_SIZE: u32 = 4096;
pub(crate) const GSP_FW_HEAP_PARAM_SIZE_PER_GB_FB: u32 = 96 << 10;
pub(crate) const GSP_FW_HEAP_PARAM_CLIENT_ALLOC_SIZE: u32 = (48 << 10) * 2048;
pub(crate) const GSP_FW_HEAP_PARAM_BASE_RM_SIZE_TU10X: u32 = 8 << 20;
pub(crate) const GSP_FW_HEAP_PARAM_BASE_RM_SIZE_GH100: u32 = 14 << 20;
pub(crate) const GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS2: u32 = 0 << 20;
pub(crate) const GSP_FW_HEAP_PARAM_OS_SIZE_LIBOS3_BAREMETAL: u32 = 22 << 20;
pub(crate) const GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MIN_MB: u32 = 64;
pub(crate) const GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS2_MAX_MB: u32 = 256;
pub(crate) const GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MIN_MB: u32 = 88;
pub(crate) const GSP_FW_HEAP_SIZE_OVERRIDE_LIBOS3_BAREMETAL_MAX_MB: u32 = 280;
pub(crate) const LOGIDX_SIZE: u32 = 5;
pub(crate) const NV01_ROOT: u32 = 0x0;
pub(crate) const NV01_DEVICE_0: u32 = 0x00000080;
pub(crate) const NV01_EVENT_KERNEL_CALLBACK_EX: u32 = 0x0000007E;
pub(crate) const NV01_EVENT_CLIENT_RM: u32 = 0x04000000;
pub(crate) const NV04_DISPLAY_COMMON: u32 = 0x00000073;
pub(crate) const NV20_SUBDEVICE_0: u32 = 0x2080;
pub(crate) const FERMI_VASPACE_A: u32 = 0x000090f1;
pub(crate) const NV2080_CTRL_GPU_SET_POWER_STATE_GPU_LEVEL_0: u32 = 0x00000000;
pub(crate) const NV2080_CTRL_GPU_SET_POWER_STATE_GPU_LEVEL_3: u32 = 0x00000003;
pub(crate) const NV_RPC_UPDATE_PDE_BAR_2: u32 = 1;
pub(crate) const TURING_CHANNEL_GPFIFO_A: u32 = 0x0000c46f;
pub(crate) const AMPERE_CHANNEL_GPFIFO_A: u32 = 0x0000c56f;
pub(crate) const FERMI_TWOD_A: u32 = 0x0000902d;
pub(crate) const KEPLER_INLINE_TO_MEMORY_B: u32 = 0x0000a140;
pub(crate) const ADA_A: u32 = 0x0000c997;
pub(crate) const ADA_COMPUTE_A: u32 = 0x0000c9c0;
pub(crate) const AMPERE_B: u32 = 0x0000c797;
pub(crate) const AMPERE_DMA_COPY_A: u32 = 0x0000c6b5;
pub(crate) const AMPERE_DMA_COPY_B: u32 = 0x0000c7b5;
pub(crate) const AMPERE_COMPUTE_B: u32 = 0x0000c7c0;
pub(crate) const TURING_A: u32 = 0x0000c597;
pub(crate) const TURING_COMPUTE_A: u32 = 0x0000c5c0;
pub(crate) const TURING_DMA_COPY_A: u32 = 0x0000c5b5;
pub(crate) const NV_VASPACE_ALLOCATION_INDEX_GPU_NEW: u32 = 0x00;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_A: u32 = 4;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_B: u32 = 0;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_CAPTURE: u32 = 0x00000004;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_COMPUTE_PREEMPT: u32 =
    0x0000000a;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_COUNT: u32 = 0x0000001a;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_DISPLAY: u32 = 0x00000005;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_ENCRYPTION: u32 =
    0x00000006;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS: u32 =
    0x00000000;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_ATTRIBUTE_CB:
    u32 = 0x00000013;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_BETACB: u32 =
    0x0000000e;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_BUNDLE_CB: u32 =
    0x00000011;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_FECS_EVENT: u32 =
    0x00000017;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_GFXP_CTRL_BLK:
    u32 = 0x00000016;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_GFXP_POOL: u32 =
    0x00000015;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PAGEPOOL: u32 =
    0x0000000d;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PAGEPOOL_GLOBAL: u32 = 0x00000012;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PATCH: u32 =
    0x00000010;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PM: u32 =
    0x00000009;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PREEMPT: u32 =
    0x0000000b;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_PRIV_ACCESS_MAP: u32 = 0x00000018;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_RTV: u32 =
    0x0000000f;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_RTV_CB_GLOBAL:
    u32 = 0x00000014;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_SETUP: u32 =
    0x00000019;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_SPILL: u32 =
    0x0000000c;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_GRAPHICS_ZCULL: u32 =
    0x00000008;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_MPEG: u32 = 0x00000003;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_POSTPROCESS: u32 =
    0x00000007;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_VIDEO: u32 = 0x00000002;
pub(crate) const NV0080_CTRL_FIFO_GET_ENGINE_CONTEXT_PROPERTIES_ENGINE_ID_VLD: u32 = 0x00000001;
pub(crate) const NV2080_CTRL_INTERNAL_GR_MAX_ENGINES: u32 = 8;
pub(crate) const NVC57A_CURSOR_IMM_CHANNEL_PIO: u32 = 0x0000c57a;
pub(crate) const NVC67A_CURSOR_IMM_CHANNEL_PIO: u32 = 0x0000c67a;
pub(crate) const NVC57B_WINDOW_IMM_CHANNEL_DMA: u32 = 0x0000c57b;
pub(crate) const NVC67B_WINDOW_IMM_CHANNEL_DMA: u32 = 0x0000C67B;
pub(crate) const NVC57D_CORE_CHANNEL_DMA: u32 = 0x0000c57d;
pub(crate) const NVC67D_CORE_CHANNEL_DMA: u32 = 0x0000C67D;
pub(crate) const NVC77D_CORE_CHANNEL_DMA: u32 = 0x0000c77d;
pub(crate) const NVC57E_WINDOW_CHANNEL_DMA: u32 = 0x0000c57e;
pub(crate) const NVC67E_WINDOW_CHANNEL_DMA: u32 = 0x0000C67E;
pub(crate) const ADDR_FBMEM: u32 = 2;
pub(crate) const NV_MEMORY_WRITECOMBINED: u32 = 2;
pub(crate) const NV2080_ENGINE_TYPE_ALLENGINES: u32 = 0xffffffff;
pub(crate) const NV2080_ENGINE_TYPE_BSP: u32 = 0x00000013;
pub(crate) const NV2080_ENGINE_TYPE_CIPHER: u32 = 0x00000023;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY0: u32 = 0x00000040;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY1: u32 = 0x00000041;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY10: u32 = 0x0000004a;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY11: u32 = 0x0000004b;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY12: u32 = 0x0000004c;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY13: u32 = 0x0000004d;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY14: u32 = 0x0000004e;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY15: u32 = 0x0000004f;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY16: u32 = 0x00000050;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY17: u32 = 0x00000051;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY18: u32 = 0x00000052;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY19: u32 = 0x00000053;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY2: u32 = 0x00000042;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY3: u32 = 0x00000043;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY4: u32 = 0x00000044;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY5: u32 = 0x00000045;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY6: u32 = 0x00000046;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY7: u32 = 0x00000047;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY8: u32 = 0x00000048;
pub(crate) const NV2080_ENGINE_TYPE_COMP_DECOMP_COPY9: u32 = 0x00000049;
pub(crate) const NV2080_ENGINE_TYPE_COPY0: u32 = 0x00000009;
pub(crate) const NV2080_ENGINE_TYPE_COPY1: u32 = 0x0000000a;
pub(crate) const NV2080_ENGINE_TYPE_COPY10: u32 = 0x00000034;
pub(crate) const NV2080_ENGINE_TYPE_COPY11: u32 = 0x00000035;
pub(crate) const NV2080_ENGINE_TYPE_COPY12: u32 = 0x00000036;
pub(crate) const NV2080_ENGINE_TYPE_COPY13: u32 = 0x00000037;
pub(crate) const NV2080_ENGINE_TYPE_COPY14: u32 = 0x00000038;
pub(crate) const NV2080_ENGINE_TYPE_COPY15: u32 = 0x00000039;
pub(crate) const NV2080_ENGINE_TYPE_COPY16: u32 = 0x0000003a;
pub(crate) const NV2080_ENGINE_TYPE_COPY17: u32 = 0x0000003b;
pub(crate) const NV2080_ENGINE_TYPE_COPY18: u32 = 0x0000003c;
pub(crate) const NV2080_ENGINE_TYPE_COPY19: u32 = 0x0000003d;
pub(crate) const NV2080_ENGINE_TYPE_COPY2: u32 = 0x0000000b;
pub(crate) const NV2080_ENGINE_TYPE_COPY3: u32 = 0x0000000c;
pub(crate) const NV2080_ENGINE_TYPE_COPY4: u32 = 0x0000000d;
pub(crate) const NV2080_ENGINE_TYPE_COPY5: u32 = 0x0000000e;
pub(crate) const NV2080_ENGINE_TYPE_COPY6: u32 = 0x0000000f;
pub(crate) const NV2080_ENGINE_TYPE_COPY7: u32 = 0x00000010;
pub(crate) const NV2080_ENGINE_TYPE_COPY8: u32 = 0x00000011;
pub(crate) const NV2080_ENGINE_TYPE_COPY9: u32 = 0x00000012;
pub(crate) const NV2080_ENGINE_TYPE_COPY_SIZE: u32 = 64;
pub(crate) const NV2080_ENGINE_TYPE_COPY_SIZE_v1A_0D: u32 = 10;
pub(crate) const NV2080_ENGINE_TYPE_COPY_SIZE_v22_00: u32 = 10;
pub(crate) const NV2080_ENGINE_TYPE_COPY_SIZE_v24_09: u32 = 64;
pub(crate) const NV2080_ENGINE_TYPE_DPU: u32 = 0x00000028;
pub(crate) const NV2080_ENGINE_TYPE_FBFLCN: u32 = 0x0000002a;
pub(crate) const NV2080_ENGINE_TYPE_GR0: u32 = NV2080_ENGINE_TYPE_GRAPHICS;
pub(crate) const NV2080_ENGINE_TYPE_GR1: u32 = 0x00000002;
pub(crate) const NV2080_ENGINE_TYPE_GR2: u32 = 0x00000003;
pub(crate) const NV2080_ENGINE_TYPE_GR3: u32 = 0x00000004;
pub(crate) const NV2080_ENGINE_TYPE_GR4: u32 = 0x00000005;
pub(crate) const NV2080_ENGINE_TYPE_GR5: u32 = 0x00000006;
pub(crate) const NV2080_ENGINE_TYPE_GR6: u32 = 0x00000007;
pub(crate) const NV2080_ENGINE_TYPE_GR7: u32 = 0x00000008;
pub(crate) const NV2080_ENGINE_TYPE_GRAPHICS: u32 = 0x00000001;
pub(crate) const NV2080_ENGINE_TYPE_GR_SIZE: u32 = 8;
pub(crate) const NV2080_ENGINE_TYPE_HOST: u32 = 0x00000027;
pub(crate) const NV2080_ENGINE_TYPE_LAST: u32 = 0x00000054;
pub(crate) const NV2080_ENGINE_TYPE_LAST_v18_01: u32 = 0x0000002a;
pub(crate) const NV2080_ENGINE_TYPE_LAST_v1A_00: u32 = 0x2a;
pub(crate) const NV2080_ENGINE_TYPE_LAST_v1C_09: u32 = 0x00000034;
pub(crate) const NV2080_ENGINE_TYPE_LAST_v27_02: u32 = 0x00000054;
pub(crate) const NV2080_ENGINE_TYPE_ME: u32 = 0x0000001f;
pub(crate) const NV2080_ENGINE_TYPE_MP: u32 = 0x00000025;
pub(crate) const NV2080_ENGINE_TYPE_MPEG: u32 = 0x00000021;
pub(crate) const NV2080_ENGINE_TYPE_MSENC: u32 = 0x0000001b;
pub(crate) const NV2080_ENGINE_TYPE_NULL: u32 = 0x00000000;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC0: u32 = NV2080_ENGINE_TYPE_BSP;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC1: u32 = 0x00000014;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC2: u32 = 0x00000015;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC3: u32 = 0x00000016;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC4: u32 = 0x00000017;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC5: u32 = 0x00000018;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC6: u32 = 0x00000019;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC7: u32 = 0x0000001a;
pub(crate) const NV2080_ENGINE_TYPE_NVDEC_SIZE: u32 = 8;
pub(crate) const NV2080_ENGINE_TYPE_NVENC0: u32 = NV2080_ENGINE_TYPE_MSENC;
pub(crate) const NV2080_ENGINE_TYPE_NVENC1: u32 = 0x0000001c;
pub(crate) const NV2080_ENGINE_TYPE_NVENC2: u32 = 0x0000001d;
pub(crate) const NV2080_ENGINE_TYPE_NVENC3: u32 = 0x0000003f;
pub(crate) const NV2080_ENGINE_TYPE_NVENC_SIZE: u32 = 4;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG0: u32 = NV2080_ENGINE_TYPE_NVJPG;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG1: u32 = 0x0000002c;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG2: u32 = 0x0000002d;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG3: u32 = 0x0000002e;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG4: u32 = 0x0000002f;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG5: u32 = 0x00000030;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG6: u32 = 0x00000031;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG7: u32 = 0x00000032;
pub(crate) const NV2080_ENGINE_TYPE_NVJPEG_SIZE: u32 = 8;
pub(crate) const NV2080_ENGINE_TYPE_NVJPG: u32 = 0x0000002b;
pub(crate) const NV2080_ENGINE_TYPE_OFA: u32 = 0x00000033;
pub(crate) const NV2080_ENGINE_TYPE_OFA0: u32 = NV2080_ENGINE_TYPE_OFA;
pub(crate) const NV2080_ENGINE_TYPE_OFA1: u32 = 0x0000003e;
pub(crate) const NV2080_ENGINE_TYPE_OFA_SIZE: u32 = 2;
pub(crate) const NV2080_ENGINE_TYPE_PMU: u32 = 0x00000029;
pub(crate) const NV2080_ENGINE_TYPE_PPP: u32 = 0x00000020;
pub(crate) const NV2080_ENGINE_TYPE_SEC2: u32 = 0x00000026;
pub(crate) const NV2080_ENGINE_TYPE_SW: u32 = 0x00000022;
pub(crate) const NV2080_ENGINE_TYPE_TSEC: u32 = NV2080_ENGINE_TYPE_CIPHER;
pub(crate) const NV2080_ENGINE_TYPE_VIC: u32 = 0x00000024;
pub(crate) const NV2080_ENGINE_TYPE_VP: u32 = 0x0000001e;
pub(crate) const RM_ENGINE_TYPE_BSP: u32 = RM_ENGINE_TYPE_NVDEC0;
pub(crate) const RM_ENGINE_TYPE_CIPHER: u32 = RM_ENGINE_TYPE_TSEC;
pub(crate) const RM_ENGINE_TYPE_COPY0: u32 = 9;
pub(crate) const RM_ENGINE_TYPE_COPY1: u32 = 10;
pub(crate) const RM_ENGINE_TYPE_COPY10: u32 = 19;
pub(crate) const RM_ENGINE_TYPE_COPY11: u32 = 20;
pub(crate) const RM_ENGINE_TYPE_COPY12: u32 = 21;
pub(crate) const RM_ENGINE_TYPE_COPY13: u32 = 22;
pub(crate) const RM_ENGINE_TYPE_COPY14: u32 = 23;
pub(crate) const RM_ENGINE_TYPE_COPY15: u32 = 24;
pub(crate) const RM_ENGINE_TYPE_COPY16: u32 = 25;
pub(crate) const RM_ENGINE_TYPE_COPY17: u32 = 26;
pub(crate) const RM_ENGINE_TYPE_COPY18: u32 = 27;
pub(crate) const RM_ENGINE_TYPE_COPY19: u32 = 28;
pub(crate) const RM_ENGINE_TYPE_COPY2: u32 = 11;
pub(crate) const RM_ENGINE_TYPE_COPY3: u32 = 12;
pub(crate) const RM_ENGINE_TYPE_COPY4: u32 = 13;
pub(crate) const RM_ENGINE_TYPE_COPY5: u32 = 14;
pub(crate) const RM_ENGINE_TYPE_COPY6: u32 = 15;
pub(crate) const RM_ENGINE_TYPE_COPY7: u32 = 16;
pub(crate) const RM_ENGINE_TYPE_COPY8: u32 = 17;
pub(crate) const RM_ENGINE_TYPE_COPY9: u32 = 18;
pub(crate) const RM_ENGINE_TYPE_COPY_SIZE: u32 = 20;
pub(crate) const RM_ENGINE_TYPE_DPU: u32 = 51;
pub(crate) const RM_ENGINE_TYPE_FBFLCN: u32 = 53;
pub(crate) const RM_ENGINE_TYPE_GR0: u32 = 1;
pub(crate) const RM_ENGINE_TYPE_GR1: u32 = 2;
pub(crate) const RM_ENGINE_TYPE_GR2: u32 = 3;
pub(crate) const RM_ENGINE_TYPE_GR3: u32 = 4;
pub(crate) const RM_ENGINE_TYPE_GR4: u32 = 5;
pub(crate) const RM_ENGINE_TYPE_GR5: u32 = 6;
pub(crate) const RM_ENGINE_TYPE_GR6: u32 = 7;
pub(crate) const RM_ENGINE_TYPE_GR7: u32 = 8;
pub(crate) const RM_ENGINE_TYPE_GRAPHICS: u32 = RM_ENGINE_TYPE_GR0;
pub(crate) const RM_ENGINE_TYPE_GR_SIZE: u32 = 8;
pub(crate) const RM_ENGINE_TYPE_HOST: u32 = 50;
pub(crate) const RM_ENGINE_TYPE_LAST: u32 = 84;
pub(crate) const RM_ENGINE_TYPE_ME: u32 = 42;
pub(crate) const RM_ENGINE_TYPE_MP: u32 = 48;
pub(crate) const RM_ENGINE_TYPE_MPEG: u32 = 44;
pub(crate) const RM_ENGINE_TYPE_MSENC: u32 = RM_ENGINE_TYPE_NVENC0;
pub(crate) const RM_ENGINE_TYPE_NULL: u32 = 0;
pub(crate) const RM_ENGINE_TYPE_NVDEC0: u32 = 29;
pub(crate) const RM_ENGINE_TYPE_NVDEC1: u32 = 30;
pub(crate) const RM_ENGINE_TYPE_NVDEC2: u32 = 31;
pub(crate) const RM_ENGINE_TYPE_NVDEC3: u32 = 32;
pub(crate) const RM_ENGINE_TYPE_NVDEC4: u32 = 33;
pub(crate) const RM_ENGINE_TYPE_NVDEC5: u32 = 34;
pub(crate) const RM_ENGINE_TYPE_NVDEC6: u32 = 35;
pub(crate) const RM_ENGINE_TYPE_NVDEC7: u32 = 36;
pub(crate) const RM_ENGINE_TYPE_NVDEC_SIZE: u32 = 8;
pub(crate) const RM_ENGINE_TYPE_NVENC0: u32 = 37;
pub(crate) const RM_ENGINE_TYPE_NVENC1: u32 = 38;
pub(crate) const RM_ENGINE_TYPE_NVENC2: u32 = 39;
pub(crate) const RM_ENGINE_TYPE_NVENC3: u32 = 40;
pub(crate) const RM_ENGINE_TYPE_NVENC_SIZE: u32 = 4;
pub(crate) const RM_ENGINE_TYPE_NVJPEG0: u32 = 54;
pub(crate) const RM_ENGINE_TYPE_NVJPEG1: u32 = 55;
pub(crate) const RM_ENGINE_TYPE_NVJPEG2: u32 = 56;
pub(crate) const RM_ENGINE_TYPE_NVJPEG3: u32 = 57;
pub(crate) const RM_ENGINE_TYPE_NVJPEG4: u32 = 58;
pub(crate) const RM_ENGINE_TYPE_NVJPEG5: u32 = 59;
pub(crate) const RM_ENGINE_TYPE_NVJPEG6: u32 = 60;
pub(crate) const RM_ENGINE_TYPE_NVJPEG7: u32 = 61;
pub(crate) const RM_ENGINE_TYPE_NVJPEG_SIZE: u32 = 8;
pub(crate) const RM_ENGINE_TYPE_NVJPG: u32 = RM_ENGINE_TYPE_NVJPEG0;
pub(crate) const RM_ENGINE_TYPE_OFA0: u32 = 62;
pub(crate) const RM_ENGINE_TYPE_OFA1: u32 = 63;
pub(crate) const RM_ENGINE_TYPE_OFA_SIZE: u32 = 2;
pub(crate) const RM_ENGINE_TYPE_PMU: u32 = 52;
pub(crate) const RM_ENGINE_TYPE_PPP: u32 = 43;
pub(crate) const RM_ENGINE_TYPE_RESERVED40: u32 = 64;
pub(crate) const RM_ENGINE_TYPE_RESERVED41: u32 = 65;
pub(crate) const RM_ENGINE_TYPE_RESERVED42: u32 = 66;
pub(crate) const RM_ENGINE_TYPE_RESERVED43: u32 = 67;
pub(crate) const RM_ENGINE_TYPE_RESERVED44: u32 = 68;
pub(crate) const RM_ENGINE_TYPE_RESERVED45: u32 = 69;
pub(crate) const RM_ENGINE_TYPE_RESERVED46: u32 = 70;
pub(crate) const RM_ENGINE_TYPE_RESERVED47: u32 = 71;
pub(crate) const RM_ENGINE_TYPE_RESERVED48: u32 = 72;
pub(crate) const RM_ENGINE_TYPE_RESERVED49: u32 = 73;
pub(crate) const RM_ENGINE_TYPE_RESERVED4a: u32 = 74;
pub(crate) const RM_ENGINE_TYPE_RESERVED4b: u32 = 75;
pub(crate) const RM_ENGINE_TYPE_RESERVED4c: u32 = 76;
pub(crate) const RM_ENGINE_TYPE_RESERVED4d: u32 = 77;
pub(crate) const RM_ENGINE_TYPE_RESERVED4e: u32 = 78;
pub(crate) const RM_ENGINE_TYPE_RESERVED4f: u32 = 79;
pub(crate) const RM_ENGINE_TYPE_RESERVED50: u32 = 80;
pub(crate) const RM_ENGINE_TYPE_RESERVED51: u32 = 81;
pub(crate) const RM_ENGINE_TYPE_RESERVED52: u32 = 82;
pub(crate) const RM_ENGINE_TYPE_RESERVED53: u32 = 83;
pub(crate) const RM_ENGINE_TYPE_SEC2: u32 = 49;
pub(crate) const RM_ENGINE_TYPE_SW: u32 = 45;
pub(crate) const RM_ENGINE_TYPE_TSEC: u32 = 46;
pub(crate) const RM_ENGINE_TYPE_VIC: u32 = 47;
pub(crate) const RM_ENGINE_TYPE_VP: u32 = 41;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_DAC_RGB_CRT: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_DSI: u32 = 0x00000011;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_PIOR_EXT_TMDS_ENC: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_DP_A: u32 = 0x00000008;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_DP_B: u32 = 0x00000009;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_DSI: u32 = 0x00000010;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_DUAL_TMDS: u32 = 0x00000005;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_LVDS_CUSTOM: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_SINGLE_TMDS_A: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_SOR_SINGLE_TMDS_B: u32 = 0x00000002;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_PROTOCOL_UNKNOWN: u32 = 0xFFFFFFFF;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_LOCATION_BOARD: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_LOCATION_CHIP: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_TYPE_DAC: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_TYPE_DSI: u32 = 0x00000005;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_TYPE_NONE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_TYPE_PIOR: u32 = 0x00000003;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_TYPE_SOR: u32 = 0x00000002;

pub(crate) struct s_rpc_message_rpc_union_field_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_message_rpc_union_field_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn spare(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_spare(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_spare(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cpuRmGfid(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cpuRmGfid(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_cpuRmGfid(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_message_header_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_message_header_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn header_version(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_header_version(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_header_version(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn signature(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_signature(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_signature(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn length(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_length(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_length(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn function(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_function(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_function(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rpc_result(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rpc_result(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_rpc_result(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rpc_result_private(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rpc_result_private(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_rpc_result_private(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn sequence(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sequence(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_sequence(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_u(&mut self) -> s_rpc_message_rpc_union_field_v03_00<'s> {
        s_rpc_message_rpc_union_field_v03_00::new(unsafe { self.ptr.byte_offset(28) })
    }
}

pub(crate) struct s_rpc_update_bar_pde_v15_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_update_bar_pde_v15_00<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn new_S_info(&mut self) -> s_UpdateBarPde_v15_00<'s> {
        s_UpdateBarPde_v15_00::new(unsafe { self.ptr.byte_offset(0) })
    }
}

pub(crate) struct s_UpdateBarPde_v15_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_UpdateBarPde_v15_00<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn barType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_barType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_barType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn entryValue(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_entryValue(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_entryValue(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn entryLevelShift(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_entryLevelShift(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_entryLevelShift(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GspFwWprMeta<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GspFwWprMeta<'s> {
    pub(crate) const fn str_size() -> usize {
        256
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 256) },
        }
    }

    pub(crate) fn magic(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_magic(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_magic(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn revision(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_revision(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_revision(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sysmemAddrOfRadix3Elf(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfRadix3Elf(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfRadix3Elf(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sizeOfRadix3Elf(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sizeOfRadix3Elf(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_sizeOfRadix3Elf(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sysmemAddrOfBootloader(self, fld: u64) -> Self {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfBootloader(&self) -> u64 {
        u64::from_le_bytes(self.store[32..40].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfBootloader(&mut self, fld: u64) {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sizeOfBootloader(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sizeOfBootloader(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_sizeOfBootloader(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bootloaderCodeOffset(self, fld: u64) -> Self {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootloaderCodeOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[48..56].try_into().unwrap())
    }
    pub(crate) fn set_bootloaderCodeOffset(&mut self, fld: u64) {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bootloaderDataOffset(self, fld: u64) -> Self {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootloaderDataOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[56..64].try_into().unwrap())
    }
    pub(crate) fn set_bootloaderDataOffset(&mut self, fld: u64) {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bootloaderManifestOffset(self, fld: u64) -> Self {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootloaderManifestOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[64..72].try_into().unwrap())
    }
    pub(crate) fn set_bootloaderManifestOffset(&mut self, fld: u64) {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sysmemAddrOfSignature(self, fld: u64) -> Self {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfSignature(&self) -> u64 {
        u64::from_le_bytes(self.store[72..80].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfSignature(&mut self, fld: u64) {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sizeOfSignature(self, fld: u64) -> Self {
        self.store[80..88].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sizeOfSignature(&self) -> u64 {
        u64::from_le_bytes(self.store[80..88].try_into().unwrap())
    }
    pub(crate) fn set_sizeOfSignature(&mut self, fld: u64) {
        self.store[80..88].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwHeapFreeListWprOffset(self, fld: u32) -> Self {
        self.store[72..76].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwHeapFreeListWprOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[72..76].try_into().unwrap())
    }
    pub(crate) fn set_gspFwHeapFreeListWprOffset(&mut self, fld: u32) {
        self.store[72..76].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn unused0(self, fld: u32) -> Self {
        self.store[76..80].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_unused0(&self) -> u32 {
        u32::from_le_bytes(self.store[76..80].try_into().unwrap())
    }
    pub(crate) fn set_unused0(&mut self, fld: u32) {
        self.store[76..80].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn unused1(self, fld: u64) -> Self {
        self.store[80..88].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_unused1(&self) -> u64 {
        u64::from_le_bytes(self.store[80..88].try_into().unwrap())
    }
    pub(crate) fn set_unused1(&mut self, fld: u64) {
        self.store[80..88].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwRsvdStart(self, fld: u64) -> Self {
        self.store[88..96].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwRsvdStart(&self) -> u64 {
        u64::from_le_bytes(self.store[88..96].try_into().unwrap())
    }
    pub(crate) fn set_gspFwRsvdStart(&mut self, fld: u64) {
        self.store[88..96].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn nonWprHeapOffset(self, fld: u64) -> Self {
        self.store[96..104].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nonWprHeapOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[96..104].try_into().unwrap())
    }
    pub(crate) fn set_nonWprHeapOffset(&mut self, fld: u64) {
        self.store[96..104].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn nonWprHeapSize(self, fld: u64) -> Self {
        self.store[104..112].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nonWprHeapSize(&self) -> u64 {
        u64::from_le_bytes(self.store[104..112].try_into().unwrap())
    }
    pub(crate) fn set_nonWprHeapSize(&mut self, fld: u64) {
        self.store[104..112].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwWprStart(self, fld: u64) -> Self {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwWprStart(&self) -> u64 {
        u64::from_le_bytes(self.store[112..120].try_into().unwrap())
    }
    pub(crate) fn set_gspFwWprStart(&mut self, fld: u64) {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwHeapOffset(self, fld: u64) -> Self {
        self.store[120..128].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwHeapOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[120..128].try_into().unwrap())
    }
    pub(crate) fn set_gspFwHeapOffset(&mut self, fld: u64) {
        self.store[120..128].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwHeapSize(self, fld: u64) -> Self {
        self.store[128..136].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwHeapSize(&self) -> u64 {
        u64::from_le_bytes(self.store[128..136].try_into().unwrap())
    }
    pub(crate) fn set_gspFwHeapSize(&mut self, fld: u64) {
        self.store[128..136].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwOffset(self, fld: u64) -> Self {
        self.store[136..144].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[136..144].try_into().unwrap())
    }
    pub(crate) fn set_gspFwOffset(&mut self, fld: u64) {
        self.store[136..144].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bootBinOffset(self, fld: u64) -> Self {
        self.store[144..152].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootBinOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[144..152].try_into().unwrap())
    }
    pub(crate) fn set_bootBinOffset(&mut self, fld: u64) {
        self.store[144..152].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn frtsOffset(self, fld: u64) -> Self {
        self.store[152..160].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_frtsOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[152..160].try_into().unwrap())
    }
    pub(crate) fn set_frtsOffset(&mut self, fld: u64) {
        self.store[152..160].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn frtsSize(self, fld: u64) -> Self {
        self.store[160..168].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_frtsSize(&self) -> u64 {
        u64::from_le_bytes(self.store[160..168].try_into().unwrap())
    }
    pub(crate) fn set_frtsSize(&mut self, fld: u64) {
        self.store[160..168].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspFwWprEnd(self, fld: u64) -> Self {
        self.store[168..176].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwWprEnd(&self) -> u64 {
        u64::from_le_bytes(self.store[168..176].try_into().unwrap())
    }
    pub(crate) fn set_gspFwWprEnd(&mut self, fld: u64) {
        self.store[168..176].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn fbSize(self, fld: u64) -> Self {
        self.store[176..184].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fbSize(&self) -> u64 {
        u64::from_le_bytes(self.store[176..184].try_into().unwrap())
    }
    pub(crate) fn set_fbSize(&mut self, fld: u64) {
        self.store[176..184].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vgaWorkspaceOffset(self, fld: u64) -> Self {
        self.store[184..192].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgaWorkspaceOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[184..192].try_into().unwrap())
    }
    pub(crate) fn set_vgaWorkspaceOffset(&mut self, fld: u64) {
        self.store[184..192].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vgaWorkspaceSize(self, fld: u64) -> Self {
        self.store[192..200].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgaWorkspaceSize(&self) -> u64 {
        u64::from_le_bytes(self.store[192..200].try_into().unwrap())
    }
    pub(crate) fn set_vgaWorkspaceSize(&mut self, fld: u64) {
        self.store[192..200].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bootCount(self, fld: u64) -> Self {
        self.store[200..208].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootCount(&self) -> u64 {
        u64::from_le_bytes(self.store[200..208].try_into().unwrap())
    }
    pub(crate) fn set_bootCount(&mut self, fld: u64) {
        self.store[200..208].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn partitionRpcAddr(self, fld: u64) -> Self {
        self.store[208..216].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_partitionRpcAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[208..216].try_into().unwrap())
    }
    pub(crate) fn set_partitionRpcAddr(&mut self, fld: u64) {
        self.store[208..216].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn partitionRpcRequestOffset(self, fld: u16) -> Self {
        self.store[216..218].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_partitionRpcRequestOffset(&self) -> u16 {
        u16::from_le_bytes(self.store[216..218].try_into().unwrap())
    }
    pub(crate) fn set_partitionRpcRequestOffset(&mut self, fld: u16) {
        self.store[216..218].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn partitionRpcReplyOffset(self, fld: u16) -> Self {
        self.store[218..220].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_partitionRpcReplyOffset(&self) -> u16 {
        u16::from_le_bytes(self.store[218..220].try_into().unwrap())
    }
    pub(crate) fn set_partitionRpcReplyOffset(&mut self, fld: u16) {
        self.store[218..220].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn elfCodeOffset(self, fld: u32) -> Self {
        self.store[220..224].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_elfCodeOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[220..224].try_into().unwrap())
    }
    pub(crate) fn set_elfCodeOffset(&mut self, fld: u32) {
        self.store[220..224].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn elfDataOffset(self, fld: u32) -> Self {
        self.store[224..228].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_elfDataOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[224..228].try_into().unwrap())
    }
    pub(crate) fn set_elfDataOffset(&mut self, fld: u32) {
        self.store[224..228].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn elfCodeSize(self, fld: u32) -> Self {
        self.store[228..232].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_elfCodeSize(&self) -> u32 {
        u32::from_le_bytes(self.store[228..232].try_into().unwrap())
    }
    pub(crate) fn set_elfCodeSize(&mut self, fld: u32) {
        self.store[228..232].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn elfDataSize(self, fld: u32) -> Self {
        self.store[232..236].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_elfDataSize(&self) -> u32 {
        u32::from_le_bytes(self.store[232..236].try_into().unwrap())
    }
    pub(crate) fn set_elfDataSize(&mut self, fld: u32) {
        self.store[232..236].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn lsUcodeVersion(self, fld: u32) -> Self {
        self.store[236..240].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_lsUcodeVersion(&self) -> u32 {
        u32::from_le_bytes(self.store[236..240].try_into().unwrap())
    }
    pub(crate) fn set_lsUcodeVersion(&mut self, fld: u32) {
        self.store[236..240].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn partitionRpcPadding(self, fld: [u32; 4]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[208..224].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_partitionRpcPadding(&mut self, fld: [u32; 4]) {
        let mut byte_data = [0u8; 16];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[208..224].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_partitionRpcPadding(&mut self) -> [u32; 4] {
        let mut array = [0u32; 4];
        for (i, chunk) in self.store[208..224].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn sysmemAddrOfCrashReportQueue(self, fld: u64) -> Self {
        self.store[224..232].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfCrashReportQueue(&self) -> u64 {
        u64::from_le_bytes(self.store[224..232].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfCrashReportQueue(&mut self, fld: u64) {
        self.store[224..232].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sizeOfCrashReportQueue(self, fld: u32) -> Self {
        self.store[232..236].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sizeOfCrashReportQueue(&self) -> u32 {
        u32::from_le_bytes(self.store[232..236].try_into().unwrap())
    }
    pub(crate) fn set_sizeOfCrashReportQueue(&mut self, fld: u32) {
        self.store[232..236].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn lsUcodeVersionPadding(self, fld: [u32; 1]) -> Self {
        let mut byte_data = [0u8; 4];
        for i in 0..1 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[236..240].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_lsUcodeVersionPadding(&mut self, fld: [u32; 1]) {
        let mut byte_data = [0u8; 4];
        for i in 0..1 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[236..240].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_lsUcodeVersionPadding(&mut self) -> [u32; 1] {
        let mut array = [0u32; 1];
        for (i, chunk) in self.store[236..240].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn gspFwHeapVfPartitionCount(self, fld: u8) -> Self {
        self.store[240..241].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspFwHeapVfPartitionCount(&self) -> u8 {
        u8::from_le_bytes(self.store[240..241].try_into().unwrap())
    }
    pub(crate) fn set_gspFwHeapVfPartitionCount(&mut self, fld: u8) {
        self.store[240..241].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u8) -> Self {
        self.store[241..242].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u8 {
        u8::from_le_bytes(self.store[241..242].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u8) {
        self.store[241..242].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn padding(self, fld: [u8; 2]) -> Self {
        let mut byte_data = [0u8; 2];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[242..244].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_padding(&mut self, fld: [u8; 2]) {
        let mut byte_data = [0u8; 2];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[242..244].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_padding(&mut self) -> [u8; 2] {
        let mut array = [0u8; 2];
        for (i, chunk) in self.store[242..244].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn pmuReservedSize(self, fld: u32) -> Self {
        self.store[244..248].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pmuReservedSize(&self) -> u32 {
        u32::from_le_bytes(self.store[244..248].try_into().unwrap())
    }
    pub(crate) fn set_pmuReservedSize(&mut self, fld: u32) {
        self.store[244..248].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn verified(self, fld: u64) -> Self {
        self.store[248..256].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_verified(&self) -> u64 {
        u64::from_le_bytes(self.store[248..256].try_into().unwrap())
    }
    pub(crate) fn set_verified(&mut self, fld: u64) {
        self.store[248..256].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GspFwSRMeta<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GspFwSRMeta<'s> {
    pub(crate) const fn str_size() -> usize {
        256
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 256) },
        }
    }

    pub(crate) fn magic(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_magic(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_magic(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn revision(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_revision(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_revision(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sysmemAddrOfSuspendResumeData(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfSuspendResumeData(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfSuspendResumeData(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn sizeOfSuspendResumeData(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sizeOfSuspendResumeData(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_sizeOfSuspendResumeData(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn internal(self, fld: [u32; 32]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[32..160].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_internal(&mut self, fld: [u32; 32]) {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[32..160].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_internal(&mut self) -> [u32; 32] {
        let mut array = [0u32; 32];
        for (i, chunk) in self.store[32..160].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[160..164].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[160..164].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[160..164].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn subrevision(self, fld: u32) -> Self {
        self.store[164..168].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subrevision(&self) -> u32 {
        u32::from_le_bytes(self.store[164..168].try_into().unwrap())
    }
    pub(crate) fn set_subrevision(&mut self, fld: u32) {
        self.store[164..168].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn padding(self, fld: [u32; 22]) -> Self {
        let mut byte_data = [0u8; 88];
        for i in 0..22 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[168..256].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_padding(&mut self, fld: [u32; 22]) {
        let mut byte_data = [0u8; 88];
        for i in 0..22 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[168..256].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_padding(&mut self) -> [u32; 22] {
        let mut array = [0u32; 22];
        for (i, chunk) in self.store[168..256].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_MESSAGE_QUEUE_INIT_ARGUMENTS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_MESSAGE_QUEUE_INIT_ARGUMENTS<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn sharedMemPhysAddr(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sharedMemPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_sharedMemPhysAddr(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pageTableEntryCount(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pageTableEntryCount(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_pageTableEntryCount(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cmdQueueOffset(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cmdQueueOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_cmdQueueOffset(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn statQueueOffset(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_statQueueOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_statQueueOffset(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SR_INIT_ARGUMENTS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SR_INIT_ARGUMENTS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn oldLevel(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_oldLevel(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_oldLevel(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bInPMTransition(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bInPMTransition(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bInPMTransition(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_ARGUMENTS_CACHED<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_ARGUMENTS_CACHED<'s> {
    pub(crate) const fn str_size() -> usize {
        72
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 72) },
        }
    }

    pub(crate) fn new_S_messageQueueInitArguments(&mut self) -> s_MESSAGE_QUEUE_INIT_ARGUMENTS<'s> {
        s_MESSAGE_QUEUE_INIT_ARGUMENTS::new(unsafe { self.ptr.byte_offset(0) })
    }

    pub(crate) fn new_S_srInitArguments(&mut self) -> s_GSP_SR_INIT_ARGUMENTS<'s> {
        s_GSP_SR_INIT_ARGUMENTS::new(unsafe { self.ptr.byte_offset(32) })
    }

    pub(crate) fn gpuInstance(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_gpuInstance(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bDmemStack(self, fld: u8) -> Self {
        self.store[48..49].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bDmemStack(&self) -> u8 {
        u8::from_le_bytes(self.store[48..49].try_into().unwrap())
    }
    pub(crate) fn set_bDmemStack(&mut self, fld: u8) {
        self.store[48..49].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn profilerArgs_pa(self, fld: u64) -> Self {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_profilerArgs_pa(&self) -> u64 {
        u64::from_le_bytes(self.store[56..64].try_into().unwrap())
    }
    pub(crate) fn set_profilerArgs_pa(&mut self, fld: u64) {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn profilerArgs_size(self, fld: u64) -> Self {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_profilerArgs_size(&self) -> u64 {
        u64::from_le_bytes(self.store[64..72].try_into().unwrap())
    }
    pub(crate) fn set_profilerArgs_size(&mut self, fld: u64) {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_MSG_QUEUE_ELEMENT<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_MSG_QUEUE_ELEMENT<'s> {
    pub(crate) const fn str_size() -> usize {
        80
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 80) },
        }
    }

    pub(crate) fn authTagBuffer(self, fld: [u8; 16]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[0..16].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_authTagBuffer(&mut self, fld: [u8; 16]) {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[0..16].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_authTagBuffer(&mut self) -> [u8; 16] {
        let mut array = [0u8; 16];
        for (i, chunk) in self.store[0..16].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn aadBuffer(self, fld: [u8; 16]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[16..32].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_aadBuffer(&mut self, fld: [u8; 16]) {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[16..32].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_aadBuffer(&mut self) -> [u8; 16] {
        let mut array = [0u8; 16];
        for (i, chunk) in self.store[16..32].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn checkSum(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_checkSum(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_checkSum(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn seqNum(self, fld: u32) -> Self {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_seqNum(&self) -> u32 {
        u32::from_le_bytes(self.store[36..40].try_into().unwrap())
    }
    pub(crate) fn set_seqNum(&mut self, fld: u32) {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn elemCount(self, fld: u32) -> Self {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_elemCount(&self) -> u32 {
        u32::from_le_bytes(self.store[40..44].try_into().unwrap())
    }
    pub(crate) fn set_elemCount(&mut self, fld: u32) {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_rpc(&mut self) -> s_rpc_message_header_v03_00<'s> {
        s_rpc_message_header_v03_00::new(unsafe { self.ptr.byte_offset(48) })
    }
}

pub(crate) struct s_LibosMemoryRegionInitArgument<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_LibosMemoryRegionInitArgument<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn id8(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_id8(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_id8(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pa(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pa(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_pa(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn size(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn kind(self, fld: u8) -> Self {
        self.store[24..25].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_kind(&self) -> u8 {
        u8::from_le_bytes(self.store[24..25].try_into().unwrap())
    }
    pub(crate) fn set_kind(&mut self, fld: u8) {
        self.store[24..25].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn loc(self, fld: u8) -> Self {
        self.store[25..26].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_loc(&self) -> u8 {
        u8::from_le_bytes(self.store[25..26].try_into().unwrap())
    }
    pub(crate) fn set_loc(&mut self, fld: u8) {
        self.store[25..26].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_gsp_rm_alloc_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_gsp_rm_alloc_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hParent(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hParent(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hParent(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObject(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObject(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_hObject(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hClass(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClass(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_hClass(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn paramsSize(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_paramsSize(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_paramsSize(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn reserved(self, fld: [u8; 4]) -> Self {
        let mut byte_data = [0u8; 4];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[28..32].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_reserved(&mut self, fld: [u8; 4]) {
        let mut byte_data = [0u8; 4];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[28..32].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_reserved(&mut self) -> [u8; 4] {
        let mut array = [0u8; 4];
        for (i, chunk) in self.store[28..32].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_rpc_gsp_rm_control_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_gsp_rm_control_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObject(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObject(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hObject(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cmd(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cmd(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_cmd(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn paramsSize(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_paramsSize(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_paramsSize(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_free_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_free_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn new_S_params(&mut self) -> s_NVOS00_PARAMETERS_v03_00<'s> {
        s_NVOS00_PARAMETERS_v03_00::new(unsafe { self.ptr.byte_offset(0) })
    }
}

pub(crate) struct s_NVOS00_PARAMETERS_v03_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NVOS00_PARAMETERS_v03_00<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn hRoot(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hRoot(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hRoot(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectParent(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectParent(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hObjectParent(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectOld(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectOld(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_hObjectOld(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQ_BUF_PAYLOAD_REG_WRITE<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQ_BUF_PAYLOAD_REG_WRITE<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn addr(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addr(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_addr(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn val(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_val(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_val(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQ_BUF_PAYLOAD_REG_MODIFY<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQ_BUF_PAYLOAD_REG_MODIFY<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn addr(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addr(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_addr(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_mask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn val(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_val(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_val(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQ_BUF_PAYLOAD_REG_POLL<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQ_BUF_PAYLOAD_REG_POLL<'s> {
    pub(crate) const fn str_size() -> usize {
        20
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 20) },
        }
    }

    pub(crate) fn addr(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addr(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_addr(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_mask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn val(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_val(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_val(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn timeout(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_timeout(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_timeout(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn error(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_error(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_error(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQ_BUF_PAYLOAD_DELAY_US<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQ_BUF_PAYLOAD_DELAY_US<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn val(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_val(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_val(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQ_BUF_PAYLOAD_REG_STORE<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQ_BUF_PAYLOAD_REG_STORE<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn addr(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addr(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_addr(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn index(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_index(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_index(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SEQUENCER_BUFFER_CMD<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SEQUENCER_BUFFER_CMD<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn opCode(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_opCode(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_opCode(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_payload_regWrite(&mut self) -> s_GSP_SEQ_BUF_PAYLOAD_REG_WRITE<'s> {
        s_GSP_SEQ_BUF_PAYLOAD_REG_WRITE::new(unsafe { self.ptr.byte_offset(4) })
    }

    pub(crate) fn new_S_payload_regModify(&mut self) -> s_GSP_SEQ_BUF_PAYLOAD_REG_MODIFY<'s> {
        s_GSP_SEQ_BUF_PAYLOAD_REG_MODIFY::new(unsafe { self.ptr.byte_offset(4) })
    }

    pub(crate) fn new_S_payload_regPoll(&mut self) -> s_GSP_SEQ_BUF_PAYLOAD_REG_POLL<'s> {
        s_GSP_SEQ_BUF_PAYLOAD_REG_POLL::new(unsafe { self.ptr.byte_offset(4) })
    }

    pub(crate) fn new_S_payload_delayUs(&mut self) -> s_GSP_SEQ_BUF_PAYLOAD_DELAY_US<'s> {
        s_GSP_SEQ_BUF_PAYLOAD_DELAY_US::new(unsafe { self.ptr.byte_offset(4) })
    }

    pub(crate) fn new_S_payload_regStore(&mut self) -> s_GSP_SEQ_BUF_PAYLOAD_REG_STORE<'s> {
        s_GSP_SEQ_BUF_PAYLOAD_REG_STORE::new(unsafe { self.ptr.byte_offset(4) })
    }
}

pub(crate) struct s_rpc_run_cpu_sequencer_v17_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_run_cpu_sequencer_v17_00<'s> {
    pub(crate) const fn str_size() -> usize {
        40
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 40) },
        }
    }

    pub(crate) fn bufferSizeDWord(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bufferSizeDWord(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_bufferSizeDWord(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cmdIndex(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cmdIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_cmdIndex(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn regSaveArea(self, fld: [u32; 8]) -> Self {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[8..40].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_regSaveArea(&mut self, fld: [u32; 8]) {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[8..40].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_regSaveArea(&mut self) -> [u32; 8] {
        let mut array = [0u32; 8];
        for (i, chunk) in self.store[8..40].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_rpc_post_event_v17_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_post_event_v17_00<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hEvent(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hEvent(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hEvent(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn notifyIndex(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_notifyIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_notifyIndex(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_data(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn info16(self, fld: u16) -> Self {
        self.store[16..18].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_info16(&self) -> u16 {
        u16::from_le_bytes(self.store[16..18].try_into().unwrap())
    }
    pub(crate) fn set_info16(&mut self, fld: u16) {
        self.store[16..18].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn eventDataSize(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_eventDataSize(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_eventDataSize(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bNotifyList(self, fld: u8) -> Self {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bNotifyList(&self) -> u8 {
        u8::from_le_bytes(self.store[28..29].try_into().unwrap())
    }
    pub(crate) fn set_bNotifyList(&mut self, fld: u8) {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_os_error_log_v17_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_os_error_log_v17_00<'s> {
    pub(crate) const fn str_size() -> usize {
        272
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 272) },
        }
    }

    pub(crate) fn exceptType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_exceptType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_exceptType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn runlistId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_runlistId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_runlistId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn chid(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_chid(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_chid(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn errString(self, fld: [u8; 256]) -> Self {
        let mut byte_data = [0u8; 256];
        for i in 0..256 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[12..268].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_errString(&mut self, fld: [u8; 256]) {
        let mut byte_data = [0u8; 256];
        for i in 0..256 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[12..268].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_errString(&mut self) -> [u8; 256] {
        let mut array = [0u8; 256];
        for (i, chunk) in self.store[12..268].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn preemptiveRemovalPreviousXid(self, fld: u32) -> Self {
        self.store[268..272].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_preemptiveRemovalPreviousXid(&self) -> u32 {
        u32::from_le_bytes(self.store[268..272].try_into().unwrap())
    }
    pub(crate) fn set_preemptiveRemovalPreviousXid(&mut self, fld: u32) {
        self.store[268..272].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_gsp_post_nocat_record_v01_00<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_gsp_post_nocat_record_v01_00<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn data(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_data(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_unloading_guest_driver_v1F_07<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_unloading_guest_driver_v1F_07<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn bInPMTransition(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bInPMTransition(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_bInPMTransition(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bGc6Entering(self, fld: u8) -> Self {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bGc6Entering(&self) -> u8 {
        u8::from_le_bytes(self.store[1..2].try_into().unwrap())
    }
    pub(crate) fn set_bGc6Entering(&mut self, fld: u8) {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn newLevel(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_newLevel(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_newLevel(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_rpc_rc_triggered_v17_02<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_rpc_rc_triggered_v17_02<'s> {
    pub(crate) const fn str_size() -> usize {
        48
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 48) },
        }
    }

    pub(crate) fn nv2080EngineType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nv2080EngineType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_nv2080EngineType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn chid(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_chid(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_chid(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gfid(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gfid(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_gfid(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn exceptLevel(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_exceptLevel(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_exceptLevel(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn exceptType(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_exceptType(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_exceptType(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn scope(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_scope(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_scope(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn partitionAttributionId(self, fld: u16) -> Self {
        self.store[24..26].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_partitionAttributionId(&self) -> u16 {
        u16::from_le_bytes(self.store[24..26].try_into().unwrap())
    }
    pub(crate) fn set_partitionAttributionId(&mut self, fld: u16) {
        self.store[24..26].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn mmuFaultAddrLo(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mmuFaultAddrLo(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_mmuFaultAddrLo(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mmuFaultAddrHi(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mmuFaultAddrHi(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_mmuFaultAddrHi(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mmuFaultType(self, fld: u32) -> Self {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mmuFaultType(&self) -> u32 {
        u32::from_le_bytes(self.store[36..40].try_into().unwrap())
    }
    pub(crate) fn set_mmuFaultType(&mut self, fld: u32) {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bCallbackNeeded(self, fld: u8) -> Self {
        self.store[40..41].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bCallbackNeeded(&self) -> u8 {
        u8::from_le_bytes(self.store[40..41].try_into().unwrap())
    }
    pub(crate) fn set_bCallbackNeeded(&mut self, fld: u8) {
        self.store[40..41].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn rcJournalBufferSize(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rcJournalBufferSize(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_rcJournalBufferSize(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_PACKED_REGISTRY_ENTRY<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_PACKED_REGISTRY_ENTRY<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn nameOffset(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nameOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_nameOffset(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rtype(self, fld: u8) -> Self {
        self.store[4..5].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rtype(&self) -> u8 {
        u8::from_le_bytes(self.store[4..5].try_into().unwrap())
    }
    pub(crate) fn set_rtype(&mut self, fld: u8) {
        self.store[4..5].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn data(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_data(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn length(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_length(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_length(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_PACKED_REGISTRY_TABLE<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_PACKED_REGISTRY_TABLE<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numEntries(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numEntries(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_numEntries(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_entries(&mut self, idx: isize) -> s_PACKED_REGISTRY_ENTRY<'s> {
        s_PACKED_REGISTRY_ENTRY::new(unsafe { self.ptr.byte_offset(idx * 16 + 8) })
    }
}

pub(crate) struct s_ACPI_METHOD_DATA<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_ACPI_METHOD_DATA<'s> {
    pub(crate) const fn str_size() -> usize {
        676
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 676) },
        }
    }

    pub(crate) fn bValid(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bValid(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_bValid(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_dodMethodData(&mut self) -> s_DOD_METHOD_DATA<'s> {
        s_DOD_METHOD_DATA::new(unsafe { self.ptr.byte_offset(4) })
    }

    pub(crate) fn new_S_jtMethodData(&mut self) -> s_JT_METHOD_DATA<'s> {
        s_JT_METHOD_DATA::new(unsafe { self.ptr.byte_offset(76) })
    }

    pub(crate) fn new_S_muxMethodData(&mut self) -> s_MUX_METHOD_DATA<'s> {
        s_MUX_METHOD_DATA::new(unsafe { self.ptr.byte_offset(88) })
    }

    pub(crate) fn new_S_capsMethodData(&mut self) -> s_CAPS_METHOD_DATA<'s> {
        s_CAPS_METHOD_DATA::new(unsafe { self.ptr.byte_offset(668) })
    }
}

pub(crate) struct s_DOD_METHOD_DATA<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_DOD_METHOD_DATA<'s> {
    pub(crate) const fn str_size() -> usize {
        72
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 72) },
        }
    }

    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn acpiIdListLen(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_acpiIdListLen(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_acpiIdListLen(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn acpiIdList(self, fld: [u32; 16]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[8..72].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_acpiIdList(&mut self, fld: [u32; 16]) {
        let mut byte_data = [0u8; 64];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[8..72].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_acpiIdList(&mut self) -> [u32; 16] {
        let mut array = [0u32; 16];
        for (i, chunk) in self.store[8..72].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_JT_METHOD_DATA<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_JT_METHOD_DATA<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn jtCaps(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_jtCaps(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_jtCaps(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn jtRevId(self, fld: u16) -> Self {
        self.store[8..10].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_jtRevId(&self) -> u16 {
        u16::from_le_bytes(self.store[8..10].try_into().unwrap())
    }
    pub(crate) fn set_jtRevId(&mut self, fld: u16) {
        self.store[8..10].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn bSBIOSCaps(self, fld: u8) -> Self {
        self.store[10..11].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSBIOSCaps(&self) -> u8 {
        u8::from_le_bytes(self.store[10..11].try_into().unwrap())
    }
    pub(crate) fn set_bSBIOSCaps(&mut self, fld: u8) {
        self.store[10..11].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_MUX_METHOD_DATA_ELEMENT<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_MUX_METHOD_DATA_ELEMENT<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn acpiId(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_acpiId(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_acpiId(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mode(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mode(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_mode(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_MUX_METHOD_DATA<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_MUX_METHOD_DATA<'s> {
    pub(crate) const fn str_size() -> usize {
        580
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 580) },
        }
    }

    pub(crate) fn tableLen(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_tableLen(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_tableLen(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_acpiIdMuxModeTable(&mut self, idx: isize) -> s_MUX_METHOD_DATA_ELEMENT<'s> {
        s_MUX_METHOD_DATA_ELEMENT::new(unsafe { self.ptr.byte_offset(idx * 12 + 4) })
    }

    pub(crate) fn new_S_acpiIdMuxPartTable(&mut self, idx: isize) -> s_MUX_METHOD_DATA_ELEMENT<'s> {
        s_MUX_METHOD_DATA_ELEMENT::new(unsafe { self.ptr.byte_offset(idx * 12 + 196) })
    }

    pub(crate) fn new_S_acpiIdMuxStateTable(
        &mut self,
        idx: isize,
    ) -> s_MUX_METHOD_DATA_ELEMENT<'s> {
        s_MUX_METHOD_DATA_ELEMENT::new(unsafe { self.ptr.byte_offset(idx * 12 + 388) })
    }
}

pub(crate) struct s_CAPS_METHOD_DATA<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_CAPS_METHOD_DATA<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn status(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_status(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_status(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn optimusCaps(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_optimusCaps(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_optimusCaps(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_BUSINFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_BUSINFO<'s> {
    pub(crate) const fn str_size() -> usize {
        10
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 10) },
        }
    }

    pub(crate) fn deviceID(self, fld: u16) -> Self {
        self.store[0..2].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_deviceID(&self) -> u16 {
        u16::from_le_bytes(self.store[0..2].try_into().unwrap())
    }
    pub(crate) fn set_deviceID(&mut self, fld: u16) {
        self.store[0..2].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn vendorID(self, fld: u16) -> Self {
        self.store[2..4].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vendorID(&self) -> u16 {
        u16::from_le_bytes(self.store[2..4].try_into().unwrap())
    }
    pub(crate) fn set_vendorID(&mut self, fld: u16) {
        self.store[2..4].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn subdeviceID(self, fld: u16) -> Self {
        self.store[4..6].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subdeviceID(&self) -> u16 {
        u16::from_le_bytes(self.store[4..6].try_into().unwrap())
    }
    pub(crate) fn set_subdeviceID(&mut self, fld: u16) {
        self.store[4..6].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn subvendorID(self, fld: u16) -> Self {
        self.store[6..8].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subvendorID(&self) -> u16 {
        u16::from_le_bytes(self.store[6..8].try_into().unwrap())
    }
    pub(crate) fn set_subvendorID(&mut self, fld: u16) {
        self.store[6..8].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn revisionID(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_revisionID(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_revisionID(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_VF_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_VF_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        40
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 40) },
        }
    }

    pub(crate) fn totalVFs(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_totalVFs(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_totalVFs(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn firstVFOffset(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_firstVFOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_firstVFOffset(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar0Address(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar0Address(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar0Address(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar1Address(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar1Address(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar1Address(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar2Address(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar2Address(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar2Address(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar0(self, fld: u8) -> Self {
        self.store[32..33].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar0(&self) -> u8 {
        u8::from_le_bytes(self.store[32..33].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar0(&mut self, fld: u8) {
        self.store[32..33].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar1(self, fld: u8) -> Self {
        self.store[33..34].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar1(&self) -> u8 {
        u8::from_le_bytes(self.store[33..34].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar1(&mut self, fld: u8) {
        self.store[33..34].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar2(self, fld: u8) -> Self {
        self.store[34..35].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar2(&self) -> u8 {
        u8::from_le_bytes(self.store[34..35].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar2(&mut self, fld: u8) {
        self.store[34..35].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_EcidManufacturingInfo<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_EcidManufacturingInfo<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn ecidLow(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ecidLow(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_ecidLow(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ecidHigh(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ecidHigh(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_ecidHigh(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ecidExtended(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ecidExtended(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_ecidExtended(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GspStaticConfigInfo<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GspStaticConfigInfo<'s> {
    pub(crate) const fn str_size() -> usize {
        1656
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 1656) },
        }
    }

    pub(crate) fn grCapsBits(self, fld: [u8; 23]) -> Self {
        let mut byte_data = [0u8; 23];
        for i in 0..23 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[0..23].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_grCapsBits(&mut self, fld: [u8; 23]) {
        let mut byte_data = [0u8; 23];
        for i in 0..23 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[0..23].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_grCapsBits(&mut self) -> [u8; 23] {
        let mut array = [0u8; 23];
        for (i, chunk) in self.store[0..23].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }

    pub(crate) fn new_S_gidInfo(&mut self) -> s_NV2080_CTRL_GPU_GET_GID_INFO_PARAMS<'s> {
        s_NV2080_CTRL_GPU_GET_GID_INFO_PARAMS::new(unsafe { self.ptr.byte_offset(24) })
    }

    pub(crate) fn new_S_SKUInfo(&mut self) -> s_NV2080_CTRL_BIOS_GET_SKU_INFO_PARAMS<'s> {
        s_NV2080_CTRL_BIOS_GET_SKU_INFO_PARAMS::new(unsafe { self.ptr.byte_offset(292) })
    }

    pub(crate) fn new_S_fbRegionInfoParams(
        &mut self,
    ) -> s_NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_PARAMS<'s> {
        s_NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_PARAMS::new(unsafe { self.ptr.byte_offset(344) })
    }

    pub(crate) fn new_S_sriovCaps(&mut self) -> s_NV0080_CTRL_GPU_GET_SRIOV_CAPS_PARAMS<'s> {
        s_NV0080_CTRL_GPU_GET_SRIOV_CAPS_PARAMS::new(unsafe { self.ptr.byte_offset(1120) })
    }

    pub(crate) fn sriovMaxGfid(self, fld: u32) -> Self {
        self.store[1200..1204].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sriovMaxGfid(&self) -> u32 {
        u32::from_le_bytes(self.store[1200..1204].try_into().unwrap())
    }
    pub(crate) fn set_sriovMaxGfid(&mut self, fld: u32) {
        self.store[1200..1204].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineCaps(self, fld: [u32; 3]) -> Self {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[1204..1216].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_engineCaps(&mut self, fld: [u32; 3]) {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[1204..1216].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_engineCaps(&mut self) -> [u32; 3] {
        let mut array = [0u32; 3];
        for (i, chunk) in self.store[1204..1216].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn poisonFuseEnabled(self, fld: u8) -> Self {
        self.store[1216..1217].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_poisonFuseEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[1216..1217].try_into().unwrap())
    }
    pub(crate) fn set_poisonFuseEnabled(&mut self, fld: u8) {
        self.store[1216..1217].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn fb_length(self, fld: u64) -> Self {
        self.store[1224..1232].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fb_length(&self) -> u64 {
        u64::from_le_bytes(self.store[1224..1232].try_into().unwrap())
    }
    pub(crate) fn set_fb_length(&mut self, fld: u64) {
        self.store[1224..1232].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn fbio_mask(self, fld: u64) -> Self {
        self.store[1232..1240].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fbio_mask(&self) -> u64 {
        u64::from_le_bytes(self.store[1232..1240].try_into().unwrap())
    }
    pub(crate) fn set_fbio_mask(&mut self, fld: u64) {
        self.store[1232..1240].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn fb_bus_width(self, fld: u32) -> Self {
        self.store[1240..1244].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fb_bus_width(&self) -> u32 {
        u32::from_le_bytes(self.store[1240..1244].try_into().unwrap())
    }
    pub(crate) fn set_fb_bus_width(&mut self, fld: u32) {
        self.store[1240..1244].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn fb_ram_type(self, fld: u32) -> Self {
        self.store[1244..1248].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fb_ram_type(&self) -> u32 {
        u32::from_le_bytes(self.store[1244..1248].try_into().unwrap())
    }
    pub(crate) fn set_fb_ram_type(&mut self, fld: u32) {
        self.store[1244..1248].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn fbp_mask(self, fld: u64) -> Self {
        self.store[1248..1256].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fbp_mask(&self) -> u64 {
        u64::from_le_bytes(self.store[1248..1256].try_into().unwrap())
    }
    pub(crate) fn set_fbp_mask(&mut self, fld: u64) {
        self.store[1248..1256].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn l2_cache_size(self, fld: u32) -> Self {
        self.store[1256..1260].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_l2_cache_size(&self) -> u32 {
        u32::from_le_bytes(self.store[1256..1260].try_into().unwrap())
    }
    pub(crate) fn set_l2_cache_size(&mut self, fld: u32) {
        self.store[1256..1260].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gpuNameString(self, fld: [u8; 64]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[1260..1324].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_gpuNameString(&mut self, fld: [u8; 64]) {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[1260..1324].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_gpuNameString(&mut self) -> [u8; 64] {
        let mut array = [0u8; 64];
        for (i, chunk) in self.store[1260..1324].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn gpuShortNameString(self, fld: [u8; 64]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[1324..1388].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_gpuShortNameString(&mut self, fld: [u8; 64]) {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[1324..1388].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_gpuShortNameString(&mut self) -> [u8; 64] {
        let mut array = [0u8; 64];
        for (i, chunk) in self.store[1324..1388].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn gpuNameString_Unicode(self, fld: [u16; 64]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 2)..((i + 1) * 2)].copy_from_slice(&bytes);
        }
        self.store[1388..1516].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_gpuNameString_Unicode(&mut self, fld: [u16; 64]) {
        let mut byte_data = [0u8; 128];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 2)..((i + 1) * 2)].copy_from_slice(&bytes);
        }
        self.store[1388..1516].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_gpuNameString_Unicode(&mut self) -> [u16; 64] {
        let mut array = [0u16; 64];
        for (i, chunk) in self.store[1388..1516].chunks_exact(2).enumerate() {
            array[i] = u16::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn bGpuInternalSku(self, fld: u8) -> Self {
        self.store[1516..1517].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bGpuInternalSku(&self) -> u8 {
        u8::from_le_bytes(self.store[1516..1517].try_into().unwrap())
    }
    pub(crate) fn set_bGpuInternalSku(&mut self, fld: u8) {
        self.store[1516..1517].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsQuadroGeneric(self, fld: u8) -> Self {
        self.store[1517..1518].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsQuadroGeneric(&self) -> u8 {
        u8::from_le_bytes(self.store[1517..1518].try_into().unwrap())
    }
    pub(crate) fn set_bIsQuadroGeneric(&mut self, fld: u8) {
        self.store[1517..1518].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsQuadroAd(self, fld: u8) -> Self {
        self.store[1518..1519].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsQuadroAd(&self) -> u8 {
        u8::from_le_bytes(self.store[1518..1519].try_into().unwrap())
    }
    pub(crate) fn set_bIsQuadroAd(&mut self, fld: u8) {
        self.store[1518..1519].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsNvidiaNvs(self, fld: u8) -> Self {
        self.store[1519..1520].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsNvidiaNvs(&self) -> u8 {
        u8::from_le_bytes(self.store[1519..1520].try_into().unwrap())
    }
    pub(crate) fn set_bIsNvidiaNvs(&mut self, fld: u8) {
        self.store[1519..1520].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsVgx(self, fld: u8) -> Self {
        self.store[1520..1521].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsVgx(&self) -> u8 {
        u8::from_le_bytes(self.store[1520..1521].try_into().unwrap())
    }
    pub(crate) fn set_bIsVgx(&mut self, fld: u8) {
        self.store[1520..1521].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bGeforceSmb(self, fld: u8) -> Self {
        self.store[1521..1522].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bGeforceSmb(&self) -> u8 {
        u8::from_le_bytes(self.store[1521..1522].try_into().unwrap())
    }
    pub(crate) fn set_bGeforceSmb(&mut self, fld: u8) {
        self.store[1521..1522].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsTitan(self, fld: u8) -> Self {
        self.store[1522..1523].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsTitan(&self) -> u8 {
        u8::from_le_bytes(self.store[1522..1523].try_into().unwrap())
    }
    pub(crate) fn set_bIsTitan(&mut self, fld: u8) {
        self.store[1522..1523].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsTesla(self, fld: u8) -> Self {
        self.store[1523..1524].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsTesla(&self) -> u8 {
        u8::from_le_bytes(self.store[1523..1524].try_into().unwrap())
    }
    pub(crate) fn set_bIsTesla(&mut self, fld: u8) {
        self.store[1523..1524].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsMobile(self, fld: u8) -> Self {
        self.store[1524..1525].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsMobile(&self) -> u8 {
        u8::from_le_bytes(self.store[1524..1525].try_into().unwrap())
    }
    pub(crate) fn set_bIsMobile(&mut self, fld: u8) {
        self.store[1524..1525].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsGc6Rtd3Allowed(self, fld: u8) -> Self {
        self.store[1525..1526].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGc6Rtd3Allowed(&self) -> u8 {
        u8::from_le_bytes(self.store[1525..1526].try_into().unwrap())
    }
    pub(crate) fn set_bIsGc6Rtd3Allowed(&mut self, fld: u8) {
        self.store[1525..1526].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsGc8Rtd3Allowed(self, fld: u8) -> Self {
        self.store[1526..1527].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGc8Rtd3Allowed(&self) -> u8 {
        u8::from_le_bytes(self.store[1526..1527].try_into().unwrap())
    }
    pub(crate) fn set_bIsGc8Rtd3Allowed(&mut self, fld: u8) {
        self.store[1526..1527].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsGcOffRtd3Allowed(self, fld: u8) -> Self {
        self.store[1527..1528].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGcOffRtd3Allowed(&self) -> u8 {
        u8::from_le_bytes(self.store[1527..1528].try_into().unwrap())
    }
    pub(crate) fn set_bIsGcOffRtd3Allowed(&mut self, fld: u8) {
        self.store[1527..1528].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsGcoffLegacyAllowed(self, fld: u8) -> Self {
        self.store[1528..1529].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGcoffLegacyAllowed(&self) -> u8 {
        u8::from_le_bytes(self.store[1528..1529].try_into().unwrap())
    }
    pub(crate) fn set_bIsGcoffLegacyAllowed(&mut self, fld: u8) {
        self.store[1528..1529].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsMigSupported(self, fld: u8) -> Self {
        self.store[1529..1530].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsMigSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[1529..1530].try_into().unwrap())
    }
    pub(crate) fn set_bIsMigSupported(&mut self, fld: u8) {
        self.store[1529..1530].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn RTD3GC6TotalBoardPower(self, fld: u16) -> Self {
        self.store[1530..1532].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_RTD3GC6TotalBoardPower(&self) -> u16 {
        u16::from_le_bytes(self.store[1530..1532].try_into().unwrap())
    }
    pub(crate) fn set_RTD3GC6TotalBoardPower(&mut self, fld: u16) {
        self.store[1530..1532].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn RTD3GC6PerstDelay(self, fld: u16) -> Self {
        self.store[1532..1534].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_RTD3GC6PerstDelay(&self) -> u16 {
        u16::from_le_bytes(self.store[1532..1534].try_into().unwrap())
    }
    pub(crate) fn set_RTD3GC6PerstDelay(&mut self, fld: u16) {
        self.store[1532..1534].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn bar1PdeBase(self, fld: u64) -> Self {
        self.store[1536..1544].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar1PdeBase(&self) -> u64 {
        u64::from_le_bytes(self.store[1536..1544].try_into().unwrap())
    }
    pub(crate) fn set_bar1PdeBase(&mut self, fld: u64) {
        self.store[1536..1544].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bar2PdeBase(self, fld: u64) -> Self {
        self.store[1544..1552].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar2PdeBase(&self) -> u64 {
        u64::from_le_bytes(self.store[1544..1552].try_into().unwrap())
    }
    pub(crate) fn set_bar2PdeBase(&mut self, fld: u64) {
        self.store[1544..1552].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bVbiosValid(self, fld: u8) -> Self {
        self.store[1552..1553].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bVbiosValid(&self) -> u8 {
        u8::from_le_bytes(self.store[1552..1553].try_into().unwrap())
    }
    pub(crate) fn set_bVbiosValid(&mut self, fld: u8) {
        self.store[1552..1553].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn vbiosSubVendor(self, fld: u32) -> Self {
        self.store[1556..1560].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vbiosSubVendor(&self) -> u32 {
        u32::from_le_bytes(self.store[1556..1560].try_into().unwrap())
    }
    pub(crate) fn set_vbiosSubVendor(&mut self, fld: u32) {
        self.store[1556..1560].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vbiosSubDevice(self, fld: u32) -> Self {
        self.store[1560..1564].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vbiosSubDevice(&self) -> u32 {
        u32::from_le_bytes(self.store[1560..1564].try_into().unwrap())
    }
    pub(crate) fn set_vbiosSubDevice(&mut self, fld: u32) {
        self.store[1560..1564].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bPageRetirementSupported(self, fld: u8) -> Self {
        self.store[1564..1565].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bPageRetirementSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[1564..1565].try_into().unwrap())
    }
    pub(crate) fn set_bPageRetirementSupported(&mut self, fld: u8) {
        self.store[1564..1565].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSplitVasBetweenServerClientRm(self, fld: u8) -> Self {
        self.store[1565..1566].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSplitVasBetweenServerClientRm(&self) -> u8 {
        u8::from_le_bytes(self.store[1565..1566].try_into().unwrap())
    }
    pub(crate) fn set_bSplitVasBetweenServerClientRm(&mut self, fld: u8) {
        self.store[1565..1566].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bClRootportNeedsNosnoopWAR(self, fld: u8) -> Self {
        self.store[1566..1567].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bClRootportNeedsNosnoopWAR(&self) -> u8 {
        u8::from_le_bytes(self.store[1566..1567].try_into().unwrap())
    }
    pub(crate) fn set_bClRootportNeedsNosnoopWAR(&mut self, fld: u8) {
        self.store[1566..1567].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_displaylessMaxHeads(
        &mut self,
    ) -> s_VIRTUAL_DISPLAY_GET_NUM_HEADS_PARAMS<'s> {
        s_VIRTUAL_DISPLAY_GET_NUM_HEADS_PARAMS::new(unsafe { self.ptr.byte_offset(1568) })
    }

    pub(crate) fn new_S_displaylessMaxResolution(
        &mut self,
    ) -> s_VIRTUAL_DISPLAY_GET_MAX_RESOLUTION_PARAMS<'s> {
        s_VIRTUAL_DISPLAY_GET_MAX_RESOLUTION_PARAMS::new(unsafe { self.ptr.byte_offset(1576) })
    }

    pub(crate) fn displaylessMaxPixels(self, fld: u64) -> Self {
        self.store[1592..1600].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displaylessMaxPixels(&self) -> u64 {
        u64::from_le_bytes(self.store[1592..1600].try_into().unwrap())
    }
    pub(crate) fn set_displaylessMaxPixels(&mut self, fld: u64) {
        self.store[1592..1600].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn hInternalClient(self, fld: u32) -> Self {
        self.store[1600..1604].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hInternalClient(&self) -> u32 {
        u32::from_le_bytes(self.store[1600..1604].try_into().unwrap())
    }
    pub(crate) fn set_hInternalClient(&mut self, fld: u32) {
        self.store[1600..1604].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hInternalDevice(self, fld: u32) -> Self {
        self.store[1604..1608].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hInternalDevice(&self) -> u32 {
        u32::from_le_bytes(self.store[1604..1608].try_into().unwrap())
    }
    pub(crate) fn set_hInternalDevice(&mut self, fld: u32) {
        self.store[1604..1608].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hInternalSubdevice(self, fld: u32) -> Self {
        self.store[1608..1612].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hInternalSubdevice(&self) -> u32 {
        u32::from_le_bytes(self.store[1608..1612].try_into().unwrap())
    }
    pub(crate) fn set_hInternalSubdevice(&mut self, fld: u32) {
        self.store[1608..1612].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bSelfHostedMode(self, fld: u8) -> Self {
        self.store[1612..1613].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSelfHostedMode(&self) -> u8 {
        u8::from_le_bytes(self.store[1612..1613].try_into().unwrap())
    }
    pub(crate) fn set_bSelfHostedMode(&mut self, fld: u8) {
        self.store[1612..1613].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bAtsSupported(self, fld: u8) -> Self {
        self.store[1613..1614].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bAtsSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[1613..1614].try_into().unwrap())
    }
    pub(crate) fn set_bAtsSupported(&mut self, fld: u8) {
        self.store[1613..1614].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsGpuUefi(self, fld: u8) -> Self {
        self.store[1614..1615].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGpuUefi(&self) -> u8 {
        u8::from_le_bytes(self.store[1614..1615].try_into().unwrap())
    }
    pub(crate) fn set_bIsGpuUefi(&mut self, fld: u8) {
        self.store[1614..1615].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsEfiInit(self, fld: u8) -> Self {
        self.store[1615..1616].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsEfiInit(&self) -> u8 {
        u8::from_le_bytes(self.store[1615..1616].try_into().unwrap())
    }
    pub(crate) fn set_bIsEfiInit(&mut self, fld: u8) {
        self.store[1615..1616].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_ecidInfo(&mut self, idx: isize) -> s_EcidManufacturingInfo<'s> {
        s_EcidManufacturingInfo::new(unsafe { self.ptr.byte_offset(idx * 12 + 1616) })
    }

    pub(crate) fn new_S_fwWprLayoutOffset(&mut self) -> s_FW_WPR_LAYOUT_OFFSET<'s> {
        s_FW_WPR_LAYOUT_OFFSET::new(unsafe { self.ptr.byte_offset(1640) })
    }
}

pub(crate) struct s_NV2080_CTRL_GPU_GET_GID_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_GID_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        268
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 268) },
        }
    }

    pub(crate) fn index(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_index(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_index(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn length(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_length(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_length(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data(self, fld: [u8; 256]) -> Self {
        let mut byte_data = [0u8; 256];
        for i in 0..256 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[12..268].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_data(&mut self, fld: [u8; 256]) {
        let mut byte_data = [0u8; 256];
        for i in 0..256 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[12..268].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_data(&mut self) -> [u8; 256] {
        let mut array = [0u8; 256];
        for (i, chunk) in self.store[12..268].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_NV2080_CTRL_GPU_GET_FERMI_ZCULL_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_FERMI_ZCULL_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn gpcId(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpcId(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_gpcId(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn zcullMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_zcullMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_zcullMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV2080_CTRL_BIOS_GET_SKU_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_BIOS_GET_SKU_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        48
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 48) },
        }
    }

    pub(crate) fn BoardID(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_BoardID(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_BoardID(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn chipSKU(self, fld: [u8; 9]) -> Self {
        let mut byte_data = [0u8; 9];
        for i in 0..9 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[4..13].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_chipSKU(&mut self, fld: [u8; 9]) {
        let mut byte_data = [0u8; 9];
        for i in 0..9 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[4..13].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_chipSKU(&mut self) -> [u8; 9] {
        let mut array = [0u8; 9];
        for (i, chunk) in self.store[4..13].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn chipSKUMod(self, fld: [u8; 5]) -> Self {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[13..18].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_chipSKUMod(&mut self, fld: [u8; 5]) {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[13..18].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_chipSKUMod(&mut self) -> [u8; 5] {
        let mut array = [0u8; 5];
        for (i, chunk) in self.store[13..18].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn skuConfigVersion(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_skuConfigVersion(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_skuConfigVersion(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn project(self, fld: [u8; 5]) -> Self {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[24..29].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_project(&mut self, fld: [u8; 5]) {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[24..29].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_project(&mut self) -> [u8; 5] {
        let mut array = [0u8; 5];
        for (i, chunk) in self.store[24..29].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn projectSKU(self, fld: [u8; 5]) -> Self {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[29..34].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_projectSKU(&mut self, fld: [u8; 5]) {
        let mut byte_data = [0u8; 5];
        for i in 0..5 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[29..34].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_projectSKU(&mut self) -> [u8; 5] {
        let mut array = [0u8; 5];
        for (i, chunk) in self.store[29..34].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn CDP(self, fld: [u8; 6]) -> Self {
        let mut byte_data = [0u8; 6];
        for i in 0..6 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[34..40].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_CDP(&mut self, fld: [u8; 6]) {
        let mut byte_data = [0u8; 6];
        for i in 0..6 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[34..40].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_CDP(&mut self) -> [u8; 6] {
        let mut array = [0u8; 6];
        for (i, chunk) in self.store[34..40].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn projectSKUMod(self, fld: [u8; 2]) -> Self {
        let mut byte_data = [0u8; 2];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[40..42].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_projectSKUMod(&mut self, fld: [u8; 2]) {
        let mut byte_data = [0u8; 2];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[40..42].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_projectSKUMod(&mut self) -> [u8; 2] {
        let mut array = [0u8; 2];
        for (i, chunk) in self.store[40..42].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn businessCycle(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_businessCycle(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_businessCycle(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV2080_CTRL_CMD_FB_GET_FB_REGION_FB_REGION_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_CMD_FB_GET_FB_REGION_FB_REGION_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        48
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 48) },
        }
    }

    pub(crate) fn base(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_base(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_base(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn limit(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_limit(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_limit(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn reserved(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_reserved(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_reserved(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn performance(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_performance(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_performance(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn supportCompressed(self, fld: u8) -> Self {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_supportCompressed(&self) -> u8 {
        u8::from_le_bytes(self.store[28..29].try_into().unwrap())
    }
    pub(crate) fn set_supportCompressed(&mut self, fld: u8) {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn supportISO(self, fld: u8) -> Self {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_supportISO(&self) -> u8 {
        u8::from_le_bytes(self.store[29..30].try_into().unwrap())
    }
    pub(crate) fn set_supportISO(&mut self, fld: u8) {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bProtected(self, fld: u8) -> Self {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bProtected(&self) -> u8 {
        u8::from_le_bytes(self.store[30..31].try_into().unwrap())
    }
    pub(crate) fn set_bProtected(&mut self, fld: u8) {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn blackList(self, fld: [u8; 17]) -> Self {
        let mut byte_data = [0u8; 17];
        for i in 0..17 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[31..48].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_blackList(&mut self, fld: [u8; 17]) {
        let mut byte_data = [0u8; 17];
        for i in 0..17 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[31..48].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_blackList(&mut self) -> [u8; 17] {
        let mut array = [0u8; 17];
        for (i, chunk) in self.store[31..48].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) struct s_NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        776
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 776) },
        }
    }

    pub(crate) fn numFBRegions(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numFBRegions(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_numFBRegions(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_fbRegion(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_CMD_FB_GET_FB_REGION_FB_REGION_INFO<'s> {
        s_NV2080_CTRL_CMD_FB_GET_FB_REGION_FB_REGION_INFO::new(unsafe {
            self.ptr.byte_offset(idx * 48 + 8)
        })
    }
}

pub(crate) struct s_NV0080_CTRL_GPU_GET_SRIOV_CAPS_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0080_CTRL_GPU_GET_SRIOV_CAPS_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        80
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 80) },
        }
    }

    pub(crate) fn totalVFs(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_totalVFs(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_totalVFs(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn firstVfOffset(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_firstVfOffset(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_firstVfOffset(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vfFeatureMask(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vfFeatureMask(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_vfFeatureMask(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar0Address(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar0Address(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar0Address(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar1Address(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar1Address(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar1Address(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn FirstVFBar2Address(self, fld: u64) -> Self {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_FirstVFBar2Address(&self) -> u64 {
        u64::from_le_bytes(self.store[32..40].try_into().unwrap())
    }
    pub(crate) fn set_FirstVFBar2Address(&mut self, fld: u64) {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bar0Size(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar0Size(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_bar0Size(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bar1Size(self, fld: u64) -> Self {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar1Size(&self) -> u64 {
        u64::from_le_bytes(self.store[48..56].try_into().unwrap())
    }
    pub(crate) fn set_bar1Size(&mut self, fld: u64) {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bar2Size(self, fld: u64) -> Self {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar2Size(&self) -> u64 {
        u64::from_le_bytes(self.store[56..64].try_into().unwrap())
    }
    pub(crate) fn set_bar2Size(&mut self, fld: u64) {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar0(self, fld: u8) -> Self {
        self.store[64..65].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar0(&self) -> u8 {
        u8::from_le_bytes(self.store[64..65].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar0(&mut self, fld: u8) {
        self.store[64..65].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar1(self, fld: u8) -> Self {
        self.store[65..66].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar1(&self) -> u8 {
        u8::from_le_bytes(self.store[65..66].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar1(&mut self, fld: u8) {
        self.store[65..66].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn b64bitBar2(self, fld: u8) -> Self {
        self.store[66..67].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bitBar2(&self) -> u8 {
        u8::from_le_bytes(self.store[66..67].try_into().unwrap())
    }
    pub(crate) fn set_b64bitBar2(&mut self, fld: u8) {
        self.store[66..67].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSriovEnabled(self, fld: u8) -> Self {
        self.store[67..68].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSriovEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[67..68].try_into().unwrap())
    }
    pub(crate) fn set_bSriovEnabled(&mut self, fld: u8) {
        self.store[67..68].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSriovHeavyEnabled(self, fld: u8) -> Self {
        self.store[68..69].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSriovHeavyEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[68..69].try_into().unwrap())
    }
    pub(crate) fn set_bSriovHeavyEnabled(&mut self, fld: u8) {
        self.store[68..69].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bEmulateVFBar0TlbInvalidationRegister(self, fld: u8) -> Self {
        self.store[69..70].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bEmulateVFBar0TlbInvalidationRegister(&self) -> u8 {
        u8::from_le_bytes(self.store[69..70].try_into().unwrap())
    }
    pub(crate) fn set_bEmulateVFBar0TlbInvalidationRegister(&mut self, fld: u8) {
        self.store[69..70].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bClientRmAllocatedCtxBuffer(self, fld: u8) -> Self {
        self.store[70..71].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bClientRmAllocatedCtxBuffer(&self) -> u8 {
        u8::from_le_bytes(self.store[70..71].try_into().unwrap())
    }
    pub(crate) fn set_bClientRmAllocatedCtxBuffer(&mut self, fld: u8) {
        self.store[70..71].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bNonPowerOf2ChannelCountSupported(self, fld: u8) -> Self {
        self.store[71..72].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bNonPowerOf2ChannelCountSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[71..72].try_into().unwrap())
    }
    pub(crate) fn set_bNonPowerOf2ChannelCountSupported(&mut self, fld: u8) {
        self.store[71..72].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bVfResizableBAR1Supported(self, fld: u8) -> Self {
        self.store[72..73].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bVfResizableBAR1Supported(&self) -> u8 {
        u8::from_le_bytes(self.store[72..73].try_into().unwrap())
    }
    pub(crate) fn set_bVfResizableBAR1Supported(&mut self, fld: u8) {
        self.store[72..73].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV2080_CTRL_INTERNAL_ENGINE_CONTEXT_BUFFER_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_ENGINE_CONTEXT_BUFFER_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn alignment(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_alignment(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_alignment(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_VIRTUAL_DISPLAY_GET_MAX_RESOLUTION_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_VIRTUAL_DISPLAY_GET_MAX_RESOLUTION_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn headIndex(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_headIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_headIndex(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxHResolution(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxHResolution(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_maxHResolution(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxVResolution(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxVResolution(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_maxVResolution(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_VIRTUAL_DISPLAY_GET_NUM_HEADS_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_VIRTUAL_DISPLAY_GET_NUM_HEADS_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn numHeads(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numHeads(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_numHeads(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxNumHeads(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxNumHeads(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_maxNumHeads(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GspSystemInfo<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GspSystemInfo<'s> {
    pub(crate) const fn str_size() -> usize {
        928
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 928) },
        }
    }

    pub(crate) fn gpuPhysAddr(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_gpuPhysAddr(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gpuPhysFbAddr(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuPhysFbAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_gpuPhysFbAddr(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gpuPhysInstAddr(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuPhysInstAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_gpuPhysInstAddr(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gpuPhysIoAddr(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuPhysIoAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_gpuPhysIoAddr(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn nvDomainBusDeviceFunc(self, fld: u64) -> Self {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nvDomainBusDeviceFunc(&self) -> u64 {
        u64::from_le_bytes(self.store[32..40].try_into().unwrap())
    }
    pub(crate) fn set_nvDomainBusDeviceFunc(&mut self, fld: u64) {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn simAccessBufPhysAddr(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_simAccessBufPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_simAccessBufPhysAddr(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn notifyOpSharedSurfacePhysAddr(self, fld: u64) -> Self {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_notifyOpSharedSurfacePhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[48..56].try_into().unwrap())
    }
    pub(crate) fn set_notifyOpSharedSurfacePhysAddr(&mut self, fld: u64) {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pcieAtomicsOpMask(self, fld: u64) -> Self {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pcieAtomicsOpMask(&self) -> u64 {
        u64::from_le_bytes(self.store[56..64].try_into().unwrap())
    }
    pub(crate) fn set_pcieAtomicsOpMask(&mut self, fld: u64) {
        self.store[56..64].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn consoleMemSize(self, fld: u64) -> Self {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_consoleMemSize(&self) -> u64 {
        u64::from_le_bytes(self.store[64..72].try_into().unwrap())
    }
    pub(crate) fn set_consoleMemSize(&mut self, fld: u64) {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn maxUserVa(self, fld: u64) -> Self {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxUserVa(&self) -> u64 {
        u64::from_le_bytes(self.store[72..80].try_into().unwrap())
    }
    pub(crate) fn set_maxUserVa(&mut self, fld: u64) {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pciConfigMirrorBase(self, fld: u32) -> Self {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pciConfigMirrorBase(&self) -> u32 {
        u32::from_le_bytes(self.store[80..84].try_into().unwrap())
    }
    pub(crate) fn set_pciConfigMirrorBase(&mut self, fld: u32) {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn pciConfigMirrorSize(self, fld: u32) -> Self {
        self.store[84..88].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pciConfigMirrorSize(&self) -> u32 {
        u32::from_le_bytes(self.store[84..88].try_into().unwrap())
    }
    pub(crate) fn set_pciConfigMirrorSize(&mut self, fld: u32) {
        self.store[84..88].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn PCIDeviceID(self, fld: u32) -> Self {
        self.store[88..92].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_PCIDeviceID(&self) -> u32 {
        u32::from_le_bytes(self.store[88..92].try_into().unwrap())
    }
    pub(crate) fn set_PCIDeviceID(&mut self, fld: u32) {
        self.store[88..92].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn PCISubDeviceID(self, fld: u32) -> Self {
        self.store[92..96].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_PCISubDeviceID(&self) -> u32 {
        u32::from_le_bytes(self.store[92..96].try_into().unwrap())
    }
    pub(crate) fn set_PCISubDeviceID(&mut self, fld: u32) {
        self.store[92..96].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn PCIRevisionID(self, fld: u32) -> Self {
        self.store[96..100].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_PCIRevisionID(&self) -> u32 {
        u32::from_le_bytes(self.store[96..100].try_into().unwrap())
    }
    pub(crate) fn set_PCIRevisionID(&mut self, fld: u32) {
        self.store[96..100].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn pcieAtomicsCplDeviceCapMask(self, fld: u32) -> Self {
        self.store[100..104].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pcieAtomicsCplDeviceCapMask(&self) -> u32 {
        u32::from_le_bytes(self.store[100..104].try_into().unwrap())
    }
    pub(crate) fn set_pcieAtomicsCplDeviceCapMask(&mut self, fld: u32) {
        self.store[100..104].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn oorArch(self, fld: u8) -> Self {
        self.store[104..105].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_oorArch(&self) -> u8 {
        u8::from_le_bytes(self.store[104..105].try_into().unwrap())
    }
    pub(crate) fn set_oorArch(&mut self, fld: u8) {
        self.store[104..105].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn clPdbProperties(self, fld: u64) -> Self {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_clPdbProperties(&self) -> u64 {
        u64::from_le_bytes(self.store[112..120].try_into().unwrap())
    }
    pub(crate) fn set_clPdbProperties(&mut self, fld: u64) {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn Chipset(self, fld: u32) -> Self {
        self.store[120..124].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_Chipset(&self) -> u32 {
        u32::from_le_bytes(self.store[120..124].try_into().unwrap())
    }
    pub(crate) fn set_Chipset(&mut self, fld: u32) {
        self.store[120..124].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bGpuBehindBridge(self, fld: u8) -> Self {
        self.store[124..125].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bGpuBehindBridge(&self) -> u8 {
        u8::from_le_bytes(self.store[124..125].try_into().unwrap())
    }
    pub(crate) fn set_bGpuBehindBridge(&mut self, fld: u8) {
        self.store[124..125].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bFlrSupported(self, fld: u8) -> Self {
        self.store[125..126].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bFlrSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[125..126].try_into().unwrap())
    }
    pub(crate) fn set_bFlrSupported(&mut self, fld: u8) {
        self.store[125..126].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn b64bBar0Supported(self, fld: u8) -> Self {
        self.store[126..127].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_b64bBar0Supported(&self) -> u8 {
        u8::from_le_bytes(self.store[126..127].try_into().unwrap())
    }
    pub(crate) fn set_b64bBar0Supported(&mut self, fld: u8) {
        self.store[126..127].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bMnocAvailable(self, fld: u8) -> Self {
        self.store[127..128].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bMnocAvailable(&self) -> u8 {
        u8::from_le_bytes(self.store[127..128].try_into().unwrap())
    }
    pub(crate) fn set_bMnocAvailable(&mut self, fld: u8) {
        self.store[127..128].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn chipsetL1ssEnable(self, fld: u32) -> Self {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_chipsetL1ssEnable(&self) -> u32 {
        u32::from_le_bytes(self.store[128..132].try_into().unwrap())
    }
    pub(crate) fn set_chipsetL1ssEnable(&mut self, fld: u32) {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bUpstreamL0sUnsupported(self, fld: u8) -> Self {
        self.store[132..133].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUpstreamL0sUnsupported(&self) -> u8 {
        u8::from_le_bytes(self.store[132..133].try_into().unwrap())
    }
    pub(crate) fn set_bUpstreamL0sUnsupported(&mut self, fld: u8) {
        self.store[132..133].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bUpstreamL1Unsupported(self, fld: u8) -> Self {
        self.store[133..134].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUpstreamL1Unsupported(&self) -> u8 {
        u8::from_le_bytes(self.store[133..134].try_into().unwrap())
    }
    pub(crate) fn set_bUpstreamL1Unsupported(&mut self, fld: u8) {
        self.store[133..134].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bUpstreamL1PorSupported(self, fld: u8) -> Self {
        self.store[134..135].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUpstreamL1PorSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[134..135].try_into().unwrap())
    }
    pub(crate) fn set_bUpstreamL1PorSupported(&mut self, fld: u8) {
        self.store[134..135].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bUpstreamL1PorMobileOnly(self, fld: u8) -> Self {
        self.store[135..136].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUpstreamL1PorMobileOnly(&self) -> u8 {
        u8::from_le_bytes(self.store[135..136].try_into().unwrap())
    }
    pub(crate) fn set_bUpstreamL1PorMobileOnly(&mut self, fld: u8) {
        self.store[135..136].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSystemHasMux(self, fld: u8) -> Self {
        self.store[136..137].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSystemHasMux(&self) -> u8 {
        u8::from_le_bytes(self.store[136..137].try_into().unwrap())
    }
    pub(crate) fn set_bSystemHasMux(&mut self, fld: u8) {
        self.store[136..137].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn upstreamAddressValid(self, fld: u8) -> Self {
        self.store[137..138].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_upstreamAddressValid(&self) -> u8 {
        u8::from_le_bytes(self.store[137..138].try_into().unwrap())
    }
    pub(crate) fn set_upstreamAddressValid(&mut self, fld: u8) {
        self.store[137..138].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_FHBBusInfo(&mut self) -> s_BUSINFO<'s> {
        s_BUSINFO::new(unsafe { self.ptr.byte_offset(138) })
    }

    pub(crate) fn new_S_chipsetIDInfo(&mut self) -> s_BUSINFO<'s> {
        s_BUSINFO::new(unsafe { self.ptr.byte_offset(148) })
    }

    pub(crate) fn new_S_acpiMethodData(&mut self) -> s_ACPI_METHOD_DATA<'s> {
        s_ACPI_METHOD_DATA::new(unsafe { self.ptr.byte_offset(160) })
    }

    pub(crate) fn hypervisorType(self, fld: u32) -> Self {
        self.store[836..840].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hypervisorType(&self) -> u32 {
        u32::from_le_bytes(self.store[836..840].try_into().unwrap())
    }
    pub(crate) fn set_hypervisorType(&mut self, fld: u32) {
        self.store[836..840].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bIsPassthru(self, fld: u8) -> Self {
        self.store[840..841].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsPassthru(&self) -> u8 {
        u8::from_le_bytes(self.store[840..841].try_into().unwrap())
    }
    pub(crate) fn set_bIsPassthru(&mut self, fld: u8) {
        self.store[840..841].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn sysTimerOffsetNs(self, fld: u64) -> Self {
        self.store[848..856].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysTimerOffsetNs(&self) -> u64 {
        u64::from_le_bytes(self.store[848..856].try_into().unwrap())
    }
    pub(crate) fn set_sysTimerOffsetNs(&mut self, fld: u64) {
        self.store[848..856].copy_from_slice(&u64::to_le_bytes(fld));
    }

    pub(crate) fn new_S_gspVFInfo(&mut self) -> s_GSP_VF_INFO<'s> {
        s_GSP_VF_INFO::new(unsafe { self.ptr.byte_offset(856) })
    }

    pub(crate) fn bIsPrimary(self, fld: u8) -> Self {
        self.store[896..897].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsPrimary(&self) -> u8 {
        u8::from_le_bytes(self.store[896..897].try_into().unwrap())
    }
    pub(crate) fn set_bIsPrimary(&mut self, fld: u8) {
        self.store[896..897].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn isGridBuild(self, fld: u8) -> Self {
        self.store[897..898].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_isGridBuild(&self) -> u8 {
        u8::from_le_bytes(self.store[897..898].try_into().unwrap())
    }
    pub(crate) fn set_isGridBuild(&mut self, fld: u8) {
        self.store[897..898].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_pcieConfigReg(&mut self) -> s_GSP_PCIE_CONFIG_REG<'s> {
        s_GSP_PCIE_CONFIG_REG::new(unsafe { self.ptr.byte_offset(900) })
    }

    pub(crate) fn gridBuildCsp(self, fld: u32) -> Self {
        self.store[904..908].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gridBuildCsp(&self) -> u32 {
        u32::from_le_bytes(self.store[904..908].try_into().unwrap())
    }
    pub(crate) fn set_gridBuildCsp(&mut self, fld: u32) {
        self.store[904..908].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bPreserveVideoMemoryAllocations(self, fld: u8) -> Self {
        self.store[908..909].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bPreserveVideoMemoryAllocations(&self) -> u8 {
        u8::from_le_bytes(self.store[908..909].try_into().unwrap())
    }
    pub(crate) fn set_bPreserveVideoMemoryAllocations(&mut self, fld: u8) {
        self.store[908..909].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bTdrEventSupported(self, fld: u8) -> Self {
        self.store[909..910].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bTdrEventSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[909..910].try_into().unwrap())
    }
    pub(crate) fn set_bTdrEventSupported(&mut self, fld: u8) {
        self.store[909..910].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bFeatureStretchVblankCapable(self, fld: u8) -> Self {
        self.store[910..911].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bFeatureStretchVblankCapable(&self) -> u8 {
        u8::from_le_bytes(self.store[910..911].try_into().unwrap())
    }
    pub(crate) fn set_bFeatureStretchVblankCapable(&mut self, fld: u8) {
        self.store[910..911].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bEnableDynamicGranularityPageArrays(self, fld: u8) -> Self {
        self.store[911..912].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bEnableDynamicGranularityPageArrays(&self) -> u8 {
        u8::from_le_bytes(self.store[911..912].try_into().unwrap())
    }
    pub(crate) fn set_bEnableDynamicGranularityPageArrays(&mut self, fld: u8) {
        self.store[911..912].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bClockBoostSupported(self, fld: u8) -> Self {
        self.store[912..913].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bClockBoostSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[912..913].try_into().unwrap())
    }
    pub(crate) fn set_bClockBoostSupported(&mut self, fld: u8) {
        self.store[912..913].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bRouteDispIntrsToCPU(self, fld: u8) -> Self {
        self.store[913..914].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bRouteDispIntrsToCPU(&self) -> u8 {
        u8::from_le_bytes(self.store[913..914].try_into().unwrap())
    }
    pub(crate) fn set_bRouteDispIntrsToCPU(&mut self, fld: u8) {
        self.store[913..914].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn hostPageSize(self, fld: u64) -> Self {
        self.store[920..928].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hostPageSize(&self) -> u64 {
        u64::from_le_bytes(self.store[920..928].try_into().unwrap())
    }
    pub(crate) fn set_hostPageSize(&mut self, fld: u64) {
        self.store[920..928].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_MEMORY_DESC_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_MEMORY_DESC_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn base(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_base(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_base(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn size(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn addressSpace(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addressSpace(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_addressSpace(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cacheAttrib(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cacheAttrib(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_cacheAttrib(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV0000_ALLOC_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0000_ALLOC_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        120
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 120) },
        }
    }

    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn processID(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_processID(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_processID(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn processName(self, fld: [u8; 100]) -> Self {
        let mut byte_data = [0u8; 100];
        for i in 0..100 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[8..108].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_processName(&mut self, fld: [u8; 100]) {
        let mut byte_data = [0u8; 100];
        for i in 0..100 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[8..108].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_processName(&mut self) -> [u8; 100] {
        let mut array = [0u8; 100];
        for (i, chunk) in self.store[8..108].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn pOsPidInfo(self, fld: u64) -> Self {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pOsPidInfo(&self) -> u64 {
        u64::from_le_bytes(self.store[112..120].try_into().unwrap())
    }
    pub(crate) fn set_pOsPidInfo(&mut self, fld: u64) {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV0005_ALLOC_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0005_ALLOC_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn hParentClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hParentClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hParentClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hSrcResource(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hSrcResource(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hSrcResource(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hClass(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClass(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_hClass(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn notifyIndex(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_notifyIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_notifyIndex(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_data(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV0080_ALLOC_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0080_ALLOC_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        56
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 56) },
        }
    }

    pub(crate) fn deviceId(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_deviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_deviceId(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hClientShare(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClientShare(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hClientShare(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hTargetClient(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hTargetClient(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_hTargetClient(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hTargetDevice(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hTargetDevice(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_hTargetDevice(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vaSpaceSize(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaSpaceSize(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_vaSpaceSize(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vaStartInternal(self, fld: u64) -> Self {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaStartInternal(&self) -> u64 {
        u64::from_le_bytes(self.store[32..40].try_into().unwrap())
    }
    pub(crate) fn set_vaStartInternal(&mut self, fld: u64) {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vaLimitInternal(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaLimitInternal(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_vaLimitInternal(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vaMode(self, fld: u32) -> Self {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaMode(&self) -> u32 {
        u32::from_le_bytes(self.store[48..52].try_into().unwrap())
    }
    pub(crate) fn set_vaMode(&mut self, fld: u32) {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV2080_ALLOC_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_ALLOC_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn subDeviceId(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceId(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_CHANNEL_ALLOC_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_CHANNEL_ALLOC_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        368
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 368) },
        }
    }

    pub(crate) fn hObjectError(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectError(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hObjectError(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectBuffer(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectBuffer(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hObjectBuffer(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gpFifoOffset(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpFifoOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_gpFifoOffset(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gpFifoEntries(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpFifoEntries(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_gpFifoEntries(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hContextShare(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hContextShare(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_hContextShare(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hVASpace(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hVASpace(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_hVASpace(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hUserdMemory(self, fld: [u32; 8]) -> Self {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[32..64].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_hUserdMemory(&mut self, fld: [u32; 8]) {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[32..64].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_hUserdMemory(&mut self) -> [u32; 8] {
        let mut array = [0u32; 8];
        for (i, chunk) in self.store[32..64].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn userdOffset(self, fld: [u64; 8]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[64..128].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_userdOffset(&mut self, fld: [u64; 8]) {
        let mut byte_data = [0u8; 64];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[64..128].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_userdOffset(&mut self) -> [u64; 8] {
        let mut array = [0u64; 8];
        for (i, chunk) in self.store[64..128].chunks_exact(8).enumerate() {
            array[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn engineType(self, fld: u32) -> Self {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineType(&self) -> u32 {
        u32::from_le_bytes(self.store[128..132].try_into().unwrap())
    }
    pub(crate) fn set_engineType(&mut self, fld: u32) {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cid(self, fld: u32) -> Self {
        self.store[132..136].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cid(&self) -> u32 {
        u32::from_le_bytes(self.store[132..136].try_into().unwrap())
    }
    pub(crate) fn set_cid(&mut self, fld: u32) {
        self.store[132..136].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn subDeviceId(self, fld: u32) -> Self {
        self.store[136..140].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[136..140].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceId(&mut self, fld: u32) {
        self.store[136..140].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectEccError(self, fld: u32) -> Self {
        self.store[140..144].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectEccError(&self) -> u32 {
        u32::from_le_bytes(self.store[140..144].try_into().unwrap())
    }
    pub(crate) fn set_hObjectEccError(&mut self, fld: u32) {
        self.store[140..144].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_instanceMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(144) })
    }

    pub(crate) fn new_S_userdMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(168) })
    }

    pub(crate) fn new_S_ramfcMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(192) })
    }

    pub(crate) fn new_S_mthdbufMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(216) })
    }

    pub(crate) fn hPhysChannelGroup(self, fld: u32) -> Self {
        self.store[240..244].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hPhysChannelGroup(&self) -> u32 {
        u32::from_le_bytes(self.store[240..244].try_into().unwrap())
    }
    pub(crate) fn set_hPhysChannelGroup(&mut self, fld: u32) {
        self.store[240..244].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn internalFlags(self, fld: u32) -> Self {
        self.store[244..248].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_internalFlags(&self) -> u32 {
        u32::from_le_bytes(self.store[244..248].try_into().unwrap())
    }
    pub(crate) fn set_internalFlags(&mut self, fld: u32) {
        self.store[244..248].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_errorNotifierMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(248) })
    }

    pub(crate) fn new_S_eccErrorNotifierMem(&mut self) -> s_NV_MEMORY_DESC_PARAMS<'s> {
        s_NV_MEMORY_DESC_PARAMS::new(unsafe { self.ptr.byte_offset(272) })
    }

    pub(crate) fn ProcessID(self, fld: u32) -> Self {
        self.store[296..300].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ProcessID(&self) -> u32 {
        u32::from_le_bytes(self.store[296..300].try_into().unwrap())
    }
    pub(crate) fn set_ProcessID(&mut self, fld: u32) {
        self.store[296..300].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn SubProcessID(self, fld: u32) -> Self {
        self.store[300..304].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_SubProcessID(&self) -> u32 {
        u32::from_le_bytes(self.store[300..304].try_into().unwrap())
    }
    pub(crate) fn set_SubProcessID(&mut self, fld: u32) {
        self.store[300..304].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn encryptIv(self, fld: [u32; 3]) -> Self {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[304..316].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_encryptIv(&mut self, fld: [u32; 3]) {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[304..316].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_encryptIv(&mut self) -> [u32; 3] {
        let mut array = [0u32; 3];
        for (i, chunk) in self.store[304..316].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn decryptIv(self, fld: [u32; 3]) -> Self {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[316..328].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_decryptIv(&mut self, fld: [u32; 3]) {
        let mut byte_data = [0u8; 12];
        for i in 0..3 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[316..328].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_decryptIv(&mut self) -> [u32; 3] {
        let mut array = [0u32; 3];
        for (i, chunk) in self.store[316..328].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn hmacNonce(self, fld: [u32; 8]) -> Self {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[328..360].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_hmacNonce(&mut self, fld: [u32; 8]) {
        let mut byte_data = [0u8; 32];
        for i in 0..8 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[328..360].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_hmacNonce(&mut self) -> [u32; 8] {
        let mut array = [0u32; 8];
        for (i, chunk) in self.store[328..360].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn tpcConfigID(self, fld: u32) -> Self {
        self.store[360..364].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_tpcConfigID(&self) -> u32 {
        u32::from_le_bytes(self.store[360..364].try_into().unwrap())
    }
    pub(crate) fn set_tpcConfigID(&mut self, fld: u32) {
        self.store[360..364].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_VASPACE_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_VASPACE_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        48
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 48) },
        }
    }

    pub(crate) fn index(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_index(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_index(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vaSize(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaSize(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_vaSize(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vaStartInternal(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaStartInternal(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_vaStartInternal(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vaLimitInternal(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaLimitInternal(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_vaLimitInternal(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bigPageSize(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bigPageSize(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_bigPageSize(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vaBase(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vaBase(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_vaBase(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_NVC0B5_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NVC0B5_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn version(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_version(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_version(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineType(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineType(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_engineType(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV50VAIO_CHANNELPIO_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV50VAIO_CHANNELPIO_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn channelInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_channelInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_channelInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectNotify(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectNotify(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hObjectNotify(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn pControl(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pControl(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_pControl(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV50VAIO_CHANNELDMA_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV50VAIO_CHANNELDMA_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        40
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 40) },
        }
    }

    pub(crate) fn channelInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_channelInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_channelInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectBuffer(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectBuffer(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hObjectBuffer(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObjectNotify(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObjectNotify(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_hObjectNotify(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn offset(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_offset(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_offset(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn pControl(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pControl(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_pControl(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn channelPBSize(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_channelPBSize(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_channelPBSize(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn subDeviceId(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceId(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_BSP_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_BSP_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn prohibitMultipleInstances(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_prohibitMultipleInstances(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_prohibitMultipleInstances(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineInstance(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_engineInstance(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_MSENC_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_MSENC_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn prohibitMultipleInstances(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_prohibitMultipleInstances(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_prohibitMultipleInstances(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineInstance(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_engineInstance(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_NVJPG_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_NVJPG_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn prohibitMultipleInstances(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_prohibitMultipleInstances(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_prohibitMultipleInstances(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineInstance(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_engineInstance(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV_OFA_ALLOCATION_PARAMETERS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV_OFA_ALLOCATION_PARAMETERS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn prohibitMultipleInstances(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_prohibitMultipleInstances(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_prohibitMultipleInstances(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineInstance(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_engineInstance(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_PCIE_CONFIG_REG<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_PCIE_CONFIG_REG<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn linkCap(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_linkCap(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_linkCap(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_FW_WPR_LAYOUT_OFFSET<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_FW_WPR_LAYOUT_OFFSET<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn nonWprHeapOffset(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nonWprHeapOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_nonWprHeapOffset(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn frtsOffset(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_frtsOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_frtsOffset(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_FMC_BOOT_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_FMC_BOOT_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        80
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 80) },
        }
    }

    pub(crate) fn new_S_initParams(&mut self) -> s_GSP_FMC_INIT_PARAMS<'s> {
        s_GSP_FMC_INIT_PARAMS::new(unsafe { self.ptr.byte_offset(0) })
    }

    pub(crate) fn new_S_bootGspRmParams(&mut self) -> s_GSP_ACR_BOOT_GSP_RM_PARAMS<'s> {
        s_GSP_ACR_BOOT_GSP_RM_PARAMS::new(unsafe { self.ptr.byte_offset(8) })
    }

    pub(crate) fn new_S_gspRmParams(&mut self) -> s_GSP_RM_PARAMS<'s> {
        s_GSP_RM_PARAMS::new(unsafe { self.ptr.byte_offset(40) })
    }

    pub(crate) fn new_S_gspSpdmParams(&mut self) -> s_GSP_SPDM_PARAMS<'s> {
        s_GSP_SPDM_PARAMS::new(unsafe { self.ptr.byte_offset(56) })
    }
}

pub(crate) struct s_GSP_FMC_INIT_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_FMC_INIT_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn regkeys(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_regkeys(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_regkeys(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_ACR_BOOT_GSP_RM_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_ACR_BOOT_GSP_RM_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn target(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_target(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_target(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gspRmDescSize(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspRmDescSize(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_gspRmDescSize(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gspRmDescOffset(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspRmDescOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_gspRmDescOffset(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn wprCarveoutOffset(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_wprCarveoutOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_wprCarveoutOffset(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn wprCarveoutSize(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_wprCarveoutSize(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_wprCarveoutSize(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bIsGspRmBoot(self, fld: u8) -> Self {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsGspRmBoot(&self) -> u8 {
        u8::from_le_bytes(self.store[28..29].try_into().unwrap())
    }
    pub(crate) fn set_bIsGspRmBoot(&mut self, fld: u8) {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_RM_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_RM_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn target(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_target(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_target(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bootArgsOffset(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bootArgsOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_bootArgsOffset(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) struct s_GSP_SPDM_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_GSP_SPDM_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn target(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_target(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_target(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn payloadBufferOffset(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_payloadBufferOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_payloadBufferOffset(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn payloadBufferSize(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_payloadBufferSize(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_payloadBufferSize(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DP_SET_MANUAL_DISPLAYPORT: u32 = 0x731365;
pub(crate) const NV0073_CTRL_CMD_DP_SET_MANUAL_DISPLAYPORT_PARAMS_MESSAGE_ID: u32 = 0x65;
pub(crate) struct s_NV0073_CTRL_CMD_DP_SET_MANUAL_DISPLAYPORT_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_CMD_DP_SET_MANUAL_DISPLAYPORT_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS: u32 = 0x731369;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_2: u32 = 0;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_2_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_2_YES: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_4: u32 = 1;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_4_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP1_4_YES: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP2_0: u32 = 2;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP2_0_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DP_VERSIONS_SUPPORTED_DP2_0_YES: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_BITS_PER_PIXEL_PRECISION_1: u32 = 0x00000005;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_BITS_PER_PIXEL_PRECISION_1_16: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_BITS_PER_PIXEL_PRECISION_1_2: u32 = 0x00000004;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_BITS_PER_PIXEL_PRECISION_1_4: u32 = 0x00000003;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_BITS_PER_PIXEL_PRECISION_1_8: u32 = 0x00000002;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_ENCODER_COLOR_FORMAT_RGB: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_ENCODER_COLOR_FORMAT_Y_CB_CR_444: u32 = 0x00000002;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_ENCODER_COLOR_FORMAT_Y_CB_CR_NATIVE_420: u32 =
    0x00000008;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_DSC_ENCODER_COLOR_FORMAT_Y_CB_CR_NATIVE_422: u32 =
    0x00000004;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_A: u32 = 2;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_B: u32 = 0;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_1_62: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_2_70: u32 = 0x00000002;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_5_40: u32 = 0x00000003;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_8_10: u32 = 0x00000004;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_MAX_LINK_RATE_NONE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_PARAMS_MESSAGE_ID: u32 = 0x69;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR10_0: u32 = 0;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR10_0_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR10_0_YES: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR13_5: u32 = 1;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR13_5_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR13_5_YES: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR20_0: u32 = 2;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR20_0_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_DP_GET_CAPS_UHBR_SUPPORTED_UHBR20_0_YES: u32 = 0x00000001;
pub(crate) struct s_NV0073_CTRL_CMD_DP_GET_CAPS_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_CMD_DP_GET_CAPS_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        64
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 64) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn sorIndex(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sorIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_sorIndex(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxLinkRate(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxLinkRate(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_maxLinkRate(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn dpVersionsSupported(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_dpVersionsSupported(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_dpVersionsSupported(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn UHBRSupportedByGpu(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_UHBRSupportedByGpu(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_UHBRSupportedByGpu(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn minPClkForCompressed(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_minPClkForCompressed(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_minPClkForCompressed(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bIsMultistreamSupported(self, fld: u8) -> Self {
        self.store[24..25].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsMultistreamSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[24..25].try_into().unwrap())
    }
    pub(crate) fn set_bIsMultistreamSupported(&mut self, fld: u8) {
        self.store[24..25].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsSCEnabled(self, fld: u8) -> Self {
        self.store[25..26].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsSCEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[25..26].try_into().unwrap())
    }
    pub(crate) fn set_bIsSCEnabled(&mut self, fld: u8) {
        self.store[25..26].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bHasIncreasedWatermarkLimits(self, fld: u8) -> Self {
        self.store[26..27].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bHasIncreasedWatermarkLimits(&self) -> u8 {
        u8::from_le_bytes(self.store[26..27].try_into().unwrap())
    }
    pub(crate) fn set_bHasIncreasedWatermarkLimits(&mut self, fld: u8) {
        self.store[26..27].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsPC2Disabled(self, fld: u8) -> Self {
        self.store[27..28].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsPC2Disabled(&self) -> u8 {
        u8::from_le_bytes(self.store[27..28].try_into().unwrap())
    }
    pub(crate) fn set_bIsPC2Disabled(&mut self, fld: u8) {
        self.store[27..28].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn isSingleHeadMSTSupported(self, fld: u8) -> Self {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_isSingleHeadMSTSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[28..29].try_into().unwrap())
    }
    pub(crate) fn set_isSingleHeadMSTSupported(&mut self, fld: u8) {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bFECSupported(self, fld: u8) -> Self {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bFECSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[29..30].try_into().unwrap())
    }
    pub(crate) fn set_bFECSupported(&mut self, fld: u8) {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsTrainPhyRepeater(self, fld: u8) -> Self {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsTrainPhyRepeater(&self) -> u8 {
        u8::from_le_bytes(self.store[30..31].try_into().unwrap())
    }
    pub(crate) fn set_bIsTrainPhyRepeater(&mut self, fld: u8) {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bOverrideLinkBw(self, fld: u8) -> Self {
        self.store[31..32].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bOverrideLinkBw(&self) -> u8 {
        u8::from_le_bytes(self.store[31..32].try_into().unwrap())
    }
    pub(crate) fn set_bOverrideLinkBw(&mut self, fld: u8) {
        self.store[31..32].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bUseRgFlushSequence(self, fld: u8) -> Self {
        self.store[32..33].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUseRgFlushSequence(&self) -> u8 {
        u8::from_le_bytes(self.store[32..33].try_into().unwrap())
    }
    pub(crate) fn set_bUseRgFlushSequence(&mut self, fld: u8) {
        self.store[32..33].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSupportDPDownSpread(self, fld: u8) -> Self {
        self.store[33..34].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSupportDPDownSpread(&self) -> u8 {
        u8::from_le_bytes(self.store[33..34].try_into().unwrap())
    }
    pub(crate) fn set_bSupportDPDownSpread(&mut self, fld: u8) {
        self.store[33..34].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_DSC(&mut self) -> s_NV0073_CTRL_CMD_DSC_CAP_PARAMS<'s> {
        s_NV0073_CTRL_CMD_DSC_CAP_PARAMS::new(unsafe { self.ptr.byte_offset(36) })
    }
}

pub(crate) struct s_NV0073_CTRL_CMD_DSC_CAP_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_CMD_DSC_CAP_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        28
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 28) },
        }
    }

    pub(crate) fn bDscSupported(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bDscSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_bDscSupported(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn encoderColorFormatMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_encoderColorFormatMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_encoderColorFormatMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn lineBufferSizeKB(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_lineBufferSizeKB(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_lineBufferSizeKB(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rateBufferSizeKB(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rateBufferSizeKB(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_rateBufferSizeKB(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bitsPerPixelPrecision(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bitsPerPixelPrecision(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_bitsPerPixelPrecision(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxNumHztSlices(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxNumHztSlices(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_maxNumHztSlices(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn lineBufferBitDepth(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_lineBufferBitDepth(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_lineBufferBitDepth(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DFP_ASSIGN_SOR: u32 = 0x731152;
pub(crate) const NV0073_CTRL_CMD_DFP_ASSIGN_SOR_MAX_SORS: u32 = 4;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_ACTIVE_SOR_NOT_AUDIO_CAPABLE: u32 = 1;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_ACTIVE_SOR_NOT_AUDIO_CAPABLE_NO: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_ACTIVE_SOR_NOT_AUDIO_CAPABLE_YES: u32 =
    0x00000001;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_AUDIO: u32 = 0;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_AUDIO_DEFAULT: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FLAGS_AUDIO_OPTIMAL: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FORCE_NONE: u32 = 0x0;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FORCE_PRIMARY_SOR_LINK: u32 = 0x1;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_FORCE_SECONDARY_SOR_LINK: u32 = 0x2;
pub(crate) const NV0073_CTRL_DFP_ASSIGN_SOR_PARAMS_MESSAGE_ID: u32 = 0x52;
pub(crate) struct s_NV0073_CTRL_DFP_ASSIGN_SOR_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DFP_ASSIGN_SOR_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        80
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 80) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn sorExcludeMask(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sorExcludeMask(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_sorExcludeMask(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn slaveDisplayId(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_slaveDisplayId(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_slaveDisplayId(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn forceSublinkConfig(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_forceSublinkConfig(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_forceSublinkConfig(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bIs2Head1Or(self, fld: u8) -> Self {
        self.store[20..21].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIs2Head1Or(&self) -> u8 {
        u8::from_le_bytes(self.store[20..21].try_into().unwrap())
    }
    pub(crate) fn set_bIs2Head1Or(&mut self, fld: u8) {
        self.store[20..21].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn sorAssignList(self, fld: [u32; 4]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[24..40].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_sorAssignList(&mut self, fld: [u32; 4]) {
        let mut byte_data = [0u8; 16];
        for i in 0..4 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[24..40].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_sorAssignList(&mut self) -> [u32; 4] {
        let mut array = [0u32; 4];
        for (i, chunk) in self.store[24..40].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }

    pub(crate) fn new_S_sorAssignListWithTag(
        &mut self,
        idx: isize,
    ) -> s_NV0073_CTRL_DFP_ASSIGN_SOR_INFO<'s> {
        s_NV0073_CTRL_DFP_ASSIGN_SOR_INFO::new(unsafe { self.ptr.byte_offset(idx * 8 + 40) })
    }

    pub(crate) fn reservedSorMask(self, fld: u8) -> Self {
        self.store[72..73].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_reservedSorMask(&self) -> u8 {
        u8::from_le_bytes(self.store[72..73].try_into().unwrap())
    }
    pub(crate) fn set_reservedSorMask(&mut self, fld: u8) {
        self.store[72..73].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[76..80].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[76..80].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[76..80].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV0073_CTRL_DFP_ASSIGN_SOR_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DFP_ASSIGN_SOR_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn displayMask(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayMask(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_displayMask(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn sorType(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sorType(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_sorType(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DFP_GET_INFO: u32 = 0x731140;
pub(crate) const NV0073_CTRL_DFP_GET_INFO_PARAMS_MESSAGE_ID: u32 = 0x40;
pub(crate) struct s_NV0073_CTRL_DFP_GET_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DFP_GET_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn UHBRSupportedByDfp(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_UHBRSupportedByDfp(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_UHBRSupportedByDfp(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DFP_SET_AUDIO_ENABLE: u32 = 0x731150;
pub(crate) const NV0073_CTRL_DFP_SET_AUDIO_ENABLE_PARAMS_MESSAGE_ID: u32 = 0x50;
pub(crate) struct s_NV0073_CTRL_DFP_SET_AUDIO_ENABLE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DFP_SET_AUDIO_ENABLE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn enable(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_enable(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_enable(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DFP_SET_ELD_AUDIO_CAPS: u32 = 0x731144;
pub(crate) const NV0073_CTRL_CMD_DP_AUXCH_CTRL: u32 = 0x731341;
pub(crate) const NV0073_CTRL_DP_AUXCH_CTRL_PARAMS_MESSAGE_ID: u32 = 0x41;
pub(crate) struct s_NV0073_CTRL_DP_AUXCH_CTRL_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DP_AUXCH_CTRL_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        48
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 48) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bAddrOnly(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bAddrOnly(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bAddrOnly(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn cmd(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cmd(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_cmd(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn addr(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addr(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_addr(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data(self, fld: [u8; 16]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[20..36].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_data(&mut self, fld: [u8; 16]) {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[20..36].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_data(&mut self) -> [u8; 16] {
        let mut array = [0u8; 16];
        for (i, chunk) in self.store[20..36].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[36..40].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn replyType(self, fld: u32) -> Self {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_replyType(&self) -> u32 {
        u32::from_le_bytes(self.store[40..44].try_into().unwrap())
    }
    pub(crate) fn set_replyType(&mut self, fld: u32) {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn retryTimeMs(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_retryTimeMs(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_retryTimeMs(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DP_CONFIG_INDEXED_LINK_RATES: u32 = 0x731377;
pub(crate) const NV0073_CTRL_CMD_DP_CONFIG_INDEXED_LINK_RATES_PARAMS_MESSAGE_ID: u32 = 0x77;
pub(crate) const NV0073_CTRL_CMD_DP_CONFIG_STREAM: u32 = 0x731362;
pub(crate) const NV0073_CTRL_CMD_DP_CONFIG_STREAM_PARAMS_MESSAGE_ID: u32 = 0x62;
pub(crate) const NV0073_CTRL_CMD_DP_SET_LANE_DATA: u32 = 0x731346;
pub(crate) const NV0073_CTRL_DP_SET_LANE_DATA_PARAMS_MESSAGE_ID: u32 = 0x46;
pub(crate) const NV0073_CTRL_CMD_DP_SET_AUDIO_MUTESTREAM: u32 = 0x731359;
pub(crate) const NV0073_CTRL_DP_SET_AUDIO_MUTESTREAM_PARAMS_MESSAGE_ID: u32 = 0x59;
pub(crate) struct s_NV0073_CTRL_DP_SET_AUDIO_MUTESTREAM_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_DP_SET_AUDIO_MUTESTREAM_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn mute(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mute(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_mute(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_DP_TOPOLOGY_FREE_DISPLAYID: u32 = 0x73135c;
pub(crate) const NV0073_CTRL_CMD_DP_TOPOLOGY_FREE_DISPLAYID_PARAMS_MESSAGE_ID: u32 = 0x5C;
pub(crate) const NV0073_CTRL_CMD_DP_TOPOLOGY_ALLOCATE_DISPLAYID: u32 = 0x73135b;
pub(crate) const NV0073_CTRL_CMD_DP_TOPOLOGY_ALLOCATE_DISPLAYID_PARAMS_MESSAGE_ID: u32 = 0x5B;
pub(crate) const NV0073_CTRL_CMD_SYSTEM_GET_ACTIVE: u32 = 0x73010c;
pub(crate) const NV0073_CTRL_SYSTEM_GET_ACTIVE_FLAGS_CLIENT: u32 = 0;
pub(crate) const NV0073_CTRL_SYSTEM_GET_ACTIVE_FLAGS_CLIENT_DISABLE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SYSTEM_GET_ACTIVE_FLAGS_CLIENT_ENABLE: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SYSTEM_GET_ACTIVE_PARAMS_MESSAGE_ID: u32 = 0x0C;
pub(crate) struct s_NV0073_CTRL_SYSTEM_GET_ACTIVE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SYSTEM_GET_ACTIVE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn head(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_head(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_head(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SYSTEM_GET_NUM_HEADS: u32 = 0x730102;
pub(crate) const NV0073_CTRL_SYSTEM_GET_NUM_HEADS_FLAGS_CLIENT: u32 = 0;
pub(crate) const NV0073_CTRL_SYSTEM_GET_NUM_HEADS_FLAGS_CLIENT_DISABLE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SYSTEM_GET_NUM_HEADS_FLAGS_CLIENT_ENABLE: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SYSTEM_GET_NUM_HEADS_PARAMS_MESSAGE_ID: u32 = 0x02;
pub(crate) struct s_NV0073_CTRL_SYSTEM_GET_NUM_HEADS_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SYSTEM_GET_NUM_HEADS_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numHeads(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numHeads(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_numHeads(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SYSTEM_GET_SUPPORTED: u32 = 0x730107;
pub(crate) const NV0073_CTRL_SYSTEM_GET_SUPPORTED_PARAMS_MESSAGE_ID: u32 = 0x07;
pub(crate) struct s_NV0073_CTRL_SYSTEM_GET_SUPPORTED_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SYSTEM_GET_SUPPORTED_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayMaskDDC(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayMaskDDC(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_displayMaskDDC(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_GET_ALL_HEAD_MASK: u32 = 0x730287;
pub(crate) const NV0073_CTRL_SPECIFIC_GET_ALL_HEAD_MASK_PARAMS_MESSAGE_ID: u32 = 0x87;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_GET_ALL_HEAD_MASK_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_GET_ALL_HEAD_MASK_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn headMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_headMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_headMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_BACKLIGHT_BRIGHTNESS: u32 = 0x730292;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_BACKLIGHT_BRIGHTNESS_PARAMS_MESSAGE_ID: u32 = 0x92;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_GET_BACKLIGHT_BRIGHTNESS: u32 = 0x730291;
pub(crate) const NV0073_CTRL_SPECIFIC_GET_BACKLIGHT_BRIGHTNESS_PARAMS_MESSAGE_ID: u32 = 0x91;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS: u32 = 0x730293;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_12_SUPPORTED: u32 = 6;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_12_SUPPORTED_FALSE: u32 =
    0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_12_SUPPORTED_TRUE: u32 =
    0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_A: u32 = 9;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_B: u32 = 7;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_3LANES_3G: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_3LANES_6G: u32 = 0x00000002;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_4LANES_10G: u32 = 0x00000005;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_4LANES_12G: u32 = 0x00000006;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_4LANES_6G: u32 = 0x00000003;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_4LANES_8G: u32 = 0x00000004;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_DSC_MAX_FRL_RATE_SUPPORTED_NONE: u32 =
    0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_GT_340MHZ_CLOCK_SUPPORTED: u32 = 0;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_GT_340MHZ_CLOCK_SUPPORTED_FALSE: u32 =
    0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_GT_340MHZ_CLOCK_SUPPORTED_TRUE: u32 =
    0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_LTE_340MHZ_SCRAMBLING_SUPPORTED: u32 =
    1;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_LTE_340MHZ_SCRAMBLING_SUPPORTED_FALSE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_LTE_340MHZ_SCRAMBLING_SUPPORTED_TRUE: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_A: u32 = 5;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_B: u32 = 3;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_3LANES_3G: u32 =
    0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_3LANES_6G: u32 =
    0x00000002;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_4LANES_10G:
    u32 = 0x00000005;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_4LANES_12G:
    u32 = 0x00000006;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_4LANES_6G: u32 =
    0x00000003;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_4LANES_8G: u32 =
    0x00000004;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_MAX_FRL_RATE_SUPPORTED_NONE: u32 =
    0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_SCDC_SUPPORTED: u32 = 2;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_SCDC_SUPPORTED_FALSE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_SINK_CAPS_SCDC_SUPPORTED_TRUE: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_SINK_CAPS_PARAMS_MESSAGE_ID: u32 = 0x93;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_SET_HDMI_SINK_CAPS_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_SET_HDMI_SINK_CAPS_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn caps(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_caps(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_caps(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_OD_PACKET: u32 = 0x730288;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_OD_PACKET_CTRL: u32 = 0x730289;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_PARAMS_MESSAGE_ID: u32 = 0x89;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ENABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ENABLE_NO: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE_NO;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ENABLE_YES: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE_YES;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ON_HBLANK: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ON_HBLANK_DISABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK_DISABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_ON_HBLANK_ENABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK_ENABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_OTHER_FRAME: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_OTHER_FRAME_DISABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME_DISABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_OTHER_FRAME_ENABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME_ENABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE:
    u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_NO: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_NO;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_YES: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_YES;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_FALSE: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_TRUE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_TRUE: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_FALSE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_STEREO_POLARITY: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_STEREO_POLARITY_FALSE: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY_FALSE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SET_STEREO_POLARITY_TRUE: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY_TRUE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SINGLE_FRAME: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SINGLE_FRAME_DISABLE:
    u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME_DISABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_SINGLE_FRAME_ENABLE: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME_ENABLE;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_VIDEO_FMT: u32 =
    NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_VIDEO_FMT_HW_CONTROLLED: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT_HW_CONTROLLED;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_CTRL_TRANSMIT_CONTROL_VIDEO_FMT_SW_CONTROLLED: u32 = NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT_SW_CONTROLLED;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_PARAMS_MESSAGE_ID: u32 = 0x88;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE: u32 = 0;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE_NO: u32 = 0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ENABLE_YES: u32 = 0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_GEN_INFOFRAME_MODE_A: u32 = 9;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_GEN_INFOFRAME_MODE_B: u32 = 8;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_GEN_INFOFRAME_MODE_INFOFRAME0: u32 = 0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_GEN_INFOFRAME_MODE_INFOFRAME1: u32 = 0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_IMMEDIATE: u32 = 4;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_IMMEDIATE_DISABLE: u32 =
    0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_IMMEDIATE_ENABLE: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK: u32 = 3;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK_DISABLE: u32 =
    0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_ON_HBLANK_ENABLE: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME: u32 = 1;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME_DISABLE: u32 =
    0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_OTHER_FRAME_ENABLE: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE: u32 = 31;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_NO: u32 =
    0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_RESERVED_LEGACY_MODE_YES: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING: u32 =
    7;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_FALSE: u32 = 0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_SELF_REFRESH_SETTING_TRUE: u32 = 0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY: u32 = 6;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY_FALSE:
    u32 = 0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SET_STEREO_POLARITY_TRUE: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME: u32 = 2;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME_DISABLE: u32 =
    0x0000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_SINGLE_FRAME_ENABLE: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT: u32 = 5;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT_HW_CONTROLLED: u32 =
    0x0000001;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_OD_PACKET_TRANSMIT_CONTROL_VIDEO_FMT_SW_CONTROLLED: u32 =
    0x0000000;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_SET_OD_PACKET_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_SET_OD_PACKET_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        60
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 60) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn transmitControl(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_transmitControl(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_transmitControl(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn packetSize(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_packetSize(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_packetSize(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn targetHead(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_targetHead(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_targetHead(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bUsePsrHeadforSdp(self, fld: u8) -> Self {
        self.store[20..21].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bUsePsrHeadforSdp(&self) -> u8 {
        u8::from_le_bytes(self.store[20..21].try_into().unwrap())
    }
    pub(crate) fn set_bUsePsrHeadforSdp(&mut self, fld: u8) {
        self.store[20..21].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn aPacket(self, fld: [u8; 36]) -> Self {
        let mut byte_data = [0u8; 36];
        for i in 0..36 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[21..57].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_aPacket(&mut self, fld: [u8; 36]) {
        let mut byte_data = [0u8; 36];
        for i in 0..36 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[21..57].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_aPacket(&mut self) -> [u8; 36] {
        let mut array = [0u8; 36];
        for (i, chunk) in self.store[21..57].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_ENABLE: u32 = 0x730273;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_ENABLE_FALSE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_ENABLE_PARAMS_MESSAGE_ID: u32 = 0x73;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_ENABLE_TRUE: u32 = 0x00000001;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_SET_HDMI_ENABLE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_SET_HDMI_ENABLE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        12
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 12) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn enable(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_enable(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_enable(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_AUDIO_MUTESTREAM: u32 = 0x730275;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_SET_HDMI_AUDIO_MUTESTREAM_PARAMS_MESSAGE_ID: u32 = 0x75;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_AUDIO_MUTESTREAM_FALSE: u32 = 0x00000000;
pub(crate) const NV0073_CTRL_SPECIFIC_SET_HDMI_AUDIO_MUTESTREAM_TRUE: u32 = 0x00000001;
pub(crate) const NV0073_CTRL_CMD_SPECIFIC_OR_GET_INFO: u32 = 0x73028b;
pub(crate) const NV0073_CTRL_SPECIFIC_OR_GET_INFO_PARAMS_MESSAGE_ID: u32 = 0x8B;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_OR_GET_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_OR_GET_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        56
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 56) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn index(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_index(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_index(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rtype(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rtype(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_rtype(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn protocol(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_protocol(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_protocol(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ditherType(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ditherType(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_ditherType(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ditherAlgo(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ditherAlgo(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_ditherAlgo(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn location(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_location(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_location(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn rootPortId(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_rootPortId(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_rootPortId(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn dcbIndex(self, fld: u32) -> Self {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_dcbIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[36..40].try_into().unwrap())
    }
    pub(crate) fn set_dcbIndex(&mut self, fld: u32) {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vbiosAddress(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vbiosAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_vbiosAddress(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bIsLitByVbios(self, fld: u8) -> Self {
        self.store[48..49].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsLitByVbios(&self) -> u8 {
        u8::from_le_bytes(self.store[48..49].try_into().unwrap())
    }
    pub(crate) fn set_bIsLitByVbios(&mut self, fld: u8) {
        self.store[48..49].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bIsDispDynamic(self, fld: u8) -> Self {
        self.store[49..50].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bIsDispDynamic(&self) -> u8 {
        u8::from_le_bytes(self.store[49..50].try_into().unwrap())
    }
    pub(crate) fn set_bIsDispDynamic(&mut self, fld: u8) {
        self.store[49..50].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV0073_CTRL_CMD_SPECIFIC_GET_CONNECTOR_DATA: u32 = 0x730250;
pub(crate) const NV0073_CTRL_SPECIFIC_GET_CONNECTOR_DATA_PARAMS_MESSAGE_ID: u32 = 0x50;
pub(crate) struct s_NV0073_CTRL_SPECIFIC_GET_CONNECTOR_DATA_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV0073_CTRL_SPECIFIC_GET_CONNECTOR_DATA_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        72
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 72) },
        }
    }

    pub(crate) fn subDeviceInstance(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceInstance(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn displayId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_displayId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_displayId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn flags(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_flags(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_flags(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn DDCPartners(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_DDCPartners(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_DDCPartners(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn count(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_count(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_count(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_0_index(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_0_index(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_data_0_index(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_0_type(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_0_type(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_data_0_type(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_0_location(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_0_location(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_data_0_location(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_1_index(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_1_index(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_data_1_index(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_1_type(self, fld: u32) -> Self {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_1_type(&self) -> u32 {
        u32::from_le_bytes(self.store[36..40].try_into().unwrap())
    }
    pub(crate) fn set_data_1_type(&mut self, fld: u32) {
        self.store[36..40].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_1_location(self, fld: u32) -> Self {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_1_location(&self) -> u32 {
        u32::from_le_bytes(self.store[40..44].try_into().unwrap())
    }
    pub(crate) fn set_data_1_location(&mut self, fld: u32) {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_2_index(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_2_index(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_data_2_index(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_2_type(self, fld: u32) -> Self {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_2_type(&self) -> u32 {
        u32::from_le_bytes(self.store[48..52].try_into().unwrap())
    }
    pub(crate) fn set_data_2_type(&mut self, fld: u32) {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_2_location(self, fld: u32) -> Self {
        self.store[52..56].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_2_location(&self) -> u32 {
        u32::from_le_bytes(self.store[52..56].try_into().unwrap())
    }
    pub(crate) fn set_data_2_location(&mut self, fld: u32) {
        self.store[52..56].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_3_index(self, fld: u32) -> Self {
        self.store[56..60].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_3_index(&self) -> u32 {
        u32::from_le_bytes(self.store[56..60].try_into().unwrap())
    }
    pub(crate) fn set_data_3_index(&mut self, fld: u32) {
        self.store[56..60].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_3_type(self, fld: u32) -> Self {
        self.store[60..64].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_3_type(&self) -> u32 {
        u32::from_le_bytes(self.store[60..64].try_into().unwrap())
    }
    pub(crate) fn set_data_3_type(&mut self, fld: u32) {
        self.store[60..64].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn data_3_location(self, fld: u32) -> Self {
        self.store[64..68].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_data_3_location(&self) -> u32 {
        u32::from_le_bytes(self.store[64..68].try_into().unwrap())
    }
    pub(crate) fn set_data_3_location(&mut self, fld: u32) {
        self.store[64..68].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn platform(self, fld: u32) -> Self {
        self.store[68..72].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_platform(&self) -> u32 {
        u32::from_le_bytes(self.store[68..72].try_into().unwrap())
    }
    pub(crate) fn set_platform(&mut self, fld: u32) {
        self.store[68..72].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV0080_CTRL_GPU_GET_SRIOV_CAPS_PARAMS_MESSAGE_ID: u32 = 0x91;
pub(crate) const NV2080_CTRL_CE_GET_FAULT_METHOD_BUFFER_SIZE_PARAMS_MESSAGE_ID: u32 = 0x8;
pub(crate) const NV2080_CTRL_CMD_CE_GET_FAULT_METHOD_BUFFER_SIZE: u32 = 0x20802a08;
pub(crate) struct s_NV2080_CTRL_CE_GET_FAULT_METHOD_BUFFER_SIZE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_CE_GET_FAULT_METHOD_BUFFER_SIZE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn size(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_EVENT_SET_NOTIFICATION: u32 = 0x20800301;
pub(crate) const NV2080_CTRL_EVENT_SET_NOTIFICATION_ACTION_DISABLE: u32 = 0x00000000;
pub(crate) const NV2080_CTRL_EVENT_SET_NOTIFICATION_ACTION_REPEAT: u32 = 0x00000002;
pub(crate) const NV2080_CTRL_EVENT_SET_NOTIFICATION_ACTION_SINGLE: u32 = 0x00000001;
pub(crate) const NV2080_CTRL_EVENT_SET_NOTIFICATION_PARAMS_MESSAGE_ID: u32 = 0x1;
pub(crate) struct s_NV2080_CTRL_EVENT_SET_NOTIFICATION_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_EVENT_SET_NOTIFICATION_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        20
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 20) },
        }
    }

    pub(crate) fn event(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_event(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_event(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn action(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_action(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_action(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bNotifyState(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bNotifyState(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bNotifyState(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn info32(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_info32(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_info32(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn info16(self, fld: u16) -> Self {
        self.store[16..18].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_info16(&self) -> u16 {
        u16::from_le_bytes(self.store[16..18].try_into().unwrap())
    }
    pub(crate) fn set_info16(&mut self, fld: u16) {
        self.store[16..18].copy_from_slice(&u16::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO: u32 = 0x20801320;
pub(crate) const NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_MAX_ENTRIES: u32 = 16;
pub(crate) const NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_MEM_TYPES: u32 = 17;
pub(crate) const NV2080_CTRL_CMD_FB_GET_FB_REGION_INFO_PARAMS_MESSAGE_ID: u32 = 0x20;
pub(crate) const NV2080_CTRL_CMD_FIFO_GET_DEVICE_INFO_TABLE: u32 = 0x20801112;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_ENGINE_DATA_TYPES: u32 = 16;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_ENGINE_MAX_NAME_LEN: u32 = 16;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_ENGINE_MAX_PBDMA: u32 = 2;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_MAX_DEVICES: u32 = 256;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_MAX_ENTRIES: u32 = 32;
pub(crate) const NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_PARAMS_MESSAGE_ID: u32 = 0x12;
pub(crate) struct s_NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_FIFO_GET_DEVICE_INFO_TABLE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        3212
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 3212) },
        }
    }

    pub(crate) fn baseIndex(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_baseIndex(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_baseIndex(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numEntries(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numEntries(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_numEntries(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bMore(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bMore(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bMore(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }

    pub(crate) fn new_S_entries(&mut self, idx: isize) -> s_NV2080_CTRL_FIFO_DEVICE_ENTRY<'s> {
        s_NV2080_CTRL_FIFO_DEVICE_ENTRY::new(unsafe { self.ptr.byte_offset(idx * 100 + 12) })
    }
}

pub(crate) struct s_NV2080_CTRL_FIFO_DEVICE_ENTRY<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_FIFO_DEVICE_ENTRY<'s> {
    pub(crate) const fn str_size() -> usize {
        100
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 100) },
        }
    }

    pub(crate) fn engineData(self, fld: [u32; 16]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[0..64].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_engineData(&mut self, fld: [u32; 16]) {
        let mut byte_data = [0u8; 64];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[0..64].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_engineData(&mut self) -> [u32; 16] {
        let mut array = [0u32; 16];
        for (i, chunk) in self.store[0..64].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn pbdmaIds(self, fld: [u32; 2]) -> Self {
        let mut byte_data = [0u8; 8];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[64..72].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_pbdmaIds(&mut self, fld: [u32; 2]) {
        let mut byte_data = [0u8; 8];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[64..72].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_pbdmaIds(&mut self) -> [u32; 2] {
        let mut array = [0u32; 2];
        for (i, chunk) in self.store[64..72].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn pbdmaFaultIds(self, fld: [u32; 2]) -> Self {
        let mut byte_data = [0u8; 8];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[72..80].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_pbdmaFaultIds(&mut self, fld: [u32; 2]) {
        let mut byte_data = [0u8; 8];
        for i in 0..2 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[72..80].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_pbdmaFaultIds(&mut self) -> [u32; 2] {
        let mut array = [0u32; 2];
        for (i, chunk) in self.store[72..80].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn numPbdmas(self, fld: u32) -> Self {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numPbdmas(&self) -> u32 {
        u32::from_le_bytes(self.store[80..84].try_into().unwrap())
    }
    pub(crate) fn set_numPbdmas(&mut self, fld: u32) {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn engineName(self, fld: [u8; 16]) -> Self {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[84..100].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_engineName(&mut self, fld: [u8; 16]) {
        let mut byte_data = [0u8; 16];
        for i in 0..16 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[84..100].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_engineName(&mut self) -> [u8; 16] {
        let mut array = [0u8; 16];
        for (i, chunk) in self.store[84..100].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) const NV2080_CTRL_CMD_GPU_GET_FERMI_TPC_INFO: u32 = 0x20800138;
pub(crate) const NV2080_CTRL_GPU_GET_FERMI_TPC_INFO_PARAMS_MESSAGE_ID: u32 = 0x38;
pub(crate) struct s_NV2080_CTRL_GPU_GET_FERMI_TPC_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_FERMI_TPC_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn gpcId(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpcId(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_gpcId(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn tpcMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_tpcMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_tpcMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_GPU_GET_FERMI_GPC_INFO: u32 = 0x20800137;
pub(crate) const NV2080_CTRL_GPU_GET_FERMI_GPC_INFO_PARAMS_MESSAGE_ID: u32 = 0x37;
pub(crate) struct s_NV2080_CTRL_GPU_GET_FERMI_GPC_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_FERMI_GPC_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn gpcMask(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpcMask(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_gpcMask(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_GPU_PROMOTE_CTX: u32 = 0x2080012b;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_ATTRIBUTE_CB: u32 = 5;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_BUFFER_BUNDLE_CB: u32 = 3;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_FECS_EVENT: u32 = 9;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_GFXP_CTRL_BLK: u32 = 8;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_GFXP_POOL: u32 = 7;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_GLOBAL_PRIV_ACCESS_MAP: u32 = 12;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_MAIN: u32 = 0;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_PAGEPOOL: u32 = 4;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_PATCH: u32 = 2;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_PM: u32 = 1;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_PRIV_ACCESS_MAP: u32 = 10;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_RTV_CB_GLOBAL: u32 = 6;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ID_UNRESTRICTED_PRIV_ACCESS_MAP: u32 = 11;
pub(crate) const NV2080_CTRL_GPU_PROMOTE_CTX_PARAMS_MESSAGE_ID: u32 = 0x2B;
pub(crate) struct s_NV2080_CTRL_GPU_PROMOTE_CTX_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_PROMOTE_CTX_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        560
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 560) },
        }
    }

    pub(crate) fn engineType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_engineType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ChID(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ChID(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_ChID(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hChanClient(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hChanClient(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_hChanClient(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hObject(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hObject(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_hObject(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hVirtMemory(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hVirtMemory(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_hVirtMemory(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn virtAddress(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_virtAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_virtAddress(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn size(self, fld: u64) -> Self {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u64 {
        u64::from_le_bytes(self.store[32..40].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u64) {
        self.store[32..40].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn entryCount(self, fld: u32) -> Self {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_entryCount(&self) -> u32 {
        u32::from_le_bytes(self.store[40..44].try_into().unwrap())
    }
    pub(crate) fn set_entryCount(&mut self, fld: u32) {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_promoteEntry(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ENTRY<'s> {
        s_NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ENTRY::new(unsafe {
            self.ptr.byte_offset(idx * 32 + 48)
        })
    }
}

pub(crate) struct s_NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ENTRY<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_PROMOTE_CTX_BUFFER_ENTRY<'s> {
    pub(crate) const fn str_size() -> usize {
        32
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 32) },
        }
    }

    pub(crate) fn gpuPhysAddr(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_gpuPhysAddr(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gpuVirtAddr(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuVirtAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_gpuVirtAddr(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn size(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_size(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_size(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn physAttr(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_physAttr(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_physAttr(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bufferId(self, fld: u16) -> Self {
        self.store[28..30].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bufferId(&self) -> u16 {
        u16::from_le_bytes(self.store[28..30].try_into().unwrap())
    }
    pub(crate) fn set_bufferId(&mut self, fld: u16) {
        self.store[28..30].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn bInitialize(self, fld: u8) -> Self {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bInitialize(&self) -> u8 {
        u8::from_le_bytes(self.store[30..31].try_into().unwrap())
    }
    pub(crate) fn set_bInitialize(&mut self, fld: u8) {
        self.store[30..31].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bNonmapped(self, fld: u8) -> Self {
        self.store[31..32].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bNonmapped(&self) -> u8 {
        u8::from_le_bytes(self.store[31..32].try_into().unwrap())
    }
    pub(crate) fn set_bNonmapped(&mut self, fld: u8) {
        self.store[31..32].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_GPU_GET_CONSTRUCTED_FALCON_INFO: u32 = 0x208001b0;
pub(crate) const NV2080_CTRL_GPU_GET_CONSTRUCTED_FALCON_INFO_PARAMS_MESSAGE_ID: u32 = 0xB0;
pub(crate) struct s_NV2080_CTRL_GPU_GET_CONSTRUCTED_FALCON_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_CONSTRUCTED_FALCON_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        1284
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 1284) },
        }
    }

    pub(crate) fn numConstructedFalcons(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numConstructedFalcons(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_numConstructedFalcons(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_constructedFalconsTable(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_GPU_CONSTRUCTED_FALCON_INFO<'s> {
        s_NV2080_CTRL_GPU_CONSTRUCTED_FALCON_INFO::new(unsafe {
            self.ptr.byte_offset(idx * 20 + 4)
        })
    }
}

pub(crate) struct s_NV2080_CTRL_GPU_CONSTRUCTED_FALCON_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_CONSTRUCTED_FALCON_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        20
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 20) },
        }
    }

    pub(crate) fn engDesc(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engDesc(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_engDesc(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ctxAttr(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ctxAttr(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_ctxAttr(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn ctxBufferSize(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ctxBufferSize(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_ctxBufferSize(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn addrSpaceList(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addrSpaceList(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_addrSpaceList(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn registerBase(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_registerBase(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_registerBase(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_GPU_GET_VMMU_SEGMENT_SIZE: u32 = 0x2080017e;
pub(crate) const NV2080_CTRL_GPU_GET_VMMU_SEGMENT_SIZE_PARAMS_MESSAGE_ID: u32 = 0x7E;
pub(crate) struct s_NV2080_CTRL_GPU_GET_VMMU_SEGMENT_SIZE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_GPU_GET_VMMU_SEGMENT_SIZE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        8
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 8) },
        }
    }

    pub(crate) fn vmmuSegmentSize(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vmmuSegmentSize(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_vmmuSegmentSize(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_INTERNAL_STATIC_GR_GET_CONTEXT_BUFFERS_INFO_PARAMS_MESSAGE_ID: u32 =
    0x33;
pub(crate) struct s_NV2080_CTRL_INTERNAL_STATIC_GR_GET_CONTEXT_BUFFERS_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_STATIC_GR_GET_CONTEXT_BUFFERS_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        1664
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 1664) },
        }
    }

    pub(crate) fn new_S_engineContextBuffersInfo(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_INTERNAL_STATIC_GR_CONTEXT_BUFFERS_INFO<'s> {
        s_NV2080_CTRL_INTERNAL_STATIC_GR_CONTEXT_BUFFERS_INFO::new(unsafe {
            self.ptr.byte_offset(idx * 208 + 0)
        })
    }
}

pub(crate) struct s_NV2080_CTRL_INTERNAL_STATIC_GR_CONTEXT_BUFFERS_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_STATIC_GR_CONTEXT_BUFFERS_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        208
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 208) },
        }
    }

    pub(crate) fn new_S_engine(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_INTERNAL_ENGINE_CONTEXT_BUFFER_INFO<'s> {
        s_NV2080_CTRL_INTERNAL_ENGINE_CONTEXT_BUFFER_INFO::new(unsafe {
            self.ptr.byte_offset(idx * 8 + 0)
        })
    }
}

pub(crate) const NV2080_CTRL_CMD_INTERNAL_STATIC_KGR_GET_CONTEXT_BUFFERS_INFO: u32 = 0x20800a32;
pub(crate) const NV2080_CTRL_INTERNAL_STATIC_KGR_GET_CONTEXT_BUFFERS_INFO_PARAMS_MESSAGE_ID: u32 =
    0x32;
pub(crate) const NV2080_CTRL_CMD_INTERNAL_DISPLAY_CHANNEL_PUSHBUFFER: u32 = 0x20800a58;
pub(crate) const NV2080_CTRL_INTERNAL_DISPLAY_CHANNEL_PUSHBUFFER_PARAMS_MESSAGE_ID: u32 = 0x58;
pub(crate) struct s_NV2080_CTRL_INTERNAL_DISPLAY_CHANNEL_PUSHBUFFER_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_DISPLAY_CHANNEL_PUSHBUFFER_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        56
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 56) },
        }
    }

    pub(crate) fn addressSpace(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_addressSpace(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_addressSpace(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn physicalAddr(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_physicalAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_physicalAddr(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn limit(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_limit(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_limit(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn cacheSnoop(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cacheSnoop(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_cacheSnoop(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hclass(self, fld: u32) -> Self {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hclass(&self) -> u32 {
        u32::from_le_bytes(self.store[28..32].try_into().unwrap())
    }
    pub(crate) fn set_hclass(&mut self, fld: u32) {
        self.store[28..32].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn channelInstance(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_channelInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_channelInstance(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn valid(self, fld: u8) -> Self {
        self.store[36..37].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_valid(&self) -> u8 {
        u8::from_le_bytes(self.store[36..37].try_into().unwrap())
    }
    pub(crate) fn set_valid(&mut self, fld: u8) {
        self.store[36..37].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn pbTargetAperture(self, fld: u32) -> Self {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pbTargetAperture(&self) -> u32 {
        u32::from_le_bytes(self.store[40..44].try_into().unwrap())
    }
    pub(crate) fn set_pbTargetAperture(&mut self, fld: u32) {
        self.store[40..44].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn channelPBSize(self, fld: u32) -> Self {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_channelPBSize(&self) -> u32 {
        u32::from_le_bytes(self.store[44..48].try_into().unwrap())
    }
    pub(crate) fn set_channelPBSize(&mut self, fld: u32) {
        self.store[44..48].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn subDeviceId(self, fld: u32) -> Self {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[48..52].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceId(&mut self, fld: u32) {
        self.store[48..52].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_INTERNAL_DISPLAY_WRITE_INST_MEM: u32 = 0x20800a49;
pub(crate) const NV2080_CTRL_INTERNAL_DISPLAY_WRITE_INST_MEM_PARAMS_MESSAGE_ID: u32 = 0x49;
pub(crate) struct s_NV2080_CTRL_INTERNAL_DISPLAY_WRITE_INST_MEM_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_DISPLAY_WRITE_INST_MEM_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn instMemPhysAddr(self, fld: u64) -> Self {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_instMemPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[0..8].try_into().unwrap())
    }
    pub(crate) fn set_instMemPhysAddr(&mut self, fld: u64) {
        self.store[0..8].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn instMemSize(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_instMemSize(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_instMemSize(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn instMemAddrSpace(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_instMemAddrSpace(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_instMemAddrSpace(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn instMemCpuCacheAttr(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_instMemCpuCacheAttr(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_instMemCpuCacheAttr(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_INTERNAL_DISPLAY_GET_STATIC_INFO: u32 = 0x20800a01;
pub(crate) const NV2080_CTRL_INTERNAL_DISPLAY_GET_STATIC_INFO_PARAMS_MESSAGE_ID: u32 = 0x1;
pub(crate) struct s_NV2080_CTRL_INTERNAL_DISPLAY_GET_STATIC_INFO_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_DISPLAY_GET_STATIC_INFO_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        36
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 36) },
        }
    }

    pub(crate) fn feHwSysCap(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_feHwSysCap(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_feHwSysCap(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn windowPresentMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_windowPresentMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_windowPresentMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bFbRemapperEnabled(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bFbRemapperEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bFbRemapperEnabled(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn numHeads(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numHeads(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_numHeads(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn i2cPort(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_i2cPort(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_i2cPort(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn internalDispActiveMask(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_internalDispActiveMask(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_internalDispActiveMask(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn embeddedDisplayPortMask(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_embeddedDisplayPortMask(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_embeddedDisplayPortMask(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bExternalMuxSupported(self, fld: u8) -> Self {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bExternalMuxSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[28..29].try_into().unwrap())
    }
    pub(crate) fn set_bExternalMuxSupported(&mut self, fld: u8) {
        self.store[28..29].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bInternalMuxSupported(self, fld: u8) -> Self {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bInternalMuxSupported(&self) -> u8 {
        u8::from_le_bytes(self.store[29..30].try_into().unwrap())
    }
    pub(crate) fn set_bInternalMuxSupported(&mut self, fld: u8) {
        self.store[29..30].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn numDispChannels(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numDispChannels(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_numDispChannels(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_INTERNAL_FBSR_INIT: u32 = 0x20800ac2;
pub(crate) const NV2080_CTRL_INTERNAL_FBSR_INIT_PARAMS_MESSAGE_ID: u32 = 0xC2;
pub(crate) struct s_NV2080_CTRL_INTERNAL_FBSR_INIT_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_FBSR_INIT_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        24
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 24) },
        }
    }

    pub(crate) fn hClient(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hClient(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hClient(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn hSysMem(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hSysMem(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_hSysMem(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bEnteringGcoffState(self, fld: u8) -> Self {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bEnteringGcoffState(&self) -> u8 {
        u8::from_le_bytes(self.store[8..9].try_into().unwrap())
    }
    pub(crate) fn set_bEnteringGcoffState(&mut self, fld: u8) {
        self.store[8..9].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn sysmemAddrOfSuspendResumeData(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_sysmemAddrOfSuspendResumeData(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_sysmemAddrOfSuspendResumeData(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_INTERNAL_INTR_GET_KERNEL_TABLE: u32 = 0x20800a5c;
pub(crate) const NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_PARAMS_MESSAGE_ID: u32 = 0x5C;
pub(crate) struct s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        2068
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 2068) },
        }
    }

    pub(crate) fn tableLen(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_tableLen(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_tableLen(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_table(
        &mut self,
        idx: isize,
    ) -> s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_ENTRY<'s> {
        s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_ENTRY::new(unsafe {
            self.ptr.byte_offset(idx * 16 + 4)
        })
    }

    pub(crate) fn new_S_subtreeMap(
        &mut self,
        idx: isize,
    ) -> s_NV2080_INTR_CATEGORY_SUBTREE_MAP<'s> {
        s_NV2080_INTR_CATEGORY_SUBTREE_MAP::new(unsafe { self.ptr.byte_offset(idx * 2 + 2052) })
    }
}

pub(crate) struct s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_ENTRY<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_INTERNAL_INTR_GET_KERNEL_TABLE_ENTRY<'s> {
    pub(crate) const fn str_size() -> usize {
        16
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 16) },
        }
    }

    pub(crate) fn engineIdx(self, fld: u16) -> Self {
        self.store[0..2].copy_from_slice(&u16::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineIdx(&self) -> u16 {
        u16::from_le_bytes(self.store[0..2].try_into().unwrap())
    }
    pub(crate) fn set_engineIdx(&mut self, fld: u16) {
        self.store[0..2].copy_from_slice(&u16::to_le_bytes(fld));
    }
    pub(crate) fn pmcIntrMask(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pmcIntrMask(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_pmcIntrMask(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vectorStall(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vectorStall(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_vectorStall(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vectorNonStall(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vectorNonStall(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_vectorNonStall(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) struct s_NV2080_INTR_CATEGORY_SUBTREE_MAP<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_INTR_CATEGORY_SUBTREE_MAP<'s> {
    pub(crate) const fn str_size() -> usize {
        2
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 2) },
        }
    }

    pub(crate) fn subtreeStart(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subtreeStart(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_subtreeStart(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn subtreeEnd(self, fld: u8) -> Self {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subtreeEnd(&self) -> u8 {
        u8::from_le_bytes(self.store[1..2].try_into().unwrap())
    }
    pub(crate) fn set_subtreeEnd(&mut self, fld: u8) {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_VGPU_MGR_INTERNAL_SHUTDOWN_GSP_VGPU_PLUGIN_TASK: u32 = 0x20804002;
pub(crate) const NV2080_CTRL_VGPU_MGR_INTERNAL_SHUTDOWN_GSP_VGPU_PLUGIN_TASK_PARAMS_MESSAGE_ID:
    u32 = 0x2;
pub(crate) struct s_NV2080_CTRL_VGPU_MGR_INTERNAL_SHUTDOWN_GSP_VGPU_PLUGIN_TASK_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_VGPU_MGR_INTERNAL_SHUTDOWN_GSP_VGPU_PLUGIN_TASK_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn gfid(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gfid(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_gfid(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_VGPU_MGR_INTERNAL_VGPU_PLUGIN_CLEANUP: u32 = 0x20804008;
pub(crate) const NV2080_CTRL_VGPU_MGR_INTERNAL_VGPU_PLUGIN_CLEANUP_PARAMS_MESSAGE_ID: u32 = 0x8;
pub(crate) struct s_NV2080_CTRL_VGPU_MGR_INTERNAL_VGPU_PLUGIN_CLEANUP_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_VGPU_MGR_INTERNAL_VGPU_PLUGIN_CLEANUP_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn gfid(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gfid(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_gfid(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_VGPU_MGR_INTERNAL_BOOTLOAD_GSP_VGPU_PLUGIN_TASK: u32 = 0x20804001;
pub(crate) const NV2080_CTRL_VGPU_MGR_INTERNAL_BOOTLOAD_GSP_VGPU_PLUGIN_TASK_PARAMS_MESSAGE_ID:
    u32 = 0x1;
pub(crate) struct s_NV2080_CTRL_VGPU_MGR_INTERNAL_BOOTLOAD_GSP_VGPU_PLUGIN_TASK_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_VGPU_MGR_INTERNAL_BOOTLOAD_GSP_VGPU_PLUGIN_TASK_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        6616
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 6616) },
        }
    }

    pub(crate) fn dbdf(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_dbdf(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_dbdf(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gfid(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gfid(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_gfid(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vgpuType(self, fld: u32) -> Self {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgpuType(&self) -> u32 {
        u32::from_le_bytes(self.store[8..12].try_into().unwrap())
    }
    pub(crate) fn set_vgpuType(&mut self, fld: u32) {
        self.store[8..12].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vmPid(self, fld: u32) -> Self {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vmPid(&self) -> u32 {
        u32::from_le_bytes(self.store[12..16].try_into().unwrap())
    }
    pub(crate) fn set_vmPid(&mut self, fld: u32) {
        self.store[12..16].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn swizzId(self, fld: u32) -> Self {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_swizzId(&self) -> u32 {
        u32::from_le_bytes(self.store[16..20].try_into().unwrap())
    }
    pub(crate) fn set_swizzId(&mut self, fld: u32) {
        self.store[16..20].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numChannels(self, fld: u32) -> Self {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numChannels(&self) -> u32 {
        u32::from_le_bytes(self.store[20..24].try_into().unwrap())
    }
    pub(crate) fn set_numChannels(&mut self, fld: u32) {
        self.store[20..24].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numPluginChannels(self, fld: u32) -> Self {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numPluginChannels(&self) -> u32 {
        u32::from_le_bytes(self.store[24..28].try_into().unwrap())
    }
    pub(crate) fn set_numPluginChannels(&mut self, fld: u32) {
        self.store[24..28].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn chidOffset(self, fld: [u32; 84]) -> Self {
        let mut byte_data = [0u8; 336];
        for i in 0..84 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[28..364].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_chidOffset(&mut self, fld: [u32; 84]) {
        let mut byte_data = [0u8; 336];
        for i in 0..84 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[28..364].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_chidOffset(&mut self) -> [u32; 84] {
        let mut array = [0u32; 84];
        for (i, chunk) in self.store[28..364].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn bDisableDefaultSmcExecPartRestore(self, fld: u8) -> Self {
        self.store[364..365].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bDisableDefaultSmcExecPartRestore(&self) -> u8 {
        u8::from_le_bytes(self.store[364..365].try_into().unwrap())
    }
    pub(crate) fn set_bDisableDefaultSmcExecPartRestore(&mut self, fld: u8) {
        self.store[364..365].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn numGuestFbSegments(self, fld: u32) -> Self {
        self.store[368..372].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numGuestFbSegments(&self) -> u32 {
        u32::from_le_bytes(self.store[368..372].try_into().unwrap())
    }
    pub(crate) fn set_numGuestFbSegments(&mut self, fld: u32) {
        self.store[368..372].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn guestFbPhysAddrList(self, fld: [u64; 384]) -> Self {
        let mut byte_data = [0u8; 3072];
        for i in 0..384 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[376..3448].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_guestFbPhysAddrList(&mut self, fld: [u64; 384]) {
        let mut byte_data = [0u8; 3072];
        for i in 0..384 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[376..3448].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_guestFbPhysAddrList(&mut self) -> [u64; 384] {
        let mut array = [0u64; 384];
        for (i, chunk) in self.store[376..3448].chunks_exact(8).enumerate() {
            array[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn guestFbLengthList(self, fld: [u64; 384]) -> Self {
        let mut byte_data = [0u8; 3072];
        for i in 0..384 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[3448..6520].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_guestFbLengthList(&mut self, fld: [u64; 384]) {
        let mut byte_data = [0u8; 3072];
        for i in 0..384 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 8)..((i + 1) * 8)].copy_from_slice(&bytes);
        }
        self.store[3448..6520].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_guestFbLengthList(&mut self) -> [u64; 384] {
        let mut array = [0u64; 384];
        for (i, chunk) in self.store[3448..6520].chunks_exact(8).enumerate() {
            array[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn pluginHeapMemoryPhysAddr(self, fld: u64) -> Self {
        self.store[6520..6528].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pluginHeapMemoryPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[6520..6528].try_into().unwrap())
    }
    pub(crate) fn set_pluginHeapMemoryPhysAddr(&mut self, fld: u64) {
        self.store[6520..6528].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pluginHeapMemoryLength(self, fld: u64) -> Self {
        self.store[6528..6536].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pluginHeapMemoryLength(&self) -> u64 {
        u64::from_le_bytes(self.store[6528..6536].try_into().unwrap())
    }
    pub(crate) fn set_pluginHeapMemoryLength(&mut self, fld: u64) {
        self.store[6528..6536].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn ctrlBuffOffset(self, fld: u64) -> Self {
        self.store[6536..6544].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ctrlBuffOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[6536..6544].try_into().unwrap())
    }
    pub(crate) fn set_ctrlBuffOffset(&mut self, fld: u64) {
        self.store[6536..6544].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn initTaskLogBuffOffset(self, fld: u64) -> Self {
        self.store[6544..6552].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_initTaskLogBuffOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[6544..6552].try_into().unwrap())
    }
    pub(crate) fn set_initTaskLogBuffOffset(&mut self, fld: u64) {
        self.store[6544..6552].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn initTaskLogBuffSize(self, fld: u64) -> Self {
        self.store[6552..6560].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_initTaskLogBuffSize(&self) -> u64 {
        u64::from_le_bytes(self.store[6552..6560].try_into().unwrap())
    }
    pub(crate) fn set_initTaskLogBuffSize(&mut self, fld: u64) {
        self.store[6552..6560].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vgpuTaskLogBuffOffset(self, fld: u64) -> Self {
        self.store[6560..6568].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgpuTaskLogBuffOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[6560..6568].try_into().unwrap())
    }
    pub(crate) fn set_vgpuTaskLogBuffOffset(&mut self, fld: u64) {
        self.store[6560..6568].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn vgpuTaskLogBuffSize(self, fld: u64) -> Self {
        self.store[6568..6576].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgpuTaskLogBuffSize(&self) -> u64 {
        u64::from_le_bytes(self.store[6568..6576].try_into().unwrap())
    }
    pub(crate) fn set_vgpuTaskLogBuffSize(&mut self, fld: u64) {
        self.store[6568..6576].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn kernelLogBuffOffset(self, fld: u64) -> Self {
        self.store[6576..6584].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_kernelLogBuffOffset(&self) -> u64 {
        u64::from_le_bytes(self.store[6576..6584].try_into().unwrap())
    }
    pub(crate) fn set_kernelLogBuffOffset(&mut self, fld: u64) {
        self.store[6576..6584].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn kernelLogBuffSize(self, fld: u64) -> Self {
        self.store[6584..6592].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_kernelLogBuffSize(&self) -> u64 {
        u64::from_le_bytes(self.store[6584..6592].try_into().unwrap())
    }
    pub(crate) fn set_kernelLogBuffSize(&mut self, fld: u64) {
        self.store[6584..6592].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn migRmHeapMemoryPhysAddr(self, fld: u64) -> Self {
        self.store[6592..6600].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_migRmHeapMemoryPhysAddr(&self) -> u64 {
        u64::from_le_bytes(self.store[6592..6600].try_into().unwrap())
    }
    pub(crate) fn set_migRmHeapMemoryPhysAddr(&mut self, fld: u64) {
        self.store[6592..6600].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn migRmHeapMemoryLength(self, fld: u64) -> Self {
        self.store[6600..6608].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_migRmHeapMemoryLength(&self) -> u64 {
        u64::from_le_bytes(self.store[6600..6608].try_into().unwrap())
    }
    pub(crate) fn set_migRmHeapMemoryLength(&mut self, fld: u64) {
        self.store[6600..6608].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn bDeviceProfilingEnabled(self, fld: u8) -> Self {
        self.store[6608..6609].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bDeviceProfilingEnabled(&self) -> u8 {
        u8::from_le_bytes(self.store[6608..6609].try_into().unwrap())
    }
    pub(crate) fn set_bDeviceProfilingEnabled(&mut self, fld: u8) {
        self.store[6608..6609].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NV2080_CTRL_CMD_VGPU_MGR_INTERNAL_PGPU_ADD_VGPU_TYPE: u32 = 0x20804003;
pub(crate) const NV2080_CTRL_VGPU_MGR_INTERNAL_PGPU_ADD_VGPU_TYPE_PARAMS_MESSAGE_ID: u32 = 0x3;
pub(crate) struct s_NV2080_CTRL_VGPU_MGR_INTERNAL_PGPU_ADD_VGPU_TYPE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV2080_CTRL_VGPU_MGR_INTERNAL_PGPU_ADD_VGPU_TYPE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        334344
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 334344) },
        }
    }

    pub(crate) fn discardVgpuTypes(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_discardVgpuTypes(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_discardVgpuTypes(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn vgpuInfoCount(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgpuInfoCount(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_vgpuInfoCount(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }

    pub(crate) fn new_S_vgpuInfo(&mut self, idx: isize) -> s_NVA081_CTRL_VGPU_INFO<'s> {
        s_NVA081_CTRL_VGPU_INFO::new(unsafe { self.ptr.byte_offset(idx * 5224 + 8) })
    }
}

pub(crate) struct s_NVA081_CTRL_VGPU_INFO<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NVA081_CTRL_VGPU_INFO<'s> {
    pub(crate) const fn str_size() -> usize {
        5224
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 5224) },
        }
    }

    pub(crate) fn vgpuType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vgpuType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_vgpuType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vgpuName(self, fld: [u8; 32]) -> Self {
        let mut byte_data = [0u8; 32];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[4..36].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_vgpuName(&mut self, fld: [u8; 32]) {
        let mut byte_data = [0u8; 32];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[4..36].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_vgpuName(&mut self) -> [u8; 32] {
        let mut array = [0u8; 32];
        for (i, chunk) in self.store[4..36].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn vgpuClass(self, fld: [u8; 32]) -> Self {
        let mut byte_data = [0u8; 32];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[36..68].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_vgpuClass(&mut self, fld: [u8; 32]) {
        let mut byte_data = [0u8; 32];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[36..68].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_vgpuClass(&mut self) -> [u8; 32] {
        let mut array = [0u8; 32];
        for (i, chunk) in self.store[36..68].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn vgpuSignature(self, fld: [u8; 128]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[68..196].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_vgpuSignature(&mut self, fld: [u8; 128]) {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[68..196].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_vgpuSignature(&mut self) -> [u8; 128] {
        let mut array = [0u8; 128];
        for (i, chunk) in self.store[68..196].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn license(self, fld: [u8; 128]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[196..324].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_license(&mut self, fld: [u8; 128]) {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[196..324].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_license(&mut self) -> [u8; 128] {
        let mut array = [0u8; 128];
        for (i, chunk) in self.store[196..324].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn maxInstance(self, fld: u32) -> Self {
        self.store[324..328].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxInstance(&self) -> u32 {
        u32::from_le_bytes(self.store[324..328].try_into().unwrap())
    }
    pub(crate) fn set_maxInstance(&mut self, fld: u32) {
        self.store[324..328].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn numHeads(self, fld: u32) -> Self {
        self.store[328..332].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numHeads(&self) -> u32 {
        u32::from_le_bytes(self.store[328..332].try_into().unwrap())
    }
    pub(crate) fn set_numHeads(&mut self, fld: u32) {
        self.store[328..332].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxResolutionX(self, fld: u32) -> Self {
        self.store[332..336].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxResolutionX(&self) -> u32 {
        u32::from_le_bytes(self.store[332..336].try_into().unwrap())
    }
    pub(crate) fn set_maxResolutionX(&mut self, fld: u32) {
        self.store[332..336].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxResolutionY(self, fld: u32) -> Self {
        self.store[336..340].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxResolutionY(&self) -> u32 {
        u32::from_le_bytes(self.store[336..340].try_into().unwrap())
    }
    pub(crate) fn set_maxResolutionY(&mut self, fld: u32) {
        self.store[336..340].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn maxPixels(self, fld: u32) -> Self {
        self.store[340..344].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_maxPixels(&self) -> u32 {
        u32::from_le_bytes(self.store[340..344].try_into().unwrap())
    }
    pub(crate) fn set_maxPixels(&mut self, fld: u32) {
        self.store[340..344].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn frlConfig(self, fld: u32) -> Self {
        self.store[344..348].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_frlConfig(&self) -> u32 {
        u32::from_le_bytes(self.store[344..348].try_into().unwrap())
    }
    pub(crate) fn set_frlConfig(&mut self, fld: u32) {
        self.store[344..348].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn cudaEnabled(self, fld: u32) -> Self {
        self.store[348..352].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_cudaEnabled(&self) -> u32 {
        u32::from_le_bytes(self.store[348..352].try_into().unwrap())
    }
    pub(crate) fn set_cudaEnabled(&mut self, fld: u32) {
        self.store[348..352].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn eccSupported(self, fld: u32) -> Self {
        self.store[352..356].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_eccSupported(&self) -> u32 {
        u32::from_le_bytes(self.store[352..356].try_into().unwrap())
    }
    pub(crate) fn set_eccSupported(&mut self, fld: u32) {
        self.store[352..356].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gpuInstanceSize(self, fld: u32) -> Self {
        self.store[356..360].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuInstanceSize(&self) -> u32 {
        u32::from_le_bytes(self.store[356..360].try_into().unwrap())
    }
    pub(crate) fn set_gpuInstanceSize(&mut self, fld: u32) {
        self.store[356..360].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn multiVgpuSupported(self, fld: u32) -> Self {
        self.store[360..364].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_multiVgpuSupported(&self) -> u32 {
        u32::from_le_bytes(self.store[360..364].try_into().unwrap())
    }
    pub(crate) fn set_multiVgpuSupported(&mut self, fld: u32) {
        self.store[360..364].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn vdevId(self, fld: u64) -> Self {
        self.store[368..376].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_vdevId(&self) -> u64 {
        u64::from_le_bytes(self.store[368..376].try_into().unwrap())
    }
    pub(crate) fn set_vdevId(&mut self, fld: u64) {
        self.store[368..376].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn pdevId(self, fld: u64) -> Self {
        self.store[376..384].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pdevId(&self) -> u64 {
        u64::from_le_bytes(self.store[376..384].try_into().unwrap())
    }
    pub(crate) fn set_pdevId(&mut self, fld: u64) {
        self.store[376..384].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn profileSize(self, fld: u64) -> Self {
        self.store[384..392].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_profileSize(&self) -> u64 {
        u64::from_le_bytes(self.store[384..392].try_into().unwrap())
    }
    pub(crate) fn set_profileSize(&mut self, fld: u64) {
        self.store[384..392].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn fbLength(self, fld: u64) -> Self {
        self.store[392..400].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fbLength(&self) -> u64 {
        u64::from_le_bytes(self.store[392..400].try_into().unwrap())
    }
    pub(crate) fn set_fbLength(&mut self, fld: u64) {
        self.store[392..400].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn gspHeapSize(self, fld: u64) -> Self {
        self.store[400..408].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gspHeapSize(&self) -> u64 {
        u64::from_le_bytes(self.store[400..408].try_into().unwrap())
    }
    pub(crate) fn set_gspHeapSize(&mut self, fld: u64) {
        self.store[400..408].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn fbReservation(self, fld: u64) -> Self {
        self.store[408..416].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_fbReservation(&self) -> u64 {
        u64::from_le_bytes(self.store[408..416].try_into().unwrap())
    }
    pub(crate) fn set_fbReservation(&mut self, fld: u64) {
        self.store[408..416].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn mappableVideoSize(self, fld: u64) -> Self {
        self.store[416..424].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_mappableVideoSize(&self) -> u64 {
        u64::from_le_bytes(self.store[416..424].try_into().unwrap())
    }
    pub(crate) fn set_mappableVideoSize(&mut self, fld: u64) {
        self.store[416..424].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn encoderCapacity(self, fld: u32) -> Self {
        self.store[424..428].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_encoderCapacity(&self) -> u32 {
        u32::from_le_bytes(self.store[424..428].try_into().unwrap())
    }
    pub(crate) fn set_encoderCapacity(&mut self, fld: u32) {
        self.store[424..428].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn bar1Length(self, fld: u64) -> Self {
        self.store[432..440].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bar1Length(&self) -> u64 {
        u64::from_le_bytes(self.store[432..440].try_into().unwrap())
    }
    pub(crate) fn set_bar1Length(&mut self, fld: u64) {
        self.store[432..440].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn frlEnable(self, fld: u32) -> Self {
        self.store[440..444].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_frlEnable(&self) -> u32 {
        u32::from_le_bytes(self.store[440..444].try_into().unwrap())
    }
    pub(crate) fn set_frlEnable(&mut self, fld: u32) {
        self.store[440..444].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn adapterName(self, fld: [u8; 64]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[444..508].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_adapterName(&mut self, fld: [u8; 64]) {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[444..508].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_adapterName(&mut self) -> [u8; 64] {
        let mut array = [0u8; 64];
        for (i, chunk) in self.store[444..508].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn adapterName_Unicode(self, fld: [u16; 64]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 2)..((i + 1) * 2)].copy_from_slice(&bytes);
        }
        self.store[508..636].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_adapterName_Unicode(&mut self, fld: [u16; 64]) {
        let mut byte_data = [0u8; 128];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 2)..((i + 1) * 2)].copy_from_slice(&bytes);
        }
        self.store[508..636].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_adapterName_Unicode(&mut self) -> [u16; 64] {
        let mut array = [0u16; 64];
        for (i, chunk) in self.store[508..636].chunks_exact(2).enumerate() {
            array[i] = u16::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn shortGpuNameString(self, fld: [u8; 64]) -> Self {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[636..700].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_shortGpuNameString(&mut self, fld: [u8; 64]) {
        let mut byte_data = [0u8; 64];
        for i in 0..64 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[636..700].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_shortGpuNameString(&mut self) -> [u8; 64] {
        let mut array = [0u8; 64];
        for (i, chunk) in self.store[636..700].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn licensedProductName(self, fld: [u8; 128]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[700..828].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_licensedProductName(&mut self, fld: [u8; 128]) {
        let mut byte_data = [0u8; 128];
        for i in 0..128 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 1)..((i + 1) * 1)].copy_from_slice(&bytes);
        }
        self.store[700..828].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_licensedProductName(&mut self) -> [u8; 128] {
        let mut array = [0u8; 128];
        for (i, chunk) in self.store[700..828].chunks_exact(1).enumerate() {
            array[i] = u8::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn vgpuExtraParams(self, fld: [u32; 1024]) -> Self {
        let mut byte_data = [0u8; 4096];
        for i in 0..1024 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[828..4924].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_vgpuExtraParams(&mut self, fld: [u32; 1024]) {
        let mut byte_data = [0u8; 4096];
        for i in 0..1024 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[828..4924].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_vgpuExtraParams(&mut self) -> [u32; 1024] {
        let mut array = [0u32; 1024];
        for (i, chunk) in self.store[828..4924].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn ftraceEnable(self, fld: u32) -> Self {
        self.store[4924..4928].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_ftraceEnable(&self) -> u32 {
        u32::from_le_bytes(self.store[4924..4928].try_into().unwrap())
    }
    pub(crate) fn set_ftraceEnable(&mut self, fld: u32) {
        self.store[4924..4928].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gpuDirectSupported(self, fld: u32) -> Self {
        self.store[4928..4932].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuDirectSupported(&self) -> u32 {
        u32::from_le_bytes(self.store[4928..4932].try_into().unwrap())
    }
    pub(crate) fn set_gpuDirectSupported(&mut self, fld: u32) {
        self.store[4928..4932].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn nvlinkP2PSupported(self, fld: u32) -> Self {
        self.store[4932..4936].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_nvlinkP2PSupported(&self) -> u32 {
        u32::from_le_bytes(self.store[4932..4936].try_into().unwrap())
    }
    pub(crate) fn set_nvlinkP2PSupported(&mut self, fld: u32) {
        self.store[4932..4936].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn multiVgpuExclusive(self, fld: u32) -> Self {
        self.store[4936..4940].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_multiVgpuExclusive(&self) -> u32 {
        u32::from_le_bytes(self.store[4936..4940].try_into().unwrap())
    }
    pub(crate) fn set_multiVgpuExclusive(&mut self, fld: u32) {
        self.store[4936..4940].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn exclusiveType(self, fld: u32) -> Self {
        self.store[4940..4944].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_exclusiveType(&self) -> u32 {
        u32::from_le_bytes(self.store[4940..4944].try_into().unwrap())
    }
    pub(crate) fn set_exclusiveType(&mut self, fld: u32) {
        self.store[4940..4944].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn exclusiveSize(self, fld: u32) -> Self {
        self.store[4944..4948].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_exclusiveSize(&self) -> u32 {
        u32::from_le_bytes(self.store[4944..4948].try_into().unwrap())
    }
    pub(crate) fn set_exclusiveSize(&mut self, fld: u32) {
        self.store[4944..4948].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn gpuInstanceProfileId(self, fld: u32) -> Self {
        self.store[4948..4952].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_gpuInstanceProfileId(&self) -> u32 {
        u32::from_le_bytes(self.store[4948..4952].try_into().unwrap())
    }
    pub(crate) fn set_gpuInstanceProfileId(&mut self, fld: u32) {
        self.store[4948..4952].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn placementSize(self, fld: u32) -> Self {
        self.store[4952..4956].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_placementSize(&self) -> u32 {
        u32::from_le_bytes(self.store[4952..4956].try_into().unwrap())
    }
    pub(crate) fn set_placementSize(&mut self, fld: u32) {
        self.store[4952..4956].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn homogeneousPlacementCount(self, fld: u32) -> Self {
        self.store[4956..4960].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_homogeneousPlacementCount(&self) -> u32 {
        u32::from_le_bytes(self.store[4956..4960].try_into().unwrap())
    }
    pub(crate) fn set_homogeneousPlacementCount(&mut self, fld: u32) {
        self.store[4956..4960].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn homogeneousPlacementIds(self, fld: [u32; 32]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[4960..5088].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_homogeneousPlacementIds(&mut self, fld: [u32; 32]) {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[4960..5088].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_homogeneousPlacementIds(&mut self) -> [u32; 32] {
        let mut array = [0u32; 32];
        for (i, chunk) in self.store[4960..5088].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
    pub(crate) fn heterogeneousPlacementCount(self, fld: u32) -> Self {
        self.store[5088..5092].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_heterogeneousPlacementCount(&self) -> u32 {
        u32::from_le_bytes(self.store[5088..5092].try_into().unwrap())
    }
    pub(crate) fn set_heterogeneousPlacementCount(&mut self, fld: u32) {
        self.store[5088..5092].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn heterogeneousPlacementIds(self, fld: [u32; 32]) -> Self {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[5092..5220].copy_from_slice(&byte_data);
        self
    }
    pub(crate) fn set_heterogeneousPlacementIds(&mut self, fld: [u32; 32]) {
        let mut byte_data = [0u8; 128];
        for i in 0..32 {
            let bytes = fld[i].to_le_bytes();
            byte_data[(i * 4)..((i + 1) * 4)].copy_from_slice(&bytes);
        }
        self.store[5092..5220].copy_from_slice(&byte_data);
    }
    pub(crate) fn get_heterogeneousPlacementIds(&mut self) -> [u32; 32] {
        let mut array = [0u32; 32];
        for (i, chunk) in self.store[5092..5220].chunks_exact(4).enumerate() {
            array[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        array
    }
}

pub(crate) const NV90F1_CTRL_CMD_VASPACE_COPY_SERVER_RESERVED_PDES: u32 = 0x90f10106;
pub(crate) const NV90F1_CTRL_VASPACE_COPY_SERVER_RESERVED_PDES_PARAMS_MESSAGE_ID: u32 = 0x6;
pub(crate) struct s_NV90F1_CTRL_VASPACE_COPY_SERVER_RESERVED_PDES_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NV90F1_CTRL_VASPACE_COPY_SERVER_RESERVED_PDES_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        184
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 184) },
        }
    }

    pub(crate) fn hSubDevice(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_hSubDevice(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_hSubDevice(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn subDeviceId(self, fld: u32) -> Self {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_subDeviceId(&self) -> u32 {
        u32::from_le_bytes(self.store[4..8].try_into().unwrap())
    }
    pub(crate) fn set_subDeviceId(&mut self, fld: u32) {
        self.store[4..8].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn pageSize(self, fld: u64) -> Self {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_pageSize(&self) -> u64 {
        u64::from_le_bytes(self.store[8..16].try_into().unwrap())
    }
    pub(crate) fn set_pageSize(&mut self, fld: u64) {
        self.store[8..16].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn virtAddrLo(self, fld: u64) -> Self {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_virtAddrLo(&self) -> u64 {
        u64::from_le_bytes(self.store[16..24].try_into().unwrap())
    }
    pub(crate) fn set_virtAddrLo(&mut self, fld: u64) {
        self.store[16..24].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn virtAddrHi(self, fld: u64) -> Self {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_virtAddrHi(&self) -> u64 {
        u64::from_le_bytes(self.store[24..32].try_into().unwrap())
    }
    pub(crate) fn set_virtAddrHi(&mut self, fld: u64) {
        self.store[24..32].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn numLevelsToCopy(self, fld: u32) -> Self {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_numLevelsToCopy(&self) -> u32 {
        u32::from_le_bytes(self.store[32..36].try_into().unwrap())
    }
    pub(crate) fn set_numLevelsToCopy(&mut self, fld: u32) {
        self.store[32..36].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_0_physAddress(self, fld: u64) -> Self {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_0_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[40..48].try_into().unwrap())
    }
    pub(crate) fn set_levels_0_physAddress(&mut self, fld: u64) {
        self.store[40..48].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_0_size(self, fld: u64) -> Self {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_0_size(&self) -> u64 {
        u64::from_le_bytes(self.store[48..56].try_into().unwrap())
    }
    pub(crate) fn set_levels_0_size(&mut self, fld: u64) {
        self.store[48..56].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_0_aperture(self, fld: u32) -> Self {
        self.store[56..60].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_0_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[56..60].try_into().unwrap())
    }
    pub(crate) fn set_levels_0_aperture(&mut self, fld: u32) {
        self.store[56..60].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_0_pageShift(self, fld: u8) -> Self {
        self.store[60..61].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_0_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[60..61].try_into().unwrap())
    }
    pub(crate) fn set_levels_0_pageShift(&mut self, fld: u8) {
        self.store[60..61].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn levels_1_physAddress(self, fld: u64) -> Self {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_1_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[64..72].try_into().unwrap())
    }
    pub(crate) fn set_levels_1_physAddress(&mut self, fld: u64) {
        self.store[64..72].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_1_size(self, fld: u64) -> Self {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_1_size(&self) -> u64 {
        u64::from_le_bytes(self.store[72..80].try_into().unwrap())
    }
    pub(crate) fn set_levels_1_size(&mut self, fld: u64) {
        self.store[72..80].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_1_aperture(self, fld: u32) -> Self {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_1_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[80..84].try_into().unwrap())
    }
    pub(crate) fn set_levels_1_aperture(&mut self, fld: u32) {
        self.store[80..84].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_1_pageShift(self, fld: u8) -> Self {
        self.store[84..85].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_1_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[84..85].try_into().unwrap())
    }
    pub(crate) fn set_levels_1_pageShift(&mut self, fld: u8) {
        self.store[84..85].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn levels_2_physAddress(self, fld: u64) -> Self {
        self.store[88..96].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_2_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[88..96].try_into().unwrap())
    }
    pub(crate) fn set_levels_2_physAddress(&mut self, fld: u64) {
        self.store[88..96].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_2_size(self, fld: u64) -> Self {
        self.store[96..104].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_2_size(&self) -> u64 {
        u64::from_le_bytes(self.store[96..104].try_into().unwrap())
    }
    pub(crate) fn set_levels_2_size(&mut self, fld: u64) {
        self.store[96..104].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_2_aperture(self, fld: u32) -> Self {
        self.store[104..108].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_2_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[104..108].try_into().unwrap())
    }
    pub(crate) fn set_levels_2_aperture(&mut self, fld: u32) {
        self.store[104..108].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_2_pageShift(self, fld: u8) -> Self {
        self.store[108..109].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_2_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[108..109].try_into().unwrap())
    }
    pub(crate) fn set_levels_2_pageShift(&mut self, fld: u8) {
        self.store[108..109].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn levels_3_physAddress(self, fld: u64) -> Self {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_3_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[112..120].try_into().unwrap())
    }
    pub(crate) fn set_levels_3_physAddress(&mut self, fld: u64) {
        self.store[112..120].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_3_size(self, fld: u64) -> Self {
        self.store[120..128].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_3_size(&self) -> u64 {
        u64::from_le_bytes(self.store[120..128].try_into().unwrap())
    }
    pub(crate) fn set_levels_3_size(&mut self, fld: u64) {
        self.store[120..128].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_3_aperture(self, fld: u32) -> Self {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_3_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[128..132].try_into().unwrap())
    }
    pub(crate) fn set_levels_3_aperture(&mut self, fld: u32) {
        self.store[128..132].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_3_pageShift(self, fld: u8) -> Self {
        self.store[132..133].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_3_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[132..133].try_into().unwrap())
    }
    pub(crate) fn set_levels_3_pageShift(&mut self, fld: u8) {
        self.store[132..133].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn levels_4_physAddress(self, fld: u64) -> Self {
        self.store[136..144].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_4_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[136..144].try_into().unwrap())
    }
    pub(crate) fn set_levels_4_physAddress(&mut self, fld: u64) {
        self.store[136..144].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_4_size(self, fld: u64) -> Self {
        self.store[144..152].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_4_size(&self) -> u64 {
        u64::from_le_bytes(self.store[144..152].try_into().unwrap())
    }
    pub(crate) fn set_levels_4_size(&mut self, fld: u64) {
        self.store[144..152].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_4_aperture(self, fld: u32) -> Self {
        self.store[152..156].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_4_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[152..156].try_into().unwrap())
    }
    pub(crate) fn set_levels_4_aperture(&mut self, fld: u32) {
        self.store[152..156].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_4_pageShift(self, fld: u8) -> Self {
        self.store[156..157].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_4_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[156..157].try_into().unwrap())
    }
    pub(crate) fn set_levels_4_pageShift(&mut self, fld: u8) {
        self.store[156..157].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn levels_5_physAddress(self, fld: u64) -> Self {
        self.store[160..168].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_5_physAddress(&self) -> u64 {
        u64::from_le_bytes(self.store[160..168].try_into().unwrap())
    }
    pub(crate) fn set_levels_5_physAddress(&mut self, fld: u64) {
        self.store[160..168].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_5_size(self, fld: u64) -> Self {
        self.store[168..176].copy_from_slice(&u64::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_5_size(&self) -> u64 {
        u64::from_le_bytes(self.store[168..176].try_into().unwrap())
    }
    pub(crate) fn set_levels_5_size(&mut self, fld: u64) {
        self.store[168..176].copy_from_slice(&u64::to_le_bytes(fld));
    }
    pub(crate) fn levels_5_aperture(self, fld: u32) -> Self {
        self.store[176..180].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_5_aperture(&self) -> u32 {
        u32::from_le_bytes(self.store[176..180].try_into().unwrap())
    }
    pub(crate) fn set_levels_5_aperture(&mut self, fld: u32) {
        self.store[176..180].copy_from_slice(&u32::to_le_bytes(fld));
    }
    pub(crate) fn levels_5_pageShift(self, fld: u8) -> Self {
        self.store[180..181].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_levels_5_pageShift(&self) -> u8 {
        u8::from_le_bytes(self.store[180..181].try_into().unwrap())
    }
    pub(crate) fn set_levels_5_pageShift(&mut self, fld: u8) {
        self.store[180..181].copy_from_slice(&u8::to_le_bytes(fld));
    }
}

pub(crate) const NVA06F_CTRL_BIND_PARAMS_MESSAGE_ID: u32 = 0x4;
pub(crate) const NVA06F_CTRL_CMD_BIND: u32 = 0xa06f0104;
pub(crate) struct s_NVA06F_CTRL_BIND_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NVA06F_CTRL_BIND_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        4
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 4) },
        }
    }

    pub(crate) fn engineType(self, fld: u32) -> Self {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_engineType(&self) -> u32 {
        u32::from_le_bytes(self.store[0..4].try_into().unwrap())
    }
    pub(crate) fn set_engineType(&mut self, fld: u32) {
        self.store[0..4].copy_from_slice(&u32::to_le_bytes(fld));
    }
}

pub(crate) const NVA06F_CTRL_CMD_GPFIFO_SCHEDULE: u32 = 0xa06f0103;
pub(crate) const NVA06F_CTRL_GPFIFO_SCHEDULE_PARAMS_MESSAGE_ID: u32 = 0x3;
pub(crate) struct s_NVA06F_CTRL_GPFIFO_SCHEDULE_PARAMS<'s> {
    ptr: *mut u8,
    store: &'s mut [u8],
}

impl<'s> s_NVA06F_CTRL_GPFIFO_SCHEDULE_PARAMS<'s> {
    pub(crate) const fn str_size() -> usize {
        2
    }
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            store: unsafe { core::slice::from_raw_parts_mut(ptr, 2) },
        }
    }

    pub(crate) fn bEnable(self, fld: u8) -> Self {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bEnable(&self) -> u8 {
        u8::from_le_bytes(self.store[0..1].try_into().unwrap())
    }
    pub(crate) fn set_bEnable(&mut self, fld: u8) {
        self.store[0..1].copy_from_slice(&u8::to_le_bytes(fld));
    }
    pub(crate) fn bSkipSubmit(self, fld: u8) -> Self {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
        self
    }

    pub(crate) fn get_bSkipSubmit(&self) -> u8 {
        u8::from_le_bytes(self.store[1..2].try_into().unwrap())
    }
    pub(crate) fn set_bSkipSubmit(&mut self, fld: u8) {
        self.store[1..2].copy_from_slice(&u8::to_le_bytes(fld));
    }
}
