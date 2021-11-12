#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DnssdRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: DnssdRegistrationStatus = DnssdRegistrationStatus(0i32);
    pub const InvalidServiceName: DnssdRegistrationStatus = DnssdRegistrationStatus(1i32);
    pub const ServerError: DnssdRegistrationStatus = DnssdRegistrationStatus(2i32);
    pub const SecurityError: DnssdRegistrationStatus = DnssdRegistrationStatus(3i32);
}
#[repr(transparent)]
pub struct DnssdServiceInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdServiceInstanceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdServiceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DnssdServiceWatcherStatus(pub i32);
impl DnssdServiceWatcherStatus {
    pub const Created: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(0i32);
    pub const Started: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(1i32);
    pub const EnumerationCompleted: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(2i32);
    pub const Stopping: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(3i32);
    pub const Stopped: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(4i32);
    pub const Aborted: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(5i32);
}
#[repr(transparent)]
pub struct IDnssdRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceInstanceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDnssdServiceWatcher(pub *mut ::core::ffi::c_void);
