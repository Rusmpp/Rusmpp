use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::{AnyOctetString, COctetString},
    values::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub enum MessageSubmissionResponseTlvValue<'a> {
    AdditionalStatusInfoText(COctetString<'a, 1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    DpfResult(DpfResult),
    NetworkErrorCode(NetworkErrorCode),
    Other {
        tag: TlvTag,
        value: AnyOctetString<'a>,
    },
}
