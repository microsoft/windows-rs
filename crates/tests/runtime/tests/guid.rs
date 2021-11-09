use windows::runtime::GUID;

    #[test]
    fn test_new() {
        let zeroed = GUID::zeroed();
        let unique = GUID::new().unwrap();
        assert_ne!(zeroed, unique);
    }
