use std::ffi::{c_char, CStr};

pub struct FoxgloveWebSocketServer {
    server: Option<foxglove::WebSocketServer>,
    handle: Option<foxglove::WebSocketServerBlockingHandle>,
}

/// Create a server. The server must later be freed with `foxglove_server_destroy`.
#[no_mangle]
pub extern "C" fn foxglove_server_create(
    name: *const c_char,
    host: *const c_char,
    port: u16,
) -> *mut FoxgloveWebSocketServer {
    let raw = Box::into_raw(Box::new(FoxgloveWebSocketServer {
        server: Some(
            foxglove::WebSocketServer::new()
                .name(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
                .bind(unsafe { CStr::from_ptr(host) }.to_str().unwrap(), port),
        ),
        handle: None,
    }));
    return raw;
}

/// Free a server created via `foxglove_server_start`.
#[no_mangle]
pub extern "C" fn foxglove_server_destroy(server: *mut FoxgloveWebSocketServer) {
    drop(unsafe { Box::from_raw(server) });
}

#[no_mangle]
pub extern "C" fn foxglove_server_start(server: *mut FoxgloveWebSocketServer) {
    let server = unsafe { &mut *server };
    if !server.handle.is_none() {
        panic!("Server already started, TODO: handle this better");
    }
    if let Some(srv) = server.server.take() {
        server.handle = Some(
            srv.start_blocking()
                .expect("TODO: handle start_blocking error"),
        );
    } else {
        panic!("Server already started, TODO: handle this better");
    }
}

#[no_mangle]
pub extern "C" fn foxglove_server_stop(server: *mut FoxgloveWebSocketServer) {
    let server = unsafe { &mut *server };
    let handle = server.handle.take().expect("TODO: handle stop error");
    handle.stop();
}
