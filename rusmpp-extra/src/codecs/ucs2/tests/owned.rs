use crate::{
    codecs::{
        owned::Encoder,
        ucs2::{Ucs2, Ucs2ConcatenateError, Ucs2EncodeError},
    },
    concatenation::{
        MAX_PARTS,
        owned::{Concatenation, Concatenator},
    },
};

mod encode {
    use super::*;

    mod error {
        use super::*;

        #[test]
        fn unencodable_character() {
            let message = "Hi 😀";

            let encoder = Ucs2::new();

            let err = encoder.encode(message).unwrap_err();

            assert!(matches!(err, Ucs2EncodeError::UnencodableCharacter))
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

            let encoder = Ucs2::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Ucs2ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn zero_message_size() {
            let message = "1234567";
            let max_message_size = 0;
            let part_header_size = 6;

            let encoder = Ucs2::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Ucs2ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn parts_count_exceeded() {
            let max_message_size = 6;
            let part_header_size = 0;
            let message = "123".repeat(MAX_PARTS + 1);

            let encoder = Ucs2::new();

            let err = encoder
                .concatenate(&message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(
                err,
                Ucs2ConcatenateError::PartsCountExceeded { actual: 256, .. }
            ))
        }

        mod no_split {
            use super::*;

            #[test]
            fn character_no_split() {
                // We would split at index 10 of encoded `5` but the part size is 1. so the char must be split
                let message = "12345";
                let max_message_size = 9;
                let part_header_size = 8;

                let encoder = Ucs2::new();

                let err = encoder
                    .concatenate(message, max_message_size, part_header_size)
                    .unwrap_err();

                assert!(matches!(err, Ucs2ConcatenateError::InvalidBoundary));
            }
        }

        mod split {
            use super::*;

            #[test]
            fn character_split() {
                // We would split at index 10 of encoded `5` but the part size is 1. But we allow splitting
                let message = "12345";
                let max_message_size = 9;
                let part_header_size = 8;

                let encoder = Ucs2::new().with_allow_split_character(true);

                let (concatenation, _) = encoder
                    .concatenate(message, max_message_size, part_header_size)
                    .expect("Concatenation failed");

                let Concatenation::Concatenated(parts) = concatenation else {
                    panic!("Expected concatenated message");
                };

                assert_eq!(parts.len(), 10);

                assert_eq!(parts[8], &[0x00]);
                assert_eq!(parts[9], &[0x35]);
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
            allow_split_character: bool,
            expected: Result<&'static [&'static [u8]], Ucs2ConcatenateError>,
        }

        let cases: &[TestCase] = &[
            TestCase {
                name: "empty_message",
                message: "",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_character: false,
                expected: Ok(&[&[]]),
            },
            TestCase {
                name: "one_part",
                message: "12345",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_character: false,
                expected: Ok(&[&[0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00, 0x35]]),
            },
            TestCase {
                name: "two_parts",
                message: "1234512345",
                max_message_size: 16,
                part_header_size: 6,
                allow_split_character: false,
                expected: Ok(&[
                    &[0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00, 0x35],
                    &[0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00, 0x35],
                ]),
            },
            TestCase {
                name: "concatenate_on_leading_surrogate_once_no_split",
                message: "1234512345",
                max_message_size: 16,
                part_header_size: 7,
                allow_split_character: false,
                expected: Ok(&[
                    &[0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34],
                    &[0x00, 0x35, 0x00, 0x31, 0x00, 0x32, 0x00, 0x33],
                    &[0x00, 0x34, 0x00, 0x35],
                ]),
            },
            TestCase {
                name: "concatenate_on_leading_surrogate_once_split",
                message: "1234512345",
                max_message_size: 16,
                part_header_size: 7,
                allow_split_character: true,
                expected: Ok(&[
                    &[0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00],
                    &[0x35, 0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34],
                    &[0x00, 0x35],
                ]),
            },
            TestCase {
                name: "concatenate_on_leading_surrogate_three_times_no_split",
                message: "123123123",
                max_message_size: 4,
                part_header_size: 1,
                allow_split_character: false,
                expected: Ok(&[
                    &[0x00, 0x31],
                    &[0x00, 0x32],
                    &[0x00, 0x33],
                    &[0x00, 0x31],
                    &[0x00, 0x32],
                    &[0x00, 0x33],
                    &[0x00, 0x31],
                    &[0x00, 0x32],
                    &[0x00, 0x33],
                ]),
            },
            TestCase {
                name: "concatenate_on_leading_surrogate_three_times_split",
                message: "123123123",
                max_message_size: 4,
                part_header_size: 1,
                allow_split_character: true,
                expected: Ok(&[
                    &[0x00, 0x31, 0x00],
                    &[0x32, 0x00, 0x33],
                    &[0x00, 0x31, 0x00],
                    &[0x32, 0x00, 0x33],
                    &[0x00, 0x31, 0x00],
                    &[0x32, 0x00, 0x33],
                ]),
            },
        ];

        for case in cases {
            let encoder = Ucs2::new().with_allow_split_character(case.allow_split_character);

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
