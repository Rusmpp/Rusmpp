/// A generic error that can occur when decoding `SMPP` values.
#[derive(Debug)]
#[non_exhaustive]
pub struct DecodeError {
    pub kind: DecodeErrorKind,
}

impl DecodeError {
    #[inline]
    pub(crate) const fn new(kind: DecodeErrorKind) -> Self {
        Self { kind }
    }

    #[inline]
    pub(crate) const fn integer_decode_error(error: IntegerDecodeError) -> Self {
        Self::new(DecodeErrorKind::IntegerDecodeError(error))
    }

    #[inline]
    pub(crate) const fn c_octet_string_decode_error(error: COctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::COctetStringDecodeError(error))
    }

    #[inline]
    pub(crate) const fn octet_string_decode_error(error: OctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::OctetStringDecodeError(error))
    }

    #[inline]
    pub(crate) const fn any_octet_string_decode_error(error: AnyOctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::AnyOctetStringDecodeError(error))
    }

    #[inline]
    pub(crate) const fn heapless_vec_decode_error(error: HeaplessVecDecodeError) -> Self {
        Self::new(DecodeErrorKind::HeaplessVecDecodeError(error))
    }

    #[inline]
    pub(crate) const fn unsupported_key(key: u32) -> Self {
        Self::new(DecodeErrorKind::UnsupportedKey { key })
    }

    #[inline]
    pub(crate) const fn concatenated_short_message_decode_error(
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
    UdhDecodeError(UdhDecodeError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a `Integer`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum IntegerDecodeError {
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
}

/// An error that can occur when decoding a `COctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum COctetStringDecodeError {
    /// The number of bytes is less than the minimum required.
    TooFewBytes { actual: usize, min: usize },
    /// The bytes are not ASCII.
    NotAscii,
    /// The bytes are not null terminated.
    NotNullTerminated,
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
}

/// An error that can occur when decoding an `OctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum OctetStringDecodeError {
    /// The number of bytes exceeds the maximum allowed.
    TooManyBytes { actual: usize, max: usize },
    /// The number of bytes is less than the minimum required.
    TooFewBytes { actual: usize, min: usize },
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
}

/// An error that can occur when decoding an `AnyOctetString`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum AnyOctetStringDecodeError {
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
}

/// An error that can occur when decoding a `heapless::Vec<T>`.
#[derive(Debug, Copy, Clone)]
pub enum HeaplessVecDecodeError {
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
    /// An error that can occur when decoding a fixed size of items.
    ///
    /// E.g. decoding `[T; N]` where `N` is the fixed size. Mostly while decoding arrays of `TLVs`.
    TooManyItems { max: usize },
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

/// An error that can occur when decoding a `Vec<T>`.
#[derive(Debug, Copy, Clone)]
pub enum VecDecodeError<E> {
    /// Unexpected end of buffer.
    UnexpectedEndOfBuffer,
    /// Item decode error.
    ItemDecodeError(E),
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
            DecodeErrorKind::HeaplessVecDecodeError(e) => {
                write!(f, "Heapless vec decode error: {e}")
            }
            DecodeErrorKind::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
            DecodeErrorKind::UdhDecodeError(e) => write!(f, "UDH decode error: {e}"),
        }
    }
}

impl core::fmt::Display for IntegerDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IntegerDecodeError::UnexpectedEndOfBuffer => {
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
            COctetStringDecodeError::UnexpectedEndOfBuffer => write!(f, "Unexpected end of buffer"),
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
            OctetStringDecodeError::UnexpectedEndOfBuffer => {
                write!(f, "Unexpected end of buffer")
            }
        }
    }
}

impl core::error::Error for OctetStringDecodeError {}

impl core::fmt::Display for AnyOctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnyOctetStringDecodeError::UnexpectedEndOfBuffer => {
                write!(f, "Unexpected end of buffer")
            }
        }
    }
}

impl core::error::Error for AnyOctetStringDecodeError {}

impl core::fmt::Display for HeaplessVecDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HeaplessVecDecodeError::UnexpectedEndOfBuffer => {
                write!(f, "Unexpected end of buffer")
            }
            HeaplessVecDecodeError::TooManyItems { max } => {
                write!(f, "Too many items. max: {max}")
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

impl<E: core::fmt::Display> core::fmt::Display for VecDecodeError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VecDecodeError::UnexpectedEndOfBuffer => {
                write!(f, "Unexpected end of buffer")
            }
            VecDecodeError::ItemDecodeError(e) => write!(f, "Item decode error: {e}"),
        }
    }
}

impl<E> From<E> for VecDecodeError<E> {
    fn from(value: E) -> Self {
        VecDecodeError::ItemDecodeError(value)
    }
}

impl<E: core::error::Error> core::error::Error for VecDecodeError<E> {}

pub(crate) trait DecodeResultExt<T, E> {
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
