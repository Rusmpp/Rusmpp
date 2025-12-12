//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example rusmppc_submit_sm
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId, CommandStatus,
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Rusmppc produces a lot of logs while managing the SMPP connection in the background.
    // You can filter them out by setting the `rusmppc` target to `off`.
    tracing_subscriber::fmt()
        .with_env_filter("rusmppc_submit_sm=info,rusmpp=trace,rusmppc=debug")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        // Every 5 seconds send an enquire link command to the server.
        .enquire_link_interval(Duration::from_secs(5))
        // If the server does not respond within 2 seconds, consider it a timeout.
        .response_timeout(Duration::from_secs(2))
        // connect to the SMPP server using plain TCP
        // or use `smpps://rusmpps.rusmpp.org:2776` for a TLS connection.
        .connect("smpp://rusmpps.rusmpp.org:2775")
        .await?;

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

    let client_clone = client.clone();

    let events = tokio::spawn(async move {
        // Listen for events like incoming commands and background errors.
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Incoming(command) = event {
                if command.id() == CommandId::DeliverSm {
                    tracing::info!("Received DeliverSm");

                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }

        tracing::info!("Connection closed");
    });

    let submit = SubmitSm::builder()
        .service_type(ServiceType::default())
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        .esm_class(EsmClass::default())
        .registered_delivery(RegisteredDelivery::request_all())
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build();

    tracing::info!("Sending SubmitSm");

    let response = client.submit_sm(submit.clone()).await?;

    tracing::info!(?response, "Got SubmitSmResp");

    // Send a command with a custom timeout.

    tracing::info!("Sending SubmitSm with a custom timeout");

    let response = client
        .response_timeout(Duration::from_secs(30))
        .submit_sm(submit.clone())
        .await?;

    tracing::info!(?response, "Got SubmitSmResp");

    // Send a command without a timeout.

    tracing::info!("Sending SubmitSm without a timeout");

    let response = client
        .no_response_timeout()
        .submit_sm(submit.clone())
        .await?;

    tracing::info!(?response, "Got SubmitSmResp");

    // Send a command without waiting for a response.
    // The response, if any, will be passed to the event stream.

    tracing::info!("Sending SubmitSm without waiting for a response");

    let sequence_number = client.no_wait().submit_sm(submit.clone()).await?;

    tracing::info!(?sequence_number, "Sent SubmitSm");

    // Send a command with a custom status.

    tracing::info!("Sending SubmitSm with a custom status");

    client
        .status(CommandStatus::EsmeRunknownerr) // This error code does not make any sense, but it is just an example.
        .submit_sm(submit.clone())
        .await
        .ok();

    // Wait a little bit to see the incoming events.

    tokio::time::sleep(Duration::from_secs(10)).await;

    tracing::info!("Unbinding from the server");

    client.unbind().await?;

    tracing::info!("Closing the connection");

    client.close().await?;

    tracing::info!("Waiting for the connection to terminate");

    client.closed().await;

    events.await?;

    Ok(())
}
