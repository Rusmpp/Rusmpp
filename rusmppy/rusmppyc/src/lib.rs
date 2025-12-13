use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult, Python,
};
use pyo3_stub_gen::define_stub_info_gatherer;

use crate::generated::add_classes;

mod client;
mod encoder;
mod error;
mod event;
mod exception;
mod generated;
mod generated_impl;
mod io;

#[pymodule]
fn rusmppyc(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<client::Client>()?;
    m.add_class::<event::Event>()?;
    m.add_class::<event::Events>()?;
    m.add_class::<error::Error>()?;

    m.add_class::<encoder::Encoder>()?;
    m.add_class::<encoder::Gsm7BitUnpacked>()?;
    m.add_class::<encoder::Ucs2>()?;
    m.add_class::<encoder::Latin1>()?;

    m.add_class::<encoder::Gsm7BitAlphabet>()?;
    m.add_class::<encoder::Gsm7BitDefaultAlphabet>()?;

    // add all auto generated structs and enums
    add_classes(m)?;

    use exception::*;
    m.add("RusmppycException", py.get_type::<RusmppycException>())?;
    m.add("ConnectException", py.get_type::<ConnectException>())?;
    m.add(
        "ConnectionClosedException",
        py.get_type::<ConnectionClosedException>(),
    )?;
    m.add("IoException", py.get_type::<IoException>())?;
    m.add("EncodeException", py.get_type::<EncodeException>())?;
    m.add("DecodeException", py.get_type::<DecodeException>())?;
    m.add(
        "ResponseTimeoutException",
        py.get_type::<ResponseTimeoutException>(),
    )?;
    m.add(
        "UnexpectedResponseException",
        py.get_type::<UnexpectedResponseException>(),
    )?;
    m.add(
        "UnsupportedInterfaceVersionException",
        py.get_type::<UnsupportedInterfaceVersionException>(),
    )?;
    m.add("ValueException", py.get_type::<ValueException>())?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
