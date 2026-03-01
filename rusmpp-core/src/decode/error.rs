// TODO: name all errors with the suffix "Error" for consistency.
// TODO: owned: make decode traits return the DecodeErrorType::Error instead of DecodeError.
// TODO: borrowed: keep the DecodeError for borrowed types. The borrowed version of the lib should be lightweight, and creating a custom decode error type like the owned version has its limitations because of the lifetimes and generic params.
// TODO: refine the DecodeError.

/// An error that can occur when decoding `SMPP` values.
#[derive(Debug)]
pub struct DecodeError {
    pub kind: DecodeErrorKind,
}

impl DecodeError {
    #[inline]
    pub const fn new(kind: DecodeErrorKind) -> Self {
        Self { kind }
    }

    #[inline]
    pub const fn kind(&self) -> DecodeErrorKind {
        self.kind
    }

    #[inline]
    pub const fn integer_decode_error(error: IntegerDecodeError) -> Self {
        Self::new(DecodeErrorKind::IntegerDecodeError(error))
    }

    #[inline]
    pub const fn c_octet_string_decode_error(error: COctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::COctetStringDecodeError(error))
    }

    #[inline]
    pub const fn octet_string_decode_error(error: OctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::OctetStringDecodeError(error))
    }

    #[inline]
    pub const fn any_octet_string_decode_error(error: AnyOctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::AnyOctetStringDecodeError(error))
    }

    #[inline]
    pub const fn heapless_vec_decode_error(error: HeaplessVecDecodeError) -> Self {
        Self::new(DecodeErrorKind::HeaplessVecDecodeError(error))
    }

    #[inline]
    pub const fn unsupported_key(key: u32) -> Self {
        Self::new(DecodeErrorKind::UnsupportedKey { key })
    }

    #[inline]
    pub const fn udh_decode_error(error: UdhDecodeError) -> Self {
        Self::new(DecodeErrorKind::UdhDecodeError(error))
    }

    #[inline]
    pub const fn concatenated_short_message_decode_error(
        error: ConcatenatedShortMessageDecodeError,
    ) -> Self {
        Self::new(DecodeErrorKind::UdhDecodeError(
            UdhDecodeError::ConcatenatedShortMessageDecodeError(error),
        ))
    }
}

/// Kind of [`DecodeError`].
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DecodeErrorKind {
    IntegerDecodeError(IntegerDecodeError),
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
    AnyOctetStringDecodeError(AnyOctetStringDecodeError),
    HeaplessVecDecodeError(HeaplessVecDecodeError),
    UnsupportedKey { key: u32 },
    UdhDecodeError(UdhDecodeError),
}

/// An error that can occur when decoding a `Integer`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum IntegerDecodeError {
    UnexpectedEof,
}

/// An error that can occur when decoding a `COctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum COctetStringDecodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
    UnexpectedEof,
}

/// An error that can occur when decoding an `OctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum OctetStringDecodeError {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
    UnexpectedEof,
}

/// An error that can occur when decoding an `AnyOctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum AnyOctetStringDecodeError {
    UnexpectedEof,
}

/// An error that can occur when decoding a `heapless::Vec<T>`.
#[derive(Debug, Copy, Clone)]
pub enum HeaplessVecDecodeError {
    UnexpectedEof,
    /// An error that can occur when decoding a fixed size of elements.
    ///
    /// E.g. decoding `[T; N]` where `N` is the fixed size. Mostly while decoding arrays of `TLVs`.
    TooManyElements {
        max: usize,
    },
}

/// An error that can occur when decoding a `UDH`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum UdhDecodeError {
    ConcatenatedShortMessageDecodeError(ConcatenatedShortMessageDecodeError),
}

