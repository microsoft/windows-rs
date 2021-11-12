#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMdmAllowPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMdmPolicyStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkplaceSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MessagingSyncPolicy(i32);
#[repr(C)]
pub struct WorkplaceSettingsContract(i32);
