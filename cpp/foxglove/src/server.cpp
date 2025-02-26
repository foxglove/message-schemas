#include <foxglove-c/foxglove-c.h>
#include <foxglove/server.hpp>

namespace foxglove {

WebSocketServer::WebSocketServer(WebSocketServerOptions options)
    : _impl(foxglove_server_create(options.name.c_str(), options.host.c_str(), options.port),
            foxglove_server_destroy) {}

void WebSocketServer::start() {
  foxglove_server_start(_impl.get());
}

void WebSocketServer::stop() {
  foxglove_server_stop(_impl.get());
}

}  // namespace foxglove
