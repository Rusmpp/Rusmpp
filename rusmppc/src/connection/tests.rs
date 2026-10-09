//! Tests in this module test the [`Connection`](crate::Connection)'s functionality based on its internal API.
//!
//! They test some unrealistic scenarios by mocking the underlying framed transport and timers.
//!
//! Bugs found in the [`Connection`](crate::Connection)'s logic should be reproduced here.
//!
//! For tests that simulate real scenarios using the public API, see `tests.rs`.

use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

use futures::StreamExt;
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{SubmitSm, SubmitSmResp},
};

use crate::{
    ConnectionBuilder,
    error::{Error, NotSentReason},
    event::Event,
    mock::framed::MockFramed,
    tests::init_tracing,
};

pin_project_lite::pin_project! {
    struct PollTraceFuture<F> {
        #[pin]
        future: F,
    }
}

impl<F> PollTraceFuture<F> {
    fn new(future: F) -> Self {
        Self { future }
    }
}

impl<F: Future> Future for PollTraceFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        tracing::info!("Polling future");

        match self.project().future.poll(cx) {
            Poll::Ready(output) => {
                tracing::info!("Future ready");

                Poll::Ready(output)
            }
            Poll::Pending => {
                tracing::info!("Future pending");

                Poll::Pending
            }
        }
    }
}

// RUST_LOG=rusmppc=trace cargo test --package rusmppc --lib -- connection::tests::server_ddos_client_should_still_send_requests_and_connection_should_still_manage_timeouts --exact --nocapture
#[tokio::test]
async fn server_ddos_client_should_still_send_requests_and_connection_should_still_manage_timeouts()
{
    init_tracing();

    let mut framed = MockFramed::new().sink_always_ready_ok();

    // This framed sends an AlertNotification pdu none stop to simulate a server DDOSing the client.
    framed.expect_poll_next_pin().returning(|_ctx| {
        Poll::Ready(Some(Ok(Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(0)
            .pdu(Pdu::AlertNotification(Default::default())))))
    });

    let (client, events, future) = ConnectionBuilder::new()
        .mock_delay()
        // Send an enquire link every 50 polls
        .enquire_link_interval(Duration::from_millis(50))
        // Wait for 5 polls for the enquire link response
        .enquire_link_response_timeout(Duration::from_millis(5))
        .no_spawn()
        .raw(framed);

    tokio::spawn(future);

    for _ in 0..1000 {
        match client
            .no_wait() // Server will not respond anyway, so we don't care about the response
            .submit_sm(SubmitSm::default())
            .await
        {
            Ok(_) => {}
            // After the enquire-link timeout the connection closes: a submit is either
            // refused before it is enqueued (definitely not sent) or was written when the
            // connection died (maybe sent).
            Err(Error::ConnectionClosed)
            | Err(Error::NotSent {
                reason: NotSentReason::ConnectionClosed,
            }) => {}
            Err(err) => {
                panic!("Failed to submit SM: {:?}", err);
            }
        }
    }

    // After the enquire link timeout, the connection should close
    let _ = events.count().await;
}

