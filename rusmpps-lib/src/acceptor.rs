use crate::bind_request::BindRequest;
use futures::{StreamExt, future};
use rusmpp::tokio_codec::DecodeError;
use rusmpp::{CommandId, Pdu};
use rusmppc::Event::Incoming;
use rusmppc::{ConnectionBuilder, Event};
use std::time::Duration;
use thiserror::Error;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Acceptor {
    pub(crate) session_timeout: Duration,
    pub(crate) response_timeout: Option<Duration>,
    pub(crate) tcp_listener: TcpListener,
    pub(crate) enquire_link_interval: Duration,
    pub(crate) check_interface_version: bool,
}

impl Acceptor {
    pub async fn next(&self) -> Result<BindRequest, BindError> {
        let (stream, addr) = self.tcp_listener.accept().await?;
        let (client, mut events) = ConnectionBuilder::new()
            // Every 5 seconds send an enquire link command to the server.
            .enquire_link_interval(Duration::from_secs(5))
            // If the server does not respond within 2 seconds, consider it a timeout.
            .response_timeout(Duration::from_secs(2))
            .connected(stream);
        tracing::debug!(%addr, "Awaiting bind operation");
        let mut filtered = (&mut events).filter(|event| {
            future::ready(
                if let Incoming(command) = &event {
                    tracing::debug!(sequence_number=command.sequence_number(), id=?command.id(), "Received command");
                    matches!(
                    command.id(),
                    CommandId::BindTransmitter | CommandId::BindReceiver | CommandId::BindTransceiver
                )
                } else {
                    false
                })
        });
        tokio::select! {
            _ = tokio::time::sleep(self.session_timeout) => {
                tracing::warn!("Session timeout reached, closing connection");

                return Err(BindError::BindTimeout);
            },
            command = filtered.next() => {
                match command {
                    None => {
                        tracing::warn!(%addr, "Connection closed before bind command was received");
                        return Err(BindError::ConnectionClosedByPeer);
                    },
                    Some(Event::Incoming(command)) => {
                        tracing::debug!(%addr, id=?command.id(), "Received bind command");
                        tracing::trace!(%addr, ?command, "Received bind command");

                         let (_, _, sequence_number, pdu) = command.into_parts().raw();
                        match pdu {
                             None => {
                                tracing::error!(%addr, "Received bind command without PDU");

                                return Err(BindError::MissingPdu);
                            }
                            Some(Pdu::BindTransmitter(bind)) => {
                                Ok(BindRequest::from((client, sequence_number, bind)))
                            }
                            Some(Pdu::BindReceiver(bind)) => {
                                Ok(BindRequest::from((client, sequence_number, bind)))
                            }
                            Some(Pdu::BindTransceiver(bind)) => {
                                Ok(BindRequest::from((client, sequence_number, bind)))
                            },
                            _ => {
                                // Should not happen

                                return Err(BindError::UnexpectedError);
                            }
                        }
                    }
                     Some(Event::Error(err)) => {
                        tracing::error!(%addr, ?err, "Failed read command");

                        return Err(err.into());
                    },
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum BindError {
    #[error("Client did not send bind operation within the specified timeout")]
    BindTimeout,
    #[error("Connection closed by peer")]
    ConnectionClosedByPeer,
    /// Protocol decode error.
    ///
    /// This error can be returned by various methods, such as sending commands or during background operations through the event stream as an [`Event::Error`](crate::event::Event::Error).
    #[error("Protocol decode error: {0}")]
    Decode(#[source] DecodeError),
    #[error("Received bind command without PDU")]
    MissingPdu,
    #[error("Unexpected error occurred during bind operation")]
    UnexpectedError,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Rusmpp error: {0}")]
    RusmppError(#[from] rusmppc::error::Error),
}
