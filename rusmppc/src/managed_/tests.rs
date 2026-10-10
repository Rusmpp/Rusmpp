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
    pdus::{BindTransceiver, BindTransceiverResp, DeliverSm, SubmitSm, SubmitSmResp},
    tokio_codec::CommandCodec,
};
use tokio::io::{AsyncRead, AsyncWrite, DuplexStream};
use tokio_util::codec::Framed;

use crate::{
    ConnectionBuilder,
    managed::{ManagedEvent, ManagedState},
    tests::init_tracing,
};

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

/// Binds successfully, then floods `DeliverSm` events — more than the whole event path can
/// hold — while the public stream is never read. Reports once the flood is written.
async fn flood_server<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(
    stream: S,
    flooded: tokio::sync::oneshot::Sender<()>,
) {
    let mut framed = Framed::new(stream, CommandCodec::new());

    while let Some(Ok(command)) = framed.next().await {
        let pdu: Pdu = match command.id() {
            CommandId::BindTransceiver => BindTransceiverResp::default().into(),
            CommandId::EnquireLink => Pdu::EnquireLinkResp,
            _ => continue,
        };

        let response = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(command.sequence_number())
            .pdu(pdu);

        if framed.send(response).await.is_err() {
            return;
        }

        break;
    }

    for sequence_number in 1..=4096u32 {
        let event = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(Pdu::DeliverSm(DeliverSm::default()));

        if framed.send(event).await.is_err() {
            return;
        }
    }

    let _ = flooded.send(());

    // Keep the connection alive until the client closes it.
    while let Some(Ok(_)) = framed.next().await {}
}

/// Schedule (final-fork2-b, defect 5): leave the public stream unread; flood the event path;
/// lose the connection; `get()` must return a reconnected client.
///
/// Lifecycle delivery must never depend on the consumer draining the public stream:
/// reconnection and binding are state, and an unread stream may not hold the client lock.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_full_event_stream_does_not_stall_reconnection() {
    init_tracing();

    let connections = Arc::new(AtomicUsize::new(0));
    let (flooded_tx, flooded_rx) = tokio::sync::oneshot::channel();
    let flooded_tx = Arc::new(std::sync::Mutex::new(Some(flooded_tx)));

    let connector = {
        let connections = connections.clone();
        let flooded_tx = flooded_tx.clone();

        move || {
            let connections = connections.clone();
            let flooded_tx = flooded_tx.clone();

            Box::pin(async move {
                let n = connections.fetch_add(1, Ordering::SeqCst);

                let (server, client) = tokio::io::duplex(1 << 20);

                if n == 0 {
                    let flooded = flooded_tx
                        .lock()
                        .unwrap()
                        .take()
                        .expect("the flood report belongs to the first connection");

                    tokio::spawn(flood_server(server, flooded));
                } else {
                    tokio::spawn(run_ok_server(server));
                }

                Ok(client)
            })
                as Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
        }
    };

    let (managed, _events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    // The public stream is deliberately never read. Wait until the peer has written its
    // whole flood, then give the relay a moment: the events fill the public channel (the
    // connection's own channel is unbounded upstream, so there is no drop count to read —
    // what matters is that a full backlog of unread events exists when the connection is
    // lost).
    let _ = tokio::time::timeout(Duration::from_secs(5), flooded_rx)
        .await
        .expect("the flood must be written");

    tokio::time::sleep(Duration::from_millis(250)).await;

    let client = managed
        .get()
        .await
        .expect("the first client must be available");

    // The connection is lost.
    client.close().await.expect("Failed to close connection");
    client.closed().await;

    // `get()` must reconnect without waiting for the unread stream to be drained: neither
    // the connection setup nor the lifecycle delivery may depend on the consumer.
    let reconnected = tokio::time::timeout(Duration::from_secs(2), managed.get())
        .await
        .expect("get() must not wait for the public stream to be drained")
        .expect("the managed client must reconnect");

    reconnected
        .submit_sm(SubmitSm::default())
        .await
        .expect("the reconnected client must work");

    assert!(connections.load(Ordering::SeqCst) >= 2);
}

