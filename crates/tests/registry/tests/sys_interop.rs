use windows_registry::*;
use windows_sys::Win32::System::Registry::*;

#[test]
fn sys_interop() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\sys_interop";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER.create(test_key)?;
    key.set_u32("1", 1)?;
    key.set_u32("2", 2)?;
    key.set_u32("3", 3)?;

    let raw: HKEY = key.as_raw();
    std::mem::forget(key);
    let owned = unsafe { Key::from_raw(raw) };

    let mut count = 0;

    unsafe {
        RegQueryInfoKeyW(
            owned.as_raw(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut count,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    assert_eq!(count, 3);
    Ok(())
}
