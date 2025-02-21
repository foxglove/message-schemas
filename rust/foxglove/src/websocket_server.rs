//! Websocket server

use std::fmt::Debug;
use std::sync::Arc;

use crate::websocket::service::{Service, ServiceId};
use crate::websocket::{
    create_server, Capability, ConnectionGraph, Parameter, Server, ServerOptions, Status,
};
use crate::{get_runtime_handle, FoxgloveError, LogContext, LogSink};
use tokio::runtime::Handle;
use tracing::warn;

/// A websocket server for live visualization.
#[must_use]
#[derive(Debug)]
pub struct WebSocketServer {
    host: String,
    port: u16,
    options: ServerOptions,
}

impl Default for WebSocketServer {
    fn default() -> Self {
        let options = ServerOptions {
            session_id: Some(Server::generate_session_id()),
            ..ServerOptions::default()
        };
        Self {
            host: "127.0.0.1".into(),
            port: 8765,
            options,
        }
    }
}

impl WebSocketServer {
    /// Creates a new websocket server with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the websocket server name to advertise to clients.
    ///
    /// By default, the server is not given a name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.options.name = Some(name.into());
        self
    }

    /// Bind a TCP port.
    ///
    /// By default, the server will bind to `127.0.0.1:8765`.
    pub fn bind(mut self, host: impl Into<String>, port: u16) -> Self {
        self.host = host.into();
        self.port = port;
        self
    }

    /// Sets the server capabilities to advertise to the client.
    ///
    /// By default, the server does not advertise any capabilities.
    pub fn capabilities(mut self, capabilities: impl IntoIterator<Item = Capability>) -> Self {
        self.options.capabilities = Some(capabilities.into_iter().collect());
        self
    }

    /// Configure an event listener to receive client message events.
    pub fn listener(mut self, listener: Arc<dyn crate::websocket::ServerListener>) -> Self {
        self.options.listener = Some(listener);
        self
    }

    /// Set the message backlog size.
    ///
    /// The server buffers outgoing log entries into a queue. If the backlog size is exceeded, the
    /// oldest entries will be dropped.
    ///
    /// By default, the server will buffer 1024 messages.
    pub fn message_backlog_size(mut self, size: usize) -> Self {
        self.options.message_backlog_size = Some(size);
        self
    }

    /// Configure the set of services to advertise to clients.
    ///
    /// Automatically adds [`Capability::Services`] to the set of advertised capabilities.
    ///
    /// Note that services can by dynamically registered and unregistered later using
    /// [`WebSocketServerHandle::add_services`] and [`WebSocketServerHandle::remove_services`].
    pub fn services(mut self, services: impl IntoIterator<Item = Service>) -> Self {
        self.options.services.clear();
        for service in services {
            let name = service.name().to_string();
            if let Some(s) = self.options.services.insert(name.clone(), service) {
                warn!("Redefining service {}", s.name());
            }
        }
        self
    }

    /// Configure the set of supported encodings for client requests.
    ///
    /// This is used for both client-side publishing as well as service call request/responses.
    pub fn supported_encodings(
        mut self,
        encodings: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.options.supported_encodings = Some(encodings.into_iter().map(|e| e.into()).collect());
        self
    }

    /// Set a session ID.
    ///
    /// This allows the client to understand if the connection is a re-connection or if it is
    /// connecting to a new server instance. This can for example be a timestamp or a UUID.
    ///
    /// By default, this is set to the number of milliseconds since the unix epoch.
    pub fn session_id(mut self, id: impl Into<String>) -> Self {
        self.options.session_id = Some(id.into());
        self
    }

    /// Configure the tokio runtime for the server to use for async tasks.
    ///
    /// By default, the server will use either the current runtime (if started with
    /// [`WebSocketServer::start`]), or spawn its own internal runtime (if started with
    /// [`WebSocketServer::start_blocking`]).
    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    pub fn tokio_runtime(mut self, handle: &Handle) -> Self {
        self.options.runtime = Some(handle.clone());
        self
    }

    /// Starts the websocket server.
    ///
    /// Returns a handle that can optionally be used to gracefully shutdown the server. The caller
    /// can safely drop the handle, and the server will run forever.
    pub async fn start(self) -> Result<WebSocketServerHandle, FoxgloveError> {
        let server = create_server(self.options);
        server.start(&self.host, self.port).await?;
        LogContext::global().add_sink(server.clone());
        Ok(WebSocketServerHandle(server))
    }

    /// Starts the websocket server.
    ///
    /// Returns a handle that can optionally be used to gracefully shutdown the server. The caller
    /// can safely drop the handle, and the server will run forever.
    ///
    /// This method will panic if invoked from an asynchronous execution context. Use
    /// [`WebSocketServer::start`] instead.
    pub fn start_blocking(mut self) -> Result<WebSocketServerBlockingHandle, FoxgloveError> {
        let runtime = self
            .options
            .runtime
            .get_or_insert_with(get_runtime_handle)
            .clone();
        let handle = runtime.block_on(self.start())?;
        Ok(WebSocketServerBlockingHandle(handle))
    }
}

