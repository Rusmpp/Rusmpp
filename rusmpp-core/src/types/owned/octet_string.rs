#![allow(path_statements)]
use alloc::{string::String, string::ToString, vec::Vec};
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    decode::{DecodeError, OctetStringDecodeError, owned::DecodeWithLength},
    encode::{Encode, Length, owned::Encode as BEncode},
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
/// let string = OctetString::<10,5>::from_static_slice(b"Hello");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct OctetString<const MIN: usize, const MAX: usize> {
    bytes: Bytes,
}

#[cfg(feature = "arbitrary")]
impl<'a, const MIN: usize, const MAX: usize> ::arbitrary::Arbitrary<'a> for OctetString<MIN, MAX> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let bytes = Vec::<u8>::arbitrary(u)?;

        Ok(Self {
            bytes: Bytes::from_owner(bytes),
        })
    }
}

impl<const MIN: usize, const MAX: usize> OctetString<MIN, MAX> {
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    const _ASSERT_VALID: () = {
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;
    };

    /// Creates a new [`OctetString`] from a sequence of bytes.
    #[inline]
    #[deprecated(note = "use `from_bytes`, `from_slice` or `from_vec` instead")]
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_slice(bytes.as_ref())
    }

    /// Creates a new empty [`OctetString`].
    ///
    /// Equivalent to [`OctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::_ASSERT_VALID;

        Self::empty()
    }

    /// Creates a new empty [`OctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self::_ASSERT_VALID;

        Self {
            bytes: Bytes::from_static(&[0; MIN]),
        }
    }

    /// Returns the number of bytes contained in the [`OctetString`].
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if the [`OctetString`] is empty.
    ///
    /// An [`OctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Creates a new [`OctetString`] from [`Bytes`].
    pub fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

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

    /// Creates a new [`OctetString`] from [`BytesMut`].
    pub fn from_bytes_mut(bytes: BytesMut) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(bytes.freeze())
    }

    /// Creates a new [`OctetString`] from `&[u8]`.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::copy_from_slice(bytes))
    }

    /// Creates a new [`OctetString`] from `&'static [u8]`.
    ///
    /// This function does not copy or allocate.
    pub fn from_static_slice(bytes: &'static [u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_static(bytes))
    }

    /// Creates a new [`OctetString`] from `&'static` [`str`].
    ///
    /// This function does not copy or allocate.
    pub fn from_static_str(str: &'static str) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_static(str.as_bytes()))
    }

    /// Creates a new [`OctetString`] from [`Vec<u8>`].
    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_owner(bytes))
    }

    /// Creates a new [`OctetString`] from [`String`].
    pub fn from_string(string: String) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_vec(string.into_bytes())
    }

    /// Converts the [`OctetString`] into [`Bytes`].
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Converts the [`OctetString`] into [`Vec<u8>`].
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.into_bytes().into()
    }

    /// Interprets the [`OctetString`] as &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>> for Bytes {
    fn from(value: OctetString<MIN, MAX>) -> Self {
        value.into_bytes()
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>> for Vec<u8> {
    fn from(value: OctetString<MIN, MAX>) -> Self {
        value.into_vec()
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
        Self::from_slice(s.as_bytes())
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl<const MIN: usize, const MAX: usize> core::convert::AsRef<[u8]> for OctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::borrow::Borrow<[u8]> for OctetString<MIN, MAX> {
    fn borrow(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::ops::Deref for OctetString<MIN, MAX> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Length for OctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for OctetString<MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(&self.bytes);

        self.len()
    }
}

impl<const MIN: usize, const MAX: usize> BEncode for OctetString<MIN, MAX> {
    fn encode(&self, dst: &mut BytesMut) {
        dst.put(&self.bytes[..]);
    }
}

impl<const MIN: usize, const MAX: usize> DecodeWithLength for OctetString<MIN, MAX> {
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_VALID;

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

        let bytes = src.split_to(length).freeze();

        Ok((Self { bytes }, length))
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>>
    for super::any_octet_string::AnyOctetString
{
    fn from(octet_string: OctetString<MIN, MAX>) -> Self {
        Self::from_bytes(octet_string.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<Bytes> for OctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<BytesMut> for OctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        Self::from_bytes_mut(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<&[u8]> for OctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for OctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::from_string(string)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<Vec<u8>> for OctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_vec(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const MIN: usize, const MAX: usize> crate::tests::TestInstance for OctetString<MIN, MAX> {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::from_vec(core::iter::repeat_n(b'1', MIN).collect::<Vec<_>>()).unwrap(),
                Self::from_vec(core::iter::repeat_n(b'1', MAX / 2).collect::<Vec<_>>()).unwrap(),
                Self::from_vec(core::iter::repeat_n(b'1', MAX).collect::<Vec<_>>()).unwrap(),
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
            let error = OctetString::<0, 5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello World!";
            let error = OctetString::<15, 20>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 12, .. }));
        }

        #[test]
        fn ok_min() {
            let bytes = b"H";
            let octet_string = OctetString::<1, 13>::from_static_slice(bytes).unwrap();
            assert_eq!(octet_string.as_ref(), bytes);
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<1, 13>::from_static_slice(bytes).unwrap();
            assert_eq!(octet_string.as_ref(), bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let bytes = b"Hello\0";
            let octet_string = OctetString::<1, 13>::from_static_slice(bytes).unwrap();
            assert_eq!(octet_string.as_ref(), bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::from_static_slice(bytes).unwrap();
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod to_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::from_static_slice(bytes).unwrap();
            assert_eq!(octet_string.to_str().unwrap(), "Hello\0World!\0");
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let mut buf = BytesMut::new();
            let error = OctetString::<0, 6>::decode(&mut buf, 5).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn too_many_bytes() {
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let error = OctetString::<0, 5>::decode(&mut buf, 15).unwrap_err();

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
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let error = OctetString::<6, 10>::decode(&mut buf, 5).unwrap_err();

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
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let (string, size) = OctetString::<0, 5>::decode(&mut buf, 5).unwrap();

            assert_eq!(string.as_ref(), b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(size, 5);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_partial() {
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let (string, size) = OctetString::<0, 5>::decode(&mut buf, 3).unwrap();

            assert_eq!(string.as_ref(), b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&buf[..], b"lo");
        }
    }
}
