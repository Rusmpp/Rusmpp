use bytes::BytesMut;
use rusmpp_macros::Rusmpp;

use crate::{
    decode::{
        DecodeError, DecodeResultExt,
        owned::{Decode, DecodeErrorType, DecodeWithKey},
    },
    encode::Length,
    types::owned::COctetString,
    values::{DestFlag, npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct DestAddress {
    flag: DestFlag,
    #[rusmpp(key = flag)]
    value: DestAddressValue,
}

impl DestAddress {
    pub fn new(value: impl Into<DestAddressValue>) -> Self {
        let value = value.into();
        let flag = value.flag();

        Self { flag, value }
    }

    pub const fn flag(&self) -> DestFlag {
        self.flag
    }

    pub const fn value(&self) -> &DestAddressValue {
        &self.value
    }
}

impl From<DestAddressValue> for DestAddress {
    fn from(value: DestAddressValue) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DestAddressValue {
    /// SME Format Destination Address.
    SmeAddress(SmeAddress),
    /// Distribution List Format Destination Address.
    DistributionListName(DistributionListName),
}

impl DestAddressValue {
    pub const fn flag(&self) -> DestFlag {
        match self {
            Self::SmeAddress(_) => DestFlag::SmeAddress,
            Self::DistributionListName(_) => DestFlag::DistributionListName,
        }
    }
}

impl Length for DestAddressValue {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

impl crate::encode::Encode for DestAddressValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl crate::encode::owned::Encode for DestAddressValue {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl DecodeErrorType for DestAddressValue {
    // TODO
    type Error = core::convert::Infallible;
}

impl DecodeWithKey for DestAddressValue {
    type Key = DestFlag;

    fn decode(key: Self::Key, src: &mut BytesMut, _: usize) -> Result<(Self, usize), DecodeError> {
        let (value, size) = match key {
            DestFlag::SmeAddress => Decode::decode(src).map_decoded(Self::SmeAddress)?,
            DestFlag::DistributionListName => {
                Decode::decode(src).map_decoded(Self::DistributionListName)?
            }
            DestFlag::Other(flag) => return Err(DecodeError::unsupported_key(flag.into())),
        };

        Ok((value, size))
    }
}

/// SME Format Destination Address.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SmeAddress {
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this short message. For mobile
    /// terminated messages, this is the directory number of the
    /// recipient MS.
    pub destination_addr: COctetString<1, 21>,
}

impl SmeAddress {
    pub const fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }
}

impl From<SmeAddress> for DestAddressValue {
    fn from(val: SmeAddress) -> Self {
        DestAddressValue::SmeAddress(val)
    }
}

/// Distribution List Format Destination Address.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct DistributionListName {
    /// Name of Distribution List.
    pub dl_name: COctetString<1, 21>,
}

impl DistributionListName {
    pub const fn new(dl_name: COctetString<1, 21>) -> Self {
        Self { dl_name }
    }
}

impl From<DistributionListName> for DestAddressValue {
    fn from(val: DistributionListName) -> Self {
        DestAddressValue::DistributionListName(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for DestAddress {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::new(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
                Self::new(DistributionListName::new(
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<DestAddress>();
        crate::tests::owned::encode_decode_test_instances::<SmeAddress>();
        crate::tests::owned::encode_decode_test_instances::<DistributionListName>();
    }
}
