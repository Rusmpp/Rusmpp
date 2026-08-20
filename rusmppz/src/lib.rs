#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//! A `zerocopy` Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! ## Features
//!
//! - `framez`: Implements [`framez`](https://docs.rs/framez/latest/framez/index.html) [`Encoder`](https://docs.rs/framez/latest/framez/encode/trait.Encoder.html) and [`Decoder`](https://docs.rs/framez/latest/framez/decode/trait.Decoder.html) traits.
//! - `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for all SMPP types.
//! - `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub mod framez;

pub mod types;

pub mod decode;
pub mod encode;

pub use rusmpp_core::{CommandId, CommandStatus, command::borrowed::Command, pdus::borrowed::Pdu};

pub mod command;

pub mod values;

pub mod tlvs;

pub mod pdus;
