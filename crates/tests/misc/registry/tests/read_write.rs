use windows_registry::*;
use windows_result::*;
use windows_sys::Win32::Foundation::*;

#[test]
fn read_write() {
    let test_key = "software\\windows-rs\\tests\\read_write";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER
        .options()
        .create()
        .write()
        .open(test_key)
        .unwrap();

    key.set_u64("u64", 123u64).unwrap();

    assert_eq!(
        key.get_u64("u64").unwrap_err().code(),
        HRESULT::from_win32(ERROR_ACCESS_DENIED)
    );

    let key = CURRENT_USER.options().read().open(test_key).unwrap();

    assert_eq!(key.get_u64("u64").unwrap(), 123u64);

    assert_eq!(
        key.set_u64("u64", 123u64).unwrap_err().code(),
        HRESULT::from_win32(ERROR_ACCESS_DENIED)
    );
}
