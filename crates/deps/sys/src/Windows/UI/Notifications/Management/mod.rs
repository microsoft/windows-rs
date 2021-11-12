#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserNotificationListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationListenerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationListenerAccessStatus(pub i32);
impl UserNotificationListenerAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
