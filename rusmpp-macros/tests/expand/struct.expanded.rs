/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct CancelSm {
    /// Docs
    ///
    /// More docs
    pub service_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub other: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for CancelSm {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSm",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
pub struct CancelSmParts {
    pub service_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub other: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for CancelSmParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSmParts",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
impl CancelSmParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        service_type: ServiceType,
        message_id: COctetString<1, 65>,
        other: u8,
    ) -> Self {
        Self {
            service_type,
            message_id,
            other,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (ServiceType, COctetString<1, 65>, u8) {
        (self.service_type, self.message_id, self.other)
    }
}
impl CancelSm {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> CancelSmParts {
        CancelSmParts {
            service_type: self.service_type,
            message_id: self.message_id,
            other: self.other,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: CancelSmParts) -> Self {
        Self {
            service_type: parts.service_type,
            message_id: parts.message_id,
            other: parts.other,
        }
    }
}
impl crate::encode::Length for CancelSm {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.service_type);
        length += crate::encode::Length::length(&self.message_id);
        length += crate::encode::Length::length(&self.other);
        length
    }
}
impl crate::encode::Encode for CancelSm {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.service_type, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.message_id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct CancelSm<'a> {
    /// Docs
    ///
    /// More docs
    pub service_type: ServiceType<'a>,
    pub message_id: COctetString<'a, 1, 65>,
    pub other: u8,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for CancelSm<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSm",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
pub struct CancelSmParts<'a> {
    pub service_type: ServiceType<'a>,
    pub message_id: COctetString<'a, 1, 65>,
    pub other: u8,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for CancelSmParts<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSmParts",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
impl<'a> CancelSmParts<'a> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        service_type: ServiceType<'a>,
        message_id: COctetString<'a, 1, 65>,
        other: u8,
    ) -> Self {
        Self {
            service_type,
            message_id,
            other,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (ServiceType<'a>, COctetString<'a, 1, 65>, u8) {
        (self.service_type, self.message_id, self.other)
    }
}
impl<'a> CancelSm<'a> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> CancelSmParts<'a> {
        CancelSmParts {
            service_type: self.service_type,
            message_id: self.message_id,
            other: self.other,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: CancelSmParts<'a>) -> Self {
        Self {
            service_type: parts.service_type,
            message_id: parts.message_id,
            other: parts.other,
        }
    }
}
impl<'a> crate::encode::Length for CancelSm<'a> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.service_type);
        length += crate::encode::Length::length(&self.message_id);
        length += crate::encode::Length::length(&self.other);
        length
    }
}
impl<'a> crate::encode::Encode for CancelSm<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.service_type, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.message_id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        size
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for CancelSm<'a> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (service_type, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::service_type,
        )?;
        let (message_id, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::message_id,
        )?;
        let (other, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::other,
        )?;
        Ok((
            Self {
                service_type,
                message_id,
                other,
            },
            size,
        ))
    }
}
