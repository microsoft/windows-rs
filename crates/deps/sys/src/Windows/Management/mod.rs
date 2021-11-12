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
pub struct IMdmAlert(pub *mut ::core::ffi::c_void);
pub struct IMdmSession(pub *mut ::core::ffi::c_void);
pub struct IMdmSessionManagerStatics(pub *mut ::core::ffi::c_void);
pub struct MdmAlert(i32);
pub struct MdmAlertDataType(i32);
pub struct MdmAlertMark(i32);
pub struct MdmSession(i32);
pub struct MdmSessionManager(i32);
pub struct MdmSessionState(i32);
