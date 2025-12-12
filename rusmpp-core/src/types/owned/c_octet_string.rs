#![allow(path_statements)]

use alloc::{string::String, string::ToString, vec::Vec};
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    decode::{COctetStringDecodeError, DecodeError, owned::Decode},
    encode::{Encode, Length, owned::Encode as BEncode},
    types::c_octet_string::Error,
};

/// A [`COctetString`] is a sequence of ASCII characters
/// terminated with a NULL octet `0x00`.
///
/// The string “Hello” would be encoded in 6 octets (5
/// characters of “Hello” and NULL octet) as follows:
///
/// 0x48656C6C6F00
///
/// Two special variants exist for use within `SMPP`. These
/// are [`COctetString`] (Decimal) and [`COctetString`]
/// (Hexadecimal), which are used to carry decimal and
/// hexadecimal digit sequences respectively. These fields
/// are encoded the same way as any ASCII string, but are
/// specifically used to designate decimal and hexadecimal
/// numbers when presented in string format.
///
/// A Decimal [`COctetString`] “123456789” would be encoded
/// as follows:
///
/// 0x31323334353637383900
///
/// A Hexadecimal [`COctetString`] “A2F5ED278FC” would be
/// encoded as follows:
///
/// 0x413246354544323738464300
///
/// A NULL string “” is encoded as 0x00
///
/// `MIN` is the minimum length of the [`COctetString`] including the NULL octet.
/// `MAX` is the maximum length of the [`COctetString`] including the NULL octet.
///
/// Possible values:
///  - Min: `[..(MIN - 1), 0x00]` where `0x00` not in `..(MIN - 1)`
///    e.g. Min = 1: `[0x00]`, Min = 2: `[0x01, 0x00]`, Min = 3: `[0x01, 0x02, 0x00]`
///  - Max: `[..(MAX - 1), 0x00]` where `0x00` not in `..(MAX - 1)`
///  - Anything in between `MIN` and `MAX`.
///
/// # Notes
///
/// `MIN` must be greater than 0.
/// ```rust, compile_fail
/// # use rusmpp_core::types::owned::c_octet_string::COctetString;
///
/// // does not compile
/// let string = COctetString::<0, 6>::from_static_slice(b"Hello\0");
/// ```
/// `MIN` must be less than or equal to `MAX`.
///
/// ```rust, compile_fail
/// # use rusmpp_core::types::owned::c_octet_string::COctetString;
///
/// // does not compile
/// let string = COctetString::<10, 6>::from_static_slice(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
#[cfg_attr(
    any(feature = "serde", feature = "serde-deserialize-unchecked"),
    serde(transparent)
)]
pub struct COctetString<const MIN: usize, const MAX: usize> {
    bytes: Bytes,
}

#[cfg(feature = "arbitrary")]
impl<'a, const MIN: usize, const MAX: usize> ::arbitrary::Arbitrary<'a> for COctetString<MIN, MAX> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let bytes = Vec::<u8>::arbitrary(u)?;

        Ok(Self {
            bytes: Bytes::from_owner(bytes),
        })
    }
}

impl<const MIN: usize, const MAX: usize> COctetString<MIN, MAX> {
    const _ASSERT_MIN_NON_ZERO: () = assert!(MIN > 0, "MIN must be greater than 0");
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    const _ASSERT_VALID: () = {
        Self::_ASSERT_MIN_NON_ZERO;
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;
    };

    const EMPTY: [u8; MIN] = {
        // Fill the buffer with (`MIN` - 1) `NOT` NULL octets
        // And put a NULL octet at the end

        let mut arr = [1u8; MIN];

        arr[MIN - 1] = 0;

        arr
    };

