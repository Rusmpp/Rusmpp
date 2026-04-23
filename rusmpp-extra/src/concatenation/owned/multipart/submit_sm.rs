use rusmpp_core::pdus::owned::SubmitSm;

use crate::encoding::gsm7bit::Gsm7BitUnpacked;

use super::SubmitSmMultipartBuilder;

/// Extension trait for [`SubmitSm`] to create multipart messages.
pub trait SubmitSmMultipartExt {
    /// Creates a new [`SubmitSmMultipartBuilder`] with the default [`Gsm7BitUnpacked`] encoder.
    ///
    /// # Notes
    ///
    /// - [`SubmitSm::esm_class`] will be updated with UDHI indicator by the multipart builder.
    /// - [`SubmitSm::data_coding`] will be overridden by the multipart builder to match the encoder.
    /// - [`SubmitSm::short_message`] will be overridden by `short_message` of the multipart builder.
    fn multipart<'a>(self, short_message: &'a str)
    -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked>;
}

impl SubmitSmMultipartExt for SubmitSm {
    fn multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked> {
        SubmitSmMultipartBuilder::new(short_message, self, Gsm7BitUnpacked::new())
    }
}
