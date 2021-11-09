use windows::runtime::Array;

    #[test]
    fn empty() {
        let empty = Array::<bool>::new();
        assert!(empty.is_empty());
        assert!(empty.is_empty());
    }

    #[test]
    fn with_len() {
        let empty = Array::<u32>::with_len(3);
        assert!(!empty.is_empty());
        assert!(empty.len() == 3);
        assert!(empty[0] == 0);
        assert!(empty[1] == 0);
        assert!(empty[2] == 0);
    }
