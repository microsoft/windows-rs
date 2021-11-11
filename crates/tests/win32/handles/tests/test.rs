use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Registry::*;
use windows::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST;

#[test]
fn hwnd() {
    let handle = HWND(0);
    let _clone = handle.clone();
    let _copy: HWND = handle;
    assert!(HWND::default() == HWND(0));
    assert!(HWND(0).is_invalid());
    assert_eq!(format!("{:?}", HWND::default()), "HWND(0)");

    assert!(HWND(1).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE) };

    assert!(HWND(0).ok().unwrap_err().code() == ERROR_INVALID_WINDOW_HANDLE.into());

    assert!(core::mem::size_of::<HWND>() == core::mem::size_of::<usize>());
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

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE) };

    assert!(HANDLE(0).ok().unwrap_err().code() == ERROR_INVALID_WINDOW_HANDLE.into());

    unsafe { SetLastError(ERROR_FILE_NOT_FOUND) };

    assert!(HANDLE(-1).ok().unwrap_err().code() == ERROR_FILE_NOT_FOUND.into());

    assert!(core::mem::size_of::<HANDLE>() == core::mem::size_of::<usize>());
}

#[test]
fn boolean() {
    // Although BOOLEAN is considered a Win32 handle type, it is not pointer-sized like most handle types.
    // This test just validates that such types have the correct layout.
    assert!(core::mem::size_of::<BOOLEAN>() == 1);
}

#[test]
fn pstr() {
    let handle = PSTR(core::ptr::null_mut());
    let _clone = handle.clone();
    let _copy: PSTR = handle;
    assert!(handle.is_null());
    assert!(PSTR::default() == unsafe { core::mem::zeroed() });
    assert_eq!(format!("{:?}", PSTR::default()), "PSTR(0x0)");
}

#[test]
fn pwstr() {
    let handle = PWSTR(core::ptr::null_mut());
    let _clone = handle.clone();
    let _copy: PWSTR = handle;
    assert!(handle.is_null());
    assert!(PWSTR::default() == unsafe { core::mem::zeroed() });
    assert_eq!(format!("{:?}", PWSTR::default()), "PWSTR(0x0)");
}

#[test]
fn lpproc_thread_attribute_list() {
    // This is an interesting test because this handle type has a pointer field unlike most others.
    let handle = LPPROC_THREAD_ATTRIBUTE_LIST(core::ptr::null_mut());
    let _clone = handle.clone();
    let _copy: LPPROC_THREAD_ATTRIBUTE_LIST = handle;
    assert!(LPPROC_THREAD_ATTRIBUTE_LIST::default() == LPPROC_THREAD_ATTRIBUTE_LIST(core::ptr::null_mut()));
    assert!(LPPROC_THREAD_ATTRIBUTE_LIST::default().is_invalid());
    assert_eq!(format!("{:?}", LPPROC_THREAD_ATTRIBUTE_LIST::default()), "LPPROC_THREAD_ATTRIBUTE_LIST(0x0)");

    assert!(LPPROC_THREAD_ATTRIBUTE_LIST(1 as _).ok().is_ok());

    unsafe { SetLastError(ERROR_INVALID_WINDOW_HANDLE) };

    assert!(LPPROC_THREAD_ATTRIBUTE_LIST::default().ok().unwrap_err().code() == ERROR_INVALID_WINDOW_HANDLE.into());

    assert!(core::mem::size_of::<LPPROC_THREAD_ATTRIBUTE_LIST>() == core::mem::size_of::<usize>());
}

#[test]
fn hbitmap() {
    fn expect_object<'a>(value: impl IntoParam<'a, HGDIOBJ>) {
        unsafe {
            assert!(value.into_param().abi().0 == 123);
        }
    }

    expect_object(HBITMAP(123));
    expect_object(HGDIOBJ(123));
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
