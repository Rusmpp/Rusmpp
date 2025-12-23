//! Tests in this module test the library's functionality based on the public API.
//!
//! They simulate real scenarios by creating in-memory connections between a test server and a client built using the library's public API.
//!
//! For more in depth tests, see `connection/tests.rs`.

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::Poll,
    time::{Duration, Instant},
};

use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{
        AlertNotification, BindReceiverResp, BindTransceiverResp, BindTransmitterResp, SubmitSm,
        SubmitSmResp,
    },
    tokio_codec::CommandCodec,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

use crate::{ConnectionBuilder, Event, Insight, InsightEvent, error::Error, mock::io::MockIo};

#[derive(Debug)]
pub struct Server {
    bind_delay: Duration,
    enquire_link_delay: Duration,
    response_delay: Duration,
    close_connection_delay: Duration,
}

impl Server {
    pub fn new() -> Self {
        Self {
            bind_delay: Duration::from_millis(500),
            enquire_link_delay: Duration::from_millis(500),
            response_delay: Duration::from_millis(500),
            close_connection_delay: Duration::from_secs(10),
        }
    }

    pub fn bind_delay(mut self, delay: Duration) -> Self {
        self.bind_delay = delay;
        self
    }

    pub fn enquire_link_delay(mut self, delay: Duration) -> Self {
        self.enquire_link_delay = delay;
        self
    }

    pub fn response_delay(mut self, delay: Duration) -> Self {
        self.response_delay = delay;
        self
    }

    pub fn close_connection_delay(mut self, delay: Duration) -> Self {
        self.close_connection_delay = delay;
        self
    }

    pub async fn run<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(self, stream: S) {
        let mut framed = Framed::new(stream, CommandCodec::new());

        let future = async move {
            while let Some(Ok(command)) = framed.next().await {
                let pdu: Pdu = match command.id() {
                    CommandId::EnquireLink => {
                        tokio::time::sleep(self.enquire_link_delay).await;

                        Pdu::EnquireLinkResp
                    }
                    CommandId::BindTransmitter => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindTransmitterResp::default().into()
                    }
                    CommandId::BindReceiver => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindReceiverResp::default().into()
                    }
                    CommandId::BindTransceiver => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindTransceiverResp::default().into()
                    }
                    CommandId::SubmitSm => {
                        tokio::time::sleep(self.response_delay).await;

                        SubmitSmResp::default().into()
                    }
                    CommandId::Unbind => {
                        tokio::time::sleep(self.response_delay).await;

                        Pdu::UnbindResp
                    }
                    CommandId::GenericNack => {
                        tracing::warn!("Received GenericNack. Crashing");

                        break;
                    }
                    _ => {
                        continue;
                    }
                };

                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(pdu);

                framed
                    .send(response)
                    .await
                    .expect("Failed to send response");
            }
        };

        let _ = tokio::time::timeout(self.close_connection_delay, future).await;
    }
}

/// A server that only issues an unbind after a delay.
#[derive(Debug)]
pub struct UnbindServer {
    delay: Duration,
}

impl UnbindServer {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }

    pub async fn run<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(self, stream: S) {
        let mut framed = Framed::new(stream, CommandCodec::new());

        let future = async {
            while let Some(Ok(command)) = framed.next().await {
                let pdu: Pdu = match command.id() {
                    CommandId::EnquireLink => Pdu::EnquireLinkResp,
                    CommandId::BindTransmitter => BindTransmitterResp::default().into(),
                    CommandId::BindReceiver => BindReceiverResp::default().into(),
                    CommandId::BindTransceiver => BindTransceiverResp::default().into(),
                    _ => {
                        continue;
                    }
                };

                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(pdu);

                framed
                    .send(response)
                    .await
                    .expect("Failed to send response");
            }
        };

        tokio::select! {
            _ = future => {

            },
            _ = tokio::time::sleep(self.delay) => {
                let unbind = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::Unbind);

                framed
                    .send(unbind)
                    .await
                    .expect("Failed to send unbind response");
            }
        }
    }
}

pub fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_line_number(true)
        .with_ansi(false)
        .try_init();
}

