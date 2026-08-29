#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use alloc::string::String;

    use crate::encoding::{ascii::AsciiDecodeError, owned::Decoder};

    /// An incremental ASCII decoder.
    #[derive(Debug)]
    pub struct AsciiDecoder {
        /// Decoded output string.
        output: String,
    }

    impl Default for AsciiDecoder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AsciiDecoder {
        /// Creates a new [`AsciiDecoder`].
        pub const fn new() -> Self {
            Self {
                output: String::new(),
            }
        }
    }

    impl Decoder for AsciiDecoder {
        type Error = AsciiDecodeError;

        fn feed(&mut self, input: &[u8], _: usize) -> Result<(), Self::Error> {
            for &byte in input {
                if byte > 0x7F {
                    return Err(AsciiDecodeError::InvalidByte(byte));
                }

                self.output.push(char::from(byte));
            }

            Ok(())
        }

        fn peek(&self) -> &str {
            &self.output
        }

        fn finish(self) -> Result<String, Self::Error> {
            Ok(self.output)
        }
    }
}