    /// Creates a new [`COctetString`] from a sequence of bytes.
    #[inline]
    #[deprecated(note = "use `from_bytes`, `from_slice` or `from_vec` instead")]
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_slice(bytes.as_ref())
    }

    /// Creates a new empty [`COctetString`].
    ///
    /// Equivalent to [`COctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::_ASSERT_VALID;

        Self::empty()
    }

    /// Creates a new empty [`COctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self::_ASSERT_VALID;

        Self {
            bytes: Bytes::from_static(&Self::EMPTY),
        }
    }

    /// Returns the number of bytes contained in the [`COctetString`] including the null terminator.
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if the [`COctetString`] is empty.
    ///
    /// A [`COctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Creates a new [`COctetString`] from [`Bytes`] including the null terminator.
    pub fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        if bytes.len() < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
                min: MIN,
            });
        }

        if bytes.len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        // Now we can index into the bytes because we know it cannot be empty

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

    /// Creates a new [`COctetString`] from [`BytesMut`] including the null terminator.
    pub fn from_bytes_mut(bytes: BytesMut) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(bytes.freeze())
    }

    /// Creates a new [`COctetString`] from `&[u8]` including the null terminator.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::copy_from_slice(bytes))
    }

    /// Creates a new [`COctetString`] from `&'static [u8]` including the null terminator.
    ///
    /// This function does not copy or allocate.
    pub fn from_static_slice(bytes: &'static [u8]) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_static(bytes))
    }

    // XXX: there is no `from_static_str` because it would allocate (Null terminator).

    /// Creates a new [`COctetString`] from [`Vec<u8>`] including the null terminator.
    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        Self::from_bytes(Bytes::from_owner(bytes))
    }

    /// Creates a new [`COctetString`] from [`String`] without the null terminator.
    pub fn from_string(string: String) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        let mut bytes = string.into_bytes();
        bytes.push(0);

        Self::from_vec(bytes)
    }

    /// Creates a new [`COctetString`] from `&'static [u8]` without checking the length and the null terminator.
    #[inline]
    pub(crate) fn from_static_slice_unchecked(bytes: &'static [u8]) -> Self {
        Self::_ASSERT_VALID;

        Self {
            bytes: Bytes::from_static(bytes),
        }
    }

    /// Converts the [`COctetString`] into [`Bytes`] including the null terminator.
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Converts the [`COctetString`] into [`Vec<u8>`] including the null terminator.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.into_bytes().into()
    }

    /// Interprets the [`COctetString`] as &[`str`] without the null terminator.
    #[inline]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
            .expect("COctetString is ascii by definition")
    }
}

impl<const MIN: usize, const MAX: usize> From<COctetString<MIN, MAX>> for Bytes {
    fn from(value: COctetString<MIN, MAX>) -> Self {
        value.into_bytes()
    }
}

impl<const MIN: usize, const MAX: usize> From<COctetString<MIN, MAX>> for Vec<u8> {
    fn from(value: COctetString<MIN, MAX>) -> Self {
        value.into_vec()
    }
}

impl<const MIN: usize, const MAX: usize> Default for COctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for COctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &crate::formatter::Formatter(&self.bytes))
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> core::str::FromStr for COctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::_ASSERT_VALID;

        Self::from_string(String::from(s))
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for COctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const MIN: usize, const MAX: usize> core::convert::AsRef<[u8]> for COctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::borrow::Borrow<[u8]> for COctetString<MIN, MAX> {
    fn borrow(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::ops::Deref for COctetString<MIN, MAX> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Length for COctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for COctetString<MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(&self.bytes);

        self.len()
    }
}

impl<const MIN: usize, const MAX: usize> BEncode for COctetString<MIN, MAX> {
    fn encode(&self, dst: &mut BytesMut) {
        dst.put(&self.bytes[..]);
    }
}

impl<const MIN: usize, const MAX: usize> Decode for COctetString<MIN, MAX> {
    fn decode(src: &mut BytesMut) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_VALID;

        if src.len() < MIN {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::TooFewBytes {
                    actual: src.len(),
                    min: MIN,
                },
            ));
        }

        let index = src.iter().take(MAX).position(|&b| b == 0).ok_or_else(|| {
            DecodeError::c_octet_string_decode_error(COctetStringDecodeError::NotNullTerminated)
        })?;

        let bytes = src.split_to(index + 1).freeze();

        if !bytes.is_ascii() {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::NotAscii,
            ));
        }

        let size = bytes.len();

        Ok((Self { bytes }, size))
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<Bytes> for COctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<BytesMut> for COctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        Self::from_bytes_mut(bytes)
    }
}

