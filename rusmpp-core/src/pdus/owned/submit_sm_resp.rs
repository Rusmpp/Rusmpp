use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{MessageSubmissionResponseTlvValue, Tlv},
    types::owned::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SubmitSmResp {
    /// This field contains the MC message ID of the submitted message.
    /// It may be used at a later stage to query the status of a message,
    /// cancel or replace the message.
    message_id: COctetString<1, 65>,
    /// Message submission response TLVs ([`MessageSubmissionResponseTlvValue`])
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl SubmitSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(
        &mut self,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionResponseTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> SubmitSmRespBuilder {
        SubmitSmRespBuilder::new()
    }
}

impl From<SubmitSmResp> for Pdu {
    fn from(value: SubmitSmResp) -> Self {
        Self::SubmitSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitSmRespBuilder {
    inner: SubmitSmResp,
}

impl SubmitSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionResponseTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SubmitSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, values::*};

    use super::*;

    impl TestInstance for SubmitSmResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("12345678901234567890123").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("12345678901234567890123").unwrap())
                    .tlvs(alloc::vec![
                        MessageSubmissionResponseTlvValue::AdditionalStatusInfoText(
                            COctetString::from_str("Octets indeed").unwrap(),
                        ),
                        MessageSubmissionResponseTlvValue::DeliveryFailureReason(
                            DeliveryFailureReason::TemporaryNetworkError,
                        ),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<SubmitSmResp>();
    }
}
