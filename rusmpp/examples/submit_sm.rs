//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator).
//!
//! Run with
//!
//! ```not_rust
//! cargo run --example submit_sm --features="tokio-codec tracing pretty-hex-fmt"
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindTransceiver, SubmitSm},
    tlvs::TlvTag,
    tokio_codec::CommandCodec,
    types::{COctetString, OctetString},
    values::{EsmClass, InterfaceVersion, Npi, RegisteredDelivery, ServiceType, Ton},
};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("submit_sm=info,rusmpp=trace")
        .init();

    let stream = TcpStream::connect("127.0.0.1:2775").await?;

    let mut framed = Framed::new(stream, CommandCodec::new());

    // Build commands. Omitted values will be set to default.
    let bind_transceiver_command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .system_type(COctetString::empty())
                .interface_version(InterfaceVersion::Smpp5_0)
                .addr_ton(Ton::Unknown)
                .addr_npi(Npi::Unknown)
                .address_range(COctetString::empty())
                .build(),
        );

    // Send commands.
    framed.send(bind_transceiver_command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed.next().await {
        if let Some(Pdu::BindTransceiverResp(_)) = command.pdu() {
            info!("BindTransceiverResp received.");

            if let CommandStatus::EsmeRok = command.status() {
                info!("Successful bind.");

                break;
            }
        }
    }

    let submit_sm_command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(2)
        .pdu(
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
        );

    framed.send(submit_sm_command).await?;

    'outer: while let Some(Ok(command)) = framed.next().await {
        match command.pdu() {
            Some(Pdu::SubmitSmResp(_)) => {
                info!("SubmitSmResp received.");

                if let CommandStatus::EsmeRok = command.status() {
                    info!("Successful submit.");
                }
            }
            Some(Pdu::DeliverSm(deliver_sm)) => {
                info!("DeliverSm received.");

                for tlv in deliver_sm.tlvs().iter() {
                    if let TlvTag::ReceiptedMessageId = tlv.tag() {
                        info!("Delivery receipt received.");

                        break 'outer;
                    }
                }
            }
            _ => {}
        }
    }

    let unbind_command = Command::new(CommandStatus::EsmeRok, 3, Pdu::Unbind);

    framed.send(unbind_command).await?;

    while let Some(Ok(command)) = framed.next().await {
        if let CommandId::UnbindResp = command.id() {
            info!("UnbindResp received.");

            if let CommandStatus::EsmeRok = command.status() {
                info!("Successful unbind.");

                break;
            }
        }
    }

    Ok(())
}