#[tokio::test]
async fn cancel_request_future_should_remove_pending_response() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(1000))
        .connected(client);

    let future = client.submit_sm(SubmitSm::default());

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            tracing::debug!("Canceling request future");
        }
        _ = future => {}
    }

    let pending_response = client
        .pending_responses()
        .await
        .expect("Failed to get pending responses");

    assert!(
        !pending_response.contains(&1),
        "Pending response was not removed"
    );

    // The submit sm response should be sent to the event stream

    let Some(Event::Incoming(command)) = events.next().await else {
        panic!("Expected command event");
    };

    assert!(matches!(command.id(), CommandId::SubmitSmResp));
    assert_eq!(command.sequence_number(), 1);

    client.close().await.expect("Failed to close connection");

    client.closed().await;

    let _ = events.count().await;
}

#[tokio::test]
async fn request_timeout_should_remove_pending_response() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .bind_delay(Duration::from_millis(200))
            .response_delay(Duration::from_secs(1))
            .run(server)
            .await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(500))
        .connected(client);

    let Error::ResponseTimeout {
        sequence_number, ..
    } = client.submit_sm(SubmitSm::default()).await.unwrap_err()
    else {
        panic!("Expected timeout error");
    };

    let pending_response = client
        .pending_responses()
        .await
        .expect("Failed to get pending responses");

    assert!(
        !pending_response.contains(&sequence_number),
        "Pending response was not removed"
    );

    // The submit sm response should be sent to the event stream

    let Some(Event::Incoming(command)) = events.next().await else {
        panic!("Expected command event");
    };

    assert!(matches!(command.id(), CommandId::SubmitSmResp));
    assert_eq!(command.sequence_number(), sequence_number);

    client.close().await.expect("Failed to close connection");

    client.closed().await;

    let _ = events.count().await;
}

#[tokio::test]
async fn no_wait_request_should_pipe_response_through_events() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(1000))
        .connected(client);

    let sequence_number = client
        .no_wait()
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");

    // The submit sm response should be sent to the event stream

    let Some(Event::Incoming(command)) = events.next().await else {
        panic!("Expected command event");
    };

    assert!(matches!(command.id(), CommandId::SubmitSmResp));
    assert_eq!(command.sequence_number(), sequence_number);

    client.close().await.expect("Failed to close connection");

    client.closed().await;

    let _ = events.count().await;
}

#[tokio::test]
async fn drop_client_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new().connected(client);

    drop(client);

    let _ = events.count().await;
}

#[tokio::test]
async fn drop_events_should_not_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, _) = ConnectionBuilder::new().connected(client);

    client
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");

    client.close().await.expect("Failed to close connection");

    client.closed().await;
}

#[tokio::test]
async fn request_after_closing_connection_should_fail() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new().connected(client);

    client.close().await.expect("Failed to close connection");

    let error = client.submit_sm(SubmitSm::default()).await.unwrap_err();

    assert!(matches!(error, Error::ConnectionClosed));

    client.closed().await;

    let _ = events.count().await;
}

#[tokio::test]
async fn close_connection_twice_should_fail() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new().connected(client);

    client.close().await.expect("Failed to close connection");

    let error = client.close().await.unwrap_err();

    assert!(matches!(error, Error::ConnectionClosed));

    let _ = events.count().await;
}

#[tokio::test]
async fn enquire_link_timeout_idle_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_secs(5))
            .run(server)
            .await;
    });

    let (_client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(2))
        .enquire_link_response_timeout(Duration::from_secs(1))
        .connected(client);

    let now = Instant::now();

    let _ = events.count().await;

    let elapsed = now.elapsed();

    assert!(
        elapsed.as_secs() == 3,
        "Enquire link timeout did not occur as expected"
    );
}

#[tokio::test]
async fn enquire_link_timeout_busy_sequential_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_secs(5))
            .response_delay(Duration::from_millis(100))
            .run(server)
            .await;
    });

    let (client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(2))
        .enquire_link_response_timeout(Duration::from_secs(1))
        .connected(client);

    let now = Instant::now();

    loop {
        if let Err(Error::ConnectionClosed) = client.submit_sm(SubmitSm::default()).await {
            // Connection closed as expected
            break;
        }
    }

    let _ = events.count().await;

    let elapsed = now.elapsed();

    assert!(
        elapsed.as_secs() == 3,
        "Enquire link timeout did not occur as expected"
    );
}

