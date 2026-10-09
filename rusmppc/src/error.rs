//! `SMPP` client error type.

use std::time::Duration;

use rusmpp::{
    Command,
    tokio_codec::{DecodeError, EncodeError},
    values::InterfaceVersion,
};

/// Why a request was never handed to the transport.
///
/// Carried by [`Error::NotSent`]: every reason here means the request definitely did not
/// reach the server, so retrying it can not duplicate a message.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum NotSentReason {
    /// The response timeout elapsed while the request was still queued.
    #[error("the response timeout elapsed before the request reached the transport")]
    Timeout,
    /// The connection ended while the request was still queued.
    #[error("the connection ended before the request reached the transport")]
    ConnectionClosed,
    /// The transport refused the write before accepting the bytes.
    #[error("the write could not begin: {0}")]
    Write(#[source] EncodeError),
    /// The connection still owes responses for too many written requests and refused this
    /// one before writing it.
    ///
    /// Every written request keeps its sequence number reserved — live or abandoned alike —
    /// until its response arrives or the connection ends: releasing one early would let a
    /// late reply be delivered to a newer request that reused the number. The reservation
    /// table is therefore bounded by admission, not by eviction, and this is the explicit
    /// refusal at that bound. Nothing was written, so retrying after some responses land
    /// (or on a new connection) can not duplicate a message.
    #[error(
        "the connection's sequence-number reservations are at capacity ({reserved} unresolved of a bound of {cap})"
    )]
    Capacity {
        /// How many sequence numbers the connection still reserves.
        reserved: usize,
        /// The bound it refuses to exceed.
        cap: usize,
    },
}

/// Errors that can occur during `SMPP` operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection to `SMPP` server failed.
    ///
    /// This error is returned by [`ConnectionBuilder::connect`](crate::ConnectionBuilder::connect).
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
    /// [`Error::ConnectionClosed`] is different from [`Error::UnexpectedEndOfStream`].
    ///
    /// - [`Error::ConnectionClosed`] means that the background connection managing the `SMPP` connection is closed (for example, the user called [`Client::close`](crate::client::Client::close) or the connection encountered a fatal error and closed itself).
    /// - [`Error::UnexpectedEndOfStream`] means that the `SMPP` stream (TCP stream) was closed unexpectedly.
    #[error("Connection closed")]
    ConnectionClosed,
    /// The `SMPP` stream was closed unexpectedly.
    ///
    /// This can happen if the connection was closed unexpectedly.
    ///
    /// This error goes through the event stream as an [`Event::Error`](crate::event::Event::Error).
    ///
    /// # Note
    ///
    /// [`Error::UnexpectedEndOfStream`] is different from [`Error::ConnectionClosed`].
    ///
    /// - [`Error::UnexpectedEndOfStream`] means that the `SMPP` stream (TCP stream) was closed unexpectedly.
    /// - [`Error::ConnectionClosed`] means that the background connection managing the `SMPP` connection is closed (for example, the user called [`Client::close`](crate::client::Client::close) or the connection encountered a fatal error and closed itself).
    #[error("Unexpected end of stream")]
    UnexpectedEndOfStream,
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
    /// The request was never handed to the transport: it definitely did not reach the
    /// server, and retrying it can not duplicate a message.
    ///
    /// The boundary is the transport's `start_send`. Before it the request is retractable:
    /// a cancellation (a dropped future, or the response timeout), a failed write that never
    /// began, or a connection that ends while the request is still queued all leave it
    /// **unsent**. After it the bytes are out: a failure of a written request is reported
    /// as an ordinary error (or a timeout), which means **maybe sent** — a conservative
    /// consumer must not retry those blindly.
    #[error("Request not sent: {reason}")]
    NotSent {
        /// Why the request was not sent.
        reason: NotSentReason,
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

    pub(crate) const fn not_sent(reason: NotSentReason) -> Self {
        Self::NotSent { reason }
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
