pub const NA_AllowMerge: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_AllowMerge");
pub const NA_CategoryReadOnly: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_CategoryReadOnly");
pub const NA_CategorySetByPolicy: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_CategorySetByPolicy");
pub const NA_DescriptionReadOnly: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_DescriptionReadOnly");
pub const NA_DescriptionSetByPolicy: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_DescriptionSetByPolicy");
pub const NA_DomainAuthenticationFailed: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_DomainAuthenticationFailed");
pub const NA_IconReadOnly: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_IconReadOnly");
pub const NA_IconSetByPolicy: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_IconSetByPolicy");
pub const NA_InternetConnectivityV4: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_InternetConnectivityV4");
pub const NA_InternetConnectivityV6: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_InternetConnectivityV6");
pub const NA_NameReadOnly: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_NameReadOnly");
pub const NA_NameSetByPolicy: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_NameSetByPolicy");
pub const NA_NetworkClass: windows_sys::core::PCWSTR = windows_sys::core::w!("NA_NetworkClass");
pub type NLM_CONNECTION_COST = i32;
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = 524288;
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = 131072;
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = 2;
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = 65536;
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = 262144;
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = 0;
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = 1;
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = 4;
pub type NLM_CONNECTION_PROPERTY_CHANGE = i32;
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = 1;
pub type NLM_CONNECTIVITY = i32;
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = 0;
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = 64;
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = 32;
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = 1;
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = 16;
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = 1024;
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = 512;
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = 2;
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = 256;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: windows_sys::core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
pub type NLM_DOMAIN_AUTHENTICATION_KIND = i32;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_LDAP: NLM_DOMAIN_AUTHENTICATION_KIND = 1;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_NONE: NLM_DOMAIN_AUTHENTICATION_KIND = 0;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_TLS: NLM_DOMAIN_AUTHENTICATION_KIND = 2;
pub type NLM_DOMAIN_TYPE = i32;
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = 2;
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 1;
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 0;
pub type NLM_ENUM_NETWORK = i32;
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = 3;
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = 1;
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = 2;
pub type NLM_INTERNET_CONNECTIVITY = i32;
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = 4;
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = 2;
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = 1;
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10;
pub type NLM_NETWORK_CATEGORY = i32;
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = 2;
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = 1;
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = 0;
pub type NLM_NETWORK_CLASS = i32;
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = 2;
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = 1;
pub type NLM_NETWORK_PROPERTY_CHANGE = i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = 16;
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = 1;
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = 2;
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = 8;
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = 4;
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl Default for NLM_SIMULATED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::FILETIME,
}
pub const NetworkListManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
