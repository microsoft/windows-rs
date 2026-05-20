pub type GetWindowLongA =
    unsafe extern "system" fn(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32;
windows_link::link!("user32.dll" "system" fn GetWindowLongA(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> i32);
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type GetWindowLongPtrA =
    unsafe extern "system" fn(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_link::link!("user32.dll" "system" fn GetWindowLongPtrA(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongA as GetWindowLongPtrA;
pub type HANDLE = *mut core::ffi::c_void;
pub type HWND = *mut core::ffi::c_void;
pub type WINDOW_LONG_PTR_INDEX = i32;
