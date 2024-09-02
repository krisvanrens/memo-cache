#pragma once

#include <algorithm>
#include <array>
#include <concepts>
#include <cstddef>
#include <functional>
#include <optional>
#include <utility>

inline namespace v1 {

///
/// A small, fixed-size, heap-allocated key/value cache with retention management, for use with regular types.
///
template<std::regular Key, std::regular Val, std::size_t Size>
class memo_cache {
  static_assert(Size > 0);
  static_assert(Size <= 128, "Semantic constraint: use this cache for small sizes only (see performance notes).");

  template<typename K = Key, typename V = Val> struct key_value_slot_t {
    K key;
    V val;
    bool empty = true;
  };

  using buffer_t = std::array<key_value_slot_t<Key, Val>, Size>;

  buffer_t buffer;
  std::size_t cursor{};

public:
  /// Get the (fixed) size of the cache.
  [[nodiscard]] constexpr std::size_t size() const noexcept {
    return Size;
  }

  /// Insert a key/value pair.
  template <typename Key_, typename Val_>
  void insert(Key_&& key, Val_&& val) requires (std::assignable_from<Key, Key_> || std::convertible_to<Key_, Key>)
                                            && (std::assignable_from<Val, Val_> || std::convertible_to<Val_, Val>)
  {
    // Overwrite values for identical keys.
    if (auto found = find(key); found) {
      found.value().get() = std::forward<Val_>(val);
    } else {
      buffer[cursor] = {.key = std::forward<Key_>(key), .val = std::forward<Val_>(val), .empty = false};

      // Move the cursor over the buffer elements sequentially, creating FIFO behavior.
      cursor = (cursor + 1) % Size; // Wrap; overwrite the oldest element next time around.
    }
  }

  /// Lookup a cache entry by key.
  [[nodiscard]] std::optional<std::reference_wrapper<Val>> find(const Key& key) {
    const auto slot = std::ranges::find_if(buffer, [&key](const auto& fSlot) { return !fSlot.empty && (fSlot.key == key); });
    if (slot != buffer.cend()) {
      return std::ref(slot->val);
    } else {
      return std::nullopt;
    }
  }
};

} // namespace v1
