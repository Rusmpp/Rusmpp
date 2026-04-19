//! Traits for decoding `SMPP` values with owned data.

use bytes::BytesMut;

use crate::Sealed;

use super::error::VecDecodeError;

/// Trait for defining the error type for all decoding traits.
///
/// - [`Decode`]
/// - [`DecodeWithLength`]
/// - [`DecodeWithKey`]
/// - [`DecodeWithKeyOptional`]
pub trait DecodeErrorType: Sealed {
    /// The error type for decoding.
    type Error;
}

impl<T> DecodeErrorType for Option<T>
where
    T: DecodeErrorType,
{
    type Error = T::Error;
}

impl<T> DecodeErrorType for alloc::vec::Vec<T>
where
    T: DecodeErrorType,
{
    type Error = VecDecodeError<T::Error>;
}

/// Trait for decoding `SMPP` values from a buffer.
pub trait Decode: DecodeErrorType + Sized + Sealed {
    /// Decode a value from a buffer.
    fn decode(src: &mut BytesMut) -> Result<(Self, usize), Self::Error>;
}

/// Trait for decoding `SMPP` values from a buffer with a specified length.
pub trait DecodeWithLength: DecodeErrorType + Sized + Sealed {
    /// Decode a value from a buffer, with a specified length
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), Self::Error>;
}

/// Everything that implements [`Decode`] also implements [`DecodeWithLength`] by ignoring the length.
impl<T: Decode> DecodeWithLength for T {
    fn decode(src: &mut BytesMut, _length: usize) -> Result<(Self, usize), Self::Error> {
        Decode::decode(src)
    }
}

/// Trait for decoding `SMPP` values from a buffer with a specified key and length.
pub trait DecodeWithKey: DecodeErrorType + Sized + Sealed {
    type Key;

    /// Decode a value from a buffer, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error>;
}

/// Trait for decoding optional `SMPP` values from a buffer with a specified key and length.
pub trait DecodeWithKeyOptional: DecodeErrorType + Sized + Sealed {
    type Key;

    /// Decode an optional value from a buffer, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error>;
}