/// The lifecycle is published as state carrying a generation: each successful connection is
/// a new generation, so a reconnect is distinguishable even though the states themselves
/// repeat.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_status_carries_the_generation() {
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

    let status = managed.status();

    assert_eq!(status.state(), ManagedState::Bound);
    assert_eq!(status.generation(), 1);

    // The connection is lost; the public stream observes the end and publishes it.
    {
        let client = managed.get().await.expect("Failed to get client");

        client.close().await.expect("Failed to close connection");
        client.closed().await;
    }

    assert!(matches!(
        events.next().await,
        Some(ManagedEvent::Disconnected)
    ));

    // The status transition is published by the connection's own termination watcher (a
    // separate task), never by the stream: the event reaches the stream's consumer first,
    // so the status needs its scheduling turn — a bounded wait, not a fixed sleep.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);

    while managed.status().state() != ManagedState::Disconnected {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the status never reached Disconnected after the connection was lost"
        );

        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    let status = managed.status();

    assert_eq!(status.generation(), 1);

    // The next connection is a new generation.
    let client = managed.get().await.expect("Failed to reconnect");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));
    assert!(matches!(events.next().await, Some(ManagedEvent::Bound)));

    let status = managed.status();

    assert_eq!(status.state(), ManagedState::Bound);
    assert_eq!(status.generation(), 2);

    client
        .submit_sm(SubmitSm::default())
        .await
        .expect("the reconnected client must work");

    assert_eq!(connect_count.load(Ordering::SeqCst), 2);
}

/// Schedule (final-fork3-b, defect 3; Opus F3): a generation whose stream is only `Pending`
/// — not ended — must not be replaced by the next one when the consumer comes back: its
/// buffered events and its `Disconnected` are lost silently, and nothing counts them.
///
/// Driven white-box, because the two states must be held apart: generation 1's stream has
/// one buffered event and then parks, while generation 2 already waits in the queue.
#[test]
fn a_buffered_generation_is_never_silently_replaced() {
    use super::{Generation, GenerationQueue, ManagedEventsStream};
    use crate::{channel::DefaultEventChannel, event::Event};
    use std::{
        collections::VecDeque,
        task::{Context, Poll},
    };

    let queue = Arc::new(GenerationQueue::<DefaultEventChannel>::new());

    let event = |sequence_number| {
        Event::Incoming(
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::DeliverSm(DeliverSm::default())),
        )
    };

    queue.push(Generation {
        sequence: 1,
        bound: false,
        events: Box::pin(futures::stream::iter(vec![event(11)]).chain(futures::stream::pending())),
    });

    // Generation 2 parks after its event as well, so any Disconnected observed below can
    // only belong to the generation that was replaced.
    queue.push(Generation {
        sequence: 2,
        bound: false,
        events: Box::pin(futures::stream::iter(vec![event(22)]).chain(futures::stream::pending())),
    });

    let mut stream = ManagedEventsStream {
        queue: Arc::clone(&queue),
        current: None,
        pending: VecDeque::new(),
    };

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let mut saw_connected = 0;
    let mut saw_event_11 = false;
    let mut saw_event_22 = false;
    let mut saw_disconnected = false;

    for _ in 0..8 {
        match stream.poll_next_unpin(&mut context) {
            Poll::Ready(Some(ManagedEvent::Connected)) => saw_connected += 1,
            Poll::Ready(Some(ManagedEvent::Event(event))) => match event {
                Event::Incoming(command) if command.sequence_number() == 11 => saw_event_11 = true,
                Event::Incoming(command) if command.sequence_number() == 22 => saw_event_22 = true,
                _ => {}
            },
            Poll::Ready(Some(ManagedEvent::Bound)) => {}
            Poll::Ready(Some(ManagedEvent::Disconnected)) => saw_disconnected = true,
            Poll::Ready(None) => break,
            Poll::Pending => break,
        }
    }

    assert!(
        saw_event_11,
        "the first generation's buffered event must be yielded"
    );

    // Generation 2 taking over while generation 1 is merely parked, with no Disconnected
    // and no counted discard, is exactly the silent replacement: the parked generation's
    // remaining events and its lifecycle transition are gone and nothing says so.
    let silent_replacement = saw_event_22
        && !saw_disconnected
        && queue.dropped_generations().load(Ordering::Relaxed) == 0;

    assert!(
        !silent_replacement,
        "generation 1 was replaced while still parked, losing its events and Disconnected with no count (connected seen {saw_connected} times)"
    );
}