#[tokio::test]
async fn enquire_link_timeout_busy_concurrent_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_secs(5))
            .response_delay(Duration::from_millis(100))
            .run(server)
            .await;
    });

    let (client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(2))
        .enquire_link_response_timeout(Duration::from_secs(1))
        .connected(client);

    let now = Instant::now();

    loop {
        if !client.is_active() {
            break;
        }

        let client_clone = client.clone();

        tokio::spawn(async move {
            let _ = client_clone.submit_sm(SubmitSm::default()).await;
        });

        // No sleep => Test hangs
        tokio::time::sleep(Duration::from_nanos(1)).await;
    }

    let _ = events.count().await;

    let elapsed = now.elapsed();

    assert!(
        elapsed.as_secs() == 3,
        "Enquire link timeout did not occur as expected"
    );
}

#[tokio::test]
async fn server_crashes_on_request_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new().connected(client);

    // Our test server crashes on GenericNack command
    client
        .status(CommandStatus::EsmeRxPAppn)
        .generic_nack(1)
        .await
        .expect("Failed to send generic_nack");

    let _ = events.count().await;
}

#[tokio::test]
async fn connection_lost_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .close_connection_delay(Duration::from_secs(1))
            .run(server)
            .await;
    });

    let (client, events) = ConnectionBuilder::new().connected(client);

    tokio::time::sleep(Duration::from_secs(2)).await;

    let error = client.submit_sm(SubmitSm::default()).await.unwrap_err();

    assert!(matches!(error, Error::ConnectionClosed));

    let _ = events.count().await;
}

#[tokio::test]
async fn server_unbinds_and_closes_connection_should_close_connection() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        UnbindServer::new(Duration::from_secs(1)).run(server).await;
    });

    let (client, mut events) = ConnectionBuilder::new().connected(client);

    while let Some(event) = events.next().await {
        if let Event::Incoming(command) = event {
            if command.id() == CommandId::Unbind {
                let error = client
                    .status(CommandStatus::EsmeRok)
                    .unbind_resp(command.sequence_number())
                    .await
                    .unwrap_err();

                assert!(matches!(error, Error::ConnectionClosed));
            }
        }
    }

    let _ = events.count().await;
}

#[tokio::test]
async fn server_sends_an_operation_with_the_same_sequence_number_of_a_pending_response_should_go_through_events()
 {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        let mut framed = Framed::new(server, CommandCodec::new());

        let Some(Ok(command)) = framed.next().await else {
            panic!("Expected command");
        };

        // Out of the blue the server decides to send an AlertNotification
        // with the same sequence number as the pending SubmitSm response
        framed
            .send(
                Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(AlertNotification::default()),
            )
            .await
            .expect("Failed to send AlertNotification");

        framed
            .send(
                Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(SubmitSmResp::default()),
            )
            .await
            .expect("Failed to send SubmitSmResp");

        tokio::time::sleep(Duration::from_secs(1)).await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(500))
        .connected(client);

    let events = tokio::spawn(async move {
        // The server sent an AlertNotification with the same sequence number as the pending response
        let Some(Event::Incoming(command)) = events.next().await else {
            panic!("Expected command event");
        };

        assert!(matches!(command.id(), CommandId::AlertNotification));
        assert_eq!(command.sequence_number(), 1);

        // Server closed the connection

        let _ = events.count().await;
    });

    client
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");

    let _ = events.await;
}

/// See `server_ddos_client_should_still_send_requests_and_connection_should_still_manage_timeouts` in `connection/tests.rs`` for a more reliable test of the same behavior.
#[tokio::test]
async fn server_ddos_client_should_still_send_requests_and_connection_should_still_manage_timeouts()
{
    // Eventually, the stream poll_next will return pending, after the duplex stream reaches max_buf_size.
    // The loop guards inside the connection do not really have any effect on the connection's ability to handle timeouts in this particular case.
    // They guard against the connection being stuck in the stream poll loop (poll_next never returns pending).
    // I will keep them since they provide a way to predict the connection's behavior.
    let (server, client) = tokio::io::duplex(1024);

    let mut framed = Framed::new(server, CommandCodec::new());

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime")
            .block_on(async move {
                loop {
                    if framed
                        .send(
                            Command::builder()
                                .status(CommandStatus::EsmeRok)
                                .sequence_number(1)
                                .pdu(AlertNotification::default()),
                        )
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            });
    });

    let (client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(1))
        .enquire_link_response_timeout(Duration::from_millis(500))
        .response_timeout(Duration::from_millis(500))
        .connected(client);

    client
        .no_wait() // Server will not respond anyway, so we don't care about the response
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");

    // After the enquire link timeout, the connection should close
    let _ = events.count().await;
}

