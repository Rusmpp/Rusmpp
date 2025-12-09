use std::boxed::Box;

use crate::{
    CommandId,
    command::borrowed::Command,
    decode::borrowed::{Decode, DecodeWithLength},
    encode::{Encode, Length},
    pdus::borrowed::*,
    tests::TestInstance,
    types::borrowed::AnyOctetString,
};

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + Decode<'static>,
{
    for original in T::instances() {
        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        let buf = buf[..size].to_vec().leak();

        let (decoded, _size) = T::decode(buf).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_with_length_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + DecodeWithLength<'static>,
{
    for original in T::instances() {
        let mut buf = [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(&mut buf);

        let buf = buf[..size].to_vec().leak();

        let (decoded, _size) = T::decode(buf, original.length()).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}

/// See [`ChainExt`](#[cfg(feature = "alloc")] crate::tests::owned:ChainExt) for details.
pub trait ChainExt {
    fn chain_instances_as_cmds<T: TestInstance + Into<Pdu<'static, 16>> + 'static>(
        self,
    ) -> Box<dyn Iterator<Item = Command<'static, 16>>>;

    fn chain_single_cmd<T: Into<Pdu<'static, 16>> + 'static>(
        self,
        pdu: T,
    ) -> Box<dyn Iterator<Item = Command<'static, 16>>>;
}

impl<I: Iterator<Item = Command<'static, 16>> + 'static> ChainExt for I {
    fn chain_instances_as_cmds<T: TestInstance + Into<Pdu<'static, 16>> + 'static>(
        self,
    ) -> Box<dyn Iterator<Item = Command<'static, 16>>> {
        Box::new(
            self.chain(
                T::instances()
                    .into_iter()
                    .map(|pdu| Command::new(Default::default(), Default::default(), pdu)),
            ),
        )
    }

    fn chain_single_cmd<T: Into<Pdu<'static, 16>> + 'static>(
        self,
        pdu: T,
    ) -> Box<dyn Iterator<Item = Command<'static, 16>>> {
        Box::new(self.chain(core::iter::once(Command::new(
            Default::default(),
            Default::default(),
            pdu,
        ))))
    }
}

/// All test commands created using [`TestInstance`].
pub fn test_commands() -> alloc::vec::Vec<Command<'static, 16>> {
    core::iter::empty()
        .chain_instances_as_cmds::<BindTransmitter<'static>>()
        .chain_instances_as_cmds::<BindTransmitterResp<'static>>()
        .chain_instances_as_cmds::<BindReceiver<'static>>()
        .chain_instances_as_cmds::<BindReceiverResp<'static>>()
        .chain_instances_as_cmds::<BindTransceiver<'static>>()
        .chain_instances_as_cmds::<BindTransceiverResp<'static>>()
        .chain_instances_as_cmds::<Outbind<'static>>()
        .chain_instances_as_cmds::<AlertNotification<'static>>()
        .chain_instances_as_cmds::<SubmitSm<'static, 16>>()
        .chain_instances_as_cmds::<SubmitSmResp<'static, 16>>()
        .chain_instances_as_cmds::<QuerySm<'static>>()
        .chain_instances_as_cmds::<QuerySmResp<'static>>()
        .chain_instances_as_cmds::<DeliverSm<'static, 16>>()
        .chain_instances_as_cmds::<DeliverSmResp<'static, 16>>()
        .chain_instances_as_cmds::<DataSm<'static, 16>>()
        .chain_instances_as_cmds::<DataSmResp<'static, 16>>()
        .chain_instances_as_cmds::<CancelSm<'static>>()
        .chain_instances_as_cmds::<ReplaceSm<'static>>()
        .chain_instances_as_cmds::<SubmitMulti<'static, 16>>()
        .chain_instances_as_cmds::<SubmitMultiResp<'static, 16>>()
        .chain_instances_as_cmds::<BroadcastSm<'static, 16>>()
        .chain_instances_as_cmds::<BroadcastSmResp<'static, 16>>()
        .chain_instances_as_cmds::<CancelBroadcastSm<'static, 16>>()
        .chain_instances_as_cmds::<QueryBroadcastSm<'static>>()
        .chain_instances_as_cmds::<QueryBroadcastSmResp<'static, 16>>()
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
            body: AnyOctetString::new(b"SMPP"),
        })
        .collect()
}

#[test]
#[ignore = "observation test"]
fn print_decode_errors() {
    let mut buf = [0u8; 1024];

    for command in test_commands() {
        if command.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = command.encode(&mut buf);
        // Destroy random bytes in the buffer
        buf[8] = 0xFF;
        buf[16] = 0xFF;
        buf[32] = 0xFF;
        buf[64] = 0xFF;

        let buf = buf[..size].to_vec().leak();

        let result = Command::<'static, 16>::decode(&buf[..size], size);

        let _ = std::dbg!(result);
    }
}
