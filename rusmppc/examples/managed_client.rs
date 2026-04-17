//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example managed_client
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    pdus::{BindTransceiver, SubmitSm},
    types::COctetString,
};
use rusmppc::ConnectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("managed_client=info,rusmpp=off,rusmppc=off")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        .managed()
        .transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .system_type(COctetString::empty())
                .build(),
        )
        .connect("smpp://localhost:2775")
        .await?;

    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");
        }

        tracing::info!("Connection closed");
    });

    for _ in 0..10 {
        let response = client
            .get()
            .await
            .ok_or("Not connected")?
            .submit_sm(SubmitSm::builder().build())
            .await?;

        tracing::info!(?response, "SubmitSm response");

        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    events.await?;

    Ok(())
}
