//! # Rusmpp
//!
//! Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! This is a low level library for implementing clients and servers. If you are looking for a client, check out [rusmppc](https://crates.io/crates/rusmppc).
//!
//! ```rust
//! # #[cfg(feature = "tokio-codec")]
//! # {
//! use core::error::Error;
//! use futures::{SinkExt, StreamExt};
//! use rusmpp::{
//!     tokio_codec::{EncodeError, CommandCodec},
//!     Command, CommandId, CommandStatus, Pdu,
//! };
//! use tokio::io::DuplexStream;
//! use tokio_util::codec::Framed;
//! use tracing::info;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Rusmpp produces a lot of logs while decoding and encoding PDUs.
//!     // You can filter them out by setting the `rusmpp` target to `off`,
//!     // or by disabling the `tracing` feature.
//!     tracing_subscriber::fmt()
//!         .with_env_filter("client=info,server=info,rusmpp=trace")
//!         .init();
//!
//!     // In-memory duplex stream to simulate a server and client.
//!     let (server_stream, client_stream) = tokio::io::duplex(4096);
//!
//!     launch_server(server_stream).await?;
//!
//!     // The CommandCodec encodes/decodes SMPP commands into/from bytes.
//!     let mut framed = Framed::new(client_stream, CommandCodec::new());
//!
//!     // Rusmpp takes care of setting the correct command ID.
//!     let command = Command::new(CommandStatus::EsmeRok, 1, Pdu::EnquireLink);
//!
//!     info!(target: "client", "EnquireLink sent");
//!
//!     framed.send(command).await?;
//!
//!     while let Some(Ok(command)) = framed.next().await {
//!         if let CommandId::EnquireLinkResp = command.id() {
//!             info!(target: "client", "EnquireLink response received");
//!     
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! async fn launch_server(stream: DuplexStream) -> Result<(), Box<dyn Error>> {
//!     tokio::spawn(async move {
//!         let mut framed = Framed::new(stream, CommandCodec::new());
//!
//!         while let Some(Ok(command)) = framed.next().await {
//!             if let CommandId::EnquireLink = command.id() {
//!                 info!(target: "server", "EnquireLink received");
//!
//!                 // We can also use the Command::builder() to create commands.
//!                 let response = Command::builder()
//!                     .status(CommandStatus::EsmeRok)
//!                     .sequence_number(command.sequence_number())
//!                     .pdu(Pdu::EnquireLinkResp);
//!
//!                 framed.send(response).await?;
//!
//!                 info!(target: "server", "EnquireLink response sent");
//!
//!                 break;
//!             }
//!         }
//!
//!         Ok::<(), EncodeError>(())
//!     });
//!
//!     Ok(())
//! }
//! # }
//!```
//!
//! ## Features
//!
//! - `tokio-codec`: Implements [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html) traits.
//! - `verbose`: Enables verbose error reports.
//! - `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for all SMPP types.
//! - `serde-deserialize-unchecked`: Implements [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait for all SMPP types, but does not check the validity of the data. Use with caution.
//! - `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
//! - `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
//! - `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.
//!

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
// #![deny(missing_docs)]

#[cfg(feature = "tokio-codec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-codec")))]
pub mod tokio_codec;

pub mod types;

pub mod decode;
pub mod encode;

pub use rusmpp_core::{CommandId, CommandStatus, command::owned::Command, pdus::owned::Pdu};

pub mod command;

pub mod values;

pub mod tlvs;

pub mod pdus;

pub mod fields;

pub mod session;

pub mod udhs;
