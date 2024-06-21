use windows::{core::w, Win32::System::Registry::*};
use windows_registry::*;

#[test]
fn bad_string() -> Result<()> {
    let bad_string_bytes = vec![
        0x00, 0xD8, // leading surrogate
        0x01, 0x01, // bogus trailing surrogate
        0x00, 0x00, // null
    ];

    let test_key = "software\\windows-rs\\tests\\bad_string";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    unsafe {
        RegSetValueExW(
            HKEY(key.as_raw()),
            w!("name"),
            0,
            REG_SZ,
            Some(&bad_string_bytes),
        )
        .ok()?
    };

    let ty = key.get_type("name")?;
    assert_eq!(ty, Type::String);

    let value_as_string = key.get_string("name")?;
    assert_eq!(value_as_string, "�ā");

    let value_as_bytes = key.get_bytes("name")?;
    assert_eq!(value_as_bytes, bad_string_bytes);
    Ok(())
}
