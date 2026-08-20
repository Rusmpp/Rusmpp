use humantime::format_duration;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen_derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};

/// An error that can occur in the background connection.
///
/// This error is sent through the event stream.
#[pyclass]
#[gen_stub_pyclass_complex_enum]
#[derive(Debug, Clone)]
pub enum Error {
    /// IO error occurred.
    Io(String),
    /// The `SMPP` stream was closed unexpectedly.
    UnexpectedEndOfStream(),
    /// Protocol encode error.
    Encode(String),
    /// Protocol decode error.
    Decode(String),
    /// The `SMPP` server did not respond to the EnquireLink request within the specified timeout.
    EnquireLinkTimeout { timeout: String },
    /// Other error type.
    ///
    /// Rusmppc error type is non-exhaustive and contains all errors returned by the library including the ones not returned by the event stream.
    /// This error should not be returned by this library and if so it should be considered a bug.
    Other(String),
}

impl From<rusmppc::error::Error> for Error {
    fn from(error: rusmppc::error::Error) -> Self {
        match error {
            rusmppc::error::Error::Io(error) => Error::Io(error.to_string()),
            rusmppc::error::Error::UnexpectedEndOfStream => Error::UnexpectedEndOfStream(),
            rusmppc::error::Error::Encode(error) => Error::Encode(error.to_string()),
            rusmppc::error::Error::Decode(error) => Error::Decode(error.to_string()),
            rusmppc::error::Error::EnquireLinkTimeout { timeout } => Error::EnquireLinkTimeout {
                timeout: format_duration(timeout).to_string(),
            },
            _ => Error::Other(error.to_string()),
        }
    }
}

#[pymethods]
#[gen_stub_pymethods]
impl Error {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}
