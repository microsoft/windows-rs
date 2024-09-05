// This just confirms that the default feature "std" for the `windows-registry` crate also enables the
// default feature "std" for both the `windows-result` and `windows-strings` dependencies.
//
// `Box<dyn std::error::Error>` requires "std" feature for the `windows-result` crate.
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let test_key = "software\\windows-rs\\tests\\default";
    _ = windows_registry::CURRENT_USER.remove_tree(test_key);
    let key = windows_registry::CURRENT_USER.create(test_key)?;

    key.set_u32("u32", 123u32)?;
    assert_eq!(key.get_u32("u32")?, 123u32);

    // `to_os_string` requires the "std" feature for the `windows-strings` crate.
    assert_eq!(
        windows_registry::HSTRING::from("value").to_os_string(),
        "value"
    );

    Ok(())
}
