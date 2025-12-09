use rusmpp::{
    tlvs::{MessageSubmissionRequestTlvValue, TlvTag},
    types::{COctetString, OctetString},
    values::*,
    CommandStatus,
};

use crate::{
    exception::{Exception, ValueExceptionExt},
    generated as g,
};

impl From<g::CommandStatus> for CommandStatus {
    fn from(value: g::CommandStatus) -> Self {
        match value {
            g::CommandStatus::EsmeRok() => Self::EsmeRok,
            g::CommandStatus::EsmeRinvmsglen() => Self::EsmeRinvmsglen,
            g::CommandStatus::EsmeRinvcmdlen() => Self::EsmeRinvcmdlen,
            g::CommandStatus::EsmeRinvcmdid() => Self::EsmeRinvcmdid,
            g::CommandStatus::EsmeRinvbndsts() => Self::EsmeRinvbndsts,
            g::CommandStatus::EsmeRalybnd() => Self::EsmeRalybnd,
            g::CommandStatus::EsmeRinvprtflg() => Self::EsmeRinvprtflg,
            g::CommandStatus::EsmeRinvregdlvflg() => Self::EsmeRinvregdlvflg,
            g::CommandStatus::EsmeRsyserr() => Self::EsmeRsyserr,
            g::CommandStatus::EsmeRinvsrcadr() => Self::EsmeRinvsrcadr,
            g::CommandStatus::EsmeRinvdstadr() => Self::EsmeRinvdstadr,
            g::CommandStatus::EsmeRinvmsgid() => Self::EsmeRinvmsgid,
            g::CommandStatus::EsmeRbindfail() => Self::EsmeRbindfail,
            g::CommandStatus::EsmeRinvpaswd() => Self::EsmeRinvpaswd,
            g::CommandStatus::EsmeRinvsysid() => Self::EsmeRinvsysid,
            g::CommandStatus::EsmeRcancelfail() => Self::EsmeRcancelfail,
            g::CommandStatus::EsmeRreplacefail() => Self::EsmeRreplacefail,
            g::CommandStatus::EsmeRmsgqful() => Self::EsmeRmsgqful,
            g::CommandStatus::EsmeRinvsertyp() => Self::EsmeRinvsertyp,
            g::CommandStatus::EsmeRinvnumdests() => Self::EsmeRinvnumdests,
            g::CommandStatus::EsmeRinvdlname() => Self::EsmeRinvdlname,
            g::CommandStatus::EsmeRinvdestflag() => Self::EsmeRinvdestflag,
            g::CommandStatus::EsmeRinvsubrep() => Self::EsmeRinvsubrep,
            g::CommandStatus::EsmeRinvesmclass() => Self::EsmeRinvesmclass,
            g::CommandStatus::EsmeRcntsubdl() => Self::EsmeRcntsubdl,
            g::CommandStatus::EsmeRsubmitfail() => Self::EsmeRsubmitfail,
            g::CommandStatus::EsmeRinvsrcton() => Self::EsmeRinvsrcton,
            g::CommandStatus::EsmeRinvsrcnpi() => Self::EsmeRinvsrcnpi,
            g::CommandStatus::EsmeRinvdstton() => Self::EsmeRinvdstton,
            g::CommandStatus::EsmeRinvdstnpi() => Self::EsmeRinvdstnpi,
            g::CommandStatus::EsmeRinvsystyp() => Self::EsmeRinvsystyp,
            g::CommandStatus::EsmeRinvrepflag() => Self::EsmeRinvrepflag,
            g::CommandStatus::EsmeRinvnummsgs() => Self::EsmeRinvnummsgs,
            g::CommandStatus::EsmeRthrottled() => Self::EsmeRthrottled,
            g::CommandStatus::EsmeRinvsched() => Self::EsmeRinvsched,
            g::CommandStatus::EsmeRinvexpiry() => Self::EsmeRinvexpiry,
            g::CommandStatus::EsmeRinvdftmsgid() => Self::EsmeRinvdftmsgid,
            g::CommandStatus::EsmeRxTAppn() => Self::EsmeRxTAppn,
            g::CommandStatus::EsmeRxPAppn() => Self::EsmeRxPAppn,
            g::CommandStatus::EsmeRxRAppn() => Self::EsmeRxRAppn,
            g::CommandStatus::EsmeRqueryfail() => Self::EsmeRqueryfail,
            g::CommandStatus::EsmeRinvtlvstream() => Self::EsmeRinvtlvstream,
            g::CommandStatus::EsmeRtlvnotallwd() => Self::EsmeRtlvnotallwd,
            g::CommandStatus::EsmeRinvtlvlen() => Self::EsmeRinvtlvlen,
            g::CommandStatus::EsmeRmissingtlv() => Self::EsmeRmissingtlv,
            g::CommandStatus::EsmeRinvtlvval() => Self::EsmeRinvtlvval,
            g::CommandStatus::EsmeRdeliveryfailure() => Self::EsmeRdeliveryfailure,
            g::CommandStatus::EsmeRunknownerr() => Self::EsmeRunknownerr,
            g::CommandStatus::EsmeRsertypunauth() => Self::EsmeRsertypunauth,
            g::CommandStatus::EsmeRprohibited() => Self::EsmeRprohibited,
            g::CommandStatus::EsmeRsertypunavail() => Self::EsmeRsertypunavail,
            g::CommandStatus::EsmeRsertypdenied() => Self::EsmeRsertypdenied,
            g::CommandStatus::EsmeRinvdcs() => Self::EsmeRinvdcs,
            g::CommandStatus::EsmeRinvsrcaddrsubunit() => Self::EsmeRinvsrcaddrsubunit,
            g::CommandStatus::EsmeRinvdstaddrsubunit() => Self::EsmeRinvdstaddrsubunit,
            g::CommandStatus::EsmeRinvbcastfreqint() => Self::EsmeRinvbcastfreqint,
            g::CommandStatus::EsmeRinvbcastaliasName() => Self::EsmeRinvbcastaliasName,
            g::CommandStatus::EsmeRinvbcastareafmt() => Self::EsmeRinvbcastareafmt,
            g::CommandStatus::EsmeRinvnumbcastAreas() => Self::EsmeRinvnumbcastAreas,
            g::CommandStatus::EsmeRinvbcastcnttype() => Self::EsmeRinvbcastcnttype,
            g::CommandStatus::EsmeRinvbcastmsgclass() => Self::EsmeRinvbcastmsgclass,
            g::CommandStatus::EsmeRbcastfail() => Self::EsmeRbcastfail,
            g::CommandStatus::EsmeRbcastqueryfail() => Self::EsmeRbcastqueryfail,
            g::CommandStatus::EsmeRbcastcancelfail() => Self::EsmeRbcastcancelfail,
            g::CommandStatus::EsmeRinvbcastRep() => Self::EsmeRinvbcastRep,
            g::CommandStatus::EsmeRinvbcastsrvgrp() => Self::EsmeRinvbcastsrvgrp,
            g::CommandStatus::EsmeRinvbcastchanind() => Self::EsmeRinvbcastchanind,
            g::CommandStatus::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::TlvTag> for TlvTag {
    fn from(value: g::TlvTag) -> Self {
        match value {
            g::TlvTag::DestAddrSubunit() => Self::DestAddrSubunit,
            g::TlvTag::DestNetworkType() => Self::DestNetworkType,
            g::TlvTag::DestBearerType() => Self::DestBearerType,
            g::TlvTag::DestTelematicsId() => Self::DestTelematicsId,
            g::TlvTag::SourceAddrSubunit() => Self::SourceAddrSubunit,
            g::TlvTag::SourceNetworkType() => Self::SourceNetworkType,
            g::TlvTag::SourceBearerType() => Self::SourceBearerType,
            g::TlvTag::SourceTelematicsId() => Self::SourceTelematicsId,
            g::TlvTag::QosTimeToLive() => Self::QosTimeToLive,
            g::TlvTag::PayloadType() => Self::PayloadType,
            g::TlvTag::AdditionalStatusInfoText() => Self::AdditionalStatusInfoText,
            g::TlvTag::ReceiptedMessageId() => Self::ReceiptedMessageId,
            g::TlvTag::MsMsgWaitFacilities() => Self::MsMsgWaitFacilities,
            g::TlvTag::PrivacyIndicator() => Self::PrivacyIndicator,
            g::TlvTag::SourceSubaddress() => Self::SourceSubaddress,
            g::TlvTag::DestSubaddress() => Self::DestSubaddress,
            g::TlvTag::UserMessageReference() => Self::UserMessageReference,
            g::TlvTag::UserResponseCode() => Self::UserResponseCode,
            g::TlvTag::SourcePort() => Self::SourcePort,
            g::TlvTag::DestPort() => Self::DestPort,
            g::TlvTag::SarMsgRefNum() => Self::SarMsgRefNum,
            g::TlvTag::LanguageIndicator() => Self::LanguageIndicator,
            g::TlvTag::SarTotalSegments() => Self::SarTotalSegments,
            g::TlvTag::SarSegmentSeqnum() => Self::SarSegmentSeqnum,
            g::TlvTag::ScInterfaceVersion() => Self::ScInterfaceVersion,
            g::TlvTag::CallbackNumPresInd() => Self::CallbackNumPresInd,
            g::TlvTag::CallbackNumAtag() => Self::CallbackNumAtag,
            g::TlvTag::NumberOfMessages() => Self::NumberOfMessages,
            g::TlvTag::CallbackNum() => Self::CallbackNum,
            g::TlvTag::DpfResult() => Self::DpfResult,
            g::TlvTag::SetDpf() => Self::SetDpf,
            g::TlvTag::MsAvailabilityStatus() => Self::MsAvailabilityStatus,
            g::TlvTag::NetworkErrorCode() => Self::NetworkErrorCode,
            g::TlvTag::MessagePayload() => Self::MessagePayload,
            g::TlvTag::DeliveryFailureReason() => Self::DeliveryFailureReason,
            g::TlvTag::MoreMessagesToSend() => Self::MoreMessagesToSend,
            g::TlvTag::MessageState() => Self::MessageState,
            g::TlvTag::CongestionState() => Self::CongestionState,
            g::TlvTag::UssdServiceOp() => Self::UssdServiceOp,
            g::TlvTag::BroadcastChannelIndicator() => Self::BroadcastChannelIndicator,
            g::TlvTag::BroadcastContentType() => Self::BroadcastContentType,
            g::TlvTag::BroadcastContentTypeInfo() => Self::BroadcastContentTypeInfo,
            g::TlvTag::BroadcastMessageClass() => Self::BroadcastMessageClass,
            g::TlvTag::BroadcastRepNum() => Self::BroadcastRepNum,
            g::TlvTag::BroadcastFrequencyInterval() => Self::BroadcastFrequencyInterval,
            g::TlvTag::BroadcastAreaIdentifier() => Self::BroadcastAreaIdentifier,
            g::TlvTag::BroadcastErrorStatus() => Self::BroadcastErrorStatus,
            g::TlvTag::BroadcastAreaSuccess() => Self::BroadcastAreaSuccess,
            g::TlvTag::BroadcastEndTime() => Self::BroadcastEndTime,
            g::TlvTag::BroadcastServiceGroup() => Self::BroadcastServiceGroup,
            g::TlvTag::BillingIdentification() => Self::BillingIdentification,
            g::TlvTag::SourceNetworkId() => Self::SourceNetworkId,
            g::TlvTag::DestNetworkId() => Self::DestNetworkId,
            g::TlvTag::SourceNodeId() => Self::SourceNodeId,
            g::TlvTag::DestNodeId() => Self::DestNodeId,
            g::TlvTag::DestAddrNpResolution() => Self::DestAddrNpResolution,
            g::TlvTag::DestAddrNpInformation() => Self::DestAddrNpInformation,
            g::TlvTag::DestAddrNpCountry() => Self::DestAddrNpCountry,
            g::TlvTag::DisplayTime() => Self::DisplayTime,
            g::TlvTag::SmsSignal() => Self::SmsSignal,
            g::TlvTag::MsValidity() => Self::MsValidity,
            g::TlvTag::AlertOnMessageDelivery() => Self::AlertOnMessageDelivery,
            g::TlvTag::ItsReplyType() => Self::ItsReplyType,
            g::TlvTag::ItsSessionInfo() => Self::ItsSessionInfo,
            g::TlvTag::Other(tag) => Self::Other(tag),
        }
    }
}

impl From<g::InterfaceVersion> for InterfaceVersion {
    fn from(value: g::InterfaceVersion) -> Self {
        match value {
            g::InterfaceVersion::Smpp3_3OrEarlier(value) => Self::Smpp3_3OrEarlier(value),
            g::InterfaceVersion::Smpp3_4() => Self::Smpp3_4,
            g::InterfaceVersion::Smpp5_0() => Self::Smpp5_0,
            g::InterfaceVersion::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Ton> for Ton {
    fn from(value: g::Ton) -> Self {
        match value {
            g::Ton::Unknown() => Self::Unknown,
            g::Ton::International() => Self::International,
            g::Ton::National() => Self::National,
            g::Ton::NetworkSpecific() => Self::NetworkSpecific,
            g::Ton::SubscriberNumber() => Self::SubscriberNumber,
            g::Ton::Alphanumeric() => Self::Alphanumeric,
            g::Ton::Abbreviated() => Self::Abbreviated,
            g::Ton::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Npi> for Npi {
    fn from(value: g::Npi) -> Self {
        match value {
            g::Npi::Unknown() => Self::Unknown,
            g::Npi::Isdn() => Self::Isdn,
            g::Npi::Data() => Self::Data,
            g::Npi::Telex() => Self::Telex,
            g::Npi::LandMobile() => Self::LandMobile,
            g::Npi::National() => Self::National,
            g::Npi::Private() => Self::Private,
            g::Npi::Ermes() => Self::Ermes,
            g::Npi::Internet() => Self::Internet,
            g::Npi::WapClientId() => Self::WapClientId,
            g::Npi::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::DataCoding> for DataCoding {
    fn from(value: g::DataCoding) -> Self {
        match value {
            g::DataCoding::McSpecific() => Self::McSpecific,
            g::DataCoding::Ia5() => Self::Ia5,
            g::DataCoding::OctetUnspecified() => Self::OctetUnspecified,
            g::DataCoding::Latin1() => Self::Latin1,
            g::DataCoding::OctetUnspecified2() => Self::OctetUnspecified2,
            g::DataCoding::Jis() => Self::Jis,
            g::DataCoding::Cyrillic() => Self::Cyrillic,
            g::DataCoding::LatinHebrew() => Self::LatinHebrew,
            g::DataCoding::Ucs2() => Self::Ucs2,
            g::DataCoding::PictogramEncoding() => Self::PictogramEncoding,
            g::DataCoding::Iso2022JpMusicCodes() => Self::Iso2022JpMusicCodes,
            g::DataCoding::ExtendedKanjiJis() => Self::ExtendedKanjiJis,
            g::DataCoding::Ksc5601() => Self::Ksc5601,
            g::DataCoding::GsmMwiControl() => Self::GsmMwiControl,
            g::DataCoding::GsmMwiControl2() => Self::GsmMwiControl2,
            g::DataCoding::GsmMessageClassControl() => Self::GsmMessageClassControl,
            g::DataCoding::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::AlertOnMessageDelivery> for AlertOnMessageDelivery {
    fn from(value: g::AlertOnMessageDelivery) -> Self {
        match value {
            g::AlertOnMessageDelivery::UseMobileDefaultAlert() => Self::UseMobileDefaultAlert,
            g::AlertOnMessageDelivery::UseLowPriorityAlert() => Self::UseLowPriorityAlert,
            g::AlertOnMessageDelivery::UseMediumPriorityAlert() => Self::UseMediumPriorityAlert,
            g::AlertOnMessageDelivery::UseHighPriorityAlert() => Self::UseHighPriorityAlert,
            g::AlertOnMessageDelivery::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Screening> for Screening {
    fn from(value: g::Screening) -> Self {
        match value {
            g::Screening::NotScreened() => Self::NotScreened,
            g::Screening::VerifiedAndPassed() => Self::VerifiedAndPassed,
            g::Screening::VerifiedAndFailed() => Self::VerifiedAndFailed,
            g::Screening::NetworkProvided() => Self::NetworkProvided,
            g::Screening::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Presentation> for Presentation {
    fn from(value: g::Presentation) -> Self {
        match value {
            g::Presentation::PresentationAllowed() => Self::PresentationAllowed,
            g::Presentation::PresentationRestricted() => Self::PresentationRestricted,
            g::Presentation::NumberNotAvailable() => Self::NumberNotAvailable,
            g::Presentation::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::CallbackNumPresInd> for CallbackNumPresInd {
    fn from(value: g::CallbackNumPresInd) -> Self {
        Self {
            presentation: Presentation::from(value.presentation),
            screening: Screening::from(value.screening),
        }
    }
}

impl From<g::DestAddrNpResolution> for DestAddrNpResolution {
    fn from(value: g::DestAddrNpResolution) -> Self {
        match value {
            g::DestAddrNpResolution::QueryNotPerformed() => Self::QueryNotPerformed,
            g::DestAddrNpResolution::QueryPerformedNumberNotPorted() => {
                Self::QueryPerformedNumberNotPorted
            }
            g::DestAddrNpResolution::QueryPerformedNumberPorted() => {
                Self::QueryPerformedNumberPorted
            }
            g::DestAddrNpResolution::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::AddrSubunit> for AddrSubunit {
    fn from(value: g::AddrSubunit) -> Self {
        match value {
            g::AddrSubunit::Unknown() => Self::Unknown,
            g::AddrSubunit::MSDisplay() => Self::MSDisplay,
            g::AddrSubunit::MobileEquipment() => Self::MobileEquipment,
            g::AddrSubunit::SmartCard() => Self::SmartCard,
            g::AddrSubunit::ExternalUnit() => Self::ExternalUnit,
            g::AddrSubunit::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::BearerType> for BearerType {
    fn from(value: g::BearerType) -> Self {
        match value {
            g::BearerType::Unknown() => Self::Unknown,
            g::BearerType::Sms() => Self::Sms,
            g::BearerType::Csd() => Self::Csd,
            g::BearerType::PacketData() => Self::PacketData,
            g::BearerType::Ussd() => Self::Ussd,
            g::BearerType::Cdpd() => Self::Cdpd,
            g::BearerType::DataTac() => Self::DataTac,
            g::BearerType::FlexReFlex() => Self::FlexReFlex,
            g::BearerType::CellBroadcast() => Self::CellBroadcast,
            g::BearerType::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::NetworkType> for NetworkType {
    fn from(value: g::NetworkType) -> Self {
        match value {
            g::NetworkType::Unknown() => Self::Unknown,
            g::NetworkType::Gsm() => Self::Gsm,
            g::NetworkType::Ansi136() => Self::Ansi136,
            g::NetworkType::Is95() => Self::Is95,
            g::NetworkType::Pdc() => Self::Pdc,
            g::NetworkType::Phs() => Self::Phs,
            g::NetworkType::IDen() => Self::IDen,
            g::NetworkType::Amps() => Self::Amps,
            g::NetworkType::PagingNetwork() => Self::PagingNetwork,
            g::NetworkType::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::SubaddressTag> for SubaddressTag {
    fn from(value: g::SubaddressTag) -> Self {
        match value {
            g::SubaddressTag::NsapEven() => Self::NsapEven,
            g::SubaddressTag::NsapOdd() => Self::NsapOdd,
            g::SubaddressTag::UserSpecified() => Self::UserSpecified,
            g::SubaddressTag::Other(value) => Self::Other(value),
        }
    }
}

impl TryFrom<g::Subaddress> for Subaddress {
    type Error = Exception;

    fn try_from(value: g::Subaddress) -> Result<Self, Self::Error> {
        Ok(Self {
            tag: value.tag.into(),
            addr: OctetString::from_bytes(value.addr).map_value_err("addr")?,
        })
    }
}

impl From<g::DisplayTime> for DisplayTime {
    fn from(value: g::DisplayTime) -> Self {
        match value {
            g::DisplayTime::Temporary() => Self::Temporary,
            g::DisplayTime::Default() => Self::Default,
            g::DisplayTime::Invoke() => Self::Invoke,
            g::DisplayTime::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::ItsSessionInfo> for ItsSessionInfo {
    fn from(value: g::ItsSessionInfo) -> Self {
        Self {
            session_number: value.session_number,
            sequence_number: value.sequence_number,
        }
    }
}

impl From<g::LanguageIndicator> for LanguageIndicator {
    fn from(value: g::LanguageIndicator) -> Self {
        match value {
            g::LanguageIndicator::Unspecified() => Self::Unspecified,
            g::LanguageIndicator::English() => Self::English,
            g::LanguageIndicator::French() => Self::French,
            g::LanguageIndicator::Spanish() => Self::Spanish,
            g::LanguageIndicator::German() => Self::German,
            g::LanguageIndicator::Portuguese() => Self::Portuguese,
            g::LanguageIndicator::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::PrivacyIndicator> for PrivacyIndicator {
    fn from(value: g::PrivacyIndicator) -> Self {
        match value {
            g::PrivacyIndicator::NotRestricted() => Self::NotRestricted,
            g::PrivacyIndicator::Restricted() => Self::Restricted,
            g::PrivacyIndicator::Confidential() => Self::Confidential,
            g::PrivacyIndicator::Secret() => Self::Secret,
            g::PrivacyIndicator::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::ItsReplyType> for ItsReplyType {
    fn from(value: g::ItsReplyType) -> Self {
        match value {
            g::ItsReplyType::Digit() => Self::Digit,
            g::ItsReplyType::Number() => Self::Number,
            g::ItsReplyType::TelephoneNo() => Self::TelephoneNo,
            g::ItsReplyType::Password() => Self::Password,
            g::ItsReplyType::CharacterLine() => Self::CharacterLine,
            g::ItsReplyType::Menu() => Self::Menu,
            g::ItsReplyType::Date() => Self::Date,
            g::ItsReplyType::Time() => Self::Time,
            g::ItsReplyType::Continue() => Self::Continue,
            g::ItsReplyType::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::MessagePayload> for MessagePayload {
    fn from(value: g::MessagePayload) -> Self {
        Self {
            value: value.value.into(),
        }
    }
}

impl From<g::MoreMessagesToSend> for MoreMessagesToSend {
    fn from(value: g::MoreMessagesToSend) -> Self {
        match value {
            g::MoreMessagesToSend::NoMoreMessagesToFollow() => Self::NoMoreMessagesToFollow,
            g::MoreMessagesToSend::MoreMessagesToFollow() => Self::MoreMessagesToFollow,
            g::MoreMessagesToSend::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Indicator> for Indicator {
    fn from(value: g::Indicator) -> Self {
        match value {
            g::Indicator::Inactive() => Self::Inactive,
            g::Indicator::Active() => Self::Active,
            g::Indicator::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::TypeOfMessage> for TypeOfMessage {
    fn from(value: g::TypeOfMessage) -> Self {
        match value {
            g::TypeOfMessage::VoicemailMessageWaiting() => Self::VoicemailMessageWaiting,
            g::TypeOfMessage::FaxMessageWaiting() => Self::FaxMessageWaiting,
            g::TypeOfMessage::ElectronicMailMessageWaiting() => Self::ElectronicMailMessageWaiting,
            g::TypeOfMessage::OtherMessageWaiting() => Self::OtherMessageWaiting,
            g::TypeOfMessage::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::MsMsgWaitFacilities> for MsMsgWaitFacilities {
    fn from(value: g::MsMsgWaitFacilities) -> Self {
        Self {
            indicator: value.indicator.into(),
            type_of_message: value.type_of_message.into(),
        }
    }
}

impl From<g::MsValidityBehavior> for MsValidityBehavior {
    fn from(value: g::MsValidityBehavior) -> Self {
        match value {
            g::MsValidityBehavior::StoreIndefinitely() => Self::StoreIndefinitely,
            g::MsValidityBehavior::PowerDown() => Self::PowerDown,
            g::MsValidityBehavior::ValidUntilRegistrationAreaChanges() => {
                Self::ValidUntilRegistrationAreaChanges
            }
            g::MsValidityBehavior::DisplayOnly() => Self::DisplayOnly,
            g::MsValidityBehavior::RelativeTimePeriod() => Self::RelativeTimePeriod,
            g::MsValidityBehavior::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::UnitsOfTime> for UnitsOfTime {
    fn from(value: g::UnitsOfTime) -> Self {
        match value {
            g::UnitsOfTime::Seconds() => Self::Seconds,
            g::UnitsOfTime::Minutes() => Self::Minutes,
            g::UnitsOfTime::Hours() => Self::Hours,
            g::UnitsOfTime::Days() => Self::Days,
            g::UnitsOfTime::Weeks() => Self::Weeks,
            g::UnitsOfTime::Months() => Self::Months,
            g::UnitsOfTime::Years() => Self::Years,
            g::UnitsOfTime::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::MsValidityInformation> for MsValidityInformation {
    fn from(value: g::MsValidityInformation) -> Self {
        Self {
            units_of_time: value.units_of_time.into(),
            number_of_time_units: value.number_of_time_units,
        }
    }
}

impl From<g::MsValidity> for MsValidity {
    fn from(value: g::MsValidity) -> Self {
        Self {
            validity_behavior: value.validity_behavior.into(),
            validity_information: value.validity_information.map(From::from),
        }
    }
}

impl From<g::NumberOfMessages> for NumberOfMessages {
    fn from(value: g::NumberOfMessages) -> Self {
        match value {
            g::NumberOfMessages::Allowed(value) => Self::Allowed(value),
            g::NumberOfMessages::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::PayloadType> for PayloadType {
    fn from(value: g::PayloadType) -> Self {
        match value {
            g::PayloadType::Default() => Self::Default,
            g::PayloadType::WcmpMessage() => Self::WcmpMessage,
            g::PayloadType::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::SetDpf> for SetDpf {
    fn from(value: g::SetDpf) -> Self {
        match value {
            g::SetDpf::NotRequested() => Self::NotRequested,
            g::SetDpf::Requested() => Self::Requested,
            g::SetDpf::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::UserMessageReference> for UserMessageReference {
    fn from(value: g::UserMessageReference) -> Self {
        Self { value: value.value }
    }
}

impl From<g::UssdServiceOp> for UssdServiceOp {
    fn from(value: g::UssdServiceOp) -> Self {
        match value {
            g::UssdServiceOp::PssdIndication() => Self::PssdIndication,
            g::UssdServiceOp::PssrIndication() => Self::PssrIndication,
            g::UssdServiceOp::UssrRequest() => Self::UssrRequest,
            g::UssdServiceOp::UssnRequest() => Self::UssnRequest,
            g::UssdServiceOp::PssdResponse() => Self::PssdResponse,
            g::UssdServiceOp::PssrResponse() => Self::PssrResponse,
            g::UssdServiceOp::UssrConfirm() => Self::UssrConfirm,
            g::UssdServiceOp::UssnConfirm() => Self::UssnConfirm,
            g::UssdServiceOp::Other(value) => Self::Other(value),
        }
    }
}

impl TryFrom<g::MessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlvValue {
    type Error = Exception;

    fn try_from(value: g::MessageSubmissionRequestTlvValue) -> Result<Self, Self::Error> {
        use g::MessageSubmissionRequestTlvValue as GValue;

        let value = match value {
            GValue::AlertOnMessageDelivery(value) => Self::AlertOnMessageDelivery(value.into()),
            GValue::BillingIdentification(value) => Self::BillingIdentification(
                OctetString::from_bytes(value).map_value_err("billing_identification")?,
            ),
            GValue::CallbackNum(value) => {
                Self::CallbackNum(OctetString::from_bytes(value).map_value_err("callback_num")?)
            }
            GValue::CallbackNumAtag(value) => Self::CallbackNumAtag(
                OctetString::from_bytes(value).map_value_err("callback_num_atag")?,
            ),
            GValue::CallbackNumPresInd(value) => Self::CallbackNumPresInd(value.into()),
            GValue::DestAddrNpCountry(value) => Self::DestAddrNpCountry(
                OctetString::from_bytes(value).map_value_err("dest_addr_np_country")?,
            ),
            GValue::DestAddrNpInformation(value) => Self::DestAddrNpInformation(
                OctetString::from_bytes(value).map_value_err("dest_addr_np_information")?,
            ),
            GValue::DestAddrNpResolution(value) => Self::DestAddrNpResolution(value.into()),
            GValue::DestAddrSubunit(value) => Self::DestAddrSubunit(value.into()),
            GValue::DestBearerType(value) => Self::DestBearerType(value.into()),
            GValue::DestNetworkId(value) => Self::DestNetworkId(
                COctetString::from_bytes(value).map_value_err("dest_network_id")?,
            ),
            GValue::DestNetworkType(value) => Self::DestNetworkType(value.into()),
            GValue::DestNodeId(value) => {
                Self::DestNodeId(OctetString::from_bytes(value).map_value_err("dest_node_id")?)
            }
            GValue::DestSubaddress(value) => {
                Self::DestSubaddress(value.try_into().map_value_err("dest_subaddress")?)
            }
            GValue::DestTelematicsId(value) => Self::DestTelematicsId(value),
            GValue::DestPort(value) => Self::DestPort(value),
            GValue::DisplayTime(value) => Self::DisplayTime(value.into()),
            GValue::ItsReplyType(value) => Self::ItsReplyType(value.into()),
            GValue::ItsSessionInfo(value) => Self::ItsSessionInfo(value.into()),
            GValue::LanguageIndicator(value) => Self::LanguageIndicator(value.into()),
            GValue::MessagePayload(value) => Self::MessagePayload(value.into()),
            GValue::MoreMessagesToSend(value) => Self::MoreMessagesToSend(value.into()),
            GValue::MsMsgWaitFacilities(value) => Self::MsMsgWaitFacilities(value.into()),
            GValue::MsValidity(value) => Self::MsValidity(value.into()),
            GValue::NumberOfMessages(value) => Self::NumberOfMessages(value.into()),
            GValue::PayloadType(value) => Self::PayloadType(value.into()),
            GValue::PrivacyIndicator(value) => Self::PrivacyIndicator(value.into()),
            GValue::QosTimeToLive(value) => Self::QosTimeToLive(value),
            GValue::SarMsgRefNum(value) => Self::SarMsgRefNum(value),
            GValue::SarSegmentSeqnum(value) => Self::SarSegmentSeqnum(value),
            GValue::SarTotalSegments(value) => Self::SarTotalSegments(value),
            GValue::SetDpf(value) => Self::SetDpf(value.into()),
            GValue::SmsSignal(value) => Self::SmsSignal(value),
            GValue::SourceAddrSubunit(value) => Self::SourceAddrSubunit(value.into()),
            GValue::SourceBearerType(value) => Self::SourceBearerType(value.into()),
            GValue::SourceNetworkId(value) => Self::SourceNetworkId(
                COctetString::from_bytes(value).map_value_err("source_network_id")?,
            ),
            GValue::SourceNetworkType(value) => Self::SourceNetworkType(value.into()),
            GValue::SourceNodeId(value) => {
                Self::SourceNodeId(OctetString::from_bytes(value).map_value_err("source_node_id")?)
            }
            GValue::SourcePort(value) => Self::SourcePort(value),
            GValue::SourceSubaddress(value) => {
                Self::SourceSubaddress(value.try_into().map_value_err("source_subaddress")?)
            }
            GValue::SourceTelematicsId(value) => Self::SourceTelematicsId(value),
            GValue::UserMessageReference(value) => Self::UserMessageReference(value.into()),
            GValue::UserResponseCode(value) => Self::UserResponseCode(value),
            GValue::UssdServiceOp(value) => Self::UssdServiceOp(value.into()),
            GValue::Other { tag, value } => Self::Other {
                tag: tag.into(),
                value: value.into(),
            },
        };

        Ok(value)
    }
}

impl From<g::EsmClass> for EsmClass {
    fn from(value: g::EsmClass) -> Self {
        Self {
            messaging_mode: value.messaging_mode.into(),
            message_type: value.message_type.into(),
            ansi41_specific: value.ansi41_specific.into(),
            gsm_features: value.gsm_features.into(),
        }
    }
}

impl From<g::MessagingMode> for MessagingMode {
    fn from(value: g::MessagingMode) -> Self {
        match value {
            g::MessagingMode::Default() => Self::Default,
            g::MessagingMode::Datagram() => Self::Datagram,
            g::MessagingMode::Forward() => Self::Forward,
            g::MessagingMode::StoreAndForward() => Self::StoreAndForward,
            g::MessagingMode::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::MessageType> for MessageType {
    fn from(value: g::MessageType) -> Self {
        match value {
            g::MessageType::Default() => Self::Default,
            g::MessageType::ShortMessageContainsMCDeliveryReceipt() => {
                Self::ShortMessageContainsMCDeliveryReceipt
            }
            g::MessageType::ShortMessageContainsIntermediateDeliveryNotification() => {
                Self::ShortMessageContainsIntermediateDeliveryNotification
            }
            g::MessageType::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::Ansi41Specific> for Ansi41Specific {
    fn from(value: g::Ansi41Specific) -> Self {
        match value {
            g::Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement() => {
                Self::ShortMessageContainsDeliveryAcknowledgement
            }
            g::Ansi41Specific::ShortMessageContainsUserAcknowledgment() => {
                Self::ShortMessageContainsUserAcknowledgment
            }
            g::Ansi41Specific::ShortMessageContainsConversationAbort() => {
                Self::ShortMessageContainsConversationAbort
            }
            g::Ansi41Specific::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::GsmFeatures> for GsmFeatures {
    fn from(value: g::GsmFeatures) -> Self {
        match value {
            g::GsmFeatures::NotSelected() => Self::NotSelected,
            g::GsmFeatures::UdhiIndicator() => Self::UdhiIndicator,
            g::GsmFeatures::SetReplyPath() => Self::SetReplyPath,
            g::GsmFeatures::SetUdhiAndReplyPath() => Self::SetUdhiAndReplyPath,
            g::GsmFeatures::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::ReplaceIfPresentFlag> for ReplaceIfPresentFlag {
    fn from(value: g::ReplaceIfPresentFlag) -> Self {
        match value {
            g::ReplaceIfPresentFlag::DoNotReplace() => Self::DoNotReplace,
            g::ReplaceIfPresentFlag::Replace() => Self::Replace,
            g::ReplaceIfPresentFlag::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::RegisteredDelivery> for RegisteredDelivery {
    fn from(value: g::RegisteredDelivery) -> Self {
        Self::new(
            value.mc_delivery_receipt.into(),
            value.sme_originated_acknowledgement.into(),
            value.intermediate_notification.into(),
            value.other,
        )
    }
}

impl From<g::MCDeliveryReceipt> for MCDeliveryReceipt {
    fn from(value: g::MCDeliveryReceipt) -> Self {
        match value {
            g::MCDeliveryReceipt::NoMcDeliveryReceiptRequested() => Self::NoMcDeliveryReceiptRequested,
            g::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure() => Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
            g::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure() => Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure,
            g::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess() => Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess,
            g::MCDeliveryReceipt::Other(value) => Self::Other(value)
        }
    }
}

impl From<g::SmeOriginatedAcknowledgement> for SmeOriginatedAcknowledgement {
    fn from(value: g::SmeOriginatedAcknowledgement) -> Self {
        match value {
            g::SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested() => {
                Self::NoReceiptSmeAcknowledgementRequested
            }
            g::SmeOriginatedAcknowledgement::SmeDeliveryAcknowledgementRequested() => {
                Self::SmeDeliveryAcknowledgementRequested
            }
            g::SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested() => {
                Self::SmeUserAcknowledgementRequested
            }
            g::SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested() => {
                Self::BothDeliveryAndUserAcknowledgmentRequested
            }
            g::SmeOriginatedAcknowledgement::Other(value) => Self::Other(value),
        }
    }
}

impl From<g::IntermediateNotification> for IntermediateNotification {
    fn from(value: g::IntermediateNotification) -> Self {
        match value {
            g::IntermediateNotification::NoIntermediaryNotificationRequested() => {
                Self::NoIntermediaryNotificationRequested
            }
            g::IntermediateNotification::IntermediateNotificationRequested() => {
                Self::IntermediateNotificationRequested
            }
            g::IntermediateNotification::Other(value) => Self::Other(value),
        }
    }
}
