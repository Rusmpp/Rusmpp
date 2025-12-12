/// Docs
///
/// More docs
#[repr(u8)]
pub enum DestFlag {
    /// Docs
    ///
    /// More docs
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    /// Docs
    ///
    /// More docs
    Other(u8),
}
#[automatically_derived]
impl ::core::fmt::Debug for DestFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DestFlag::SmeAddress => ::core::fmt::Formatter::write_str(f, "SmeAddress"),
            DestFlag::DistributionListName => {
                ::core::fmt::Formatter::write_str(f, "DistributionListName")
            }
            DestFlag::Other(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Other", &__self_0)
            }
        }
    }
}
impl crate::encode::Length for DestFlag {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}
impl crate::encode::Encode for DestFlag {
    fn encode(&self, dst: &mut [u8]) -> usize {
        u8::from(*self).encode(dst)
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for DestFlag {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        u8::decode(src).map(|(this, size)| (Self::from(this), size))
    }
}
impl From<u8> for DestFlag {
    fn from(value: u8) -> Self {
        match value {
            0x01 => DestFlag::SmeAddress,
            0x02 => DestFlag::DistributionListName,
            other => DestFlag::Other(other),
        }
    }
}
impl From<DestFlag> for u8 {
    fn from(value: DestFlag) -> Self {
        match value {
            DestFlag::SmeAddress => 0x01,
            DestFlag::DistributionListName => 0x02,
            DestFlag::Other(other) => other,
        }
    }
}
