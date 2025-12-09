use rusmpp_macros::Rusmpp;

/// This field indicates the current status of the broadcast message.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[repr(u8)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MessageState {
    /// The message is scheduled. Delivery has not
    /// yet been initiated.
    ///
    /// A message submitted with a scheduled
    /// delivery time may return this state when
    /// queried. This value was added for V5.0 of
    /// `SMPP` and V3.4 and earlier MCs are likely to
    /// return ENROUTE for scheduled messages.
    #[default]
    Scheduled = 0,
    /// The message is in enroute state.
    ///
    /// This is a general state used to describe a
    /// message as being active within the MC. The
    /// message may be in retry or dispatched to a
    /// mobile network for delivery to the mobile.
    Enroute = 1,
    /// Message is delivered to destination
    ///
    /// The message has been delivered to the
    /// destination. No further deliveries will occur.
    Delivered = 2,
    /// Message validity period has expired.
    ///
    /// The message has failed to be delivered within
    /// its validity period and/or retry period. No
    /// further delivery attempts will be made.
    Expired = 3,
    /// Message has been deleted.
    ///
    /// The message has been cancelled or deleted
    /// from the MC. No further delivery attempts will
    /// take place.
    Deleted = 4,
    /// Message is undeliverable.
    ///
    /// The message has encountered a delivery error
    /// and is deemed permanently undeliverable. No
    /// further delivery attempts will be made.
    ///
    /// Certain network or MC internal errors result in
    /// the permanent non-delivery of a message.
    /// Examples of such errors would be an unknown
    /// subscriber or network error that indicated that
    /// the given destination mobile was denied SMS
    /// service or could not support SMS.
    Undeliverable = 5,
    /// Message is in accepted state (i.e. has been
    /// manually read on behalf of the subscriber by
    /// customer service)
    ///
    /// This state is used to depict intervention on the
    /// MC side. Sometimes a malformed message
    /// can cause a mobile to power-off or experience
    /// problems. The result is that all messages to
    /// that mobile may remain queued until the
    /// problem message is removed or expires.
    ///
    /// In certain circumstances, a mobile network
    /// support service or administrator may manually
    /// accept a message to prevent further deliveries
    /// and allow other queued messages to be
    /// delivered.
    Accepted = 6,
    /// Message is in invalid state.
    ///
    /// The message state is unknown. This may be
    /// due to some internal MC problem which may
    /// be intermediate or a permanent.
    /// This state should never be returned. A MC
    /// experiencing difficulties that prevents it from
    /// returning a message state, would use this
    /// state.
    Unknown = 7,
    /// Message is in a rejected state.
    ///
    /// The message has been rejected by a delivery
    /// interface. The reasons for this rejection are
    /// vendor and network specific. No further
    /// delivery attempts will be made
    Rejected = 8,
    /// The message was accepted but not.
    ///
    /// transmitted or broadcast on the network.
    /// A skipped message is one that was
    /// deliberately ignored according to vendor or
    /// network-specific rules. No further delivery
    /// attempts will be made.
    Skipped = 9,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MessageState>();
        crate::tests::borrowed::encode_decode_test_instances::<MessageState>();
    }
}
