use rusmpp_core::values::DataCoding;

use crate::encoding::{IdentityMapChar, gsm7bit::alphabet::Gsm7BitAlphabet};

/// GSM 7-bit packed codec.
#[non_exhaustive]
#[derive(Debug)]
pub struct Gsm7BitPacked<C = IdentityMapChar> {
    /// The GSM 7-bit alphabet to use for encoding.
    alphabet: Gsm7BitAlphabet,
    /// Whether to allow splitting extended characters across message parts.
    allow_split_extended_character: bool,
    /// Whether to apply the CR padding fix when packing.
    ///
    /// [GSM 03.38](https://en.wikipedia.org/wiki/GSM_03.38#GSM_7-bit_default_alphabet_and_extension_table_of_3GPP_TS_23.038_/_GSM_03.38):
    ///
    /// if packing would leave exactly 7
    /// spare bits in the final octet, a virtual `CR` septet is appended
    /// before packing so those bits carry `0x0D` instead of `0`.
    ///
    /// Since the spare space is exactly one septet wide, this never changes the resulting octet count.
    cr_padding: bool,
    /// The character mapping for the codec.
    map: C,
}

impl Default for Gsm7BitPacked {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7BitPacked {
    /// Creates a new [`Gsm7BitPacked`] with [`Gsm7BitAlphabet::Default`].
    ///
    /// # Defaults
    ///
    /// - `alphabet`: [`Gsm7BitAlphabet::Default`]
    /// - `allow_split_extended_character`: `false`
    /// - `cr_padding`: `true`
    pub const fn new() -> Self {
        Self {
            alphabet: Gsm7BitAlphabet::default(),
            allow_split_extended_character: false,
            cr_padding: true,
            map: IdentityMapChar,
        }
    }
}

impl<C> Gsm7BitPacked<C> {
    /// Sets the alphabet for the codec.
    pub const fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
        self.alphabet = alphabet;
        self
    }

    /// Returns whether splitting extended characters is allowed.
    pub const fn allow_split_extended_character(&self) -> bool {
        self.allow_split_extended_character
    }

    /// Sets whether to allow splitting extended characters across message parts.
    pub const fn with_allow_split_extended_character(mut self, allow: bool) -> Self {
        self.allow_split_extended_character = allow;
        self
    }

    /// Returns the associated [`Gsm7BitAlphabet`].
    pub const fn alphabet(&self) -> &Gsm7BitAlphabet {
        &self.alphabet
    }

    /// Returns the associated [`DataCoding`].
    pub const fn data_coding(&self) -> DataCoding {
        DataCoding::McSpecific
    }

    /// Returns whether the CR padding fix is applied when packing.
    pub const fn cr_padding(&self) -> bool {
        self.cr_padding
    }

    /// Sets whether to apply the CR padding fix when packing.
    pub const fn with_cr_padding(mut self, cr_padding: bool) -> Self {
        self.cr_padding = cr_padding;
        self
    }

    /// Sets the character mapping for the codec.
    pub fn with_map<U>(self, map: U) -> Gsm7BitPacked<U> {
        Gsm7BitPacked {
            alphabet: self.alphabet,
            allow_split_extended_character: self.allow_split_extended_character,
            cr_padding: self.cr_padding,
            map,
        }
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use alloc::vec::Vec;

    use crate::{
        concatenation::{
            MAX_PARTS,
            owned::{Concatenation, Concatenator},
        },
        encoding::{
            MapChar,
            gsm7bit::{
                alphabet::ESCAPE_CHARACTER,
                errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError},
            },
            owned::Encoder,
        },
    };

    use super::*;

    /// The 7-bit code for the Carriage Return (CR) character.
    const CR_FILL_SEPTET: u8 = 0x0D;

    impl<C: MapChar> Gsm7BitPacked<C> {
        /// Encodes the given message into a vector of bytes and packs the septets into octets.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, Gsm7BitEncodeError> {
            let encoded = self.encode_unpacked_to_vec(input)?;

            let padding = Self::padding(0);

            Ok(self.pack_with_cr_padding(&encoded, padding))
        }

        /// Encodes the given message into a vector of bytes.
        ///
        /// # Note
        ///
        /// This function does not pack the septets into octets.
        fn encode_unpacked_to_vec(&self, input: &str) -> Result<Vec<u8>, Gsm7BitEncodeError> {
            self.alphabet
                .encode_to_vec(input, &self.map)
                .map_err(Gsm7BitEncodeError::UnencodableCharacter)
        }

        /// Returns the number of padding bits needed to align the first septet after a header of `header_size` octets.
        const fn padding(header_size: usize) -> usize {
            (7 - ((header_size * 8) % 7)) % 7
        }

        /// Number of octets needed to pack `n_septets` septets, given `padding`
        /// leading bits used to realign after a header whose size (in octets) isn't a multiple of 7.
        const fn packed_octets(n_septets: usize, padding: usize) -> usize {
            if n_septets == 0 {
                0
            } else {
                (padding + n_septets * 7).div_ceil(8)
            }
        }

        /// Number of unused bits left in the last octet once `n_septets` septets are packed with `padding` leading bits.
        const fn spare_bits(n_septets: usize, padding: usize) -> usize {
            if n_septets == 0 {
                return 0;
            }

            let total_bits = padding + n_septets * 7;

            Self::packed_octets(n_septets, padding) * 8 - total_bits
        }

