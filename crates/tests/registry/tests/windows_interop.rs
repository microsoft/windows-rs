use windows::{core::*, Win32::System::Registry::*};
use windows_registry::*;

#[test]
fn windows_interop() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\windows_interop";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER.create(test_key)?;
    key.set_u32("1", 1)?;
    key.set_u32("2", 2)?;
    key.set_u32("3", 3)?;

    let raw = HKEY(key.as_raw());
    std::mem::forget(key);
    let owned = unsafe { Key::from_raw(raw.0) };

    let mut count = 0;

    unsafe {
        RegQueryInfoKeyW(
            HKEY(owned.as_raw()),
            PWSTR::null(),
            None,
            None,
            None,
            None,
            None,
            Some(&mut count),
            None,
            None,
            None,
            None,
        )
        .ok()?;
    };

    assert_eq!(count, 3);
    Ok(())
}
