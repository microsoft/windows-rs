#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IEnumNetworkConnections(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetworkConnections {}
impl ::core::clone::Clone for IEnumNetworkConnections {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetworks(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetworks {}
impl ::core::clone::Clone for IEnumNetworks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetwork {}
impl ::core::clone::Clone for INetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkConnection {}
impl ::core::clone::Clone for INetworkConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkConnectionCost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkConnectionCost {}
impl ::core::clone::Clone for INetworkConnectionCost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkConnectionCostEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkConnectionCostEvents {}
impl ::core::clone::Clone for INetworkConnectionCostEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkConnectionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkConnectionEvents {}
impl ::core::clone::Clone for INetworkConnectionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkCostManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkCostManager {}
impl ::core::clone::Clone for INetworkCostManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkCostManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkCostManagerEvents {}
impl ::core::clone::Clone for INetworkCostManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkEvents {}
impl ::core::clone::Clone for INetworkEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkListManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkListManager {}
impl ::core::clone::Clone for INetworkListManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkListManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkListManagerEvents {}
impl ::core::clone::Clone for INetworkListManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NLM_CONNECTION_COST_UNKNOWN: i32 = 0i32;
pub const NLM_CONNECTION_COST_UNRESTRICTED: i32 = 1i32;
pub const NLM_CONNECTION_COST_FIXED: i32 = 2i32;
pub const NLM_CONNECTION_COST_VARIABLE: i32 = 4i32;
pub const NLM_CONNECTION_COST_OVERDATALIMIT: i32 = 65536i32;
pub const NLM_CONNECTION_COST_CONGESTED: i32 = 131072i32;
pub const NLM_CONNECTION_COST_ROAMING: i32 = 262144i32;
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: i32 = 524288i32;
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: i32 = 1i32;
pub const NLM_CONNECTIVITY_DISCONNECTED: i32 = 0i32;
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: i32 = 1i32;
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: i32 = 2i32;
pub const NLM_CONNECTIVITY_IPV4_SUBNET: i32 = 16i32;
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: i32 = 32i32;
pub const NLM_CONNECTIVITY_IPV4_INTERNET: i32 = 64i32;
pub const NLM_CONNECTIVITY_IPV6_SUBNET: i32 = 256i32;
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: i32 = 512i32;
pub const NLM_CONNECTIVITY_IPV6_INTERNET: i32 = 1024i32;
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
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: i32 = 0i32;
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: i32 = 1i32;
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: i32 = 2i32;
pub const NLM_ENUM_NETWORK_CONNECTED: i32 = 1i32;
pub const NLM_ENUM_NETWORK_DISCONNECTED: i32 = 2i32;
pub const NLM_ENUM_NETWORK_ALL: i32 = 3i32;
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: i32 = 1i32;
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: i32 = 2i32;
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: i32 = 4i32;
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
pub const NLM_NETWORK_CATEGORY_PUBLIC: i32 = 0i32;
pub const NLM_NETWORK_CATEGORY_PRIVATE: i32 = 1i32;
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: i32 = 2i32;
pub const NLM_NETWORK_IDENTIFYING: i32 = 1i32;
pub const NLM_NETWORK_IDENTIFIED: i32 = 2i32;
pub const NLM_NETWORK_UNIDENTIFIED: i32 = 3i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: i32 = 1i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: i32 = 2i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: i32 = 4i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: i32 = 8i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: i32 = 16i32;
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
pub const NetworkListManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3702524929,
    data2: 22287,
    data3: 19099,
    data4: [141, 105, 25, 159, 219, 165, 114, 59],
};
