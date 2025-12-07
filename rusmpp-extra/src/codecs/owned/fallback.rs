use rusmpp_core::values::DataCoding;

use crate::{
    codecs::owned::Encoder,
    fallback::{Fallback, FallbackError},
};

impl<T, U> Encoder for Fallback<T, U>
where
    T: Encoder,
    U: Encoder,
{
    type Error = FallbackError<T::Error, U::Error>;

    fn encode(&self, message: &str) -> Result<(alloc::vec::Vec<u8>, DataCoding), Self::Error> {
        match self.first.encode(message) {
            Ok(result) => Ok(result),
            Err(first_err) => match self.second.encode(message) {
                Ok(result) => Ok(result),
                Err(second_err) => Err(FallbackError::new(first_err, second_err)),
            },
        }
    }
}
