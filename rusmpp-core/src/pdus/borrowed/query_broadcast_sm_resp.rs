use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{QueryBroadcastResponseTlvValue, Tlv},
    types::borrowed::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct QueryBroadcastSmResp<'a, const N: usize> {
    /// Message ID of the queried message. This must be the MC
    /// assigned Message ID allocated to the original short message
    /// when submitted to the MC by the broadcast_sm, command, and
    /// returned in the broadcast_sm_resp PDU by the MC.
    pub message_id: COctetString<'a, 1, 65>,
    /// Query broadcast response TLVs ([`QueryBroadcastResponseTlvValue`]).
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> QueryBroadcastSmResp<'a, N> {
    pub fn new(
        message_id: COctetString<'a, 1, 65>,
        tlvs: heapless::vec::Vec<QueryBroadcastResponseTlvValue<'a>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<QueryBroadcastResponseTlvValue<'a>, N>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<QueryBroadcastResponseTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> QueryBroadcastSmRespBuilder<'a, N> {
        QueryBroadcastSmRespBuilder::new()
    }
}

impl<'a, const N: usize> From<QueryBroadcastSmResp<'a, N>> for Pdu<'a, N> {
    fn from(value: QueryBroadcastSmResp<'a, N>) -> Self {
        Self::QueryBroadcastSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct QueryBroadcastSmRespBuilder<'a, const N: usize> {
    inner: QueryBroadcastSmResp<'a, N>,
}

impl<'a, const N: usize> QueryBroadcastSmRespBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(mut self, tlvs: heapless::vec::Vec<QueryBroadcastResponseTlvValue<'a>, N>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(
        mut self,
        tlv: impl Into<QueryBroadcastResponseTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> QueryBroadcastSmResp<'a, N> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::{tests::TestInstance, types::borrowed::OctetString, values::*};

    use super::*;

    impl<const N: usize> TestInstance for QueryBroadcastSmResp<'_, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"12345678901234567890123\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"12345678901234567890123\0").unwrap())
                    .push_tlv(QueryBroadcastResponseTlvValue::BroadcastEndTime(
                        OctetString::new(b"2023-10-01\0").unwrap(),
                    ))
                    .unwrap()
                    .push_tlv(QueryBroadcastResponseTlvValue::UserMessageReference(
                        UserMessageReference::new(69),
                    ))
                    .unwrap()
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            QueryBroadcastSmResp<'static, 16>,
        >();
    }
}
