use rusmpp_macros::Rusmpp;

use crate::{CommandId, CommandStatus, pdus::borrowed::Pdu};

/// `SMPP` command.
///
/// The following PDU example illustrates how a `SMPP` PDU is decoded:
///
/// Sample PDU (Values are shown in Hex format):
///
/// 00 00 00 2F 00 00 00 02 00 00 00 00 00 00 00 01
///
/// 53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
///
/// 30 38 00 53 55 42 4D 49 54 31 00 50 01 01 00
///
/// The 16-octet header would be decoded as follows:
///
/// | Octets | Description |
/// | ------ | ----------- |
/// | 00 00 00 2F | Command Length (47) |
/// | 00 00 00 02 | Command ID (bind_transmitter) |
/// | 00 00 00 00 | Command Status (0) |
/// | 00 00 00 01 | Sequence Number (1)|
///
/// The remaining data represents the PDU body (which in this example relates to the
/// bind_transmitter PDU). This is diagnosed as follows:
///
/// | Octets | Value |
/// | ------ | ----- |
/// | 53 4D 50 50 33 54 45 53 54 00 | system_id (“SMPP3TEST”) |
/// | 73 65 63 72 65 74 30 38 00    | password (“secret08”) |
/// | 53 55 42 4D 49 54 31 00       | system_type (“SUBMIT1”) |
/// | 50                            | interface_version (0x50 “V5.0 compliant”) |
/// | 01                            | addr_ton (0x01) |
/// | 01                            | addr_npi (0x01) |
/// | 00                            | addr_range (NULL) |
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
pub struct Command<'a, const N: usize> {
    /// See [`CommandId`]
    id: CommandId,
    /// See [`CommandStatus`]
    pub status: CommandStatus,
    /// The sequence_number represents a means of uniquely
    /// identifying each PDU within a `SMPP` session. It also provides a means of correlating request
    /// and response PDUs based on matching sequence number.
    pub sequence_number: u32,
    /// See [`Pdu`]
    ///
    /// Optional because incoming commands may not have a PDU.
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu<'a, N>>,
}

impl<'a, const N: usize> Default for Command<'a, N> {
    fn default() -> Self {
        Self {
            id: CommandId::EnquireLink,
            status: CommandStatus::EsmeRok,
            sequence_number: 0,
            pdu: Some(Pdu::EnquireLink),
        }
    }
}

impl<'a, const N: usize> Command<'a, N> {
    pub fn new(status: CommandStatus, sequence_number: u32, pdu: impl Into<Pdu<'a, N>>) -> Self {
        Self::new_const(status, sequence_number, pdu.into())
    }

    pub const fn new_const(status: CommandStatus, sequence_number: u32, pdu: Pdu<'a, N>) -> Self {
        let id = pdu.command_id();

        Self {
            id,
            status,
            sequence_number,
            pdu: Some(pdu),
        }
    }

    #[inline]
    pub const fn id(&self) -> CommandId {
        self.id
    }

    #[inline]
    pub const fn status(&self) -> CommandStatus {
        self.status
    }

    #[inline]
    pub const fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    #[inline]
    pub const fn pdu(&self) -> Option<&Pdu<'a, N>> {
        self.pdu.as_ref()
    }

    #[inline]
    pub fn set_pdu(&mut self, pdu: impl Into<Pdu<'a, N>>) {
        let pdu = pdu.into();

        self.id = pdu.command_id();

        self.pdu = Some(pdu);
    }

    #[inline]
    pub fn builder() -> CommandStatusBuilder<'a, N> {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct CommandStatusBuilder<'a, const N: usize> {
    inner: Command<'a, N>,
}

impl<'a, const N: usize> CommandStatusBuilder<'a, N> {
    #[inline]
    pub fn status(mut self, status: CommandStatus) -> SequenceNumberBuilder<'a, N> {
        self.inner.status = status;

        SequenceNumberBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct SequenceNumberBuilder<'a, const N: usize> {
    inner: Command<'a, N>,
}

impl<'a, const N: usize> SequenceNumberBuilder<'a, N> {
    #[inline]
    pub fn sequence_number(mut self, sequence_number: u32) -> PduBuilder<'a, N> {
        self.inner.sequence_number = sequence_number;

        PduBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct PduBuilder<'a, const N: usize> {
    inner: Command<'a, N>,
}

