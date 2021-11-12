#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISystemUpdateItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemUpdateLastErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemUpdateManagerStatics(pub *mut ::core::ffi::c_void);
pub struct SystemUpdateAttentionRequiredReason(i32);
#[repr(transparent)]
pub struct SystemUpdateItem(pub *mut ::core::ffi::c_void);
pub struct SystemUpdateItemState(i32);
#[repr(transparent)]
pub struct SystemUpdateLastErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemUpdateManager(pub *mut ::core::ffi::c_void);
pub struct SystemUpdateManagerState(i32);
pub struct SystemUpdateStartInstallAction(i32);
