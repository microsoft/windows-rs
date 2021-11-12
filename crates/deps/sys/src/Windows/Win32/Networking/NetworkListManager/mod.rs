#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IEnumNetworkConnections(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetworks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkConnectionCost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkConnectionCostEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkConnectionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkCostManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkCostManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkListManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkListManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NLM_CONNECTION_COST(pub i32);
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = NLM_CONNECTION_COST(0i32);
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(1i32);
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(2i32);
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = NLM_CONNECTION_COST(4i32);
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(65536i32);
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(131072i32);
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = NLM_CONNECTION_COST(262144i32);
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(524288i32);
#[repr(transparent)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = NLM_CONNECTION_PROPERTY_CHANGE(1i32);
#[repr(transparent)]
pub struct NLM_CONNECTIVITY(pub i32);
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = NLM_CONNECTIVITY(0i32);
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1i32);
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(2i32);
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(16i32);
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(32i32);
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(64i32);
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(256i32);
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(512i32);
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1024i32);
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: ::windows_sys::core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NLM_DOMAIN_TYPE(pub i32);
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(0i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(1i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(2i32);
#[repr(transparent)]
pub struct NLM_ENUM_NETWORK(pub i32);
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(1i32);
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(2i32);
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(3i32);
#[repr(transparent)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(1i32);
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(2i32);
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(4i32);
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[repr(transparent)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(0i32);
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(1i32);
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(2i32);
#[repr(transparent)]
pub struct NLM_NETWORK_CLASS(pub i32);
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(1i32);
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(2i32);
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(3i32);
#[repr(transparent)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(1i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(2i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(4i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(8i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(16i32);
#[repr(C)]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl ::core::marker::Copy for NLM_SIMULATED_PROFILE_INFO {}
impl ::core::clone::Clone for NLM_SIMULATED_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl ::core::marker::Copy for NLM_SOCKADDR {}
impl ::core::clone::Clone for NLM_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NetworkListManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3702524929,
    data2: 22287,
    data3: 19099,
    data4: [141, 105, 25, 159, 219, 165, 114, 59],
};
