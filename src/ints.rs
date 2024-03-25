use crate::{Reader, EndOfInput};

macro_rules! impl_reader_fn {
    ($type:ident, $size:expr, $func:ident, $docname:expr) => {
        #[inline]
        #[doc="Reads a `"] #[doc=$docname] #[doc="`."]
        pub fn $func(&mut self) -> Result<$type, EndOfInput> {
            Ok($type::from_be_bytes(self.read_array::<$size>()?))
        }
    };
}

/// Functions that produce integers.
/// All functions are in big-endian byte order.
impl Reader {
    /// Reads a `u8`. Identical to [`read_byte`](Self::read_byte).
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, EndOfInput> {
        self.read_byte()
    }

    /// Reads an `i8`.
    pub fn read_i8(&mut self) -> Result<i8, EndOfInput> {
        // SAFETY: It's literally one byte, this is completely fine.
        unsafe { Ok(core::mem::transmute::<u8, i8>(self.read_u8()?)) }
    }

    impl_reader_fn!(u16, 2, read_u16, "u16");
    impl_reader_fn!(u32, 4, read_u32, "u32");
    impl_reader_fn!(u64, 8, read_u64, "u64");
    impl_reader_fn!(u128, 16, read_u128, "u128");
    impl_reader_fn!(i16, 2, read_i16, "i16");
    impl_reader_fn!(i32, 4, read_i32, "i32");
    impl_reader_fn!(i64, 8, read_i64, "i64");
    impl_reader_fn!(i128, 16, read_i128, "i128");
}