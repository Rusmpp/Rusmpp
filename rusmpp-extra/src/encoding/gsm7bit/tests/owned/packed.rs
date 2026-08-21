use crate::{
    concatenation::{MAX_PARTS, owned::Concatenator},
    encoding::{
        gsm7bit::{
            errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError},
            packed::Gsm7BitPacked,
        },
        owned::Encoder,
    },
};

mod encode {
    use super::*;

    #[test]
    fn cases() {
        // c-spell: disable
        let cases: &[(&str, &[u8])] = &[
            ("", &[]),
            ("1", &[0x31]),
            ("12", &[0x31, 0x19]),
            ("123", &[0x31, 0xD9, 0x0C]),
            ("1234", &[0x31, 0xD9, 0x8C, 0x06]),
            ("12345", &[0x31, 0xD9, 0x8C, 0x56, 0x03]),
            ("123456", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0x01]),
            ("1234567", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x1A]), // CR spare-bit fill
            ("12345678", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x70]),
            (
                "123456789",
                &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x70, 0x39],
            ),
            ("\n", &[0x0A]),
            ("\r", &[0x0D]),
            (
                "^{}\\[~]|€",
                &[
                    0x1B, 0xCA, 0x06, 0xB5, 0x49, 0x6D, 0x5E, 0x1B, 0xDE, 0xA6, 0xB7, 0xF1, 0x6D,
                    0x80, 0x9B, 0x32,
                ],
            ),
            (
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec nunc venenatis, ultricies ipsum id, volutpat ante. Sed pretium ac metus a interdum metus.",
                b"\xCC\xB7\xBC\xDC\x06\xA5\xE1\xF3\x7A\x1B\x44\x7E\xB3\xDF\x72\xD0\x3C\x4D\x07\x85\xDB\x65\x3A\x0B\x34\x7E\xBB\xE7\xE5\x31\xBD\x4C\xAF\xCB\x41\x61\x72\x1A\x9E\x9E\x8F\xD3\xEE\x33\xA8\xCC\x4E\xD3\x5D\xA0\x61\x5D\x1E\x16\xA7\xE9\x75\x39\xC8\x5D\x1E\x83\xDC\x75\xF7\x18\x64\x2F\xBB\xCB\xEE\x30\x3D\x3D\x67\x81\xEA\x6C\xBA\x3C\x3D\x4E\x97\xE7\xA0\x34\x7C\x5E\x6F\x83\xD2\x64\x16\xC8\xFE\x66\xD7\xE9\xF0\x30\x1D\x14\x76\xD3\xCB\x2E\xD0\xB4\x4C\x06\xC1\xE5\x65\x7A\xBA\xDE\x06\x85\xC7\xA0\x76\x99\x5E\x9F\x83\xC2\xA0\xB4\x9B\x5E\x96\x93\xEB\x6D\x50\xBB\x4C\xAF\xCF\x5D",
            ),
            (
                "@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !\"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà",
                &[
                    0x80, 0x80, 0x60, 0x40, 0x28, 0x18, 0x0E, 0x88, 0xC4, 0x82, 0xE1, 0x78, 0x40,
                    0x22, 0x92, 0x09, 0xA5, 0x62, 0xB9, 0x60, 0x32, 0x1A, 0x4E, 0xC7, 0xF3, 0x01,
                    0x85, 0x44, 0x23, 0x52, 0xC9, 0x74, 0x42, 0xA5, 0x54, 0x2B, 0x56, 0xCB, 0xF5,
                    0x82, 0xC5, 0x64, 0x33, 0x5A, 0xCD, 0x76, 0xC3, 0xE5, 0x74, 0x3B, 0x5E, 0xCF,
                    0xF7, 0x03, 0x06, 0x85, 0x43, 0x62, 0xD1, 0x78, 0x44, 0x26, 0x95, 0x4B, 0x66,
                    0xD3, 0xF9, 0x84, 0x46, 0xA5, 0x53, 0x6A, 0xD5, 0x7A, 0xC5, 0x66, 0xB5, 0x5B,
                    0x6E, 0xD7, 0xFB, 0x05, 0x87, 0xC5, 0x63, 0x72, 0xD9, 0x7C, 0x46, 0xA7, 0xD5,
                    0x6B, 0x76, 0xDB, 0xFD, 0x86, 0xC7, 0xE5, 0x73, 0x7A, 0xDD, 0x7E, 0xC7, 0xE7,
                    0xF5, 0x7B, 0x7E, 0xDF, 0xFF, 0x07,
                ],
            ),
        ];
        // c-spell: enable

        let encoder = Gsm7BitPacked::new();

        for (text, expected) in cases {
            let (encoded, _) = encoder.encode(text).expect("Encoding failed");

            assert_eq!(encoded, *expected, "Encoding failed for text: {text:?}");
        }
    }

    mod error {
        use super::*;

        #[test]
        fn unencodable_character() {
            let message = "Hi ✓";

            let encoder = Gsm7BitPacked::new();

            let err = encoder.encode(message).unwrap_err();

            assert!(matches!(err, Gsm7BitEncodeError::UnencodableCharacter('✓')))
        }
    }
}

mod concatenate {
    use super::*;

    mod error {
        use super::*;

