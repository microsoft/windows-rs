#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Networking_BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Networking_Connectivity")]
pub mod Connectivity;
#[cfg(feature = "Networking_NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Networking_Proximity")]
pub mod Proximity;
#[cfg(feature = "Networking_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Networking_ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Networking_Sockets")]
pub mod Sockets;
#[cfg(feature = "Networking_Vpn")]
pub mod Vpn;
#[cfg(feature = "Networking_XboxLive")]
pub mod XboxLive;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: DomainNameType = DomainNameType(0i32);
    pub const FullyQualified: DomainNameType = DomainNameType(1i32);
}
#[repr(transparent)]
pub struct EndpointPair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HostName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: HostNameSortOptions = HostNameSortOptions(0u32);
    pub const OptimizeForLongConnections: HostNameSortOptions = HostNameSortOptions(2u32);
}
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: HostNameType = HostNameType(0i32);
    pub const Ipv4: HostNameType = HostNameType(1i32);
    pub const Ipv6: HostNameType = HostNameType(2i32);
    pub const Bluetooth: HostNameType = HostNameType(3i32);
}
#[repr(transparent)]
pub struct IEndpointPair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEndpointPairFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostNameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostNameStatics(pub *mut ::core::ffi::c_void);
