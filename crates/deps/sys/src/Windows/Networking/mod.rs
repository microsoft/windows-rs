#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EndpointPair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EndpointPair {}
impl ::core::clone::Clone for EndpointPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HostName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HostName {}
impl ::core::clone::Clone for HostName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEndpointPair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEndpointPair {}
impl ::core::clone::Clone for IEndpointPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEndpointPairFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEndpointPairFactory {}
impl ::core::clone::Clone for IEndpointPairFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHostName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHostName {}
impl ::core::clone::Clone for IHostName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHostNameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHostNameFactory {}
impl ::core::clone::Clone for IHostNameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHostNameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHostNameStatics {}
impl ::core::clone::Clone for IHostNameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
