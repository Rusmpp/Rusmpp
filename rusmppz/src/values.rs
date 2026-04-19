//! `SMPP` values.

pub mod parts {
    pub use rusmpp_core::values::borrowed::parts::{
        BroadcastAreaIdentifierParts, DistributionListNameParts, MessagePayloadParts,
        ServiceTypeParts, SmeAddressParts, SubaddressParts, UnsuccessSmeParts,
    };
    pub use rusmpp_core::values::parts::{
        BroadcastContentTypeParts, BroadcastFrequencyIntervalParts, BroadcastRepNumParts,
        CallbackNumPresIndParts, EsmClassParts, ItsSessionInfoParts, MsMsgWaitFacilitiesParts,
        NetworkErrorCodeParts, PriorityFlagParts, RegisteredDeliveryParts,
        UserMessageReferenceParts, {MsValidityInformationParts, MsValidityParts},
    };
}

pub use rusmpp_core::values::{
    AddrSubunit, AlertOnMessageDelivery, Ansi41Cbs, Ansi41Specific, Ansi136, BearerType,
    BroadcastAreaFormat, BroadcastAreaSuccess, BroadcastChannelIndicator, BroadcastContentType,
    BroadcastFrequencyInterval, BroadcastMessageClass, BroadcastRepNum, CallbackNumPresInd,
    CongestionState, DataCoding, DeliveryFailureReason, DestAddrNpResolution, DestFlag,
    DisplayTime, DpfResult, EncodingContentType, ErrorCodeNetworkType, EsmClass,
    GenericServiceType, GsmCbs, GsmFeatures, GsmSms, Indicator, InterfaceVersion,
    IntermediateNotification, Is95, ItsReplyType, ItsSessionInfo, LanguageIndicator,
    MCDeliveryReceipt, MessageState, MessageType, MessagingMode, MoreMessagesToSend,
    MsAvailabilityStatus, MsMsgWaitFacilities, MsValidity, MsValidityBehavior,
    MsValidityInformation, NetworkErrorCode, NetworkType, Npi, NumberOfMessages, PayloadType,
    Presentation, PriorityFlag, PriorityFlagType, PrivacyIndicator, RegisteredDelivery,
    ReplaceIfPresentFlag, Screening, SetDpf, SmeOriginatedAcknowledgement, SubaddressTag, Ton,
    TypeOfMessage, TypeOfNetwork, UnitOfTime, UnitsOfTime, UserMessageReference, UssdServiceOp,
    borrowed::{
        BroadcastAreaIdentifier, DestAddress, DestAddressValue, DistributionListName,
        MessagePayload, ServiceType, SmeAddress, Subaddress, UnsuccessSme,
    },
};
