use rusmpp_core::values::DataCoding;

use crate::{
    concatenation::owned::{Concatenation, Concatenator},
    fallback::{Fallback, FallbackError},
};

impl<T, U> Concatenator for Fallback<T, U>
where
    T: Concatenator,
    U: Concatenator,
{
    type Error = FallbackError<T::Error, U::Error>;

    fn concatenate(
        &self,
        message: &str,
        max_message_size: usize,
        part_header_size: usize,
    ) -> Result<(Concatenation, DataCoding), Self::Error> {
        match self
            .first
            .concatenate(message, max_message_size, part_header_size)
        {
            Ok(result) => Ok(result),
            Err(first_err) => {
                match self
                    .second
                    .concatenate(message, max_message_size, part_header_size)
                {
                    Ok(result) => Ok(result),
                    Err(second_err) => Err(FallbackError::new(first_err, second_err)),
                }
            }
        }
    }
}
