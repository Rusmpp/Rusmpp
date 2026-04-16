//! `SMPP` values.

pub mod parts {
    pub use super::{
        broadcast_content_type::BroadcastContentTypeParts,
        broadcast_frequency_interval::BroadcastFrequencyIntervalParts,
        broadcast_rep_num::BroadcastRepNumParts,
        callback_num_pres_ind::CallbackNumPresIndParts,
        esm_class::EsmClassParts,
        its_session_info::ItsSessionInfoParts,
        ms_msg_wait_facilities::MsMsgWaitFacilitiesParts,
        ms_validity::{MsValidityInformationParts, MsValidityParts},
        network_error_code::NetworkErrorCodeParts,
        priority_flag::PriorityFlagParts,
        registered_delivery::RegisteredDeliveryParts,
        user_message_reference::UserMessageReferenceParts,
    };
}

pub mod errors {
    pub use super::{
        broadcast_content_type::{
            BroadcastContentTypeDecodeError, BroadcastContentTypeDecodeErrorContext,
        },
        broadcast_frequency_interval::{
            BroadcastFrequencyIntervalDecodeError, BroadcastFrequencyIntervalDecodeErrorContext,
        },
        broadcast_rep_num::{BroadcastRepNumDecodeError, BroadcastRepNumDecodeErrorContext},
        its_session_info::{ItsSessionInfoDecodeError, ItsSessionInfoDecodeErrorContext},
        ms_validity::{
            MsValidityDecodeError, MsValidityDecodeErrorContext, MsValidityInformationDecodeError,
            MsValidityInformationDecodeErrorContext,
        },
        network_error_code::{NetworkErrorCodeDecodeError, NetworkErrorCodeDecodeErrorContext},
        user_message_reference::{
            UserMessageReferenceDecodeError, UserMessageReferenceDecodeErrorContext,
        },
    };
}

mod addr_subunit;
pub use addr_subunit::AddrSubunit;

mod alert_on_msg_delivery;
pub use alert_on_msg_delivery::AlertOnMessageDelivery;

mod bearer_type;
pub use bearer_type::BearerType;

mod broadcast_area_identifier;
pub use broadcast_area_identifier::BroadcastAreaFormat;

mod broadcast_area_success;
pub use broadcast_area_success::BroadcastAreaSuccess;

mod broadcast_channel_indicator;
pub use broadcast_channel_indicator::BroadcastChannelIndicator;

mod broadcast_content_type;
pub use broadcast_content_type::{BroadcastContentType, EncodingContentType, TypeOfNetwork};

mod broadcast_frequency_interval;
pub use broadcast_frequency_interval::{BroadcastFrequencyInterval, UnitOfTime};

mod broadcast_message_class;
pub use broadcast_message_class::BroadcastMessageClass;

mod callback_num_pres_ind;
pub use callback_num_pres_ind::{CallbackNumPresInd, Presentation, Screening};

mod congestion_state;
pub use congestion_state::CongestionState;

mod data_coding;
pub use data_coding::DataCoding;

mod delivery_failure_reason;
pub use delivery_failure_reason::DeliveryFailureReason;

mod dest_addr_np_resolution;
pub use dest_addr_np_resolution::DestAddrNpResolution;

mod dest_address;
pub use dest_address::DestFlag;

mod display_time;
pub use display_time::DisplayTime;

mod dpf_result;
pub use dpf_result::DpfResult;

mod esm_class;
pub use esm_class::{Ansi41Specific, EsmClass, GsmFeatures, MessageType, MessagingMode};

mod interface_version;
pub use interface_version::InterfaceVersion;

mod its_reply_type;
pub use its_reply_type::ItsReplyType;

mod its_session_info;
pub use its_session_info::ItsSessionInfo;

mod language_indicator;
pub use language_indicator::LanguageIndicator;

mod message_state;
pub use message_state::MessageState;

mod more_messages_to_send;
pub use more_messages_to_send::MoreMessagesToSend;

mod ms_availability_status;
pub use ms_availability_status::MsAvailabilityStatus;

mod ms_msg_wait_facilities;
pub use ms_msg_wait_facilities::{Indicator, MsMsgWaitFacilities, TypeOfMessage};

