use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{BroadcastResponseTlvValue, Tlv},
    types::owned::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct BroadcastSmResp {
    /// This field contains the MC message ID of the submitted
    /// message. It may be used at a later stage to perform
    /// subsequent operations on the message.
    pub message_id: COctetString<1, 65>,
    /// Broadcast response TLVs ([`BroadcastResponseTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl BroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        tlvs: alloc::vec::Vec<impl Into<BroadcastResponseTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<BroadcastResponseTlvValue>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<BroadcastResponseTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> BroadcastSmRespBuilder {
        BroadcastSmRespBuilder::new()
    }
}

impl From<BroadcastSmResp> for Pdu {
    fn from(value: BroadcastSmResp) -> Self {
        Self::BroadcastSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct BroadcastSmRespBuilder {
    inner: BroadcastSmResp,
}

impl BroadcastSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(mut self, tlvs: alloc::vec::Vec<impl Into<BroadcastResponseTlvValue>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<BroadcastResponseTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> BroadcastSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        CommandStatus,
        tests::TestInstance,
        types::owned::AnyOctetString,
        values::{owned::*, *},
    };

    use super::*;

    impl TestInstance for BroadcastSmResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(
                        COctetString::from_str("12345678901234567890123456789012345678901234")
                            .unwrap(),
                    )
                    .build(),
                Self::builder()
                    .message_id(
                        COctetString::from_str("12345678901234567890123456789012345678901234")
                            .unwrap(),
                    )
                    .push_tlv(BroadcastResponseTlvValue::BroadcastErrorStatus(
                        CommandStatus::EsmeRalybnd,
                    ))
                    .build(),
                Self::builder()
                    .message_id(
                        COctetString::from_str("12345678901234567890123456789012345678901234")
                            .unwrap(),
                    )
                    .tlvs(alloc::vec![
                        BroadcastResponseTlvValue::BroadcastErrorStatus(
                            CommandStatus::EsmeRbcastcancelfail,
                        ),
                        BroadcastResponseTlvValue::BroadcastAreaIdentifier(
                            BroadcastAreaIdentifier::new(
                                BroadcastAreaFormat::Polygon,
                                AnyOctetString::new(b"Polygon Area".to_vec()),
                            ),
                        ),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<BroadcastSmResp>();
    }
}
