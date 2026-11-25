#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: *const super::WCHAR,
    pub pwcsMime: *const super::WCHAR,
    pub pClsid: *const super::CLSID,
    pub pwcsOverride: *const super::WCHAR,
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
impl Default for FILTERED_DATA_SOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
