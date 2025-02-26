#include <foxglove/server.hpp>

#include <chrono>
#include <csignal>
#include <iostream>
#include <memory>
#include <thread>

using namespace std::chrono_literals;

static std::function<void()> sigintHandler;

int main(int argc, const char* argv[]) {
  std::signal(SIGINT, [](int) {
    if (sigintHandler) sigintHandler();
  });

  foxglove::WebSocketServerOptions options{
    .name = "ws-demo",
    .host = "127.0.0.1",
    .port = 8765,
  };
  foxglove::WebSocketServer server{options};
  server.start();
  std::cerr << "Started server" << std::endl;

  std::atomic_bool done = false;
  sigintHandler = [&] {
    std::cerr << "Shutting down..." << std::endl;
    server.stop();
    done = true;
  };

  while (!done) {
    std::this_thread::sleep_for(10ms);
  }

  std::cerr << "Done" << std::endl;
  return 0;
}
