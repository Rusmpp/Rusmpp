#![allow(path_statements)]

use crate::{
    decode::{COctetStringDecodeError, DecodeError, borrowed::Decode},
    encode::{Encode, Length},
    types::empty_or_full_c_octet_string::Error,
};

/// Empty or full [`COctetString`](struct@crate::types::borrowed::c_octet_string::COctetString).
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
/// # use rusmpp_core::types::borrowed::empty_or_full_c_octet_string::EmptyOrFullCOctetString;
///
/// // does not compile
/// let string = EmptyOrFullCOctetString::<0>::new(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct EmptyOrFullCOctetString<'a, const N: usize> {
    bytes: &'a [u8],
}

impl<'a, const N: usize> EmptyOrFullCOctetString<'a, N> {
    const _ASSERT_NON_ZERO: () = assert!(N > 0, "N must be greater than 0");

    /// Create a new empty [`EmptyOrFullCOctetString`].
    ///
    /// Equivalent to [`EmptyOrFullCOctetString::empty`].
    #[inline]
    pub const fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`EmptyOrFullCOctetString`].
    #[inline]
    pub const fn empty() -> Self {
        Self::_ASSERT_NON_ZERO;

        Self { bytes: &[0] }
    }

    /// Check if an [`EmptyOrFullCOctetString`] is empty.
    ///
    /// An [`EmptyOrFullCOctetString`] is considered empty if it
    /// contains only a single NULL octet `(0x00)`.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bytes.len() == 1
    }

    /// Create a new [`EmptyOrFullCOctetString`] from a sequence of bytes including a null terminator.
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::_ASSERT_NON_ZERO;

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

    /// Convert an [`EmptyOrFullCOctetString`] to a &[`str`] without the null terminator.
    #[inline]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[0..self.bytes.len() - 1])
            .expect("EmptyOrFullCOctetString is ascii by definition")
    }

    /// Get the bytes of an [`EmptyOrFullCOctetString`].
    #[inline]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

impl<const N: usize> core::fmt::Debug for EmptyOrFullCOctetString<'_, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EmptyOrFullCOctetString")
            .field("bytes", &crate::formatter::Formatter(self.bytes))
            .field("string", &self.as_str())
            .finish()
    }
}

impl<const N: usize> Default for EmptyOrFullCOctetString<'_, N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> core::fmt::Display for EmptyOrFullCOctetString<'_, N> {
    /// Format an [`EmptyOrFullCOctetString`] without the null terminator.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize> Length for EmptyOrFullCOctetString<'_, N> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const N: usize> Encode for EmptyOrFullCOctetString<'_, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(self.bytes);

        self.bytes.len()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> crate::encode::owned::Encode for EmptyOrFullCOctetString<'_, N> {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        dst.put_slice(self.bytes);
    }
}

impl<'a, const N: usize> Decode<'a> for EmptyOrFullCOctetString<'a, N> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_NON_ZERO;

        for i in 0..N {
            if i >= src.len() {
                return Err(DecodeError::unexpected_eof());
            }

            let bytes = &src[..i + 1];

            if src[i] == 0 {
                let len = i + 1;

                if bytes.len() > 1 && bytes.len() < N {
                    return Err(DecodeError::c_octet_string_decode_error(
                        COctetStringDecodeError::TooFewBytes {
                            actual: bytes.len(),
                            min: N,
                        },
                    ));
                }

                if !bytes.is_ascii() {
                    return Err(DecodeError::c_octet_string_decode_error(
                        COctetStringDecodeError::NotAscii,
                    ));
                }

                return Ok((Self { bytes }, len));
            }
        }

        Err(DecodeError::c_octet_string_decode_error(
            COctetStringDecodeError::NotNullTerminated,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<const N: usize> crate::tests::TestInstance for EmptyOrFullCOctetString<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::empty(),
                Self::new(
                    core::iter::repeat_n(b'1', N - 1)
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
        crate::tests::borrowed::encode_decode_test_instances::<EmptyOrFullCOctetString<'static, 1>>(
        );
        crate::tests::borrowed::encode_decode_test_instances::<EmptyOrFullCOctetString<'static, 2>>(
        );
        crate::tests::borrowed::encode_decode_test_instances::<EmptyOrFullCOctetString<'static, 3>>(
        );
    }

    mod new {
        use super::*;

        #[test]
        fn empty_too_few_bytes() {
            let bytes = b"";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 0 }));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, max: 5 }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0lo\0";
            let error = EmptyOrFullCOctetString::<7>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod as_str {
        use super::*;

        #[test]
        fn empty_ok() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert!(string.as_str().is_empty());
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.as_str(), "Hello");
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let bytes = b"";
            let error = EmptyOrFullCOctetString::<6>::decode(bytes).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hi";
            let error = EmptyOrFullCOctetString::<2>::decode(bytes).unwrap_err();

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
            let error = EmptyOrFullCOctetString::<5>::decode(bytes).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(
                    COctetStringDecodeError::NotNullTerminated,
                )
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::decode(bytes).unwrap_err();

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
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::decode(bytes).unwrap_err();

            assert!(matches!(
                error.kind(),
                DecodeErrorKind::COctetStringDecodeError(COctetStringDecodeError::NotAscii)
            ));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!";
            let (string, size) = EmptyOrFullCOctetString::<6>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(size, 6);
            assert_eq!(&bytes[size..], b"World!");
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0World!";
            let (string, size) = EmptyOrFullCOctetString::<6>::decode(bytes).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(size, 1);
            assert_eq!(&bytes[size..], b"World!");
        }
    }
}