/// A handle to the websocket server.
///
/// This handle can safely be dropped and the server will run forever.
pub struct WebSocketServerHandle(Arc<Server>);

impl Debug for WebSocketServerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WebSocketServerHandle").finish()
    }
}

impl WebSocketServerHandle {
    /// Returns a handle to the async runtime.
    fn runtime(&self) -> &Handle {
        self.0.runtime()
    }

    /// Advertises support for the provided services.
    ///
    /// These services will be available for clients to use until they are removed with
    /// [`remove_services`][WebSocketServerHandle::remove_services].
    ///
    /// This method will fail if the server was not configured with [`Capability::Services`].
    pub fn add_services(
        &self,
        services: impl IntoIterator<Item = Service>,
    ) -> Result<(), FoxgloveError> {
        self.0.add_services(services.into_iter().collect())
    }

    /// Removes services that were previously advertised.
    pub fn remove_services(&self, ids: impl IntoIterator<Item = ServiceId>) {
        self.0.remove_services(&ids.into_iter().collect::<Vec<_>>());
    }

    /// Publishes the current server timestamp to all clients.
    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    pub fn broadcast_time(&self, timestamp_nanos: u64) {
        self.0.broadcast_time(timestamp_nanos);
    }

    /// Sets a new session ID and notifies all clients, causing them to reset their state.
    /// If no session ID is provided, generates a new one based on the current timestamp.
    pub fn clear_session(&self, new_session_id: Option<String>) {
        self.0.clear_session(new_session_id);
    }

    /// Publishes parameter values to all clients.
    pub fn publish_parameter_values(&self, parameters: Vec<Parameter>) {
        self.0.publish_parameter_values(parameters);
    }

    /// Publishes a status message to all clients.
    ///
    /// For more information, refer to the [Status][status] message specification.
    ///
    /// [status]: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#status
    pub fn publish_status(&self, status: Status) {
        self.0.publish_status(status);
    }

    /// Removes status messages by id from all clients.
    ///
    /// For more information, refer to the [Remove Status][remove-status] message specification.
    ///
    /// [remove-status]: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#remove-status
    pub fn remove_status(&self, status_ids: Vec<String>) {
        self.0.remove_status(status_ids);
    }

    /// Publishes a connection graph update to all subscribed clients.
    pub fn publish_connection_graph(&self, update: ConnectionGraph) -> Result<(), FoxgloveError> {
        self.0.connection_graph_update(update)
    }

    /// Gracefully shutdown the websocket server.
    pub async fn stop(self) {
        let sink = self.0.clone() as Arc<dyn LogSink>;
        LogContext::global().remove_sink(&sink);
        self.0.stop().await;
    }
}

/// A blocking wrapper around a WebSocketServerHandle.
#[derive(Debug)]
pub struct WebSocketServerBlockingHandle(WebSocketServerHandle);

impl WebSocketServerBlockingHandle {
    /// Advertises support for the provided services.
    ///
    /// These services will be available for clients to use until they are removed with
    /// [`remove_services`][WebSocketServerBlockingHandle::remove_services].
    ///
    /// This method will fail if the server was not configured with [`Capability::Services`].
    pub fn add_services(
        &self,
        services: impl IntoIterator<Item = Service>,
    ) -> Result<(), FoxgloveError> {
        self.0.add_services(services)
    }

    /// Removes services that were previously advertised.
    pub fn remove_services(&self, ids: impl IntoIterator<Item = ServiceId>) {
        self.0.remove_services(ids);
    }

    /// Publishes the current server timestamp to all clients.
    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    pub fn broadcast_time(&self, timestamp_nanos: u64) {
        self.0.broadcast_time(timestamp_nanos);
    }

    /// Sets a new session ID and notifies all clients, causing them to reset their state.
    /// If no session ID is provided, generates a new one based on the current timestamp.
    pub fn clear_session(&self, new_session_id: Option<String>) {
        self.0.clear_session(new_session_id);
    }

    /// Publishes parameter values to all clients.
    pub fn publish_parameter_values(&self, parameters: Vec<Parameter>) {
        self.0.publish_parameter_values(parameters)
    }

    /// Publishes a status message to all clients.
    ///
    /// For more information, refer to the [Status][status] message specification.
    ///
    /// [status]: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#status
    pub fn publish_status(&self, status: Status) {
        self.0.publish_status(status);
    }

    /// Removes status messages by id from all clients.
    ///
    /// For more information, refer to the [Remove Status][remove-status] message specification.
    ///
    /// [remove-status]: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#remove-status
    pub fn remove_status(&self, status_ids: Vec<String>) {
        self.0.remove_status(status_ids);
    }

    /// Publishes a connection graph update to all subscribed clients.
    pub fn publish_connection_graph(&self, update: ConnectionGraph) -> Result<(), FoxgloveError> {
        self.0.publish_connection_graph(update)
    }

    /// Gracefully shutdown the websocket server.
    pub fn stop(self) {
        self.0.runtime().clone().block_on(self.0.stop());
    }
}
