// SPDX-License-Identifier: GPL-2.0

//! Traits for transmuting types.

/// Types for which any bit pattern is valid.
///
/// Not all types are valid for all values. For example, a `bool` must be either zero or one, so
/// reading arbitrary bytes into something that contains a `bool` is not okay.
///
/// It's okay for the type to have padding, as initializing those bytes has no effect.
///
/// # Examples
///
/// ```
/// use kernel::transmute::FromBytes;
///
/// let foo = [1, 2, 3, 4];
///
/// let result = u32::from_bytes(&foo).unwrap();
///
/// #[cfg(target_endian = "little")]
/// assert_eq!(*result, 0x4030201);
///
/// #[cfg(target_endian = "big")]
/// assert_eq!(*result, 0x1020304);
/// ```
///
/// # Safety
///
/// All bit-patterns must be valid for this type. This type must not have interior mutability.
pub unsafe trait FromBytes {
    /// Converts a slice of bytes to a reference to `Self` when possible.
    fn from_bytes(bytes: &[u8]) -> Option<&Self>;

    /// Converts a mutable slice of bytes to a reference to `Self` when possible.
    fn from_mut_bytes(bytes: &mut [u8]) -> Option<&mut Self>
    where
        Self: AsBytes;
}

/// Just a proxy trait for FromBytes, if you need an implementation for your type use this instead.
///
/// # Safety
///
/// All bit-patterns must be valid for this type. This type must not have interior mutability.
pub unsafe trait FromBytesSized: Sized {}

macro_rules! impl_frombytessized {
    ($($({$($generics:tt)*})? $t:ty, )*) => {
        // SAFETY: Safety comments written in the macro invocation.
        $(unsafe impl$($($generics)*)? FromBytesSized for $t {})*
    };
}

impl_frombytessized! {
    // SAFETY: All bit patterns are acceptable values of the types below.
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,

    // SAFETY: If all bit patterns are acceptable for individual values in an array, then all bit
    // patterns are also acceptable for arrays of that type.
    {<T: FromBytesSized, const N: usize>} [T; N],
}

// SAFETY: All bit patterns are acceptable values of the types and in array case if all bit patterns
// are acceptable for individual values in an array, then all bit patterns are also acceptable
// for arrays of that type.
unsafe impl<T> FromBytes for T
where
    T: FromBytesSized,
{
    fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        let slice_ptr = bytes.as_ptr().cast::<T>();
        if bytes.len() == ::core::mem::size_of::<T>() && slice_ptr.is_aligned() {
            // SAFETY: Since the code checks the size and alignment, the slice is valid.
            unsafe { Some(&*slice_ptr) }
        } else {
            None
        }
    }

    fn from_mut_bytes(bytes: &mut [u8]) -> Option<&mut Self>
    where
        Self: AsBytes,
    {
        let slice_ptr = bytes.as_mut_ptr().cast::<T>();
        if bytes.len() == ::core::mem::size_of::<T>() && slice_ptr.is_aligned() {
            // SAFETY: Since the code checks the size and alignment, the slice is valid.
            unsafe { Some(&mut *slice_ptr) }
        } else {
            None
        }
    }
}

// SAFETY: If all bit patterns are acceptable for individual values in an array, then all bit
// patterns are also acceptable for arrays of that type.
unsafe impl<T: FromBytes> FromBytes for [T] {
    fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        let slice_ptr = bytes.as_ptr().cast::<T>();
        if bytes.len() % ::core::mem::size_of::<T>() == 0 && slice_ptr.is_aligned() {
            // SAFETY: Since the code checks the size and alignment, the slice is valid.
            unsafe { Some(::core::slice::from_raw_parts(slice_ptr, bytes.len())) }
        } else {
            None
        }
    }

    fn from_mut_bytes(bytes: &mut [u8]) -> Option<&mut Self>
    where
        Self: AsBytes,
    {
        let slice_ptr = bytes.as_mut_ptr().cast::<T>();
        if bytes.len() % ::core::mem::size_of::<T>() == 0 && slice_ptr.is_aligned() {
            // SAFETY: Since the code checks the size and alignment, the slice is valid.
            unsafe { Some(::core::slice::from_raw_parts_mut(slice_ptr, bytes.len())) }
        } else {
            None
        }
    }
}

/// Types that can be viewed as an immutable slice of initialized bytes.
///
/// If a struct implements this trait, then it is okay to copy it byte-for-byte to userspace. This
/// means that it should not have any padding, as padding bytes are uninitialized. Reading
/// uninitialized memory is not just undefined behavior, it may even lead to leaking sensitive
/// information on the stack to userspace.
///
/// The struct should also not hold kernel pointers, as kernel pointer addresses are also considered
/// sensitive. However, leaking kernel pointers is not considered undefined behavior by Rust, so
/// this is a correctness requirement, but not a safety requirement.
///
/// # Safety
///
/// Values of this type may not contain any uninitialized bytes. This type must not have interior
/// mutability.
pub unsafe trait AsBytes {
    /// Returns `self` as a slice of bytes.
    fn as_bytes(&self) -> &[u8] {
        let data = core::ptr::from_ref(self).cast::<u8>();
        let len = size_of_val(self);

        // SAFETY: `data` is non-null and valid for reads over `len * sizeof::<u8>()` bytes.
        unsafe { core::slice::from_raw_parts(data, len) }
    }

    /// Returns `self` as a mutable slice of bytes.
    fn as_bytes_mut(&mut self) -> &mut [u8]
    where
        Self: FromBytes,
    {
        let data = core::ptr::from_mut(self).cast::<u8>();
        let len = size_of_val(self);

        // SAFETY: `data` is non-null and valid for read and writes over `len * sizeof::<u8>()`
        // bytes. Since `Self` implements `FromBytes` it can be represented by any value.
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }
}

macro_rules! impl_asbytes {
    ($($({$($generics:tt)*})? $t:ty, )*) => {
        // SAFETY: Safety comments written in the macro invocation.
        $(unsafe impl$($($generics)*)? AsBytes for $t {})*
    };
}

impl_asbytes! {
    // SAFETY: Instances of the following types have no uninitialized portions.
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    bool,
    char,
    str,

    // SAFETY: If individual values in an array have no uninitialized portions, then the array
    // itself does not have any uninitialized portions either.
    {<T: AsBytes>} [T],
    {<T: AsBytes, const N: usize>} [T; N],
}
