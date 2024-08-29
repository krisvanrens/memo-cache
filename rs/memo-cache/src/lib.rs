use core::borrow::Borrow;

/// Key equivalence trait, to support `Borrow` types as keys.
trait Equivalent<K: ?Sized> {
    /// Returns `true` if two values are equivalent, `false` if otherwise.
    fn equivalent(&self, k: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
where
    Q: Eq,
    K: Borrow<Q>,
{
    #[inline]
    fn equivalent(&self, k: &K) -> bool {
        self == k.borrow()
    }
}

/// A single key/value slot used in the cache.
#[derive(Clone, PartialEq)]
enum KeyValueSlot<K, V> {
    Used((K, V)),
    Empty,
}

impl<K, V> KeyValueSlot<K, V> {
    /// Check a used slot key for equivalence.
    #[inline]
    fn is_key<Q>(&self, k: &Q) -> bool
    where
        Q: Equivalent<K> + ?Sized,
    {
        if let KeyValueSlot::Used(kv) = self {
            k.equivalent(&kv.0)
        } else {
            false
        }
    }

    /// Get the value of a used slot.
    #[inline]
    fn get_value(&self) -> Option<&V> {
        if let KeyValueSlot::Used(kv) = self {
            Some(&kv.1)
        } else {
            None
        }
    }

    /// Update the value of a used slot.
    #[inline]
    fn update_value(&mut self, v: V) {
        if let KeyValueSlot::Used(kv) = self {
            kv.1 = v
        }
    }
}

/// A small, fixed-size, heap-allocated key/value cache with retention management.
pub struct MemoCache<K, V, const SIZE: usize> {
    buffer: Vec<KeyValueSlot<K, V>>,
    cursor: usize,
}

impl<K, V, const SIZE: usize> MemoCache<K, V, SIZE>
where
    K: Clone + Eq,
    V: Clone,
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
    #[inline]
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
    #[inline]
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
    /// assert!(c.get(&42).is_none());
    ///
    /// c.insert(42, "The Answer".to_owned());
    ///
    /// assert!(c.get(&42).is_some_and(|v| v == "The Answer"));
    /// ```
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        match self.buffer.iter_mut().find(|e| e.is_key(&k)) {
            Some(s) => s.update_value(v),
            None => {
                *self
                    .buffer
                    .get_mut(self.cursor)
                    .expect("invalid cursor value") = KeyValueSlot::Used((k, v));

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
    /// assert!(c.get(&42).is_none());
    ///
    /// c.insert(42, "The Answer".to_owned());
    ///
    /// assert!(c.get(&42).is_some_and(|v| v == "The Answer"));
    /// ```
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.buffer
            .iter()
            .find(|e| e.is_key(k))
            .map(|e| e.get_value().unwrap())
    }
}

impl<K, V, const SIZE: usize> Default for MemoCache<K, V, SIZE>
where
    K: Clone + Eq,
    V: Clone,
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
