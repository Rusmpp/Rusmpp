//! Traits for decoding `SMPP` values with owned data by copying.

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

#[cfg(feature = "alloc")]
impl<T> DecodeErrorType for alloc::vec::Vec<T>
where
    T: DecodeErrorType,
{
    type Error = VecDecodeError<T::Error>;
}

/// Trait for decoding `SMPP` values from a slice.
pub trait Decode: DecodeErrorType + Sized + Sealed {
    /// Decode a value from a slice.
    fn decode(src: &[u8]) -> Result<(Self, usize), Self::Error>;
}

/// Trait for decoding `SMPP` values from a slice with a specified length.
pub trait DecodeWithLength: DecodeErrorType + Sized + Sealed {
    /// Decode a value from a slice, with a specified length
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), Self::Error>;
}

/// Everything that implements [`Decode`] also implements [`DecodeWithLength`] by ignoring the length.
impl<T: Decode> DecodeWithLength for T {
    fn decode(src: &[u8], _length: usize) -> Result<(Self, usize), Self::Error> {
        Decode::decode(src)
    }
}

/// Trait for decoding `SMPP` values from a slice with a specified key and length.
pub trait DecodeWithKey: DecodeErrorType + Sized + Sealed {
    type Key;

    /// Decode a value from a slice, using a key to determine the type.
    fn decode(key: Self::Key, src: &[u8], length: usize) -> Result<(Self, usize), Self::Error>;
}

/// Trait for decoding optional `SMPP` values from a slice with a specified key and length.
pub trait DecodeWithKeyOptional: DecodeErrorType + Sized + Sealed {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error>;
}

#[cfg(feature = "alloc")]
pub(crate) trait DecodeExt: Decode {
    fn decode_move(src: &[u8], size: usize) -> Result<(Self, usize), Self::Error> {
        Self::decode(&src[size..]).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a vector of values from a slice with a specified count.
    fn counted(src: &[u8], count: usize) -> Result<(alloc::vec::Vec<Self>, usize), Self::Error> {
        (0..count).try_fold(
            (alloc::vec::Vec::with_capacity(count), 0),
            |(mut vec, size), _| {
                Self::decode(&src[size..]).map(|(item, size_)| {
                    vec.push(item);

                    (vec, size + size_)
                })
            },
        )
    }

    fn counted_move(
        src: &[u8],
        count: usize,
        size: usize,
    ) -> Result<(alloc::vec::Vec<Self>, usize), Self::Error> {
        Self::counted(&src[size..], count).map(|(vec, size_)| (vec, size + size_))
    }

    /// Decode a value from a slice.
    ///
    /// If the length is 0, return `None`.
    fn length_checked_decode(
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::length_checked_decode(&src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: Decode> DecodeExt for T {}

pub(crate) trait DecodeWithLengthExt: DecodeWithLength {
    fn decode_move(src: &[u8], length: usize, size: usize) -> Result<(Self, usize), Self::Error> {
        Self::decode(&src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithLength> DecodeWithLengthExt for T {}

pub(crate) trait DecodeWithKeyExt: DecodeWithKey {
    /// Decode a value from a slice, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::optional_length_checked_decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }

    /// Decode a value from a slice, using a key to determine the type ignoring the length.
    fn no_length_decode_move(
        key: Self::Key,
        src: &[u8],
        size: usize,
    ) -> Result<(Self, usize), Self::Error> {
        Self::decode(key, &src[size..], 0).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithKey> DecodeWithKeyExt for T {}

pub(crate) trait DecodeWithKeyOptionalExt: DecodeWithKeyOptional {
    fn decode_move(
        key: Self::Key,
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, Self::Error> {
        Self::decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKeyOptional> DecodeWithKeyOptionalExt for T {}

#[cfg(feature = "alloc")]
impl<T: Decode> DecodeWithLength for alloc::vec::Vec<T> {
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), Self::Error> {
        if length == 0 {
            return Ok((alloc::vec::Vec::new(), 0));
        }

        if length > src.len() {
            return Err(VecDecodeError::UnexpectedEndOfBuffer);
        }

        let mut size = 0;

        let mut vec = alloc::vec::Vec::new();

        while size < length {
            let (item, size_) = T::decode(&src[size..length])?;

            size += size_;

            vec.push(item);
        }

        Ok((vec, size))
    }
}
