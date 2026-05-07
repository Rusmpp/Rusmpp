//! An opinionated reconnect logic implementation, since rusmppc does not handle reconnects.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example manual_reconnect
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId,
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{Client, ConnectionBuilder, event::Event};
use tokio::sync::{mpsc, oneshot};

type BoxedError = Box<dyn core::error::Error + Send + Sync>;

struct Factory {
    rx: mpsc::UnboundedReceiver<ConnectedClientRequest>,
}

#[derive(Clone)]
struct FactoryHandle {
    tx: mpsc::UnboundedSender<ConnectedClientRequest>,
}

struct ConnectedClientRequest {
    tx: oneshot::Sender<Client>,
}

impl ConnectedClientRequest {
    fn new() -> (Self, oneshot::Receiver<Client>) {
        let (tx, rx) = oneshot::channel();

        (Self { tx }, rx)
    }
}

impl Factory {
    fn new() -> (Self, FactoryHandle) {
        let (tx, rx) = mpsc::unbounded_channel::<ConnectedClientRequest>();

        (Self { rx }, FactoryHandle { tx })
    }

    fn run(self) {
        tokio::spawn(async move {
            let mut rx = self.rx;

            'outer: loop {
                tracing::info!("Connecting...");

                let (client, mut events) = match ConnectionBuilder::new()
                    .enquire_link_interval(Duration::from_secs(5))
                    .response_timeout(Duration::from_secs(2))
                    .connect("smpp://127.0.0.1:2775")
                    .await
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        tracing::error!(?err, "Failed to connect, retrying in 5 seconds");

                        tokio::time::sleep(Duration::from_secs(5)).await;

                        continue 'outer;
                    }
                };

                client
                    .bind_transceiver(
                        BindTransceiver::builder()
                            .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                            .password(COctetString::from_str("rEZYMq5j")?)
                            .system_type(COctetString::empty())
                            .addr_ton(Ton::Unknown)
                            .addr_npi(Npi::Unknown)
                            .address_range(COctetString::empty())
                            .build(),
                    )
                    .await?;

                'inner: loop {
                    tokio::select! {
                        request = rx.recv() => {
                            match request {
                                None => {
                                    tracing::info!("Factory closed, stopping");
                                    break 'outer;
                                }
                                Some(request) => {
                                    tracing::info!("Received request for connected client");

                                    request.tx.send(client.clone()).ok();
                                }
                            }
                        },
                        event = events.next() => {
                            match event {
                                None => {
                                    tracing::warn!("Connection closed, retrying in 5 seconds");

                                    break 'inner;
                                },
                                Some(event) => {
                                    tracing::info!(?event, "Event");

                                    if let Event::Incoming(command) = event {
                                        if command.id() == CommandId::DeliverSm {
                                            tracing::info!("Received DeliverSm");

                                            let _ = client
                                                .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                                                .await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }

            Ok::<(), BoxedError>(())
        });
    }
}

impl FactoryHandle {
    async fn client(&self) -> Result<Client, BoxedError> {
        let (request, response) = ConnectedClientRequest::new();

        self.tx.send(request)?;

        let client = response.await?;

        Ok(client)
    }
}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    tracing_subscriber::fmt()
        .with_env_filter("manual_reconnect=info,rusmpp=off,rusmppc=debug")
        .init();

    let (factory, handle) = Factory::new();

    factory.run();

    for _ in 0..10 {
        tracing::info!("Requesting connected client");

        // This will block until a connected client is available.
        let client = handle.client().await?;

        tracing::info!("Got connected client");

        let response = client
            .submit_sm(
                SubmitSm::builder()
                    .service_type(ServiceType::default())
                    .source_addr_ton(Ton::Unknown)
                    .source_addr_npi(Npi::Unknown)
                    .source_addr(COctetString::from_str("12345")?)
                    .destination_addr(COctetString::from_str("491701234567")?)
                    .esm_class(EsmClass::default())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .short_message(OctetString::from_str("Hi, I am a short message.")?)
                    .build(),
            )
            .await?;

        tracing::info!(?response, "SubmitSm response");

        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    // Drop the handle to stop the factory.
    drop(handle);

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
