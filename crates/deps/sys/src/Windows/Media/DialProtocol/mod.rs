#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DialApp(i32);
pub struct DialAppLaunchResult(i32);
pub struct DialAppState(i32);
pub struct DialAppStateDetails(i32);
pub struct DialAppStopResult(i32);
pub struct DialDevice(i32);
pub struct DialDeviceDisplayStatus(i32);
pub struct DialDevicePicker(i32);
pub struct DialDevicePickerFilter(i32);
pub struct DialDeviceSelectedEventArgs(i32);
pub struct DialDisconnectButtonClickedEventArgs(i32);
pub struct DialReceiverApp(i32);
pub struct IDialApp(pub *mut ::core::ffi::c_void);
pub struct IDialAppStateDetails(pub *mut ::core::ffi::c_void);
pub struct IDialDevice(pub *mut ::core::ffi::c_void);
pub struct IDialDevice2(pub *mut ::core::ffi::c_void);
pub struct IDialDevicePicker(pub *mut ::core::ffi::c_void);
pub struct IDialDevicePickerFilter(pub *mut ::core::ffi::c_void);
pub struct IDialDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IDialDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IDialDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IDialReceiverApp(pub *mut ::core::ffi::c_void);
pub struct IDialReceiverApp2(pub *mut ::core::ffi::c_void);
pub struct IDialReceiverAppStatics(pub *mut ::core::ffi::c_void);
