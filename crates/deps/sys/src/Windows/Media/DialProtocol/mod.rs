#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DialApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: DialAppLaunchResult = DialAppLaunchResult(0i32);
    pub const FailedToLaunch: DialAppLaunchResult = DialAppLaunchResult(1i32);
    pub const NotFound: DialAppLaunchResult = DialAppLaunchResult(2i32);
    pub const NetworkFailure: DialAppLaunchResult = DialAppLaunchResult(3i32);
}
#[repr(transparent)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: DialAppState = DialAppState(0i32);
    pub const Stopped: DialAppState = DialAppState(1i32);
    pub const Running: DialAppState = DialAppState(2i32);
    pub const NetworkFailure: DialAppState = DialAppState(3i32);
}
#[repr(transparent)]
pub struct DialAppStateDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: DialAppStopResult = DialAppStopResult(0i32);
    pub const StopFailed: DialAppStopResult = DialAppStopResult(1i32);
    pub const OperationNotSupported: DialAppStopResult = DialAppStopResult(2i32);
    pub const NetworkFailure: DialAppStopResult = DialAppStopResult(3i32);
}
#[repr(transparent)]
pub struct DialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: DialDeviceDisplayStatus = DialDeviceDisplayStatus(0i32);
    pub const Connecting: DialDeviceDisplayStatus = DialDeviceDisplayStatus(1i32);
    pub const Connected: DialDeviceDisplayStatus = DialDeviceDisplayStatus(2i32);
    pub const Disconnecting: DialDeviceDisplayStatus = DialDeviceDisplayStatus(3i32);
    pub const Disconnected: DialDeviceDisplayStatus = DialDeviceDisplayStatus(4i32);
    pub const Error: DialDeviceDisplayStatus = DialDeviceDisplayStatus(5i32);
}
#[repr(transparent)]
pub struct DialDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialReceiverApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialAppStateDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialReceiverApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialReceiverApp2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialReceiverAppStatics(pub *mut ::core::ffi::c_void);
