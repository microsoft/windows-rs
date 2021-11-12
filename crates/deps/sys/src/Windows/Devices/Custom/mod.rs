#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CustomDevice(i32);
pub struct CustomDeviceContract(i32);
pub struct DeviceAccessMode(i32);
pub struct DeviceSharingMode(i32);
pub struct ICustomDevice(pub *mut ::core::ffi::c_void);
pub struct ICustomDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IIOControlCode(pub *mut ::core::ffi::c_void);
pub struct IIOControlCodeFactory(pub *mut ::core::ffi::c_void);
pub struct IKnownDeviceTypesStatics(pub *mut ::core::ffi::c_void);
pub struct IOControlAccessMode(i32);
pub struct IOControlBufferingMethod(i32);
pub struct IOControlCode(i32);
pub struct KnownDeviceTypes(i32);
