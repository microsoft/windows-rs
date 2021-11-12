#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CustomDeviceContract(i32);
#[repr(C)]
pub struct DeviceAccessMode(i32);
#[repr(C)]
pub struct DeviceSharingMode(i32);
#[repr(transparent)]
pub struct ICustomDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIOControlCode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIOControlCodeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownDeviceTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IOControlAccessMode(i32);
#[repr(C)]
pub struct IOControlBufferingMethod(i32);
#[repr(transparent)]
pub struct IOControlCode(pub *mut ::core::ffi::c_void);
