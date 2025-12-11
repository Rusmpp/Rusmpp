//! `SMPP` Fields.

/// Every field defined in the `SMPP` protocol and extra fields defined in this library.
///
/// Used for verbose error handling while decoding invalid pdus, if the `verbose` feature is enabled.
///
/// # Example
///
/// ```rust
/// # #[cfg(all(feature = "verbose", feature = "alloc"))]
/// # {
/// # use rusmpp_core::{command::owned::Command, decode::bytes::DecodeWithLength, fields::SmppField};
/// # use bytes::BytesMut;
/// // bind_transmitter bytes
/// // The `password` field is not null terminated.
/// // The `decode` method will return an error with
/// // the `SmppField::password` field as a source in
/// // the sources tree.
/// let mut bytes = BytesMut::from(&[
///     // Header
///     0x00, 0x00, 0x00, 0x2E, // Command Length (46 bytes total)
///     0x00, 0x00, 0x00, 0x02, // Command ID (bind_transmitter)
///     0x00, 0x00, 0x00, 0x00, // Command Status (0 - OK)
///     0x00, 0x00, 0x00, 0x01, // Sequence Number (1)
///
///     // system_id: "SMPP3TEST\0"
///     0x53, 0x4D, 0x50, 0x50, 0x33, 0x54, 0x45, 0x53, 0x54, 0x00,
///     // password: "secret08" WRONG! not null terminated!
///     0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x30, 0x38,
///     // system_type: "SUBMIT1"
///     0x53, 0x55, 0x42, 0x4D, 0x49, 0x54, 0x31, 0x00,
///     // interface_version
///     0x50,
///     // addr_ton
///     0x01,
///     // addr_npi
///     0x01,
///     // addr_range
///     0x00,
/// ][..]);
///
/// let error = Command::decode(&mut bytes.split_off(4), 46 - 4).unwrap_err();
///
/// assert!(error.field_exists(SmppField::password));
///
/// // Knowing that the `password` field is invalid,
/// // we can respond with `ESME_RINVPASWD` (Invalid Password).
/// # }
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SmppField {
    addr,
    addr_npi,
    addr_ton,
    address_range,
    area,
    data_coding,
    dest_addr_npi,
    dest_addr_ton,
    dest_address,
    destination_addr,
    dl_name,
    encoding_content_type,
    error_code,
    error_status_code,
    esm_class,
    esme_addr,
    esme_addr_npi,
    esme_addr_ton,
    final_date,
    format,
    id,
    interface_version,
    message_id,
    message_payload,
    message_state,
    ms_availability_status,
    network_type,
    no_unsuccess,
    number_of_dests,
    number_of_time_units,
    password,
    pdu,
    priority_flag,
    protocol_id,
    registered_delivery,
    replace_if_present_flag,
    sc_interface_version,
    schedule_delivery_time,
    sequence_number,
    service_type,
    session_number,
    short_message,
    sm_default_msg_id,
    sm_length,
    source_addr,
    source_addr_npi,
    source_addr_ton,
    status,
    system_id,
    system_type,
    tag,
    tlvs,
    type_of_network,
    unit,
    units_of_time,
    unsuccess_sme,
    user_message_reference,
    validity_behavior,
    validity_information,
    validity_period,
    value,
    value_length,
    udh_length,
    udh_id,
    udh_value,
}
