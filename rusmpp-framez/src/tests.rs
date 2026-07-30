use embedded_io_adapters::tokio_1::FromTokio;
use framez::{FramedRead, FramedWrite, ReadError, next};

use tokio::io::AsyncWriteExt;

use rusmpp_core::{
    command::borrowed::Command,
    encode::Encode,
    framez::{CommandCodec, DecodeError},
    pdus::borrowed::*,
    tests::borrowed::test_commands,
};

/// Encode and decode every possible test command created using [`TestInstance`](crate::tests::TestInstance).
#[tokio::test]
async fn encode_decode() {
    let commands = test_commands();

    let (writer, reader) = tokio::io::duplex(128);

    let writer_commands = commands.clone();

    tokio::spawn(async move {
        let buffer = &mut [0u8; 1024];
        let mut framed_writer =
            FramedWrite::new(CommandCodec::<16>::new(), FromTokio::new(writer), buffer);

        for command in writer_commands {
            framed_writer
                .send(command)
                .await
                .expect("Failed to send PDU");
        }
    });

    let buffer = std::vec![0u8; 1024].leak();
    let mut framed_reader =
        FramedRead::new(CommandCodec::<16>::new(), FromTokio::new(reader), buffer);

    let mut index = 0;
    while let Some(command) = next!(framed_reader)
        .transpose()
        .expect("Failed to read command")
    {
        // We cant move out of framed so we compare in place.
        assert_eq!(command, commands[index]);
        index += 1;
    }
}

#[tokio::test]
async fn max_length() {
    let (writer, reader) = tokio::io::duplex(1024);

    let buffer = &mut [0u8; 1024];
    let mut framed_writer =
        FramedWrite::new(CommandCodec::<16>::new(), FromTokio::new(writer), buffer);

    let buffer = std::vec![0u8; 16].leak();
    let mut framed_reader =
        FramedRead::new(CommandCodec::<16>::new(), FromTokio::new(reader), buffer);

    let command = Command::new(Default::default(), Default::default(), SubmitSm::default());

    framed_writer.send(command).await.unwrap();

    match next!(framed_reader).unwrap().unwrap_err() {
        ReadError::BufferTooSmall => {}
        _ => {
            panic!("Decode must fail with `ReadError::BufferTooSmall`")
        }
    }
}

#[tokio::test]
async fn min_length() {
    let (mut writer, reader) = tokio::io::duplex(1024);

    let buf = &mut [0; 16];

    let command = Command::new(
        Default::default(),
        Default::default(),
        Pdu::<'_, 16>::EnquireLink,
    );
    let _ = command.encode(buf);

    let wrong_command_length = 15_u32;
    buf[0..4].copy_from_slice(&wrong_command_length.to_be_bytes());

    writer.write_all(&buf[..]).await.unwrap();

    let buffer = std::vec![0u8; 1024].leak();
    let mut framed_reader =
        FramedRead::new(CommandCodec::<16>::new(), FromTokio::new(reader), buffer);

    match next!(framed_reader).unwrap().unwrap_err() {
        ReadError::Decode(DecodeError::MinLength { actual, min }) => {
            assert_eq!(actual, wrong_command_length as usize);
            assert_eq!(min, 16);
        }
        _ => {
            panic!("Decode must fail with `DecodeError::MinLength`")
        }
    }
}
