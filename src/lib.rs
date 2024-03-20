//! Ergonomic, panic-free forward-only cursors, based on `bytes`.

#![cfg_attr(not(feature="std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature="std")]
use std::{error::Error, fmt::Display};

use bytes::*;

static EMPTY_SLICE: &[u8] = &[];

/// Panic-free forward-only cursor. Will never panic.
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
    pub fn consumed(&self) -> usize {
        self.index
    }

    /// Skips `amt` bytes.
    pub fn skip(&mut self, amt: usize) {
        self.index = (self.index + amt).max(self.inner.len())
    }

    /// Returns `true` 
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

    /// Returns the next `len` bytes as a [`Bytes`], advancing the cursor.
    pub fn read_bytes(&mut self, len: usize) -> Result<Bytes, EndOfInput> {
        if !self.has_remaining(len) { return Err(EndOfInput); }
        Ok(self.inner.slice(self.index..self.index+len))
    }

    /// Returns the next `len` bytes as a slice, advancing the cursor.
    pub fn read_slice(&mut self, len: usize) -> Result<&[u8], EndOfInput> {
        if !self.has_remaining(len) { return Err(EndOfInput); }
        Ok(&self.inner[self.index..self.index+len])
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