use windows::{core::w, Win32::System::Registry::*};
use windows_registry::*;

#[test]
fn bad_string() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\bad_string";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    // Test value taken from https://github.com/rust-lang/rustup/blob/master/tests/suite/cli_paths.rs
    let bad_string_bytes = [
        0x00, 0xD8, // leading surrogate
        0x01, 0x01, // bogus trailing surrogate
        0x00, 0x00, // null
    ];

    unsafe {
        RegSetValueExW(
            HKEY(key.as_raw()),
            w!("name"),
            None,
            REG_SZ,
            Some(&bad_string_bytes),
        )
        .ok()?;
    }

    let ty = key.get_type("name")?;
    assert_eq!(ty, Type::String);

    let value_as_hstring = key.get_hstring("name")?;
    assert_eq!(value_as_hstring.to_string_lossy(), "�ā");

    let value = key.get_value("name")?;
    assert_eq!(*value, bad_string_bytes);

    Ok(())
}
