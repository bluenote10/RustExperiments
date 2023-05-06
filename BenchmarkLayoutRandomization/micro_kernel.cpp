#include <iostream>
#include <x86intrin.h>

// Adapted C++ version of micro kernel example.

// Using approach from:
// https://stackoverflow.com/a/51907627/1804173
inline
uint64_t readTSC() {
    // _mm_lfence();  // optionally wait for earlier insns to retire before reading the clock
    uint64_t tsc = __rdtsc();
    // _mm_lfence();  // optionally block later instructions until rdtsc retires
    return tsc;
}

static int i = 0, j = 0, k = 0;

int main() {
  uint64_t cycles_before = readTSC();
  int g = 0, inc = 1;
  for (; g<65536; g++) {
    i += inc;
    j += inc;
    k += inc;
  }
  uint64_t cycles_after = readTSC();
  std::cout << cycles_after - cycles_before << std::endl;
  return 0;
}
