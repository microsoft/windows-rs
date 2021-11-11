#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DnssdRegistrationResult();
    fn DnssdRegistrationStatus();
    fn DnssdServiceInstance();
    fn DnssdServiceInstanceCollection();
    fn DnssdServiceWatcher();
    fn DnssdServiceWatcherStatus();
    fn IDnssdRegistrationResult();
    fn IDnssdServiceInstance();
    fn IDnssdServiceInstanceFactory();
    fn IDnssdServiceWatcher();
}
