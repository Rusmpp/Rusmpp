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
        use crate::{
            pdus::borrowed::SubmitSm,
            values::{GenericServiceType, borrowed::ServiceType},
        };

        use super::*;

        #[test]
        fn serialize_deserialize() {
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

            let mut buf = [0u8; 1024];

            let serialized =
                postcard::to_slice(&command, &mut buf[..]).expect("Failed to serialize command");

            let deserialized: Command<'_, 16> =
                postcard::from_bytes(serialized).expect("Failed to deserialize command");

            assert_eq!(command, deserialized);
        }
    }
}
