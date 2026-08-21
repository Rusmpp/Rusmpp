//! A `COctetString` is a sequence of ASCII characters
//! terminated with a NULL octet `0x00`.

/// An Error that can occur when creating a `COctetString`.
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Error::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            Error::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
            Error::NotNullTerminated => write!(f, "Not null terminated"),
            Error::NotAscii => write!(f, "Not ASCII"),
            Error::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl ::core::error::Error for Error {}
