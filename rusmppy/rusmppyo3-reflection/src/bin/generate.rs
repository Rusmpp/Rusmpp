//! Run with
//!
//! ```bash
//! cargo run -p rusmppyo3-reflection --bin generate && cargo fmt
//! ```

use std::io::Write;

use rusmpp::{Command, CommandId, CommandStatus, Pdu, pdus::*, tlvs::*, values::*};
use rusmppyo3_reflection::generate::CodeGenerator;
use serde_generate::CodeGeneratorConfig;
use serde_reflection::{Tracer, TracerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracer = Tracer::new(TracerConfig::default());

    tracer.trace_simple_type::<Command>()?;
    tracer.trace_simple_type::<CommandId>()?;
    tracer.trace_simple_type::<CommandStatus>()?;
    tracer.trace_simple_type::<InterfaceVersion>()?;
    tracer.trace_simple_type::<Npi>()?;
    tracer.trace_simple_type::<Ton>()?;
    tracer.trace_simple_type::<Pdu>()?;
    tracer.trace_simple_type::<Ansi41Specific>()?;
    tracer.trace_simple_type::<DataCoding>()?;
    tracer.trace_simple_type::<DestFlag>()?;
    tracer.trace_simple_type::<DestAddressValue>()?;
    tracer.trace_simple_type::<DestAddress>()?;
    tracer.trace_simple_type::<GsmFeatures>()?;
    tracer.trace_simple_type::<IntermediateNotification>()?;
    tracer.trace_simple_type::<MCDeliveryReceipt>()?;
    tracer.trace_simple_type::<MessageState>()?;
    tracer.trace_simple_type::<MessageType>()?;
    tracer.trace_simple_type::<MessagingMode>()?;
    tracer.trace_simple_type::<ReplaceIfPresentFlag>()?;
    tracer.trace_simple_type::<SmeOriginatedAcknowledgement>()?;
    tracer.trace_simple_type::<TlvTag>()?;
    tracer.trace_simple_type::<TlvValue>()?;
    tracer.trace_simple_type::<Tlv>()?;
    tracer.trace_simple_type::<AddrSubunit>()?;
    tracer.trace_simple_type::<AlertOnMessageDelivery>()?;
    tracer.trace_simple_type::<BearerType>()?;
    tracer.trace_simple_type::<BroadcastAreaFormat>()?;
    tracer.trace_simple_type::<BroadcastAreaSuccess>()?;
    tracer.trace_simple_type::<BroadcastChannelIndicator>()?;
    tracer.trace_simple_type::<BroadcastMessageClass>()?;
    tracer.trace_simple_type::<CongestionState>()?;
    tracer.trace_simple_type::<DeliveryFailureReason>()?;
    tracer.trace_simple_type::<DestAddrNpResolution>()?;
    tracer.trace_simple_type::<DisplayTime>()?;
    tracer.trace_simple_type::<DpfResult>()?;
    tracer.trace_simple_type::<EncodingContentType>()?;
    tracer.trace_simple_type::<ErrorCodeNetworkType>()?;
    tracer.trace_simple_type::<Indicator>()?;
    tracer.trace_simple_type::<ItsReplyType>()?;
    tracer.trace_simple_type::<LanguageIndicator>()?;
    tracer.trace_simple_type::<MoreMessagesToSend>()?;
    tracer.trace_simple_type::<MsAvailabilityStatus>()?;
    tracer.trace_simple_type::<MsValidityBehavior>()?;
    tracer.trace_simple_type::<NetworkType>()?;
    tracer.trace_simple_type::<NumberOfMessages>()?;
    tracer.trace_simple_type::<PayloadType>()?;
    tracer.trace_simple_type::<Presentation>()?;
    tracer.trace_simple_type::<PrivacyIndicator>()?;
    tracer.trace_simple_type::<Screening>()?;
    tracer.trace_simple_type::<SetDpf>()?;
    tracer.trace_simple_type::<SubaddressTag>()?;
    tracer.trace_simple_type::<TypeOfMessage>()?;
    tracer.trace_simple_type::<TypeOfNetwork>()?;
    tracer.trace_simple_type::<UnitOfTime>()?;
    tracer.trace_simple_type::<UnitsOfTime>()?;
    tracer.trace_simple_type::<UssdServiceOp>()?;
    tracer.trace_simple_type::<BindReceiver>()?;
    tracer.trace_simple_type::<BindTransmitter>()?;
    tracer.trace_simple_type::<BindTransceiver>()?;
    tracer.trace_simple_type::<BindReceiverResp>()?;
    tracer.trace_simple_type::<BindTransmitterResp>()?;
    tracer.trace_simple_type::<BindTransceiverResp>()?;
    tracer.trace_simple_type::<AlertNotification>()?;
    tracer.trace_simple_type::<CancelSm>()?;
    tracer.trace_simple_type::<DataSm>()?;
    tracer.trace_simple_type::<DeliverSm>()?;
    tracer.trace_simple_type::<Outbind>()?;
    tracer.trace_simple_type::<QuerySm>()?;
    tracer.trace_simple_type::<QuerySmResp>()?;
    tracer.trace_simple_type::<ReplaceSm>()?;
    tracer.trace_simple_type::<DataSmResp>()?;
    tracer.trace_simple_type::<DeliverSmResp>()?;
    tracer.trace_simple_type::<SubmitSm>()?;
    tracer.trace_simple_type::<SubmitSmResp>()?;
    tracer.trace_simple_type::<SubmitMulti>()?;
    tracer.trace_simple_type::<SubmitMultiResp>()?;
    tracer.trace_simple_type::<BroadcastSm>()?;
    tracer.trace_simple_type::<BroadcastSmResp>()?;
    tracer.trace_simple_type::<QueryBroadcastSm>()?;
    tracer.trace_simple_type::<QueryBroadcastSmResp>()?;
    tracer.trace_simple_type::<CancelBroadcastSm>()?;
    tracer.trace_simple_type::<BroadcastRequestTlvValue>()?;
    tracer.trace_simple_type::<BroadcastResponseTlvValue>()?;
    tracer.trace_simple_type::<CancelBroadcastTlvValue>()?;
    tracer.trace_simple_type::<MessageDeliveryRequestTlvValue>()?;
    tracer.trace_simple_type::<MessageDeliveryResponseTlvValue>()?;
    tracer.trace_simple_type::<MessageSubmissionRequestTlvValue>()?;
    tracer.trace_simple_type::<MessageSubmissionResponseTlvValue>()?;
    tracer.trace_simple_type::<QueryBroadcastResponseTlvValue>()?;

    let registry = tracer.registry()?;

    let config = CodeGeneratorConfig::new(String::from("types")).with_serialization(false);

    let generator = CodeGenerator::new(&config)
        // It is okay to use `set_all` here because these are just data containers.
        // Converting to rusmpp types will take care of validation.
        .with_custom_derive_block(Some(String::from("#[::pyo3::pyclass(get_all, set_all)]")));

    let mut output = Vec::new();

    generator.output(&mut output, &registry)?;

    let mut file = std::fs::File::create("rusmppy/rusmppyc/src/generated.rs")?;

    file.write_all(&output)?;

    Ok(())
}
