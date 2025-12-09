//! `SMPP` values.

pub mod parts {
    pub use super::broadcast_content_type::BroadcastContentTypeParts;
    pub use super::broadcast_frequency_interval::BroadcastFrequencyIntervalParts;
    pub use super::broadcast_rep_num::BroadcastRepNumParts;
    pub use super::callback_num_pres_ind::CallbackNumPresIndParts;
    pub use super::esm_class::EsmClassParts;
    pub use super::its_session_info::ItsSessionInfoParts;
    pub use super::ms_msg_wait_facilities::MsMsgWaitFacilitiesParts;
    pub use super::ms_validity::{MsValidityInformationParts, MsValidityParts};
    pub use super::network_error_code::NetworkErrorCodeParts;
    pub use super::priority_flag::PriorityFlagParts;
    pub use super::registered_delivery::RegisteredDeliveryParts;
    pub use super::user_message_reference::UserMessageReferenceParts;
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
    pub use super::dest_address::borrowed::{DestAddress, DistributionListName, SmeAddress};
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
        pub use super::super::broadcast_area_identifier::owned::BroadcastAreaIdentifierParts;
        pub use super::super::dest_address::owned::{DistributionListNameParts, SmeAddressParts};
        pub use super::super::message_payload::owned::MessagePayloadParts;
        pub use super::super::service_type::owned::ServiceTypeParts;
        pub use super::super::sub_address::owned::SubaddressParts;
        pub use super::super::unsuccess_sme::owned::UnsuccessSmeParts;
    }

    pub use super::broadcast_area_identifier::owned::BroadcastAreaIdentifier;
    pub use super::dest_address::owned::{DestAddress, DistributionListName, SmeAddress};
    pub use super::message_payload::owned::MessagePayload;
    pub use super::service_type::owned::ServiceType;
    pub use super::sub_address::owned::Subaddress;
    pub use super::unsuccess_sme::owned::UnsuccessSme;
}
