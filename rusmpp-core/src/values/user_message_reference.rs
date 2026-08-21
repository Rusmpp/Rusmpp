use rusmpp_macros::Rusmpp;

/// A reference assigned by the originating SME to the short message. Depending on the
/// destination network technology, this field may be passed directly to the mobile device.
///
/// The user_message_reference TLV is also applicable in ancillary broadcast operations as a
/// means of identifying a previously submitted message. In such cases, the
/// user_message_reference can be used to substitute an actual message_id or may be used in
/// conjunction with a message_id.
#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct UserMessageReference {
    pub value: u16,
}

impl ::core::fmt::Debug for UserMessageReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl UserMessageReference {
    pub const fn new(value: u16) -> Self {
        Self { value }
    }
}

impl From<u16> for UserMessageReference {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<UserMessageReference> for u16 {
    fn from(value: UserMessageReference) -> Self {
        value.value
    }
}
