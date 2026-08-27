//! Gsm 7-bit encoding/decoding support.

mod alphabet;
mod decode;
mod encode;
mod errors;

pub use alphabet::{
    ESCAPE_CHARACTER, Encoded, Gsm7BitAlphabet, Gsm7BitDefaultAlphabet, Gsm7BitSpanishAlphabet,
};
pub use encode::{packed::Gsm7BitPackedEncoder, unpacked::Gsm7BitUnpackedEncoder};
pub use errors::{Gsm7BitConcatenateError, Gsm7BitDecodeError, Gsm7BitEncodeError};

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use decode::unpacked::owned::Gsm7BitUnpackedDecoder;

#[cfg(test)]
mod tests;
