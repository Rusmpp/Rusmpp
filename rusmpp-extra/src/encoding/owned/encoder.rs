use rusmpp_core::values::DataCoding;

/// A trait for encoding messages into byte vectors.
pub trait Encoder {
    /// The type of errors that can occur during encoding.
    type Error;

    /// Encodes the given message into a vector of bytes and its associated [`DataCoding`].
    fn encode(&self, message: &str) -> Result<(alloc::vec::Vec<u8>, DataCoding), Self::Error>;
}
