#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_DISPLAY_DATA0 {
    pub name: *mut u16,
    pub description: *mut u16,
}
impl Default for FWPM_DISPLAY_DATA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_ACTION_BLOCK: i32 = 4097;
pub const FWP_ACTION_CALLOUT_INSPECTION: i32 = 24580;
pub const FWP_ACTION_CALLOUT_TERMINATING: i32 = 20483;
pub const FWP_ACTION_CALLOUT_UNKNOWN: i32 = 16389;
pub const FWP_ACTION_CONTINUE: i32 = 8198;
pub const FWP_ACTION_FLAG_CALLOUT: i32 = 16384;
pub const FWP_ACTION_FLAG_NON_TERMINATING: i32 = 8192;
pub const FWP_ACTION_FLAG_TERMINATING: i32 = 4096;
pub const FWP_ACTION_NONE: i32 = 7;
pub const FWP_ACTION_NONE_NO_MATCH: i32 = 8;
pub const FWP_ACTION_PERMIT: i32 = 4098;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FWP_ACTION_TYPE(pub u32);
pub const FWP_ACTRL_MATCH_FILTER: i32 = 1;
pub type FWP_AF = i32;
pub const FWP_AF_ETHER: FWP_AF = 2;
pub const FWP_AF_INET: FWP_AF = 0;
pub const FWP_AF_INET6: FWP_AF = 1;
pub const FWP_AF_NONE: FWP_AF = 3;
pub const FWP_BYTEMAP_ARRAY64_SIZE: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWP_BYTE_ARRAY16 {
    pub byteArray16: [u8; 16],
}
impl Default for FWP_BYTE_ARRAY16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_BYTE_ARRAY16_TYPE: FWP_DATA_TYPE = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWP_BYTE_ARRAY6 {
    pub byteArray6: [u8; 6],
}
impl Default for FWP_BYTE_ARRAY6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_BYTE_ARRAY6_SIZE: i32 = 6;
pub const FWP_BYTE_ARRAY6_TYPE: FWP_DATA_TYPE = 18;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWP_BYTE_BLOB {
    pub size: u32,
    pub data: *mut u8,
}
impl Default for FWP_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_BYTE_BLOB_TYPE: FWP_DATA_TYPE = 12;
pub const FWP_CALLOUT_FLAG_ALLOW_L2_BATCH_CLASSIFY: i32 = 128;
pub const FWP_CALLOUT_FLAG_ALLOW_MID_STREAM_INSPECTION: i32 = 8;
pub const FWP_CALLOUT_FLAG_ALLOW_OFFLOAD: i32 = 2;
pub const FWP_CALLOUT_FLAG_ALLOW_RECLASSIFY: i32 = 16;
pub const FWP_CALLOUT_FLAG_ALLOW_RSC: i32 = 64;
pub const FWP_CALLOUT_FLAG_ALLOW_URO: i32 = 512;
pub const FWP_CALLOUT_FLAG_ALLOW_USO: i32 = 256;
pub const FWP_CALLOUT_FLAG_CONDITIONAL_ON_FLOW: i32 = 1;
pub const FWP_CALLOUT_FLAG_ENABLE_COMMIT_ADD_NOTIFY: i32 = 4;
pub const FWP_CALLOUT_FLAG_RESERVED1: i32 = 32;
pub const FWP_CALLOUT_FLAG_RESERVED2: i32 = 1024;
pub const FWP_CLASSIFY_OPTION_LOCAL_ONLY_MAPPING: FWP_CLASSIFY_OPTION_TYPE = 7;
pub const FWP_CLASSIFY_OPTION_LOOSE_SOURCE_MAPPING: FWP_CLASSIFY_OPTION_TYPE = 1;
pub const FWP_CLASSIFY_OPTION_MAX: FWP_CLASSIFY_OPTION_TYPE = 8;
pub const FWP_CLASSIFY_OPTION_MCAST_BCAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE = 3;
pub const FWP_CLASSIFY_OPTION_MULTICAST_STATE: FWP_CLASSIFY_OPTION_TYPE = 0;
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_MM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE = 5;
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_QM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE = 6;
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_SECURITY_FLAGS: FWP_CLASSIFY_OPTION_TYPE = 4;
pub type FWP_CLASSIFY_OPTION_TYPE = i32;
pub const FWP_CLASSIFY_OPTION_UNICAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE = 2;
pub const FWP_CONDITION_FLAG_IS_APPCONTAINER_LOOPBACK: i32 = 4194304;
pub const FWP_CONDITION_FLAG_IS_AUTH_FW: i32 = 65536;
pub const FWP_CONDITION_FLAG_IS_CONNECTION_REDIRECTED: i32 = 1048576;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT: i32 = 32;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT_GROUP: i32 = 64;
pub const FWP_CONDITION_FLAG_IS_HONORING_POLICY_AUTHORIZE: i32 = 33554432;
pub const FWP_CONDITION_FLAG_IS_IMPLICIT_BIND: i32 = 512;
pub const FWP_CONDITION_FLAG_IS_INBOUND_PASS_THRU: i32 = 524288;
pub const FWP_CONDITION_FLAG_IS_IPSEC_NATT_RECLASSIFY: i32 = 128;
pub const FWP_CONDITION_FLAG_IS_IPSEC_SECURED: i32 = 2;
pub const FWP_CONDITION_FLAG_IS_LOOPBACK: i32 = 1;
pub const FWP_CONDITION_FLAG_IS_NAME_APP_SPECIFIED: i32 = 16384;
pub const FWP_CONDITION_FLAG_IS_NON_APPCONTAINER_LOOPBACK: i32 = 8388608;
pub const FWP_CONDITION_FLAG_IS_OUTBOUND_PASS_THRU: i32 = 262144;
pub const FWP_CONDITION_FLAG_IS_PROMISCUOUS: i32 = 32768;
pub const FWP_CONDITION_FLAG_IS_PROXY_CONNECTION: i32 = 2097152;
pub const FWP_CONDITION_FLAG_IS_RAW_ENDPOINT: i32 = 16;
pub const FWP_CONDITION_FLAG_IS_REASSEMBLED: i32 = 1024;
pub const FWP_CONDITION_FLAG_IS_REAUTHORIZE: i32 = 4;
pub const FWP_CONDITION_FLAG_IS_RECLASSIFY: i32 = 131072;
pub const FWP_CONDITION_FLAG_IS_RESERVED: i32 = 16777216;
pub const FWP_CONDITION_FLAG_IS_WILDCARD_BIND: i32 = 8;
pub const FWP_CONDITION_FLAG_REQUIRES_ALE_CLASSIFY: i32 = 256;
pub const FWP_CONDITION_L2_IF_CONNECTOR_PRESENT: i32 = 128;
pub const FWP_CONDITION_L2_IS_IP_FRAGMENT_GROUP: i32 = 64;
pub const FWP_CONDITION_L2_IS_MALFORMED_PACKET: i32 = 32;
pub const FWP_CONDITION_L2_IS_MOBILE_BROADBAND: i32 = 4;
pub const FWP_CONDITION_L2_IS_NATIVE_ETHERNET: i32 = 1;
pub const FWP_CONDITION_L2_IS_VM2VM: i32 = 16;
pub const FWP_CONDITION_L2_IS_WIFI: i32 = 2;
pub const FWP_CONDITION_L2_IS_WIFI_DIRECT_DATA: i32 = 8;
pub const FWP_CONDITION_REAUTHORIZE_REASON_CHECK_OFFLOAD: i32 = 65536;
pub const FWP_CONDITION_REAUTHORIZE_REASON_CLASSIFY_COMPLETION: i32 = 16;
pub const FWP_CONDITION_REAUTHORIZE_REASON_EDP_POLICY_CHANGED: i32 = 512;
pub const FWP_CONDITION_REAUTHORIZE_REASON_IPSEC_PROPERTIES_CHANGED: i32 = 32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_MID_STREAM_INSPECTION: i32 = 64;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_ARRIVAL_INTERFACE: i32 = 2;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_INBOUND_MCAST_BCAST_PACKET: i32 = 256;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_NEXTHOP_INTERFACE: i32 = 4;
pub const FWP_CONDITION_REAUTHORIZE_REASON_POLICY_CHANGE: i32 = 1;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROFILE_CROSSING: i32 = 8;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROXY_HANDLE_CHANGED: i32 = 16384;
pub const FWP_CONDITION_REAUTHORIZE_REASON_SOCKET_PROPERTY_CHANGED: i32 = 128;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_ALLOW_EDGE_TRAFFIC: i32 = 2;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_DENY_EDGE_TRAFFIC: i32 = 4;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_IS_SYSTEM_PORT_RPC: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FWP_CONDITION_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_CONDITION_VALUE0_0,
}
#[cfg(feature = "winnt")]
impl Default for FWP_CONDITION_VALUE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FWP_CONDITION_VALUE0_0 {
    pub uint8: u8,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: *mut u64,
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
    pub int64: *mut i64,
    pub float32: f32,
    pub double64: *mut f64,
    pub byteArray16: *mut FWP_BYTE_ARRAY16,
    pub byteBlob: *mut FWP_BYTE_BLOB,
    pub sid: *mut super::SID,
    pub sd: *mut FWP_BYTE_BLOB,
    pub tokenInformation: *mut FWP_TOKEN_INFORMATION,
    pub tokenAccessInformation: *mut FWP_BYTE_BLOB,
    pub unicodeString: windows_core::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
    pub v4AddrMask: *mut FWP_V4_ADDR_AND_MASK,
    pub v6AddrMask: *mut FWP_V6_ADDR_AND_MASK,
    pub rangeValue: *mut FWP_RANGE0,
}
#[cfg(feature = "winnt")]
impl Default for FWP_CONDITION_VALUE0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FWP_DATA_TYPE = i32;
pub const FWP_DATA_TYPE_MAX: FWP_DATA_TYPE = 259;
pub type FWP_DIRECTION = i32;
pub const FWP_DIRECTION_INBOUND: FWP_DIRECTION = 1;
pub const FWP_DIRECTION_MAX: FWP_DIRECTION = 2;
pub const FWP_DIRECTION_OUTBOUND: FWP_DIRECTION = 0;
pub const FWP_DOUBLE: FWP_DATA_TYPE = 10;
pub const FWP_EMPTY: FWP_DATA_TYPE = 0;
pub type FWP_ETHER_ENCAP_METHOD = i32;
pub const FWP_ETHER_ENCAP_METHOD_ETHER_V2: FWP_ETHER_ENCAP_METHOD = 0;
pub const FWP_ETHER_ENCAP_METHOD_SNAP: FWP_ETHER_ENCAP_METHOD = 1;
pub const FWP_ETHER_ENCAP_METHOD_SNAP_W_OUI_ZERO: FWP_ETHER_ENCAP_METHOD = 3;
pub const FWP_FILTER_ENUM_FLAG_BEST_TERMINATING_MATCH: i32 = 1;
pub const FWP_FILTER_ENUM_FLAG_BOOTTIME_ONLY: i32 = 4;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_BOOTTIME: i32 = 8;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_DISABLED: i32 = 16;
pub const FWP_FILTER_ENUM_FLAG_RESERVED1: i32 = 32;
pub const FWP_FILTER_ENUM_FLAG_SORTED: i32 = 2;
pub const FWP_FILTER_ENUM_FULLY_CONTAINED: FWP_FILTER_ENUM_TYPE = 0;
pub const FWP_FILTER_ENUM_OVERLAPPING: FWP_FILTER_ENUM_TYPE = 1;
pub type FWP_FILTER_ENUM_TYPE = i32;
pub const FWP_FILTER_ENUM_TYPE_MAX: FWP_FILTER_ENUM_TYPE = 2;
pub const FWP_FILTER_ENUM_VALID_FLAGS: i32 = 3;
pub const FWP_FLOAT: FWP_DATA_TYPE = 9;
pub const FWP_INT16: FWP_DATA_TYPE = 6;
pub const FWP_INT32: FWP_DATA_TYPE = 7;
pub const FWP_INT64: FWP_DATA_TYPE = 8;
pub const FWP_INT8: FWP_DATA_TYPE = 5;
pub type FWP_IP_VERSION = i32;
pub const FWP_IP_VERSION_MAX: FWP_IP_VERSION = 3;
pub const FWP_IP_VERSION_NONE: FWP_IP_VERSION = 2;
pub const FWP_IP_VERSION_V4: FWP_IP_VERSION = 0;
pub const FWP_IP_VERSION_V6: FWP_IP_VERSION = 1;
pub const FWP_MATCH_EQUAL: FWP_MATCH_TYPE = 0;
pub const FWP_MATCH_EQUAL_CASE_INSENSITIVE: FWP_MATCH_TYPE = 9;
pub const FWP_MATCH_FLAGS_ALL_SET: FWP_MATCH_TYPE = 6;
pub const FWP_MATCH_FLAGS_ANY_SET: FWP_MATCH_TYPE = 7;
pub const FWP_MATCH_FLAGS_NONE_SET: FWP_MATCH_TYPE = 8;
pub const FWP_MATCH_GREATER: FWP_MATCH_TYPE = 1;
pub const FWP_MATCH_GREATER_OR_EQUAL: FWP_MATCH_TYPE = 3;
pub const FWP_MATCH_LESS: FWP_MATCH_TYPE = 2;
pub const FWP_MATCH_LESS_OR_EQUAL: FWP_MATCH_TYPE = 4;
pub const FWP_MATCH_NOT_EQUAL: FWP_MATCH_TYPE = 10;
pub const FWP_MATCH_NOT_PREFIX: FWP_MATCH_TYPE = 12;
pub const FWP_MATCH_PREFIX: FWP_MATCH_TYPE = 11;
pub const FWP_MATCH_RANGE: FWP_MATCH_TYPE = 5;
pub type FWP_MATCH_TYPE = i32;
pub const FWP_MATCH_TYPE_MAX: FWP_MATCH_TYPE = 13;
pub const FWP_NETWORK_CONNECTION_POLICY_MAX: FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE = 3;
pub const FWP_NETWORK_CONNECTION_POLICY_NEXT_HOP: FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE = 2;
pub const FWP_NETWORK_CONNECTION_POLICY_NEXT_HOP_INTERFACE: FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE = 1;
pub type FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE = i32;
pub const FWP_NETWORK_CONNECTION_POLICY_SOURCE_ADDRESS: FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE = 0;
pub const FWP_OPTION_VALUE_ALLOW_GLOBAL_MULTICAST_STATE: i32 = 2;
pub const FWP_OPTION_VALUE_ALLOW_MULTICAST_STATE: i32 = 0;
pub const FWP_OPTION_VALUE_DENY_MULTICAST_STATE: i32 = 1;
pub const FWP_OPTION_VALUE_DISABLE_LOCAL_ONLY_MAPPING: i32 = 0;
pub const FWP_OPTION_VALUE_DISABLE_LOOSE_SOURCE: i32 = 0;
pub const FWP_OPTION_VALUE_ENABLE_LOCAL_ONLY_MAPPING: i32 = 1;
pub const FWP_OPTION_VALUE_ENABLE_LOOSE_SOURCE: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FWP_RANGE0 {
    pub valueLow: FWP_VALUE0,
    pub valueHigh: FWP_VALUE0,
}
#[cfg(feature = "winnt")]
impl Default for FWP_RANGE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_RANGE_TYPE: FWP_DATA_TYPE = 258;
pub const FWP_SECURITY_DESCRIPTOR_TYPE: FWP_DATA_TYPE = 14;
pub const FWP_SID: FWP_DATA_TYPE = 13;
pub const FWP_SINGLE_DATA_TYPE_MAX: FWP_DATA_TYPE = 255;
pub const FWP_TOKEN_ACCESS_INFORMATION_TYPE: FWP_DATA_TYPE = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWP_TOKEN_INFORMATION {
    pub sidCount: u32,
    pub sids: super::PSID_AND_ATTRIBUTES,
    pub restrictedSidCount: u32,
    pub restrictedSids: super::PSID_AND_ATTRIBUTES,
}
pub const FWP_TOKEN_INFORMATION_TYPE: FWP_DATA_TYPE = 15;
pub const FWP_UINT16: FWP_DATA_TYPE = 2;
pub const FWP_UINT32: FWP_DATA_TYPE = 3;
pub const FWP_UINT64: FWP_DATA_TYPE = 4;
pub const FWP_UINT8: FWP_DATA_TYPE = 1;
pub const FWP_UNICODE_STRING_TYPE: FWP_DATA_TYPE = 17;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWP_V4_ADDR_AND_MASK {
    pub addr: u32,
    pub mask: u32,
}
pub const FWP_V4_ADDR_MASK: FWP_DATA_TYPE = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWP_V6_ADDR_AND_MASK {
    pub addr: [u8; 16],
    pub prefixLength: u8,
}
impl Default for FWP_V6_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWP_V6_ADDR_MASK: FWP_DATA_TYPE = 257;
pub const FWP_V6_ADDR_SIZE: i32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FWP_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_VALUE0_0,
}
#[cfg(feature = "winnt")]
impl Default for FWP_VALUE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FWP_VALUE0_0 {
    pub uint8: u8,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: *mut u64,
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
    pub int64: *mut i64,
    pub float32: f32,
    pub double64: *mut f64,
    pub byteArray16: *mut FWP_BYTE_ARRAY16,
    pub byteBlob: *mut FWP_BYTE_BLOB,
    pub sid: *mut super::SID,
    pub sd: *mut FWP_BYTE_BLOB,
    pub tokenInformation: *mut FWP_TOKEN_INFORMATION,
    pub tokenAccessInformation: *mut FWP_BYTE_BLOB,
    pub unicodeString: windows_core::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
}
#[cfg(feature = "winnt")]
impl Default for FWP_VALUE0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FWP_VSWITCH_NETWORK_TYPE = i32;
pub const FWP_VSWITCH_NETWORK_TYPE_EXTERNAL: FWP_VSWITCH_NETWORK_TYPE = 3;
pub const FWP_VSWITCH_NETWORK_TYPE_INTERNAL: FWP_VSWITCH_NETWORK_TYPE = 2;
pub const FWP_VSWITCH_NETWORK_TYPE_PRIVATE: FWP_VSWITCH_NETWORK_TYPE = 1;
pub const FWP_VSWITCH_NETWORK_TYPE_UNKNOWN: FWP_VSWITCH_NETWORK_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    pub virtualIfTunnelId: u64,
    pub trafficSelectorId: u64,
}
