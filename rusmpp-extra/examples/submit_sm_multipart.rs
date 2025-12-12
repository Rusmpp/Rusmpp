//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_multipart --features="alloc,concatenation"
//! ```

use std::str::FromStr;

use rusmpp_core::{
    pdus::owned::SubmitSm,
    types::owned::{COctetString, OctetString},
    values::{DataCoding, EsmClass, Npi, Ton},
};
use rusmpp_extra::concatenation::owned::SubmitSmMultipartExt;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // c-spell: disable
    let message = r##"GSM 3 parts : Hello world!

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
        // esm_class will be updated with UDHI indicator by the multipart builder.
        .esm_class(EsmClass::default())
        // data_coding will be overridden by the multipart builder to match the encoder.
        .data_coding(DataCoding::default())
        // short_message will be overridden by `short_message` of the multipart builder.
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build()
        .multipart()
        .short_message(message)
        .reference_u16(1)
        .gsm7bit_unpacked()
        .build()?;

    let total = multipart.len();

    println!("Submitting multipart message: total {total}");

    for (i, sm) in multipart.into_iter().enumerate() {
        println!(
            "Submitting part {}: short_message_len = {}, esm_class = {:?}, data_coding = {:?}, short_message = {:?}",
            i + 1,
            sm.short_message().len(),
            sm.esm_class,
            sm.data_coding,
            sm.short_message()
        );
        println!()
    }

    Ok(())
}
