#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use alloc::string::String;

    use crate::encoding::{
        gsm7bit::{
            Gsm7BitDecodeError, Gsm7BitPackedEncoder, Gsm7BitUnpackedDecoder,
            alphabet::Gsm7BitAlphabet,
        },
        owned::Decoder,
    };

    /// An incremental GSM 7-bit packed decoder.
    #[derive(Debug)]
    pub struct Gsm7BitPackedDecoder {
        decoder: Gsm7BitUnpackedDecoder,
    }

    impl Default for Gsm7BitPackedDecoder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Gsm7BitPackedDecoder {
        /// Creates a new [`Gsm7BitUnpackedDecoder`] with [`Gsm7BitAlphabet::Default`].
        pub const fn new() -> Self {
            Self {
                decoder: Gsm7BitUnpackedDecoder::new(),
            }
        }

        /// Sets the alphabet for the decoder.
        pub fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
            self.decoder = self.decoder.with_alphabet(alphabet);
            self
        }

        /// Returns the associated [`Gsm7BitAlphabet`].
        pub const fn alphabet(&self) -> &Gsm7BitAlphabet {
            self.decoder.alphabet()
        }
    }

    impl Decoder for Gsm7BitPackedDecoder {
        type Error = Gsm7BitDecodeError;

        fn feed(&mut self, input: &[u8], header_size: usize) -> Result<(), Self::Error> {
            let padding = Gsm7BitPackedEncoder::padding(header_size);
            let n_septets = Gsm7BitPackedEncoder::n_septets(input.len(), padding);

            let unpacked = Gsm7BitPackedEncoder::unpack_pop_cr_padding(input, padding, n_septets);

            self.decoder.feed(&unpacked, header_size)
        }

        fn peek(&self) -> &str {
            self.decoder.peek()
        }

        fn finish(self) -> Result<String, Self::Error> {
            self.decoder.finish()
        }
    }
}
