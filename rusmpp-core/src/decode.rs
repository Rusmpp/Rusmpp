//! Traits for decoding `SMPP` values.

pub mod borrowed;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

mod error;
pub use error::*;
