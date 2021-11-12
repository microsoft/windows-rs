#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IServiceDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PortableDeviceContract(i32);
#[repr(transparent)]
pub struct ServiceDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ServiceDeviceType(i32);
#[repr(transparent)]
pub struct StorageDevice(pub *mut ::core::ffi::c_void);
