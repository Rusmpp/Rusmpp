use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{BroadcastResponseTlvValue, Tlv},
    types::borrowed::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct BroadcastSmResp<'a, const N: usize> {
    /// This field contains the MC message ID of the submitted
    /// message. It may be used at a later stage to perform
    /// subsequent operations on the message.
    pub message_id: COctetString<'a, 1, 65>,
    /// Broadcast response TLVs ([`BroadcastResponseTlvValue`]).
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> BroadcastSmResp<'a, N> {
    pub fn new(
        message_id: COctetString<'a, 1, 65>,
        tlvs: heapless::vec::Vec<BroadcastResponseTlvValue<'a>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<BroadcastResponseTlvValue<'a>, N>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<BroadcastResponseTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> BroadcastSmRespBuilder<'a, N> {
        BroadcastSmRespBuilder::new()
    }
}

impl<'a, const N: usize> From<BroadcastSmResp<'a, N>> for Pdu<'a, N> {
    fn from(value: BroadcastSmResp<'a, N>) -> Self {
        Self::BroadcastSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct BroadcastSmRespBuilder<'a, const N: usize> {
    inner: BroadcastSmResp<'a, N>,
}

impl<'a, const N: usize> BroadcastSmRespBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(mut self, tlvs: heapless::vec::Vec<BroadcastResponseTlvValue<'a>, N>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(
        mut self,
        tlv: impl Into<BroadcastResponseTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> BroadcastSmResp<'a, N> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::{
        CommandStatus,
        tests::TestInstance,
        types::borrowed::AnyOctetString,
        values::{borrowed::*, *},
    };

    use super::*;

    impl<const N: usize> TestInstance for BroadcastSmResp<'_, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(
                        COctetString::new(b"12345678901234567890123456789012345678901234\0")
                            .unwrap(),
                    )
                    .build(),
                Self::builder()
                    .message_id(
                        COctetString::new(b"12345678901234567890123456789012345678901234\0")
                            .unwrap(),
                    )
                    .push_tlv(BroadcastResponseTlvValue::BroadcastErrorStatus(
                        CommandStatus::EsmeRalybnd,
                    ))
                    .unwrap()
                    .build(),
                Self::builder()
                    .message_id(
                        COctetString::new(b"12345678901234567890123456789012345678901234\0")
                            .unwrap(),
                    )
                    .tlvs(
                        [
                            BroadcastResponseTlvValue::BroadcastErrorStatus(
                                CommandStatus::EsmeRbcastcancelfail,
                            ),
                            BroadcastResponseTlvValue::BroadcastAreaIdentifier(
                                BroadcastAreaIdentifier::new(
                                    BroadcastAreaFormat::Polygon,
                                    AnyOctetString::new(b"Polygon Area"),
                                ),
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
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            BroadcastSmResp<'static, 16>,
        >();
    }
}
