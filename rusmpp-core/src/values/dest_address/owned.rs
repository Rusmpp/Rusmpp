use rusmpp_macros::Rusmpp;

use crate::{
    decode::{
        DecodeError, DecodeResultExt,
        owned::{Decode, DecodeExt},
    },
    encode::Length,
    types::owned::COctetString,
    values::{dest_address::DestFlag, npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DestAddress {
    /// SME Format Destination Address.
    SmeAddress(SmeAddress),
    /// Distribution List Format Destination Address.
    DistributionListName(DistributionListName),
}

impl Length for DestAddress {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

impl crate::encode::Encode for DestAddress {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl crate::encode::owned::Encode for DestAddress {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl Decode for DestAddress {
    fn decode(src: &mut bytes::BytesMut) -> Result<(Self, usize), DecodeError> {
        let size = 0;

        let (flag, size) = DestFlag::decode_move(src, size)?;

        match flag {
            DestFlag::SmeAddress => {
                SmeAddress::decode_move(src, size).map_decoded(Self::SmeAddress)
            }
            DestFlag::DistributionListName => {
                DistributionListName::decode_move(src, size).map_decoded(Self::DistributionListName)
            }
            DestFlag::Other(flag) => Err(DecodeError::unsupported_key(flag.into())),
        }
    }
}

/// SME Format Destination Address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SmeAddress {
    /// 0x01 (SME Address).
    ///
    /// Can't and shouldn't be updated
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
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
            dest_flag: DestFlag::SmeAddress,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }

    pub fn dest_flag(&self) -> DestFlag {
        self.dest_flag
    }
}

/// Distribution List Format Destination Address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct DistributionListName {
    /// 0x02 (Distribution List).
    ///
    /// Can't and shouldn't be updated.
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    /// Name of Distribution List.
    pub dl_name: COctetString<1, 21>,
}

impl DistributionListName {
    pub fn new(dl_name: COctetString<1, 21>) -> Self {
        Self {
            dest_flag: DestFlag::DistributionListName,
            dl_name,
        }
    }

    pub fn dest_flag(&self) -> DestFlag {
        self.dest_flag
    }
}

#[cfg(test)]
mod tests {
    //! See tests in [borrowed](super::super::borrowed) for more details.

    use super::*;

    impl crate::tests::TestInstance for DestAddress {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::SmeAddress(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
                Self::DistributionListName(DistributionListName::new(
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<DestAddress>();
    }
}
