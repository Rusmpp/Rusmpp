use crate::{
    codecs::{
        gsm7bit::{
            errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError},
            unpacked::Gsm7BitUnpacked,
        },
        owned::Encoder,
    },
    concatenation::{
        MAX_PARTS,
        owned::{Concatenation, Concatenator},
    },
};

mod encode {
    use super::*;

    #[test]
    fn encode() {
        // c-spell: disable
        let input = r##"Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
        // c-spell: enable

        let (encoded, _) = Gsm7BitUnpacked::new()
            .encode(input)
            .expect("Encoding failed");

        let expected: &[u8] = &[
            // "Hello world!\n\n"
            b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 0x0a, 0x0a,
            // 00–09, 0b–0c, 0e–0f
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0b, 0x0c, 0x0e, 0x0f,
            // 10–1f
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1c, 0x1d, 0x1e,
            0x1f,
            // ASCII printable range 0x20–0x7f
            // !"#$%&'()*+,-./0123456789:;<=>?@
            b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-',
            b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';',
            b'<', b'=', b'>', b'?', b'@', // A–Z
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
            b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', // [\]^_`
            b'[', b'\\', b']', b'^', b'_', b'`', // a–z
            b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y',
            b'z', // {|}~ 0x7f
            b'{', b'|', b'}', b'~', 0x7f, // 0a 0a
            0x0a, 0x0a, // 1b 14 1b ( 1b ) 1b / 1b < 1b = 1b > 1b @ 1b e
            0x1b, 0x14, 0x1b, b'(', 0x1b, b')', 0x1b, b'/', 0x1b, b'<', 0x1b, b'=', 0x1b, b'>',
            0x1b, b'@', 0x1b, b'e',
        ];

        assert_eq!(encoded.as_slice(), expected);
    }

    #[test]
    fn cases() {
        // c-spell: disable
        let cases: &[(&str, &[u8])] = &[
            ("", &[]),
            ("1", &[0x31]),
            ("12", &[0x31, 0x32]),
            ("123", &[0x31, 0x32, 0x33]),
            ("1234", &[0x31, 0x32, 0x33, 0x34]),
            ("12345", &[0x31, 0x32, 0x33, 0x34, 0x35]),
            ("123456", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36]),
            ("1234567", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37]),
            ("12345678", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38]),
            ("123456789", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]),

            ("12345[6", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x1B, 0x3C, 0x36]),

            (
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec nunc venenatis, ultricies ipsum id, volutpat ante. Sed pretium ac metus a interdum metus.",
                b"\x4C\x6F\x72\x65\x6D\x20\x69\x70\x73\x75\x6D\x20\x64\x6F\x6C\x6F\x72\x20\x73\x69\x74\x20\x61\x6D\x65\x74\x2C\x20\x63\x6F\x6E\x73\x65\x63\x74\x65\x74\x75\x72\x20\x61\x64\x69\x70\x69\x73\x63\x69\x6E\x67\x20\x65\x6C\x69\x74\x2E\x20\x43\x75\x72\x61\x62\x69\x74\x75\x72\x20\x6E\x65\x63\x20\x6E\x75\x6E\x63\x20\x76\x65\x6E\x65\x6E\x61\x74\x69\x73\x2C\x20\x75\x6C\x74\x72\x69\x63\x69\x65\x73\x20\x69\x70\x73\x75\x6D\x20\x69\x64\x2C\x20\x76\x6F\x6C\x75\x74\x70\x61\x74\x20\x61\x6E\x74\x65\x2E\x20\x53\x65\x64\x20\x70\x72\x65\x74\x69\x75\x6D\x20\x61\x63\x20\x6D\x65\x74\x75\x73\x20\x61\x20\x69\x6E\x74\x65\x72\x64\x75\x6D\x20\x6D\x65\x74\x75\x73\x2E",
            ),

            ("\n", &[0x0A]),
            ("\r", &[0x0D]),

            (
                "^{}\\[~]|€",
                &[
                    0x1B, 0x14,
                    0x1B, 0x28,
                    0x1B, 0x29,
                    0x1B, 0x2F,
                    0x1B, 0x3C,
                    0x1B, 0x3D,
                    0x1B, 0x3E,
                    0x1B, 0x40,
                    0x1B, 0x65,
                ],
            ),

            (
                "@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !\"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà",
                b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0B\x0C\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1C\x1D\x1E\x1F\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F",
            ),
        ];
        // c-spell: enable

        let encoder = Gsm7BitUnpacked::new();

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

            let encoder = Gsm7BitUnpacked::new();

            let err = encoder.encode(message).unwrap_err();

            assert!(matches!(err, Gsm7BitEncodeError::UnencodableCharacter('✓')))
        }
    }
}

