// SPDX-License-Identifier: GPL-2.0

//! Direct VRAM access through PRAMIN window before page tables are set up.
//!
//! # Usage Examples
//!
//! ## Writing u32 data to VRAM
//!
//! ```rust
//! use crate::driver::Bar0;
//! use crate::mm::vram::PraminVram;
//!
//! fn write_data_to_vram(bar: &Bar0) -> Result {
//!     let pramin = PraminVram::new(bar);
//!     // Write to VRAM at offset 0x10000
//!     let data: [u32; 4] = [0xDEADBEEF, 0xCAFEBABE, 0x12345678, 0x87654321];
//!     pramin.write::<u32>(0x10000, &data)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Reading bytes from VRAM
//!
//! ```rust
//! use crate::driver::Bar0;
//! use crate::mm::vram::PraminVram;
//!
//! fn read_data_from_vram(bar: &Bar0, buffer: &mut KVec<u8>) -> Result {
//!     let pramin = PraminVram::new(bar);
//!     // Read from VRAM starting at offset 0x20000
//!     pramin.read::<u8>(0x20000, buffer)?;
//!     Ok(())
//! }
//! ```

use crate::driver::Bar0;
use crate::regs;
use core::mem;
use kernel::prelude::*;

/// PRAMIN is a window into the VRAM (not a hardware block) that is used to access
/// the VRAM directly. These addresses are consistent across all GPUs.
#[allow(dead_code)]
const PRAMIN_BASE: usize = 0x700000; // PRAMIN is always at BAR0 + 0x700000
#[allow(dead_code)]
const PRAMIN_SIZE: usize = 0x100000; // 1MB aperture - max access per window position

/// Trait for types that can be read/written through PRAMIN
#[allow(dead_code)]
pub(crate) trait PraminNum: Copy + Default + Sized {
    fn read_from_bar(bar: &Bar0, offset: usize) -> Result<Self>;

    fn write_to_bar(self, bar: &Bar0, offset: usize) -> Result;

    fn size_bytes() -> usize {
        mem::size_of::<Self>()
    }

    fn alignment() -> usize {
        Self::size_bytes()
    }
}

impl PraminNum for u8 {
    fn read_from_bar(bar: &Bar0, offset: usize) -> Result<Self> {
        bar.try_read8(offset)
    }

    fn write_to_bar(self, bar: &Bar0, offset: usize) -> Result {
        bar.try_write8(self, offset)
    }
}

impl PraminNum for u16 {
    fn read_from_bar(bar: &Bar0, offset: usize) -> Result<Self> {
        bar.try_read16(offset)
    }

    fn write_to_bar(self, bar: &Bar0, offset: usize) -> Result {
        bar.try_write16(self, offset)
    }
}

impl PraminNum for u32 {
    fn read_from_bar(bar: &Bar0, offset: usize) -> Result<Self> {
        bar.try_read32(offset)
    }

    fn write_to_bar(self, bar: &Bar0, offset: usize) -> Result {
        bar.try_write32(self, offset)
    }
}

impl PraminNum for u64 {
    fn read_from_bar(bar: &Bar0, offset: usize) -> Result<Self> {
        bar.try_read64(offset)
    }

    fn write_to_bar(self, bar: &Bar0, offset: usize) -> Result {
        bar.try_write64(self, offset)
    }
}

/// Direct VRAM access through PRAMIN window before page tables are set up
#[allow(dead_code)]
pub(crate) struct PraminVram<'a> {
    bar: &'a Bar0,
    saved_window: usize,
}

#[allow(dead_code)]
impl<'a> PraminVram<'a> {
    /// Create a new PRAMIN VRAM accessor, saving current window state,
    /// the state is restored when the accessor is dropped.
    ///
    /// The BAR0 window base must be 64KB aligned but provides 1MB of VRAM access.
    /// Window is repositioned automatically when accessing data beyond 1MB boundaries.
    pub(crate) fn new(bar: &'a Bar0) -> Self {
        let saved_window = Self::get_window(bar);
        Self { bar, saved_window }
    }

