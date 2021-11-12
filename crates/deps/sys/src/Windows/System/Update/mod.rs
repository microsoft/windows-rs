#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISystemUpdateItem(pub *mut ::core::ffi::c_void);
pub struct ISystemUpdateLastErrorInfo(pub *mut ::core::ffi::c_void);
pub struct ISystemUpdateManagerStatics(pub *mut ::core::ffi::c_void);
pub struct SystemUpdateAttentionRequiredReason(i32);
pub struct SystemUpdateItem(i32);
pub struct SystemUpdateItemState(i32);
pub struct SystemUpdateLastErrorInfo(i32);
pub struct SystemUpdateManager(i32);
pub struct SystemUpdateManagerState(i32);
pub struct SystemUpdateStartInstallAction(i32);