        // XXX: Do not expose
        pub(crate) fn pack(&self, encoded: &[u8], padding: usize) -> Vec<u8> {
            let mut packed = Vec::new();

            let mut chars_cur = 7;

            if padding > 0 && !encoded.is_empty() {
                chars_cur = padding;

                let cur = encoded[0] << padding;

                packed.push(cur);

                chars_cur -= 1;
            }

            for (i, data) in encoded.iter().enumerate() {
                if chars_cur == 0 {
                    chars_cur = 7;

                    continue;
                }

                let mut cur = (*data & 0b01111111) >> (7 - chars_cur);

                let next = if let Some(n) = encoded.get(i + 1) {
                    *n << chars_cur
                } else {
                    0
                };

                cur |= next;

                packed.push(cur);

                chars_cur -= 1;
            }

            packed
        }

        /// Packs `encoded` with `pack`, applying the CR spare-bit fix if [`Self::cr_padding`] is true.
        ///
        /// See [`Self::cr_padding`] for details.
        fn pack_with_cr_padding(&self, encoded: &[u8], padding: usize) -> Vec<u8> {
            if self.cr_padding && Self::spare_bits(encoded.len(), padding) == 7 {
                let mut with_cr = Vec::with_capacity(encoded.len() + 1);

                with_cr.extend_from_slice(encoded);
                with_cr.push(CR_FILL_SEPTET);

                self.pack(&with_cr, padding)
            } else {
                self.pack(encoded, padding)
            }
        }

        // XXX: Do not expose
        /// Unpacks `packed` octets back into septets, reversing [`Self::pack`].
        ///
        /// `padding` must be the same value passed to [`Self::pack`] when the data
        /// was packed. `n_septets` is the number of septets to extract.
        ///
        /// # Note
        ///
        /// Unlike packing, the septet count can't be reliably derived from the
        /// packed octet count alone: depending on `padding`, up to 7 bits of the
        /// final octet may be spare padding rather than the start of another
        /// septet (see [`Self::spare_bits`]). Callers must track and pass the
        /// original septet count themselves.
        ///
        /// If `packed` runs out of bits before `n_septets` septets have been
        /// extracted, the result is truncated to however many complete septets
        /// were actually available.
        #[cfg(test)]
        pub(crate) fn unpack(&self, packed: &[u8], padding: usize, n_septets: usize) -> Vec<u8> {
            let mut septets = Vec::with_capacity(n_septets);

            let mut bit_pos = padding;

            for _ in 0..n_septets {
                let mut septet: u8 = 0;

                for bit in 0..7 {
                    let byte_idx = bit_pos / 8;
                    let bit_idx = bit_pos % 8;

                    let Some(&byte) = packed.get(byte_idx) else {
                        return septets;
                    };

                    septet |= ((byte >> bit_idx) & 1) << bit;

                    bit_pos += 1;
                }

                septets.push(septet);
            }

            septets
        }
    }

    impl<C: MapChar> Encoder for Gsm7BitPacked<C> {
        type Error = Gsm7BitEncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl<C: MapChar> Concatenator for Gsm7BitPacked<C> {
        type Error = Gsm7BitConcatenateError;

        fn concatenate(
            &self,
            message: &str,
            max_message_size: usize,
            part_header_size: usize,
        ) -> Result<(Concatenation, DataCoding), Self::Error> {
            let encoded = self.encode_unpacked_to_vec(message)?;

            let total = encoded.len();

            // Try a single part first (no concatenation UDH at all).
            let single_padding = Self::padding(0);

            if Self::packed_octets(total, single_padding) <= max_message_size {
                let packed = self.pack_with_cr_padding(&encoded, single_padding);

                return Ok((Concatenation::single(packed), self.data_coding()));
            }

            // Otherwise split across multiple parts, each prefixed by a
            // `part_header_size`-octet UDH. `padding` realigns the first
            // septet of each part's payload to the next octet boundary.
            let padding = Self::padding(part_header_size);

            let payload_octets = max_message_size.saturating_sub(part_header_size);

            if payload_octets == 0 {
                return Err(Gsm7BitConcatenateError::PartCapacityExceeded);
            }

            let available_bits = payload_octets * 8;

            if available_bits <= padding {
                return Err(Gsm7BitConcatenateError::PartCapacityExceeded);
            }

            // Septets that fit per part, leaving room for the header + padding.
            let part_payload_size = (available_bits - padding) / 7;

            if part_payload_size == 0 {
                return Err(Gsm7BitConcatenateError::PartCapacityExceeded);
            }

            let mut parts: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;

            while i < total {
                let mut end = (i + part_payload_size).min(total);

                // avoid splitting extended characters unless allow_split_extended_character == true
                if !self.allow_split_extended_character {
                    // If not last part AND the last byte of this part is 0x1B,
                    // we must shrink the part to avoid splitting ESC + next byte.
                    if end < total && encoded[end - 1] == ESCAPE_CHARACTER {
                        end -= 1;

                        // If shrinking removed the entire part -> impossible
                        if end == i {
                            return Err(Gsm7BitConcatenateError::InvalidBoundary);
                        }
                    }
                }

                let packed = self.pack_with_cr_padding(&encoded[i..end], padding);

                parts.push(packed);

                i = end;
            }

            if parts.len() > MAX_PARTS {
                return Err(Gsm7BitConcatenateError::parts_count_exceeded(parts.len()));
            }

            Ok((Concatenation::concatenated(parts), self.data_coding()))
        }
    }
}
