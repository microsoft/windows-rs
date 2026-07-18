#![cfg(windows)]
use windows::{core::w, Win32::HKEY, Win32::*};
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
        windows_result::WIN32_ERROR(
            RegSetValueExW(
                HKEY(key.as_raw()),
                w!("name"),
                None,
                REG_SZ,
                Some(&bad_string_bytes),
            )
            .0,
        )
        .ok()?;
    }

    let ty = key.get_type("name")?;
    assert_eq!(ty, Type::String);

    let value_as_hstring = key.get_hstring("name")?;
    assert_eq!(value_as_hstring.to_string_lossy(), "�ā");

    // get_string should fail because the data isn't valid UTF-16.
    assert!(key.get_string("name").is_err());

    // get_value still works for inspecting raw bytes.
    let value = key.get_value("name")?;
    assert_eq!(*value, bad_string_bytes);

    Ok(())
}

#[test]
fn bad_multi_string_missing_double_null() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\bad_multi_string";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    // REG_MULTI_SZ without the required double-null terminator.
    let bad_multi_bytes: &[u8] = &[
        b'a', 0, b'b', 0, // "ab" in UTF-16LE
        0, 0, // single null (separator), but no second null terminator
    ];

    unsafe {
        windows_result::WIN32_ERROR(
            RegSetValueExW(
                HKEY(key.as_raw()),
                w!("multi"),
                None,
                REG_MULTI_SZ,
                Some(bad_multi_bytes),
            )
            .0,
        )
        .ok()?;
    }

    // Parsing should fail because the double-null terminator is missing.
    assert!(key.get_multi_string("multi").is_err());

    Ok(())
}
