#include <foxglove/dummy.hpp>
#include <foxglove_sdk_cxxbridge_cpp/lib.h>

namespace foxglove {

uint64_t dummy_cpp() {
  return 42;
}

uint64_t dummy_rs() {
  return foxglove_sdk_cxxbridge::dummy_rs();
}

}  // namespace foxglove
