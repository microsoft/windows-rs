#[repr(C)]
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CATEGORYINFO {
    pub catid: CATID,
    pub lcid: super::winnt::LCID,
    pub szDescription: [super::wtypesbase::OLECHAR; 128],
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CATID = windows_sys::core::GUID;
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
pub type LPCATEGORYINFO = *mut CATEGORYINFO;
pub type REFCATID = *const windows_sys::core::GUID;
