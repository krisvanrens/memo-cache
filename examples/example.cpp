#include <memo_cache.hpp>

#include <algorithm>
#include <chrono>
#include <format>
#include <iostream>
#include <numbers>
#include <numeric>
#include <random>
#include <thread>
#include <unordered_map>

float some_expensive_calculation(int) {
  using namespace std::chrono_literals;
  std::this_thread::sleep_for(20ms);
  return static_cast<float>(std::numbers::pi);
}

struct Process {
  std::unordered_map<int, float> cache1;
  mc::memo_cache<int, float, 32> cache2;

  /// Regular method, taking the calculation penalty, always.
  float regular(int input) { return some_expensive_calculation(input); }

  /// Memoized method, using a `std::unordered_map` cache (no retention management).
  float memoized1(int input) {
    if (auto v = cache1.find(input); v != cache1.cend()) {
      return v->second;
    } else {
      auto result = some_expensive_calculation(input);
      cache1[input] = result;
      return result;
    }
  }

  /// Memoized method, using a `MemoCache` cache (using `get` and `insert`).
  float memoized2a(int input) {
    if (auto v = cache2.find(input); v) {
      return *v;
    } else {
      auto result = some_expensive_calculation(input);
      cache2.insert(input, result);
      return result;
    }
  }

  /// Memoized method, using a `MemoCache` cache (using `get_or_insert_with`).
  float memoized2b(int input) {
    return cache2.find_or_insert_with(input, [](int i) { return some_expensive_calculation(i); });
  }
};

int main() {
  // This test runs three individual test cases:
  //
  //   1. a regular (non-memoized) method,
  //   2. a method memoized using a hash map,
  //   3. a method memoized using a MemoCache cache (two notation variants).
  //
  // Each of the methods are fed a series of random input numbers from a
  // normal distribution for which they (fake) "calculate" a result value.
  // The memoized methods keep a local cache of result values for input
  // values. The hash map will definitely perform best, but has no retention
  // management -- its memory usage will grow with every new inserted input
  // value. The method using the MemoCache cache will use a fixed-capacity
  // cache and will perform at best as good as the hash map cache version,
  // and in the worst case as bad as the regular (non-memoized) method.

  std::random_device rd;
  std::mt19937 generator(rd());
  std::normal_distribution<> dist(0.0, 30.0);

  // Use the same input data for all tests:
  const auto inputs = [&] {
    std::vector<int> result(100);
    std::ranges::generate(result, [&] { return static_cast<int>(dist(generator)); });
    return result;
  }();

  Process p;

  std::cout << "Running tests..\n";

  using clock = std::chrono::system_clock;

  auto start = clock::now();
  std::reduce(inputs.cbegin(), inputs.cend(), 0.0f,
              [&p](float sum, int i) { return sum + p.regular(i); });

  const auto d_regular = clock::now() - start;

  start = clock::now();
  std::reduce(inputs.cbegin(), inputs.cend(), 0.0f,
              [&p](float sum, int i) { return sum + p.memoized1(i); });

  const auto d_memoized1 = clock::now() - start;

  start = clock::now();
  std::reduce(inputs.cbegin(), inputs.cend(), 0.0f,
              [&p](float sum, int i) { return sum + p.memoized2a(i); });

  const auto d_memoized2a = clock::now() - start;

  start = clock::now();
  std::reduce(inputs.cbegin(), inputs.cend(), 0.0f,
              [&p](float sum, int i) { return sum + p.memoized2b(i); });

  const auto d_memoized2b = clock::now() - start;

  std::cout << "Done. Timing results:\n";

  using Ms = std::chrono::milliseconds;
  std::cout << std::format("Regular:                {} ms\n", std::chrono::duration_cast<Ms>(d_regular).count());
  std::cout << std::format("Memoized (hash):        {} ms\n", std::chrono::duration_cast<Ms>(d_memoized1).count());
  std::cout << std::format("Memoized (MemoCache A): {} ms\n", std::chrono::duration_cast<Ms>(d_memoized2a).count());
  std::cout << std::format("Memoized (MemoCache B): {} ms\n", std::chrono::duration_cast<Ms>(d_memoized2b).count());

  const auto get_size = [](std::size_t capacity) { return capacity * (sizeof(int) + sizeof(float)); };

  std::cout << "Post-test occupied cache sizes:\n";
  std::cout << std::format("  std::unordered_map: {} bytes\n", get_size(p.cache1.size()));
  std::cout << std::format("  memo_cache:         {} bytes\n", get_size(p.cache2.size()));
}

// Compiler Explorer: https://www.godbolt.org/z/qaezrq78P
