pub type CH_DESCRIPTION_TYPE = i32;
pub const DISCOVERY_FILTER_BITMASK_ANY: i32 = 15;
pub const DISCOVERY_FILTER_BITMASK_DEVICE: i32 = 1;
pub const DISCOVERY_FILTER_BITMASK_GO: i32 = 2;
pub type DOT11_AC_PARAM = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_ADDITIONAL_IE {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: super::ULONG,
    pub uBeaconIEsLength: super::ULONG,
    pub uResponseIEsOffset: super::ULONG,
    pub uResponseIEsLength: super::ULONG,
}
pub const DOT11_ADDITIONAL_IE_REVISION_1: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_AKM_CIPHER_PAIR {
    pub akm: RSNA_AKM_SUITE,
    pub cipher: RSNA_CIPHER_SUITE,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub Status: DOT11_ANQP_QUERY_RESULT,
    pub hContext: super::HANDLE,
    pub uResponseLength: super::ULONG,
}
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_ANQP_QUERY_RESULT = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_AP_JOIN_REQUEST {
    pub uJoinFailureTimeout: super::ULONG,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: super::ULONG,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub uStatus: DOT11_ASSOC_STATUS,
    pub bReAssocReq: super::BOOLEAN,
    pub bReAssocResp: super::BOOLEAN,
    pub uAssocReqOffset: super::ULONG,
    pub uAssocReqSize: super::ULONG,
    pub uAssocRespOffset: super::ULONG,
    pub uAssocRespSize: super::ULONG,
    pub uBeaconOffset: super::ULONG,
    pub uBeaconSize: super::ULONG,
    pub uIHVDataOffset: super::ULONG,
    pub uIHVDataSize: super::ULONG,
    pub AuthAlgo: super::DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: super::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: super::DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: super::ULONG,
    pub uActivePhyListSize: super::ULONG,
    pub bFourAddressSupported: super::BOOLEAN,
    pub bPortAuthorized: super::BOOLEAN,
    pub ucActiveQoSProtocol: super::UCHAR,
    pub DSInfo: DOT11_DS_INFO,
    pub uEncapTableOffset: super::ULONG,
    pub uEncapTableSize: super::ULONG,
    pub MulticastMgmtCipher: super::DOT11_CIPHER_ALGORITHM,
    pub uAssocComebackTime: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DOT11_ASSOCIATION_INFO_EX {
    pub PeerMacAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub usCapabilityInformation: super::USHORT,
    pub usListenInterval: super::USHORT,
    pub ucPeerSupportedRates: [super::UCHAR; 255],
    pub usAssociationID: super::USHORT,
    pub dot11AssociationState: DOT11_ASSOCIATION_STATE,
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: super::LARGE_INTEGER,
    pub ullNumOfTxPacketSuccesses: super::ULONGLONG,
    pub ullNumOfTxPacketFailures: super::ULONGLONG,
    pub ullNumOfRxPacketSuccesses: super::ULONGLONG,
    pub ullNumOfRxPacketFailures: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DOT11_ASSOCIATION_INFO_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11AssocInfo: [DOT11_ASSOCIATION_INFO_EX; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_ASSOCIATION_PARAMS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub uAssocRequestIEsOffset: super::ULONG,
    pub uAssocRequestIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_ASSOCIATION_START_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub SSID: super::DOT11_SSID,
    pub uIHVDataOffset: super::ULONG,
    pub uIHVDataSize: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_ASSOCIATION_STATE = i32;
pub const DOT11_ASSOC_ERROR_SOURCE_OS: i32 = 0;
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: i32 = 255;
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: i32 = 1;
pub type DOT11_ASSOC_STATUS = u32;
pub const DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE: u32 = 196608;
pub const DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE_END: u32 = 262143;
pub const DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE_START: u32 = 196608;
pub const DOT11_ASSOC_STATUS_CANCELLED: u32 = 5;
pub const DOT11_ASSOC_STATUS_CANDIDATE_LIST_EXHAUSTED: u32 = 6;
pub const DOT11_ASSOC_STATUS_DISASSOCIATED_BY_OS: u32 = 7;
pub const DOT11_ASSOC_STATUS_DISASSOCIATED_BY_RESET: u32 = 9;
pub const DOT11_ASSOC_STATUS_DISASSOCIATED_BY_ROAMING: u32 = 8;
pub const DOT11_ASSOC_STATUS_FAILURE: u32 = 1;
pub const DOT11_ASSOC_STATUS_IHV_END: u32 = 4294967295;
pub const DOT11_ASSOC_STATUS_IHV_START: u32 = 2147483648;
pub const DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED: u32 = 65536;
pub const DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED_END: u32 = 131071;
pub const DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED_START: u32 = 65536;
pub const DOT11_ASSOC_STATUS_PEER_DISASSOCIATED: u32 = 131072;
pub const DOT11_ASSOC_STATUS_PEER_DISASSOCIATED_END: u32 = 196607;
pub const DOT11_ASSOC_STATUS_PEER_DISASSOCIATED_START: u32 = 131072;
pub const DOT11_ASSOC_STATUS_PHY_DISABLED: u32 = 4;
pub const DOT11_ASSOC_STATUS_RADIO_OFF: u32 = 3;
pub const DOT11_ASSOC_STATUS_REASON_CODE_MASK: u32 = 65535;
pub const DOT11_ASSOC_STATUS_ROAMING_ADHOC: u32 = 13;
pub const DOT11_ASSOC_STATUS_ROAMING_ASSOCIATION_LOST: u32 = 12;
pub const DOT11_ASSOC_STATUS_ROAMING_BETTER_AP_FOUND: u32 = 11;
pub const DOT11_ASSOC_STATUS_SUCCESS: i32 = 0;
pub const DOT11_ASSOC_STATUS_SYSTEM_ERROR: u32 = 10;
pub const DOT11_ASSOC_STATUS_UNREACHABLE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_AUTH_ALGORITHM_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub AlgorithmIds: [super::DOT11_AUTH_ALGORITHM; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: i32 = 1;
pub const DOT11_AUTH_ALGO_MICHAEL: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_AUTH_CIPHER_PAIR_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub AuthCipherPairs: [super::DOT11_AUTH_CIPHER_PAIR; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_AVAILABLE_CHANNEL_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub uChannelNumber: [super::ULONG; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_AVAILABLE_FREQUENCY_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub uFrequencyValue: [super::ULONG; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: i32 = 1;
pub type DOT11_BAND = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_BSSID_CANDIDATE {
    pub BSSID: DOT11_MAC_ADDRESS,
    pub uFlags: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_BSSID_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub BSSIDs: [DOT11_MAC_ADDRESS; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_BSSID_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_BSS_DESCRIPTION {
    pub uReserved: super::ULONG,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::DOT11_BSS_TYPE,
    pub usBeaconPeriod: super::USHORT,
    pub ullTimestamp: super::ULONGLONG,
    pub usCapabilityInformation: super::USHORT,
    pub uBufferLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy)]
pub struct DOT11_BSS_ENTRY {
    pub uPhyId: super::ULONG,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::DOT11_BSS_TYPE,
    pub lRSSI: super::LONG,
    pub uLinkQuality: super::ULONG,
    pub bInRegDomain: super::BOOLEAN,
    pub usBeaconPeriod: super::USHORT,
    pub ullTimestamp: super::ULONGLONG,
    pub ullHostTimestamp: super::ULONGLONG,
    pub usCapabilityInformation: super::USHORT,
    pub uBufferLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    pub uChCenterFrequency: super::ULONG,
    pub FHSS: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    pub uHopPattern: super::ULONG,
    pub uHopSet: super::ULONG,
    pub uDwellTime: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_BYTE_ARRAY {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfBytes: super::ULONG,
    pub uTotalNumOfBytes: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ulReason: super::ULONG,
}
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080;
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: i32 = 128;
pub const DOT11_CAPABILITY_DSSSOFDM: i32 = 8192;
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: i32 = 4;
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: i32 = 8;
pub const DOT11_CAPABILITY_INFO_ESS: i32 = 1;
pub const DOT11_CAPABILITY_INFO_IBSS: i32 = 2;
pub const DOT11_CAPABILITY_INFO_PRIVACY: i32 = 16;
pub const DOT11_CAPABILITY_PBCC: i32 = 64;
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: i32 = 32;
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: i32 = 1024;
pub const DOT11_CCA_MODE_CS_ONLY: i32 = 2;
pub const DOT11_CCA_MODE_CS_WITH_TIMER: i32 = 8;
pub const DOT11_CCA_MODE_ED_ONLY: i32 = 1;
pub const DOT11_CCA_MODE_ED_and_CS: i32 = 4;
pub const DOT11_CCA_MODE_HRCS_AND_ED: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CHANNEL_HINT {
    pub Dot11PhyType: DOT11_PHY_TYPE,
    pub uChannelNumber: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_CIPHER_ALGORITHM_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub AlgorithmIds: [super::DOT11_CIPHER_ALGORITHM; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uKeyIndex: super::ULONG,
    pub AlgorithmId: super::DOT11_CIPHER_ALGORITHM,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub bDelete: super::BOOLEAN,
    pub bStatic: super::BOOLEAN,
    pub usKeyLength: super::USHORT,
    pub ucKey: [super::UCHAR; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub AlgorithmId: super::DOT11_CIPHER_ALGORITHM,
    pub Direction: DOT11_DIRECTION,
    pub bDelete: super::BOOLEAN,
    pub bStatic: super::BOOLEAN,
    pub usKeyLength: super::USHORT,
    pub ucKey: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: i32 = 1;
pub const DOT11_CONF_ALGO_TKIP: i32 = 2;
pub const DOT11_CONF_ALGO_WEP_RC4: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uStatus: DOT11_ASSOC_STATUS,
}
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_CONNECTION_START_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub BSSType: super::DOT11_BSS_TYPE,
    pub AdhocBSSID: DOT11_MAC_ADDRESS,
    pub AdhocSSID: super::DOT11_SSID,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_CONNECTION_STATUS_CANCELLED: u32 = 5;
pub const DOT11_CONNECTION_STATUS_CANDIDATE_LIST_EXHAUSTED: u32 = 6;
pub const DOT11_CONNECTION_STATUS_FAILURE: u32 = 1;
pub const DOT11_CONNECTION_STATUS_IHV_END: u32 = 4294967295;
pub const DOT11_CONNECTION_STATUS_IHV_START: u32 = 2147483648;
pub const DOT11_CONNECTION_STATUS_PHY_POWER_DOWN: u32 = 3;
pub const DOT11_CONNECTION_STATUS_SUCCESS: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_COUNTERS_ENTRY {
    pub uTransmittedFragmentCount: super::ULONG,
    pub uMulticastTransmittedFrameCount: super::ULONG,
    pub uFailedCount: super::ULONG,
    pub uRetryCount: super::ULONG,
    pub uMultipleRetryCount: super::ULONG,
    pub uFrameDuplicateCount: super::ULONG,
    pub uRTSSuccessCount: super::ULONG,
    pub uRTSFailureCount: super::ULONG,
    pub uACKFailureCount: super::ULONG,
    pub uReceivedFragmentCount: super::ULONG,
    pub uMulticastReceivedFrameCount: super::ULONG,
    pub uFCSErrorCount: super::ULONG,
    pub uTransmittedFrameCount: super::ULONG,
}
pub type DOT11_COUNTRY_OR_REGION_STRING = [u8; 3];
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub CountryOrRegionStrings: [DOT11_COUNTRY_OR_REGION_STRING; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY {
    pub uReserved: super::ULONG,
    pub uFlags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CURRENT_OPERATION_MODE {
    pub uReserved: super::ULONG,
    pub uCurrentOpMode: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY {
    pub uReserved: super::ULONG,
    pub bDot11CFPollable: super::BOOLEAN,
    pub bDot11PCF: super::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::BOOLEAN,
}
pub const DOT11_DATA_RATE_INDEX_MASK: u32 = 127;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_DATA_RATE_MAPPING_ENTRY {
    pub ucDataRateIndex: super::UCHAR,
    pub ucDataRateFlag: super::UCHAR,
    pub usDataRateValue: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DATA_RATE_MAPPING_TABLE {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uDataRateMappingLength: super::ULONG,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: i32 = 1;
pub const DOT11_DATA_RATE_NON_STANDARD: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DEFAULT_WEP_OFFLOAD {
    pub uReserved: super::ULONG,
    pub hOffloadContext: super::HANDLE,
    pub hOffload: super::HANDLE,
    pub dwIndex: super::ULONG,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: super::ULONG,
    pub uFlags: super::ULONG,
    pub dot11KeyDirection: DOT11_KEY_DIRECTION,
    pub ucMacAddress: [super::UCHAR; 6],
    pub uNumOfRWsOnMe: super::ULONG,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [super::USHORT; 16],
    pub usKeyLength: super::USHORT,
    pub ucKey: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DEFAULT_WEP_UPLOAD {
    pub uReserved: super::ULONG,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::HANDLE,
    pub uNumOfRWsUsed: super::ULONG,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [super::USHORT; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: i32 = 1;
pub type DOT11_DIALOG_TOKEN = u8;
pub type DOT11_DIRECTION = i32;
pub const DOT11_DIR_BOTH: DOT11_DIRECTION = 3;
pub const DOT11_DIR_INBOUND: DOT11_DIRECTION = 1;
pub const DOT11_DIR_OUTBOUND: DOT11_DIRECTION = 2;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DISASSOCIATE_PEER_REQUEST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub usReason: super::USHORT,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DISASSOCIATION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub uReason: DOT11_ASSOC_STATUS,
    pub uIHVDataOffset: super::ULONG,
    pub uIHVDataSize: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_DISASSOC_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_DISASSOC_REASON_IHV_START: u32 = 2147483648;
pub const DOT11_DISASSOC_REASON_OS: u32 = 7;
pub const DOT11_DISASSOC_REASON_PEER_DEAUTHENTICATED: u32 = 65536;
pub const DOT11_DISASSOC_REASON_PEER_DISASSOCIATED: u32 = 131072;
pub const DOT11_DISASSOC_REASON_PEER_UNREACHABLE: u32 = 2;
pub const DOT11_DISASSOC_REASON_PHY_DISABLED: u32 = 4;
pub const DOT11_DISASSOC_REASON_RADIO_OFF: u32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_DIVERSITY_SELECTION_RX {
    pub uAntennaListIndex: super::ULONG,
    pub bDiversitySelectionRX: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11DiversitySelectionRx: [DOT11_DIVERSITY_SELECTION_RX; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_DIVERSITY_SUPPORT = i32;
pub const DOT11_DS_CHANGED: DOT11_DS_INFO = 0;
pub type DOT11_DS_INFO = i32;
pub const DOT11_DS_UNCHANGED: DOT11_DS_INFO = 1;
pub const DOT11_DS_UNKNOWN: DOT11_DS_INFO = 2;
pub const DOT11_ENCAP_802_1H: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_ENCAP_ENTRY {
    pub usEtherType: super::USHORT,
    pub usEncapType: super::USHORT,
}
pub const DOT11_ENCAP_RFC_1042: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_ERP_PHY_ATTRIBUTES {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub bERPPBCCOptionImplemented: super::BOOLEAN,
    pub bDSSSOFDMOptionImplemented: super::BOOLEAN,
    pub bShortSlotTimeOptionImplemented: super::BOOLEAN,
}
pub const DOT11_EXEMPT_ALWAYS: i32 = 1;
pub const DOT11_EXEMPT_BOTH: i32 = 3;
pub const DOT11_EXEMPT_MULTICAST: i32 = 2;
pub const DOT11_EXEMPT_NO_EXEMPTION: i32 = 0;
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: i32 = 2;
pub const DOT11_EXEMPT_UNICAST: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_EXTAP_ATTRIBUTES {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: super::ULONG,
    pub uDesiredSSIDListSize: super::ULONG,
    pub uPrivacyExemptionListSize: super::ULONG,
    pub uAssociationTableSize: super::ULONG,
    pub uDefaultKeyTableSize: super::ULONG,
    pub uWEPKeyValueMaxLength: super::ULONG,
    pub bStrictlyOrderedServiceClassImplemented: super::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: super::ULONG,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uInfraNumSupportedUcastAlgoPairs: super::ULONG,
    pub pInfraSupportedUcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: super::ULONG,
    pub pInfraSupportedMcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
}
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: i32 = 1;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type DOT11_EXTAP_RECV_CONTEXT = DOT11_EXTSTA_RECV_CONTEXT;
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: i32 = 1;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type DOT11_EXTAP_SEND_CONTEXT = DOT11_EXTSTA_SEND_CONTEXT;
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_EXTSTA_ATTRIBUTES {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: super::ULONG,
    pub uDesiredBSSIDListSize: super::ULONG,
    pub uDesiredSSIDListSize: super::ULONG,
    pub uExcludedMacAddressListSize: super::ULONG,
    pub uPrivacyExemptionListSize: super::ULONG,
    pub uKeyMappingTableSize: super::ULONG,
    pub uDefaultKeyTableSize: super::ULONG,
    pub uWEPKeyValueMaxLength: super::ULONG,
    pub uPMKIDCacheSize: super::ULONG,
    pub uMaxNumPerSTADefaultKeyTables: super::ULONG,
    pub bStrictlyOrderedServiceClassImplemented: super::BOOLEAN,
    pub ucSupportedQoSProtocolFlags: super::UCHAR,
    pub bSafeModeImplemented: super::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: super::ULONG,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uInfraNumSupportedUcastAlgoPairs: super::ULONG,
    pub pInfraSupportedUcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: super::ULONG,
    pub pInfraSupportedMcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedUcastAlgoPairs: super::ULONG,
    pub pAdhocSupportedUcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedMcastAlgoPairs: super::ULONG,
    pub pAdhocSupportedMcastAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub bAutoPowerSaveMode: super::BOOLEAN,
    pub uMaxNetworkOffloadListSize: super::ULONG,
    pub bMFPCapable: super::BOOLEAN,
    pub uInfraNumSupportedMcastMgmtAlgoPairs: super::ULONG,
    pub pInfraSupportedMcastMgmtAlgoPairs: super::PDOT11_AUTH_CIPHER_PAIR,
    pub bNeighborReportSupported: super::BOOLEAN,
    pub bAPChannelReportSupported: super::BOOLEAN,
    pub bActionFramesSupported: super::BOOLEAN,
    pub bANQPQueryOffloadSupported: super::BOOLEAN,
    pub bHESSIDConnectionSupported: super::BOOLEAN,
}
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: i32 = 1;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: i32 = 2;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: i32 = 3;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: i32 = 4;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: i32 = 2;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: i32 = 1;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: i32 = 12;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_EXTSTA_CAPABILITY {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: super::ULONG,
    pub uDesiredBSSIDListSize: super::ULONG,
    pub uDesiredSSIDListSize: super::ULONG,
    pub uExcludedMacAddressListSize: super::ULONG,
    pub uPrivacyExemptionListSize: super::ULONG,
    pub uKeyMappingTableSize: super::ULONG,
    pub uDefaultKeyTableSize: super::ULONG,
    pub uWEPKeyValueMaxLength: super::ULONG,
    pub uPMKIDCacheSize: super::ULONG,
    pub uMaxNumPerSTADefaultKeyTables: super::ULONG,
}
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_EXTSTA_RECV_CONTEXT {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uReceiveFlags: super::ULONG,
    pub uPhyId: super::ULONG,
    pub uChCenterFrequency: super::ULONG,
    pub usNumberOfMPDUsReceived: super::USHORT,
    pub lRSSI: super::LONG,
    pub ucDataRate: super::UCHAR,
    pub uSizeMediaSpecificInfo: super::ULONG,
    pub pvMediaSpecificInfo: super::PVOID,
    pub ullTimestamp: super::ULONGLONG,
}
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_EXTSTA_SEND_CONTEXT {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub usExemptionActionType: super::USHORT,
    pub uPhyId: super::ULONG,
    pub uDelayedSleepValue: super::ULONG,
    pub pvMediaSpecificInfo: super::PVOID,
    pub uSendFlags: super::ULONG,
}
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: i32 = 1;
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: i32 = 4;
pub const DOT11_FLAGS_80211B_PBCC: i32 = 2;
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: i32 = 1;
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: i32 = 128;
pub const DOT11_FLAGS_80211G_DSSS_OFDM: i32 = 16;
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: i32 = 64;
pub const DOT11_FLAGS_80211G_USE_PROTECTION: i32 = 32;
pub const DOT11_FLAGS_PS_ON: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_FRAGMENT_DESCRIPTOR {
    pub uOffset: super::ULONG,
    pub uLength: super::ULONG,
}
pub const DOT11_FREQUENCY_BANDS_LOWER: i32 = 1;
pub const DOT11_FREQUENCY_BANDS_MIDDLE: i32 = 2;
pub const DOT11_FREQUENCY_BANDS_UPPER: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_HESSID = [u8; 6];
pub const DOT11_HESSID_LENGTH: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_HOPPING_PATTERN_ENTRY {
    pub uHoppingPatternIndex: super::ULONG,
    pub uRandomTableFieldNumber: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11HoppingPatternEntry: [DOT11_HOPPING_PATTERN_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_HOP_ALGO_ADOPTED = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES {
    pub bShortPreambleOptionImplemented: super::BOOLEAN,
    pub bPBCCOptionImplemented: super::BOOLEAN,
    pub bChannelAgilityPresent: super::BOOLEAN,
    pub uHRCCAModeSupported: super::ULONG,
}
pub const DOT11_HR_CCA_MODE_CS_AND_ED: i32 = 4;
pub const DOT11_HR_CCA_MODE_CS_ONLY: i32 = 2;
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: i32 = 8;
pub const DOT11_HR_CCA_MODE_ED_ONLY: i32 = 1;
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: i32 = 16;
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: i32 = 8;
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: i32 = 4;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: i32 = 32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: i32 = 16;
pub const DOT11_HW_WEP_SUPPORTED_RX: i32 = 2;
pub const DOT11_HW_WEP_SUPPORTED_TX: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_IBSS_PARAMS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bJoinOnly: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
pub const DOT11_IBSS_PARAMS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub uStatus: super::ULONG,
    pub ucErrorSource: super::UCHAR,
    pub bReAssocReq: super::BOOLEAN,
    pub bReAssocResp: super::BOOLEAN,
    pub uAssocReqOffset: super::ULONG,
    pub uAssocReqSize: super::ULONG,
    pub uAssocRespOffset: super::ULONG,
    pub uAssocRespSize: super::ULONG,
    pub AuthAlgo: super::DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: super::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: super::DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: super::ULONG,
    pub uActivePhyListSize: super::ULONG,
    pub uBeaconOffset: super::ULONG,
    pub uBeaconSize: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_DECISION {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bAccept: super::BOOLEAN,
    pub usReasonCode: super::USHORT,
    pub uAssocResponseIEsOffset: super::ULONG,
    pub uAssocResponseIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: i32 = 1;
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_DECISION_V2 {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bAccept: super::BOOLEAN,
    pub usReasonCode: super::USHORT,
    pub uAssocResponseIEsOffset: super::ULONG,
    pub uAssocResponseIEsLength: super::ULONG,
    pub WFDStatus: DOT11_WFD_STATUS_CODE,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bReAssocReq: super::BOOLEAN,
    pub uAssocReqOffset: super::ULONG,
    pub uAssocReqSize: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_INVALID_CHANNEL_NUMBER: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub ReceiverAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_IV48_COUNTER {
    pub uIV32Counter: super::ULONG,
    pub usIV16Counter: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_JOIN_REQUEST {
    pub uJoinFailureTimeout: super::ULONG,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: super::ULONG,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_BIP {
    pub ucIPN: [super::UCHAR; 6],
    pub ulBIPKeyLength: super::ULONG,
    pub ucBIPKey: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_BIP_GMAC_256 {
    pub ucIPN: [super::UCHAR; 6],
    pub ulBIPGmac256KeyLength: super::ULONG,
    pub ucBIPGmac256Key: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_CCMP {
    pub ucIV48Counter: [super::UCHAR; 6],
    pub ulCCMPKeyLength: super::ULONG,
    pub ucCCMPKey: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_GCMP {
    pub ucIV48Counter: [super::UCHAR; 6],
    pub ulGCMPKeyLength: super::ULONG,
    pub ucGCMPKey: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_GCMP_256 {
    pub ucIV48Counter: [super::UCHAR; 6],
    pub ulGCMP256KeyLength: super::ULONG,
    pub ucGCMP256Key: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_KEY_ALGO_TKIP_MIC {
    pub ucIV48Counter: [super::UCHAR; 6],
    pub ulTKIPKeyLength: super::ULONG,
    pub ulMICKeyLength: super::ULONG,
    pub ucTKIPMICKeys: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_KEY_DIRECTION = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_LINK_QUALITY_ENTRY {
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub ucLinkQuality: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for DOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_LINK_QUALITY_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uLinkQualityListSize: super::ULONG,
    pub uLinkQualityListOffset: super::ULONG,
}
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_MAC_ADDRESS = [u8; 6];
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MAC_ADDRESS_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub MacAddrs: [DOT11_MAC_ADDRESS; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: i32 = 1;
pub const DOT11_MAC_AUTO_CONFIG_ENABLED_FLAG: u32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: super::ULONGLONG,
    pub ullReceivedFrameCount: super::ULONGLONG,
    pub ullTransmittedFailureFrameCount: super::ULONGLONG,
    pub ullReceivedFailureFrameCount: super::ULONGLONG,
    pub ullWEPExcludedCount: super::ULONGLONG,
    pub ullTKIPLocalMICFailures: super::ULONGLONG,
    pub ullTKIPReplays: super::ULONGLONG,
    pub ullTKIPICVErrorCount: super::ULONGLONG,
    pub ullCCMPReplays: super::ULONGLONG,
    pub ullCCMPDecryptErrors: super::ULONGLONG,
    pub ullWEPUndecryptableCount: super::ULONGLONG,
    pub ullWEPICVErrorCount: super::ULONGLONG,
    pub ullDecryptSuccessCount: super::ULONGLONG,
    pub ullDecryptFailureCount: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MAC_INFO {
    pub uReserved: super::ULONG,
    pub uNdisPortNumber: super::ULONG,
    pub MacAddr: DOT11_MAC_ADDRESS,
}
#[cfg(feature = "shared")]
impl Default for DOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MAC_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uOpmodeMask: super::ULONG,
}
pub const DOT11_MAC_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub dot11ManufacturingCallbackType: DOT11_MANUFACTURING_CALLBACK_TYPE,
    pub uStatus: super::ULONG,
    pub pvContext: super::PVOID,
}
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: i32 = 1;
pub type DOT11_MANUFACTURING_CALLBACK_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    pub Dot11Band: DOT11_BAND,
    pub uChannel: super::ULONG,
    pub ADCPowerLevel: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    pub bEnabled: super::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: super::ULONG,
    pub PowerLevel: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    pub bEnable: super::BOOLEAN,
    pub bOpenLoop: super::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: super::ULONG,
    pub uSetPowerLevel: super::ULONG,
    pub ADCPowerLevel: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: super::ULONG,
    pub bResult: super::BOOLEAN,
    pub uPinFailedBitMask: super::ULONG,
    pub pvContext: super::PVOID,
    pub uBytesWrittenOut: super::ULONG,
    pub ucBufferOut: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: super::ULONG,
    pub uPinBitMask: super::ULONG,
    pub pvContext: super::PVOID,
    pub uBufferLength: super::ULONG,
    pub ucBufferIn: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_MANUFACTURING_SELF_TEST_TYPE = i32;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_BT_COEXISTENCE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 3;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 1;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_RF_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST {
    pub dot11ManufacturingTestType: DOT11_MANUFACTURING_TEST_TYPE,
    pub uBufferLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA {
    pub uKey: super::ULONG,
    pub uOffset: super::ULONG,
    pub uBufferLength: super::ULONG,
    pub uBytesRead: super::ULONG,
    pub ucBufferOut: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MANUFACTURING_TEST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_SET_DATA {
    pub uKey: super::ULONG,
    pub uOffset: super::ULONG,
    pub uBufferLength: super::ULONG,
    pub ucBufferIn: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_SLEEP {
    pub uSleepTime: super::ULONG,
    pub pvContext: super::PVOID,
}
pub type DOT11_MANUFACTURING_TEST_TYPE = i32;
pub const DOT11_MAX_CHANNEL_HINTS: i32 = 4;
pub const DOT11_MAX_NUM_DEFAULT_KEY: i32 = 4;
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: i32 = 6;
pub const DOT11_MAX_NUM_OF_FRAGMENTS: i32 = 16;
pub const DOT11_MAX_PDU_SIZE: i32 = 2346;
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: i32 = 255;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11MDCapabilityEntry: [DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MIN_PDU_SIZE: i32 = 256;
#[cfg(feature = "shared")]
pub const DOT11_MIN_SIZEOF_OFFLOAD_NETWORK_LIST_INFO_REVISION_1: super::LONG = 0x18_u32 as _;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uPhyId: super::ULONG,
    pub uMPDUMaxLength: super::ULONG,
}
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    pub uMultiDomainCapabilityIndex: super::ULONG,
    pub uFirstChannelNumber: super::ULONG,
    pub uNumberOfChannels: super::ULONG,
    pub lMaximumTransmitPowerLevel: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_NIC_SPECIFIC_EXTENSION {
    pub uBufferLength: super::ULONG,
    pub uTotalBufferLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: i32 = 4;
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: i32 = 2;
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_OFDM_PHY_ATTRIBUTES {
    pub uFrequencyBandsSupported: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_OFFLOAD_CAPABILITY {
    pub uReserved: super::ULONG,
    pub uFlags: super::ULONG,
    pub uSupportedWEPAlgorithms: super::ULONG,
    pub uNumOfReplayWindows: super::ULONG,
    pub uMaxWEPKeyMappingLength: super::ULONG,
    pub uSupportedAuthAlgorithms: super::ULONG,
    pub uMaxAuthKeyMappingLength: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK {
    pub Ssid: super::DOT11_SSID,
    pub UnicastCipher: super::DOT11_CIPHER_ALGORITHM,
    pub AuthAlgo: super::DOT11_AUTH_ALGORITHM,
    pub Dot11ChannelHints: [DOT11_CHANNEL_HINT; 4],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ulFlags: super::ULONG,
    pub FastScanPeriod: super::ULONG,
    pub FastScanIterations: super::ULONG,
    pub SlowScanPeriod: super::ULONG,
    pub uNumOfEntries: super::ULONG,
    pub offloadNetworkList: [DOT11_OFFLOAD_NETWORK; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub Status: super::NDIS_STATUS,
}
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_OFFLOAD_TYPE = i32;
pub const DOT11_OPERATION_MODE_AP: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_OPERATION_MODE_CAPABILITY {
    pub uReserved: super::ULONG,
    pub uMajorVersion: super::ULONG,
    pub uMinorVersion: super::ULONG,
    pub uNumOfTXBuffers: super::ULONG,
    pub uNumOfRXBuffers: super::ULONG,
    pub uOpModeCapability: super::ULONG,
}
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: i32 = 8;
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: i32 = 4;
pub const DOT11_OPERATION_MODE_MANUFACTURING: i32 = 1073741824;
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648;
pub const DOT11_OPERATION_MODE_STATION: i32 = 1;
pub const DOT11_OPERATION_MODE_UNKNOWN: i32 = 0;
pub const DOT11_OPERATION_MODE_WFD_CLIENT: i32 = 64;
pub const DOT11_OPERATION_MODE_WFD_DEVICE: i32 = 16;
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_OPTIONAL_CAPABILITY {
    pub uReserved: super::ULONG,
    pub bDot11PCF: super::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::BOOLEAN,
}
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: i32 = 4096;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: i32 = 16384;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: i32 = 8192;
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: i32 = 64;
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: i32 = 256;
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: i32 = 128;
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: i32 = 1;
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: i32 = 4;
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: i32 = 2;
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: i32 = 8;
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: i32 = 32;
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: i32 = 16;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: i32 = 512;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: i32 = 2048;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: i32 = 1024;
pub const DOT11_PACKET_TYPE_RESERVED: i32 = -32768;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wlan"))]
#[derive(Clone, Copy)]
pub struct DOT11_PEER_INFO {
    pub MacAddress: DOT11_MAC_ADDRESS,
    pub usCapabilityInformation: super::USHORT,
    pub AuthAlgo: super::DOT11_AUTH_ALGORITHM,
    pub UnicastCipherAlgo: super::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipherAlgo: super::DOT11_CIPHER_ALGORITHM,
    pub bWpsEnabled: super::BOOLEAN,
    pub usListenInterval: super::USHORT,
    pub ucSupportedRates: [super::UCHAR; 255],
    pub usAssociationID: super::USHORT,
    pub AssociationState: DOT11_ASSOCIATION_STATE,
    pub PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: super::LARGE_INTEGER,
    pub Statistics: DOT11_PEER_STATISTICS,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wlan"))]
impl Default for DOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
#[derive(Clone, Copy)]
pub struct DOT11_PEER_INFO_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub PeerInfo: [DOT11_PEER_INFO; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
impl Default for DOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PEER_INFO_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PEER_STATISTICS {
    pub ullDecryptSuccessCount: super::ULONGLONG,
    pub ullDecryptFailureCount: super::ULONGLONG,
    pub ullTxPacketSuccessCount: super::ULONGLONG,
    pub ullTxPacketFailureCount: super::ULONGLONG,
    pub ullRxPacketSuccessCount: super::ULONGLONG,
    pub ullRxPacketFailureCount: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PER_MSDU_COUNTERS {
    pub uTransmittedFragmentCount: super::ULONG,
    pub uRetryCount: super::ULONG,
    pub uRTSSuccessCount: super::ULONG,
    pub uRTSFailureCount: super::ULONG,
    pub uACKFailureCount: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct DOT11_PHY_ATTRIBUTES {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PhyType: DOT11_PHY_TYPE,
    pub bHardwarePhyState: super::BOOLEAN,
    pub bSoftwarePhyState: super::BOOLEAN,
    pub bCFPollable: super::BOOLEAN,
    pub uMPDUMaxLength: super::ULONG,
    pub TempType: DOT11_TEMP_TYPE,
    pub DiversitySupport: DOT11_DIVERSITY_SUPPORT,
    pub PhySpecificAttributes: DOT11_PHY_ATTRIBUTES_0,
    pub uNumberSupportedPowerLevels: super::ULONG,
    pub TxPowerLevels: [super::ULONG; 8],
    pub uNumDataRateMappingEntries: super::ULONG,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
    pub SupportedDataRatesValue: DOT11_SUPPORTED_DATA_RATES_VALUE_V2,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DOT11_PHY_ATTRIBUTES_0 {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub OFDMAttributes: DOT11_OFDM_PHY_ATTRIBUTES,
    pub ERPAttributes: DOT11_ERP_PHY_ATTRIBUTES,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: i32 = 1;
pub const DOT11_PHY_AUTO_CONFIG_ENABLED_FLAG: u32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: super::ULONGLONG,
    pub ullMulticastTransmittedFrameCount: super::ULONGLONG,
    pub ullFailedCount: super::ULONGLONG,
    pub ullRetryCount: super::ULONGLONG,
    pub ullMultipleRetryCount: super::ULONGLONG,
    pub ullMaxTXLifetimeExceededCount: super::ULONGLONG,
    pub ullTransmittedFragmentCount: super::ULONGLONG,
    pub ullRTSSuccessCount: super::ULONGLONG,
    pub ullRTSFailureCount: super::ULONGLONG,
    pub ullACKFailureCount: super::ULONGLONG,
    pub ullReceivedFrameCount: super::ULONGLONG,
    pub ullMulticastReceivedFrameCount: super::ULONGLONG,
    pub ullPromiscuousReceivedFrameCount: super::ULONGLONG,
    pub ullMaxRXLifetimeExceededCount: super::ULONGLONG,
    pub ullFrameDuplicateCount: super::ULONGLONG,
    pub ullReceivedFragmentCount: super::ULONGLONG,
    pub ullPromiscuousReceivedFragmentCount: super::ULONGLONG,
    pub ullFCSErrorCount: super::ULONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ulPhyId: super::ULONG,
    pub Anonymous: DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    pub ulChannel: super::ULONG,
    pub ulFrequency: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_PHY_ID_ANY: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PHY_ID_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11PhyId: [super::ULONG; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ID_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PHY_STATE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uPhyId: super::ULONG,
    pub bHardwarePhyState: super::BOOLEAN,
    pub bSoftwarePhyState: super::BOOLEAN,
}
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: i32 = 1;
pub type DOT11_PHY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PHY_TYPE_INFO {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub bUseParameters: super::BOOLEAN,
    pub uProbeDelay: super::ULONG,
    pub uMinChannelTime: super::ULONG,
    pub uMaxChannelTime: super::ULONG,
    pub ChDescriptionType: CH_DESCRIPTION_TYPE,
    pub uChannelListSize: super::ULONG,
    pub ucChannelListBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PHY_TYPE_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11PhyType: [DOT11_PHY_TYPE; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_TYPE_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uCandidateListSize: super::ULONG,
    pub uCandidateListOffset: super::ULONG,
}
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_PMKID_CANDIDATE_PREAUTH_ENABLED: u32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PMKID_ENTRY {
    pub BSSID: DOT11_MAC_ADDRESS,
    pub PMKID: DOT11_PMKID_VALUE,
    pub uFlags: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PMKID_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub PMKIDs: [DOT11_PMKID_ENTRY; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PMKID_LIST_REVISION_1: i32 = 1;
pub type DOT11_PMKID_VALUE = [u8; 16];
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PORT_STATE_NOTIFICATION {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerMac: DOT11_MAC_ADDRESS,
    pub bOpen: super::BOOLEAN,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bEnabled: super::BOOLEAN,
}
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_POWER_MGMT_MODE {
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: super::ULONG,
    pub usListenInterval: super::USHORT,
    pub usAID: super::USHORT,
    pub bReceiveDTIMs: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PowerSaveMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: super::ULONG,
    pub Reason: DOT11_POWER_MODE_REASON,
}
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: i32 = 1;
pub type DOT11_POWER_MODE = i32;
pub type DOT11_POWER_MODE_REASON = i32;
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: i32 = 2;
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: i32 = 1;
pub const DOT11_POWER_SAVING_FAST_PSP: i32 = 8;
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: i32 = 24;
pub const DOT11_POWER_SAVING_MAX_PSP: i32 = 16;
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: i32 = 0;
pub const DOT11_PRIORITY_CONTENTION: i32 = 0;
pub const DOT11_PRIORITY_CONTENTION_FREE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_PRIVACY_EXEMPTION {
    pub usEtherType: super::USHORT,
    pub usExemptionActionType: super::USHORT,
    pub usExemptionPacketType: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PRIVACY_EXEMPTION_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub PrivacyExemptionEntries: [DOT11_PRIVACY_EXEMPTION; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub ReceiverAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::NDIS_STATUS,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
impl Default for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_QOS_PARAMS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ucEnabledQoSProtocolFlags: super::UCHAR,
}
pub const DOT11_QOS_PARAMS_REVISION_1: i32 = 1;
pub const DOT11_QOS_PROTOCOL_FLAG_11E: u32 = 2;
pub const DOT11_QOS_PROTOCOL_FLAG_WMM: u32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_QOS_TX_DURATION {
    pub uNominalMSDUSize: super::ULONG,
    pub uMinPHYRate: super::ULONG,
    pub uDuration: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_QOS_TX_MEDIUM_TIME {
    pub dot11PeerAddress: DOT11_MAC_ADDRESS,
    pub ucQoSPriority: super::UCHAR,
    pub uMediumTimeAdmited: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RATE_SET {
    pub uRateSetLength: super::ULONG,
    pub ucRateSet: [super::UCHAR; 126],
}
#[cfg(feature = "shared")]
impl Default for DOT11_RATE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RATE_SET_MAX_LENGTH: i32 = 126;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub ResponseContext: super::PVOID,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECV_EXTENSION_INFO {
    pub uVersion: super::ULONG,
    pub pvReserved: super::PVOID,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: super::ULONG,
    pub lRSSI: super::LONG,
    pub lRSSIMin: super::LONG,
    pub lRSSIMax: super::LONG,
    pub uRSSI: super::ULONG,
    pub ucPriority: super::UCHAR,
    pub ucDataRate: super::UCHAR,
    pub ucPeerMacAddress: [super::UCHAR; 6],
    pub dwExtendedStatus: super::ULONG,
    pub hWEPOffloadContext: super::HANDLE,
    pub hAuthOffloadContext: super::HANDLE,
    pub usWEPAppliedMask: super::USHORT,
    pub usWPAMSDUPriority: super::USHORT,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: super::USHORT,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: super::USHORT,
    pub usNumberOfMPDUsReceived: super::USHORT,
    pub usNumberOfFragments: super::USHORT,
    pub pNdisPackets: [super::PVOID; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RECV_EXTENSION_INFO_V2 {
    pub uVersion: super::ULONG,
    pub pvReserved: super::PVOID,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: super::ULONG,
    pub lRSSI: super::LONG,
    pub uRSSI: super::ULONG,
    pub ucPriority: super::UCHAR,
    pub ucDataRate: super::UCHAR,
    pub ucPeerMacAddress: [super::UCHAR; 6],
    pub dwExtendedStatus: super::ULONG,
    pub hWEPOffloadContext: super::HANDLE,
    pub hAuthOffloadContext: super::HANDLE,
    pub usWEPAppliedMask: super::USHORT,
    pub usWPAMSDUPriority: super::USHORT,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: super::USHORT,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: super::USHORT,
    pub usNumberOfMPDUsReceived: super::USHORT,
    pub usNumberOfFragments: super::USHORT,
    pub pNdisPackets: [super::PVOID; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECV_FLAG_RAW_PACKET: u32 = 1;
pub const DOT11_RECV_FLAG_RAW_PACKET_FCS_FAILURE: u32 = 2;
pub const DOT11_RECV_FLAG_RAW_PACKET_TIMESTAMP: u32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_RECV_SENSITIVITY {
    pub ucDataRate: super::UCHAR,
    pub lRSSIMin: super::LONG,
    pub lRSSIMax: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DOT11_RECV_SENSITIVITY_LIST {
    pub Anonymous: DOT11_RECV_SENSITIVITY_LIST_0,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11RecvSensitivity: [DOT11_RECV_SENSITIVITY; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOT11_RECV_SENSITIVITY_LIST_0 {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uPhyId: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11RegDomainValue: [DOT11_REG_DOMAIN_VALUE; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_REG_DOMAIN_DOC: i32 = 32;
pub const DOT11_REG_DOMAIN_ETSI: i32 = 48;
pub const DOT11_REG_DOMAIN_FCC: i32 = 16;
pub const DOT11_REG_DOMAIN_FRANCE: i32 = 50;
pub const DOT11_REG_DOMAIN_MKK: i32 = 64;
pub const DOT11_REG_DOMAIN_OTHER: i32 = 0;
pub const DOT11_REG_DOMAIN_SPAIN: i32 = 49;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_REG_DOMAIN_VALUE {
    pub uRegDomainsSupportIndex: super::ULONG,
    pub uRegDomainsSupportValue: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_RESET_REQUEST {
    pub dot11ResetType: DOT11_RESET_TYPE,
    pub dot11MacAddress: DOT11_MAC_ADDRESS,
    pub bSetDefaultMIB: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for DOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_RESET_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uStatus: DOT11_ASSOC_STATUS,
}
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_ROAMING_REASON_ADHOC: u32 = 13;
pub const DOT11_ROAMING_REASON_ASSOCIATION_LOST: u32 = 12;
pub const DOT11_ROAMING_REASON_BETTER_AP_FOUND: u32 = 11;
pub const DOT11_ROAMING_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_ROAMING_REASON_IHV_START: u32 = 2147483648;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_ROAMING_START_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub AdhocBSSID: DOT11_MAC_ADDRESS,
    pub AdhocSSID: super::DOT11_SSID,
    pub uRoamingReason: DOT11_ASSOC_STATUS,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_RSSI_RANGE {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uRSSIMin: super::ULONG,
    pub uRSSIMax: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SCAN_REQUEST {
    pub dot11BSSType: super::DOT11_BSS_TYPE,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11SSID: super::DOT11_SSID,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::BOOLEAN,
    pub bUseRequestIE: super::BOOLEAN,
    pub uRequestIDsOffset: super::ULONG,
    pub uNumOfRequestIDs: super::ULONG,
    pub uPhyTypesOffset: super::ULONG,
    pub uNumOfPhyTypes: super::ULONG,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SCAN_REQUEST_V2 {
    pub dot11BSSType: super::DOT11_BSS_TYPE,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::BOOLEAN,
    pub udot11SSIDsOffset: super::ULONG,
    pub uNumOfdot11SSIDs: super::ULONG,
    pub bUseRequestIE: super::BOOLEAN,
    pub uRequestIDsOffset: super::ULONG,
    pub uNumOfRequestIDs: super::ULONG,
    pub uPhyTypeInfosOffset: super::ULONG,
    pub uNumOfPhyTypeInfos: super::ULONG,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_SCAN_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub ResponseContext: super::PVOID,
    pub uSendTimeout: super::ULONG,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
impl Default for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uSendTimeout: super::ULONG,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: DOT11_MAC_ADDRESS,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uSendTimeout: super::ULONG,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: DOT11_MAC_ADDRESS,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
impl Default for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub uSendTimeout: super::ULONG,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub InvitationFlags: DOT11_WFD_INVITATION_FLAGS,
    pub GroupBSSID: DOT11_MAC_ADDRESS,
    pub bUseGroupBSSID: super::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::BOOLEAN,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bLocalGO: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uSendTimeout: super::ULONG,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub GroupBSSID: DOT11_MAC_ADDRESS,
    pub bUseGroupBSSID: super::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub uSendTimeout: super::ULONG,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::BOOLEAN,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: super::PVOID,
    pub uSendTimeout: super::ULONG,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
impl Default for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: i32 = 0;
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: i32 = 1;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_2: u64 = 28;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_MAC_PARAMETERS_REVISION_1: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_MAC_PARAMETERS_REVISION_1: u64 = 8;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u64 = 8;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_POWER_MGMT_AUTO_MODE_ENABLE_INFO_REVISION_1: u32 = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_POWER_MGMT_AUTO_MODE_ENABLE_INFO_REVISION_1: u64 = 6;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u64 = 16;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 20;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u64 = 20;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u64 = 40;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u64 = 28;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u64 = 40;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u64 = 28;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u64 = 88;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u64 = 36;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 88;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u64 = 96;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 88;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u64 = 88;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u64 = 56;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u64 = 76;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u64 = 40;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_1: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_1: u64 = 16;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_2: u32 = 20;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_2: u64 = 20;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_3: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_3: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_ADDITIONAL_IE_REVISION_1: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_ADDITIONAL_IE_REVISION_1: u64 = 28;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_ATTRIBUTES_REVISION_1: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_ATTRIBUTES_REVISION_1: u64 = 72;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_DEVICE_CAPABILITY_CONFIG_1: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_DEVICE_CAPABILITY_CONFIG_1: u64 = 16;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_DEVICE_INFO_REVISION_1: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_DEVICE_INFO_REVISION_1: u64 = 56;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u64 = 6;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u64 = 24;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_DISCOVER_REQUEST_REVISION_1: u64 = 36;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_1: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_1: u64 = 16;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_2: u32 = 20;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_2: u64 = 20;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 20;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u64 = 20;
#[cfg(target_arch = "x86")]
pub const DOT11_SIZEOF_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 10;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DOT11_SIZEOF_WFD_GROUP_START_PARAMETERS_REVISION_1: u64 = 10;
#[cfg(feature = "shared")]
pub const DOT11_SIZEOF_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: super::LONG = 0xC_u32 as _;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SSID_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub SSIDs: [super::DOT11_SSID; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
impl Default for DOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SSID_LIST_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_START_REQUEST {
    pub uStartFailureTimeout: super::ULONG,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: super::ULONG,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_STATISTICS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ullFourWayHandshakeFailures: super::ULONGLONG,
    pub ullTKIPCounterMeasuresInvoked: super::ULONGLONG,
    pub ullReserved: super::ULONGLONG,
    pub MacUcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub PhyCounters: [DOT11_PHY_FRAME_STATISTICS; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_STATISTICS_REVISION_1: i32 = 1;
#[cfg(feature = "shared")]
pub const DOT11_STATISTICS_UNKNOWN: super::ULONGLONG = 18446744073709551615;
pub const DOT11_STATUS_AP_JOIN_CONFIRM: i32 = 5;
pub const DOT11_STATUS_AUTH_FAILED: i32 = 131072;
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: i32 = 32768;
pub const DOT11_STATUS_AUTH_VERIFIED: i32 = 65536;
pub const DOT11_STATUS_ENCRYPTION_FAILED: i32 = 512;
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: i32 = 256;
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: i32 = 16384;
pub const DOT11_STATUS_ICV_VERIFIED: i32 = 2048;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_STATUS_INDICATION {
    pub uStatusType: super::ULONG,
    pub ndisStatus: super::NDIS_STATUS,
}
pub const DOT11_STATUS_JOIN_CONFIRM: i32 = 2;
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: i32 = 6;
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: i32 = 8192;
pub const DOT11_STATUS_PACKET_REASSEMBLED: i32 = 4096;
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: i32 = 262144;
pub const DOT11_STATUS_RESET_CONFIRM: i32 = 4;
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: i32 = 2;
pub const DOT11_STATUS_SCAN_CONFIRM: i32 = 1;
pub const DOT11_STATUS_START_CONFIRM: i32 = 3;
pub const DOT11_STATUS_SUCCESS: i32 = 1;
pub const DOT11_STATUS_UNAVAILABLE_BSS: i32 = 128;
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: i32 = 16;
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: i32 = 32;
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: i32 = 4;
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: i32 = 8;
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: i32 = 1024;
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: i32 = 64;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_STOP_AP_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ulReason: super::ULONG,
}
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: i32 = 1;
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: i32 = 3;
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: i32 = 2;
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: i32 = 1;
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_ANTENNA {
    pub uAntennaListIndex: super::ULONG,
    pub bSupportedAntenna: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_ANTENNA_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11SupportedAntenna: [DOT11_SUPPORTED_ANTENNA; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE {
    pub ucSupportedTxDataRatesValue: [super::UCHAR; 8],
    pub ucSupportedRxDataRatesValue: [super::UCHAR; 8],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type DOT11_SUPPORTED_DATA_RATES_VALUE_V1 = DOT11_SUPPORTED_DATA_RATES_VALUE_V2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    pub ucSupportedTxDataRatesValue: [super::UCHAR; 255],
    pub ucSupportedRxDataRatesValue: [super::UCHAR; 255],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL {
    pub uChannel: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11SupportedDSSSChannel: [DOT11_SUPPORTED_DSSS_CHANNEL; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY {
    pub uCenterFrequency: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11SupportedOFDMFrequency: [DOT11_SUPPORTED_OFDM_FREQUENCY; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_PHY_TYPES {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11PHYType: [DOT11_PHY_TYPE; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_SUPPORTED_POWER_LEVELS {
    pub uNumOfSupportedPowerLevels: super::ULONG,
    pub uTxPowerLevelValues: [super::ULONG; 8],
}
#[cfg(feature = "shared")]
impl Default for DOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_TEMP_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bDefaultKeyFailure: super::BOOLEAN,
    pub uKeyIndex: super::ULONG,
    pub PeerMac: DOT11_MAC_ADDRESS,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_UPDATE_IE {
    pub dot11UpdateIEOp: DOT11_UPDATE_IE_OP,
    pub uBufferLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_UPDATE_IE_OP = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_VWIFI_ATTRIBUTES {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uTotalNumOfEntries: super::ULONG,
    pub Combinations: [DOT11_VWIFI_COMBINATION; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: super::ULONG,
    pub uNumAdhoc: super::ULONG,
    pub uNumSoftAP: super::ULONG,
}
pub const DOT11_VWIFI_COMBINATION_REVISION_1: i32 = 1;
pub const DOT11_VWIFI_COMBINATION_REVISION_2: i32 = 2;
pub const DOT11_VWIFI_COMBINATION_REVISION_3: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION_V2 {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: super::ULONG,
    pub uNumAdhoc: super::ULONG,
    pub uNumSoftAP: super::ULONG,
    pub uNumVirtualStation: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION_V3 {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: super::ULONG,
    pub uNumAdhoc: super::ULONG,
    pub uNumSoftAP: super::ULONG,
    pub uNumVirtualStation: super::ULONG,
    pub uNumWFDGroup: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WEP_OFFLOAD {
    pub uReserved: super::ULONG,
    pub hOffloadContext: super::HANDLE,
    pub hOffload: super::HANDLE,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: super::ULONG,
    pub bRowIsOutbound: super::BOOLEAN,
    pub bUseDefault: super::BOOLEAN,
    pub uFlags: super::ULONG,
    pub ucMacAddress: [super::UCHAR; 6],
    pub uNumOfRWsOnPeer: super::ULONG,
    pub uNumOfRWsOnMe: super::ULONG,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [super::USHORT; 16],
    pub usKeyLength: super::USHORT,
    pub ucKey: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WEP_UPLOAD {
    pub uReserved: super::ULONG,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::HANDLE,
    pub uNumOfRWsUsed: super::ULONG,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [super::USHORT; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_ADDITIONAL_IE {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: super::ULONG,
    pub uBeaconIEsLength: super::ULONG,
    pub uProbeResponseIEsOffset: super::ULONG,
    pub uProbeResponseIEsLength: super::ULONG,
    pub uDefaultRequestIEsOffset: super::ULONG,
    pub uDefaultRequestIEsLength: super::ULONG,
}
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    pub AdvertisementID: super::ULONG,
    pub ConfigMethods: super::USHORT,
    pub ServiceNameLength: super::UCHAR,
    pub ServiceName: [super::UCHAR; 255],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST {
    pub ServiceCount: super::USHORT,
    pub AdvertisedService: [DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_ADVERTISEMENT_ID {
    pub AdvertisementID: super::ULONG,
    pub ServiceAddress: DOT11_MAC_ADDRESS,
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: i32 = 21;
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: i32 = 63;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_ATTRIBUTES {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumConcurrentGORole: super::ULONG,
    pub uNumConcurrentClientRole: super::ULONG,
    pub WPSVersionsSupported: super::ULONG,
    pub bServiceDiscoverySupported: super::BOOLEAN,
    pub bClientDiscoverabilitySupported: super::BOOLEAN,
    pub bInfrastructureManagementSupported: super::BOOLEAN,
    pub uMaxSecondaryDeviceTypeListSize: super::ULONG,
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub uInterfaceAddressListCount: super::ULONG,
    pub pInterfaceAddressList: PDOT11_MAC_ADDRESS,
    pub uNumSupportedCountryOrRegionStrings: super::ULONG,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uDiscoveryFilterListSize: super::ULONG,
    pub uGORoleClientTableSize: super::ULONG,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_CHANNEL {
    pub CountryRegionString: DOT11_COUNTRY_OR_REGION_STRING,
    pub OperatingClass: super::UCHAR,
    pub ChannelNumber: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT {
    pub GOTimeout: super::UCHAR,
    pub ClientTimeout: super::UCHAR,
}
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: i32 = 16;
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bServiceDiscoveryEnabled: super::BOOLEAN,
    pub bClientDiscoverabilityEnabled: super::BOOLEAN,
    pub bConcurrentOperationSupported: super::BOOLEAN,
    pub bInfrastructureManagementEnabled: super::BOOLEAN,
    pub bDeviceLimitReached: super::BOOLEAN,
    pub bInvitationProcedureEnabled: super::BOOLEAN,
    pub WPSVersionsEnabled: super::ULONG,
}
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: i32 = 1;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: i32 = 2;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: i32 = 16;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: i32 = 8;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: i32 = 32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: i32 = 64;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: i32 = 128;
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy)]
pub struct DOT11_WFD_DEVICE_ENTRY {
    pub uPhyId: super::ULONG,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::DOT11_BSS_TYPE,
    pub TransmitterAddress: DOT11_MAC_ADDRESS,
    pub lRSSI: super::LONG,
    pub uLinkQuality: super::ULONG,
    pub usBeaconPeriod: super::USHORT,
    pub ullTimestamp: super::ULONGLONG,
    pub ullBeaconHostTimestamp: super::ULONGLONG,
    pub ullProbeResponseHostTimestamp: super::ULONGLONG,
    pub usCapabilityInformation: super::USHORT,
    pub uBeaconIEsOffset: super::ULONG,
    pub uBeaconIEsLength: super::ULONG,
    pub uProbeResponseIEsOffset: super::ULONG,
    pub uProbeResponseIEsLength: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: i32 = 24;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_DEVICE_INFO {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub ConfigMethods: super::USHORT,
    pub PrimaryDeviceType: DOT11_WFD_DEVICE_TYPE,
    pub DeviceName: DOT11_WPS_DEVICE_NAME,
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub ChannelNumber: super::UCHAR,
}
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: i32 = 1;
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_DEVICE_TYPE {
    pub CategoryID: super::USHORT,
    pub SubCategoryID: super::USHORT,
    pub OUI: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: i32 = 128;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub Status: super::NDIS_STATUS,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub uListOffset: super::ULONG,
    pub uListLength: super::ULONG,
}
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER {
    pub DeviceID: DOT11_MAC_ADDRESS,
    pub ucBitmask: super::UCHAR,
    pub GroupSSID: super::DOT11_SSID,
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_DISCOVER_REQUEST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub DiscoverType: DOT11_WFD_DISCOVER_TYPE,
    pub ScanType: DOT11_WFD_SCAN_TYPE,
    pub uDiscoverTimeout: super::ULONG,
    pub uDeviceFilterListOffset: super::ULONG,
    pub uNumDeviceFilters: super::ULONG,
    pub uIEsOffset: super::ULONG,
    pub uIEsLength: super::ULONG,
    pub bForceScanLegacyNetworks: super::BOOLEAN,
}
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: i32 = 1;
pub type DOT11_WFD_DISCOVER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_GO_INTENT {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl DOT11_WFD_GO_INTENT {
    pub fn TieBreaker(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_TieBreaker(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Intent(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Intent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
pub type DOT11_WFD_GROUP_CAPABILITY = u8;
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: i32 = 16;
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: i32 = 128;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: i32 = 4;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: i32 = 1;
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: i32 = 8;
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: i32 = 64;
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: i32 = 0;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: i32 = 2;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: i32 = 32;
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: i32 = 128;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wlan"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_GROUP_ID {
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub SSID: super::DOT11_SSID,
}
#[cfg(all(feature = "shared", feature = "wlan"))]
impl Default for DOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub GOOperatingChannel: DOT11_WFD_CHANNEL,
    pub GOConfigTime: super::ULONG,
    pub bInGroupFormation: super::BOOLEAN,
    pub bWaitForWPSReady: super::BOOLEAN,
}
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::BOOLEAN,
    pub bCrossConnectionSupported: super::BOOLEAN,
    pub bPersistentReconnectSupported: super::BOOLEAN,
    pub bGroupFormationEnabled: super::BOOLEAN,
    pub uMaximumGroupLimit: super::ULONG,
}
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: i32 = 1;
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::BOOLEAN,
    pub bCrossConnectionSupported: super::BOOLEAN,
    pub bPersistentReconnectSupported: super::BOOLEAN,
    pub bGroupFormationEnabled: super::BOOLEAN,
    pub uMaximumGroupLimit: super::ULONG,
    pub bEapolKeyIpAddressAllocationSupported: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_GROUP_START_PARAMETERS {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub AdvertisedOperatingChannel: DOT11_WFD_CHANNEL,
}
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WFD_INVITATION_FLAGS {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl DOT11_WFD_INVITATION_FLAGS {
    pub fn InvitationType(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_InvitationType(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
pub type DOT11_WFD_MINOR_REASON_CODE = u8;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: i32 = 1;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: i32 = 4;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: i32 = 2;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: i32 = 3;
pub const DOT11_WFD_MINOR_REASON_SUCCESS: i32 = 0;
pub type DOT11_WFD_SCAN_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "objectheader", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    pub Header: super::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub SecondaryDeviceTypes: [DOT11_WFD_DEVICE_TYPE; 1],
}
#[cfg(all(feature = "objectheader", feature = "shared"))]
impl Default for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: i32 = 1;
pub type DOT11_WFD_SERVICE_HASH = [u8; 6];
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_SERVICE_HASH_LIST {
    pub ServiceHashCount: super::USHORT,
    pub ServiceHash: [DOT11_WFD_SERVICE_HASH; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: i32 = 65535;
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: i32 = 255;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_SESSION_ID {
    pub SessionID: super::ULONG,
    pub SessionAddress: DOT11_MAC_ADDRESS,
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WFD_SESSION_INFO {
    pub uSessionInfoLength: super::USHORT,
    pub ucSessionInfo: [super::UCHAR; 144],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: i32 = 144;
pub type DOT11_WFD_STATUS_CODE = u8;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: i32 = 2;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: i32 = 10;
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: i32 = 1;
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: i32 = 4;
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: i32 = 3;
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: i32 = 9;
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: i32 = 7;
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: i32 = 6;
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: i32 = 11;
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: i32 = 5;
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: i32 = 8;
pub const DOT11_WFD_STATUS_SUCCESS: i32 = 0;
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: i32 = 12;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WME_AC_PARAMETERS {
    pub ucAccessCategoryIndex: super::UCHAR,
    pub ucAIFSN: super::UCHAR,
    pub ucECWmin: super::UCHAR,
    pub ucECWmax: super::UCHAR,
    pub usTXOPLimit: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WME_AC_PARAMETERS_LIST {
    pub uNumOfEntries: super::ULONG,
    pub uTotalNumOfEntries: super::ULONG,
    pub dot11WMEACParameters: [DOT11_WME_AC_PARAMETERS; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WME_AC_PARAMETERS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WME_PACKET: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WME_UPDATE_IE {
    pub uParamElemMinBeaconIntervals: super::ULONG,
    pub uWMEInfoElemOffset: super::ULONG,
    pub uWMEInfoElemLength: super::ULONG,
    pub uWMEParamElemOffset: super::ULONG,
    pub uWMEParamElemLength: super::ULONG,
    pub ucBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOT11_WPA_TSC {
    pub uReserved: super::ULONG,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::HANDLE,
    pub dot11IV48Counter: DOT11_IV48_COUNTER,
}
pub type DOT11_WPS_CONFIG_METHOD = i32;
pub const DOT11_WPS_CONFIG_METHOD_DISPLAY: DOT11_WPS_CONFIG_METHOD = 8;
pub const DOT11_WPS_CONFIG_METHOD_KEYPAD: DOT11_WPS_CONFIG_METHOD = 256;
pub const DOT11_WPS_CONFIG_METHOD_NFC_INTERFACE: DOT11_WPS_CONFIG_METHOD = 64;
pub const DOT11_WPS_CONFIG_METHOD_NFC_TAG: DOT11_WPS_CONFIG_METHOD = 32;
pub const DOT11_WPS_CONFIG_METHOD_NULL: DOT11_WPS_CONFIG_METHOD = 0;
pub const DOT11_WPS_CONFIG_METHOD_PUSHBUTTON: DOT11_WPS_CONFIG_METHOD = 128;
pub const DOT11_WPS_CONFIG_METHOD_WFDS_DEFAULT: DOT11_WPS_CONFIG_METHOD = 4096;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOT11_WPS_DEVICE_NAME {
    pub uDeviceNameLength: super::ULONG,
    pub ucDeviceName: [super::UCHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for DOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: i32 = 32;
pub type DOT11_WPS_DEVICE_PASSWORD_ID = i32;
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: i32 = 32;
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: i32 = 32;
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: i32 = 8;
pub const DOT11_WPS_PASSWORD_ID_DEFAULT: DOT11_WPS_DEVICE_PASSWORD_ID = 0;
pub const DOT11_WPS_PASSWORD_ID_MACHINE_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 2;
pub const DOT11_WPS_PASSWORD_ID_NFC_CONNECTION_HANDOVER: DOT11_WPS_DEVICE_PASSWORD_ID = 7;
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MAX: DOT11_WPS_DEVICE_PASSWORD_ID = 65535;
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MIN: DOT11_WPS_DEVICE_PASSWORD_ID = 16;
pub const DOT11_WPS_PASSWORD_ID_PUSHBUTTON: DOT11_WPS_DEVICE_PASSWORD_ID = 4;
pub const DOT11_WPS_PASSWORD_ID_REGISTRAR_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 5;
pub const DOT11_WPS_PASSWORD_ID_REKEY: DOT11_WPS_DEVICE_PASSWORD_ID = 3;
pub const DOT11_WPS_PASSWORD_ID_USER_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 1;
pub const DOT11_WPS_PASSWORD_ID_WFD_SERVICES: DOT11_WPS_DEVICE_PASSWORD_ID = 8;
pub const DOT11_WPS_VERSION_1_0: i32 = 1;
pub const DOT11_WPS_VERSION_2_0: i32 = 2;
pub const GUID_NWF_ACTIVE_PHY_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d3_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_AP_JOIN_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ASSOCIATION_PARAMS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4dd_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ATIM_WINDOW: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a448_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_AUTO_CONFIG_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4b6_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_BEACON_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a459_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CCA_MODE_SUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a474_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CCA_WATCHDOG_COUNT_MAX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a478_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CCA_WATCHDOG_COUNT_MIN: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CCA_WATCHDOG_TIMER_MAX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a477_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CCA_WATCHDOG_TIMER_MIN: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a479_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CFP_MAX_DURATION: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a456_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CFP_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a455_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CF_POLLABLE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a454_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CHANNEL_AGILITY_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a486_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CHANNEL_AGILITY_PRESENT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a485_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CIPHER_DEFAULT_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c9_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CIPHER_DEFAULT_KEY_ID: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c8_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CIPHER_KEY_MAPPING_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4ca_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CONNECT_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4bf_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_COUNTERS_ENTRY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a463_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_COUNTRY_STRING: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_ADDRESS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a840_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_CCA_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a475_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_CHANNEL: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a473_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_CHANNEL_NUMBER: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_DWELL_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_FREQUENCY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a480_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_INDEX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a472_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_OFFLOAD_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a43f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_OPERATION_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a446_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_OPTIONAL_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a451_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_PACKET_FILTER: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a447_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_PATTERN: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a471_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_PHY_ID: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d0_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_PHY_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_REG_DOMAIN: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a465_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_RX_ANTENNA: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a469_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_SET: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a470_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_TX_ANTENNA: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a467_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_CURRENT_TX_POWER_LEVEL: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DATA_RATE_MAPPING_TABLE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d5_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DEFAULT_WEP_OFFLOAD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a442_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DEFAULT_WEP_UPLOAD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a443_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DESIRED_BSSID_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4bc_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DESIRED_BSS_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4bd_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DESIRED_COUNTRY_OR_REGION_STRING: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d7_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DESIRED_PHY_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4cf_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DESIRED_SSID_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4ba_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DISCONNECT_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4cc_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DIVERSITY_SELECTION_RX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DIVERSITY_SUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a468_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DSSS_OFDM_OPTION_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DSSS_OFDM_OPTION_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_DTIM_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ED_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a476_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EHCC_CAPABILITY_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EHCC_CAPABILITY_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EHCC_PRIME_RADIX: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ENABLED_AUTHENTICATION_ALGORITHM: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c3_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ENABLED_MULTICAST_CIPHER_ALGORITHM: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c7_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ENABLED_UNICAST_CIPHER_ALGORITHM: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c5_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ENUM_ASSOCIATION_INFO: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4cb_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ENUM_BSS_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4b7_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ERP_PBCC_OPTION_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_ERP_PBCC_OPTION_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EXCLUDED_MAC_ADDRESS_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4bb_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EXCLUDE_UNENCRYPTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c0_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_EXTSTA_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d4_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_FLUSH_BSS_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4b8_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_FRAGMENTATION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a460_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_FREQUENCY_BANDS_SUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a482_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HIDDEN_NETWORK_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4dc_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HOPPING_PATTERN: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a495_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HOP_ALGORITHM_ADOPTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a490_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HOP_MODULUS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a493_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HOP_OFFSET: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a494_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HOP_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_HR_CCA_MODE_SUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a487_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_IBSS_PARAMS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d9_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_JOIN_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_LONG_RETRY_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAC_ADDRESS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAXIMUM_LIST_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a843_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAX_DWELL_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAX_MAC_ADDRESS_STATES: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a2_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAX_RECEIVE_LIFETIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a462_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MAX_TRANSMIT_MSDU_LIFETIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a461_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MEDIA_STREAMING_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d1_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MEDIUM_OCCUPANCY_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a453_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MPDU_MAX_LENGTH: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a444_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MULTICAST_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a842_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MULTI_DOMAIN_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a48b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MULTI_DOMAIN_CAPABILITY_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a489_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a488_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_NIC_POWER_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_NIC_SPECIFIC_EXTENSION: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a49a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_NUMBER_OF_HOPPING_SETS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a492_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_OFFLOAD_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a43e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_OPERATIONAL_RATE_SET: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a458_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_OPERATION_MODE_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a445_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_OPTIONAL_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a450_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PBCC_OPTION_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a484_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PERMANENT_ADDRESS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a841_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PHY_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4ce_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PMKID_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4be_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PORT_STATE_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d8_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_POWER_MGMT_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a457_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_POWER_MGMT_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4b9_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_PRIVACY_EXEMPTION_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c2_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_QOS_PARAMS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4da_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_QOS_TX_DURATION: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a9_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_QOS_TX_MEDIUM_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4aa_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_QOS_TX_QUEUES_SUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a8_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RANDOM_TABLE_FIELD_NUMBER: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a496_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RANDOM_TABLE_FLAG: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a491_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RECV_SENSITIVITY_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a3_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_REG_DOMAINS_SUPPORT_VALUE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RESET_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RF_USAGE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a499_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RSSI_RANGE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a498_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_RTS_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SAFE_MODE_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4db_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SCAN_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a449_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SHORT_PREAMBLE_OPTION_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a483_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SHORT_RETRY_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45e_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SHORT_SLOT_TIME_OPTION_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a1_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a0_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_START_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_STATION_ID: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a452_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_STATISTICS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c1_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_COUNTRY_OR_REGION_STRING: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d6_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_DATA_RATES_VALUE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47f_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_DSSS_CHANNEL_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4ac_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_MULTICAST_ALGORITHM_PAIR: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c6_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_OFDM_FREQUENCY_LIST: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4ab_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_PHY_TYPES: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a464_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_POWER_LEVELS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a46a_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_RX_ANTENNA: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_TX_ANTENNA: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a47c_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_SUPPORTED_UNICAST_ALGORITHM_PAIR: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4c4_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_TEMP_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a466_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_TI_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a481_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_UNICAST_USE_GROUP_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4cd_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_UNREACHABLE_DETECTION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4d2_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_UPDATE_IE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a44d_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WEP_ICV_ERROR_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a45b_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WEP_OFFLOAD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a440_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WEP_UPLOAD: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a441_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WME_AC_PARAMETERS: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a6_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WME_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a5_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WME_IMPLEMENTED: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a4_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WME_UPDATE_IE: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a4a7_c45f_4039_9fe6_d08cb057184c);
pub const GUID_NWF_WPA_TSC: windows_core::GUID = windows_core::GUID::from_u128(0x6cb9a497_c45f_4039_9fe6_d08cb057184c);
pub const MAX_NUM_SUPPORTED_RATES: i32 = 8;
pub const MAX_NUM_SUPPORTED_RATES_V2: i32 = 255;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: i32 = 4;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_MGMT: u32 = 1048576;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_CTRL: u32 = 16777216;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: i32 = 8;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_MGMT: u32 = 262144;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_CTRL: u32 = 8388608;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: i32 = 1;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_MGMT: u32 = 131072;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: i32 = 2;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_MGMT: u32 = 524288;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_CTRL: u32 = 33554432;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: i32 = 32;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_MGMT: u32 = 2097152;
pub const NDIS_PACKET_TYPE_802_11_RAW_DATA: u32 = 65536;
pub const NDIS_PACKET_TYPE_802_11_RAW_MGMT: u32 = 4194304;
pub const NDIS_PACKET_TYPE_ALL_802_11_FILTERS: u32 = 67043375;
pub const NDIS_PACKET_TYPE_MEDIA_SPECIFIC_MASK: u32 = 268369920;
#[cfg(target_arch = "x86")]
pub const NDIS_SIZEOF_DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NDIS_SIZEOF_DOT11_EXTAP_ATTRIBUTES_REVISION_1: u64 = 80;
pub const NWF_EXTAP_OID: u32 = 3;
pub const NWF_MANDATORY_OID: u32 = 1;
pub const NWF_MANUFACTURING_OID: u32 = 8;
pub const NWF_OPERATIONAL_OID: u32 = 1;
pub const NWF_OPTIONAL_OID: u32 = 2;
pub const NWF_POWER_SAVE_OID: u32 = 7;
pub const NWF_STATISTICS_OID: u32 = 2;
pub const NWF_VWIFI_OID: u32 = 4;
pub const NWF_WFD_DEVICE_OID: u32 = 5;
pub const NWF_WFD_ROLE_OID: u32 = 6;
pub const OID_DOT11_ACTIVE_PHY_LIST: u32 = 234946965;
pub const OID_DOT11_ADDITIONAL_IE: u32 = 235077895;
pub const OID_DOT11_AP_JOIN_REQUEST: i32 = 218170205;
pub const OID_DOT11_ASSOCIATION_PARAMS: u32 = 234946975;
pub const OID_DOT11_ATIM_WINDOW: i32 = 218170122;
pub const OID_DOT11_AUTO_CONFIG_ENABLED: u32 = 234946936;
pub const OID_DOT11_AVAILABLE_CHANNEL_LIST: u32 = 235077891;
pub const OID_DOT11_AVAILABLE_FREQUENCY_LIST: u32 = 235077892;
pub const OID_DOT11_BEACON_PERIOD: i32 = 218170139;
pub const OID_DOT11_CCA_MODE_SUPPORTED: i32 = 218170166;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: i32 = 218170170;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: i32 = 218170172;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: i32 = 218170169;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: i32 = 218170171;
pub const OID_DOT11_CFP_MAX_DURATION: i32 = 218170136;
pub const OID_DOT11_CFP_PERIOD: i32 = 218170135;
pub const OID_DOT11_CF_POLLABLE: i32 = 218170134;
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: i32 = 218170184;
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: i32 = 218170183;
pub const OID_DOT11_CIPHER_DEFAULT_KEY: u32 = 234946955;
pub const OID_DOT11_CIPHER_DEFAULT_KEY_ID: u32 = 234946954;
pub const OID_DOT11_CIPHER_KEY_MAPPING_KEY: u32 = 234946956;
pub const OID_DOT11_CONNECT_REQUEST: u32 = 234946945;
pub const OID_DOT11_COUNTERS_ENTRY: i32 = 218170149;
pub const OID_DOT11_COUNTRY_STRING: i32 = 218170188;
pub const OID_DOT11_CREATE_MAC: u32 = 235143425;
pub const OID_DOT11_CURRENT_ADDRESS: i32 = 218171138;
pub const OID_DOT11_CURRENT_CCA_MODE: i32 = 218170167;
pub const OID_DOT11_CURRENT_CHANNEL: i32 = 218170165;
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: i32 = 218170159;
pub const OID_DOT11_CURRENT_DWELL_TIME: i32 = 218170161;
pub const OID_DOT11_CURRENT_FREQUENCY: i32 = 218170178;
pub const OID_DOT11_CURRENT_INDEX: i32 = 218170164;
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: i32 = 218170113;
pub const OID_DOT11_CURRENT_OPERATION_MODE: i32 = 218170120;
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: i32 = 218170131;
pub const OID_DOT11_CURRENT_PACKET_FILTER: i32 = 218170121;
pub const OID_DOT11_CURRENT_PATTERN: i32 = 218170163;
pub const OID_DOT11_CURRENT_PHY_ID: u32 = 234946962;
pub const OID_DOT11_CURRENT_PHY_TYPE: i32 = 218170124;
pub const OID_DOT11_CURRENT_REG_DOMAIN: i32 = 218170151;
pub const OID_DOT11_CURRENT_RX_ANTENNA: i32 = 218170155;
pub const OID_DOT11_CURRENT_SET: i32 = 218170162;
pub const OID_DOT11_CURRENT_TX_ANTENNA: i32 = 218170153;
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: i32 = 218170157;
pub const OID_DOT11_DATA_RATE_MAPPING_TABLE: u32 = 234946967;
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: i32 = 218170116;
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: i32 = 218170117;
pub const OID_DOT11_DELETE_MAC: u32 = 235143426;
pub const OID_DOT11_DESIRED_BSSID_LIST: u32 = 234946942;
pub const OID_DOT11_DESIRED_BSS_TYPE: u32 = 234946943;
pub const OID_DOT11_DESIRED_COUNTRY_OR_REGION_STRING: u32 = 234946969;
pub const OID_DOT11_DESIRED_PHY_LIST: u32 = 234946961;
pub const OID_DOT11_DESIRED_SSID_LIST: u32 = 234946940;
pub const OID_DOT11_DISASSOCIATE_PEER_REQUEST: u32 = 235077893;
pub const OID_DOT11_DISCONNECT_REQUEST: u32 = 234946958;
pub const OID_DOT11_DIVERSITY_SELECTION_RX: i32 = 218170176;
pub const OID_DOT11_DIVERSITY_SUPPORT: i32 = 218170154;
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: i32 = 218170209;
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: i32 = 218170208;
pub const OID_DOT11_DTIM_PERIOD: i32 = 218170140;
pub const OID_DOT11_ED_THRESHOLD: i32 = 218170168;
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: i32 = 218170193;
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: i32 = 218170192;
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: i32 = 218170191;
pub const OID_DOT11_EHCC_PRIME_RADIX: i32 = 218170190;
pub const OID_DOT11_ENABLED_AUTHENTICATION_ALGORITHM: u32 = 234946949;
pub const OID_DOT11_ENABLED_MULTICAST_CIPHER_ALGORITHM: u32 = 234946953;
pub const OID_DOT11_ENABLED_UNICAST_CIPHER_ALGORITHM: u32 = 234946951;
pub const OID_DOT11_ENUM_ASSOCIATION_INFO: u32 = 234946957;
pub const OID_DOT11_ENUM_BSS_LIST: u32 = 234946937;
pub const OID_DOT11_ENUM_PEER_INFO: u32 = 235077896;
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: i32 = 218170207;
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: i32 = 218170206;
pub const OID_DOT11_EXCLUDED_MAC_ADDRESS_LIST: u32 = 234946941;
pub const OID_DOT11_EXCLUDE_UNENCRYPTED: u32 = 234946946;
pub const OID_DOT11_EXTSTA_CAPABILITY: u32 = 234946966;
pub const OID_DOT11_FLUSH_BSS_LIST: u32 = 234946938;
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: i32 = 218170146;
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: i32 = 218170180;
pub const OID_DOT11_HARDWARE_PHY_STATE: u32 = 234946960;
pub const OID_DOT11_HIDDEN_NETWORK_ENABLED: u32 = 234946974;
pub const OID_DOT11_HOPPING_PATTERN: i32 = 218170199;
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: i32 = 218170194;
pub const OID_DOT11_HOP_MODULUS: i32 = 218170197;
pub const OID_DOT11_HOP_OFFSET: i32 = 218170198;
pub const OID_DOT11_HOP_TIME: i32 = 218170158;
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: i32 = 218170185;
pub const OID_DOT11_IBSS_PARAMS: u32 = 234946971;
pub const OID_DOT11_INCOMING_ASSOCIATION_DECISION: u32 = 235077894;
pub const OID_DOT11_JOIN_REQUEST: i32 = 218170125;
pub const OID_DOT11_LONG_RETRY_LIMIT: i32 = 218170145;
pub const OID_DOT11_MAC_ADDRESS: i32 = 218170142;
pub const OID_DOT11_MANUFACTURING_TEST: u32 = 235405825;
pub const OID_DOT11_MAXIMUM_LIST_SIZE: i32 = 218171141;
pub const OID_DOT11_MAX_DWELL_TIME: i32 = 218170160;
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: i32 = 218170212;
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: i32 = 218170148;
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: i32 = 218170147;
pub const OID_DOT11_MEDIA_STREAMING_ENABLED: u32 = 234946963;
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: i32 = 218170133;
pub const OID_DOT11_MPDU_MAX_LENGTH: i32 = 218170118;
pub const OID_DOT11_MULTICAST_LIST: i32 = 218171140;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: i32 = 218170189;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: i32 = 218170187;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: i32 = 218170186;
pub const OID_DOT11_NDIS_START: i32 = 218170112;
pub const OID_DOT11_NIC_POWER_STATE: i32 = 218170129;
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: i32 = 218170204;
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: i32 = 218170196;
pub const OID_DOT11_OFFLOAD_CAPABILITY: i32 = 218170112;
pub const OID_DOT11_OFFLOAD_NETWORK_LIST: u32 = 235340035;
pub const OID_DOT11_OPERATIONAL_RATE_SET: i32 = 218170138;
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: i32 = 218170119;
pub const OID_DOT11_OPTIONAL_CAPABILITY: i32 = 218170130;
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: i32 = 218170182;
pub const OID_DOT11_PERMANENT_ADDRESS: i32 = 218171139;
pub const OID_DOT11_PMKID_LIST: u32 = 234947200;
pub const OID_DOT11_PORT_STATE_NOTIFICATION: u32 = 234947226;
pub const OID_DOT11_POWER_MGMT_MODE: i32 = 218170137;
pub const OID_DOT11_POWER_MGMT_MODE_AUTO_ENABLED: u32 = 235340033;
pub const OID_DOT11_POWER_MGMT_MODE_STATUS: u32 = 235340034;
pub const OID_DOT11_POWER_MGMT_REQUEST: u32 = 234946939;
pub const OID_DOT11_PREFERRED_MAC: u32 = 235143427;
pub const OID_DOT11_PRIVACY_EXEMPTION_LIST: u32 = 234946948;
pub const OID_DOT11_PRIVATE_OIDS_START: i32 = 218171136;
pub const OID_DOT11_QOS_PARAMS: u32 = 234947228;
pub const OID_DOT11_QOS_TX_DURATION: i32 = 218170219;
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: i32 = 218170220;
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: i32 = 218170218;
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: i32 = 218170200;
pub const OID_DOT11_RANDOM_TABLE_FLAG: i32 = 218170195;
pub const OID_DOT11_RECV_SENSITIVITY_LIST: i32 = 218170213;
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: i32 = 218170173;
pub const OID_DOT11_RESET_REQUEST: i32 = 218170128;
pub const OID_DOT11_RF_USAGE: i32 = 218170203;
pub const OID_DOT11_RSSI_RANGE: i32 = 218170202;
pub const OID_DOT11_RTS_THRESHOLD: i32 = 218170143;
pub const OID_DOT11_SAFE_MODE_ENABLED: u32 = 234946973;
pub const OID_DOT11_SAFE_MODE_HT_ENABLED: u32 = 234946976;
pub const OID_DOT11_SCAN_REQUEST: i32 = 218170123;
pub const OID_DOT11_SET_FT_REASSOCIATION_PARAMETERS: u32 = 234946920;
pub const OID_DOT11_SET_NWF_PMKID_LIST: u32 = 234946933;
pub const OID_DOT11_SET_SAE_AUTH_PARAMS: u32 = 234946930;
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: i32 = 218170181;
pub const OID_DOT11_SHORT_RETRY_LIMIT: i32 = 218170144;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: i32 = 218170211;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: i32 = 218170210;
pub const OID_DOT11_START_AP_REQUEST: u32 = 235077890;
pub const OID_DOT11_START_REQUEST: i32 = 218170126;
pub const OID_DOT11_STATION_ID: i32 = 218170132;
pub const OID_DOT11_STATISTICS: u32 = 235012483;
pub const OID_DOT11_SUPPORTED_COUNTRY_OR_REGION_STRING: u32 = 234946968;
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: i32 = 218170177;
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: i32 = 218170222;
pub const OID_DOT11_SUPPORTED_MULTICAST_ALGORITHM_PAIR: u32 = 234946952;
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: i32 = 218170221;
pub const OID_DOT11_SUPPORTED_PHY_TYPES: i32 = 218170150;
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: i32 = 218170156;
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: i32 = 218170175;
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: i32 = 218170174;
pub const OID_DOT11_SUPPORTED_UNICAST_ALGORITHM_PAIR: u32 = 234946950;
pub const OID_DOT11_TEMP_TYPE: i32 = 218170152;
pub const OID_DOT11_TI_THRESHOLD: i32 = 218170179;
pub const OID_DOT11_UNICAST_USE_GROUP_ENABLED: u32 = 234946959;
pub const OID_DOT11_UNREACHABLE_DETECTION_THRESHOLD: u32 = 234946964;
pub const OID_DOT11_UPDATE_IE: i32 = 218170127;
pub const OID_DOT11_VIRTUAL_STATION_CAPABILITY: u32 = 235143684;
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: i32 = 218170141;
pub const OID_DOT11_WEP_OFFLOAD: i32 = 218170114;
pub const OID_DOT11_WEP_UPLOAD: i32 = 218170115;
pub const OID_DOT11_WFD_ADDITIONAL_IE: u32 = 235208968;
pub const OID_DOT11_WFD_CONNECT_TO_GROUP_REQUEST: u32 = 235274500;
pub const OID_DOT11_WFD_DESIRED_GROUP_ID: u32 = 235274497;
pub const OID_DOT11_WFD_DEVICE_CAPABILITY: u32 = 235208961;
pub const OID_DOT11_WFD_DEVICE_INFO: u32 = 235208963;
pub const OID_DOT11_WFD_DEVICE_LISTEN_CHANNEL: u32 = 235208980;
pub const OID_DOT11_WFD_DISCONNECT_FROM_GROUP_REQUEST: u32 = 235274501;
pub const OID_DOT11_WFD_DISCOVER_REQUEST: u32 = 235208965;
pub const OID_DOT11_WFD_ENABLE_HRDSSS_DEVICES: u32 = 235209235;
pub const OID_DOT11_WFD_ENUM_DEVICE_LIST: u32 = 235208966;
pub const OID_DOT11_WFD_FLUSH_DEVICE_LIST: u32 = 235208969;
pub const OID_DOT11_WFD_GET_DIALOG_TOKEN: u32 = 235208977;
pub const OID_DOT11_WFD_GROUP_JOIN_PARAMETERS: u32 = 235274502;
pub const OID_DOT11_WFD_GROUP_OWNER_CAPABILITY: u32 = 235208962;
pub const OID_DOT11_WFD_GROUP_START_PARAMETERS: u32 = 235274499;
pub const OID_DOT11_WFD_LISTEN_STATE_DISCOVERABILITY: u32 = 235208967;
pub const OID_DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST: u32 = 235208964;
pub const OID_DOT11_WFD_SEND_GO_NEGOTIATION_CONFIRMATION: u32 = 235208972;
pub const OID_DOT11_WFD_SEND_GO_NEGOTIATION_REQUEST: u32 = 235208970;
pub const OID_DOT11_WFD_SEND_GO_NEGOTIATION_RESPONSE: u32 = 235208971;
pub const OID_DOT11_WFD_SEND_INVITATION_REQUEST: u32 = 235208973;
pub const OID_DOT11_WFD_SEND_INVITATION_RESPONSE: u32 = 235208974;
pub const OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_REQUEST: u32 = 235208975;
pub const OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_RESPONSE: u32 = 235208976;
pub const OID_DOT11_WFD_START_GO_REQUEST: u32 = 235274498;
pub const OID_DOT11_WFD_STOP_DISCOVERY: u32 = 235208978;
pub const OID_DOT11_WME_AC_PARAMETERS: i32 = 218170216;
pub const OID_DOT11_WME_ENABLED: i32 = 218170215;
pub const OID_DOT11_WME_IMPLEMENTED: i32 = 218170214;
pub const OID_DOT11_WME_UPDATE_IE: i32 = 218170217;
pub const OID_DOT11_WPA_TSC: i32 = 218170201;
pub const OID_DOT11_WPS_ENABLED: u32 = 235077889;
pub type PCH_DESCRIPTION_TYPE = *mut CH_DESCRIPTION_TYPE;
pub type PDOT11_AC_PARAM = *mut DOT11_AC_PARAM;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_ADDITIONAL_IE = *mut DOT11_ADDITIONAL_IE;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_ANQP_QUERY_COMPLETE_PARAMETERS = *mut DOT11_ANQP_QUERY_COMPLETE_PARAMETERS;
pub type PDOT11_ANQP_QUERY_RESULT = *mut DOT11_ANQP_QUERY_RESULT;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_AP_JOIN_REQUEST = *mut DOT11_AP_JOIN_REQUEST;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_ASSOCIATION_COMPLETION_PARAMETERS = *mut DOT11_ASSOCIATION_COMPLETION_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_ASSOCIATION_INFO_EX = *mut DOT11_ASSOCIATION_INFO_EX;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_ASSOCIATION_INFO_LIST = *mut DOT11_ASSOCIATION_INFO_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_ASSOCIATION_PARAMS = *mut DOT11_ASSOCIATION_PARAMS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_ASSOCIATION_START_PARAMETERS = *mut DOT11_ASSOCIATION_START_PARAMETERS;
pub type PDOT11_ASSOCIATION_STATE = *mut DOT11_ASSOCIATION_STATE;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_AUTH_ALGORITHM_LIST = *mut DOT11_AUTH_ALGORITHM_LIST;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_AUTH_CIPHER_PAIR_LIST = *mut DOT11_AUTH_CIPHER_PAIR_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_AVAILABLE_CHANNEL_LIST = *mut DOT11_AVAILABLE_CHANNEL_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_AVAILABLE_FREQUENCY_LIST = *mut DOT11_AVAILABLE_FREQUENCY_LIST;
pub type PDOT11_BAND = *mut DOT11_BAND;
#[cfg(feature = "shared")]
pub type PDOT11_BSSID_CANDIDATE = *mut DOT11_BSSID_CANDIDATE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_BSSID_LIST = *mut DOT11_BSSID_LIST;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_BSS_DESCRIPTION = *mut DOT11_BSS_DESCRIPTION;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_BSS_ENTRY = *mut DOT11_BSS_ENTRY;
#[cfg(feature = "shared")]
pub type PDOT11_BSS_ENTRY_PHY_SPECIFIC_INFO = *mut DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_BYTE_ARRAY = *mut DOT11_BYTE_ARRAY;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_CAN_SUSTAIN_AP_PARAMETERS = *mut DOT11_CAN_SUSTAIN_AP_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_CHANNEL_HINT = *mut DOT11_CHANNEL_HINT;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_CIPHER_ALGORITHM_LIST = *mut DOT11_CIPHER_ALGORITHM_LIST;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_CIPHER_DEFAULT_KEY_VALUE = *mut DOT11_CIPHER_DEFAULT_KEY_VALUE;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE = *mut DOT11_CIPHER_KEY_MAPPING_KEY_VALUE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_CONNECTION_COMPLETION_PARAMETERS = *mut DOT11_CONNECTION_COMPLETION_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_CONNECTION_START_PARAMETERS = *mut DOT11_CONNECTION_START_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_COUNTERS_ENTRY = *mut DOT11_COUNTERS_ENTRY;
pub type PDOT11_COUNTRY_OR_REGION_STRING = *mut [u8; 3];
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_COUNTRY_OR_REGION_STRING_LIST = *mut DOT11_COUNTRY_OR_REGION_STRING_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_CURRENT_OFFLOAD_CAPABILITY = *mut DOT11_CURRENT_OFFLOAD_CAPABILITY;
#[cfg(feature = "shared")]
pub type PDOT11_CURRENT_OPERATION_MODE = *mut DOT11_CURRENT_OPERATION_MODE;
#[cfg(feature = "shared")]
pub type PDOT11_CURRENT_OPTIONAL_CAPABILITY = *mut DOT11_CURRENT_OPTIONAL_CAPABILITY;
#[cfg(feature = "shared")]
pub type PDOT11_DATA_RATE_MAPPING_ENTRY = *mut DOT11_DATA_RATE_MAPPING_ENTRY;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_DATA_RATE_MAPPING_TABLE = *mut DOT11_DATA_RATE_MAPPING_TABLE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_DEFAULT_WEP_OFFLOAD = *mut DOT11_DEFAULT_WEP_OFFLOAD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_DEFAULT_WEP_UPLOAD = *mut DOT11_DEFAULT_WEP_UPLOAD;
pub type PDOT11_DIRECTION = *mut DOT11_DIRECTION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_DISASSOCIATE_PEER_REQUEST = *mut DOT11_DISASSOCIATE_PEER_REQUEST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_DISASSOCIATION_PARAMETERS = *mut DOT11_DISASSOCIATION_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_DIVERSITY_SELECTION_RX = *mut DOT11_DIVERSITY_SELECTION_RX;
#[cfg(feature = "shared")]
pub type PDOT11_DIVERSITY_SELECTION_RX_LIST = *mut DOT11_DIVERSITY_SELECTION_RX_LIST;
pub type PDOT11_DIVERSITY_SUPPORT = *mut DOT11_DIVERSITY_SUPPORT;
pub type PDOT11_DS_INFO = *mut DOT11_DS_INFO;
#[cfg(feature = "shared")]
pub type PDOT11_ENCAP_ENTRY = *mut DOT11_ENCAP_ENTRY;
#[cfg(feature = "shared")]
pub type PDOT11_ERP_PHY_ATTRIBUTES = *mut DOT11_ERP_PHY_ATTRIBUTES;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_EXTAP_ATTRIBUTES = *mut DOT11_EXTAP_ATTRIBUTES;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_EXTAP_RECV_CONTEXT = *mut DOT11_EXTSTA_RECV_CONTEXT;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_EXTAP_SEND_CONTEXT = *mut DOT11_EXTSTA_SEND_CONTEXT;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_EXTSTA_ATTRIBUTES = *mut DOT11_EXTSTA_ATTRIBUTES;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_EXTSTA_CAPABILITY = *mut DOT11_EXTSTA_CAPABILITY;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_EXTSTA_RECV_CONTEXT = *mut DOT11_EXTSTA_RECV_CONTEXT;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_EXTSTA_SEND_CONTEXT = *mut DOT11_EXTSTA_SEND_CONTEXT;
#[cfg(feature = "shared")]
pub type PDOT11_FRAGMENT_DESCRIPTOR = *mut DOT11_FRAGMENT_DESCRIPTOR;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS = *mut DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS = *mut DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS = *mut DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS;
pub type PDOT11_HESSID = *mut [u8; 6];
#[cfg(feature = "shared")]
pub type PDOT11_HOPPING_PATTERN_ENTRY = *mut DOT11_HOPPING_PATTERN_ENTRY;
#[cfg(feature = "shared")]
pub type PDOT11_HOPPING_PATTERN_ENTRY_LIST = *mut DOT11_HOPPING_PATTERN_ENTRY_LIST;
pub type PDOT11_HOP_ALGO_ADOPTED = *mut DOT11_HOP_ALGO_ADOPTED;
#[cfg(feature = "shared")]
pub type PDOT11_HRDSSS_PHY_ATTRIBUTES = *mut DOT11_HRDSSS_PHY_ATTRIBUTES;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_IBSS_PARAMS = *mut DOT11_IBSS_PARAMS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS = *mut DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_INCOMING_ASSOC_DECISION = *mut DOT11_INCOMING_ASSOC_DECISION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_INCOMING_ASSOC_DECISION_V2 = *mut DOT11_INCOMING_ASSOC_DECISION_V2;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS = *mut DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_INCOMING_ASSOC_STARTED_PARAMETERS = *mut DOT11_INCOMING_ASSOC_STARTED_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS = *mut DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS = *mut DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_IV48_COUNTER = *mut DOT11_IV48_COUNTER;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_JOIN_REQUEST = *mut DOT11_JOIN_REQUEST;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_BIP = *mut DOT11_KEY_ALGO_BIP;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_BIP_GMAC_256 = *mut DOT11_KEY_ALGO_BIP_GMAC_256;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_CCMP = *mut DOT11_KEY_ALGO_CCMP;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_GCMP = *mut DOT11_KEY_ALGO_GCMP;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_GCMP_256 = *mut DOT11_KEY_ALGO_GCMP_256;
#[cfg(feature = "shared")]
pub type PDOT11_KEY_ALGO_TKIP_MIC = *mut DOT11_KEY_ALGO_TKIP_MIC;
pub type PDOT11_KEY_DIRECTION = *mut DOT11_KEY_DIRECTION;
#[cfg(feature = "shared")]
pub type PDOT11_LINK_QUALITY_ENTRY = *mut DOT11_LINK_QUALITY_ENTRY;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_LINK_QUALITY_PARAMETERS = *mut DOT11_LINK_QUALITY_PARAMETERS;
pub type PDOT11_MAC_ADDRESS = *mut [u8; 6];
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_MAC_ADDRESS_LIST = *mut DOT11_MAC_ADDRESS_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_MAC_FRAME_STATISTICS = *mut DOT11_MAC_FRAME_STATISTICS;
#[cfg(feature = "shared")]
pub type PDOT11_MAC_INFO = *mut DOT11_MAC_INFO;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_MAC_PARAMETERS = *mut DOT11_MAC_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_MANUFACTURING_CALLBACK_PARAMETERS = *mut DOT11_MANUFACTURING_CALLBACK_PARAMETERS;
pub type PDOT11_MANUFACTURING_CALLBACK_TYPE = *mut DOT11_MANUFACTURING_CALLBACK_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC = *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_FUNCTIONAL_TEST_RX = *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_FUNCTIONAL_TEST_TX = *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS = *mut DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_MANUFACTURING_SELF_TEST_SET_PARAMS = *mut DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS;
pub type PDOT11_MANUFACTURING_SELF_TEST_TYPE = *mut DOT11_MANUFACTURING_SELF_TEST_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_TEST = *mut DOT11_MANUFACTURING_TEST;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_TEST_QUERY_DATA = *mut DOT11_MANUFACTURING_TEST_QUERY_DATA;
#[cfg(feature = "shared")]
pub type PDOT11_MANUFACTURING_TEST_SET_DATA = *mut DOT11_MANUFACTURING_TEST_SET_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_MANUFACTURING_TEST_SLEEP = *mut DOT11_MANUFACTURING_TEST_SLEEP;
pub type PDOT11_MANUFACTURING_TEST_TYPE = *mut DOT11_MANUFACTURING_TEST_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_MD_CAPABILITY_ENTRY_LIST = *mut DOT11_MD_CAPABILITY_ENTRY_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_MPDU_MAX_LENGTH_INDICATION = *mut DOT11_MPDU_MAX_LENGTH_INDICATION;
#[cfg(feature = "shared")]
pub type PDOT11_MULTI_DOMAIN_CAPABILITY_ENTRY = *mut DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY;
#[cfg(feature = "shared")]
pub type PDOT11_NIC_SPECIFIC_EXTENSION = *mut DOT11_NIC_SPECIFIC_EXTENSION;
#[cfg(feature = "shared")]
pub type PDOT11_OFDM_PHY_ATTRIBUTES = *mut DOT11_OFDM_PHY_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDOT11_OFFLOAD_CAPABILITY = *mut DOT11_OFFLOAD_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_OFFLOAD_NETWORK = *mut DOT11_OFFLOAD_NETWORK;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_OFFLOAD_NETWORK_LIST_INFO = *mut DOT11_OFFLOAD_NETWORK_LIST_INFO;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS = *mut DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS;
pub type PDOT11_OFFLOAD_TYPE = *mut DOT11_OFFLOAD_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_OPERATION_MODE_CAPABILITY = *mut DOT11_OPERATION_MODE_CAPABILITY;
#[cfg(feature = "shared")]
pub type PDOT11_OPTIONAL_CAPABILITY = *mut DOT11_OPTIONAL_CAPABILITY;
#[cfg(all(feature = "shared", feature = "winnt", feature = "wlan"))]
pub type PDOT11_PEER_INFO = *mut DOT11_PEER_INFO;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
pub type PDOT11_PEER_INFO_LIST = *mut DOT11_PEER_INFO_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_PEER_STATISTICS = *mut DOT11_PEER_STATISTICS;
#[cfg(feature = "shared")]
pub type PDOT11_PER_MSDU_COUNTERS = *mut DOT11_PER_MSDU_COUNTERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PHY_ATTRIBUTES = *mut DOT11_PHY_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDOT11_PHY_FRAME_STATISTICS = *mut DOT11_PHY_FRAME_STATISTICS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS = *mut DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PHY_ID_LIST = *mut DOT11_PHY_ID_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PHY_STATE_PARAMETERS = *mut DOT11_PHY_STATE_PARAMETERS;
pub type PDOT11_PHY_TYPE = *mut DOT11_PHY_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_PHY_TYPE_INFO = *mut DOT11_PHY_TYPE_INFO;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PHY_TYPE_LIST = *mut DOT11_PHY_TYPE_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PMKID_CANDIDATE_LIST_PARAMETERS = *mut DOT11_PMKID_CANDIDATE_LIST_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_PMKID_ENTRY = *mut DOT11_PMKID_ENTRY;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PMKID_LIST = *mut DOT11_PMKID_LIST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PORT_STATE_NOTIFICATION = *mut DOT11_PORT_STATE_NOTIFICATION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO = *mut DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO;
#[cfg(feature = "shared")]
pub type PDOT11_POWER_MGMT_MODE = *mut DOT11_POWER_MGMT_MODE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_POWER_MGMT_MODE_STATUSINFO = *mut DOT11_POWER_MGMT_MODE_STATUS_INFO;
pub type PDOT11_POWER_MODE = *mut DOT11_POWER_MODE;
#[cfg(feature = "shared")]
pub type PDOT11_PRIVACY_EXEMPTION = *mut DOT11_PRIVACY_EXEMPTION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_PRIVACY_EXEMPTION_LIST = *mut DOT11_PRIVACY_EXEMPTION_LIST;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS = *mut DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS = *mut DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_QOS_PARAMS = *mut DOT11_QOS_PARAMS;
#[cfg(feature = "shared")]
pub type PDOT11_QOS_TX_DURATION = *mut DOT11_QOS_TX_DURATION;
#[cfg(feature = "shared")]
pub type PDOT11_QOS_TX_MEDIUM_TIME = *mut DOT11_QOS_TX_MEDIUM_TIME;
#[cfg(feature = "shared")]
pub type PDOT11_RATE_SET = *mut DOT11_RATE_SET;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS = *mut DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS = *mut DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS = *mut DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS = *mut DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS = *mut DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS = *mut DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS = *mut DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_RECV_EXTENSION_INFO = *mut DOT11_RECV_EXTENSION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_RECV_EXTENSION_INFO_V2 = *mut DOT11_RECV_EXTENSION_INFO_V2;
#[cfg(feature = "shared")]
pub type PDOT11_RECV_SENSITIVITY = *mut DOT11_RECV_SENSITIVITY;
#[cfg(feature = "shared")]
pub type PDOT11_RECV_SENSITIVITY_LIST = *mut DOT11_RECV_SENSITIVITY_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_REG_DOMAINS_SUPPORT_VALUE = *mut DOT11_REG_DOMAINS_SUPPORT_VALUE;
#[cfg(feature = "shared")]
pub type PDOT11_REG_DOMAIN_VALUE = *mut DOT11_REG_DOMAIN_VALUE;
#[cfg(feature = "shared")]
pub type PDOT11_RESET_REQUEST = *mut DOT11_RESET_REQUEST;
pub type PDOT11_RESET_TYPE = *mut DOT11_RESET_TYPE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_ROAMING_COMPLETION_PARAMETERS = *mut DOT11_ROAMING_COMPLETION_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_ROAMING_START_PARAMETERS = *mut DOT11_ROAMING_START_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_RSSI_RANGE = *mut DOT11_RSSI_RANGE;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_SCAN_REQUEST = *mut DOT11_SCAN_REQUEST;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_SCAN_REQUEST_V2 = *mut DOT11_SCAN_REQUEST_V2;
pub type PDOT11_SCAN_TYPE = *mut DOT11_SCAN_TYPE;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
pub type PDOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS = *mut DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS = *mut DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt", feature = "wlan"))]
pub type PDOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS = *mut DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_SEND_INVITATION_REQUEST_PARAMETERS = *mut DOT11_SEND_INVITATION_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_SEND_INVITATION_RESPONSE_PARAMETERS = *mut DOT11_SEND_INVITATION_RESPONSE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS = *mut DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "winnt"))]
pub type PDOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS = *mut DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "wlan"))]
pub type PDOT11_SSID_LIST = *mut DOT11_SSID_LIST;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_START_REQUEST = *mut DOT11_START_REQUEST;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_STATISTICS = *mut DOT11_STATISTICS;
#[cfg(all(feature = "shared", feature = "types"))]
pub type PDOT11_STATUS_INDICATION = *mut DOT11_STATUS_INDICATION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_STOP_AP_PARAMETERS = *mut DOT11_STOP_AP_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_ANTENNA = *mut DOT11_SUPPORTED_ANTENNA;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_ANTENNA_LIST = *mut DOT11_SUPPORTED_ANTENNA_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_DATA_RATES_VALUE = *mut DOT11_SUPPORTED_DATA_RATES_VALUE;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_DATA_RATES_VALUE_V1 = *mut DOT11_SUPPORTED_DATA_RATES_VALUE_V2;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_DATA_RATES_VALUE_V2 = *mut DOT11_SUPPORTED_DATA_RATES_VALUE_V2;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_DSSS_CHANNEL = *mut DOT11_SUPPORTED_DSSS_CHANNEL;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_DSSS_CHANNEL_LIST = *mut DOT11_SUPPORTED_DSSS_CHANNEL_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_OFDM_FREQUENCY = *mut DOT11_SUPPORTED_OFDM_FREQUENCY;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_OFDM_FREQUENCY_LIST = *mut DOT11_SUPPORTED_OFDM_FREQUENCY_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_PHY_TYPES = *mut DOT11_SUPPORTED_PHY_TYPES;
#[cfg(feature = "shared")]
pub type PDOT11_SUPPORTED_POWER_LEVELS = *mut DOT11_SUPPORTED_POWER_LEVELS;
pub type PDOT11_TEMP_TYPE = *mut DOT11_TEMP_TYPE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_TKIPMIC_FAILURE_PARAMETERS = *mut DOT11_TKIPMIC_FAILURE_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_UPDATE_IE = *mut DOT11_UPDATE_IE;
pub type PDOT11_UPDATE_IE_OP = *mut DOT11_UPDATE_IE_OP;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_VWIFI_ATTRIBUTES = *mut DOT11_VWIFI_ATTRIBUTES;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_VWIFI_COMBINATION = *mut DOT11_VWIFI_COMBINATION;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_VWIFI_COMBINATION_V2 = *mut DOT11_VWIFI_COMBINATION_V2;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_VWIFI_COMBINATION_V3 = *mut DOT11_VWIFI_COMBINATION_V3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_WEP_OFFLOAD = *mut DOT11_WEP_OFFLOAD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_WEP_UPLOAD = *mut DOT11_WEP_UPLOAD;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_ADDITIONAL_IE = *mut DOT11_WFD_ADDITIONAL_IE;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR = *mut DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_ADVERTISED_SERVICE_LIST = *mut DOT11_WFD_ADVERTISED_SERVICE_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_ADVERTISEMENT_ID = *mut DOT11_WFD_ADVERTISEMENT_ID;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_ATTRIBUTES = *mut DOT11_WFD_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_CHANNEL = *mut DOT11_WFD_CHANNEL;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_CONFIGURATION_TIMEOUT = *mut DOT11_WFD_CONFIGURATION_TIMEOUT;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_DEVICE_CAPABILITY_CONFIG = *mut DOT11_WFD_DEVICE_CAPABILITY_CONFIG;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_WFD_DEVICE_ENTRY = *mut DOT11_WFD_DEVICE_ENTRY;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_DEVICE_INFO = *mut DOT11_WFD_DEVICE_INFO;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_DEVICE_LISTEN_CHANNEL = *mut DOT11_WFD_DEVICE_LISTEN_CHANNEL;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_DEVICE_TYPE = *mut DOT11_WFD_DEVICE_TYPE;
#[cfg(all(feature = "objectheader", feature = "shared", feature = "types"))]
pub type PDOT11_WFD_DISCOVER_COMPLETE_PARAMETERS = *mut DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_WFD_DISCOVER_DEVICE_FILTER = *mut DOT11_WFD_DISCOVER_DEVICE_FILTER;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_DISCOVER_REQUEST = *mut DOT11_WFD_DISCOVER_REQUEST;
pub type PDOT11_WFD_DISCOVER_TYPE = *mut DOT11_WFD_DISCOVER_TYPE;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_GO_INTENT = *mut DOT11_WFD_GO_INTENT;
#[cfg(all(feature = "shared", feature = "wlan"))]
pub type PDOT11_WFD_GROUP_ID = *mut DOT11_WFD_GROUP_ID;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_GROUP_JOIN_PARAMETERS = *mut DOT11_WFD_GROUP_JOIN_PARAMETERS;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG = *mut DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 = *mut DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_GROUP_START_PARAMETERS = *mut DOT11_WFD_GROUP_START_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_INVITATION_FLAGS = *mut DOT11_WFD_INVITATION_FLAGS;
pub type PDOT11_WFD_SCAN_TYPE = *mut DOT11_WFD_SCAN_TYPE;
#[cfg(all(feature = "objectheader", feature = "shared"))]
pub type PDOT11_WFD_SECONDARY_DEVICE_TYPE_LIST = *mut DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_SERVICE_HASH_LIST = *mut DOT11_WFD_SERVICE_HASH_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_SESSION_ID = *mut DOT11_WFD_SESSION_ID;
#[cfg(feature = "shared")]
pub type PDOT11_WFD_SESSION_INFO = *mut DOT11_WFD_SESSION_INFO;
#[cfg(feature = "shared")]
pub type PDOT11_WME_AC_PARAMETERS = *mut DOT11_WME_AC_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDOT11_WME_AC_PARAMETERS_LIST = *mut DOT11_WME_AC_PARAMETERS_LIST;
#[cfg(feature = "shared")]
pub type PDOT11_WME_UPDATE_IE = *mut DOT11_WME_UPDATE_IE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDOT11_WPA_TSC = *mut DOT11_WPA_TSC;
pub type PDOT11_WPS_CONFIG_METHOD = *mut DOT11_WPS_CONFIG_METHOD;
#[cfg(feature = "shared")]
pub type PDOT11_WPS_DEVICE_NAME = *mut DOT11_WPS_DEVICE_NAME;
pub type PDOT11_WPS_DEVICE_PASSWORD_ID = *mut DOT11_WPS_DEVICE_PASSWORD_ID;
#[cfg(feature = "shared")]
pub type PWFDSVC_CONNECTION_CAPABILITY = *mut WFDSVC_CONNECTION_CAPABILITY;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RSNA_AKM_CIPHER_PAIR {
    pub akm: RSNA_AKM_SUITE,
    pub cipher: RSNA_CIPHER_SUITE,
}
pub type RSNA_AKM_SUITE = i32;
pub type RSNA_CIPHER_SUITE = i32;
pub const RSNA_OUI_PREFIX: i32 = 11276032;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WFDSVC_CONNECTION_CAPABILITY {
    pub bNew: super::BOOLEAN,
    pub bClient: super::BOOLEAN,
    pub bGO: super::BOOLEAN,
}
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: i32 = 2;
pub const WFDSVC_CONNECTION_CAPABILITY_GO: i32 = 4;
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: i32 = 1;
pub type WPA_AKM_SUITE = i32;
pub type WPA_CIPHER_SUITE = i32;
pub const WPA_OUI_PREFIX: i32 = 15880192;
pub const ch_description_type_center_frequency: CH_DESCRIPTION_TYPE = 2;
pub const ch_description_type_logical: CH_DESCRIPTION_TYPE = 1;
pub const ch_description_type_phy_specific: CH_DESCRIPTION_TYPE = 3;
pub const dot11_AC_param_BE: DOT11_AC_PARAM = 0;
pub const dot11_AC_param_BK: DOT11_AC_PARAM = 1;
pub const dot11_AC_param_VI: DOT11_AC_PARAM = 2;
pub const dot11_AC_param_VO: DOT11_AC_PARAM = 3;
pub const dot11_AC_param_max: DOT11_AC_PARAM = 4;
pub const dot11_ANQP_query_result_access_issues: DOT11_ANQP_QUERY_RESULT = 7;
pub const dot11_ANQP_query_result_advertisement_protocol_not_supported_on_remote: DOT11_ANQP_QUERY_RESULT = 4;
pub const dot11_ANQP_query_result_advertisement_server_not_responding: DOT11_ANQP_QUERY_RESULT = 6;
pub const dot11_ANQP_query_result_failure: DOT11_ANQP_QUERY_RESULT = 1;
pub const dot11_ANQP_query_result_gas_protocol_failure: DOT11_ANQP_QUERY_RESULT = 5;
pub const dot11_ANQP_query_result_resources: DOT11_ANQP_QUERY_RESULT = 3;
pub const dot11_ANQP_query_result_success: DOT11_ANQP_QUERY_RESULT = 0;
pub const dot11_ANQP_query_result_timed_out: DOT11_ANQP_QUERY_RESULT = 2;
pub const dot11_assoc_state_auth_assoc: DOT11_ASSOCIATION_STATE = 3;
pub const dot11_assoc_state_auth_unassoc: DOT11_ASSOCIATION_STATE = 2;
pub const dot11_assoc_state_unauth_unassoc: DOT11_ASSOCIATION_STATE = 1;
pub const dot11_assoc_state_zero: DOT11_ASSOCIATION_STATE = 0;
pub const dot11_band_2p4g: DOT11_BAND = 1;
pub const dot11_band_4p9g: DOT11_BAND = 2;
pub const dot11_band_5g: DOT11_BAND = 3;
pub const dot11_diversity_support_dynamic: DOT11_DIVERSITY_SUPPORT = 3;
pub const dot11_diversity_support_fixedlist: DOT11_DIVERSITY_SUPPORT = 1;
pub const dot11_diversity_support_notsupported: DOT11_DIVERSITY_SUPPORT = 2;
pub const dot11_diversity_support_unknown: DOT11_DIVERSITY_SUPPORT = 0;
pub const dot11_hop_algo_current: DOT11_HOP_ALGO_ADOPTED = 0;
pub const dot11_hop_algo_hcc: DOT11_HOP_ALGO_ADOPTED = 2;
pub const dot11_hop_algo_hop_index: DOT11_HOP_ALGO_ADOPTED = 1;
pub const dot11_key_direction_both: DOT11_KEY_DIRECTION = 1;
pub const dot11_key_direction_inbound: DOT11_KEY_DIRECTION = 2;
pub const dot11_key_direction_outbound: DOT11_KEY_DIRECTION = 3;
pub const dot11_manufacturing_callback_IHV_end: DOT11_MANUFACTURING_CALLBACK_TYPE = -1;
pub const dot11_manufacturing_callback_IHV_start: DOT11_MANUFACTURING_CALLBACK_TYPE = -2147483648;
pub const dot11_manufacturing_callback_self_test_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = 1;
pub const dot11_manufacturing_callback_sleep_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = 2;
pub const dot11_manufacturing_callback_unknown: DOT11_MANUFACTURING_CALLBACK_TYPE = 0;
pub const dot11_manufacturing_test_IHV_end: DOT11_MANUFACTURING_TEST_TYPE = -1;
pub const dot11_manufacturing_test_IHV_start: DOT11_MANUFACTURING_TEST_TYPE = -2147483648;
pub const dot11_manufacturing_test_awake: DOT11_MANUFACTURING_TEST_TYPE = 9;
pub const dot11_manufacturing_test_query_adc: DOT11_MANUFACTURING_TEST_TYPE = 5;
pub const dot11_manufacturing_test_query_data: DOT11_MANUFACTURING_TEST_TYPE = 7;
pub const dot11_manufacturing_test_rx: DOT11_MANUFACTURING_TEST_TYPE = 3;
pub const dot11_manufacturing_test_self_query_result: DOT11_MANUFACTURING_TEST_TYPE = 2;
pub const dot11_manufacturing_test_self_start: DOT11_MANUFACTURING_TEST_TYPE = 1;
pub const dot11_manufacturing_test_set_data: DOT11_MANUFACTURING_TEST_TYPE = 6;
pub const dot11_manufacturing_test_sleep: DOT11_MANUFACTURING_TEST_TYPE = 8;
pub const dot11_manufacturing_test_tx: DOT11_MANUFACTURING_TEST_TYPE = 4;
pub const dot11_manufacturing_test_unknown: DOT11_MANUFACTURING_TEST_TYPE = 0;
pub const dot11_offload_type_auth: DOT11_OFFLOAD_TYPE = 2;
pub const dot11_offload_type_wep: DOT11_OFFLOAD_TYPE = 1;
pub const dot11_phy_type_IHV_end: DOT11_PHY_TYPE = -1;
pub const dot11_phy_type_IHV_start: DOT11_PHY_TYPE = -2147483648;
pub const dot11_phy_type_any: DOT11_PHY_TYPE = 0;
pub const dot11_phy_type_dmg: DOT11_PHY_TYPE = 9;
pub const dot11_phy_type_dsss: DOT11_PHY_TYPE = 2;
pub const dot11_phy_type_eht: DOT11_PHY_TYPE = 11;
pub const dot11_phy_type_erp: DOT11_PHY_TYPE = 6;
pub const dot11_phy_type_fhss: DOT11_PHY_TYPE = 1;
pub const dot11_phy_type_he: DOT11_PHY_TYPE = 10;
pub const dot11_phy_type_hrdsss: DOT11_PHY_TYPE = 5;
pub const dot11_phy_type_ht: DOT11_PHY_TYPE = 7;
pub const dot11_phy_type_irbaseband: DOT11_PHY_TYPE = 3;
pub const dot11_phy_type_ofdm: DOT11_PHY_TYPE = 4;
pub const dot11_phy_type_unknown: DOT11_PHY_TYPE = 0;
pub const dot11_phy_type_vht: DOT11_PHY_TYPE = 8;
pub const dot11_power_mode_active: DOT11_POWER_MODE = 1;
pub const dot11_power_mode_powersave: DOT11_POWER_MODE = 2;
pub const dot11_power_mode_reason_compliant_AP: DOT11_POWER_MODE_REASON = 3;
pub const dot11_power_mode_reason_compliant_WFD_device: DOT11_POWER_MODE_REASON = 4;
pub const dot11_power_mode_reason_legacy_WFD_device: DOT11_POWER_MODE_REASON = 2;
pub const dot11_power_mode_reason_no_change: DOT11_POWER_MODE_REASON = 0;
pub const dot11_power_mode_reason_noncompliant_AP: DOT11_POWER_MODE_REASON = 1;
pub const dot11_power_mode_reason_others: DOT11_POWER_MODE_REASON = 5;
pub const dot11_power_mode_unknown: DOT11_POWER_MODE = 0;
pub const dot11_reset_type_mac: DOT11_RESET_TYPE = 2;
pub const dot11_reset_type_phy: DOT11_RESET_TYPE = 1;
pub const dot11_reset_type_phy_and_mac: DOT11_RESET_TYPE = 3;
pub const dot11_scan_type_active: DOT11_SCAN_TYPE = 1;
pub const dot11_scan_type_auto: DOT11_SCAN_TYPE = 3;
pub const dot11_scan_type_forced: DOT11_SCAN_TYPE = -2147483648;
pub const dot11_scan_type_passive: DOT11_SCAN_TYPE = 2;
pub const dot11_temp_type_1: DOT11_TEMP_TYPE = 1;
pub const dot11_temp_type_2: DOT11_TEMP_TYPE = 2;
pub const dot11_temp_type_unknown: DOT11_TEMP_TYPE = 0;
pub const dot11_update_ie_op_create_replace: DOT11_UPDATE_IE_OP = 1;
pub const dot11_update_ie_op_delete: DOT11_UPDATE_IE_OP = 2;
pub const dot11_wfd_discover_type_auto: DOT11_WFD_DISCOVER_TYPE = 3;
pub const dot11_wfd_discover_type_find_only: DOT11_WFD_DISCOVER_TYPE = 2;
pub const dot11_wfd_discover_type_forced: DOT11_WFD_DISCOVER_TYPE = -2147483648;
pub const dot11_wfd_discover_type_scan_only: DOT11_WFD_DISCOVER_TYPE = 1;
pub const dot11_wfd_discover_type_scan_social_channels: DOT11_WFD_DISCOVER_TYPE = 4;
pub const dot11_wfd_scan_type_active: DOT11_WFD_SCAN_TYPE = 1;
pub const dot11_wfd_scan_type_auto: DOT11_WFD_SCAN_TYPE = 3;
pub const dot11_wfd_scan_type_passive: DOT11_WFD_SCAN_TYPE = 2;
pub const rsna_akm_1x: RSNA_AKM_SUITE = 28053248;
pub const rsna_akm_1x_sha256: RSNA_AKM_SUITE = 95162112;
pub const rsna_akm_1x_sha384: RSNA_AKM_SUITE = 397152000;
pub const rsna_akm_1x_suite_b_sha256: RSNA_AKM_SUITE = 195825408;
pub const rsna_akm_1x_suite_b_sha384: RSNA_AKM_SUITE = 212602624;
pub const rsna_akm_fils_1x_sha256: RSNA_AKM_SUITE = 246157056;
pub const rsna_akm_fils_1x_sha384: RSNA_AKM_SUITE = 262934272;
pub const rsna_akm_ft_1x_sha256: RSNA_AKM_SUITE = 61607680;
pub const rsna_akm_ft_1x_sha384: RSNA_AKM_SUITE = 380374784;
pub const rsna_akm_ft_1x_sha384_cmp_256: RSNA_AKM_SUITE = 229379840;
pub const rsna_akm_ft_fils_1x_sha256: RSNA_AKM_SUITE = 279711488;
pub const rsna_akm_ft_fils_sha384: RSNA_AKM_SUITE = 296488704;
pub const rsna_akm_ft_psk_sha256: RSNA_AKM_SUITE = 78384896;
pub const rsna_akm_ft_psk_sha384: RSNA_AKM_SUITE = 330043136;
pub const rsna_akm_ft_sae_pmk256: RSNA_AKM_SUITE = 162270976;
pub const rsna_akm_ft_sae_pmk384: RSNA_AKM_SUITE = 430706432;
pub const rsna_akm_max: RSNA_AKM_SUITE = 430706432;
pub const rsna_akm_none: RSNA_AKM_SUITE = 11276032;
pub const rsna_akm_owe: RSNA_AKM_SUITE = 313265920;
pub const rsna_akm_peerkey_sha256: RSNA_AKM_SUITE = 179048192;
pub const rsna_akm_psk: RSNA_AKM_SUITE = 44830464;
pub const rsna_akm_psk_sha256: RSNA_AKM_SUITE = 111939328;
pub const rsna_akm_psk_sha384: RSNA_AKM_SUITE = 346820352;
pub const rsna_akm_sae_pmk256: RSNA_AKM_SUITE = 145493760;
pub const rsna_akm_sae_pmk384: RSNA_AKM_SUITE = 413929216;
pub const rsna_akm_tdls_sha256: RSNA_AKM_SUITE = 128716544;
pub const rsna_cipher_bip_cmac_128: RSNA_CIPHER_SUITE = 111939328;
pub const rsna_cipher_bip_cmac_256: RSNA_CIPHER_SUITE = 229379840;
pub const rsna_cipher_bip_gmac_128: RSNA_CIPHER_SUITE = 195825408;
pub const rsna_cipher_bip_gmac_256: RSNA_CIPHER_SUITE = 212602624;
pub const rsna_cipher_ccmp_128: RSNA_CIPHER_SUITE = 78384896;
pub const rsna_cipher_ccmp_256: RSNA_CIPHER_SUITE = 179048192;
pub const rsna_cipher_gcmp_128: RSNA_CIPHER_SUITE = 145493760;
pub const rsna_cipher_gcmp_256: RSNA_CIPHER_SUITE = 162270976;
pub const rsna_cipher_group: RSNA_CIPHER_SUITE = 11276032;
pub const rsna_cipher_max: RSNA_CIPHER_SUITE = 229379840;
pub const rsna_cipher_no_group_traffic: RSNA_CIPHER_SUITE = 128716544;
pub const rsna_cipher_reserved: RSNA_CIPHER_SUITE = 61607680;
pub const rsna_cipher_tkip: RSNA_CIPHER_SUITE = 44830464;
pub const rsna_cipher_wep104: RSNA_CIPHER_SUITE = 95162112;
pub const rsna_cipher_wep40: RSNA_CIPHER_SUITE = 28053248;
pub const wpa_akm_1x: WPA_AKM_SUITE = 32657408;
pub const wpa_akm_max: WPA_AKM_SUITE = 49434624;
pub const wpa_akm_none: WPA_AKM_SUITE = 15880192;
pub const wpa_akm_psk: WPA_AKM_SUITE = 49434624;
pub const wpa_cipher_bip_cmac_128: WPA_CIPHER_SUITE = 116543488;
pub const wpa_cipher_ccmp_128: WPA_CIPHER_SUITE = 82989056;
pub const wpa_cipher_max: WPA_CIPHER_SUITE = 116543488;
pub const wpa_cipher_none: WPA_CIPHER_SUITE = 15880192;
pub const wpa_cipher_tkip: WPA_CIPHER_SUITE = 49434624;
pub const wpa_cipher_wep104: WPA_CIPHER_SUITE = 99766272;
pub const wpa_cipher_wep40: WPA_CIPHER_SUITE = 32657408;
