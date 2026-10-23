#[cfg(all(feature = "shared", feature = "shtypes"))]
pub type LPCWINDOWDATA = *const WINDOWDATA;
#[cfg(all(feature = "shared", feature = "shtypes"))]
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
#[cfg(all(feature = "shared", feature = "shtypes"))]
#[derive(Clone, Copy, Default)]
pub struct WINDOWDATA {
    pub dwWindowID: super::DWORD,
    pub uiCP: super::UINT,
    pub pidl: super::LPITEMIDLIST,
    pub lpszUrl: windows_sys::core::PWSTR,
    pub lpszUrlLocation: windows_sys::core::PWSTR,
    pub lpszTitle: windows_sys::core::PWSTR,
}
pub type tagTLENUMF = i32;
