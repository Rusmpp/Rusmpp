use std::net::SocketAddr;
use std::{str::FromStr, sync::Arc, time::Duration};

use futures::{SinkExt, StreamExt, TryStreamExt, future};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindReceiverResp, BindTransceiverResp, BindTransmitterResp, SubmitSmResp},
    tokio_codec::CommandCodec,
    types::COctetString,
    values::InterfaceVersion,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::mpsc,
};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    bind_mode::BindMode,
    client::{Action, Client, ClientSession, ConnectedClients, SequenceNumber},
    timer::Timer,
};

#[derive(Debug)]
pub struct ConnectionConfig {
    pub connected_clients: ConnectedClients,
    pub clients: Vec<Client>,
    pub enquire_link_interval: Option<Duration>,
    pub enquire_link_response_timeout: Duration,
    pub session_timeout: Duration,
    pub bind_delay: Option<Duration>,
    pub response_delay: Option<Duration>,
    pub enquire_link_response_delay: Option<Duration>,
}

#[derive(Debug)]
pub struct Connection {
    session_id: u64,
    socket_addr: SocketAddr,
    config: Arc<ConnectionConfig>,
}

impl Connection {
    pub fn new(socket_addr: SocketAddr, session_id: u64, config: Arc<ConnectionConfig>) -> Self {
        Self {
            socket_addr,
            session_id,
            config,
        }
    }

    pub async fn run<S>(self, stream: S)
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        let session_id = self.session_id;

        let (reader, writer) = tokio::io::split(stream);

        let mut reader = FramedRead::new(reader, CommandCodec::new().with_max_length(1024));
        let mut writer = FramedWrite::new(writer, CommandCodec::new().with_max_length(1024));

        tracing::debug!(session_id, "Awaiting bind operation");

        let mut filtered = (&mut reader).try_filter(|command| {
            tracing::debug!(session_id, sequence_number=command.sequence_number(), id=?command.id(), "Received command");

            future::ready(matches!(
                command.id(),
                CommandId::BindTransmitter | CommandId::BindReceiver | CommandId::BindTransceiver
            ))
        });

        let (sequence_number, system_id, _, bind_mode) = tokio::select! {
            _ = tokio::time::sleep(self.config.session_timeout) => {
                tracing::warn!(session_id, "Session timeout reached, closing connection");

                return;
            },
            command = filtered.next() => {
                match command {
                    None => {
                        tracing::warn!(session_id, "Connection closed before bind command was received");

                        return;
                    },
                    Some(Err(err)) => {
                        tracing::error!(session_id, ?err, "Failed read command");

                        return;
                    },
                    Some(Ok(command)) => {
                        tracing::debug!(session_id, id=?command.id(), "Received bind command");
                        tracing::trace!(session_id, ?command, "Received bind command");

                        let (_, status, sequence_number, pdu) = command.into_parts().raw();

                        if !(matches!(status, CommandStatus::EsmeRok)) {
                            // Really?

                            tracing::error!(session_id, ?status, "Received bind command with non-OK status");

                            return;
                        }

                        let (system_id, password, bind_mode) = match pdu {
                            None => {
                                tracing::error!(session_id, "Received bind command without PDU");

                                return;
                            }
                            Some(Pdu::BindTransmitter(bind)) => {
                                (bind.system_id, bind.password, BindMode::Tx)
                            }
                            Some(Pdu::BindReceiver(bind)) => {
                                (bind.system_id, bind.password, BindMode::Rx)
                            }
                            Some(Pdu::BindTransceiver(bind)) => {
                                (bind.system_id, bind.password, BindMode::Trx)
                            },
                            _ => {
                                // Should not happen

                                return;
                            }
                        };

                        (sequence_number, system_id, password, bind_mode)
                    }
                }
            }
        };

        // TODO: Do the verification of system_id and password here

        let mc_system_id = COctetString::from_str("Rusmpps").expect("Must be valid system ID");
        let sc_interface_version = Some(InterfaceVersion::Smpp5_0);

