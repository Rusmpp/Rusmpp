use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{Tlv, TlvValue},
    types::owned::COctetString,
    values::*,
};

/// This command is issued by the ESME to query the status of a previously submitted
/// broadcast message. The message can be queried either on the basis of the Message Center
/// assigned reference message_id returned in the broadcast_sm_resp or by the ESME
/// assigned message reference number user_message_reference as indicated in the
/// broadcast_sm operation associated with that message.
///
/// Note:  Where the broadcast is queried on the basis of the ESME assigned message
/// reference user_message_reference this should be qualified within the service by the
/// system_id and/or the system_type associated with the query_broadcast_sm operation
/// (specified in the bind operation). If more than one message with the same
/// user_message_reference value is present in the Message Center, the details of the most
/// recently submitted message with the specified user_message_reference value will be
/// returned in the query_broadcast_sm_resp.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct QueryBroadcastSm {
    /// Message ID of the message to be queried. This must be
    /// the MC assigned Message ID allocated to the original
    /// short message when submitted to the MC by the
    /// broadcast_sm command, and returned in the response
    /// PDU by the MC.
    ///
    /// Set to NULL if setting user_message_reference.
    pub message_id: COctetString<1, 65>,
    /// Type of Number for source address.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_npi: Npi,
    /// Address of SME which originated this message.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// ESME assigned message reference number. [`UserMessageReference`].
    #[rusmpp(length = "checked")]
    user_message_reference: Option<Tlv>,
}

impl QueryBroadcastSm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        user_message_reference: Option<UserMessageReference>,
    ) -> Self {
        let user_message_reference = user_message_reference
            .map(TlvValue::UserMessageReference)
            .map(From::from);

        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            user_message_reference,
        }
    }

    pub const fn user_message_reference_tlv(&self) -> Option<&Tlv> {
        self.user_message_reference.as_ref()
    }

    pub fn user_message_reference(&self) -> Option<UserMessageReference> {
        self.user_message_reference_tlv()
            .and_then(|tlv| match tlv.value() {
                Some(TlvValue::UserMessageReference(value)) => Some(value),
                _ => None,
            })
            .copied()
    }

    pub fn set_user_message_reference(
        &mut self,
        user_message_reference: Option<UserMessageReference>,
    ) {
        self.user_message_reference = user_message_reference
            .map(TlvValue::UserMessageReference)
            .map(From::from);
    }

    pub fn builder() -> QueryBroadcastSmBuilder {
        QueryBroadcastSmBuilder::new()
    }
}

impl From<QueryBroadcastSm> for Pdu {
    fn from(value: QueryBroadcastSm) -> Self {
        Self::QueryBroadcastSm(value)
    }
}

#[derive(Debug, Default)]
pub struct QueryBroadcastSmBuilder {
    inner: QueryBroadcastSm,
}

impl QueryBroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
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

    pub fn user_message_reference(
        mut self,
        user_message_reference: Option<UserMessageReference>,
    ) -> Self {
        self.inner
            .set_user_message_reference(user_message_reference);
        self
    }

    pub fn build(self) -> QueryBroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for QueryBroadcastSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678901234567890").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678901234567890").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .user_message_reference(Some(UserMessageReference::new(69)))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<QueryBroadcastSm>();
    }
}
