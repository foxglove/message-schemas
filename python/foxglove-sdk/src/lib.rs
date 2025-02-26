use errors::PyFoxgloveError;
use foxglove::{
    Channel, ChannelBuilder, LogContext, McapWriter, McapWriterHandle, PartialMetadata, Schema,
};
use generated::channels;
use generated::schemas;
use log::LevelFilter;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use websocket_server::{
    start_server, PyCapability, PyChannelView, PyClient, PyMessageSchema, PyParameter,
    PyParameterType, PyParameterValue, PyRequest, PySchema, PyService, PyServiceSchema,
    PyStatusLevel, PyWebSocketServer,
};

mod errors;
mod generated;
mod websocket_server;

#[pyclass(module = "foxglove")]
struct BaseChannel(Arc<Channel>);

/// A writer for logging messages to an MCAP file.
///
/// Obtain an instance by calling :py:func:`open_mcap`.
///
/// This class may be used as a context manager, in which case the writer will
/// be closed when you exit the context.
///
/// If the writer is not closed by the time it is garbage collected, it will be
/// closed automatically, and any errors will be logged.
#[pyclass(name = "MCAPWriter", module = "foxglove")]
struct PyMcapWriter(Option<McapWriterHandle<BufWriter<File>>>);

impl Drop for PyMcapWriter {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            log::error!("Failed to close MCAP writer: {e}");
        }
    }
}

#[pymethods]
impl PyMcapWriter {
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __exit__(
        &mut self,
        _exc_type: Py<PyAny>,
        _exc_value: Py<PyAny>,
        _traceback: Py<PyAny>,
    ) -> PyResult<()> {
        self.close()
    }

    /// Close the MCAP writer.
    ///
    /// You may call this to explicitly close the writer. Note that the writer will be automatically
    /// closed for you when it is garbage collected, or when exiting the context manager.
    fn close(&mut self) -> PyResult<()> {
        if let Some(writer) = self.0.take() {
            writer.close().map_err(PyFoxgloveError::from)?;
        }
        Ok(())
    }
}

#[pymethods]
impl BaseChannel {
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, schema_name, schema_encoding=None, schema_data=None, metadata=None)
    )]
    fn new(
        topic: &str,
        message_encoding: &str,
        schema_name: Option<String>,
        schema_encoding: Option<String>,
        schema_data: Option<Vec<u8>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let schema = match (
            schema_name,
            schema_encoding.filter(|s| !s.is_empty()),
            schema_data.filter(|s| !s.is_empty()),
        ) {
            (Some(name), Some(encoding), Some(data)) => Some(Schema::new(name, encoding, data)),
            (_, None, None) => None,
            (_, None, Some(_)) => {
                return Err(PyValueError::new_err(
                    "Schema encoding must be provided if schema data is provided.",
                ));
            }
            (_, Some(_), None) => {
                return Err(PyValueError::new_err(
                    "Schema data must be provided if schema encoding is provided.",
                ));
            }
            _ => {
                return Err(PyValueError::new_err(
                    "Schema name must be provided if schema data or encoding is provided.",
                ));
            }
        };

        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema)
            .metadata(metadata.unwrap_or_default())
            .build()
            .map_err(PyFoxgloveError::from)?;

        Ok(BaseChannel(channel))
    }

    #[pyo3(signature = (msg, log_time=None, publish_time=None, sequence=None))]
    fn log(
        &self,
        msg: &[u8],
        log_time: Option<u64>,
        publish_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        let metadata = PartialMetadata {
            log_time,
            publish_time,
            sequence,
        };
        self.0.log_with_meta(msg, metadata);
        Ok(())
    }
}

/// Open a new mcap file for recording.
///
/// :param path: The path to the MCAP file. This file will be created and must not already exist.
/// :param allow_overwrite: Set this flag in order to overwrite an existing file at this path.
/// :rtype: :py:class:`MCAPWriter`
#[pyfunction]
#[pyo3(signature = (path, *, allow_overwrite = false))]
fn open_mcap(path: PathBuf, allow_overwrite: bool) -> PyResult<PyMcapWriter> {
    let file = if allow_overwrite {
        File::create(path)?
    } else {
        File::create_new(path)?
    };
    let writer = BufWriter::new(file);
    let handle = McapWriter::new()
        .create(writer)
        .map_err(PyFoxgloveError::from)?;
    Ok(PyMcapWriter(Some(handle)))
}

#[pyfunction]
fn get_channel_for_topic(topic: &str) -> PyResult<Option<BaseChannel>> {
    let channel = LogContext::global().get_channel_by_topic(topic);
    Ok(channel.map(BaseChannel))
}

// Not public. Re-exported in a wrapping function.
#[pyfunction]
fn enable_logging(level: &str) -> PyResult<()> {
    let level = match level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => return Err(PyErr::new::<PyValueError, _>("Invalid log level")),
    };

    log::set_max_level(level);
    Ok(())
}

// Not public. Re-exported in a wrapping function.
#[pyfunction]
fn disable_logging() -> PyResult<()> {
    log::set_max_level(LevelFilter::Off);
    Ok(())
}

// Not public. Registered as an atexit handler.
#[pyfunction]
fn shutdown(py: Python<'_>) {
    py.allow_threads(foxglove::shutdown_runtime);
}

/// Our public API is in the `python` directory.
/// Rust bindings are exported as `_foxglove_py` and should not be imported directly.
#[pymodule]
fn _foxglove_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(enable_logging, m)?)?;
    m.add_function(wrap_pyfunction!(disable_logging, m)?)?;
    m.add_function(wrap_pyfunction!(shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(open_mcap, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(get_channel_for_topic, m)?)?;
    m.add_class::<BaseChannel>()?;
    m.add_class::<PyMcapWriter>()?;

    // Websocket server classes
    m.add_class::<PyWebSocketServer>()?;
    m.add_class::<PyCapability>()?;
    m.add_class::<PyClient>()?;
    m.add_class::<PyChannelView>()?;
    m.add_class::<PyParameter>()?;
    m.add_class::<PyParameterType>()?;
    m.add_class::<PyParameterValue>()?;
    m.add_class::<PyStatusLevel>()?;
    // Services
    m.add_class::<PyService>()?;
    m.add_class::<PyRequest>()?;
    m.add_class::<PyServiceSchema>()?;
    m.add_class::<PyMessageSchema>()?;
    m.add_class::<PySchema>()?;

    // Register the schema & channel modules
    // A declarative submodule is created in generated/schemas_module.rs, but this is currently
    // easier to work with and function modules haven't yet been deprecated.
    schemas::register_submodule(m)?;
    channels::register_submodule(m)?;
    Ok(())
}