/// The drain order around a generation hand-off (the consult's schedule, second half):
/// after generation 1's buffered event it ends, and its `Disconnected` must come **before**
/// generation 2's lifecycle events — with nothing discarded. A stream that replaced the
/// parked generation would skip both.
#[test]
fn a_generation_drains_completely_before_the_next_one_starts() {
    use super::{Generation, GenerationQueue, ManagedEventsStream};
    use crate::{channel::DefaultEventChannel, event::Event};
    use std::{
        collections::VecDeque,
        task::{Context, Poll},
    };

    let queue = Arc::new(GenerationQueue::<DefaultEventChannel>::new());

    let event = |sequence_number| {
        Event::Incoming(
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::DeliverSm(DeliverSm::default())),
        )
    };

    // Generation 1: one buffered event, then its stream ends. Generation 2 already waits.
    queue.push(Generation {
        sequence: 1,
        bound: true,
        events: Box::pin(futures::stream::iter(vec![event(11)])),
    });

    queue.push(Generation {
        sequence: 2,
        bound: true,
        events: Box::pin(futures::stream::iter(vec![event(22)])),
    });

    let mut stream = ManagedEventsStream {
        queue: Arc::clone(&queue),
        current: None,
        pending: VecDeque::new(),
    };

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let mut seen = Vec::<String>::new();

    loop {
        match stream.poll_next_unpin(&mut context) {
            Poll::Ready(Some(ManagedEvent::Connected)) => seen.push("connected".into()),
            Poll::Ready(Some(ManagedEvent::Bound)) => seen.push("bound".into()),
            Poll::Ready(Some(ManagedEvent::Disconnected)) => seen.push("disconnected".into()),
            Poll::Ready(Some(ManagedEvent::Event(event))) => match event {
                Event::Incoming(command) => {
                    seen.push(format!("event({})", command.sequence_number()))
                }
                _ => panic!("unexpected event"),
            },
            Poll::Ready(None) => break,
            Poll::Pending => break,
        }
    }

    assert_eq!(
        seen,
        [
            "connected",
            "bound",
            "event(11)",
            "disconnected",
            "connected",
            "bound",
            "event(22)",
            // Generation 2's stream ends too; nothing waits on its producer here.
            "disconnected",
        ],
        "generation 1 must drain to its Disconnected before generation 2 starts"
    );

    assert_eq!(
        queue.dropped_generations().load(Ordering::Relaxed),
        0,
        "no generation may be discarded here"
    );
}

/// The hand-off bound is not a comment: generations beyond the cap are discarded from the
/// stalest end and counted (the Opus review found nothing exercised the cap or the counter).
#[test]
fn generations_beyond_the_cap_are_discarded_and_counted() {
    use super::{GENERATIONS_CAP, Generation, GenerationQueue};
    use crate::channel::DefaultEventChannel;

    let queue = GenerationQueue::<DefaultEventChannel>::new();

    let generations = GENERATIONS_CAP as u64 + 3;

    for sequence in 1..=generations {
        queue.push(Generation {
            sequence,
            bound: false,
            events: Box::pin(futures::stream::empty()),
        });
    }

    assert_eq!(
        queue.dropped_generations().load(Ordering::Relaxed),
        3,
        "the stalest generations past the cap must be discarded and counted"
    );

    let mut surviving = Vec::new();

    while let Some(generation) = queue.pop() {
        surviving.push(generation.sequence);
    }

    assert_eq!(
        surviving,
        (4..=generations).collect::<Vec<_>>(),
        "the newest generations survive; the stalest are the ones discarded"
    );
}

