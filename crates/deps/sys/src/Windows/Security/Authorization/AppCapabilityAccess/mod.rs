#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: AppCapabilityAccessStatus = AppCapabilityAccessStatus(0i32);
    pub const NotDeclaredByApp: AppCapabilityAccessStatus = AppCapabilityAccessStatus(1i32);
    pub const DeniedByUser: AppCapabilityAccessStatus = AppCapabilityAccessStatus(2i32);
    pub const UserPromptRequired: AppCapabilityAccessStatus = AppCapabilityAccessStatus(3i32);
    pub const Allowed: AppCapabilityAccessStatus = AppCapabilityAccessStatus(4i32);
}
#[repr(transparent)]
pub struct IAppCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCapabilityStatics(pub *mut ::core::ffi::c_void);
