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
use rusmpp::{
    Command, CommandStatus, Pdu,
    pdus::{BindTransceiver, SubmitSm, SubmitSmResp},
    types::COctetString,
};
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
    // The `send` method sends the command and returns a tuple of the sent sequence number and a future
    // that resolves to the response command.
    let (sequence_number, response) = client
        .raw()
        .response_timeout(Duration::from_secs(30))
        .send(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .build(),
        )
        .await?;

    tracing::info!(?sequence_number, "Sent BindTransceiver successfully");

    // Manually await the response.
    let response: Command = response.await?;

    // It is your responsibility to check for the correct response command and handle it appropriately.
    let Some(Pdu::BindTransceiverResp(_)) = response.pdu() else {
        tracing::error!(?response, "Unexpected response");

        return Err("Unexpected response".into());
    };

    tracing::info!(?response, "Bound successfully");

    // If you are not expecting any responses, you should drop the response future as it will never resolve with a response.
    let (sequence_number, _) = client
        .raw()
        .status(CommandStatus::EsmeRunknownerr)
        .send(Pdu::GenericNack)
        .await?;

    tracing::info!(?sequence_number, "Sent GenericNack successfully");

    // Or you can use a type safe method that sends a request and returns a future resolving to the type safe response PDU.
    let (sequence_number, response) = client.raw().submit_sm(SubmitSm::builder().build()).await?;

    tracing::info!(?sequence_number, "Sent SubmitSm successfully");

    // Unlike the `send` method, the type safe methods will check the appropriate response PDU.
    let response: SubmitSmResp = response.await?;

    tracing::info!(
        ?sequence_number,
        ?response,
        "Received a successful SubmitSmResp response"
    );

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
