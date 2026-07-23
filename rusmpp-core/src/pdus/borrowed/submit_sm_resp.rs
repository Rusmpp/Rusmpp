use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{MessageSubmissionResponseTlvValue, Tlv},
    types::borrowed::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct SubmitSmResp<'a, const N: usize> {
    /// This field contains the MC message ID of the submitted message.
    /// It may be used at a later stage to query the status of a message,
    /// cancel or replace the message.
    message_id: COctetString<'a, 1, 65>,
    /// Message submission response TLVs ([`MessageSubmissionResponseTlvValue`])
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> SubmitSmResp<'a, N> {
    pub fn new(
        message_id: COctetString<'a, 1, 65>,
        tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&'_ self) -> &'_ COctetString<'_, 1, 65> {
        &self.message_id
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<MessageSubmissionResponseTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> SubmitSmRespBuilder<'a, N> {
        SubmitSmRespBuilder::new()
    }
}

impl<'a, const N: usize> From<SubmitSmResp<'a, N>> for Pdu<'a, N> {
    fn from(value: SubmitSmResp<'a, N>) -> Self {
        Self::SubmitSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitSmRespBuilder<'a, const N: usize> {
    inner: SubmitSmResp<'a, N>,
}

impl<'a, const N: usize> SubmitSmRespBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(
        mut self,
        tlv: impl Into<MessageSubmissionResponseTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> SubmitSmResp<'a, N> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::{tests::TestInstance, values::*};

    use super::*;

    impl<const N: usize> TestInstance for SubmitSmResp<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"12345678901234567890123\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"12345678901234567890123\0").unwrap())
                    .tlvs(
                        [
                            MessageSubmissionResponseTlvValue::AdditionalStatusInfoText(
                                COctetString::new(b"Octets indeed\0").unwrap(),
                            ),
                            MessageSubmissionResponseTlvValue::DeliveryFailureReason(
                                DeliveryFailureReason::TemporaryNetworkError,
                            ),
                        ]
                        .into()
                    )
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<SubmitSmResp<'static, 16>>(
        );
    }
}