/// Schedule (final-fork3-b, defect 4; Opus F1): the lifecycle state must not depend on the
/// consumer reading the event stream. The stream is dropped outright; the connection then
/// ends, and `status()` must say so without ever being polled.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn disconnect_is_published_even_when_the_stream_is_never_read() {
    init_tracing();

    let (connector, _connect_count) = counting_connector();

    let (managed, events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    assert_eq!(managed.status().state(), ManagedState::Bound);

    // The consumer walks away from the events entirely.
    drop(events);

    {
        let client = managed.get().await.expect("Failed to get client");

        client.close().await.expect("Failed to close connection");
        client.closed().await;
    }

    // The end of the connection is state: observable without any consumer.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);

    while managed.status().state() != ManagedState::Disconnected {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the status stayed {:?} after the connection ended, with the stream unread",
            managed.status().state()
        );

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// Schedule (final-fork3-b, defect 5): once the last client is gone and no reconnection can
/// happen, the public stream must end — a consumer awaiting completion can not finish
/// against a stream that parks forever.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_managed_stream_ends_when_no_generation_can_come() {
    init_tracing();

    let (connector, _connect_count) = counting_connector();

    let (managed, mut events) = ConnectionBuilder::new()
        .managed()
        .unbound()
        .no_auto_reconnect_interval()
        .connect_fn(connector)
        .await
        .expect("Failed to build managed client");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));

    // The last handle is dropped: the connection dies with it, and nothing can produce
    // another generation.
    drop(managed);

    let drained = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(event) = events.next().await {
            if matches!(event, ManagedEvent::Disconnected) {
                return true;
            }
        }

        // The stream ended on its own, which is also a clean finish.
        true
    })
    .await;

    assert!(
        matches!(drained, Ok(true)),
        "the stream stalled before reporting the connection's end"
    );

    // Nothing can produce another generation: the stream must end, not park.
    match tokio::time::timeout(Duration::from_secs(2), events.next()).await {
        Ok(None) => {}
        Ok(Some(event)) => panic!("no producer remains, but the stream yielded {event:?}"),
        Err(_) => panic!("the managed stream never ended after the last client was dropped"),
    }
}

/// `publish_status`'s two ordering guards, pinned directly: a publication for a generation
/// older than the published one is **stale**, and a generation whose termination is
/// published is **terminal** — a delayed `Bound` (the bind-completion race) must not
/// resurrect it. Either guard being deleted fails these assertions.
#[test]
fn publish_status_never_moves_backwards() {
    use super::{ManagedState, ManagedStatus, publish_status};
    use tokio::sync::watch;

    let (status, _rx) = watch::channel(ManagedStatus {
        generation: 2,
        state: ManagedState::Bound,
    });

    // A stale generation's termination (generation 1's watcher firing after generation 2
    // is already bound) must not overwrite the newer status.
    publish_status(&status, 1, ManagedState::Disconnected);

    assert_eq!(
        *status.borrow(),
        ManagedStatus {
            generation: 2,
            state: ManagedState::Bound
        },
        "a stale generation's publication must be dropped"
    );

    // And the current generation's termination is terminal: a delayed bind completion must
    // not resurrect it.
    publish_status(&status, 2, ManagedState::Disconnected);

    assert_eq!(
        *status.borrow(),
        ManagedStatus {
            generation: 2,
            state: ManagedState::Disconnected
        }
    );

    publish_status(&status, 2, ManagedState::Bound);

    assert_eq!(
        *status.borrow(),
        ManagedStatus {
            generation: 2,
            state: ManagedState::Disconnected
        },
        "a terminated generation must not be resurrected by a delayed Bound"
    );

    // The ordinary forward transitions still apply.
    let (status, _rx) = watch::channel(ManagedStatus {
        generation: 1,
        state: ManagedState::Connected,
    });

    publish_status(&status, 1, ManagedState::Bound);
    publish_status(&status, 2, ManagedState::Connected);

    assert_eq!(
        *status.borrow(),
        ManagedStatus {
            generation: 2,
            state: ManagedState::Connected
        }
    );

    publish_status(&status, 2, ManagedState::Bound);

    assert_eq!(
        *status.borrow(),
        ManagedStatus {
            generation: 2,
            state: ManagedState::Bound
        }
    );
}

