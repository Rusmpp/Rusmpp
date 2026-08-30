use rusmpp_macros::Rusmpp;

use crate::{
    Sealed,
    decode::{
        AnyOctetStringDecodeError, ConcatenatedShortMessageDecodeError, DecodeResultExt,
        IntegerDecodeError,
        owned::{Decode, DecodeErrorType, DecodeWithKey, DecodeWithLength},
    },
    encode::Length,
    types::owned::AnyOctetString,
    udhs::{
        UdhId,
        concatenation::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit},
        language::NationalLanguageIndicator,
    },
};

/// User Data Header (UDH).
///
/// The UDH can contain multiple [`UdhElement`]s.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
pub struct Udh {
    /// UDH length (excluding the length field itself).
    length: u8,
    /// UDH elements.
    #[rusmpp(length = length)]
    elements: alloc::vec::Vec<UdhElement>,
}

impl Udh {
    /// Creates a new [`Udh`] from the given vector of [`UdhElement`]s.
    pub fn new(elements: alloc::vec::Vec<UdhElement>) -> Self {
        Self {
            length: elements.length() as u8,
            elements,
        }
    }

    /// Returns the elements of the [`Udh`].
    pub fn elements(&self) -> &[UdhElement] {
        &self.elements
    }

    /// Sets the elements of the [`Udh`].
    pub fn set_elements(&mut self, elements: alloc::vec::Vec<UdhElement>) {
        self.elements = elements;
        self.length = self.elements.length() as u8;
    }

    /// Clears the elements of the [`Udh`].
    pub fn clear_elements(&mut self) {
        self.elements.clear();
        self.length = 0;
    }

    /// Pushes a new element to the [`Udh`].
    pub fn push_element(&mut self, element: impl Into<UdhElement>) {
        self.elements.push(element.into());
        self.length = self.elements.length() as u8;
    }
}

/// User Data Header (UDH) element.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
pub struct UdhElement {
    /// UDH identifier.
    id: UdhId,
    /// The length of the UDH value.
    value_length: u8,
    #[rusmpp(key = id, length = value_length)]
    value: Option<UdhValue>,
}

impl UdhElement {
    /// Creates a new [`UdhElement`] from the given [`UdhValue`].
    pub fn new(value: impl Into<UdhValue>) -> Self {
        let value = value.into();
        let id = value.id();
        let value_length = value.length() as u8;

        Self {
            id,
            value_length,
            value: Some(value),
        }
    }

    /// Returns the [`UdhId`].
    pub const fn id(&self) -> UdhId {
        self.id
    }

    /// Returns the [`UdhValue`] length.
    pub const fn value_length(&self) -> u8 {
        self.value_length
    }

    /// Returns a reference to the UDH value.
    pub const fn value(&self) -> Option<&UdhValue> {
        self.value.as_ref()
    }
}

impl From<UdhValue> for UdhElement {
    fn from(value: UdhValue) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "serde")]
const _: () = {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize)]
    #[serde(transparent)]
    struct SerUdhElement<'a> {
        value: Cow<'a, UdhValue>,
    }

    impl<'a> From<&'a UdhElement> for SerUdhElement<'a> {
        fn from(udh: &'a UdhElement) -> Self {
            let value =
                udh.value
                    .as_ref()
                    .map(Cow::Borrowed)
                    .unwrap_or(Cow::Owned(UdhValue::Other {
                        udh_id: udh.id,
                        value: Default::default(),
                    }));

            Self { value }
        }
    }

    impl Serialize for UdhElement {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            SerUdhElement::from(self).serialize(serializer)
        }
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    struct DeUdhElement {
        value: UdhValue,
    }

    impl From<DeUdhElement> for UdhElement {
        fn from(udh: DeUdhElement) -> Self {
            Self::new(udh.value)
        }
    }

    impl<'de> Deserialize<'de> for UdhElement {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let udh = DeUdhElement::deserialize(deserializer)?;

            Ok(Self::from(udh))
        }
    }
};

/// User Data Header (UDH) value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum UdhValue {
    /// 8-bit Concatenated Short Message.
    ConcatenatedShortMessage8Bit(ConcatenatedShortMessage8Bit),
    /// 16-bit Concatenated Short Message.
    ConcatenatedShortMessage16Bit(ConcatenatedShortMessage16Bit),
    /// National Language Single Shift.
    NationalLanguageSingleShift(NationalLanguageIndicator),
    /// National Language Locking Shift.
    NationalLanguageLockingShift(NationalLanguageIndicator),
    /// Other UDH values.
    Other {
        udh_id: UdhId,
        value: AnyOctetString,
    },
}

