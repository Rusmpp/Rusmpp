use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum TypeOfNetwork {
    #[default]
    Generic = 0,
    Gsm = 1,
    Tdma = 2,
    Cdma = 3,
    Other(u8),
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum EncodingContentType {
    #[default]
    Index = 0x0000,
    EmergencyBroadcasts = 0x0001,
    IrdbDownload = 0x0002,
    NewsFlashes = 0x0003,
    GeneralNewsLocal = 0x0011,
    GeneralNewsRegional = 0x0012,
    GeneralNewsNational = 0x0013,
    GeneralNewsInternational = 0x0014,
    BusinessFinancialNewsLocal = 0x0015,
    BusinessFinancialNewsRegional = 0x0016,
    BusinessFinancialNewsNational = 0x0017,
    BusinessFinancialNewsInternational = 0x0018,
    SportsNewsLocal = 0x0019,
    SportsNewsRegional = 0x001A,
    SportsNewsNational = 0x001B,
    SportsNewsInternational = 0x001C,
    EntertainmentNewsLocal = 0x001D,
    EntertainmentNewsRegional = 0x001E,
    EntertainmentNewsNational = 0x001F,
    EntertainmentNewsInternational = 0x0020,
    MedicalHealthHospitals = 0x0021,
    Doctors = 0x0022,
    Pharmacy = 0x0023,
    LocalTrafficRoadReports = 0x0030,
    LongDistanceTrafficRoadReports = 0x0031,
    Taxis = 0x0032,
    Weather = 0x0033,
    LocalAirportFlightSchedules = 0x0034,
    Restaurants = 0x0035,
    Lodgings = 0x0036,
    RetailDirectory = 0x0037,
    Advertisements = 0x0038,
    StockQuotes = 0x0039,
    EmploymentOpportunities = 0x0040,
    TechnologyNews = 0x0041,
    DistrictBaseStationInfo = 0x0070,
    NetworkInformation = 0x0071,
    OperatorServices = 0x0080,
    DirectoryEnquiriesNational = 0x0081,
    DirectoryEnquiriesInternational = 0x0082,
    CustomerCareNational = 0x0083,
    CustomerCareInternational = 0x0084,
    LocalDateTimeTimeZone = 0x0085,
    MultiCategoryServices = 0x0100,
    Other(u16),
}

/// Specifies the content type of the message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct BroadcastContentType {
    pub type_of_network: TypeOfNetwork,
    pub encoding_content_type: EncodingContentType,
}

impl BroadcastContentType {
    pub fn new(type_of_network: TypeOfNetwork, encoding_content_type: EncodingContentType) -> Self {
        Self {
            type_of_network,
            encoding_content_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<TypeOfNetwork>();
        crate::tests::borrowed::encode_decode_test_instances::<TypeOfNetwork>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<EncodingContentType>();
        crate::tests::borrowed::encode_decode_test_instances::<EncodingContentType>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BroadcastContentType>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastContentType>();
    }
}
