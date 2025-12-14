/// Docs
///
/// More docs
#[rusmpp(decode = skip)]
pub struct DistributionListName {
    /// Docs
    ///
    /// More docs
    dest_flag: DestFlag,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistributionListName {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "DistributionListName",
            "dest_flag",
            &&self.dest_flag,
        )
    }
}
pub struct DistributionListNameParts {
    pub dest_flag: DestFlag,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistributionListNameParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "DistributionListNameParts",
            "dest_flag",
            &&self.dest_flag,
        )
    }
}
impl DistributionListNameParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(dest_flag: DestFlag) -> Self {
        Self { dest_flag }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (DestFlag) {
        (self.dest_flag)
    }
}
impl DistributionListName {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> DistributionListNameParts {
        DistributionListNameParts {
            dest_flag: self.dest_flag,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: DistributionListNameParts) -> Self {
        Self { dest_flag: parts.dest_flag }
    }
}
impl crate::encode::Length for DistributionListName {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.dest_flag);
        length
    }
}
impl crate::encode::Encode for DistributionListName {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.dest_flag, dst, size);
        size
    }
}
