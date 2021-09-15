use test_win32_handles::*;
use windows::*;
use Windows::Win32::Foundation::*;
use Windows::Win32::System::Diagnostics::Debug::*;

#[test]
fn hwnd() {
    let handle = HWND::NULL;
    let _clone = handle.clone();
    let _copy: HWND = handle;
    assert!(HWND::default() == HWND::NULL);
    assert!(HWND(0) == HWND::NULL);
    assert_eq!(format!("{:?}", HWND::NULL), "HWND(0)");
    assert!(HWND::NULL.is_null());

    assert!(HWND(1).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE.0) };

    assert!(
        HWND::NULL.ok().unwrap_err().code() == HRESULT::from_win32(ERROR_INVALID_WINDOW_HANDLE.0)
    );

    assert!(std::mem::size_of::<HWND>() == std::mem::size_of::<usize>());
}

#[test]
fn handle() {
    let handle = HANDLE::NULL;
    let _clone = handle.clone();
    let _copy: HANDLE = handle;
    assert!(HANDLE::default() == HANDLE::NULL);
    assert!(HANDLE(0) == HANDLE::NULL);
    assert!(HANDLE(-1) == HANDLE::INVALID);
    assert_eq!(format!("{:?}", HANDLE::NULL), "HANDLE(0)");
    assert!(HANDLE::NULL.is_null());
    assert!(!HANDLE::INVALID.is_null());
    assert!(!HANDLE::NULL.is_invalid());
    assert!(HANDLE::INVALID.is_invalid());

    assert!(HANDLE(1).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE.0) };

    assert!(
        HANDLE::NULL.ok().unwrap_err().code() == HRESULT::from_win32(ERROR_INVALID_WINDOW_HANDLE.0)
    );

    unsafe { SetLastError(ERROR_FILE_NOT_FOUND.0) };

    assert!(
        HANDLE::INVALID.ok().unwrap_err().code() == HRESULT::from_win32(ERROR_FILE_NOT_FOUND.0)
    );

    assert!(std::mem::size_of::<HANDLE>() == std::mem::size_of::<usize>());
}

#[test]
fn boolean() {
    // Although BOOLEAN is considered a Win32 handle type, it is not pointer-sized like most handle types.
    // This test just validates that such types have the correct layout.
    assert!(std::mem::size_of::<BOOLEAN>() == 1);
}
