//! # Rusmppc
//!
//! A Rust [SMPP v5](https://smpp.org/SMPP_v5.pdf) client.
//!
//! ## Features
//!
//! - `tokio`: Enables support for [`tokio`](https://docs.rs/tokio/latest/tokio/) runtime. Enabled by default.
//! - `wasm`: Enables support for `wasm`.
//! - `rustls`:  Enables TLS support via [`rustls`](https://docs.rs/rustls/latest/rustls/). Enabled by default.
//! - `rustls-tls-native-roots`: Uses the platform's native root certificates through [`rustls-native-certs`](https://docs.rs/rustls-native-certs/latest/rustls_native_certs/) while using default configuration. Enables the `rustls` feature and is enabled by default.
//! - `rustls-tls-webpki-roots`: Uses the [`webpki-roots`](https://docs.rs/webpki-roots/latest/webpki_roots/) crate's root certificates while using default configuration. Enables the `rustls` feature and is enabled by default.
//! - `rustls-aws-lc-rs`: Uses [`aws-lc-rs`](https://docs.rs/aws-lc-rs/latest/aws_lc_rs/) as the TLS backend for [`rustls`](https://docs.rs/rustls/latest/rustls/). Enabled by default.
//! - `rustls-ring`: Uses [`ring`](https://docs.rs/ring/latest/ring/) as the TLS backend for [`rustls`](https://docs.rs/rustls/latest/rustls/).
//! - `native-tls`: Enables TLS support via [`native-tls`](https://docs.rs/native-tls/latest/native_tls/).
//!
//! # Example
//!
//!```rust, no_run
//! use std::{str::FromStr, time::Duration};
//!
//! use futures::StreamExt;
//! use rusmpp::{
//!     CommandId,
//!     pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
//!     types::{COctetString, OctetString},
//!     values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
//! };
//! use rusmppc::{ConnectionBuilder, event::Event};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (client, mut events) = ConnectionBuilder::new()
//!         // Set the maximum command length for incoming commands.
//!         .max_command_length(1024)
//!         // Every 5 seconds send an enquire link command to the server.
//!         .enquire_link_interval(Duration::from_secs(5))
//!         // If the server did not respond within 3 seconds, close the connection.
//!         .enquire_link_response_timeout(Duration::from_secs(3))
//!         // If the server does not respond within 2 seconds, consider it a timeout.
//!         // The operation is assumed to have failed.
//!         .response_timeout(Duration::from_secs(2))
//!         .connect("smpp://127.0.0.1:2775")
//!         .await?;
//!     
//!     // Bind the client as a transceiver.
//!     client
//!         .bind_transceiver(
//!             BindTransceiver::builder()
//!                 .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
//!                 .password(COctetString::from_str("rEZYMq5j")?)
//!                 .system_type(COctetString::empty())
//!                 .addr_ton(Ton::Unknown)
//!                 .addr_npi(Npi::Unknown)
//!                 .address_range(COctetString::empty())
//!                 .build(),
//!         )
//!         .await?;
//!     
//!     let events = tokio::spawn(async move {
//!         // Listen for events like incoming commands and background errors.
//!         while let Some(event) = events.next().await {
//!             println!("Event: {:?}", event);
//!         }
//!     });
//!     
//!     // Send commands to the server and wait for the responses.
//!     let response = client
//!         .submit_sm(
//!             SubmitSm::builder()
//!                 .service_type(ServiceType::default())
//!                 .source_addr_ton(Ton::Unknown)
//!                 .source_addr_npi(Npi::Unknown)
//!                 .source_addr(COctetString::from_str("12345")?)
//!                 .destination_addr(COctetString::from_str("491701234567")?)
//!                 .esm_class(EsmClass::default())
//!                 .registered_delivery(RegisteredDelivery::request_all())
//!                 .short_message(OctetString::from_str("Hi, I am a short message.")?)
//!                 .build(),
//!         )
//!         .await?;
//!     
//!     println!("SubmitSm response: {:?}", response);
//!     
//!     // Send an unbind command to the server and wait for the response.
//!     client.unbind().await?;
//!     
//!     // Close the connection.
//!     client.close().await?;
//!     
//!     // Wait for the connection to be closed.
//!     client.closed().await;
//!     
//!     // When the connection is closed, the event stream will also be closed.
//!     events.await?;
//!     
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/// The initial sequence number of [`Client`].
///
/// This value is incremented by [`SEQUENCE_NUMBER_INCREMENT`] wrapping back to [`INITIAL_CLIENT_SEQUENCE_NUMBER`] if it exceeds [`MAX_SEQUENCE_NUMBER`].
///
/// The [`Client`]'s sequence number is therefore always `odd`.
const INITIAL_CLIENT_SEQUENCE_NUMBER: u32 = 1;

