use windows_registry::*;

#[test]
fn volatile_key() {
    let test_key = "software\\windows-rs\\tests\\volatile";
    _ = CURRENT_USER.remove_tree(test_key);

    // Create a volatile key - this key will not persist when the system restarts
    let key = CURRENT_USER
        .options()
        .create()
        .volatile()
        .read()
        .write()
        .open(test_key)
        .unwrap();

    // Write some data to the volatile key
    key.set_u64("test_value", 12345u64).unwrap();

    // Verify we can read the data
    assert_eq!(key.get_u64("test_value").unwrap(), 12345u64);
}
