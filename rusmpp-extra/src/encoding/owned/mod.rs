//! Owned encoding and decoding support.

mod encoder;
pub use encoder::Encoder;

mod decoder;
pub use decoder::{Decoder, SupportedDecodeError, SupportedDecoder};

mod traits;
pub use traits::{Encoded, EncodedBuilder};

mod fallback;
