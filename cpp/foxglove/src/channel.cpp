#include <foxglove-c/foxglove-c.h>
#include <foxglove/channel.hpp>

namespace foxglove {

Channel::Channel(
  const std::string& topic, const std::string& messageEncoding, std::optional<Schema> schema
)
    : _impl(
        foxglove_channel_create(
          topic.data(), messageEncoding.data(),
          schema ? &((const foxglove_schema&)foxglove_schema{
                     schema->name.data(),
                     schema->encoding.data(),
                     reinterpret_cast<const uint8_t*>(schema->data),
                     schema->dataLen,
                   })
                 : nullptr
        ),
        foxglove_channel_free
      ) {}

uint64_t Channel::id() const {
  return foxglove_channel_get_id(_impl.get());
}

void Channel::log(
  const std::byte* data, size_t dataLen, std::optional<uint64_t> logTime,
  std::optional<uint64_t> publishTime, std::optional<uint32_t> sequence
) {
  foxglove_channel_log(
    _impl.get(),
    reinterpret_cast<const uint8_t*>(data),
    dataLen,
    logTime ? &*logTime : nullptr,
    publishTime ? &*publishTime : nullptr,
    sequence ? &*sequence : nullptr
  );
}

}  // namespace foxglove
