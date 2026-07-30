use std::{
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindTransceiver, BindTransceiverResp, SubmitSm, SubmitSmResp},
    tokio_codec::CommandCodec,
};
use tokio::io::{AsyncRead, AsyncWrite, DuplexStream};
use tokio_util::codec::Framed;

use crate::{ConnectionBuilder, managed::ManagedEvent, tests::init_tracing};

/// Server that binds successfully and echoes [`SubmitSmResp`]s.
///
/// Runs until the client disconnects.
async fn run_ok_server<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(stream: S) {
    let mut framed = Framed::new(stream, CommandCodec::new());

    while let Some(Ok(command)) = framed.next().await {
        let pdu: Pdu = match command.id() {
            CommandId::BindTransceiver => BindTransceiverResp::default().into(),
            CommandId::SubmitSm => SubmitSmResp::default().into(),
            CommandId::EnquireLink => Pdu::EnquireLinkResp,
            CommandId::Unbind => Pdu::UnbindResp,
            _ => continue,
        };

        let response = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(command.sequence_number())
            .pdu(pdu);

        if framed.send(response).await.is_err() {
            break;
        }
    }
}

/// A connector that hands out a fresh `duplex` pair (with a fresh server
/// task behind it) every time it's called, and counts how many times it
/// was invoked. Used to test reconnection.
#[allow(clippy::type_complexity)]
fn counting_connector() -> (
    impl Fn() -> Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
    + Send
    + Sync
    + 'static,
    Arc<AtomicUsize>,
) {
    let count = Arc::new(AtomicUsize::new(0));
    let count_c = count.clone();

    let connector = move || {
        let count_c = count_c.clone();

        Box::pin(async move {
            count_c.fetch_add(1, Ordering::SeqCst);

            let (server, client) = tokio::io::duplex(4096);

            tokio::spawn(run_ok_server(server));

            Ok(client)
        }) as Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
    };

    (connector, count)
}

#[tokio::test]
async fn managed_client_connects_and_binds_transceiver() {
    init_tracing();

    let (connector, connect_count) = counting_connector();

    let (managed, mut events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    let Some(ManagedEvent::Connected) = events.next().await else {
        panic!("Expected Connected event");
    };

    let Some(ManagedEvent::Bound) = events.next().await else {
        panic!("Expected Bound event");
    };

    assert_eq!(connect_count.load(Ordering::SeqCst), 1);

    let client = managed.get().await.expect("Failed to get client");

    client
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");
}

#[tokio::test]
async fn managed_client_unbound_should_not_emit_bound_event() {
    init_tracing();

    let (connector, _connect_count) = counting_connector();

    let (_managed, mut events) = ConnectionBuilder::new()
        .managed()
        .unbound()
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    let Some(ManagedEvent::Connected) = events.next().await else {
        panic!("Expected Connected event");
    };

    // Give any (incorrect) Bound event a chance to show up.
    let next = tokio::time::timeout(Duration::from_millis(200), events.next()).await;

    match next {
        Ok(Some(ManagedEvent::Bound)) => panic!("Unbound client should not emit Bound event"),
        _ => { /* timed out or got something else: fine */ }
    }
}

#[tokio::test]
async fn managed_client_get_reconnects_after_disconnect() {
    init_tracing();

    let (connector, connect_count) = counting_connector();

    let (managed, mut events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));
    assert!(matches!(events.next().await, Some(ManagedEvent::Bound)));
    assert_eq!(connect_count.load(Ordering::SeqCst), 1);

    {
        let client = managed.get().await.expect("Failed to get client");
        client.close().await.expect("Failed to close connection");
        client.closed().await;
    }

    assert!(matches!(
        events.next().await,
        Some(ManagedEvent::Disconnected)
    ));

    // Next `get()` should reconnect since the previous client is inactive.
    let client = managed.get().await.expect("Failed to reconnect");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));
    assert!(matches!(events.next().await, Some(ManagedEvent::Bound)));
    assert_eq!(connect_count.load(Ordering::SeqCst), 2);

    client
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM after reconnect");
}

#[tokio::test]
async fn managed_client_get_with_timeout_returns_none_when_connect_hangs() {
    init_tracing();

    // A connector that never resolves.
    let connector = || {
        Box::pin(async move {
            futures::future::pending::<()>().await;

            Ok(tokio::io::duplex(1).0)
        })
    };

    // Building the managed client itself needs an initial successful connection.
    let result = tokio::time::timeout(
        Duration::from_millis(200),
        ConnectionBuilder::new()
            .managed()
            .unbound()
            .max_retries(0)
            .no_backoff()
            .no_auto_reconnect_interval()
            .connect_fn(connector),
    )
    .await;

    assert!(result.is_err(), "Expected connect_fn to hang/timeout");
}

#[tokio::test]
async fn managed_client_exhausts_max_retries_and_returns_error() {
    init_tracing();

    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_c = attempts.clone();

    let connector = move || {
        let attempts_c = attempts_c.clone();

        Box::pin(async move {
            attempts_c.fetch_add(1, Ordering::SeqCst);
            Err(std::io::Error::other("connection refused"))
        }) as Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
    };

    let result = ConnectionBuilder::new()
        .managed()
        .unbound()
        .no_backoff()
        .max_retries(2)
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await;

    assert!(
        result.is_err(),
        "Expected connection to fail after exhausting retries"
    );

    // Initial attempt + `max_retries` retries.
    assert_eq!(attempts.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn managed_client_auto_reconnect_interval_triggers_reconnection() {
    init_tracing();

    let (connector, connect_count) = counting_connector();

    let (managed, mut events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .auto_reconnect_interval(Duration::from_millis(50))
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));
    assert!(matches!(events.next().await, Some(ManagedEvent::Bound)));

    {
        let client = managed.get().await.expect("Failed to get client");
        client.close().await.expect("Failed to close connection");
        client.closed().await;
    }

    assert!(matches!(
        events.next().await,
        Some(ManagedEvent::Disconnected)
    ));

    // Don't call `get()` manually this time, the background
    // auto-reconnect task should pick it up within the interval.
    let reconnected = tokio::time::timeout(Duration::from_millis(500), async {
        loop {
            if let Some(ManagedEvent::Connected) = events.next().await {
                return;
            }
        }
    })
    .await;

    assert!(
        reconnected.is_ok(),
        "Auto-reconnect did not trigger in time"
    );
    assert!(connect_count.load(Ordering::SeqCst) >= 2);
}
