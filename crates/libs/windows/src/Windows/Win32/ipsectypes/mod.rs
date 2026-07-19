#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_ADDRESS_INFO0 {
    pub numV4Addresses: u32,
    pub v4Addresses: *mut u32,
    pub numV6Addresses: u32,
    pub v6Addresses: *mut super::FWP_BYTE_ARRAY16,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_ADDRESS_INFO0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub udpEspValidationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub invalidClearTextInbound: u32,
    pub saNotInitializedOnInbound: u32,
    pub receiveOverIncorrectSaInbound: u32,
    pub secureReceivesNotMatchingFilters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub udpEspValidationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub invalidClearTextInbound: u32,
    pub saNotInitializedOnInbound: u32,
    pub receiveOverIncorrectSaInbound: u32,
    pub secureReceivesNotMatchingFilters: u32,
    pub totalDropPacketsInbound: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AGGREGATE_SA_STATISTICS0 {
    pub activeSas: u32,
    pub pendingSaNegotiations: u32,
    pub totalSasAdded: u32,
    pub totalSasDeleted: u32,
    pub successfulRekeys: u32,
    pub activeTunnels: u32,
    pub offloadedSas: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AH_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
pub const IPSEC_AUTH_AES_128: IPSEC_AUTH_TYPE = 3;
pub const IPSEC_AUTH_AES_192: IPSEC_AUTH_TYPE = 4;
pub const IPSEC_AUTH_AES_256: IPSEC_AUTH_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IPSEC_AUTH_CONFIG(pub u8);
pub const IPSEC_AUTH_CONFIG_GCM_AES_128: u32 = 3;
pub const IPSEC_AUTH_CONFIG_GCM_AES_192: u32 = 4;
pub const IPSEC_AUTH_CONFIG_GCM_AES_256: u32 = 5;
pub const IPSEC_AUTH_CONFIG_HMAC_MD5_96: u32 = 0;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_1_96: u32 = 1;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_256_128: u32 = 2;
pub const IPSEC_AUTH_CONFIG_MAX: u32 = 6;
pub const IPSEC_AUTH_MAX: IPSEC_AUTH_TYPE = 6;
pub const IPSEC_AUTH_MD5: IPSEC_AUTH_TYPE = 0;
pub const IPSEC_AUTH_SHA_1: IPSEC_AUTH_TYPE = 1;
pub const IPSEC_AUTH_SHA_256: IPSEC_AUTH_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_AUTH_TRANSFORM0 {
    pub authTransformId: IPSEC_AUTH_TRANSFORM_ID0,
    pub cryptoModuleId: *mut IPSEC_CRYPTO_MODULE_ID,
}
impl Default for IPSEC_AUTH_TRANSFORM0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_AUTH_TRANSFORM_ID0 {
    pub authType: IPSEC_AUTH_TYPE,
    pub authConfig: IPSEC_AUTH_CONFIG,
}
pub type IPSEC_AUTH_TYPE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IPSEC_CIPHER_CONFIG(pub u8);
pub const IPSEC_CIPHER_CONFIG_CBC_3DES: u32 = 2;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_128: u32 = 3;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_192: u32 = 4;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_256: u32 = 5;
pub const IPSEC_CIPHER_CONFIG_CBC_DES: u32 = 1;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_128: u32 = 6;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_192: u32 = 7;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_256: u32 = 8;
pub const IPSEC_CIPHER_CONFIG_MAX: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_CIPHER_TRANSFORM0 {
    pub cipherTransformId: IPSEC_CIPHER_TRANSFORM_ID0,
    pub cryptoModuleId: *mut IPSEC_CRYPTO_MODULE_ID,
}
impl Default for IPSEC_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_CIPHER_TRANSFORM_ID0 {
    pub cipherType: IPSEC_CIPHER_TYPE,
    pub cipherConfig: IPSEC_CIPHER_CONFIG,
}
pub type IPSEC_CIPHER_TYPE = i32;
pub const IPSEC_CIPHER_TYPE_3DES: IPSEC_CIPHER_TYPE = 2;
pub const IPSEC_CIPHER_TYPE_AES_128: IPSEC_CIPHER_TYPE = 3;
pub const IPSEC_CIPHER_TYPE_AES_192: IPSEC_CIPHER_TYPE = 4;
pub const IPSEC_CIPHER_TYPE_AES_256: IPSEC_CIPHER_TYPE = 5;
pub const IPSEC_CIPHER_TYPE_DES: IPSEC_CIPHER_TYPE = 1;
pub const IPSEC_CIPHER_TYPE_MAX: IPSEC_CIPHER_TYPE = 6;
pub type IPSEC_CRYPTO_MODULE_ID = windows_core::GUID;
pub const IPSEC_DOSP_DSCP_DISABLE_VALUE: u32 = 255;
pub const IPSEC_DOSP_FLAG_DISABLE_AUTHIP: u32 = 4;
pub const IPSEC_DOSP_FLAG_DISABLE_DEFAULT_BLOCK: u32 = 8;
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV1: u32 = 1;
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV2: u32 = 2;
pub const IPSEC_DOSP_FLAG_FILTER_BLOCK: u32 = 16;
pub const IPSEC_DOSP_FLAG_FILTER_EXEMPT: u32 = 32;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_DOSP_OPTIONS0 {
    pub stateIdleTimeoutSeconds: u32,
    pub perIPRateLimitQueueIdleTimeoutSeconds: u32,
    pub ipV6IPsecUnauthDscp: u8,
    pub ipV6IPsecUnauthRateLimitBytesPerSec: u32,
    pub ipV6IPsecUnauthPerIPRateLimitBytesPerSec: u32,
    pub ipV6IPsecAuthDscp: u8,
    pub ipV6IPsecAuthRateLimitBytesPerSec: u32,
    pub icmpV6Dscp: u8,
    pub icmpV6RateLimitBytesPerSec: u32,
    pub ipV6FilterExemptDscp: u8,
    pub ipV6FilterExemptRateLimitBytesPerSec: u32,
    pub defBlockExemptDscp: u8,
    pub defBlockExemptRateLimitBytesPerSec: u32,
    pub maxStateEntries: u32,
    pub maxPerIPRateLimitQueues: u32,
    pub flags: u32,
    pub numPublicIFLuids: u32,
    pub publicIFLuids: *mut u64,
    pub numInternalIFLuids: u32,
    pub internalIFLuids: *mut u64,
    pub publicV6AddrMask: super::FWP_V6_ADDR_AND_MASK,
    pub internalV6AddrMask: super::FWP_V6_ADDR_AND_MASK,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_DOSP_OPTIONS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPSEC_DOSP_RATE_LIMIT_DISABLE_VALUE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_DOSP_STATE0 {
    pub publicHostV6Addr: [u8; 16],
    pub internalHostV6Addr: [u8; 16],
    pub totalInboundIPv6IPsecAuthPackets: u64,
    pub totalOutboundIPv6IPsecAuthPackets: u64,
    pub durationSecs: u32,
}
impl Default for IPSEC_DOSP_STATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    pub publicV6AddrMask: super::FWP_V6_ADDR_AND_MASK,
    pub internalV6AddrMask: super::FWP_V6_ADDR_AND_MASK,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_DOSP_STATISTICS0 {
    pub totalStateEntriesCreated: u64,
    pub currentStateEntries: u64,
    pub totalInboundAllowedIPv6IPsecUnauthPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundOtherDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundAllowedIPv6IPsecAuthPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6IPsecAuthPkts: u64,
    pub totalInboundOtherDiscardedIPv6IPsecAuthPkts: u64,
    pub totalInboundAllowedICMPv6Pkts: u64,
    pub totalInboundRatelimitDiscardedICMPv6Pkts: u64,
    pub totalInboundAllowedIPv6FilterExemptPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6FilterExemptPkts: u64,
    pub totalInboundDiscardedIPv6FilterBlockPkts: u64,
    pub totalInboundAllowedDefBlockExemptPkts: u64,
    pub totalInboundRatelimitDiscardedDefBlockExemptPkts: u64,
    pub totalInboundDiscardedDefBlockPkts: u64,
    pub currentInboundIPv6IPsecUnauthPerIPRateLimitQueues: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
pub const IPSEC_FAILURE_ME: IPSEC_FAILURE_POINT = 1;
pub const IPSEC_FAILURE_NONE: IPSEC_FAILURE_POINT = 0;
pub const IPSEC_FAILURE_PEER: IPSEC_FAILURE_POINT = 2;
pub type IPSEC_FAILURE_POINT = i32;
pub const IPSEC_FAILURE_POINT_MAX: IPSEC_FAILURE_POINT = 3;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_GETSPI0 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC0,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI0_0,
    pub rngCryptoModuleID: *mut IPSEC_CRYPTO_MODULE_ID,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_GETSPI0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_GETSPI0_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_GETSPI0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_GETSPI1 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC1,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI1_0,
    pub rngCryptoModuleID: *mut IPSEC_CRYPTO_MODULE_ID,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_GETSPI1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_GETSPI1_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_GETSPI1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_ID0 {
    pub mmTargetName: *mut u16,
    pub emTargetName: *mut u16,
    pub numTokens: u32,
    pub tokens: *mut IPSEC_TOKEN0,
    pub explicitCredentials: u64,
    pub logonId: u64,
}
impl Default for IPSEC_ID0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_KEYING_POLICY0 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut windows_core::GUID,
}
impl Default for IPSEC_KEYING_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_KEYING_POLICY1 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut windows_core::GUID,
    pub flags: u32,
}
impl Default for IPSEC_KEYING_POLICY1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPSEC_KEYING_POLICY_FLAG_TERMINATING_MATCH: u32 = 1;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_KEYMODULE_STATE0 {
    pub keyModuleKey: windows_core::GUID,
    pub stateBlob: super::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_KEY_MANAGER0 {
    pub keyManagerKey: windows_core::GUID,
    pub displayData: super::FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub keyDictationTimeoutHint: u8,
}
pub const IPSEC_KEY_MANAGER_FLAG_DICTATE_KEY: u32 = 1;
pub const IPSEC_PFS_1: IPSEC_PFS_GROUP = 1;
pub const IPSEC_PFS_14: IPSEC_PFS_GROUP = 3;
pub const IPSEC_PFS_2: IPSEC_PFS_GROUP = 2;
pub const IPSEC_PFS_2048: IPSEC_PFS_GROUP = 3;
pub const IPSEC_PFS_24: IPSEC_PFS_GROUP = 7;
pub const IPSEC_PFS_ECP_256: IPSEC_PFS_GROUP = 4;
pub const IPSEC_PFS_ECP_384: IPSEC_PFS_GROUP = 5;
pub type IPSEC_PFS_GROUP = i32;
pub const IPSEC_PFS_MAX: IPSEC_PFS_GROUP = 8;
pub const IPSEC_PFS_MM: IPSEC_PFS_GROUP = 6;
pub const IPSEC_PFS_NONE: IPSEC_PFS_GROUP = 0;
pub const IPSEC_POLICY_FLAG_BANDWIDTH1: u32 = 268435456;
pub const IPSEC_POLICY_FLAG_BANDWIDTH2: u32 = 536870912;
pub const IPSEC_POLICY_FLAG_BANDWIDTH3: u32 = 1073741824;
pub const IPSEC_POLICY_FLAG_BANDWIDTH4: u32 = 2147483648;
pub const IPSEC_POLICY_FLAG_CLEAR_DF_ON_TUNNEL: u32 = 8;
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_BYTE_LIFETIME: u32 = 128;
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_SECOND_LIFETIME: u32 = 64;
pub const IPSEC_POLICY_FLAG_ENABLE_SERVER_ADDR_ASSIGNMENT: u32 = 512;
pub const IPSEC_POLICY_FLAG_ENABLE_V6_IN_V4_TUNNELING: u32 = 256;
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_DICTATE_KEY: u32 = 8192;
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_NOTIFY_KEY: u32 = 16384;
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_GENERAL_NAT_TRAVERSAL: u32 = 32;
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_PEER_BEHIND_NAT: u32 = 16;
pub const IPSEC_POLICY_FLAG_ND_BOUNDARY: u32 = 4;
pub const IPSEC_POLICY_FLAG_ND_SECURE: u32 = 2;
pub const IPSEC_POLICY_FLAG_POINT_TO_SITE_TUNNEL: u32 = 131072;
pub const IPSEC_POLICY_FLAG_RESERVED1: u32 = 32768;
pub const IPSEC_POLICY_FLAG_SITE_TO_SITE_TUNNEL: u32 = 65536;
pub const IPSEC_POLICY_FLAG_TUNNEL_ALLOW_OUTBOUND_CLEAR_CONNECTION: u32 = 1024;
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ALREADY_SECURE_CONNECTION: u32 = 2048;
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ICMPV6: u32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_PROPOSAL0 {
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub numSaTransforms: u32,
    pub saTransforms: *mut IPSEC_SA_TRANSFORM0,
    pub pfsGroup: IPSEC_PFS_GROUP,
}
impl Default for IPSEC_PROPOSAL0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_SA0 {
    pub spi: IPSEC_SA_SPI,
    pub saTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA0_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_SA0_0 {
    pub ahInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espAuthInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espCipherInformation: *mut IPSEC_SA_CIPHER_INFORMATION0,
    pub espAuthAndCipherInformation: *mut IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0,
    pub espAuthFwInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    pub saCipherInformation: IPSEC_SA_CIPHER_INFORMATION0,
    pub saAuthInformation: IPSEC_SA_AUTH_INFORMATION0,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_AUTH_INFORMATION0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub authKey: super::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_BUNDLE0 {
    pub flags: u32,
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub idleTimeoutSeconds: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub ipsecId: *mut IPSEC_ID0,
    pub napContext: u32,
    pub qmSaId: u32,
    pub numSAs: u32,
    pub saList: *mut IPSEC_SA0,
    pub keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_SA_BUNDLE0_0,
    pub mmSaId: u64,
    pub pfsGroup: IPSEC_PFS_GROUP,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA_BUNDLE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_SA_BUNDLE0_0 {
    pub peerV4PrivateAddress: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA_BUNDLE0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_BUNDLE1 {
    pub flags: u32,
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub idleTimeoutSeconds: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub ipsecId: *mut IPSEC_ID0,
    pub napContext: u32,
    pub qmSaId: u32,
    pub numSAs: u32,
    pub saList: *mut IPSEC_SA0,
    pub keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_SA_BUNDLE1_0,
    pub mmSaId: u64,
    pub pfsGroup: IPSEC_PFS_GROUP,
    pub saLookupContext: windows_core::GUID,
    pub qmFilterId: u64,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA_BUNDLE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_SA_BUNDLE1_0 {
    pub peerV4PrivateAddress: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_SA_BUNDLE1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPSEC_SA_BUNDLE_FLAG_ALLOW_NULL_TARGET_NAME_MATCH: u32 = 512;
pub const IPSEC_SA_BUNDLE_FLAG_ASSUME_UDP_CONTEXT_OUTBOUND: u32 = 2048;
pub const IPSEC_SA_BUNDLE_FLAG_CLEAR_DF_ON_TUNNEL: u32 = 1024;
pub const IPSEC_SA_BUNDLE_FLAG_ENABLE_OPTIONAL_ASYMMETRIC_IDLE: u32 = 262144;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_INBOUND_CONNECTIONS: u32 = 32768;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_OUTBOUND_CONNECTIONS: u32 = 65536;
pub const IPSEC_SA_BUNDLE_FLAG_FORWARD_PATH_INITIATOR: u32 = 131072;
pub const IPSEC_SA_BUNDLE_FLAG_GUARANTEE_ENCRYPTION: u32 = 8;
pub const IPSEC_SA_BUNDLE_FLAG_IP_IN_IP_PKT: u32 = 4194304;
pub const IPSEC_SA_BUNDLE_FLAG_LOCALLY_DICTATED_KEYS: u32 = 1048576;
pub const IPSEC_SA_BUNDLE_FLAG_LOW_POWER_MODE_SUPPORT: u32 = 8388608;
pub const IPSEC_SA_BUNDLE_FLAG_ND_BOUNDARY: u32 = 2;
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_BOUNDARY: u32 = 4096;
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_NAT_BOUNDARY: u32 = 4;
pub const IPSEC_SA_BUNDLE_FLAG_ND_SECURE: u32 = 1;
pub const IPSEC_SA_BUNDLE_FLAG_NLB: u32 = 16;
pub const IPSEC_SA_BUNDLE_FLAG_NO_EXPLICIT_CRED_MATCH: u32 = 128;
pub const IPSEC_SA_BUNDLE_FLAG_NO_IMPERSONATION_LUID_VERIFY: u32 = 64;
pub const IPSEC_SA_BUNDLE_FLAG_NO_MACHINE_LUID_VERIFY: u32 = 32;
pub const IPSEC_SA_BUNDLE_FLAG_PEER_SUPPORTS_GUARANTEE_ENCRYPTION: u32 = 16384;
pub const IPSEC_SA_BUNDLE_FLAG_SA_OFFLOADED: u32 = 2097152;
pub const IPSEC_SA_BUNDLE_FLAG_SUPPRESS_DUPLICATE_DELETION: u32 = 8192;
pub const IPSEC_SA_BUNDLE_FLAG_TUNNEL_BANDWIDTH1: u32 = 268435456;
pub const IPSEC_SA_BUNDLE_FLAG_TUNNEL_BANDWIDTH2: u32 = 536870912;
pub const IPSEC_SA_BUNDLE_FLAG_TUNNEL_BANDWIDTH3: u32 = 1073741824;
pub const IPSEC_SA_BUNDLE_FLAG_TUNNEL_BANDWIDTH4: u32 = 2147483648;
pub const IPSEC_SA_BUNDLE_FLAG_USING_DICTATED_KEYS: u32 = 524288;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_CIPHER_INFORMATION0 {
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
    pub cipherKey: super::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_SA_CONTEXT0 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS0,
    pub outboundSa: *mut IPSEC_SA_DETAILS0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_CONTEXT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_SA_CONTEXT1 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS1,
    pub outboundSa: *mut IPSEC_SA_DETAILS1,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_CONTEXT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_CONTEXT_CHANGE0 {
    pub changeType: IPSEC_SA_CONTEXT_EVENT_TYPE0,
    pub saContextId: u64,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    pub localSubNet: super::FWP_CONDITION_VALUE0,
    pub remoteSubNet: super::FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPSEC_SA_CONTEXT_EVENT_ADD: IPSEC_SA_CONTEXT_EVENT_TYPE0 = 1;
pub const IPSEC_SA_CONTEXT_EVENT_DELETE: IPSEC_SA_CONTEXT_EVENT_TYPE0 = 2;
pub const IPSEC_SA_CONTEXT_EVENT_MAX: IPSEC_SA_CONTEXT_EVENT_TYPE0 = 3;
pub type IPSEC_SA_CONTEXT_EVENT_TYPE0 = i32;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: windows_core::GUID,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_DETAILS0 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub saDirection: super::FWP_DIRECTION,
    pub traffic: IPSEC_TRAFFIC0,
    pub saBundle: IPSEC_SA_BUNDLE0,
    pub Anonymous: IPSEC_SA_DETAILS0_0,
    pub transportFilter: *mut super::FWPM_FILTER0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union IPSEC_SA_DETAILS0_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_DETAILS1 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub saDirection: super::FWP_DIRECTION,
    pub traffic: IPSEC_TRAFFIC1,
    pub saBundle: IPSEC_SA_BUNDLE1,
    pub Anonymous: IPSEC_SA_DETAILS1_0,
    pub transportFilter: *mut super::FWPM_FILTER0,
    pub virtualIfTunnelInfo: super::IPSEC_VIRTUAL_IF_TUNNEL_INFO0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union IPSEC_SA_DETAILS1_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
impl Default for IPSEC_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_ENUM_TEMPLATE0 {
    pub saDirection: super::FWP_DIRECTION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_IDLE_TIMEOUT0 {
    pub idleTimeoutSeconds: u32,
    pub idleTimeoutSecondsFailOver: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_SA_LIFETIME0 {
    pub lifetimeSeconds: u32,
    pub lifetimeKilobytes: u32,
    pub lifetimePackets: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IPSEC_SA_SPI(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IPSEC_SA_TRANSFORM0 {
    pub ipsecTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA_TRANSFORM0_0,
}
impl Default for IPSEC_SA_TRANSFORM0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IPSEC_SA_TRANSFORM0_0 {
    pub ahTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espAuthTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espCipherTransform: *mut IPSEC_CIPHER_TRANSFORM0,
    pub espAuthAndCipherTransform: *mut IPSEC_AUTH_AND_CIPHER_TRANSFORM0,
    pub espAuthFwTransform: *mut IPSEC_AUTH_TRANSFORM0,
}
impl Default for IPSEC_SA_TRANSFORM0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_STATISTICS0 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_STATISTICS1 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_TOKEN0 {
    pub r#type: IPSEC_TOKEN_TYPE,
    pub principal: IPSEC_TOKEN_PRINCIPAL,
    pub mode: IPSEC_TOKEN_MODE,
    pub token: IPSEC_TOKEN_HANDLE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IPSEC_TOKEN_HANDLE(pub u64);
pub type IPSEC_TOKEN_MODE = i32;
pub const IPSEC_TOKEN_MODE_EXTENDED: IPSEC_TOKEN_MODE = 1;
pub const IPSEC_TOKEN_MODE_MAIN: IPSEC_TOKEN_MODE = 0;
pub const IPSEC_TOKEN_MODE_MAX: IPSEC_TOKEN_MODE = 2;
pub type IPSEC_TOKEN_PRINCIPAL = i32;
pub const IPSEC_TOKEN_PRINCIPAL_LOCAL: IPSEC_TOKEN_PRINCIPAL = 0;
pub const IPSEC_TOKEN_PRINCIPAL_MAX: IPSEC_TOKEN_PRINCIPAL = 2;
pub const IPSEC_TOKEN_PRINCIPAL_PEER: IPSEC_TOKEN_PRINCIPAL = 1;
pub type IPSEC_TOKEN_TYPE = i32;
pub const IPSEC_TOKEN_TYPE_IMPERSONATION: IPSEC_TOKEN_TYPE = 1;
pub const IPSEC_TOKEN_TYPE_MACHINE: IPSEC_TOKEN_TYPE = 0;
pub const IPSEC_TOKEN_TYPE_MAX: IPSEC_TOKEN_TYPE = 2;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TRAFFIC0 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TRAFFIC0_0,
    pub Anonymous2: IPSEC_TRAFFIC0_1,
    pub trafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous3: IPSEC_TRAFFIC0_2,
    pub remotePort: u16,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC0_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TRAFFIC1 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TRAFFIC1_0,
    pub Anonymous2: IPSEC_TRAFFIC1_1,
    pub trafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous3: IPSEC_TRAFFIC1_2,
    pub remotePort: u16,
    pub localPort: u16,
    pub ipProtocol: u8,
    pub localIfLuid: u64,
    pub realIfProfileId: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC1_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TRAFFIC_SELECTOR0 {
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TRAFFIC_SELECTOR0_0,
    pub Anonymous2: IPSEC_TRAFFIC_SELECTOR0_1,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC_SELECTOR0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC_SELECTOR0_0 {
    pub startV4Address: u32,
    pub startV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC_SELECTOR0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TRAFFIC_SELECTOR0_1 {
    pub endV4Address: u32,
    pub endV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC_SELECTOR0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    pub flags: u32,
    pub numLocalTrafficSelectors: u32,
    pub localTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0,
    pub numRemoteTrafficSelectors: u32,
    pub remoteTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_TRAFFIC_STATISTICS0 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_TRAFFIC_STATISTICS1 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
    pub totalSuccessfulPackets: u64,
}
pub type IPSEC_TRAFFIC_TYPE = i32;
pub const IPSEC_TRAFFIC_TYPE_MAX: IPSEC_TRAFFIC_TYPE = 2;
pub const IPSEC_TRAFFIC_TYPE_TRANSPORT: IPSEC_TRAFFIC_TYPE = 0;
pub const IPSEC_TRAFFIC_TYPE_TUNNEL: IPSEC_TRAFFIC_TYPE = 1;
pub const IPSEC_TRANSFORM_AH: IPSEC_TRANSFORM_TYPE = 1;
pub const IPSEC_TRANSFORM_ESP_AUTH: IPSEC_TRANSFORM_TYPE = 2;
pub const IPSEC_TRANSFORM_ESP_AUTH_AND_CIPHER: IPSEC_TRANSFORM_TYPE = 4;
pub const IPSEC_TRANSFORM_ESP_AUTH_FW: IPSEC_TRANSFORM_TYPE = 5;
pub const IPSEC_TRANSFORM_ESP_CIPHER: IPSEC_TRANSFORM_TYPE = 3;
pub type IPSEC_TRANSFORM_TYPE = i32;
pub const IPSEC_TRANSFORM_TYPE_MAX: IPSEC_TRANSFORM_TYPE = 6;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_TRANSPORT_POLICY0 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TRANSPORT_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_TRANSPORT_POLICY1 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY1,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TRANSPORT_POLICY1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPSEC_TRANSPORT_POLICY2 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY2,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TRANSPORT_POLICY2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_ENDPOINT0 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINT0_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINT0_0 {
    pub v4Address: u32,
    pub v6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_ENDPOINTS0 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINTS0_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS0_1,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_ENDPOINTS1 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINTS1_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS1_1,
    pub localIfLuid: u64,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_ENDPOINTS2 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINTS2_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS2_1,
    pub localIfLuid: u64,
    pub remoteFqdn: *mut u16,
    pub numAddresses: u32,
    pub remoteAddresses: *mut IPSEC_TUNNEL_ENDPOINT0,
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS2_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IPSEC_TUNNEL_ENDPOINTS2_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_POLICY0 {
    pub flags: u32,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS0,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TUNNEL_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_POLICY1 {
    pub flags: u32,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS1,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY1,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TUNNEL_POLICY1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_POLICY2 {
    pub flags: u32,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY2,
    pub fwdPathSaLifetime: u32,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TUNNEL_POLICY2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
#[derive(Clone, Copy)]
pub struct IPSEC_TUNNEL_POLICY3 {
    pub flags: u32,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut super::IKEEXT_EM_POLICY2,
    pub fwdPathSaLifetime: u32,
    pub compartmentId: u32,
    pub numTrafficSelectorPolicy: u32,
    pub trafficSelectorPolicies: *mut IPSEC_TRAFFIC_SELECTOR_POLICY0,
}
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
impl Default for IPSEC_TUNNEL_POLICY3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPSEC_V4_UDP_ENCAPSULATION0 {
    pub localUdpEncapPort: u16,
    pub remoteUdpEncapPort: u16,
}
