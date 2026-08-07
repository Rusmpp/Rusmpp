use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::AnyOctetString,
    values::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]

pub enum CancelBroadcastTlvValue {
    /// Specifies the content type of the message.
    BroadcastContentType(BroadcastContentType),
    /// ESME assigned message reference number.
    ///
    /// Note: The message_id field should be set to NULL if
    /// using the user_message_reference TLV.
    UserMessageReference(UserMessageReference),
    Other {
        tag: TlvTag,
        value: AnyOctetString,
    },
}
