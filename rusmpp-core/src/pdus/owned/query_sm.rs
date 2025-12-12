use rusmpp_macros::Rusmpp;

use crate::{pdus::owned::Pdu, types::owned::COctetString, values::*};

/// This command is issued by the ESME to query the status of a previously submitted short
/// message.
/// The matching mechanism is based on the MC assigned message_id and source address.
/// Where the original submit_sm, data_sm or submit_multi ‘source address’ was defaulted to
/// NULL, then the source address in the query_sm command should also be set to NULL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct QuerySm {
    /// Message ID of the message whose state
    /// is to be queried. This must be the MC
    /// assigned Message ID allocated to the
    /// original short message when submitted
    /// to the MC by the submit_sm, data_sm or
    /// submit_multi command, and returned in
    /// the response PDU by the MC.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original request PDU (e.g. submit_sm).
    ///
    /// If not known, set to NULL.
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message
    /// originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.
    ///
    /// If not known, set to NULL.
    pub source_addr_npi: Npi,
    /// Address of message originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original request PDU (e.g. submit_sm).
    ///
    /// If not known, set to NULL.
    pub source_addr: COctetString<1, 21>,
}

impl QuerySm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        }
    }

    pub fn builder() -> QuerySmBuilder {
        QuerySmBuilder::new()
    }
}

impl From<QuerySm> for Pdu {
    fn from(value: QuerySm) -> Self {
        Self::QuerySm(value)
    }
}

#[derive(Debug, Default)]
pub struct QuerySmBuilder {
    inner: QuerySm,
}

impl QuerySmBuilder {
    pub fn new() -> Self {
        Default::default()
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

    pub fn build(self) -> QuerySm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for QuerySm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890123456").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("Source Addr").unwrap())
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<QuerySm>();
    }
}
