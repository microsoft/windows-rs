#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
impl ::core::marker::Copy for WindowsMediaLibrarySharingDeviceAuthorizationStatus {}
impl ::core::clone::Clone for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WindowsMediaLibrarySharingServices: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2908232448,
    data2: 31588,
    data3: 20057,
    data4: [163, 141, 210, 197, 191, 81, 221, 179],
};
