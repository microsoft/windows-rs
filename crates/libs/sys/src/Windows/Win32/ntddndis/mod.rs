pub const AUTHENTICATE: OFFLOAD_OPERATION_E = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BSSID_INFO {
    pub BSSID: NDIS_802_11_MAC_ADDRESS,
    pub PMKID: NDIS_802_11_PMKID_VALUE,
}
impl Default for BSSID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLOCK_NETWORK_DERIVED: u32 = 2;
pub const CLOCK_PRECISION: u32 = 4;
pub const ENCRYPT: OFFLOAD_OPERATION_E = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GEN_GET_NETCARD_TIME {
    pub ReadTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GEN_GET_TIME_CAPS {
    pub Flags: u32,
    pub ClockPrecision: u32,
}
pub const IOCTL_NDIS_ADD_TDI_DEVICE: u32 = 1507348;
pub const IOCTL_NDIS_DO_PNP_OPERATION: u32 = 1507336;
pub const IOCTL_NDIS_ENUMERATE_INTERFACES: u32 = 1507344;
pub const IOCTL_NDIS_GET_LOG_DATA: u32 = 1507358;
pub const IOCTL_NDIS_GET_VERSION: u32 = 1507360;
pub const IOCTL_NDIS_QUERY_ALL_STATS: u32 = 1507334;
pub const IOCTL_NDIS_QUERY_GLOBAL_STATS: u32 = 1507330;
pub const IOCTL_NDIS_QUERY_SELECTED_STATS: u32 = 1507342;
pub const IOCTL_NDIS_RESERVED1: u32 = 1507364;
pub const IOCTL_NDIS_RESERVED10: u32 = 1507400;
pub const IOCTL_NDIS_RESERVED11: u32 = 1507404;
pub const IOCTL_NDIS_RESERVED12: u32 = 1507408;
pub const IOCTL_NDIS_RESERVED13: u32 = 1507412;
pub const IOCTL_NDIS_RESERVED14: u32 = 1507416;
pub const IOCTL_NDIS_RESERVED15: u32 = 1507420;
pub const IOCTL_NDIS_RESERVED16: u32 = 1507424;
pub const IOCTL_NDIS_RESERVED17: u32 = 1507428;
pub const IOCTL_NDIS_RESERVED18: u32 = 1507432;
pub const IOCTL_NDIS_RESERVED19: u32 = 1507436;
pub const IOCTL_NDIS_RESERVED2: u32 = 1507368;
pub const IOCTL_NDIS_RESERVED20: u32 = 1507440;
pub const IOCTL_NDIS_RESERVED21: u32 = 1507444;
pub const IOCTL_NDIS_RESERVED22: u32 = 1507448;
pub const IOCTL_NDIS_RESERVED23: u32 = 1507452;
pub const IOCTL_NDIS_RESERVED24: u32 = 1507456;
pub const IOCTL_NDIS_RESERVED25: u32 = 1507460;
pub const IOCTL_NDIS_RESERVED26: u32 = 1507464;
pub const IOCTL_NDIS_RESERVED27: u32 = 1507468;
pub const IOCTL_NDIS_RESERVED28: u32 = 1507472;
pub const IOCTL_NDIS_RESERVED29: u32 = 1507476;
pub const IOCTL_NDIS_RESERVED3: u32 = 1507372;
pub const IOCTL_NDIS_RESERVED30: u32 = 1507480;
pub const IOCTL_NDIS_RESERVED4: u32 = 1507376;
pub const IOCTL_NDIS_RESERVED5: u32 = 1507380;
pub const IOCTL_NDIS_RESERVED6: u32 = 1540152;
pub const IOCTL_NDIS_RESERVED7: u32 = 1507390;
pub const IOCTL_NDIS_RESERVED8: u32 = 1507392;
pub const IOCTL_NDIS_RESERVED9: u32 = 1507396;
pub type IPAddr = u32;
pub type IPMask = u32;
pub const MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_AI_REQFI {
    pub Capabilities: u16,
    pub ListenInterval: u16,
    pub CurrentAPAddress: NDIS_802_11_MAC_ADDRESS,
}
impl Default for NDIS_802_11_AI_REQFI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_802_11_AI_REQFI_CAPABILITIES: u32 = 1;
pub const NDIS_802_11_AI_REQFI_CURRENTAPADDRESS: u32 = 4;
pub const NDIS_802_11_AI_REQFI_LISTENINTERVAL: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_AI_RESFI {
    pub Capabilities: u16,
    pub StatusCode: u16,
    pub AssociationId: u16,
}
pub const NDIS_802_11_AI_RESFI_ASSOCIATIONID: u32 = 4;
pub const NDIS_802_11_AI_RESFI_CAPABILITIES: u32 = 1;
pub const NDIS_802_11_AI_RESFI_STATUSCODE: u32 = 2;
pub type NDIS_802_11_ANTENNA = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_ASSOCIATION_INFORMATION {
    pub Length: u32,
    pub AvailableRequestFixedIEs: u16,
    pub RequestFixedIEs: NDIS_802_11_AI_REQFI,
    pub RequestIELength: u32,
    pub OffsetRequestIEs: u32,
    pub AvailableResponseFixedIEs: u16,
    pub ResponseFixedIEs: NDIS_802_11_AI_RESFI,
    pub ResponseIELength: u32,
    pub OffsetResponseIEs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    pub AuthModeSupported: NDIS_802_11_AUTHENTICATION_MODE,
    pub EncryptStatusSupported: NDIS_802_11_ENCRYPTION_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_AUTHENTICATION_EVENT {
    pub Status: NDIS_802_11_STATUS_INDICATION,
    pub Request: [NDIS_802_11_AUTHENTICATION_REQUEST; 1],
}
impl Default for NDIS_802_11_AUTHENTICATION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_AUTHENTICATION_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_AUTHENTICATION_REQUEST {
    pub Length: u32,
    pub Bssid: NDIS_802_11_MAC_ADDRESS,
    pub Flags: u32,
}
impl Default for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_802_11_AUTH_REQUEST_AUTH_FIELDS: u32 = 15;
pub const NDIS_802_11_AUTH_REQUEST_GROUP_ERROR: u32 = 14;
pub const NDIS_802_11_AUTH_REQUEST_KEYUPDATE: u32 = 2;
pub const NDIS_802_11_AUTH_REQUEST_PAIRWISE_ERROR: u32 = 6;
pub const NDIS_802_11_AUTH_REQUEST_REAUTH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_BSSID_LIST {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID; 1],
}
impl Default for NDIS_802_11_BSSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_BSSID_LIST_EX {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID_EX; 1],
}
impl Default for NDIS_802_11_BSSID_LIST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_CAPABILITY {
    pub Length: u32,
    pub Version: u32,
    pub NoOfPMKIDs: u32,
    pub NoOfAuthEncryptPairsSupported: u32,
    pub AuthenticationEncryptionSupported: [NDIS_802_11_AUTHENTICATION_ENCRYPTION; 1],
}
impl Default for NDIS_802_11_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_CONFIGURATION {
    pub Length: u32,
    pub BeaconPeriod: u32,
    pub ATIMWindow: u32,
    pub DSConfig: u32,
    pub FHConfig: NDIS_802_11_CONFIGURATION_FH,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_CONFIGURATION_FH {
    pub Length: u32,
    pub HopPattern: u32,
    pub HopSet: u32,
    pub DwellTime: u32,
}
pub type NDIS_802_11_ENCRYPTION_STATUS = NDIS_802_11_WEP_STATUS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_FIXED_IEs {
    pub Timestamp: [u8; 8],
    pub BeaconInterval: u16,
    pub Capabilities: u16,
}
impl Default for NDIS_802_11_FIXED_IEs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_FRAGMENTATION_THRESHOLD = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub BSSID: NDIS_802_11_MAC_ADDRESS,
    pub KeyRSC: NDIS_802_11_KEY_RSC,
    pub KeyMaterial: [u8; 1],
}
impl Default for NDIS_802_11_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_KEY_INDEX = u32;
pub type NDIS_802_11_KEY_RSC = u64;
pub const NDIS_802_11_LENGTH_RATES: u32 = 8;
pub const NDIS_802_11_LENGTH_RATES_EX: u32 = 16;
pub const NDIS_802_11_LENGTH_SSID: u32 = 32;
pub type NDIS_802_11_MAC_ADDRESS = [u8; 6];
pub type NDIS_802_11_MEDIA_STREAM_MODE = i32;
pub type NDIS_802_11_NETWORK_INFRASTRUCTURE = i32;
pub type NDIS_802_11_NETWORK_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_NETWORK_TYPE_LIST {
    pub NumberOfItems: u32,
    pub NetworkType: [NDIS_802_11_NETWORK_TYPE; 1],
}
impl Default for NDIS_802_11_NETWORK_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_NON_BCAST_SSID_LIST {
    pub NumberOfItems: u32,
    pub Non_Bcast_Ssid: [NDIS_802_11_SSID; 1],
}
impl Default for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_PMKID {
    pub Length: u32,
    pub BSSIDInfoCount: u32,
    pub BSSIDInfo: [BSSID_INFO; 1],
}
impl Default for NDIS_802_11_PMKID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_PMKID_CANDIDATE_LIST {
    pub Version: u32,
    pub NumCandidates: u32,
    pub CandidateList: [PMKID_CANDIDATE; 1],
}
impl Default for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_802_11_PMKID_CANDIDATE_PREAUTH_ENABLED: u32 = 1;
pub type NDIS_802_11_PMKID_VALUE = [u8; 16];
pub type NDIS_802_11_POWER_MODE = i32;
pub type NDIS_802_11_PRIVACY_FILTER = i32;
pub type NDIS_802_11_RADIO_STATUS = i32;
pub type NDIS_802_11_RATES = [u8; 8];
pub type NDIS_802_11_RATES_EX = [u8; 16];
pub type NDIS_802_11_RELOAD_DEFAULTS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_REMOVE_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub BSSID: NDIS_802_11_MAC_ADDRESS,
}
impl Default for NDIS_802_11_REMOVE_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_RSSI = i32;
pub type NDIS_802_11_RTS_THRESHOLD = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_SSID {
    pub SsidLength: u32,
    pub Ssid: [u8; 32],
}
impl Default for NDIS_802_11_SSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_STATISTICS {
    pub Length: u32,
    pub TransmittedFragmentCount: i64,
    pub MulticastTransmittedFrameCount: i64,
    pub FailedCount: i64,
    pub RetryCount: i64,
    pub MultipleRetryCount: i64,
    pub RTSSuccessCount: i64,
    pub RTSFailureCount: i64,
    pub ACKFailureCount: i64,
    pub FrameDuplicateCount: i64,
    pub ReceivedFragmentCount: i64,
    pub MulticastReceivedFrameCount: i64,
    pub FCSErrorCount: i64,
    pub TKIPLocalMICFailures: i64,
    pub TKIPICVErrorCount: i64,
    pub TKIPCounterMeasuresInvoked: i64,
    pub TKIPReplays: i64,
    pub CCMPFormatErrors: i64,
    pub CCMPReplays: i64,
    pub CCMPDecryptErrors: i64,
    pub FourWayHandshakeFailures: i64,
    pub WEPUndecryptableCount: i64,
    pub WEPICVErrorCount: i64,
    pub DecryptSuccessCount: i64,
    pub DecryptFailureCount: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_802_11_STATUS_INDICATION {
    pub StatusType: NDIS_802_11_STATUS_TYPE,
}
pub type NDIS_802_11_STATUS_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_TEST {
    pub Length: u32,
    pub Type: u32,
    pub Anonymous: NDIS_802_11_TEST_0,
}
impl Default for NDIS_802_11_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NDIS_802_11_TEST_0 {
    pub AuthenticationEvent: NDIS_802_11_AUTHENTICATION_EVENT,
    pub RssiTrigger: NDIS_802_11_RSSI,
}
impl Default for NDIS_802_11_TEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_TX_POWER_LEVEL = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_VARIABLE_IEs {
    pub ElementID: u8,
    pub Length: u8,
    pub data: [u8; 1],
}
impl Default for NDIS_802_11_VARIABLE_IEs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_802_11_WEP {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub KeyMaterial: [u8; 1],
}
impl Default for NDIS_802_11_WEP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_802_11_WEP_STATUS = i32;
pub const NDIS_802_3_MAC_OPTION_PRIORITY: u32 = 1;
pub type NDIS_802_5_RING_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_CO_DEVICE_PROFILE {
    pub DeviceDescription: NDIS_VAR_DATA_DESC,
    pub DevSpecificInfo: NDIS_VAR_DATA_DESC,
    pub ulTAPISupplementaryPassThru: u32,
    pub ulAddressModes: u32,
    pub ulNumAddresses: u32,
    pub ulBearerModes: u32,
    pub ulMaxTxRate: u32,
    pub ulMinTxRate: u32,
    pub ulMaxRxRate: u32,
    pub ulMinRxRate: u32,
    pub ulMediaModes: u32,
    pub ulGenerateToneModes: u32,
    pub ulGenerateToneMaxNumFreq: u32,
    pub ulGenerateDigitModes: u32,
    pub ulMonitorToneMaxNumFreq: u32,
    pub ulMonitorToneMaxNumEntries: u32,
    pub ulMonitorDigitModes: u32,
    pub ulGatherDigitsMinTimeout: u32,
    pub ulGatherDigitsMaxTimeout: u32,
    pub ulDevCapFlags: u32,
    pub ulMaxNumActiveCalls: u32,
    pub ulAnswerMode: u32,
    pub ulUUIAcceptSize: u32,
    pub ulUUIAnswerSize: u32,
    pub ulUUIMakeCallSize: u32,
    pub ulUUIDropSize: u32,
    pub ulUUISendUserUserInfoSize: u32,
    pub ulUUICallInfoSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_CO_LINK_SPEED {
    pub Outbound: u32,
    pub Inbound: u32,
}
pub const NDIS_CO_MAC_OPTION_DYNAMIC_LINK_SPEED: u32 = 1;
pub type NDIS_DEVICE_POWER_STATE = i32;
pub const NDIS_DEVICE_TYPE_ENDPOINT: u32 = 1;
pub const NDIS_DEVICE_WAKE_ON_MAGIC_PACKET_ENABLE: u32 = 4;
pub const NDIS_DEVICE_WAKE_ON_PATTERN_MATCH_ENABLE: u32 = 2;
pub const NDIS_DEVICE_WAKE_UP_ENABLE: u32 = 1;
pub const NDIS_ETH_TYPE_802_1Q: u32 = 33024;
pub const NDIS_ETH_TYPE_802_1X: u32 = 34958;
pub const NDIS_ETH_TYPE_ARP: u32 = 2054;
pub const NDIS_ETH_TYPE_IPV4: u32 = 2048;
pub const NDIS_ETH_TYPE_IPV6: u32 = 34525;
pub const NDIS_ETH_TYPE_SLOW_PROTOCOL: u32 = 34825;
pub type NDIS_FDDI_ATTACHMENT_TYPE = i32;
pub type NDIS_FDDI_LCONNECTION_STATE = i32;
pub type NDIS_FDDI_RING_MGT_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_oidtypes", feature = "Win32_types"))]
#[derive(Clone, Copy)]
pub struct NDIS_GUID {
    pub Guid: windows_sys::core::GUID,
    pub Anonymous: NDIS_GUID_0,
    pub Size: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_oidtypes", feature = "Win32_types"))]
impl Default for NDIS_GUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oidtypes", feature = "Win32_types"))]
#[derive(Clone, Copy)]
pub union NDIS_GUID_0 {
    pub Oid: super::oidtypes::NDIS_OID,
    pub Status: super::types::NDIS_STATUS,
}
#[cfg(all(feature = "Win32_oidtypes", feature = "Win32_types"))]
impl Default for NDIS_GUID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_HARDWARE_CROSSTIMESTAMP {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
pub const NDIS_HARDWARE_CROSSTIMESTAMP_REVISION_1: u32 = 1;
pub type NDIS_HARDWARE_STATUS = i32;
#[cfg(feature = "Win32_ifdef")]
pub type NDIS_IF_COUNTED_STRING = super::ifdef::IF_COUNTED_STRING;
pub const NDIS_IF_MAX_STRING_SIZE: u32 = 256;
#[cfg(feature = "Win32_ifdef")]
pub type NDIS_IF_PHYSICAL_ADDRESS = super::ifdef::IF_PHYSICAL_ADDRESS;
pub type NDIS_INTERRUPT_MODERATION = i32;
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_REINITIALIZE: u32 = 2;
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_RESET: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_INTERRUPT_MODERATION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub InterruptModeration: NDIS_INTERRUPT_MODERATION,
}
pub const NDIS_INTERRUPT_MODERATION_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_IPSEC_OFFLOAD_V1_0,
    pub IPv4AH: NDIS_IPSEC_OFFLOAD_V1_1,
    pub IPv4ESP: NDIS_IPSEC_OFFLOAD_V1_2,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IPSEC_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IPSEC_OFFLOAD_V1_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IPSEC_OFFLOAD_V1_2 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IP_OPER_STATE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub IpOperationalStatus: NDIS_IP_OPER_STATUS,
}
pub const NDIS_IP_OPER_STATE_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_ifdef")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IP_OPER_STATUS {
    pub AddressFamily: u32,
    pub OperationalStatus: super::ifdef::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_IP_OPER_STATUS_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub NumberofAddressFamiliesReturned: u32,
    pub IpOperationalStatus: [NDIS_IP_OPER_STATUS; 32],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
impl Default for NDIS_IP_OPER_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_IP_OPER_STATUS_INFO_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_IRDA_PACKET_INFO {
    pub ExtraBOFs: u32,
    pub MinTurnAroundTime: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_LINK_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MediaDuplexState: NDIS_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
pub const NDIS_LINK_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_LINK_SPEED {
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
}
pub const NDIS_LINK_SPEED_UNKNOWN: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_LINK_STATE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MediaConnectState: NDIS_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: NDIS_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
pub const NDIS_LINK_STATE_DUPLEX_AUTO_NEGOTIATED: u32 = 4;
pub const NDIS_LINK_STATE_PAUSE_FUNCTIONS_AUTO_NEGOTIATED: u32 = 8;
pub const NDIS_LINK_STATE_RCV_LINK_SPEED_AUTO_NEGOTIATED: u32 = 2;
pub const NDIS_LINK_STATE_REVISION_1: u32 = 1;
pub const NDIS_LINK_STATE_XMIT_LINK_SPEED_AUTO_NEGOTIATED: u32 = 1;
pub const NDIS_MAC_OPTION_8021P_PRIORITY: u32 = 64;
pub const NDIS_MAC_OPTION_8021Q_VLAN: u32 = 512;
pub const NDIS_MAC_OPTION_COPY_LOOKAHEAD_DATA: u32 = 1;
pub const NDIS_MAC_OPTION_EOTX_INDICATION: u32 = 32;
pub const NDIS_MAC_OPTION_FULL_DUPLEX: u32 = 16;
pub const NDIS_MAC_OPTION_NO_LOOPBACK: u32 = 8;
pub const NDIS_MAC_OPTION_RECEIVE_AT_DPC: u32 = 256;
pub const NDIS_MAC_OPTION_RECEIVE_SERIALIZED: u32 = 2;
pub const NDIS_MAC_OPTION_RESERVED: u32 = 2147483648;
pub const NDIS_MAC_OPTION_SUPPORTS_MAC_ADDRESS_OVERWRITE: u32 = 128;
pub const NDIS_MAC_OPTION_TRANSFERS_NOT_PEND: u32 = 4;
pub const NDIS_MAX_PHYS_ADDRESS_LENGTH: u32 = 32;
pub const NDIS_MEDIA_CAP_RECEIVE: u32 = 2;
pub const NDIS_MEDIA_CAP_TRANSMIT: u32 = 1;
#[cfg(feature = "Win32_ifdef")]
pub type NDIS_MEDIA_CONNECT_STATE = super::ifdef::NET_IF_MEDIA_CONNECT_STATE;
#[cfg(feature = "Win32_ifdef")]
pub type NDIS_MEDIA_DUPLEX_STATE = super::ifdef::NET_IF_MEDIA_DUPLEX_STATE;
pub type NDIS_MEDIA_STATE = i32;
pub type NDIS_MEDIUM = i32;
pub type NDIS_NETWORK_CHANGE_TYPE = i32;
pub const NDIS_OBJECT_TYPE_BIND_PARAMETERS: u32 = 134;
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 147;
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 142;
pub const NDIS_OBJECT_TYPE_CONFIGURATION_OBJECT: u32 = 169;
pub const NDIS_OBJECT_TYPE_CO_CALL_MANAGER_OPTIONAL_HANDLERS: u32 = 165;
pub const NDIS_OBJECT_TYPE_CO_CLIENT_OPTIONAL_HANDLERS: u32 = 166;
pub const NDIS_OBJECT_TYPE_CO_MINIPORT_CHARACTERISTICS: u32 = 145;
pub const NDIS_OBJECT_TYPE_CO_PROTOCOL_CHARACTERISTICS: u32 = 144;
pub const NDIS_OBJECT_TYPE_DEFAULT: u32 = 128;
pub const NDIS_OBJECT_TYPE_DEVICE_OBJECT_ATTRIBUTES: u32 = 133;
pub const NDIS_OBJECT_TYPE_DRIVER_WRAPPER_OBJECT: u32 = 170;
pub const NDIS_OBJECT_TYPE_FILTER_ATTACH_PARAMETERS: u32 = 153;
pub const NDIS_OBJECT_TYPE_FILTER_ATTRIBUTES: u32 = 141;
pub const NDIS_OBJECT_TYPE_FILTER_DRIVER_CHARACTERISTICS: u32 = 139;
pub const NDIS_OBJECT_TYPE_FILTER_PARTIAL_CHARACTERISTICS: u32 = 140;
pub const NDIS_OBJECT_TYPE_FILTER_PAUSE_PARAMETERS: u32 = 154;
pub const NDIS_OBJECT_TYPE_FILTER_RESTART_PARAMETERS: u32 = 155;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES: u32 = 159;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NATIVE_802_11_ATTRIBUTES: u32 = 161;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_OFFLOAD_ATTRIBUTES: u32 = 160;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES: u32 = 158;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADD_DEVICE_REGISTRATION_ATTRIBUTES: u32 = 164;
pub const NDIS_OBJECT_TYPE_MINIPORT_DRIVER_CHARACTERISTICS: u32 = 138;
pub const NDIS_OBJECT_TYPE_MINIPORT_INIT_PARAMETERS: u32 = 129;
pub const NDIS_OBJECT_TYPE_MINIPORT_INTERRUPT: u32 = 132;
pub const NDIS_OBJECT_TYPE_MINIPORT_PNP_CHARACTERISTICS: u32 = 146;
pub const NDIS_OBJECT_TYPE_NSI_COMPARTMENT_RW_STRUCT: u32 = 173;
pub const NDIS_OBJECT_TYPE_NSI_INTERFACE_PERSIST_RW_STRUCT: u32 = 174;
pub const NDIS_OBJECT_TYPE_NSI_NETWORK_RW_STRUCT: u32 = 172;
pub const NDIS_OBJECT_TYPE_OFFLOAD: u32 = 167;
pub const NDIS_OBJECT_TYPE_OFFLOAD_ENCAPSULATION: u32 = 168;
pub const NDIS_OBJECT_TYPE_OPEN_PARAMETERS: u32 = 135;
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1: u32 = 1;
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2: u32 = 2;
pub const NDIS_OBJECT_TYPE_PORT_CHARACTERISTICS: u32 = 156;
pub const NDIS_OBJECT_TYPE_PORT_STATE: u32 = 157;
pub const NDIS_OBJECT_TYPE_PROTOCOL_DRIVER_CHARACTERISTICS: u32 = 149;
pub const NDIS_OBJECT_TYPE_PROTOCOL_RESTART_PARAMETERS: u32 = 163;
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 148;
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 143;
pub const NDIS_OBJECT_TYPE_REQUEST_EX: u32 = 150;
pub const NDIS_OBJECT_TYPE_RESTART_GENERAL_ATTRIBUTES: u32 = 162;
pub const NDIS_OBJECT_TYPE_RSS_CAPABILITIES: u32 = 136;
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS: u32 = 137;
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS_V2: u32 = 200;
pub const NDIS_OBJECT_TYPE_RSS_SET_INDIRECTION_ENTRIES: u32 = 201;
pub const NDIS_OBJECT_TYPE_SG_DMA_DESCRIPTION: u32 = 131;
pub const NDIS_OBJECT_TYPE_STATUS_INDICATION: u32 = 152;
pub const NDIS_OBJECT_TYPE_TIMER_CHARACTERISTICS: u32 = 151;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_OFFLOAD {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
pub const NDIS_OFFLOAD_FLAGS_GROUP_CHECKSUM_CAPABILITIES: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_OFFLOAD_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub IPv4Checksum: u8,
    pub TCPIPv4Checksum: u8,
    pub UDPIPv4Checksum: u8,
    pub TCPIPv6Checksum: u8,
    pub UDPIPv6Checksum: u8,
    pub LsoV1: u8,
    pub IPsecV1: u8,
    pub LsoV2IPv4: u8,
    pub LsoV2IPv6: u8,
    pub TcpConnectionIPv4: u8,
    pub TcpConnectionIPv6: u8,
    pub Flags: u32,
}
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_DISABLED: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_ENABLED: u32 = 2;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_AND_ESP_ENABLED: u32 = 4;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_ENABLED: u32 = 2;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_DISABLED: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_ESP_ENABLED: u32 = 3;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_DISABLED: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_ENABLED: u32 = 2;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_DISABLED: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_ENABLED: u32 = 2;
pub const NDIS_OFFLOAD_PARAMETERS_NO_CHANGE: u32 = 0;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_1: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_RX_ENABLED_TX_DISABLED: u32 = 3;
pub const NDIS_OFFLOAD_PARAMETERS_TX_ENABLED_RX_DISABLED: u32 = 2;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_DISABLED: u32 = 1;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_ENABLED: u32 = 4;
pub const NDIS_OFFLOAD_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_OPER_STATE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub OperationalStatus: super::ifdef::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
pub const NDIS_OPER_STATE_REVISION_1: u32 = 1;
pub const NDIS_PACKET_TYPE_ALL_FUNCTIONAL: u32 = 8192;
pub const NDIS_PACKET_TYPE_ALL_LOCAL: u32 = 128;
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: u32 = 4;
pub const NDIS_PACKET_TYPE_BROADCAST: u32 = 8;
pub const NDIS_PACKET_TYPE_DIRECTED: u32 = 1;
pub const NDIS_PACKET_TYPE_FUNCTIONAL: u32 = 16384;
pub const NDIS_PACKET_TYPE_GROUP: u32 = 4096;
pub const NDIS_PACKET_TYPE_MAC_FRAME: u32 = 32768;
pub const NDIS_PACKET_TYPE_MULTICAST: u32 = 2;
pub const NDIS_PACKET_TYPE_NO_LOCAL: u32 = 65536;
pub const NDIS_PACKET_TYPE_PROMISCUOUS: u32 = 32;
pub const NDIS_PACKET_TYPE_SMT: u32 = 64;
pub const NDIS_PACKET_TYPE_SOURCE_ROUTING: u32 = 16;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub DeviceType: u32,
    pub CurrentSpeedAndMode: u32,
    pub CurrentPayloadSize: u32,
    pub MaxPayloadSize: u32,
    pub MaxReadRequestSize: u32,
    pub CurrentLinkSpeed: u32,
    pub CurrentLinkWidth: u32,
    pub MaxLinkSpeed: u32,
    pub MaxLinkWidth: u32,
    pub PciExpressVersion: u32,
    pub InterruptType: u32,
    pub MaxInterruptMessages: u32,
}
pub type NDIS_PHYSICAL_MEDIUM = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PM_PACKET_PATTERN {
    pub Priority: u32,
    pub Reserved: u32,
    pub MaskSize: u32,
    pub PatternOffset: u32,
    pub PatternSize: u32,
    pub PatternFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PM_WAKE_UP_CAPABILITIES {
    pub MinMagicPacketWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinPatternWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinLinkChangeWakeUp: NDIS_DEVICE_POWER_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PNP_CAPABILITIES {
    pub Flags: u32,
    pub WakeUpCapabilities: NDIS_PM_WAKE_UP_CAPABILITIES,
}
pub const NDIS_PNP_WAKE_UP_LINK_CHANGE: u32 = 4;
pub const NDIS_PNP_WAKE_UP_MAGIC_PACKET: u32 = 1;
pub const NDIS_PNP_WAKE_UP_PATTERN_MATCH: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_PORT {
    pub Next: PNDIS_PORT,
    pub NdisReserved: *mut core::ffi::c_void,
    pub MiniportReserved: *mut core::ffi::c_void,
    pub ProtocolReserved: *mut core::ffi::c_void,
    pub PortCharacteristics: NDIS_PORT_CHARACTERISTICS,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
impl Default for NDIS_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_PORT_ARRAY {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub NumberOfPorts: u32,
    pub OffsetFirstPort: u32,
    pub ElementSize: u32,
    pub Ports: [NDIS_PORT_CHARACTERISTICS; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
impl Default for NDIS_PORT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_PORT_ARRAY_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PORT_AUTHENTICATION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
pub const NDIS_PORT_AUTHENTICATION_PARAMETERS_REVISION_1: u32 = 1;
pub type NDIS_PORT_AUTHORIZATION_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PORT_CHARACTERISTICS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PortNumber: super::ndisport::NDIS_PORT_NUMBER,
    pub Flags: u32,
    pub Type: super::ndisport::NDIS_PORT_TYPE,
    pub MediaConnectState: NDIS_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::ifdef::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
pub const NDIS_PORT_CHARACTERISTICS_REVISION_1: u32 = 1;
pub const NDIS_PORT_CHAR_USE_DEFAULT_AUTH_SETTINGS: u32 = 1;
pub type NDIS_PORT_CONTROLL_STATE = NDIS_PORT_CONTROL_STATE;
pub type NDIS_PORT_CONTROL_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy, Default)]
pub struct NDIS_PORT_STATE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MediaConnectState: NDIS_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::ifdef::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub Flags: u32,
}
pub const NDIS_PORT_STATE_REVISION_1: u32 = 1;
pub type NDIS_PROCESSOR_VENDOR = i32;
pub const NDIS_PROTOCOL_ID_DEFAULT: u32 = 0;
pub const NDIS_PROTOCOL_ID_IP6: u32 = 3;
pub const NDIS_PROTOCOL_ID_IPX: u32 = 6;
pub const NDIS_PROTOCOL_ID_MASK: u32 = 15;
pub const NDIS_PROTOCOL_ID_MAX: u32 = 15;
pub const NDIS_PROTOCOL_ID_NBF: u32 = 7;
pub const NDIS_PROTOCOL_ID_TCP_IP: u32 = 2;
pub const NDIS_PROT_OPTION_ESTIMATED_LENGTH: u32 = 1;
pub const NDIS_PROT_OPTION_NO_LOOPBACK: u32 = 2;
pub const NDIS_PROT_OPTION_NO_RSVD_ON_RCVPKT: u32 = 4;
pub const NDIS_PROT_OPTION_SEND_RESTRICTED: u32 = 8;
pub const NDIS_RECEIVE_HASH_FLAG_ENABLE_HASH: u32 = 1;
pub const NDIS_RECEIVE_HASH_FLAG_HASH_INFO_UNCHANGED: u32 = 2;
pub const NDIS_RECEIVE_HASH_FLAG_HASH_KEY_UNCHANGED: u32 = 4;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_RECEIVE_HASH_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub HashInformation: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
pub const NDIS_RECEIVE_HASH_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_RECEIVE_SCALE_CAPABILITIES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub CapabilitiesFlags: NDIS_RSS_CAPS_FLAGS,
    pub NumberOfInterruptMessages: u32,
    pub NumberOfReceiveQueues: u32,
}
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_RECEIVE_SCALE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u16,
    pub BaseCpuNumber: u16,
    pub HashInformation: u32,
    pub IndirectionTableSize: u16,
    pub IndirectionTableOffset: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_1: u32 = 1;
pub const NDIS_RING_AUTO_REMOVAL_ERROR: u32 = 1024;
pub const NDIS_RING_COUNTER_OVERFLOW: u32 = 256;
pub const NDIS_RING_HARD_ERROR: u32 = 16384;
pub const NDIS_RING_LOBE_WIRE_FAULT: u32 = 2048;
pub const NDIS_RING_REMOVE_RECEIVED: u32 = 512;
pub const NDIS_RING_RING_RECOVERY: u32 = 64;
pub const NDIS_RING_SIGNAL_LOSS: u32 = 32768;
pub const NDIS_RING_SINGLE_STATION: u32 = 128;
pub const NDIS_RING_SOFT_ERROR: u32 = 8192;
pub const NDIS_RING_TRANSMIT_BEACON: u32 = 4096;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_DPC: u32 = 67108864;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_ISR: u32 = 33554432;
pub type NDIS_RSS_CAPS_FLAGS = u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV4: u32 = 256;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6: u32 = 512;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6_EX: u32 = 1024;
pub const NDIS_RSS_CAPS_MESSAGE_SIGNALED_INTERRUPTS: u32 = 16777216;
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_1: u32 = 40;
pub const NDIS_RSS_HASH_SECRET_KEY_SIZE_REVISION_1: u32 = 40;
pub const NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_1: u32 = 128;
pub const NDIS_RSS_INDIRECTION_TABLE_SIZE_REVISION_1: u32 = 128;
pub const NDIS_RSS_PARAM_FLAG_BASE_CPU_UNCHANGED: u32 = 1;
pub const NDIS_RSS_PARAM_FLAG_DISABLE_RSS: u32 = 16;
pub const NDIS_RSS_PARAM_FLAG_HASH_INFO_UNCHANGED: u32 = 2;
pub const NDIS_RSS_PARAM_FLAG_HASH_KEY_UNCHANGED: u32 = 8;
pub const NDIS_RSS_PARAM_FLAG_ITABLE_UNCHANGED: u32 = 4;
pub const NDIS_SIZEOF_HARDWARE_CROSSTIMESTAMP_REVISION_1: u32 = 32;
pub const NDIS_SIZEOF_INTERRUPT_MODERATION_PARAMETERS_REVISION_1: u32 = 12;
pub const NDIS_SIZEOF_IP_OPER_STATE_REVISION_1: u32 = 20;
pub const NDIS_SIZEOF_LINK_PARAMETERS_REVISION_1: u32 = 32;
pub const NDIS_SIZEOF_LINK_STATE_REVISION_1: u32 = 40;
pub const NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_1: u32 = 112;
pub const NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_1: u32 = 224;
pub const NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_1: u32 = 20;
pub const NDIS_SIZEOF_OPER_STATE_REVISION_1: u32 = 12;
pub const NDIS_SIZEOF_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1: u32 = 40;
pub const NDIS_SIZEOF_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2: u32 = 52;
pub const NDIS_SIZEOF_PORT_ARRAY_REVISION_1: u32 = 80;
pub const NDIS_SIZEOF_PORT_AUTHENTICATION_PARAMETERS_REVISION_1: u32 = 20;
pub const NDIS_SIZEOF_PORT_CHARACTERISTICS_REVISION_1: u32 = 60;
pub const NDIS_SIZEOF_PORT_STATE_REVISION_1: u32 = 48;
pub const NDIS_SIZEOF_RECEIVE_HASH_PARAMETERS_REVISION_1: u32 = 20;
pub const NDIS_SIZEOF_RECEIVE_SCALE_CAPABILITIES_REVISION_1: u32 = 16;
pub const NDIS_SIZEOF_RECEIVE_SCALE_PARAMETERS_REVISION_1: u32 = 28;
pub const NDIS_SIZEOF_STATISTICS_INFO_REVISION_1: u32 = 152;
pub const NDIS_SIZEOF_TCP_CONNECTION_OFFLOAD_REVISION_1: u32 = 20;
pub const NDIS_SIZEOF_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1: u32 = 16;
pub const NDIS_SIZEOF_TIMESTAMP_CAPABILITIES_REVISION_1: u32 = 54;
pub const NDIS_SIZEOF_WMI_ENUM_ADAPTER_REVISION_1: u32 = 19;
pub const NDIS_SIZEOF_WMI_EVENT_HEADER_REVISION_1: u32 = 40;
pub const NDIS_SIZEOF_WMI_METHOD_HEADER_REVISION_1: u32 = 32;
pub const NDIS_SIZEOF_WMI_OUTPUT_INFO_REVISION_1: u32 = 16;
pub const NDIS_SIZEOF_WMI_SET_HEADER_REVISION_1: u32 = 32;
pub const NDIS_SIZEOF_WMI_TCP_CONNECTION_OFFLOAD_REVISION_1: u32 = 32;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_RCV: u32 = 262144;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_XMIT: u32 = 2097152;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_RCV: u32 = 4;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_XMIT: u32 = 256;
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_RCV: u32 = 8;
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_XMIT: u32 = 512;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_RCV: u32 = 65536;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_XMIT: u32 = 524288;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_RCV: u32 = 1;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_XMIT: u32 = 64;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_RCV: u32 = 131072;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_XMIT: u32 = 1048576;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_RCV: u32 = 2;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_XMIT: u32 = 128;
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_DISCARDS: u32 = 16;
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_ERROR: u32 = 32;
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_DISCARDS: u32 = 32768;
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_ERROR: u32 = 1024;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_STATISTICS_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub SupportedStatistics: u32,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
}
pub const NDIS_STATISTICS_INFO_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_oidtypes")]
#[derive(Clone, Copy)]
pub struct NDIS_STATISTICS_VALUE {
    pub Oid: super::oidtypes::NDIS_OID,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_oidtypes")]
impl Default for NDIS_STATISTICS_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_oidtypes")]
#[derive(Clone, Copy)]
pub struct NDIS_STATISTICS_VALUE_EX {
    pub Oid: super::oidtypes::NDIS_OID,
    pub DataLength: u32,
    pub Length: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_oidtypes")]
impl Default for NDIS_STATISTICS_VALUE_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDIS_SUPPORTED_PAUSE_FUNCTIONS = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_CONNECTION_OFFLOAD {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub _bitfield: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
pub const NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv4Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv6Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_2,
    pub IPv6Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_3,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy)]
pub struct NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub TimeoutArrayLength: u32,
    pub TimeoutArray: [u32; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TIMESTAMP_CAPABILITIES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub HardwareClockFrequencyHz: u64,
    pub CrossTimestamp: bool,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub TimestampFlags: NDIS_TIMESTAMP_CAPABILITY_FLAGS,
}
pub const NDIS_TIMESTAMP_CAPABILITIES_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    pub PtpV2OverUdpIPv4EventMsgReceiveHw: bool,
    pub PtpV2OverUdpIPv4AllMsgReceiveHw: bool,
    pub PtpV2OverUdpIPv4EventMsgTransmitHw: bool,
    pub PtpV2OverUdpIPv4AllMsgTransmitHw: bool,
    pub PtpV2OverUdpIPv6EventMsgReceiveHw: bool,
    pub PtpV2OverUdpIPv6AllMsgReceiveHw: bool,
    pub PtpV2OverUdpIPv6EventMsgTransmitHw: bool,
    pub PtpV2OverUdpIPv6AllMsgTransmitHw: bool,
    pub AllReceiveHw: bool,
    pub AllTransmitHw: bool,
    pub TaggedTransmitHw: bool,
    pub AllReceiveSw: bool,
    pub AllTransmitSw: bool,
    pub TaggedTransmitSw: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_VAR_DATA_DESC {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Offset: usize,
}
pub type NDIS_VLAN_ID = u32;
pub type NDIS_WAN_HEADER_FORMAT = i32;
pub type NDIS_WAN_MEDIUM_SUBTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WAN_PROTOCOL_CAPS {
    pub Flags: u32,
    pub Reserved: u32,
}
pub type NDIS_WAN_QUALITY = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_WLAN_BSSID {
    pub Length: u32,
    pub MacAddress: NDIS_802_11_MAC_ADDRESS,
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: NDIS_802_11_RSSI,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: NDIS_802_11_RATES,
}
impl Default for NDIS_WLAN_BSSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NDIS_WLAN_BSSID_EX {
    pub Length: u32,
    pub MacAddress: NDIS_802_11_MAC_ADDRESS,
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: NDIS_802_11_RSSI,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: NDIS_802_11_RATES_EX,
    pub IELength: u32,
    pub IEs: [u8; 1],
}
impl Default for NDIS_WLAN_BSSID_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_WMI_DEFAULT_METHOD_ID: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_WMI_ENUM_ADAPTER {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub IfIndex: super::ifdef::NET_IFINDEX,
    pub NetLuid: super::ifdef::NET_LUID,
    pub DeviceNameLength: u16,
    pub DeviceName: [i8; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
impl Default for NDIS_WMI_ENUM_ADAPTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_WMI_ENUM_ADAPTER_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_WMI_EVENT_HEADER {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub IfIndex: super::ifdef::NET_IFINDEX,
    pub NetLuid: super::ifdef::NET_LUID,
    pub RequestId: u64,
    pub PortNumber: super::ndisport::NDIS_PORT_NUMBER,
    pub DeviceNameLength: u32,
    pub DeviceNameOffset: u32,
    pub Padding: [u8; 4],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
impl Default for NDIS_WMI_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_WMI_EVENT_HEADER_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_WMI_IPSEC_OFFLOAD_V1_0,
    pub IPv4AH: NDIS_WMI_IPSEC_OFFLOAD_V1_1,
    pub IPv4ESP: NDIS_WMI_IPSEC_OFFLOAD_V1_2,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    pub Md5: u32,
    pub Sha_1: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    pub Des: u32,
    pub Reserved: u32,
    pub TripleDes: u32,
    pub NullEsp: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_WMI_METHOD_HEADER {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PortNumber: super::ndisport::NDIS_PORT_NUMBER,
    pub NetLuid: super::ifdef::NET_LUID,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
impl Default for NDIS_WMI_METHOD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_WMI_METHOD_HEADER_REVISION_1: u32 = 1;
pub const NDIS_WMI_OBJECT_TYPE_ENUM_ADAPTER: u32 = 4;
pub const NDIS_WMI_OBJECT_TYPE_EVENT: u32 = 3;
pub const NDIS_WMI_OBJECT_TYPE_METHOD: u32 = 2;
pub const NDIS_WMI_OBJECT_TYPE_OUTPUT_INFO: u32 = 5;
pub const NDIS_WMI_OBJECT_TYPE_SET: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_OFFLOAD {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_WMI_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_OUTPUT_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SupportedRevision: u8,
    pub DataOffset: u32,
}
pub const NDIS_WMI_OUTPUT_INFO_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
#[derive(Clone, Copy)]
pub struct NDIS_WMI_SET_HEADER {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PortNumber: super::ndisport::NDIS_PORT_NUMBER,
    pub NetLuid: super::ifdef::NET_LUID,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
impl Default for NDIS_WMI_SET_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDIS_WMI_SET_HEADER_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub SupportIPv4: u32,
    pub SupportIPv6: u32,
    pub SupportIPv6ExtensionHeaders: u32,
    pub SupportSack: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv4Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv6Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2,
    pub IPv6Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub TcpOptions: u32,
    pub IpOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETWORK_ADDRESS {
    pub AddressLength: u16,
    pub AddressType: u16,
    pub Address: [u8; 1],
}
impl Default for NETWORK_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETWORK_ADDRESS_IP {
    pub sin_port: u16,
    pub in_addr: u32,
    pub sin_zero: [u8; 8],
}
impl Default for NETWORK_ADDRESS_IP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETWORK_ADDRESS_IP6 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl Default for NETWORK_ADDRESS_IP6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETWORK_ADDRESS_IPX {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl Default for NETWORK_ADDRESS_IPX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETWORK_ADDRESS_LIST {
    pub AddressCount: i32,
    pub AddressType: u16,
    pub Address: [NETWORK_ADDRESS; 1],
}
impl Default for NETWORK_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Ndis802_11AuthModeAutoSwitch: NDIS_802_11_AUTHENTICATION_MODE = 2;
pub const Ndis802_11AuthModeMax: NDIS_802_11_AUTHENTICATION_MODE = 11;
pub const Ndis802_11AuthModeOpen: NDIS_802_11_AUTHENTICATION_MODE = 0;
pub const Ndis802_11AuthModeShared: NDIS_802_11_AUTHENTICATION_MODE = 1;
pub const Ndis802_11AuthModeWPA: NDIS_802_11_AUTHENTICATION_MODE = 3;
pub const Ndis802_11AuthModeWPA2: NDIS_802_11_AUTHENTICATION_MODE = 6;
pub const Ndis802_11AuthModeWPA2PSK: NDIS_802_11_AUTHENTICATION_MODE = 7;
pub const Ndis802_11AuthModeWPA3: NDIS_802_11_AUTHENTICATION_MODE = 8;
pub const Ndis802_11AuthModeWPA3Ent: NDIS_802_11_AUTHENTICATION_MODE = 10;
pub const Ndis802_11AuthModeWPA3Ent192: NDIS_802_11_AUTHENTICATION_MODE = 8;
pub const Ndis802_11AuthModeWPA3SAE: NDIS_802_11_AUTHENTICATION_MODE = 9;
pub const Ndis802_11AuthModeWPANone: NDIS_802_11_AUTHENTICATION_MODE = 5;
pub const Ndis802_11AuthModeWPAPSK: NDIS_802_11_AUTHENTICATION_MODE = 4;
pub const Ndis802_11AutoUnknown: NDIS_802_11_NETWORK_INFRASTRUCTURE = 2;
pub const Ndis802_11Automode: NDIS_802_11_NETWORK_TYPE = 4;
pub const Ndis802_11DS: NDIS_802_11_NETWORK_TYPE = 1;
pub const Ndis802_11Encryption1Enabled: NDIS_802_11_WEP_STATUS = 0;
pub const Ndis802_11Encryption1KeyAbsent: NDIS_802_11_WEP_STATUS = 2;
pub const Ndis802_11Encryption2Enabled: NDIS_802_11_WEP_STATUS = 4;
pub const Ndis802_11Encryption2KeyAbsent: NDIS_802_11_WEP_STATUS = 5;
pub const Ndis802_11Encryption3Enabled: NDIS_802_11_WEP_STATUS = 6;
pub const Ndis802_11Encryption3KeyAbsent: NDIS_802_11_WEP_STATUS = 7;
pub const Ndis802_11EncryptionDisabled: NDIS_802_11_WEP_STATUS = 1;
pub const Ndis802_11EncryptionNotSupported: NDIS_802_11_WEP_STATUS = 3;
pub const Ndis802_11FH: NDIS_802_11_NETWORK_TYPE = 0;
pub const Ndis802_11IBSS: NDIS_802_11_NETWORK_INFRASTRUCTURE = 0;
pub const Ndis802_11Infrastructure: NDIS_802_11_NETWORK_INFRASTRUCTURE = 1;
pub const Ndis802_11InfrastructureMax: NDIS_802_11_NETWORK_INFRASTRUCTURE = 3;
pub const Ndis802_11MediaStreamOff: NDIS_802_11_MEDIA_STREAM_MODE = 0;
pub const Ndis802_11MediaStreamOn: NDIS_802_11_MEDIA_STREAM_MODE = 1;
pub const Ndis802_11NetworkTypeMax: NDIS_802_11_NETWORK_TYPE = 5;
pub const Ndis802_11OFDM24: NDIS_802_11_NETWORK_TYPE = 3;
pub const Ndis802_11OFDM5: NDIS_802_11_NETWORK_TYPE = 2;
pub const Ndis802_11PowerModeCAM: NDIS_802_11_POWER_MODE = 0;
pub const Ndis802_11PowerModeFast_PSP: NDIS_802_11_POWER_MODE = 2;
pub const Ndis802_11PowerModeMAX_PSP: NDIS_802_11_POWER_MODE = 1;
pub const Ndis802_11PowerModeMax: NDIS_802_11_POWER_MODE = 3;
pub const Ndis802_11PrivFilter8021xWEP: NDIS_802_11_PRIVACY_FILTER = 1;
pub const Ndis802_11PrivFilterAcceptAll: NDIS_802_11_PRIVACY_FILTER = 0;
pub const Ndis802_11RadioStatusHardwareOff: NDIS_802_11_RADIO_STATUS = 1;
pub const Ndis802_11RadioStatusHardwareSoftwareOff: NDIS_802_11_RADIO_STATUS = 3;
pub const Ndis802_11RadioStatusMax: NDIS_802_11_RADIO_STATUS = 4;
pub const Ndis802_11RadioStatusOn: NDIS_802_11_RADIO_STATUS = 0;
pub const Ndis802_11RadioStatusSoftwareOff: NDIS_802_11_RADIO_STATUS = 2;
pub const Ndis802_11ReloadWEPKeys: NDIS_802_11_RELOAD_DEFAULTS = 0;
pub const Ndis802_11StatusTypeMax: NDIS_802_11_STATUS_TYPE = 3;
pub const Ndis802_11StatusType_Authentication: NDIS_802_11_STATUS_TYPE = 0;
pub const Ndis802_11StatusType_MediaStreamMode: NDIS_802_11_STATUS_TYPE = 1;
pub const Ndis802_11StatusType_PMKID_CandidateList: NDIS_802_11_STATUS_TYPE = 2;
pub const Ndis802_11WEPDisabled: NDIS_802_11_WEP_STATUS = 1;
pub const Ndis802_11WEPEnabled: NDIS_802_11_WEP_STATUS = 0;
pub const Ndis802_11WEPKeyAbsent: NDIS_802_11_WEP_STATUS = 2;
pub const Ndis802_11WEPNotSupported: NDIS_802_11_WEP_STATUS = 3;
pub const NdisDefinitelyNetworkChange: NDIS_NETWORK_CHANGE_TYPE = 2;
pub const NdisDeviceStateD0: NDIS_DEVICE_POWER_STATE = 1;
pub const NdisDeviceStateD1: NDIS_DEVICE_POWER_STATE = 2;
pub const NdisDeviceStateD2: NDIS_DEVICE_POWER_STATE = 3;
pub const NdisDeviceStateD3: NDIS_DEVICE_POWER_STATE = 4;
pub const NdisDeviceStateMaximum: NDIS_DEVICE_POWER_STATE = 5;
pub const NdisDeviceStateUnspecified: NDIS_DEVICE_POWER_STATE = 0;
pub const NdisFddiRingDetect: NDIS_FDDI_RING_MGT_STATE = 4;
pub const NdisFddiRingDirected: NDIS_FDDI_RING_MGT_STATE = 7;
pub const NdisFddiRingIsolated: NDIS_FDDI_RING_MGT_STATE = 1;
pub const NdisFddiRingNonOperational: NDIS_FDDI_RING_MGT_STATE = 2;
pub const NdisFddiRingNonOperationalDup: NDIS_FDDI_RING_MGT_STATE = 5;
pub const NdisFddiRingOperational: NDIS_FDDI_RING_MGT_STATE = 3;
pub const NdisFddiRingOperationalDup: NDIS_FDDI_RING_MGT_STATE = 6;
pub const NdisFddiRingTrace: NDIS_FDDI_RING_MGT_STATE = 8;
pub const NdisFddiStateActive: NDIS_FDDI_LCONNECTION_STATE = 9;
pub const NdisFddiStateBreak: NDIS_FDDI_LCONNECTION_STATE = 2;
pub const NdisFddiStateConnect: NDIS_FDDI_LCONNECTION_STATE = 4;
pub const NdisFddiStateJoin: NDIS_FDDI_LCONNECTION_STATE = 7;
pub const NdisFddiStateMaintenance: NDIS_FDDI_LCONNECTION_STATE = 10;
pub const NdisFddiStateNext: NDIS_FDDI_LCONNECTION_STATE = 5;
pub const NdisFddiStateOff: NDIS_FDDI_LCONNECTION_STATE = 1;
pub const NdisFddiStateSignal: NDIS_FDDI_LCONNECTION_STATE = 6;
pub const NdisFddiStateTrace: NDIS_FDDI_LCONNECTION_STATE = 3;
pub const NdisFddiStateVerify: NDIS_FDDI_LCONNECTION_STATE = 8;
pub const NdisFddiTypeCWrapA: NDIS_FDDI_ATTACHMENT_TYPE = 10;
pub const NdisFddiTypeCWrapB: NDIS_FDDI_ATTACHMENT_TYPE = 11;
pub const NdisFddiTypeCWrapS: NDIS_FDDI_ATTACHMENT_TYPE = 12;
pub const NdisFddiTypeIsolated: NDIS_FDDI_ATTACHMENT_TYPE = 1;
pub const NdisFddiTypeLocalA: NDIS_FDDI_ATTACHMENT_TYPE = 2;
pub const NdisFddiTypeLocalAB: NDIS_FDDI_ATTACHMENT_TYPE = 4;
pub const NdisFddiTypeLocalB: NDIS_FDDI_ATTACHMENT_TYPE = 3;
pub const NdisFddiTypeLocalS: NDIS_FDDI_ATTACHMENT_TYPE = 5;
pub const NdisFddiTypeThrough: NDIS_FDDI_ATTACHMENT_TYPE = 13;
pub const NdisFddiTypeWrapA: NDIS_FDDI_ATTACHMENT_TYPE = 6;
pub const NdisFddiTypeWrapAB: NDIS_FDDI_ATTACHMENT_TYPE = 8;
pub const NdisFddiTypeWrapB: NDIS_FDDI_ATTACHMENT_TYPE = 7;
pub const NdisFddiTypeWrapS: NDIS_FDDI_ATTACHMENT_TYPE = 9;
pub const NdisHardwareStatusClosing: NDIS_HARDWARE_STATUS = 3;
pub const NdisHardwareStatusInitializing: NDIS_HARDWARE_STATUS = 1;
pub const NdisHardwareStatusNotReady: NDIS_HARDWARE_STATUS = 4;
pub const NdisHardwareStatusReady: NDIS_HARDWARE_STATUS = 0;
pub const NdisHardwareStatusReset: NDIS_HARDWARE_STATUS = 2;
pub const NdisInterruptModerationDisabled: NDIS_INTERRUPT_MODERATION = 3;
pub const NdisInterruptModerationEnabled: NDIS_INTERRUPT_MODERATION = 2;
pub const NdisInterruptModerationNotSupported: NDIS_INTERRUPT_MODERATION = 1;
pub const NdisInterruptModerationUnknown: NDIS_INTERRUPT_MODERATION = 0;
pub const NdisMediaStateConnected: NDIS_MEDIA_STATE = 0;
pub const NdisMediaStateDisconnected: NDIS_MEDIA_STATE = 1;
pub const NdisMedium1394: NDIS_MEDIUM = 13;
pub const NdisMedium802_3: NDIS_MEDIUM = 0;
pub const NdisMedium802_5: NDIS_MEDIUM = 1;
pub const NdisMediumArcnet878_2: NDIS_MEDIUM = 7;
pub const NdisMediumArcnetRaw: NDIS_MEDIUM = 6;
pub const NdisMediumAtm: NDIS_MEDIUM = 8;
pub const NdisMediumBpc: NDIS_MEDIUM = 11;
pub const NdisMediumCoWan: NDIS_MEDIUM = 12;
pub const NdisMediumDix: NDIS_MEDIUM = 5;
pub const NdisMediumFddi: NDIS_MEDIUM = 2;
pub const NdisMediumIP: NDIS_MEDIUM = 19;
pub const NdisMediumInfiniBand: NDIS_MEDIUM = 14;
pub const NdisMediumIrda: NDIS_MEDIUM = 10;
pub const NdisMediumLocalTalk: NDIS_MEDIUM = 4;
pub const NdisMediumLoopback: NDIS_MEDIUM = 17;
pub const NdisMediumMax: NDIS_MEDIUM = 20;
pub const NdisMediumNative802_11: NDIS_MEDIUM = 16;
pub const NdisMediumTunnel: NDIS_MEDIUM = 15;
pub const NdisMediumWan: NDIS_MEDIUM = 3;
pub const NdisMediumWiMAX: NDIS_MEDIUM = 18;
pub const NdisMediumWirelessWan: NDIS_MEDIUM = 9;
pub const NdisNetworkChangeFromMediaConnect: NDIS_NETWORK_CHANGE_TYPE = 3;
pub const NdisNetworkChangeMax: NDIS_NETWORK_CHANGE_TYPE = 4;
pub const NdisPauseFunctionsReceiveOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = 2;
pub const NdisPauseFunctionsSendAndReceive: NDIS_SUPPORTED_PAUSE_FUNCTIONS = 3;
pub const NdisPauseFunctionsSendOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = 1;
pub const NdisPauseFunctionsUnknown: NDIS_SUPPORTED_PAUSE_FUNCTIONS = 4;
pub const NdisPauseFunctionsUnsupported: NDIS_SUPPORTED_PAUSE_FUNCTIONS = 0;
pub const NdisPhysicalMedium1394: NDIS_PHYSICAL_MEDIUM = 7;
pub const NdisPhysicalMedium802_3: NDIS_PHYSICAL_MEDIUM = 14;
pub const NdisPhysicalMedium802_5: NDIS_PHYSICAL_MEDIUM = 15;
pub const NdisPhysicalMediumBluetooth: NDIS_PHYSICAL_MEDIUM = 10;
pub const NdisPhysicalMediumCableModem: NDIS_PHYSICAL_MEDIUM = 2;
pub const NdisPhysicalMediumDSL: NDIS_PHYSICAL_MEDIUM = 5;
pub const NdisPhysicalMediumFibreChannel: NDIS_PHYSICAL_MEDIUM = 6;
pub const NdisPhysicalMediumInfiniband: NDIS_PHYSICAL_MEDIUM = 11;
pub const NdisPhysicalMediumIrda: NDIS_PHYSICAL_MEDIUM = 16;
pub const NdisPhysicalMediumMax: NDIS_PHYSICAL_MEDIUM = 21;
pub const NdisPhysicalMediumNative802_11: NDIS_PHYSICAL_MEDIUM = 9;
pub const NdisPhysicalMediumNative802_15_4: NDIS_PHYSICAL_MEDIUM = 20;
pub const NdisPhysicalMediumOther: NDIS_PHYSICAL_MEDIUM = 19;
pub const NdisPhysicalMediumPhoneLine: NDIS_PHYSICAL_MEDIUM = 3;
pub const NdisPhysicalMediumPowerLine: NDIS_PHYSICAL_MEDIUM = 4;
pub const NdisPhysicalMediumUWB: NDIS_PHYSICAL_MEDIUM = 13;
pub const NdisPhysicalMediumUnspecified: NDIS_PHYSICAL_MEDIUM = 0;
pub const NdisPhysicalMediumWiMax: NDIS_PHYSICAL_MEDIUM = 12;
pub const NdisPhysicalMediumWiredCoWan: NDIS_PHYSICAL_MEDIUM = 18;
pub const NdisPhysicalMediumWiredWAN: NDIS_PHYSICAL_MEDIUM = 17;
pub const NdisPhysicalMediumWirelessLan: NDIS_PHYSICAL_MEDIUM = 1;
pub const NdisPhysicalMediumWirelessWan: NDIS_PHYSICAL_MEDIUM = 8;
pub const NdisPortAuthorizationUnknown: NDIS_PORT_AUTHORIZATION_STATE = 0;
pub const NdisPortAuthorized: NDIS_PORT_AUTHORIZATION_STATE = 1;
pub const NdisPortControlStateControlled: NDIS_PORT_CONTROL_STATE = 1;
pub const NdisPortControlStateUncontrolled: NDIS_PORT_CONTROL_STATE = 2;
pub const NdisPortControlStateUnknown: NDIS_PORT_CONTROL_STATE = 0;
pub const NdisPortReauthorizing: NDIS_PORT_AUTHORIZATION_STATE = 3;
pub const NdisPortUnauthorized: NDIS_PORT_AUTHORIZATION_STATE = 2;
pub const NdisPossibleNetworkChange: NDIS_NETWORK_CHANGE_TYPE = 1;
pub const NdisProcessorVendorAuthenticAMD: NDIS_PROCESSOR_VENDOR = 2;
pub const NdisProcessorVendorGenuinIntel: NDIS_PROCESSOR_VENDOR = 1;
pub const NdisProcessorVendorGenuineIntel: NDIS_PROCESSOR_VENDOR = 1;
pub const NdisProcessorVendorUnknown: NDIS_PROCESSOR_VENDOR = 0;
pub const NdisRingStateClosed: NDIS_802_5_RING_STATE = 2;
pub const NdisRingStateClosing: NDIS_802_5_RING_STATE = 4;
pub const NdisRingStateOpenFailure: NDIS_802_5_RING_STATE = 5;
pub const NdisRingStateOpened: NDIS_802_5_RING_STATE = 1;
pub const NdisRingStateOpening: NDIS_802_5_RING_STATE = 3;
pub const NdisRingStateRingFailure: NDIS_802_5_RING_STATE = 6;
pub const NdisWanErrorControl: NDIS_WAN_QUALITY = 1;
pub const NdisWanHeaderEthernet: NDIS_WAN_HEADER_FORMAT = 1;
pub const NdisWanHeaderNative: NDIS_WAN_HEADER_FORMAT = 0;
pub const NdisWanMediumAgileVPN: NDIS_WAN_MEDIUM_SUBTYPE = 14;
pub const NdisWanMediumAtm: NDIS_WAN_MEDIUM_SUBTYPE = 5;
pub const NdisWanMediumFrameRelay: NDIS_WAN_MEDIUM_SUBTYPE = 4;
pub const NdisWanMediumGre: NDIS_WAN_MEDIUM_SUBTYPE = 15;
pub const NdisWanMediumHub: NDIS_WAN_MEDIUM_SUBTYPE = 0;
pub const NdisWanMediumIrda: NDIS_WAN_MEDIUM_SUBTYPE = 10;
pub const NdisWanMediumIsdn: NDIS_WAN_MEDIUM_SUBTYPE = 2;
pub const NdisWanMediumL2TP: NDIS_WAN_MEDIUM_SUBTYPE = 9;
pub const NdisWanMediumPPTP: NDIS_WAN_MEDIUM_SUBTYPE = 8;
pub const NdisWanMediumParallel: NDIS_WAN_MEDIUM_SUBTYPE = 11;
pub const NdisWanMediumPppoe: NDIS_WAN_MEDIUM_SUBTYPE = 12;
pub const NdisWanMediumSSTP: NDIS_WAN_MEDIUM_SUBTYPE = 13;
pub const NdisWanMediumSW56K: NDIS_WAN_MEDIUM_SUBTYPE = 7;
pub const NdisWanMediumSerial: NDIS_WAN_MEDIUM_SUBTYPE = 3;
pub const NdisWanMediumSonet: NDIS_WAN_MEDIUM_SUBTYPE = 6;
pub const NdisWanMediumSubTypeMax: NDIS_WAN_MEDIUM_SUBTYPE = 16;
pub const NdisWanMediumX_25: NDIS_WAN_MEDIUM_SUBTYPE = 1;
pub const NdisWanRaw: NDIS_WAN_QUALITY = 0;
pub const NdisWanReliable: NDIS_WAN_QUALITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OFFLOAD_ALGO_INFO {
    pub algoIdentifier: u32,
    pub algoKeylen: u32,
    pub algoRounds: u32,
}
pub type OFFLOAD_CONF_ALGO = i32;
pub const OFFLOAD_INBOUND_SA: u32 = 1;
pub type OFFLOAD_INTEGRITY_ALGO = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct OFFLOAD_IPSEC_ADD_SA {
    pub SrcAddr: IPAddr,
    pub SrcMask: IPMask,
    pub DestAddr: IPAddr,
    pub DestMask: IPMask,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DestPort: u16,
    pub SrcTunnelAddr: IPAddr,
    pub DestTunnelAddr: IPAddr,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::winnt::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for OFFLOAD_IPSEC_ADD_SA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct OFFLOAD_IPSEC_ADD_UDPESP_SA {
    pub SrcAddr: IPAddr,
    pub SrcMask: IPMask,
    pub DstAddr: IPAddr,
    pub DstMask: IPMask,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DstPort: u16,
    pub SrcTunnelAddr: IPAddr,
    pub DstTunnelAddr: IPAddr,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::winnt::HANDLE,
    pub EncapTypeEntry: OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY,
    pub EncapTypeEntryOffldHandle: super::winnt::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFFLOAD_IPSEC_CONF_3_DES: OFFLOAD_CONF_ALGO = 3;
pub const OFFLOAD_IPSEC_CONF_DES: OFFLOAD_CONF_ALGO = 1;
pub const OFFLOAD_IPSEC_CONF_MAX: OFFLOAD_CONF_ALGO = 4;
pub const OFFLOAD_IPSEC_CONF_NONE: OFFLOAD_CONF_ALGO = 0;
pub const OFFLOAD_IPSEC_CONF_RESERVED: OFFLOAD_CONF_ALGO = 2;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct OFFLOAD_IPSEC_DELETE_SA {
    pub OffloadHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for OFFLOAD_IPSEC_DELETE_SA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    pub OffloadHandle: super::winnt::HANDLE,
    pub EncapTypeEntryOffldHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFFLOAD_IPSEC_INTEGRITY_MAX: OFFLOAD_INTEGRITY_ALGO = 3;
pub const OFFLOAD_IPSEC_INTEGRITY_MD5: OFFLOAD_INTEGRITY_ALGO = 1;
pub const OFFLOAD_IPSEC_INTEGRITY_NONE: OFFLOAD_INTEGRITY_ALGO = 0;
pub const OFFLOAD_IPSEC_INTEGRITY_SHA: OFFLOAD_INTEGRITY_ALGO = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    pub UdpEncapType: UDP_ENCAP_TYPE,
    pub DstEncapPort: u16,
}
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_IKE: UDP_ENCAP_TYPE = 0;
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_OTHER: UDP_ENCAP_TYPE = 1;
pub const OFFLOAD_MAX_SAS: u32 = 3;
pub type OFFLOAD_OPERATION_E = i32;
pub const OFFLOAD_OUTBOUND_SA: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OFFLOAD_SECURITY_ASSOCIATION {
    pub Operation: OFFLOAD_OPERATION_E,
    pub SPI: SPI_TYPE,
    pub IntegrityAlgo: OFFLOAD_ALGO_INFO,
    pub ConfAlgo: OFFLOAD_ALGO_INFO,
    pub Reserved: OFFLOAD_ALGO_INFO,
}
pub const OID_1394_LOCAL_NODE_INFO: u32 = 201392385;
pub const OID_1394_VC_INFO: u32 = 201392386;
pub const OID_802_11_ADD_KEY: u32 = 218169629;
pub const OID_802_11_ADD_WEP: u32 = 218169619;
pub const OID_802_11_ASSOCIATION_INFORMATION: u32 = 218169631;
pub const OID_802_11_AUTHENTICATION_MODE: u32 = 218169624;
pub const OID_802_11_BSSID: u32 = 218169601;
pub const OID_802_11_BSSID_LIST: u32 = 218169879;
pub const OID_802_11_BSSID_LIST_SCAN: u32 = 218169626;
pub const OID_802_11_CAPABILITY: u32 = 218169634;
pub const OID_802_11_CONFIGURATION: u32 = 218169873;
pub const OID_802_11_DESIRED_RATES: u32 = 218169872;
pub const OID_802_11_DISASSOCIATE: u32 = 218169621;
pub const OID_802_11_ENCRYPTION_STATUS: u32 = 218169627;
pub const OID_802_11_FRAGMENTATION_THRESHOLD: u32 = 218169865;
pub const OID_802_11_INFRASTRUCTURE_MODE: u32 = 218169608;
pub const OID_802_11_MEDIA_STREAM_MODE: u32 = 218169633;
pub const OID_802_11_NETWORK_TYPES_SUPPORTED: u32 = 218169859;
pub const OID_802_11_NETWORK_TYPE_IN_USE: u32 = 218169860;
pub const OID_802_11_NON_BCAST_SSID_LIST: u32 = 218169636;
pub const OID_802_11_NUMBER_OF_ANTENNAS: u32 = 218169867;
pub const OID_802_11_PMKID: u32 = 218169635;
pub const OID_802_11_POWER_MODE: u32 = 218169878;
pub const OID_802_11_PRIVACY_FILTER: u32 = 218169625;
pub const OID_802_11_RADIO_STATUS: u32 = 218169637;
pub const OID_802_11_RELOAD_DEFAULTS: u32 = 218169628;
pub const OID_802_11_REMOVE_KEY: u32 = 218169630;
pub const OID_802_11_REMOVE_WEP: u32 = 218169620;
pub const OID_802_11_RSSI: u32 = 218169862;
pub const OID_802_11_RSSI_TRIGGER: u32 = 218169863;
pub const OID_802_11_RTS_THRESHOLD: u32 = 218169866;
pub const OID_802_11_RX_ANTENNA_SELECTED: u32 = 218169868;
pub const OID_802_11_SSID: u32 = 218169602;
pub const OID_802_11_STATISTICS: u32 = 218235410;
pub const OID_802_11_SUPPORTED_RATES: u32 = 218169870;
pub const OID_802_11_TEST: u32 = 218169632;
pub const OID_802_11_TX_ANTENNA_SELECTED: u32 = 218169869;
pub const OID_802_11_TX_POWER_LEVEL: u32 = 218169861;
pub const OID_802_11_WEP_STATUS: u32 = 218169627;
pub const OID_802_3_ADD_MULTICAST_ADDRESS: u32 = 16843272;
pub const OID_802_3_CURRENT_ADDRESS: u32 = 16843010;
pub const OID_802_3_DELETE_MULTICAST_ADDRESS: u32 = 16843273;
pub const OID_802_3_MAC_OPTIONS: u32 = 16843013;
pub const OID_802_3_MAXIMUM_LIST_SIZE: u32 = 16843012;
pub const OID_802_3_MULTICAST_LIST: u32 = 16843011;
pub const OID_802_3_PERMANENT_ADDRESS: u32 = 16843009;
pub const OID_802_3_RCV_ERROR_ALIGNMENT: u32 = 16908545;
pub const OID_802_3_RCV_OVERRUN: u32 = 16908803;
pub const OID_802_3_XMIT_DEFERRED: u32 = 16908801;
pub const OID_802_3_XMIT_HEARTBEAT_FAILURE: u32 = 16908805;
pub const OID_802_3_XMIT_LATE_COLLISIONS: u32 = 16908807;
pub const OID_802_3_XMIT_MAX_COLLISIONS: u32 = 16908802;
pub const OID_802_3_XMIT_MORE_COLLISIONS: u32 = 16908547;
pub const OID_802_3_XMIT_ONE_COLLISION: u32 = 16908546;
pub const OID_802_3_XMIT_TIMES_CRS_LOST: u32 = 16908806;
pub const OID_802_3_XMIT_UNDERRUN: u32 = 16908804;
pub const OID_802_5_ABORT_DELIMETERS: u32 = 33686019;
pub const OID_802_5_AC_ERRORS: u32 = 33686018;
pub const OID_802_5_BURST_ERRORS: u32 = 33686017;
pub const OID_802_5_CURRENT_ADDRESS: u32 = 33620226;
pub const OID_802_5_CURRENT_FUNCTIONAL: u32 = 33620227;
pub const OID_802_5_CURRENT_GROUP: u32 = 33620228;
pub const OID_802_5_CURRENT_RING_STATE: u32 = 33620231;
pub const OID_802_5_CURRENT_RING_STATUS: u32 = 33620230;
pub const OID_802_5_FRAME_COPIED_ERRORS: u32 = 33686020;
pub const OID_802_5_FREQUENCY_ERRORS: u32 = 33686021;
pub const OID_802_5_INTERNAL_ERRORS: u32 = 33686023;
pub const OID_802_5_LAST_OPEN_STATUS: u32 = 33620229;
pub const OID_802_5_LINE_ERRORS: u32 = 33685761;
pub const OID_802_5_LOST_FRAMES: u32 = 33685762;
pub const OID_802_5_PERMANENT_ADDRESS: u32 = 33620225;
pub const OID_802_5_TOKEN_ERRORS: u32 = 33686022;
pub const OID_ARCNET_CURRENT_ADDRESS: u32 = 100729090;
pub const OID_ARCNET_PERMANENT_ADDRESS: u32 = 100729089;
pub const OID_ARCNET_RECONFIGURATIONS: u32 = 100794881;
pub const OID_ATM_ACQUIRE_ACCESS_NET_RESOURCES: u32 = 134283779;
pub const OID_ATM_ALIGNMENT_REQUIRED: u32 = 134283784;
pub const OID_ATM_ASSIGNED_VPI: u32 = 134283778;
pub const OID_ATM_CALL_ALERTING: u32 = 134283788;
pub const OID_ATM_CALL_NOTIFY: u32 = 134283790;
pub const OID_ATM_CALL_PROCEEDING: u32 = 134283787;
pub const OID_ATM_CELLS_HEC_ERROR: u32 = 134349314;
pub const OID_ATM_DIGITAL_BROADCAST_VPIVCI: u32 = 134283782;
pub const OID_ATM_GET_NEAREST_FLOW: u32 = 134283783;
pub const OID_ATM_HW_CURRENT_ADDRESS: u32 = 134283524;
pub const OID_ATM_ILMI_VPIVCI: u32 = 134283781;
pub const OID_ATM_LECS_ADDRESS: u32 = 134283785;
pub const OID_ATM_MAX_AAL0_PACKET_SIZE: u32 = 134283528;
pub const OID_ATM_MAX_AAL1_PACKET_SIZE: u32 = 134283529;
pub const OID_ATM_MAX_AAL34_PACKET_SIZE: u32 = 134283530;
pub const OID_ATM_MAX_AAL5_PACKET_SIZE: u32 = 134283531;
pub const OID_ATM_MAX_ACTIVE_VCI_BITS: u32 = 134283526;
pub const OID_ATM_MAX_ACTIVE_VCS: u32 = 134283525;
pub const OID_ATM_MAX_ACTIVE_VPI_BITS: u32 = 134283527;
pub const OID_ATM_MY_IP_NM_ADDRESS: u32 = 134283791;
pub const OID_ATM_PARTY_ALERTING: u32 = 134283789;
pub const OID_ATM_RCV_CELLS_DROPPED: u32 = 134349059;
pub const OID_ATM_RCV_CELLS_OK: u32 = 134349057;
pub const OID_ATM_RCV_INVALID_VPI_VCI: u32 = 134349313;
pub const OID_ATM_RCV_REASSEMBLY_ERROR: u32 = 134349315;
pub const OID_ATM_RELEASE_ACCESS_NET_RESOURCES: u32 = 134283780;
pub const OID_ATM_SERVICE_ADDRESS: u32 = 134283786;
pub const OID_ATM_SIGNALING_VPIVCI: u32 = 134283777;
pub const OID_ATM_SUPPORTED_AAL_TYPES: u32 = 134283523;
pub const OID_ATM_SUPPORTED_SERVICE_CATEGORY: u32 = 134283522;
pub const OID_ATM_SUPPORTED_VC_RATES: u32 = 134283521;
pub const OID_ATM_XMIT_CELLS_OK: u32 = 134349058;
pub const OID_CO_ADDRESS_CHANGE: u32 = 4261412871;
pub const OID_CO_ADD_ADDRESS: u32 = 4261412868;
pub const OID_CO_ADD_PVC: u32 = 4261412865;
pub const OID_CO_AF_CLOSE: u32 = 4261412874;
pub const OID_CO_DELETE_ADDRESS: u32 = 4261412869;
pub const OID_CO_DELETE_PVC: u32 = 4261412866;
pub const OID_CO_GET_ADDRESSES: u32 = 4261412870;
pub const OID_CO_GET_CALL_INFORMATION: u32 = 4261412867;
pub const OID_CO_SIGNALING_DISABLED: u32 = 4261412873;
pub const OID_CO_SIGNALING_ENABLED: u32 = 4261412872;
pub const OID_CO_TAPI_ADDRESS_CAPS: u32 = 4261416963;
pub const OID_CO_TAPI_CM_CAPS: u32 = 4261416961;
pub const OID_CO_TAPI_DONT_REPORT_DIGITS: u32 = 4261416969;
pub const OID_CO_TAPI_GET_CALL_DIAGNOSTICS: u32 = 4261416967;
pub const OID_CO_TAPI_LINE_CAPS: u32 = 4261416962;
pub const OID_CO_TAPI_REPORT_DIGITS: u32 = 4261416968;
pub const OID_CO_TAPI_TRANSLATE_NDIS_CALLPARAMS: u32 = 4261416965;
pub const OID_CO_TAPI_TRANSLATE_TAPI_CALLPARAMS: u32 = 4261416964;
pub const OID_CO_TAPI_TRANSLATE_TAPI_SAP: u32 = 4261416966;
pub const OID_FDDI_ATTACHMENT_TYPE: u32 = 50462977;
pub const OID_FDDI_DOWNSTREAM_NODE_LONG: u32 = 50462979;
pub const OID_FDDI_FRAMES_LOST: u32 = 50462981;
pub const OID_FDDI_FRAME_ERRORS: u32 = 50462980;
pub const OID_FDDI_IF_ADMIN_STATUS: u32 = 50528894;
pub const OID_FDDI_IF_DESCR: u32 = 50528889;
pub const OID_FDDI_IF_IN_DISCARDS: u32 = 50528900;
pub const OID_FDDI_IF_IN_ERRORS: u32 = 50528901;
pub const OID_FDDI_IF_IN_NUCAST_PKTS: u32 = 50528899;
pub const OID_FDDI_IF_IN_OCTETS: u32 = 50528897;
pub const OID_FDDI_IF_IN_UCAST_PKTS: u32 = 50528898;
pub const OID_FDDI_IF_IN_UNKNOWN_PROTOS: u32 = 50528902;
pub const OID_FDDI_IF_LAST_CHANGE: u32 = 50528896;
pub const OID_FDDI_IF_MTU: u32 = 50528891;
pub const OID_FDDI_IF_OPER_STATUS: u32 = 50528895;
pub const OID_FDDI_IF_OUT_DISCARDS: u32 = 50528906;
pub const OID_FDDI_IF_OUT_ERRORS: u32 = 50528907;
pub const OID_FDDI_IF_OUT_NUCAST_PKTS: u32 = 50528905;
pub const OID_FDDI_IF_OUT_OCTETS: u32 = 50528903;
pub const OID_FDDI_IF_OUT_QLEN: u32 = 50528908;
pub const OID_FDDI_IF_OUT_UCAST_PKTS: u32 = 50528904;
pub const OID_FDDI_IF_PHYS_ADDRESS: u32 = 50528893;
pub const OID_FDDI_IF_SPECIFIC: u32 = 50528909;
pub const OID_FDDI_IF_SPEED: u32 = 50528892;
pub const OID_FDDI_IF_TYPE: u32 = 50528890;
pub const OID_FDDI_LCONNECTION_STATE: u32 = 50462985;
pub const OID_FDDI_LCT_FAILURES: u32 = 50462983;
pub const OID_FDDI_LEM_REJECTS: u32 = 50462984;
pub const OID_FDDI_LONG_CURRENT_ADDR: u32 = 50397442;
pub const OID_FDDI_LONG_MAX_LIST_SIZE: u32 = 50397444;
pub const OID_FDDI_LONG_MULTICAST_LIST: u32 = 50397443;
pub const OID_FDDI_LONG_PERMANENT_ADDR: u32 = 50397441;
pub const OID_FDDI_MAC_AVAILABLE_PATHS: u32 = 50528803;
pub const OID_FDDI_MAC_BRIDGE_FUNCTIONS: u32 = 50528800;
pub const OID_FDDI_MAC_COPIED_CT: u32 = 50528828;
pub const OID_FDDI_MAC_CURRENT_PATH: u32 = 50528804;
pub const OID_FDDI_MAC_DA_FLAG: u32 = 50528842;
pub const OID_FDDI_MAC_DOWNSTREAM_NBR: u32 = 50528806;
pub const OID_FDDI_MAC_DOWNSTREAM_PORT_TYPE: u32 = 50528811;
pub const OID_FDDI_MAC_DUP_ADDRESS_TEST: u32 = 50528809;
pub const OID_FDDI_MAC_ERROR_CT: u32 = 50528831;
pub const OID_FDDI_MAC_FRAME_CT: u32 = 50528827;
pub const OID_FDDI_MAC_FRAME_ERROR_FLAG: u32 = 50528844;
pub const OID_FDDI_MAC_FRAME_ERROR_RATIO: u32 = 50528838;
pub const OID_FDDI_MAC_FRAME_ERROR_THRESHOLD: u32 = 50528837;
pub const OID_FDDI_MAC_FRAME_STATUS_FUNCTIONS: u32 = 50528799;
pub const OID_FDDI_MAC_HARDWARE_PRESENT: u32 = 50528847;
pub const OID_FDDI_MAC_INDEX: u32 = 50528812;
pub const OID_FDDI_MAC_LATE_CT: u32 = 50528835;
pub const OID_FDDI_MAC_LONG_GRP_ADDRESS: u32 = 50528814;
pub const OID_FDDI_MAC_LOST_CT: u32 = 50528832;
pub const OID_FDDI_MAC_MA_UNITDATA_AVAILABLE: u32 = 50528846;
pub const OID_FDDI_MAC_MA_UNITDATA_ENABLE: u32 = 50528848;
pub const OID_FDDI_MAC_NOT_COPIED_CT: u32 = 50528834;
pub const OID_FDDI_MAC_NOT_COPIED_FLAG: u32 = 50528845;
pub const OID_FDDI_MAC_NOT_COPIED_RATIO: u32 = 50528840;
pub const OID_FDDI_MAC_NOT_COPIED_THRESHOLD: u32 = 50528839;
pub const OID_FDDI_MAC_OLD_DOWNSTREAM_NBR: u32 = 50528808;
pub const OID_FDDI_MAC_OLD_UPSTREAM_NBR: u32 = 50528807;
pub const OID_FDDI_MAC_REQUESTED_PATHS: u32 = 50528810;
pub const OID_FDDI_MAC_RING_OP_CT: u32 = 50528836;
pub const OID_FDDI_MAC_RMT_STATE: u32 = 50528841;
pub const OID_FDDI_MAC_SHORT_GRP_ADDRESS: u32 = 50528815;
pub const OID_FDDI_MAC_SMT_ADDRESS: u32 = 50528813;
pub const OID_FDDI_MAC_TOKEN_CT: u32 = 50528830;
pub const OID_FDDI_MAC_TRANSMIT_CT: u32 = 50528829;
pub const OID_FDDI_MAC_TVX_CAPABILITY: u32 = 50528802;
pub const OID_FDDI_MAC_TVX_EXPIRED_CT: u32 = 50528833;
pub const OID_FDDI_MAC_TVX_VALUE: u32 = 50528819;
pub const OID_FDDI_MAC_T_MAX: u32 = 50528818;
pub const OID_FDDI_MAC_T_MAX_CAPABILITY: u32 = 50528801;
pub const OID_FDDI_MAC_T_NEG: u32 = 50528817;
pub const OID_FDDI_MAC_T_PRI0: u32 = 50528820;
pub const OID_FDDI_MAC_T_PRI1: u32 = 50528821;
pub const OID_FDDI_MAC_T_PRI2: u32 = 50528822;
pub const OID_FDDI_MAC_T_PRI3: u32 = 50528823;
pub const OID_FDDI_MAC_T_PRI4: u32 = 50528824;
pub const OID_FDDI_MAC_T_PRI5: u32 = 50528825;
pub const OID_FDDI_MAC_T_PRI6: u32 = 50528826;
pub const OID_FDDI_MAC_T_REQ: u32 = 50528816;
pub const OID_FDDI_MAC_UNDA_FLAG: u32 = 50528843;
pub const OID_FDDI_MAC_UPSTREAM_NBR: u32 = 50528805;
pub const OID_FDDI_PATH_CONFIGURATION: u32 = 50528854;
pub const OID_FDDI_PATH_INDEX: u32 = 50528849;
pub const OID_FDDI_PATH_MAX_T_REQ: u32 = 50528859;
pub const OID_FDDI_PATH_RING_LATENCY: u32 = 50528850;
pub const OID_FDDI_PATH_SBA_AVAILABLE: u32 = 50528856;
pub const OID_FDDI_PATH_SBA_OVERHEAD: u32 = 50528853;
pub const OID_FDDI_PATH_SBA_PAYLOAD: u32 = 50528852;
pub const OID_FDDI_PATH_TRACE_STATUS: u32 = 50528851;
pub const OID_FDDI_PATH_TVX_LOWER_BOUND: u32 = 50528857;
pub const OID_FDDI_PATH_T_MAX_LOWER_BOUND: u32 = 50528858;
pub const OID_FDDI_PATH_T_R_MODE: u32 = 50528855;
pub const OID_FDDI_PORT_ACTION: u32 = 50528888;
pub const OID_FDDI_PORT_AVAILABLE_PATHS: u32 = 50528867;
pub const OID_FDDI_PORT_BS_FLAG: u32 = 50528873;
pub const OID_FDDI_PORT_CONNECTION_CAPABILITIES: u32 = 50528870;
pub const OID_FDDI_PORT_CONNECTION_POLICIES: u32 = 50528862;
pub const OID_FDDI_PORT_CONNNECT_STATE: u32 = 50528882;
pub const OID_FDDI_PORT_CURRENT_PATH: u32 = 50528864;
pub const OID_FDDI_PORT_EB_ERROR_CT: u32 = 50528875;
pub const OID_FDDI_PORT_HARDWARE_PRESENT: u32 = 50528886;
pub const OID_FDDI_PORT_INDEX: u32 = 50528871;
pub const OID_FDDI_PORT_LCT_FAIL_CT: u32 = 50528876;
pub const OID_FDDI_PORT_LEM_CT: u32 = 50528879;
pub const OID_FDDI_PORT_LEM_REJECT_CT: u32 = 50528878;
pub const OID_FDDI_PORT_LER_ALARM: u32 = 50528881;
pub const OID_FDDI_PORT_LER_CUTOFF: u32 = 50528880;
pub const OID_FDDI_PORT_LER_ESTIMATE: u32 = 50528877;
pub const OID_FDDI_PORT_LER_FLAG: u32 = 50528885;
pub const OID_FDDI_PORT_MAC_INDICATED: u32 = 50528863;
pub const OID_FDDI_PORT_MAC_LOOP_TIME: u32 = 50528868;
pub const OID_FDDI_PORT_MAC_PLACEMENT: u32 = 50528866;
pub const OID_FDDI_PORT_MAINT_LS: u32 = 50528872;
pub const OID_FDDI_PORT_MY_TYPE: u32 = 50528860;
pub const OID_FDDI_PORT_NEIGHBOR_TYPE: u32 = 50528861;
pub const OID_FDDI_PORT_PCM_STATE: u32 = 50528883;
pub const OID_FDDI_PORT_PC_LS: u32 = 50528874;
pub const OID_FDDI_PORT_PC_WITHHOLD: u32 = 50528884;
pub const OID_FDDI_PORT_PMD_CLASS: u32 = 50528869;
pub const OID_FDDI_PORT_REQUESTED_PATHS: u32 = 50528865;
pub const OID_FDDI_RING_MGT_STATE: u32 = 50462982;
pub const OID_FDDI_SHORT_CURRENT_ADDR: u32 = 50397446;
pub const OID_FDDI_SHORT_MAX_LIST_SIZE: u32 = 50397448;
pub const OID_FDDI_SHORT_MULTICAST_LIST: u32 = 50397447;
pub const OID_FDDI_SHORT_PERMANENT_ADDR: u32 = 50397445;
pub const OID_FDDI_SMT_AVAILABLE_PATHS: u32 = 50528779;
pub const OID_FDDI_SMT_BYPASS_PRESENT: u32 = 50528788;
pub const OID_FDDI_SMT_CF_STATE: u32 = 50528790;
pub const OID_FDDI_SMT_CONFIG_CAPABILITIES: u32 = 50528780;
pub const OID_FDDI_SMT_CONFIG_POLICY: u32 = 50528781;
pub const OID_FDDI_SMT_CONNECTION_POLICY: u32 = 50528782;
pub const OID_FDDI_SMT_ECM_STATE: u32 = 50528789;
pub const OID_FDDI_SMT_HI_VERSION_ID: u32 = 50528771;
pub const OID_FDDI_SMT_HOLD_STATE: u32 = 50528791;
pub const OID_FDDI_SMT_LAST_SET_STATION_ID: u32 = 50528798;
pub const OID_FDDI_SMT_LO_VERSION_ID: u32 = 50528772;
pub const OID_FDDI_SMT_MAC_CT: u32 = 50528776;
pub const OID_FDDI_SMT_MAC_INDEXES: u32 = 50528787;
pub const OID_FDDI_SMT_MANUFACTURER_DATA: u32 = 50528773;
pub const OID_FDDI_SMT_MASTER_CT: u32 = 50528778;
pub const OID_FDDI_SMT_MIB_VERSION_ID: u32 = 50528775;
pub const OID_FDDI_SMT_MSG_TIME_STAMP: u32 = 50528795;
pub const OID_FDDI_SMT_NON_MASTER_CT: u32 = 50528777;
pub const OID_FDDI_SMT_OP_VERSION_ID: u32 = 50528770;
pub const OID_FDDI_SMT_PEER_WRAP_FLAG: u32 = 50528794;
pub const OID_FDDI_SMT_PORT_INDEXES: u32 = 50528786;
pub const OID_FDDI_SMT_REMOTE_DISCONNECT_FLAG: u32 = 50528792;
pub const OID_FDDI_SMT_SET_COUNT: u32 = 50528797;
pub const OID_FDDI_SMT_STATION_ACTION: u32 = 50528887;
pub const OID_FDDI_SMT_STATION_ID: u32 = 50528769;
pub const OID_FDDI_SMT_STATION_STATUS: u32 = 50528793;
pub const OID_FDDI_SMT_STAT_RPT_POLICY: u32 = 50528784;
pub const OID_FDDI_SMT_TRACE_MAX_EXPIRATION: u32 = 50528785;
pub const OID_FDDI_SMT_TRANSITION_TIME_STAMP: u32 = 50528796;
pub const OID_FDDI_SMT_T_NOTIFY: u32 = 50528783;
pub const OID_FDDI_SMT_USER_DATA: u32 = 50528774;
pub const OID_FDDI_UPSTREAM_NODE_LONG: u32 = 50462978;
pub const OID_FFP_ADAPTER_STATS: u32 = 4227990033;
pub const OID_FFP_CONTROL: u32 = 4227924498;
pub const OID_FFP_DATA: u32 = 4227924500;
pub const OID_FFP_DRIVER_STATS: u32 = 4227990032;
pub const OID_FFP_FLUSH: u32 = 4227924497;
pub const OID_FFP_PARAMS: u32 = 4227924499;
pub const OID_FFP_SUPPORT: u32 = 4227924496;
pub const OID_GEN_ADMIN_STATUS: u32 = 66184;
pub const OID_GEN_ALIAS: u32 = 66185;
pub const OID_GEN_BROADCAST_BYTES_RCV: u32 = 131595;
pub const OID_GEN_BROADCAST_BYTES_XMIT: u32 = 131589;
pub const OID_GEN_BROADCAST_FRAMES_RCV: u32 = 131596;
pub const OID_GEN_BROADCAST_FRAMES_XMIT: u32 = 131590;
pub const OID_GEN_BYTES_RCV: u32 = 131609;
pub const OID_GEN_BYTES_XMIT: u32 = 131610;
pub const OID_GEN_CO_BYTES_RCV: u32 = 131591;
pub const OID_GEN_CO_BYTES_XMIT: u32 = 131585;
pub const OID_GEN_CO_BYTES_XMIT_OUTSTANDING: u32 = 131617;
pub const OID_GEN_CO_DEVICE_PROFILE: u32 = 131602;
pub const OID_GEN_CO_DRIVER_VERSION: u32 = 65808;
pub const OID_GEN_CO_GET_NETCARD_TIME: u32 = 131600;
pub const OID_GEN_CO_GET_TIME_CAPS: u32 = 131599;
pub const OID_GEN_CO_HARDWARE_STATUS: u32 = 65794;
pub const OID_GEN_CO_LINK_SPEED: u32 = 65799;
pub const OID_GEN_CO_MAC_OPTIONS: u32 = 65811;
pub const OID_GEN_CO_MEDIA_CONNECT_STATUS: u32 = 65812;
pub const OID_GEN_CO_MEDIA_IN_USE: u32 = 65796;
pub const OID_GEN_CO_MEDIA_SUPPORTED: u32 = 65795;
pub const OID_GEN_CO_MINIMUM_LINK_SPEED: u32 = 131360;
pub const OID_GEN_CO_NETCARD_LOAD: u32 = 131601;
pub const OID_GEN_CO_PROTOCOL_OPTIONS: u32 = 65810;
pub const OID_GEN_CO_RCV_CRC_ERROR: u32 = 131597;
pub const OID_GEN_CO_RCV_PDUS_ERROR: u32 = 131332;
pub const OID_GEN_CO_RCV_PDUS_NO_BUFFER: u32 = 131333;
pub const OID_GEN_CO_RCV_PDUS_OK: u32 = 131330;
pub const OID_GEN_CO_SUPPORTED_GUIDS: u32 = 65815;
pub const OID_GEN_CO_SUPPORTED_LIST: u32 = 65793;
pub const OID_GEN_CO_TRANSMIT_QUEUE_LENGTH: u32 = 131598;
pub const OID_GEN_CO_VENDOR_DESCRIPTION: u32 = 65805;
pub const OID_GEN_CO_VENDOR_DRIVER_VERSION: u32 = 65814;
pub const OID_GEN_CO_VENDOR_ID: u32 = 65804;
pub const OID_GEN_CO_XMIT_PDUS_ERROR: u32 = 131331;
pub const OID_GEN_CO_XMIT_PDUS_OK: u32 = 131329;
pub const OID_GEN_CURRENT_LOOKAHEAD: u32 = 65807;
pub const OID_GEN_CURRENT_PACKET_FILTER: u32 = 65806;
pub const OID_GEN_DEVICE_PROFILE: u32 = 131602;
pub const OID_GEN_DIRECTED_BYTES_RCV: u32 = 131591;
pub const OID_GEN_DIRECTED_BYTES_XMIT: u32 = 131585;
pub const OID_GEN_DIRECTED_FRAMES_RCV: u32 = 131592;
pub const OID_GEN_DIRECTED_FRAMES_XMIT: u32 = 131586;
pub const OID_GEN_DISCONTINUITY_TIME: u32 = 66178;
pub const OID_GEN_DRIVER_VERSION: u32 = 65808;
pub const OID_GEN_ENUMERATE_PORTS: u32 = 66061;
pub const OID_GEN_FRIENDLY_NAME: u32 = 131606;
pub const OID_GEN_GET_NETCARD_TIME: u32 = 131600;
pub const OID_GEN_GET_TIME_CAPS: u32 = 131599;
pub const OID_GEN_HARDWARE_STATUS: u32 = 65794;
pub const OID_GEN_INIT_TIME_MS: u32 = 131603;
pub const OID_GEN_INTERFACE_INFO: u32 = 66183;
pub const OID_GEN_INTERRUPT_MODERATION: u32 = 66057;
pub const OID_GEN_IP_OPER_STATUS: u32 = 66189;
pub const OID_GEN_ISOLATION_PARAMETERS: u32 = 66304;
pub const OID_GEN_LAST_CHANGE: u32 = 66177;
pub const OID_GEN_LINK_PARAMETERS: u32 = 66056;
pub const OID_GEN_LINK_SPEED: u32 = 65799;
pub const OID_GEN_LINK_SPEED_EX: u32 = 66187;
pub const OID_GEN_LINK_STATE: u32 = 66055;
pub const OID_GEN_MACHINE_NAME: u32 = 66074;
pub const OID_GEN_MAC_ADDRESS: u32 = 66053;
pub const OID_GEN_MAC_OPTIONS: u32 = 65811;
pub const OID_GEN_MAXIMUM_FRAME_SIZE: u32 = 65798;
pub const OID_GEN_MAXIMUM_LOOKAHEAD: u32 = 65797;
pub const OID_GEN_MAXIMUM_SEND_PACKETS: u32 = 65813;
pub const OID_GEN_MAXIMUM_TOTAL_SIZE: u32 = 65809;
pub const OID_GEN_MAX_LINK_SPEED: u32 = 66054;
pub const OID_GEN_MEDIA_CAPABILITIES: u32 = 66049;
pub const OID_GEN_MEDIA_CONNECT_STATUS: u32 = 65812;
pub const OID_GEN_MEDIA_CONNECT_STATUS_EX: u32 = 66186;
pub const OID_GEN_MEDIA_DUPLEX_STATE: u32 = 66188;
pub const OID_GEN_MEDIA_IN_USE: u32 = 65796;
pub const OID_GEN_MEDIA_SENSE_COUNTS: u32 = 131605;
pub const OID_GEN_MEDIA_SUPPORTED: u32 = 65795;
pub const OID_GEN_MINIPORT_RESTART_ATTRIBUTES: u32 = 66077;
pub const OID_GEN_MULTICAST_BYTES_RCV: u32 = 131593;
pub const OID_GEN_MULTICAST_BYTES_XMIT: u32 = 131587;
pub const OID_GEN_MULTICAST_FRAMES_RCV: u32 = 131594;
pub const OID_GEN_MULTICAST_FRAMES_XMIT: u32 = 131588;
pub const OID_GEN_NDIS_RESERVED_1: u32 = 131607;
pub const OID_GEN_NDIS_RESERVED_2: u32 = 131608;
pub const OID_GEN_NDIS_RESERVED_3: u32 = 66058;
pub const OID_GEN_NDIS_RESERVED_4: u32 = 66059;
pub const OID_GEN_NDIS_RESERVED_5: u32 = 66060;
pub const OID_GEN_NDIS_RESERVED_6: u32 = 66066;
pub const OID_GEN_NDIS_RESERVED_7: u32 = 131614;
pub const OID_GEN_NETCARD_LOAD: u32 = 131601;
pub const OID_GEN_NETWORK_LAYER_ADDRESSES: u32 = 65816;
pub const OID_GEN_OPERATIONAL_STATUS: u32 = 66179;
pub const OID_GEN_PCI_DEVICE_CUSTOM_PROPERTIES: u32 = 66065;
pub const OID_GEN_PHYSICAL_MEDIUM: u32 = 66050;
pub const OID_GEN_PHYSICAL_MEDIUM_EX: u32 = 66067;
pub const OID_GEN_PORT_AUTHENTICATION_PARAMETERS: u32 = 66063;
pub const OID_GEN_PORT_STATE: u32 = 66062;
pub const OID_GEN_PROMISCUOUS_MODE: u32 = 66176;
pub const OID_GEN_PROTOCOL_OPTIONS: u32 = 65810;
pub const OID_GEN_RCV_CRC_ERROR: u32 = 131597;
pub const OID_GEN_RCV_DISCARDS: u32 = 131611;
pub const OID_GEN_RCV_ERROR: u32 = 131332;
pub const OID_GEN_RCV_LINK_SPEED: u32 = 66181;
pub const OID_GEN_RCV_NO_BUFFER: u32 = 131333;
pub const OID_GEN_RCV_OK: u32 = 131330;
pub const OID_GEN_RECEIVE_BLOCK_SIZE: u32 = 65803;
pub const OID_GEN_RECEIVE_BUFFER_SPACE: u32 = 65801;
pub const OID_GEN_RECEIVE_HASH: u32 = 66079;
pub const OID_GEN_RECEIVE_SCALE_CAPABILITIES: u32 = 66051;
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS: u32 = 66052;
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS_V2: u32 = 66068;
pub const OID_GEN_RESET_COUNTS: u32 = 131604;
pub const OID_GEN_RNDIS_CONFIG_PARAMETER: u32 = 66075;
pub const OID_GEN_RSS_SET_INDIRECTION_TABLE_ENTRIES: u32 = 66240;
pub const OID_GEN_STATISTICS: u32 = 131334;
pub const OID_GEN_SUPPORTED_GUIDS: u32 = 65815;
pub const OID_GEN_SUPPORTED_LIST: u32 = 65793;
pub const OID_GEN_TIMEOUT_DPC_REQUEST_CAPABILITIES: u32 = 66064;
pub const OID_GEN_TRANSMIT_BLOCK_SIZE: u32 = 65802;
pub const OID_GEN_TRANSMIT_BUFFER_SPACE: u32 = 65800;
pub const OID_GEN_TRANSMIT_QUEUE_LENGTH: u32 = 131598;
pub const OID_GEN_TRANSPORT_HEADER_OFFSET: u32 = 65817;
pub const OID_GEN_UNKNOWN_PROTOS: u32 = 66182;
pub const OID_GEN_VENDOR_DESCRIPTION: u32 = 65805;
pub const OID_GEN_VENDOR_DRIVER_VERSION: u32 = 65814;
pub const OID_GEN_VENDOR_ID: u32 = 65804;
pub const OID_GEN_VLAN_ID: u32 = 66076;
pub const OID_GEN_XMIT_DISCARDS: u32 = 131612;
pub const OID_GEN_XMIT_ERROR: u32 = 131331;
pub const OID_GEN_XMIT_LINK_SPEED: u32 = 66180;
pub const OID_GEN_XMIT_OK: u32 = 131329;
pub const OID_IP4_OFFLOAD_STATS: u32 = 4227924489;
pub const OID_IP6_OFFLOAD_STATS: u32 = 4227924490;
pub const OID_IRDA_EXTRA_RCV_BOFS: u32 = 167838208;
pub const OID_IRDA_LINK_SPEED: u32 = 167837955;
pub const OID_IRDA_MAX_RECEIVE_WINDOW_SIZE: u32 = 167838212;
pub const OID_IRDA_MAX_SEND_WINDOW_SIZE: u32 = 167838213;
pub const OID_IRDA_MAX_UNICAST_LIST_SIZE: u32 = 167838211;
pub const OID_IRDA_MEDIA_BUSY: u32 = 167837956;
pub const OID_IRDA_RATE_SNIFF: u32 = 167838209;
pub const OID_IRDA_RECEIVING: u32 = 167837952;
pub const OID_IRDA_RESERVED1: u32 = 167838218;
pub const OID_IRDA_RESERVED2: u32 = 167838223;
pub const OID_IRDA_SUPPORTED_SPEEDS: u32 = 167837954;
pub const OID_IRDA_TURNAROUND_TIME: u32 = 167837953;
pub const OID_IRDA_UNICAST_LIST: u32 = 167838210;
pub const OID_LTALK_COLLISIONS: u32 = 84017666;
pub const OID_LTALK_CURRENT_NODE_ID: u32 = 83951874;
pub const OID_LTALK_DEFERS: u32 = 84017667;
pub const OID_LTALK_FCS_ERRORS: u32 = 84017670;
pub const OID_LTALK_IN_BROADCASTS: u32 = 84017409;
pub const OID_LTALK_IN_LENGTH_ERRORS: u32 = 84017410;
pub const OID_LTALK_NO_DATA_ERRORS: u32 = 84017668;
pub const OID_LTALK_OUT_NO_HANDLERS: u32 = 84017665;
pub const OID_LTALK_RANDOM_CTS_ERRORS: u32 = 84017669;
pub const OID_NIC_SWITCH_ALLOCATE_VF: u32 = 66117;
pub const OID_NIC_SWITCH_CREATE_SWITCH: u32 = 66103;
pub const OID_NIC_SWITCH_CREATE_VPORT: u32 = 66113;
pub const OID_NIC_SWITCH_CURRENT_CAPABILITIES: u32 = 66095;
pub const OID_NIC_SWITCH_DELETE_SWITCH: u32 = 66105;
pub const OID_NIC_SWITCH_DELETE_VPORT: u32 = 66116;
pub const OID_NIC_SWITCH_ENUM_SWITCHES: u32 = 66112;
pub const OID_NIC_SWITCH_ENUM_VFS: u32 = 66120;
pub const OID_NIC_SWITCH_ENUM_VPORTS: u32 = 66115;
pub const OID_NIC_SWITCH_FREE_VF: u32 = 66118;
pub const OID_NIC_SWITCH_HARDWARE_CAPABILITIES: u32 = 66094;
pub const OID_NIC_SWITCH_PARAMETERS: u32 = 66104;
pub const OID_NIC_SWITCH_VF_PARAMETERS: u32 = 66119;
pub const OID_NIC_SWITCH_VPORT_PARAMETERS: u32 = 66114;
pub const OID_OFFLOAD_ENCAPSULATION: u32 = 16843018;
pub const OID_PM_ADD_PROTOCOL_OFFLOAD: u32 = 4244701453;
pub const OID_PM_ADD_WOL_PATTERN: u32 = 4244701450;
pub const OID_PM_CURRENT_CAPABILITIES: u32 = 4244701447;
pub const OID_PM_GET_PROTOCOL_OFFLOAD: u32 = 4244701454;
pub const OID_PM_HARDWARE_CAPABILITIES: u32 = 4244701448;
pub const OID_PM_PARAMETERS: u32 = 4244701449;
pub const OID_PM_PROTOCOL_OFFLOAD_LIST: u32 = 4244701456;
pub const OID_PM_REMOVE_PROTOCOL_OFFLOAD: u32 = 4244701455;
pub const OID_PM_REMOVE_WOL_PATTERN: u32 = 4244701451;
pub const OID_PM_RESERVED_1: u32 = 4244701457;
pub const OID_PM_WOL_PATTERN_LIST: u32 = 4244701452;
pub const OID_PNP_ADD_WAKE_UP_PATTERN: u32 = 4244701443;
pub const OID_PNP_CAPABILITIES: u32 = 4244701440;
pub const OID_PNP_ENABLE_WAKE_UP: u32 = 4244701446;
pub const OID_PNP_QUERY_POWER: u32 = 4244701442;
pub const OID_PNP_REMOVE_WAKE_UP_PATTERN: u32 = 4244701444;
pub const OID_PNP_SET_POWER: u32 = 4244701441;
pub const OID_PNP_WAKE_UP_ERROR: u32 = 4244767233;
pub const OID_PNP_WAKE_UP_OK: u32 = 4244767232;
pub const OID_PNP_WAKE_UP_PATTERN_LIST: u32 = 4244701445;
pub const OID_QOS_RESERVED1: u32 = 4211147008;
pub const OID_QOS_RESERVED10: u32 = 4211147017;
pub const OID_QOS_RESERVED11: u32 = 4211147018;
pub const OID_QOS_RESERVED12: u32 = 4211147019;
pub const OID_QOS_RESERVED13: u32 = 4211147020;
pub const OID_QOS_RESERVED14: u32 = 4211147021;
pub const OID_QOS_RESERVED15: u32 = 4211147022;
pub const OID_QOS_RESERVED16: u32 = 4211147023;
pub const OID_QOS_RESERVED17: u32 = 4211147024;
pub const OID_QOS_RESERVED18: u32 = 4211147025;
pub const OID_QOS_RESERVED19: u32 = 4211147026;
pub const OID_QOS_RESERVED2: u32 = 4211147009;
pub const OID_QOS_RESERVED20: u32 = 4211147027;
pub const OID_QOS_RESERVED3: u32 = 4211147010;
pub const OID_QOS_RESERVED4: u32 = 4211147011;
pub const OID_QOS_RESERVED5: u32 = 4211147012;
pub const OID_QOS_RESERVED6: u32 = 4211147013;
pub const OID_QOS_RESERVED7: u32 = 4211147014;
pub const OID_QOS_RESERVED8: u32 = 4211147015;
pub const OID_QOS_RESERVED9: u32 = 4211147016;
pub const OID_RECEIVE_FILTER_ALLOCATE_QUEUE: u32 = 66083;
pub const OID_RECEIVE_FILTER_CLEAR_FILTER: u32 = 66088;
pub const OID_RECEIVE_FILTER_CURRENT_CAPABILITIES: u32 = 66093;
pub const OID_RECEIVE_FILTER_ENUM_FILTERS: u32 = 66089;
pub const OID_RECEIVE_FILTER_ENUM_QUEUES: u32 = 66085;
pub const OID_RECEIVE_FILTER_FREE_QUEUE: u32 = 66084;
pub const OID_RECEIVE_FILTER_GLOBAL_PARAMETERS: u32 = 66082;
pub const OID_RECEIVE_FILTER_HARDWARE_CAPABILITIES: u32 = 66081;
pub const OID_RECEIVE_FILTER_PARAMETERS: u32 = 66090;
pub const OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE: u32 = 66091;
pub const OID_RECEIVE_FILTER_QUEUE_PARAMETERS: u32 = 66086;
pub const OID_RECEIVE_FILTER_SET_FILTER: u32 = 66087;
pub const OID_SRIOV_BAR_RESOURCES: u32 = 66137;
pub const OID_SRIOV_CONFIG_STATE: u32 = 66145;
pub const OID_SRIOV_CURRENT_CAPABILITIES: u32 = 66128;
pub const OID_SRIOV_HARDWARE_CAPABILITIES: u32 = 66121;
pub const OID_SRIOV_PF_LUID: u32 = 66144;
pub const OID_SRIOV_PROBED_BARS: u32 = 66136;
pub const OID_SRIOV_READ_VF_CONFIG_BLOCK: u32 = 66131;
pub const OID_SRIOV_READ_VF_CONFIG_SPACE: u32 = 66129;
pub const OID_SRIOV_RESET_VF: u32 = 66133;
pub const OID_SRIOV_SET_VF_POWER_STATE: u32 = 66134;
pub const OID_SRIOV_VF_INVALIDATE_CONFIG_BLOCK: u32 = 66153;
pub const OID_SRIOV_VF_SERIAL_NUMBER: u32 = 66146;
pub const OID_SRIOV_VF_VENDOR_DEVICE_ID: u32 = 66135;
pub const OID_SRIOV_WRITE_VF_CONFIG_BLOCK: u32 = 66132;
pub const OID_SRIOV_WRITE_VF_CONFIG_SPACE: u32 = 66130;
pub const OID_SWITCH_FEATURE_STATUS_QUERY: u32 = 66151;
pub const OID_SWITCH_NIC_ARRAY: u32 = 66167;
pub const OID_SWITCH_NIC_CONNECT: u32 = 66171;
pub const OID_SWITCH_NIC_CREATE: u32 = 66170;
pub const OID_SWITCH_NIC_DELETE: u32 = 66173;
pub const OID_SWITCH_NIC_DISCONNECT: u32 = 66172;
pub const OID_SWITCH_NIC_REQUEST: u32 = 66160;
pub const OID_SWITCH_NIC_RESTORE: u32 = 66194;
pub const OID_SWITCH_NIC_RESTORE_COMPLETE: u32 = 66195;
pub const OID_SWITCH_NIC_SAVE: u32 = 66192;
pub const OID_SWITCH_NIC_SAVE_COMPLETE: u32 = 66193;
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_FINISHED: u32 = 66202;
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_STARTED: u32 = 66201;
pub const OID_SWITCH_NIC_UPDATED: u32 = 66196;
pub const OID_SWITCH_PARAMETERS: u32 = 66165;
pub const OID_SWITCH_PORT_ARRAY: u32 = 66166;
pub const OID_SWITCH_PORT_CREATE: u32 = 66168;
pub const OID_SWITCH_PORT_DELETE: u32 = 66169;
pub const OID_SWITCH_PORT_FEATURE_STATUS_QUERY: u32 = 66174;
pub const OID_SWITCH_PORT_PROPERTY_ADD: u32 = 66161;
pub const OID_SWITCH_PORT_PROPERTY_DELETE: u32 = 66163;
pub const OID_SWITCH_PORT_PROPERTY_ENUM: u32 = 66164;
pub const OID_SWITCH_PORT_PROPERTY_UPDATE: u32 = 66162;
pub const OID_SWITCH_PORT_TEARDOWN: u32 = 66175;
pub const OID_SWITCH_PORT_UPDATED: u32 = 66197;
pub const OID_SWITCH_PROPERTY_ADD: u32 = 66147;
pub const OID_SWITCH_PROPERTY_DELETE: u32 = 66149;
pub const OID_SWITCH_PROPERTY_ENUM: u32 = 66150;
pub const OID_SWITCH_PROPERTY_UPDATE: u32 = 66148;
pub const OID_TAPI_ACCEPT: u32 = 117637377;
pub const OID_TAPI_ANSWER: u32 = 117637378;
pub const OID_TAPI_CLOSE: u32 = 117637379;
pub const OID_TAPI_CLOSE_CALL: u32 = 117637380;
pub const OID_TAPI_CONDITIONAL_MEDIA_DETECTION: u32 = 117637381;
pub const OID_TAPI_CONFIG_DIALOG: u32 = 117637382;
pub const OID_TAPI_DEV_SPECIFIC: u32 = 117637383;
pub const OID_TAPI_DIAL: u32 = 117637384;
pub const OID_TAPI_DROP: u32 = 117637385;
pub const OID_TAPI_GATHER_DIGITS: u32 = 117637411;
pub const OID_TAPI_GET_ADDRESS_CAPS: u32 = 117637386;
pub const OID_TAPI_GET_ADDRESS_ID: u32 = 117637387;
pub const OID_TAPI_GET_ADDRESS_STATUS: u32 = 117637388;
pub const OID_TAPI_GET_CALL_ADDRESS_ID: u32 = 117637389;
pub const OID_TAPI_GET_CALL_INFO: u32 = 117637390;
pub const OID_TAPI_GET_CALL_STATUS: u32 = 117637391;
pub const OID_TAPI_GET_DEV_CAPS: u32 = 117637392;
pub const OID_TAPI_GET_DEV_CONFIG: u32 = 117637393;
pub const OID_TAPI_GET_EXTENSION_ID: u32 = 117637394;
pub const OID_TAPI_GET_ID: u32 = 117637395;
pub const OID_TAPI_GET_LINE_DEV_STATUS: u32 = 117637396;
pub const OID_TAPI_MAKE_CALL: u32 = 117637397;
pub const OID_TAPI_MONITOR_DIGITS: u32 = 117637412;
pub const OID_TAPI_NEGOTIATE_EXT_VERSION: u32 = 117637398;
pub const OID_TAPI_OPEN: u32 = 117637399;
pub const OID_TAPI_PROVIDER_INITIALIZE: u32 = 117637400;
pub const OID_TAPI_PROVIDER_SHUTDOWN: u32 = 117637401;
pub const OID_TAPI_SECURE_CALL: u32 = 117637402;
pub const OID_TAPI_SELECT_EXT_VERSION: u32 = 117637403;
pub const OID_TAPI_SEND_USER_USER_INFO: u32 = 117637404;
pub const OID_TAPI_SET_APP_SPECIFIC: u32 = 117637405;
pub const OID_TAPI_SET_CALL_PARAMS: u32 = 117637406;
pub const OID_TAPI_SET_DEFAULT_MEDIA_DETECTION: u32 = 117637407;
pub const OID_TAPI_SET_DEV_CONFIG: u32 = 117637408;
pub const OID_TAPI_SET_MEDIA_MODE: u32 = 117637409;
pub const OID_TAPI_SET_STATUS_MESSAGES: u32 = 117637410;
pub const OID_TCP4_OFFLOAD_STATS: u32 = 4227924487;
pub const OID_TCP6_OFFLOAD_STATS: u32 = 4227924488;
pub const OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: u32 = 4227924494;
pub const OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924495;
pub const OID_TCP_CONNECTION_OFFLOAD_PARAMETERS: u32 = 4228055553;
pub const OID_TCP_OFFLOAD_CURRENT_CONFIG: u32 = 4227924491;
pub const OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924493;
pub const OID_TCP_OFFLOAD_PARAMETERS: u32 = 4227924492;
pub const OID_TCP_RSC_STATISTICS: u32 = 131613;
pub const OID_TCP_SAN_SUPPORT: u32 = 4227924484;
pub const OID_TCP_TASK_IPSEC_ADD_SA: u32 = 4227924482;
pub const OID_TCP_TASK_IPSEC_ADD_UDPESP_SA: u32 = 4227924485;
pub const OID_TCP_TASK_IPSEC_DELETE_SA: u32 = 4227924483;
pub const OID_TCP_TASK_IPSEC_DELETE_UDPESP_SA: u32 = 4227924486;
pub const OID_TCP_TASK_OFFLOAD: u32 = 4227924481;
pub const OID_TIMESTAMP_CAPABILITY: u32 = 10485761;
pub const OID_TIMESTAMP_CURRENT_CONFIG: u32 = 10485762;
pub const OID_TIMESTAMP_GET_CROSSTIMESTAMP: u32 = 10485763;
pub const OID_TUNNEL_INTERFACE_RELEASE_OID: u32 = 251724039;
pub const OID_TUNNEL_INTERFACE_SET_OID: u32 = 251724038;
pub const OID_VLAN_RESERVED1: u32 = 66097;
pub const OID_VLAN_RESERVED2: u32 = 66098;
pub const OID_VLAN_RESERVED3: u32 = 66099;
pub const OID_VLAN_RESERVED4: u32 = 66100;
pub const OID_WAN_CO_GET_COMP_INFO: u32 = 67175040;
pub const OID_WAN_CO_GET_INFO: u32 = 67174784;
pub const OID_WAN_CO_GET_LINK_INFO: u32 = 67174786;
pub const OID_WAN_CO_GET_STATS_INFO: u32 = 67175042;
pub const OID_WAN_CO_SET_COMP_INFO: u32 = 67175041;
pub const OID_WAN_CO_SET_LINK_INFO: u32 = 67174785;
pub const OID_WAN_CURRENT_ADDRESS: u32 = 67174658;
pub const OID_WAN_GET_BRIDGE_INFO: u32 = 67174922;
pub const OID_WAN_GET_COMP_INFO: u32 = 67174924;
pub const OID_WAN_GET_INFO: u32 = 67174663;
pub const OID_WAN_GET_LINK_INFO: u32 = 67174665;
pub const OID_WAN_GET_STATS_INFO: u32 = 67174926;
pub const OID_WAN_HEADER_FORMAT: u32 = 67174662;
pub const OID_WAN_LINE_COUNT: u32 = 67174666;
pub const OID_WAN_MEDIUM_SUBTYPE: u32 = 67174661;
pub const OID_WAN_PERMANENT_ADDRESS: u32 = 67174657;
pub const OID_WAN_PROTOCOL_CAPS: u32 = 67174667;
pub const OID_WAN_PROTOCOL_TYPE: u32 = 67174660;
pub const OID_WAN_QUALITY_OF_SERVICE: u32 = 67174659;
pub const OID_WAN_SET_BRIDGE_INFO: u32 = 67174923;
pub const OID_WAN_SET_COMP_INFO: u32 = 67174925;
pub const OID_WAN_SET_LINK_INFO: u32 = 67174664;
pub const OID_WWAN_AUTH_CHALLENGE: u32 = 234946837;
pub const OID_WWAN_BASE_STATIONS_INFO: u32 = 234946888;
pub const OID_WWAN_CONNECT: u32 = 234946828;
pub const OID_WWAN_CREATE_MAC: u32 = 234946854;
pub const OID_WWAN_DELETE_MAC: u32 = 234946855;
pub const OID_WWAN_DEVICE_BINDINGS: u32 = 234946865;
pub const OID_WWAN_DEVICE_CAPS: u32 = 234946817;
pub const OID_WWAN_DEVICE_CAPS_EX: u32 = 234946862;
pub const OID_WWAN_DEVICE_RESET: u32 = 234946887;
pub const OID_WWAN_DEVICE_SERVICE_COMMAND: u32 = 234946840;
pub const OID_WWAN_DEVICE_SERVICE_SESSION: u32 = 234946851;
pub const OID_WWAN_DEVICE_SERVICE_SESSION_WRITE: u32 = 234946852;
pub const OID_WWAN_DRIVER_CAPS: u32 = 234946816;
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICES: u32 = 234946838;
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICE_COMMANDS: u32 = 234946850;
pub const OID_WWAN_HOME_PROVIDER: u32 = 234946822;
pub const OID_WWAN_IMS_VOICE_STATE: u32 = 234946867;
pub const OID_WWAN_LOCATION_STATE: u32 = 234946869;
pub const OID_WWAN_LTE_ATTACH_CONFIG: u32 = 234946882;
pub const OID_WWAN_LTE_ATTACH_STATUS: u32 = 234946883;
pub const OID_WWAN_MBIM_VERSION: u32 = 234946860;
pub const OID_WWAN_MODEM_CONFIG_INFO: u32 = 234946884;
pub const OID_WWAN_MODEM_LOGGING_CONFIG: u32 = 234946891;
pub const OID_WWAN_MPDP: u32 = 234946889;
pub const OID_WWAN_NETWORK_BLACKLIST: u32 = 234946881;
pub const OID_WWAN_NETWORK_IDLE_HINT: u32 = 234946871;
pub const OID_WWAN_NETWORK_PARAMS: u32 = 234946893;
pub const OID_WWAN_NITZ: u32 = 234946870;
pub const OID_WWAN_PACKET_SERVICE: u32 = 234946826;
pub const OID_WWAN_PCO: u32 = 234946885;
pub const OID_WWAN_PIN: u32 = 234946820;
pub const OID_WWAN_PIN_EX: u32 = 234946849;
pub const OID_WWAN_PIN_EX2: u32 = 234946859;
pub const OID_WWAN_PIN_LIST: u32 = 234946821;
pub const OID_WWAN_PREFERRED_MULTICARRIER_PROVIDERS: u32 = 234946853;
pub const OID_WWAN_PREFERRED_PROVIDERS: u32 = 234946823;
pub const OID_WWAN_PRESHUTDOWN: u32 = 234946872;
pub const OID_WWAN_PROVISIONED_CONTEXTS: u32 = 234946829;
pub const OID_WWAN_PS_MEDIA_CONFIG: u32 = 234946878;
pub const OID_WWAN_RADIO_STATE: u32 = 234946819;
pub const OID_WWAN_READY_INFO: u32 = 234946818;
pub const OID_WWAN_REGISTER_PARAMS: u32 = 234946892;
pub const OID_WWAN_REGISTER_STATE: u32 = 234946825;
pub const OID_WWAN_REGISTER_STATE_EX: u32 = 234946866;
pub const OID_WWAN_SAR_CONFIG: u32 = 234946879;
pub const OID_WWAN_SAR_TRANSMISSION_STATUS: u32 = 234946880;
pub const OID_WWAN_SERVICE_ACTIVATION: u32 = 234946830;
pub const OID_WWAN_SIGNAL_STATE: u32 = 234946827;
pub const OID_WWAN_SIGNAL_STATE_EX: u32 = 234946868;
pub const OID_WWAN_SLOT_INFO_STATUS: u32 = 234946864;
pub const OID_WWAN_SMS_CONFIGURATION: u32 = 234946831;
pub const OID_WWAN_SMS_DELETE: u32 = 234946834;
pub const OID_WWAN_SMS_READ: u32 = 234946832;
pub const OID_WWAN_SMS_SEND: u32 = 234946833;
pub const OID_WWAN_SMS_STATUS: u32 = 234946835;
pub const OID_WWAN_SUBSCRIBE_DEVICE_SERVICE_EVENTS: u32 = 234946839;
pub const OID_WWAN_SYS_CAPS: u32 = 234946861;
pub const OID_WWAN_SYS_SLOTMAPPINGS: u32 = 234946863;
pub const OID_WWAN_UE_POLICY: u32 = 234946894;
pub const OID_WWAN_UICC_ACCESS_BINARY: u32 = 234946857;
pub const OID_WWAN_UICC_ACCESS_RECORD: u32 = 234946858;
pub const OID_WWAN_UICC_APDU: u32 = 234946876;
pub const OID_WWAN_UICC_APP_LIST: u32 = 234946890;
pub const OID_WWAN_UICC_ATR: u32 = 234946873;
pub const OID_WWAN_UICC_CLOSE_CHANNEL: u32 = 234946875;
pub const OID_WWAN_UICC_FILE_STATUS: u32 = 234946856;
pub const OID_WWAN_UICC_OPEN_CHANNEL: u32 = 234946874;
pub const OID_WWAN_UICC_RESET: u32 = 234946886;
pub const OID_WWAN_UICC_TERMINAL_CAPABILITY: u32 = 234946877;
pub const OID_WWAN_USSD: u32 = 234946841;
pub const OID_WWAN_VENDOR_SPECIFIC: u32 = 234946836;
pub const OID_WWAN_VISIBLE_PROVIDERS: u32 = 234946824;
pub const OID_XBOX_ACC_RESERVED0: u32 = 4194304000;
pub type PBSSID_INFO = *mut BSSID_INFO;
pub type PGEN_GET_NETCARD_TIME = *mut GEN_GET_NETCARD_TIME;
pub type PGEN_GET_TIME_CAPS = *mut GEN_GET_TIME_CAPS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PMKID_CANDIDATE {
    pub BSSID: NDIS_802_11_MAC_ADDRESS,
    pub Flags: u32,
}
impl Default for PMKID_CANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PNDIS_802_11_AI_REQFI = *mut NDIS_802_11_AI_REQFI;
pub type PNDIS_802_11_AI_RESFI = *mut NDIS_802_11_AI_RESFI;
pub type PNDIS_802_11_ASSOCIATION_INFORMATION = *mut NDIS_802_11_ASSOCIATION_INFORMATION;
pub type PNDIS_802_11_AUTHENTICATION_ENCRYPTION = *mut NDIS_802_11_AUTHENTICATION_ENCRYPTION;
pub type PNDIS_802_11_AUTHENTICATION_EVENT = *mut NDIS_802_11_AUTHENTICATION_EVENT;
pub type PNDIS_802_11_AUTHENTICATION_MODE = *mut NDIS_802_11_AUTHENTICATION_MODE;
pub type PNDIS_802_11_AUTHENTICATION_REQUEST = *mut NDIS_802_11_AUTHENTICATION_REQUEST;
pub type PNDIS_802_11_BSSID_LIST = *mut NDIS_802_11_BSSID_LIST;
pub type PNDIS_802_11_BSSID_LIST_EX = *mut NDIS_802_11_BSSID_LIST_EX;
pub type PNDIS_802_11_CAPABILITY = *mut NDIS_802_11_CAPABILITY;
pub type PNDIS_802_11_CONFIGURATION = *mut NDIS_802_11_CONFIGURATION;
pub type PNDIS_802_11_CONFIGURATION_FH = *mut NDIS_802_11_CONFIGURATION_FH;
pub type PNDIS_802_11_ENCRYPTION_STATUS = *mut NDIS_802_11_WEP_STATUS;
pub type PNDIS_802_11_FIXED_IEs = *mut NDIS_802_11_FIXED_IEs;
pub type PNDIS_802_11_KEY = *mut NDIS_802_11_KEY;
pub type PNDIS_802_11_MEDIA_STREAM_MODE = *mut NDIS_802_11_MEDIA_STREAM_MODE;
pub type PNDIS_802_11_NETWORK_INFRASTRUCTURE = *mut NDIS_802_11_NETWORK_INFRASTRUCTURE;
pub type PNDIS_802_11_NETWORK_TYPE = *mut NDIS_802_11_NETWORK_TYPE;
pub type PNDIS_802_11_NETWORK_TYPE_LIST = *mut NDIS_802_11_NETWORK_TYPE_LIST;
pub type PNDIS_802_11_NON_BCAST_SSID_LIST = *mut NDIS_802_11_NON_BCAST_SSID_LIST;
pub type PNDIS_802_11_PMKID = *mut NDIS_802_11_PMKID;
pub type PNDIS_802_11_PMKID_CANDIDATE_LIST = *mut NDIS_802_11_PMKID_CANDIDATE_LIST;
pub type PNDIS_802_11_POWER_MODE = *mut NDIS_802_11_POWER_MODE;
pub type PNDIS_802_11_PRIVACY_FILTER = *mut NDIS_802_11_PRIVACY_FILTER;
pub type PNDIS_802_11_RADIO_STATUS = *mut NDIS_802_11_RADIO_STATUS;
pub type PNDIS_802_11_RELOAD_DEFAULTS = *mut NDIS_802_11_RELOAD_DEFAULTS;
pub type PNDIS_802_11_REMOVE_KEY = *mut NDIS_802_11_REMOVE_KEY;
pub type PNDIS_802_11_SSID = *mut NDIS_802_11_SSID;
pub type PNDIS_802_11_STATISTICS = *mut NDIS_802_11_STATISTICS;
pub type PNDIS_802_11_STATUS_INDICATION = *mut NDIS_802_11_STATUS_INDICATION;
pub type PNDIS_802_11_STATUS_TYPE = *mut NDIS_802_11_STATUS_TYPE;
pub type PNDIS_802_11_TEST = *mut NDIS_802_11_TEST;
pub type PNDIS_802_11_VARIABLE_IEs = *mut NDIS_802_11_VARIABLE_IEs;
pub type PNDIS_802_11_WEP = *mut NDIS_802_11_WEP;
pub type PNDIS_802_11_WEP_STATUS = *mut NDIS_802_11_WEP_STATUS;
pub type PNDIS_802_5_RING_STATE = *mut NDIS_802_5_RING_STATE;
pub type PNDIS_CO_DEVICE_PROFILE = *mut NDIS_CO_DEVICE_PROFILE;
pub type PNDIS_CO_LINK_SPEED = *mut NDIS_CO_LINK_SPEED;
pub type PNDIS_DEVICE_POWER_STATE = *mut NDIS_DEVICE_POWER_STATE;
pub type PNDIS_FDDI_ATTACHMENT_TYPE = *mut NDIS_FDDI_ATTACHMENT_TYPE;
pub type PNDIS_FDDI_LCONNECTION_STATE = *mut NDIS_FDDI_LCONNECTION_STATE;
pub type PNDIS_FDDI_RING_MGT_STATE = *mut NDIS_FDDI_RING_MGT_STATE;
#[cfg(all(feature = "Win32_oidtypes", feature = "Win32_types"))]
pub type PNDIS_GUID = *mut NDIS_GUID;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_HARDWARE_CROSSTIMESTAMP = *mut NDIS_HARDWARE_CROSSTIMESTAMP;
pub type PNDIS_HARDWARE_STATUS = *mut NDIS_HARDWARE_STATUS;
#[cfg(feature = "Win32_ifdef")]
pub type PNDIS_IF_COUNTED_STRING = *mut super::ifdef::IF_COUNTED_STRING;
#[cfg(feature = "Win32_ifdef")]
pub type PNDIS_IF_PHYSICAL_ADDRESS = *mut super::ifdef::IF_PHYSICAL_ADDRESS;
pub type PNDIS_INTERRUPT_MODERATION = *mut NDIS_INTERRUPT_MODERATION;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_INTERRUPT_MODERATION_PARAMETERS = *mut NDIS_INTERRUPT_MODERATION_PARAMETERS;
pub type PNDIS_IPSEC_OFFLOAD_V1 = *mut NDIS_IPSEC_OFFLOAD_V1;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_IP_OPER_STATE = *mut NDIS_IP_OPER_STATE;
#[cfg(feature = "Win32_ifdef")]
pub type PNDIS_IP_OPER_STATUS = *mut NDIS_IP_OPER_STATUS;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_IP_OPER_STATUS_INFO = *mut NDIS_IP_OPER_STATUS_INFO;
pub type PNDIS_IRDA_PACKET_INFO = *mut NDIS_IRDA_PACKET_INFO;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_LINK_PARAMETERS = *mut NDIS_LINK_PARAMETERS;
pub type PNDIS_LINK_SPEED = *mut NDIS_LINK_SPEED;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_LINK_STATE = *mut NDIS_LINK_STATE;
#[cfg(feature = "Win32_ifdef")]
pub type PNDIS_MEDIA_CONNECT_STATE = *mut super::ifdef::NET_IF_MEDIA_CONNECT_STATE;
#[cfg(feature = "Win32_ifdef")]
pub type PNDIS_MEDIA_DUPLEX_STATE = *mut super::ifdef::NET_IF_MEDIA_DUPLEX_STATE;
pub type PNDIS_MEDIA_STATE = *mut NDIS_MEDIA_STATE;
pub type PNDIS_MEDIUM = *mut NDIS_MEDIUM;
pub type PNDIS_NETWORK_CHANGE_TYPE = *mut NDIS_NETWORK_CHANGE_TYPE;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_OFFLOAD = *mut NDIS_OFFLOAD;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_OFFLOAD_PARAMETERS = *mut NDIS_OFFLOAD_PARAMETERS;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_OPER_STATE = *mut NDIS_OPER_STATE;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_PCI_DEVICE_CUSTOM_PROPERTIES = *mut NDIS_PCI_DEVICE_CUSTOM_PROPERTIES;
pub type PNDIS_PHYSICAL_MEDIUM = *mut NDIS_PHYSICAL_MEDIUM;
pub type PNDIS_PM_PACKET_PATTERN = *mut NDIS_PM_PACKET_PATTERN;
pub type PNDIS_PM_WAKE_UP_CAPABILITIES = *mut NDIS_PM_WAKE_UP_CAPABILITIES;
pub type PNDIS_PNP_CAPABILITIES = *mut NDIS_PNP_CAPABILITIES;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_PORT = *mut NDIS_PORT;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_PORT_ARRAY = *mut NDIS_PORT_ARRAY;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_PORT_AUTHENTICATION_PARAMETERS = *mut NDIS_PORT_AUTHENTICATION_PARAMETERS;
pub type PNDIS_PORT_AUTHORIZATION_STATE = *mut NDIS_PORT_AUTHORIZATION_STATE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_PORT_CHARACTERISTICS = *mut NDIS_PORT_CHARACTERISTICS;
pub type PNDIS_PORT_CONTROLL_STATE = PNDIS_PORT_CONTROL_STATE;
pub type PNDIS_PORT_CONTROL_STATE = *mut NDIS_PORT_CONTROL_STATE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_PORT_STATE = *mut NDIS_PORT_STATE;
pub type PNDIS_PROCESSOR_VENDOR = *mut NDIS_PROCESSOR_VENDOR;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_RECEIVE_HASH_PARAMETERS = *mut NDIS_RECEIVE_HASH_PARAMETERS;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_RECEIVE_SCALE_CAPABILITIES = *mut NDIS_RECEIVE_SCALE_CAPABILITIES;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_RECEIVE_SCALE_PARAMETERS = *mut NDIS_RECEIVE_SCALE_PARAMETERS;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_STATISTICS_INFO = *mut NDIS_STATISTICS_INFO;
#[cfg(feature = "Win32_oidtypes")]
pub type PNDIS_STATISTICS_VALUE = *mut NDIS_STATISTICS_VALUE;
#[cfg(feature = "Win32_oidtypes")]
pub type PNDIS_STATISTICS_VALUE_EX = *mut NDIS_STATISTICS_VALUE_EX;
pub type PNDIS_SUPPORTED_PAUSE_FUNCTIONS = *mut NDIS_SUPPORTED_PAUSE_FUNCTIONS;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_TCP_CONNECTION_OFFLOAD = *mut NDIS_TCP_CONNECTION_OFFLOAD;
pub type PNDIS_TCP_IP_CHECKSUM_OFFLOAD = *mut NDIS_TCP_IP_CHECKSUM_OFFLOAD;
pub type PNDIS_TCP_LARGE_SEND_OFFLOAD_V1 = *mut NDIS_TCP_LARGE_SEND_OFFLOAD_V1;
pub type PNDIS_TCP_LARGE_SEND_OFFLOAD_V2 = *mut NDIS_TCP_LARGE_SEND_OFFLOAD_V2;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES = *mut NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_TIMESTAMP_CAPABILITIES = *mut NDIS_TIMESTAMP_CAPABILITIES;
pub type PNDIS_TIMESTAMP_CAPABILITY_FLAGS = *mut NDIS_TIMESTAMP_CAPABILITY_FLAGS;
pub type PNDIS_VAR_DATA_DESC = *mut NDIS_VAR_DATA_DESC;
pub type PNDIS_WAN_HEADER_FORMAT = *mut NDIS_WAN_HEADER_FORMAT;
pub type PNDIS_WAN_MEDIUM_SUBTYPE = *mut NDIS_WAN_MEDIUM_SUBTYPE;
pub type PNDIS_WAN_PROTOCOL_CAPS = *mut NDIS_WAN_PROTOCOL_CAPS;
pub type PNDIS_WAN_QUALITY = *mut NDIS_WAN_QUALITY;
pub type PNDIS_WLAN_BSSID = *mut NDIS_WLAN_BSSID;
pub type PNDIS_WLAN_BSSID_EX = *mut NDIS_WLAN_BSSID_EX;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_objectheader"))]
pub type PNDIS_WMI_ENUM_ADAPTER = *mut NDIS_WMI_ENUM_ADAPTER;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_WMI_EVENT_HEADER = *mut NDIS_WMI_EVENT_HEADER;
pub type PNDIS_WMI_IPSEC_OFFLOAD_V1 = *mut NDIS_WMI_IPSEC_OFFLOAD_V1;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_WMI_METHOD_HEADER = *mut NDIS_WMI_METHOD_HEADER;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_WMI_OFFLOAD = *mut NDIS_WMI_OFFLOAD;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_WMI_OUTPUT_INFO = *mut NDIS_WMI_OUTPUT_INFO;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ndisport", feature = "Win32_objectheader"))]
pub type PNDIS_WMI_SET_HEADER = *mut NDIS_WMI_SET_HEADER;
#[cfg(feature = "Win32_objectheader")]
pub type PNDIS_WMI_TCP_CONNECTION_OFFLOAD = *mut NDIS_WMI_TCP_CONNECTION_OFFLOAD;
pub type PNDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD = *mut NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD;
pub type PNDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 = *mut NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1;
pub type PNDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 = *mut NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2;
pub type PNETWORK_ADDRESS = *mut NETWORK_ADDRESS;
pub type PNETWORK_ADDRESS_IP = *mut NETWORK_ADDRESS_IP;
pub type PNETWORK_ADDRESS_IP6 = *mut NETWORK_ADDRESS_IP6;
pub type PNETWORK_ADDRESS_IPX = *mut NETWORK_ADDRESS_IPX;
pub type PNETWORK_ADDRESS_LIST = *mut NETWORK_ADDRESS_LIST;
pub type POFFLOAD_ALGO_INFO = *mut OFFLOAD_ALGO_INFO;
#[cfg(feature = "Win32_winnt")]
pub type POFFLOAD_IPSEC_ADD_SA = *mut OFFLOAD_IPSEC_ADD_SA;
#[cfg(feature = "Win32_winnt")]
pub type POFFLOAD_IPSEC_ADD_UDPESP_SA = *mut OFFLOAD_IPSEC_ADD_UDPESP_SA;
#[cfg(feature = "Win32_winnt")]
pub type POFFLOAD_IPSEC_DELETE_SA = *mut OFFLOAD_IPSEC_DELETE_SA;
#[cfg(feature = "Win32_winnt")]
pub type POFFLOAD_IPSEC_DELETE_UDPESP_SA = *mut OFFLOAD_IPSEC_DELETE_UDPESP_SA;
pub type POFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY = *mut OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY;
pub type POFFLOAD_SECURITY_ASSOCIATION = *mut OFFLOAD_SECURITY_ASSOCIATION;
pub type PPMKID_CANDIDATE = *mut PMKID_CANDIDATE;
pub type PTRANSPORT_HEADER_OFFSET = *mut TRANSPORT_HEADER_OFFSET;
pub type PUDP_ENCAP_TYPE = *mut UDP_ENCAP_TYPE;
pub type Priority_802_3 = u32;
pub const READABLE_LOCAL_CLOCK: u32 = 1;
pub const RECEIVE_TIME_INDICATION_CAPABLE: u32 = 8;
pub type SPI_TYPE = u32;
pub const TIMED_SEND_CAPABLE: u32 = 16;
pub const TIME_STAMP_CAPABLE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORT_HEADER_OFFSET {
    pub ProtocolType: u16,
    pub HeaderOffset: u16,
}
pub type UDP_ENCAP_TYPE = i32;
pub const WAN_PROTOCOL_KEEPS_STATS: u32 = 1;
pub const fNDIS_GUID_ALLOW_READ: u32 = 32;
pub const fNDIS_GUID_ALLOW_WRITE: u32 = 64;
pub const fNDIS_GUID_ANSI_STRING: u32 = 4;
pub const fNDIS_GUID_ARRAY: u32 = 16;
pub const fNDIS_GUID_METHOD: u32 = 128;
pub const fNDIS_GUID_NDIS_RESERVED: u32 = 256;
pub const fNDIS_GUID_SUPPORT_COMMON_HEADER: u32 = 512;
pub const fNDIS_GUID_TO_OID: u32 = 1;
pub const fNDIS_GUID_TO_STATUS: u32 = 2;
pub const fNDIS_GUID_UNICODE_STRING: u32 = 8;
