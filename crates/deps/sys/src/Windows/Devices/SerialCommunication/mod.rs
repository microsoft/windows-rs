#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISerialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISerialDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SerialDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SerialError(i32);
#[repr(C)]
pub struct SerialHandshake(i32);
#[repr(C)]
pub struct SerialParity(i32);
#[repr(C)]
pub struct SerialPinChange(i32);
#[repr(C)]
pub struct SerialStopBitCount(i32);
