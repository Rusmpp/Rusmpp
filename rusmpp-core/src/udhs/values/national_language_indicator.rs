use rusmpp_macros::Rusmpp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[repr(u8)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NationalLanguageIndicator {
    Turkish = 0x01,
    // (Single Shift not available, use default)
    Spanish = 0x02,
    Portuguese = 0x03,
    Bengali = 0x04,
    Gujarati = 0x05,
    Hindi = 0x06,
    Kannada = 0x07,
    Malayalam = 0x08,
    Oriya = 0x09,
    Punjabi = 0x0A,
    Tamil = 0x0B,
    Telugu = 0x0C,
    Urdu = 0x0D,
    Other(u8),
}

impl Default for NationalLanguageIndicator {
    fn default() -> Self {
        NationalLanguageIndicator::Other(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<NationalLanguageIndicator>();
        crate::tests::borrowed::encode_decode_test_instances::<NationalLanguageIndicator>();
    }
}
