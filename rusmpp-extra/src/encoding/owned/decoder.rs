/// A trait for decoding messages from byte vectors.
pub trait Decoder {
    /// The type of errors that can occur during decoding.
    type Error;

    /// Feeds the given input bytes into the decoder.
    fn feed(&mut self, input: &[u8], header_size: usize) -> Result<(), Self::Error>;

    /// Peeks at the current state of the decoder without consuming it.
    fn peek(&self) -> &str;

    /// Finishes the decoding process and returns the final decoded string.
    fn finish(self) -> Result<alloc::string::String, Self::Error>;
}