    /// Set BAR0 window to point to specific FB region
    fn set_window(&self, fb_offset: usize) -> Result {
        // FB offset must be 64KB aligned (hardware requirement for window_base field)
        // Once positioned, the window provides access to 1MB of VRAM through PRAMIN aperture
        if fb_offset & 0xFFFF != 0 {
            return Err(EINVAL);
        }

        // Calculate window base (bits 39:16 of FB address)
        let window_base = ((fb_offset >> 16) & 0xFFFFFF) as u32;

        // Prepare window register value with TARGET = 0 (VID_MEM)
        let window_reg = regs::NV_PBUS_BAR0_WINDOW::default()
            .set_window_base(window_base)
            .set_target(0);

        window_reg.write(self.bar);

        // Read back to ensure it took effect
        let readback = regs::NV_PBUS_BAR0_WINDOW::read(self.bar);
        if readback.window_base() != window_base {
            return Err(EIO);
        }

        Ok(())
    }

    /// Get current BAR0 window offset
    fn get_window(bar: &Bar0) -> usize {
        let window_reg = regs::NV_PBUS_BAR0_WINDOW::read(bar);
        (window_reg.window_base() as usize) << 16
    }

    /// Common logic for accessing VRAM data through PRAMIN with windowing
    ///
    fn access_vram<T: PraminNum, F>(
        &self,
        fb_offset: usize,
        num_items: usize,
        mut operation: F,
    ) -> Result
    where
        F: FnMut(usize, usize) -> Result,
    {
        // FB offset must be aligned to the size of T
        if fb_offset & (T::alignment() - 1) != 0 {
            return Err(EINVAL);
        }

        let mut offset_bytes = fb_offset;
        let mut remaining_items = num_items;
        let mut data_index = 0;

        while remaining_items > 0 {
            // Align the window to 64KB boundary
            let target_window = offset_bytes & !0xFFFF;
            let window_offset = offset_bytes - target_window;

            // Set window if needed
            if target_window != Self::get_window(self.bar) {
                self.set_window(target_window)?;
            }

            // Calculate how many items we can access from this window position
            // We can access up to 1MB total, minus the offset within the window
            let remaining_in_window = PRAMIN_SIZE - window_offset;
            let max_items_in_window = remaining_in_window / T::size_bytes();
            let items_to_write = core::cmp::min(remaining_items, max_items_in_window);

            // Process data through PRAMIN
            for i in 0..items_to_write {
                // Calculate the byte offset in the PRAMIN window to write to.
                let pramin_offset_bytes =
                    PRAMIN_BASE + window_offset + (i * T::size_bytes());
                operation(data_index + i, pramin_offset_bytes)?;
            }

            // Move to next chunk
            data_index += items_to_write;
            offset_bytes += items_to_write * T::size_bytes();
            remaining_items -= items_to_write;
        }

        Ok(())
    }

    /// Generic write for data to VRAM through PRAMIN
    pub(crate) fn write<T: PraminNum>(&self, fb_offset: usize, data: &[T]) -> Result {
        self.access_vram::<T, _>(fb_offset, data.len(), |data_idx, pramin_offset| {
            data[data_idx].write_to_bar(self.bar, pramin_offset)
        })
    }

    /// Generic read data from VRAM through PRAMIN
    pub(crate) fn read<T: PraminNum>(&self, fb_offset: usize, data: &mut [T]) -> Result {
        self.access_vram::<T, _>(fb_offset, data.len(), |data_idx, pramin_offset| {
            data[data_idx] = T::read_from_bar(self.bar, pramin_offset)?;
            Ok(())
        })
    }
}

impl<'a> Drop for PraminVram<'a> {
    fn drop(&mut self) {
        let _ = self.set_window(self.saved_window); // Restore original window
    }
}
