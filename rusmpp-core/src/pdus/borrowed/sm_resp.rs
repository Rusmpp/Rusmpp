use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{MessageDeliveryResponseTlvValue, Tlv},
    types::borrowed::COctetString,
};

macro_rules! sm_resp {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
        #[rusmpp(decode = borrowed, test = skip)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]

        pub struct $name<'a, const N: usize> {
            /// This field contains the MC message ID of the submitted message.
            /// It may be used at a later stage to query the status of a message,
            /// cancel or replace the message.
            message_id: COctetString<'a, 1, 65>,
            /// Message delivery response TLVs ([`MessageDeliveryResponseTlvValue`])
            #[rusmpp(length = "unchecked")]
            #[cfg_attr(feature = "arbitrary", arbitrary(default))]
            tlvs: heapless::vec::Vec<Tlv<'a>, N>,
        }

        impl<'a, const N: usize> $name<'a, N> {
            pub fn new(
                message_id: COctetString<'a, 1, 65>,
                tlvs: heapless::vec::Vec<MessageDeliveryResponseTlvValue<'a>, N>,
            ) -> Self {
                let tlvs = tlvs.into_iter().map(From::from).collect();

                Self { message_id, tlvs }
            }

            pub fn message_id(&'_ self) -> &'_ COctetString<'a, 1, 65> {
                &self.message_id
            }

            pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
                &self.tlvs
            }

            pub fn set_tlvs(
                &mut self,
                tlvs: heapless::vec::Vec<MessageDeliveryResponseTlvValue<'a>, N>,
            ) {
                self.tlvs = tlvs.into_iter().map(From::from).collect();
            }

            pub fn push_tlv(&mut self, tlv: impl Into<MessageDeliveryResponseTlvValue<'a>>) -> Result<(), Tlv<'a>> {
                self.tlvs.push(Tlv::from(tlv.into()))?;
                Ok(())
            }

            ::pastey::paste! {
                pub fn builder() -> [<$name Builder>]<'a, N> {
                    [<$name Builder>]::new()
                }
            }
        }

        ::pastey::paste! {
            #[derive(Debug, Default)]
            pub struct [<$name Builder>]<'a, const N: usize> {
               inner: $name<'a, N>,
            }

            impl<'a, const N: usize> [<$name Builder>]<'a, N> {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
                    self.inner.message_id = message_id;
                    self
                }

                pub fn tlvs(
                    mut self,
                    tlvs: heapless::vec::Vec<MessageDeliveryResponseTlvValue<'a>, N>,
                ) -> Self {
                    self.inner.set_tlvs(tlvs);
                    self
                }

                pub fn push_tlv(mut self, tlv: impl Into<MessageDeliveryResponseTlvValue<'a>>) -> Result<Self, Tlv<'a>> {
                    self.inner.push_tlv(tlv)?;
                    Ok(self)
                }

                pub fn build(self) -> $name<'a, N> {
                    self.inner
                }
            }
        }
    };
}

sm_resp!(DeliverSmResp);
sm_resp!(DataSmResp);

impl<'a, const N: usize> From<DeliverSmResp<'a, N>> for Pdu<'a, N> {
    fn from(value: DeliverSmResp<'a, N>) -> Self {
        Self::DeliverSmResp(value)
    }
}

impl<'a, const N: usize> From<DataSmResp<'a, N>> for Pdu<'a, N> {
    fn from(value: DataSmResp<'a, N>) -> Self {
        Self::DataSmResp(value)
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::{tests::TestInstance, values::*};

    use super::*;

    impl<const N: usize> TestInstance for DeliverSmResp<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"123456789012345678\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"123456789012345678\0").unwrap())
                    .tlvs(
                        [
                            MessageDeliveryResponseTlvValue::AdditionalStatusInfoText(
                                COctetString::new(b"Octets\0").unwrap(),
                            ),
                            MessageDeliveryResponseTlvValue::DeliveryFailureReason(
                                DeliveryFailureReason::TemporaryNetworkError,
                            ),
                        ]
                        .into()
                    )
                    .build(),
            ]
        }
    }

    impl<const N: usize> TestInstance for DataSmResp<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"123456789012345678\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"123456789012345678\0").unwrap())
                    .tlvs(
                        [
                            MessageDeliveryResponseTlvValue::AdditionalStatusInfoText(
                                COctetString::new(b"Octets on steroids\0").unwrap(),
                            ),
                            MessageDeliveryResponseTlvValue::NetworkErrorCode(
                                NetworkErrorCode::new(ErrorCodeNetworkType::SmppError, 1,)
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
            DeliverSmResp<'static, 16>,
        >();
        crate::tests::borrowed::encode_decode_with_length_test_instances::<DataSmResp<'static, 16>>(
        );
    }
}
