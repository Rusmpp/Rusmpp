#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use alloc::string::String;

    use crate::encoding::{latin1::Latin1DecodeError, owned::Decoder};

    /// An incremental Latin1 decoder.
    #[derive(Debug)]
    pub struct Latin1Decoder {
        /// Decoded output string.
        output: String,
    }

    impl Default for Latin1Decoder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Latin1Decoder {
        /// Creates a new [`Latin1Decoder`].
        pub const fn new() -> Self {
            Self {
                output: String::new(),
            }
        }
    }

    impl Decoder for Latin1Decoder {
        type Error = Latin1DecodeError;

        fn feed(&mut self, input: &[u8], _: usize) -> Result<(), Self::Error> {
            // Every Latin1 byte maps to a Unicode scalar value in U+0000..=U+00FF.
            for &byte in input {
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
