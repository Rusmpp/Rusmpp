#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//! ## Features
//!
//! - `alloc`:  Enables the `alloc` crate.
//! - `arbitrary`: Implements [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait for all SMPP types.
//! - `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for all SMPP types.
//! - `serde-deserialize-unchecked`: Implements [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait for owned SMPP types, but does not check the validity of the data. Use with caution.
//! - `tokio-codec`: Implements [`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/index.html) [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html) traits.
//! - `framez`: Implements [`framez`](https://docs.rs/framez/latest/framez/index.html) [`Encoder`](https://docs.rs/framez/latest/framez/encode/trait.Encoder.html) and [`Decoder`](https://docs.rs/framez/latest/framez/decode/trait.Decoder.html) traits.
//! - `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
//! - `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
//! - `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(any(test, feature = "arbitrary", feature = "tokio-codec"))]
extern crate std;

pub mod pdus;

pub mod values;

mod command_id;
pub use command_id::CommandId;

mod command_status;
pub use command_status::CommandStatus;

pub mod command;

pub mod session;

pub mod decode;

pub mod encode;

pub mod types;

pub mod tlvs;

#[cfg(test)]
pub(crate) mod tests;

pub(crate) mod formatter;

#[cfg(any(feature = "framez", feature = "tokio-codec"))]
pub(crate) mod logging;

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub mod framez;

#[cfg(feature = "tokio-codec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-codec")))]
pub mod tokio_codec;

pub mod udhs;
