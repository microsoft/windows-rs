#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_OBJECT_HEADER {
    pub Type: super::UCHAR,
    pub Revision: super::UCHAR,
    pub Size: super::USHORT,
}
pub const NDIS_OBJECT_REVISION_1: i32 = 1;
#[cfg(feature = "shared")]
pub type PNDIS_OBJECT_HEADER = *mut NDIS_OBJECT_HEADER;
