#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

struct foxglove_channel;

namespace foxglove {

struct Schema {
  std::string_view name;
  std::string_view encoding;
  const std::byte* data;
  size_t dataLen;
};

class Channel final {
public:
  Channel(std::string_view topic, std::string_view messageEncoding, std::optional<Schema> schema);

  void log(
    const std::byte* data, size_t dataLen, std::optional<uint64_t> logTime = std::nullopt,
    std::optional<uint64_t> publishTime = std::nullopt,
    std::optional<uint32_t> sequence = std::nullopt
  );

private:
  std::unique_ptr<foxglove_channel, void (*)(foxglove_channel*)> _impl;
};

}  // namespace foxglove