// RUST_LOG=rusmppc=trace cargo test --package rusmppc --lib -- connection::tests::client_ddos_and_server_ddos_connection_should_still_respond_to_enquire_link_spawned --exact --nocapture
#[tokio::test]
async fn client_ddos_and_server_ddos_connection_should_still_respond_to_enquire_link_spawned() {
    init_tracing();

    let sequence_number = Arc::new(AtomicU32::new(0));

    let sent_enquire_link_sequence_numbers = Arc::new(Mutex::new(Vec::new()));
    let received_enquire_link_response_sequence_number = Arc::new(Mutex::new(Vec::new()));

    let mock_sent_enquire_link_sequence_numbers = sent_enquire_link_sequence_numbers.clone();
    let mock_received_enquire_link_response_sequence_number =
        received_enquire_link_response_sequence_number.clone();

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    // Only send enquire links
    framed.expect_poll_next_pin().returning(move |_ctx| {
        let seq_number = sequence_number.fetch_add(1, Ordering::SeqCst);

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(seq_number)
            .pdu(Pdu::EnquireLink);

        mock_sent_enquire_link_sequence_numbers
            .lock()
            .unwrap()
            .push(seq_number);

        Poll::Ready(Some(Ok(command)))
    });

    // Ignore everything sent to you. except the enquire link response
    framed.expect_start_send_pin().returning(move |command| {
        if let CommandId::EnquireLinkResp = command.id() {
            mock_received_enquire_link_response_sequence_number
                .lock()
                .unwrap()
                .push(command.sequence_number());
        }

        Ok(())
    });

    let (client, events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    tokio::spawn(PollTraceFuture::new(future));

    let mut handles = Vec::new();

    for _ in 0..1000 {
        let client_clone = client.clone();

        let handle = tokio::spawn(async move {
            client_clone
                .no_wait() // Server will not respond anyway, so we don't care about the response
                .submit_sm(SubmitSm::default())
                .await
                .expect("Failed to submit SM");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task panicked");
    }

    client.close().await.expect("Failed to close connection");

    let _ = events.count().await;

    assert_eq!(
        *sent_enquire_link_sequence_numbers.lock().unwrap(),
        *received_enquire_link_response_sequence_number
            .lock()
            .unwrap()
    );
}

// RUST_LOG=rusmppc=trace cargo test --package rusmppc --lib -- connection::tests::client_ddos_and_server_ddos_connection_should_still_respond_to_enquire_link_not_spawned --exact --nocapture
#[tokio::test]
async fn client_ddos_and_server_ddos_connection_should_still_respond_to_enquire_link_not_spawned() {
    init_tracing();

    let sequence_number = Arc::new(AtomicU32::new(0));

    let sent_enquire_link_sequence_numbers = Arc::new(Mutex::new(Vec::new()));
    let received_enquire_link_response_sequence_number = Arc::new(Mutex::new(Vec::new()));

    let mock_sent_enquire_link_sequence_numbers = sent_enquire_link_sequence_numbers.clone();
    let mock_received_enquire_link_response_sequence_number =
        received_enquire_link_response_sequence_number.clone();

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    // Only send enquire links
    framed.expect_poll_next_pin().returning(move |_ctx| {
        let seq_number = sequence_number.fetch_add(1, Ordering::SeqCst);

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(seq_number)
            .pdu(Pdu::EnquireLink);

        mock_sent_enquire_link_sequence_numbers
            .lock()
            .unwrap()
            .push(seq_number);

        Poll::Ready(Some(Ok(command)))
    });

    // Ignore everything sent to you. except the enquire link response
    framed.expect_start_send_pin().returning(move |command| {
        if let CommandId::EnquireLinkResp = command.id() {
            mock_received_enquire_link_response_sequence_number
                .lock()
                .unwrap()
                .push(command.sequence_number());
        }

        Ok(())
    });

    let (client, events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    tokio::spawn(PollTraceFuture::new(future));

    for _ in 0..1000 {
        client
            .no_wait() // Server will not respond anyway, so we don't care about the response
            .submit_sm(SubmitSm::default())
            .await
            .expect("Failed to submit SM");
    }

    client.close().await.expect("Failed to close connection");

    let _ = events.count().await;

    assert_eq!(
        *sent_enquire_link_sequence_numbers.lock().unwrap(),
        *received_enquire_link_response_sequence_number
            .lock()
            .unwrap()
    );
}

// RUST_LOG=rusmppc=trace cargo test --package rusmppc --lib -- connection::tests::sink_first_poll_ready_pending_pending_request_should_be_sent --exact --nocapture
#[tokio::test]
async fn sink_first_poll_ready_pending_pending_request_should_be_sent() {
    init_tracing();

    let mut framed = MockFramed::new()
        .poll_next_always_pending()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    // first poll_ready is pending
    framed.expect_poll_ready_pin().times(1).returning(|cx| {
        cx.waker().wake_by_ref();
        Poll::Pending
    });

    // second poll_ready is ready
    framed
        .expect_poll_ready_pin()
        .times(1)
        .returning(|_cx| Poll::Ready(Ok(())));

    // Assert start send gets submit sm with correct sequence number
    framed
        .expect_start_send_pin()
        .times(1)
        .returning(|command| {
            assert!(matches!(command.pdu(), Some(Pdu::SubmitSm(_))));
            assert_eq!(command.sequence_number(), 1);
            Ok(())
        });

    let (client, events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    tokio::spawn(PollTraceFuture::new(future));

    client
        .no_wait()
        .submit_sm(SubmitSm::default())
        .await
        .expect("Failed to submit SM");

    client.close().await.expect("Failed to close connection");

    let _ = events.count().await;
}

// RUST_LOG=rusmppc=trace cargo test --package rusmppc --lib -- connection::tests::sink_first_poll_flush_pending_pending_request_should_be_sent --exact --nocapture
#[tokio::test]
async fn sink_first_poll_flush_pending_pending_request_should_be_sent() {
    init_tracing();

    let submit_count = 10;

    let mut framed = MockFramed::new()
        .poll_next_always_pending()
        .poll_ready_always_ready_ok()
        .poll_close_always_ready_ok();

    // first poll_flush is pending
    framed.expect_poll_flush_pin().times(1).returning(|cx| {
        cx.waker().wake_by_ref();
        Poll::Pending
    });

    // second poll_flush is ready
    framed
        .expect_poll_flush_pin()
        .returning(|_cx| Poll::Ready(Ok(())));

    for n in 0..submit_count {
        let i = 1 + n * 2;

        // Assert start send gets submit sm with correct sequence number
        framed
            .expect_start_send_pin()
            .times(1)
            .returning(move |command| {
                assert!(matches!(command.pdu(), Some(Pdu::SubmitSm(_))));
                assert_eq!(command.sequence_number(), i);
                Ok(())
            });
    }

    let (client, events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    tokio::spawn(PollTraceFuture::new(future));

    for _ in 0..submit_count {
        client
            .no_wait()
            .submit_sm(SubmitSm::default())
            .await
            .expect("Failed to submit SM");
    }

    client.close().await.expect("Failed to close connection");

    let _ = events.count().await;
}
/// The mock server's read side: commands queued for the connection to read, plus the
/// waker to poke when one is added.
///
/// These tests run the connection as a spawned task, so every wake is real. That matters:
/// tokio's unbounded mpsc hands a full block (32 messages) over on a wake, so a
/// hand-polled receiver under a noop waker stops observing sends after four blocks — a
/// harness that appears to work and then silently strands traffic (measured while
/// writing these tests: the 129th request was sent, `Ok(())`, and never observed by a
/// hand-polled connection; the product path, with real task wakes, is unaffected).
struct MockServer {
    queue: VecDeque<Command>,
    waker: Option<std::task::Waker>,
    /// How many commands the connection has read off the queue.
    read: u32,
}

impl MockServer {
    fn serve(server: &Arc<Mutex<Self>>, command: Command) {
        let mut server = server.lock().unwrap();

        server.queue.push_back(command);

        if let Some(waker) = server.waker.take() {
            waker.wake();
        }
    }
}

/// A mock framed transport with a live read queue: writes are recorded (sequence numbers)
/// and always succeed; reads yield the queue, parking with a stored waker when empty.
/// Waits (with a real deadline) until `condition` holds, yielding between checks.
///
/// A fixed yield budget is not a synchronization primitive: under load the peer task may
/// not be scheduled inside it, so such a wait fails for machine load instead of for the
/// defect it guards. The deadline makes a failure mean "the connection genuinely did not
/// progress".
async fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    while !condition() {
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for: {what}"
        );

        tokio::task::yield_now().await;
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

fn mock_with_server(server: &Arc<Mutex<MockServer>>, written: &Arc<Mutex<Vec<u32>>>) -> MockFramed {
    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let server = server.clone();

        framed.expect_poll_next_pin().returning(move |cx| {
            let mut server = server.lock().unwrap();

            match server.queue.pop_front() {
                Some(command) => {
                    server.read += 1;

                    Poll::Ready(Some(Ok(command)))
                }
                None => {
                    server.waker = Some(cx.waker().clone());

                    Poll::Pending
                }
            }
        });
    }

    framed
}

/// Schedule (final-fork2-b, defect 1): retain request A on sequence 1; seed the allocator so
/// the next request proposes the same number; queue B and cancel it before it is written.
///
/// B's cancellation must never touch A's registration: A's response must still reach A.
/// Removal by sequence number cannot tell B's request from A's, and the wrapped number makes
/// B's cleanup hit A.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancelling_a_wrapped_request_must_not_remove_an_older_live_waiter() {
    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));
    let flush_ready = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    {
        let flush_ready = flush_ready.clone();

        framed.expect_poll_flush_pin().returning(move |_cx| {
            if flush_ready.load(Ordering::SeqCst) == 0 {
                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // Request A is written on sequence 1; its flush stalls, so it holds the sink with its
    // registration live.
    let a = client.submit_sm(SubmitSm::default());
    let mut a = std::pin::pin!(a);

    assert!(a.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // The allocator wraps (seeded): the next proposal is sequence 1 again — the number A
    // still holds.
    client.seed_sequence_number(u32::MAX);

    // Request B is queued behind A and then cancelled before it can be written.
    let b = client.submit_sm(SubmitSm::default());
    let mut b = Box::pin(b);

    assert!(b.as_mut().poll(&mut context).is_pending());

    drop(b);

    // The connection processes B's cancellation while B is still queued.
    assert!(future.as_mut().poll(&mut context).is_pending());

    // A's flush completes and the connection continues.
    flush_ready.store(1, Ordering::SeqCst);

    for _ in 0..10 {
        let _ = future.as_mut().poll(&mut context);
    }

    // B must never have been written (its caller is gone).
    assert_eq!(
        written.lock().unwrap().as_slice(),
        &[1],
        "a request cancelled before it was written must not be transmitted"
    );

    // A's response arrives: it must reach A. B's cancellation carried no ownership of A's
    // sequence number, so A's registration is intact.
    queue.lock().unwrap().push_back(
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
    );

    let mut polls = 0;

    loop {
        if let Poll::Ready(result) = a.as_mut().poll(&mut context) {
            match result {
                Ok(response) => assert_eq!(response, SubmitSmResp::default()),
                other => panic!(
                    "the live waiter must receive its own response, got {other:?} \
                     (a cancelled wrapped request removed it)"
                ),
            }

            break;
        }

        let _ = future.as_mut().poll(&mut context);

        polls += 1;

        assert!(polls < 100, "the live waiter never received its response");
    }
}

/// Schedule (final-fork2-b, defect 2): a raw registered request whose response is decoded
/// while the flush is pending; the stream then ends before the caller is polled again.
///
/// The write stage must resolve with the confirmed response. A closed acknowledgement
/// channel is not evidence that nothing was sent past a response that already arrived.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_raw_request_keeps_a_confirmed_response_when_the_stream_ends() {
    init_tracing();

    let sent_sequence = Arc::new(AtomicU32::new(0));
    let served = Arc::new(AtomicU32::new(0));
    let eof = Arc::new(AtomicU32::new(0));
    let flush_ready = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let sent_sequence = sent_sequence.clone();

        framed.expect_start_send_pin().returning(move |item| {
            sent_sequence.store(item.sequence_number(), Ordering::SeqCst);

            Ok(())
        });
    }

    {
        let sent_sequence = sent_sequence.clone();
        let served = served.clone();
        let eof = eof.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if served.swap(1, Ordering::SeqCst) == 1 {
                if eof.load(Ordering::SeqCst) == 1 {
                    return Poll::Ready(None);
                }

                return Poll::Pending;
            }

            Poll::Ready(Some(Ok(Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sent_sequence.load(Ordering::SeqCst))
                .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))))
        });
    }

    {
        let flush_ready = flush_ready.clone();

        framed.expect_poll_flush_pin().returning(move |_cx| {
            if flush_ready.load(Ordering::SeqCst) == 0 {
                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // The raw send's write stage: the request is written (flush pending) and the peer's
    // response is decoded in the same poll.
    let send = client.raw().send(SubmitSm::default());
    let mut send = std::pin::pin!(send);

    assert!(send.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(sent_sequence.load(Ordering::SeqCst), 1);

    // The stream ends before the caller is polled again.
    eof.store(1, Ordering::SeqCst);

    assert!(future.as_mut().poll(&mut context).is_ready());

    match send.as_mut().poll(&mut context) {
        Poll::Ready(Ok((sequence_number, response))) => {
            assert_eq!(sequence_number, 1);

            let mut response = std::pin::pin!(response);

            match response.as_mut().poll(&mut context) {
                Poll::Ready(Ok(response)) => {
                    assert!(matches!(response.pdu(), Some(Pdu::SubmitSmResp(_))));
                    assert_eq!(response.sequence_number(), 1);
                }
                Poll::Ready(Err(error)) => panic!(
                    "the response future must resolve with the confirmed response, got {error:?}"
                ),
                Poll::Pending => panic!("the response future never resolved"),
            }
        }
        Poll::Ready(Err(error)) => panic!(
            "the confirmed response must survive the end of the stream, got {error:?} \
             (the closed acknowledgement channel discarded it)"
        ),
        Poll::Pending => panic!("the write stage never resolved"),
    }
}

/// Schedule (final-fork2-b, defect 3): the sink is not ready when the request is popped; the
/// caller is cancelled while the request waits; the sink then becomes ready.
///
/// A request cancelled before `start_send` must never be transmitted: it was already claimed
/// for writing before the sink accepted it, and the requeue lost the cancellation check.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_request_cancelled_while_the_sink_is_not_ready_is_never_written() {
    init_tracing();

    let sink_ready = Arc::new(AtomicU32::new(0));
    let written = Arc::new(Mutex::new(Vec::<u32>::new()));

    let mut framed = MockFramed::new()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok()
        .poll_next_always_pending();

    {
        let sink_ready = sink_ready.clone();

        framed.expect_poll_ready_pin().returning(move |cx| {
            if sink_ready.load(Ordering::SeqCst) == 0 {
                cx.waker().wake_by_ref();

                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        });
    }

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // The request is queued and then popped while the sink is not ready: it goes back on the
    // queue untouched.
    let mut submit = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(submit.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert!(
        written.lock().unwrap().is_empty(),
        "the sink is not ready: nothing may be written yet"
    );

    // The caller is cancelled while the request waits on the not-ready sink. Dropping the
    // future (not a `Pin` of it) is the cancellation: the guard abandons the request's cell
    // and queues the cleanup hint.
    drop(submit);

    assert!(future.as_mut().poll(&mut context).is_pending());

    // The sink becomes ready.
    sink_ready.store(1, Ordering::SeqCst);

    for _ in 0..10 {
        let _ = future.as_mut().poll(&mut context);
    }

    assert!(
        written.lock().unwrap().is_empty(),
        "a request cancelled before start_send must not be transmitted"
    );
}

/// Blocks the connection's thread at the pre-fix collision-refusal log, so the caller can be
/// polled inside the refusal window (the schedule's "pause at the existing collision log"):
/// the waiter's sender is already dropped there, and the refusal has not been acknowledged.
///
/// The fix deletes the refusal — colliding proposals are renumbered at the write gate — so
/// the log never fires and this subscriber never blocks.
struct PauseOnRefusal {
    release: Mutex<std::sync::mpsc::Receiver<()>>,
}

impl PauseOnRefusal {
    fn new(release: std::sync::mpsc::Receiver<()>) -> Self {
        Self {
            release: Mutex::new(release),
        }
    }
}

struct MessageField(Option<String>);

impl tracing::field::Visit for MessageField {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = Some(format!("{value:?}"));
        }
    }
}

impl tracing::Subscriber for PauseOnRefusal {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

    fn event(&self, event: &tracing::Event<'_>) {
        let mut message = MessageField(None);

        event.record(&mut message);

        if let Some(message) = message.0
            && message.contains("refusing the request")
        {
            // The window: the refused request's response sender is gone, and the refusal is
            // not yet acknowledged. Blocking here lets the caller be polled inside it.
            let _ = self
                .release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(5));
        }
    }

    fn enter(&self, _span: &tracing::span::Id) {}

    fn exit(&self, _span: &tracing::span::Id) {}
}

/// Schedule (final-fork2-b, defect 4): retain waiter 1; queue another request proposing 1.
/// The refusal window (the waiter's sender already dropped, the collision acknowledgement not
/// yet sent) surfaced `ConnectionClosed` to the caller instead of a renumbered send.
///
/// The request must be written with a free sequence number and complete normally — the
/// refusal path does not exist after the write gate assigns sequence numbers.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_colliding_request_is_written_with_a_free_number_not_refused() {
    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let server = Arc::new(Mutex::new(MockServer {
        queue: VecDeque::new(),
        waker: None,
        read: 0,
    }));

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(mock_with_server(&server, &written));

    // The connection runs on its own thread with the refusal pause installed for that thread
    // only: the window inside the connection's poll must be observable from the caller.
    let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        tracing::subscriber::with_default(PauseOnRefusal::new(release_rx), || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("the runtime must build");

            runtime.block_on(future);
        });
    });

    // Request A is written on sequence 1 and never answered: its registration stays live.
    let a = tokio::spawn({
        let client = client.clone();

        async move { client.submit_sm(SubmitSm::default()).await }
    });

    wait_until("request A written", || !written.lock().unwrap().is_empty()).await;

    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // The allocator wraps (seeded): the next request proposes sequence 1 — A's number.
    client.seed_sequence_number(u32::MAX);

    let b = tokio::spawn({
        let client = client.clone();

        async move { client.submit_sm(SubmitSm::default()).await }
    });

    // Serve B's response as soon as it is written (renumbered to a free number after the
    // fix; the refused request never reaches the peer before it).
    {
        let written = written.clone();
        let server = server.clone();

        tokio::spawn(async move {
            wait_until("request B written", || written.lock().unwrap().len() >= 2).await;

            let sequence_number = written.lock().unwrap().get(1).copied();

            if let Some(sequence_number) = sequence_number {
                MockServer::serve(
                    &server,
                    Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
                );
            }
        });
    }

    let result = tokio::time::timeout(Duration::from_secs(3), b)
        .await
        .expect("the colliding request must resolve")
        .expect("the task must not panic");

    // Release the pause if it engaged (only the pre-fix code can).
    let _ = release_tx.send(());

    match result {
        Ok(response) => assert_eq!(response, SubmitSmResp::default()),
        Err(error) => {
            panic!("a colliding sequence number must be skipped, not refused, got {error:?}")
        }
    }

    assert_eq!(
        written.lock().unwrap().as_slice(),
        &[1, 3],
        "the colliding proposal must be renumbered to a free sequence number"
    );

    drop(a);
}

