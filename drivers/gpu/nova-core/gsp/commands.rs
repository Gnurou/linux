// SPDX-License-Identifier: GPL-2.0

use core::alloc::Layout;

use kernel::alloc::allocator::Kmalloc;
use kernel::alloc::Allocator;
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
use crate::nvfw::r570_144 as fw;
use crate::sbuffer::SBuffer;

// SAFETY: These structs don't meet the no-padding requirements of AsBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl AsBytes for fw::GspSystemInfo {}

// SAFETY: These structs don't meet the no-padding requirements of FromBytes but
//         that is not a problem because they are not used outside the kernel.
unsafe impl FromBytes for fw::GspSystemInfo {}

struct GspInitDone {}
impl GspMessageFromGsp for GspInitDone {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_EVENT_GSP_INIT_DONE;
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

const GSP_REGISTRY_NUM_ENTRIES: usize = 2;
struct RegistryEntry {
    key: &'static str,
    value: u32,
}

struct RegistryTable {
    entries: [RegistryEntry; GSP_REGISTRY_NUM_ENTRIES],
}

struct GspRegistryTable;
impl GspCommandToGsp for GspRegistryTable {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_SET_REGISTRY;
}

impl RegistryTable {
    fn serialize_registry_table(&self) -> Result<KVec<u8>> {
        let entries = &self.entries;
        let total_size = self.size();
        let align = core::mem::align_of::<fw::PACKED_REGISTRY_TABLE>();
        let layout = Layout::from_size_align(total_size, align).map_err(|_| ENOMEM)?;
        debug_assert_eq!(layout.size(), total_size);
        let mut string_data_offset = size_of::<fw::PACKED_REGISTRY_TABLE>()
            + GSP_REGISTRY_NUM_ENTRIES * size_of::<fw::PACKED_REGISTRY_ENTRY>();
        let allocation = Kmalloc::alloc(layout, GFP_KERNEL)?;
        let ptr = allocation.as_ptr().cast::<u8>();

        // We allocate the memory for the vector ourselves to ensure it has the
        // correct layout to cast to a fw::PACKED_REGISTRY_TABLE and subsequent
        // fw:PACKED_REGISTRY_ENTRIES.
        //
        // SAFETY:
        //  - ptr was allocated with Kmalloc as required for KVec.
        //  - ptr trivally meets the alignment requirements for u8.
        //  - No elements have been initialised so this is zero length.
        //  - The capacity matches the total size of the allocation.
        let mut table_vec = unsafe { KVec::<u8>::from_raw_parts(ptr, 0, layout.size()) };
        let table_slice = table_vec.spare_capacity_mut();
        let table = table_slice.as_mut_ptr().cast::<fw::PACKED_REGISTRY_TABLE>();

        // SAFETY: We ensured the alignment was correct when allocating the vector.
        unsafe {
            // Set the table header
            (*table).numEntries = GSP_REGISTRY_NUM_ENTRIES as u32;
            (*table).size = total_size as u32;
        }

        for (i, entry) in entries.iter().enumerate().take(GSP_REGISTRY_NUM_ENTRIES) {
            // SAFETY: The allocation meets the alignment requirements for
            // fw::PACKED_REGISTRY_TABLE which includes a zero length array for the entries.
            unsafe {
                let entry_ptr = table_slice
                    .as_mut_ptr()
                    .add(
                        size_of::<fw::PACKED_REGISTRY_TABLE>()
                            + i * size_of::<fw::PACKED_REGISTRY_ENTRY>(),
                    )
                    .cast::<fw::PACKED_REGISTRY_ENTRY>();

                // Set entry metadata
                (*entry_ptr).nameOffset = string_data_offset as u32;
                (*entry_ptr).type_ = fw::REGISTRY_TABLE_ENTRY_TYPE_DWORD as u8;
                (*entry_ptr).data = entry.value;
                (*entry_ptr).length = 0;
            }

            let key_bytes = entry.key.as_bytes();
            let string_dest_slice =
                &mut table_slice[string_data_offset..string_data_offset + key_bytes.len() + 1];

            // Can't use copy_from_slice() because string_dest_slice is MaybeUninit<u8>.
            for (i, &byte) in key_bytes.iter().enumerate() {
                string_dest_slice[i].write(byte);
            }

            // Add null terminator
            string_dest_slice[key_bytes.len()].write(0);

            // Update offset for next string
            string_data_offset += string_dest_slice.len();
        }

        debug_assert_eq!(total_size, string_data_offset);

        // SAFETY: All data has been written to as asserted above and the
        // capacity matches the original allocation.
        unsafe { table_vec.inc_len(layout.size()) };

        Ok(table_vec)
    }

    fn copy_to_sbuf_iter(&self, mut sbuf: SBuffer<core::array::IntoIter<&mut [u8], 2>>) -> Result {
        let table_vec = self.serialize_registry_table()?;
        sbuf.write_all(&table_vec)?;
        Ok(())
    }

    fn size(&self) -> usize {
        let mut key_size = 0;
        for i in 0..GSP_REGISTRY_NUM_ENTRIES {
            key_size += self.entries[i].key.len() + 1; // +1 for NULL terminator
        }
        size_of::<fw::PACKED_REGISTRY_TABLE>()
            + GSP_REGISTRY_NUM_ENTRIES * size_of::<fw::PACKED_REGISTRY_ENTRY>()
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
    let mut msg = cmdq.alloc_gsp_queue_command(registry.size())?;
    {
        let (_, some_sbuf) = msg.try_as::<GspRegistryTable>();
        let sbuf = some_sbuf.ok_or(ENOMEM)?;
        registry.copy_to_sbuf_iter(sbuf)?;
    }
    msg.send_to_gsp(bar)?;

    Ok(())
}

impl GspCommandToGsp for fw::GspSystemInfo {
    const FUNCTION: u32 = fw::NV_VGPU_MSG_FUNCTION_GSP_SET_SYSTEM_INFO;
}

pub(crate) fn set_system_info(
    cmdq: &mut GspCmdq,
    dev: &pci::Device<device::Bound>,
    bar: &Bar0,
) -> Result {
    build_assert!(size_of::<fw::GspSystemInfo>() < GSP_PAGE_SIZE);
    let mut msg = cmdq.alloc_gsp_queue_command(size_of::<fw::GspSystemInfo>())?;
    {
        let (info, _) = msg.try_as::<fw::GspSystemInfo>();

        info.gpuPhysAddr = dev.resource_start(0)?;
        info.gpuPhysFbAddr = dev.resource_start(1)?;
        info.gpuPhysInstAddr = dev.resource_start(3)?;
        info.nvDomainBusDeviceFunc = u64::from(dev.dev_id());

        // Using TASK_SIZE in r535_gsp_rpc_set_system_info() seems wrong because
        // TASK_SIZE is per-task. That's probably a design issue in GSP-RM though.
        info.maxUserVa = (1 << 47) - 4096;
        info.pciConfigMirrorBase = 0x088000;
        info.pciConfigMirrorSize = 0x001000;

        info.PCIDeviceID = (u32::from(dev.device_id()) << 16) | u32::from(dev.vendor_id());
        info.PCISubDeviceID =
            (u32::from(dev.subsystem_device_id()) << 16) | u32::from(dev.subsystem_vendor_id());
        info.PCIRevisionID = u32::from(dev.revision_id());
        info.bIsPrimary = 0;
        info.bPreserveVideoMemoryAllocations = 0;
    }
    msg.send_to_gsp(bar)?;
    Ok(())
}
