use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{Tlv, TlvValue},
    types::owned::COctetString,
    values::*,
};

macro_rules! bind_resp {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
        #[rusmpp(decode = owned, test = skip)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
        #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
        pub struct $name {
            /// MC identifier.
            ///
            /// Identifies the MC to the ESME.
            pub system_id: COctetString<1, 16>,
            /// `SMPP` version supported by MC. [`ScInterfaceVersion`].
            #[rusmpp(length = "checked")]
            sc_interface_version: Option<Tlv>,
        }

        impl $name {
            pub fn new(
                system_id: COctetString<1, 16>,
                sc_interface_version: Option<InterfaceVersion>,
            ) -> Self {
                Self {
                    system_id,
                    sc_interface_version: sc_interface_version
                        .map(TlvValue::ScInterfaceVersion)
                        .map(From::from),
                }
            }

            pub const fn sc_interface_version_tlv(&self) -> Option<&Tlv> {
                self.sc_interface_version.as_ref()
            }

            pub fn sc_interface_version(&self) -> Option<InterfaceVersion> {
                self.sc_interface_version_tlv()
                    .and_then(|tlv| match tlv.value() {
                        Some(TlvValue::ScInterfaceVersion(value)) => Some(value),
                        _ => None,
                    })
                    .copied()
            }

            pub fn set_sc_interface_version(
                &mut self,
                sc_interface_version: Option<InterfaceVersion>,
            ) {
                self.sc_interface_version = sc_interface_version
                    .map(TlvValue::ScInterfaceVersion)
                    .map(From::from);
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

                pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
                    self.inner.system_id = system_id;
                    self
                }

                pub fn sc_interface_version(
                    mut self,
                    sc_interface_version: Option<InterfaceVersion>,
                ) -> Self {
                    self.inner.set_sc_interface_version(sc_interface_version);
                    self
                }

                pub fn build(self) -> $name {
                    self.inner
                }
            }
        }
    };
}

bind_resp!(BindTransmitterResp);
bind_resp!(BindReceiverResp);
bind_resp!(BindTransceiverResp);

impl From<BindTransmitterResp> for Pdu {
    fn from(value: BindTransmitterResp) -> Self {
        Self::BindTransmitterResp(value)
    }
}

impl From<BindReceiverResp> for Pdu {
    fn from(value: BindReceiverResp) -> Self {
        Self::BindReceiverResp(value)
    }
}

impl From<BindTransceiverResp> for Pdu {
    fn from(value: BindTransceiverResp) -> Self {
        Self::BindTransceiverResp(value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for BindTransmitterResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::from_str("system_id").unwrap())
                    .sc_interface_version(Some(InterfaceVersion::Smpp5_0))
                    .build(),
            ]
        }
    }

    impl TestInstance for BindReceiverResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::from_str("system_id").unwrap())
                    .sc_interface_version(Some(InterfaceVersion::Smpp3_4))
                    .build(),
            ]
        }
    }

    impl TestInstance for BindTransceiverResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::from_str("system_id").unwrap())
                    .sc_interface_version(Some(InterfaceVersion::Smpp3_3OrEarlier(1)))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<BindTransmitterResp>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<BindReceiverResp>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<BindTransceiverResp>();
    }
}
