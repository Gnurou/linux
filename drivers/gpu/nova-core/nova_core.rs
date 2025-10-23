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

use core::pin::Pin;
use kernel::debugfs::Dir;

pub(crate) const MODULE_NAME: &kernel::str::CStr = <LocalModule as kernel::ModuleMetadata>::NAME;

kernel::sync::global_lock! {
    // FIXME: Cursor came up with this.
    // SAFETY: I have no idea
    pub unsafe(uninit) static DEBUGFS_ROOT: Mutex<Option<Dir>> = None;
}

kernel::module_pci_driver! {
    type: driver::NovaCore,
    init: || {
        kernel::pr_info!("Nova Core GPU driver initializing...\n");
        // FIXME: Cursor came up with this.  It doesn't delete the /nova_core/ directory
        // on module load.
        // SAFETY: This isn't safe
        unsafe { DEBUGFS_ROOT.init() };
        let dir = Dir::new(kernel::c_str!("nova_core"));
        *DEBUGFS_ROOT.lock() = Some(dir);
    },
    name: "NovaCore",
    authors: ["Danilo Krummrich"],
    description: "Nova Core GPU driver",
    license: "GPL v2",
    firmware: [],
}

kernel::module_firmware!(firmware::ModInfoBuilder);
