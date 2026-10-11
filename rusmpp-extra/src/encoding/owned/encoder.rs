use rusmpp_core::{udhs::values::NationalLanguageIndicator, values::DataCoding};

/// A trait for encoding messages into byte vectors.
pub trait Encoder {
    /// The type of errors that can occur during encoding.
    type Error;

    /// Encodes the given message into a vector of bytes and its associated [`DataCoding`].
    fn encode(&self, message: &str) -> Result<(alloc::vec::Vec<u8>, DataCoding), Self::Error>;

    /// Returns the National Language Single Shift.
    ///
    /// Only relevant for GSM 7-bit encoding.
    fn national_language_single_shift(&self) -> Option<NationalLanguageIndicator> {
        None
    }

    /// Returns the National Language Locking Shift.
    ///
    /// Only relevant for GSM 7-bit encoding.
    fn national_language_locking_shift(&self) -> Option<NationalLanguageIndicator> {
        None
    }
}
