use std::{format, println, str::FromStr, vec::Vec};

use futures::{SinkExt, StreamExt};
use testcontainers::{
    GenericImage,
    core::{ContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_util::codec::{Framed, FramedRead, FramedWrite};

use crate::{
    CommandStatus,
    command::owned::Command,
    encode::{Encode, Length},
    pdus::owned::*,
    tests::owned::test_commands,
    tlvs::owned::{BroadcastRequestTlvValue, MessageSubmissionRequestTlvValue},
    tokio_codec::{CommandCodec, DecodeError},
    types::owned::{AnyOctetString, COctetString, OctetString},
    values::{owned::*, *},
};

/// Encode and decode every possible test command created using [`TestInstance`](crate::tests::TestInstance).
#[tokio::test]
async fn encode_decode() {
    let commands = test_commands();

    let (writer, reader) = tokio::io::duplex(128);

    let mut framed_writer = Framed::new(writer, CommandCodec::new());
    let mut framed_reader = Framed::new(reader, CommandCodec::new());

    let writer_commands = commands.clone();
    tokio::spawn(async move {
        for command in writer_commands {
            framed_writer
                .send(command)
                .await
                .expect("Failed to send PDU");
        }
    });

    let mut client_commands = Vec::new();

    while let Some(Ok(command)) = framed_reader.next().await {
        client_commands.push(command);
    }

    assert_eq!(client_commands, commands);
}

#[tokio::test]
async fn max_length() {
    let max_length = 16;

    let (writer, reader) = tokio::io::duplex(1024);

    let mut framed_writer = Framed::new(writer, CommandCodec::new());
    let mut framed_reader = Framed::new(reader, CommandCodec::new().with_max_length(max_length));

    let command = Command::new(Default::default(), Default::default(), SubmitSm::default());
    let command_length = 4 + command.length();

    framed_writer.send(&command).await.unwrap();

    match framed_reader.next().await.unwrap().unwrap_err() {
        DecodeError::MaxLength { actual, max } => {
            assert_eq!(actual, command_length);
            assert_eq!(max, max_length);
        }
        _ => {
            panic!("Decode must fail with `DecodeError::Length`")
        }
    }
}

#[tokio::test]
async fn min_length() {
    let (mut writer, reader) = tokio::io::duplex(1024);

    let buf = &mut [0; 16];

    let command = Command::new(Default::default(), Default::default(), Pdu::EnquireLink);
    let _ = command.encode(buf);

    let wrong_command_length = 15_u32;
    buf[0..4].copy_from_slice(&wrong_command_length.to_be_bytes());

    writer.write_all(&buf[..]).await.unwrap();

    let mut framed_reader = Framed::new(reader, CommandCodec::new());

    match framed_reader.next().await.unwrap().unwrap_err() {
        DecodeError::MinLength { actual, min } => {
            assert_eq!(actual, wrong_command_length as usize);
            assert_eq!(min, 16);
        }
        _ => {
            panic!("Decode must fail with `DecodeError::Length`")
        }
    }
}

/// Connect to localhost:2775 and send a command.
///
/// I use this function to throw random commands at a server and catch them in wireshark.
async fn connect_and_send(command: Command) {
    let stream = TcpStream::connect("127.0.0.1:2775")
        .await
        .expect("Failed to connect");

    let mut framed = Framed::new(stream, CommandCodec::new());

    framed.send(command).await.expect("Failed to send PDU");

    while let Some(command) = framed.next().await {
        println!("Received: {command:#?}");
    }
}

// cargo test send_bind_transmitter --features tokio-codec -- --ignored --nocapture
#[tokio::test]
#[ignore = "observation test"]
async fn send_bind_transmitter() {
    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(BindTransmitter::builder().build());

    connect_and_send(command).await;
}

// cargo test send_alert_notification --features tokio-codec -- --ignored --nocapture
#[tokio::test]
#[ignore = "observation test"]
async fn send_alert_notification() {
    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(
            AlertNotification::builder()
                .ms_availability_status(Some(MsAvailabilityStatus::Denied))
                .build(),
        );

    connect_and_send(command).await;
}

// cargo test send_broadcast_sm --features tokio-codec -- --ignored --nocapture
#[tokio::test]
#[ignore = "observation test"]
async fn send_broadcast_sm() {
    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(
            BroadcastSm::builder()
                .push_tlv(BroadcastRequestTlvValue::AlertOnMessageDelivery(
                    AlertOnMessageDelivery::UseMediumPriorityAlert,
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier::new(
                        BroadcastAreaFormat::Polygon,
                        AnyOctetString::new(b"Polygon Area".to_vec()),
                    ),
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier::new(
                        BroadcastAreaFormat::AliasName,
                        AnyOctetString::new(b"AliasName Area".to_vec()),
                    ),
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier::new(
                        BroadcastAreaFormat::EllipsoidArc,
                        AnyOctetString::new(b"EllipsoidArc Area".to_vec()),
                    ),
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier::new(
                        BroadcastAreaFormat::EllipsoidArc,
                        AnyOctetString::new(b"EllipsoidArc Area 2".to_vec()),
                    ),
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier::new(
                        BroadcastAreaFormat::EllipsoidArc,
                        AnyOctetString::new(b"EllipsoidArc Area 3".to_vec()),
                    ),
                ))
                .push_tlv(BroadcastRequestTlvValue::BroadcastMessageClass(
                    BroadcastMessageClass::Class2,
                ))
                .build(),
        );

    connect_and_send(command).await;
}

// cargo test send_submit_sm --features tokio-codec -- --ignored --nocapture
#[tokio::test]
#[ignore = "observation test"]
async fn send_submit_sm() {
    let command = Command::builder()
        .status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(
            SubmitSm::builder()
                .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                    MessagePayload::new(AnyOctetString::new(b"Message Payload".to_vec())),
                ))
                .build(),
        );

    connect_and_send(command).await;
}

