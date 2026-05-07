use std::{pin::Pin, sync::Arc};

use futures::{Stream, StreamExt};
use pyo3::{
    exceptions::PyStopAsyncIteration, pyclass, pymethods, Bound, PyAny, PyRef, PyResult, Python,
};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum;
use tokio::sync::RwLock;

use crate::error::Error;

/// `SMPP` event.
///
/// Events are sent from the open connection through the event stream.
#[pyclass]
#[gen_stub_pyclass_complex_enum]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /// A command was received from the server.
    Incoming(crate::generated::Command),
    /// An error occurred.
    Error(Error),
}

impl From<rusmppc::event::Event> for Event {
    fn from(event: rusmppc::event::Event) -> Self {
        match event {
            rusmppc::event::Event::Incoming(command) => {
                Event::Incoming(crate::generated::Command::from(command))
            }
            rusmppc::event::Event::Error(error) => Event::Error(Error::from(error)),
        }
    }
}

/// An async stream of `Event`s.
///
/// This class represents a stream of events that can be iterated over asynchronously using `async for`.
#[pyclass]
pub struct Events {
    inner: Arc<RwLock<Pin<Box<dyn Stream<Item = Event> + Send + Sync + Unpin + 'static>>>>,
}

impl Events {
    /// Creates a new [`Events`] instance from a stream of events.
    pub fn new(stream: Pin<Box<dyn Stream<Item = Event> + Send + Sync + Unpin + 'static>>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(stream)),
        }
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[pymethods]
impl Events {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        Ok(slf)
    }

    fn __anext__<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let events = self.clone();

        let fut = future_into_py(py, async move {
            events
                .inner
                .write()
                .await
                .next()
                .await
                .ok_or(PyStopAsyncIteration::new_err("Stream exhausted"))
        })?;

        Ok(fut)
    }
}