        let pdu: Pdu = match bind_mode {
            BindMode::Tx => BindTransmitterResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
            BindMode::Rx => BindReceiverResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
            BindMode::Trx => BindTransceiverResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
        };

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu);

        if let Some(delay) = self.config.bind_delay {
            tokio::time::sleep(delay).await;
        }

        tracing::debug!(session_id, id=?command.id(), "Sending response");
        tracing::trace!(session_id, ?command, "Sending response");

        if let Err(err) = writer.send(command).await {
            tracing::error!(session_id, ?err, "Failed to send response");

            return;
        }

        let (tx, rx) = mpsc::channel(100);

        let mut sequence_number = SequenceNumber::new();
        let system_id = system_id.to_string();
        let session = ClientSession::new(tx, bind_mode.into());

        self.config
            .connected_clients
            .insert_session(system_id.clone(), session_id, session)
            .await;

        let mut actions = ReceiverStream::new(rx);

        let mut last_enquire_link_sequence_number = None;

        let enquire_link_resp_timer = Timer::new();
        tokio::pin!(enquire_link_resp_timer);

        let enquire_link_timer = self
            .config
            .enquire_link_interval
            .map(|delay| Timer::new().activated(delay))
            .unwrap_or_default();

        tokio::pin!(enquire_link_timer);

        loop {
            tokio::select! {
                _ = &mut enquire_link_resp_timer => {
                    tracing::warn!(session_id, "EnquireLink response timeout reached, closing connection");

                    break
                }
                _ = &mut enquire_link_timer => {
                    tracing::debug!(session_id, "Sending EnquireLink command");

                    let sequence_number = sequence_number.current_and_increment();

                    last_enquire_link_sequence_number = Some(sequence_number);

                    let command = Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::EnquireLink);

                    if let Err(err) = writer.send(command).await {
                        tracing::error!(session_id, sequence_number, ?err, "Failed to send EnquireLink command");

                        break
                    }

                    enquire_link_resp_timer.as_mut().activate(self.config.enquire_link_response_timeout);

                    if let Some(delay) = self.config.enquire_link_interval {
                         enquire_link_timer.as_mut().activate(delay);
                    }

                    tracing::debug!(session_id, sequence_number, "EnquireLink response timer activated");
                }
                action = actions.next() => {
                    let action = match action {
                        None => break,
                        Some(action) => action,
                    };

                    match action {
                        Action::Send(pdu) => {
                            let sequence_number = sequence_number.current_and_increment();
                            let command = Command::builder()
                                .status(CommandStatus::EsmeRok)
                                .sequence_number(sequence_number)
                                .pdu(pdu);
                            tracing::debug!(session_id, sequence_number, id=?command.id(), "Sending command");
                            tracing::trace!(session_id, sequence_number, ?command, "Sending command");

                            if let Err(err) = writer.send(command).await {
                                tracing::error!(session_id, ?err, "Failed to send command");

                                break
                            }
                        }
                    }
                }
                command = reader.next() => {
                    let command = match command {
                        None =>  break,
                        Some(Ok(command)) => command,
                        Some(Err(err)) => {
                            tracing::error!(session_id, ?err, "Failed to read command");

                            break
                        }
                    };

                    let sequence_number = command.sequence_number();

                    tracing::debug!(session_id, sequence_number, id=?command.id(), "Received command");
                    tracing::trace!(session_id, sequence_number, ?command, "Received command");

                    let (id, _, sequence_number, pdu) = command.into_parts().raw();

                    let (pdu, status) = match pdu {
                        Some(Pdu::Unbind) => {
                            (Pdu::UnbindResp, CommandStatus::EsmeRok)
                        },
                        Some(Pdu::EnquireLink) => {
                            (Pdu::EnquireLinkResp, CommandStatus::EsmeRok)
                        },
                        Some(Pdu::SubmitSm(_)) => {
                            (SubmitSmResp::builder()
                                .build()
                                .into(), CommandStatus::EsmeRok)
                        },
                        Some(Pdu::EnquireLinkResp) => {
                            match last_enquire_link_sequence_number {
                                Some(seq) => {
                                    if sequence_number != seq {
                                        tracing::warn!(session_id, id=?id, expected=seq, got=sequence_number, "Received EnquireLinkResp with unexpected sequence number");

                                        return
                                    }

                                    tracing::trace!(session_id, sequence_number, id=?id, "Received EnquireLinkResp");

                                    last_enquire_link_sequence_number = None;

                                    enquire_link_resp_timer.as_mut().disable();

                                    tracing::debug!(session_id, sequence_number, "EnquireLink response timer disabled");
                                }
                                None => {
                                    tracing::warn!(session_id, sequence_number, "Received EnquireLinkResp without a previous EnquireLink");
                                }
                            }

                            continue
                        }
                        _ => {
                            tracing::warn!(session_id, sequence_number, id=?id, "Received unsupported command");

                            continue
                        }
                    };

                    let delay = match pdu {
                        Pdu::EnquireLinkResp => self.config.enquire_link_response_delay,
                        _ => self.config.response_delay,
                    };

                    let command = Command::builder()
                        .status(status)
                        .sequence_number(sequence_number)
                        .pdu(pdu);

                    if let Some(delay) = delay {
                        // XXX: This will block the reading process
                        tokio::time::sleep(delay).await;
                    }

                    tracing::debug!(session_id, sequence_number, id=?command.id(), "Sending response");
                    tracing::trace!(session_id, sequence_number, ?command, "Sending response");

                    if let Err(err) = writer.send(command).await {
                        tracing::error!(session_id, sequence_number, ?err, "Failed to send response");

                        break;
                    }
                }
            }
        }

        self.config
            .connected_clients
            .remove_session(&system_id, session_id)
            .await;
    }
}
