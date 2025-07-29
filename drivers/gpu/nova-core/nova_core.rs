// SPDX-License-Identifier: GPL-2.0

//! Nova Core GPU Driver

#[macro_use]
mod macros {
    // Stack-safe `FromBytes` implementation using MaybeUninit.
    // This avoids the stack overflow issues of the previous version that created
    // large temporary arrays like [u8; size_of::<Self>()] on the stack.
    macro_rules! impl_from_bytes {
        ($name:ty) => {
            impl $name {
                pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self> {
                    use core::mem::{size_of, MaybeUninit};

                    if bytes.len() < size_of::<Self>() {
                        return Err(EINVAL);
                    }

                    // STACK-SAFE: Use MaybeUninit instead of creating large arrays on stack
                    let mut result: MaybeUninit<Self> = MaybeUninit::uninit();
                    let result_ptr = result.as_mut_ptr() as *mut u8;

                    // Copy bytes directly into MaybeUninit buffer
                    // SAFETY: We're copying exactly size_of::<Self>() bytes to properly allocated memory
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            bytes.as_ptr(),
                            result_ptr,
                            size_of::<Self>(),
                        );
                    }

                    // Convert endianness in-place to avoid additional stack allocation
                    const U32_SIZE: usize = size_of::<u32>();
                    let size = size_of::<Self>();

                    // Process u32-aligned chunks for endianness conversion
                    for i in (0..size).step_by(U32_SIZE) {
                        if i + U32_SIZE <= size {
                            // SAFETY: We're within bounds and working with properly aligned memory
                            unsafe {
                                let chunk_ptr = result_ptr.add(i) as *mut u32;
                                let value = chunk_ptr.read_unaligned();
                                chunk_ptr.write_unaligned(u32::from_le(value));
                            }
                        }
                    }

                    // SAFETY: We've initialized all bytes and FromBytes guarantees any byte pattern is valid
                    unsafe { Ok(result.assume_init()) }
                }
            }
        };
    }
}

mod dma;
mod driver;
mod falcon;
mod fb;
mod firmware;
mod gfw;
mod gpu;
mod gsp;
mod nvfw;
mod regs;
mod sbuffer;
mod util;
mod vbios;

pub(crate) const MODULE_NAME: &kernel::str::CStr = <LocalModule as kernel::ModuleMetadata>::NAME;

kernel::module_pci_driver! {
    type: driver::NovaCore,
    name: "NovaCore",
    author: "Danilo Krummrich",
    description: "Nova Core GPU driver",
    license: "GPL v2",
    firmware: [],
}

kernel::module_firmware!(firmware::ModInfoBuilder);
