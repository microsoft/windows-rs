#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: *const u16,
    pub pwcsMime: *const u16,
    pub pClsid: *const windows_sys::core::GUID,
    pub pwcsOverride: *const u16,
}
impl Default for FILTERED_DATA_SOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
