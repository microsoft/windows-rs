use windows_registry::*;
use windows_result::*;
use windows_sys::Win32::{Foundation::*, System::Registry::*};

#[test]
fn access() {
    let test_key = "software\\windows-rs\\tests\\access";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER
        .options()
        .create()
        .access(KEY_WRITE)
        .open(test_key)
        .unwrap();

    key.set_u64("u64", 123u64).unwrap();

    assert_eq!(
        key.get_u64("u64").unwrap_err().code(),
        HRESULT::from_win32(ERROR_ACCESS_DENIED)
    );

    let key = CURRENT_USER
        .options()
        .access(KEY_READ)
        .open(test_key)
        .unwrap();

    assert_eq!(key.get_u64("u64").unwrap(), 123u64);

    assert_eq!(
        key.set_u64("u64", 123u64).unwrap_err().code(),
        HRESULT::from_win32(ERROR_ACCESS_DENIED)
    );
}

#[test]
fn flags() {
    // `OpenOptions` defaults to no access
    let mut options = CURRENT_USER.options();
    assert_eq!(get_access(&options), 0);

    // `read` and `write` equate to `KEY_READ` and `KEY_WRITE`
    options.read().write();
    assert_eq!(get_access(&options), KEY_READ | KEY_WRITE);

    // Combine additional access rights
    options.access(KEY_WOW64_32KEY);
    assert_eq!(get_access(&options), KEY_WOW64_32KEY | KEY_READ | KEY_WRITE);

    // Start with specific access rights
    let mut options = CURRENT_USER.options();
    options.access(KEY_WOW64_32KEY | KEY_QUERY_VALUE);
    assert_eq!(get_access(&options), KEY_WOW64_32KEY | KEY_QUERY_VALUE);

    // `read` is additive
    options.read();
    assert_eq!(
        get_access(&options),
        KEY_WOW64_32KEY | KEY_QUERY_VALUE | KEY_READ
    );

    // `write` is additive
    options.write();
    assert_eq!(
        get_access(&options),
        KEY_WOW64_32KEY | KEY_QUERY_VALUE | KEY_READ | KEY_WRITE
    );
}

fn get_access(options: &OpenOptions) -> u32 {
    regex::Regex::new(r#"access:\s*(\d+)"#)
        .unwrap()
        .captures(&format!("{options:?}"))
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}
