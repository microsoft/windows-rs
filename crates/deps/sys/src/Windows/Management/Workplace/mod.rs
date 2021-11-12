#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMdmAllowPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMdmPolicyStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkplaceSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MdmPolicy(pub *mut ::core::ffi::c_void);
pub struct MessagingSyncPolicy(i32);
#[repr(transparent)]
pub struct WorkplaceSettings(pub *mut ::core::ffi::c_void);
pub struct WorkplaceSettingsContract(i32);