impl<'a, const MIN: usize, const MAX: usize> TryFrom<&'a [u8]> for COctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Self::from_slice(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for COctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::from_string(string)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<Vec<u8>> for COctetString<MIN, MAX> {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_vec(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const MIN: usize, const MAX: usize> crate::tests::TestInstance for COctetString<MIN, MAX> {
        fn instances() -> Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::from_vec(
                    core::iter::repeat_n(b'1', MIN - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
                Self::from_vec(
                    core::iter::repeat_n(b'1', ((MIN + MAX) / 2) - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
                Self::from_vec(
                    core::iter::repeat_n(b'1', MAX - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<COctetString<1, 5>>();
        crate::tests::owned::encode_decode_test_instances::<COctetString<2, 5>>();
        crate::tests::owned::encode_decode_test_instances::<COctetString<3, 5>>();
    }

    mod new {
        use super::*;

        #[test]
        fn empty_too_few_bytes() {
            let bytes = b"";
            let error = COctetString::<1, 5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 0, min: 1 }));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<1, 5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, max: 5 }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<10, 20>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 6, min: 10 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = COctetString::<1, 5>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<1, 6>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0o\0";
            let error = COctetString::<1, 6>::from_static_slice(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok_min() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let bytes = b"Hel\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use core::str::FromStr;

        use super::*;

        #[test]
        fn null_null_byte_found() {
            let string = "\0";
            let error = COctetString::<1, 5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = COctetString::<1, 5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let string = "Hello";
            let error = COctetString::<10, 20>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 6, .. }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0o";
            let error = COctetString::<1, 6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn null_byte_found_at_the_end() {
            let string = "Hell\0";
            let error = COctetString::<1, 6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok_min() {
            let string = "";
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_max() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let string = "Hel";
            let bytes = b"Hel\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.as_ref(), bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod as_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::from_static_slice(bytes).unwrap();
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
            let error = COctetString::<1, 6>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
                    actual: 0,
                    min: 1,
                })
            ));
        }

        #[test]
        fn not_null_terminated() {
            let mut buf = BytesMut::from(&b"hi"[..]);
            let error = COctetString::<1, 6>::decode(&mut buf).unwrap_err();

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
            let error = COctetString::<1, 5>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated
                )
            ));
        }

        #[test]
        fn too_few_bytes() {
            let mut buf = BytesMut::from(&b"Hello\0"[..]);
            let error = COctetString::<10, 20>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
                    actual: 6,
                    min: 10,
                })
            ));
        }

        #[test]
        fn not_ascii() {
            let mut buf = BytesMut::from(&b"Hell\xF0\0"[..]);
            let error = COctetString::<1, 6>::decode(&mut buf).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotAscii)
            ));
        }

        #[test]
        fn ok_max() {
            let mut buf = BytesMut::from(&b"Hello\0"[..]);
            let (string, size) = COctetString::<1, 6>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_not_max() {
            let mut buf = BytesMut::from(&b"Hello\0"[..]);
            let (string, size) = COctetString::<1, 25>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_empty_max() {
            let mut buf = BytesMut::from(&b"\0"[..]);
            let (string, size) = COctetString::<1, 1>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_empty_not_max() {
            let mut buf = BytesMut::from(&b"\0"[..]);
            let (string, size) = COctetString::<1, 25>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
            assert!(buf.is_empty());
        }

        #[test]
        fn ok_remaining() {
            let mut buf = BytesMut::from(&b"Hello\0World!"[..]);
            let (string, size) = COctetString::<1, 10>::decode(&mut buf).unwrap();

            assert_eq!(string.as_ref(), b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert_eq!(&buf[..], b"World!");
        }
    }
}
