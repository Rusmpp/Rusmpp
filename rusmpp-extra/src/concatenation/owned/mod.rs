//! Owned concatenation support.

mod concatenation;
pub use concatenation::Concatenation;

mod concatenator;
pub use concatenator::Concatenator;

mod multipart;
pub use multipart::{SubmitSmMultipartBuilder, SubmitSmMultipartExt};

mod fallback;