pub(crate) trait DecodeExt: Decode {
    fn decode_move(src: &mut BytesMut, size: usize) -> Result<(Self, usize), Self::Error> {
        Self::decode(src).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a vector of values from a buffer with a specified count.
    fn counted(
        src: &mut BytesMut,
        count: usize,
    ) -> Result<(alloc::vec::Vec<Self>, usize), Self::Error> {
        (0..count).try_fold(
            (alloc::vec::Vec::with_capacity(count), 0),
            |(mut vec, size), _| {
                Self::decode(src).map(|(item, size_)| {
                    vec.push(item);

                    (vec, size + size_)
                })
            },
        )
    }

    fn counted_move(
        src: &mut BytesMut,
        count: usize,
        size: usize,
    ) -> Result<(alloc::vec::Vec<Self>, usize), Self::Error> {
        Self::counted(src, count).map(|(vec, size_)| (vec, size + size_))
    }

    /// Decode a value from a buffer.
    ///
    /// If the length is 0, return `None`.
    fn length_checked_decode(
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::length_checked_decode(src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: Decode> DecodeExt for T {}

pub(crate) trait DecodeWithLengthExt: DecodeWithLength {
    fn decode_move(
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), Self::Error> {
        Self::decode(src, length).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithLength> DecodeWithLengthExt for T {}

pub(crate) trait DecodeWithKeyExt: DecodeWithKey {
    /// Decode a value from a buffer, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::optional_length_checked_decode(key, src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }

    /// Decode a value from a slice, using a key to determine the type ignoring the length.
    fn no_length_decode_move(
        key: Self::Key,
        src: &mut BytesMut,
        size: usize,
    ) -> Result<(Self, usize), Self::Error> {
        Self::decode(key, src, 0).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithKey> DecodeWithKeyExt for T {}

pub(crate) trait DecodeWithKeyOptionalExt: DecodeWithKeyOptional {
    fn decode_move(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::decode(key, src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKeyOptional> DecodeWithKeyOptionalExt for T {}

impl<T: Decode> DecodeWithLength for alloc::vec::Vec<T> {
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), Self::Error> {
        if length == 0 {
            return Ok((alloc::vec::Vec::new(), 0));
        }

        if length > src.len() {
            return Err(VecDecodeError::UnexpectedEndOfBuffer);
        }

        let mut src = src.split_to(length);

        let mut size = 0;

        let mut vec = alloc::vec::Vec::new();

        while size < length {
            let (item, size_) = T::decode(&mut src)?;

            size += size_;

            vec.push(item);
        }

        Ok((vec, size))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::{
        decode::{COctetStringDecodeError, IntegerDecodeError},
        types::owned::{COctetString, EmptyOrFullCOctetString},
    };

    use super::*;

    /// Testing [`counted_move`](DecodeExt::counted_move) will automatically test [`counted`](DecodeExt::counted).
    #[test]
    fn counted() {
        // Count is 0
        let mut buf = BytesMut::from(&[0, 1, 2][..]);

        let (values, size) = u8::counted(&mut buf, 0).unwrap();

        assert_eq!(size, 0);
        assert_eq!(values.len(), 0);
        assert_eq!(&buf[..], &[0, 1, 2]);
        assert_eq!(values, Vec::<u8>::new());

        // Count is more than the buffer
        let mut buf = BytesMut::from(&[0, 1, 2][..]);

        let error = u8::counted(&mut buf, 5).unwrap_err();
        assert!(matches!(error, IntegerDecodeError::UnexpectedEndOfBuffer));

        // Count is within the buffer
        let mut buf = BytesMut::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);

        let (values, size) = u8::counted(&mut buf, 10).unwrap();

        assert_eq!(size, 10);
        assert_eq!(values.len(), 10);
        assert!(buf.is_empty());
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut buf =
            BytesMut::from(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9][..]);

        let (values, size) = u16::counted(&mut buf, 10).unwrap();

        assert_eq!(size, 20);
        assert_eq!(values.len(), 10);
        assert!(buf.is_empty());
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut buf = BytesMut::from(
            &[
                0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6,
                0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9,
            ][..],
        );

        // Actually 10 values, 12 will break
        let error = u32::counted(&mut buf, 12).unwrap_err();

        assert!(matches!(error, IntegerDecodeError::UnexpectedEndOfBuffer));

        let mut buf = BytesMut::from(&b"Hello\0World\0"[..]);

        let (values, size) = COctetString::<1, 6>::counted_move(&mut buf, 2, 0).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[..].is_empty());
        assert_eq!(
            values,
            alloc::vec![
                COctetString::<1, 6>::from_static_slice(b"Hello\0").unwrap(),
                COctetString::<1, 6>::from_static_slice(b"World\0").unwrap(),
            ]
        );

        let mut buf = BytesMut::from(&b"Hello\0World\0"[..]);

        let (values, size) = EmptyOrFullCOctetString::<6>::counted_move(&mut buf, 2, 0).unwrap();

        assert_eq!(size, 12);
        assert!(&buf[..].is_empty());
        assert_eq!(
            values,
            alloc::vec![
                EmptyOrFullCOctetString::<6>::from_static_slice(b"Hello\0").unwrap(),
                EmptyOrFullCOctetString::<6>::from_static_slice(b"World\0").unwrap(),
            ]
        );

        let mut buf = BytesMut::from(&b"Hello\0World\0Hi"[..]);

        let error = COctetString::<1, 6>::counted_move(&mut buf, 3, 0).unwrap_err();

        assert!(matches!(error, COctetStringDecodeError::NotNullTerminated));

        // Remaining bytes
        let mut buf = BytesMut::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);

        let (values, size) = u8::counted_move(&mut buf, 5, 0).unwrap();

        assert_eq!(size, 5);
        assert_eq!(&buf[..], &[5, 6, 7, 8, 9]);
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4]);

        let mut buf =
            BytesMut::from(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9][..]);

        let (values, size) = u16::counted_move(&mut buf, 5, 0).unwrap();

        assert_eq!(size, 10);
        assert_eq!(&buf[..], &[0, 5, 0, 6, 0, 7, 0, 8, 0, 9]);
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn decode_with_length_vec() {
        // Length is 0
        let mut buf = BytesMut::from(&[0, 1, 2][..]);

        let (values, size) = Vec::<u8>::decode(&mut buf, 0).unwrap();

        assert_eq!(size, 0);
        assert_eq!(&buf[..], &[0, 1, 2]);
        assert_eq!(values, Vec::<u8>::new());

        // Length is bigger than the buffer
        let mut buf = BytesMut::from(&[0, 1, 2][..]);

        let error = Vec::<u8>::decode(&mut buf, 5).unwrap_err();

        assert!(matches!(error, VecDecodeError::UnexpectedEndOfBuffer));

        // Length is within the buffer
        let mut buf = BytesMut::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);

        let (values, size) = Vec::<u8>::decode(&mut buf, 10).unwrap();

        assert_eq!(size, 10);
        assert!(buf.is_empty());
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut buf =
            BytesMut::from(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9][..]);

        let (values, size) = Vec::<u16>::decode(&mut buf, 20).unwrap();

        assert_eq!(size, 20);
        assert!(buf.is_empty());
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut buf = BytesMut::from(
            &[
                0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6,
                0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9,
            ][..],
        );

        // Actually 40 bytes, 50 will break
        let error = Vec::<u32>::decode(&mut buf, 50).unwrap_err();

        assert!(matches!(error, VecDecodeError::UnexpectedEndOfBuffer));

        let mut buf = BytesMut::from(&b"Hello\0World\0"[..]);

        let (values, size) = Vec::<COctetString<1, 6>>::decode(&mut buf, 12).unwrap();

        assert_eq!(size, 12);
        assert!(buf.is_empty());
        assert_eq!(
            values,
            alloc::vec![
                COctetString::<1, 6>::from_static_slice(b"Hello\0").unwrap(),
                COctetString::<1, 6>::from_static_slice(b"World\0").unwrap(),
            ]
        );

        let mut buf = BytesMut::from(&b"Hello\0World\0"[..]);

        let (values, size) = Vec::<EmptyOrFullCOctetString<6>>::decode(&mut buf, 12).unwrap();

        assert_eq!(size, 12);
        assert!(buf.is_empty());
        assert_eq!(
            values,
            alloc::vec![
                EmptyOrFullCOctetString::<6>::from_static_slice(b"Hello\0").unwrap(),
                EmptyOrFullCOctetString::<6>::from_static_slice(b"World\0").unwrap(),
            ]
        );

        let mut buf = BytesMut::from(&b"Hello\0World\0Hi"[..]);

        // This will try to decode 11 bytes b"Hello\0World"
        let error = Vec::<COctetString<1, 6>>::decode(&mut buf, 11).unwrap_err();

        assert!(matches!(
            error,
            VecDecodeError::ItemDecodeError(COctetStringDecodeError::NotNullTerminated)
        ));

        // Remaining bytes
        let mut buf = BytesMut::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);

        let (values, size) = Vec::<u8>::decode(&mut buf, 5).unwrap();

        assert_eq!(size, 5);
        assert_eq!(&buf[..], &[5, 6, 7, 8, 9]);
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4]);

        let mut buf =
            BytesMut::from(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9][..]);

        let (values, size) = Vec::<u16>::decode(&mut buf, 10).unwrap();

        assert_eq!(size, 10);
        assert_eq!(&buf[..], &[0, 5, 0, 6, 0, 7, 0, 8, 0, 9]);
        assert_eq!(values, alloc::vec![0, 1, 2, 3, 4]);
    }
}
