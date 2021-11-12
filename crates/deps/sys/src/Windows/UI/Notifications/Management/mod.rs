#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserNotificationListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationListenerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationListener(pub *mut ::core::ffi::c_void);
pub struct UserNotificationListenerAccessStatus(i32);
