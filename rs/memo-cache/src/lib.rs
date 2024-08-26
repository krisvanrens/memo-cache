/// A single key/value slot used in the cache.
#[derive(Clone)]
struct KeyValueSlot<Key, Val> {
    key: Key,
    val: Val,
    empty: bool,
}

impl<Key, Val> KeyValueSlot<Key, Val>
where
    Key: Clone + Default,
    Val: Clone + Default,
{
    fn new() -> Self {
        Self {
            key: Key::default(),
            val: Val::default(),
            empty: true,
        }
    }
}

/// A small, fixed-size, heap-allocated key/value cache with retention management.
pub struct MemoCache<Key, Val> {
    buffer: Vec<KeyValueSlot<Key, Val>>,
    cursor: usize,
}

impl<Key, Val> MemoCache<Key, Val>
where
    Key: Clone + Default + Eq,
    Val: Clone + Default,
{
    /// Create a new cache with a fixed size.
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(size, KeyValueSlot::new());

        Self { buffer, cursor: 0 }
    }

    /// Get the (fixed) size of the cache.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Insert a key/value pair.
    pub fn insert(&mut self, key: Key, val: Val) {
        match self.buffer.iter_mut().find(|e| !e.empty && (e.key == key)) {
            Some(s) => s.val = val,
            None => {
                *self
                    .buffer
                    .get_mut(self.cursor)
                    .expect("invalid cursor value") = KeyValueSlot {
                    key,
                    val,
                    empty: false,
                };

                // Move the cursor over the buffer elements sequentially, creating FIFO behavior.
                self.cursor = (self.cursor + 1) % self.buffer.len();
            }
        }
    }

    /// Lookup a cache entry by key.
    pub fn find(&self, key: Key) -> Option<&Val> {
        self.buffer
            .iter()
            .find(|e| !e.empty && (e.key == key))
            .map(|e| &e.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        const SIZE: usize = 8;

        let c = MemoCache::<i32, i32>::new(SIZE);

        // Verify cache size.
        assert_eq!(c.len(), SIZE);
        assert_eq!(c.buffer.len(), SIZE);

        // All slots should be empty.
        assert!(c.buffer.iter().all(|s| s.empty));
    }

    #[test]
    fn test_empty() {
        let c = MemoCache::<bool, bool>::new(2);

        // NOTE: Even though the cache memory is pre-allocated, each cache slot should be marked as "empty".
        assert!(c.find(true).is_none());
        assert!(c.find(false).is_none());
    }

    #[test]
    fn test_cursor() {
        let mut c = MemoCache::<i32, i32>::new(2);

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

    #[test]
    fn test_nonempty() {
        let mut c = MemoCache::<String, i32>::new(3);

        let kvs = vec![("veni", 19), ("vidi", 23), ("vici", 29)];
        let kv0 = kvs.get(0).unwrap();
        let kv1 = kvs.get(1).unwrap();
        let kv2 = kvs.get(2).unwrap();

        assert!(c.find(kv0.0.to_owned()).is_none());
        assert!(c.find(kv1.0.to_owned()).is_none());
        assert!(c.find(kv2.0.to_owned()).is_none());

        c.insert(kv0.0.to_owned(), kv0.1);

        assert_eq!(c.find(kv0.0.to_owned()), Some(&kv0.1));

        c.insert(kv1.0.to_owned(), kv1.1);

        assert_eq!(c.find(kv1.0.to_owned()), Some(&kv1.1));

        c.insert(kv2.0.to_owned(), kv2.1);

        assert_eq!(c.find(kv2.0.to_owned()), Some(&kv2.1));

        // The cache is now full, and another insertion will make the first key/value be removed.

        c.insert("blah".to_owned(), 42);

        assert!(c.find(kv0.0.to_owned()).is_none());
        assert!(c.find(kv1.0.to_owned()).is_some());
        assert!(c.find(kv2.0.to_owned()).is_some());

        c.insert("bleh".to_owned(), 42);
        c.insert("bloh".to_owned(), 42);

        assert!(c.find(kv1.0.to_owned()).is_none());
        assert!(c.find(kv2.0.to_owned()).is_none());
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut c = MemoCache::<String, i32>::new(2);

        let kv0 = ("John", 17);
        let kv1 = ("Doe", 19);

        assert!(c.find(kv0.0.to_owned()).is_none());
        assert!(c.find(kv1.0.to_owned()).is_none());

        c.insert(kv0.0.to_owned(), kv0.1);
        c.insert(kv1.0.to_owned(), kv1.1);

        assert!(c.find(kv0.0.to_owned()).is_some());
        assert!(c.find(kv1.0.to_owned()).is_some());

        // Inserting a duplicate key/value should be a no-op.

        c.insert(kv0.0.to_owned(), kv0.1);

        assert!(c.find(kv0.0.to_owned()).is_some());
        assert!(c.find(kv1.0.to_owned()).is_some());

        // Inserting a duplicate key with a new value should update the value.

        assert_eq!(c.find(kv0.0.to_owned()), Some(&kv0.1));
        assert_eq!(c.find(kv1.0.to_owned()), Some(&kv1.1));

        c.insert(kv0.0.to_owned(), 42);

        assert_eq!(c.find(kv0.0.to_owned()), Some(&42)); // Updated.
        assert_eq!(c.find(kv1.0.to_owned()), Some(&kv1.1));
    }
}
