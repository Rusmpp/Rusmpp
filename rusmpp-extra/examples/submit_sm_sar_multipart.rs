//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_sar_multipart --features="alloc,concatenation"
//! ```

use std::str::FromStr;

use rusmpp_core::{
    pdus::owned::SubmitSm,
    types::owned::{COctetString, OctetString},
    values::{Npi, Ton},
};
use rusmpp_extra::concatenation::owned::Multipart;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // c-spell: disable
    let message = r##"GSM 3 parts : !

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
    // c-spell: enable

    let multipart = SubmitSm::builder()
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        // short_message will be overridden by `short_message` of the multipart builder.
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build()
        .sar_multipart(message)
        .reference(1)
        .gsm7bit_unpacked()
        // SAR Tlvs will be added by the multipart builder.
        .build()?;

    let total = multipart.len();

    println!("Submitting sar multipart message: total {total}");

    for (i, sm) in multipart.into_iter().enumerate() {
        println!(
            "Submitting part {}: short_message_len = {}, tlvs = {:?}, short_message = {:?}",
            i + 1,
            sm.short_message().len(),
            sm.tlvs(),
            sm.short_message()
        );
        println!()
    }

    Ok(())
}
