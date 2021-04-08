use test_handles::{
    Windows::Win32::Gdi::HGDIOBJ,
    Windows::Win32::SystemServices::{HANDLE, PSTR, PWSTR},
};

#[test]
fn test() {
    assert_eq!(HANDLE::NULL, HANDLE(0));
    assert_eq!(HANDLE::NULL.is_null(), true);
    assert_eq!(HANDLE::NULL.is_invalid(), false);

    assert_eq!(HANDLE::INVALID, HANDLE(-1));
    assert_eq!(HANDLE::INVALID.is_null(), false);
    assert_eq!(HANDLE::INVALID.is_invalid(), true);

    assert_eq!(PSTR::NULL, PSTR(std::ptr::null_mut()));
    assert_eq!(PSTR::NULL.is_null(), true);

    assert_eq!(PWSTR::NULL, PWSTR(std::ptr::null_mut()));
    assert_eq!(PWSTR::NULL.is_null(), true);

    assert_eq!(HGDIOBJ::NULL, HGDIOBJ(0));
    assert_eq!(HGDIOBJ::NULL.is_null(), true);
    assert_eq!(HGDIOBJ(1).is_null(), false);
}
