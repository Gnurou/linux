// SPDX-License-Identifier: GPL-2.0

//! Direct VRAM access through the PRAMIN aperture.
//!
//! PRAMIN provides a 1MB sliding window into VRAM through BAR0, allowing the CPU to access
//! video memory directly. Access is managed through a two-level API:
//!
//! - [`Pramin`]: The parent object that owns the BAR0 reference and synchronization lock.
//! - [`PraminWindow`]: A guard object that holds exclusive PRAMIN access for its lifetime.
//!
//! The PRAMIN aperture is a 1MB region at BAR0 + 0x700000 for all GPUs. The window base is
//! controlled by the `NV_PBUS_BAR0_WINDOW` register and must be 64KB aligned.
//!
//! # Examples
//!
//! ## Basic read/write
//!
//! ```no_run
//! use crate::driver::Bar0;
//! use crate::mm::pramin;
//! use kernel::devres::Devres;
//! use kernel::prelude::*;
//! use kernel::sync::Arc;
//!
//! fn example(devres_bar: Arc<Devres<Bar0>>) -> Result<()> {
//!     let pramin = Arc::pin_init(pramin::Pramin::new(devres_bar)?, GFP_KERNEL)?;
//!     let mut window = pramin.window()?;
//!
//!     // Write and read back.
//!     window.try_write32(0x100, 0xDEADBEEF)?;
//!     let val = window.try_read32(0x100)?;
//!     assert_eq!(val, 0xDEADBEEF);
//!
//!     Ok(())
//!     // Original window position restored on drop.
//! }
//! ```
//!
//! ## Auto-repositioning across VRAM regions
//!
//! ```no_run
//! use crate::driver::Bar0;
//! use crate::mm::pramin;
//! use kernel::devres::Devres;
//! use kernel::prelude::*;
//! use kernel::sync::Arc;
//!
//! fn example(devres_bar: Arc<Devres<Bar0>>) -> Result<()> {
//!     let pramin = Arc::pin_init(pramin::Pramin::new(devres_bar)?, GFP_KERNEL)?;
//!     let mut window = pramin.window()?;
//!
//!     // Access first 1MB region.
//!     window.try_write32(0x100, 0x11111111)?;
//!
//!     // Access at 2MB - window auto-repositions.
//!     window.try_write32(0x200000, 0x22222222)?;
//!
//!     // Back to first region - window repositions again.
//!     let val = window.try_read32(0x100)?;
//!     assert_eq!(val, 0x11111111);
//!
//!     Ok(())
//! }
//! ```

#![allow(unused)]

use crate::{
    driver::Bar0,
    regs, //
};

use kernel::bits::genmask_u64;
use kernel::devres::Devres;
use kernel::io::Io;
use kernel::new_mutex;
use kernel::prelude::*;
use kernel::ptr::{
    Alignable,
    Alignment, //
};
use kernel::sizes::{
    SZ_1M,
    SZ_64K, //
};
use kernel::sync::{
    lock::mutex::MutexGuard,
    Arc,
    Mutex, //
};

/// PRAMIN aperture base offset in BAR0.
const PRAMIN_BASE: usize = 0x700000;

/// PRAMIN aperture size (1MB).
const PRAMIN_SIZE: usize = SZ_1M;

/// 64KB alignment for window base.
const WINDOW_ALIGN: Alignment = Alignment::new::<SZ_64K>();

/// Maximum addressable VRAM offset (40-bit address space).
///
/// The `NV_PBUS_BAR0_WINDOW` register has a 24-bit `window_base` field (bits 23:0) that stores
/// bits [39:16] of the target VRAM address. This limits the addressable space to 2^40 bytes.
///
/// CAST: On 64-bit systems, this fits in usize.
const MAX_VRAM_OFFSET: usize = genmask_u64(0..=39) as usize;