/// The verdict matrix, state one: a request that is still waiting in the write queue when
/// its response timeout elapses is **definitely not sent** — the write gate refuses an
/// abandoned request — and the caller is told so (`NotSent`), not handed a maybe-sent
/// timeout.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_response_timeout_on_a_queued_request_is_not_sent() {
    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_close_always_ready_ok()
        .poll_next_always_pending();

    // The flush never completes: the first request holds the sink, so the second request
    // stays queued for as long as the test needs.
    framed
        .expect_poll_flush_pin()
        .returning(|_cx| Poll::Pending);

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    tokio::spawn(future);

    // Request A is written; its flush stalls forever.
    let a = tokio::spawn({
        let client = client.clone();

        async move { client.submit_sm(SubmitSm::default()).await }
    });

    for _ in 0..100_000 {
        if !written.lock().unwrap().is_empty() {
            break;
        }

        tokio::task::yield_now().await;
    }

    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // Request B queues behind A's stalled flush. Its response timeout elapses while B is
    // still queued: it never reached the transport, and it never will.
    let result = client
        .response_timeout(Duration::from_millis(50))
        .submit_sm(SubmitSm::default())
        .await;

    match result {
        Err(Error::NotSent {
            reason: NotSentReason::Timeout,
        }) => {}
        other => panic!(
            "a request that timed out while still queued must be reported as not sent, got {other:?}"
        ),
    }

    assert_eq!(
        written.lock().unwrap().as_slice(),
        &[1],
        "the queued request must not have been transmitted"
    );

    let pending_response = client
        .pending_responses()
        .await
        .expect("Failed to get pending responses");

    assert!(
        !pending_response.contains(&3),
        "the queued request must not have been registered"
    );

    drop(a);
}

