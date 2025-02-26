#include <foxglove/server.hpp>

#include <catch2/catch_test_macros.hpp>

TEST_CASE("Start and stop server") {
  foxglove::WebSocketServerOptions options{
    .name = "unit-test",
    .host = "127.0.0.1",
    .port = 8765,
  };
  foxglove::WebSocketServer server{options};
  server.stop();
}
