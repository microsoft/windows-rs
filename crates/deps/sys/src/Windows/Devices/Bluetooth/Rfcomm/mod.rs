#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRfcommDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommDeviceService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommDeviceService3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommDeviceServicesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommServiceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommServiceIdStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommServiceProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommServiceProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommDeviceServicesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommServiceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommServiceProvider(pub *mut ::core::ffi::c_void);