impl UdhValue {
    /// Returns the [`UdhId`].
    pub const fn id(&self) -> UdhId {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(_) => UdhId::ConcatenatedShortMessages8Bit,
            UdhValue::ConcatenatedShortMessage16Bit(_) => UdhId::ConcatenatedShortMessages16Bit,
            UdhValue::NationalLanguageSingleShift(_) => UdhId::NationalLanguageSingleShift,
            UdhValue::NationalLanguageLockingShift(_) => UdhId::NationalLanguageLockingShift,
            UdhValue::Other { udh_id, .. } => *udh_id,
        }
    }
}

impl Sealed for UdhValue {}

impl Length for UdhValue {
    fn length(&self) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(value) => value.length(),
            UdhValue::ConcatenatedShortMessage16Bit(value) => value.length(),
            UdhValue::NationalLanguageSingleShift(value) => value.length(),
            UdhValue::NationalLanguageLockingShift(value) => value.length(),
            UdhValue::Other { value, .. } => value.length(),
        }
    }
}

impl crate::encode::Encode for UdhValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(value) => value.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(value) => value.encode(dst),
            UdhValue::NationalLanguageSingleShift(value) => value.encode(dst),
            UdhValue::NationalLanguageLockingShift(value) => value.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl crate::encode::owned::Encode for UdhValue {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(value) => value.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(value) => value.encode(dst),
            UdhValue::NationalLanguageSingleShift(value) => value.encode(dst),
            UdhValue::NationalLanguageLockingShift(value) => value.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UdhValueDecodeError {
    #[error("ConcatenatedShortMessage8Bit decode error: {0}")]
    ConcatenatedShortMessage8Bit(#[source] ConcatenatedShortMessageDecodeError),
    #[error("ConcatenatedShortMessage16Bit decode error: {0}")]
    ConcatenatedShortMessage16Bit(#[source] ConcatenatedShortMessageDecodeError),
    #[error("NationalLanguageSingleShift decode error: {0}")]
    NationalLanguageSingleShift(#[source] IntegerDecodeError),
    #[error("NationalLanguageLockingShift decode error: {0}")]
    NationalLanguageLockingShift(#[source] IntegerDecodeError),
    #[error("Other decode error: {0}")]
    Other(
        #[from]
        #[source]
        AnyOctetStringDecodeError,
    ),
}

impl DecodeErrorType for UdhValue {
    type Error = UdhValueDecodeError;
}

impl DecodeWithKey for UdhValue {
    type Key = UdhId;

    fn decode(
        key: Self::Key,
        src: &mut bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let (value, size) = match key {
            UdhId::ConcatenatedShortMessages8Bit => Decode::decode(src)
                .map_decoded(Self::ConcatenatedShortMessage8Bit)
                .map_err(Self::Error::ConcatenatedShortMessage8Bit)?,
            UdhId::ConcatenatedShortMessages16Bit => Decode::decode(src)
                .map_decoded(Self::ConcatenatedShortMessage16Bit)
                .map_err(Self::Error::ConcatenatedShortMessage16Bit)?,
            UdhId::NationalLanguageSingleShift => Decode::decode(src)
                .map_decoded(Self::NationalLanguageSingleShift)
                .map_err(Self::Error::NationalLanguageSingleShift)?,
            UdhId::NationalLanguageLockingShift => Decode::decode(src)
                .map_decoded(Self::NationalLanguageLockingShift)
                .map_err(Self::Error::NationalLanguageLockingShift)?,
            other => DecodeWithLength::decode(src, length)
                .map_decoded(|value| UdhValue::Other {
                    udh_id: other,
                    value,
                })
                .map_err(Self::Error::Other)?,
        };

        Ok((value, size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod element {
        use super::*;

        mod encode {
            use super::*;

            mod borrowed {
                use super::*;

                #[test]
                fn ok() {
                    use crate::encode::Encode;

                    let udh =
                        UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap());

                    let expected = [
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = [0u8; 24];
                    let size = udh.encode(&mut buf);

                    assert_eq!(size, 6);
                    assert_eq!(&buf[..size], &expected);

                    let udh =
                        UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap());
                    let expected = [
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = [0u8; 24];
                    let size = udh.encode(&mut buf);

                    assert_eq!(size, 5);
                    assert_eq!(&buf[..size], &expected);
                }
            }

            mod owned {
                use super::*;

                #[test]
                fn ok() {
                    use bytes::BytesMut;

                    use crate::encode::{Length, owned::Encode};

                    let udh =
                        UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap());

                    let expected = [
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = BytesMut::with_capacity(Length::length(&udh));

                    udh.encode(&mut buf);

                    let encoded = buf.split_to(Length::length(&udh));

                    assert_eq!(encoded.len(), 6);
                    assert_eq!(&encoded[..], &expected);

                    let udh =
                        UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap());
                    let expected = [
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = BytesMut::with_capacity(Length::length(&udh));

                    udh.encode(&mut buf);

                    let encoded = buf.split_to(Length::length(&udh));

                    assert_eq!(encoded.len(), 5);
                    assert_eq!(&encoded[..], &expected);
                }
            }
        }

        mod decode {
            use bytes::BytesMut;

            use crate::decode::owned::Decode;

            use super::*;

            #[test]
            fn ok() {
                let mut buf = BytesMut::from(
                    &[
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                        0x00, // Extra bytes
                        0x00,
                    ][..],
                );

                let (udh, size) = <UdhElement as Decode>::decode(&mut buf).unwrap();

                assert_eq!(size, 6);
                assert_eq!(
                    udh,
                    UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap())
                );
                assert_eq!(&buf[..], &[0x00, 0x00][..]);

                let mut buf = BytesMut::from(
                    &[
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                        0x00, // Extra bytes
                        0x00,
                    ][..],
                );

                let (udh, size) = <UdhElement as Decode>::decode(&mut buf).unwrap();
                assert_eq!(size, 5);
                assert_eq!(
                    udh,
                    UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap())
                );
                assert_eq!(&buf[..], &[0x00, 0x00][..]);
            }
        }
    }

    mod udh {
        use super::*;

        mod encode {
            use super::*;

            mod borrowed {
                use super::*;

                #[test]
                fn ok() {
                    use crate::encode::Encode;

                    let udh = Udh::new(alloc::vec![
                        UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap()),
                        UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap()),
                    ]);

                    let expected = [
                        0x0B, // UDH length = 11 bytes
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = [0u8; 24];
                    let size = udh.encode(&mut buf);

                    assert_eq!(size, expected.len());
                    assert_eq!(&buf[..size], &expected);
                }
            }

            mod owned {
                use super::*;

                #[test]
                fn ok() {
                    use bytes::BytesMut;

                    use crate::encode::{Length, owned::Encode};

                    let udh = Udh::new(alloc::vec![
                        UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap()),
                        UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap()),
                    ]);

                    let expected = [
                        0x0B, // UDH length = 11 bytes
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                    ];

                    let mut buf = BytesMut::with_capacity(Length::length(&udh));

                    udh.encode(&mut buf);

                    let encoded = buf.split_to(Length::length(&udh));

                    assert_eq!(encoded.len(), expected.len());
                    assert_eq!(&encoded[..], &expected);
                }
            }
        }

        mod decode {
            use super::*;

            #[test]
            fn ok() {
                use bytes::BytesMut;

                use crate::decode::owned::DecodeWithLength;

                let mut buf = BytesMut::from(
                    &[
                        0x0B, // UDH length = 11 bytes
                        0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                        0x04, // IE Data Length = 4 bytes
                        0x12, // Ref high
                        0x34, // Ref low
                        0x03, // Total parts
                        0x01, // Part number
                        0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                        0x03, // IE Data Length = 3 bytes
                        0x12, // Ref
                        0x03, // Total parts
                        0x01, // Part number
                    ][..],
                );

                let (udh, size) = <Udh as DecodeWithLength>::decode(&mut buf, 12).unwrap();

                assert_eq!(size, 12);
                assert_eq!(
                    udh,
                    Udh::new(alloc::vec![
                        UdhElement::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap()),
                        UdhElement::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap()),
                    ])
                );

                let mut buf =
                    BytesMut::from(&[0x08, 0x00, 0x03, 0x4A, 0x03, 0x01, 0x25, 0x01, 0x01][..]);

                let (udh, size) = <Udh as DecodeWithLength>::decode(&mut buf, 9).unwrap();

                assert_eq!(size, 9);
                assert_eq!(
                    udh,
                    Udh::new(alloc::vec![
                        UdhElement::new(
                            ConcatenatedShortMessage8Bit::new(0x4A, 0x03, 0x01).unwrap()
                        ),
                        UdhElement::new(UdhValue::NationalLanguageLockingShift(
                            NationalLanguageIndicator::Turkish
                        )),
                    ])
                );
            }
        }
    }
}
