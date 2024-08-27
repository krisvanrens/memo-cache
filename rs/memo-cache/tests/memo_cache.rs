mod tests_external {
    use memo_cache::MemoCache;

    #[test]
    fn test_empty() {
        let c = MemoCache::<bool, bool, 2>::new();

        assert_eq!(c.capacity(), 2);

        // NOTE: Even though the cache memory is pre-allocated, each cache slot should be marked as "empty".
        assert!(c.get(&true).is_none());
        assert!(c.get(&false).is_none());
    }

    #[test]
    fn test_nonempty() {
        let mut c = MemoCache::<String, i32, 3>::new();

        assert_eq!(c.capacity(), 3);

        let kvs = vec![
            ("veni".to_owned(), 19),
            ("vidi".to_owned(), 23),
            ("vici".to_owned(), 29),
        ];
        let kv0 = kvs.get(0).unwrap();
        let kv1 = kvs.get(1).unwrap();
        let kv2 = kvs.get(2).unwrap();

        assert!(c.get(&kv0.0).is_none());
        assert!(c.get(&kv1.0).is_none());
        assert!(c.get(&kv2.0).is_none());

        c.insert(kv0.0.clone(), kv0.1);

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));

        c.insert(kv1.0.clone(), kv1.1);

        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        c.insert(kv2.0.clone(), kv2.1);

        assert_eq!(c.get(&kv2.0), Some(&kv2.1));

        // The cache is now full, and another insertion will make the first key/value be removed.

        c.insert("blah".to_owned(), 42);

        assert!(c.get(&kv0.0).is_none());
        assert!(c.get(&kv1.0).is_some());
        assert!(c.get(&kv2.0).is_some());

        c.insert("bleh".to_owned(), 42);
        c.insert("bloh".to_owned(), 42);

        assert!(c.get(&kv1.0).is_none());
        assert!(c.get(&kv2.0).is_none());
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut c = MemoCache::<String, i32, 2>::new();

        let kv0 = ("John".to_owned(), 17);
        let kv1 = ("Doe".to_owned(), 19);

        assert!(c.get(&kv0.0).is_none());
        assert!(c.get(&kv1.0).is_none());

        c.insert(kv0.0.to_owned(), kv0.1);
        c.insert(kv1.0.to_owned(), kv1.1);

        assert!(c.get(&kv0.0).is_some());
        assert!(c.get(&kv1.0).is_some());

        // Inserting a duplicate key/value should be a no-op.

        c.insert(kv0.0.to_owned(), kv0.1);

        assert!(c.get(&kv0.0).is_some());
        assert!(c.get(&kv1.0).is_some());

        // Inserting a duplicate key with a new value should update the value.

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        c.insert(kv0.0.to_owned(), 42);

        assert_eq!(c.get(&kv0.0), Some(&42)); // Updated.
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));
    }
}
