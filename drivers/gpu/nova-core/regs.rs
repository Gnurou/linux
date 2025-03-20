// SPDX-License-Identifier: GPL-2.0

use core::ops::Deref;
use kernel::io::Io;
use kernel::register;

use crate::gpu::Chipset;

register!(Boot0@0x00000000, "Basic revision information about the GPU";
    3:0     minor_rev => as u8, "minor revision of the chip";
    7:4     major_rev => as u8, "major revision of the chip";
    28:20   chipset => try_into Chipset, "chipset model"
);
