#[cfg(feature = "shtypes")]
pub type LPCWINDOWDATA = *const WINDOWDATA;
#[cfg(feature = "shtypes")]
pub type LPWINDOWDATA = *mut WINDOWDATA;
pub const TLEF_ABSOLUTE: tagTLENUMF = 49;
pub const TLEF_EXCLUDE_ABOUT_PAGES: tagTLENUMF = 256;
pub const TLEF_EXCLUDE_SUBFRAME_ENTRIES: tagTLENUMF = 128;
pub const TLEF_INCLUDE_UNINVOKEABLE: tagTLENUMF = 64;
pub const TLEF_RELATIVE_BACK: tagTLENUMF = 16;
pub const TLEF_RELATIVE_FORE: tagTLENUMF = 32;
pub const TLEF_RELATIVE_INCLUDE_CURRENT: tagTLENUMF = 1;
pub type TLENUMF = u32;
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct WINDOWDATA {
    pub dwWindowID: u32,
    pub uiCP: u32,
    pub pidl: super::LPITEMIDLIST,
    pub lpszUrl: windows_sys::core::PWSTR,
    pub lpszUrlLocation: windows_sys::core::PWSTR,
    pub lpszTitle: windows_sys::core::PWSTR,
}
#[cfg(feature = "shtypes")]
impl Default for WINDOWDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagTLENUMF = i32;
