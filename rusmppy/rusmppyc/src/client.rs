#![allow(clippy::too_many_arguments)]

// XXX: Current limitations:
// - Sending requests without waiting for a response is not supported. In rusmppc:
//   client.no_wait().submit_sm(...)
// - Sending requests with a custom timeout or without a timeout is not supported. In rusmppc:
//   client.response_timeout(...).submit_sm(...)
//   client.no_response_timeout().submit_sm(...)

use std::time::Duration;

use futures::StreamExt;
use pyo3::{pyclass, pymethods, types::PyType, Bound, Py, PyAny, PyResult, Python};
use pyo3_async_runtimes::tokio::future_into_py;
use rusmpp::{
    extra::concatenation::SubmitSmMultipartExt,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, DeliverSmResp, SubmitSm},
    tlvs::MessageSubmissionRequestTlvValue,
    types::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::ServiceType,
};
use rusmppc::ConnectionBuilder;

use crate::{
    event::{Event, Events},
    exception::{Exception, ValueExceptionExt},
    io::IO,
};

#[pyclass]
#[derive(Clone)]
pub struct Client {
    inner: rusmppc::Client,
}

#[pymethods]
impl Client {
    #[classmethod]
    #[pyo3(signature=(url,
        max_command_length=4096,
        enquire_link_response_timeout=2000,
        enquire_link_interval=5000,
        auto_enquire_link_response=true,
        response_timeout=2000,
        interface_version_check=true))]
    fn connect<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        url: String,
        max_command_length: usize,
        enquire_link_response_timeout: u64,
        enquire_link_interval: Option<u64>,
        auto_enquire_link_response: bool,
        response_timeout: Option<u64>,
        interface_version_check: bool,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let builder = ConnectionBuilder::new()
                .max_command_length(max_command_length)
                .enquire_link_response_timeout(Duration::from_millis(enquire_link_response_timeout))
                .with_enquire_link_interval(enquire_link_interval.map(Duration::from_millis))
                .with_auto_enquire_link_response(auto_enquire_link_response)
                .with_response_timeout(response_timeout.map(Duration::from_millis))
                .with_interface_version_check(interface_version_check);

            let (client, events) = builder.connect(url).await.map_err(Exception::from)?;

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[classmethod]
    #[pyo3(signature=(read,
        write,
        max_command_length=4096,
        enquire_link_response_timeout=2000,
        enquire_link_interval=5000,
        auto_enquire_link_response=true,
        response_timeout=2000,
        interface_version_check=true))]
    fn connected<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        read: Py<PyAny>,
        write: Py<PyAny>,
        max_command_length: usize,
        enquire_link_response_timeout: u64,
        enquire_link_interval: Option<u64>,
        auto_enquire_link_response: bool,
        response_timeout: Option<u64>,
        interface_version_check: bool,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let read_write = (read, write).into_tokio_async_read_and_write();

            let builder = ConnectionBuilder::new()
                .max_command_length(max_command_length)
                .enquire_link_response_timeout(Duration::from_millis(enquire_link_response_timeout))
                .with_enquire_link_interval(enquire_link_interval.map(Duration::from_millis))
                .with_auto_enquire_link_response(auto_enquire_link_response)
                .with_response_timeout(response_timeout.map(Duration::from_millis))
                .with_interface_version_check(interface_version_check);

            let (client, events, connection) = builder.no_spawn().connected(read_write);

            // the read and write are python-futures, we spawn them with current locals
            let task_locals = Python::attach(pyo3_async_runtimes::tokio::get_current_locals)?;
            tokio::spawn(pyo3_async_runtimes::tokio::scope(task_locals, connection));

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transmitter<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindTransmitter::builder()
            .system_id(COctetString::from_string(system_id).map_value_err("system_id")?)
            .password(COctetString::from_string(password).map_value_err("password")?)
            .system_type(COctetString::from_string(system_type).map_value_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_string(address_range).map_value_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transmitter(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransmitterResp::from(response))
        })
    }

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_receiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindReceiver::builder()
            .system_id(COctetString::from_string(system_id).map_value_err("system_id")?)
            .password(COctetString::from_string(password).map_value_err("password")?)
            .system_type(COctetString::from_string(system_type).map_value_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_string(address_range).map_value_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_receiver(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindReceiverResp::from(response))
        })
    }

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transceiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindTransceiver::builder()
            .system_id(COctetString::from_string(system_id).map_value_err("system_id")?)
            .password(COctetString::from_string(password).map_value_err("password")?)
            .system_type(COctetString::from_string(system_type).map_value_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_string(address_range).map_value_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transceiver(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransceiverResp::from(response))
        })
    }

    #[pyo3(signature=(
        submit_sm=None,
        service_type=String::new(),
        source_addr_ton=crate::generated::Ton::Unknown(),
        source_addr_npi=crate::generated::Npi::Unknown(),
        source_addr=String::new(),
        dest_addr_ton=crate::generated::Ton::Unknown(),
        dest_addr_npi=crate::generated::Npi::Unknown(),
        destination_addr=String::new(),
        esm_class=crate::generated::EsmClass::default_(),
        protocol_id=u8::default(),
        priority_flag=u8::default(),
        schedule_delivery_time=String::new(),
        validity_period=String::new(),
        registered_delivery=crate::generated::RegisteredDelivery::default_(),
        replace_if_present_flag=crate::generated::ReplaceIfPresentFlag::DoNotReplace(),
        data_coding=crate::generated::DataCoding::McSpecific(),
        sm_default_msg_id=u8::default(),
        short_message=Vec::new(),
        tlvs=Vec::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn submit_sm<'p>(
        &self,
        py: Python<'p>,
        submit_sm: Option<crate::generated::SubmitSm>,
        service_type: String,
        source_addr_ton: crate::generated::Ton,
        source_addr_npi: crate::generated::Npi,
        source_addr: String,
        dest_addr_ton: crate::generated::Ton,
        dest_addr_npi: crate::generated::Npi,
        destination_addr: String,
        esm_class: crate::generated::EsmClass,
        protocol_id: u8,
        priority_flag: u8,
        schedule_delivery_time: String,
        validity_period: String,
        registered_delivery: crate::generated::RegisteredDelivery,
        replace_if_present_flag: crate::generated::ReplaceIfPresentFlag,
        data_coding: crate::generated::DataCoding,
        sm_default_msg_id: u8,
        short_message: Vec<u8>,
        tlvs: Vec<crate::generated::MessageSubmissionRequestTlvValue>,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = match submit_sm {
            Some(submit_sm) => SubmitSm::try_from(submit_sm).map_value_err("submit_sm")?,
            None => {
                let builder = SubmitSm::builder()
                    .service_type(ServiceType::new(
                        COctetString::from_string(service_type).map_value_err("service_type")?,
                    ))
                    .source_addr_ton(source_addr_ton.into())
                    .source_addr_npi(source_addr_npi.into())
                    .source_addr(
                        COctetString::from_string(source_addr).map_value_err("source_addr")?,
                    )
                    .dest_addr_ton(dest_addr_ton.into())
                    .dest_addr_npi(dest_addr_npi.into())
                    .destination_addr(
                        COctetString::from_string(destination_addr)
                            .map_value_err("destination_addr")?,
                    )
                    .esm_class(esm_class.into())
                    .protocol_id(protocol_id)
                    .priority_flag(priority_flag.into())
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::from_string(schedule_delivery_time)
                            .map_value_err("schedule_delivery_time")?,
                    )
                    .validity_period(
                        EmptyOrFullCOctetString::from_string(validity_period)
                            .map_value_err("validity_period")?,
                    )
                    .registered_delivery(registered_delivery.into())
                    .replace_if_present_flag(replace_if_present_flag.into())
                    .data_coding(data_coding.into())
                    .sm_default_msg_id(sm_default_msg_id)
                    .short_message(
                        OctetString::from_vec(short_message).map_value_err("short_message")?,
                    )
                    .tlvs(
                        tlvs.into_iter()
                            .map(MessageSubmissionRequestTlvValue::try_from)
                            .collect::<Result<Vec<_>, _>>()
                            .map_value_err("tlvs")?,
                    );

                let pdu = builder.build();

                tracing::debug!(?pdu, "Built Pdu");

                pdu
            }
        };

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .submit_sm(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::SubmitSmResp::from(response))
        })
    }

    #[pyo3(signature=(sequence_number, message_id=String::new(), status=crate::generated::CommandStatus::EsmeRok()))]
    fn deliver_sm_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        message_id: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = DeliverSmResp::builder()
            .message_id(COctetString::from_string(message_id).map_value_err("message_id")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .deliver_sm_resp(sequence_number, pdu)
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(status=crate::generated::CommandStatus::EsmeRok()))]
    fn unbind<'p>(
        &self,
        py: Python<'p>,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .unbind()
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(sequence_number, status=crate::generated::CommandStatus::EsmeRok()))]
    fn unbind_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .unbind_resp(sequence_number)
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(sequence_number, status=crate::generated::CommandStatus::EsmeRok()))]
    fn generic_nack<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .generic_nack(sequence_number)
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    fn close<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.close().await.map_err(Exception::from)?;

            Ok(())
        })
    }

    fn closed<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.closed().await;

            Ok(())
        })
    }

    fn is_closed(&self) -> PyResult<bool> {
        Ok(self.inner.is_closed())
    }

    fn is_active(&self) -> PyResult<bool> {
        Ok(self.inner.is_active())
    }

    #[classmethod]
    #[pyo3(signature=(
        short_message=String::new(),
        max_short_message_size=140,
        reference=0,
        service_type=String::new(),
        source_addr_ton=crate::generated::Ton::Unknown(),
        source_addr_npi=crate::generated::Npi::Unknown(),
        source_addr=String::new(),
        dest_addr_ton=crate::generated::Ton::Unknown(),
        dest_addr_npi=crate::generated::Npi::Unknown(),
        destination_addr=String::new(),
        protocol_id=u8::default(),
        priority_flag=u8::default(),
        schedule_delivery_time=String::new(),
        validity_period=String::new(),
        registered_delivery=crate::generated::RegisteredDelivery::default_(),
        replace_if_present_flag=crate::generated::ReplaceIfPresentFlag::DoNotReplace(),
        sm_default_msg_id=u8::default(),
        tlvs=Vec::new()))]
    fn submit_sm_multipart<'p>(
        _cls: &'p Bound<'p, PyType>,
        short_message: String,
        max_short_message_size: usize,
        reference: u8,
        service_type: String,
        source_addr_ton: crate::generated::Ton,
        source_addr_npi: crate::generated::Npi,
        source_addr: String,
        dest_addr_ton: crate::generated::Ton,
        dest_addr_npi: crate::generated::Npi,
        destination_addr: String,
        protocol_id: u8,
        priority_flag: u8,
        schedule_delivery_time: String,
        validity_period: String,
        registered_delivery: crate::generated::RegisteredDelivery,
        replace_if_present_flag: crate::generated::ReplaceIfPresentFlag,
        sm_default_msg_id: u8,
        tlvs: Vec<crate::generated::MessageSubmissionRequestTlvValue>,
    ) -> PyResult<Vec<crate::generated::SubmitSm>> {
        let builder = SubmitSm::builder()
            .service_type(ServiceType::new(
                COctetString::from_string(service_type).map_value_err("service_type")?,
            ))
            .source_addr_ton(source_addr_ton.into())
            .source_addr_npi(source_addr_npi.into())
            .source_addr(COctetString::from_string(source_addr).map_value_err("source_addr")?)
            .dest_addr_ton(dest_addr_ton.into())
            .dest_addr_npi(dest_addr_npi.into())
            .destination_addr(
                COctetString::from_string(destination_addr).map_value_err("destination_addr")?,
            )
            .protocol_id(protocol_id)
            .priority_flag(priority_flag.into())
            .schedule_delivery_time(
                EmptyOrFullCOctetString::from_string(schedule_delivery_time)
                    .map_value_err("schedule_delivery_time")?,
            )
            .validity_period(
                EmptyOrFullCOctetString::from_string(validity_period)
                    .map_value_err("validity_period")?,
            )
            .registered_delivery(registered_delivery.into())
            .replace_if_present_flag(replace_if_present_flag.into())
            .sm_default_msg_id(sm_default_msg_id)
            .tlvs(
                tlvs.into_iter()
                    .map(MessageSubmissionRequestTlvValue::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_value_err("tlvs")?,
            );

        let pdu = builder.build();

        tracing::debug!(?pdu, "Built Pdu");

        let multipart = pdu
            .multipart(&short_message)
            .reference_u8(reference)
            .max_short_message_size(max_short_message_size)
            .build()
            // TODO: the encoder and the encoder exception
            // TODO: handle error properly
            .expect("TODO: remove this. Failed to create multipart")
            .into_iter()
            .map(From::from)
            .collect();

        Ok(multipart)
    }
}
