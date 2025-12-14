use rusmpp_macros::Rusmpp;

use crate::{CommandId, CommandStatus, pdus::owned::Pdu};

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
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Command {
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
    pdu: Option<Pdu>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            id: CommandId::EnquireLink,
            status: CommandStatus::EsmeRok,
            sequence_number: 0,
            pdu: Some(Pdu::EnquireLink),
        }
    }
}

impl Command {
    pub fn new(status: CommandStatus, sequence_number: u32, pdu: impl Into<Pdu>) -> Self {
        Self::new_const(status, sequence_number, pdu.into())
    }

    pub const fn new_const(status: CommandStatus, sequence_number: u32, pdu: Pdu) -> Self {
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
    pub const fn pdu(&self) -> Option<&Pdu> {
        self.pdu.as_ref()
    }

    #[inline]
    pub fn set_pdu(&mut self, pdu: impl Into<Pdu>) {
        let pdu = pdu.into();

        self.id = pdu.command_id();

        self.pdu = Some(pdu);
    }

    #[inline]
    pub fn builder() -> CommandStatusBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct CommandStatusBuilder {
    inner: Command,
}

impl CommandStatusBuilder {
    #[inline]
    pub fn status(mut self, status: CommandStatus) -> SequenceNumberBuilder {
        self.inner.status = status;

        SequenceNumberBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct SequenceNumberBuilder {
    inner: Command,
}

impl SequenceNumberBuilder {
    #[inline]
    pub fn sequence_number(mut self, sequence_number: u32) -> PduBuilder {
        self.inner.sequence_number = sequence_number;

        PduBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct PduBuilder {
    inner: Command,
}

impl PduBuilder {
    #[inline]
    pub fn pdu(mut self, pdu: impl Into<Pdu>) -> Command {
        self.inner.set_pdu(pdu);
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<Command>();
    }
}
