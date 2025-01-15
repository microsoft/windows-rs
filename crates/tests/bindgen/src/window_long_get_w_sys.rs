#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_link::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
windows_link::link!("user32.dll" "system" fn GetWindowLongW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> i32);
pub type HANDLE = *mut core::ffi::c_void;
pub type HWND = *mut core::ffi::c_void;
pub type WINDOW_LONG_PTR_INDEX = i32;
