use rusmpp_macros::Rusmpp;

use crate::{
    decode::{
        DecodeError, DecodeResultExt,
        borrowed::{Decode, DecodeExt},
    },
    encode::Length,
    types::borrowed::COctetString,
    values::{dest_address::DestFlag, npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub enum DestAddress<'a> {
    /// SME Format Destination Address.
    SmeAddress(SmeAddress<'a>),
    /// Distribution List Format Destination Address.
    DistributionListName(DistributionListName<'a>),
}

impl Length for DestAddress<'_> {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

impl crate::encode::Encode for DestAddress<'_> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for DestAddress<'_> {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl<'a> Decode<'a> for DestAddress<'a> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError> {
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
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct SmeAddress<'a> {
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
    pub destination_addr: COctetString<'a, 1, 21>,
}

impl<'a> SmeAddress<'a> {
    pub const fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<'a, 1, 21>,
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
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct DistributionListName<'a> {
    /// 0x02 (Distribution List).
    ///
    /// Can't and shouldn't be updated.
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    /// Name of Distribution List.
    pub dl_name: COctetString<'a, 1, 21>,
}

impl<'a> DistributionListName<'a> {
    pub fn new(dl_name: COctetString<'a, 1, 21>) -> Self {
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
    //! # Note
    //!
    //! [`encode_decode_test_instances`](crate::tests::borrowed::encode_decode_test_instances) will fail for [`SmeAddress`] and [`DistributionListName`]
    //! because they encode the `dest_flag` field but skip decoding it, since it will be extracted while decoding [`DestAddress`].
    //!
    //! Another implementation for [`DestAddress`] that looks like `Pdu` or `Tlv` requires using the [`DestFlag`] as a key for decoding the `DestAddressVariant`,
    //! and making [`DestAddress`] a struct with a `dest_flag` field and a `variant` field.
    //! This means we should implement `DecodeWithKey` for `DestAddressVariant` with the [`DestFlag`] as a key.
    //! But `DecodeWithKey` needs a `length` parameter, and our macro supports only `key` and `length` attributes.
    //! So we have to create new trait that uses a key but does not require a length, and we have to update our macro to support only a key without a length.

    use super::*;

    impl crate::tests::TestInstance for DestAddress<'static> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::SmeAddress(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
                Self::DistributionListName(DistributionListName::new(
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<DestAddress>();
    }
}