/// Generate a PRAMIN read accessor.
macro_rules! define_pramin_read {
    ($name:ident, $ty:ty) => {
        #[doc = concat!("Read a `", stringify!($ty), "` from VRAM at the given offset.")]
        pub(crate) fn $name(&mut self, vram_offset: usize) -> Result<$ty> {
            // Compute window parameters without bar reference.
            let (bar_offset, new_base) =
                self.compute_window(vram_offset, ::core::mem::size_of::<$ty>())?;

            // Update window base if needed and perform read.
            let bar = self.bar.try_access().ok_or(ENODEV)?;
            if let Some(base) = new_base {
                Self::write_window_base(&bar, base);
                self.state.current_base = base;
            }
            bar.$name(bar_offset)
        }
    };
}

/// Generate a PRAMIN write accessor.
macro_rules! define_pramin_write {
    ($name:ident, $ty:ty) => {
        #[doc = concat!("Write a `", stringify!($ty), "` to VRAM at the given offset.")]
        pub(crate) fn $name(&mut self, vram_offset: usize, value: $ty) -> Result {
            // Compute window parameters without bar reference.
            let (bar_offset, new_base) =
                self.compute_window(vram_offset, ::core::mem::size_of::<$ty>())?;

            // Update window base if needed and perform write.
            let bar = self.bar.try_access().ok_or(ENODEV)?;
            if let Some(base) = new_base {
                Self::write_window_base(&bar, base);
                self.state.current_base = base;
            }
            bar.$name(value, bar_offset)
        }
    };
}

/// PRAMIN state protected by mutex.
struct PraminState {
    current_base: usize,
}

/// PRAMIN aperture manager.
///
/// Call [`Pramin::window()`] to acquire exclusive PRAMIN access.
#[pin_data]
pub(crate) struct Pramin {
    bar: Arc<Devres<Bar0>>,
    /// PRAMIN aperture state, protected by a mutex.
    ///
    /// # Safety
    ///
    /// This lock is acquired during the DMA fence signalling critical path.
    /// It must NEVER be held across any reclaimable CPU memory / allocations
    /// (`GFP_KERNEL`), because the memory reclaim path can call
    /// `dma_fence_wait()`, which would deadlock with this lock held.
    #[pin]
    state: Mutex<PraminState>,
}

impl Pramin {
    /// Create a pin-initializer for PRAMIN.
    pub(crate) fn new(bar: Arc<Devres<Bar0>>) -> Result<impl PinInit<Self>> {
        let bar_access = bar.try_access().ok_or(ENODEV)?;
        let current_base = Self::try_read_window_base(&bar_access)?;

        Ok(pin_init!(Self {
            bar,
            state <- new_mutex!(PraminState { current_base }, "pramin_state"),
        }))
    }

    /// Acquire exclusive PRAMIN access.
    ///
    /// Returns a [`PraminWindow`] guard that provides VRAM read/write accessors.
    /// The [`PraminWindow`] is exclusive and only one can exist at a time.
    pub(crate) fn window(&self) -> Result<PraminWindow<'_>> {
        let state = self.state.lock();
        let saved_base = state.current_base;
        Ok(PraminWindow {
            bar: self.bar.clone(),
            state,
            saved_base,
        })
    }

    /// Read the current window base from the BAR0_WINDOW register.
    fn try_read_window_base(bar: &Bar0) -> Result<usize> {
        let reg = regs::NV_PBUS_BAR0_WINDOW::read(bar);
        let base = u64::from(reg.window_base());
        let shifted = base.checked_shl(16).ok_or(EOVERFLOW)?;
        shifted.try_into().map_err(|_| EOVERFLOW)
    }
}

/// PRAMIN window guard for direct VRAM access.
///
/// This guard holds exclusive access to the PRAMIN aperture. The window auto-repositions
/// when accessing VRAM offsets outside the current 1MB range. Original window position
/// is saved on creation and restored on drop.
///
/// Only one [`PraminWindow`] can exist at a time per [`Pramin`] instance (enforced by the
/// internal `MutexGuard`).
pub(crate) struct PraminWindow<'a> {
    bar: Arc<Devres<Bar0>>,
    state: MutexGuard<'a, PraminState>,
    saved_base: usize,
}

