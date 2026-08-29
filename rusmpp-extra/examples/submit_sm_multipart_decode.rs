//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_multipart_decode --features="alloc,concatenation"
//! ```

use std::collections::HashMap;

use itertools::Itertools;
use rusmpp_core::{pdus::owned::SubmitSm, values::DataCoding};
use rusmpp_extra::{
    concatenation::owned::Multipart,
    encoding::{
        gsm7bit::{Gsm7BitPackedDecoder, Gsm7BitUnpackedDecoder},
        latin1::Latin1Decoder,
        owned::{Decoder, SupportedDecoder},
        ucs2::Ucs2Decoder,
    },
};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // c-spell: disable
    let gsm7bit_message = r##"Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;

    let ucs2_message = "Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! Hello world!";

    let latin1_message = "Hello Latin1!";

    let unsupported_data_coding_message =
        "Hello world! This message has an unsupported data coding";
    // c-spell: enable

    // Normally we do not have to set the data coding manually, the multipart builder will set it for us,
    // but we are setting them here explicitly to differentiate between `GSM 7 bit unpacked` and `GSM 7 bit packed`,
    // since both of them would set the data codding to `DataCoding::McSpecific` by default.
    // `DataCoding::Other(0b1111111)` is just a made up value for `GSM 7 bit packed`.
    let gsm7bit_unpacked_data_coding = DataCoding::McSpecific;
    let gsm7bit_packed_data_coding = DataCoding::Other(0b1111111);
    let latin1_data_coding = DataCoding::Latin1;
    let ucs2_data_coding = DataCoding::Ucs2;
    let unsupported_data_coding = DataCoding::Cyrillic;

    let gsm7bit_unpacked_udh_multipart = SubmitSm::builder()
        .build()
        .udh_multipart(gsm7bit_message)
        .reference_u8(1)
        .gsm7bit_unpacked()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_unpacked_data_coding));

    let gsm7bit_unpacked_single = SubmitSm::builder()
        .build()
        .udh_multipart("Hi, I am single")
        .reference_u8(2)
        .gsm7bit_unpacked()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_unpacked_data_coding));

    let gsm7bit_packed_udh_multipart = SubmitSm::builder()
        .build()
        .udh_multipart(gsm7bit_message)
        .reference_u16(3)
        .gsm7bit_packed()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_packed_data_coding));

    let ucs2_sar_multipart = SubmitSm::builder()
        .build()
        .sar_multipart(ucs2_message)
        .reference(4)
        .ucs2()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(ucs2_data_coding));

    let latin1_single = SubmitSm::builder()
        .build()
        .sar_multipart(latin1_message)
        .reference(5)
        .latin1()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(latin1_data_coding));

    let unsupported_data_coding_single = SubmitSm::builder()
        .build()
        .udh_multipart(unsupported_data_coding_message)
        .reference_u8(5)
        .gsm7bit_unpacked()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(unsupported_data_coding));

    // Shuffle the messages to simulate receiving them in a random order.
    let messages = gsm7bit_unpacked_udh_multipart
        .into_iter()
        .interleave(gsm7bit_packed_udh_multipart)
        .interleave(gsm7bit_unpacked_single)
        .interleave(ucs2_sar_multipart)
        .interleave(latin1_single)
        .interleave(unsupported_data_coding_single);

    fn decoder(data_coding: DataCoding) -> Option<SupportedDecoder> {
        match data_coding {
            DataCoding::McSpecific => Some(SupportedDecoder::Gsm7BitUnpacked(
                Gsm7BitUnpackedDecoder::new(),
            )),
            DataCoding::Other(0b1111111) => {
                Some(SupportedDecoder::Gsm7BitPacked(Gsm7BitPackedDecoder::new()))
            }
            DataCoding::Ucs2 => Some(SupportedDecoder::Ucs2(Ucs2Decoder::new())),
            DataCoding::Latin1 => Some(SupportedDecoder::Latin1(Latin1Decoder::new())),
            _ => None,
        }
    }

    let mut state: HashMap<u16, SupportedDecoder> = HashMap::new();

    for sm in messages {
        let Some(mut decoder) = decoder(sm.data_coding) else {
            println!("Unsupported data coding: {:?}", sm.data_coding);

            println!();
            println!("--------------------------------");
            println!();

            continue;
        };

        match sm.multipart_segment().transpose()? {
            Some((segment, message)) => {
                println!(
                    "Received multipart segment: data coding = {:?}, reference = {}, total_parts = {}, part_number = {}, header_size = {}, message_len = {}",
                    sm.data_coding,
                    segment.reference,
                    segment.total_parts,
                    segment.part_number,
                    segment.header_size(),
                    message.len()
                );

                if segment.is_first() {
                    state.insert(segment.reference, decoder);
                }

                if let Some(decoder) = state.get_mut(&segment.reference) {
                    decoder.feed(message, segment.header_size())?;

                    println!(
                        "Decoded so far: data coding = {:?}, reference = {}, total_parts = {}, part_number = {}:",
                        sm.data_coding, segment.reference, segment.total_parts, segment.part_number,
                    );
                    println!("{}", decoder.peek());
                }

                if segment.is_last() {
                    if let Some(decoder) = state.remove(&segment.reference) {
                        let decoded = decoder.finish()?;

                        println!(
                            "Decoded multipart message: data coding = {:?}, reference = {}, total_parts = {}, part_number = {}:",
                            sm.data_coding,
                            segment.reference,
                            segment.total_parts,
                            segment.part_number
                        );
                        println!("{decoded}");
                    }
                }
            }
            None => {
                println!(
                    "Received single part message: data coding = {:?}, short_message_len = {}",
                    sm.data_coding,
                    sm.short_message().len()
                );

                decoder.feed(sm.short_message(), 0)?;

                let decoded = decoder.finish()?;

                println!(
                    "Decoded single part message: data coding = {:?}, short_message_len = {}:",
                    sm.data_coding,
                    sm.short_message().len()
                );
                println!("{decoded}");
            }
        }

        println!();
        println!("--------------------------------");
        println!();
    }

    Ok(())
}
