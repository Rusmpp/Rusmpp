#![allow(path_statements)]

use alloc::{string::String, string::ToString, vec::Vec};
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    decode::{COctetStringDecodeError, DecodeError, owned::Decode},
    encode::{Encode, Length, owned::Encode as BEncode},
    types::empty_or_full_c_octet_string::Error,
};

/// Empty or full [`COctetString`](struct@crate::types::owned::COctetString).
///
/// `N` is the maximum length of the string, including the null terminator.
///
/// Possible values:
///  - Empty: `[0x00]`
///  - Full: `[..(N - 1), 0x00]` where `0x00` not in `..(N - 1)`
///
/// # Notes
///
/// `N` must be greater than `0`.
/// ```rust, compile_fail
/// # use rusmpp_core::types::owned::empty_or_full_c_octet_string::EmptyOrFullCOctetString;
///
/// // does not compile
/// let string = EmptyOrFullCOctetString::<0>::from_static_slice(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    bytes: Bytes,
}

#[cfg(feature = "arbitrary")]
impl<'a, const N: usize> ::arbitrary::Arbitrary<'a> for EmptyOrFullCOctetString<N> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let bytes = Vec::<u8>::arbitrary(u)?;

        Ok(Self {
            bytes: Bytes::from_owner(bytes),
        })
    }
}

impl<const N: usize> EmptyOrFullCOctetString<N> {
    const _ASSERT_NON_ZERO: () = assert!(N > 0, "N must be greater than 0");

    const _ASSERT_VALID: () = {
        Self::_ASSERT_NON_ZERO;
    };

    /// Creates a new [`EmptyOrFullCOctetString`] from a sequence of bytes.
    #[inline]
    #[deprecated(note = "use `from_bytes`, `from_slice` or `from_vec` instead")]
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_slice(bytes.as_ref())
    }

    /// Creates a new empty [`EmptyOrFullCOctetString`].
    ///
    /// Equivalent to [`EmptyOrFullCOctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::_ASSERT_VALID;

        Self::empty()
    }

    /// Creates a new empty [`EmptyOrFullCOctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self::_ASSERT_VALID;

        Self {
            bytes: Bytes::from_static(&[0]),
        }
    }

    /// Returns the number of bytes contained in the [`EmptyOrFullCOctetString`] including the null terminator.
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if the [`EmptyOrFullCOctetString`] is empty.
    ///
    /// An [`EmptyOrFullCOctetString`] is considered empty if it
    /// contains only a single NULL octet `(0x00)`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Creates a new [`EmptyOrFullCOctetString`] from [`Bytes`] including the null terminator.
    pub fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        // We must have at least the null terminator
        if bytes.is_empty() {
            return Err(Error::TooFewBytes { actual: 0 });
        }

        // If we have at least one byte, it must be the null terminator
        if bytes.len() == 1 {
            if bytes[0] != 0 {
                return Err(Error::NotNullTerminated);
            }

            return Ok(Self { bytes });
        }

        if bytes.len() < N {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
            });
        }

        if bytes.len() > N {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: N,
            });
        }

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        Ok(Self { bytes })
    }

    /// Creates a new [`EmptyOrFullCOctetString`] from [`BytesMut`] including the null terminator.
    pub fn from_bytes_mut(bytes: BytesMut) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(bytes.freeze())
    }

    /// Creates a new [`EmptyOrFullCOctetString`] from `&[u8]` including the null terminator.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::copy_from_slice(bytes))
    }

    /// Creates a new [`EmptyOrFullCOctetString`] from `&'static [u8]` including the null terminator.
    ///
    /// This function does not copy or allocate.
    pub fn from_static_slice(bytes: &'static [u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_static(bytes))
    }

    // XXX: there is no `from_static_str` because it would allocate (Null terminator).

    /// Creates a new [`EmptyOrFullCOctetString`] from [`Vec<u8>`] including the null terminator.
    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_owner(bytes))
    }

    /// Creates a new [`EmptyOrFullCOctetString`] from [`String`] without the null terminator.
    pub fn from_string(string: String) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        let mut bytes = string.into_bytes();
        bytes.push(0);

        Self::from_vec(bytes)
    }

    /// Converts the [`EmptyOrFullCOctetString`] into [`Bytes`] including the null terminator.
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Converts the [`EmptyOrFullCOctetString`] into [`Vec<u8>`] including the null terminator.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.into_bytes().into()
    }

    /// Interprets the [`EmptyOrFullCOctetString`] as &[`str`] without the null terminator.
    #[inline]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[0..self.bytes.len() - 1])
            .expect("EmptyOrFullCOctetString is ascii by definition")
    }
}

impl<const N: usize> From<EmptyOrFullCOctetString<N>> for Bytes {
    fn from(value: EmptyOrFullCOctetString<N>) -> Self {
        value.into_bytes()
    }
}

impl<const N: usize> From<EmptyOrFullCOctetString<N>> for Vec<u8> {
    fn from(value: EmptyOrFullCOctetString<N>) -> Self {
        value.into_vec()
    }
}

