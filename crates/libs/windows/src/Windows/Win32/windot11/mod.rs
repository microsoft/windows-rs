pub type CH_DESCRIPTION_TYPE = i32;
pub const DISCOVERY_FILTER_BITMASK_ANY: u32 = 15;
pub const DISCOVERY_FILTER_BITMASK_DEVICE: u32 = 1;
pub const DISCOVERY_FILTER_BITMASK_GO: u32 = 2;
pub type DOT11_AC_PARAM = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_ADDITIONAL_IE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uResponseIEsOffset: u32,
    pub uResponseIEsLength: u32,
}
pub const DOT11_ADDITIONAL_IE_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_AKM_CIPHER_PAIR {
    pub akm: RSNA_AKM_SUITE,
    pub cipher: RSNA_CIPHER_SUITE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Status: DOT11_ANQP_QUERY_RESULT,
    pub hContext: super::winnt::HANDLE,
    pub uResponseLength: u32,
}
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_ANQP_QUERY_RESULT = i32;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_AP_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub uStatus: DOT11_ASSOC_STATUS,
    pub bReAssocReq: bool,
    pub bReAssocResp: bool,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
    pub AuthAlgo: super::wlan::DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub bFourAddressSupported: bool,
    pub bPortAuthorized: bool,
    pub ucActiveQoSProtocol: u8,
    pub DSInfo: DOT11_DS_INFO,
    pub uEncapTableOffset: u32,
    pub uEncapTableSize: u32,
    pub MulticastMgmtCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub uAssocComebackTime: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ASSOCIATION_INFO_EX {
    pub PeerMacAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub usCapabilityInformation: u16,
    pub usListenInterval: u16,
    pub ucPeerSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub dot11AssociationState: DOT11_ASSOCIATION_STATE,
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub ullNumOfTxPacketSuccesses: u64,
    pub ullNumOfTxPacketFailures: u64,
    pub ullNumOfRxPacketSuccesses: u64,
    pub ullNumOfRxPacketFailures: u64,
}
impl Default for DOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ASSOCIATION_INFO_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11AssocInfo: [DOT11_ASSOCIATION_INFO_EX; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ASSOCIATION_PARAMS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub uAssocRequestIEsOffset: u32,
    pub uAssocRequestIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ASSOCIATION_START_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub SSID: super::wlan::DOT11_SSID,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_ASSOCIATION_STATE = i32;
pub const DOT11_ASSOC_ERROR_SOURCE_OS: u32 = 0;
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: u32 = 255;
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DOT11_ASSOC_STATUS(pub u32);
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
pub const DOT11_ASSOC_STATUS_SUCCESS: u32 = 0;
pub const DOT11_ASSOC_STATUS_SYSTEM_ERROR: u32 = 10;
pub const DOT11_ASSOC_STATUS_UNREACHABLE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_AUTH_ALGORITHM_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [super::wlan::DOT11_AUTH_ALGORITHM; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: u32 = 1;
pub const DOT11_AUTH_ALGO_MICHAEL: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_AUTH_CIPHER_PAIR_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AuthCipherPairs: [super::wlan::DOT11_AUTH_CIPHER_PAIR; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_AVAILABLE_CHANNEL_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uChannelNumber: [u32; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_AVAILABLE_FREQUENCY_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uFrequencyValue: [u32; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: u32 = 1;
pub type DOT11_BAND = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_BSSID_CANDIDATE {
    pub BSSID: DOT11_MAC_ADDRESS,
    pub uFlags: u32,
}
impl Default for DOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_BSSID_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub BSSIDs: [DOT11_MAC_ADDRESS; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_BSSID_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_BSS_DESCRIPTION {
    pub uReserved: u32,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::wlan::DOT11_BSS_TYPE,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy)]
pub struct DOT11_BSS_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::wlan::DOT11_BSS_TYPE,
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: bool,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    pub uChCenterFrequency: u32,
    pub FHSS: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0,
}
impl Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    pub uHopPattern: u32,
    pub uHopSet: u32,
    pub uDwellTime: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_BYTE_ARRAY {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfBytes: u32,
    pub uTotalNumOfBytes: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080;
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: u32 = 128;
pub const DOT11_CAPABILITY_DSSSOFDM: u32 = 8192;
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: u32 = 4;
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: u32 = 8;
pub const DOT11_CAPABILITY_INFO_ESS: u32 = 1;
pub const DOT11_CAPABILITY_INFO_IBSS: u32 = 2;
pub const DOT11_CAPABILITY_INFO_PRIVACY: u32 = 16;
pub const DOT11_CAPABILITY_PBCC: u32 = 64;
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: u32 = 32;
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: u32 = 1024;
pub const DOT11_CCA_MODE_CS_ONLY: u32 = 2;
pub const DOT11_CCA_MODE_CS_WITH_TIMER: u32 = 8;
pub const DOT11_CCA_MODE_ED_ONLY: u32 = 1;
pub const DOT11_CCA_MODE_ED_and_CS: u32 = 4;
pub const DOT11_CCA_MODE_HRCS_AND_ED: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CHANNEL_HINT {
    pub Dot11PhyType: DOT11_PHY_TYPE,
    pub uChannelNumber: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_CIPHER_ALGORITHM_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [super::wlan::DOT11_CIPHER_ALGORITHM; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uKeyIndex: u32,
    pub AlgorithmId: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub bDelete: bool,
    pub bStatic: bool,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub AlgorithmId: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub Direction: DOT11_DIRECTION,
    pub bDelete: bool,
    pub bStatic: bool,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: u32 = 1;
pub const DOT11_CONF_ALGO_TKIP: u32 = 2;
pub const DOT11_CONF_ALGO_WEP_RC4: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uStatus: DOT11_ASSOC_STATUS,
}
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_CONNECTION_START_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub BSSType: super::wlan::DOT11_BSS_TYPE,
    pub AdhocBSSID: DOT11_MAC_ADDRESS,
    pub AdhocSSID: super::wlan::DOT11_SSID,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_CONNECTION_STATUS_CANCELLED: u32 = 5;
pub const DOT11_CONNECTION_STATUS_CANDIDATE_LIST_EXHAUSTED: u32 = 6;
pub const DOT11_CONNECTION_STATUS_FAILURE: u32 = 1;
pub const DOT11_CONNECTION_STATUS_IHV_END: i32 = -1;
pub const DOT11_CONNECTION_STATUS_IHV_START: i32 = -2147483648;
pub const DOT11_CONNECTION_STATUS_PHY_POWER_DOWN: u32 = 3;
pub const DOT11_CONNECTION_STATUS_SUCCESS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_COUNTERS_ENTRY {
    pub uTransmittedFragmentCount: u32,
    pub uMulticastTransmittedFrameCount: u32,
    pub uFailedCount: u32,
    pub uRetryCount: u32,
    pub uMultipleRetryCount: u32,
    pub uFrameDuplicateCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
    pub uReceivedFragmentCount: u32,
    pub uMulticastReceivedFrameCount: u32,
    pub uFCSErrorCount: u32,
    pub uTransmittedFrameCount: u32,
}
pub type DOT11_COUNTRY_OR_REGION_STRING = [u8; 3];
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub CountryOrRegionStrings: [DOT11_COUNTRY_OR_REGION_STRING; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CURRENT_OPERATION_MODE {
    pub uReserved: u32,
    pub uCurrentOpMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11CFPollable: bool,
    pub bDot11PCF: bool,
    pub bDot11PCFMPDUTransferToPC: bool,
    pub bStrictlyOrderedServiceClass: bool,
}
pub const DOT11_DATA_RATE_INDEX_MASK: u32 = 127;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_DATA_RATE_MAPPING_ENTRY {
    pub ucDataRateIndex: u8,
    pub ucDataRateFlag: u8,
    pub usDataRateValue: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DATA_RATE_MAPPING_TABLE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uDataRateMappingLength: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: u32 = 1;
pub const DOT11_DATA_RATE_NON_STANDARD: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DEFAULT_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::winnt::HANDLE,
    pub hOffload: super::winnt::HANDLE,
    pub dwIndex: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub uFlags: u32,
    pub dot11KeyDirection: DOT11_KEY_DIRECTION,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DEFAULT_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::winnt::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DOT11_DIALOG_TOKEN(pub u8);
pub type DOT11_DIRECTION = i32;
pub const DOT11_DIR_BOTH: DOT11_DIRECTION = 3;
pub const DOT11_DIR_INBOUND: DOT11_DIRECTION = 1;
pub const DOT11_DIR_OUTBOUND: DOT11_DIRECTION = 2;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DISASSOCIATE_PEER_REQUEST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub usReason: u16,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DISASSOCIATION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub MacAddr: DOT11_MAC_ADDRESS,
    pub uReason: DOT11_ASSOC_STATUS,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_DISASSOC_REASON_IHV_END: i32 = -1;
pub const DOT11_DISASSOC_REASON_IHV_START: i32 = -2147483648;
pub const DOT11_DISASSOC_REASON_OS: u32 = 7;
pub const DOT11_DISASSOC_REASON_PEER_DEAUTHENTICATED: u32 = 65536;
pub const DOT11_DISASSOC_REASON_PEER_DISASSOCIATED: u32 = 131072;
pub const DOT11_DISASSOC_REASON_PEER_UNREACHABLE: u32 = 2;
pub const DOT11_DISASSOC_REASON_PHY_DISABLED: u32 = 4;
pub const DOT11_DISASSOC_REASON_RADIO_OFF: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_DIVERSITY_SELECTION_RX {
    pub uAntennaListIndex: u32,
    pub bDiversitySelectionRX: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11DiversitySelectionRx: [DOT11_DIVERSITY_SELECTION_RX; 1],
}
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
pub const DOT11_ENCAP_802_1H: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_ENCAP_ENTRY {
    pub usEtherType: u16,
    pub usEncapType: u16,
}
pub const DOT11_ENCAP_RFC_1042: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_ERP_PHY_ATTRIBUTES {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub bERPPBCCOptionImplemented: bool,
    pub bDSSSOFDMOptionImplemented: bool,
    pub bShortSlotTimeOptionImplemented: bool,
}
pub const DOT11_EXEMPT_ALWAYS: u32 = 1;
pub const DOT11_EXEMPT_BOTH: u32 = 3;
pub const DOT11_EXEMPT_MULTICAST: u32 = 2;
pub const DOT11_EXEMPT_NO_EXEMPTION: u32 = 0;
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: u32 = 2;
pub const DOT11_EXEMPT_UNICAST: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_EXTAP_ATTRIBUTES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uAssociationTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub bStrictlyOrderedServiceClassImplemented: bool,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
}
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 1;
#[cfg(feature = "Win32_objectheader")]
pub type DOT11_EXTAP_RECV_CONTEXT = DOT11_EXTSTA_RECV_CONTEXT;
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: u32 = 1;
#[cfg(feature = "Win32_objectheader")]
pub type DOT11_EXTAP_SEND_CONTEXT = DOT11_EXTSTA_SEND_CONTEXT;
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_EXTSTA_ATTRIBUTES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
    pub bStrictlyOrderedServiceClassImplemented: bool,
    pub ucSupportedQoSProtocolFlags: u8,
    pub bSafeModeImplemented: bool,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedUcastAlgoPairs: u32,
    pub pAdhocSupportedUcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedMcastAlgoPairs: u32,
    pub pAdhocSupportedMcastAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub bAutoPowerSaveMode: bool,
    pub uMaxNetworkOffloadListSize: u32,
    pub bMFPCapable: bool,
    pub uInfraNumSupportedMcastMgmtAlgoPairs: u32,
    pub pInfraSupportedMcastMgmtAlgoPairs: super::wlan::PDOT11_AUTH_CIPHER_PAIR,
    pub bNeighborReportSupported: bool,
    pub bAPChannelReportSupported: bool,
    pub bActionFramesSupported: bool,
    pub bANQPQueryOffloadSupported: bool,
    pub bHESSIDConnectionSupported: bool,
}
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: u32 = 1;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: u32 = 2;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: u32 = 3;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: u32 = 4;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: u32 = 2;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: u32 = 1;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: u32 = 12;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_EXTSTA_CAPABILITY {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
}
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_EXTSTA_RECV_CONTEXT {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uReceiveFlags: u32,
    pub uPhyId: u32,
    pub uChCenterFrequency: u32,
    pub usNumberOfMPDUsReceived: u16,
    pub lRSSI: i32,
    pub ucDataRate: u8,
    pub uSizeMediaSpecificInfo: u32,
    pub pvMediaSpecificInfo: *mut core::ffi::c_void,
    pub ullTimestamp: u64,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_EXTSTA_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_EXTSTA_SEND_CONTEXT {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub usExemptionActionType: u16,
    pub uPhyId: u32,
    pub uDelayedSleepValue: u32,
    pub pvMediaSpecificInfo: *mut core::ffi::c_void,
    pub uSendFlags: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_EXTSTA_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: u32 = 1;
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: u32 = 4;
pub const DOT11_FLAGS_80211B_PBCC: u32 = 2;
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: u32 = 1;
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: u32 = 128;
pub const DOT11_FLAGS_80211G_DSSS_OFDM: u32 = 16;
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: u32 = 64;
pub const DOT11_FLAGS_80211G_USE_PROTECTION: u32 = 32;
pub const DOT11_FLAGS_PS_ON: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_FRAGMENT_DESCRIPTOR {
    pub uOffset: u32,
    pub uLength: u32,
}
pub const DOT11_FREQUENCY_BANDS_LOWER: u32 = 1;
pub const DOT11_FREQUENCY_BANDS_MIDDLE: u32 = 2;
pub const DOT11_FREQUENCY_BANDS_UPPER: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_HESSID = [u8; 6];
pub const DOT11_HESSID_LENGTH: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_HOPPING_PATTERN_ENTRY {
    pub uHoppingPatternIndex: u32,
    pub uRandomTableFieldNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11HoppingPatternEntry: [DOT11_HOPPING_PATTERN_ENTRY; 1],
}
impl Default for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_HOP_ALGO_ADOPTED = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES {
    pub bShortPreambleOptionImplemented: bool,
    pub bPBCCOptionImplemented: bool,
    pub bChannelAgilityPresent: bool,
    pub uHRCCAModeSupported: u32,
}
pub const DOT11_HR_CCA_MODE_CS_AND_ED: u32 = 4;
pub const DOT11_HR_CCA_MODE_CS_ONLY: u32 = 2;
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: u32 = 8;
pub const DOT11_HR_CCA_MODE_ED_ONLY: u32 = 1;
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: u32 = 16;
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: u32 = 8;
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: u32 = 4;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: u32 = 32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: u32 = 16;
pub const DOT11_HW_WEP_SUPPORTED_RX: u32 = 2;
pub const DOT11_HW_WEP_SUPPORTED_TX: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_IBSS_PARAMS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bJoinOnly: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
pub const DOT11_IBSS_PARAMS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub uStatus: u32,
    pub ucErrorSource: u8,
    pub bReAssocReq: bool,
    pub bReAssocResp: bool,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub AuthAlgo: super::wlan::DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_DECISION {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bAccept: bool,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 1;
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_DECISION_V2 {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bAccept: bool,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
    pub WFDStatus: DOT11_WFD_STATUS_CODE,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub bReAssocReq: bool,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_INVALID_CHANNEL_NUMBER: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub ReceiverAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_IV48_COUNTER {
    pub uIV32Counter: u32,
    pub usIV16Counter: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_BIP {
    pub ucIPN: [u8; 6],
    pub ulBIPKeyLength: u32,
    pub ucBIPKey: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_BIP_GMAC_256 {
    pub ucIPN: [u8; 6],
    pub ulBIPGmac256KeyLength: u32,
    pub ucBIPGmac256Key: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_CCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulCCMPKeyLength: u32,
    pub ucCCMPKey: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_GCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMPKeyLength: u32,
    pub ucGCMPKey: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_GCMP_256 {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMP256KeyLength: u32,
    pub ucGCMP256Key: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_KEY_ALGO_TKIP_MIC {
    pub ucIV48Counter: [u8; 6],
    pub ulTKIPKeyLength: u32,
    pub ulMICKeyLength: u32,
    pub ucTKIPMICKeys: [u8; 1],
}
impl Default for DOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_KEY_DIRECTION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_LINK_QUALITY_ENTRY {
    pub PeerMacAddr: DOT11_MAC_ADDRESS,
    pub ucLinkQuality: u8,
}
impl Default for DOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_LINK_QUALITY_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uLinkQualityListSize: u32,
    pub uLinkQualityListOffset: u32,
}
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_MAC_ADDRESS = [u8; 6];
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MAC_ADDRESS_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub MacAddrs: [DOT11_MAC_ADDRESS; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: u32 = 1;
pub const DOT11_MAC_AUTO_CONFIG_ENABLED_FLAG: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullTransmittedFailureFrameCount: u64,
    pub ullReceivedFailureFrameCount: u64,
    pub ullWEPExcludedCount: u64,
    pub ullTKIPLocalMICFailures: u64,
    pub ullTKIPReplays: u64,
    pub ullTKIPICVErrorCount: u64,
    pub ullCCMPReplays: u64,
    pub ullCCMPDecryptErrors: u64,
    pub ullWEPUndecryptableCount: u64,
    pub ullWEPICVErrorCount: u64,
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MAC_INFO {
    pub uReserved: u32,
    pub uNdisPortNumber: u32,
    pub MacAddr: DOT11_MAC_ADDRESS,
}
impl Default for DOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MAC_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uOpmodeMask: u32,
}
pub const DOT11_MAC_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub dot11ManufacturingCallbackType: DOT11_MANUFACTURING_CALLBACK_TYPE,
    pub uStatus: u32,
    pub pvContext: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: u32 = 1;
pub type DOT11_MANUFACTURING_CALLBACK_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub ADCPowerLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    pub bEnabled: bool,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub PowerLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    pub bEnable: bool,
    pub bOpenLoop: bool,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub uSetPowerLevel: u32,
    pub ADCPowerLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub bResult: bool,
    pub uPinFailedBitMask: u32,
    pub pvContext: *mut core::ffi::c_void,
    pub uBytesWrittenOut: u32,
    pub ucBufferOut: [u8; 1],
}
impl Default for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub uPinBitMask: u32,
    pub pvContext: *mut core::ffi::c_void,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST {
    pub dot11ManufacturingTestType: DOT11_MANUFACTURING_TEST_TYPE,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl Default for DOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub uBytesRead: u32,
    pub ucBufferOut: [u8; 1],
}
impl Default for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MANUFACTURING_TEST_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_SET_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
impl Default for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MANUFACTURING_TEST_SLEEP {
    pub uSleepTime: u32,
    pub pvContext: *mut core::ffi::c_void,
}
impl Default for DOT11_MANUFACTURING_TEST_SLEEP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_MANUFACTURING_TEST_TYPE = i32;
pub const DOT11_MAX_CHANNEL_HINTS: u32 = 4;
pub const DOT11_MAX_NUM_DEFAULT_KEY: u32 = 4;
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: u32 = 6;
pub const DOT11_MAX_NUM_OF_FRAGMENTS: u32 = 16;
pub const DOT11_MAX_PDU_SIZE: u32 = 2346;
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11MDCapabilityEntry: [DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY; 1],
}
impl Default for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_MIN_PDU_SIZE: u32 = 256;
pub const DOT11_MIN_SIZEOF_OFFLOAD_NETWORK_LIST_INFO_REVISION_1: u32 = 24;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub uMPDUMaxLength: u32,
}
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    pub uMultiDomainCapabilityIndex: u32,
    pub uFirstChannelNumber: u32,
    pub uNumberOfChannels: u32,
    pub lMaximumTransmitPowerLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_NIC_SPECIFIC_EXTENSION {
    pub uBufferLength: u32,
    pub uTotalBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl Default for DOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: u32 = 4;
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: u32 = 2;
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_OFDM_PHY_ATTRIBUTES {
    pub uFrequencyBandsSupported: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
    pub uSupportedWEPAlgorithms: u32,
    pub uNumOfReplayWindows: u32,
    pub uMaxWEPKeyMappingLength: u32,
    pub uSupportedAuthAlgorithms: u32,
    pub uMaxAuthKeyMappingLength: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK {
    pub Ssid: super::wlan::DOT11_SSID,
    pub UnicastCipher: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub AuthAlgo: super::wlan::DOT11_AUTH_ALGORITHM,
    pub Dot11ChannelHints: [DOT11_CHANNEL_HINT; 4],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ulFlags: u32,
    pub FastScanPeriod: u32,
    pub FastScanIterations: u32,
    pub SlowScanPeriod: u32,
    pub uNumOfEntries: u32,
    pub offloadNetworkList: [DOT11_OFFLOAD_NETWORK; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Status: super::types::NDIS_STATUS,
}
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_OFFLOAD_TYPE = i32;
pub const DOT11_OPERATION_MODE_AP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_OPERATION_MODE_CAPABILITY {
    pub uReserved: u32,
    pub uMajorVersion: u32,
    pub uMinorVersion: u32,
    pub uNumOfTXBuffers: u32,
    pub uNumOfRXBuffers: u32,
    pub uOpModeCapability: u32,
}
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: u32 = 8;
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: u32 = 4;
pub const DOT11_OPERATION_MODE_MANUFACTURING: u32 = 1073741824;
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648;
pub const DOT11_OPERATION_MODE_STATION: u32 = 1;
pub const DOT11_OPERATION_MODE_UNKNOWN: u32 = 0;
pub const DOT11_OPERATION_MODE_WFD_CLIENT: u32 = 64;
pub const DOT11_OPERATION_MODE_WFD_DEVICE: u32 = 16;
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11PCF: bool,
    pub bDot11PCFMPDUTransferToPC: bool,
    pub bStrictlyOrderedServiceClass: bool,
}
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: u32 = 4096;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: u32 = 16384;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: u32 = 8192;
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: u32 = 64;
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: u32 = 256;
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: u32 = 128;
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: u32 = 1;
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: u32 = 4;
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: u32 = 2;
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: u32 = 8;
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: u32 = 32;
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: u32 = 16;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: u32 = 512;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: u32 = 2048;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: u32 = 1024;
pub const DOT11_PACKET_TYPE_RESERVED: i32 = -32768;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PEER_INFO {
    pub MacAddress: DOT11_MAC_ADDRESS,
    pub usCapabilityInformation: u16,
    pub AuthAlgo: super::wlan::DOT11_AUTH_ALGORITHM,
    pub UnicastCipherAlgo: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub MulticastCipherAlgo: super::wlan::DOT11_CIPHER_ALGORITHM,
    pub bWpsEnabled: bool,
    pub usListenInterval: u16,
    pub ucSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub AssociationState: DOT11_ASSOCIATION_STATE,
    pub PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub Statistics: DOT11_PEER_STATISTICS,
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PEER_INFO_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PeerInfo: [DOT11_PEER_INFO; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PEER_INFO_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PEER_STATISTICS {
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
    pub ullTxPacketSuccessCount: u64,
    pub ullTxPacketFailureCount: u64,
    pub ullRxPacketSuccessCount: u64,
    pub ullRxPacketFailureCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PER_MSDU_COUNTERS {
    pub uTransmittedFragmentCount: u32,
    pub uRetryCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy)]
pub struct DOT11_PHY_ATTRIBUTES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PhyType: DOT11_PHY_TYPE,
    pub bHardwarePhyState: bool,
    pub bSoftwarePhyState: bool,
    pub bCFPollable: bool,
    pub uMPDUMaxLength: u32,
    pub TempType: DOT11_TEMP_TYPE,
    pub DiversitySupport: DOT11_DIVERSITY_SUPPORT,
    pub PhySpecificAttributes: DOT11_PHY_ATTRIBUTES_0,
    pub uNumberSupportedPowerLevels: u32,
    pub TxPowerLevels: [u32; 8],
    pub uNumDataRateMappingEntries: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
    pub SupportedDataRatesValue: DOT11_SUPPORTED_DATA_RATES_VALUE_V2,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy)]
pub union DOT11_PHY_ATTRIBUTES_0 {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub OFDMAttributes: DOT11_OFDM_PHY_ATTRIBUTES,
    pub ERPAttributes: DOT11_ERP_PHY_ATTRIBUTES,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: u32 = 1;
pub const DOT11_PHY_AUTO_CONFIG_ENABLED_FLAG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullMulticastTransmittedFrameCount: u64,
    pub ullFailedCount: u64,
    pub ullRetryCount: u64,
    pub ullMultipleRetryCount: u64,
    pub ullMaxTXLifetimeExceededCount: u64,
    pub ullTransmittedFragmentCount: u64,
    pub ullRTSSuccessCount: u64,
    pub ullRTSFailureCount: u64,
    pub ullACKFailureCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullMulticastReceivedFrameCount: u64,
    pub ullPromiscuousReceivedFrameCount: u64,
    pub ullMaxRXLifetimeExceededCount: u64,
    pub ullFrameDuplicateCount: u64,
    pub ullReceivedFragmentCount: u64,
    pub ullPromiscuousReceivedFragmentCount: u64,
    pub ullFCSErrorCount: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy)]
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ulPhyId: u32,
    pub Anonymous: DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy)]
pub union DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    pub ulChannel: u32,
    pub ulFrequency: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_PHY_ID_ANY: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PHY_ID_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyId: [u32; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ID_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PHY_STATE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub bHardwarePhyState: bool,
    pub bSoftwarePhyState: bool,
}
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: u32 = 1;
pub type DOT11_PHY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PHY_TYPE_INFO {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub bUseParameters: bool,
    pub uProbeDelay: u32,
    pub uMinChannelTime: u32,
    pub uMaxChannelTime: u32,
    pub ChDescriptionType: CH_DESCRIPTION_TYPE,
    pub uChannelListSize: u32,
    pub ucChannelListBuffer: [u8; 1],
}
impl Default for DOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PHY_TYPE_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyType: [DOT11_PHY_TYPE; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PHY_TYPE_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uCandidateListSize: u32,
    pub uCandidateListOffset: u32,
}
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_PMKID_CANDIDATE_PREAUTH_ENABLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PMKID_ENTRY {
    pub BSSID: DOT11_MAC_ADDRESS,
    pub PMKID: DOT11_PMKID_VALUE,
    pub uFlags: u32,
}
impl Default for DOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PMKID_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PMKIDs: [DOT11_PMKID_ENTRY; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PMKID_LIST_REVISION_1: u32 = 1;
pub type DOT11_PMKID_VALUE = [u8; 16];
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PORT_STATE_NOTIFICATION {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerMac: DOT11_MAC_ADDRESS,
    pub bOpen: bool,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bEnabled: bool,
}
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_POWER_MGMT_MODE {
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub usListenInterval: u16,
    pub usAID: u16,
    pub bReceiveDTIMs: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PowerSaveMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub Reason: DOT11_POWER_MODE_REASON,
}
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 1;
pub type DOT11_POWER_MODE = i32;
pub type DOT11_POWER_MODE_REASON = i32;
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: u32 = 2;
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: u32 = 1;
pub const DOT11_POWER_SAVING_FAST_PSP: u32 = 8;
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: u32 = 24;
pub const DOT11_POWER_SAVING_MAX_PSP: u32 = 16;
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: u32 = 0;
pub const DOT11_PRIORITY_CONTENTION: u32 = 0;
pub const DOT11_PRIORITY_CONTENTION_FREE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_PRIVACY_EXEMPTION {
    pub usEtherType: u16,
    pub usExemptionActionType: u16,
    pub usExemptionPacketType: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PRIVACY_EXEMPTION_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PrivacyExemptionEntries: [DOT11_PRIVACY_EXEMPTION; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub ReceiverAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub Status: super::types::NDIS_STATUS,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_QOS_PARAMS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ucEnabledQoSProtocolFlags: u8,
}
pub const DOT11_QOS_PARAMS_REVISION_1: u32 = 1;
pub const DOT11_QOS_PROTOCOL_FLAG_11E: u32 = 2;
pub const DOT11_QOS_PROTOCOL_FLAG_WMM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_QOS_TX_DURATION {
    pub uNominalMSDUSize: u32,
    pub uMinPHYRate: u32,
    pub uDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_QOS_TX_MEDIUM_TIME {
    pub dot11PeerAddress: DOT11_MAC_ADDRESS,
    pub ucQoSPriority: u8,
    pub uMediumTimeAdmited: u32,
}
impl Default for DOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RATE_SET {
    pub uRateSetLength: u32,
    pub ucRateSet: [u8; 126],
}
impl Default for DOT11_RATE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RATE_SET_MAX_LENGTH: u32 = 126;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub ResponseContext: *mut core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: DOT11_MAC_ADDRESS,
    pub BSSID: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECV_EXTENSION_INFO {
    pub uVersion: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::winnt::HANDLE,
    pub hAuthOffloadContext: super::winnt::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RECV_EXTENSION_INFO_V2 {
    pub uVersion: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::winnt::HANDLE,
    pub hAuthOffloadContext: super::winnt::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_RECV_FLAG_RAW_PACKET: u32 = 1;
pub const DOT11_RECV_FLAG_RAW_PACKET_FCS_FAILURE: u32 = 2;
pub const DOT11_RECV_FLAG_RAW_PACKET_TIMESTAMP: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_RECV_SENSITIVITY {
    pub ucDataRate: u8,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOT11_RECV_SENSITIVITY_LIST {
    pub Anonymous: DOT11_RECV_SENSITIVITY_LIST_0,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RecvSensitivity: [DOT11_RECV_SENSITIVITY; 1],
}
impl Default for DOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOT11_RECV_SENSITIVITY_LIST_0 {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uPhyId: u32,
}
impl Default for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RegDomainValue: [DOT11_REG_DOMAIN_VALUE; 1],
}
impl Default for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_REG_DOMAIN_DOC: u32 = 32;
pub const DOT11_REG_DOMAIN_ETSI: u32 = 48;
pub const DOT11_REG_DOMAIN_FCC: u32 = 16;
pub const DOT11_REG_DOMAIN_FRANCE: u32 = 50;
pub const DOT11_REG_DOMAIN_MKK: u32 = 64;
pub const DOT11_REG_DOMAIN_OTHER: u32 = 0;
pub const DOT11_REG_DOMAIN_SPAIN: u32 = 49;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_REG_DOMAIN_VALUE {
    pub uRegDomainsSupportIndex: u32,
    pub uRegDomainsSupportValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_RESET_REQUEST {
    pub dot11ResetType: DOT11_RESET_TYPE,
    pub dot11MacAddress: DOT11_MAC_ADDRESS,
    pub bSetDefaultMIB: bool,
}
impl Default for DOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_RESET_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uStatus: DOT11_ASSOC_STATUS,
}
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_ROAMING_REASON_ADHOC: u32 = 13;
pub const DOT11_ROAMING_REASON_ASSOCIATION_LOST: u32 = 12;
pub const DOT11_ROAMING_REASON_BETTER_AP_FOUND: u32 = 11;
pub const DOT11_ROAMING_REASON_IHV_END: i32 = -1;
pub const DOT11_ROAMING_REASON_IHV_START: i32 = -2147483648;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_ROAMING_START_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub AdhocBSSID: DOT11_MAC_ADDRESS,
    pub AdhocSSID: super::wlan::DOT11_SSID,
    pub uRoamingReason: DOT11_ASSOC_STATUS,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_RSSI_RANGE {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uRSSIMin: u32,
    pub uRSSIMax: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SCAN_REQUEST {
    pub dot11BSSType: super::wlan::DOT11_BSS_TYPE,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11SSID: super::wlan::DOT11_SSID,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: bool,
    pub bUseRequestIE: bool,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypesOffset: u32,
    pub uNumOfPhyTypes: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SCAN_REQUEST_V2 {
    pub dot11BSSType: super::wlan::DOT11_BSS_TYPE,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: bool,
    pub udot11SSIDsOffset: u32,
    pub uNumOfdot11SSIDs: u32,
    pub bUseRequestIE: bool,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypeInfosOffset: u32,
    pub uNumOfPhyTypeInfos: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_SCAN_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub ResponseContext: *mut core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub uSendTimeout: u32,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: DOT11_MAC_ADDRESS,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: DOT11_MAC_ADDRESS,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub uSendTimeout: u32,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub InvitationFlags: DOT11_WFD_INVITATION_FLAGS,
    pub GroupBSSID: DOT11_MAC_ADDRESS,
    pub bUseGroupBSSID: bool,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: bool,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bLocalGO: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: DOT11_WFD_STATUS_CODE,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub GroupBSSID: DOT11_MAC_ADDRESS,
    pub bUseGroupBSSID: bool,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub PeerDeviceAddress: DOT11_MAC_ADDRESS,
    pub uSendTimeout: u32,
    pub GroupCapability: DOT11_WFD_GROUP_CAPABILITY,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: bool,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: DOT11_MAC_ADDRESS,
    pub DialogToken: DOT11_DIALOG_TOKEN,
    pub RequestContext: *mut core::ffi::c_void,
    pub uSendTimeout: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: u32 = 0;
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: u32 = 1;
pub const DOT11_SIZEOF_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 12;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SSID_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SSIDs: [super::wlan::DOT11_SSID; 1],
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for DOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SSID_LIST_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_START_REQUEST {
    pub uStartFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_STATISTICS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub PhyCounters: [DOT11_PHY_FRAME_STATISTICS; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_STATISTICS_REVISION_1: u32 = 1;
pub const DOT11_STATISTICS_UNKNOWN: i32 = -1;
pub const DOT11_STATUS_AP_JOIN_CONFIRM: u32 = 5;
pub const DOT11_STATUS_AUTH_FAILED: u32 = 131072;
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: u32 = 32768;
pub const DOT11_STATUS_AUTH_VERIFIED: u32 = 65536;
pub const DOT11_STATUS_ENCRYPTION_FAILED: u32 = 512;
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: u32 = 256;
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: u32 = 16384;
pub const DOT11_STATUS_ICV_VERIFIED: u32 = 2048;
#[repr(C)]
#[cfg(feature = "Win32_types")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_STATUS_INDICATION {
    pub uStatusType: u32,
    pub ndisStatus: super::types::NDIS_STATUS,
}
pub const DOT11_STATUS_JOIN_CONFIRM: u32 = 2;
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: u32 = 6;
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: u32 = 8192;
pub const DOT11_STATUS_PACKET_REASSEMBLED: u32 = 4096;
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: u32 = 262144;
pub const DOT11_STATUS_RESET_CONFIRM: u32 = 4;
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: u32 = 2;
pub const DOT11_STATUS_SCAN_CONFIRM: u32 = 1;
pub const DOT11_STATUS_START_CONFIRM: u32 = 3;
pub const DOT11_STATUS_SUCCESS: u32 = 1;
pub const DOT11_STATUS_UNAVAILABLE_BSS: u32 = 128;
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: u32 = 16;
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: u32 = 32;
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: u32 = 4;
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: u32 = 8;
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: u32 = 1024;
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: u32 = 64;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_STOP_AP_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: u32 = 1;
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: u32 = 3;
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: u32 = 2;
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: u32 = 1;
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295;
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_SUPPORTED_ANTENNA {
    pub uAntennaListIndex: u32,
    pub bSupportedAntenna: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_ANTENNA_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedAntenna: [DOT11_SUPPORTED_ANTENNA; 1],
}
impl Default for DOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE {
    pub ucSupportedTxDataRatesValue: [u8; 8],
    pub ucSupportedRxDataRatesValue: [u8; 8],
}
impl Default for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_SUPPORTED_DATA_RATES_VALUE_V1 = DOT11_SUPPORTED_DATA_RATES_VALUE_V2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    pub ucSupportedTxDataRatesValue: [u8; 255],
    pub ucSupportedRxDataRatesValue: [u8; 255],
}
impl Default for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL {
    pub uChannel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedDSSSChannel: [DOT11_SUPPORTED_DSSS_CHANNEL; 1],
}
impl Default for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY {
    pub uCenterFrequency: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedOFDMFrequency: [DOT11_SUPPORTED_OFDM_FREQUENCY; 1],
}
impl Default for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_PHY_TYPES {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PHYType: [DOT11_PHY_TYPE; 1],
}
impl Default for DOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_SUPPORTED_POWER_LEVELS {
    pub uNumOfSupportedPowerLevels: u32,
    pub uTxPowerLevelValues: [u32; 8],
}
impl Default for DOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_TEMP_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bDefaultKeyFailure: bool,
    pub uKeyIndex: u32,
    pub PeerMac: DOT11_MAC_ADDRESS,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_UPDATE_IE {
    pub dot11UpdateIEOp: DOT11_UPDATE_IE_OP,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl Default for DOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOT11_UPDATE_IE_OP = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_VWIFI_ATTRIBUTES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uTotalNumOfEntries: u32,
    pub Combinations: [DOT11_VWIFI_COMBINATION; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
}
pub const DOT11_VWIFI_COMBINATION_REVISION_1: u32 = 1;
pub const DOT11_VWIFI_COMBINATION_REVISION_2: u32 = 2;
pub const DOT11_VWIFI_COMBINATION_REVISION_3: u32 = 3;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION_V2 {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_VWIFI_COMBINATION_V3 {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
    pub uNumWFDGroup: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::winnt::HANDLE,
    pub hOffload: super::winnt::HANDLE,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub bRowIsOutbound: bool,
    pub bUseDefault: bool,
    pub uFlags: u32,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnPeer: u32,
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::winnt::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_ADDITIONAL_IE {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
    pub uDefaultRequestIEsOffset: u32,
    pub uDefaultRequestIEsLength: u32,
}
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    pub AdvertisementID: u32,
    pub ConfigMethods: u16,
    pub ServiceNameLength: u8,
    pub ServiceName: [u8; 255],
}
impl Default for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST {
    pub ServiceCount: u16,
    pub AdvertisedService: [DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR; 1],
}
impl Default for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_ADVERTISEMENT_ID {
    pub AdvertisementID: u32,
    pub ServiceAddress: DOT11_MAC_ADDRESS,
}
impl Default for DOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: u32 = 21;
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: u32 = 63;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_ATTRIBUTES {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumConcurrentGORole: u32,
    pub uNumConcurrentClientRole: u32,
    pub WPSVersionsSupported: u32,
    pub bServiceDiscoverySupported: bool,
    pub bClientDiscoverabilitySupported: bool,
    pub bInfrastructureManagementSupported: bool,
    pub uMaxSecondaryDeviceTypeListSize: u32,
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub uInterfaceAddressListCount: u32,
    pub pInterfaceAddressList: PDOT11_MAC_ADDRESS,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: PDOT11_COUNTRY_OR_REGION_STRING,
    pub uDiscoveryFilterListSize: u32,
    pub uGORoleClientTableSize: u32,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_CHANNEL {
    pub CountryRegionString: DOT11_COUNTRY_OR_REGION_STRING,
    pub OperatingClass: u8,
    pub ChannelNumber: u8,
}
impl Default for DOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT {
    pub GOTimeout: u8,
    pub ClientTimeout: u8,
}
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: u32 = 16;
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: u32 = 4;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bServiceDiscoveryEnabled: bool,
    pub bClientDiscoverabilityEnabled: bool,
    pub bConcurrentOperationSupported: bool,
    pub bInfrastructureManagementEnabled: bool,
    pub bDeviceLimitReached: bool,
    pub bInvitationProcedureEnabled: bool,
    pub WPSVersionsEnabled: u32,
}
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: u32 = 1;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: u32 = 2;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: u32 = 16;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: u32 = 8;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: u32 = 32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: u32 = 64;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: u32 = 128;
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy)]
pub struct DOT11_WFD_DEVICE_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: DOT11_MAC_ADDRESS,
    pub dot11BSSType: super::wlan::DOT11_BSS_TYPE,
    pub TransmitterAddress: DOT11_MAC_ADDRESS,
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullBeaconHostTimestamp: u64,
    pub ullProbeResponseHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: u32 = 24;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_DEVICE_INFO {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub ConfigMethods: u16,
    pub PrimaryDeviceType: DOT11_WFD_DEVICE_TYPE,
    pub DeviceName: DOT11_WPS_DEVICE_NAME,
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub ChannelNumber: u8,
}
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 1;
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_DEVICE_TYPE {
    pub CategoryID: u16,
    pub SubCategoryID: u16,
    pub OUI: [u8; 4],
}
impl Default for DOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: u32 = 128;
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub Status: super::types::NDIS_STATUS,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uListOffset: u32,
    pub uListLength: u32,
}
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER {
    pub DeviceID: DOT11_MAC_ADDRESS,
    pub ucBitmask: u8,
    pub GroupSSID: super::wlan::DOT11_SSID,
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_DISCOVER_REQUEST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub DiscoverType: DOT11_WFD_DISCOVER_TYPE,
    pub ScanType: DOT11_WFD_SCAN_TYPE,
    pub uDiscoverTimeout: u32,
    pub uDeviceFilterListOffset: u32,
    pub uNumDeviceFilters: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub bForceScanLegacyNetworks: bool,
}
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 1;
pub type DOT11_WFD_DISCOVER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_GO_INTENT {
    pub _bitfield: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DOT11_WFD_GROUP_CAPABILITY(pub u8);
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: u32 = 16;
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: u32 = 128;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: u32 = 4;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: u32 = 1;
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: u32 = 8;
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: u32 = 64;
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: u32 = 0;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: u32 = 2;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: u32 = 32;
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: u32 = 128;
#[repr(C)]
#[cfg(feature = "Win32_wlan")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_GROUP_ID {
    pub DeviceAddress: DOT11_MAC_ADDRESS,
    pub SSID: super::wlan::DOT11_SSID,
}
#[cfg(feature = "Win32_wlan")]
impl Default for DOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub GOOperatingChannel: DOT11_WFD_CHANNEL,
    pub GOConfigTime: u32,
    pub bInGroupFormation: bool,
    pub bWaitForWPSReady: bool,
}
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: bool,
    pub bIntraBSSDistributionSupported: bool,
    pub bCrossConnectionSupported: bool,
    pub bPersistentReconnectSupported: bool,
    pub bGroupFormationEnabled: bool,
    pub uMaximumGroupLimit: u32,
}
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: u32 = 1;
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: bool,
    pub bIntraBSSDistributionSupported: bool,
    pub bCrossConnectionSupported: bool,
    pub bPersistentReconnectSupported: bool,
    pub bGroupFormationEnabled: bool,
    pub uMaximumGroupLimit: u32,
    pub bEapolKeyIpAddressAllocationSupported: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_GROUP_START_PARAMETERS {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub AdvertisedOperatingChannel: DOT11_WFD_CHANNEL,
}
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WFD_INVITATION_FLAGS {
    pub _bitfield: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DOT11_WFD_MINOR_REASON_CODE(pub u8);
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: u32 = 1;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: u32 = 4;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: u32 = 2;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: u32 = 3;
pub const DOT11_WFD_MINOR_REASON_SUCCESS: u32 = 0;
pub type DOT11_WFD_SCAN_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_objectheader")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    pub Header: super::objectheader::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SecondaryDeviceTypes: [DOT11_WFD_DEVICE_TYPE; 1],
}
#[cfg(feature = "Win32_objectheader")]
impl Default for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 1;
pub type DOT11_WFD_SERVICE_HASH = [u8; 6];
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_SERVICE_HASH_LIST {
    pub ServiceHashCount: u16,
    pub ServiceHash: [DOT11_WFD_SERVICE_HASH; 1],
}
impl Default for DOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: u32 = 65535;
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_SESSION_ID {
    pub SessionID: u32,
    pub SessionAddress: DOT11_MAC_ADDRESS,
}
impl Default for DOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WFD_SESSION_INFO {
    pub uSessionInfoLength: u16,
    pub ucSessionInfo: [u8; 144],
}
impl Default for DOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: u32 = 144;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DOT11_WFD_STATUS_CODE(pub u8);
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: u32 = 2;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: u32 = 10;
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: u32 = 1;
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: u32 = 4;
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: u32 = 3;
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: u32 = 9;
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: u32 = 7;
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: u32 = 6;
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: u32 = 11;
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: u32 = 5;
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: u32 = 8;
pub const DOT11_WFD_STATUS_SUCCESS: u32 = 0;
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WME_AC_PARAMETERS {
    pub ucAccessCategoryIndex: u8,
    pub ucAIFSN: u8,
    pub ucECWmin: u8,
    pub ucECWmax: u8,
    pub usTXOPLimit: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WME_AC_PARAMETERS_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11WMEACParameters: [DOT11_WME_AC_PARAMETERS; 1],
}
impl Default for DOT11_WME_AC_PARAMETERS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WME_PACKET: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WME_UPDATE_IE {
    pub uParamElemMinBeaconIntervals: u32,
    pub uWMEInfoElemOffset: u32,
    pub uWMEInfoElemLength: u32,
    pub uWMEParamElemOffset: u32,
    pub uWMEParamElemLength: u32,
    pub ucBuffer: [u8; 1],
}
impl Default for DOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOT11_WPA_TSC {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::winnt::HANDLE,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOT11_WPS_DEVICE_NAME {
    pub uDeviceNameLength: u32,
    pub ucDeviceName: [u8; 32],
}
impl Default for DOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: u32 = 32;
pub type DOT11_WPS_DEVICE_PASSWORD_ID = i32;
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: u32 = 32;
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: u32 = 32;
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: u32 = 8;
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
pub const DOT11_WPS_VERSION_1_0: u32 = 1;
pub const DOT11_WPS_VERSION_2_0: u32 = 2;
pub const MAX_NUM_SUPPORTED_RATES: u32 = 8;
pub const MAX_NUM_SUPPORTED_RATES_V2: u32 = 255;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: u32 = 4;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_MGMT: u32 = 1048576;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_CTRL: u32 = 16777216;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: u32 = 8;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_MGMT: u32 = 262144;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_CTRL: u32 = 8388608;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: u32 = 1;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_MGMT: u32 = 131072;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: u32 = 2;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_MGMT: u32 = 524288;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_CTRL: u32 = 33554432;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: u32 = 32;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_MGMT: u32 = 2097152;
pub const NDIS_PACKET_TYPE_802_11_RAW_DATA: u32 = 65536;
pub const NDIS_PACKET_TYPE_802_11_RAW_MGMT: u32 = 4194304;
pub const NDIS_PACKET_TYPE_ALL_802_11_FILTERS: u32 = 67043375;
pub const NDIS_PACKET_TYPE_MEDIA_SPECIFIC_MASK: u32 = 268369920;
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
pub const OID_DOT11_AP_JOIN_REQUEST: u32 = 218170205;
pub const OID_DOT11_ASSOCIATION_PARAMS: u32 = 234946975;
pub const OID_DOT11_ATIM_WINDOW: u32 = 218170122;
pub const OID_DOT11_AUTO_CONFIG_ENABLED: u32 = 234946936;
pub const OID_DOT11_AVAILABLE_CHANNEL_LIST: u32 = 235077891;
pub const OID_DOT11_AVAILABLE_FREQUENCY_LIST: u32 = 235077892;
pub const OID_DOT11_BEACON_PERIOD: u32 = 218170139;
pub const OID_DOT11_CCA_MODE_SUPPORTED: u32 = 218170166;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: u32 = 218170170;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: u32 = 218170172;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: u32 = 218170169;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: u32 = 218170171;
pub const OID_DOT11_CFP_MAX_DURATION: u32 = 218170136;
pub const OID_DOT11_CFP_PERIOD: u32 = 218170135;
pub const OID_DOT11_CF_POLLABLE: u32 = 218170134;
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: u32 = 218170184;
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: u32 = 218170183;
pub const OID_DOT11_CIPHER_DEFAULT_KEY: u32 = 234946955;
pub const OID_DOT11_CIPHER_DEFAULT_KEY_ID: u32 = 234946954;
pub const OID_DOT11_CIPHER_KEY_MAPPING_KEY: u32 = 234946956;
pub const OID_DOT11_CONNECT_REQUEST: u32 = 234946945;
pub const OID_DOT11_COUNTERS_ENTRY: u32 = 218170149;
pub const OID_DOT11_COUNTRY_STRING: u32 = 218170188;
pub const OID_DOT11_CREATE_MAC: u32 = 235143425;
pub const OID_DOT11_CURRENT_ADDRESS: u32 = 218171138;
pub const OID_DOT11_CURRENT_CCA_MODE: u32 = 218170167;
pub const OID_DOT11_CURRENT_CHANNEL: u32 = 218170165;
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: u32 = 218170159;
pub const OID_DOT11_CURRENT_DWELL_TIME: u32 = 218170161;
pub const OID_DOT11_CURRENT_FREQUENCY: u32 = 218170178;
pub const OID_DOT11_CURRENT_INDEX: u32 = 218170164;
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: u32 = 218170113;
pub const OID_DOT11_CURRENT_OPERATION_MODE: u32 = 218170120;
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: u32 = 218170131;
pub const OID_DOT11_CURRENT_PACKET_FILTER: u32 = 218170121;
pub const OID_DOT11_CURRENT_PATTERN: u32 = 218170163;
pub const OID_DOT11_CURRENT_PHY_ID: u32 = 234946962;
pub const OID_DOT11_CURRENT_PHY_TYPE: u32 = 218170124;
pub const OID_DOT11_CURRENT_REG_DOMAIN: u32 = 218170151;
pub const OID_DOT11_CURRENT_RX_ANTENNA: u32 = 218170155;
pub const OID_DOT11_CURRENT_SET: u32 = 218170162;
pub const OID_DOT11_CURRENT_TX_ANTENNA: u32 = 218170153;
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: u32 = 218170157;
pub const OID_DOT11_DATA_RATE_MAPPING_TABLE: u32 = 234946967;
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: u32 = 218170116;
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: u32 = 218170117;
pub const OID_DOT11_DELETE_MAC: u32 = 235143426;
pub const OID_DOT11_DESIRED_BSSID_LIST: u32 = 234946942;
pub const OID_DOT11_DESIRED_BSS_TYPE: u32 = 234946943;
pub const OID_DOT11_DESIRED_COUNTRY_OR_REGION_STRING: u32 = 234946969;
pub const OID_DOT11_DESIRED_PHY_LIST: u32 = 234946961;
pub const OID_DOT11_DESIRED_SSID_LIST: u32 = 234946940;
pub const OID_DOT11_DISASSOCIATE_PEER_REQUEST: u32 = 235077893;
pub const OID_DOT11_DISCONNECT_REQUEST: u32 = 234946958;
pub const OID_DOT11_DIVERSITY_SELECTION_RX: u32 = 218170176;
pub const OID_DOT11_DIVERSITY_SUPPORT: u32 = 218170154;
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: u32 = 218170209;
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: u32 = 218170208;
pub const OID_DOT11_DTIM_PERIOD: u32 = 218170140;
pub const OID_DOT11_ED_THRESHOLD: u32 = 218170168;
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: u32 = 218170193;
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: u32 = 218170192;
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: u32 = 218170191;
pub const OID_DOT11_EHCC_PRIME_RADIX: u32 = 218170190;
pub const OID_DOT11_ENABLED_AUTHENTICATION_ALGORITHM: u32 = 234946949;
pub const OID_DOT11_ENABLED_MULTICAST_CIPHER_ALGORITHM: u32 = 234946953;
pub const OID_DOT11_ENABLED_UNICAST_CIPHER_ALGORITHM: u32 = 234946951;
pub const OID_DOT11_ENUM_ASSOCIATION_INFO: u32 = 234946957;
pub const OID_DOT11_ENUM_BSS_LIST: u32 = 234946937;
pub const OID_DOT11_ENUM_PEER_INFO: u32 = 235077896;
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: u32 = 218170207;
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: u32 = 218170206;
pub const OID_DOT11_EXCLUDED_MAC_ADDRESS_LIST: u32 = 234946941;
pub const OID_DOT11_EXCLUDE_UNENCRYPTED: u32 = 234946946;
pub const OID_DOT11_EXTSTA_CAPABILITY: u32 = 234946966;
pub const OID_DOT11_FLUSH_BSS_LIST: u32 = 234946938;
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: u32 = 218170146;
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: u32 = 218170180;
pub const OID_DOT11_HARDWARE_PHY_STATE: u32 = 234946960;
pub const OID_DOT11_HIDDEN_NETWORK_ENABLED: u32 = 234946974;
pub const OID_DOT11_HOPPING_PATTERN: u32 = 218170199;
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: u32 = 218170194;
pub const OID_DOT11_HOP_MODULUS: u32 = 218170197;
pub const OID_DOT11_HOP_OFFSET: u32 = 218170198;
pub const OID_DOT11_HOP_TIME: u32 = 218170158;
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: u32 = 218170185;
pub const OID_DOT11_IBSS_PARAMS: u32 = 234946971;
pub const OID_DOT11_INCOMING_ASSOCIATION_DECISION: u32 = 235077894;
pub const OID_DOT11_JOIN_REQUEST: u32 = 218170125;
pub const OID_DOT11_LONG_RETRY_LIMIT: u32 = 218170145;
pub const OID_DOT11_MAC_ADDRESS: u32 = 218170142;
pub const OID_DOT11_MANUFACTURING_TEST: u32 = 235405825;
pub const OID_DOT11_MAXIMUM_LIST_SIZE: u32 = 218171141;
pub const OID_DOT11_MAX_DWELL_TIME: u32 = 218170160;
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: u32 = 218170212;
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: u32 = 218170148;
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: u32 = 218170147;
pub const OID_DOT11_MEDIA_STREAMING_ENABLED: u32 = 234946963;
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: u32 = 218170133;
pub const OID_DOT11_MPDU_MAX_LENGTH: u32 = 218170118;
pub const OID_DOT11_MULTICAST_LIST: u32 = 218171140;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: u32 = 218170189;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: u32 = 218170187;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: u32 = 218170186;
pub const OID_DOT11_NDIS_START: u32 = 218170112;
pub const OID_DOT11_NIC_POWER_STATE: u32 = 218170129;
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: u32 = 218170204;
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: u32 = 218170196;
pub const OID_DOT11_OFFLOAD_CAPABILITY: u32 = 218170112;
pub const OID_DOT11_OFFLOAD_NETWORK_LIST: u32 = 235340035;
pub const OID_DOT11_OPERATIONAL_RATE_SET: u32 = 218170138;
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: u32 = 218170119;
pub const OID_DOT11_OPTIONAL_CAPABILITY: u32 = 218170130;
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: u32 = 218170182;
pub const OID_DOT11_PERMANENT_ADDRESS: u32 = 218171139;
pub const OID_DOT11_PMKID_LIST: u32 = 234947200;
pub const OID_DOT11_PORT_STATE_NOTIFICATION: u32 = 234947226;
pub const OID_DOT11_POWER_MGMT_MODE: u32 = 218170137;
pub const OID_DOT11_POWER_MGMT_MODE_AUTO_ENABLED: u32 = 235340033;
pub const OID_DOT11_POWER_MGMT_MODE_STATUS: u32 = 235340034;
pub const OID_DOT11_POWER_MGMT_REQUEST: u32 = 234946939;
pub const OID_DOT11_PREFERRED_MAC: u32 = 235143427;
pub const OID_DOT11_PRIVACY_EXEMPTION_LIST: u32 = 234946948;
pub const OID_DOT11_PRIVATE_OIDS_START: u32 = 218171136;
pub const OID_DOT11_QOS_PARAMS: u32 = 234947228;
pub const OID_DOT11_QOS_TX_DURATION: u32 = 218170219;
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: u32 = 218170220;
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: u32 = 218170218;
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: u32 = 218170200;
pub const OID_DOT11_RANDOM_TABLE_FLAG: u32 = 218170195;
pub const OID_DOT11_RECV_SENSITIVITY_LIST: u32 = 218170213;
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: u32 = 218170173;
pub const OID_DOT11_RESET_REQUEST: u32 = 218170128;
pub const OID_DOT11_RF_USAGE: u32 = 218170203;
pub const OID_DOT11_RSSI_RANGE: u32 = 218170202;
pub const OID_DOT11_RTS_THRESHOLD: u32 = 218170143;
pub const OID_DOT11_SAFE_MODE_ENABLED: u32 = 234946973;
pub const OID_DOT11_SAFE_MODE_HT_ENABLED: u32 = 234946976;
pub const OID_DOT11_SCAN_REQUEST: u32 = 218170123;
pub const OID_DOT11_SET_FT_REASSOCIATION_PARAMETERS: u32 = 234946920;
pub const OID_DOT11_SET_NWF_PMKID_LIST: u32 = 234946933;
pub const OID_DOT11_SET_SAE_AUTH_PARAMS: u32 = 234946930;
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: u32 = 218170181;
pub const OID_DOT11_SHORT_RETRY_LIMIT: u32 = 218170144;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: u32 = 218170211;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: u32 = 218170210;
pub const OID_DOT11_START_AP_REQUEST: u32 = 235077890;
pub const OID_DOT11_START_REQUEST: u32 = 218170126;
pub const OID_DOT11_STATION_ID: u32 = 218170132;
pub const OID_DOT11_STATISTICS: u32 = 235012483;
pub const OID_DOT11_SUPPORTED_COUNTRY_OR_REGION_STRING: u32 = 234946968;
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: u32 = 218170177;
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: u32 = 218170222;
pub const OID_DOT11_SUPPORTED_MULTICAST_ALGORITHM_PAIR: u32 = 234946952;
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: u32 = 218170221;
pub const OID_DOT11_SUPPORTED_PHY_TYPES: u32 = 218170150;
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: u32 = 218170156;
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: u32 = 218170175;
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: u32 = 218170174;
pub const OID_DOT11_SUPPORTED_UNICAST_ALGORITHM_PAIR: u32 = 234946950;
pub const OID_DOT11_TEMP_TYPE: u32 = 218170152;
pub const OID_DOT11_TI_THRESHOLD: u32 = 218170179;
pub const OID_DOT11_UNICAST_USE_GROUP_ENABLED: u32 = 234946959;
pub const OID_DOT11_UNREACHABLE_DETECTION_THRESHOLD: u32 = 234946964;
pub const OID_DOT11_UPDATE_IE: u32 = 218170127;
pub const OID_DOT11_VIRTUAL_STATION_CAPABILITY: u32 = 235143684;
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: u32 = 218170141;
pub const OID_DOT11_WEP_OFFLOAD: u32 = 218170114;
pub const OID_DOT11_WEP_UPLOAD: u32 = 218170115;
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
pub const OID_DOT11_WME_AC_PARAMETERS: u32 = 218170216;
pub const OID_DOT11_WME_ENABLED: u32 = 218170215;
pub const OID_DOT11_WME_IMPLEMENTED: u32 = 218170214;
pub const OID_DOT11_WME_UPDATE_IE: u32 = 218170217;
pub const OID_DOT11_WPA_TSC: u32 = 218170201;
pub const OID_DOT11_WPS_ENABLED: u32 = 235077889;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCH_DESCRIPTION_TYPE(pub *mut CH_DESCRIPTION_TYPE);
impl PCH_DESCRIPTION_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCH_DESCRIPTION_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AC_PARAM(pub *mut DOT11_AC_PARAM);
impl PDOT11_AC_PARAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_AC_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ADDITIONAL_IE(pub *mut DOT11_ADDITIONAL_IE);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_ADDITIONAL_IE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ANQP_QUERY_COMPLETE_PARAMETERS(pub *mut DOT11_ANQP_QUERY_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_winnt"))]
impl PDOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_winnt"))]
impl Default for PDOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ANQP_QUERY_RESULT(pub *mut DOT11_ANQP_QUERY_RESULT);
impl PDOT11_ANQP_QUERY_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_ANQP_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AP_JOIN_REQUEST(pub *mut DOT11_AP_JOIN_REQUEST);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_AP_JOIN_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_AP_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_COMPLETION_PARAMETERS(pub *mut DOT11_ASSOCIATION_COMPLETION_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_INFO_EX(pub *mut DOT11_ASSOCIATION_INFO_EX);
impl PDOT11_ASSOCIATION_INFO_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_INFO_LIST(pub *mut DOT11_ASSOCIATION_INFO_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_ASSOCIATION_INFO_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_PARAMS(pub *mut DOT11_ASSOCIATION_PARAMS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_ASSOCIATION_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_START_PARAMETERS(pub *mut DOT11_ASSOCIATION_START_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_ASSOCIATION_START_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ASSOCIATION_STATE(pub *mut DOT11_ASSOCIATION_STATE);
impl PDOT11_ASSOCIATION_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_ASSOCIATION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AUTH_ALGORITHM_LIST(pub *mut DOT11_AUTH_ALGORITHM_LIST);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_AUTH_ALGORITHM_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AUTH_CIPHER_PAIR_LIST(pub *mut DOT11_AUTH_CIPHER_PAIR_LIST);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_AUTH_CIPHER_PAIR_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AVAILABLE_CHANNEL_LIST(pub *mut DOT11_AVAILABLE_CHANNEL_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_AVAILABLE_CHANNEL_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_AVAILABLE_FREQUENCY_LIST(pub *mut DOT11_AVAILABLE_FREQUENCY_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_AVAILABLE_FREQUENCY_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BAND(pub *mut DOT11_BAND);
impl PDOT11_BAND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_BAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BSSID_CANDIDATE(pub *mut DOT11_BSSID_CANDIDATE);
impl PDOT11_BSSID_CANDIDATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BSSID_LIST(pub *mut DOT11_BSSID_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_BSSID_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BSS_DESCRIPTION(pub *mut DOT11_BSS_DESCRIPTION);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_BSS_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BSS_ENTRY(pub *mut DOT11_BSS_ENTRY);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_BSS_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BSS_ENTRY_PHY_SPECIFIC_INFO(pub *mut DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO);
impl PDOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_BYTE_ARRAY(pub *mut DOT11_BYTE_ARRAY);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_BYTE_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CAN_SUSTAIN_AP_PARAMETERS(pub *mut DOT11_CAN_SUSTAIN_AP_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_CAN_SUSTAIN_AP_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CHANNEL_HINT(pub *mut DOT11_CHANNEL_HINT);
impl PDOT11_CHANNEL_HINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_CHANNEL_HINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CIPHER_ALGORITHM_LIST(pub *mut DOT11_CIPHER_ALGORITHM_LIST);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_CIPHER_ALGORITHM_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CIPHER_DEFAULT_KEY_VALUE(pub *mut DOT11_CIPHER_DEFAULT_KEY_VALUE);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_CIPHER_DEFAULT_KEY_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE(pub *mut DOT11_CIPHER_KEY_MAPPING_KEY_VALUE);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CONNECTION_COMPLETION_PARAMETERS(pub *mut DOT11_CONNECTION_COMPLETION_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_CONNECTION_COMPLETION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CONNECTION_START_PARAMETERS(pub *mut DOT11_CONNECTION_START_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_CONNECTION_START_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_COUNTERS_ENTRY(pub *mut DOT11_COUNTERS_ENTRY);
impl PDOT11_COUNTERS_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_COUNTERS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_COUNTRY_OR_REGION_STRING(pub *mut DOT11_COUNTRY_OR_REGION_STRING);
impl PDOT11_COUNTRY_OR_REGION_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_COUNTRY_OR_REGION_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_COUNTRY_OR_REGION_STRING_LIST(pub *mut DOT11_COUNTRY_OR_REGION_STRING_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_COUNTRY_OR_REGION_STRING_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CURRENT_OFFLOAD_CAPABILITY(pub *mut DOT11_CURRENT_OFFLOAD_CAPABILITY);
impl PDOT11_CURRENT_OFFLOAD_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CURRENT_OPERATION_MODE(pub *mut DOT11_CURRENT_OPERATION_MODE);
impl PDOT11_CURRENT_OPERATION_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_CURRENT_OPERATION_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_CURRENT_OPTIONAL_CAPABILITY(pub *mut DOT11_CURRENT_OPTIONAL_CAPABILITY);
impl PDOT11_CURRENT_OPTIONAL_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DATA_RATE_MAPPING_ENTRY(pub *mut DOT11_DATA_RATE_MAPPING_ENTRY);
impl PDOT11_DATA_RATE_MAPPING_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DATA_RATE_MAPPING_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DATA_RATE_MAPPING_TABLE(pub *mut DOT11_DATA_RATE_MAPPING_TABLE);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_DATA_RATE_MAPPING_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DEFAULT_WEP_OFFLOAD(pub *mut DOT11_DEFAULT_WEP_OFFLOAD);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_DEFAULT_WEP_OFFLOAD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DEFAULT_WEP_UPLOAD(pub *mut DOT11_DEFAULT_WEP_UPLOAD);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_DEFAULT_WEP_UPLOAD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DIRECTION(pub *mut DOT11_DIRECTION);
impl PDOT11_DIRECTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DIRECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DISASSOCIATE_PEER_REQUEST(pub *mut DOT11_DISASSOCIATE_PEER_REQUEST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_DISASSOCIATE_PEER_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DISASSOCIATION_PARAMETERS(pub *mut DOT11_DISASSOCIATION_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_DISASSOCIATION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DIVERSITY_SELECTION_RX(pub *mut DOT11_DIVERSITY_SELECTION_RX);
impl PDOT11_DIVERSITY_SELECTION_RX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DIVERSITY_SELECTION_RX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DIVERSITY_SELECTION_RX_LIST(pub *mut DOT11_DIVERSITY_SELECTION_RX_LIST);
impl PDOT11_DIVERSITY_SELECTION_RX_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DIVERSITY_SELECTION_RX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DIVERSITY_SUPPORT(pub *mut DOT11_DIVERSITY_SUPPORT);
impl PDOT11_DIVERSITY_SUPPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DIVERSITY_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_DS_INFO(pub *mut DOT11_DS_INFO);
impl PDOT11_DS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_DS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ENCAP_ENTRY(pub *mut DOT11_ENCAP_ENTRY);
impl PDOT11_ENCAP_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_ENCAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ERP_PHY_ATTRIBUTES(pub *mut DOT11_ERP_PHY_ATTRIBUTES);
impl PDOT11_ERP_PHY_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_ERP_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTAP_ATTRIBUTES(pub *mut DOT11_EXTAP_ATTRIBUTES);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_EXTAP_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_EXTAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTAP_RECV_CONTEXT(pub *mut DOT11_EXTSTA_RECV_CONTEXT);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_EXTAP_RECV_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_EXTAP_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTAP_SEND_CONTEXT(pub *mut DOT11_EXTSTA_SEND_CONTEXT);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_EXTAP_SEND_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_EXTAP_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTSTA_ATTRIBUTES(pub *mut DOT11_EXTSTA_ATTRIBUTES);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_EXTSTA_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_EXTSTA_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTSTA_CAPABILITY(pub *mut DOT11_EXTSTA_CAPABILITY);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_EXTSTA_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_EXTSTA_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTSTA_RECV_CONTEXT(pub *mut DOT11_EXTSTA_RECV_CONTEXT);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_EXTSTA_RECV_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_EXTSTA_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_EXTSTA_SEND_CONTEXT(pub *mut DOT11_EXTSTA_SEND_CONTEXT);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_EXTSTA_SEND_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_EXTSTA_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_FRAGMENT_DESCRIPTOR(pub *mut DOT11_FRAGMENT_DESCRIPTOR);
impl PDOT11_FRAGMENT_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_FRAGMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_HESSID(pub *mut DOT11_HESSID);
impl PDOT11_HESSID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_HESSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_HOPPING_PATTERN_ENTRY(pub *mut DOT11_HOPPING_PATTERN_ENTRY);
impl PDOT11_HOPPING_PATTERN_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_HOPPING_PATTERN_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_HOPPING_PATTERN_ENTRY_LIST(pub *mut DOT11_HOPPING_PATTERN_ENTRY_LIST);
impl PDOT11_HOPPING_PATTERN_ENTRY_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_HOP_ALGO_ADOPTED(pub *mut DOT11_HOP_ALGO_ADOPTED);
impl PDOT11_HOP_ALGO_ADOPTED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_HOP_ALGO_ADOPTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_HRDSSS_PHY_ATTRIBUTES(pub *mut DOT11_HRDSSS_PHY_ATTRIBUTES);
impl PDOT11_HRDSSS_PHY_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_HRDSSS_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_IBSS_PARAMS(pub *mut DOT11_IBSS_PARAMS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_IBSS_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_IBSS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS(pub *mut DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INCOMING_ASSOC_DECISION(pub *mut DOT11_INCOMING_ASSOC_DECISION);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_INCOMING_ASSOC_DECISION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INCOMING_ASSOC_DECISION_V2(pub *mut DOT11_INCOMING_ASSOC_DECISION_V2);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_INCOMING_ASSOC_DECISION_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS(pub *mut DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INCOMING_ASSOC_STARTED_PARAMETERS(pub *mut DOT11_INCOMING_ASSOC_STARTED_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_IV48_COUNTER(pub *mut DOT11_IV48_COUNTER);
impl PDOT11_IV48_COUNTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_IV48_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_JOIN_REQUEST(pub *mut DOT11_JOIN_REQUEST);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_JOIN_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_BIP(pub *mut DOT11_KEY_ALGO_BIP);
impl PDOT11_KEY_ALGO_BIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_BIP_GMAC_256(pub *mut DOT11_KEY_ALGO_BIP_GMAC_256);
impl PDOT11_KEY_ALGO_BIP_GMAC_256 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_CCMP(pub *mut DOT11_KEY_ALGO_CCMP);
impl PDOT11_KEY_ALGO_CCMP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_GCMP(pub *mut DOT11_KEY_ALGO_GCMP);
impl PDOT11_KEY_ALGO_GCMP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_GCMP_256(pub *mut DOT11_KEY_ALGO_GCMP_256);
impl PDOT11_KEY_ALGO_GCMP_256 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_ALGO_TKIP_MIC(pub *mut DOT11_KEY_ALGO_TKIP_MIC);
impl PDOT11_KEY_ALGO_TKIP_MIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_KEY_DIRECTION(pub *mut DOT11_KEY_DIRECTION);
impl PDOT11_KEY_DIRECTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_KEY_DIRECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_LINK_QUALITY_ENTRY(pub *mut DOT11_LINK_QUALITY_ENTRY);
impl PDOT11_LINK_QUALITY_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_LINK_QUALITY_PARAMETERS(pub *mut DOT11_LINK_QUALITY_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_LINK_QUALITY_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_LINK_QUALITY_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MAC_ADDRESS(pub *mut DOT11_MAC_ADDRESS);
impl PDOT11_MAC_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MAC_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MAC_ADDRESS_LIST(pub *mut DOT11_MAC_ADDRESS_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_MAC_ADDRESS_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MAC_FRAME_STATISTICS(pub *mut DOT11_MAC_FRAME_STATISTICS);
impl PDOT11_MAC_FRAME_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MAC_INFO(pub *mut DOT11_MAC_INFO);
impl PDOT11_MAC_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MAC_PARAMETERS(pub *mut DOT11_MAC_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_MAC_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_MAC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_CALLBACK_PARAMETERS(pub *mut DOT11_MANUFACTURING_CALLBACK_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_CALLBACK_TYPE(pub *mut DOT11_MANUFACTURING_CALLBACK_TYPE);
impl PDOT11_MANUFACTURING_CALLBACK_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_CALLBACK_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC(pub *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC);
impl PDOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_FUNCTIONAL_TEST_RX(pub *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX);
impl PDOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_FUNCTIONAL_TEST_TX(pub *mut DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX);
impl PDOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS(pub *mut DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS);
impl PDOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_SELF_TEST_SET_PARAMS(pub *mut DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS);
impl PDOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_SELF_TEST_TYPE(pub *mut DOT11_MANUFACTURING_SELF_TEST_TYPE);
impl PDOT11_MANUFACTURING_SELF_TEST_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_TEST(pub *mut DOT11_MANUFACTURING_TEST);
impl PDOT11_MANUFACTURING_TEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_TEST_QUERY_DATA(pub *mut DOT11_MANUFACTURING_TEST_QUERY_DATA);
impl PDOT11_MANUFACTURING_TEST_QUERY_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_TEST_SET_DATA(pub *mut DOT11_MANUFACTURING_TEST_SET_DATA);
impl PDOT11_MANUFACTURING_TEST_SET_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_TEST_SLEEP(pub *mut DOT11_MANUFACTURING_TEST_SLEEP);
impl PDOT11_MANUFACTURING_TEST_SLEEP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_TEST_SLEEP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MANUFACTURING_TEST_TYPE(pub *mut DOT11_MANUFACTURING_TEST_TYPE);
impl PDOT11_MANUFACTURING_TEST_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MANUFACTURING_TEST_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MD_CAPABILITY_ENTRY_LIST(pub *mut DOT11_MD_CAPABILITY_ENTRY_LIST);
impl PDOT11_MD_CAPABILITY_ENTRY_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MPDU_MAX_LENGTH_INDICATION(pub *mut DOT11_MPDU_MAX_LENGTH_INDICATION);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_MPDU_MAX_LENGTH_INDICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_MPDU_MAX_LENGTH_INDICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_MULTI_DOMAIN_CAPABILITY_ENTRY(pub *mut DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY);
impl PDOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_NIC_SPECIFIC_EXTENSION(pub *mut DOT11_NIC_SPECIFIC_EXTENSION);
impl PDOT11_NIC_SPECIFIC_EXTENSION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFDM_PHY_ATTRIBUTES(pub *mut DOT11_OFDM_PHY_ATTRIBUTES);
impl PDOT11_OFDM_PHY_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_OFDM_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFFLOAD_CAPABILITY(pub *mut DOT11_OFFLOAD_CAPABILITY);
impl PDOT11_OFFLOAD_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFFLOAD_NETWORK(pub *mut DOT11_OFFLOAD_NETWORK);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_OFFLOAD_NETWORK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFFLOAD_NETWORK_LIST_INFO(pub *mut DOT11_OFFLOAD_NETWORK_LIST_INFO);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_OFFLOAD_NETWORK_LIST_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS(pub *mut DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OFFLOAD_TYPE(pub *mut DOT11_OFFLOAD_TYPE);
impl PDOT11_OFFLOAD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_OFFLOAD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OPERATION_MODE_CAPABILITY(pub *mut DOT11_OPERATION_MODE_CAPABILITY);
impl PDOT11_OPERATION_MODE_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_OPERATION_MODE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_OPTIONAL_CAPABILITY(pub *mut DOT11_OPTIONAL_CAPABILITY);
impl PDOT11_OPTIONAL_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PEER_INFO(pub *mut DOT11_PEER_INFO);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_PEER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PEER_INFO_LIST(pub *mut DOT11_PEER_INFO_LIST);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_PEER_INFO_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PEER_STATISTICS(pub *mut DOT11_PEER_STATISTICS);
impl PDOT11_PEER_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PEER_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PER_MSDU_COUNTERS(pub *mut DOT11_PER_MSDU_COUNTERS);
impl PDOT11_PER_MSDU_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PER_MSDU_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_ATTRIBUTES(pub *mut DOT11_PHY_ATTRIBUTES);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PHY_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_FRAME_STATISTICS(pub *mut DOT11_PHY_FRAME_STATISTICS);
impl PDOT11_PHY_FRAME_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS(pub *mut DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_ID_LIST(pub *mut DOT11_PHY_ID_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PHY_ID_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_STATE_PARAMETERS(pub *mut DOT11_PHY_STATE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PHY_STATE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PHY_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_TYPE(pub *mut DOT11_PHY_TYPE);
impl PDOT11_PHY_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PHY_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_TYPE_INFO(pub *mut DOT11_PHY_TYPE_INFO);
impl PDOT11_PHY_TYPE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PHY_TYPE_LIST(pub *mut DOT11_PHY_TYPE_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PHY_TYPE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PMKID_CANDIDATE_LIST_PARAMETERS(pub *mut DOT11_PMKID_CANDIDATE_LIST_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PMKID_ENTRY(pub *mut DOT11_PMKID_ENTRY);
impl PDOT11_PMKID_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PMKID_LIST(pub *mut DOT11_PMKID_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PMKID_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PORT_STATE_NOTIFICATION(pub *mut DOT11_PORT_STATE_NOTIFICATION);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PORT_STATE_NOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO(pub *mut DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_POWER_MGMT_MODE(pub *mut DOT11_POWER_MGMT_MODE);
impl PDOT11_POWER_MGMT_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_POWER_MGMT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_POWER_MGMT_MODE_STATUSINFO(pub *mut DOT11_POWER_MGMT_MODE_STATUS_INFO);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_POWER_MGMT_MODE_STATUSINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_POWER_MGMT_MODE_STATUSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_POWER_MODE(pub *mut DOT11_POWER_MODE);
impl PDOT11_POWER_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_POWER_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PRIVACY_EXEMPTION(pub *mut DOT11_PRIVACY_EXEMPTION);
impl PDOT11_PRIVACY_EXEMPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_PRIVACY_EXEMPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PRIVACY_EXEMPTION_LIST(pub *mut DOT11_PRIVACY_EXEMPTION_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_PRIVACY_EXEMPTION_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS(pub *mut DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_QOS_PARAMS(pub *mut DOT11_QOS_PARAMS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_QOS_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_QOS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_QOS_TX_DURATION(pub *mut DOT11_QOS_TX_DURATION);
impl PDOT11_QOS_TX_DURATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_QOS_TX_DURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_QOS_TX_MEDIUM_TIME(pub *mut DOT11_QOS_TX_MEDIUM_TIME);
impl PDOT11_QOS_TX_MEDIUM_TIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RATE_SET(pub *mut DOT11_RATE_SET);
impl PDOT11_RATE_SET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RATE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(pub *mut DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS(pub *mut DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS(pub *mut DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS(pub *mut DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS(pub *mut DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS(pub *mut DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(pub *mut DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECV_EXTENSION_INFO(pub *mut DOT11_RECV_EXTENSION_INFO);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_RECV_EXTENSION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECV_EXTENSION_INFO_V2(pub *mut DOT11_RECV_EXTENSION_INFO_V2);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_RECV_EXTENSION_INFO_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECV_SENSITIVITY(pub *mut DOT11_RECV_SENSITIVITY);
impl PDOT11_RECV_SENSITIVITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RECV_SENSITIVITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RECV_SENSITIVITY_LIST(pub *mut DOT11_RECV_SENSITIVITY_LIST);
impl PDOT11_RECV_SENSITIVITY_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_REG_DOMAINS_SUPPORT_VALUE(pub *mut DOT11_REG_DOMAINS_SUPPORT_VALUE);
impl PDOT11_REG_DOMAINS_SUPPORT_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_REG_DOMAIN_VALUE(pub *mut DOT11_REG_DOMAIN_VALUE);
impl PDOT11_REG_DOMAIN_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_REG_DOMAIN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RESET_REQUEST(pub *mut DOT11_RESET_REQUEST);
impl PDOT11_RESET_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RESET_TYPE(pub *mut DOT11_RESET_TYPE);
impl PDOT11_RESET_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RESET_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ROAMING_COMPLETION_PARAMETERS(pub *mut DOT11_ROAMING_COMPLETION_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_ROAMING_COMPLETION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_ROAMING_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_ROAMING_START_PARAMETERS(pub *mut DOT11_ROAMING_START_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_ROAMING_START_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_RSSI_RANGE(pub *mut DOT11_RSSI_RANGE);
impl PDOT11_RSSI_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_RSSI_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SCAN_REQUEST(pub *mut DOT11_SCAN_REQUEST);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_SCAN_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SCAN_REQUEST_V2(pub *mut DOT11_SCAN_REQUEST_V2);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_SCAN_REQUEST_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SCAN_TYPE(pub *mut DOT11_SCAN_TYPE);
impl PDOT11_SCAN_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SCAN_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(pub *mut DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS(pub *mut DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS(pub *mut DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_INVITATION_REQUEST_PARAMETERS(pub *mut DOT11_SEND_INVITATION_REQUEST_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_INVITATION_RESPONSE_PARAMETERS(pub *mut DOT11_SEND_INVITATION_RESPONSE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS(pub *mut DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(pub *mut DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SSID_LIST(pub *mut DOT11_SSID_LIST);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl PDOT11_SSID_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_wlan"))]
impl Default for PDOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_START_REQUEST(pub *mut DOT11_START_REQUEST);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_START_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_START_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_STATISTICS(pub *mut DOT11_STATISTICS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_types")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_STATUS_INDICATION(pub *mut DOT11_STATUS_INDICATION);
#[cfg(feature = "Win32_types")]
impl PDOT11_STATUS_INDICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_types")]
impl Default for PDOT11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_STOP_AP_PARAMETERS(pub *mut DOT11_STOP_AP_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_STOP_AP_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_STOP_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_ANTENNA(pub *mut DOT11_SUPPORTED_ANTENNA);
impl PDOT11_SUPPORTED_ANTENNA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_ANTENNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_ANTENNA_LIST(pub *mut DOT11_SUPPORTED_ANTENNA_LIST);
impl PDOT11_SUPPORTED_ANTENNA_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_DATA_RATES_VALUE(pub *mut DOT11_SUPPORTED_DATA_RATES_VALUE);
impl PDOT11_SUPPORTED_DATA_RATES_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_DATA_RATES_VALUE_V1(pub *mut DOT11_SUPPORTED_DATA_RATES_VALUE_V2);
impl PDOT11_SUPPORTED_DATA_RATES_VALUE_V1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_DATA_RATES_VALUE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_DATA_RATES_VALUE_V2(pub *mut DOT11_SUPPORTED_DATA_RATES_VALUE_V2);
impl PDOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_DSSS_CHANNEL(pub *mut DOT11_SUPPORTED_DSSS_CHANNEL);
impl PDOT11_SUPPORTED_DSSS_CHANNEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_DSSS_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_DSSS_CHANNEL_LIST(pub *mut DOT11_SUPPORTED_DSSS_CHANNEL_LIST);
impl PDOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_OFDM_FREQUENCY(pub *mut DOT11_SUPPORTED_OFDM_FREQUENCY);
impl PDOT11_SUPPORTED_OFDM_FREQUENCY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_OFDM_FREQUENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_OFDM_FREQUENCY_LIST(pub *mut DOT11_SUPPORTED_OFDM_FREQUENCY_LIST);
impl PDOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_PHY_TYPES(pub *mut DOT11_SUPPORTED_PHY_TYPES);
impl PDOT11_SUPPORTED_PHY_TYPES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_SUPPORTED_POWER_LEVELS(pub *mut DOT11_SUPPORTED_POWER_LEVELS);
impl PDOT11_SUPPORTED_POWER_LEVELS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_TEMP_TYPE(pub *mut DOT11_TEMP_TYPE);
impl PDOT11_TEMP_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_TEMP_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_TKIPMIC_FAILURE_PARAMETERS(pub *mut DOT11_TKIPMIC_FAILURE_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_TKIPMIC_FAILURE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_UPDATE_IE(pub *mut DOT11_UPDATE_IE);
impl PDOT11_UPDATE_IE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_UPDATE_IE_OP(pub *mut DOT11_UPDATE_IE_OP);
impl PDOT11_UPDATE_IE_OP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_UPDATE_IE_OP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_VWIFI_ATTRIBUTES(pub *mut DOT11_VWIFI_ATTRIBUTES);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_VWIFI_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_VWIFI_COMBINATION(pub *mut DOT11_VWIFI_COMBINATION);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_VWIFI_COMBINATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_VWIFI_COMBINATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_VWIFI_COMBINATION_V2(pub *mut DOT11_VWIFI_COMBINATION_V2);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_VWIFI_COMBINATION_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_VWIFI_COMBINATION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_VWIFI_COMBINATION_V3(pub *mut DOT11_VWIFI_COMBINATION_V3);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_VWIFI_COMBINATION_V3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_VWIFI_COMBINATION_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WEP_OFFLOAD(pub *mut DOT11_WEP_OFFLOAD);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_WEP_OFFLOAD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WEP_UPLOAD(pub *mut DOT11_WEP_UPLOAD);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_WEP_UPLOAD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_ADDITIONAL_IE(pub *mut DOT11_WFD_ADDITIONAL_IE);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_ADDITIONAL_IE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR(pub *mut DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR);
impl PDOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_ADVERTISED_SERVICE_LIST(pub *mut DOT11_WFD_ADVERTISED_SERVICE_LIST);
impl PDOT11_WFD_ADVERTISED_SERVICE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_ADVERTISEMENT_ID(pub *mut DOT11_WFD_ADVERTISEMENT_ID);
impl PDOT11_WFD_ADVERTISEMENT_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_ATTRIBUTES(pub *mut DOT11_WFD_ATTRIBUTES);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_CHANNEL(pub *mut DOT11_WFD_CHANNEL);
impl PDOT11_WFD_CHANNEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_CONFIGURATION_TIMEOUT(pub *mut DOT11_WFD_CONFIGURATION_TIMEOUT);
impl PDOT11_WFD_CONFIGURATION_TIMEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_CONFIGURATION_TIMEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DEVICE_CAPABILITY_CONFIG(pub *mut DOT11_WFD_DEVICE_CAPABILITY_CONFIG);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DEVICE_ENTRY(pub *mut DOT11_WFD_DEVICE_ENTRY);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_WFD_DEVICE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DEVICE_INFO(pub *mut DOT11_WFD_DEVICE_INFO);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DEVICE_LISTEN_CHANNEL(pub *mut DOT11_WFD_DEVICE_LISTEN_CHANNEL);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_DEVICE_LISTEN_CHANNEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DEVICE_TYPE(pub *mut DOT11_WFD_DEVICE_TYPE);
impl PDOT11_WFD_DEVICE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DISCOVER_COMPLETE_PARAMETERS(pub *mut DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl PDOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_types"))]
impl Default for PDOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DISCOVER_DEVICE_FILTER(pub *mut DOT11_WFD_DISCOVER_DEVICE_FILTER);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_WFD_DISCOVER_DEVICE_FILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DISCOVER_REQUEST(pub *mut DOT11_WFD_DISCOVER_REQUEST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_DISCOVER_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_DISCOVER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_DISCOVER_TYPE(pub *mut DOT11_WFD_DISCOVER_TYPE);
impl PDOT11_WFD_DISCOVER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_DISCOVER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GO_INTENT(pub *mut DOT11_WFD_GO_INTENT);
impl PDOT11_WFD_GO_INTENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_GO_INTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wlan")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GROUP_ID(pub *mut DOT11_WFD_GROUP_ID);
#[cfg(feature = "Win32_wlan")]
impl PDOT11_WFD_GROUP_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wlan")]
impl Default for PDOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GROUP_JOIN_PARAMETERS(pub *mut DOT11_WFD_GROUP_JOIN_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_GROUP_JOIN_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG(pub *mut DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2(pub *mut DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_GROUP_START_PARAMETERS(pub *mut DOT11_WFD_GROUP_START_PARAMETERS);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_GROUP_START_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_GROUP_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_INVITATION_FLAGS(pub *mut DOT11_WFD_INVITATION_FLAGS);
impl PDOT11_WFD_INVITATION_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_INVITATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_SCAN_TYPE(pub *mut DOT11_WFD_SCAN_TYPE);
impl PDOT11_WFD_SCAN_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_SCAN_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_objectheader")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_SECONDARY_DEVICE_TYPE_LIST(pub *mut DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST);
#[cfg(feature = "Win32_objectheader")]
impl PDOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_objectheader")]
impl Default for PDOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_SERVICE_HASH_LIST(pub *mut DOT11_WFD_SERVICE_HASH_LIST);
impl PDOT11_WFD_SERVICE_HASH_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_SESSION_ID(pub *mut DOT11_WFD_SESSION_ID);
impl PDOT11_WFD_SESSION_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WFD_SESSION_INFO(pub *mut DOT11_WFD_SESSION_INFO);
impl PDOT11_WFD_SESSION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WME_AC_PARAMETERS(pub *mut DOT11_WME_AC_PARAMETERS);
impl PDOT11_WME_AC_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WME_AC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WME_AC_PARAMETERS_LIST(pub *mut DOT11_WME_AC_PARAMETERS_LIST);
impl PDOT11_WME_AC_PARAMETERS_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WME_AC_PARAMETERS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WME_UPDATE_IE(pub *mut DOT11_WME_UPDATE_IE);
impl PDOT11_WME_UPDATE_IE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WPA_TSC(pub *mut DOT11_WPA_TSC);
#[cfg(feature = "Win32_winnt")]
impl PDOT11_WPA_TSC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDOT11_WPA_TSC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WPS_CONFIG_METHOD(pub *mut DOT11_WPS_CONFIG_METHOD);
impl PDOT11_WPS_CONFIG_METHOD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WPS_CONFIG_METHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WPS_DEVICE_NAME(pub *mut DOT11_WPS_DEVICE_NAME);
impl PDOT11_WPS_DEVICE_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOT11_WPS_DEVICE_PASSWORD_ID(pub *mut DOT11_WPS_DEVICE_PASSWORD_ID);
impl PDOT11_WPS_DEVICE_PASSWORD_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOT11_WPS_DEVICE_PASSWORD_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWFDSVC_CONNECTION_CAPABILITY(pub *mut WFDSVC_CONNECTION_CAPABILITY);
impl PWFDSVC_CONNECTION_CAPABILITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWFDSVC_CONNECTION_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSNA_AKM_CIPHER_PAIR {
    pub akm: RSNA_AKM_SUITE,
    pub cipher: RSNA_CIPHER_SUITE,
}
pub type RSNA_AKM_SUITE = i32;
pub type RSNA_CIPHER_SUITE = i32;
pub const RSNA_OUI_PREFIX: u32 = 11276032;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WFDSVC_CONNECTION_CAPABILITY {
    pub bNew: bool,
    pub bClient: bool,
    pub bGO: bool,
}
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: u32 = 2;
pub const WFDSVC_CONNECTION_CAPABILITY_GO: u32 = 4;
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: u32 = 1;
pub type WPA_AKM_SUITE = i32;
pub type WPA_CIPHER_SUITE = i32;
pub const WPA_OUI_PREFIX: u32 = 15880192;
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
