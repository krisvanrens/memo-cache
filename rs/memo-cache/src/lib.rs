/// A single key/value slot used in the cache.
#[derive(Clone, PartialEq)]
enum KeyValueSlot<Key, Val> {
    Used((Key, Val)),
    Empty,
}

impl<Key, Val> KeyValueSlot<Key, Val>
where
    Key: Eq,
{
    /// Check a used slot key.
    fn is_key(&self, key: &Key) -> bool {
        if let KeyValueSlot::Used(kv) = self {
            kv.0 == *key
        } else {
            false
        }
    }

    /// Get the value of a used slot.
    fn get_value(&self) -> Option<&Val> {
        if let KeyValueSlot::Used(kv) = self {
            Some(&kv.1)
        } else {
            None
        }
    }

    /// Update the value of a used slot.
    fn update_value(&mut self, val: Val) {
        if let KeyValueSlot::Used(kv) = self {
            kv.1 = val
        }
    }
}

/// A small, fixed-size, heap-allocated key/value cache with retention management.
pub struct MemoCache<Key, Val, const SIZE: usize> {
    buffer: Vec<KeyValueSlot<Key, Val>>,
    cursor: usize,
}

impl<Key, Val, const SIZE: usize> MemoCache<Key, Val, SIZE>
where
    Key: Clone + Eq,
    Val: Clone,
{
    /// Create a new cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let c = MemoCache::<u32, String, 4>::new();
    /// ```
    pub fn new() -> Self {
        let mut buffer = Vec::new();
        buffer.resize(SIZE, KeyValueSlot::Empty);

        Self { buffer, cursor: 0 }
    }

    /// Get the (fixed) capacity of the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let c = MemoCache::<u32, String, 8>::new();
    ///
    /// assert_eq!(c.capacity(), 8);
    /// ```
    pub const fn capacity(&self) -> usize {
        SIZE
    }

    /// Insert a key/value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, String, 4>::new();
    ///
    /// assert!(c.get(42).is_none());
    ///
    /// c.insert(42, "The Answer".to_owned());
    ///
    /// assert!(c.get(42).is_some_and(|v| v == "The Answer"));
    /// ```
    pub fn insert(&mut self, key: Key, val: Val) {
        match self.buffer.iter_mut().find(|e| e.is_key(&key)) {
            Some(s) => s.update_value(val),
            None => {
                *self
                    .buffer
                    .get_mut(self.cursor)
                    .expect("invalid cursor value") = KeyValueSlot::Used((key, val));

                // Move the cursor over the buffer elements sequentially, creating FIFO behavior.
                self.cursor = (self.cursor + 1) % SIZE;
            }
        }
    }

    /// Lookup a cache entry by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, String, 4>::new();
    ///
    /// assert!(c.get(42).is_none());
    ///
    /// c.insert(42, "The Answer".to_owned());
    ///
    /// assert!(c.get(42).is_some_and(|v| v == "The Answer"));
    /// ```
    pub fn get(&self, key: Key) -> Option<&Val> {
        self.buffer
            .iter()
            .find(|e| e.is_key(&key))
            .map(|e| e.get_value().unwrap())
    }
}

impl<Key, Val, const SIZE: usize> Default for MemoCache<Key, Val, SIZE>
where
    Key: Clone + Eq,
    Val: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_internal {
    use super::*;

    #[test]
    fn test_new_state() {
        const SIZE: usize = 8;

        let c = MemoCache::<i32, i32, SIZE>::new();

        // Verify cache size.
        assert_eq!(c.buffer.len(), SIZE);
        assert_eq!(c.capacity(), SIZE);

        // All slots should be empty.
        assert!(c.buffer.iter().all(|s| s == &KeyValueSlot::Empty));
    }

    #[test]
    fn test_cursor_state() {
        let mut c = MemoCache::<i32, i32, 2>::new();

        assert_eq!(c.cursor, 0);

        c.insert(1, 2);

        assert_eq!(c.cursor, 1);

        c.insert(3, 4);

        assert_eq!(c.cursor, 0);

        c.insert(5, 6);

        assert_eq!(c.cursor, 1);

        c.insert(7, 8);

        assert_eq!(c.cursor, 0);
    }
}