impl<'a, const N: usize> PduBuilder<'a, N> {
    #[inline]
    pub fn pdu(mut self, pdu: impl Into<Pdu<'a, N>>) -> Command<'a, N> {
        self.inner.set_pdu(pdu);
        self.inner
    }
}

#[cfg(feature = "serde")]
const _: () = {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::types::borrowed::AnyOctetString;

    #[derive(Serialize)]
    struct SerCommand<'a, const N: usize> {
        status: CommandStatus,
        sequence_number: u32,
        pdu: &'a Pdu<'a, N>,
    }

    impl<'a, const N: usize> Serialize for Command<'a, N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let pdu = Pdu::Other {
                command_id: self.id(),
                body: AnyOctetString::empty(),
            };

            let pdu = self.pdu.as_ref().unwrap_or(&pdu);

            let command = SerCommand {
                status: self.status,
                sequence_number: self.sequence_number,
                pdu,
            };

            command.serialize(serializer)
        }
    }

    #[derive(Deserialize)]
    #[serde(bound(deserialize = "'de: 'a"))]
    struct DeCommand<'a, const N: usize> {
        status: CommandStatus,
        sequence_number: u32,
        pdu: Pdu<'a, N>,
    }

    impl<'a, const N: usize> From<DeCommand<'a, N>> for Command<'a, N> {
        fn from(command: DeCommand<'a, N>) -> Self {
            Self::new(command.status, command.sequence_number, command.pdu)
        }
    }

    impl<'de: 'a, 'a, const N: usize> Deserialize<'de> for Command<'a, N> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let command = DeCommand::deserialize(deserializer)?;

            Ok(Self::from(command))
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<Command<'static, 16>>();
    }

    #[cfg(feature = "serde")]
    mod serde {
        use std::println;

        use crate::{
            pdus::borrowed::SubmitSm,
            values::{GenericServiceType, borrowed::ServiceType},
        };

        use super::*;

        #[test]
        // #[ignore = "Prints the serialized JSON for manual inspection"]
        fn serialize() {
            let command = Command::<'_, 16>::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(1)
                .pdu(
                    SubmitSm::builder()
                        .service_type(ServiceType::new(
                            GenericServiceType::CellularMessaging.into(),
                        ))
                        .build(),
                );

            let mut serialized = [0u8; 1024];
            let len = serde_json_core::to_slice(&command, &mut serialized[..])
                .expect("Failed to serialize command");

            println!("{:?}", &serialized[..len]);
        }

        #[test]
        fn deserialize() {
            let json: &[u8] = &[
                123, 34, 115, 116, 97, 116, 117, 115, 34, 58, 34, 69, 115, 109, 101, 82, 111, 107,
                34, 44, 34, 115, 101, 113, 117, 101, 110, 99, 101, 95, 110, 117, 109, 98, 101, 114,
                34, 58, 49, 44, 34, 112, 100, 117, 34, 58, 123, 34, 83, 117, 98, 109, 105, 116, 83,
                109, 34, 58, 123, 34, 115, 101, 114, 118, 105, 99, 101, 95, 116, 121, 112, 101, 34,
                58, 67, 77, 84, 0, 44, 34, 115, 111, 117, 114, 99, 101, 95, 97, 100, 100, 114, 95,
                116, 111, 110, 34, 58, 34, 85, 110, 107, 110, 111, 119, 110, 34, 44, 34, 115, 111,
                117, 114, 99, 101, 95, 97, 100, 100, 114, 95, 110, 112, 105, 34, 58, 34, 85, 110,
                107, 110, 111, 119, 110, 34, 44, 34, 115, 111, 117, 114, 99, 101, 95, 97, 100, 100,
                114, 34, 58, 0, 44, 34, 100, 101, 115, 116, 95, 97, 100, 100, 114, 95, 116, 111,
                110, 34, 58, 34, 85, 110, 107, 110, 111, 119, 110, 34, 44, 34, 100, 101, 115, 116,
                95, 97, 100, 100, 114, 95, 110, 112, 105, 34, 58, 34, 85, 110, 107, 110, 111, 119,
                110, 34, 44, 34, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 95, 97, 100,
                100, 114, 34, 58, 0, 44, 34, 101, 115, 109, 95, 99, 108, 97, 115, 115, 34, 58, 123,
                34, 109, 101, 115, 115, 97, 103, 105, 110, 103, 95, 109, 111, 100, 101, 34, 58, 34,
                68, 101, 102, 97, 117, 108, 116, 34, 44, 34, 109, 101, 115, 115, 97, 103, 101, 95,
                116, 121, 112, 101, 34, 58, 34, 68, 101, 102, 97, 117, 108, 116, 34, 44, 34, 97,
                110, 115, 105, 52, 49, 95, 115, 112, 101, 99, 105, 102, 105, 99, 34, 58, 34, 78,
                111, 116, 83, 101, 108, 101, 99, 116, 101, 100, 34, 44, 34, 103, 115, 109, 95, 102,
                101, 97, 116, 117, 114, 101, 115, 34, 58, 34, 78, 111, 116, 83, 101, 108, 101, 99,
                116, 101, 100, 34, 125, 44, 34, 112, 114, 111, 116, 111, 99, 111, 108, 95, 105,
                100, 34, 58, 48, 44, 34, 112, 114, 105, 111, 114, 105, 116, 121, 95, 102, 108, 97,
                103, 34, 58, 48, 44, 34, 115, 99, 104, 101, 100, 117, 108, 101, 95, 100, 101, 108,
                105, 118, 101, 114, 121, 95, 116, 105, 109, 101, 34, 58, 0, 44, 34, 118, 97, 108,
                105, 100, 105, 116, 121, 95, 112, 101, 114, 105, 111, 100, 34, 58, 0, 44, 34, 114,
                101, 103, 105, 115, 116, 101, 114, 101, 100, 95, 100, 101, 108, 105, 118, 101, 114,
                121, 34, 58, 123, 34, 109, 99, 95, 100, 101, 108, 105, 118, 101, 114, 121, 95, 114,
                101, 99, 101, 105, 112, 116, 34, 58, 34, 78, 111, 77, 99, 68, 101, 108, 105, 118,
                101, 114, 121, 82, 101, 99, 101, 105, 112, 116, 82, 101, 113, 117, 101, 115, 116,
                101, 100, 34, 44, 34, 115, 109, 101, 95, 111, 114, 105, 103, 105, 110, 97, 116,
                101, 100, 95, 97, 99, 107, 110, 111, 119, 108, 101, 100, 103, 101, 109, 101, 110,
                116, 34, 58, 34, 78, 111, 82, 101, 99, 101, 105, 112, 116, 83, 109, 101, 65, 99,
                107, 110, 111, 119, 108, 101, 100, 103, 101, 109, 101, 110, 116, 82, 101, 113, 117,
                101, 115, 116, 101, 100, 34, 44, 34, 105, 110, 116, 101, 114, 109, 101, 100, 105,
                97, 116, 101, 95, 110, 111, 116, 105, 102, 105, 99, 97, 116, 105, 111, 110, 34, 58,
                34, 78, 111, 73, 110, 116, 101, 114, 109, 101, 100, 105, 97, 114, 121, 78, 111,
                116, 105, 102, 105, 99, 97, 116, 105, 111, 110, 82, 101, 113, 117, 101, 115, 116,
                101, 100, 34, 44, 34, 111, 116, 104, 101, 114, 34, 58, 48, 125, 44, 34, 114, 101,
                112, 108, 97, 99, 101, 95, 105, 102, 95, 112, 114, 101, 115, 101, 110, 116, 95,
                102, 108, 97, 103, 34, 58, 34, 68, 111, 78, 111, 116, 82, 101, 112, 108, 97, 99,
                101, 34, 44, 34, 100, 97, 116, 97, 95, 99, 111, 100, 105, 110, 103, 34, 58, 34, 77,
                99, 83, 112, 101, 99, 105, 102, 105, 99, 34, 44, 34, 115, 109, 95, 100, 101, 102,
                97, 117, 108, 116, 95, 109, 115, 103, 95, 105, 100, 34, 58, 48, 44, 34, 115, 109,
                95, 108, 101, 110, 103, 116, 104, 34, 58, 48, 44, 34, 115, 104, 111, 114, 116, 95,
                109, 101, 115, 115, 97, 103, 101, 34, 58, 44, 34, 116, 108, 118, 115, 34, 58, 91,
                93, 125, 125, 125,
            ];

            let (command, _) = serde_json_core::from_slice::<Command<'_, 16>>(json)
                .expect("Failed to deserialize command");

            assert_eq!(command.status(), CommandStatus::EsmeRok);
            assert_eq!(command.sequence_number(), 1);

            let pdu = command.pdu().expect("Expected PDU to be present");
            let pdu = match pdu {
                Pdu::SubmitSm(submit_sm) => submit_sm,
                _ => panic!("Expected PDU to be SubmitSm"),
            };

            assert_eq!(pdu, &SubmitSm::builder().build());
        }
    }
}
