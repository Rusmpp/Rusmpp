use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{MessageDeliveryResponseTlvValue, Tlv},
    types::owned::COctetString,
};

macro_rules! sm_resp {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
        #[rusmpp(decode = owned, test = skip)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
        #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
        pub struct $name {
            /// This field contains the MC message ID of the submitted message.
            /// It may be used at a later stage to query the status of a message,
            /// cancel or replace the message.
            message_id: COctetString<1, 65>,
            /// Message delivery response TLVs ([`MessageDeliveryResponseTlvValue`])
            #[rusmpp(length = "unchecked")]
            tlvs: alloc::vec::Vec<Tlv>,
        }

        impl $name {
            pub fn new(
                message_id: COctetString<1, 65>,
                tlvs: alloc::vec::Vec<impl Into<MessageDeliveryResponseTlvValue>>,
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
                tlvs: alloc::vec::Vec<impl Into<MessageDeliveryResponseTlvValue>>,
            ) {
                self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
            }

            pub fn push_tlv(&mut self, tlv: impl Into<MessageDeliveryResponseTlvValue>) {
                self.tlvs.push(Tlv::from(tlv.into()));
            }

            ::pastey::paste! {
                pub fn builder() -> [<$name Builder>] {
                    [<$name Builder>]::new()
                }
            }
        }

        ::pastey::paste! {
            #[derive(Debug, Default)]
            pub struct [<$name Builder>] {
               inner: $name,
            }

            impl [<$name Builder>] {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
                    self.inner.message_id = message_id;
                    self
                }

                pub fn tlvs(
                    mut self,
                    tlvs: alloc::vec::Vec<impl Into<MessageDeliveryResponseTlvValue>>,
                ) -> Self {
                    self.inner.set_tlvs(tlvs);
                    self
                }

                pub fn push_tlv(mut self, tlv: impl Into<MessageDeliveryResponseTlvValue>) -> Self {
                    self.inner.push_tlv(tlv);
                    self
                }

                pub fn build(self) -> $name {
                    self.inner
                }
            }
        }
    };
}

sm_resp!(DeliverSmResp);
sm_resp!(DataSmResp);

impl From<DeliverSmResp> for Pdu {
    fn from(value: DeliverSmResp) -> Self {
        Self::DeliverSmResp(value)
    }
}

impl From<DataSmResp> for Pdu {
    fn from(value: DataSmResp) -> Self {
        Self::DataSmResp(value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, values::*};

    use super::*;

    impl TestInstance for DeliverSmResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678").unwrap())
                    .tlvs(alloc::vec![
                        MessageDeliveryResponseTlvValue::AdditionalStatusInfoText(
                            COctetString::from_str("Octets").unwrap(),
                        ),
                        MessageDeliveryResponseTlvValue::DeliveryFailureReason(
                            DeliveryFailureReason::TemporaryNetworkError,
                        ),
                    ])
                    .build(),
            ]
        }
    }

    impl TestInstance for DataSmResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678").unwrap())
                    .tlvs(alloc::vec![
                        MessageDeliveryResponseTlvValue::AdditionalStatusInfoText(
                            COctetString::from_str("Octets on steroids").unwrap(),
                        ),
                        MessageDeliveryResponseTlvValue::NetworkErrorCode(NetworkErrorCode::new(
                            ErrorCodeNetworkType::SmppError,
                            1,
                        )),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<DeliverSmResp>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<DataSmResp>();
    }
}