impl PraminWindow<'_> {
    /// Write a new window base to the BAR0_WINDOW register.
    fn write_window_base(bar: &Bar0, base: usize) {
        // CAST:
        // - We have guaranteed that the base is within the addressable range (40-bits).
        // - After >> 16, a 40-bit aligned base becomes 24 bits, which fits in u32.
        regs::NV_PBUS_BAR0_WINDOW::default()
            .set_window_base((base >> 16) as u32)
            .write(bar);
    }

    /// Compute window parameters for a VRAM access.
    ///
    /// Returns (`bar_offset`, `new_base`) where:
    /// - `bar_offset`: The BAR0 offset to use for the access.
    /// - `new_base`: `Some(base)` if window needs repositioning, `None` otherwise.
    fn compute_window(
        &self,
        vram_offset: usize,
        access_size: usize,
    ) -> Result<(usize, Option<usize>)> {
        // Validate VRAM offset is within addressable range (40-bit address space).
        let end_offset = vram_offset.checked_add(access_size).ok_or(EINVAL)?;
        if end_offset > MAX_VRAM_OFFSET + 1 {
            return Err(EINVAL);
        }

        // Calculate which 64KB-aligned base we need.
        let needed_base = vram_offset.align_down(WINDOW_ALIGN);

        // Calculate offset within the window.
        let offset_in_window = vram_offset - needed_base;

        // Check if access fits in 1MB window from this base.
        if offset_in_window + access_size > PRAMIN_SIZE {
            return Err(EINVAL);
        }

        // Return bar offset and whether window needs repositioning.
        let new_base = if self.state.current_base != needed_base {
            Some(needed_base)
        } else {
            None
        };

        Ok((PRAMIN_BASE + offset_in_window, new_base))
    }

    define_pramin_read!(try_read8, u8);
    define_pramin_read!(try_read16, u16);
    define_pramin_read!(try_read32, u32);
    define_pramin_read!(try_read64, u64);

    define_pramin_write!(try_write8, u8);
    define_pramin_write!(try_write16, u16);
    define_pramin_write!(try_write32, u32);
    define_pramin_write!(try_write64, u64);
}

impl Drop for PraminWindow<'_> {
    fn drop(&mut self) {
        // Restore the original window base if it changed.
        if self.state.current_base != self.saved_base {
            if let Some(bar) = self.bar.try_access() {
                Self::write_window_base(&bar, self.saved_base);

                // Update state to reflect the restored base.
                self.state.current_base = self.saved_base;
            }
        }
        // MutexGuard drops automatically, releasing the lock.
    }
}

