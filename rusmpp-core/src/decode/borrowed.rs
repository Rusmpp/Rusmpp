//! Traits for decoding `SMPP` values with borrowed data.

use crate::{
    Sealed,
    decode::{DecodeError, HeaplessVecDecodeError},
};

/// Trait for decoding `SMPP` values from a slice.
pub trait Decode<'a>: 'a + Sized + Sealed {
    /// Decode a value from a slice.
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a slice with a specified length.
pub trait DecodeWithLength<'a>: 'a + Sized + Sealed {
    /// Decode a value from a slice, with a specified length
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Everything that implements [`Decode`] also implements [`DecodeWithLength`] by ignoring the length.
impl<'a, T: Decode<'a>> DecodeWithLength<'a> for T {
    fn decode(src: &'a [u8], _length: usize) -> Result<(Self, usize), DecodeError> {
        Decode::decode(src)
    }
}

/// Trait for decoding `SMPP` values from a slice with a specified key and length.
pub trait DecodeWithKey<'a>: 'a + Sized + Sealed {
    type Key;

    /// Decode a value from a slice, using a key to determine the type.
    fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding optional `SMPP` values from a slice with a specified key and length.
pub trait DecodeWithKeyOptional<'a>: 'a + Sized + Sealed {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

pub(crate) trait DecodeExt<'a>: Decode<'a> {
    fn decode_move(src: &'a [u8], size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..]).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a vector of values from a slice with a specified count.
    fn counted<const N: usize>(
        src: &'a [u8],
        count: usize,
    ) -> Result<(heapless::vec::Vec<Self, N>, usize), DecodeError> {
        (0..count).try_fold((heapless::vec::Vec::new(), 0), |(mut vec, size), _| {
            let (item, size_) = Self::decode(&src[size..])?;

            vec.push(item).map_err(|_| {
                DecodeError::heapless_vec_decode_error(HeaplessVecDecodeError::TooManyItems {
                    max: N,
                })
            })?;

            Ok((vec, size + size_))
        })
    }

    fn counted_move<const N: usize>(
        src: &'a [u8],
        count: usize,
        size: usize,
    ) -> Result<(heapless::vec::Vec<Self, N>, usize), DecodeError> {
        Self::counted(&src[size..], count).map(|(vec, size_)| (vec, size + size_))
    }

    /// Decode a value from a slice.
    ///
    /// If the length is 0, return `None`.
    fn length_checked_decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::length_checked_decode(&src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<'a, T: Decode<'a>> DecodeExt<'a> for T {}

pub(crate) trait DecodeWithLengthExt<'a>: DecodeWithLength<'a> {
    fn decode_move(
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<'a, T: DecodeWithLength<'a>> DecodeWithLengthExt<'a> for T {}

pub(crate) trait DecodeWithKeyExt<'a>: DecodeWithKey<'a> {
    /// Decode a value from a slice, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::optional_length_checked_decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }

    /// Decode a value from a slice, using a key to determine the type ignoring the length.
    fn no_length_decode_move(
        key: Self::Key,
        src: &'a [u8],
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(key, &src[size..], 0).map(|(this, size_)| (this, size + size_))
    }
}

impl<'a, T: DecodeWithKey<'a>> DecodeWithKeyExt<'a> for T {}

pub(crate) trait DecodeWithKeyOptionalExt<'a>: DecodeWithKeyOptional<'a> {
    fn decode_move(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<'a, T: DecodeWithKeyOptional<'a>> DecodeWithKeyOptionalExt<'a> for T {}

impl<'a, const N: usize, T: Decode<'a>> DecodeWithLength<'a> for heapless::vec::Vec<T, N> {
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
        if length == 0 {
            return Ok((heapless::vec::Vec::new(), 0));
        }

        if length > src.len() {
            return Err(DecodeError::heapless_vec_decode_error(
                HeaplessVecDecodeError::UnexpectedEndOfBuffer,
            ));
        }

        let mut size = 0;

        let mut vec = heapless::vec::Vec::new();

        while size < length {
            let (item, size_) = T::decode(&src[size..length])?;

            size += size_;

            vec.push(item).map_err(|_| {
                DecodeError::heapless_vec_decode_error(HeaplessVecDecodeError::TooManyItems {
                    max: N,
                })
            })?;
        }

        Ok((vec, size))
    }
}

#[cfg(test)]
mod tests {
    use heapless::vec::Vec;

    use crate::{
        decode::{COctetStringDecodeError, DecodeErrorKind, IntegerDecodeError},
        types::borrowed::{COctetString, EmptyOrFullCOctetString},
    };

    use super::*;

    const N: usize = 32;

    /// Testing [`counted_move`](DecodeExt::counted_move) will automatically test [`counted`](DecodeExt::counted).
    #[test]
    fn counted_move() {
        // Count is 0
        let buf = &[0, 1, 2];

        let (values, size) = u8::counted_move::<N>(buf, 0, 0).unwrap();

        assert_eq!(size, 0);
        assert_eq!(&buf[size..], &[0, 1, 2]);
        assert_eq!(values, Vec::<u8, N>::new());

        // Count is more than the buffer
        let buf = &[0, 1, 2];

        let error = u8::counted_move::<N>(buf, 5, 0).unwrap_err();
        assert!(matches!(
            error.kind,
            DecodeErrorKind::IntegerDecodeError(IntegerDecodeError::UnexpectedEndOfBuffer)
        ));

        // Count is within the buffer
        let buf = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let (values, size) = u8::counted_move::<N>(buf, 10, 0).unwrap();

        assert_eq!(size, 10);
        assert!(&buf[size..].is_empty());
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let buf = &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9];

        let (values, size) = u16::counted_move::<N>(buf, 10, 0).unwrap();

        assert_eq!(size, 20);
        assert!(&buf[size..].is_empty());
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let buf = &[
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0,
            0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9,
        ];

        // Actually 10 values, 12 will break
        let error = u32::counted_move::<N>(buf, 12, 0).unwrap_err();

        assert!(matches!(
            error.kind,
            DecodeErrorKind::IntegerDecodeError(IntegerDecodeError::UnexpectedEndOfBuffer)
        ));

        let buf = b"Hello\0World\0";

        let (values, size) = COctetString::<1, 6>::counted_move::<N>(buf, 2, 0).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[size..].is_empty());
        assert_eq!(
            values,
            Vec::<_, N>::from([
                COctetString::<'static, 1, 6>::new(b"Hello\0").unwrap(),
                COctetString::<'static, 1, 6>::new(b"World\0").unwrap(),
            ])
        );

        let buf = b"Hello\0World\0";

        let (values, size) =
            EmptyOrFullCOctetString::<'static, 6>::counted_move::<N>(buf, 2, 0).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[size..].is_empty());
        assert_eq!(
            values,
            Vec::<_, N>::from([
                EmptyOrFullCOctetString::<'static, 6>::new(b"Hello\0").unwrap(),
                EmptyOrFullCOctetString::<'static, 6>::new(b"World\0").unwrap(),
            ])
        );

        let buf = b"Hello\0World\0Hi";

        let error = COctetString::<'static, 1, 6>::counted_move::<N>(buf, 3, 0).unwrap_err();

        assert!(matches!(
            error.kind,
            DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
        ));

        // Remaining bytes
        let buf = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let (values, size) = u8::counted_move::<N>(buf, 5, 0).unwrap();

        assert_eq!(size, 5);
        assert_eq!(&buf[size..], &[5, 6, 7, 8, 9]);
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4]));

        let buf = &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9];

