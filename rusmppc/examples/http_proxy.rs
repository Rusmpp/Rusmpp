//! This example demonstrates how to use `rusmppc` to create a simple HTTP proxy that accepts POST requests and forwards them as SMPP SubmitSm commands to an SMPP server.
//! The connection to the SMPP server is managed using `ManagedClient`, which automatically handles reconnection.
//!
//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example http_proxy
//! ```
//!
//! Then you can send a POST request to `http://localhost:3000/submit` with a JSON body like
//!
//! ```not_rust
//! curl -X POST http://localhost:3000/submit \
//!   -H "Content-Type: application/json" \
//!   -d '{
//!     "source_addr": "12345",
//!     "destination_addr": "491234567890",
//!     "short_message": "Hello from RUSMPP proxy"
//!   }'
//! ```

use std::{str::FromStr, time::Duration};

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use futures::StreamExt;
use rusmpp::{
    pdus::{BindTransceiver, SubmitSm},
    types::{COctetString, OctetString},
};
use rusmppc::{ConnectionBuilder, managed::ManagedClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("http_proxy=info,rusmpp=off,rusmppc=debug")
        .init();

    tracing::info!("Connecting to SMPP server");

    let (client, mut events): (ManagedClient, _) = ConnectionBuilder::new()
        .managed()
        .transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .system_type(COctetString::empty())
                .build(),
        )
        .auto_reconnect_interval(Duration::from_secs(15))
        .connect("smpp://127.0.0.1:2775")
        //.connect("smpps://rusmpps.rusmpp.org:2776")
        .await?;

    tracing::info!("Connected to SMPP server");

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");
        }

        tracing::info!("Connection closed");
    });

    let app = Router::new()
        .route("/submit", post(submit_sm))
        .with_state(client);

    let addr = "0.0.0.0:3000";

    tracing::info!(addr, "Starting HTTP server");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn submit_sm(
    State(client): State<ManagedClient>,
    Json(request): Json<SubmitSmRequest>,
) -> Result<StatusCode, StatusCode> {
    let sm = SubmitSm::builder()
        .source_addr(
            COctetString::from_string(request.source_addr).map_err(|err| {
                tracing::error!(?err, "Failed to create source address");

                StatusCode::BAD_REQUEST
            })?,
        )
        .destination_addr(
            COctetString::from_string(request.destination_addr).map_err(|err| {
                tracing::error!(?err, "Failed to create destination address");

                StatusCode::BAD_REQUEST
            })?,
        )
        .short_message(
            OctetString::from_string(request.short_message).map_err(|err| {
                tracing::error!(?err, "Failed to create short message");

                StatusCode::BAD_REQUEST
            })?,
        )
        .build();

    let client = client
        .get_with_timeout(Duration::from_secs(3))
        .await
        .ok_or_else(|| {
            tracing::warn!("Timed out while waiting for a connected client");

            StatusCode::REQUEST_TIMEOUT
        })?
        .map_err(|err| {
            tracing::error!(?err, "Failed to get a connected client");

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    client.submit_sm(sm).await.map_err(|err| {
        tracing::error!(?err, "Failed to submit sm");

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, serde::Deserialize)]
struct SubmitSmRequest {
    source_addr: String,
    destination_addr: String,
    short_message: String,
}
