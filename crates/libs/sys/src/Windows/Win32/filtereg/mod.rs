#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: *const u16,
    pub pwcsMime: *const u16,
    pub pClsid: *const windows_sys::core::GUID,
    pub pwcsOverride: *const u16,
}
