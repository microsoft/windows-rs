use windows_registry::*;

#[test]
fn volatile() {
    let test_key = "software\\windows-rs\\tests\\volatile";
    _ = CURRENT_USER.remove_tree(test_key);

    // Create a volatile key - this key will not persist when the system restarts
    let key = CURRENT_USER
        .options()
        .create()
        .volatile()
        .write()
        .open(test_key)
        .unwrap();

    // Write some data to the volatile key
    key.set_u64("test_value", 12345u64).unwrap();

    // Verify we can read the data
    assert_eq!(key.get_u64("test_value").unwrap(), 12345u64);

    // Verify the key was created (can be opened without create)
    let read_key = CURRENT_USER.options().read().open(test_key).unwrap();

    assert_eq!(read_key.get_u64("test_value").unwrap(), 12345u64);

    // Clean up
    _ = CURRENT_USER.remove_tree(test_key);
}

#[test]
fn non_volatile_default() {
    let test_key = "software\\windows-rs\\tests\\non_volatile_default";
    _ = CURRENT_USER.remove_tree(test_key);

    // Create a key without specifying volatile (should default to non-volatile)
    let key = CURRENT_USER
        .options()
        .create()
        .write()
        .open(test_key)
        .unwrap();

    // Write some data
    key.set_u64("test_value", 67890u64).unwrap();

    // Verify we can read the data
    assert_eq!(key.get_u64("test_value").unwrap(), 67890u64);

    // Clean up
    _ = CURRENT_USER.remove_tree(test_key);
}
