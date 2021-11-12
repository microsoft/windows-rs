#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct DomainNameType(i32);
#[repr(transparent)]
pub struct EndpointPair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HostName(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HostNameSortOptions(i32);
#[repr(C)]
pub struct HostNameType(i32);
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
