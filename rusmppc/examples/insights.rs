//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example insights
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    pdus::{BindTransceiver, SubmitSm},
    types::{COctetString, OctetString},
};
use rusmppc::{ConnectionBuilder, Insight, InsightEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("insights=info,rusmpp=off,rusmppc=off")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        // Every 3 seconds send an enquire link command to the server.
        .enquire_link_interval(Duration::from_secs(3))
        .events()
        // Receive insights like sent and received enquire link commands in the background.
        .insights()
        // connect to the SMPP server using plain TCP
        // or use `smpps://rusmpps.rusmpp.org:2776` for a TLS connection.
        .connect("smpp://rusmpps.rusmpp.org:2775")
        .await?;

    client
        .bind_transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .build(),
        )
        .await?;

    let events = tokio::spawn(async move {
        // Listen for events like incoming commands, insights and background errors.
        while let Some(event) = events.next().await {
            match event {
                InsightEvent::Incoming(command) => {
                    tracing::info!(?command, "Incoming command");
                }
                InsightEvent::Insight(Insight::SentEnquireLink(sequence_number)) => {
                    tracing::info!(%sequence_number, "Sent EnquireLink");
                }
                InsightEvent::Insight(Insight::ReceivedEnquireLinkResp(sequence_number)) => {
                    tracing::info!(%sequence_number, "Received EnquireLinkResp");
                }
                InsightEvent::Insight(Insight::ReceivedEnquireLink(sequence_number)) => {
                    tracing::info!(%sequence_number, "Received EnquireLink");
                }
                InsightEvent::Insight(Insight::SentEnquireLinkResp(sequence_number)) => {
                    tracing::info!(%sequence_number, "Sent EnquireLinkResp");
                }
                InsightEvent::Error(error) => {
                    tracing::error!(?error, "Connection error");
                }
                _ => {
                    // Insight is non-exhaustive
                    // we may add timer events in the future
                }
            }
        }

        tracing::info!("Connection closed");
    });

    let submit = SubmitSm::builder()
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build();

    tracing::info!("Sending SubmitSm");

    let response = client.submit_sm(submit.clone()).await?;

    tracing::info!(?response, "Got SubmitSmResp");

    // Wait a little bit to see the incoming events.

    tokio::time::sleep(Duration::from_secs(30)).await;

    tracing::info!("Unbinding from the server");

    client.unbind().await?;

    tracing::info!("Closing the connection");

    client.close().await?;

    tracing::info!("Waiting for the connection to terminate");

    client.closed().await;

    events.await?;

    Ok(())
}
