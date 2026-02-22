//! Traits for decoding `SMPP` values with borrowed data.

use crate::decode::{DecodeError, DecodeErrorType, VecDecodeError};

impl<const N: usize, T> DecodeErrorType for heapless::vec::Vec<T, N>
where
    T: DecodeErrorType,
{
    type Error = VecDecodeError<T::Error>;
}

/// Trait for decoding `SMPP` values from a slice.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::decode::{borrowed::Decode, DecodeError};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl<'a> Decode<'a> for Foo {
///     fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (c, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// let buf = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: 0x04050607,
/// };
///
/// let (foo, size) = Foo::decode(buf).unwrap();
///
/// assert_eq!(size, 7);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[size..], &[0x08]);
/// ```
pub trait Decode<'a>: 'a + Sized {
    /// Decode a value from a slice.
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a slice with a specified length.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo<'a> {
///     a: u8,
///     b: u16,
///     c: AnyOctetString<'a>,
/// }
///
/// impl<'a> DecodeWithLength<'a> for Foo<'a> {
///     fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (c, size) = AnyOctetString::decode(&src[index..], length - index)?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// // Received over the wire
/// let length = 8;
///
/// let buf = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: AnyOctetString::new(&[0x04, 0x05, 0x06, 0x07, 0x08]),
/// };
///
/// let (foo, size) = Foo::decode(buf, length).unwrap();
///
/// assert_eq!(size, 8);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[size..], &[0x09]);
/// ```
pub trait DecodeWithLength<'a>: 'a + Sized {
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
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithKey, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo<'a> {
///     A(u16),
///     B(AnyOctetString<'a>),
/// }
///
/// impl<'a> DecodeWithKey<'a> for Foo<'a> {
///     type Key = u32;
///
///     fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
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
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::A(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let buf = &[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(AnyOctetString::new(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKey<'a>: 'a + Sized {
    type Key;

    /// Decode a value from a slice, using a key to determine the type.
    fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding optional `SMPP` values from a slice with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithKeyOptional, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo<'a> {
///     A,
///     B(u16),
///     C(AnyOctetString<'a>),
/// }
///
/// impl<'a> DecodeWithKeyOptional<'a> for Foo<'a> {
///     type Key = u32;
///
///     fn decode(
///         key: Self::Key,
///         src: &'a [u8],
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
/// let buf = &[
///     0x00, 0x00, 0x00, 0x00, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::A;
///
/// assert_eq!(size, 0);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 4;
///
/// // Key is B, but the received length indicates no value
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let value = Foo::decode(key, &buf[index..], length - index).unwrap();
///
/// assert!(value.is_none());
/// assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is C
/// let buf = &[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::C(AnyOctetString::new(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKeyOptional<'a>: 'a + Sized {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

#[doc(hidden)]
pub trait DecodeExt<'a>: Decode<'a> {
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

            vec.push(item)
                .map_err(|_| DecodeError::too_many_elements(N))?;

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

#[doc(hidden)]
pub trait DecodeWithLengthExt<'a>: DecodeWithLength<'a> {
    fn decode_move(
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<'a, T: DecodeWithLength<'a>> DecodeWithLengthExt<'a> for T {}

#[doc(hidden)]
pub trait DecodeWithKeyExt<'a>: DecodeWithKey<'a> {
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
}

impl<'a, T: DecodeWithKey<'a>> DecodeWithKeyExt<'a> for T {}

#[doc(hidden)]
pub trait DecodeWithKeyOptionalExt<'a>: DecodeWithKeyOptional<'a> {
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
            return Err(DecodeError::vec_decode_error(VecDecodeError::UnexpectedEof));
        }

        let mut size = 0;

        let mut vec = heapless::vec::Vec::new();

        while size < length {
            let (item, size_) = T::decode(&src[size..length])?;

            size += size_;

            vec.push(item).map_err(|_| {
                DecodeError::vec_decode_error(VecDecodeError::TooManyElements { max: N })
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
            error.kind(),
            DecodeErrorKind::IntegerDecodeError(IntegerDecodeError::UnexpectedEof)
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
            error.kind(),
            DecodeErrorKind::IntegerDecodeError(IntegerDecodeError::UnexpectedEof)
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
            error.kind(),
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
            error.kind(),
            DecodeErrorKind::VecDecodeError(VecDecodeError::UnexpectedEof)
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
            error.kind(),
            DecodeErrorKind::VecDecodeError(VecDecodeError::UnexpectedEof)
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
            error.kind(),
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
