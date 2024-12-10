use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Registry::*;

#[test]
fn handle() {
    let handle = HANDLE(0 as _);
    let _clone = handle;
    let _copy: HANDLE = handle;
    assert!(HANDLE::default() == HANDLE(0 as _));
    assert!(HANDLE(0 as _).is_invalid());
    assert!(HANDLE(-1 as _).is_invalid());

    assert!(core::mem::size_of::<HANDLE>() == core::mem::size_of::<usize>());
}

#[test]
fn pstr() {
    let handle = PSTR(core::ptr::null_mut());
    let _clone = handle;
    let _copy: PSTR = handle;
    assert!(handle.is_null());
}

#[test]
fn pwstr() {
    let handle = PWSTR::null();
    let _clone = handle;
    let _copy: PWSTR = handle;
    assert!(handle.is_null());
}

#[test]
fn hkey() {
    // This test validates that handle constants can be used in match constant expressions.
    // This requires PartialEq and Eq to be derived.
    fn to_string(key: HKEY) -> &'static str {
        match key {
            HKEY_CURRENT_USER => "HKCU",
            HKEY_LOCAL_MACHINE => "HKLM",
            _ => "",
        }
    }

    assert!(to_string(HKEY_CURRENT_USER) == "HKCU");
    assert!(to_string(HKEY_LOCAL_MACHINE) == "HKLM");
}
