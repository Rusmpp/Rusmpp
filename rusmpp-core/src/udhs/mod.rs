//! User Data Headers (UDHs).

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

mod id;
pub use id::UdhId;

pub mod errors;

pub mod concatenation;
