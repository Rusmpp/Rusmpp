use crate::fields::SmppField;

// TODO: name all errors with the suffix "Error" for consistency.
// TODO: delete the DecodeError. and the verbose feature.
// TODO: make decode traits return the DecodeErrorType::Error instead of DecodeError.

pub trait DecodeErrorType {
    type Error;
}

impl<T> DecodeErrorType for Option<T>
where
    T: DecodeErrorType,
{
    type Error = T::Error;
}

/// An error that can occur when decoding `SMPP` values.
#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind,
    #[cfg(feature = "verbose")]
    source: Option<alloc::boxed::Box<DecodeErrorSource>>,
}

impl DecodeError {
    #[inline]
    pub const fn new(kind: DecodeErrorKind) -> Self {
        #[cfg(feature = "verbose")]
        return Self { kind, source: None };

        #[cfg(not(feature = "verbose"))]
        Self { kind }
    }

    #[inline]
    #[cold]
    #[cfg(feature = "verbose")]
    #[cfg_attr(docsrs, doc(cfg(feature = "verbose")))]
    pub fn with_source(mut self, field: SmppField, error: DecodeError) -> Self {
        self.source = Some(alloc::boxed::Box::new(DecodeErrorSource { field, error }));
        self
    }

    #[inline]
    #[cold]
    #[cfg(feature = "verbose")]
    #[cfg_attr(docsrs, doc(cfg(feature = "verbose")))]
    pub fn as_source(self, field: SmppField) -> DecodeError {
        DecodeError::new(self.kind).with_source(field, self)
    }

    #[inline]
    #[cfg(feature = "verbose")]
    #[cfg_attr(docsrs, doc(cfg(feature = "verbose")))]
    pub fn source(&self) -> Option<&DecodeErrorSource> {
        self.source.as_deref()
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
    pub const fn vec_decode_error(error: VecDecodeError<usize>) -> Self {
        Self::new(DecodeErrorKind::VecDecodeError(error))
    }

    #[inline]
    pub const fn unsupported_key(key: u32) -> Self {
        Self::new(DecodeErrorKind::UnsupportedKey { key })
    }

    #[inline]
    pub const fn too_many_elements(max: usize) -> Self {
        Self::new(DecodeErrorKind::TooManyElements { max })
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

    /// Checks recursively if the field exists in the sources tree.
    #[cfg(feature = "verbose")]
    #[cfg_attr(docsrs, doc(cfg(feature = "verbose")))]
    pub fn field_exists(&self, field: SmppField) -> bool {
        if let Some(source) = &self.source {
            if source.field == field {
                return true;
            }

            return source.error.field_exists(field);
        }

        false
    }
}

/// Source of [`DecodeError`].
#[derive(Debug)]
#[cfg(feature = "verbose")]
#[cfg_attr(docsrs, doc(cfg(feature = "verbose")))]
pub struct DecodeErrorSource {
    field: SmppField,
    error: DecodeError,
}

#[cfg(feature = "verbose")]
impl DecodeErrorSource {
    pub const fn field(&self) -> SmppField {
        self.field
    }

    pub const fn error(&self) -> &DecodeError {
        &self.error
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
    VecDecodeError(VecDecodeError<usize>), // TODO: we erase the inner error type here until we removed the whole DecodeError and the verbose feature.
    UnsupportedKey {
        key: u32,
    },
    /// An error that can occur when decoding a fixed size of elements.
    ///
    /// E.g. decoding `[T; N]` where `N` is the fixed size. Mostly while decoding arrays of `TLVs`.
    TooManyElements {
        max: usize,
    },
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

/// An error that can occur when decoding a `Vec<T>`.
#[derive(Debug, Copy, Clone)]
pub enum VecDecodeError<E> {
    UnexpectedEof,
    TooManyElements { max: usize },
    ItemDecodeError(E),
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

#[cfg(feature = "verbose")]
impl core::fmt::Display for DecodeErrorSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "field: {:?}, error: {}", self.field, self.error)
    }
}

#[cfg(feature = "verbose")]
impl core::error::Error for DecodeErrorSource {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.error)
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "verbose")]
        return match &self.source {
            Some(source) => {
                write!(f, "Decode error. kind: {}, source: [{source}]", self.kind,)
            }
            None => write!(f, "Decode error. kind: {}", self.kind),
        };

        #[cfg(not(feature = "verbose"))]
        write!(f, "Decode error. kind: {}", self.kind)
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        #[cfg(feature = "verbose")]
        return match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        };

        #[cfg(not(feature = "verbose"))]
        None
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
            DecodeErrorKind::VecDecodeError(e) => write!(f, "Vec decode error: {e}"),
            DecodeErrorKind::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
            DecodeErrorKind::TooManyElements { max } => {
                write!(f, "Too many elements. max: {max}")
            }
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

impl<E: core::fmt::Display> core::fmt::Display for VecDecodeError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VecDecodeError::UnexpectedEof => {
                write!(f, "Unexpected end of buffer")
            }
            VecDecodeError::TooManyElements { max } => {
                write!(f, "Too many elements. max: {max}")
            }
            VecDecodeError::ItemDecodeError(e) => write!(f, "Item decode error: {e}"),
        }
    }
}

impl<E: core::error::Error> core::error::Error for VecDecodeError<E> {}

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

#[doc(hidden)]
pub trait DecodeErrorExt<T> {
    fn map_as_source(self, field: SmppField) -> Result<T, DecodeError>;
}

impl<T> DecodeErrorExt<T> for Result<T, DecodeError> {
    #[cold]
    fn map_as_source(self, _field: SmppField) -> Result<T, DecodeError> {
        #[cfg(feature = "verbose")]
        return self.map_err(|error| error.as_source(_field));

        #[cfg(not(feature = "verbose"))]
        self
    }
}
