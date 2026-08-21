//! Empty or full [`COctetString`](struct@crate::types::c_octet_string).

/// An error that can occur when creating an `EmptyOrFullCOctetString`.
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            Self::TooFewBytes { actual } => {
                write!(f, "Too few bytes. actual: {actual}, min: 1")
            }
            Self::NotNullTerminated => write!(f, "Not null terminated"),
            Self::NotAscii => write!(f, "Not ASCII"),
            Self::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl ::core::error::Error for Error {}
