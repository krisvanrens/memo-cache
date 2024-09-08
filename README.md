![build status](https://github.com/krisvanrens/memo-cache/actions/workflows/build-and-test.yml/badge.svg)

# memo-cache

A small, fixed-size cache with retention management for memoization.

## Introduction

Sometimes it can be beneficial to speedup pure function calls by using [memoization](https://en.wikipedia.org/wiki/Memoization).

The cache storage required for implementing such a speedup often is an associative container (i.e. a key/value store).
Programming language standard libraries provide such containers, often implemented as a [hash table](https://en.wikipedia.org/wiki/Hash_table) or a [red-black tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree).
These implementations are fine for performance, but do not actually cover all cases because of the lack of retention management

Suppose your input data covers the whole space that can be represented by a 64-bit integer.
There probably is some (generally non-uniform) distribution with which the input values arrive, but it's possible that over time *all* possible values pass by.
Any cache without retention management will then grow to potentially enormous dimensions in memory which is undesirable.

The cache implemented in this library uses a FIFO-style sequential data storage with fixed size, pre-allocated memory.
When the cache is full, the oldest item is evicted.

### Performance notes

The use of a sequential data storage does have performance impact, especially for key lookup.
That's why this cache will only be beneficial performance-wise when used with a relatively small size, up to about 128 elements.

Looking up items from the cache uses a linear search function, and storage is in FIFO order, resulting in O(N) complexity for `find()`.
In C++ for example, This lookup complexity is easily outperformed by `find()` on `std::map` (O(log N)) or `std::unordered_map` (constant to O(N)), however both of which do not implement retention management.

## Usage

This library requires a C++20 compiler, and can be used as a single-header library by directly including the file `include/memo_cache.hpp`.

It can be used like this (using `find_or_insert_with`):

```c++
// A function performing expensive, deterministic calculations.
float calculate(int input) {
  static mc::memo_cache<int, float> cache;

  return cache.find_or_insert_with(input, [](auto& key) {
    return /* ..some expensive calculation.. */;
  });
}
```

Or like this (using separate `find` and `insert` calls):

```c++
// A function performing expensive, deterministic calculations.
float calculate(int input) {
  static mc::memo_cache<int, float> cache;

  float result{};

  if (const auto found = cache.find(input); found) {
    result = found.value(); // Cache hit, return found value.
  } else {
    result = /* ..some expensive calculation.. */;
    cache.insert(input, result); // Update cache with new key/value.
  }

  return result;
}
```

Generally speaking, the use of `static` variables in functions are not desirable as they introduce (hidden) global state.
Always try to have the cache be stored non-statically as a class member for methods for example.

### Example

An example program comparing `std::unordered_map` with `mc::memo_cache` can be found [here](examples/example.cpp) (and on [the excellent Compiler Explorer](https://www.godbolt.org/z/KExn3fxjx)).

### Test build instructions

Install `doctest-dev` from the package repository, or get it [here](https://github.com/doctest/doctest).
Then build the tests:

```
mkdir build
cd build
clang++-18 -std=c++20 -Wall -Wextra -Werror -Wconversion -O3 -I../include ../tests/memo_cache.cpp -o memo_cache
```

### Benchmark

See a benchmark of `mc::memo_cache` against `std::map` and `std::unordered_map` [here at Quick-Bench](https://quick-bench.com/q/5Ga72_Fxi8V8eq7kXwk2WZAaFps).

## TODO

- Create automated build/test setup.
- Workout benchmarks in more detail.
- Remove platform-dependent instructions.
- Re-consider "LRU" aspect i.c.w. cursor movement.

## License

This software is licensed under the [MIT license](LICENSE).
