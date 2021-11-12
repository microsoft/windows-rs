#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CastingConnection(i32);
pub struct CastingConnectionErrorOccurredEventArgs(i32);
pub struct CastingConnectionErrorStatus(i32);
pub struct CastingConnectionState(i32);
pub struct CastingDevice(i32);
pub struct CastingDevicePicker(i32);
pub struct CastingDevicePickerFilter(i32);
pub struct CastingDeviceSelectedEventArgs(i32);
pub struct CastingPlaybackTypes(i32);
pub struct CastingSource(i32);
pub struct ICastingConnection(pub *mut ::core::ffi::c_void);
pub struct ICastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICastingDevice(pub *mut ::core::ffi::c_void);
pub struct ICastingDevicePicker(pub *mut ::core::ffi::c_void);
pub struct ICastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
pub struct ICastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICastingDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct ICastingSource(pub *mut ::core::ffi::c_void);
