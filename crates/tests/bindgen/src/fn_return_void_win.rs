#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS) {
    windows_targets::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : *mut MEMORYSTATUS));
    unsafe { GlobalMemoryStatus(lpbuffer as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
impl Default for MEMORYSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
