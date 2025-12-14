macro_rules! owned_short_message {
    ($name:ident, $builder_name:ident, $tlv:ident) => {
        impl $name {
            /// Sets the `short_message` and `sm_length`.
            ///
            /// # Note
            ///
            /// `short_message` is superceded by [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) and should only be used if
            /// [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) is not present.
            pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) {
                self.short_message = short_message;
                self.sm_length = self.short_message.length() as u8;
            }

            /// Attempts to set the `short_message` and `sm_length` and returns `true` if successful.
            ///
            /// See [`Self::set_short_message`] for details.
            pub fn try_set_short_message(&mut self, short_message: OctetString<0, 255>) -> bool {
                if !self.message_payload_exists() {
                    self.set_short_message(short_message);

                    return true;
                }

                false
            }

            /// Clears the `short_message` and sets the `sm_length` to `0`.
            pub fn clear_short_message(&mut self) {
                self.short_message = OctetString::empty();
                self.sm_length = 0;
            }

            /// Attempts to set the TLVs and returns `true` if successful.
            ///
            /// # Note
            ///
            /// If one of the TLVs is [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload), it will not be set
            /// if the `short_message` is set, as [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) supersedes the `short_message`.
            pub fn try_set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<$tlv>>) -> bool {
                let tlvs: alloc::vec::Vec<Tlv> =
                    tlvs.into_iter().map(Into::into).map(From::from).collect();

                if tlvs
                    .iter()
                    .any(|value| matches!(value.tag(), TlvTag::MessagePayload))
                {
                    if !self.short_message_exists() {
                        self.tlvs = tlvs;

                        return true;
                    }

                    return false;
                }

                self.tlvs = tlvs;

                true
            }

            #[doc = concat!("Attempts to push a [`", stringify!($tlv), "`] and returns `true` if successful.")]
            ///
            /// If the TLV is [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload), it will not be pushed
            /// if the `short_message` is set, as [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) supersedes the `short_message`.
            pub fn try_push_tlv(&mut self, tlv: impl Into<$tlv>) -> bool {
                let tlv = tlv.into();

                if matches!(tlv, $tlv::MessagePayload(_)) {
                    if !self.short_message_exists() {
                        self.tlvs.push(Tlv::from(tlv));

                        return true;
                    }

                    return false;
                }

                self.tlvs.push(Tlv::from(tlv));

                true
            }

            /// Checks if [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) exists.
            pub fn message_payload_exists(&self) -> bool {
                self.tlvs
                    .iter()
                    .any(|value| matches!(value.tag(), TlvTag::MessagePayload))
            }

            /// Checks if the `short_message` is set.
            pub fn short_message_exists(&self) -> bool {
                !self.short_message.is_empty()
            }
        }

        impl $builder_name {
            pub fn try_short_message(mut self, short_message: OctetString<0, 255>) -> Option<Self> {
                if self.inner.try_set_short_message(short_message) {
                    Some(self)
                } else {
                    None
                }
            }

            pub fn try_tlvs(mut self, tlvs: alloc::vec::Vec<impl Into<$tlv>>) -> Option<Self> {
                if self.inner.try_set_tlvs(tlvs) {
                    Some(self)
                } else {
                    None
                }
            }

            pub fn try_push_tlv(mut self, tlv: impl Into<$tlv>) -> Option<Self> {
                if self.inner.try_push_tlv(tlv) {
                    Some(self)
                } else {
                    None
                }
            }
        }
    };
}

pub(crate) use owned_short_message;

// TODO: the borrowed version
