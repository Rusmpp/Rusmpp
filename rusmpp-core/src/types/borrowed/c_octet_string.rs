#![allow(path_statements)]

use crate::{
    decode::{COctetStringDecodeError, DecodeError, borrowed::Decode},
    encode::{Encode, Length},
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
/// # use rusmpp_core::types::borrowed::c_octet_string::COctetString;
///
/// // does not compile
/// let string = COctetString::<0, 6>::new(b"Hello\0");
/// ```
/// `MIN` must be less than or equal to `MAX`.
///
/// ```rust, compile_fail
/// # use rusmpp_core::types::borrowed::c_octet_string::COctetString;
///
/// // does not compile
/// let string = COctetString::<10, 6>::new(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct COctetString<'a, const MIN: usize, const MAX: usize> {
    bytes: &'a [u8],
}

impl<'a, const MIN: usize, const MAX: usize> COctetString<'a, MIN, MAX> {
    const _ASSERT_MIN_NON_ZERO: () = assert!(MIN > 0, "MIN must be greater than 0");
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    const _ASSERT_VALID: () = {
        Self::_ASSERT_MIN_NON_ZERO;
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;
    };

    const EMPTY: [u8; MIN] = {
        let mut arr = [1u8; MIN];

        arr[MIN - 1] = 0;

        arr
    };

    /// Creates a new empty [`COctetString`].
    ///
    /// Equivalent to [`COctetString::empty`].
    #[inline]
    pub const fn null() -> Self {
        Self::_ASSERT_VALID;

        Self::empty()
    }

    /// Creates a new empty [`COctetString`].
    #[inline]
    pub const fn empty() -> Self {
        Self::_ASSERT_VALID;

        Self {
            bytes: &Self::EMPTY,
        }
    }

    /// Returns the number of bytes contained in the [`COctetString`] including the null terminator.
    #[inline]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Checks if the [`COctetString`] is empty.
    ///
    /// A [`COctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Creates a new [`COctetString`] from &[[`u8`]] including the null terminator.
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
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

    /// Create a new [`COctetString`] from `&'static [u8]` without checking the length and the null terminator.
    #[inline]
    pub(crate) const fn new_unchecked(bytes: &'a [u8]) -> Self {
        Self::_ASSERT_VALID;

        Self { bytes }
    }

    /// Returns the bytes of the [`COctetString`].
    #[inline]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// Interprets the [`COctetString`] as &[`str`] without the null terminator.
    #[inline]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
            .expect("COctetString is ascii by definition")
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for COctetString<'_, MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &crate::formatter::Formatter(self.bytes))
            .field("string", &self.as_str())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for COctetString<'_, MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for COctetString<'_, MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const MIN: usize, const MAX: usize> core::convert::AsRef<[u8]> for COctetString<'_, MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::borrow::Borrow<[u8]> for COctetString<'_, MIN, MAX> {
    fn borrow(&self) -> &[u8] {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::ops::Deref for COctetString<'_, MIN, MAX> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Length for COctetString<'_, MIN, MAX> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for COctetString<'_, MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.len()].copy_from_slice(self.bytes);

        self.len()
    }
}

#[cfg(feature = "alloc")]
impl<const MIN: usize, const MAX: usize> crate::encode::owned::Encode
    for COctetString<'_, MIN, MAX>
{
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        dst.put_slice(self.bytes);
    }
}

impl<'a, const MIN: usize, const MAX: usize> Decode<'a> for COctetString<'a, MIN, MAX> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_VALID;

        if src.len() < MIN {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::TooFewBytes {
                    actual: src.len(),
                    min: MIN,
                },
            ));
        }

        let mut bytes = src;

        for (i, &byte) in src.iter().take(MAX).enumerate() {
            bytes = &src[..i + 1];

            if byte == 0 {
                break;
            }
        }

        if bytes.last() != Some(&0x00) {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::NotNullTerminated,
            ));
        }

        if !bytes.is_ascii() {
            return Err(DecodeError::c_octet_string_decode_error(
                COctetStringDecodeError::NotAscii,
            ));
        }

        Ok((Self { bytes }, bytes.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const MIN: usize, const MAX: usize> crate::tests::TestInstance
        for COctetString<'static, MIN, MAX>
    {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(
                    core::iter::repeat_n(b'1', MIN - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<alloc::vec::Vec<_>>()
                        .leak(),
                )
                .unwrap(),
                Self::new(
                    core::iter::repeat_n(b'1', ((MIN + MAX) / 2) - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<alloc::vec::Vec<_>>()
                        .leak(),
                )
                .unwrap(),
                Self::new(
                    core::iter::repeat_n(b'1', MAX - 1)
                        .chain(core::iter::once(b'\0'))
                        .collect::<alloc::vec::Vec<_>>()
                        .leak(),
                )
                .unwrap(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<COctetString<'static, 1, 5>>();
        crate::tests::borrowed::encode_decode_test_instances::<COctetString<'static, 2, 5>>();
        crate::tests::borrowed::encode_decode_test_instances::<COctetString<'static, 3, 5>>();
    }

    mod new {
        use super::*;

        #[test]
        fn empty_too_few_bytes() {
            let bytes = b"";
            let error = COctetString::<1, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 0, min: 1 }));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<1, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, max: 5 }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<10, 20>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 6, min: 10 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = COctetString::<1, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<1, 6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0o\0";
            let error = COctetString::<1, 6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok_min() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_between_min_max() {
            let bytes = b"Hel\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod as_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.as_str(), "Hello");
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let bytes = b"";
            let error = COctetString::<1, 6>::decode(bytes).unwrap_err();

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
            let bytes = b"hi";
            let error = COctetString::<1, 6>::decode(bytes).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated
                )
            ));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<1, 5>::decode(bytes).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated
                )
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<10, 20>::decode(bytes).unwrap_err();

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
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<1, 6>::decode(bytes).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotAscii)
            ));
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0";
            let (string, size) = COctetString::<1, 6>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
        }

        #[test]
        fn ok_not_max() {
            let bytes = b"Hello\0";
            let (string, size) = COctetString::<1, 25>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
        }

        #[test]
        fn ok_empty_max() {
            let bytes = b"\0";
            let (string, size) = COctetString::<1, 1>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
        }

        #[test]
        fn ok_empty_not_max() {
            let bytes = b"\0";
            let (string, size) = COctetString::<1, 25>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
        }

        #[test]
        fn ok_remaining() {
            let bytes = b"Hello\0World!";
            let (string, size) = COctetString::<1, 10>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert_eq!(&bytes[size..], b"World!");
        }
    }
}
