#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use alloc::string::String;

    use crate::encoding::{owned::Decoder, ucs2::Ucs2DecodeError};

    /// An incremental UCS2 decoder.
    #[derive(Debug)]
    pub struct Ucs2Decoder {
        /// Decoded output string.
        output: String,
        /// The high byte of a code unit, set if [`Self::feed()`] previously ended mid code-unit .
        pending_high_byte: Option<u8>,
    }

    impl Default for Ucs2Decoder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Ucs2Decoder {
        /// Creates a new [`Ucs2Decoder`].
        pub const fn new() -> Self {
            Self {
                output: String::new(),
                pending_high_byte: None,
            }
        }
    }

    impl Decoder for Ucs2Decoder {
        type Error = Ucs2DecodeError;

        fn feed(&mut self, input: &[u8], _: usize) -> Result<(), Self::Error> {
            let mut i = 0;
            let mut high_byte = self.pending_high_byte.take();

            while i < input.len() {
                let byte = input[i];
                i += 1;

                match high_byte.take() {
                    Some(hb) => {
                        let code_unit = (u16::from(hb) << 8) | u16::from(byte);

                        let ch = char::from_u32(u32::from(code_unit))
                            .ok_or(Ucs2DecodeError::InvalidCodeUnit(code_unit))?;

                        self.output.push(ch);
                    }
                    None => {
                        high_byte = Some(byte);
                    }
                }
            }

            self.pending_high_byte = high_byte;

            Ok(())
        }

        fn peek(&self) -> &str {
            &self.output
        }

        fn finish(self) -> Result<String, Self::Error> {
            if self.pending_high_byte.is_some() {
                return Err(Ucs2DecodeError::TrailingByte);
            }

            Ok(self.output)
        }
    }
}
