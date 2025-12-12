use alloc::{string::String, string::ToString, vec::Vec};
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    decode::{DecodeError, owned::DecodeWithLength},
    encode::{Encode, Length, owned::Encode as BEncode},
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
    /// Creates a new [`AnyOctetString`] from a sequence of bytes.
    #[inline]
    #[deprecated(note = "use `from_bytes`, `from_slice` or `from_vec` instead")]
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self::from_slice(bytes.as_ref())
    }

    /// Creates a new empty [`AnyOctetString`].
    ///
    /// Equivalent to [`AnyOctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Creates a new empty [`AnyOctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self {
            bytes: Bytes::new(),
        }
    }

    /// Returns the number of bytes contained in the [`AnyOctetString`].
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Creates a new [`AnyOctetString`] from [`Bytes`].
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self { bytes }
    }

    /// Creates a new [`AnyOctetString`] from [`BytesMut`].
    pub fn from_bytes_mut(bytes: BytesMut) -> Self {
        Self::from_bytes(bytes.freeze())
    }

    /// Creates a new [`AnyOctetString`] from `&[u8]`.
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(Bytes::copy_from_slice(bytes))
    }

    /// Creates a new [`AnyOctetString`] from `&'static [u8]`.
    ///
    /// This function does not copy or allocate.
    pub fn from_static_slice(bytes: &'static [u8]) -> Self {
        Self::from_bytes(Bytes::from_static(bytes))
    }

    /// Creates a new [`AnyOctetString`] from `&'static` [`str`].
    ///
    /// This function does not copy or allocate.
    pub fn from_static_str(str: &'static str) -> Self {
        Self::from_bytes(Bytes::from_static(str.as_bytes()))
    }

    /// Creates a new [`AnyOctetString`] from [`Vec<u8>`].
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self::from_bytes(Bytes::from_owner(bytes))
    }

    /// Creates a new [`AnyOctetString`] from [`String`].
    pub fn from_string(string: String) -> Self {
        Self::from_vec(string.into_bytes())
    }

    /// Converts the [`AnyOctetString`] into [`Bytes`].
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Converts the [`AnyOctetString`] into [`Vec<u8>`].
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.into_bytes().into()
    }

    /// Interprets the [`AnyOctetString`] as &[`str`].
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

impl BEncode for AnyOctetString {
    fn encode(&self, dst: &mut BytesMut) {
        dst.put(&self.bytes[..]);
    }
}

impl DecodeWithLength for AnyOctetString {
    fn decode(src: &mut BytesMut, length: usize) -> Result<(Self, usize), DecodeError> {
        if src.len() < length {
            return Err(DecodeError::unexpected_eof());
        }

        let bytes = src.split_to(length).freeze();

        Ok((Self { bytes }, length))
    }
}

impl From<Bytes> for AnyOctetString {
    fn from(bytes: Bytes) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<BytesMut> for AnyOctetString {
    fn from(bytes: BytesMut) -> Self {
        Self::from_bytes_mut(bytes)
    }
}

impl From<&[u8]> for AnyOctetString {
    fn from(bytes: &[u8]) -> Self {
        Self::from_slice(bytes)
    }
}

impl From<String> for AnyOctetString {
    fn from(string: String) -> Self {
        Self::from_string(string)
    }
}

impl From<Vec<u8>> for AnyOctetString {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes)
    }
}

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
        crate::tests::owned::encode_decode_with_length_test_instances::<AnyOctetString>();
    }

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::from_static_slice(bytes);
            assert_eq!(octet_string.as_ref(), bytes);
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
            let mut buf = BytesMut::new();
            let error = AnyOctetString::decode(&mut buf, 5).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn ok_all() {
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let (string, size) = AnyOctetString::decode(&mut buf, 5).unwrap();

            assert_eq!(string.as_ref(), b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(size, 5);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_partial() {
            let mut buf = BytesMut::from(&b"Hello"[..]);
            let (string, size) = AnyOctetString::decode(&mut buf, 3).unwrap();

            assert_eq!(string.as_ref(), b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&buf[..], b"lo");
        }
    }
}
