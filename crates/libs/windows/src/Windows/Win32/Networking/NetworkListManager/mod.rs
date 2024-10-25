pub const NA_AllowMerge: windows_core::PCWSTR = windows_core::w!("NA_AllowMerge");
pub const NA_CategoryReadOnly: windows_core::PCWSTR = windows_core::w!("NA_CategoryReadOnly");
pub const NA_CategorySetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_CategorySetByPolicy");
pub const NA_DescriptionReadOnly: windows_core::PCWSTR = windows_core::w!("NA_DescriptionReadOnly");
pub const NA_DescriptionSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_DescriptionSetByPolicy");
pub const NA_DomainAuthenticationFailed: windows_core::PCWSTR = windows_core::w!("NA_DomainAuthenticationFailed");
pub const NA_IconReadOnly: windows_core::PCWSTR = windows_core::w!("NA_IconReadOnly");
pub const NA_IconSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_IconSetByPolicy");
pub const NA_InternetConnectivityV4: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV4");
pub const NA_InternetConnectivityV6: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV6");
pub const NA_NameReadOnly: windows_core::PCWSTR = windows_core::w!("NA_NameReadOnly");
pub const NA_NameSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_NameSetByPolicy");
pub const NA_NetworkClass: windows_core::PCWSTR = windows_core::w!("NA_NetworkClass");
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = 524288i32;
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = 131072i32;
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = 2i32;
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = 65536i32;
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = 262144i32;
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = 0i32;
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = 1i32;
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = 4i32;
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = 1i32;
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = 0i32;
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = 64i32;
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = 32i32;
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = 1i32;
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = 16i32;
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = 1024i32;
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = 512i32;
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = 2i32;
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = 256i32;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_LDAP: NLM_DOMAIN_AUTHENTICATION_KIND = 1i32;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_NONE: NLM_DOMAIN_AUTHENTICATION_KIND = 0i32;
pub const NLM_DOMAIN_AUTHENTICATION_KIND_TLS: NLM_DOMAIN_AUTHENTICATION_KIND = 2i32;
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = 2i32;
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 1i32;
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 0i32;
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = 3i32;
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = 1i32;
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = 2i32;
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = 4i32;
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = 2i32;
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = 1i32;
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = 2i32;
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = 1i32;
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = 0i32;
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = 2i32;
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = 1i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = 16i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = 1i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = 2i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = 8i32;
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = 4i32;
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = 3i32;
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_CONNECTION_COST(pub i32);
impl windows_core::TypeKind for NLM_CONNECTION_COST {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
impl windows_core::TypeKind for NLM_CONNECTION_PROPERTY_CHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_CONNECTIVITY(pub i32);
impl windows_core::TypeKind for NLM_CONNECTIVITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_DOMAIN_AUTHENTICATION_KIND(pub i32);
impl windows_core::TypeKind for NLM_DOMAIN_AUTHENTICATION_KIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_DOMAIN_TYPE(pub i32);
impl windows_core::TypeKind for NLM_DOMAIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_ENUM_NETWORK(pub i32);
impl windows_core::TypeKind for NLM_ENUM_NETWORK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
impl windows_core::TypeKind for NLM_INTERNET_CONNECTIVITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_NETWORK_CLASS(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_PROPERTY_CHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: windows_core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
impl Default for NLM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NLM_DATAPLAN_STATUS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for NLM_SIMULATED_PROFILE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NLM_SOCKADDR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
impl Default for NLM_USAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NLM_USAGE_DATA {
    type TypeKind = windows_core::CloneType;
}
pub const NetworkListManager: windows_core::GUID = windows_core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
