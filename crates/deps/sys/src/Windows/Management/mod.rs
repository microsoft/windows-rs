#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Management_Core")]
pub mod Core;
#[cfg(feature = "Management_Deployment")]
pub mod Deployment;
#[cfg(feature = "Management_Policies")]
pub mod Policies;
#[cfg(feature = "Management_Update")]
pub mod Update;
#[cfg(feature = "Management_Workplace")]
pub mod Workplace;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMdmAlert(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMdmSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMdmSessionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MdmAlert(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MdmAlertDataType(i32);
#[repr(C)]
pub struct MdmAlertMark(i32);
#[repr(transparent)]
pub struct MdmSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MdmSessionManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MdmSessionState(i32);
