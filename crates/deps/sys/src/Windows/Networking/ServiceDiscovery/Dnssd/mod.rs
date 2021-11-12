#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DnssdRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DnssdRegistrationStatus(i32);
#[repr(transparent)]
pub struct DnssdServiceInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdServiceInstanceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdServiceWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DnssdServiceWatcherStatus(i32);
#[repr(transparent)]
pub struct IDnssdRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceInstanceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceWatcher(pub *mut ::core::ffi::c_void);
