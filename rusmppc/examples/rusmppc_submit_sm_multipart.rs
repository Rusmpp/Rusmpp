//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator)
//! or with the public SMPP server at `smpp://rusmpps.rusmpp.org:2775` or `smpps://rusmpps.rusmpp.org:2776`.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example rusmppc_submit_sm_multipart
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId,
    extra::{concatenation::SubmitSmMultipartExt, encoding::ucs2::Ucs2},
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{DataCoding, EsmClass, Npi, Ton},
};
use rusmppc::{ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmppc_submit_sm_multipart=info,rusmpp=debug,rusmppc=debug")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
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

    let client_clone = client.clone();

    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Incoming(command) = event {
                if command.id() == CommandId::DeliverSm {
                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }

        tracing::info!("Connection closed");
    });

    // c-spell: disable
    let message = r##"GSM 3 parts : Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
    // c-spell: enable

    let multipart = SubmitSm::builder()
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        // esm_class will be updated with UDHI indicator by the multipart builder.
        .esm_class(EsmClass::default())
        // data_coding will be overridden by the multipart builder to match the encoder.
        .data_coding(DataCoding::default())
        // short_message will be overridden by `short_message` of the multipart builder.
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build()
        .multipart(message)
        // Use 16-bit reference number.
        .reference_u16(1)
        // Use gsm7bit unpacked encoding.
        .gsm7bit_unpacked()
        // Fallback to ucs2 encoding if the message can not be encoded in gsm7bit.
        .fallback(Ucs2::new())
        .build()?;

    let total = multipart.len();

    tracing::info!(total, "Submitting multipart message");

    for (i, sm) in multipart.into_iter().enumerate() {
        tracing::info!(part=i+1, short_message_len=sm.short_message().len(), esm_class=?sm.esm_class, data_coding=?sm.data_coding, "Sending SubmitSm");

        let response = client.submit_sm(sm).await?;

        tracing::info!(?response, "Got SubmitSmResp");
    }

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
