//! Owned encoding and decoding support.

mod encoder;
pub use encoder::Encoder;

mod submit_sm;
pub use submit_sm::{EncodedSubmitSmBuilder, EncodedSubmitSmExt};

mod fallback;
