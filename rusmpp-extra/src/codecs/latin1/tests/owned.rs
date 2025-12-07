use crate::{
    codecs::{
        latin1::{Latin1, Latin1ConcatenateError, Latin1EncodeError},
        owned::Encoder,
    },
    concatenation::{MAX_PARTS, owned::Concatenator},
};

mod encode {
    use super::*;

    mod error {
        use super::*;

        #[test]
        fn unencodable_character() {
            let message = "Hi 😀";

            let encoder = Latin1::new();

            let err = encoder.encode(message).unwrap_err();

            assert!(matches!(err, Latin1EncodeError::UnencodableCharacter))
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

            let encoder = Latin1::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Latin1ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn zero_message_size() {
            let message = "1234567";
            let max_message_size = 0;
            let part_header_size = 6;

            let encoder = Latin1::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Latin1ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn parts_count_exceeded() {
            let max_message_size = 6;
            let part_header_size = 0;
            let message = "123456".repeat(MAX_PARTS + 1);

            let encoder = Latin1::new();

            let err = encoder
                .concatenate(&message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(
                err,
                Latin1ConcatenateError::PartsCountExceeded { actual: 256, .. }
            ))
        }
    }

    #[test]
    fn cases() {
        struct TestCase {
            name: &'static str,
            message: &'static str,
            max_message_size: usize,
            part_header_size: usize,
            expected: Result<&'static [&'static [u8]], Latin1ConcatenateError>,
        }

        let cases: &[TestCase] = &[
            TestCase {
                name: "empty_message",
                message: "",
                max_message_size: 16,
                part_header_size: 6,
                expected: Ok(&[&[]]),
            },
            TestCase {
                name: "one_part",
                // cspell: disable-next-line
                message: "agjwklgjkwPÓ",
                max_message_size: 16,
                part_header_size: 4,
                expected: Ok(&[&[
                    0x61, 0x67, 0x6A, 0x77, 0x6B, 0x6C, 0x67, 0x6A, 0x6B, 0x77, 0x50, 0xD3,
                ]]),
            },
            TestCase {
                name: "two_parts",
                // cspell: disable-next-line
                message: "agjwklgjkwPÓ",
                max_message_size: 10,
                part_header_size: 2,
                expected: Ok(&[
                    &[0x61, 0x67, 0x6A, 0x77, 0x6B, 0x6C, 0x67, 0x6A],
                    &[0x6B, 0x77, 0x50, 0xD3],
                ]),
            },
        ];

        for case in cases {
            let encoder = Latin1::new();

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
