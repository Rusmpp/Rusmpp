use rusmpp_macros::Rusmpp;

use crate::{pdus::owned::Pdu, types::owned::COctetString};

/// Authentication PDU used by a Message Centre to Outbind to
/// an ESME to inform it that messages are present in the MC.
/// The PDU contains identification, and access password for the
/// ESME. If the ESME authenticates the request, it will respond
/// with a bind_receiver or bind_transceiver to begin the process
/// of binding into the MC.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Outbind {
    /// MC identifier.
    ///
    /// Identifies the MC to the ESME.
    pub system_id: COctetString<1, 16>,
    /// The password may be used by the
    /// ESME for security reasons to
    /// authenticate the MC originating the
    /// outbind.
    pub password: COctetString<1, 9>,
}

impl Outbind {
    pub fn new(system_id: COctetString<1, 16>, password: COctetString<1, 9>) -> Self {
        Self {
            system_id,
            password,
        }
    }

    pub fn builder() -> OutbindBuilder {
        OutbindBuilder::new()
    }
}

impl From<Outbind> for Pdu {
    fn from(value: Outbind) -> Self {
        Self::Outbind(value)
    }
}

#[derive(Debug, Default)]
pub struct OutbindBuilder {
    inner: Outbind,
}

impl OutbindBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.inner.system_id = system_id;
        self
    }

    pub fn password(mut self, password: COctetString<1, 9>) -> Self {
        self.inner.password = password;
        self
    }

    pub fn build(self) -> Outbind {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for Outbind {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .system_id(COctetString::from_str("system_id").unwrap())
                    .password(COctetString::from_str("password").unwrap())
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Outbind>();
    }
}
