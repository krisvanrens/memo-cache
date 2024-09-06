#include "memocache.hpp"

#include <map>
#include <random>
#include <unordered_map>

// NOTE: The test is run using the 'int' input space, and a normal distribution to
//        produce values with a mean of 0 and a variance as defined here:
constexpr auto INPUT_VARIANCE = 100;

#define MemoCacheBench_impl(s) \
  static void MemoCache##s(benchmark::State& state) { \
    mc::memo_cache<int, int, s> c; \
    \
    std::random_device device{}; \
    std::mt19937 generator{device()}; \
    std::normal_distribution<> distribution{0, INPUT_VARIANCE}; \
    \
    for (auto _ : state) { \
      const auto x = static_cast<int>(distribution(generator)); \
      if (const auto v = c.find(x); v) { \
        benchmark::DoNotOptimize(v); \
      } else { \
        c.insert(x, x); \
      } \
    } \
  } \
  BENCHMARK(MemoCache##s);

MemoCacheBench_impl(4)
MemoCacheBench_impl(8)
MemoCacheBench_impl(16)
MemoCacheBench_impl(32)
MemoCacheBench_impl(64)
MemoCacheBench_impl(128)
MemoCacheBench_impl(256)
MemoCacheBench_impl(512)
// NOTE: Behavior gets worse beyond this size.

static void OrderedMap(benchmark::State& state) {
  std::map<int, int> c;

  std::random_device device{};
  std::mt19937 generator{device()};
  std::normal_distribution<> distribution{0, INPUT_VARIANCE};

  for (auto _ : state) {
    const auto x = static_cast<int>(distribution(generator));
    if (const auto v = c.find(x); v != c.cend()) {
      benchmark::DoNotOptimize(v);
    } else {
      c[x] = x;
    }
  }
}
BENCHMARK(OrderedMap);

static void UnorderedMap(benchmark::State& state) {
  std::unordered_map<int, int> c;

  std::random_device device{};
  std::mt19937 generator{device()};
  std::normal_distribution<> distribution{0, INPUT_VARIANCE};

  for (auto _ : state) {
    const auto x = static_cast<int>(distribution(generator));
    if (const auto v = c.find(x); v != c.cend()) {
      benchmark::DoNotOptimize(v);
    } else {
      c[x] = x;
    }
  }
}
BENCHMARK(UnorderedMap);