/// The verdict matrix, state two: a request that is still waiting in the write queue when
/// the connection ends is **definitely not sent** — and one that had been handed to the
/// transport in the same teardown is only **maybe sent**. The state cell tells them apart.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_request_queued_when_the_connection_ends_is_not_sent_and_a_written_one_may_be() {
    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let eof = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_close_always_ready_ok();

    // The write is handed to the sink but its flush never completes: A stays written (and
    // unflushed), B stays queued.
    framed
        .expect_poll_flush_pin()
        .returning(|_cx| Poll::Pending);

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let eof = eof.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if eof.load(Ordering::SeqCst) == 1 {
                return Poll::Ready(None);
            }

            Poll::Pending
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // Request A is written (flush pending); request B queues behind it.
    let mut a = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(a.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    let mut b = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(b.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // The stream ends: the connection terminates with A written and B still queued.
    eof.store(1, Ordering::SeqCst);

    assert!(future.as_mut().poll(&mut context).is_ready());

    match b.as_mut().poll(&mut context) {
        Poll::Ready(Err(Error::NotSent {
            reason: NotSentReason::ConnectionClosed,
        })) => {}
        other => panic!(
            "a request queued when the connection ends must be reported as not sent, got {other:?}"
        ),
    }

    match a.as_mut().poll(&mut context) {
        Poll::Ready(Err(Error::ConnectionClosed)) => {}
        other => panic!("a written request whose connection ends may be sent, got {other:?}"),
    }
}

/// Schedule (consult-fork3, finding 2): the reservation table is bounded by **admission**,
/// never by eviction. Fill it with written requests, abandon them all — every one stays
/// unresolved, holding its sequence number — and the next application write is refused with
/// a `NotSent` capacity verdict **before** the gate: nothing is assigned, registered or
/// written. The oldest reserved number's late reply then arrives and must surface as a late
/// event, never satisfying another waiter.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_full_reservation_table_refuses_new_writes_before_the_gate() {
    use futures::StreamExt;
    use rusmpp::types::COctetString;
    use std::str::FromStr;

    init_tracing();

    let cap = super::UNRESOLVED_REQUESTS_CAP;

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let server = Arc::new(Mutex::new(MockServer {
        queue: VecDeque::new(),
        waker: None,
        read: 0,
    }));

    let (client, mut events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(mock_with_server(&server, &written));

    // The connection runs as a task (real wakes): a hand-polled connection under a noop
    // waker stops observing a long queue after a few blocks of its channel.
    tokio::spawn(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // Every request is written and never answered, so abandoning it leaves an unresolved
    // registration holding its sequence number.
    let mut requests = Vec::with_capacity(cap);

    for _ in 0..cap {
        let mut request = Box::pin(client.submit_sm(SubmitSm::default()));

        assert!(request.as_mut().poll(&mut context).is_pending());

        requests.push(request);
    }

    wait_until("every request written", || {
        written.lock().unwrap().len() >= cap
    })
    .await;

    // Abandon them all: each one stays unresolved, holding its sequence number.
    drop(requests);

    // The allocator wraps onto the oldest reserved number: the next request proposes it.
    client.seed_sequence_number(u32::MAX);

    let mut refused = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(refused.as_mut().poll(&mut context).is_pending());

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    let refusal = loop {
        if let Poll::Ready(result) = refused.as_mut().poll(&mut context) {
            break result;
        }

        assert!(
            std::time::Instant::now() < deadline,
            "the refused request must resolve"
        );

        tokio::task::yield_now().await;
    };

    match refusal {
        Err(Error::NotSent {
            reason:
                NotSentReason::Capacity {
                    reserved,
                    cap: bound,
                },
        }) => {
            assert_eq!(reserved, cap);
            assert_eq!(bound, cap);
        }
        other => panic!(
            "a write at the reservation bound must be refused with a capacity verdict, got {other:?}"
        ),
    }

    assert_eq!(
        written.lock().unwrap().len(),
        cap,
        "the refused request must not have been written"
    );

    let pending_response = client
        .pending_responses()
        .await
        .expect("Failed to get pending responses");

    assert_eq!(
        pending_response.len(),
        cap,
        "no reservation may be released or evicted to make room"
    );

    // Protocol traffic that takes no reservation keeps flowing at capacity: an
    // application-sent response PDU is written, never refused (it never enters the table,
    // so the bound must not count or refuse it).
    let before = written.lock().unwrap().len();

    client
        .deliver_sm_resp(1, rusmpp::pdus::DeliverSmResp::default())
        .await
        .expect("a response PDU must not be refused at the reservation bound");

    wait_until("the response PDU written", || {
        written.lock().unwrap().len() > before
    })
    .await;

    assert_eq!(
        written.lock().unwrap().len(),
        before + 1,
        "the response PDU must have been written at the reservation bound"
    );

    // The oldest reserved number's late reply arrives (its caller is long gone): it must
    // surface as a late event — the refused request never registered under that number, and
    // nothing else may take it.
    MockServer::serve(
        &server,
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSmResp(
                SubmitSmResp::builder()
                    .message_id(COctetString::from_str("OLDEST").expect("a short id"))
                    .build(),
            )),
    );

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    let late = loop {
        match events.poll_next_unpin(&mut context) {
            Poll::Ready(Some(Event::Incoming(command))) => break command,
            Poll::Ready(other) => panic!("unexpected event: {other:?}"),
            Poll::Pending => {
                assert!(
                    std::time::Instant::now() < deadline,
                    "the abandoned request's late reply must surface as a late event"
                );

                tokio::task::yield_now().await;
            }
        }
    };

    assert_eq!(
        late.sequence_number(),
        1,
        "the late event must be the old request's reply"
    );

    assert_eq!(
        client.late_responses_dropped(),
        0,
        "the late reply was delivered"
    );

    // And the reservation is claimed by its own reply: the table is one smaller, and the
    // next write is admitted.
    let before = written.lock().unwrap().len();

    let mut admitted = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(admitted.as_mut().poll(&mut context).is_pending());

    wait_until("the next write admitted", || {
        written.lock().unwrap().len() > before
    })
    .await;

    drop(admitted);
}

/// A poll that leaves a loop on its quota must schedule another poll: the queue can still
/// hold work, and an idle server schedules nothing — so without the wakeup the remaining
/// actions strand until unrelated traffic arrives. (The same rule covers the sink loop's
/// write quota.)
#[test]
fn hitting_the_actions_poll_quota_should_schedule_another_poll() {
    struct CountingWake(std::sync::atomic::AtomicUsize);

    impl std::task::Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            let _ = self.0.fetch_add(1, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            let _ = self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    // An idle server that schedules nothing: `Pending` WITHOUT waking (the helper
    // `poll_next_always_pending` wakes by ref, which would mask the bug).
    framed
        .expect_poll_next_pin()
        .returning(|_ctx| Poll::Pending);

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    // More no-op actions than one poll's quota (`ACTIONS_POLL_LIMIT`).
    for _ in 0..(super::ACTIONS_POLL_LIMIT as usize + 1) {
        assert!(client.is_active(), "the ping must be queued");
    }

    let wakes = Arc::new(CountingWake(std::sync::atomic::AtomicUsize::new(0)));
    let waker = std::task::Waker::from(wakes.clone());
    let mut context = Context::from_waker(&waker);

    assert!(future.as_mut().poll(&mut context).is_pending());
    assert!(
        wakes.0.load(Ordering::SeqCst) >= 1,
        "leaving the actions loop on its quota with work still queued must schedule another poll"
    );
}

/// Schedule (final-fork3-b, defect 1): a request whose caller abandoned it while the
/// response was still owed must not swallow the response into a receiver nobody will read
/// again. The reproduced schedule pauses between the abandonment and the receiver's
/// destruction; this test holds those two moments apart directly, with the connection built
/// from raw parts so the request's own cell and outcome receiver stay in the test's hands.
#[test]
fn a_response_delivered_after_the_abandon_is_surfaced_as_a_late_event() {
    use crate::{
        AbandonOutcome, Action,
        channel::DefaultEventChannel,
        event::Event,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    let (connection, _watch, actions, mut events, _late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let armed = Arc::new(AtomicU32::new(0));
    let served = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_start_send_always_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let armed = armed.clone();
        let served = served.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            // The answer is served exactly once, and only once the test has abandoned the
            // request: the schedule pauses between the abandonment and the receiver's
            // destruction, and the answer lands in that window.
            if armed.load(Ordering::SeqCst) == 0 || served.swap(1, Ordering::SeqCst) == 1 {
                return Poll::Pending;
            }

            Poll::Ready(Some(Ok(Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(1)
                .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))))
        });
    }

    let mut connection = std::pin::pin!(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // The request, with its outcome receiver kept alive by this test.
    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(Pdu::SubmitSm(SubmitSm::default()));

    let (request, mut outcome) = RegisteredRequest::new(1, command);
    let cell = request.cell.clone();

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    // The connection writes it.
    assert!(connection.as_mut().poll(&mut context).is_pending());

    // The write acknowledgement is already in the channel (the write completed); the
    // response is what must never follow it into a receiver the caller walked away from.
    assert!(matches!(
        outcome.try_recv(),
        Ok(crate::Outcome::Written { sequence_number: 1 })
    ));

    // The caller abandons it — the cell, synchronously, exactly as the guard or the
    // response timeout does (nothing is stored yet, so the abandonment takes nothing) —
    // and the cleanup hint follows.
    assert!(
        cell.abandon_or_take().is_none(),
        "no response is committed yet: nothing is taken"
    );

    assert!(matches!(
        cell.resolution(),
        AbandonOutcome::Written { sequence_number: 1 }
    ));

    let _ = actions.send(Action::Cancel(1));

    // The peer's answer is decoded only now: the receiver is still alive (this test holds
    // it), which is the window the reproduced schedule pauses in.
    armed.store(1, Ordering::SeqCst);

    let _ = connection.as_mut().poll(&mut context);

    // The answer must surface as a late reply on the event stream, never be left in a
    // receiver the caller has already walked away from.
    match events.poll_next_unpin(&mut context) {
        Poll::Ready(Some(Event::Incoming(command))) => {
            assert_eq!(command.sequence_number(), 1);

            // Nothing else was buffered for the abandoned receiver: it is empty, or
            // already closed (its last sender went with the claimed registration) — a
            // response left there would still surface as `Ok` here.
            assert!(
                matches!(
                    outcome.try_recv(),
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty)
                        | Err(tokio::sync::mpsc::error::TryRecvError::Disconnected)
                ),
                "the abandoned receiver must not have swallowed the response"
            );
        }
        other => {
            panic!("the abandoned request's response must surface as a late event, got {other:?}")
        }
    }
}

