#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DnssdRegistrationResult(i32);
pub struct DnssdRegistrationStatus(i32);
pub struct DnssdServiceInstance(i32);
#[cfg(feature = "Foundation_Collections")]
pub struct DnssdServiceInstanceCollection(i32);
pub struct DnssdServiceWatcher(i32);
pub struct DnssdServiceWatcherStatus(i32);
pub struct IDnssdRegistrationResult(i32);
pub struct IDnssdServiceInstance(i32);
pub struct IDnssdServiceInstanceFactory(i32);
pub struct IDnssdServiceWatcher(i32);
