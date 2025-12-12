use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    types::owned::COctetString,
    values::{owned::*, *},
};

/// This command is issued by the ESME to cancel one or more previously submitted short
/// messages that are pending delivery. The command may specify a particular message to
/// cancel, or all messages matching a particular source, destination and service_type.
///
/// If the message_id is set to the ID of a previously submitted message, then provided the
/// source address supplied by the ESME matches that of the stored message, that message
/// will be cancelled.
///
/// If the message_id is NULL, all outstanding undelivered messages with matching source and
/// destination addresses (and service_type if specified) are cancelled.
/// Where the original submit_sm, data_sm or submit_multi ‘source address’ is defaulted to
/// NULL, then the source address in the cancel_sm command should also be NULL.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct CancelSm {
    /// Set to indicate SMS Application service,
    /// if cancellation of a group of application
    /// service messages is desired.
    /// Otherwise set to NULL.
    pub service_type: ServiceType,
    /// Message ID of the message to be
    /// cancelled. This must be the MC
    /// assigned Message ID of the original
    /// message.
    ///
    /// Set to NULL if cancelling a group of
    /// messages.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request PDU.
    ///
    /// If not known, set to NULL.
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message
    /// originator.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request PDU.
    ///
    /// If not known, set to NULL.
    pub source_addr_npi: Npi,
    /// Source address of message(s) to be
    /// cancelled. This is used for verification
    /// purposes, and must match that supplied
    /// in the original message submission
    /// request PDU(s).
    ///
    /// If not known, set to NULL.
    pub source_addr: COctetString<1, 21>,
    /// Type of number of destination SME
    /// address of the message(s) to be cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU (e.g. submit_sm).
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator of destination
    /// SME address of the message(s) to be
    /// cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub dest_addr_npi: Npi,
    /// Destination address of message(s) to be
    /// cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub destination_addr: COctetString<1, 21>,
}

impl CancelSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }

    pub fn builder() -> CancelSmBuilder {
        CancelSmBuilder::new()
    }
}

impl From<CancelSm> for Pdu {
    fn from(value: CancelSm) -> Self {
        Self::CancelSm(value)
    }
}

#[derive(Debug, Default)]
pub struct CancelSmBuilder {
    inner: CancelSm,
}

impl CancelSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.inner.service_type = service_type;
        self
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn source_addr_ton(mut self, source_addr_ton: Ton) -> Self {
        self.inner.source_addr_ton = source_addr_ton;
        self
    }

    pub fn source_addr_npi(mut self, source_addr_npi: Npi) -> Self {
        self.inner.source_addr_npi = source_addr_npi;
        self
    }

    pub fn source_addr(mut self, source_addr: COctetString<1, 21>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn dest_addr_ton(mut self, dest_addr_ton: Ton) -> Self {
        self.inner.dest_addr_ton = dest_addr_ton;
        self
    }

    pub fn dest_addr_npi(mut self, dest_addr_npi: Npi) -> Self {
        self.inner.dest_addr_npi = dest_addr_npi;
        self
    }

    pub fn destination_addr(mut self, destination_addr: COctetString<1, 21>) -> Self {
        self.inner.destination_addr = destination_addr;
        self
    }

    pub fn build(self) -> CancelSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for CancelSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .message_id(COctetString::from_str("message_id").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Unknown)
                    .source_addr(COctetString::from_str("source_addr").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Unknown)
                    .destination_addr(COctetString::from_str("destination_addr").unwrap())
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<CancelSm>();
    }
}
