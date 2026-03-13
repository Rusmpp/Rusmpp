//! `SMPP` client error type.

use std::time::Duration;

use rusmpp::{
    Command,
    tokio_codec::{DecodeError, EncodeError},
    values::InterfaceVersion,
};

/// Errors that can occur during `SMPP` operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection to `SMPP` server failed.
    ///
    /// This error is returned by [`ConnectionBuilder::connect`](crate::builder::ConnectionBuilder::connect).
    #[error("Failed to connect to the server: {0}")]
    Connect(#[source] std::io::Error),
    /// I/O error occurred.
    ///
    /// This error can occur during reading from or writing to the network stream.
    ///
    /// This error can be returned by various methods, such as sending commands or during background operations through the event stream as an [`Event::Error`](crate::event::Event::Error).
    #[error("I/O error: {0}")]
    Io(#[source] std::io::Error),
    /// The connection to the `SMPP` server is closed.
    ///
    /// This can happen when the client tries to send a command on a closed connection.
    ///
    /// This error is returned by methods that send commands, such as [`bind_transceiver`](crate::client::Client::bind_transceiver) and [`submit_sm`](crate::client::Client::submit_sm).
    ///
    /// # Note
    ///
    /// [`Error::ConnectionClosed`] is different from [`Error::ConnectionClosedByPeer`].
    ///
    /// - [`Error::ConnectionClosed`] means that the background connection managing the `SMPP` connection is closed (for example, the user called [`Client::close`](crate::client::Client::close) or the connection encountered a fatal error and closed itself).
    /// - [`Error::ConnectionClosedByPeer`] means that the `SMPP` server closed the (TCP) connection unexpectedly.
    #[error("Connection closed")]
    ConnectionClosed,
    /// The connection to the `SMPP` server was closed by the peer.
    ///
    /// This can happen if the server closes the connection unexpectedly.
    ///
    /// This error goes through the event stream as an [`Event::Error`](crate::event::Event::Error).
    ///
    /// # Note
    ///
    /// [`Error::ConnectionClosedByPeer`] is different from [`Error::ConnectionClosed`].
    ///
    /// - [`Error::ConnectionClosedByPeer`] means that the `SMPP` server closed the (TCP) connection unexpectedly.
    /// - [`Error::ConnectionClosed`] means that the background connection managing the `SMPP` connection is closed (for example, the user called [`Client::close`](crate::client::Client::close) or the connection encountered a fatal error and closed itself).
    #[error("Connection closed by peer")]
    ConnectionClosedByPeer,
    /// Protocol encode error.
    ///
    /// This error can be returned by various methods, such as sending commands or during background operations through the event stream as an [`Event::Error`](crate::event::Event::Error).
    #[error("Protocol encode error: {0}")]
    Encode(#[source] EncodeError),
    /// Protocol decode error.
    ///
    /// This error can be returned by various methods, such as sending commands or during background operations through the event stream as an [`Event::Error`](crate::event::Event::Error).
    #[error("Protocol decode error: {0}")]
    Decode(#[source] DecodeError),
    /// The `SMPP` server did not respond to the [`EnquireLink`](rusmpp::Pdu::EnquireLink) request within the specified timeout.
    ///
    /// This error goes through the event stream as an [`Event::Error`](crate::event::Event::Error).
    #[error("Server did not respond to enquire link: timeout: {timeout:?}")]
    EnquireLinkTimeout {
        /// The timeout duration.
        timeout: Duration,
    },
    /// The `SMPP` operation timed out.
    ///
    /// The server did not respond to the request within the specified timeout.
    ///
    /// This error is returned by methods that send commands and wait for a response, such as [`bind_transceiver`](crate::client::Client::bind_transceiver) and [`submit_sm`](crate::client::Client::submit_sm).
    #[error("Response timed out: sequence number: {sequence_number}, timeout: {timeout:?}")]
    ResponseTimeout {
        /// The sequence number of the request that timed out.
        sequence_number: u32,
        /// The timeout duration.
        timeout: Duration,
    },
    /// The `SMPP` operation failed with an error response from the server.
    ///
    /// Error responses are responses with the status code other than [`EsmeRok`](rusmpp::CommandStatus::EsmeRok).
    ///
    /// This error is returned by methods that send commands and wait for a response, such as [`bind_transceiver`](crate::client::Client::bind_transceiver) and [`submit_sm`](crate::client::Client::submit_sm).
    #[error("Unexpected response from the server: response: {response:?}")]
    UnexpectedResponse {
        /// The response that was received from the server.
        response: Box<Command>,
    },
    /// The client used an interface version that is not supported by the library.
    ///
    /// The library supports only `SMPP v5.0`.
    ///
    /// This error is returned by methods that send bind commands, such as [`bind_transceiver`](crate::client::Client::bind_transceiver), [`bind_receiver`](crate::client::Client::bind_receiver), and [`bind_transmitter`](crate::client::Client::bind_transmitter).
    #[error("Unsupported interface version: {version:?}, supported version: {supported_version:?}")]
    UnsupportedInterfaceVersion {
        /// The requested interface version.
        version: InterfaceVersion,
        /// The version that is supported by the library.
        supported_version: InterfaceVersion,
    },
}

impl Error {
    pub(crate) fn unexpected_response(response: impl Into<Box<Command>>) -> Self {
        Self::UnexpectedResponse {
            response: response.into(),
        }
    }

    pub(crate) const fn unsupported_interface_version(version: InterfaceVersion) -> Self {
        Self::UnsupportedInterfaceVersion {
            version,
            supported_version: InterfaceVersion::Smpp5_0,
        }
    }

    pub(crate) const fn response_timeout(sequence_number: u32, timeout: Duration) -> Self {
        Self::ResponseTimeout {
            sequence_number,
            timeout,
        }
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::Io(error) => Error::Io(error),
            error => Error::Decode(error),
        }
    }
}

impl From<EncodeError> for Error {
    fn from(value: EncodeError) -> Self {
        match value {
            EncodeError::Io(error) => Error::Io(error),
            error => Error::Encode(error),
        }
    }
}
