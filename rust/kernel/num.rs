// SPDX-License-Identifier: GPL-2.0

//! Numerical and binary utilities for primitive types.

/// Useful operations for `u64`.
pub trait U64Ext {
    /// Build a `u64` by combining its `high` and `low` parts.
    ///
    /// ```
    /// use kernel::num::U64Ext;
    /// assert_eq!(u64::from_u32s(0x01234567, 0x89abcdef), 0x01234567_89abcdef);
    /// ```
    fn from_u32s(high: u32, low: u32) -> Self;

    /// Returns the upper 32 bits of `self`.
    fn upper_32_bits(self) -> u32;

    /// Returns the lower 32 bits of `self`.
    fn lower_32_bits(self) -> u32;
}

impl U64Ext for u64 {
    fn from_u32s(high: u32, low: u32) -> Self {
        ((high as u64) << u32::BITS) | low as u64
    }

    fn upper_32_bits(self) -> u32 {
        (self >> u32::BITS) as u32
    }

    fn lower_32_bits(self) -> u32 {
        self as u32
    }
}

/// Same as [`U64Ext::upper_32_bits`], but defined outside of the trait so it can be used in a
/// `const` context.
pub const fn upper_32_bits(v: u64) -> u32 {
    (v >> u32::BITS) as u32
}

/// Same as [`U64Ext::lower_32_bits`], but defined outside of the trait so it can be used in a
/// `const` context.
pub const fn lower_32_bits(v: u64) -> u32 {
    v as u32
}

/// Same as [`U64Ext::from_u32s`], but defined outside of the trait so it can be used in a `const`
/// context.
pub const fn u64_from_u32s(high: u32, low: u32) -> u64 {
    ((high as u64) << u32::BITS) | low as u64
}
