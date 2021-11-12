#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppBroadcastingContract(i32);
pub struct AppBroadcastingMonitor(i32);
pub struct AppBroadcastingStatus(i32);
pub struct AppBroadcastingStatusDetails(i32);
pub struct AppBroadcastingUI(i32);
pub struct IAppBroadcastingMonitor(pub *mut ::core::ffi::c_void);
pub struct IAppBroadcastingStatus(pub *mut ::core::ffi::c_void);
pub struct IAppBroadcastingStatusDetails(pub *mut ::core::ffi::c_void);
pub struct IAppBroadcastingUI(pub *mut ::core::ffi::c_void);
pub struct IAppBroadcastingUIStatics(pub *mut ::core::ffi::c_void);
