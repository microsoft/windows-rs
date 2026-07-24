#![cfg(windows)]
use windows_registry::*;
use windows_result::*;
use windows_sys::Win32::*;

#[test]
fn access() {
    let test_key = "software\\windows-rs\\tests\\access";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER
        .options()
        .create()
        .access(KEY_WRITE as u32)
        .open(test_key)
        .unwrap();

    key.set_u64("u64", 123u64).unwrap();

    assert_eq!(
        key.get_u64("u64").unwrap_err().code(),
        WIN32_ERROR(ERROR_ACCESS_DENIED as u32).into()
    );

    let key = CURRENT_USER
        .options()
        .access(KEY_READ as u32)
        .open(test_key)
        .unwrap();

    assert_eq!(key.get_u64("u64").unwrap(), 123u64);

    assert_eq!(
        key.set_u64("u64", 123u64).unwrap_err().code(),
        WIN32_ERROR(ERROR_ACCESS_DENIED as u32).into()
    );
}

#[test]
fn flags() {
    // `OpenOptions` defaults to no access
    let mut options = CURRENT_USER.options();
    assert_eq!(get_access(&options), 0);

    // `read` and `write` equate to `KEY_READ` and `KEY_WRITE`
    options.read().write();
    assert_eq!(get_access(&options), (KEY_READ | KEY_WRITE) as u32);

    // Combine additional access rights
    options.access(KEY_WOW64_32KEY as u32);
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_32KEY | KEY_READ | KEY_WRITE) as u32
    );

    // Start with specific access rights
    let mut options = CURRENT_USER.options();
    options.access((KEY_WOW64_32KEY | KEY_QUERY_VALUE) as u32);
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_32KEY | KEY_QUERY_VALUE) as u32
    );

    // `read` is additive
    options.read();
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_32KEY | KEY_QUERY_VALUE | KEY_READ) as u32
    );

    // `write` is additive
    options.write();
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_32KEY | KEY_QUERY_VALUE | KEY_READ | KEY_WRITE) as u32
    );
}

#[test]
fn wow64() {
    // `wow64_32` and `wow64_64` select the 32-bit / 64-bit registry view.
    let mut options = CURRENT_USER.options();
    options.wow64_32();
    assert_eq!(get_access(&options), KEY_WOW64_32KEY as u32);

    // The two views are mutually exclusive; the last call wins.
    options.wow64_64();
    assert_eq!(get_access(&options), KEY_WOW64_64KEY as u32);
    options.wow64_32();
    assert_eq!(get_access(&options), KEY_WOW64_32KEY as u32);

    // The view flag is additive with `read`/`write` and other access rights.
    options.read().write().access(KEY_QUERY_VALUE as u32);
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_32KEY | KEY_READ | KEY_WRITE | KEY_QUERY_VALUE) as u32
    );

    // Switching views preserves the other access bits.
    options.wow64_64();
    assert_eq!(
        get_access(&options),
        (KEY_WOW64_64KEY | KEY_READ | KEY_WRITE | KEY_QUERY_VALUE) as u32
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