// FIXME: Not really helpful.
// cargo test do_codec --features tokio-codec -- --ignored --nocapture
#[tokio::test]
#[ignore = "integration test"]
async fn do_codec() {
    // See https://github.com/JadKHaddad/smpp-smsc-simulator
    let container = GenericImage::new("jadkhaddad/smpp-smsc-simulator", "1.0.0")
        .with_wait_for(WaitFor::message_on_stdout(
            "Listening for SMPP on port 2775",
        ))
        .with_exposed_port(ContainerPort::Tcp(2775))
        .start()
        .await
        .expect("Failed to start smpp-smsc-simulator");

    let port = container
        .get_host_port_ipv4(2775)
        .await
        .expect("Failed to get container port");

    let stream = TcpStream::connect(format!("127.0.0.1:{port}"))
        .await
        .expect("Failed to connect");

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec::new());
    let mut framed_write = FramedWrite::new(writer, CommandCodec::new());

    tokio::spawn(async move {
        while let Some(command) = framed_read.next().await {
            println!("{command:#?}");
            println!();
        }
    });

    let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);

    framed_write
        .send(&enquire_link_command)
        .await
        .expect("Failed to send PDU");

    let bind_transceiver_command = Command::new(
        CommandStatus::EsmeRok,
        1,
        BindTransceiver::builder()
            .system_id(
                COctetString::from_str("NfDfddEKVI0NCxO").expect("Failed to create system_id"), // cspell:disable-line
            )
            .password(COctetString::from_str("rEZYMq5j").expect("Failed to create password"))
            .system_type(COctetString::empty())
            .interface_version(InterfaceVersion::Smpp5_0)
            .addr_ton(Ton::Unknown)
            .addr_npi(Npi::Unknown)
            .address_range(COctetString::empty())
            .build(),
    );

    framed_write
        .send(&bind_transceiver_command)
        .await
        .expect("Failed to send PDU");

    let submit_sm_command = Command::new(
        CommandStatus::EsmeRok,
        2,
        SubmitSm::builder()
            .service_type(ServiceType::default())
            .source_addr_ton(Ton::Unknown)
            .source_addr_npi(Npi::Unknown)
            .source_addr(COctetString::from_str("some_source").expect("Failed to create source"))
            .dest_addr_ton(Ton::Unknown)
            .dest_addr_npi(Npi::Unknown)
            .destination_addr(COctetString::from_str("some_dest").expect("Failed to create dest"))
            .esm_class(EsmClass::default())
            // Use default values to "not" get a delivery receipt
            .registered_delivery(RegisteredDelivery::request_all())
            .replace_if_present_flag(ReplaceIfPresentFlag::default())
            .data_coding(DataCoding::default())
            .short_message(
                OctetString::from_str("Hi, I am a short message. I will be overridden :(")
                    .expect("Failed to create short message"),
            )
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                MessagePayload::new(
                    AnyOctetString::from_str(
                        "Hi, I am a very long message that will override the short message :D",
                    )
                    .expect("Failed to create message_payload"),
                ),
            ))
            .build(),
    );

    framed_write
        .send(&submit_sm_command)
        .await
        .expect("Failed to send PDU");

    // wait for delivery receipt
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let unbind_command = Command::new(CommandStatus::EsmeRok, 3, Pdu::Unbind);

    framed_write
        .send(&unbind_command)
        .await
        .expect("Failed to send PDU");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
