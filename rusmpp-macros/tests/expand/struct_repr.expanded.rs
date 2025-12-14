/// Docs
///
/// More docs
#[rusmpp(repr = "u8")]
pub struct CallbackNumPresInd {
    /// Docs
    ///
    /// More docs
    pub presentation: Presentation,
    pub screening: Screening,
}
#[automatically_derived]
impl ::core::fmt::Debug for CallbackNumPresInd {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CallbackNumPresInd",
            "presentation",
            &self.presentation,
            "screening",
            &&self.screening,
        )
    }
}
pub struct CallbackNumPresIndParts {
    pub presentation: Presentation,
    pub screening: Screening,
}
#[automatically_derived]
impl ::core::fmt::Debug for CallbackNumPresIndParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CallbackNumPresIndParts",
            "presentation",
            &self.presentation,
            "screening",
            &&self.screening,
        )
    }
}
impl CallbackNumPresIndParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(presentation: Presentation, screening: Screening) -> Self {
        Self { presentation, screening }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (Presentation, Screening) {
        (self.presentation, self.screening)
    }
}
impl CallbackNumPresInd {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> CallbackNumPresIndParts {
        CallbackNumPresIndParts {
            presentation: self.presentation,
            screening: self.screening,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: CallbackNumPresIndParts) -> Self {
        Self {
            presentation: parts.presentation,
            screening: parts.screening,
        }
    }
}
impl crate::encode::Length for CallbackNumPresInd {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}
impl crate::encode::Encode for CallbackNumPresInd {
    fn encode(&self, dst: &mut [u8]) -> usize {
        u8::from(*self).encode(dst)
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for CallbackNumPresInd {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        u8::decode(src).map(|(this, size)| (Self::from(this), size))
    }
}
