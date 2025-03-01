// On by default in rust 2024
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unsafe_attr_outside_unsafe)]

use std::ffi::{c_char, CStr};

pub struct FoxgloveWebSocketServer(Option<foxglove::WebSocketServerBlockingHandle>);

/// Create and start a server. The server must later be freed with `foxglove_server_free`.
///
/// `port` may be 0, in which case an available port will be automatically selected.
///
/// # Safety
/// `name` and `host` must be null-terminated strings with valid UTF8.
#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn foxglove_server_start(
    name: *const c_char,
    host: *const c_char,
    port: u16,
) -> *mut FoxgloveWebSocketServer {
    Box::into_raw(Box::new(FoxgloveWebSocketServer(Some(
        foxglove::WebSocketServer::new()
            .name(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
            .bind(unsafe { CStr::from_ptr(host) }.to_str().unwrap(), port)
            .start_blocking()
            .expect("Server failed to start"),
    ))))
}

/// Get the port on which the server is listening.
#[unsafe(no_mangle)]
pub extern "C" fn foxglove_server_get_port(server: Option<&FoxgloveWebSocketServer>) -> u16 {
    let Some(server) = server else {
        panic!("Expected a non-null server");
    };
    let Some(ref handle) = server.0 else {
        panic!("Server already stopped");
    };
    handle.port()
}

/// Free a server created via `foxglove_server_start`.
///
/// If the server has not already been stopped, it will be stopped automatically.
#[unsafe(no_mangle)]
pub extern "C" fn foxglove_server_free(server: Option<&mut FoxgloveWebSocketServer>) {
    let Some(server) = server else {
        return;
    };
    if let Some(handle) = server.0.take() {
        handle.stop();
    }
    drop(unsafe { Box::from_raw(server) });
}

/// Stop and shut down a server.
#[unsafe(no_mangle)]
pub extern "C" fn foxglove_server_stop(server: Option<&mut FoxgloveWebSocketServer>) {
    let Some(server) = server else {
        panic!("Expected a non-null server");
    };
    let Some(handle) = server.0.take() else {
        panic!("Server already stopped");
    };
    handle.stop();
}