/// The write gate's cell check, pinned alone (consult-fork3): the sink is held pending, the
/// caller's cell is abandoned, and **no cleanup action is ever sent** — then the sink
/// becomes ready. The gate must refuse the request on the cell alone: zero writes, zero
/// registrations. The action-based tests can not pin this (a processed cancel masks the
/// gate), so the hint is withheld entirely.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_write_gate_refuses_an_abandoned_cell_without_any_cancel_hint() {
    use crate::{
        Action, PendingResponses,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    init_tracing();

    let sink_ready = Arc::new(AtomicU32::new(0));
    let written = Arc::new(Mutex::new(Vec::<u32>::new()));

    let (connection, _watch, actions, _events, _late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let mut framed = MockFramed::new()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok()
        .poll_next_always_pending();

    {
        let sink_ready = sink_ready.clone();

        framed.expect_poll_ready_pin().returning(move |cx| {
            if sink_ready.load(Ordering::SeqCst) == 0 {
                cx.waker().wake_by_ref();

                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        });
    }

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    let mut connection = std::pin::pin!(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(Pdu::SubmitSm(SubmitSm::default()));

    let (request, _outcome) = RegisteredRequest::new(1, command);
    let cell = request.cell.clone();

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    // The gate runs while the sink is not ready: the request is requeued, untouched.
    assert!(connection.as_mut().poll(&mut context).is_pending());
    assert!(written.lock().unwrap().is_empty());

    // The caller abandons it. No `Action::Cancel` is ever sent: the gate alone must refuse
    // the request, by the cell.
    assert!(
        cell.abandon_or_take().is_none(),
        "no response is committed: the abandonment takes nothing"
    );

    // The sink becomes ready and the gate runs for the requeued request.
    sink_ready.store(1, Ordering::SeqCst);

    assert!(connection.as_mut().poll(&mut context).is_pending());

    assert!(
        written.lock().unwrap().is_empty(),
        "an abandoned cell must be refused at the write gate, hint or no hint"
    );

    // Registrations too, not just writes: the sequence number must stay unreserved.
    let (pending_responses, ack) = PendingResponses::new();

    actions
        .send(Action::PendingResponses(pending_responses))
        .expect("the connection is alive");

    assert!(connection.as_mut().poll(&mut context).is_pending());

    let pending = ack
        .await
        .expect("the connection must ack the pending-responses request")
        .expect("pending responses");

    assert!(
        pending.is_empty(),
        "an abandoned request must not be registered, got {pending:?}"
    );
}

/// The reservation's guarantee, pinned: a number a written-but-abandoned request still owes
/// an answer for is not reused by a new request — so that answer can never be delivered to
/// the new caller. The mutation that skips only non-abandoned entries at the gate survives
/// every other test; this one kills it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_tombstoned_number_is_not_reused_by_a_new_request() {
    use futures::StreamExt;
    use rusmpp::types::COctetString;
    use std::str::FromStr;

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (client, mut events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // Request A is written on sequence 1 and then abandoned: an unresolved registration
    // holds 1.
    let mut a = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(a.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    drop(a);

    assert!(future.as_mut().poll(&mut context).is_pending());

    // Request B proposes 1 again (the allocator wraps): the reservation must make the gate
    // skip it, so B is written with a different number.
    client.seed_sequence_number(u32::MAX);

    let mut b = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(b.as_mut().poll(&mut context).is_pending());

    for _ in 0..10 {
        let _ = future.as_mut().poll(&mut context);
    }

    assert_eq!(
        written.lock().unwrap().as_slice(),
        &[1, 3],
        "a reserved number must not be reused by a new request"
    );

    // A's late answer for 1, and B's own answer for 3, distinguishable by message id.
    queue.lock().unwrap().push_back(
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSmResp(
                SubmitSmResp::builder()
                    .message_id(COctetString::from_str("LATE-A").expect("a short id"))
                    .build(),
            )),
    );

    queue.lock().unwrap().push_back(
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(3)
            .pdu(Pdu::SubmitSmResp(
                SubmitSmResp::builder()
                    .message_id(COctetString::from_str("B").expect("a short id"))
                    .build(),
            )),
    );

    let mut polls = 0;

    loop {
        if let Poll::Ready(result) = b.as_mut().poll(&mut context) {
            match result {
                Ok(response) => assert_eq!(
                    response.message_id,
                    COctetString::from_str("B").expect("a short id"),
                    "the new request must never receive the abandoned request's answer"
                ),
                other => panic!("the new request must resolve with its own answer: {other:?}"),
            }

            break;
        }

        let _ = future.as_mut().poll(&mut context);

        polls += 1;

        assert!(polls < 100, "the new request never resolved");
    }

    // A's late answer surfaces as an event instead.
    let mut polls = 0;

    loop {
        if let Poll::Ready(Some(crate::event::Event::Incoming(command))) =
            events.poll_next_unpin(&mut context)
        {
            assert_eq!(command.sequence_number(), 1);

            break;
        }

        let _ = future.as_mut().poll(&mut context);

        polls += 1;

        assert!(
            polls < 100,
            "the abandoned request's late answer never surfaced as an event"
        );
    }
}

/// P3 (consult-fork3): a request that could not even be enqueued has written no bytes, so
/// every send path reports it as `NotSent` — never a plain closed-connection outcome that a
/// caller could mistake for a maybe-sent failure.
#[tokio::test]
async fn a_failed_enqueue_is_reported_as_not_sent() {
    init_tracing();

    let framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok()
        .poll_next_always_pending();

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    // The connection is gone (never polled, dropped): its action channel is closed, so
    // every enqueue fails.
    drop(future);

    match client.submit_sm(SubmitSm::default()).await {
        Err(Error::NotSent {
            reason: NotSentReason::ConnectionClosed,
        }) => {}
        other => panic!("a submit that was never enqueued must be not-sent, got {other:?}"),
    }

    match client
        .deliver_sm_resp(1, rusmpp::pdus::DeliverSmResp::default())
        .await
    {
        Err(Error::NotSent {
            reason: NotSentReason::ConnectionClosed,
        }) => {}
        other => panic!("a response that was never enqueued must be not-sent, got {other:?}"),
    }

    match client.raw().send(Pdu::SubmitSm(SubmitSm::default())).await {
        Err(Error::NotSent {
            reason: NotSentReason::ConnectionClosed,
        }) => {}
        // The Ok variant holds an opaque future, so it can not be formatted.
        Ok(_) => panic!("a raw send that was never enqueued must be not-sent"),
        Err(other) => panic!("a raw send that was never enqueued must be not-sent, got {other:?}"),
    }
}

/// The late lane's actual delivery, end to end: a response committed to the cell of a
/// caller that never read it, and then dropped, must reach the application as exactly one
/// late event **through the lane**. (The cell-level test only pins that the drop *takes*
/// the response; a `forward_late` that is a no-op keeps it green and loses the response
/// here.)
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_stored_response_left_by_a_dropped_caller_is_delivered_through_the_late_lane() {
    use futures::StreamExt;

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (client, mut events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    let mut future = std::pin::pin!(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    // The request is written, and its response is committed to its cell while the caller
    // never polls again.
    let mut submit = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(submit.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    queue.lock().unwrap().push_back(
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
    );

    assert!(future.as_mut().poll(&mut context).is_pending());

    // The caller is dropped without ever consuming the response: the guard takes it out of
    // the cell and hands it to the connection's late lane.
    drop(submit);

    // The lane's delivery: the next poll of the connection surfaces it as an incoming
    // event, exactly once.
    assert!(future.as_mut().poll(&mut context).is_pending());

    match events.poll_next_unpin(&mut context) {
        Poll::Ready(Some(crate::event::Event::Incoming(command))) => {
            assert_eq!(command.sequence_number(), 1);
        }
        other => {
            panic!("the stored response must be delivered through the late lane, got {other:?}")
        }
    }

    assert!(
        matches!(events.poll_next_unpin(&mut context), Poll::Pending),
        "the response must be delivered exactly once"
    );

    assert_eq!(
        client.late_responses_dropped(),
        0,
        "the delivery must not be counted as a drop"
    );
}

/// The lane's failure path, counted: a caller that drops after its connection is gone has
/// nothing left to deliver its stored response — the forward fails, and the loss is the
/// explicit `late_responses_dropped` count.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_late_response_with_no_connection_left_is_counted() {
    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (client, _events, future) = ConnectionBuilder::new()
        .mock_delay()
        .no_enquire_link_interval()
        .no_spawn()
        .raw(framed);

    // Boxed: this test drops the connection future itself (a `pin!` borrow would not).
    let mut future = Box::pin(future);

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let mut submit = Box::pin(client.submit_sm(SubmitSm::default()));

    assert!(submit.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_pending());

    queue.lock().unwrap().push_back(
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
    );

    assert!(future.as_mut().poll(&mut context).is_pending());

    // The connection task is gone before the caller drops: the lane has no receiver left.
    drop(future);

    drop(submit);

    assert_eq!(
        client.late_responses_dropped(),
        1,
        "a late response that can no longer be delivered must be counted"
    );
}

/// Schedule (rr-fork-b3, finding 1): six committed late replies are forwarded into the
/// lane while the connection is not polled; EOF then arrives on the next poll. Every
/// accepted late message must surface — a per-poll quota that drains five and leaves the
/// sixth to be discarded by teardown is a silent loss.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn every_accepted_late_reply_survives_termination() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };
    use futures::StreamExt;

    init_tracing();

    const N: u32 = 6;

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));
    let eof = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();
        let eof = eof.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if eof.load(Ordering::SeqCst) == 1 {
                return Poll::Ready(None);
            }

            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (connection, _watch, actions, mut events, _late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    // Boxed: this test drops the connection itself (a `pin!` borrow would not).
    let mut connection = Box::pin(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let mut cells = Vec::new();
    // Keep the outcome receivers alive: the commit's notification must land (a closed
    // channel would fail it) — the payload is in the cell either way.
    let mut receivers = Vec::new();

    // Six requests, each written and then answered while the caller never reads it.
    for i in 0..N {
        let sequence_number = i * 2 + 1;

        let (request, outcome) = RegisteredRequest::new(
            u64::from(i),
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::SubmitSm(SubmitSm::default())),
        );

        cells.push(request.cell.clone());

        actions
            .send(Action::registered_request(request))
            .expect("the connection is alive");

        assert!(connection.as_mut().poll(&mut context).is_pending());

        queue.lock().unwrap().push_back(
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
        );

        receivers.push(outcome);
    }

    // Let every response commit (the stream stage serves a bounded number per poll).
    for _ in 0..4 {
        let _ = connection.as_mut().poll(&mut context);
    }

    assert_eq!(written.lock().unwrap().len(), N as usize);

    // Now the callers walk away: each drop takes its stored response and forwards it into
    // the lane — all six while the connection is not being polled.
    for cell in &cells {
        let command = cell
            .abandon_or_take()
            .expect("each caller's stored response must be taken on its drop");

        cell.forward_late(command);
    }

    // EOF arrives on the next poll: a bounded number of late replies surface, and the rest
    // are the teardown sweep's to deliver (the per-poll quota never abandons them).
    eof.store(1, Ordering::SeqCst);

    let _ = connection.as_mut().poll(&mut context);

    drop(connection);

    let mut late = Vec::new();

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    while late.len() < N as usize {
        assert!(
            std::time::Instant::now() < deadline,
            "every accepted late reply must surface before termination, got {late:?}"
        );

        // The reads consume the test task's cooperative budget too: yield so it resets.
        tokio::task::yield_now().await;

        match events.poll_next_unpin(&mut context) {
            Poll::Ready(Some(crate::event::Event::Incoming(command))) => {
                late.push(command.sequence_number());
            }
            // The terminal error event rides the same stream; not what this test counts.
            Poll::Ready(Some(_)) => {}
            Poll::Ready(None) => break,
            Poll::Pending => {}
        }
    }

    assert_eq!(
        late.len(),
        N as usize,
        "every accepted late reply must surface before termination, got {late:?}"
    );

    // Keep the receivers alive to the end of the test.
    drop(receivers);
}

/// Schedule (rr-fork-b3, finding 1, teardown half): a late reply that lands in the lane
/// after the connection's last poll — accepted, never drained — must still be surfaced
/// when the connection is dropped, as long as the event stream can take it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_late_lane_is_swept_at_teardown() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };
    use futures::StreamExt;

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let serve = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let serve = serve.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if serve.swap(0, Ordering::SeqCst) == 1 {
                return Poll::Ready(Some(Ok(Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))));
            }

            Poll::Pending
        });
    }

    let (connection, _watch, actions, mut events, late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let mut connection = Box::pin(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let (request, _outcome) = RegisteredRequest::new(
        0,
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSm(SubmitSm::default())),
    );
    let cell = request.cell.clone();

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    assert!(connection.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // The response commits on the next poll.
    serve.store(1, Ordering::SeqCst);

    assert!(connection.as_mut().poll(&mut context).is_pending());

    // The caller walks away: its stored reply is forwarded into the lane, and no poll
    // follows — only the teardown sweep can surface it.
    let command = cell
        .abandon_or_take()
        .expect("the stored response must be taken on the drop");

    cell.forward_late(command);

    drop(connection);

    match events.poll_next_unpin(&mut context) {
        Poll::Ready(Some(crate::event::Event::Incoming(command))) => {
            assert_eq!(command.sequence_number(), 1);
        }
        other => panic!("the swept late reply must surface at teardown, got {other:?}"),
    }

    assert_eq!(
        late_dropped.load(Ordering::Relaxed),
        0,
        "a swept reply the stream can take is a delivery, not a drop"
    );
}

/// The other half of the sweep: when the event stream is already gone, the reply that
/// teardown finds can not be surfaced — it is counted, never silent.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_late_reply_swept_without_a_stream_is_counted() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let serve = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    let served = Arc::new(AtomicU32::new(0));

    {
        let serve = serve.clone();
        let served = served.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if serve.swap(0, Ordering::SeqCst) == 1 {
                let n = served.fetch_add(1, Ordering::SeqCst);

                return Poll::Ready(Some(Ok(Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(n * 2 + 1)
                    .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))));
            }

            Poll::Pending
        });
    }

    let (connection, _watch, actions, events, late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None, Duration::from_secs(5), true
    );

    let mut connection = Box::pin(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    const N: u32 = 3;

    let mut cells = Vec::new();
    let mut receivers = Vec::new();

    for i in 0..N {
        let sequence_number = i * 2 + 1;

        let (request, outcome) = RegisteredRequest::new(
            u64::from(i),
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::SubmitSm(SubmitSm::default())),
        );

        cells.push(request.cell.clone());
        receivers.push(outcome);

        actions
            .send(Action::registered_request(request))
            .expect("the connection is alive");

        assert!(connection.as_mut().poll(&mut context).is_pending());

        serve.store(1, Ordering::SeqCst);

        assert!(connection.as_mut().poll(&mut context).is_pending());
    }

    for cell in &cells {
        let command = cell
            .abandon_or_take()
            .expect("each stored response must be taken on the drop");

        cell.forward_late(command);
    }

    // The consumer is gone before the connection: the sweep can not surface the replies.
    drop(events);

    drop(connection);

    assert_eq!(
        late_dropped.load(Ordering::Relaxed),
        N as u64,
        "every late reply with no stream left must be counted once at teardown"
    );

    drop(receivers);
}

