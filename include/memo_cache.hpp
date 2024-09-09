#pragma once

#include <algorithm>
#include <array>
#include <concepts>
#include <cstddef>
#include <functional>
#include <iterator>
#include <optional>
#include <utility>

namespace mc {

inline namespace v1 {

///
/// A small, fixed-size, heap-allocated key/value cache with retention management, for use with regular types.
///
template<std::regular Key, std::regular Val, std::size_t Size>
class memo_cache {
  static_assert(Size > 0);
  static_assert(Size <= 128, "Semantic constraint: use this cache for small sizes only (see performance notes).");

  template<typename K = Key, typename V = Val> struct key_value_slot_t {
    K                key;
    std::optional<V> val;
  };

  using buffer_t = std::array<key_value_slot_t<Key, Val>, Size>;

  buffer_t buffer;
  std::size_t cursor{};

  /// Replace slot under cursor and shift cursor position. Returns a reference to the replaced slot value.
  template<typename Key_, typename Val_>
  Val& replace_and_shift(Key_&& key, Val_&& val) {
    buffer[cursor] = {.key = std::forward<Key_>(key), .val = {std::forward<Val_>(val)}};

    // SAFETY: The option value is occupied above.
    auto& value = *(buffer[cursor].val);

    // Move the cursor over the buffer elements sequentially, creating FIFO behavior.
    cursor = (cursor + 1) % Size; // Wrap; overwrite the oldest element next time around.

    return value;
  }

public:
  /// Get the (fixed) size of the cache.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<std::string, float, 4> c;
  ///
  /// assert(c.size() == 4);
  /// ```
  [[nodiscard]] constexpr std::size_t size() const noexcept {
    return Size;
  }

  /// Insert a key/value pair.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<std::string, float, 4> c;
  ///
  /// assert(!c.find("hello").has_value());
  ///
  /// c.insert("hello", 42);
  ///
  /// assert(c.find("hello").has_value());
  /// assert(c.find("hello").value() == 42);
  /// ```
  template<typename Key_, typename Val_>
  void insert(Key_&& key, Val_&& val) requires (std::assignable_from<Key, Key_> || std::convertible_to<Key_, Key>)
                                            && (std::assignable_from<Val, Val_> || std::convertible_to<Val_, Val>)
  {
    // Overwrite values for identical keys.
    if (auto found = find(key); found) {
      found.value().get() = std::forward<Val_>(val);
    } else {
      replace_and_shift(std::forward<Key_>(key), std::forward<Val_>(val));
    }
  }

  /// Lookup a cache entry by key.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<std::string, float, 4> c;
  ///
  /// assert(!c.find("hello").has_value());
  ///
  /// c.insert("hello", 42);
  ///
  /// assert(c.find("hello").has_value());
  /// assert(c.find("hello").value() == 42);
  /// ```
  [[nodiscard]] std::optional<std::reference_wrapper<Val>> find(const Key& key) {
    const auto slot = std::ranges::find_if(buffer, [&key](const auto& fSlot) { return fSlot.val && (fSlot.key == key); });
    if (slot != buffer.cend()) {
      // SAFETY: The slot value was found by definition.
      return std::ref(*slot->val);
    } else {
      return std::nullopt;
    }
  }

  /// Get a value, or, if it does not exist in the cache, insert it using the value computed by `f`.
  /// Returns a reference to the found, or newly inserted value associated with the given key.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<int, std::string, 4> c;
  ///
  /// assert(!c.contains(42));
  ///
  /// auto v = c.find_or_insert_with(42, [] { return "The Answer"; });
  ///
  /// assert(v == "The Answer");
  /// assert(c.find(42).has_value());
  /// assert(c.find(42).value() == "The Answer");
  /// ```
  template<typename F>
  [[nodiscard]] std::reference_wrapper<Val> find_or_insert_with(const Key& key, F f) {
    if (auto slot = find(key); slot) {
      return *slot;
    } else {
      return replace_and_shift(key, f(key));
    }
  }

  /// Returns `true` if the cache contains a value for the specified key.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<int, std::string, 4> c;
  ///
  /// assert(!c.contains(42));
  ///
  /// c.insert(42, "The Answer");
  ///
  /// assert(c.contains(42));
  /// ```
  [[nodiscard]] bool contains(const Key& key) const {
    return std::ranges::any_of(buffer, [&key](const auto& fSlot) { return fSlot.val && (fSlot.key == key); });
  }

  /// Clear the cache.
  ///
  /// # Examples
  ///
  /// ```
  /// #include <cassert>
  /// #include <memo_cache.hpp>
  ///
  /// memo_cache<std::string, float, 4> c;
  ///
  /// c.insert("hello", 42);
  ///
  /// assert(c.find("hello").has_value());
  /// assert(c.find("hello").value() == 42);
  ///
  /// c.clear();
  ///
  /// assert(!c.find("hello").has_value());
  /// ```
  void clear() {
    buffer = {};
  }
};

} // namespace v1

} // namespace mc
