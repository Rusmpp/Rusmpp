#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use alloc::string::String;

    use crate::encoding::{
        gsm7bit::{ESCAPE_CHARACTER, Encoded, Gsm7BitDecodeError, alphabet::Gsm7BitAlphabet},
        owned::Decoder,
    };

    /// An incremental GSM 7-bit unpacked decoder.
    #[derive(Debug)]
    pub struct Gsm7BitUnpackedDecoder {
        /// The GSM 7-bit alphabet to use for decoding.
        alphabet: Gsm7BitAlphabet,
        /// The decoded output string.
        output: String,
        /// true if the previous [`Self::feed()`] ended on a lone [`ESCAPE_CHARACTER`].
        pending_escape: bool,
    }

    impl Default for Gsm7BitUnpackedDecoder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Gsm7BitUnpackedDecoder {
        /// Creates a new [`Gsm7BitUnpackedDecoder`] with [`Gsm7BitAlphabet::Default`].
        pub const fn new() -> Self {
            Self {
                alphabet: Gsm7BitAlphabet::default(),
                output: String::new(),
                pending_escape: false,
            }
        }

        /// Sets the alphabet for the decoder.
        pub const fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
            self.alphabet = alphabet;
            self
        }

        /// Returns the associated [`Gsm7BitAlphabet`].
        pub const fn alphabet(&self) -> &Gsm7BitAlphabet {
            &self.alphabet
        }
    }

    impl Decoder for Gsm7BitUnpackedDecoder {
        type Error = Gsm7BitDecodeError;

        fn feed(&mut self, input: &[u8], _: usize) -> Result<(), Self::Error> {
            let mut i = 0;

            if self.pending_escape {
                let Some(&next) = input.first() else {
                    return Ok(());
                };

                let ch = self
                    .alphabet
                    .decode(Encoded::Extended(next))
                    .ok_or(Gsm7BitDecodeError::InvalidExtendedByte(next))?;

                self.output.push(ch);

                self.pending_escape = false;

                i = 1;
            }

            while i < input.len() {
                let byte = input[i];

                if byte == ESCAPE_CHARACTER {
                    match input.get(i + 1) {
                        Some(&next) => {
                            let ch = self
                                .alphabet
                                .decode(Encoded::Extended(next))
                                .ok_or(Gsm7BitDecodeError::InvalidExtendedByte(next))?;

                            self.output.push(ch);

                            i += 2;
                        }
                        None => {
                            self.pending_escape = true;

                            return Ok(());
                        }
                    }
                } else {
                    let ch = self
                        .alphabet
                        .decode(Encoded::Standard(byte))
                        .ok_or(Gsm7BitDecodeError::InvalidByte(byte))?;

                    self.output.push(ch);

                    i += 1;
                }
            }

            Ok(())
        }

        fn peek(&self) -> &str {
            &self.output
        }

        fn finish(self) -> Result<String, Self::Error> {
            if self.pending_escape {
                return Err(Gsm7BitDecodeError::TrailingEscape);
            };

            Ok(self.output)
        }
    }
}
