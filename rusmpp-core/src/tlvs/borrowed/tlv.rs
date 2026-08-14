use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    tlvs::{borrowed::TlvValue, tag::TlvTag},
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

// TODO: serde like owned.

/// See module level documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub struct Tlv<'a> {
    tag: TlvTag,
    value_length: u16,
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue<'a>>,
}

impl<'a> Tlv<'a> {
    pub fn new(value: impl Into<TlvValue<'a>>) -> Self {
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

    pub const fn value(&'_ self) -> Option<&'_ TlvValue<'_>> {
        self.value.as_ref()
    }
}

impl<'a> From<TlvValue<'a>> for Tlv<'a> {
    fn from(value: TlvValue<'a>) -> Self {
        Self::new(value)
    }
}
