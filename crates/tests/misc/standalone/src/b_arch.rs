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
windows_targets::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
pub type HWND = *mut core::ffi::c_void;
pub type PSTR = *mut u8;
pub type WINDOW_LONG_PTR_INDEX = i32;
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
}