mod ms_validity;
pub use ms_validity::{MsValidity, MsValidityBehavior, MsValidityInformation, UnitsOfTime};

mod network_error_code;
pub use network_error_code::{ErrorCodeNetworkType, NetworkErrorCode};

mod network_type;
pub use network_type::NetworkType;

mod npi;
pub use npi::Npi;

mod number_of_messages;
pub use number_of_messages::NumberOfMessages;

mod payload_type;
pub use payload_type::PayloadType;

mod priority_flag;
pub use priority_flag::{Ansi41Cbs, Ansi136, GsmCbs, GsmSms, Is95, PriorityFlag, PriorityFlagType};

mod privacy_indicator;
pub use privacy_indicator::PrivacyIndicator;

mod registered_delivery;
pub use registered_delivery::{
    IntermediateNotification, MCDeliveryReceipt, RegisteredDelivery, SmeOriginatedAcknowledgement,
};

mod replace_if_present_flag;
pub use replace_if_present_flag::ReplaceIfPresentFlag;

mod service_type;
pub use service_type::GenericServiceType;

mod set_dpf;
pub use set_dpf::SetDpf;

mod sub_address;
pub use sub_address::SubaddressTag;

mod ton;
pub use ton::Ton;

mod unsuccess_sme;

mod ussd_service_op;
pub use ussd_service_op::UssdServiceOp;

mod user_message_reference;
pub use user_message_reference::UserMessageReference;

mod broadcast_rep_num;
pub use broadcast_rep_num::BroadcastRepNum;

mod message_payload;

pub mod borrowed {
    //! Borrowed `SMPP` values.

    pub mod parts {
        pub use super::super::broadcast_area_identifier::borrowed::BroadcastAreaIdentifierParts;
        pub use super::super::dest_address::borrowed::{
            DistributionListNameParts, SmeAddressParts,
        };
        pub use super::super::message_payload::borrowed::MessagePayloadParts;
        pub use super::super::service_type::borrowed::ServiceTypeParts;
        pub use super::super::sub_address::borrowed::SubaddressParts;
        pub use super::super::unsuccess_sme::borrowed::UnsuccessSmeParts;
    }

    pub use super::broadcast_area_identifier::borrowed::BroadcastAreaIdentifier;
    pub use super::dest_address::borrowed::{
        DestAddress, DestAddressValue, DistributionListName, SmeAddress,
    };
    pub use super::message_payload::borrowed::MessagePayload;
    pub use super::service_type::borrowed::ServiceType;
    pub use super::sub_address::borrowed::Subaddress;
    pub use super::unsuccess_sme::borrowed::UnsuccessSme;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned {
    //! Owned `SMPP` values.

    pub mod parts {
        pub use super::super::{
            broadcast_area_identifier::owned::BroadcastAreaIdentifierParts,
            dest_address::owned::{DistributionListNameParts, SmeAddressParts},
            message_payload::owned::MessagePayloadParts,
            service_type::owned::ServiceTypeParts,
            sub_address::owned::SubaddressParts,
            unsuccess_sme::owned::UnsuccessSmeParts,
        };
    }

    pub mod errors {
        pub use super::super::{
            broadcast_area_identifier::owned::{
                BroadcastAreaIdentifierDecodeError, BroadcastAreaIdentifierDecodeErrorContext,
            },
            dest_address::owned::{
                DistributionListNameDecodeError, DistributionListNameDecodeErrorContext,
                SmeAddressDecodeError, SmeAddressDecodeErrorContext,
            },
            message_payload::owned::{MessagePayloadDecodeError, MessagePayloadDecodeErrorContext},
            service_type::owned::{ServiceTypeDecodeError, ServiceTypeDecodeErrorContext},
            sub_address::owned::{SubaddressDecodeError, SubaddressDecodeErrorContext},
            unsuccess_sme::owned::{UnsuccessSmeDecodeError, UnsuccessSmeDecodeErrorContext},
        };
    }

    pub use super::{
        broadcast_area_identifier::owned::BroadcastAreaIdentifier,
        dest_address::owned::{DestAddress, DestAddressValue, DistributionListName, SmeAddress},
        message_payload::owned::MessagePayload,
        service_type::owned::ServiceType,
        sub_address::owned::Subaddress,
        unsuccess_sme::owned::UnsuccessSme,
    };
}
