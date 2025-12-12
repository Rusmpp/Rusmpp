//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_encode --features="alloc,concatenation"
//! ```

use std::str::FromStr;

use rusmpp_core::{
    pdus::owned::SubmitSm,
    types::owned::{COctetString, OctetString},
    values::{DataCoding, Npi, Ton},
};
use rusmpp_extra::codecs::{owned::EncodedSubmitSmExt, ucs2::Ucs2};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // c-spell: disable
    let message = r##"Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
    // c-spell: enable

    let sm = SubmitSm::builder()
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        // data_coding will be overridden by the encoding builder to match the encoder.
        .data_coding(DataCoding::default())
        // short_message will be overridden by `short_message` of the encoding builder.
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build()
        .encode()
        .short_message(message)
        .gsm7bit_unpacked()
        .fallback(Ucs2::new())
        .build()?;

    println!(
        "Encoded: short_message_len = {}, data_coding = {:?}, short_message = {:?}",
        sm.short_message().len(),
        sm.data_coding,
        sm.short_message()
    );

    Ok(())
}
