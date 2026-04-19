/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
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
    #[rusmpp(key = id, length = length - 1)]
    value: Option<UdhValue>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
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
    #[rusmpp(key = id, length = length - 1)]
    value: Option<UdhValue<'a, N>>,
}
