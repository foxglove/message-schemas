#include <foxglove_sdk_cxxbridge_cpp/lib.h>

#include <cstdint>

namespace foxglove {

uint64_t dummy_cpp();
uint64_t dummy_rs();

using WebSocketServerOptions = foxglove_sdk_cxxbridge::WebSocketServerOptions;

class WebSocketServer {
public:
  WebSocketServer(WebSocketServerOptions options)
      : _options(std::move(options)) {}

  void start() {
    foxglove_sdk_cxxbridge::start_server(_options);
  }

private:
  WebSocketServerOptions _options;
};

}  // namespace foxglove
