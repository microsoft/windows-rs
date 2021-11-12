#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AppBroadcastingContract(i32);
#[repr(transparent)]
pub struct AppBroadcastingMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastingStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastingStatusDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastingUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastingMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastingStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastingStatusDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastingUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastingUIStatics(pub *mut ::core::ffi::c_void);
