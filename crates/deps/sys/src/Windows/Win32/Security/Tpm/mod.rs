#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RemoteTpmVirtualSmartCardManager(i32);
#[repr(C)]
pub struct TPMVSCMGR_ERROR(i32);
#[repr(C)]
pub struct TPMVSCMGR_STATUS(i32);
#[repr(C)]
pub struct TPMVSC_ATTESTATION_TYPE(i32);
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
#[repr(C)]
pub struct TpmVirtualSmartCardManager(i32);
