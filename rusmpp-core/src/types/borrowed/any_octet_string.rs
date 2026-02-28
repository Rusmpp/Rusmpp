use crate::{
    decode::{AnyOctetStringDecodeError, DecodeError, borrowed::DecodeWithLength},
    encode::{Encode, Length},
};

/// No fixed size [`OctetString`](struct@crate::types::borrowed::octet_string::OctetString).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct AnyOctetString<'a> {
    bytes: &'a [u8],
}

impl<'a> AnyOctetString<'a> {
    /// Creates a new empty [`AnyOctetString`].
    ///
    /// Equivalent to [`AnyOctetString::empty`].
    #[inline]
    pub const fn null() -> Self {
        Self::empty()
    }

    /// Creates a new empty [`AnyOctetString`].
    #[inline]
    pub const fn empty() -> Self {
        Self { bytes: &[] }
    }

    /// Returns the number of bytes contained in the [`AnyOctetString`].
    #[inline]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Creates a new [`AnyOctetString`] from &[[`u8`]].
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Returns the bytes of the [`AnyOctetString`].
    #[inline]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// Interprets the [`AnyOctetString`] as &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(self.bytes)
    }
}

impl core::fmt::Debug for AnyOctetString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AnyOctetString")
            .field("bytes", &crate::formatter::Formatter(self.bytes))
            .field("string", &self.to_str().unwrap_or("<invalid utf-8>"))
            .finish()
    }
}

impl Default for AnyOctetString<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

impl core::fmt::Display for AnyOctetString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str().unwrap_or("<invalid utf-8>"))
    }
}

impl core::convert::AsRef<[u8]> for AnyOctetString<'_> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl core::borrow::Borrow<[u8]> for AnyOctetString<'_> {
    fn borrow(&self) -> &[u8] {
        self.bytes
    }
}

impl core::ops::Deref for AnyOctetString<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl Length for AnyOctetString<'_> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Encode for AnyOctetString<'_> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(self.bytes);

        self.len()
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for AnyOctetString<'_> {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        dst.put_slice(self.bytes);
    }
}

impl<'a> DecodeWithLength<'a> for AnyOctetString<'a> {
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
        if src.len() < length {
            return Err(DecodeError::any_octet_string_decode_error(
                AnyOctetStringDecodeError::UnexpectedEof,
            ));
        }

        let bytes = &src[..length];

        Ok((Self { bytes }, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for AnyOctetString<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(
                    std::iter::repeat_n(b'1', 100)
                        .collect::<alloc::vec::Vec<_>>()
                        .leak()
                )
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<AnyOctetString<'static>>(
        );
    }

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::new(bytes);
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::new(bytes);
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

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::AnyOctetStringDecodeError(
                    AnyOctetStringDecodeError::UnexpectedEof
                )
            ));
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
