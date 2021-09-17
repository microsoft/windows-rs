use test_win32_handles::*;
use windows::*;
use Windows::Win32::Foundation::*;
use Windows::Win32::System::Diagnostics::Debug::*;

#[test]
fn hwnd() {
    let handle = HWND(0);
    let _clone = handle.clone();
    let _copy: HWND = handle;
    assert!(HWND::default() == HWND(0));
    assert!(HWND(0).is_invalid());
    assert_eq!(format!("{:?}", HWND::default()), "HWND(0)");

    assert!(HWND(1).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE.0) };

    assert!(HWND(0).ok().unwrap_err().code() == ERROR_INVALID_WINDOW_HANDLE.into());

    assert!(std::mem::size_of::<HWND>() == std::mem::size_of::<usize>());
}

#[test]
fn handle() {
    let handle = HANDLE(0);
    let _clone = handle.clone();
    let _copy: HANDLE = handle;
    assert!(HANDLE::default() == HANDLE(0));
    assert!(HANDLE(0).is_invalid());
    assert!(HANDLE(-1).is_invalid());
    assert_eq!(format!("{:?}", HANDLE::default()), "HANDLE(0)");

    assert!(HANDLE(1).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE.0) };

    assert!(HANDLE(0).ok().unwrap_err().code() == ERROR_INVALID_WINDOW_HANDLE.into());

    unsafe { SetLastError(ERROR_FILE_NOT_FOUND.0) };

    assert!(HANDLE(-1).ok().unwrap_err().code() == ERROR_FILE_NOT_FOUND.into());

    assert!(std::mem::size_of::<HANDLE>() == std::mem::size_of::<usize>());
}

#[test]
fn boolean() {
    // Although BOOLEAN is considered a Win32 handle type, it is not pointer-sized like most handle types.
    // This test just validates that such types have the correct layout.
    assert!(std::mem::size_of::<BOOLEAN>() == 1);
}

#[test]
fn pstr() {
    let handle = PSTR(std::ptr::null_mut());
    let _clone = handle.clone();
    let _copy: PSTR = handle;
    assert!(PSTR::default() == unsafe { std::mem::zeroed() });
    assert_eq!(format!("{:?}", PSTR::default()), "PSTR(0x0)");
}

#[test]
fn pwstr() {
    let handle = PWSTR(std::ptr::null_mut());
    let _clone = handle.clone();
    let _copy: PWSTR = handle;
    assert!(PWSTR::default() == unsafe { std::mem::zeroed() });
    assert_eq!(format!("{:?}", PWSTR::default()), "PWSTR(0x0)");
}
