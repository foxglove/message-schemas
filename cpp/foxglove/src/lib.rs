use foxglove::{
    websocket::{Client, ClientChannelView, ServerListener},
    FoxgloveError, WebSocketServer, WebSocketServerBlockingHandle,
};
use std::{pin::Pin, time};

#[cxx::bridge(namespace = "foxglove_sdk_cxxbridge")]
mod ffi {
    struct WebSocketServerOptions {
        host: String,
        port: u16,
        name: String,
    }

    extern "Rust" {
        fn dummy_rs() -> u64;

        type CppWebSocketServer;
        fn start_server(options: WebSocketServerOptions) -> Result<Box<CppWebSocketServer>>;
        fn stop_server(server: Box<CppWebSocketServer>);
    }
}

fn dummy_rs() -> u64 {
    42
}

pub struct CppWebSocketServer(WebSocketServerBlockingHandle);

impl CppWebSocketServer {
    // pub fn stop(&mut self, py: Python<'_>) {
    //     if let Some(server) = self.0.take() {
    //         py.allow_threads(|| server.stop())
    //     }
    // }

    // pub fn stop(self: Pin<&mut Self>) {
    //     self.0.stop();
    // }

    // /// Sets a new session ID and notifies all clients, causing them to reset their state.
    // /// If no session ID is provided, generates a new one based on the current timestamp.
    // #[pyo3(signature = (session_id=None))]
    // pub fn clear_session(&self, session_id: Option<String>) -> PyResult<()> {
    //     if let Some(server) = &self.0 {
    //         server.clear_session(session_id);
    //     }
    //     Ok(())
    // }

    // pub fn broadcast_time(&self, timestamp_nanos: u64) -> PyResult<()> {
    //     if let Some(server) = &self.0 {
    //         server.broadcast_time(timestamp_nanos);
    //     }
    //     Ok(())
    // }
}

fn start_server(
    options: ffi::WebSocketServerOptions,
    // name: &str,
    // host: &str,
    // port: u16,
    // capabilities: Option<Vec<PyCapability>>,
    // server_listener: Option<Py<PyAny>>,
    // supported_encodings: Option<Vec<String>>,
) -> Result<Box<CppWebSocketServer>, FoxgloveError> {
    let session_id = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Failed to create session ID; invalid system time")
        .as_millis()
        .to_string();

    let server = WebSocketServer::new()
        .session_id(session_id)
        .bind(options.host, options.port)
        .name(options.name);

    let handle = server.start_blocking()?;

    Ok(Box::new(CppWebSocketServer(handle)))

    // if let Some(py_obj) = server_listener {
    //     let listener = PyServerListener { listener: py_obj };
    //     server = server.listener(Arc::new(listener));
    // }

    // if let Some(name) = name {
    //     server = server.name(name);
    // }

    // if let Some(capabilities) = capabilities {
    //     server = server.capabilities(capabilities.into_iter().map(PyCapability::into));
    // }

    // if let Some(supported_encodings) = supported_encodings {
    //     server = server.supported_encodings(supported_encodings);
    // }

    // let handle = py
    //     .allow_threads(|| server.start_blocking())
    //     .map_err(PyFoxgloveError::from)?;

    // Ok(PyWebSocketServer(Some(handle)))
}

fn stop_server(server: Box<CppWebSocketServer>) {
    server.0.stop();
}