/// A generation that dies before its bind completes ends `Disconnected`, never stuck at
/// `Connected`: the generation's termination watcher is installed before the bind, so a
/// bind failure (the peer is gone) still publishes the transition.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_failed_bind_ends_the_generation_disconnected() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_c = attempts.clone();

    let connector = move || {
        let attempts = attempts_c.clone();

        Box::pin(async move {
            let n = attempts.fetch_add(1, Ordering::SeqCst);

            let (server, client) = tokio::io::duplex(4096);

            if n == 0 {
                tokio::spawn(run_ok_server(server));
            } else {
                // The peer is gone before the bind can complete: the generation comes up
                // and dies at once.
                drop(server);
            }

            Ok(client)
        }) as Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
    };

    let (managed, _events) = ConnectionBuilder::new()
        .managed()
        .transceiver(BindTransceiver::default())
        .auto_reconnect_interval(Duration::from_millis(20))
        .max_retries(0)
        .connect_fn(connector)
        .await
        .expect("the first generation must come up");

    assert_eq!(managed.status().state(), ManagedState::Bound);

    // End generation 1; the reconnect task then brings up generation 2, which dies before
    // its bind.
    {
        let client = managed.get().await.expect("the first generation");

        let _ = client.close().await;
        client.closed().await;
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);

    loop {
        let status = managed.status();

        if status.generation() >= 2 && status.state() == ManagedState::Disconnected {
            break;
        }

        assert!(
            tokio::time::Instant::now() < deadline,
            "a generation that died before its bind left the status at {:?} (generation {})",
            status.state(),
            status.generation()
        );

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// Finding 5's cancellation: the reconnect attempt itself must be cancellable. Generation 1
/// comes up; it dies; the reconnect task starts generation 2's connect, which stalls
/// forever; the last client is dropped — the task must cancel the stalled attempt and exit,
/// which releases its producer and lets the public stream end. Without the inner select the
/// task stays parked in `get()` and the stream never ends.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_last_client_drop_cancels_a_stalled_reconnect() {
    init_tracing();

    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_c = attempts.clone();

    let connector = move || {
        let attempts = attempts_c.clone();

        Box::pin(async move {
            let n = attempts.fetch_add(1, Ordering::SeqCst);

            if n == 0 {
                let (server, client) = tokio::io::duplex(4096);

                tokio::spawn(run_ok_server(server));

                Ok(client)
            } else {
                // The reconnect stalls: this connect never resolves.
                let never: Result<DuplexStream, std::io::Error> = futures::future::pending().await;

                never
            }
        }) as Pin<Box<dyn Future<Output = Result<DuplexStream, std::io::Error>> + Send>>
    };

    let (managed, mut events) = ConnectionBuilder::new()
        .managed()
        .unbound()
        .auto_reconnect_interval(Duration::from_millis(20))
        .max_retries(0)
        .connect_fn(connector)
        .await
        .expect("the first generation must come up");

    assert!(matches!(events.next().await, Some(ManagedEvent::Connected)));

    // End generation 1: the reconnect task then starts generation 2's connect, which
    // stalls forever.
    {
        let client = managed.get().await.expect("the first generation");

        let _ = client.close().await;
        client.closed().await;
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);

    while attempts.load(Ordering::SeqCst) < 2 {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the reconnect attempt never started"
        );

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    // The last client is dropped while that attempt is parked.
    drop(managed);

    // The stream must drain (generation 1's Disconnected) and then end: only a cancelled
    // attempt lets the reconnect task drop the inner it holds.
    let drained = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(event) = events.next().await {
            if matches!(event, ManagedEvent::Disconnected) {
                return true;
            }
        }

        true
    })
    .await;

    assert!(
        matches!(drained, Ok(true)),
        "the stream stalled before reporting the connection's end"
    );

    match tokio::time::timeout(Duration::from_secs(5), events.next()).await {
        Ok(None) => {}
        Ok(Some(event)) => panic!("no producer remains, but the stream yielded {event:?}"),
        Err(_) => panic!("the last-client drop did not cancel the stalled reconnect"),
    }
}
