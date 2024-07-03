use core::ops::{Deref, DerefMut};
use crate::*;

impl Reader {
    /// Returns a [`ReaderMayPanic`].
    /// This crate's no-panic guarantee is forfeited if this function is used.
    #[inline]
    pub fn may_panic(&mut self) -> ReaderMayPanic {
        ReaderMayPanic(self)
    }
}

/// A usage of a [`Reader`] that can panic, but exposes further APIs as a result.
pub struct ReaderMayPanic<'a>(&'a mut Reader);

impl AsMut<Reader> for ReaderMayPanic<'_> {
    fn as_mut(&mut self) -> &mut Reader {
        &mut self.0
    }
}

impl<'a> Deref for ReaderMayPanic<'a> {
    type Target = Reader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ReaderMayPanic<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a> Buf for ReaderMayPanic<'a> {
    #[inline]
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn chunk(&self) -> &[u8] {
        &self.inner[self.index..]
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.0.increment(cnt)
    }

    #[inline]
    fn copy_to_bytes(&mut self, len: usize) -> bytes::Bytes {
        self.read_bytes(len).unwrap()
    }
}