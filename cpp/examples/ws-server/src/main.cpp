#include <foxglove/dummy.hpp>

#include <chrono>
#include <iostream>
#include <memory>

int main(int argc, const char* argv[]) {
  {
    auto t1 = std::chrono::high_resolution_clock::now();
    uint64_t sum;
    for (int i = 0; i < 5'000'000; i++) {
      sum = foxglove::dummy_cpp();
    }
    auto t2 = std::chrono::high_resolution_clock::now();

    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count();

    std::cerr << "Calling C++ dummy function: " << sum << ", time elapsed: " << duration << " ms"
              << std::endl;
  }

  {
    auto t1 = std::chrono::high_resolution_clock::now();
    uint64_t sum;
    for (int i = 0; i < 5'000'000; i++) {
      sum = foxglove::dummy_rs();
    }
    auto t2 = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count();

    std::cerr << "Calling Rust dummy function: " << sum << ", time elapsed: " << duration << " ms"
              << std::endl;
  }

  foxglove::WebSocketServerOptions options;
  options.name = "ws-demo";
  options.host = "127.0.0.1";
  options.port = 8765;

  foxglove::WebSocketServer server{options};
  server.start();

  return 0;
}
