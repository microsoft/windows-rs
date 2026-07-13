pub type NDIS_HANDLE = *mut core::ffi::c_void;
pub type NDIS_STATUS = i32;
pub type PNDIS_HANDLE = *mut *mut core::ffi::c_void;
pub type PNDIS_STATUS = *mut i32;
