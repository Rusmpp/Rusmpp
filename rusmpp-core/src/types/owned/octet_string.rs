#![allow(path_statements)]
use alloc::{string::String, string::ToString, vec::Vec};

use crate::{
    decode::{DecodeError, OctetStringDecodeError, owned::DecodeWithLength},
    encode::{Encode, Length},
    types::octet_string::Error,
};

/// An [`OctetString`] is a sequence of octets not necessarily
/// terminated with a NULL octet `0x00`.
///
/// Such fields using Octet String encoding,
/// typically represent fields that can be
/// used to encode raw binary data. In all circumstances, the
/// field will be either a fixed length field or explicit length field
/// where another field indicates the length of the Octet
/// String field. An example of this is the short_message field
/// of the submit_sm PDU that is [`OctetString`] encoded and
/// the previous message_length field specifies its length.
///
/// A NULL [`OctetString`] is not encoded. The explicit length
/// field that indicates its length should be set to zero.
///
///
/// `MIN` is the minimum length of the [`OctetString`].
/// `MAX` is the maximum length of the [`OctetString`].
///
/// Possible values:
///  - Min: `[..MIN]`
///  - Max: `[..MAX]`
///  - Anything in between `MIN` and `MAX`.
///
/// # Notes
///
/// `MIN` must be less than or equal to `MAX`.
/// ```rust, compile_fail
/// # use rusmpp_core::types::owned::octet_string::OctetString;
///
/// // does not compile
/// let string = OctetString::<10,5>::new(b"Hello");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct OctetString<const MIN: usize, const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MIN: usize, const MAX: usize> OctetString<MIN, MAX> {
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    /// Create a new empty [`OctetString`].
    ///
    /// Equivalent to [`OctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`OctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        Self {
            bytes: alloc::vec![0; MIN],
        }
    }

    /// Check if an [`OctetString`] is empty.
    ///
    /// An [`OctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`OctetString`] from the given bytes.
    ///
    /// This should replace the current `new` method.
    pub fn new(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        if bytes.len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        if bytes.len() < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
                min: MIN,
            });
        }

        Ok(Self { bytes })
    }

    /// Convert an [`OctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of an [`OctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`OctetString`] to a [`Vec`] of [`u8`].
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>> for Vec<u8> {
    fn from(value: OctetString<MIN, MAX>) -> Self {
        value.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &crate::formatter::Formatter(&self.bytes))
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for OctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::str::FromStr for OctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes().to_vec())
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for OctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>>
    for super::any_octet_string::AnyOctetString
{
    fn from(octet_string: OctetString<MIN, MAX>) -> Self {
        Self::new(octet_string.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> Length for OctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for OctetString<MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(&self.bytes);

        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> DecodeWithLength for OctetString<MIN, MAX> {
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        if length > MAX {
            return Err(DecodeError::octet_string_decode_error(
                OctetStringDecodeError::TooManyBytes {
                    actual: length,
                    max: MAX,
                },
            ));
        }

        if length < MIN {
            return Err(DecodeError::octet_string_decode_error(
                OctetStringDecodeError::TooFewBytes {
                    actual: length,
                    min: MIN,
                },
            ));
        }

        if src.len() < length {
            return Err(DecodeError::unexpected_eof());
        }

        let bytes = src[..length].to_vec();

        Ok((Self { bytes }, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const MIN: usize, const MAX: usize> crate::tests::TestInstance for OctetString<MIN, MAX> {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(core::iter::repeat_n(b'1', MIN).collect::<Vec<_>>()).unwrap(),
                Self::new(core::iter::repeat_n(b'1', MAX / 2).collect::<Vec<_>>()).unwrap(),
                Self::new(core::iter::repeat_n(b'1', MAX).collect::<Vec<_>>()).unwrap(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<OctetString<0, 5>>();
        crate::tests::owned::encode_decode_with_length_test_instances::<OctetString<1, 5>>();
        crate::tests::owned::encode_decode_with_length_test_instances::<OctetString<2, 5>>();
    }

    mod new {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0World!\0";
            let error = OctetString::<0, 5>::new(bytes.to_vec()).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello World!";
            let error = OctetString::<15, 20>::new(bytes.to_vec()).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 12, .. }));
        }

        #[test]
        fn ok_min() {
            let bytes = b"H";
            let octet_string = OctetString::<1, 13>::new(bytes.to_vec()).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<1, 13>::new(bytes.to_vec()).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let bytes = b"Hello\0";
            let octet_string = OctetString::<1, 13>::new(bytes.to_vec()).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes.to_vec()).unwrap();
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod to_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes.to_vec()).unwrap();
            assert_eq!(octet_string.to_str().unwrap(), "Hello\0World!\0");
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let bytes = b"";
            let error = OctetString::<0, 6>::decode(bytes, 5).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<0, 5>::decode(bytes, 15).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::OctetStringDecodeError(OctetStringDecodeError::TooManyBytes {
                    actual: 15,
                    max: 5,
                },)
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<6, 10>::decode(bytes, 5).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::OctetStringDecodeError(OctetStringDecodeError::TooFewBytes {
                    actual: 5,
                    min: 6,
                },)
            ));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let (string, size) = OctetString::<0, 5>::decode(bytes, 5).unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);

            assert_eq!(size, 5);
            assert_eq!(&bytes[size..], b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let (string, size) = OctetString::<0, 5>::decode(bytes, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&bytes[size..], b"lo");
        }
    }
}
