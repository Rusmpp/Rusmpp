use bytes::BytesMut;
use rusmpp_macros::Rusmpp;

use crate::{
    Sealed,
    decode::{
        DecodeResultExt,
        owned::{Decode, DecodeErrorType, DecodeWithKey},
    },
    encode::Length,
    types::owned::COctetString,
    values::{DestFlag, npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
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
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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

impl Sealed for DestAddressValue {}

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

#[derive(Debug, Clone, thiserror::Error)]
pub enum DestAddressValueDecodeError {
    #[error("SmeAddress decode error: {0}")]
    SmeAddress(
        #[from]
        #[source]
        SmeAddressDecodeError,
    ),
    #[error("DistributionListName decode error: {0}")]
    DistributionListName(
        #[from]
        #[source]
        DistributionListNameDecodeError,
    ),
    #[error("Unsupported DestFlag: {0:?}")]
    UnsupportedFlag(DestFlag),
}

impl DecodeErrorType for DestAddressValue {
    type Error = DestAddressValueDecodeError;
}

impl DecodeWithKey for DestAddressValue {
    type Key = DestFlag;

    fn decode(key: Self::Key, src: &mut BytesMut, _: usize) -> Result<(Self, usize), Self::Error> {
        let (value, size) = match key {
            DestFlag::SmeAddress => Decode::decode(src).map_decoded(Self::SmeAddress)?,
            DestFlag::DistributionListName => {
                Decode::decode(src).map_decoded(Self::DistributionListName)?
            }
            DestFlag::Other(_) => return Err(Self::Error::UnsupportedFlag(key)),
        };

        Ok((value, size))
    }
}

/// SME Format Destination Address.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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

#[cfg(feature = "serde")]
const _: () = {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize)]
    #[serde(transparent)]
    struct SerDestAddress<'a> {
        value: &'a DestAddressValue,
    }

    impl<'a> From<&'a DestAddress> for SerDestAddress<'a> {
        fn from(dest_address: &'a DestAddress) -> Self {
            Self {
                value: &dest_address.value,
            }
        }
    }

    impl Serialize for DestAddress {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            SerDestAddress::from(self).serialize(serializer)
        }
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    struct DeDestAddress {
        value: DestAddressValue,
    }

    impl From<DeDestAddress> for DestAddress {
        fn from(dest_address: DeDestAddress) -> Self {
            Self::new(dest_address.value)
        }
    }

    impl<'de> Deserialize<'de> for DestAddress {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let dest_address = DeDestAddress::deserialize(deserializer)?;

            Ok(Self::from(dest_address))
        }
    }
};

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
