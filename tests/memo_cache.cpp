#include <memo_cache.hpp>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <string>
#include <tuple>

TEST_SUITE("memo_cache")
{

  TEST_CASE("Cache size")
  {
    constexpr std::size_t SIZE = 16;

    memo_cache<std::string, int, SIZE> c;

    CHECK_EQ(c.size(), SIZE);
  }

  TEST_CASE("Clear")
  {
    memo_cache<std::string, int, 3> c;

    CHECK_FALSE(c.find("hello").has_value());

    c.insert("hello", 42);

    REQUIRE(c.find("hello").has_value());
    CHECK_EQ(c.find("hello").value(), 42);

    c.clear();

    CHECK_FALSE(c.find("hello").has_value());
  }

  TEST_CASE("Empty cache")
  {
    memo_cache<bool, bool, 2> c;

    // NOTE: Even though the cache memory is pre-allocated, each cache slot should be marked as "empty".
    CHECK_FALSE(c.find(true).has_value());
    CHECK_FALSE(c.find(false).has_value());
  }

  TEST_CASE("Non-empty cache")
  {
    memo_cache<std::string, int, 3> c;

    const std::array<std::pair<std::string, int>, 3> KVS = {std::make_pair("veni", 19), std::make_pair("vidi", 23), std::make_pair("vici", 29)};

    const auto &KV0 = KVS.at(0);
    const auto &KV1 = KVS.at(1);
    const auto &KV2 = KVS.at(2);

    const auto insertKV = [&c](const auto &fKV) { c.insert(fKV.first, fKV.second); };

    CHECK_FALSE(c.find(KV0.first).has_value());
    CHECK_FALSE(c.find(KV1.first).has_value());
    CHECK_FALSE(c.find(KV2.first).has_value());

    insertKV(KV0);

    REQUIRE(c.find(KV0.first).has_value());
    CHECK_EQ(c.find(KV0.first).value(), KV0.second);

    insertKV(KV1);

    REQUIRE(c.find(KV1.first).has_value());
    CHECK_EQ(c.find(KV1.first).value(), KV1.second);

    insertKV(KV2);

    REQUIRE(c.find(KV2.first).has_value());
    CHECK_EQ(c.find(KV2.first).value(), KV2.second);

    // The cache is now full, and another insertion will make the first key/value be removed.

    c.insert("blah", 42);

    CHECK_FALSE(c.find(KV0.first).has_value());
    CHECK(c.find(KV1.first).has_value());
    CHECK(c.find(KV2.first).has_value());

    c.insert("bleh", 42);
    c.insert("bloh", 42);

    CHECK_FALSE(c.find(KV1.first).has_value());
    CHECK_FALSE(c.find(KV2.first).has_value());
  }

  TEST_CASE("Duplicate insertions")
  {
    memo_cache<std::string, int, 2> c;

    const auto KV0 = std::make_pair("John", 17);
    const auto KV1 = std::make_pair("Doe", 19);

    CHECK_FALSE(c.find(KV0.first).has_value());
    CHECK_FALSE(c.find(KV1.first).has_value());

    c.insert(KV0.first, KV0.second);
    c.insert(KV1.first, KV1.second);

    CHECK(c.find(KV0.first).has_value());
    CHECK(c.find(KV1.first).has_value());

    // Inserting a duplicate key/value should be a no-op.

    c.insert(KV0.first, KV0.second);

    CHECK(c.find(KV0.first).has_value());
    CHECK(c.find(KV1.first).has_value());

    // Inserting a duplicate key with a new value should update the value.

    CHECK_EQ(c.find(KV0.first).value(), KV0.second);
    CHECK_EQ(c.find(KV1.first).value(), KV1.second);

    c.insert(KV0.first, 42);

    CHECK_EQ(c.find(KV0.first).value(), 42); // Updated.
    CHECK_EQ(c.find(KV1.first).value(), KV1.second);
  }

  TEST_CASE("Static type properties")
  {
    using Cache = memo_cache<std::string, std::string, 8>;

    // The cache is semiregular:
    static_assert(std::is_constructible_v<Cache>);
    static_assert(std::is_default_constructible_v<Cache>);
    static_assert(std::is_copy_assignable_v<Cache>);
    static_assert(std::is_copy_constructible_v<Cache>);
    static_assert(std::is_move_assignable_v<Cache>);
    static_assert(std::is_move_constructible_v<Cache>);
  }

} // TEST_SUITE
