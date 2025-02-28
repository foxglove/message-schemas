#include <foxglove/server.hpp>

#include <catch2/catch_test_macros.hpp>

TEST_CASE("Start and stop server") {
  foxglove::WebSocketServerOptions options;
  options.name = "unit-test";
  options.host = "127.0.0.1";
  options.port = 0;
  foxglove::WebSocketServer server{options};
  REQUIRE(server.port() != 0);
  server.stop();
}
