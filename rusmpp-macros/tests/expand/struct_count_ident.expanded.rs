/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct SubmitMulti {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMulti {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMulti",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
pub struct SubmitMultiParts {
    pub other: u8,
    pub number_of_dests: u8,
    pub dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMultiParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiParts",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
impl SubmitMultiParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        number_of_dests: u8,
        dest_address: ::alloc::vec::Vec<DestAddress>,
    ) -> Self {
        Self {
            other,
            number_of_dests,
            dest_address,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, ::alloc::vec::Vec<DestAddress>) {
        (self.other, self.number_of_dests, self.dest_address)
    }
}
impl SubmitMulti {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitMultiParts {
        SubmitMultiParts {
            other: self.other,
            number_of_dests: self.number_of_dests,
            dest_address: self.dest_address,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitMultiParts) -> Self {
        Self {
            other: parts.other,
            number_of_dests: parts.number_of_dests,
            dest_address: parts.dest_address,
        }
    }
}
impl crate::encode::Length for SubmitMulti {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.number_of_dests);
        length += crate::encode::Length::length(&self.dest_address);
        length
    }
}
impl crate::encode::Encode for SubmitMulti {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.number_of_dests,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.dest_address, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct SubmitMulti<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitMulti<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMulti",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
pub struct SubmitMultiParts<'a, const N: usize> {
    pub other: u8,
    pub number_of_dests: u8,
    pub dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitMultiParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiParts",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
impl<'a, const N: usize> SubmitMultiParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        number_of_dests: u8,
        dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
    ) -> Self {
        Self {
            other,
            number_of_dests,
            dest_address,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, ::heapless::vec::Vec<DestAddress<'a>, N>) {
        (self.other, self.number_of_dests, self.dest_address)
    }
}
impl<'a, const N: usize> SubmitMulti<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitMultiParts<'a, N> {
        SubmitMultiParts {
            other: self.other,
            number_of_dests: self.number_of_dests,
            dest_address: self.dest_address,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitMultiParts<'a, N>) -> Self {
        Self {
            other: parts.other,
            number_of_dests: parts.number_of_dests,
            dest_address: parts.dest_address,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for SubmitMulti<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.number_of_dests);
        length += crate::encode::Length::length(&self.dest_address);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for SubmitMulti<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.number_of_dests,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.dest_address, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::DecodeWithLength<'a>
for SubmitMulti<'a, N> {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (other, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::other,
        )?;
        let (number_of_dests, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::number_of_dests,
        )?;
        let (dest_address, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::counted_move(
                src,
                number_of_dests as usize,
                size,
            ),
            crate::fields::SmppField::dest_address,
        )?;
        Ok((
            Self {
                other,
                number_of_dests,
                dest_address,
            },
            size,
        ))
    }
}
