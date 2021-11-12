#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IServiceDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IStorageDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct PortableDeviceContract(i32);
pub struct ServiceDevice(i32);
pub struct ServiceDeviceType(i32);
pub struct StorageDevice(i32);
