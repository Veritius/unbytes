#![doc=include_str!("../README.md")]
#![cfg_attr(not(feature="std"), no_std)]
#![warn(missing_docs)]

mod ints;

use core::ops::Add;
#[cfg(feature="std")]
use std::{error::Error, fmt::Display};

use bytes::*;

static EMPTY_SLICE: &[u8] = &[];

/// Panic-free forward-only cursor. See the [module level docs][crate].
pub struct Reader {
    index: usize,
    inner: Bytes,
}

impl Reader {
    /// Creates a new Reader from anything that implements `Into<Bytes>`.
    /// 
    /// This does not allocate by itself, but the `Into<Bytes>` implementation might.
    pub fn new<T: Into<Bytes>>(bytes: T) -> Self {
        Self {
            index: 0,
            inner: bytes.into(),
        }
    }

    #[inline]
    fn increment(&mut self, amt: usize) {
        self.index = self.index.add(amt).min(self.inner.len())
    }

    /// Returns how many bytes have not been read.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.inner.len().saturating_sub(self.index)
    }

    /// Returns `true` if at least `len` many bytes are unread.
    #[inline]
    pub fn has_remaining(&self, len: usize) -> bool {
        self.remaining() >= len
    }

    /// Returns how many bytes have been read so far.
    #[inline]
    pub fn consumed(&self) -> usize {
        self.index
    }

    /// Skips `amt` bytes.
    pub fn skip(&mut self, amt: usize) {
        self.increment(amt)
    }

    /// Returns `true` if there is another byte to read and it is equal to `val`.
    pub fn peek(&self, val: u8) -> bool {
        if !self.has_remaining(1) { return false; }
        self.inner[self.index + 1] == val
    }

    /// Returns the rest of the unread data, consuming the iterator.
    /// 
    /// **Special behavior:**
    /// If there are no bytes left, the inner value will be dropped,
    /// instead returning a Bytes pointing to a static, empty slice.
    pub fn read_to_end(self) -> Bytes {
        if self.index == self.inner.len() {
            return Bytes::from_static(EMPTY_SLICE)
        }

        self.inner.slice(self.index..)
    } 

    /// Returns a `Reader` that can read the next `len` bytes,
    /// advancing the original cursor by the same amount.
    pub fn subreader(&mut self, len: usize) -> Result<Self, EndOfInput> {
        if len == 0 { return Err(EndOfInput); }
        Ok(Self::new(self.read_bytes(len)?))
    }

    /// Reads a single byte. Identical to [`read_u8`](Self::read_u8).
    pub fn read_byte(&mut self) -> Result<u8, EndOfInput> {
        if !self.has_remaining(1) { return Err(EndOfInput); }
        let r = self.inner[self.index];
        self.increment(1);
        return Ok(r);
    }

    /// Returns the next `len` bytes as a [`Bytes`], advancing the cursor.
    pub fn read_bytes(&mut self, len: usize) -> Result<Bytes, EndOfInput> {
        if !self.has_remaining(len) { return Err(EndOfInput); }
        let old_idx = self.index;
        self.increment(len);
        Ok(self.inner.slice(old_idx..old_idx+len))
    }

    /// Returns the next `len` bytes as a slice, advancing the cursor.
    /// The returned slice will always be of length `len`.
    pub fn read_slice(&mut self, len: usize) -> Result<&[u8], EndOfInput> {
        if !self.has_remaining(len) { return Err(EndOfInput); }
        let old_idx = self.index;
        self.increment(len);
        Ok(&self.inner[old_idx..old_idx+len])
    }

    /// Returns an array of size `N`, advancing the cursor.
    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N], EndOfInput> {
        let slice = self.read_slice(N)?;
        let mut array = [0u8; N];
        array.copy_from_slice(slice);
        Ok(array)
    }
}

#[cfg(feature="std")]
impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = self.remaining().min(buf.len());
        if amt == 0 { return Ok(0) }
        buf[..amt].copy_from_slice(self.read_slice(amt).unwrap());
        Ok(0)
    }
}

impl From<Bytes> for Reader {
    #[inline]
    fn from(value: Bytes) -> Self {
        Self {
            index: 0,
            inner: value,
        }
    }
}

/// Error returned when the end of the cursor is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EndOfInput;

#[cfg(feature="std")]
impl Display for EndOfInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("end of input")
    }
}

#[cfg(feature="std")]
impl Error for EndOfInput {}

#[test]
fn static_slice_test() {
    let slice: &'static [u8; 20] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let bytes = Bytes::from_static(slice);

    let mut reader = Reader::new(bytes.clone());
    assert_eq!(slice, &*reader.read_bytes(20).unwrap());

    let mut reader = Reader::new(bytes.clone());
    assert_eq!(slice, reader.read_slice(20).unwrap());

    let mut reader = Reader::new(bytes.clone());
    assert_eq!(slice, &reader.read_array::<20>().unwrap());

    let mut reader = Reader::new(bytes.clone());
    assert_eq!(&[1,2,3,4,5], &*reader.read_bytes(5).unwrap());
    assert_eq!(&[6,7,8,9,10], reader.read_slice(5).unwrap());
    assert_eq!(&[11,12,13,14,15], &reader.read_array::<5>().unwrap());
    assert_eq!(16, reader.read_u8().unwrap());

    assert_eq!(reader.consumed(), 16);
    assert_eq!(reader.remaining(), 4);
    assert!(reader.has_remaining(4));
}