//! Gsm 7-bit encoding/decoding support.

mod alphabet;
mod errors;
mod packed;
mod unpacked;

pub use alphabet::{
    ESCAPE_CHARACTER, Encoded, Gsm7BitAlphabet, Gsm7BitDefaultAlphabet, Gsm7BitSpanishAlphabet,
};
pub use errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError};
pub use packed::Gsm7BitPackedEncoder;
pub use unpacked::Gsm7BitUnpackedEncoder;

#[cfg(test)]
mod tests;
