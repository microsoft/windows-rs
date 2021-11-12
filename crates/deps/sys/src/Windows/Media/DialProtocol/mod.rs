#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DialApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: Self = Self(0i32);
    pub const FailedToLaunch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DialAppStateDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: Self = Self(0i32);
    pub const StopFailed: Self = Self(1i32);
    pub const OperationNotSupported: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Disconnected: Self = Self(4i32);
    pub const Error: Self = Self(5i32);
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