        // We have to concatenate but the part size (after subtracting the header) was zero.
        #[test]
        fn zero_part_size() {
            let message = "123456789012345678901"; // 21 septets, doesn't fit a single 6-octet part
            let max_message_size = 6;
            let part_header_size = 6;

            let encoder = Gsm7BitPacked::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Gsm7BitConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn zero_message_size() {
            let message = "123456789";
            let max_message_size = 0;
            let part_header_size = 6;

            let encoder = Gsm7BitPacked::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Gsm7BitConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn parts_count_exceeded() {
            let max_message_size = 8;
            let part_header_size = 6;
            let message = "123456".repeat(MAX_PARTS + 1);

            let encoder = Gsm7BitPacked::new();

            let err = encoder
                .concatenate(&message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(
                err,
                Gsm7BitConcatenateError::PartsCountExceeded { .. }
            ))
        }

        mod no_split {
            use super::*;

            // With a 6-octet header and 7-octet parts, each part carries 1
            // septet. Splitting "123456789[" lands the escape byte alone at
            // a part boundary, which must be rejected rather than sent as a
            // dangling escape.
            #[test]
            fn extended_character_no_split() {
                let message = "123456789[";
                let max_message_size = 7;
                let part_header_size = 6;

                let encoder = Gsm7BitPacked::new();

                let err = encoder
                    .concatenate(message, max_message_size, part_header_size)
                    .unwrap_err();

                assert!(matches!(err, Gsm7BitConcatenateError::InvalidBoundary));
            }
        }
    }

    #[test]
    fn cases() {
        struct TestCase {
            name: &'static str,
            message: &'static str,
            max_message_size: usize,
            part_header_size: usize,
            allow_split_extended_character: bool,
            expected: Result<&'static [&'static [u8]], Gsm7BitConcatenateError>,
        }

        let cases: &[TestCase] = &[
            TestCase {
                name: "empty_message",
                message: "",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[&[]]),
            },
            TestCase {
                name: "cr_fill_single_part",
                // 7 septets: exercises the NOTE's CR spare-bit fill inside a
                // concatenation call that still resolves to a single part.
                message: "1234567",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[&[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x1A]]),
            },
            TestCase {
                name: "one_part",
                // 18 septets packs to exactly 16 octets: fits in one part
                // with no header/padding at all.
                message: "123456789012345678",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[&[
                    0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x70, 0x39, 0x58, 0x4C, 0x36, 0xA3, 0xD5,
                    0x6C, 0x37, 0x1C,
                ]]),
            },
            TestCase {
                name: "two_parts",
                // 19 septets: one more than fits in a single 16-octet part,
                // so it splits into an 11-septet part and an 8-septet part
                // (11 = floor((10*8 - 1) / 7) for a 6-octet header).
                message: "1234567890123456789",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[
                    &[0x62, 0xB2, 0x19, 0xAD, 0x66, 0xBB, 0xE1, 0x72, 0xB0, 0x18],
                    &[0x64, 0x33, 0x5A, 0xCD, 0x76, 0xC3, 0xE5, 0x1A],
                ]),
            },
            TestCase {
                name: "concatenate_on_extended_character_no_split",
                // 15 septets, 2-septet parts (7-octet parts, 6-octet header).
                // The escape byte for '[' would land alone at the boundary
                // between parts 5 and 6, so it's pushed into part 6 instead,
                // leaving part 5 with a single septet.
                message: "123456789[6789",
                max_message_size: 8,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[
                    &[0x62, 0x32],
                    &[0x66, 0x34],
                    &[0x6A, 0x36],
                    &[0x6E, 0x38],
                    &[0x72],
                    &[0x36, 0x3C],
                    &[0x6C, 0x37],
                    &[0x70, 0x39],
                ]),
            },
            TestCase {
                name: "concatenate_on_extended_character_split",
                message: "123456789[6789",
                max_message_size: 8,
                part_header_size: 6,
                allow_split_extended_character: true,
                expected: Ok(&[
                    &[0x62, 0x32],
                    &[0x66, 0x34],
                    &[0x6A, 0x36],
                    &[0x6E, 0x38],
                    &[0x72, 0x1B],
                    &[0x78, 0x36],
                    &[0x6E, 0x38],
                    &[0x72],
                ]),
            },
        ];

        for case in cases {
            let encoder = Gsm7BitPacked::new()
                .with_allow_split_extended_character(case.allow_split_extended_character);

            let result =
                encoder.concatenate(case.message, case.max_message_size, case.part_header_size);

            match (result, &case.expected) {
                (Ok((concatenation, _)), Ok(expected_parts)) => {
                    let parts = concatenation.collect().into_iter();

                    assert_eq!(
                        parts.len(),
                        expected_parts.len(),
                        "Test case '{}' failed: number of parts mismatch",
                        case.name
                    );

                    for (part, expected) in parts.zip(expected_parts.iter()) {
                        assert_eq!(
                            part.as_slice(),
                            *expected,
                            "Test case '{}' failed: part content mismatch",
                            case.name
                        );
                    }
                }
                (Err(err), Err(expected_err)) => {
                    assert_eq!(
                        &err, expected_err,
                        "Test case '{}' failed: error mismatch",
                        case.name
                    );
                }
                _ => panic!(
                    "Test case '{}' failed: result and expected do not match",
                    case.name
                ),
            }
        }
    }
}
