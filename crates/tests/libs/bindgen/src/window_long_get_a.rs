#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn GetWindowLongA(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowLongA(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> i32);
    unsafe { GetWindowLongA(hwnd, nindex) }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[inline]
pub unsafe fn GetWindowLongPtrA(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize {
    windows_core::link!("user32.dll" "system" fn GetWindowLongPtrA(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
    unsafe { GetWindowLongPtrA(hwnd, nindex) }
}
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongA as GetWindowLongPtrA;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}
impl Default for HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWND(pub *mut core::ffi::c_void);
impl HWND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HANDLE> for HWND {}
impl From<HWND> for HANDLE {
    fn from(value: HWND) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOW_LONG_PTR_INDEX(pub i32);
