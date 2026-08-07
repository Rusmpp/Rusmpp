use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    tlvs::{owned::TlvValue, tag::TlvTag},
};

mod broadcast_request;
pub use broadcast_request::*;

mod broadcast_response;
pub use broadcast_response::*;

mod cancel_broadcast;
pub use cancel_broadcast::*;

mod message_delivery_request;
pub use message_delivery_request::*;

mod message_delivery_response;
pub use message_delivery_response::*;

mod message_submission_request;
pub use message_submission_request::*;

mod message_submission_response;
pub use message_submission_response::*;

mod query_broadcast_response;
pub use query_broadcast_response::*;

/// See module level documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Tlv {
    tag: TlvTag,
    value_length: u16,
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue>,
}

impl Tlv {
    pub fn new(value: impl Into<TlvValue>) -> Self {
        let value = value.into();
        let tag = value.tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    pub const fn tag(&self) -> TlvTag {
        self.tag
    }

    pub const fn value_length(&self) -> u16 {
        self.value_length
    }

    pub const fn value(&self) -> Option<&TlvValue> {
        self.value.as_ref()
    }
}

impl From<TlvValue> for Tlv {
    fn from(value: TlvValue) -> Self {
        Self::new(value)
    }
}