        let (values, size) = u16::counted_move::<N>(buf, 5, 0).unwrap();

        assert_eq!(size, 10);
        assert_eq!(&buf[size..], &[0, 5, 0, 6, 0, 7, 0, 8, 0, 9]);
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4]));
    }

    #[test]
    fn decode_with_length_vec() {
        // Length is 0
        let buf = &[0, 1, 2];

        let (values, size) = Vec::<u8, N>::decode(buf, 0).unwrap();

        assert_eq!(size, 0);
        assert_eq!(&buf[size..], &[0, 1, 2]);
        assert_eq!(values, Vec::<u8, N>::new());

        // Length is bigger than the buffer
        let buf = &[0, 1, 2];

        let error = Vec::<u8, N>::decode(buf, 5).unwrap_err();

        assert!(matches!(
            error.kind,
            DecodeErrorKind::HeaplessVecDecodeError(HeaplessVecDecodeError::UnexpectedEndOfBuffer)
        ));

        // Length is within the buffer
        let buf = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let (values, size) = Vec::<u8, N>::decode(buf, 10).unwrap();

        assert_eq!(size, 10);
        assert!(&buf[size..].is_empty());
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let buf = &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9];

        let (values, size) = Vec::<u16, N>::decode(buf, 20).unwrap();

        assert_eq!(size, 20);
        assert!(&buf[size..].is_empty());
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let buf = &[
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0,
            0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9,
        ];

        // Actually 40 bytes, 50 will break
        let error = Vec::<u32, N>::decode(buf, 50).unwrap_err();

        assert!(matches!(
            error.kind,
            DecodeErrorKind::HeaplessVecDecodeError(HeaplessVecDecodeError::UnexpectedEndOfBuffer)
        ));

        let buf = b"Hello\0World\0";

        let (values, size) = Vec::<COctetString<1, 6>, N>::decode(buf, 12).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[size..].is_empty());
        assert_eq!(
            values,
            heapless::Vec::<_, N>::from([
                COctetString::<1, 6>::new(b"Hello\0").unwrap(),
                COctetString::<1, 6>::new(b"World\0").unwrap(),
            ])
        );

        let buf = b"Hello\0World\0";

        let (values, size) = Vec::<EmptyOrFullCOctetString<6>, N>::decode(buf, 12).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[size..].is_empty());
        assert_eq!(
            values,
            heapless::Vec::<_, N>::from([
                EmptyOrFullCOctetString::<6>::new(b"Hello\0").unwrap(),
                EmptyOrFullCOctetString::<6>::new(b"World\0").unwrap(),
            ])
        );

        let buf = b"Hello\0World\0Hi";

        // This will try to decode 11 bytes b"Hello\0World"
        let error = Vec::<COctetString<1, 6>, N>::decode(buf, 11).unwrap_err();

        assert!(matches!(
            error.kind,
            DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
        ));

        // Remaining bytes
        let buf = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let (values, size) = Vec::<u8, N>::decode(buf, 5).unwrap();

        assert_eq!(size, 5);
        assert_eq!(&buf[size..], &[5, 6, 7, 8, 9]);
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4]));

        let buf = &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9];

        let (values, size) = Vec::<u16, N>::decode(buf, 10).unwrap();

        assert_eq!(size, 10);
        assert_eq!(&buf[size..], &[0, 5, 0, 6, 0, 7, 0, 8, 0, 9]);
        assert_eq!(values, Vec::<_, N>::from([0, 1, 2, 3, 4]));
    }
}
