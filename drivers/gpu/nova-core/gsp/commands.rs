// SPDX-License-Identifier: GPL-2.0

use kernel::build_assert;
use kernel::device;
use kernel::pci;
use kernel::prelude::*;
use kernel::time::Delta;
use kernel::transmute::{AsBytes, FromBytes};

use crate::driver::Bar0;
use crate::gsp::cmdq::{GspCommandToGsp, GspMessageFromGsp};
use crate::gsp::GspCmdq;
use crate::gsp::GSP_PAGE_SIZE;
use crate::gsp::fw::{
    GspStaticConfigInfo_t, GspSystemInfo, NV_VGPU_MSG_EVENT_GSP_INIT_DONE,
    NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO, NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO,
    NV_VGPU_MSG_FUNCTION_SET_REGISTRY, PACKED_REGISTRY_ENTRY, PACKED_REGISTRY_TABLE,
    REGISTRY_TABLE_ENTRY_TYPE_DWORD,
};
use crate::sbuffer::SBuffer;

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for GspSystemInfo {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for GspSystemInfo {}

unsafe impl FromBytes for GspStaticConfigInfo_t {}
unsafe impl AsBytes for GspStaticConfigInfo_t {}

pub(crate) struct GspStaticConfigInfo {
    pub gpu_name: [u8; 40],
}

struct GspInitDone {}
impl GspMessageFromGsp for GspInitDone {
    const FUNCTION: u32 = NV_VGPU_MSG_EVENT_GSP_INIT_DONE;
}

pub(crate) fn gsp_init_done(cmdq: &mut GspCmdq, timeout: Delta) -> Result {
    loop {
        cmdq.wait_for_msg_from_gsp(timeout)?;
        let msg = loop {
            match cmdq.receive_msg_from_gsp() {
                Ok(x) => break Ok(x),
                Err(EAGAIN) => continue,
                Err(x) => break Err(x),
            };
        }?;

        let init_done = msg.try_as::<GspInitDone>().map(|_| ());

        msg.ack()?;

        match init_done {
            Ok(()) => break Ok(()),
            Err(ERANGE) => continue,
            Err(e) => break Err(e),
        };
    }
}

impl GspMessageFromGsp for GspStaticConfigInfo_t {
    const FUNCTION: u32 = NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO;
}

impl GspCommandToGsp for GspStaticConfigInfo_t {
    const FUNCTION: u32 = NV_VGPU_MSG_FUNCTION_GET_GSP_STATIC_INFO;
}

pub(crate) fn get_gsp_info(cmdq: &mut GspCmdq, bar: &Bar0) -> Result<GspStaticConfigInfo> {
    cmdq.send_gsp_command::<GspStaticConfigInfo_t>(
        bar,
        size_of::<GspStaticConfigInfo_t>(),
        |_, _| Ok(()),
    )?;

    cmdq.wait_for_msg_from_gsp(Delta::from_secs(5))?;
    let msg = cmdq.receive_msg_from_gsp()?;
    let info = msg.try_as::<GspStaticConfigInfo_t>().map(|(x, _)| x)?;

    let gpu_name_str = info
        .gpuNameString
        .get(
            0..=info
                .gpuNameString
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(info.gpuNameString.len() - 1),
        )
        .and_then(|bytes| CStr::from_bytes_with_nul(bytes).ok())
        .and_then(|cstr| cstr.to_str().ok())
        .unwrap_or("invalid utf8");

    let mut gpu_name = [0u8; 40];
    let bytes = gpu_name_str.as_bytes();
    let copy_len = core::cmp::min(bytes.len(), gpu_name.len());
    gpu_name[..copy_len].copy_from_slice(&bytes[..copy_len]);
    gpu_name[copy_len] = b'\0';

    let config_info = GspStaticConfigInfo { gpu_name };

    msg.ack()?;
    Ok(config_info)
}

const GSP_REGISTRY_NUM_ENTRIES: usize = 2;
struct RegistryEntry {
    key: &'static str,
    value: u32,
}

struct RegistryTable {
    entries: [RegistryEntry; GSP_REGISTRY_NUM_ENTRIES],
}

struct GspRegistryTable;

unsafe impl FromBytes for GspRegistryTable {}
unsafe impl AsBytes for GspRegistryTable {}

impl GspCommandToGsp for GspRegistryTable {
    const FUNCTION: u32 = NV_VGPU_MSG_FUNCTION_SET_REGISTRY;
}

unsafe impl AsBytes for PACKED_REGISTRY_TABLE {}
unsafe impl AsBytes for PACKED_REGISTRY_ENTRY {}

impl RegistryTable {
    fn write_into_sbuffer<'a, I: Iterator<Item = &'a mut [u8]>>(
        &self,
        mut sbuffer: SBuffer<I>,
    ) -> Result {
        sbuffer.write_all(
            PACKED_REGISTRY_TABLE {
                numEntries: GSP_REGISTRY_NUM_ENTRIES as u32,
                size: self.size() as u32,
                entries: Default::default(),
            }
            .as_bytes(),
        )?;

        let string_data_start_offset = size_of::<PACKED_REGISTRY_TABLE>()
            + GSP_REGISTRY_NUM_ENTRIES * size_of::<PACKED_REGISTRY_ENTRY>();

        // Array for string data.
        let mut string_data = KVec::new();

        for entry in self.entries.iter().take(GSP_REGISTRY_NUM_ENTRIES) {
            sbuffer.write_all(
                PACKED_REGISTRY_ENTRY {
                    nameOffset: (string_data_start_offset + string_data.len()) as u32,
                    type_: REGISTRY_TABLE_ENTRY_TYPE_DWORD as u8,
                    __bindgen_padding_0: Default::default(),
                    data: entry.value,
                    length: 0,
                }
                .as_bytes(),
            )?;

            let key_bytes = entry.key.as_bytes();
            string_data.extend_from_slice(key_bytes, GFP_KERNEL)?;
            string_data.push(0, GFP_KERNEL)?;
        }

        sbuffer.write_all(string_data.as_slice())
    }

    fn size(&self) -> usize {
        let mut key_size = 0;
        for i in 0..GSP_REGISTRY_NUM_ENTRIES {
            key_size += self.entries[i].key.len() + 1; // +1 for NULL terminator
        }
        size_of::<PACKED_REGISTRY_TABLE>()
            + GSP_REGISTRY_NUM_ENTRIES * size_of::<PACKED_REGISTRY_ENTRY>()
            + key_size
    }
}

pub(crate) fn build_registry(cmdq: &mut GspCmdq, bar: &Bar0) -> Result {
    let registry = RegistryTable {
        entries: [
            RegistryEntry {
                key: "RMSecBusResetEnable",
                value: 1,
            },
            RegistryEntry {
                key: "RMForcePcieConfigSave",
                value: 1,
            },
        ],
    };

    cmdq.send_gsp_command::<GspRegistryTable>(bar, registry.size(), |_, sbuffer| {
        registry.write_into_sbuffer(sbuffer)
    })
}

impl GspCommandToGsp for GspSystemInfo {
    const FUNCTION: u32 = NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO;
}

pub(crate) fn set_system_info(
    cmdq: &mut GspCmdq,
    dev: &pci::Device<device::Bound>,
    bar: &Bar0,
) -> Result {
    build_assert!(size_of::<GspSystemInfo>() < GSP_PAGE_SIZE);
    cmdq.send_gsp_command::<GspSystemInfo>(bar, size_of::<GspSystemInfo>(), |info, _| {
        info.gpuPhysAddr = dev.resource_start(0)?;
        info.gpuPhysFbAddr = dev.resource_start(1)?;
        info.gpuPhysInstAddr = dev.resource_start(3)?;
        info.nvDomainBusDeviceFunc = u64::from(dev.dev_id());

        // Using TASK_SIZE in r535_gsp_rpc_set_system_info() seems wrong because
        // TASK_SIZE is per-task. That's probably a design issue in GSP-RM though.
        info.maxUserVa = (1 << 47) - 4096;
        info.pciConfigMirrorBase = 0x088000;
        info.pciConfigMirrorSize = 0x001000;

        info.PCIDeviceID = (u32::from(dev.device_id()) << 16) | u32::from(dev.vendor_id().as_raw());
        info.PCISubDeviceID =
            (u32::from(dev.subsystem_device_id()) << 16) | u32::from(dev.subsystem_vendor_id());
        info.PCIRevisionID = u32::from(dev.revision_id());
        info.bIsPrimary = 0;
        info.bPreserveVideoMemoryAllocations = 0;

        Ok(())
    })?;

    Ok(())
}
