use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{QueryBroadcastResponseTlvValue, Tlv},
    types::owned::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct QueryBroadcastSmResp {
    /// Message ID of the queried message. This must be the MC
    /// assigned Message ID allocated to the original short message
    /// when submitted to the MC by the broadcast_sm, command, and
    /// returned in the broadcast_sm_resp PDU by the MC.
    pub message_id: COctetString<1, 65>,
    /// Query broadcast response TLVs ([`QueryBroadcastResponseTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl QueryBroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        tlvs: alloc::vec::Vec<impl Into<QueryBroadcastResponseTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<QueryBroadcastResponseTlvValue>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<QueryBroadcastResponseTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> QueryBroadcastSmRespBuilder {
        QueryBroadcastSmRespBuilder::new()
    }
}

impl From<QueryBroadcastSmResp> for Pdu {
    fn from(value: QueryBroadcastSmResp) -> Self {
        Self::QueryBroadcastSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct QueryBroadcastSmRespBuilder {
    inner: QueryBroadcastSmResp,
}

impl QueryBroadcastSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: alloc::vec::Vec<impl Into<QueryBroadcastResponseTlvValue>>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<QueryBroadcastResponseTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> QueryBroadcastSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, types::owned::OctetString, values::*};

    use super::*;

    impl TestInstance for QueryBroadcastSmResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("12345678901234567890123").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("12345678901234567890123").unwrap())
                    .push_tlv(QueryBroadcastResponseTlvValue::BroadcastEndTime(
                        OctetString::from_str("2023-10-01").unwrap(),
                    ))
                    .push_tlv(QueryBroadcastResponseTlvValue::UserMessageReference(
                        UserMessageReference::new(69),
                    ))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<QueryBroadcastSmResp>();
    }
}