/// An error that can occur when decoding a `ConcatenatedShortMessage` UDH.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ConcatenatedShortMessageDecodeError {
    /// The length of the information element is invalid.
    InvalidInformationElementLength {
        actual: u8,
        expected: u8,
    },
    /// The total number of parts is zero.
    TotalPartsZero,
    /// The part number is zero.
    PartNumberZero,
    /// The part number exceeds the total number of parts.
    PartNumberExceedsTotalParts {
        part_number: u8,
        total_parts: u8,
    },
    TooFewBytes {
        actual: usize,
        min: usize,
    },
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Decode error. kind: {}", self.kind)
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self.kind {
            DecodeErrorKind::IntegerDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
            DecodeErrorKind::COctetStringDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
            DecodeErrorKind::OctetStringDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
            DecodeErrorKind::AnyOctetStringDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
            DecodeErrorKind::HeaplessVecDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
            DecodeErrorKind::UnsupportedKey { .. } => None,
            DecodeErrorKind::UdhDecodeError(err) => {
                Some(err as &(dyn ::core::error::Error + 'static))
            }
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        core::error::Error::source(self)
    }
}

impl core::fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeErrorKind::IntegerDecodeError(e) => write!(f, "Integer decode error: {e}"),
            DecodeErrorKind::COctetStringDecodeError(e) => write!(f, "COctetString error: {e}"),
            DecodeErrorKind::OctetStringDecodeError(e) => write!(f, "OctetString error: {e}"),
            DecodeErrorKind::AnyOctetStringDecodeError(e) => write!(f, "AnyOctetString error: {e}"),
            DecodeErrorKind::HeaplessVecDecodeError(e) => write!(f, "Vec decode error: {e}"),
            DecodeErrorKind::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
            DecodeErrorKind::UdhDecodeError(e) => write!(f, "UDH decode error: {e}"),
        }
    }
}

impl core::fmt::Display for IntegerDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IntegerDecodeError::UnexpectedEof => {
                write!(f, "Unexpected end of buffer")
            }
        }
    }
}

impl core::error::Error for IntegerDecodeError {}

impl core::fmt::Display for COctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            COctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
            COctetStringDecodeError::NotAscii => write!(f, "Not ASCII"),
            COctetStringDecodeError::NotNullTerminated => write!(f, "Not null terminated"),
            COctetStringDecodeError::UnexpectedEof => write!(f, "Unexpected end of buffer"),
        }
    }
}

impl core::error::Error for COctetStringDecodeError {}

impl core::fmt::Display for OctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OctetStringDecodeError::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            OctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
            OctetStringDecodeError::UnexpectedEof => {
                write!(f, "Unexpected end of buffer")
            }
        }
    }
}

impl core::error::Error for OctetStringDecodeError {}

impl core::fmt::Display for AnyOctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnyOctetStringDecodeError::UnexpectedEof => {
                write!(f, "Unexpected end of buffer")
            }
        }
    }
}

impl core::error::Error for AnyOctetStringDecodeError {}

impl core::fmt::Display for HeaplessVecDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HeaplessVecDecodeError::UnexpectedEof => {
                write!(f, "Unexpected end of buffer")
            }
            HeaplessVecDecodeError::TooManyElements { max } => {
                write!(f, "Too many elements. max: {max}")
            }
        }
    }
}

impl core::error::Error for HeaplessVecDecodeError {}

impl core::fmt::Display for UdhDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UdhDecodeError::ConcatenatedShortMessageDecodeError(e) => {
                write!(f, "ConcatenatedShortMessage decode error: {e}")
            }
        }
    }
}

impl core::error::Error for UdhDecodeError {}

impl core::fmt::Display for ConcatenatedShortMessageDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConcatenatedShortMessageDecodeError::InvalidInformationElementLength {
                actual,
                expected,
            } => {
                write!(
                    f,
                    "Invalid information element length. actual: {actual}, expected: {expected}"
                )
            }
            ConcatenatedShortMessageDecodeError::PartNumberZero => {
                write!(f, "Part number cannot be zero")
            }
            ConcatenatedShortMessageDecodeError::PartNumberExceedsTotalParts {
                part_number,
                total_parts,
            } => {
                write!(
                    f,
                    "Part number {} exceeds total parts {}",
                    part_number, total_parts
                )
            }
            ConcatenatedShortMessageDecodeError::TotalPartsZero => {
                write!(f, "Total parts cannot be zero")
            }
            ConcatenatedShortMessageDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
        }
    }
}

impl core::error::Error for ConcatenatedShortMessageDecodeError {}

#[doc(hidden)]
pub trait DecodeResultExt<T, E> {
    fn map_decoded<F, U>(self, op: F) -> Result<(U, usize), E>
    where
        F: FnOnce(T) -> U;
}

impl<T, E> DecodeResultExt<T, E> for Result<(T, usize), E> {
    fn map_decoded<F, U>(self, op: F) -> Result<(U, usize), E>
    where
        F: FnOnce(T) -> U,
    {
        self.map(|(this, size)| (op(this), size))
    }
}
