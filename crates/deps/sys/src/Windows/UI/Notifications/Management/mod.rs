#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUserNotificationListener(pub *mut ::core::ffi::c_void);
pub struct IUserNotificationListenerStatics(pub *mut ::core::ffi::c_void);
pub struct UserNotificationListener(i32);
pub struct UserNotificationListenerAccessStatus(i32);
