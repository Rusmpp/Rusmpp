//! Owned encoding and decoding support.

mod encoder;
pub use encoder::Encoder;

mod decoder;
pub use decoder::{Decoder, SupportedDecodeError, SupportedDecoder};

mod submit_sm;
pub use submit_sm::{EncodedSubmitSmBuilder, EncodedSubmitSmExt};

mod fallback;
