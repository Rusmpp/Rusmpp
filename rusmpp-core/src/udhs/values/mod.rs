//! `UDH` values.

pub mod parts {
    pub use super::{
        concatenated_short_message_8_bit::ConcatenatedShortMessage8BitParts,
        concatenated_short_message_16_bit::ConcatenatedShortMessage16BitParts,
    };
}

mod concatenated_short_message_16_bit;
pub use concatenated_short_message_16_bit::ConcatenatedShortMessage16Bit;

mod concatenated_short_message_8_bit;
pub use concatenated_short_message_8_bit::ConcatenatedShortMessage8Bit;

mod national_language_indicator;
pub use national_language_indicator::NationalLanguageIndicator;

pub mod borrowed {
    //! Borrowed `UDH` values.
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned {
    //! Owned `UDH` values.

    pub mod errors {
        pub use super::super::concatenated_short_message_8_bit::{
            ConcatenatedShortMessage8BitDecodeError, ConcatenatedShortMessage8BitDecodeErrorContext,
        };
        pub use super::super::concatenated_short_message_16_bit::{
            ConcatenatedShortMessage16BitDecodeError,
            ConcatenatedShortMessage16BitDecodeErrorContext,
        };
    }
}
