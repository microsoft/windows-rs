#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
::windows_targets::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
pub type HWND = isize;
pub type PSTR = *mut u8;
pub type WINDOW_LONG_PTR_INDEX = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl Copy for WSADATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl Clone for WSADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
}
#[cfg(target_arch = "x86")]
impl Copy for WSADATA {}
#[cfg(target_arch = "x86")]
impl Clone for WSADATA {
    fn clone(&self) -> Self {
        *self
    }
}
