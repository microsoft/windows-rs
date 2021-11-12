#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct MdmAlertDataType(pub i32);
impl MdmAlertDataType {
    pub const String: MdmAlertDataType = MdmAlertDataType(0i32);
    pub const Base64: MdmAlertDataType = MdmAlertDataType(1i32);
    pub const Boolean: MdmAlertDataType = MdmAlertDataType(2i32);
    pub const Integer: MdmAlertDataType = MdmAlertDataType(3i32);
}
#[repr(transparent)]
pub struct MdmAlertMark(pub i32);
impl MdmAlertMark {
    pub const None: MdmAlertMark = MdmAlertMark(0i32);
    pub const Fatal: MdmAlertMark = MdmAlertMark(1i32);
    pub const Critical: MdmAlertMark = MdmAlertMark(2i32);
    pub const Warning: MdmAlertMark = MdmAlertMark(3i32);
    pub const Informational: MdmAlertMark = MdmAlertMark(4i32);
}
#[repr(transparent)]
pub struct MdmSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MdmSessionState(pub i32);
impl MdmSessionState {
    pub const NotStarted: MdmSessionState = MdmSessionState(0i32);
    pub const Starting: MdmSessionState = MdmSessionState(1i32);
    pub const Connecting: MdmSessionState = MdmSessionState(2i32);
    pub const Communicating: MdmSessionState = MdmSessionState(3i32);
    pub const AlertStatusAvailable: MdmSessionState = MdmSessionState(4i32);
    pub const Retrying: MdmSessionState = MdmSessionState(5i32);
    pub const Completed: MdmSessionState = MdmSessionState(6i32);
}
