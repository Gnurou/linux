// SPDX-License-Identifier: GPL-2.0

use kernel::ptr::Alignment;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_ALIGNMENT: Alignment = Alignment::new(1 << 20);
