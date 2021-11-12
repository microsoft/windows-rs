#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWindowsMediaLibrarySharingDevice(pub *mut ::core::ffi::c_void);
pub struct IWindowsMediaLibrarySharingDeviceProperties(pub *mut ::core::ffi::c_void);
pub struct IWindowsMediaLibrarySharingDeviceProperty(pub *mut ::core::ffi::c_void);
pub struct IWindowsMediaLibrarySharingDevices(pub *mut ::core::ffi::c_void);
pub struct IWindowsMediaLibrarySharingServices(pub *mut ::core::ffi::c_void);
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(i32);
pub struct WindowsMediaLibrarySharingServices(i32);
