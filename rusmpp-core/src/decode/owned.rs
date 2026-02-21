//! Traits for decoding `SMPP` values with owned data.

use bytes::BytesMut;

use crate::decode::{DecodeError, DecodeErrorType};

/// Trait for decoding `SMPP` values from a buffer.
///
/// # Implementation
///
/// ```rust
/// # use bytes::BytesMut;
/// # use rusmpp_core::decode::{owned::Decode, DecodeError};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl Decode for Foo {
///     fn decode(src: &mut BytesMut) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(src)?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(src)?;
///         let index = index + size;
///
///         let (c, size) = Decode::decode(src)?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// let mut buf = BytesMut::from(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08][..]);
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: 0x04050607,
/// };
///
/// let (foo, size) = Foo::decode(&mut buf).unwrap();
///
/// assert_eq!(size, 7);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x08]);
/// ```
pub trait Decode: DecodeErrorType + Sized {
    /// Decode a value from a buffer.
    fn decode(src: &mut BytesMut) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a buffer with a specified length.
///
/// # Implementation
///
/// ```rust
/// # use bytes::BytesMut;
/// # use rusmpp_core::{
/// #     decode::{owned::{Decode, DecodeWithLength}, DecodeError},
/// #     types::owned::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: AnyOctetString,
/// }
///
/// impl DecodeWithLength for Foo {
///     fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(src)?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(src)?;
///         let index = index + size;
///
///         let (c, size) = AnyOctetString::decode(src, length - index)?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// // Received over the wire
/// let length = 8;
///
/// let mut buf = BytesMut::from(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09][..]);
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: AnyOctetString::from_static_slice(&[0x04, 0x05, 0x06, 0x07, 0x08]),
/// };
///
/// let (foo, size) = Foo::decode(&mut buf, length).unwrap();
///
/// assert_eq!(size, 8);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x09]);
/// ```
pub trait DecodeWithLength: DecodeErrorType + Sized {
    /// Decode a value from a buffer, with a specified length
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Everything that implements [`Decode`] also implements [`DecodeWithLength`] by ignoring the length.
impl<T: Decode> DecodeWithLength for T {
    fn decode(src: &mut BytesMut, _length: usize) -> Result<(Self, usize), DecodeError> {
        Decode::decode(src)
    }
}

/// Trait for decoding `SMPP` values from a buffer with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// # use bytes::BytesMut;
/// # use rusmpp_core::{
/// #     decode::{owned::{Decode, DecodeWithKey, DecodeWithLength}, DecodeError},
/// #     types::owned::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo {
///     A(u16),
///     B(AnyOctetString),
/// }
///
/// impl DecodeWithKey for Foo {
///     type Key = u32;
///
///     fn decode(key: Self::Key, src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError> {
///         match key {
///             0x01020304 => {
///                 let (a, size) = Decode::decode(src)?;
///
///                 Ok((Foo::A(a), size))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok((Foo::B(b), size))
///             }
///             _ => Err(DecodeError::unsupported_key(key)),
///         }
///     }
/// }
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is A
/// let mut buf = BytesMut::from(&[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &mut buf, length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::A(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let mut buf = BytesMut::from(&[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &mut buf, length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(AnyOctetString::from_static_slice(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKey: Sized {
    type Key;

    /// Decode a value from a buffer, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding optional `SMPP` values from a buffer with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// # use bytes::BytesMut;
/// # use rusmpp_core::{
/// #     decode::{owned::{Decode, DecodeWithKeyOptional, DecodeWithLength}, DecodeError},
/// #     types::owned::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo {
///     A,
///     B(u16),
///     C(AnyOctetString),
/// }
///
/// impl DecodeWithKeyOptional for Foo {
///     type Key = u32;
///
///     fn decode(
///         key: Self::Key,
///         src: &mut BytesMut,
///         length: usize,
///     ) -> Result<Option<(Self, usize)>, DecodeError> {
///         if length == 0 {
///             match key {
///                 0x00000000 => return Ok(Some((Foo::A, 0))),
///                 _ => return Ok(None),
///             }
///         }
///
///         match key {
///             0x01020304 => {
///                 let (a, size) = Decode::decode(src)?;
///
///                 Ok(Some((Foo::B(a), size)))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok(Some((Foo::C(b), size)))
///             }
///             _ => Err(DecodeError::unsupported_key(key)),
///         }
///     }
/// }
///
/// // Received over the wire
/// let length = 4;
///
/// // Key is A
/// let mut buf = BytesMut::from(&[
///     0x00, 0x00, 0x00, 0x00, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &mut buf, length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::A;
///
/// assert_eq!(size, 0);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 4;
///
/// // Key is B, but the received length indicates no value
/// let mut buf = BytesMut::from(&[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let value = Foo::decode(key, &mut buf, length - index).unwrap();
///
/// assert!(value.is_none());
/// assert_eq!(&buf[..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let mut buf = BytesMut::from(&[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &mut buf, length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is C
/// let mut buf = BytesMut::from(&[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ][..]);
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(&mut buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &mut buf, length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::C(AnyOctetString::from_static_slice(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKeyOptional: Sized {
    type Key;

    /// Decode an optional value from a buffer, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

#[doc(hidden)]
pub trait DecodeExt: Decode {
    fn decode_move(src: &mut BytesMut, size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(src).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a vector of values from a buffer with a specified count.
    fn counted(
        src: &mut BytesMut,
        count: usize,
    ) -> Result<(alloc::vec::Vec<Self>, usize), DecodeError> {
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
    ) -> Result<(alloc::vec::Vec<Self>, usize), DecodeError> {
        Self::counted(src, count).map(|(vec, size_)| (vec, size + size_))
    }

    /// Decode a value from a buffer.
    ///
    /// If the length is 0, return `None`.
    fn length_checked_decode(
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::length_checked_decode(src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: Decode> DecodeExt for T {}

#[doc(hidden)]
pub trait DecodeWithLengthExt: DecodeWithLength {
    fn decode_move(
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(src, length).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithLength> DecodeWithLengthExt for T {}

#[doc(hidden)]
pub trait DecodeWithKeyExt: DecodeWithKey {
    /// Decode a value from a buffer, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
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
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::optional_length_checked_decode(key, src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKey> DecodeWithKeyExt for T {}

#[doc(hidden)]
pub trait DecodeWithKeyOptionalExt: DecodeWithKeyOptional {
    fn decode_move(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::decode(key, src, length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKeyOptional> DecodeWithKeyOptionalExt for T {}

impl<T: Decode> DecodeWithLength for alloc::vec::Vec<T> {
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError> {
        if length == 0 {
            return Ok((alloc::vec::Vec::new(), 0));
        }

        if length > src.len() {
            return Err(DecodeError::unexpected_eof());
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
        decode::{COctetStringDecodeError, DecodeErrorKind},
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
        assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));

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

        assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));

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

        assert!(matches!(
            error.kind(),
            DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
        ));

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

        assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));

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

        assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));

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
            error.kind(),
            DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
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