impl<const N: usize> Default for EmptyOrFullCOctetString<N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> core::fmt::Debug for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EmptyOrFullCOctetString")
            .field("bytes", &crate::formatter::Formatter(&self.bytes))
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const N: usize> core::str::FromStr for EmptyOrFullCOctetString<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::_ASSERT_VALID;

        Self::from_string(String::from(s))
    }
}

impl<const N: usize> core::fmt::Display for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize> core::convert::AsRef<[u8]> for EmptyOrFullCOctetString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> core::borrow::Borrow<[u8]> for EmptyOrFullCOctetString<N> {
    fn borrow(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> core::ops::Deref for EmptyOrFullCOctetString<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> Length for EmptyOrFullCOctetString<N> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> Encode for EmptyOrFullCOctetString<N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(&self.bytes);

        self.len()
    }
}

impl<const N: usize> BEncode for EmptyOrFullCOctetString<N> {
    fn encode(&self, dst: &mut BytesMut) {
        dst.put(&self.bytes[..]);
    }
}

impl<const N: usize> Decode for EmptyOrFullCOctetString<N> {
    fn decode(src: &mut BytesMut) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_VALID;

        if src.is_empty() {
            return Err(DecodeError::unexpected_eof());
        }

        let index = src.iter().take(N).position(|&b| b == 0).ok_or_else(|| {
            DecodeError::c_octet_string_decode_error(COctetStringDecodeError::NotNullTerminated)
        })?;

        let len = index + 1;

        let bytes = src.split_to(len).freeze();

        if len > 1 && len < N {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::TooFewBytes {
                    actual: len,
                    min: N,
                },
            ));
        }

        if !bytes.is_ascii() {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::NotAscii,
            ));
        }

        Ok((Self { bytes }, len))
    }
}

impl<const N: usize> TryFrom<Bytes> for EmptyOrFullCOctetString<N> {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

impl<const N: usize> TryFrom<BytesMut> for EmptyOrFullCOctetString<N> {
    type Error = Error;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        Self::from_bytes_mut(bytes)
    }
}

impl<const N: usize> TryFrom<&[u8]> for EmptyOrFullCOctetString<N> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(bytes)
    }
}

impl<const N: usize> TryFrom<String> for EmptyOrFullCOctetString<N> {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::from_string(string)
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for EmptyOrFullCOctetString<N> {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_vec(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const N: usize> crate::tests::TestInstance for EmptyOrFullCOctetString<N> {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::from_vec(
                    core::iter::repeat_n(b'1', N - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<EmptyOrFullCOctetString<1>>();
        crate::tests::owned::encode_decode_test_instances::<EmptyOrFullCOctetString<2>>();
        crate::tests::owned::encode_decode_test_instances::<EmptyOrFullCOctetString<3>>();
    }

    mod new {
        use super::*;

        #[test]
        fn empty_too_few_bytes() {
            let bytes = b"";
            let error = EmptyOrFullCOctetString::<5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 0 }));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = EmptyOrFullCOctetString::<5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, max: 5 }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = EmptyOrFullCOctetString::<5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0lo\0";
            let error = EmptyOrFullCOctetString::<7>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use core::str::FromStr;

        use super::*;

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let string = "Hel";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0lo";
            let error = EmptyOrFullCOctetString::<7>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn not_ascii() {
            let string = "Hellö"; // ö is 2 bytes. Hellö = 6 bytes, + 1 null terminator = 7 bytes
            let error = EmptyOrFullCOctetString::<7>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn ok() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod as_str {
        use super::*;

        #[test]
        fn empty_ok() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap();
            assert!(string.as_str().is_empty());
            assert!(string.to_string().is_empty());
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_str(), "Hello");
            assert_eq!(string.to_string(), "Hello");
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let mut buf = BytesMut::new();
            let error = EmptyOrFullCOctetString::<6>::decode(&mut buf).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn not_null_terminated() {
            let mut buf = BytesMut::from(&b"Hi"[..]);
            let error = EmptyOrFullCOctetString::<2>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated
                )
            ));
        }

        #[test]
        fn too_many_bytes() {
            let mut buf = BytesMut::from(&b"Hello\0"[..]);
            let error = EmptyOrFullCOctetString::<5>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated,
                )
            ));
        }

        #[test]
        fn too_few_bytes() {
            let mut buf = BytesMut::from(&b"Hel\0"[..]);
            let error = EmptyOrFullCOctetString::<5>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
                    actual: 4,
                    min: 5,
                },)
            ));
        }

        #[test]
        fn not_ascii() {
            let mut buf = BytesMut::from(&b"Hell\xF0\0"[..]);
            let error = EmptyOrFullCOctetString::<6>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotAscii)
            ));
        }

        #[test]
        fn ok() {
            let mut buf = BytesMut::from(&b"Hello\0World!"[..]);
            let (string, size) = EmptyOrFullCOctetString::<6>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert_eq!(&buf[..], b"World!");
        }

        #[test]
        fn ok_empty() {
            let mut buf = BytesMut::from(&b"\0World!"[..]);
            let (string, size) = EmptyOrFullCOctetString::<6>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
            assert_eq!(&buf[..], b"World!");
        }
    }
}
