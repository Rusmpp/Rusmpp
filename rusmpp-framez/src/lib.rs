//! A [`framez`](https://docs.rs/framez/latest/framez/) codec for [Rusmpp](https://crates.io/crates/rusmpp).
//!
//! ## Features
//!
//! - `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
//!
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

mod codec;

pub(crate) mod logging;

pub mod error;

pub use codec::CommandCodec;

#[cfg(test)]
mod tests;
