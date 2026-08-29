//! Encoding and decoding support.

pub mod gsm7bit;
pub mod latin1;
pub mod ucs2;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

pub mod errors;
