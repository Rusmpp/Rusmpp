use rusmpp_macros::Rusmpp;

/// Indicates Message Mode and Message Type.
///
/// The esm_class parameter is used to indicate special message attributes associated with the
/// short message.
///
/// The esm_class parameter is encoded as follows in the submit_sm, submit_multi and
/// data_sm (ESME -> MC) PDUs
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(repr = "u8", test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct EsmClass {
    /// Messaging Mode (bits 1-0).
    pub messaging_mode: MessagingMode,
    /// Message Type (bits 2 and 5).
    pub message_type: MessageType,
    /// ANSI-41 Specific (bits 5-2).
    pub ansi41_specific: Ansi41Specific,
    /// GSM Specific (bits 7-6).
    pub gsm_features: GsmFeatures,
}

impl EsmClass {
    pub fn new(
        messaging_mode: MessagingMode,
        message_type: MessageType,
        ansi41_specific: Ansi41Specific,
        gsm_features: GsmFeatures,
    ) -> Self {
        Self {
            messaging_mode,
            message_type,
            ansi41_specific,
            gsm_features,
        }
    }

    /// Sets the UDH Indicator bit in the GSM Features field.
    pub const fn with_udhi_indicator(self) -> Self {
        Self {
            messaging_mode: self.messaging_mode,
            message_type: self.message_type,
            ansi41_specific: self.ansi41_specific,
            gsm_features: GsmFeatures::UdhiIndicator,
        }
    }
}

impl From<u8> for EsmClass {
    fn from(value: u8) -> Self {
        Self {
            messaging_mode: MessagingMode::from(value & 0b00_00_00_11),
            message_type: MessageType::from(value & 0b00_10_01_00),
            ansi41_specific: Ansi41Specific::from(value & 0b00_01_10_00),
            gsm_features: GsmFeatures::from(value & 0b11_00_00_00),
        }
    }
}

impl From<EsmClass> for u8 {
    fn from(value: EsmClass) -> Self {
        u8::from(value.messaging_mode)
            | u8::from(value.message_type)
            | u8::from(value.ansi41_specific)
            | u8::from(value.gsm_features)
    }
}

/// Messaging Mode (bits 1-0).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MessagingMode {
    /// Default MC Mode (e.g. Store and Forward).
    #[default]
    Default = 0b00_00_00_00,
    /// Datagram mode.
    Datagram = 0b00_00_00_01,
    /// Forward (i.e. Transaction) mode.
    Forward = 0b00_00_00_10,
    /// Store and Forward mode
    ///
    /// (use to select Store and Forward mode if Default MC
    /// Mode is non Store and Forward).
    StoreAndForward = 0b00_00_00_11,
    Other(u8),
}

/// Message Type (bits 2 and 5).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MessageType {
    /// Default message Type (i.e. normal message).
    #[default]
    Default = 0b00_00_00_00,
    /// Short Message contains MC Delivery Receipt.
    ShortMessageContainsMCDeliveryReceipt = 0b00_00_01_00,
    /// Short Message contains Intermediate Delivery Notification.
    ShortMessageContainsIntermediateDeliveryNotification = 0b00_10_00_00,
    Other(u8),
}

/// ANSI-41 Specific (bits 5-2).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum Ansi41Specific {
    /// Short Message contains Delivery Acknowledgement.
    #[default]
    ShortMessageContainsDeliveryAcknowledgement = 0b00_00_10_00,
    /// Short Message contains Manual/User Acknowledgement.
    ShortMessageContainsUserAcknowledgment = 0b00_01_00_00,
    /// Short Message contains Conversation Abort (Korean CDMA).
    ShortMessageContainsConversationAbort = 0b00_01_10_00,
    Other(u8),
}

/// GSM Specific (bits 7-6).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum GsmFeatures {
    /// No specific features selected.
    #[default]
    NotSelected = 0b00_00_00_00,
    /// UDH Indicator.
    UdhiIndicator = 0b01_00_00_00,
    /// Set Reply Path (only relevant for GSM network).
    SetReplyPath = 0b10_00_00_00,
    /// Set UDHI and Reply Path (only relevant for GSM network).
    SetUdhiAndReplyPath = 0b11_00_00_00,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for EsmClass {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::new(
                    MessagingMode::Forward,
                    MessageType::ShortMessageContainsIntermediateDeliveryNotification,
                    Ansi41Specific::ShortMessageContainsConversationAbort,
                    GsmFeatures::UdhiIndicator,
                ),
                Self::new(
                    MessagingMode::Datagram,
                    MessageType::ShortMessageContainsMCDeliveryReceipt,
                    Ansi41Specific::ShortMessageContainsUserAcknowledgment,
                    GsmFeatures::UdhiIndicator,
                ),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<EsmClass>();
        crate::tests::borrowed::encode_decode_test_instances::<EsmClass>();

        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MessagingMode>();
        crate::tests::borrowed::encode_decode_test_instances::<MessagingMode>();

        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MessageType>();
        crate::tests::borrowed::encode_decode_test_instances::<MessageType>();

        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Ansi41Specific>();
        crate::tests::borrowed::encode_decode_test_instances::<GsmFeatures>();
    }
}
