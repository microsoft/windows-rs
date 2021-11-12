#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct INotificationActivationCallback(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_USER_INPUT_DATA(i32);
