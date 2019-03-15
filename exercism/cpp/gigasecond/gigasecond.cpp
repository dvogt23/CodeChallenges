#include "gigasecond.h"

namespace gigasecond {
ptime advance(ptime actual) {
  return actual + time_duration(seconds(1000000000));
}
} // namespace gigasecond
