use rusmpp_macros::Rusmpp;

use crate::{pdus::borrowed::Pdu, types::borrowed::COctetString};

/// Authentication PDU used by a Message Centre to Outbind to
/// an ESME to inform it that messages are present in the MC.
/// The PDU contains identification, and access password for the
/// ESME. If the ESME authenticates the request, it will respond
/// with a bind_receiver or bind_transceiver to begin the process
/// of binding into the MC.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct Outbind<'a> {
    /// MC identifier.
    ///
    /// Identifies the MC to the ESME.
    pub system_id: COctetString<'a, 1, 16>,
    /// The password may be used by the
    /// ESME for security reasons to
    /// authenticate the MC originating the
    /// outbind.
    pub password: COctetString<'a, 1, 9>,
}

impl<'a> Outbind<'a> {
    pub fn new(system_id: COctetString<'a, 1, 16>, password: COctetString<'a, 1, 9>) -> Self {
        Self {
            system_id,
            password,
        }
    }

    pub fn builder() -> OutbindBuilder<'a> {
        OutbindBuilder::new()
    }
}

impl<'a, const N: usize> From<Outbind<'a>> for Pdu<'a, N> {
    fn from(value: Outbind<'a>) -> Self {
        Self::Outbind(value)
    }
}

#[derive(Debug, Default)]
pub struct OutbindBuilder<'a> {
    inner: Outbind<'a>,
}

impl<'a> OutbindBuilder<'a> {
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

    pub fn build(self) -> Outbind<'a> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for Outbind<'_> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::new(b"system_id\0").unwrap())
                    .password(COctetString::new(b"password\0").unwrap())
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<Outbind>();
    }
}
