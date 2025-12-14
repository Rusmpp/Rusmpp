/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct DistributionListName {
    /// Docs
    ///
    /// More docs
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    pub dl_name: COctetString<1, 21>,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistributionListName {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DistributionListName",
            "dest_flag",
            &self.dest_flag,
            "dl_name",
            &&self.dl_name,
        )
    }
}
pub struct DistributionListNameParts {
    pub dest_flag: DestFlag,
    pub dl_name: COctetString<1, 21>,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistributionListNameParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DistributionListNameParts",
            "dest_flag",
            &self.dest_flag,
            "dl_name",
            &&self.dl_name,
        )
    }
}
impl DistributionListNameParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(dest_flag: DestFlag, dl_name: COctetString<1, 21>) -> Self {
        Self { dest_flag, dl_name }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (DestFlag, COctetString<1, 21>) {
        (self.dest_flag, self.dl_name)
    }
}
impl DistributionListName {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> DistributionListNameParts {
        DistributionListNameParts {
            dest_flag: self.dest_flag,
            dl_name: self.dl_name,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: DistributionListNameParts) -> Self {
        Self {
            dest_flag: parts.dest_flag,
            dl_name: parts.dl_name,
        }
    }
}
impl crate::encode::Length for DistributionListName {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.dest_flag);
        length += crate::encode::Length::length(&self.dl_name);
        length
    }
}
impl crate::encode::Encode for DistributionListName {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.dest_flag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.dl_name, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct DistributionListName<'a> {
    /// Docs
    ///
    /// More docs
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    pub dl_name: COctetString<'a, 1, 21>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for DistributionListName<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DistributionListName",
            "dest_flag",
            &self.dest_flag,
            "dl_name",
            &&self.dl_name,
        )
    }
}
pub struct DistributionListNameParts<'a> {
    pub dest_flag: DestFlag,
    pub dl_name: COctetString<'a, 1, 21>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for DistributionListNameParts<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DistributionListNameParts",
            "dest_flag",
            &self.dest_flag,
            "dl_name",
            &&self.dl_name,
        )
    }
}
impl<'a> DistributionListNameParts<'a> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(dest_flag: DestFlag, dl_name: COctetString<'a, 1, 21>) -> Self {
        Self { dest_flag, dl_name }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (DestFlag, COctetString<'a, 1, 21>) {
        (self.dest_flag, self.dl_name)
    }
}
impl<'a> DistributionListName<'a> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> DistributionListNameParts<'a> {
        DistributionListNameParts {
            dest_flag: self.dest_flag,
            dl_name: self.dl_name,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: DistributionListNameParts<'a>) -> Self {
        Self {
            dest_flag: parts.dest_flag,
            dl_name: parts.dl_name,
        }
    }
}
impl<'a> crate::encode::Length for DistributionListName<'a> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.dest_flag);
        length += crate::encode::Length::length(&self.dl_name);
        length
    }
}
impl<'a> crate::encode::Encode for DistributionListName<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.dest_flag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.dl_name, dst, size);
        size
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for DistributionListName<'a> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (dl_name, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::dl_name,
        )?;
        Ok((Self::new(dl_name), size))
    }
}
