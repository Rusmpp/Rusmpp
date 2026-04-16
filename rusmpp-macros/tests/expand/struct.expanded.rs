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
#[non_exhaustive]
pub struct CancelSmDecodeErrorContext {
    pub service_type: ::core::option::Option<
        ::core::result::Result<
            ServiceType,
            <ServiceType as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub message_id: ::core::option::Option<
        ::core::result::Result<
            COctetString<1, 65>,
            <COctetString<1, 65> as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub other: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for CancelSmDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSmDecodeErrorContext",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CancelSmDecodeErrorContext {
    #[inline]
    fn clone(&self) -> CancelSmDecodeErrorContext {
        CancelSmDecodeErrorContext {
            service_type: ::core::clone::Clone::clone(&self.service_type),
            message_id: ::core::clone::Clone::clone(&self.message_id),
            other: ::core::clone::Clone::clone(&self.other),
        }
    }
}
#[non_exhaustive]
pub struct CancelSmDecodeError {
    pub context: CancelSmDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for CancelSmDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CancelSmDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CancelSmDecodeError {
    #[inline]
    fn clone(&self) -> CancelSmDecodeError {
        CancelSmDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for CancelSmDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "CancelSm"))?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .service_type
        {
            f.write_fmt(format_args!("{0}: {1}", "service_type", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .message_id
        {
            f.write_fmt(format_args!("{0}: {1}", "message_id", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .other
        {
            f.write_fmt(format_args!("{0}: {1}", "other", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for CancelSmDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .service_type
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .message_id
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .other
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        ::core::option::Option::None
    }
    fn cause(&self) -> Option<&dyn ::core::error::Error> {
        self.source()
    }
}
impl crate::decode::owned::DecodeErrorType for CancelSm {
    type Error = CancelSmDecodeError;
}
impl crate::decode::owned::Decode for CancelSm {
    fn decode(src: &mut ::bytes::BytesMut) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (service_type, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CancelSmDecodeErrorContext {
                    service_type: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    message_id: ::core::option::Option::None,
                    other: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (message_id, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CancelSmDecodeErrorContext {
                    service_type: ::core::option::Option::Some(
                        ::core::result::Result::Ok(service_type),
                    ),
                    message_id: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    other: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (other, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CancelSmDecodeErrorContext {
                    service_type: ::core::option::Option::Some(
                        ::core::result::Result::Ok(service_type),
                    ),
                    message_id: ::core::option::Option::Some(
                        ::core::result::Result::Ok(message_id),
                    ),
                    other: ::core::option::Option::Some(::core::result::Result::Err(err)),
                };
                return Err(Self::Error { context });
            }
        };
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
        let (service_type, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (message_id, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (other, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
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