/// A test event channel that runs a closure when the sink's `events` field is dropped.
///
/// The deterministic instrument for the teardown window (rr-fork-d): `EventSink`'s `Drop`
/// BODY closes the lane and sweeps it, and only then are the struct's fields dropped in
/// declaration order — `events` (the channel, arbitrary user code) BEFORE `late` (the lane
/// handle). A closure run from this channel's drop therefore lands exactly after the close,
/// with no threads and no timing: the window a producer's forward could race, and it must
/// be rejected at the sender and counted — never accepted and then discarded.
struct TeardownWindowChannel {
    inner: crate::event_::DefaultEventChannel,
    on_drop: Option<Box<dyn FnOnce() + Send>>,
}

/// How the probe hands its closure to `TeardownWindowChannel::new` (the `EventChannel`
/// trait constructs the channel itself, so it takes no extra arguments). Only the probe
/// test installs and consumes it; one test, one construction.
static TEARDOWN_WINDOW_HOOK: Mutex<Option<Box<dyn FnOnce() + Send>>> = Mutex::new(None);

impl crate::event_::EventChannel for TeardownWindowChannel {
    type Event = Event;

    fn new(events: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self {
        Self {
            inner: <crate::event_::DefaultEventChannel as crate::event_::EventChannel>::new(events),
            on_drop: TEARDOWN_WINDOW_HOOK.lock().unwrap().take(),
        }
    }

    fn send_error(
        &self,
        error: Error,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        self.inner.send_error(error)
    }

    fn send_incoming(
        &self,
        command: Command,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        self.inner.send_incoming(command)
    }

    fn send_insight(
        &self,
        insight: crate::event::Insight,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        self.inner.send_insight(insight)
    }
}

impl Drop for TeardownWindowChannel {
    fn drop(&mut self) {
        if let Some(hook) = self.on_drop.take() {
            hook();
        }
    }
}

/// Schedule (rr-fork-d): a forward that lands after the sweep's last `try_recv` — but
/// before the lane's receiver is destroyed — must fail at the SENDER and be counted
/// there; it must not be accepted into a queue that is then discarded.
///
/// Without closing the receiver first, the send succeeds (nothing is closed, the receiver
/// still exists), the message is queued, and the receiver's own destruction discards it:
/// zero delivered, zero counted. The window is not theoretical — `events` drops before
/// `late`, so any code the channel runs in its drop is scheduled exactly there; this probe
/// runs the caller's own forward from that drop.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_forward_racing_the_teardown_sweep_is_counted_not_discarded() {
    use crate::{
        Action,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let serve = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let serve = serve.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if serve.swap(0, Ordering::SeqCst) == 1 {
                Poll::Ready(Some(Ok(Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))))
            } else {
                Poll::Pending
            }
        });
    }

    // The caller's forward, run from the channel's drop — i.e. inside the sink's field
    // destruction, after the sweep's last `try_recv`. The cell is not known yet when the
    // channel is constructed (the request comes later), so the closure reads it from a
    // slot the test fills in after the write gate installed the late handle.
    let cell_slot: Arc<Mutex<Option<crate::request::RequestCell>>> = Arc::new(Mutex::new(None));

    {
        let slot = Arc::clone(&cell_slot);

        TEARDOWN_WINDOW_HOOK
            .lock()
            .unwrap()
            .replace(Box::new(move || {
                if let Some(cell) = slot.lock().unwrap().clone() {
                    if let Some(command) = cell.abandon_or_take() {
                        cell.forward_late(command);
                    }
                }
            }));
    }

    let (connection, _watch, actions, mut events, late_dropped) = crate::connection::Connection::<
        (),
        TeardownWindowChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let mut connection = Box::pin(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let (request, _outcome) = RegisteredRequest::new(
        0,
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSm(SubmitSm::default())),
    );
    let cell = request.cell.clone();

    *cell_slot.lock().unwrap() = Some(cell.clone());

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    assert!(connection.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    // The response commits to the cell; the caller never consumes it, and no poll follows
    // the drop — the teardown is the next thing that runs, and the forward happens inside
    // it.
    serve.store(1, Ordering::SeqCst);

    assert!(connection.as_mut().poll(&mut context).is_pending());

    drop(connection);

    assert_eq!(
        late_dropped.load(Ordering::Relaxed),
        1,
        "the forward the sweep could not see must be rejected at the sender and counted, exactly once"
    );
    assert!(
        matches!(events.poll_next_unpin(&mut context), Poll::Ready(None)),
        "the reply must not ALSO be delivered: rejected-and-counted and delivered-anyway are mutually exclusive"
    );
}

/// One late reply with the given sequence number (the lane tests' fixture).
fn late_command(sequence_number: u32) -> Command {
    Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(sequence_number)
        .pdu(Pdu::SubmitSmResp(SubmitSmResp::default()))
}

/// A waker that counts its wakes (the lane's wake-ordering tests).
#[derive(Default)]
struct CountingWake {
    wakes: std::sync::atomic::AtomicUsize,
}

impl std::task::Wake for CountingWake {
    fn wake(self: Arc<Self>) {
        self.wakes.fetch_add(1, Ordering::SeqCst);
    }
}

/// A waker that forwards another late command when woken — the reentrancy probe: the wake
/// runs after the producing enqueue RELEASED the lane lock, so the nested forward must take
/// it again without deadlocking.
struct ForwardingWake {
    handle: crate::request::LateHandle,
    extra: Mutex<Option<Command>>,
}

impl std::task::Wake for ForwardingWake {
    fn wake(self: Arc<Self>) {
        if let Some(command) = self.extra.lock().unwrap().take() {
            self.handle.forward(command);
        }
    }
}

/// A test event channel that runs a callback when an incoming event is surfaced — the
/// delivery-side reentrancy probe. The callback runs from the sink's teardown loop, i.e.
/// OUTSIDE the lane lock (delivery is user code); this channel's callback forwards a late
/// command, which must therefore be rejected-and-counted, not deadlocked.
struct CallbackChannel {
    inner: crate::event_::DefaultEventChannel,
    on_incoming: Mutex<Option<Box<dyn Fn() + Send>>>,
}

impl crate::event_::EventChannel for CallbackChannel {
    type Event = Event;

