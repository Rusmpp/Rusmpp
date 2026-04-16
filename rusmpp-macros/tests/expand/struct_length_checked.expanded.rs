/// Docs
///
/// More docs
pub struct MsValidity {
    /// Docs
    ///
    /// More docs
    pub validity_behavior: MsValidityBehavior,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "checked")]
    pub validity_information: Option<MsValidityInformation>,
}
#[automatically_derived]
impl ::core::fmt::Debug for MsValidity {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MsValidity",
            "validity_behavior",
            &self.validity_behavior,
            "validity_information",
            &&self.validity_information,
        )
    }
}
pub struct MsValidityParts {
    pub validity_behavior: MsValidityBehavior,
    pub validity_information: Option<MsValidityInformation>,
}
#[automatically_derived]
impl ::core::fmt::Debug for MsValidityParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MsValidityParts",
            "validity_behavior",
            &self.validity_behavior,
            "validity_information",
            &&self.validity_information,
        )
    }
}
impl MsValidityParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        validity_behavior: MsValidityBehavior,
        validity_information: Option<MsValidityInformation>,
    ) -> Self {
        Self {
            validity_behavior,
            validity_information,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (MsValidityBehavior, Option<MsValidityInformation>) {
        (self.validity_behavior, self.validity_information)
    }
}
impl MsValidity {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> MsValidityParts {
        MsValidityParts {
            validity_behavior: self.validity_behavior,
            validity_information: self.validity_information,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: MsValidityParts) -> Self {
        Self {
            validity_behavior: parts.validity_behavior,
            validity_information: parts.validity_information,
        }
    }
}
impl crate::encode::Length for MsValidity {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.validity_behavior);
        length += crate::encode::Length::length(&self.validity_information);
        length
    }
}
impl crate::encode::Encode for MsValidity {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(
            &self.validity_behavior,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(
            &self.validity_information,
            dst,
            size,
        );
        size
    }
}
#[non_exhaustive]
pub struct MsValidityDecodeErrorContext {
    pub validity_behavior: ::core::option::Option<
        ::core::result::Result<
            MsValidityBehavior,
            <MsValidityBehavior as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub validity_information: ::core::option::Option<
        ::core::result::Result<
            Option<MsValidityInformation>,
            <Option<
                MsValidityInformation,
            > as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for MsValidityDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MsValidityDecodeErrorContext",
            "validity_behavior",
            &self.validity_behavior,
            "validity_information",
            &&self.validity_information,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MsValidityDecodeErrorContext {
    #[inline]
    fn clone(&self) -> MsValidityDecodeErrorContext {
        MsValidityDecodeErrorContext {
            validity_behavior: ::core::clone::Clone::clone(&self.validity_behavior),
            validity_information: ::core::clone::Clone::clone(&self.validity_information),
        }
    }
}
#[non_exhaustive]
pub struct MsValidityDecodeError {
    pub context: MsValidityDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for MsValidityDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "MsValidityDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MsValidityDecodeError {
    #[inline]
    fn clone(&self) -> MsValidityDecodeError {
        MsValidityDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for MsValidityDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "MsValidity"))?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .validity_behavior
        {
            f.write_fmt(format_args!("{0}: {1}", "validity_behavior", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .validity_information
        {
            f.write_fmt(format_args!("{0}: {1}", "validity_information", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for MsValidityDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .validity_behavior
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .validity_information
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
impl crate::decode::owned::DecodeErrorType for MsValidity {
    type Error = MsValidityDecodeError;
}
impl<'a> crate::decode::borrowed::DecodeWithLength<'a> for MsValidity {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (validity_behavior, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (validity_information, size) = crate::decode::borrowed::DecodeExt::length_checked_decode_move(
                src,
                length.saturating_sub(size),
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                validity_behavior,
                validity_information,
            },
            size,
        ))
    }
}
impl crate::decode::owned::DecodeWithLength for MsValidity {
    fn decode(
        src: &mut ::bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (validity_behavior, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = MsValidityDecodeErrorContext {
                    validity_behavior: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    validity_information: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let opt = match crate::decode::owned::DecodeExt::length_checked_decode_move(
            src,
            length.saturating_sub(size),
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = MsValidityDecodeErrorContext {
                    validity_behavior: ::core::option::Option::Some(
                        ::core::result::Result::Ok(validity_behavior),
                    ),
                    validity_information: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                };
                return Err(Self::Error { context });
            }
        };
        let (validity_information, size) = opt
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                validity_behavior,
                validity_information,
            },
            size,
        ))
    }
}
