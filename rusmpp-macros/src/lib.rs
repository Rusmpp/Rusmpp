// Do not break MSRV 1.85.1
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod container_attributes;
mod derive;
mod enums;
mod parts;
mod repr;
mod structs;

/// Implements `Length`, `Encode`, `Decode` and `TestInstance` with one default value for structs and enums.
/// And creates parts structs for structs with `new` and `raw` methods and adds `into_parts` and `from_parts` methods to the original struct.
///
/// # Enums
///
/// ## Container attributes
///
/// - `#[repr(u8)]`, `#[repr(u16)]`, or `#[repr(u32)]`: Use the `From<u8>`, `From<u16>`, or `From<u32>`/`Into<u8>`, `Into<u16>`, or `Into<u32>` representation for decoding.
/// - `#[rusmpp(decode = skip|owned|borrowed|all)]`: Control which `Decode` implementations to generate. Default is `all`.
/// - `#[rusmpp(test = skip)]`: Skip impl `TestInstance` for the enum.
/// - `#[rusmpp(from_into = skip)]`: Skip implementing `From<repr>` and `From<Enum>` for the enum.
///
/// # Structs
///
/// ## Container attributes
///
/// - `#[rusmpp(repr = "u8")]`: Use the `From<u8>`/`Into<u8>` representation for decoding.
/// - `#[rusmpp(decode = skip|owned|borrowed|all)]`: Control which `Decode` implementations to generate. Default is `all`.
/// - `#[rusmpp(test = skip)]`: Skip impl `TestInstance` for the struct.
///
/// ## Field attributes
///
/// - `#[rusmpp(skip_decode)]`: Skip decoding the field (requires a corresponding `new` constructor that does not take the skipped field as an argument).
/// - `#[rusmpp(length = "unchecked")]`: Decode without length checks.
/// - `#[rusmpp(length = "checked")]`: Decode using `length_checked_decode`.
/// - `#[rusmpp(length = ident)]`: Use the value of another field (`ident`) as the length for decoding.
/// - `#[rusmpp(key = ident, length = "unchecked")]`: Decode using a key and unchecked length.
/// - `#[rusmpp(key = ident, length = ident)]`: Decode using a key and the value of another field (`ident`) as the length.
/// - `#[rusmpp(count = ident)]`: Decode a vector of values, where `ident` is the number of elements to decode.
///
/// # Examples
///
/// See `tests/expand`.
#[proc_macro_derive(Rusmpp, attributes(rusmpp))]
pub fn rusmpp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::rusmpp(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Creates a `TlvValue`-like and implements `Into<TlvValue>` and `Into<Tlv>`.
#[proc_macro_derive(TlvValue)]
pub fn tlv_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::tlv_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
