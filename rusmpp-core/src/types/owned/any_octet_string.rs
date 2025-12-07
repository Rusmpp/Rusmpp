use alloc::{string::String, string::ToString, vec::Vec};

use crate::{
    decode::{DecodeError, owned::DecodeWithLength},
    encode::{Encode, Length},
};

/// No fixed size [`OctetString`](struct@crate::types::owned::octet_string::OctetString).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct AnyOctetString {
    bytes: Vec<u8>,
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
        Self { bytes: Vec::new() }
    }

    /// Check if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`AnyOctetString`] from a sequence of bytes.
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Convert an [`AnyOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of an [`AnyOctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`AnyOctetString`] to a [`Vec`] of [`u8`].
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl From<Vec<u8>> for AnyOctetString {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl From<AnyOctetString> for Vec<u8> {
    fn from(value: AnyOctetString) -> Self {
        value.bytes
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
        Ok(Self::new(s.as_bytes().to_vec()))
    }
}

impl core::fmt::Display for AnyOctetString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl AsRef<[u8]> for AnyOctetString {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Length for AnyOctetString {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl Encode for AnyOctetString {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(&self.bytes);

        self.bytes.len()
    }
}

impl DecodeWithLength for AnyOctetString {
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
        if src.len() < length {
            return Err(DecodeError::unexpected_eof());
        }

        let mut bytes = Vec::with_capacity(length);

        bytes.extend_from_slice(&src[..length]);

        Ok((Self { bytes }, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for AnyOctetString {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(std::iter::repeat_n(b'1', 100).collect::<Vec<_>>()),
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
            let octet_string = AnyOctetString::new(bytes.to_vec());
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::new(bytes.to_vec());
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

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(size, 5);
            assert_eq!(&bytes[size..], b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let (string, size) = AnyOctetString::decode(bytes, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&bytes[size..], b"lo");
        }
    }
}
