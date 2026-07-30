use rusmpp_macros::Rusmpp;

use crate::{
    CommandStatus,
    types::borrowed::COctetString,
    values::{npi::Npi, ton::Ton},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct UnsuccessSme<'a> {
    /// Type of number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination Address of SME.
    pub destination_addr: COctetString<'a, 1, 21>,
    /// Indicates the success or failure of the [`SubmitMulti`](type@crate::pdus::borrowed::SubmitMulti) request to this SME address.
    pub error_status_code: CommandStatus,
}

impl Default for UnsuccessSme<'_> {
    fn default() -> Self {
        Self {
            dest_addr_ton: Ton::default(),
            dest_addr_npi: Npi::default(),
            destination_addr: COctetString::default(),
            error_status_code: CommandStatus::EsmeRunknownerr,
        }
    }
}

impl<'a> UnsuccessSme<'a> {
    pub const fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<'a, 1, 21>,
        error_status_code: CommandStatus,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            error_status_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<UnsuccessSme>();
    }
}
