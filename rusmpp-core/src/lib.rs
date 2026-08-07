#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_const_for_fn)]
//! ## Features
//!
//! - `alloc`:  Enables the `alloc` crate.
//! - `arbitrary`: Implements [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait for all SMPP types.
//! - `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for all SMPP types.

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(any(test, feature = "arbitrary", feature = "test"))]
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

#[cfg(any(test, feature = "test"))]
pub mod tests;

pub mod udhs;

mod sealed;
use sealed::Sealed;
