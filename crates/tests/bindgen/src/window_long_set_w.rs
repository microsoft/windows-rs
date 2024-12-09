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
#[inline]
pub unsafe fn SetWindowLongPtrW(
    hwnd: HWND,
    nindex: WINDOW_LONG_PTR_INDEX,
    dwnewlong: isize,
) -> isize {
    windows_targets::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : isize) -> isize);
    SetWindowLongPtrW(
        core::mem::transmute(hwnd),
        core::mem::transmute(nindex),
        core::mem::transmute(dwnewlong),
    )
}
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
#[inline]
pub unsafe fn SetWindowLongW(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32 {
    windows_targets::link!("user32.dll" "system" fn SetWindowLongW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : i32) -> i32);
    SetWindowLongW(
        core::mem::transmute(hwnd),
        core::mem::transmute(nindex),
        core::mem::transmute(dwnewlong),
    )
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);
impl windows_core::TypeKind for HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : *mut core::ffi::c_void) -> i32);
            CloseHandle(self.0);
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
impl windows_core::TypeKind for HWND {
    type TypeKind = windows_core::CopyType;
}
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
