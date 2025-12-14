/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct Tlv {
    /// Docs
    ///
    /// More docs
    tag: TlvTag,
    value_length: u16,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tlv {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Tlv",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
pub struct TlvParts {
    pub tag: TlvTag,
    pub value_length: u16,
    pub value: Option<TlvValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for TlvParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "TlvParts",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
impl TlvParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(tag: TlvTag, value_length: u16, value: Option<TlvValue>) -> Self {
        Self { tag, value_length, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (TlvTag, u16, Option<TlvValue>) {
        (self.tag, self.value_length, self.value)
    }
}
impl Tlv {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> TlvParts {
        TlvParts {
            tag: self.tag,
            value_length: self.value_length,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: TlvParts) -> Self {
        Self {
            tag: parts.tag,
            value_length: parts.value_length,
            value: parts.value,
        }
    }
}
impl crate::encode::Length for Tlv {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.tag);
        length += crate::encode::Length::length(&self.value_length);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl crate::encode::Encode for Tlv {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.tag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value_length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct Tlv<'a> {
    /// Docs
    ///
    /// More docs
    tag: TlvTag,
    value_length: u16,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue<'a>>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for Tlv<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Tlv",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
pub struct TlvParts<'a> {
    pub tag: TlvTag,
    pub value_length: u16,
    pub value: Option<TlvValue<'a>>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for TlvParts<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "TlvParts",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
impl<'a> TlvParts<'a> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        tag: TlvTag,
        value_length: u16,
        value: Option<TlvValue<'a>>,
    ) -> Self {
        Self { tag, value_length, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (TlvTag, u16, Option<TlvValue<'a>>) {
        (self.tag, self.value_length, self.value)
    }
}
impl<'a> Tlv<'a> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> TlvParts<'a> {
        TlvParts {
            tag: self.tag,
            value_length: self.value_length,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: TlvParts<'a>) -> Self {
        Self {
            tag: parts.tag,
            value_length: parts.value_length,
            value: parts.value,
        }
    }
}
impl<'a> crate::encode::Length for Tlv<'a> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.tag);
        length += crate::encode::Length::length(&self.value_length);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl<'a> crate::encode::Encode for Tlv<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.tag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value_length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for Tlv<'a> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (tag, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::tag,
        )?;
        let (value_length, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::value_length,
        )?;
        let (value, size) = crate::decode::DecodeErrorExt::map_as_source(
                crate::decode::borrowed::DecodeWithKeyExt::optional_length_checked_decode_move(
                    tag,
                    src,
                    value_length as usize,
                    size,
                ),
                crate::fields::SmppField::value,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { tag, value_length, value }, size))
    }
}