    fn new(events: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self {
        Self {
            inner: <crate::event_::DefaultEventChannel as crate::event_::EventChannel>::new(events),
            on_incoming: Mutex::new(None),
        }
    }

    fn send_error(
        &self,
        error: Error,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        self.inner.send_error(error)
    }

    fn send_incoming(
        &self,
        command: Command,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        // Take the callback OUT of the lock before calling it: the callback re-enters the
        // lane, never this mutex.
        let callback = self.on_incoming.lock().unwrap().take();
        if let Some(callback) = callback {
            callback();
        }
        self.inner.send_incoming(command)
    }

    fn send_insight(
        &self,
        insight: crate::event::Insight,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Self::Event>> {
        self.inner.send_insight(insight)
    }
}

/// Schedule (consult-fork-latelane-ruling, test 1): the acceptance/publication boundary,
/// pinned deterministically inside the real lane.
///
/// A producer pauses under the lane's mutex AFTER observing `closed == false` and BEFORE
/// the push (the equivalent of the window tokio's own increment-before-push split hides in
/// `send`). The closer's `try_lock` probe must observe that contention, and the resumed
/// producer's command is then the closer's responsibility: delivered by the real teardown
/// sweep — or counted exactly once when the event channel refuses it. Zero deliveries and
/// zero counts is the failure this pins away.
#[test]
fn a_forward_paused_in_the_lane_window_is_delivered_or_counted() {
    for refuse in [false, true] {
        let (entered_tx, entered_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let release_rx = std::sync::Mutex::new(release_rx);

        let mut lane = crate::connection::LateLane::new();
        lane.set_test_hooks(
            None,
            Some(Box::new(move || {
                entered_tx.send(()).expect("the test is alive");
                release_rx
                    .lock()
                    .unwrap()
                    .recv()
                    .expect("the closer releases the producer");
            })),
        );
        let lane = Arc::new(lane);
        let dropped = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let handle = crate::request::LateHandle::new(Arc::clone(&lane), Arc::clone(&dropped));

        let producer = std::thread::spawn(move || handle.forward(late_command(1)));

        entered_rx
            .recv()
            .expect("the producer must reach the window");

        // The closer observes CONTENTION: the producer holds the lane lock at the window.
        assert!(
            lane.state.try_lock().is_err(),
            "the producer must hold the lane lock at the window"
        );

        release_tx.send(()).expect("the producer is alive");
        producer.join().expect("the producer completes");

        // The real teardown sweep: open channel = delivery, closed = refusal.
        let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();

        let sink = crate::connection::EventSink {
            events: <crate::event_::DefaultEventChannel as crate::event_::EventChannel>::new(
                events_tx,
            ),
            late: Arc::clone(&lane),
            late_responses_dropped: Arc::clone(&dropped),
        };

        if refuse {
            drop(events_rx);
            drop(sink);

            assert_eq!(
                dropped.load(Ordering::Relaxed),
                1,
                "a refused delivery is counted exactly once"
            );
        } else {
            let mut events_rx = events_rx;

            drop(sink);

            assert!(
                events_rx.try_recv().is_ok(),
                "the accepted late reply must be delivered by the sweep"
            );
            assert_eq!(
                dropped.load(Ordering::Relaxed),
                0,
                "a delivery is not a drop"
            );
        }
    }
}

/// Schedule (consult-fork-latelane-ruling, test 2): CLOSE WINS a race decided at the lock.
/// The producer pauses BEFORE acquiring the lock; the teardown completes first (close +
/// take); the resumed producer acquires, observes `closed`, and the rejection is counted
/// exactly once at the sender.
#[test]
fn close_wins_over_a_forward_paused_before_the_lock() {
    let (entered_tx, entered_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let release_rx = std::sync::Mutex::new(release_rx);

    let mut lane = crate::connection::LateLane::new();
    lane.set_test_hooks(
        Some(Box::new(move || {
            entered_tx.send(()).expect("the test is alive");
            release_rx
                .lock()
                .unwrap()
                .recv()
                .expect("the closer releases the producer");
        })),
        None,
    );
    let lane = Arc::new(lane);
    let dropped = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let handle = crate::request::LateHandle::new(Arc::clone(&lane), Arc::clone(&dropped));

    let producer = std::thread::spawn(move || handle.forward(late_command(1)));

    entered_rx
        .recv()
        .expect("the producer pauses before the lock");
    let taken = lane.close_and_take();
    assert!(taken.is_empty(), "nothing was accepted before the close");

    release_tx.send(()).expect("the producer is alive");
    producer.join().expect("the producer completes");

    assert_eq!(
        dropped.load(Ordering::Relaxed),
        1,
        "close wins: the resumed forward observes closed and counts exactly one rejection"
    );
}

/// Schedule (consult-fork-latelane-ruling, test 3): the wake contract — enqueue before
/// registration, after registration, and after an empty inspection; waker replacement; the
/// five-per-pass quota with its `more` signal; and waker release at close. Registration
/// order cannot lose a command, and an empty pass never self-wakes.
#[test]
fn the_lane_wakes_correctly_across_registration_orders() {
    let lane = crate::connection::LateLane::new();
    let first = Arc::new(CountingWake::default());

    // Enqueue BEFORE any registration: no waker to wake, and the command is still there
    // for the next inspection.
    assert!(lane.enqueue(late_command(1)));
    assert_eq!(
        first.wakes.load(Ordering::SeqCst),
        0,
        "no waker was registered yet"
    );
    lane.waker()
        .register(&std::task::Waker::from(Arc::clone(&first)));
    let (batch, more) = lane.take_up_to(5);
    assert_eq!(
        batch.len(),
        1,
        "the pre-registration enqueue is delivered by the next inspection"
    );
    assert!(!more);

    // Enqueue WITH a registered waker: woken exactly once, after the unlock.
    assert!(lane.enqueue(late_command(2)));
    assert_eq!(first.wakes.load(Ordering::SeqCst), 1);

    // Waker replacement: the newest registration is the one woken.
    let second = Arc::new(CountingWake::default());
    lane.waker()
        .register(&std::task::Waker::from(Arc::clone(&second)));
    assert!(lane.enqueue(late_command(3)));
    assert_eq!(second.wakes.load(Ordering::SeqCst), 1);
    assert_eq!(
        first.wakes.load(Ordering::SeqCst),
        1,
        "the replaced waker is not woken again"
    );

    let (batch, more) = lane.take_up_to(5);
    assert_eq!(batch.len(), 2);
    assert!(!more);

    // Six queued: five on the first pass with `more` set (the connection self-wakes on it),
    // then one.
    for n in 10..16 {
        assert!(lane.enqueue(late_command(n)));
    }
    let (batch, more) = lane.take_up_to(5);
    assert_eq!(batch.len(), 5);
    assert!(more, "more remain: the connection self-wakes");
    let (batch, more) = lane.take_up_to(5);
    assert_eq!(batch.len(), 1);
    assert!(!more);

    // An empty pass reports nothing to do (and never self-wakes).
    let (batch, more) = lane.take_up_to(5);
    assert!(batch.is_empty() && !more);

    // Waker release at close: the stored waker is taken and dropped.
    lane.waker()
        .register(&std::task::Waker::from(Arc::clone(&second)));
    assert_eq!(Arc::strong_count(&second), 2, "test + registered");
    assert!(lane.close_and_take().is_empty());
    assert_eq!(
        Arc::strong_count(&second),
        1,
        "close releases the stored waker"
    );

    // And a post-close enqueue is rejected for the sender to count.
    assert!(!lane.enqueue(late_command(99)));
}

/// Schedule (consult-fork-latelane-ruling, test 4a): a waker that forwards another command
/// re-enters the lane AFTER the producing enqueue released its lock — it must complete
/// without deadlock and both commands must be deliverable.
#[test]
fn a_waker_that_forwards_completes_without_deadlock() {
    let lane = Arc::new(crate::connection::LateLane::new());
    let dropped = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let handle = crate::request::LateHandle::new(Arc::clone(&lane), Arc::clone(&dropped));

    let forwarding = Arc::new(ForwardingWake {
        handle: handle.clone(),
        extra: Mutex::new(Some(late_command(2))),
    });
    lane.waker()
        .register(&std::task::Waker::from(Arc::clone(&forwarding)));

    handle.forward(late_command(1));

    let (batch, more) = lane.take_up_to(10);
    assert_eq!(
        batch.len(),
        2,
        "the forwarded (nested) command is enqueued as well"
    );
    assert!(!more);
    assert_eq!(dropped.load(Ordering::Relaxed), 0);
}

/// Schedule (consult-fork-latelane-ruling, test 4b): an event callback that forwards a late
/// reply WHILE the sink's teardown is delivering. Delivery runs outside the lane lock, so
/// the nested forward must not deadlock; the lane is already closed by then, so it is
/// rejected and counted exactly once.
#[test]
fn an_event_callback_that_forwards_during_delivery_is_counted() {
    let lane = Arc::new(crate::connection::LateLane::new());
    let dropped = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let handle = crate::request::LateHandle::new(Arc::clone(&lane), Arc::clone(&dropped));

    // One reply waiting for the sweep.
    assert!(lane.enqueue(late_command(1)));

    let (events_tx, mut events_rx) = tokio::sync::mpsc::unbounded_channel();
    let channel = CallbackChannel {
        inner: <crate::event_::DefaultEventChannel as crate::event_::EventChannel>::new(events_tx),
        on_incoming: Mutex::new(Some(Box::new(move || handle.forward(late_command(2))))),
    };

    let sink = crate::connection::EventSink {
        events: channel,
        late: Arc::clone(&lane),
        late_responses_dropped: Arc::clone(&dropped),
    };
    drop(sink);

    assert!(events_rx.try_recv().is_ok(), "the first reply is delivered");
    assert_eq!(
        dropped.load(Ordering::Relaxed),
        1,
        "the callback's nested forward is rejected (closed) and counted exactly once"
    );
}

/// Schedule (consult-fork-latelane-ruling, test 5): under concurrency,
/// `delivered + late_dropped == forwarded` with no duplicate deliveries. Eight producers
/// race a bounded-batch drainer; the drained set plus the closed lane's final take must
/// account for every forwarded command exactly once.
#[test]
fn under_concurrency_delivered_plus_dropped_equals_forwarded() {
    use std::collections::HashSet;

    let lane = Arc::new(crate::connection::LateLane::new());
    let dropped = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let delivered: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::new()));

    const PRODUCERS: u32 = 8;
    const PER_PRODUCER: u32 = 250;

    let done = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    std::thread::scope(|scope| {
        for tid in 0..PRODUCERS {
            let lane = Arc::clone(&lane);
            let dropped = Arc::clone(&dropped);
            let done = Arc::clone(&done);

            scope.spawn(move || {
                let handle = crate::request::LateHandle::new(lane, dropped);

                for i in 0..PER_PRODUCER {
                    handle.forward(late_command(tid * PER_PRODUCER + i));
                }

                done.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Drain concurrently with the producers.
        while done.load(Ordering::SeqCst) < PRODUCERS as usize {
            let (batch, _more) = lane.take_up_to(64);

            let mut seen = delivered.lock().unwrap();
            for command in batch {
                assert!(
                    seen.insert(command.sequence_number()),
                    "no duplicate deliveries"
                );
            }

            std::thread::yield_now();
        }
    });

    // Producers are joined (the scope exited); take whatever is left.
    loop {
        let (batch, more) = lane.take_up_to(64);

        let mut seen = delivered.lock().unwrap();
        for command in batch {
            assert!(
                seen.insert(command.sequence_number()),
                "no duplicate deliveries"
            );
        }
        drop(seen);

        if !more {
            break;
        }
    }

    let delivered = delivered.lock().unwrap().len() as u64;
    let dropped = dropped.load(Ordering::Relaxed);

    assert_eq!(
        delivered + dropped,
        u64::from(PRODUCERS) * u64::from(PER_PRODUCER),
        "every forwarded reply is delivered or counted, exactly once"
    );
}

/// Schedule (rr-fork-b3, finding 2): a reply whose request was already abandoned and whose
/// surfacing fails must be counted — the abandoned branch may not swallow the loss.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_abandoned_reply_whose_surfacing_fails_is_counted() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    init_tracing();

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let serve = Arc::new(AtomicU32::new(0));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let serve = serve.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            if serve.swap(0, Ordering::SeqCst) == 1 {
                Poll::Ready(Some(Ok(Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())))))
            } else {
                Poll::Pending
            }
        });
    }

    let (connection, _watch, actions, events, late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None, Duration::from_secs(5), true
    );

    let mut connection = std::pin::pin!(connection.with_framed(&mut framed));

    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);

    let (request, _outcome) = RegisteredRequest::new(
        0,
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(Pdu::SubmitSm(SubmitSm::default())),
    );
    let cell = request.cell.clone();

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    // The request is written, and the caller abandons it.
    assert!(connection.as_mut().poll(&mut context).is_pending());
    assert_eq!(written.lock().unwrap().as_slice(), &[1]);

    assert!(
        cell.abandon_or_take().is_none(),
        "no response is committed yet: nothing is taken"
    );

    // The consumer is gone: surfacing the abandoned request's reply can not succeed.
    drop(events);

    // The response arrives for the abandoned request: the connection takes the Err path,
    // and the failed surfacing must be counted.
    serve.store(1, Ordering::SeqCst);

    assert!(connection.as_mut().poll(&mut context).is_pending());

    assert_eq!(
        late_dropped.load(Ordering::Relaxed),
        1,
        "an abandoned request's reply that can not be surfaced must be counted"
    );
}

