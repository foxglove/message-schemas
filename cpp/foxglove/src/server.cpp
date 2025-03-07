#include <foxglove-c/foxglove-c.h>
#include <foxglove/server.hpp>

namespace {

bool hasAnyCallbacks(const foxglove::WebSocketServerCallbacks& callbacks) {
  return callbacks.onSubscribe || callbacks.onUnsubscribe;
}

}  // anonymous namespace

namespace foxglove {

WebSocketServer::WebSocketServer(WebSocketServerOptions options)
    : _callbacks(options.callbacks)
    , _impl(
        foxglove_server_start(&(const foxglove_server_options&)foxglove_server_options{
          options.name.c_str(),
          options.host.c_str(),
          options.port,
          !hasAnyCallbacks(options.callbacks)
            ? nullptr
            : &(const foxglove_server_callbacks&)foxglove_server_callbacks{
              this,
              !options.callbacks.onSubscribe ? nullptr : [](uint64_t channel_id, const void* context) {
                (reinterpret_cast<const WebSocketServer*>(context))->_callbacks.onSubscribe(channel_id);
              },
              !options.callbacks.onUnsubscribe ? nullptr : [](uint64_t channel_id, const void* context) {
                (reinterpret_cast<const WebSocketServer*>(context))->_callbacks.onUnsubscribe(channel_id);
              },
            }
        }),
        foxglove_server_free
      ) {}

void WebSocketServer::stop() {
  foxglove_server_stop(_impl.get());
}

uint16_t WebSocketServer::port() const {
  return foxglove_server_get_port(_impl.get());
}

}  // namespace foxglove
