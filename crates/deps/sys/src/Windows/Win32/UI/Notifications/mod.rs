#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct INotificationActivationCallback(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_USER_INPUT_DATA(i32);
