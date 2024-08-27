mod tests_external {
    use memo_cache::MemoCache;

    #[test]
    fn test_empty() {
        let c = MemoCache::<bool, bool, 2>::new();

        // NOTE: Even though the cache memory is pre-allocated, each cache slot should be marked as "empty".
        assert!(c.find(true).is_none());
        assert!(c.find(false).is_none());
    }

    #[test]
    fn test_nonempty() {
        let mut c = MemoCache::<String, i32, 3>::new();

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
        let mut c = MemoCache::<String, i32, 2>::new();

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
