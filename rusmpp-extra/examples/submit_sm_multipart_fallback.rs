//! Run with
//!
//! ```not_rust
//! cargo run -p rusmpp-extra --example submit_sm_multipart_fallback --features="alloc,concatenation"
//! ```

// TODO: re-enable
fn main() {}

// use std::str::FromStr;

// use rusmpp_core::{
//     pdus::owned::SubmitSm,
//     types::owned::COctetString,
//     values::{Npi, Ton},
// };
// use rusmpp_extra::{
//     concatenation::owned::Multipart,
//     encoding::{
//         gsm7bit::{Gsm7BitAlphabet, Gsm7BitUnpackedEncoder},
//         ucs2::Ucs2Encoder,
//     },
// };

// fn main() -> Result<(), Box<dyn core::error::Error>> {
//     let multipart = SubmitSm::builder()
//         .source_addr_ton(Ton::Unknown)
//         .source_addr_npi(Npi::Unknown)
//         .source_addr(COctetString::from_str("12345")?)
//         .destination_addr(COctetString::from_str("491701234567")?)
//         .build()
//         .udh_multipart("Hi This message can not be encoded in gsm 7-bit default alphabet so it will fallback to ucs2: 你好")
//         .encoder(Gsm7BitUnpackedEncoder::new().with_alphabet(Gsm7BitAlphabet::default()))
//         .fallback(Ucs2Encoder::new())
//         .build()?;

//     let total = multipart.len();

//     println!("Submitting multipart message: total {total}");

//     for (i, sm) in multipart.into_iter().enumerate() {
//         assert_eq!(sm.data_coding, Ucs2Encoder::new().data_coding());

//         println!(
//             "Submitting part {}: short_message_len = {}, esm_class = {:?}, data_coding = {:?}, short_message = {:?}",
//             i + 1,
//             sm.short_message().len(),
//             sm.esm_class,
//             sm.data_coding,
//             sm.short_message()
//         );
//         println!()
//     }

//     Ok(())
// }
