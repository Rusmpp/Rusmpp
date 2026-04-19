use rusmpp_macros::Rusmpp;

use crate::{
    decode::{
        DecodeError, DecodeResultExt,
        borrowed::{Decode, DecodeWithKey},
    },
    encode::Length,
    types::borrowed::COctetString,
    values::{DestFlag, npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct DestAddress<'a> {
    flag: DestFlag,
    #[rusmpp(key = flag)]
    value: DestAddressValue<'a>,
}

impl<'a> DestAddress<'a> {
    pub fn new(value: impl Into<DestAddressValue<'a>>) -> Self {
        let value = value.into();
        let flag = value.flag();

        Self { flag, value }
    }

    pub const fn flag(&self) -> DestFlag {
        self.flag
    }

    pub const fn value(&self) -> &DestAddressValue<'a> {
        &self.value
    }
}

impl<'a> From<DestAddressValue<'a>> for DestAddress<'a> {
    fn from(value: DestAddressValue<'a>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub enum DestAddressValue<'a> {
    /// SME Format Destination Address.
    SmeAddress(SmeAddress<'a>),
    /// Distribution List Format Destination Address.
    DistributionListName(DistributionListName<'a>),
}

impl<'a> DestAddressValue<'a> {
    pub const fn flag(&self) -> DestFlag {
        match self {
            Self::SmeAddress(_) => DestFlag::SmeAddress,
            Self::DistributionListName(_) => DestFlag::DistributionListName,
        }
    }
}

impl<'a> Length for DestAddressValue<'a> {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

impl<'a> crate::encode::Encode for DestAddressValue<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> crate::encode::owned::Encode for DestAddressValue<'a> {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl<'a> DecodeWithKey<'a> for DestAddressValue<'a> {
    type Key = DestFlag;

    fn decode(key: Self::Key, src: &'a [u8], _: usize) -> Result<(Self, usize), DecodeError> {
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
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct SmeAddress<'a> {
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this short message. For mobile
    /// terminated messages, this is the directory number of the
    /// recipient MS.
    pub destination_addr: COctetString<'a, 1, 21>,
}

impl<'a> SmeAddress<'a> {
    pub const fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<'a, 1, 21>,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }
}

impl<'a> From<SmeAddress<'a>> for DestAddressValue<'a> {
    fn from(val: SmeAddress<'a>) -> Self {
        DestAddressValue::SmeAddress(val)
    }
}

/// Distribution List Format Destination Address.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct DistributionListName<'a> {
    /// Name of Distribution List.
    pub dl_name: COctetString<'a, 1, 21>,
}

impl<'a> DistributionListName<'a> {
    pub const fn new(dl_name: COctetString<'a, 1, 21>) -> Self {
        Self { dl_name }
    }
}

impl<'a> From<DistributionListName<'a>> for DestAddressValue<'a> {
    fn from(val: DistributionListName<'a>) -> Self {
        DestAddressValue::DistributionListName(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for DestAddress<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::new(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
                Self::new(DistributionListName::new(
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<DestAddress>();
        crate::tests::borrowed::encode_decode_test_instances::<SmeAddress>();
        crate::tests::borrowed::encode_decode_test_instances::<DistributionListName>();
    }
}
