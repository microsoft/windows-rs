#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ErrorReceivedEventArgs(i32);
pub struct IErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPinChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISerialDevice(pub *mut ::core::ffi::c_void);
pub struct ISerialDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct PinChangedEventArgs(i32);
pub struct SerialDevice(i32);
pub struct SerialError(i32);
pub struct SerialHandshake(i32);
pub struct SerialParity(i32);
pub struct SerialPinChange(i32);
pub struct SerialStopBitCount(i32);
