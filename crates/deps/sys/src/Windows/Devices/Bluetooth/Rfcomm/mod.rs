#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IRfcommDeviceService(pub *mut ::core::ffi::c_void);
pub struct IRfcommDeviceService2(pub *mut ::core::ffi::c_void);
pub struct IRfcommDeviceService3(pub *mut ::core::ffi::c_void);
pub struct IRfcommDeviceServiceStatics(pub *mut ::core::ffi::c_void);
pub struct IRfcommDeviceServiceStatics2(pub *mut ::core::ffi::c_void);
pub struct IRfcommDeviceServicesResult(pub *mut ::core::ffi::c_void);
pub struct IRfcommServiceId(pub *mut ::core::ffi::c_void);
pub struct IRfcommServiceIdStatics(pub *mut ::core::ffi::c_void);
pub struct IRfcommServiceProvider(pub *mut ::core::ffi::c_void);
pub struct IRfcommServiceProvider2(pub *mut ::core::ffi::c_void);
pub struct IRfcommServiceProviderStatics(pub *mut ::core::ffi::c_void);
pub struct RfcommDeviceService(i32);
pub struct RfcommDeviceServicesResult(i32);
pub struct RfcommServiceId(i32);
pub struct RfcommServiceProvider(i32);
