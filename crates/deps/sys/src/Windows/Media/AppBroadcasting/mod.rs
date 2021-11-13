#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppBroadcastingMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastingMonitor {}
impl ::core::clone::Clone for AppBroadcastingMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastingStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastingStatus {}
impl ::core::clone::Clone for AppBroadcastingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastingStatusDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastingStatusDetails {}
impl ::core::clone::Clone for AppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastingUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastingUI {}
impl ::core::clone::Clone for AppBroadcastingUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastingMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastingMonitor {}
impl ::core::clone::Clone for IAppBroadcastingMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastingStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastingStatus {}
impl ::core::clone::Clone for IAppBroadcastingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastingStatusDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastingStatusDetails {}
impl ::core::clone::Clone for IAppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastingUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastingUI {}
impl ::core::clone::Clone for IAppBroadcastingUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastingUIStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastingUIStatics {}
impl ::core::clone::Clone for IAppBroadcastingUIStatics {
    fn clone(&self) -> Self {
        *self
    }
}
