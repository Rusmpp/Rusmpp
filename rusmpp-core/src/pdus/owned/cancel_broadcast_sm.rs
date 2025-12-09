use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{CancelBroadcastTlvValue, Tlv},
    types::owned::COctetString,
    values::{owned::*, *},
};
/// This command is issued by the ESME to cancel a broadcast message which has been
/// previously submitted to the Message Centre for broadcast via broadcast_sm and which is still
/// pending delivery.
///
/// If the message_id is set to the ID of a previously submitted message, then provided the
/// source address supplied by the ESME matches that of the stored message, that message
/// will be cancelled.
///
/// If the message_id is NULL, all outstanding undelivered messages with matching source and
/// destination addresses (and service_type if specified) are cancelled.
///
/// If the user_message_reference is set to the ESME-assigned reference of a previously
/// submitted message, then provided the source address supplied by the ESME matches that of
/// the stored message, that message will be cancelled.
///
/// Where the original broadcast_sm ‘source address’ was defaulted to NULL, then the source
/// address in the cancel_broadcast_sm command should also be NULL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct CancelBroadcastSm {
    /// Set to indicate CBS Application service, if
    /// cancellation of a group of application service
    /// messages is desired.
    ///
    /// Otherwise set to NULL.
    pub service_type: ServiceType,
    /// Message ID of the message to be cancelled. This must
    /// be the MC assigned Message ID of the original message.
    ///
    /// Set to NULL if setting user_message_reference.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator. This is used for
    /// verification purposes, and must match that supplied in
    /// the original message submission request PDU.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message originator.
    ///
    /// This is used for verification purposes, and must match
    /// that supplied in the original message submission
    /// request PDU.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_npi: Npi,
    /// Source address of message to be cancelled. This is used
    /// for verification purposes, and must match that supplied in
    /// the original message submission request PDU.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// Cancel broadcast  TLVs ([`CancelBroadcastTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl CancelBroadcastSm {
    pub fn new(
        service_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        tlvs: alloc::vec::Vec<impl Into<CancelBroadcastTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self {
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<CancelBroadcastTlvValue>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<CancelBroadcastTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> CancelBroadcastSmBuilder {
        CancelBroadcastSmBuilder::new()
    }
}

impl From<CancelBroadcastSm> for Pdu {
    fn from(value: CancelBroadcastSm) -> Self {
        Self::CancelBroadcastSm(value)
    }
}

#[derive(Debug, Default)]
pub struct CancelBroadcastSmBuilder {
    inner: CancelBroadcastSm,
}

impl CancelBroadcastSmBuilder {
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

    pub fn tlvs(mut self, tlvs: alloc::vec::Vec<impl Into<CancelBroadcastTlvValue>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<CancelBroadcastTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> CancelBroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for CancelBroadcastSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Ermes)
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .tlvs(alloc::vec![
                        CancelBroadcastTlvValue::BroadcastContentType(BroadcastContentType::new(
                            TypeOfNetwork::Gsm,
                            EncodingContentType::BusinessFinancialNewsInternational,
                        )),
                        CancelBroadcastTlvValue::UserMessageReference(UserMessageReference::new(
                            16,
                        )),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<CancelBroadcastSm>();
    }
}
