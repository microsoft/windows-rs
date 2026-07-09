#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDIS_OBJECT_HEADER {
    pub Type: u8,
    pub Revision: u8,
    pub Size: u16,
}
pub const NDIS_OBJECT_REVISION_1: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_OBJECT_HEADER(pub *mut NDIS_OBJECT_HEADER);
impl PNDIS_OBJECT_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDIS_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
