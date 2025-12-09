use alloc::{string::String, string::ToString, vec::Vec};
use bytes::{Bytes, BytesMut};

use crate::{
    decode::{DecodeError, owned::DecodeWithLength},
    encode::{Encode, Length},
};

/// No fixed size [`OctetString`](struct@crate::types::owned::octet_string::OctetString).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct AnyOctetString {
    bytes: Bytes,
}

#[cfg(feature = "arbitrary")]
impl<'a> ::arbitrary::Arbitrary<'a> for AnyOctetString {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let bytes = Vec::<u8>::arbitrary(u)?;

        Ok(Self {
            bytes: Bytes::from_owner(bytes),
        })
    }
}

impl AnyOctetString {
    /// Create a new empty [`AnyOctetString`].
    ///
    /// Equivalent to [`AnyOctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`AnyOctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self {
            bytes: Bytes::new(),
        }
    }

    /// Returns the number of bytes contained in this [`AnyOctetString`].
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`AnyOctetString`] from [`Bytes`].
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self { bytes }
    }

    /// Create a new [`AnyOctetString`] from [`BytesMut`].
    pub fn from_bytes_mut(bytes: BytesMut) -> Self {
        Self::from_bytes(bytes.freeze())
    }

    /// Create a new [`AnyOctetString`] from `&[u8]`.
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(Bytes::copy_from_slice(bytes))
    }

    /// Create a new [`AnyOctetString`] from `&'static [u8]`.
    pub fn from_static_slice(bytes: &'static [u8]) -> Self {
        Self::from_bytes(Bytes::from_static(bytes))
    }

    /// Create a new [`AnyOctetString`] from `&'static` [`str`].
    pub fn from_static_str(str: &'static str) -> Self {
        Self::from_bytes(Bytes::from_static(str.as_bytes()))
    }

    /// Create a new [`AnyOctetString`] from [`Vec<u8>`].
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self::from_bytes(Bytes::from_owner(bytes))
    }

    /// Create a new [`AnyOctetString`] from [`String`].
    pub fn from_string(string: String) -> Self {
        Self::from_vec(string.into_bytes())
    }

    /// Get the bytes of a [`AnyOctetString`].
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert a [`AnyOctetString`] to [`Bytes`].
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Convert a [`AnyOctetString`] to [`Vec<u8>`].
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.into_bytes().into()
    }

    /// Convert an [`AnyOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }
}

impl From<AnyOctetString> for Bytes {
    fn from(value: AnyOctetString) -> Self {
        value.into_bytes()
    }
}

impl From<AnyOctetString> for Vec<u8> {
    fn from(value: AnyOctetString) -> Self {
        value.into_vec()
    }
}

impl core::fmt::Debug for AnyOctetString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AnyOctetString")
            .field("bytes", &crate::formatter::Formatter(&self.bytes))
            .field("string", &self.to_string())
            .finish()
    }
}

impl Default for AnyOctetString {
    fn default() -> Self {
        Self::empty()
    }
}

impl core::str::FromStr for AnyOctetString {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_slice(s.as_bytes()))
    }
}

impl core::fmt::Display for AnyOctetString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl core::convert::AsRef<[u8]> for AnyOctetString {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl core::borrow::Borrow<[u8]> for AnyOctetString {
    fn borrow(&self) -> &[u8] {
        &self.bytes
    }
}

impl core::ops::Deref for AnyOctetString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl Length for AnyOctetString {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Encode for AnyOctetString {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(&self.bytes);

        self.len()
    }
}

impl DecodeWithLength for AnyOctetString {
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
        if src.len() < length {
            return Err(DecodeError::unexpected_eof());
        }

        let mut bytes = Vec::with_capacity(length);

        bytes.extend_from_slice(&src[..length]);

        Ok((
            Self {
                bytes: Bytes::from_owner(bytes),
            },
            length,
        ))
    }
}

impl From<Vec<u8>> for AnyOctetString {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes)
    }
}

// TODO: add From Traits

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for AnyOctetString {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::from_vec(std::iter::repeat_n(b'1', 100).collect::<Vec<_>>()),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<AnyOctetString>();
    }

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::from_static_slice(bytes);
            assert_eq!(octet_string.as_bytes(), bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::from_static_slice(bytes);
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let bytes = b"";
            let error = AnyOctetString::decode(bytes, 5).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let (string, size) = AnyOctetString::decode(bytes, 5).unwrap();

            assert_eq!(string.as_bytes(), b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(size, 5);
            assert_eq!(&bytes[size..], b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let (string, size) = AnyOctetString::decode(bytes, 3).unwrap();

            assert_eq!(string.as_bytes(), b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&bytes[size..], b"lo");
        }
    }
}
