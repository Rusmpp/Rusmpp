//! Gsm 7-bit encoding/decoding support.

mod alphabet;
mod encode;
mod errors;

pub use alphabet::{
    ESCAPE_CHARACTER, Encoded, Gsm7BitAlphabet, Gsm7BitDefaultAlphabet, Gsm7BitSpanishAlphabet,
};
pub use encode::{packed::Gsm7BitPackedEncoder, unpacked::Gsm7BitUnpackedEncoder};
pub use errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError};

#[cfg(test)]
mod tests;
