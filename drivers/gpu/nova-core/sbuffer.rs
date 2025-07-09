// SPDX-License-Identifier: GPL-2.0

use core::ops::Deref;

use kernel::alloc::KVec;
use kernel::error::code::*;
use kernel::prelude::*;

/// A buffer abstraction for discontiguous byte slices.
///
/// This allows you to treat multiple non-contiguous `&mut [u8]` slices
/// as a single stream-like read/write buffer.
///
/// Example:
///
/// let mut buf1 = [0u8; 3];
/// let mut buf2 = [0u8; 5];
/// let mut sbuffer = SWriteBuffer::new([&buf1, &buf2]);
///
/// let data = b"hellowo";
/// let result = sbuffer.write_all(0, data);
///

/// A sliding window of slices to proceed.
///
/// Both read and write buffers are implemented in terms of operating on slices of a requested
/// size. This base class implements logic that can be shared between the two to support that.
///
/// `S` is a slice type, `I` is an iterator yielding `S`.
pub(crate) struct SBuffer<I: Iterator> {
    /// `Some` if we are not at the end of the data yet.
    cur_slice: Option<I::Item>,
    /// All the slices remaining after `cur_slice`.
    slices: I,
}

impl<'a, I> SBuffer<I>
where
    I: Iterator,
{
    pub(crate) fn new_reader(slices: impl IntoIterator<IntoIter = I>) -> Self
    where
        I: Iterator<Item = &'a [u8]>,
    {
        Self::new(slices)
    }

    pub(crate) fn new_writer(slices: impl IntoIterator<IntoIter = I>) -> Self
    where
        I: Iterator<Item = &'a mut [u8]>,
    {
        Self::new(slices)
    }

    fn new(slices: impl IntoIterator<IntoIter = I>) -> Self
    where
        I::Item: Deref<Target = [u8]>,
    {
        let mut slices = slices.into_iter();

        Self {
            // Skip empty slices to avoid trouble down the road.
            cur_slice: slices.find(|s| !s.deref().is_empty()),
            slices,
        }
    }

    fn get_slice_internal(
        &mut self,
        len: usize,
        mut f: impl FnMut(I::Item, usize) -> (I::Item, I::Item),
    ) -> Option<I::Item>
    where
        I::Item: Deref<Target = [u8]>,
    {
        match self.cur_slice.take() {
            None => None,
            Some(cur_slice) => {
                if len >= cur_slice.len() {
                    // Caller requested more data than is in the current slice, return it entirely
                    // and prepare the following slice for being used. Skip empty slices to avoid
                    // trouble.
                    self.cur_slice = self.slices.find(|s| !s.deref().is_empty());

                    Some(cur_slice)
                } else {
                    // The current slice can satisfy the request, split it and return a slice of
                    // the requested size.
                    let (ret, next) = f(cur_slice, len);
                    self.cur_slice = Some(next);

                    Some(ret)
                }
            }
        }
    }
}

/// Provides a way to get non-mutable slices of data to read from.
impl<'a, I> SBuffer<I>
where
    I: Iterator<Item = &'a [u8]>,
{
    /// Returns a slice of at most `len` bytes, or `None` if we are at the end of the data.
    ///
    /// If a slice shorter than `len` bytes has been returned, the caller can call this method
    /// again until it returns `None` to try and obtain the remainder of the data.
    fn get_slice(&mut self, len: usize) -> Option<&'a [u8]> {
        self.get_slice_internal(len, |s, pos| s.split_at(pos))
    }

    /// Ideally we would implement `Read`, but it is not available in `core`.
    /// So mimic `std::io::Read::read_exact`.
    #[expect(unused)]
    pub(crate) fn read_exact(&mut self, mut dst: &mut [u8]) -> Result {
        while !dst.is_empty() {
            match self.get_slice(dst.len()) {
                None => return Err(ETOOSMALL),
                Some(src) => {
                    let dst_slice;
                    (dst_slice, dst) = dst.split_at_mut(src.len());
                    dst_slice.copy_from_slice(src);
                }
            }
        }

        Ok(())
    }

    /// Read all the remaining data into a `KVec`.
    ///
    /// `self` will be empty after this operation.
    pub(crate) fn read_into_kvec(&mut self, flags: kernel::alloc::Flags) -> Result<KVec<u8>> {
        let mut buf = KVec::<u8>::new();

        if let Some(slice) = core::mem::take(&mut self.cur_slice) {
            buf.extend_from_slice(slice, flags)?;
        }
        for slice in &mut self.slices {
            buf.extend_from_slice(slice, flags)?;
        }

        Ok(buf)
    }
}

/// Provides a way to get mutable slices of data to write into.
impl<'a, I> SBuffer<I>
where
    I: Iterator<Item = &'a mut [u8]>,
{
    /// Returns a mutable slice of at most `len` bytes, or `None` if we are at the end of the data.
    ///
    /// If a slice shorter than `len` bytes has been returned, the caller can call this method
    /// again until it returns `None` to try and obtain the remainder of the data.
    fn get_slice_mut(&mut self, len: usize) -> Option<&'a mut [u8]> {
        self.get_slice_internal(len, |s, pos| s.split_at_mut(pos))
    }

    /// Ideally we would implement `Write`, but it is not available in `core`.
    /// So mimic `std::io::Write::write_all`.
    pub(crate) fn write_all(&mut self, mut src: &[u8]) -> Result {
        while !src.is_empty() {
            match self.get_slice_mut(src.len()) {
                None => return Err(ETOOSMALL),
                Some(dst) => {
                    let src_slice;
                    (src_slice, src) = src.split_at(dst.len());
                    dst.copy_from_slice(src_slice);
                }
            }
        }

        Ok(())
    }
}

impl<'a, I> Iterator for SBuffer<I>
where
    I: Iterator<Item = &'a [u8]>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Returned slices are guaranteed to not be empty so we can safely index the first entry.
        self.get_slice(1).map(|s| s[0])
    }
}