/// Schedule (rr-fork-c, finding 1): the teardown sweep must not treat cooperative-budget
/// exhaustion as the end of the lane. Two hundred committed replies are forwarded into the
/// lane unpolled; the drop's sweep may drain only the first budget's worth unless it
/// receives synchronously — and everything it skips is silently lost, uncounted.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_sweep_beyond_the_cooperative_budget_loses_nothing() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };
    use futures::StreamExt;

    init_tracing();

    const N: u32 = 200;

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (connection, _watch, actions, mut events, late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let mut connection = Box::pin(connection.with_framed(framed));

    // The test task's real waker: a noop waker starves the tokio mpsc handoff after a few
    // blocks, and these tests queue hundreds of actions.
    let waker = futures::future::poll_fn(|cx| Poll::Ready(cx.waker().clone())).await;
    let mut context = Context::from_waker(&waker);

    let mut cells = Vec::new();
    let mut receivers = Vec::new();

    // One request per poll: a hand-polled connection under a noop waker only sees the
    // first blocks of a long channel burst (tokio hands messages over in blocks on wake).
    for i in 0..N {
        let sequence_number = i * 2 + 1;

        let (request, outcome) = RegisteredRequest::new(
            u64::from(i),
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::SubmitSm(SubmitSm::default())),
        );

        cells.push(request.cell.clone());
        receivers.push(outcome);

        actions
            .send(Action::registered_request(request))
            .expect("the connection is alive");

        assert!(connection.as_mut().poll(&mut context).is_pending());

        // The poll consumes this task's cooperative budget; the yield lets it reset, or
        // the connection's channel reads start returning Pending after 128 iterations.
        tokio::task::yield_now().await;
    }

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    assert_eq!(written.lock().unwrap().len(), N as usize);

    for i in 0..N {
        queue.lock().unwrap().push_back(
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(i * 2 + 1)
                .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
        );
    }

    while cells
        .iter()
        .any(|cell| cell.stored_response_sequence_number().is_none())
    {
        assert!(
            std::time::Instant::now() < deadline,
            "every response must commit"
        );

        assert!(connection.as_mut().poll(&mut context).is_pending());

        tokio::task::yield_now().await;
    }

    // Every caller walks away: all N replies land in the lane, and nothing polls the
    // connection again.
    for cell in &cells {
        let command = cell
            .abandon_or_take()
            .expect("each committed response must be taken");

        cell.forward_late(command);
    }

    drop(connection);

    let mut late = Vec::new();

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    while late.len() < N as usize {
        assert!(
            std::time::Instant::now() < deadline,
            "every swept reply must surface at teardown, whatever the budget (got {})",
            late.len()
        );

        // The reads consume the test task's cooperative budget too: yield so it resets.
        tokio::task::yield_now().await;

        match events.poll_next_unpin(&mut context) {
            Poll::Ready(Some(Event::Incoming(command))) => late.push(command.sequence_number()),
            Poll::Ready(Some(_)) => {}
            Poll::Ready(None) => break,
            Poll::Pending => {}
        }
    }

    assert_eq!(
        late.len(),
        N as usize,
        "every swept reply must surface at teardown, whatever the budget"
    );

    assert_eq!(
        late_dropped.load(Ordering::Relaxed),
        0,
        "a swept reply the stream takes is a delivery, never a drop"
    );

    drop(receivers);
}

/// Schedule (rr-fork-c, finding 2): a large late backlog must not starve the rest of the
/// connection. Two hundred replies sit in the lane and one request waits in the action
/// queue: if a poll's late drain can spend the whole cooperative budget, the action queue
/// is never read and the write never happens — the drain must yield to the other stages.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_late_backlog_never_starves_the_write_queue() {
    use crate::{
        Action,
        channel::DefaultEventChannel,
        mock::{delay::MockDelay, runtime::MockRuntime},
        request::RegisteredRequest,
        runtime::Tokio,
    };

    init_tracing();

    const N: u32 = 200;

    let written = Arc::new(Mutex::new(Vec::<u32>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<Command>::new()));

    let mut framed = MockFramed::new()
        .poll_ready_always_ready_ok()
        .poll_flush_always_ready_ok()
        .poll_close_always_ready_ok();

    {
        let written = written.clone();

        framed.expect_start_send_pin().returning(move |item| {
            written.lock().unwrap().push(item.sequence_number());

            Ok(())
        });
    }

    {
        let queue = queue.clone();

        framed.expect_poll_next_pin().returning(move |_cx| {
            match queue.lock().unwrap().pop_front() {
                Some(command) => Poll::Ready(Some(Ok(command))),
                None => Poll::Pending,
            }
        });
    }

    let (connection, _watch, actions, _events, _late_dropped) = crate::connection::Connection::<
        (),
        DefaultEventChannel,
        MockRuntime<MockDelay, Tokio>,
    >::new(
        None,
        Duration::from_secs(5),
        true,
    );

    let mut connection = Box::pin(connection.with_framed(framed));

    // The test task's real waker: a noop waker starves the tokio mpsc handoff after a few
    // blocks, and these tests queue hundreds of actions.
    let waker = futures::future::poll_fn(|cx| Poll::Ready(cx.waker().clone())).await;
    let mut context = Context::from_waker(&waker);

    let mut cells = Vec::new();
    let mut receivers = Vec::new();

    // One request per poll: a hand-polled connection under a noop waker only sees the
    // first blocks of a long channel burst (tokio hands messages over in blocks on wake).
    for i in 0..N {
        let sequence_number = i * 2 + 1;

        let (request, outcome) = RegisteredRequest::new(
            u64::from(i),
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(Pdu::SubmitSm(SubmitSm::default())),
        );

        cells.push(request.cell.clone());
        receivers.push(outcome);

        actions
            .send(Action::registered_request(request))
            .expect("the connection is alive");

        assert!(connection.as_mut().poll(&mut context).is_pending());

        // The poll consumes this task's cooperative budget; the yield lets it reset, or
        // the connection's channel reads start returning Pending after 128 iterations.
        tokio::task::yield_now().await;
    }

    let deadline = std::time::Instant::now() + Duration::from_secs(10);

    assert_eq!(written.lock().unwrap().len(), N as usize);

    for i in 0..N {
        queue.lock().unwrap().push_back(
            Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(i * 2 + 1)
                .pdu(Pdu::SubmitSmResp(SubmitSmResp::default())),
        );
    }

    while cells
        .iter()
        .any(|cell| cell.stored_response_sequence_number().is_none())
    {
        assert!(
            std::time::Instant::now() < deadline,
            "every response must commit"
        );

        assert!(connection.as_mut().poll(&mut context).is_pending());

        tokio::task::yield_now().await;
    }

    // All N replies land in the lane; one fresh request waits in the action queue.
    for cell in &cells {
        let command = cell
            .abandon_or_take()
            .expect("each committed response must be taken");

        cell.forward_late(command);
    }

    let queued_sequence_number = N * 2 + 1;

    let (request, _outcome) = RegisteredRequest::new(
        u64::from(N) + 1,
        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(queued_sequence_number)
            .pdu(Pdu::SubmitSm(SubmitSm::default())),
    );

    actions
        .send(Action::registered_request(request))
        .expect("the connection is alive");

    // Two polls later the queued write must be on the wire: the late backlog may delay
    // nothing but itself.
    for _ in 0..2 {
        let _ = connection.as_mut().poll(&mut context);
    }

    assert!(
        written.lock().unwrap().contains(&queued_sequence_number),
        "a large late backlog must not starve the write queue, wrote {:?}",
        written.lock().unwrap()
    );

    drop(receivers);
}
