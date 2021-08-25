use test_pwstr::Windows::Win32::Foundation::PWSTR;
use std::ffi::{OsStr, OsString};
use windows::IntoParam;

#[test]
fn test() {

    let expected = "hello\0";
    let len = expected.chars().count();

    let p = get_pwstr_from("hello");
    assert_eq!(from_parts(p.0, len), expected);

    let p = get_pwstr_from(String::from("hello"));
    assert_eq!(from_parts(p.0, len), expected);

    let p = get_pwstr_from(OsStr::new("hello"));
    assert_eq!(from_parts(p.0, len), expected);

    let p = get_pwstr_from(OsString::from("hello"));
    assert_eq!(from_parts(p.0, len), expected);
}

fn get_pwstr_from<'a>(t: impl IntoParam<'a, PWSTR>) -> PWSTR {
    t.into_param().abi()
}

fn from_parts(p: *const u16, len: usize) -> String {
    let buf = unsafe { std::slice::from_raw_parts(p, len) };
    String::from_utf16_lossy(buf)
}
