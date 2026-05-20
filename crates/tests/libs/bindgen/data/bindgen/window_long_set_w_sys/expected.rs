#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type SetWindowLongPtrW =
    unsafe extern "system" fn(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_link::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : isize) -> isize);
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
pub type SetWindowLongW =
    unsafe extern "system" fn(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32;
windows_link::link!("user32.dll" "system" fn SetWindowLongW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : i32) -> i32);
pub type HANDLE = *mut core::ffi::c_void;
pub type HWND = *mut core::ffi::c_void;
pub type WINDOW_LONG_PTR_INDEX = i32;
