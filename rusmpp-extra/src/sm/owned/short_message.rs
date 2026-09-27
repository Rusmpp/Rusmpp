use alloc::vec;
use alloc::vec::Vec;
use rusmpp_core::udhs::{
    owned::{Udh, UdhElement, UdhValue},
    values::NationalLanguageIndicator,
};

use crate::{
    concatenation::{MultipartError, owned::Concatenator},
    sm::owned::Sm,
};

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

    fn _push_udh_element(&mut self, element: UdhElement) {
        if let Some(ref mut udh) = self.udh {
            udh.push_element(element);
        } else {
            self.udh = Some(Udh::new(vec![element]));
        }
    }

    pub fn push_udh_element(mut self, element: UdhElement) -> Self {
        self._push_udh_element(element);
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
    ///
    /// This method must be called once before consuming the builder.
    ///
    /// Successive calls to this method will add duplicate UDH elements.
    fn add_udh_alphabet_indicators(&mut self)
    where
        E: Concatenator,
    {
        if let Some(lang) = self.encoder.national_language_locking_shift() {
            self._push_udh_element(UdhElement::new(UdhValue::NationalLanguageLockingShift(
                lang,
            )));
        }

        if let Some(lang) = self.encoder.national_language_single_shift() {
            self._push_udh_element(UdhElement::new(UdhValue::NationalLanguageSingleShift(lang)));
        }
    }
}

enum UdhReferenceNumber {
    EightBit(u8),
    SixteenBit(u16),
}

pub struct ShortMessageUdhMultipartBuilder<S, E> {
    builder: ShortMessageBuilder<S, E>,
    reference: UdhReferenceNumber,
}

impl<S: Sm, E> ShortMessageUdhMultipartBuilder<S, E> {
    pub fn build(
        mut self,
        short_message: &str,
    ) -> Result<Vec<S>, MultipartError<<E as Concatenator>::Error>>
    where
        E: Concatenator,
    {
        self.builder.add_udh_alphabet_indicators();

        todo!()
    }
}

pub struct ShortMessageSarMultipartBuilder<S, E> {
    builder: ShortMessageBuilder<S, E>,
    reference: u16,
}

impl<S: Sm, E> ShortMessageSarMultipartBuilder<S, E> {
    pub fn build(
        mut self,
        short_message: &str,
    ) -> Result<Vec<S>, MultipartError<<E as Concatenator>::Error>>
    where
        E: Concatenator,
    {
        self.builder.add_udh_alphabet_indicators();

        todo!()
    }
}
