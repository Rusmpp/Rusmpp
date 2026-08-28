//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_multipart_decode --features="alloc,concatenation"
//! ```

use std::collections::HashMap;

use itertools::Itertools;
use rusmpp_core::{pdus::owned::SubmitSm, values::DataCoding};
use rusmpp_extra::{
    concatenation::owned::SubmitSmMultipartExt,
    encoding::{
        gsm7bit::{Gsm7BitPackedDecoder, Gsm7BitUnpackedDecoder},
        owned::{Decoder, SupportedDecoder},
    },
};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // c-spell: disable
    let message = r##"GSM 3 parts : Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
    // c-spell: enable

    // Normally we do not have to set the data coding manually, the multipart builder will set it for us,
    // but we are setting them here explicitly to differentiate between `GSM 7 bit unpacked` and `GSM 7 bit packed`,
    // since both of them would set the data codding to `DataCoding::McSpecific` by default.
    // `DataCoding::Other(0b1111111)` is just a made up value for `GSM 7 bit packed`.
    let gsm7bit_unpacked_data_coding = DataCoding::McSpecific;
    let gsm7bit_packed_data_coding = DataCoding::Other(0b1111111);
    let ucs2_data_coding = DataCoding::Ucs2;

    let gsm7bit_unpacked_udh_multipart = SubmitSm::builder()
        .build()
        .multipart(message)
        .reference_u8(1)
        .gsm7bit_unpacked()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_unpacked_data_coding));

    let gsm7bit_unpacked_single = SubmitSm::builder()
        .build()
        .multipart("Hi, I am single")
        .reference_u8(2)
        .gsm7bit_unpacked()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_unpacked_data_coding));

    let gsm7bit_packed_udh_multipart = SubmitSm::builder()
        .build()
        .multipart(message)
        .reference_u8(3)
        .gsm7bit_packed()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(gsm7bit_packed_data_coding));

    let ucs2_sar_multipart = SubmitSm::builder()
        .build()
        .sar_multipart(message)
        .reference(4)
        .ucs2()
        .build()?
        .into_iter()
        .map(|sm| sm.with_data_coding(ucs2_data_coding));

    // Shuffle the messages to simulate receiving them in a random order.
    let messages = gsm7bit_unpacked_udh_multipart
        .into_iter()
        .interleave(gsm7bit_packed_udh_multipart)
        .interleave(gsm7bit_unpacked_single)
        .interleave(ucs2_sar_multipart);

    fn decoder(data_coding: DataCoding) -> Option<SupportedDecoder> {
        match data_coding {
            DataCoding::McSpecific => Some(SupportedDecoder::Gsm7BitUnpacked(
                Gsm7BitUnpackedDecoder::new(),
            )),
            DataCoding::Other(0b1111111) => {
                Some(SupportedDecoder::Gsm7BitPacked(Gsm7BitPackedDecoder::new()))
            }
            _ => None,
        }
    }

    let mut state: HashMap<u16, SupportedDecoder> = HashMap::new();

    for sm in messages {
        match sm.multipart_segment().transpose()? {
            Some((segment, message)) => {
                let Some(decoder) = decoder(sm.data_coding) else {
                    println!("Unsupported data coding: {:?}", sm.data_coding);

                    continue;
                };

                if segment.is_first() {
                    state.insert(segment.reference, decoder);
                }

                if let Some(decoder) = state.get_mut(&segment.reference) {
                    decoder.feed(message, segment.header_size())?;
                }

                if segment.is_last() {
                    if let Some(decoder) = state.remove(&segment.reference) {
                        let decoded = decoder.finish()?;

                        println!("Decoded message: {decoded}");
                    }
                }
            }
            None => {
                let Some(mut decoder) = decoder(sm.data_coding) else {
                    println!("Unsupported data coding: {:?}", sm.data_coding);

                    continue;
                };

                decoder.feed(sm.short_message(), 0)?;

                let decoded = decoder.finish()?;

                println!("Decoded message: {decoded}");
            }
        }
    }

    Ok(())
}
