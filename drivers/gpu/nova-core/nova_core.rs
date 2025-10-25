// SPDX-License-Identifier: GPL-2.0

//! Nova Core GPU Driver

#[macro_use]
mod bitfield;

mod dma;
mod driver;
mod falcon;
mod fb;
mod firmware;
mod gfw;
mod gpu;
mod gsp;
mod num;
mod regs;
mod sbuffer;
mod vbios;

use kernel::debugfs::Dir;

pub(crate) const MODULE_NAME: &kernel::str::CStr = <LocalModule as kernel::ModuleMetadata>::NAME;

static mut DEBUGFS_ROOT: Option<Dir> = None;

kernel::module_pci_driver! {
    type: driver::NovaCore,
    init: || {
        kernel::pr_info!("Nova Core GPU driver initializing...\n");
        let dir = Dir::new(kernel::c_str!("nova_core"));
        // SAFETY: we are the only driver code running, so there cannot be any concurrent access to
        // `DEBUGFS_ROOT`.
        unsafe { DEBUGFS_ROOT = Some(dir) };
    },
    name: "NovaCore",
    authors: ["Danilo Krummrich"],
    description: "Nova Core GPU driver",
    license: "GPL v2",
    firmware: [],
}

kernel::module_firmware!(firmware::ModInfoBuilder);
