// SPDX-License-Identifier: GPL-2.0

use kernel::prelude::*;

use crate::driver::Bar0;
use crate::fb::hal::{FbHal, GspFwHeapParams};
use crate::nvfw;
use crate::regs;

fn vidmem_size_ga102(bar: &Bar0) -> u64 {
    regs::NV_USABLE_FB_SIZE_IN_MB::read(bar).usable_fb_size()
}

fn heap_params_ga102() -> GspFwHeapParams {
    GspFwHeapParams {
        libos: &nvfw::LIBOS3_PARAMS,
        ..super::tu102::heap_params_tu102()
    }
}

struct Ga102;

impl FbHal for Ga102 {
    fn read_sysmem_flush_page(&self, bar: &Bar0) -> u64 {
        super::ga100::read_sysmem_flush_page_ga100(bar)
    }

    fn write_sysmem_flush_page(&self, bar: &Bar0, addr: u64) -> Result {
        super::ga100::write_sysmem_flush_page_ga100(bar, addr);

        Ok(())
    }

    fn supports_display(&self, bar: &Bar0) -> bool {
        super::ga100::display_enabled_ga100(bar)
    }

    fn vidmem_size(&self, bar: &Bar0) -> u64 {
        vidmem_size_ga102(bar)
    }

    fn heap_params(&self) -> GspFwHeapParams {
        heap_params_ga102()
    }
}

const GA102: Ga102 = Ga102;
pub(super) const GA102_HAL: &dyn FbHal = &GA102;
