//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example raw_client
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{CommandStatus, Pdu, pdus::BindTransceiver, types::COctetString};
use rusmppc::ConnectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("raw_client=info,rusmpp=off,rusmppc=debug")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        .connect("smpp://rusmpps.rusmpp.org:2775")
        .await?;

    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");
        }

        tracing::info!("Connection closed");
    });

    // Create a raw client that can send any command.
    let raw = client.raw();

    // `send` method sends the command and returns a tuple of the sent command and a future
    // that resolves to the response command.
    let (command, response) = raw
        .send(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .build(),
        )
        .await?;

    tracing::info!(?command, "Sent BindTransceiver command successfully");

    // Manually await the response.
    let response = response.await?;

    tracing::info!(?response, "Bound successfully");

    // If you are not expecting any responses, you should drop the response future as it will never resolve with a response.
    let (command, _) = raw
        .status(CommandStatus::EsmeRunknownerr)
        .send(Pdu::GenericNack)
        .await?;

    tracing::info!(?command, "Sent GenericNack command successfully");

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
