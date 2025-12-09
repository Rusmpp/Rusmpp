use rusmpp_macros::Rusmpp;

/// Represents the means by which an ESME or MC sends an error code
/// to its peer.
///
/// This field is only relevant in response PDUs. Thus PDU requests always have this
/// field set to NULL (0x00000000).
///
/// The [`CommandStatus`] field of a `SMPP` message response indicates the success or failure of
/// a `SMPP` request. It is relevant only in the `SMPP` response message and should be set to
/// NULL in `SMPP` request messages.
///
/// The `SMPP` Error status codes are returned by the MC in the [`CommandStatus`] field of the
/// `SMPP` message header and in the error_status_code field of a submit_multi_resp message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[repr(u32)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum CommandStatus {
    /// No Error.
    ///
    /// Specified in a response PDU to indicate
    /// the success of the corresponding request
    /// PDU.
    #[default]
    EsmeRok = 0x00000000,
    /// Message Length is invalid.
    ///
    /// short_message field or
    /// message_payload TLV has an invalid
    /// length (usually too long for the given MC
    /// or underlying network technology).
    EsmeRinvmsglen = 0x00000001,
    /// Command Length is invalid.
    ///
    /// PDU length is considered invalid, either
    /// because the value is too short or too
    /// large for the given PDU.
    EsmeRinvcmdlen = 0x00000002,
    /// Invalid Command ID.
    ///
    /// Command ID is not recognized, either
    /// because the operation is not supported
    /// or unknown.
    EsmeRinvcmdid = 0x00000003,
    /// Incorrect BIND Status for given command.
    ///
    /// PDU has been sent in the wrong session
    /// state. E.g. sending a submit_sm without
    /// first establishing a Bound_TX session
    /// state.
    EsmeRinvbndsts = 0x00000004,
    /// ESME Already in Bound State.
    ///
    /// A bind request has been issued within a
    /// session that is already bound.
    EsmeRalybnd = 0x00000005,
    /// Invalid Priority Flag.
    ///
    /// Priority flag contains an illegal or
    /// unsupported value.
    EsmeRinvprtflg = 0x00000006,
    /// Invalid Registered Delivery Flag.
    ///
    /// Registered field contains an invalid
    /// setting.
    EsmeRinvregdlvflg = 0x00000007,
    /// System Error.
    ///
    /// MC system error indicating that all or part
    /// of the MC is currently unavailable. This
    /// can be returned in any response PDU.
    EsmeRsyserr = 0x00000008,
    /// Invalid Source Address.
    ///
    /// Source address of message is
    /// considered invalid. Usually this is
    /// because the field is either too long or
    /// contains invalid characters.
    EsmeRinvsrcadr = 0x0000000A,
    /// Invalid Destination Address.
    ///
    /// Destination address of message is
    /// considered invalid. Usually this is
    /// because the field is either zero length,
    /// too long or contains invalid characters.
    EsmeRinvdstadr = 0x0000000B,
    /// Message ID is invalid.
    ///
    /// Message ID specified in cancel_sm,
    /// query_sm or other operations is invalid.
    EsmeRinvmsgid = 0x0000000C,
    /// Bind Failed.
    ///
    /// A generic failure scenario for a bind
    /// attempt. This may be due to a
    /// provisioning error, incorrect password or
    /// other reason. A MC will typically return
    /// this error for an invalid system_id,
    /// system_type, password or other attribute
    /// that may cause a bind failure.
    EsmeRbindfail = 0x0000000D,
    /// Invalid Password.
    ///
    /// Password field in bind PDU is invalid.
    /// This is usually returned when the length
    /// is too short or too long. It is not supposed
    /// to be returned when the ESME has
    /// specified the incorrect password.
    EsmeRinvpaswd = 0x0000000E,
    /// Invalid System ID.
    ///
    /// The System ID field in bind PDU is
    /// invalid. This is usually returned when the
    /// length is too short or too long. It is not
    /// supposed to be returned when the ESME
    /// has specified the incorrect system id.
    EsmeRinvsysid = 0x0000000F,
    /// Cancel SM Failed.
    ///
    /// Generic failure error for cancel_sm
    /// operation.
    EsmeRcancelfail = 0x00000011,
    /// Replace SM Failed.
    ///
    /// Generic failure for replace_sm operation.
    EsmeRreplacefail = 0x00000013,
    /// Message Queue Full.
    ///
    /// Used to indicate a resource error within
    /// the MC. This may be interpreted as the
    /// maximum number of messages
    /// addressed to a single destination or a
    /// global maximum of undelivered
    /// messages within the MC.
    EsmeRmsgqful = 0x00000014,
    /// Invalid Service Type.
    ///
    /// Service type is rejected either because it
    /// is not recognized by the MC or because
    /// its length is not within the defined range.
    EsmeRinvsertyp = 0x00000015,
    /// Invalid number of destinations.
    ///
    /// The number_of_dests field in the
    /// submit_multi PDU is invalid.
    EsmeRinvnumdests = 0x00000033,
    /// Invalid Distribution List name.
    ///
    /// The dl_name field specified in the
    /// submit_multi PDU is either invalid, or
    /// non-existent.
    EsmeRinvdlname = 0x00000034,
    /// Destination flag is invalid (submit_multi).
    ///
    /// The dest_flag field in the submit_multi
    /// PDU has been encoded with an invalid
    /// setting.
    EsmeRinvdestflag = 0x00000040,
    /// Submit w/replace functionality has been
    /// requested where it is either unsupported
    /// or inappropriate for the particular MC.
    /// This can typically occur with submit_multi
    /// where the context of “replace if present”
    /// is often a best effort operation and MCs
    /// may not support the feature in
    /// submit_multi.
    ///
    /// Another reason for returning this error
    /// would be where the feature has been
    /// denied to an ESME.
    EsmeRinvsubrep = 0x00000042,
    /// Invalid esm_class field data.
    ///
    /// The esm_class field has an unsupported
    /// setting.
    EsmeRinvesmclass = 0x00000043,
    /// Cannot Submit to Distribution List.
    ///
    /// Distribution lists are not supported, are
    /// denied or unavailable.
    EsmeRcntsubdl = 0x00000044,
    /// submit_sm, data_sm or submit_multi
    /// failed.
    ///
    /// Generic failure error for submission
    /// operations.
    EsmeRsubmitfail = 0x00000045,
    /// Invalid Source address TON.
    ///
    /// The source TON of the message is either
    /// invalid or unsupported.
    EsmeRinvsrcton = 0x00000048,
    /// Invalid Source address NPI.
    ///
    /// The source NPI of the message is either
    /// invalid or unsupported.
    EsmeRinvsrcnpi = 0x00000049,
    /// Invalid Destination address TON.
    ///
    /// The destination TON of the message is
    /// either invalid or unsupported.
    EsmeRinvdstton = 0x00000050,
    /// Invalid Destination address NPI.
    ///
    /// The destination NPI of the message is
    /// either invalid or unsupported.
    EsmeRinvdstnpi = 0x00000051,
    /// Invalid system_type field.
    ///
    /// The System type of bind PDU has an
    /// incorrect length or contains illegal
    /// characters.
    EsmeRinvsystyp = 0x00000053,
    /// Invalid replace_if_present flag.
    ///
    /// The replace_if_present flag has been
    /// encoded with an invalid or unsupported
    /// setting.
    EsmeRinvrepflag = 0x00000054,
    /// Invalid number of messages.
    EsmeRinvnummsgs = 0x00000055,
    /// Throttling error (ESME has exceeded
    /// allowed message limits).
    ///
    /// This type of error is usually returned
    /// where an ESME has exceeded a
    /// predefined messaging rate restriction
    /// applied by the operator.
    EsmeRthrottled = 0x00000058,
    /// Invalid Scheduled Delivery Time.
    ///
    /// Scheduled delivery time is either the
    /// incorrect length or is invalid.
    EsmeRinvsched = 0x00000061,
    /// Invalid message validity period (Expiry
    /// time).
    ///
    /// Expiry time is either the incorrect length
    /// or is invalid.
    EsmeRinvexpiry = 0x00000062,
    /// Predefined Message ID is Invalid or
    /// specified predefined message was not
    /// found.
    ///
    /// The default (pre-defined) message id is
    /// either invalid or refers to a non-existent
    /// pre-defined message.
    EsmeRinvdftmsgid = 0x00000063,
    /// ESME Receiver Temporary App Error
    /// Code.
    ///
    /// Rx or Trx ESME is unable to process a
    /// delivery due to a temporary problem and
    /// is requesting that the message be retried
    /// at some future point.
    EsmeRxTAppn = 0x00000064,
    /// ESME Receiver Permanent App Error
    /// Code.
    ///
    /// Rx or Trx ESME is unable to process a
    /// delivery due to a permanent problem
    /// relating to the given destination address
    /// and is requesting that the message and
    /// all other messages queued to the same
    /// destination should NOT be retried any
    /// further.
    EsmeRxPAppn = 0x00000065,
    /// ESME Receiver Reject Message Error
    /// Code.
    ///
    /// Rx or Trx ESME is unable to process a
    /// delivery due to a problem relating to the
    /// given message and is requesting that the
    /// message is rejected and not retried. This
    /// does not affect other messages queued
    /// for the same ESME or destination
    /// address.
    EsmeRxRAppn = 0x00000066,
    /// query_sm request failed.
    ///
    /// Generic failure scenario for a query
    /// request.
    EsmeRqueryfail = 0x00000067,
    /// Error in the optional part of the PDU
    /// Body.
    ///
    /// Decoding of TLVs (Optional Parameters)
    /// has resulted in one of the following
    /// scenarios:
    ///  • PDU decoding completed with 1-
    ///    3 octets of data remaining,
    ///    indicating a corrupt PDU.
    ///
    ///  • A TLV indicated a length that
    ///    was not present in the remaining
    ///    PDU data (e.g. a TLV specifying
    ///    a length of 10 where only 6
    ///    octets of PDU data remain).
    EsmeRinvtlvstream = 0x000000C0,
    /// TLV not allowed.
    ///
    /// A TLV has been used in an invalid
    /// context, either inappropriate or
    /// deliberately rejected by the operator.
    EsmeRtlvnotallwd = 0x000000C1,
    /// Invalid Parameter Length.
    ///
    /// A TLV has specified a length that is
    /// considered invalid
    EsmeRinvtlvlen = 0x000000C2,
    /// Expected TLV missing.
    ///
    /// A mandatory TLV such as the
    /// message_payload TLV within a data_sm
    /// PDU is missing.
    EsmeRmissingtlv = 0x000000C3,
    /// Invalid TLV Value.
    ///
    /// The data content of a TLV is invalid and
    /// cannot be decoded.
    EsmeRinvtlvval = 0x000000C4,
    /// Transaction Delivery Failure.
    ///
    /// A data_sm or submit_sm operation
    /// issued in transaction mode has resulted
    /// in a failed delivery.
    EsmeRdeliveryfailure = 0x000000FE,
    /// Unknown Error.
    ///
    /// Some unexpected error has occurred.
    EsmeRunknownerr = 0x000000FF,
    /// ESME Not authorized to use specified
    /// service_type.
    ///
    /// Specific service_type has been denied
    /// for use by the given ESME.
    EsmeRsertypunauth = 0x00000100,
    /// ESME Prohibited from using specified
    /// operation.
    ///
    /// The PDU request was recognized but is
    /// denied to the ESME.
    EsmeRprohibited = 0x00000101,
    /// Specified service_type is unavailable.
    ///
    /// Due to a service outage within the MC, a
    /// service is unavailable.
    EsmeRsertypunavail = 0x00000102,
    /// Specified service_type is denied.
    ///
    /// Due to inappropriate message content
    /// wrt. the selected service_type.
    EsmeRsertypdenied = 0x00000103,
    /// Invalid Data Coding Scheme.
    ///
    /// Specified DCS is invalid or MC does not
    /// support it.
    EsmeRinvdcs = 0x00000104,
    /// Source Address Sub unit is Invalid.
    EsmeRinvsrcaddrsubunit = 0x00000105,
    /// Destination Address Sub unit is Invalid.
    EsmeRinvdstaddrsubunit = 0x00000106,
    /// Broadcast Frequency Interval is invalid.
    ///
    /// Specified value is either invalid or not
    /// supported.
    EsmeRinvbcastfreqint = 0x00000107,
    /// Broadcast Alias Name is invalid.
    ///
    /// Specified value has an incorrect length
    /// or contains invalid/unsupported
    /// characters.
    EsmeRinvbcastaliasName = 0x00000108,
    /// Broadcast Area Format is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastareafmt = 0x00000109,
    /// Number of Broadcast Areas is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvnumbcastAreas = 0x0000010A,
    /// Broadcast Content Type is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastcnttype = 0x0000010B,
    /// Broadcast Message Class is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastmsgclass = 0x0000010C,
    /// broadcast_sm operation failed.
    EsmeRbcastfail = 0x0000010D,
    /// query_broadcast_sm operation failed.
    EsmeRbcastqueryfail = 0x0000010E,
    /// cancel_broadcast_sm operation failed.
    EsmeRbcastcancelfail = 0x0000010F,
    /// Number of Repeated Broadcasts is
    /// invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastRep = 0x00000110,
    /// Broadcast Service Group is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastsrvgrp = 0x00000111,
    /// Broadcast Channel Indicator is invalid.
    ///
    /// Specified value violates protocol or is
    /// unsupported.
    EsmeRinvbcastchanind = 0x00000112,
    Other(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let status: u32 = CommandStatus::EsmeRok.into();
        assert_eq!(status, 0x00000000);

        let status: u32 = CommandStatus::Other(0x00000115).into();
        assert_eq!(status, 0x00000115);
    }

    #[test]
    fn from() {
        let status = CommandStatus::from(0x00000000);
        assert_eq!(status, CommandStatus::EsmeRok);

        let status = CommandStatus::from(0x00000115);
        assert_eq!(status, CommandStatus::Other(0x00000115));
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<CommandStatus>();
        crate::tests::borrowed::encode_decode_test_instances::<CommandStatus>();
    }
}
