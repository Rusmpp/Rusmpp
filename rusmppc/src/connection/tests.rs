//! Tests in this module test the [`Connection`](crate::Connection)'s functionality based on its internal API.
//!
//! They test some unrealistic scenarios by mocking the underlying framed transport and timers.
//!
//! Bugs found in the [`Connection`](crate::Connection)'s logic should be reproduced here.
//!
//! For tests that simulate real scenarios using the public API, see `tests.rs`.

use std::{
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

use futures::StreamExt;
use rusmpp::{Command, CommandId, CommandStatus, Pdu, pdus::SubmitSm};

use crate::{ConnectionBuilder, error::Error, mock::framed::MockFramed, tests::init_tracing};

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
            Err(Error::ConnectionClosed) => {}
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
