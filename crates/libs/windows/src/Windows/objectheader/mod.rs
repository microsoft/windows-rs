#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDIS_OBJECT_HEADER {
    pub Type: u8,
    pub Revision: u8,
    pub Size: u16,
}
pub const NDIS_OBJECT_REVISION_1: u32 = 1;
pub type PNDIS_OBJECT_HEADER = *mut NDIS_OBJECT_HEADER;
