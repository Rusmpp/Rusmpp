use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    types::borrowed::COctetString,
    values::{InterfaceVersion, Npi, Ton},
};

macro_rules! bind {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
        #[rusmpp(decode = borrowed, test = skip)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]

        pub struct $name<'a> {
            /// Identifies the ESME system
            /// requesting to bind with the MC.
            pub system_id: COctetString<'a, 1, 16>,
            /// The password may be used by the
            /// MC to authenticate the ESME
            /// requesting to bind.
            pub password: COctetString<'a, 1, 9>,
            /// Identifies the type of ESME system
            /// requesting to bind with the MC.
            pub system_type: COctetString<'a, 1, 13>,
            /// Identifies the version of the `SMPP`
            /// protocol supported by the ESME.
            pub interface_version: InterfaceVersion,
            /// Type of Number (TON) for ESME
            /// address(es) served via this `SMPP` session.
            ///
            /// Set to NULL (Unknown) if not known.
            pub addr_ton: Ton,
            /// Numbering Plan Indicator (NPI) for
            /// ESME address(es) served via this `SMPP` session.
            ///
            /// Set to NULL (Unknown) if not known.
            pub addr_npi: Npi,
            /// A single ESME address or a range of
            /// ESME addresses served via this `SMPP` session.
            ///
            /// Set to NULL if not known.
            pub address_range: COctetString<'a, 1, 41>,
        }

        impl<'a> $name<'a> {
            pub const fn new(
                system_id: COctetString<'a, 1, 16>,
                password: COctetString<'a, 1, 9>,
                system_type: COctetString<'a, 1, 13>,
                interface_version: InterfaceVersion,
                addr_ton: Ton,
                addr_npi: Npi,
                address_range: COctetString<'a, 1, 41>,
            ) -> Self {
                Self {
                    system_id,
                    password,
                    system_type,
                    interface_version,
                    addr_ton,
                    addr_npi,
                    address_range,
                }
            }

            ::pastey::paste! {
                pub fn builder() -> [<$name Builder>]<'a> {
                    [<$name Builder>]::new()
                }
            }
        }

        ::pastey::paste! {
            #[derive(Debug, Default)]
            pub struct [<$name Builder>]<'a> {
               inner: $name<'a>,
            }

            impl<'a> [<$name Builder>]<'a> {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn system_id(mut self, system_id: COctetString<'a, 1, 16>) -> Self {
                    self.inner.system_id = system_id;
                    self
                }

                pub fn password(mut self, password: COctetString<'a, 1, 9>) -> Self {
                    self.inner.password = password;
                    self
                }

                pub fn system_type(mut self, system_type: COctetString<'a, 1, 13>) -> Self {
                    self.inner.system_type = system_type;
                    self
                }

                pub const fn interface_version(mut self, interface_version: InterfaceVersion) -> Self {
                    self.inner.interface_version = interface_version;
                    self
                }

                pub const fn addr_ton(mut self, addr_ton: Ton) -> Self {
                    self.inner.addr_ton = addr_ton;
                    self
                }

                pub const fn addr_npi(mut self, addr_npi: Npi) -> Self {
                    self.inner.addr_npi = addr_npi;
                    self
                }

                pub fn address_range(mut self, address_range: COctetString<'a, 1, 41>) -> Self {
                    self.inner.address_range = address_range;
                    self
                }

                pub fn build(self) -> $name<'a> {
                    self.inner
                }
            }
        }
    };
}

bind!(BindTransmitter);
bind!(BindReceiver);
bind!(BindTransceiver);

bind!(BindAny);

impl<'a, const N: usize> From<BindTransmitter<'a>> for Pdu<'a, N> {
    fn from(value: BindTransmitter<'a>) -> Self {
        Self::BindTransmitter(value)
    }
}

impl<'a, const N: usize> From<BindReceiver<'a>> for Pdu<'a, N> {
    fn from(value: BindReceiver<'a>) -> Self {
        Self::BindReceiver(value)
    }
}

impl<'a, const N: usize> From<BindTransceiver<'a>> for Pdu<'a, N> {
    fn from(value: BindTransceiver<'a>) -> Self {
        Self::BindTransceiver(value)
    }
}

impl<'a> From<BindAny<'a>> for BindTransmitter<'a> {
    fn from(value: BindAny<'a>) -> Self {
        Self {
            system_id: value.system_id,
            password: value.password,
            system_type: value.system_type,
            interface_version: value.interface_version,
            addr_ton: value.addr_ton,
            addr_npi: value.addr_npi,
            address_range: value.address_range,
        }
    }
}

impl<'a> From<BindAny<'a>> for BindReceiver<'a> {
    fn from(value: BindAny<'a>) -> Self {
        Self {
            system_id: value.system_id,
            password: value.password,
            system_type: value.system_type,
            interface_version: value.interface_version,
            addr_ton: value.addr_ton,
            addr_npi: value.addr_npi,
            address_range: value.address_range,
        }
    }
}

impl<'a> From<BindAny<'a>> for BindTransceiver<'a> {
    fn from(value: BindAny<'a>) -> Self {
        Self {
            system_id: value.system_id,
            password: value.password,
            system_type: value.system_type,
            interface_version: value.interface_version,
            addr_ton: value.addr_ton,
            addr_npi: value.addr_npi,
            address_range: value.address_range,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for BindTransmitter<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::new(b"system_id\0").unwrap())
                    .password(COctetString::new(b"password\0").unwrap())
                    .system_type(COctetString::new(b"system_type\0").unwrap())
                    .interface_version(InterfaceVersion::Smpp5_0)
                    .addr_ton(Ton::International)
                    .addr_npi(Npi::Isdn)
                    .address_range(COctetString::new(b"address_range\0").unwrap())
                    .build(),
            ]
        }
    }

    impl TestInstance for BindReceiver<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::new(b"system_id\0").unwrap())
                    .password(COctetString::new(b"password\0").unwrap())
                    .system_type(COctetString::new(b"system_type\0").unwrap())
                    .interface_version(InterfaceVersion::Smpp3_4)
                    .addr_ton(Ton::Alphanumeric)
                    .addr_npi(Npi::Ermes)
                    .address_range(COctetString::new(b"address_range\0").unwrap())
                    .build(),
            ]
        }
    }

    impl TestInstance for BindTransceiver<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::new(b"system_id\0").unwrap())
                    .password(COctetString::new(b"password\0").unwrap())
                    .system_type(COctetString::new(b"system_type\0").unwrap())
                    .interface_version(InterfaceVersion::Smpp3_3OrEarlier(2))
                    .addr_ton(Ton::International)
                    .addr_npi(Npi::Ermes)
                    .address_range(COctetString::new(b"address_range\0").unwrap())
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<BindTransmitter<'static>>();
        crate::tests::borrowed::encode_decode_test_instances::<BindReceiver>();
        crate::tests::borrowed::encode_decode_test_instances::<BindTransceiver>();
    }
}
