#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INotificationActivationCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotificationActivationCallback {}
impl ::core::clone::Clone for INotificationActivationCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: super::super::Foundation::PWSTR,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