/// This test relies on [`enquire_link_timeout_idle_should_close_connection`] to be correct.
///
/// This function uses the same setup as the previous test, but disables the enquire link interval.
#[tokio::test]
async fn enquire_link_interval_none_should_not_send_enquire_link_commands() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_secs(3))
            .run(server)
            .await;
    });

    let (client, _events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(1))
        .enquire_link_response_timeout(Duration::from_secs(1))
        .no_enquire_link_interval()
        .connected(client);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // after 3 seconds the connection should still be active since no enquire link commands were sent
    assert!(client.is_active(), "Connection was closed unexpectedly");
}

/// The connection should not treat the enquire link response as a response to an enquire link sent by the connection.
///
/// The response should be passed to the client to handle it.
#[tokio::test]
async fn client_sends_enquire_link_connection_should_pass_response_to_client() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_millis(10))
        .connected(client);

    // Wait for the automatic enquire link to be sent
    // We can not guarantee that an enquire link with seq (x) was sent before we send our own with seq (y) while the connection is still waiting for the response with seq (x)
    tokio::time::sleep(Duration::from_millis(50)).await;

    client
        .enquire_link()
        .await
        .expect("Failed to send enquire_link");

    client.close().await.expect("Failed to close connection");

    client.closed().await;

    let _ = events.count().await;
}

#[tokio::test]
async fn disabled_auto_enquire_link_response_should_pipe_enquire_link_through_events() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        let mut framed = Framed::new(server, CommandCodec::new());

        framed
            .send(
                Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::EnquireLink),
            )
            .await
            .expect("Failed to send EnquireLink");
    });

    let (_client, mut events) = ConnectionBuilder::new()
        .disable_auto_enquire_link_response()
        .connected(client);

    // The enquire link request should be sent to the event stream
    let Some(Event::Incoming(command)) = events.next().await else {
        panic!("Expected command event");
    };

    assert!(matches!(command.id(), CommandId::EnquireLink));
}

#[tokio::test]
async fn stream_shutdown_should_be_called() {
    init_tracing();

    let shutdown_called = Arc::new(AtomicBool::new(false));
    let shutdown_called_clone = shutdown_called.clone();

    let mut io = MockIo::new();

    io.expect_poll_flush_pin()
        .returning(move |_cx| Poll::Ready(Ok(())));

    io.expect_poll_shutdown_pin().returning(move |_cx| {
        shutdown_called_clone.store(true, Ordering::SeqCst);
        Poll::Ready(Ok(()))
    });

    let (client, _) = ConnectionBuilder::new().connected(io);

    client.close().await.expect("Failed to close connection");

    assert!(
        shutdown_called.load(Ordering::SeqCst),
        "Stream shutdown was not called"
    );
}

#[tokio::test]
async fn sent_enquire_link_and_received_enquire_link_response_should_be_sent_through_events() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_millis(0))
            .run(server)
            .await;
    });

    let (client, events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_millis(1))
        .events()
        .insights()
        .connected(client);

    tokio::time::sleep(Duration::from_millis(10)).await;

    client.close().await.expect("Failed to close connection");

    let events = events
        .filter_map(|event| async {
            match event {
                InsightEvent::Insight(insight) => Some(insight),
                _ => None,
            }
        })
        .take(2)
        .collect::<Vec<_>>()
        .await;

    let expected_events = vec![
        Insight::SentEnquireLink(2),
        Insight::ReceivedEnquireLinkResp(2),
    ];

    assert_eq!(events, expected_events);
}

#[tokio::test]
async fn received_enquire_link_and_sent_enquire_link_response_should_be_sent_through_events() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    // Keep the server (framed) alive, otherwise the connection will fail on sending the response
    let handle = tokio::spawn(async move {
        let mut framed = Framed::new(server, CommandCodec::new());

        framed
            .send(
                Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::EnquireLink),
            )
            .await
            .expect("Failed to send EnquireLink");

        framed
    });

    let (client, events) = ConnectionBuilder::new()
        .no_enquire_link_interval()
        .events()
        .insights()
        .connected(client);

    tokio::time::sleep(Duration::from_millis(10)).await;

    client.close().await.expect("Failed to close connection");

    let events = events
        .filter_map(|event| async {
            match event {
                InsightEvent::Insight(insight) => Some(insight),
                _ => None,
            }
        })
        .take(2)
        .collect::<Vec<_>>()
        .await;

    let _ = handle.await.expect("Failed await server handle");

    let expected_events = vec![
        Insight::ReceivedEnquireLink(1),
        Insight::SentEnquireLinkResp(1),
    ];

    assert_eq!(events, expected_events);
}
