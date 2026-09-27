#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//! Extra components for [`rusmpp-core`](https://crates.io/crates/rusmpp-core).
//!
//! ## Features
//!
//! - `alloc`:  Enables the `alloc` crate.
//! - `concatenation`: Enables concatenation support.
//! - `encoding`: Enables encoding/decoding support.

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

pub mod concatenation;

pub mod encoding;

pub mod fallback;

mod sealed;
use sealed::Sealed;

pub mod sm;
