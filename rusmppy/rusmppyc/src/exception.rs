use humantime::format_duration;
use pyo3::{create_exception, exceptions::PyException, PyErr};
use rusmpp::extra::{
    concatenation::errors::MultipartError, encoding::errors::EncodeError as REncodeError,
};

use crate::encoder::EncodeError;

create_exception!(
    exceptions,
    RusmppycException,
    PyException,
    "Base class for all exceptions in the Rusmppyc library."
);

create_exception!(
    exceptions,
    ConnectException,
    RusmppycException,
    "Connection to `SMPP` server failed."
);

create_exception!(
    exceptions,
    ConnectionClosedException,
    RusmppycException,
    "Connection to the `SMPP` server is closed."
);

create_exception!(
    exceptions,
    IoException,
    RusmppycException,
    "IO error occurred."
);

create_exception!(
    exceptions,
    EncodeException,
    RusmppycException,
    "Failed to encode `SMPP` PDU."
);

create_exception!(
    exceptions,
    DecodeException,
    RusmppycException,
    "Failed to decode `SMPP` PDU."
);

create_exception!(
    exceptions,
    ResponseTimeoutException,
    RusmppycException,
    "The `SMPP` operation timed out."
);

create_exception!(
    exceptions,
    UnexpectedResponseException,
    RusmppycException,
    "The `SMPP` operation failed with an error response from the server."
);

create_exception!(
    exceptions,
    UnsupportedInterfaceVersionException,
    RusmppycException,
    "The client used an interface version that is not supported by the library."
);

create_exception!(
    exceptions,
    ValueException,
    RusmppycException,
    "The client created an invalid `SMPP` value."
);

create_exception!(
    exceptions,
    ShortMessageEncodeException,
    RusmppycException,
    "Failed to encode `short_message`."
);

create_exception!(
    exceptions,
    ShortMessageMultipartException,
    RusmppycException,
    "Failed to create `short_message` parts."
);

/// Errors that can occur while calling Rusmppyc functions.
///
/// These errors are not sent through the event stream, but are raised directly when calling the functions.
///
/// See [`Error`](rusmppc::error::Error).
#[derive(Debug, Clone)]
pub enum Exception {
    Connect(String),
    Io(String),
    ConnectionClosed(),
    Encode(String),
    Decode(String),
    ResponseTimeout {
        sequence_number: u32,
        timeout: String,
    },
    UnexpectedResponse {
        response: String,
    },
    UnsupportedInterfaceVersion {
        version: crate::generated::InterfaceVersion,
        supported_version: crate::generated::InterfaceVersion,
    },
    /// The user created an invalid `SMPP` value.
    Value {
        /// The name of the value that caused the error.
        name: String,
        /// The error message.
        error: String,
    },
    /// Failed to encode `short_message`.
    ShortMessageEncode(String),
    /// Failed to create `short_message` parts.
    ShortMessageMultipart(String),
    /// Other error type.
    ///
    /// Rusmppc error type is non-exhaustive and contains all errors returned by the library including the ones returned by the event stream.
    /// This error should not be returned by this library and if so it should be considered a bug.
    Other(String),
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::Connect(error) => write!(f, "Connect error: {}", error),
            Exception::Io(error) => write!(f, "IO error: {}", error),
            Exception::ConnectionClosed() => write!(f, "Connection closed"),
            Exception::Encode(error) => write!(f, "Encode error: {}", error),
            Exception::Decode(error) => write!(f, "Decode error: {}", error),
            Exception::ResponseTimeout {
                sequence_number,
                timeout,
            } => write!(
                f,
                "Response timeout: sequence number: {}, timeout: {}",
                sequence_number, timeout
            ),
            Exception::UnexpectedResponse { response } => {
                write!(f, "Unexpected response: {}", response)
            }
            Exception::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => write!(
                f,
                "Unsupported interface version: {:?}, supported version: {:?}",
                version, supported_version
            ),
            Exception::Value { name, error } => {
                write!(f, "Invalid SMPP value: name: {}, error: {}", name, error)
            }
            Exception::ShortMessageEncode(error) => {
                write!(f, "Failed to encode short_message: {}", error)
            }
            Exception::ShortMessageMultipart(error) => {
                write!(f, "Failed to create short_message parts: {}", error)
            }
            Exception::Other(error) => write!(f, "Other error: {}", error),
        }
    }
}

impl std::error::Error for Exception {}

impl From<rusmppc::error::Error> for Exception {
    fn from(error: rusmppc::error::Error) -> Self {
        match error {
            rusmppc::error::Error::Connect(error) => Exception::Connect(error.to_string()),
            rusmppc::error::Error::Io(error) => Exception::Io(error.to_string()),
            rusmppc::error::Error::ConnectionClosed => Exception::ConnectionClosed(),
            rusmppc::error::Error::Encode(error) => Exception::Encode(error.to_string()),
            rusmppc::error::Error::Decode(error) => Exception::Decode(error.to_string()),
            rusmppc::error::Error::ResponseTimeout {
                sequence_number,
                timeout,
            } => Exception::ResponseTimeout {
                sequence_number,
                timeout: format_duration(timeout).to_string(),
            },
            rusmppc::error::Error::UnexpectedResponse { response } => {
                Exception::UnexpectedResponse {
                    response: format!("{response:?}"),
                }
            }
            rusmppc::error::Error::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => Exception::UnsupportedInterfaceVersion {
                version: version.into(),
                supported_version: supported_version.into(),
            },
            _ => Exception::Other(error.to_string()),
        }
    }
}

impl From<MultipartError<EncodeError>> for Exception {
    fn from(error: MultipartError<EncodeError>) -> Self {
        Exception::ShortMessageMultipart(error.to_string())
    }
}

impl From<REncodeError<EncodeError>> for Exception {
    fn from(error: REncodeError<EncodeError>) -> Self {
        Exception::ShortMessageEncode(error.to_string())
    }
}

impl From<Exception> for PyErr {
    fn from(error: Exception) -> Self {
        match error {
            Exception::Connect(_) => ConnectException::new_err(error.to_string()),
            Exception::Io(_) => IoException::new_err(error.to_string()),
            Exception::ConnectionClosed() => ConnectionClosedException::new_err(error.to_string()),
            Exception::Encode(_) => EncodeException::new_err(error.to_string()),
            Exception::Decode(_) => DecodeException::new_err(error.to_string()),
            Exception::ResponseTimeout { .. } => {
                ResponseTimeoutException::new_err(error.to_string())
            }
            Exception::UnexpectedResponse { .. } => {
                UnexpectedResponseException::new_err(error.to_string())
            }
            Exception::UnsupportedInterfaceVersion { .. } => {
                UnsupportedInterfaceVersionException::new_err(error.to_string())
            }
            Exception::Value { .. } => ValueException::new_err(error.to_string()),
            Exception::ShortMessageEncode(_) => {
                ShortMessageEncodeException::new_err(error.to_string())
            }
            Exception::ShortMessageMultipart(_) => {
                ShortMessageMultipartException::new_err(error.to_string())
            }
            Exception::Other(_) => RusmppycException::new_err(error.to_string()),
        }
    }
}

pub trait ValueExceptionExt<T> {
    fn map_value_err(self, name: &'static str) -> Result<T, Exception>;
}

impl<T, E: std::error::Error> ValueExceptionExt<T> for Result<T, E> {
    fn map_value_err(self, name: &'static str) -> Result<T, Exception> {
        self.map_err(|error| Exception::Value {
            name: name.to_string(),
            error: error.to_string(),
        })
    }
}
