#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NDIS_HANDLE(pub *mut core::ffi::c_void);
impl Default for NDIS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDIS_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_HANDLE(pub *mut *mut core::ffi::c_void);
impl Default for PNDIS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PNDIS_STATUS = *mut i32;