mod concatenate {

    use super::*;

    mod error {
        use super::*;

        // We have to concatenate but the part size was zero
        #[test]
        fn zero_part_size() {
            let message = "1234567";
            let max_message_size = 6;
            let part_header_size = 6;

            let encoder = Gsm7BitUnpacked::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Gsm7BitConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn zero_message_size() {
            let message = "1234567";
            let max_message_size = 0;
            let part_header_size = 6;

            let encoder = Gsm7BitUnpacked::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Gsm7BitConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn parts_count_exceeded() {
            let max_message_size = 6;
            let part_header_size = 0;
            let message = "123456".repeat(MAX_PARTS + 1);

            let encoder = Gsm7BitUnpacked::new();

            let err = encoder
                .concatenate(&message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(
                err,
                Gsm7BitConcatenateError::PartsCountExceeded { actual: 256, .. }
            ))
        }

        mod no_split {
            use super::*;

            // We have to split and part size = 1 and we do not allow splitting extended chars
            #[test]
            fn extended_character_no_split() {
                // encoded to 11 bytes, we split at 10 (0x1B) of encoded `[` but the part size is 1. so the char must be split
                let message = "123456789[";
                let max_message_size = 9;
                let part_header_size = 8;

                let encoder = Gsm7BitUnpacked::new();

                let err = encoder
                    .concatenate(message, max_message_size, part_header_size)
                    .unwrap_err();

                assert!(matches!(err, Gsm7BitConcatenateError::InvalidBoundary));
            }
        }

        mod split {
            use super::*;

            // We have to split and part size = 1 but we allow splitting extended chars
            #[test]
            fn extended_character_split() {
                // encoded to 11 bytes, we split at 10 (0x1B) of encoded `[` even though the part size is 1, we allow splitting
                let message = "123456789[";
                let max_message_size = 9;
                let part_header_size = 8;

                let encoder = Gsm7BitUnpacked::new().with_allow_split_extended_character(true);

                let (concatenation, _) = encoder
                    .concatenate(message, max_message_size, part_header_size)
                    .expect("Concatenation failed");

                let Concatenation::Concatenated(parts) = concatenation else {
                    panic!("Expected concatenated message");
                };

                assert_eq!(parts.len(), 11);

                assert_eq!(parts[9], &[0x1B]);
                assert_eq!(parts[10], &[0x3C]);
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
                name: "one_part",
                message: "123456789",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[b"123456789"]),
            },
            TestCase {
                name: "two_parts",
                message: "123456789123456789",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[b"1234567891", b"23456789"]),
            },
            TestCase {
                name: "concatenate_on_extended_character_once_no_split",
                message: "123456789[3456789",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[
                    b"123456789",
                    &[0x1B, 0x3C, b'3', b'4', b'5', b'6', b'7', b'8', b'9'],
                ]),
            },
            TestCase {
                name: "concatenate_on_extended_character_once_split",
                message: "123456789[3456789",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: true,
                expected: Ok(&[
                    &[b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0x1B],
                    &[0x3C, b'3', b'4', b'5', b'6', b'7', b'8', b'9'],
                ]),
            },
            TestCase {
                name: "concatenate_on_extended_character_three_times_no_split",
                message: "123456789[23456789[23456789[",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: false,
                expected: Ok(&[
                    b"123456789",
                    &[0x1B, 0x3C, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'],
                    &[0x1B, 0x3C, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'],
                    &[0x1B, 0x3C],
                ]),
            },
            TestCase {
                name: "concatenate_on_extended_character_three_times_split",
                message: "123456789[23456789[23456789[",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_extended_character: true,
                expected: Ok(&[
                    &[b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0x1B],
                    &[0x3C, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0x1B],
                    &[0x3C, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0x1B],
                    &[0x3C],
                ]),
            },
        ];

        for case in cases {
            let encoder = Gsm7BitUnpacked::new()
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
