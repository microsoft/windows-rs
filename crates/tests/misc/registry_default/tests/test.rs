#![cfg(windows)]
// The default `std` feature must enable `std` for `windows-result` and `windows-strings`.
//
// `Box<dyn std::error::Error>` requires `std` from `windows-result`.
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let test_key = "software\\windows-rs\\tests\\default";
    _ = windows_registry::CURRENT_USER.remove_tree(test_key);
    let key = windows_registry::CURRENT_USER.create(test_key)?;

    key.set_u32("u32", 123u32)?;
    assert_eq!(key.get_u32("u32")?, 123u32);

    // `to_os_string` requires `std` from `windows-strings`.
    assert_eq!(
        windows_registry::HSTRING::from("value").to_os_string(),
        "value"
    );

    Ok(())
}
