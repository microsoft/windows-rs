#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DeploymentPreviewContract(i32);
#[repr(transparent)]
pub struct IClassicAppManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstalledClassicAppInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InstalledClassicAppInfo(pub *mut ::core::ffi::c_void);
