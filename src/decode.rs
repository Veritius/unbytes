use core::mem::transmute;
use crate::*;

/// A trait for decoding types.
pub trait Decode: Sized {
    /// Attempt to decode the type from the reader.
    fn decode(reader: impl AsMut<Reader>) -> Result<Self, EndOfInput>;
}

impl Decode for u8 {
    #[inline]
    fn decode(mut reader: impl AsMut<Reader>) -> Result<Self, EndOfInput> {
        reader.as_mut().read_byte()
    }
}

impl Decode for i8 {
    #[inline]
    fn decode(mut reader: impl AsMut<Reader>) -> Result<Self, EndOfInput> {
        Ok(unsafe { transmute::<u8, i8>(reader.as_mut().read_byte()?) })
    }
}

/// A trait for decoding types that may have different representations in different endians.
pub trait DecodeEndian: Sized {
    /// Decode `T` in little-endian byte order.
    fn decode_le(reader: impl AsMut<Reader>) -> Result<Self, EndOfInput>;

    /// Decode `T` in big-endian byte order.
    fn decode_be(reader: impl AsMut<Reader>) -> Result<Self, EndOfInput>;

    /// Decode `T` in native-endian byte order.
    fn decode_ne(reader: impl AsMut<Reader>) -> Result<Self, EndOfInput> {
        #[cfg(target_endian="little")]
        return Self::decode_le(reader);

        #[cfg(target_endian="big")]
        return Self::decode_bee(reader);
    }
}

macro_rules! decode_endian_impl {
    ($type:ty, $size:expr) => {
        impl DecodeEndian for $type {
            #[inline]
            fn decode_le(mut reader: impl AsMut<Reader>) -> Result<$type, EndOfInput> {
                let array = reader.as_mut().read_array::<$size>()?;
                Ok(<$type>::from_le_bytes(array))
            }

            #[inline]
            fn decode_be(mut reader: impl AsMut<Reader>) -> Result<$type, EndOfInput> {
                let array = reader.as_mut().read_array::<$size>()?;
                Ok(<$type>::from_be_bytes(array))
            }
        }
    };
}

decode_endian_impl!(u16, 2);
decode_endian_impl!(u32, 4);
decode_endian_impl!(u64, 8);
decode_endian_impl!(u128, 16);

decode_endian_impl!(i16, 2);
decode_endian_impl!(i32, 4);
decode_endian_impl!(i64, 8);
decode_endian_impl!(i128, 16);