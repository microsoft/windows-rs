#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DialApp(pub *mut ::core::ffi::c_void);
pub struct DialAppLaunchResult(i32);
pub struct DialAppState(i32);
#[repr(transparent)]
pub struct DialAppStateDetails(pub *mut ::core::ffi::c_void);
pub struct DialAppStopResult(i32);
#[repr(transparent)]
pub struct DialDevice(pub *mut ::core::ffi::c_void);
pub struct DialDeviceDisplayStatus(i32);
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
