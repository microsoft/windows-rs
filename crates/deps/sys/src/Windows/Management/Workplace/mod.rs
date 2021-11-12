#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IMdmAllowPolicyStatics(pub *mut ::core::ffi::c_void);
pub struct IMdmPolicyStatics2(pub *mut ::core::ffi::c_void);
pub struct IWorkplaceSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct MdmPolicy(i32);
pub struct MessagingSyncPolicy(i32);
pub struct WorkplaceSettings(i32);
pub struct WorkplaceSettingsContract(i32);
