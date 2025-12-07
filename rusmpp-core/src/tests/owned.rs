use std::boxed::Box;

use crate::{
    CommandId,
    command::owned::Command,
    decode::owned::{Decode, DecodeWithLength},
    encode::{Encode, Length},
    pdus::owned::*,
    tests::TestInstance,
    types::owned::AnyOctetString,
};

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + Decode,
{
    for original in T::instances() {
        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        let (decoded, _size) = T::decode(&buf[..size]).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_with_length_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + DecodeWithLength,
{
    for original in T::instances() {
        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        let (decoded, _size) =
            T::decode(&buf[..size], original.length()).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}

/// Trait for chaining test commands.
///
/// Type erased, otherwise rustc will allocate 30 quintillion petabytes during monomorphization when compiling [`test_commands`] and fail due to `OOM`.
pub trait ChainExt {
    fn chain_instances_as_cmds<T: TestInstance + Into<Pdu> + 'static>(
        self,
    ) -> Box<dyn Iterator<Item = Command>>;

    fn chain_single_cmd<T: Into<Pdu> + 'static>(self, pdu: T) -> Box<dyn Iterator<Item = Command>>;
}

impl<I: Iterator<Item = Command> + 'static> ChainExt for I {
    fn chain_instances_as_cmds<T: TestInstance + Into<Pdu> + 'static>(
        self,
    ) -> Box<dyn Iterator<Item = Command>> {
        Box::new(
            self.chain(
                T::instances()
                    .into_iter()
                    .map(|pdu| Command::new(Default::default(), Default::default(), pdu)),
            ),
        )
    }

    fn chain_single_cmd<T: Into<Pdu> + 'static>(self, pdu: T) -> Box<dyn Iterator<Item = Command>> {
        Box::new(self.chain(core::iter::once(Command::new(
            Default::default(),
            Default::default(),
            pdu,
        ))))
    }
}

/// All test commands created using [`TestInstance`].
pub fn test_commands() -> alloc::vec::Vec<Command> {
    core::iter::empty()
        .chain_instances_as_cmds::<BindTransmitter>()
        .chain_instances_as_cmds::<BindTransmitterResp>()
        .chain_instances_as_cmds::<BindReceiver>()
        .chain_instances_as_cmds::<BindReceiverResp>()
        .chain_instances_as_cmds::<BindTransceiver>()
        .chain_instances_as_cmds::<BindTransceiverResp>()
        .chain_instances_as_cmds::<Outbind>()
        .chain_instances_as_cmds::<AlertNotification>()
        .chain_instances_as_cmds::<SubmitSm>()
        .chain_instances_as_cmds::<SubmitSmResp>()
        .chain_instances_as_cmds::<QuerySm>()
        .chain_instances_as_cmds::<QuerySmResp>()
        .chain_instances_as_cmds::<DeliverSm>()
        .chain_instances_as_cmds::<DeliverSmResp>()
        .chain_instances_as_cmds::<DataSm>()
        .chain_instances_as_cmds::<DataSmResp>()
        .chain_instances_as_cmds::<CancelSm>()
        .chain_instances_as_cmds::<ReplaceSm>()
        .chain_instances_as_cmds::<SubmitMulti>()
        .chain_instances_as_cmds::<SubmitMultiResp>()
        .chain_instances_as_cmds::<BroadcastSm>()
        .chain_instances_as_cmds::<BroadcastSmResp>()
        .chain_instances_as_cmds::<CancelBroadcastSm>()
        .chain_instances_as_cmds::<QueryBroadcastSm>()
        .chain_instances_as_cmds::<QueryBroadcastSmResp>()
        .chain_single_cmd(Pdu::Unbind)
        .chain_single_cmd(Pdu::UnbindResp)
        .chain_single_cmd(Pdu::EnquireLink)
        .chain_single_cmd(Pdu::EnquireLinkResp)
        .chain_single_cmd(Pdu::GenericNack)
        .chain_single_cmd(Pdu::CancelSmResp)
        .chain_single_cmd(Pdu::ReplaceSmResp)
        .chain_single_cmd(Pdu::CancelBroadcastSmResp)
        .chain_single_cmd(Pdu::Other {
            command_id: CommandId::Other(100),
            body: AnyOctetString::new(b"SMPP".to_vec()),
        })
        .collect()
}

#[test]
#[ignore = "observation test"]
fn print_decode_errors() {
    let buf = &mut [0u8; 1024];

    for command in test_commands() {
        if command.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = command.encode(buf);
        // Destroy random bytes in the buffer
        buf[8] = 0xFF;
        buf[16] = 0xFF;
        buf[32] = 0xFF;
        buf[64] = 0xFF;

        let result = Command::decode(&buf[..size], size);

        let _ = std::dbg!(result);
    }
}