/// Run PRAMIN self-tests during boot if self-tests are enabled.
#[cfg(CONFIG_NOVA_MM_SELFTESTS)]
pub(crate) fn run_self_test(
    dev: &kernel::device::Device,
    bar: Arc<Devres<Bar0>>,
    mmu_version: super::pagetable::MmuVersion,
) -> Result {
    use super::pagetable::MmuVersion;

    // PRAMIN support is only for MMU v2 for now (Turing/Ampere/Ada).
    if mmu_version != MmuVersion::V2 {
        dev_info!(
            dev,
            "PRAMIN: Skipping self-tests for MMU {:?} (only V2 supported)\n",
            mmu_version
        );
        return Ok(());
    }

    dev_info!(dev, "PRAMIN: Starting self-test...\n");

    let pramin = Arc::pin_init(Pramin::new(bar)?, GFP_KERNEL)?;
    let mut win = pramin.window()?;

    // Use offset 0x1000 as test area.
    let base: usize = 0x1000;

    // Test 1: Read/write at byte-aligned locations.
    for i in 0u8..4 {
        let offset = base + 1 + usize::from(i); // Offsets 0x1001, 0x1002, 0x1003, 0x1004
        let val = 0xA0 + i;
        win.try_write8(offset, val)?;
        let read_val = win.try_read8(offset)?;
        if read_val != val {
            dev_err!(
                dev,
                "PRAMIN: FAIL - offset {:#x}: wrote {:#x}, read {:#x}\n",
                offset,
                val,
                read_val
            );
            return Err(EIO);
        }
    }

    // Test 2: Write `u32` and read back as `u8`s.
    let test2_offset = base + 0x10;
    let test2_val: u32 = 0xDEADBEEF;
    win.try_write32(test2_offset, test2_val)?;

    // Read back as individual bytes (little-endian: EF BE AD DE).
    let expected_bytes: [u8; 4] = [0xEF, 0xBE, 0xAD, 0xDE];
    for (i, &expected) in expected_bytes.iter().enumerate() {
        let read_val = win.try_read8(test2_offset + i)?;
        if read_val != expected {
            dev_err!(
                dev,
                "PRAMIN: FAIL - offset {:#x}: expected {:#x}, read {:#x}\n",
                test2_offset + i,
                expected,
                read_val
            );
            return Err(EIO);
        }
    }

    // Test 3: Window repositioning across 1MB boundaries.
    // Write to offset > 1MB to trigger window slide, then verify.
    let test3_offset_a: usize = base; // First 1MB region.
    let test3_offset_b: usize = 0x200000 + base; // 2MB + base (different 1MB region).
    let val_a: u32 = 0x11111111;
    let val_b: u32 = 0x22222222;

    // Write to first region.
    win.try_write32(test3_offset_a, val_a)?;

    // Write to second region (triggers window reposition).
    win.try_write32(test3_offset_b, val_b)?;

    // Read back from second region.
    let read_b = win.try_read32(test3_offset_b)?;
    if read_b != val_b {
        dev_err!(
            dev,
            "PRAMIN: FAIL - offset {:#x}: expected {:#x}, read {:#x}\n",
            test3_offset_b,
            val_b,
            read_b
        );
        return Err(EIO);
    }

    // Read back from first region (triggers window reposition again).
    let read_a = win.try_read32(test3_offset_a)?;
    if read_a != val_a {
        dev_err!(
            dev,
            "PRAMIN: FAIL - offset {:#x}: expected {:#x}, read {:#x}\n",
            test3_offset_a,
            val_a,
            read_a
        );
        return Err(EIO);
    }

    // Test 4: Invalid offset rejection (beyond 40-bit address space).
    {
        // 40-bit address space limit check.
        let invalid_offset: usize = MAX_VRAM_OFFSET + 1;
        let result = win.try_read32(invalid_offset);
        if result.is_ok() {
            dev_err!(
                dev,
                "PRAMIN: FAIL - read at invalid offset {:#x} should have failed\n",
                invalid_offset
            );
            return Err(EIO);
        }
    }

    // Test 5: Misaligned multi-byte access rejection.
    // Verify that misaligned `u16`/`u32`/`u64` accesses are properly rejected.
    {
        // `u16` at odd offset (not 2-byte aligned).
        let offset_u16 = base + 0x21;
        if win.try_write16(offset_u16, 0xABCD).is_ok() {
            dev_err!(
                dev,
                "PRAMIN: FAIL - misaligned u16 write at {:#x} should have failed\n",
                offset_u16
            );
            return Err(EIO);
        }

        // `u32` at 2-byte-aligned (not 4-byte-aligned) offset.
        let offset_u32 = base + 0x32;
        if win.try_write32(offset_u32, 0x12345678).is_ok() {
            dev_err!(
                dev,
                "PRAMIN: FAIL - misaligned u32 write at {:#x} should have failed\n",
                offset_u32
            );
            return Err(EIO);
        }

        // `u64` read at 4-byte-aligned (not 8-byte-aligned) offset.
        let offset_u64 = base + 0x44;
        if win.try_read64(offset_u64).is_ok() {
            dev_err!(
                dev,
                "PRAMIN: FAIL - misaligned u64 read at {:#x} should have failed\n",
                offset_u64
            );
            return Err(EIO);
        }
    }

    dev_info!(dev, "PRAMIN: All self-tests PASSED\n");
    Ok(())
}
