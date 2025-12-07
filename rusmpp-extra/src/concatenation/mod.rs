//! Concatenation support.

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

pub mod errors;

/// The minimum number of parts in a concatenated message.
pub const MIN_PARTS: usize = 2;

/// The maximum number of parts in a concatenated message.
pub const MAX_PARTS: usize = 255;