/// The initial sequence number of [`Connection`](crate::connection::Connection).
///
/// This value is incremented by [`SEQUENCE_NUMBER_INCREMENT`] wrapping back to [`INITIAL_CLIENT_SEQUENCE_NUMBER`] if it exceeds [`MAX_SEQUENCE_NUMBER`].
///
/// The [`Connection`](crate::connection::Connection)'s sequence number is therefore always `even`.
const INITIAL_CONNECTION_SEQUENCE_NUMBER: u32 = 2;

/// This describes how we increment the sequence numbers in [`Client`] and [`Connection`](crate::connection::Connection).
const SEQUENCE_NUMBER_INCREMENT: u32 = 2;

/// The minimum sequence number must be at least [`SEQUENCE_NUMBER_INCREMENT`],
/// because we subtract [`SEQUENCE_NUMBER_INCREMENT`] from [`INITIAL_CLIENT_SEQUENCE_NUMBER`] or [`INITIAL_CONNECTION_SEQUENCE_NUMBER`] and then compare it to [`MAX_SEQUENCE_NUMBER`].
///
/// The subtraction must not underflow.
const MIN_SEQUENCE_NUMBER: u32 = SEQUENCE_NUMBER_INCREMENT;

/// The maximum sequence number.
///
/// Sequence numbers must not exceed this value.
///
/// See [`4.7.24 sequence_number`](https://smpp.org/SMPP_v5.pdf)
const MAX_SEQUENCE_NUMBER: u32 = 0x7FFFFFFF;

mod action;
pub(crate) use action::Action;

mod connection;

mod builder_;
pub use builder_::{
    ConnectionBuilder, DefaultTokioConnectionBuilder, DefaultWasmConnectionBuilder,
};

mod event_;

pub mod event {
    //! Types related to events emitted by the `SMPP` connection.
    pub use super::event_::{Event, Insight, InsightEvent};
}

pub mod channel {
    //! Types related to event channels.
    pub use super::event_::{DefaultEventChannel, DiscardEventChannel, InsightEventChannel};
}

pub mod builder {
    //! Types related to building an `SMPP` connection or operations.
    pub use super::builder_::{EventsConnectionBuilder, NoSpawnConnectionBuilder};
    pub use super::client::{
        NoWaitRequestBuilder, RawRegisteredRequestBuilder, RegisteredRequestBuilder,
        UnregisteredRequestBuilder,
    };
}

mod request;
pub(crate) use request::{CloseRequest, RegisteredRequest, Request, UnregisteredRequest};

mod timer;
pub(crate) use timer::Timer;

mod client;
pub use client::Client;

pub mod error;

mod command;
pub(crate) use command::CommandExt;

mod futures;
pub(crate) use futures::RequestFutureGuard;

mod response;
pub(crate) use response::PendingResponses;

#[cfg(feature = "tokio")]
mod tcp_stream;
#[cfg(feature = "tokio")]
pub(crate) use tcp_stream::MaybeTlsStream;

mod runtime_;
pub mod runtime {
    //! Supported runtimes.
    pub use super::runtime_::{tokio::Tokio, wasm::Wasm};
}

mod cmp;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[cfg(feature = "tokio")]
mod managed_;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod managed {
    //! A managed `SMPP` client that automatically handles reconnection and binding.
    pub use super::managed_::{ManagedClient, ManagedEvent};

    pub mod builder {
        //! Types related to building a managed `SMPP` client.
        pub use super::super::managed_::{
            ManagedConnectionBuilder, UnboundManagedConnectionBuilder,
        };
    }
}
