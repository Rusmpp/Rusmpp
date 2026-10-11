use alloc::vec;
use alloc::vec::Vec;
use rusmpp_core::udhs::owned::{Udh, UdhElement, UdhValue};

use crate::{
    concatenation::{MultipartError, owned::Concatenator},
    encoding::owned::Encoder,
    sm::owned::Sm,
};

#[derive(Debug, Clone)]
pub struct ShortMessageBuilder<S, E> {
    max_short_message_size: usize,
    sm: S,
    encoder: E,
    udh: Option<Udh>,
}

impl<S, E> ShortMessageBuilder<S, E> {
    pub fn max_short_message_size(mut self, size: usize) -> Self {
        self.max_short_message_size = size;
        self
    }

    pub fn udh(mut self, udh: Udh) -> Self {
        self.udh = Some(udh);
        self
    }

    pub fn push_udh_element(mut self, element: UdhElement) -> Self {
        match self.udh {
            Some(ref mut udh) => udh.push_element(element),
            None => self.udh = Some(Udh::new(vec![element])),
        }

        self
    }

    pub fn udh_multipart(self) -> ShortMessageUdhMultipartBuilder<S, E> {
        ShortMessageUdhMultipartBuilder {
            builder: self,
            reference: UdhReferenceNumber::EightBit(0),
        }
    }

    pub fn sar_multipart(self) -> ShortMessageSarMultipartBuilder<S, E> {
        ShortMessageSarMultipartBuilder {
            builder: self,
            reference: 0,
        }
    }
}

impl<S, E> ShortMessageBuilder<S, E> {
    /// Adds UDH elements for national language locking shift and single shift indicators if they exist in the encoder.
    fn add_udh_alphabet_indicators(&self) -> Option<Udh>
    where
        E: Encoder,
    {
        let mut udh = self.udh.clone();

        if let Some(element) = self
            .encoder
            .national_language_locking_shift()
            .map(UdhValue::NationalLanguageLockingShift)
            .map(UdhElement::new)
        {
            match udh {
                Some(ref mut udh) => udh.push_element(element),
                None => udh = Some(Udh::new(vec![element])),
            }
        }

        if let Some(element) = self
            .encoder
            .national_language_single_shift()
            .map(UdhValue::NationalLanguageSingleShift)
            .map(UdhElement::new)
        {
            match udh {
                Some(ref mut udh) => udh.push_element(element),
                None => udh = Some(Udh::new(vec![element])),
            }
        }

        udh
    }
}

#[derive(Debug, Clone)]
enum UdhReferenceNumber {
    EightBit(u8),
    SixteenBit(u16),
}

#[derive(Debug, Clone)]
pub struct ShortMessageUdhMultipartBuilder<S, E> {
    builder: ShortMessageBuilder<S, E>,
    reference: UdhReferenceNumber,
}

impl<S: Sm, E> ShortMessageUdhMultipartBuilder<S, E> {
    pub fn build(
        &self,
        short_message: &str,
    ) -> Result<Vec<S>, MultipartError<<E as Concatenator>::Error>>
    where
        E: Concatenator,
    {
        let data_coding = self.builder.encoder.data_coding();
        let udh = self.builder.add_udh_alphabet_indicators();

        // now we have to calculate the udh length for a part if it happens to be multipart.
        // if the udh is already set, we can add the length of our concatenation udh value.
        //      (we can calculate the length of the concatenation udh and add it to the udh length)
        // if the udh is not set, we have to add a new udh with the concatenation udh value.
        //      (we can not simply calculate the length of the concatenation udh value and add it, because the udh itself has a length field of one byte)

        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ShortMessageSarMultipartBuilder<S, E> {
    builder: ShortMessageBuilder<S, E>,
    reference: u16,
}

impl<S: Sm, E> ShortMessageSarMultipartBuilder<S, E> {
    pub fn build(
        &self,
        short_message: &str,
    ) -> Result<Vec<S>, MultipartError<<E as Concatenator>::Error>>
    where
        E: Concatenator,
    {
        let data_coding = self.builder.encoder.data_coding();
        let udh = self.builder.add_udh_alphabet_indicators();

        // udh length calculation is straight forward here, udh.len() = 0 or length

        todo!()
    }
}
