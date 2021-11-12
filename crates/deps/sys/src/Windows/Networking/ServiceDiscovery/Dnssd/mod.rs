#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DnssdRegistrationResult(i32);
pub struct DnssdRegistrationStatus(i32);
pub struct DnssdServiceInstance(i32);
pub struct DnssdServiceInstanceCollection(i32);
pub struct DnssdServiceWatcher(i32);
pub struct DnssdServiceWatcherStatus(i32);
pub struct IDnssdRegistrationResult(pub *mut ::core::ffi::c_void);
pub struct IDnssdServiceInstance(pub *mut ::core::ffi::c_void);
pub struct IDnssdServiceInstanceFactory(pub *mut ::core::ffi::c_void);
pub struct IDnssdServiceWatcher(pub *mut ::core::ffi::c_void);
