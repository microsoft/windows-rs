#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CastingConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
pub struct CastingConnectionErrorStatus(i32);
pub struct CastingConnectionState(i32);
#[repr(transparent)]
pub struct CastingDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct CastingPlaybackTypes(i32);
#[repr(transparent)]
pub struct CastingSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingSource(pub *mut ::core::ffi::c_void);
