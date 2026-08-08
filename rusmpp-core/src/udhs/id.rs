use rusmpp_macros::Rusmpp;

/// User Data Header (UDH) Identifier.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[repr(u8)]
#[rusmpp(test = skip)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum UdhId {
    /// Concatenated short messages, 8-bit reference number.
    ConcatenatedShortMessages8Bit = 0x00,
    /// Concatenated short messages, 16-bit reference number.
    ConcatenatedShortMessages16Bit = 0x08,
    /// Application port addressing scheme, 8-bit address.
    ApplicationPortAddressing8Bit = 0x04,
    /// Application port addressing scheme, 16-bit address.
    ApplicationPortAddressing16Bit = 0x05,
    /// National language single shift.
    NationalLanguageSingleShift = 0x24,
    /// National language locking shift.
    NationalLanguageLockingShift = 0x25,
    /// Other UDH identifier.
    Other(u8),
}
