#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: *const super::WCHAR,
    pub pwcsMime: *const super::WCHAR,
    pub pClsid: *const super::CLSID,
    pub pwcsOverride: *const super::WCHAR,
}
