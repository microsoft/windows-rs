pub type DL_ADDRESS_TYPE = i32;
pub const DlBroadcast: DL_ADDRESS_TYPE = 2;
pub const DlMulticast: DL_ADDRESS_TYPE = 1;
pub const DlUnicast: DL_ADDRESS_TYPE = 0;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct FWPM_ACTION0 {
    pub r#type: super::fwptypes::FWP_ACTION_TYPE,
    pub Anonymous: FWPM_ACTION0_0,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_ACTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union FWPM_ACTION0_0 {
    pub filterType: windows_core::GUID,
    pub calloutKey: windows_core::GUID,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_ACTION0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT: FWPM_APPC_NETWORK_CAPABILITY_TYPE = 0;
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER: FWPM_APPC_NETWORK_CAPABILITY_TYPE = 1;
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_PRIVATE_NETWORK: FWPM_APPC_NETWORK_CAPABILITY_TYPE = 2;
pub type FWPM_APPC_NETWORK_CAPABILITY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_CALLOUT0 {
    pub calloutKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub applicableLayer: windows_core::GUID,
    pub calloutId: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_CALLOUT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_CALLOUT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub calloutKey: windows_core::GUID,
    pub calloutId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_CALLOUT_ENUM_TEMPLATE0 {
    pub providerKey: *mut windows_core::GUID,
    pub layerKey: windows_core::GUID,
}
impl Default for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_CALLOUT_FLAG_PERSISTENT: u32 = 65536;
pub const FWPM_CALLOUT_FLAG_REGISTERED: u32 = 262144;
pub const FWPM_CALLOUT_FLAG_USES_PROVIDER_CONTEXT: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_CALLOUT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CALLOUT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
impl Default for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_CHANGE_ADD: FWPM_CHANGE_TYPE = 1;
pub const FWPM_CHANGE_DELETE: FWPM_CHANGE_TYPE = 2;
pub type FWPM_CHANGE_TYPE = i32;
pub const FWPM_CHANGE_TYPE_MAX: FWPM_CHANGE_TYPE = 3;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_CLASSIFY_OPTION0 {
    pub r#type: super::fwptypes::FWP_CLASSIFY_OPTION_TYPE,
    pub value: super::fwptypes::FWP_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_CLASSIFY_OPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_CLASSIFY_OPTIONS0 {
    pub numOptions: u32,
    pub options: *mut FWPM_CLASSIFY_OPTION0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_CLASSIFY_OPTIONS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_CLASSIFY_OPTIONS_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 7;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct FWPM_CONNECTION0 {
    pub connectionId: u64,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: FWPM_CONNECTION0_0,
    pub Anonymous2: FWPM_CONNECTION0_1,
    pub providerKey: *mut windows_core::GUID,
    pub ipsecTrafficModeType: super::ipsectypes::IPSEC_TRAFFIC_TYPE,
    pub keyModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub mmCrypto: super::iketypes::IKEEXT_PROPOSAL0,
    pub mmPeer: super::iketypes::IKEEXT_CREDENTIAL2,
    pub emPeer: super::iketypes::IKEEXT_CREDENTIAL2,
    pub bytesTransferredIn: u64,
    pub bytesTransferredOut: u64,
    pub bytesTransferredTotal: u64,
    pub startSysTime: super::minwindef::FILETIME,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
impl Default for FWPM_CONNECTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub union FWPM_CONNECTION0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
impl Default for FWPM_CONNECTION0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub union FWPM_CONNECTION0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
impl Default for FWPM_CONNECTION0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_CONNECTION_ENUM_FLAG_QUERY_BYTES_TRANSFERRED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_CONNECTION_ENUM_TEMPLATE0 {
    pub connectionId: u64,
    pub flags: u32,
}
pub const FWPM_CONNECTION_EVENT_ADD: FWPM_CONNECTION_EVENT_TYPE = 0;
pub const FWPM_CONNECTION_EVENT_DELETE: FWPM_CONNECTION_EVENT_TYPE = 1;
pub const FWPM_CONNECTION_EVENT_MAX: FWPM_CONNECTION_EVENT_TYPE = 2;
pub type FWPM_CONNECTION_EVENT_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_CONNECTION_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CONNECTION_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
impl Default for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_ENGINE_COLLECT_NET_EVENTS: FWPM_ENGINE_OPTION = 0;
pub const FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS: FWPM_ENGINE_OPTION = 3;
pub const FWPM_ENGINE_NAME_CACHE: FWPM_ENGINE_OPTION = 2;
pub const FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS: FWPM_ENGINE_OPTION = 1;
pub type FWPM_ENGINE_OPTION = i32;
pub const FWPM_ENGINE_OPTION_MAX: FWPM_ENGINE_OPTION = 6;
pub const FWPM_ENGINE_OPTION_PACKET_BATCH_INBOUND: u32 = 4;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_FORWARD: u32 = 2;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_INBOUND: u32 = 1;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_NONE: u32 = 0;
pub const FWPM_ENGINE_PACKET_QUEUING: FWPM_ENGINE_OPTION = 4;
pub const FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC: FWPM_ENGINE_OPTION = 5;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_FIELD0 {
    pub fieldKey: *mut windows_core::GUID,
    pub r#type: FWPM_FIELD_TYPE,
    pub dataType: super::fwptypes::FWP_DATA_TYPE,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_FIELD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_FIELD_FLAGS: FWPM_FIELD_TYPE = 2;
pub const FWPM_FIELD_IP_ADDRESS: FWPM_FIELD_TYPE = 1;
pub const FWPM_FIELD_RAW_DATA: FWPM_FIELD_TYPE = 0;
pub type FWPM_FIELD_TYPE = i32;
pub const FWPM_FIELD_TYPE_MAX: FWPM_FIELD_TYPE = 3;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_FILTER0 {
    pub filterKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub layerKey: windows_core::GUID,
    pub subLayerKey: windows_core::GUID,
    pub weight: super::fwptypes::FWP_VALUE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub action: FWPM_ACTION0,
    pub Anonymous: FWPM_FILTER0_0,
    pub reserved: *mut windows_core::GUID,
    pub filterId: u64,
    pub effectiveWeight: super::fwptypes::FWP_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_FILTER0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_FILTER0_0 {
    pub rawContext: u64,
    pub providerContextKey: windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_FILTER0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_FILTER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub filterKey: windows_core::GUID,
    pub filterId: u64,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_FILTER_CONDITION0 {
    pub fieldKey: windows_core::GUID,
    pub matchType: super::fwptypes::FWP_MATCH_TYPE,
    pub conditionValue: super::fwptypes::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_FILTER_CONDITION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_FILTER_ENUM_TEMPLATE0 {
    pub providerKey: *mut windows_core::GUID,
    pub layerKey: windows_core::GUID,
    pub enumType: super::fwptypes::FWP_FILTER_ENUM_TYPE,
    pub flags: u32,
    pub providerContextTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub actionMask: u32,
    pub calloutKey: *mut windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_FILTER_FLAG_BOOTTIME: u32 = 2;
pub const FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT: u32 = 8;
pub const FWPM_FILTER_FLAG_DISABLED: u32 = 32;
pub const FWPM_FILTER_FLAG_GAMEOS_ONLY: u32 = 512;
pub const FWPM_FILTER_FLAG_HAS_PROVIDER_CONTEXT: u32 = 4;
pub const FWPM_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: u32 = 128;
pub const FWPM_FILTER_FLAG_INDEXED: u32 = 64;
pub const FWPM_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: u32 = 2048;
pub const FWPM_FILTER_FLAG_NONE: u32 = 0;
pub const FWPM_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: u32 = 16;
pub const FWPM_FILTER_FLAG_PERSISTENT: u32 = 1;
pub const FWPM_FILTER_FLAG_RESERVED0: u32 = 4096;
pub const FWPM_FILTER_FLAG_RESERVED1: u32 = 8192;
pub const FWPM_FILTER_FLAG_RESERVED2: u32 = 16384;
pub const FWPM_FILTER_FLAG_SILENT_MODE: u32 = 1024;
pub const FWPM_FILTER_FLAG_SYSTEMOS_ONLY: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_FILTER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_FILTER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_FILTER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_GENERAL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 8;
pub const FWPM_IPSEC_AUTHIP_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 6;
pub const FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 3;
pub const FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 4;
pub const FWPM_IPSEC_DOSP_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 11;
pub const FWPM_IPSEC_IKEV2_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 10;
pub const FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 12;
pub const FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 9;
pub const FWPM_IPSEC_IKE_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 5;
pub const FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 1;
pub const FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 2;
pub const FWPM_IPSEC_KEYING_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 0;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_LAYER0 {
    pub layerKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub numFields: u32,
    pub field: *mut FWPM_FIELD0,
    pub defaultSubLayerKey: windows_core::GUID,
    pub layerId: u16,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_LAYER0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_LAYER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
pub const FWPM_LAYER_FLAG_BUFFERED: u32 = 8;
pub const FWPM_LAYER_FLAG_BUILTIN: u32 = 2;
pub const FWPM_LAYER_FLAG_CLASSIFY_MOSTLY: u32 = 4;
pub const FWPM_LAYER_FLAG_KERNEL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_LAYER_STATISTICS1 {
    pub layerId: windows_core::GUID,
    pub classifyPermitCount: u32,
    pub classifyBlockCount: u32,
    pub classifyVetoCount: u32,
    pub numCacheEntries: u32,
    pub filterCount: u32,
    pub totalFilterSize: u32,
}
pub const FWPM_NETWORK_CONNECTION_POLICY_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = 13;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NETWORK_CONNECTION_POLICY_SETTING0 {
    pub r#type: super::fwptypes::FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE,
    pub value: super::fwptypes::FWP_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_NETWORK_CONNECTION_POLICY_SETTING0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0 {
    pub numSettings: u32,
    pub settings: *mut FWPM_NETWORK_CONNECTION_POLICY_SETTING0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT0 {
    pub header: FWPM_NET_EVENT_HEADER0,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT0_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT0_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE0,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE0,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP0,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT1 {
    pub header: FWPM_NET_EVENT_HEADER1,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT1_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT1_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP1,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT2 {
    pub header: FWPM_NET_EVENT_HEADER2,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT2_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT2_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT3 {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT3_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT3_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT4 {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT4_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT4_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE2,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE1,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT5 {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT5_0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT5_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE2,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE1,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
    pub lpmPacketArrival: *mut FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CAPABILITY_DROP0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP0 {
    pub filterId: u64,
    pub layerId: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP1 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP2 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: windows_core::BOOL,
    pub vSwitchId: super::fwptypes::FWP_BYTE_BLOB,
    pub vSwitchSourcePort: u32,
    pub vSwitchDestinationPort: u32,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    pub localMacAddr: super::fwptypes::FWP_BYTE_ARRAY6,
    pub remoteMacAddr: super::fwptypes::FWP_BYTE_ARRAY6,
    pub mediaType: u32,
    pub ifType: u32,
    pub etherType: u16,
    pub ndisPortNumber: u32,
    pub reserved: u32,
    pub vlanTag: u16,
    pub ifLuid: u64,
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: windows_core::BOOL,
    pub vSwitchId: super::fwptypes::FWP_BYTE_BLOB,
    pub vSwitchSourcePort: u32,
    pub vSwitchDestinationPort: u32,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    pub startTime: super::minwindef::FILETIME,
    pub endTime: super::minwindef::FILETIME,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_NET_EVENT_FLAG_APP_ID_SET: u32 = 32;
pub const FWPM_NET_EVENT_FLAG_EFFECTIVE_NAME_SET: u32 = 8192;
pub const FWPM_NET_EVENT_FLAG_ENTERPRISE_ID_SET: u32 = 2048;
pub const FWPM_NET_EVENT_FLAG_IP_PROTOCOL_SET: u32 = 1;
pub const FWPM_NET_EVENT_FLAG_IP_VERSION_SET: u32 = 256;
pub const FWPM_NET_EVENT_FLAG_LOCAL_ADDR_SET: u32 = 2;
pub const FWPM_NET_EVENT_FLAG_LOCAL_PORT_SET: u32 = 8;
pub const FWPM_NET_EVENT_FLAG_PACKAGE_ID_SET: u32 = 1024;
pub const FWPM_NET_EVENT_FLAG_POLICY_FLAGS_SET: u32 = 4096;
pub const FWPM_NET_EVENT_FLAG_REAUTH_REASON_SET: u32 = 512;
pub const FWPM_NET_EVENT_FLAG_REMOTE_ADDR_SET: u32 = 4;
pub const FWPM_NET_EVENT_FLAG_REMOTE_PORT_SET: u32 = 16;
pub const FWPM_NET_EVENT_FLAG_SCOPE_ID_SET: u32 = 128;
pub const FWPM_NET_EVENT_FLAG_USER_ID_SET: u32 = 64;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_HEADER0 {
    pub timeStamp: super::minwindef::FILETIME,
    pub flags: u32,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous: FWPM_NET_EVENT_HEADER0_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER0_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: super::fwptypes::FWP_BYTE_BLOB,
    pub userId: *mut super::winnt::SID,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER0_0 {
    pub localAddrV4: u32,
    pub localAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER0_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_HEADER1 {
    pub timeStamp: super::minwindef::FILETIME,
    pub flags: u32,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous: FWPM_NET_EVENT_HEADER1_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER1_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: super::fwptypes::FWP_BYTE_BLOB,
    pub userId: *mut super::winnt::SID,
    pub Anonymous3: FWPM_NET_EVENT_HEADER1_2,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER1_0 {
    pub localAddrV4: u32,
    pub localAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER1_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER1_2 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_HEADER1_2_0 {
    pub reserved1: super::fwptypes::FWP_AF,
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER1_2_0_0 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0_0,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    pub reserved2: super::fwptypes::FWP_BYTE_ARRAY6,
    pub reserved3: super::fwptypes::FWP_BYTE_ARRAY6,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u16,
    pub reserved7: u32,
    pub reserved8: u32,
    pub reserved9: u16,
    pub reserved10: u64,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_HEADER2 {
    pub timeStamp: super::minwindef::FILETIME,
    pub flags: u32,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous: FWPM_NET_EVENT_HEADER2_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER2_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: super::fwptypes::FWP_BYTE_BLOB,
    pub userId: *mut super::winnt::SID,
    pub addressFamily: super::fwptypes::FWP_AF,
    pub packageSid: *mut super::winnt::SID,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER2_0 {
    pub localAddrV4: u32,
    pub localAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER2_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_HEADER3 {
    pub timeStamp: super::minwindef::FILETIME,
    pub flags: u32,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous: FWPM_NET_EVENT_HEADER3_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER3_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: super::fwptypes::FWP_BYTE_BLOB,
    pub userId: *mut super::winnt::SID,
    pub addressFamily: super::fwptypes::FWP_AF,
    pub packageSid: *mut super::winnt::SID,
    pub enterpriseId: *mut u16,
    pub policyFlags: u64,
    pub effectiveName: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER3_0 {
    pub localAddrV4: u32,
    pub localAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_HEADER3_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: super::fwptypes::FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_HEADER3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub emState: super::iketypes::IKEEXT_EM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub emAuthMethod: super::iketypes::IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub qmFilterId: u64,
}
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
impl Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub emState: super::iketypes::IKEEXT_EM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub emAuthMethod: super::iketypes::IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub qmFilterId: u64,
    pub localPrincipalNameForAuth: *mut u16,
    pub remotePrincipalNameForAuth: *mut u16,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut windows_core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut windows_core::PWSTR,
    pub saTrafficType: super::ipsectypes::IPSEC_TRAFFIC_TYPE,
}
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
impl Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_BENIGN: u32 = 2;
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_MULTIPLE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub mmState: super::iketypes::IKEEXT_MM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub mmAuthMethod: super::iketypes::IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
}
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
impl Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub mmState: super::iketypes::IKEEXT_MM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub mmAuthMethod: super::iketypes::IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
    pub localPrincipalNameForAuth: *mut u16,
    pub remotePrincipalNameForAuth: *mut u16,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut windows_core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut windows_core::PWSTR,
}
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
impl Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub mmState: super::iketypes::IKEEXT_MM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub mmAuthMethod: super::iketypes::IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
    pub localPrincipalNameForAuth: *mut u16,
    pub remotePrincipalNameForAuth: *mut u16,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut windows_core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut windows_core::PWSTR,
    pub providerContextKey: *mut windows_core::GUID,
}
#[cfg(all(feature = "iketypes", feature = "ipsectypes"))]
impl Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_BENIGN: u32 = 1;
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_MULTIPLE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub keyingModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub qmState: super::iketypes::IKEEXT_QM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub saTrafficType: super::ipsectypes::IPSEC_TRAFFIC_TYPE,
    pub Anonymous: FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0,
    pub Anonymous2: FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1,
    pub qmFilterId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    pub localSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    pub remoteSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE1 {
    pub failureErrorCode: u32,
    pub failurePoint: super::ipsectypes::IPSEC_FAILURE_POINT,
    pub keyingModuleType: super::iketypes::IKEEXT_KEY_MODULE_TYPE,
    pub qmState: super::iketypes::IKEEXT_QM_SA_STATE,
    pub saRole: super::iketypes::IKEEXT_SA_ROLE,
    pub saTrafficType: super::ipsectypes::IPSEC_TRAFFIC_TYPE,
    pub Anonymous: FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_0,
    pub Anonymous2: FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_1,
    pub qmFilterId: u64,
    pub mmSaLuid: u64,
    pub mmProviderContextKey: windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_0 {
    pub localSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_1 {
    pub remoteSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0,
    pub Anonymous2: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1,
    pub failureStatus: i32,
    pub direction: super::fwptypes::FWP_DIRECTION,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    pub publicHostV4Addr: u32,
    pub publicHostV6Addr: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    pub internalHostV4Addr: u32,
    pub internalHostV6Addr: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    pub failureStatus: i32,
    pub direction: super::fwptypes::FWP_DIRECTION,
    pub spi: super::ipsectypes::IPSEC_SA_SPI,
    pub filterId: u64,
    pub layerId: u16,
}
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_ALLOW: u32 = 8;
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_DROP: u32 = 4;
pub const FWPM_NET_EVENT_KEYWORD_CLASSIFY_ALLOW: u32 = 16;
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_BCAST: u32 = 2;
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_MCAST: u32 = 1;
pub const FWPM_NET_EVENT_KEYWORD_PORT_SCANNING_DROP: u32 = 32;
#[repr(C)]
#[cfg(feature = "ipsectypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 {
    pub spi: super::ipsectypes::IPSEC_SA_SPI,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_NET_EVENT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_NET_EVENT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
impl Default for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FWPM_NET_EVENT_TYPE = i32;
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW: FWPM_NET_EVENT_TYPE = 8;
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_DROP: FWPM_NET_EVENT_TYPE = 7;
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW: FWPM_NET_EVENT_TYPE = 6;
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP: FWPM_NET_EVENT_TYPE = 3;
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC: FWPM_NET_EVENT_TYPE = 9;
pub const FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE: FWPM_NET_EVENT_TYPE = 2;
pub const FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE: FWPM_NET_EVENT_TYPE = 0;
pub const FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE: FWPM_NET_EVENT_TYPE = 1;
pub const FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP: FWPM_NET_EVENT_TYPE = 5;
pub const FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP: FWPM_NET_EVENT_TYPE = 4;
pub const FWPM_NET_EVENT_TYPE_LPM_PACKET_ARRIVAL: FWPM_NET_EVENT_TYPE = 10;
pub const FWPM_NET_EVENT_TYPE_MAX: FWPM_NET_EVENT_TYPE = 11;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_PROVIDER0 {
    pub providerKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub serviceName: *mut u16,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_PROVIDER0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_PROVIDER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerKey: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_PROVIDER_CONTEXT0 {
    pub providerContextKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT0_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_PROVIDER_CONTEXT0_0 {
    pub keyingPolicy: *mut super::ipsectypes::IPSEC_KEYING_POLICY0,
    pub ikeQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY0,
    pub ikeQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY0,
    pub authipQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY0,
    pub authipQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY0,
    pub ikeMmPolicy: *mut super::iketypes::IKEEXT_POLICY0,
    pub authIpMmPolicy: *mut super::iketypes::IKEEXT_POLICY0,
    pub dataBuffer: *mut super::fwptypes::FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_PROVIDER_CONTEXT1 {
    pub providerContextKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT1_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_PROVIDER_CONTEXT1_0 {
    pub keyingPolicy: *mut super::ipsectypes::IPSEC_KEYING_POLICY0,
    pub ikeQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY1,
    pub ikeQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY1,
    pub authipQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY1,
    pub authipQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY1,
    pub ikeMmPolicy: *mut super::iketypes::IKEEXT_POLICY1,
    pub authIpMmPolicy: *mut super::iketypes::IKEEXT_POLICY1,
    pub dataBuffer: *mut super::fwptypes::FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY1,
    pub ikeV2MmPolicy: *mut super::iketypes::IKEEXT_POLICY1,
    pub idpOptions: *mut super::ipsectypes::IPSEC_DOSP_OPTIONS0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_PROVIDER_CONTEXT2 {
    pub providerContextKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT2_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_PROVIDER_CONTEXT2_0 {
    pub keyingPolicy: *mut super::ipsectypes::IPSEC_KEYING_POLICY1,
    pub ikeQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub ikeQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY2,
    pub authipQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub authipQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY2,
    pub ikeMmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub authIpMmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub dataBuffer: *mut super::fwptypes::FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY2,
    pub ikeV2QmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub ikeV2MmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub idpOptions: *mut super::ipsectypes::IPSEC_DOSP_OPTIONS0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FWPM_PROVIDER_CONTEXT3 {
    pub providerContextKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT3_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FWPM_PROVIDER_CONTEXT3_0 {
    pub keyingPolicy: *mut super::ipsectypes::IPSEC_KEYING_POLICY1,
    pub ikeQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub ikeQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY3,
    pub authipQmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub authipQmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY3,
    pub ikeMmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub authIpMmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub dataBuffer: *mut super::fwptypes::FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut super::ipsectypes::IPSEC_TUNNEL_POLICY3,
    pub ikeV2QmTransportPolicy: *mut super::ipsectypes::IPSEC_TRANSPORT_POLICY2,
    pub ikeV2MmPolicy: *mut super::iketypes::IKEEXT_POLICY2,
    pub idpOptions: *mut super::ipsectypes::IPSEC_DOSP_OPTIONS0,
    pub networkConnectionPolicy: *mut FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
impl Default for FWPM_PROVIDER_CONTEXT3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_PROVIDER_CONTEXT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerContextKey: windows_core::GUID,
    pub providerContextId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    pub providerKey: *mut windows_core::GUID,
    pub providerContextType: FWPM_PROVIDER_CONTEXT_TYPE,
}
impl Default for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_PROVIDER_CONTEXT_FLAG_DOWNLEVEL: u32 = 2;
pub const FWPM_PROVIDER_CONTEXT_FLAG_PERSISTENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
impl Default for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FWPM_PROVIDER_CONTEXT_TYPE = i32;
pub const FWPM_PROVIDER_CONTEXT_TYPE_MAX: FWPM_PROVIDER_CONTEXT_TYPE = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_PROVIDER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
pub const FWPM_PROVIDER_FLAG_DISABLED: u32 = 16;
pub const FWPM_PROVIDER_FLAG_PERSISTENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_PROVIDER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
impl Default for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_SERVICE_RUNNING: FWPM_SERVICE_STATE = 3;
pub const FWPM_SERVICE_START_PENDING: FWPM_SERVICE_STATE = 1;
pub type FWPM_SERVICE_STATE = i32;
pub const FWPM_SERVICE_STATE_MAX: FWPM_SERVICE_STATE = 4;
pub const FWPM_SERVICE_STOPPED: FWPM_SERVICE_STATE = 0;
pub const FWPM_SERVICE_STOP_PENDING: FWPM_SERVICE_STATE = 2;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SESSION0 {
    pub sessionKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub txnWaitTimeoutInMSec: u32,
    pub processId: u32,
    pub sid: *mut super::winnt::SID,
    pub username: *mut u16,
    pub kernelMode: windows_core::BOOL,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for FWPM_SESSION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_SESSION_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
pub const FWPM_SESSION_FLAG_DYNAMIC: u32 = 1;
pub const FWPM_SESSION_FLAG_RESERVED: u32 = 268435456;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_STATISTICS1 {
    pub numLayerStatistics: u32,
    pub layerStatistics: *mut FWPM_LAYER_STATISTICS1,
    pub inboundAllowedConnectionsV4: u32,
    pub inboundBlockedConnectionsV4: u32,
    pub outboundAllowedConnectionsV4: u32,
    pub outboundBlockedConnectionsV4: u32,
    pub inboundAllowedConnectionsV6: u32,
    pub inboundBlockedConnectionsV6: u32,
    pub outboundAllowedConnectionsV6: u32,
    pub outboundBlockedConnectionsV6: u32,
    pub inboundActiveConnectionsV4: u32,
    pub outboundActiveConnectionsV4: u32,
    pub inboundActiveConnectionsV6: u32,
    pub outboundActiveConnectionsV6: u32,
    pub reauthDirInbound: u64,
    pub reauthDirOutbound: u64,
    pub reauthFamilyV4: u64,
    pub reauthFamilyV6: u64,
    pub reauthProtoOther: u64,
    pub reauthProtoIPv4: u64,
    pub reauthProtoIPv6: u64,
    pub reauthProtoICMP: u64,
    pub reauthProtoICMP6: u64,
    pub reauthProtoUDP: u64,
    pub reauthProtoTCP: u64,
    pub reauthReasonPolicyChange: u64,
    pub reauthReasonNewArrivalInterface: u64,
    pub reauthReasonNewNextHopInterface: u64,
    pub reauthReasonProfileCrossing: u64,
    pub reauthReasonClassifyCompletion: u64,
    pub reauthReasonIPSecPropertiesChanged: u64,
    pub reauthReasonMidStreamInspection: u64,
    pub reauthReasonSocketPropertyChanged: u64,
    pub reauthReasonNewInboundMCastBCastPacket: u64,
    pub reauthReasonEDPPolicyChanged: u64,
    pub reauthReasonProxyHandleChanged: u64,
}
impl Default for FWPM_STATISTICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SUBLAYER0 {
    pub subLayerKey: windows_core::GUID,
    pub displayData: super::fwptypes::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut windows_core::GUID,
    pub providerData: super::fwptypes::FWP_BYTE_BLOB,
    pub weight: u16,
}
#[cfg(feature = "fwptypes")]
impl Default for FWPM_SUBLAYER0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_SUBLAYER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub subLayerKey: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    pub providerKey: *mut windows_core::GUID,
}
impl Default for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_SUBLAYER_FLAG_PERSISTENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SUBLAYER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_SUBLAYER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
impl Default for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_ADD: u32 = 1;
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_DELETE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SYSTEM_PORTS0 {
    pub numTypes: u32,
    pub types: *mut FWPM_SYSTEM_PORTS_BY_TYPE0,
}
impl Default for FWPM_SYSTEM_PORTS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_SYSTEM_PORTS_BY_TYPE0 {
    pub r#type: FWPM_SYSTEM_PORT_TYPE,
    pub numPorts: u32,
    pub ports: *mut u16,
}
impl Default for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_SYSTEM_PORT_IPHTTPS_IN: FWPM_SYSTEM_PORT_TYPE = 2;
pub const FWPM_SYSTEM_PORT_IPHTTPS_OUT: FWPM_SYSTEM_PORT_TYPE = 3;
pub const FWPM_SYSTEM_PORT_RPC_EPMAP: FWPM_SYSTEM_PORT_TYPE = 0;
pub const FWPM_SYSTEM_PORT_TEREDO: FWPM_SYSTEM_PORT_TYPE = 1;
pub type FWPM_SYSTEM_PORT_TYPE = i32;
pub const FWPM_SYSTEM_PORT_TYPE_MAX: FWPM_SYSTEM_PORT_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FWPM_VSWITCH_EVENT0 {
    pub eventType: FWPM_VSWITCH_EVENT_TYPE,
    pub vSwitchId: *mut u16,
    pub Anonymous: FWPM_VSWITCH_EVENT0_0,
}
impl Default for FWPM_VSWITCH_EVENT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FWPM_VSWITCH_EVENT0_0 {
    pub positionInfo: FWPM_VSWITCH_EVENT0_0_0,
    pub reorderInfo: FWPM_VSWITCH_EVENT0_0_1,
}
impl Default for FWPM_VSWITCH_EVENT0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_VSWITCH_EVENT0_0_0 {
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut windows_core::PWSTR,
}
impl Default for FWPM_VSWITCH_EVENT0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FWPM_VSWITCH_EVENT0_0_1 {
    pub inRequiredPosition: windows_core::BOOL,
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut windows_core::PWSTR,
}
impl Default for FWPM_VSWITCH_EVENT0_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE = 3;
pub const FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE = 2;
pub const FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER: FWPM_VSWITCH_EVENT_TYPE = 0;
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION: FWPM_VSWITCH_EVENT_TYPE = 1;
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER: FWPM_VSWITCH_EVENT_TYPE = 4;
pub const FWPM_VSWITCH_EVENT_MAX: FWPM_VSWITCH_EVENT_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
pub type FWPM_VSWITCH_EVENT_TYPE = i32;
pub const IKEEXT_CERT_HASH_LEN: u32 = 20;
pub type PDL_ADDRESS_TYPE = *mut DL_ADDRESS_TYPE;
