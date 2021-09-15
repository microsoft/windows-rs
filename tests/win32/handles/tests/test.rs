use test_win32_handles::*;
use windows::*;
use Windows::Win32::Foundation::HWND;
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
    assert!(HWND::NULL.ok().unwrap_err().code() == HRESULT::from_win32(ERROR_INVALID_WINDOW_HANDLE.0));
}
