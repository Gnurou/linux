// SPDX-License-Identifier: GPL-2.0

//! Nova Core GPU Driver

#[macro_use]
mod macros {
    /// Convenience macro to run a closure while holding [`crate::driver::Bar0`].
    ///
    /// If the bar cannot be acquired, then `ENXIO` is returned.
    macro_rules! with_bar {
        ($bar:expr, $closure:expr) => {
            $bar.try_access_with($closure).ok_or(ENXIO)
        };
    }
}

mod driver;
mod falcon;
mod firmware;
mod gpu;
mod regs;
mod timer;
mod util;

kernel::module_pci_driver! {
    type: driver::NovaCore,
    name: "NovaCore",
    author: "Danilo Krummrich",
    description: "Nova Core GPU driver",
    license: "GPL v2",
    firmware: [],
}

kernel::module_firmware!(firmware::ModInfoBuilder);
