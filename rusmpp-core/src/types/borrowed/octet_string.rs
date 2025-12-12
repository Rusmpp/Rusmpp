#![allow(path_statements)]

use crate::{
    decode::{DecodeError, OctetStringDecodeError, borrowed::DecodeWithLength},
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
/// # use rusmpp_core::types::borrowed::octet_string::OctetString;
///
/// // does not compile
/// let string = OctetString::<10,5>::new(b"Hello");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct OctetString<'a, const MIN: usize, const MAX: usize> {
    bytes: &'a [u8],
}

impl<'a, const MIN: usize, const MAX: usize> OctetString<'a, MIN, MAX> {
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    /// Create a new empty [`OctetString`].
    ///
    /// Equivalent to [`OctetString::empty`].
    #[inline]
    pub const fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`OctetString`].
    #[inline]
    pub const fn empty() -> Self {
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        Self { bytes: &[0; MIN] }
    }

    /// Check if an [`OctetString`] is empty.
    ///
    /// An [`OctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub const fn new(bytes: &'a [u8]) -> Result<Self, Error> {
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
        core::str::from_utf8(self.bytes)
    }

    /// Get the bytes of an [`OctetString`].
    #[inline]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for OctetString<'_, MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &crate::formatter::Formatter(self.bytes))
            .field("string", &self.to_str().unwrap_or("<invalid utf-8>"))
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for OctetString<'_, MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for OctetString<'_, MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str().unwrap_or("<invalid utf-8>"))
    }
}

impl<'a, const MIN: usize, const MAX: usize> From<OctetString<'a, MIN, MAX>>
    for super::any_octet_string::AnyOctetString<'a>
{
    fn from(octet_string: OctetString<'a, MIN, MAX>) -> Self {
        Self::new(octet_string.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> Length for OctetString<'_, MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for OctetString<'_, MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(self.bytes);

        self.bytes.len()
    }
}

#[cfg(feature = "alloc")]
impl<const MIN: usize, const MAX: usize> crate::encode::owned::Encode
    for OctetString<'_, MIN, MAX>
{
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        dst.put_slice(self.bytes);
    }
}

impl<'a, const MIN: usize, const MAX: usize> DecodeWithLength<'a> for OctetString<'a, MIN, MAX> {
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
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

        let bytes = &src[..length];

        Ok((Self { bytes }, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const MIN: usize, const MAX: usize> crate::tests::TestInstance
        for OctetString<'static, MIN, MAX>
    {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(
                    core::iter::repeat_n(b'1', MIN)
                        .collect::<alloc::vec::Vec<_>>()
                        .leak()
                )
                .unwrap(),
                Self::new(
                    core::iter::repeat_n(b'1', MAX / 2)
                        .collect::<alloc::vec::Vec<_>>()
                        .leak()
                )
                .unwrap(),
                Self::new(
                    core::iter::repeat_n(b'1', MAX)
                        .collect::<alloc::vec::Vec<_>>()
                        .leak()
                )
                .unwrap(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            OctetString<'static, 0, 5>,
        >();
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            OctetString<'static, 1, 5>,
        >();
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            OctetString<'static, 2, 5>,
        >();
    }

    mod new {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0World!\0";
            let error = OctetString::<0, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello World!";
            let error = OctetString::<15, 20>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 12, .. }));
        }

        #[test]
        fn ok_min() {
            let bytes = b"H";
            let octet_string = OctetString::<1, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<1, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let bytes = b"Hello\0";
            let octet_string = OctetString::<1, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod to_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
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
