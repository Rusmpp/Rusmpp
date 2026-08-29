#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//! Extra components for [`Rusmpp-Core`](https://crates.io/crates/rusmpp-core).
//!
//! ## Features
//!
//! - `alloc`:  Enables the `alloc` crate.
//! - `concatenation`: Enables concatenation support.
//! - `encoding`: Enables encoding/decoding support.

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(feature = "concatenation")]
#[cfg_attr(docsrs, doc(cfg(feature = "concatenation")))]
pub mod concatenation;

#[cfg(feature = "encoding")]
#[cfg_attr(docsrs, doc(cfg(feature = "encoding")))]
pub mod encoding;

pub mod fallback;

mod sealed;
use sealed::Sealed;

mod traits;
