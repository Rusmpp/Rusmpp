use rusmpp_macros::Rusmpp;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[repr(u16)]
#[rusmpp(test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]

pub enum TlvTag {
    /// The subcomponent in the destination device for which the user data is intended.
    ///
    /// The dest_addr_subunit parameter is used to route messages when received by a mobile
    /// station, for example to a smart card in the mobile station or to an external device connected
    /// to the mobile station.
    DestAddrSubunit = 0x0005,
    /// The correct network for the destination device.
    ///
    /// The dest_network_type parameter is used to indicate a network type associated with the
    /// destination address of a message. In the case that the receiving system (e.g. MC) does not
    /// support the indicated network type, it may treat this a failure and return a response PDU
    /// reporting a failure.
    DestNetworkType = 0x0006,
    /// The correct bearer type for delivering the user data to the destination.
    ///
    /// The dest_bearer_type parameter is used to request the desired bearer for delivery of the
    /// message to the destination address. In the case that the receiving system (e.g. MC) does not
    /// support the indicated bearer type, it may treat this a failure and return a response PDU
    /// reporting a failure.
    DestBearerType = 0x0007,
    /// The telematics identifier associated with the destination
    ///
    /// This parameter defines the telematic interworking to be used by the delivering system for the
    /// destination address. This is only useful when a specific dest_bearer_type parameter has also
    /// been specified, as the value is bearer dependent. In the case that the receiving system (e.g.
    /// MC) does not support the indicated telematic interworking, it may treat this a failure and
    /// return a response PDU reporting a failure.
    DestTelematicsId = 0x0008,
    /// The subcomponent in the destination device, which created the user data
    ///
    /// The source_addr_subunit parameter is used to indicate where a message originated in the
    /// mobile station, for example a smart card in the mobile station or an external device
    /// connected to the mobile station.
    SourceAddrSubunit = 0x000D,
    /// The correct network associated with the originating device.
    ///
    /// The source_network_type parameter is used to indicate the network type associated with the
    /// device that originated the message.
    SourceNetworkType = 0x000E,
    /// The correct bearer type for delivering the user data to the destination
    ///
    ///The source_bearer_type parameter indicates the wireless bearer over which the message
    /// originated.
    SourceBearerType = 0x000F,
    /// The telematics identifier associated with the source
    ///
    /// The source_telematics_id parameter indicates the type of telematics interface over which the
    /// message originated.
    SourceTelematicsId = 0x0010,
    /// Time to live as a relative time in seconds from submission.
    ///
    /// This parameter defines the number of seconds which the sender requests the MC to keep
    /// the message if undelivered before it is deemed expired. If the parameter is not present, the
    /// MC may apply a default value
    QosTimeToLive = 0x0017,
    /// defines the type of payload (e.g. WDP, WCMP, etc.).
    ///
    /// The payload_type parameter defines the higher layer PDU type contained in the message payload.
    PayloadType = 0x0019,
    /// ASCII text giving a description of the meaning of the response.
    ///
    /// The additional_status_info_text parameter gives an ASCII textual description of the meaning
    /// of a response PDU. It is to be used by an implementation to allow easy diagnosis of problems
    AdditionalStatusInfoText = 0x001D,
    /// MC message ID of message being receipted. Should be
    /// present for MC Delivery Receipts and Intermediate
    /// Notifications.
    ///
    /// The receipted_message_id parameter indicates the ID of the message being receipted in a
    /// MC Delivery Receipt. This is the opaque MC message identifier that was returned in the
    /// message_id parameter of the SMPP response PDU that acknowledged the submission of the
    /// original message.
    ReceiptedMessageId = 0x001E,
    /// This parameter controls the indication and specifies the
    /// message type (of the message associated with the MWI)
    /// at the mobile station.
    ///
    /// The ms_msg_wait_facilities parameter allows an indication to be provided to an MS that
    /// there are messages waiting for the subscriber on systems on the PLMN. The indication can
    /// be an icon on the MS screen or other MMI indication.
    ///
    /// The ms_msg_wait_facilities can also specify the type of message associated with the
    /// message waiting indication.
    MsMsgWaitFacilities = 0x0030,
    /// Indicates the level of privacy associated with the message.
    ///
    /// The privacy_indicator indicates the privacy level of the message.
    PrivacyIndicator = 0x0201,
    /// The sub-address of the message originator.
    ///
    /// The source_subaddress parameter specifies a subaddress associated with the originator of the message.
    SourceSubaddress = 0x0202,
    /// The sub-address of the message destination.
    ///
    /// The dest_subaddress parameter specifies a subaddress associated with the destination of the message.
    DestSubaddress = 0x0203,
    /// ESME assigned message reference number.
    ///
    /// A reference assigned by the originating SME to the short message. Depending on the
    /// destination network technology, this field may be passed directly to the mobile device.
    ///
    /// The user_message_reference TLV is also applicable in ancillary broadcast operations as a
    /// means of identifying a previously submitted message. In such cases, the
    /// user_message_reference can be used to substitute an actual message_id or may be used in
    /// conjunction with a message_id
    UserMessageReference = 0x0204,
    /// A user response code. The actual response codes are implementation specific.
    ///
    /// A response code set by the user in a User Acknowledgement/Reply message. The response
    /// codes are application specific.
    UserResponseCode = 0x0205,
    /// Indicates the application port number associated with the
    /// source address of the message. This parameter should be
    /// present for WAP applications.
    ///
    /// The source_port parameter is used to indicate the application port number associated with
    /// the source address of the message.
    SourcePort = 0x020A,
    /// Indicates the application port number associated with the
    /// destination address of the message. This parameter
    /// should be present for WAP applications.
    ///
    /// The dest_port parameter is used to indicate the application port number associated with the
    /// destination address of the message.
    DestPort = 0x020B,
    /// The reference number for a particular concatenated short message.
    ///
    /// The sar_msg_ref_num parameter is used to indicate the reference number for a particular
    /// concatenated short message.
    /// The concatenation related parameters are sar_msg_ref_num, sar_total_segments and
    /// sar_segment_seqnum. Where these are present the other parameters of the message
    /// should remain unchanged for each short message fragment which forms part of a mobile
    /// terminated concatenated short message, with the exception of those parameters for which it
    /// makes sense to change them (such as the user data in the short_message parameter).
    SarMsgRefNum = 0x020C,
    /// Indicates the language of an alphanumeric text message.
    ///
    /// The language_indicator parameter is used to indicate the language of the short message.
    LanguageIndicator = 0x020D,
    /// Indicates the total number of short message segments within the concatenated short message.
    ///
    /// The sar_total_segments parameter is used to indicate the total number of short messages
    /// within the concatenated short message.
    SarTotalSegments = 0x020E,
    /// Indicates the sequence number of a particular short
    /// message fragment within the concatenated short message
    ///
    /// The sar_segment_seqnum parameter is used to indicate the sequence number of a particular
    /// short message within the concatenated short message.
    SarSegmentSeqnum = 0x020F,
    /// SMPP version supported by MC
    ///
    /// The sc_interface_version parameter is used to indicate the SMPP version supported by the
    /// MC. It is returned in the bind response PDUs.
    ScInterfaceVersion = 0x0210,
    /// Defines the call-back number presentation and screening.
    /// If this parameter is present and there are multiple
    /// instances of the callback_num parameter then this
    /// parameter must occur an equal number of instances and
    /// the order of occurrence determines the particular
    /// callback_num_pres_ind which corresponds to a particular
    /// callback_num.
    CallbackNumPresInd = 0x0302,
    /// Associates a displayable alphanumeric tag with the call-
    /// back number.
    /// If this parameter is present and there are multiple
    /// instances of the callback_num parameter then this
    /// parameter must occur an equal number of instances and
    /// the order of occurrence determines the particular
    /// callback_num_atag which corresponds to a particular
    /// callback_num.
    ///
    /// The callback_num_atag parameter associates an alphanumeric display with the call back
    /// number.
    CallbackNumAtag = 0x0303,
    /// Indicates the number of messages stored in a mail box
    ///
    /// The number_of_messages parameter is used to indicate the number of messages stored in a
    /// mailbox.
    NumberOfMessages = 0x0304,
    /// A call-back number associated with the short message.
    /// This parameter can be included a number of times for
    /// multiple call-back addresses.
    ///
    /// The callback_num parameter associates a call back number with the message. In TDMA
    /// networks, it is possible to send and receive multiple call-back numbers to/from TDMA mobile
    /// stations.
    CallbackNum = 0x0381,
    /// Indicates whether the Delivery Pending Flag was set
    ///
    /// The dpf_result parameter is used to indicate if delivery pending flag (DPF) was set for a
    /// delivery failure of a short message.
    ///
    /// When used in conjunction with transaction mode, dpf_result can be returned in a
    /// submit_sm_resp or data_sm_resp PDU. Where store and forward or datagram modes are
    /// used in the original submission, dpf_result may be returned as part of a delivery receipt in the
    /// form of a deliver_sm or data_sm PDU.
    /// If the dpf_result parameter is not returned, then the ESME should assume that DPF is not
    /// set.
    DpfResult = 0x0420,
    /// Indicator for setting Delivery Pending Flag on delivery failure.
    ///
    /// An ESME may use the set_dpf parameter to request the setting of a delivery pending flag
    /// (DPF) for certain delivery failure scenarios, such as MS unavailability (as indicated by the
    /// HLR).
    ///
    /// The MC should respond to such a request with an alert_notification PDU when it detects that
    /// the destination MS has become available.
    ///
    /// The delivery failure scenarios under which DPF is set is MC implementation and network
    /// implementation specific. If a delivery pending flag is set by the MC or network (e.g. HLR),
    /// then the MC should indicate this to the ESME in the submit_sm_resp or data_sm_resp PDU
    /// via the dpf_result parameter. It may also use a delivery receipt to relay this information and
    /// as a result may use a deliver_sm or data_sm PDU to carry the dpf_result. For more
    /// information see 4.8.4.32
    SetDpf = 0x0421,
    /// The status of the mobile station
    ///
    /// The ms_availability_status parameter is used in the alert_notification operation to indicate the
    /// availability state of the MS to the ESME.
    /// If the MC does not include the parameter in the alert_notification operation, the ESME should
    /// assume that the MS is in an “available” state.
    MsAvailabilityStatus = 0x0422,
    /// Error code specific to a wireless network.
    ///
    /// The network_error_code parameter is used to indicate the actual network error code for a
    /// delivery failure. The network error code is technology specific.
    NetworkErrorCode = 0x0423,
    /// Contains the extended short message user data. Up to
    /// 64K octets can be transmitted.
    /// Note: The short message data should be inserted in either
    /// the short_message or message_payload fields. Both fields
    /// should not be used simultaneously.
    /// The sm_length field should be set to zero if using the
    /// message_payload parameter.
    /// Note: In the case of data_sm, the message_payload TLV
    /// is the only means of specifying text.
    ///
    /// The message_payload parameter contains the user data. Its function is to provide an
    /// alternative means of carrying text lengths above the 255 octet limit of the short_message
    /// field.
    ///
    /// Applications, which need to send messages longer than 255 octets, should use the
    /// message_payload TLV. When used in the context of a submit_sm PDU, the sm_length field
    /// should be set to zero.
    MessagePayload = 0x0424,
    /// Include to indicate reason for delivery failure.
    ///
    /// The delivery_failure_reason parameter is used in the data_sm_resp operation to indicate the
    /// outcome of the message delivery attempt (only applicable for transaction message mode). If
    /// a delivery failure due to a network error is indicated, the ESME may check the
    /// network_error_code parameter (if present) for the actual network error code.
    DeliveryFailureReason = 0x0425,
    /// Indicates that there are more messages to follow for the destination SME.
    ///
    /// The more_messages_to_send parameter is used by the ESME in the submit_sm and
    /// data_sm operations to indicate to the MC that there are further messages for the same
    /// destination SME. The MC may use this setting for network resource optimization.
    MoreMessagesToSend = 0x0426,
    /// Should be present for MC Delivery Receipts and Intermediate Notifications.
    ///
    /// The message_state TLV is used by the MC in the deliver_sm and data_sm PDUs to indicate
    /// to the ESME the final message state for a MC Delivery Receipt. The message_state TLV is
    /// also returned by the MC to the ESME as part of the query_broadcast_sm_resp PDU.
    MessageState = 0x0427,
    /// The congestion_state parameter is used to pass congestion status information between
    /// ESME and MC as a means of providing flow control and congestion avoidance capabilities to
    /// the sending peer. The TLV can be used in any SMPP operation response PDU as a means
    /// of passing congestion status from one peer to another. Typical uses of this would be in
    /// submit_sm/submit_sm_resp sequences where an ESME would drive a batch of submissions
    /// at a high rate and use continual tracking of the returned congestion_state values as a means
    /// of gauging the congestion. Reaction to a variation in congestion_state would involve
    /// increasing/decreasing the rate as required to maintain the balance in the Optimum range
    CongestionState = 0x0428,
    /// This parameter is used to identify the required USSD
    /// Service type when interfacing to a USSD system.
    ///
    /// The ussd_service_op parameter is required to define the USSD service operation when
    /// SMPP is being used as an interface to a (GSM) USSD system.
    UssdServiceOp = 0x0501,
    /// Specifies the Cell Broadcast channel that should be used for broadcasting the message.
    ///
    /// The broadcast_channel_indicator parameter specifies the Cell Broadcast channel that should
    /// be used for broadcasting the message.
    BroadcastChannelIndicator = 0x0600,
    /// Specifies the content type of the message.
    ///
    /// The broadcast_content_type parameter specifies the content_type of the message content.
    BroadcastContentType = 0x0601,
    /// This parameter contains additional free format information specific to the
    /// broadcast_content_type.
    ///
    /// The broadcast_content_type_info parameter contains additional information specific to the
    /// broadcast_content_type.
    BroadcastContentTypeInfo = 0x0602,
    /// This field specifies the class of message to be broadcast.
    ///
    /// The broadcast_message_class parameter is used to route messages when received by a
    /// mobile station to user-defined destinations or to Terminal Equipment.
    BroadcastMessageClass = 0x0603,
    /// This field indicates the number of repeated broadcasts of a message requested by the submitter.
    BroadcastRepNum = 0x0604,
    /// This field indicates the frequency interval at which the broadcasts of a message should be repeated.
    ///
    /// The broadcast_frequency_interval parameter specifies the frequency interval at which the
    /// broadcasts of a message should be repeated.
    BroadcastFrequencyInterval = 0x0605,
    /// Identifies the target Broadcast Area(s) for the requested message broadcast.
    /// This parameter can be included a number of times for multiple target Broadcast Areas(s).
    ///
    /// The broadcast_area_identifier defines the Broadcast Area in terms of a geographical
    /// descriptor.
    BroadcastAreaIdentifier = 0x0606,
    /// This field will indicate the nature of the failure associated with the broadcast request for the indicated area.
    /// If this parameter is present and there are multiple instances of the failed*broadcast_area* identifier(s)
    /// parameter then this parameter must occur an equal number of instances and the order of occurrence determines the particular
    /// broadcast*error_status, which corresponds to a particular failed_broadcast_area* identifier(s).
    ///
    /// The broadcast_error_status parameter specifies the nature of the failure associated with a
    /// particular broadcast_area_identifier specified in a broadcast request.
    BroadcastErrorStatus = 0x0607,
    /// The success rate indicator, defined as the ratio of the number of BTSs that accepted
    /// the message and the total number of BTSs that should have accepted
    /// the message, for a particular broadcast_area_identifier.
    ///
    /// The broadcast_area_success parameter is a success rate indicator, defined as the ratio of
    /// the number of BTSs who accepted the message and the total number of BTSs who should
    /// accept the message, for a particular broadcast_area_identifier.
    BroadcastAreaSuccess = 0x0608,
    /// The date and time at which the broadcasting state of this message was set to terminated in the Message Centre.
    ///
    /// The broadcast_end_time parameter indicates the date and time at which the broadcasting
    /// state of this message was set to terminated in the Message Centre.
    BroadcastEndTime = 0x0609,
    /// This parameter is used to specify special target groups for broadcast information.
    ///
    /// The broadcast_service_group parameter is used to specify special target groups for
    /// broadcast information.
    BroadcastServiceGroup = 0x060A,
    /// Billing information passed from ESME to MC
    BillingIdentification = 0x060B,
    /// Identification of source network
    ///
    /// The source_network_id assigned to a wireless network operator or ESME operator is a
    /// unique address that may be derived and assigned by the node owner without establishing a
    /// central assignment and management authority. When this TLV is specified, it must be
    /// accompanied with a source_node_id TLV Ref. 4.8.4.58.
    SourceNetworkId = 0x060D,
    /// Identification of destination network
    ///
    /// The dest_network_id assigned to a wireless network operator or ESME operator is a unique
    /// address that may be derived and assigned by the node owner without establishing a central
    /// assignment and management authority. When this TLV is specified, it must be accompanied
    /// with a dest_node_id TLV Ref.4.8.4.27
    DestNetworkId = 0x060E,
    /// Identification of source node
    ///
    /// The source_node_id is a unique number assigned within a single ESME or MC network and must
    /// uniquely identify an originating node within the context of the MC or ESME. The content of a
    /// source_node_id is comprised of decimal digits and is at the discretion of the owning ESME or MC.
    SourceNodeId = 0x060F,
    /// Identification of destination node
    ///
    /// The dest_node_id is a unique number assigned within a single ESME or MC network and
    /// must uniquely identify a destination node within the context of the MC or ESME. The content
    /// of a dest_node_id is comprised of decimal digits and is at the discretion of the owning ESME
    /// or MC.
    DestNodeId = 0x0610,
    /// Number portability query indicator
    ///
    /// The dest_addr_np_resolution TLV is used to pass an indicator relating to a number portability
    /// query. If this TLV is omitted, the default value is assumed.
    DestAddrNpResolution = 0x0611,
    /// Number portability information for the destination address
    ///
    /// The dest_addr_np_information TLV is used to carry number portability information.
    DestAddrNpInformation = 0x0612,
    /// E.164 information to the operator country code
    ///
    /// The dest_addr_np_country TLV is used to carry E.164 information relating to the operator country code.
    DestAddrNpCountry = 0x0613,
    /// Provides the receiving MS with a display time associated with the message.
    ///
    /// The display_time parameter is used to associate a display time of the short message on the MS.
    DisplayTime = 0x1201,
    /// Indicates the alerting mechanism when the message is received by an MS.
    ///
    /// The sms_signal parameter is used to provide a TDMA MS with alert tone information
    /// associated with the received short message
    SmsSignal = 0x1203,
    /// Indicates validity information for this message to the recipient MS.
    ///
    /// The ms_validity parameter is used to provide an MS with validity information associated with
    /// the received short message.
    MsValidity = 0x1204,
    /// Request an MS alert signal be invoked on message delivery.
    ///
    /// The alert_on_message_delivery parameter is set to instruct a MS to alert the user (in a MS
    /// implementation specific manner) when the short message arrives at the MS.
    AlertOnMessageDelivery = 0x130C,
    /// The MS user’s reply method to an SMS delivery message
    /// received from the network is indicated and controlled by
    /// this parameter.
    ///
    /// The its_reply_type parameter is a required parameter for the CDMA Interactive Teleservice
    /// as defined by the Korean PCS carriers \[KORITS\]. It indicates and controls the MS user’s
    /// reply method to an SMS delivery message received from the ESME.
    ItsReplyType = 0x1380,
    /// Session control information for Interactive Teleservice.
    ///
    /// The its_session_info parameter is a required parameter for the CDMA Interactive Teleservice
    /// as defined by the Korean PCS carriers \[KORITS\]. It contains control information for the
    /// interactive session between an MS and an ESME.
    ItsSessionInfo = 0x1383,
    /// Tag not recognized by this version; stores the raw value.
    ///
    /// This variant allows handling of future extensions or vendor-specific
    /// TLVs not currently defined. It preserves the raw tag value for processing
    /// or logging purposes.
    Other(u16),
}
