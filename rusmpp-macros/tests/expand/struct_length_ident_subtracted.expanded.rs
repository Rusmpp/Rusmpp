/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct Udh {
    /// Docs
    ///
    /// More docs
    pub length: u8,
    id: UdhId,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = length-1)]
    value: Option<UdhValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Udh {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Udh",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
pub struct UdhParts {
    pub length: u8,
    pub id: UdhId,
    pub value: Option<UdhValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for UdhParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdhParts",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
impl UdhParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(length: u8, id: UdhId, value: Option<UdhValue>) -> Self {
        Self { length, id, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, UdhId, Option<UdhValue>) {
        (self.length, self.id, self.value)
    }
}
impl Udh {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> UdhParts {
        UdhParts {
            length: self.length,
            id: self.id,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: UdhParts) -> Self {
        Self {
            length: parts.length,
            id: parts.id,
            value: parts.value,
        }
    }
}
impl crate::encode::Length for Udh {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.length);
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl crate::encode::Encode for Udh {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct Udh<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub length: u8,
    id: UdhId,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = length-1)]
    value: Option<UdhValue<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for Udh<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Udh",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
pub struct UdhParts<'a, const N: usize> {
    pub length: u8,
    pub id: UdhId,
    pub value: Option<UdhValue<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for UdhParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdhParts",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
impl<'a, const N: usize> UdhParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(length: u8, id: UdhId, value: Option<UdhValue<'a, N>>) -> Self {
        Self { length, id, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, UdhId, Option<UdhValue<'a, N>>) {
        (self.length, self.id, self.value)
    }
}
impl<'a, const N: usize> Udh<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> UdhParts<'a, N> {
        UdhParts {
            length: self.length,
            id: self.id,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: UdhParts<'a, N>) -> Self {
        Self {
            length: parts.length,
            id: parts.id,
            value: parts.value,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for Udh<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.length);
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for Udh<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::Decode<'a> for Udh<'a, N> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (length, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (id, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let _length = (length as usize).saturating_sub(1usize);
        let (value, size) = crate::decode::borrowed::DecodeWithKeyExt::optional_length_checked_decode_move(
                id,
                src,
                _length,
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { length, id, value }, size))
    }
}
