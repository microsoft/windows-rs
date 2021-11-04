#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_MODE_MESSAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_MODE_STREAMING: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct AAL5_PARAMETERS {
    pub ForwardMaxCPCSSDUSize: u32,
    pub BackwardMaxCPCSSDUSize: u32,
    pub Mode: u8,
    pub SSCSType: u8,
}
impl AAL5_PARAMETERS {}
impl ::std::default::Default for AAL5_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AAL5_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AAL5_PARAMETERS").field("ForwardMaxCPCSSDUSize", &self.ForwardMaxCPCSSDUSize).field("BackwardMaxCPCSSDUSize", &self.BackwardMaxCPCSSDUSize).field("Mode", &self.Mode).field("SSCSType", &self.SSCSType).finish()
    }
}
impl ::std::cmp::PartialEq for AAL5_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ForwardMaxCPCSSDUSize == other.ForwardMaxCPCSSDUSize && self.BackwardMaxCPCSSDUSize == other.BackwardMaxCPCSSDUSize && self.Mode == other.Mode && self.SSCSType == other.SSCSType
    }
}
impl ::std::cmp::Eq for AAL5_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for AAL5_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_SSCS_FRAME_RELAY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_SSCS_NULL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_SSCS_SSCOP_ASSURED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AAL5_SSCS_SSCOP_NON_ASSURED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct AALUSER_PARAMETERS {
    pub UserDefined: u32,
}
impl AALUSER_PARAMETERS {}
impl ::std::default::Default for AALUSER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AALUSER_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AALUSER_PARAMETERS").field("UserDefined", &self.UserDefined).finish()
    }
}
impl ::std::cmp::PartialEq for AALUSER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UserDefined == other.UserDefined
    }
}
impl ::std::cmp::Eq for AALUSER_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for AALUSER_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct AAL_PARAMETERS_IE {
    pub AALType: AAL_TYPE,
    pub AALSpecificParameters: AAL_PARAMETERS_IE_0,
}
impl AAL_PARAMETERS_IE {}
impl ::std::default::Default for AAL_PARAMETERS_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AAL_PARAMETERS_IE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AAL_PARAMETERS_IE {}
unsafe impl ::windows::runtime::Abi for AAL_PARAMETERS_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union AAL_PARAMETERS_IE_0 {
    pub AAL5Parameters: AAL5_PARAMETERS,
    pub AALUserParameters: AALUSER_PARAMETERS,
}
impl AAL_PARAMETERS_IE_0 {}
impl ::std::default::Default for AAL_PARAMETERS_IE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AAL_PARAMETERS_IE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AAL_PARAMETERS_IE_0 {}
unsafe impl ::windows::runtime::Abi for AAL_PARAMETERS_IE_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AAL_TYPE(pub i32);
pub const AALTYPE_5: AAL_TYPE = AAL_TYPE(5i32);
pub const AALTYPE_USER: AAL_TYPE = AAL_TYPE(16i32);
impl ::std::convert::From<i32> for AAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AAL_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct ADDRINFOA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut ADDRINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl ADDRINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ADDRINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ADDRINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ADDRINFOA")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ADDRINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ADDRINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ADDRINFOA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ADDRINFOEX_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ADDRINFOEX_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ADDRINFOEX_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ADDRINFOEX_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ADDRINFOEX_VERSION_6: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct AFPROTOCOLS {
    pub iAddressFamily: i32,
    pub iProtocol: i32,
}
impl AFPROTOCOLS {}
impl ::std::default::Default for AFPROTOCOLS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AFPROTOCOLS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AFPROTOCOLS").field("iAddressFamily", &self.iAddressFamily).field("iProtocol", &self.iProtocol).finish()
    }
}
impl ::std::cmp::PartialEq for AFPROTOCOLS {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressFamily == other.iAddressFamily && self.iProtocol == other.iProtocol
    }
}
impl ::std::cmp::Eq for AFPROTOCOLS {}
unsafe impl ::windows::runtime::Abi for AFPROTOCOLS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_12844: u16 = 25u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_APPLETALK: u16 = 16u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_ATM: u16 = 22u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_BAN: u16 = 21u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_CCITT: u16 = 10u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_CHAOS: u16 = 5u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_CLUSTER: u16 = 24u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_DATAKIT: u16 = 9u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_DECnet: u16 = 12u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_DLI: u16 = 13u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_ECMA: u16 = 8u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_FIREFOX: u16 = 19u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_HYLINK: u16 = 15u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_HYPERV: u16 = 34u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_ICLFXBM: u16 = 31u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_IMPLINK: u16 = 3u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_IPX: u16 = 6u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_IRDA: u16 = 26u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_ISO: u16 = 7u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_LAT: u16 = 14u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_LINK: u16 = 33u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_MAX: u16 = 29u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_NETBIOS: u16 = 17u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_NETDES: u16 = 28u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_NS: u16 = 6u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_OSI: u16 = 7u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_PUP: u16 = 4u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_SNA: u16 = 11u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_TCNMESSAGE: u16 = 30u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_TCNPROCESS: u16 = 29u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_UNIX: u16 = 1u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_UNKNOWN1: u16 = 20u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AF_VOICEVIEW: u16 = 18u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_ADDRCONFIG: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_ALL: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_BYPASS_DNS_CACHE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_CANONNAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DISABLE_IDN_ENCODING: u32 = 524288u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_ONLY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_RESPONSE_HOSTFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_RESPONSE_SECURE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_SERVER_TYPE_DOH: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_SERVER_TYPE_UDP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_DNS_SERVER_UDP_FALLBACK: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_EXTENDED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_FILESERVER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_FORCE_CLEAR_TEXT: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_FQDN: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_NON_AUTHORITATIVE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_NUMERICHOST: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_NUMERICSERV: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_PASSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_REQUIRE_SECURE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_RESOLUTION_HANDLE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_RETURN_PREFERRED_NAMES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_RETURN_RESPONSE_FLAGS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_RETURN_TTL: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_SECURE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_SECURE_WITH_FALLBACK: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const AI_V4MAPPED: u32 = 2048u32;
pub const ASSOCIATE_NAMERES_CONTEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1503890279, 54526, 18145, [186, 60, 135, 234, 116, 202, 48, 73]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub Handle: u64,
}
impl ASSOCIATE_NAMERES_CONTEXT_INPUT {}
impl ::std::default::Default for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ASSOCIATE_NAMERES_CONTEXT_INPUT").field("TransportSettingId", &self.TransportSettingId).field("Handle", &self.Handle).finish()
    }
}
impl ::std::cmp::PartialEq for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.Handle == other.Handle
    }
}
impl ::std::cmp::Eq for ASSOCIATE_NAMERES_CONTEXT_INPUT {}
unsafe impl ::windows::runtime::Abi for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATMPROTO_AAL1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATMPROTO_AAL2: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATMPROTO_AAL34: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATMPROTO_AAL5: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATMPROTO_AALUSER: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_ADDRESS {
    pub AddressType: u32,
    pub NumofDigits: u32,
    pub Addr: [u8; 20],
}
impl ATM_ADDRESS {}
impl ::std::default::Default for ATM_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_ADDRESS").field("AddressType", &self.AddressType).field("NumofDigits", &self.NumofDigits).field("Addr", &self.Addr).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.NumofDigits == other.NumofDigits && self.Addr == other.Addr
    }
}
impl ::std::cmp::Eq for ATM_ADDRESS {}
unsafe impl ::windows::runtime::Abi for ATM_ADDRESS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATM_ADDR_SIZE: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATM_AESA: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_BHLI {
    pub HighLayerInfoType: u32,
    pub HighLayerInfoLength: u32,
    pub HighLayerInfo: [u8; 8],
}
impl ATM_BHLI {}
impl ::std::default::Default for ATM_BHLI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_BHLI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_BHLI").field("HighLayerInfoType", &self.HighLayerInfoType).field("HighLayerInfoLength", &self.HighLayerInfoLength).field("HighLayerInfo", &self.HighLayerInfo).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_BHLI {
    fn eq(&self, other: &Self) -> bool {
        self.HighLayerInfoType == other.HighLayerInfoType && self.HighLayerInfoLength == other.HighLayerInfoLength && self.HighLayerInfo == other.HighLayerInfo
    }
}
impl ::std::cmp::Eq for ATM_BHLI {}
unsafe impl ::windows::runtime::Abi for ATM_BHLI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_BLLI {
    pub Layer2Protocol: u32,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ATM_BLLI {}
impl ::std::default::Default for ATM_BLLI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_BLLI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_BLLI")
            .field("Layer2Protocol", &self.Layer2Protocol)
            .field("Layer2UserSpecifiedProtocol", &self.Layer2UserSpecifiedProtocol)
            .field("Layer3Protocol", &self.Layer3Protocol)
            .field("Layer3UserSpecifiedProtocol", &self.Layer3UserSpecifiedProtocol)
            .field("Layer3IPI", &self.Layer3IPI)
            .field("SnapID", &self.SnapID)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATM_BLLI {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol && self.Layer3Protocol == other.Layer3Protocol && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol && self.Layer3IPI == other.Layer3IPI && self.SnapID == other.SnapID
    }
}
impl ::std::cmp::Eq for ATM_BLLI {}
unsafe impl ::windows::runtime::Abi for ATM_BLLI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_BLLI_IE {
    pub Layer2Protocol: u32,
    pub Layer2Mode: u8,
    pub Layer2WindowSize: u8,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3Mode: u8,
    pub Layer3DefaultPacketSize: u8,
    pub Layer3PacketWindowSize: u8,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ATM_BLLI_IE {}
impl ::std::default::Default for ATM_BLLI_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_BLLI_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_BLLI_IE")
            .field("Layer2Protocol", &self.Layer2Protocol)
            .field("Layer2Mode", &self.Layer2Mode)
            .field("Layer2WindowSize", &self.Layer2WindowSize)
            .field("Layer2UserSpecifiedProtocol", &self.Layer2UserSpecifiedProtocol)
            .field("Layer3Protocol", &self.Layer3Protocol)
            .field("Layer3Mode", &self.Layer3Mode)
            .field("Layer3DefaultPacketSize", &self.Layer3DefaultPacketSize)
            .field("Layer3PacketWindowSize", &self.Layer3PacketWindowSize)
            .field("Layer3UserSpecifiedProtocol", &self.Layer3UserSpecifiedProtocol)
            .field("Layer3IPI", &self.Layer3IPI)
            .field("SnapID", &self.SnapID)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATM_BLLI_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol
            && self.Layer2Mode == other.Layer2Mode
            && self.Layer2WindowSize == other.Layer2WindowSize
            && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol
            && self.Layer3Protocol == other.Layer3Protocol
            && self.Layer3Mode == other.Layer3Mode
            && self.Layer3DefaultPacketSize == other.Layer3DefaultPacketSize
            && self.Layer3PacketWindowSize == other.Layer3PacketWindowSize
            && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol
            && self.Layer3IPI == other.Layer3IPI
            && self.SnapID == other.SnapID
    }
}
impl ::std::cmp::Eq for ATM_BLLI_IE {}
unsafe impl ::windows::runtime::Abi for ATM_BLLI_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_BROADBAND_BEARER_CAPABILITY_IE {
    pub BearerClass: u8,
    pub TrafficType: u8,
    pub TimingRequirements: u8,
    pub ClippingSusceptability: u8,
    pub UserPlaneConnectionConfig: u8,
}
impl ATM_BROADBAND_BEARER_CAPABILITY_IE {}
impl ::std::default::Default for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_BROADBAND_BEARER_CAPABILITY_IE")
            .field("BearerClass", &self.BearerClass)
            .field("TrafficType", &self.TrafficType)
            .field("TimingRequirements", &self.TimingRequirements)
            .field("ClippingSusceptability", &self.ClippingSusceptability)
            .field("UserPlaneConnectionConfig", &self.UserPlaneConnectionConfig)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn eq(&self, other: &Self) -> bool {
        self.BearerClass == other.BearerClass && self.TrafficType == other.TrafficType && self.TimingRequirements == other.TimingRequirements && self.ClippingSusceptability == other.ClippingSusceptability && self.UserPlaneConnectionConfig == other.UserPlaneConnectionConfig
    }
}
impl ::std::cmp::Eq for ATM_BROADBAND_BEARER_CAPABILITY_IE {}
unsafe impl ::windows::runtime::Abi for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_CALLING_PARTY_NUMBER_IE {
    pub ATM_Number: ATM_ADDRESS,
    pub Presentation_Indication: u8,
    pub Screening_Indicator: u8,
}
impl ATM_CALLING_PARTY_NUMBER_IE {}
impl ::std::default::Default for ATM_CALLING_PARTY_NUMBER_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_CALLING_PARTY_NUMBER_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_CALLING_PARTY_NUMBER_IE").field("ATM_Number", &self.ATM_Number).field("Presentation_Indication", &self.Presentation_Indication).field("Screening_Indicator", &self.Screening_Indicator).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_CALLING_PARTY_NUMBER_IE {
    fn eq(&self, other: &Self) -> bool {
        self.ATM_Number == other.ATM_Number && self.Presentation_Indication == other.Presentation_Indication && self.Screening_Indicator == other.Screening_Indicator
    }
}
impl ::std::cmp::Eq for ATM_CALLING_PARTY_NUMBER_IE {}
unsafe impl ::windows::runtime::Abi for ATM_CALLING_PARTY_NUMBER_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_CAUSE_IE {
    pub Location: u8,
    pub Cause: u8,
    pub DiagnosticsLength: u8,
    pub Diagnostics: [u8; 4],
}
impl ATM_CAUSE_IE {}
impl ::std::default::Default for ATM_CAUSE_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_CAUSE_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_CAUSE_IE").field("Location", &self.Location).field("Cause", &self.Cause).field("DiagnosticsLength", &self.DiagnosticsLength).field("Diagnostics", &self.Diagnostics).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_CAUSE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location && self.Cause == other.Cause && self.DiagnosticsLength == other.DiagnosticsLength && self.Diagnostics == other.Diagnostics
    }
}
impl ::std::cmp::Eq for ATM_CAUSE_IE {}
unsafe impl ::windows::runtime::Abi for ATM_CAUSE_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_CONNECTION_ID {
    pub DeviceNumber: u32,
    pub VPI: u32,
    pub VCI: u32,
}
impl ATM_CONNECTION_ID {}
impl ::std::default::Default for ATM_CONNECTION_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_CONNECTION_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_CONNECTION_ID").field("DeviceNumber", &self.DeviceNumber).field("VPI", &self.VPI).field("VCI", &self.VCI).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_CONNECTION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceNumber == other.DeviceNumber && self.VPI == other.VPI && self.VCI == other.VCI
    }
}
impl ::std::cmp::Eq for ATM_CONNECTION_ID {}
unsafe impl ::windows::runtime::Abi for ATM_CONNECTION_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATM_E164: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ATM_NSAP: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
pub struct ATM_PVC_PARAMS {
    pub PvcConnectionId: ATM_CONNECTION_ID,
    pub PvcQos: super::super::NetworkManagement::QoS::QOS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
impl ATM_PVC_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
impl ::std::default::Default for ATM_PVC_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
impl ::std::cmp::PartialEq for ATM_PVC_PARAMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
impl ::std::cmp::Eq for ATM_PVC_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
unsafe impl ::windows::runtime::Abi for ATM_PVC_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_QOS_CLASS_IE {
    pub QOSClassForward: u8,
    pub QOSClassBackward: u8,
}
impl ATM_QOS_CLASS_IE {}
impl ::std::default::Default for ATM_QOS_CLASS_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_QOS_CLASS_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_QOS_CLASS_IE").field("QOSClassForward", &self.QOSClassForward).field("QOSClassBackward", &self.QOSClassBackward).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_QOS_CLASS_IE {
    fn eq(&self, other: &Self) -> bool {
        self.QOSClassForward == other.QOSClassForward && self.QOSClassBackward == other.QOSClassBackward
    }
}
impl ::std::cmp::Eq for ATM_QOS_CLASS_IE {}
unsafe impl ::windows::runtime::Abi for ATM_QOS_CLASS_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct ATM_TD {
    pub PeakCellRate_CLP0: u32,
    pub PeakCellRate_CLP01: u32,
    pub SustainableCellRate_CLP0: u32,
    pub SustainableCellRate_CLP01: u32,
    pub MaxBurstSize_CLP0: u32,
    pub MaxBurstSize_CLP01: u32,
    pub Tagging: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ATM_TD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ATM_TD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ATM_TD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_TD")
            .field("PeakCellRate_CLP0", &self.PeakCellRate_CLP0)
            .field("PeakCellRate_CLP01", &self.PeakCellRate_CLP01)
            .field("SustainableCellRate_CLP0", &self.SustainableCellRate_CLP0)
            .field("SustainableCellRate_CLP01", &self.SustainableCellRate_CLP01)
            .field("MaxBurstSize_CLP0", &self.MaxBurstSize_CLP0)
            .field("MaxBurstSize_CLP01", &self.MaxBurstSize_CLP01)
            .field("Tagging", &self.Tagging)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ATM_TD {
    fn eq(&self, other: &Self) -> bool {
        self.PeakCellRate_CLP0 == other.PeakCellRate_CLP0 && self.PeakCellRate_CLP01 == other.PeakCellRate_CLP01 && self.SustainableCellRate_CLP0 == other.SustainableCellRate_CLP0 && self.SustainableCellRate_CLP01 == other.SustainableCellRate_CLP01 && self.MaxBurstSize_CLP0 == other.MaxBurstSize_CLP0 && self.MaxBurstSize_CLP01 == other.MaxBurstSize_CLP01 && self.Tagging == other.Tagging
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ATM_TD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ATM_TD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct ATM_TRAFFIC_DESCRIPTOR_IE {
    pub Forward: ATM_TD,
    pub Backward: ATM_TD,
    pub BestEffort: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ATM_TRAFFIC_DESCRIPTOR_IE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_TRAFFIC_DESCRIPTOR_IE").field("Forward", &self.Forward).field("Backward", &self.Backward).field("BestEffort", &self.BestEffort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Forward == other.Forward && self.Backward == other.Backward && self.BestEffort == other.BestEffort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ATM_TRAFFIC_DESCRIPTOR_IE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ATM_TRAFFIC_DESCRIPTOR_IE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct ATM_TRANSIT_NETWORK_SELECTION_IE {
    pub TypeOfNetworkId: u8,
    pub NetworkIdPlan: u8,
    pub NetworkIdLength: u8,
    pub NetworkId: [u8; 1],
}
impl ATM_TRANSIT_NETWORK_SELECTION_IE {}
impl ::std::default::Default for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATM_TRANSIT_NETWORK_SELECTION_IE").field("TypeOfNetworkId", &self.TypeOfNetworkId).field("NetworkIdPlan", &self.NetworkIdPlan).field("NetworkIdLength", &self.NetworkIdLength).field("NetworkId", &self.NetworkId).finish()
    }
}
impl ::std::cmp::PartialEq for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn eq(&self, other: &Self) -> bool {
        self.TypeOfNetworkId == other.TypeOfNetworkId && self.NetworkIdPlan == other.NetworkIdPlan && self.NetworkIdLength == other.NetworkIdLength && self.NetworkId == other.NetworkId
    }
}
impl ::std::cmp::Eq for ATM_TRANSIT_NETWORK_SELECTION_IE {}
unsafe impl ::windows::runtime::Abi for ATM_TRANSIT_NETWORK_SELECTION_IE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn AcceptEx<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, SOCKET>>(slistensocket: Param0, sacceptsocket: Param1, lpoutputbuffer: *mut ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcceptEx(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AcceptEx(
            slistensocket.into_param().abi(),
            sacceptsocket.into_param().abi(),
            ::std::mem::transmute(lpoutputbuffer),
            ::std::mem::transmute(dwreceivedatalength),
            ::std::mem::transmute(dwlocaladdresslength),
            ::std::mem::transmute(dwremoteaddresslength),
            ::std::mem::transmute(lpdwbytesreceived),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BASE_PROTOCOL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BCOB_A: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BCOB_C: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BCOB_X: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BHLI_HighLayerProfile: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BHLI_ISO: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BHLI_UserSpecific: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BHLI_VendorSpecificAppId: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BIGENDIAN: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BITS_PER_BYTE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_ELAPB: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_HDLC_ABM: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_HDLC_ARM: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_HDLC_NRM: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_ISO_1745: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_ISO_7776: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_LLC: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_MODE_EXT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_MODE_NORMAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_Q921: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_Q922: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_USER_SPECIFIED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_X25L: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_X25M: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L2_X75: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_IPI_IP: u32 = 204u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_IPI_SNAP: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_ISO_8208: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_ISO_TR9577: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_MODE_EXT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_MODE_NORMAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_1024: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_128: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_16: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_2048: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_256: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_32: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_4096: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_512: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_PACKET_64: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_SIO_8473: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_T70: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_USER_SPECIFIED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_X223: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const BLLI_L3_X25: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_AAL_PARAMETERS_UNSUPPORTED: u32 = 93u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_ACCESS_INFORMAION_DISCARDED: u32 = 43u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_BEARER_CAPABILITY_UNAUTHORIZED: u32 = 57u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_BEARER_CAPABILITY_UNAVAILABLE: u32 = 58u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_BEARER_CAPABILITY_UNIMPLEMENTED: u32 = 65u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_CALL_REJECTED: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_CHANNEL_NONEXISTENT: u32 = 82u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_COND_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_COND_TRANSIENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_COND_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_DESTINATION_OUT_OF_ORDER: u32 = 27u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INCOMPATIBLE_DESTINATION: u32 = 88u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INCORRECT_MESSAGE_LENGTH: u32 = 104u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_CALL_REFERENCE: u32 = 81u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_ENDPOINT_REFERENCE: u32 = 89u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_IE_CONTENTS: u32 = 100u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_NUMBER_FORMAT: u32 = 28u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_STATE_FOR_MESSAGE: u32 = 101u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_INVALID_TRANSIT_NETWORK_SELECTION: u32 = 91u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_BEYOND_INTERWORKING: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_INTERNATIONAL_NETWORK: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_PRIVATE_LOCAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_PRIVATE_REMOTE: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_PUBLIC_LOCAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_PUBLIC_REMOTE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_TRANSIT_NETWORK: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_LOC_USER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_MANDATORY_IE_MISSING: u32 = 96u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NA_ABNORMAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NA_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NETWORK_OUT_OF_ORDER: u32 = 38u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NORMAL_CALL_CLEARING: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NORMAL_UNSPECIFIED: u32 = 31u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NO_ROUTE_TO_DESTINATION: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NO_ROUTE_TO_TRANSIT_NETWORK: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NO_USER_RESPONDING: u32 = 18u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NO_VPI_VCI_AVAILABLE: u32 = 45u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_NUMBER_CHANGED: u32 = 22u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_OPTION_UNAVAILABLE: u32 = 63u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_PROTOCOL_ERROR: u32 = 111u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_PU_PROVIDER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_PU_USER: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_QOS_UNAVAILABLE: u32 = 49u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_REASON_IE_INSUFFICIENT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_REASON_IE_MISSING: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_REASON_USER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_RECOVERY_ON_TIMEOUT: u32 = 102u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_RESOURCE_UNAVAILABLE: u32 = 47u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_STATUS_ENQUIRY_RESPONSE: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_TEMPORARY_FAILURE: u32 = 41u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_TOO_MANY_PENDING_ADD_PARTY: u32 = 92u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_UNALLOCATED_NUMBER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_UNIMPLEMENTED_IE: u32 = 99u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_UNIMPLEMENTED_MESSAGE_TYPE: u32 = 97u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_UNSUPPORTED_TRAFFIC_PARAMETERS: u32 = 73u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_USER_BUSY: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_USER_CELL_RATE_UNAVAILABLE: u32 = 51u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_USER_REJECTS_CLIR: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_VPI_VCI_UNACCEPTABLE: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CAUSE_VPI_VCI_UNAVAILABLE: u32 = 35u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CF_ACCEPT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CF_DEFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CF_REJECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CLIP_NOT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const CLIP_SUS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONTROL_CHANNEL_TRIGGER_STATUS(pub i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(0i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(1i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(2i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(3i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(4i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(5i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(6i32);
impl ::std::convert::From<i32> for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONTROL_CHANNEL_TRIGGER_STATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl CSADDR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CSADDR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CSADDR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CSADDR_INFO").field("LocalAddr", &self.LocalAddr).field("RemoteAddr", &self.RemoteAddr).field("iSocketType", &self.iSocketType).field("iProtocol", &self.iProtocol).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CSADDR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddr == other.LocalAddr && self.RemoteAddr == other.RemoteAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CSADDR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CSADDR_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const DE_REUSE_SOCKET: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn EnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(EnumProtocolsA(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn EnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(EnumProtocolsW(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_ACCEPT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_ACCEPT_BIT: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_ADDRESS_LIST_CHANGE_BIT: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_CLOSE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_CLOSE_BIT: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_CONNECT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_CONNECT_BIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_GROUP_QOS_BIT: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_MAX_EVENTS: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_OOB: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_OOB_BIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_QOS_BIT: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_READ_BIT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_SETSIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_WRITE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FD_WRITE_BIT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const FROM_PROTOCOL_INFO: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn FreeAddrInfoEx(paddrinfoex: *const addrinfoexA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeAddrInfoEx(paddrinfoex: *const addrinfoexA);
        }
        ::std::mem::transmute(FreeAddrInfoEx(::std::mem::transmute(paddrinfoex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn FreeAddrInfoExW(paddrinfoex: *const addrinfoexW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeAddrInfoExW(paddrinfoex: *const addrinfoexW);
        }
        ::std::mem::transmute(FreeAddrInfoExW(::std::mem::transmute(paddrinfoex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn FreeAddrInfoW(paddrinfo: *const addrinfoW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeAddrInfoW(paddrinfo: *const addrinfoW);
        }
        ::std::mem::transmute(FreeAddrInfoW(::std::mem::transmute(paddrinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const GAI_STRERROR_BUFFER_SIZE: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct GROUP_FILTER {
    pub gf_interface: u32,
    pub gf_group: SOCKADDR_STORAGE,
    pub gf_fmode: MULTICAST_MODE_TYPE,
    pub gf_numsrc: u32,
    pub gf_slist: [SOCKADDR_STORAGE; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl GROUP_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GROUP_FILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GROUP_FILTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GROUP_FILTER").field("gf_interface", &self.gf_interface).field("gf_group", &self.gf_group).field("gf_fmode", &self.gf_fmode).field("gf_numsrc", &self.gf_numsrc).field("gf_slist", &self.gf_slist).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GROUP_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.gf_interface == other.gf_interface && self.gf_group == other.gf_group && self.gf_fmode == other.gf_fmode && self.gf_numsrc == other.gf_numsrc && self.gf_slist == other.gf_slist
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GROUP_FILTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GROUP_FILTER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Foundation")]
impl GROUP_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GROUP_REQ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GROUP_REQ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GROUP_REQ").field("gr_interface", &self.gr_interface).field("gr_group", &self.gr_group).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GROUP_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gr_interface == other.gr_interface && self.gr_group == other.gr_group
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GROUP_REQ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GROUP_REQ {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct GROUP_SOURCE_REQ {
    pub gsr_interface: u32,
    pub gsr_group: SOCKADDR_STORAGE,
    pub gsr_source: SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Foundation")]
impl GROUP_SOURCE_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GROUP_SOURCE_REQ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GROUP_SOURCE_REQ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GROUP_SOURCE_REQ").field("gsr_interface", &self.gsr_interface).field("gsr_group", &self.gsr_group).field("gsr_source", &self.gsr_source).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GROUP_SOURCE_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gsr_interface == other.gsr_interface && self.gsr_group == other.gsr_group && self.gsr_source == other.gsr_source
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GROUP_SOURCE_REQ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GROUP_SOURCE_REQ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetAcceptExSockaddrs(lpoutputbuffer: *const ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAcceptExSockaddrs(lpoutputbuffer: *const ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
        }
        ::std::mem::transmute(GetAcceptExSockaddrs(
            ::std::mem::transmute(lpoutputbuffer),
            ::std::mem::transmute(dwreceivedatalength),
            ::std::mem::transmute(dwlocaladdresslength),
            ::std::mem::transmute(dwremoteaddresslength),
            ::std::mem::transmute(localsockaddr),
            ::std::mem::transmute(localsockaddrlength),
            ::std::mem::transmute(remotesockaddr),
            ::std::mem::transmute(remotesockaddrlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn GetAddrInfoExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    pname: Param0,
    pservicename: Param1,
    dwnamespace: u32,
    lpnspid: *const ::windows::runtime::GUID,
    hints: *const addrinfoexA,
    ppresult: *mut *mut addrinfoexA,
    timeout: *const timeval,
    lpoverlapped: *const super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<LPLOOKUPSERVICE_COMPLETION_ROUTINE>,
    lpnamehandle: *mut super::super::Foundation::HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, hints: *const addrinfoexA, ppresult: *mut *mut addrinfoexA, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(GetAddrInfoExA(
            pname.into_param().abi(),
            pservicename.into_param().abi(),
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpnspid),
            ::std::mem::transmute(hints),
            ::std::mem::transmute(ppresult),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
            ::std::mem::transmute(lpnamehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetAddrInfoExCancel(lphandle: *const super::super::Foundation::HANDLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddrInfoExCancel(lphandle: *const super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(GetAddrInfoExCancel(::std::mem::transmute(lphandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> i32;
        }
        ::std::mem::transmute(GetAddrInfoExOverlappedResult(::std::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn GetAddrInfoExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    pname: Param0,
    pservicename: Param1,
    dwnamespace: u32,
    lpnspid: *const ::windows::runtime::GUID,
    hints: *const addrinfoexW,
    ppresult: *mut *mut addrinfoexW,
    timeout: *const timeval,
    lpoverlapped: *const super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<LPLOOKUPSERVICE_COMPLETION_ROUTINE>,
    lphandle: *mut super::super::Foundation::HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, hints: *const addrinfoexW, ppresult: *mut *mut addrinfoexW, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lphandle: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(GetAddrInfoExW(
            pname.into_param().abi(),
            pservicename.into_param().abi(),
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpnspid),
            ::std::mem::transmute(hints),
            ::std::mem::transmute(ppresult),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
            ::std::mem::transmute(lphandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetAddrInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pnodename: Param0, pservicename: Param1, phints: *const addrinfoW, ppresult: *mut *mut addrinfoW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddrInfoW(pnodename: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, phints: *const addrinfoW, ppresult: *mut *mut addrinfoW) -> i32;
        }
        ::std::mem::transmute(GetAddrInfoW(pnodename.into_param().abi(), pservicename.into_param().abi(), ::std::mem::transmute(phints), ::std::mem::transmute(ppresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetAddressByNameA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwnamespace: u32, lpservicetype: *const ::windows::runtime::GUID, lpservicename: Param2, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpcsaddrbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: Param8, lpdwaliasbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddressByNameA(dwnamespace: u32, lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpcsaddrbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PSTR, lpdwaliasbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetAddressByNameA(
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpservicetype),
            lpservicename.into_param().abi(),
            ::std::mem::transmute(lpiprotocols),
            ::std::mem::transmute(dwresolution),
            ::std::mem::transmute(lpserviceasyncinfo),
            ::std::mem::transmute(lpcsaddrbuffer),
            ::std::mem::transmute(lpdwbufferlength),
            lpaliasbuffer.into_param().abi(),
            ::std::mem::transmute(lpdwaliasbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetAddressByNameW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    dwnamespace: u32,
    lpservicetype: *const ::windows::runtime::GUID,
    lpservicename: Param2,
    lpiprotocols: *const i32,
    dwresolution: u32,
    lpserviceasyncinfo: *const SERVICE_ASYNC_INFO,
    lpcsaddrbuffer: *mut ::std::ffi::c_void,
    lpdwbufferlength: *mut u32,
    lpaliasbuffer: Param8,
    lpdwaliasbufferlength: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAddressByNameW(dwnamespace: u32, lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpcsaddrbuffer: *mut ::std::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PWSTR, lpdwaliasbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetAddressByNameW(
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpservicetype),
            lpservicename.into_param().abi(),
            ::std::mem::transmute(lpiprotocols),
            ::std::mem::transmute(dwresolution),
            ::std::mem::transmute(lpserviceasyncinfo),
            ::std::mem::transmute(lpcsaddrbuffer),
            ::std::mem::transmute(lpdwbufferlength),
            lpaliasbuffer.into_param().abi(),
            ::std::mem::transmute(lpdwaliasbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetHostNameW(name: super::super::Foundation::PWSTR, namelen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHostNameW(name: super::super::Foundation::PWSTR, namelen: i32) -> i32;
        }
        ::std::mem::transmute(GetHostNameW(::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetNameByTypeA(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, dwnamelength: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNameByTypeA(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, dwnamelength: u32) -> i32;
        }
        ::std::mem::transmute(GetNameByTypeA(::std::mem::transmute(lpservicetype), ::std::mem::transmute(lpservicename), ::std::mem::transmute(dwnamelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetNameByTypeW(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, dwnamelength: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNameByTypeW(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, dwnamelength: u32) -> i32;
        }
        ::std::mem::transmute(GetNameByTypeW(::std::mem::transmute(lpservicetype), ::std::mem::transmute(lpservicename), ::std::mem::transmute(dwnamelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PWSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PWSTR, servicebuffersize: u32, flags: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PWSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PWSTR, servicebuffersize: u32, flags: i32) -> i32;
        }
        ::std::mem::transmute(GetNameInfoW(::std::mem::transmute(psockaddr), ::std::mem::transmute(sockaddrlength), ::std::mem::transmute(pnodebuffer), ::std::mem::transmute(nodebuffersize), ::std::mem::transmute(pservicebuffer), ::std::mem::transmute(servicebuffersize), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetServiceA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: Param2, dwproperties: u32, lpbuffer: *mut ::std::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceA(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, dwproperties: u32, lpbuffer: *mut ::std::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>) -> i32;
        }
        ::std::mem::transmute(GetServiceA(::std::mem::transmute(dwnamespace), ::std::mem::transmute(lpguid), lpservicename.into_param().abi(), ::std::mem::transmute(dwproperties), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwbuffersize), ::std::mem::transmute(lpserviceasyncinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetServiceW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: Param2, dwproperties: u32, lpbuffer: *mut ::std::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceW(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, dwproperties: u32, lpbuffer: *mut ::std::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>) -> i32;
        }
        ::std::mem::transmute(GetServiceW(::std::mem::transmute(dwnamespace), ::std::mem::transmute(lpguid), lpservicename.into_param().abi(), ::std::mem::transmute(dwproperties), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwbuffersize), ::std::mem::transmute(lpserviceasyncinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetTypeByNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpservicename: Param0, lpservicetype: *mut ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTypeByNameA(lpservicename: super::super::Foundation::PSTR, lpservicetype: *mut ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(GetTypeByNameA(lpservicename.into_param().abi(), ::std::mem::transmute(lpservicetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetTypeByNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpservicename: Param0, lpservicetype: *mut ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTypeByNameW(lpservicename: super::super::Foundation::PWSTR, lpservicetype: *mut ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(GetTypeByNameW(lpservicename.into_param().abi(), ::std::mem::transmute(lpservicetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HWSAEVENT(pub isize);
impl ::std::default::Default for HWSAEVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWSAEVENT {}
unsafe impl ::windows::runtime::Abi for HWSAEVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_ATTRIB_INT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_ATTRIB_NO_ATTRIB: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_ATTRIB_NO_CLASS: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_ATTRIB_OCTETSEQ: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_ATTRIB_STR: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_MAX_ATTRIBNAME: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_MAX_CLASSNAME: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_MAX_OCTET_STRING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IAS_MAX_USER_STRING: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct ICMP_ERROR_INFO {
    pub srcaddress: SOCKADDR_INET,
    pub protocol: IPPROTO,
    pub r#type: u8,
    pub code: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ICMP_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ICMP_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ICMP_ERROR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ICMP_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ICMP_ERROR_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IFF_BROADCAST: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IFF_LOOPBACK: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IFF_MULTICAST: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IFF_POINTTOPOINT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IFF_UP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IMPLINK_HIGHEXPER: u32 = 158u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IMPLINK_IP: u32 = 155u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IMPLINK_LOWEXPER: u32 = 156u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN4ADDR_LOOPBACK: u32 = 16777343u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN6_ADDR {
    pub u: IN6_ADDR_0,
}
impl IN6_ADDR {}
impl ::std::default::Default for IN6_ADDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN6_ADDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN6_ADDR {}
unsafe impl ::windows::runtime::Abi for IN6_ADDR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union IN6_ADDR_0 {
    pub Byte: [u8; 16],
    pub Word: [u16; 8],
}
impl IN6_ADDR_0 {}
impl ::std::default::Default for IN6_ADDR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN6_ADDR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN6_ADDR_0 {}
unsafe impl ::windows::runtime::Abi for IN6_ADDR_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN6_PKTINFO {
    pub ipi6_addr: IN6_ADDR,
    pub ipi6_ifindex: u32,
}
impl IN6_PKTINFO {}
impl ::std::default::Default for IN6_PKTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN6_PKTINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN6_PKTINFO {}
unsafe impl ::windows::runtime::Abi for IN6_PKTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INADDR_LOOPBACK: u32 = 2130706433u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INADDR_NONE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INCL_WINSOCK_API_PROTOTYPES: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INCL_WINSOCK_API_TYPEDEFS: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INET6_ADDRSTRLEN: u32 = 65u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INET_ADDRSTRLEN: u32 = 22u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct INET_PORT_RANGE {
    pub StartPort: u16,
    pub NumberOfPorts: u16,
}
impl INET_PORT_RANGE {}
impl ::std::default::Default for INET_PORT_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INET_PORT_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_PORT_RANGE").field("StartPort", &self.StartPort).field("NumberOfPorts", &self.NumberOfPorts).finish()
    }
}
impl ::std::cmp::PartialEq for INET_PORT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartPort == other.StartPort && self.NumberOfPorts == other.NumberOfPorts
    }
}
impl ::std::cmp::Eq for INET_PORT_RANGE {}
unsafe impl ::windows::runtime::Abi for INET_PORT_RANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct INET_PORT_RESERVATION_INFORMATION {
    pub OwningPid: u32,
}
impl INET_PORT_RESERVATION_INFORMATION {}
impl ::std::default::Default for INET_PORT_RESERVATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INET_PORT_RESERVATION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_PORT_RESERVATION_INFORMATION").field("OwningPid", &self.OwningPid).finish()
    }
}
impl ::std::cmp::PartialEq for INET_PORT_RESERVATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPid == other.OwningPid
    }
}
impl ::std::cmp::Eq for INET_PORT_RESERVATION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INET_PORT_RESERVATION_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct INET_PORT_RESERVATION_INSTANCE {
    pub Reservation: INET_PORT_RANGE,
    pub Token: INET_PORT_RESERVATION_TOKEN,
}
impl INET_PORT_RESERVATION_INSTANCE {}
impl ::std::default::Default for INET_PORT_RESERVATION_INSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INET_PORT_RESERVATION_INSTANCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_PORT_RESERVATION_INSTANCE").field("Reservation", &self.Reservation).field("Token", &self.Token).finish()
    }
}
impl ::std::cmp::PartialEq for INET_PORT_RESERVATION_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.Reservation == other.Reservation && self.Token == other.Token
    }
}
impl ::std::cmp::Eq for INET_PORT_RESERVATION_INSTANCE {}
unsafe impl ::windows::runtime::Abi for INET_PORT_RESERVATION_INSTANCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct INET_PORT_RESERVATION_TOKEN {
    pub Token: u64,
}
impl INET_PORT_RESERVATION_TOKEN {}
impl ::std::default::Default for INET_PORT_RESERVATION_TOKEN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INET_PORT_RESERVATION_TOKEN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_PORT_RESERVATION_TOKEN").field("Token", &self.Token).finish()
    }
}
impl ::std::cmp::PartialEq for INET_PORT_RESERVATION_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token
    }
}
impl ::std::cmp::Eq for INET_PORT_RESERVATION_TOKEN {}
unsafe impl ::windows::runtime::Abi for INET_PORT_RESERVATION_TOKEN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct INTERFACE_INFO {
    pub iiFlags: u32,
    pub iiAddress: sockaddr_gen,
    pub iiBroadcastAddress: sockaddr_gen,
    pub iiNetmask: sockaddr_gen,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERFACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INTERFACE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INTERFACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INTERFACE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: SOCKET_ADDRESS,
    pub iiBroadcastAddress: SOCKET_ADDRESS,
    pub iiNetmask: SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERFACE_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INTERFACE_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INTERFACE_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERFACE_INFO_EX").field("iiFlags", &self.iiFlags).field("iiAddress", &self.iiAddress).field("iiBroadcastAddress", &self.iiBroadcastAddress).field("iiNetmask", &self.iiNetmask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INTERFACE_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.iiFlags == other.iiFlags && self.iiAddress == other.iiAddress && self.iiBroadcastAddress == other.iiBroadcastAddress && self.iiNetmask == other.iiNetmask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INTERFACE_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INTERFACE_INFO_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const INVALID_SOCKET: SOCKET = SOCKET(4294967295u32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_ADDR {
    pub S_un: IN_ADDR_0,
}
impl IN_ADDR {}
impl ::std::default::Default for IN_ADDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN_ADDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN_ADDR {}
unsafe impl ::windows::runtime::Abi for IN_ADDR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union IN_ADDR_0 {
    pub S_un_b: IN_ADDR_0_0,
    pub S_un_w: IN_ADDR_0_1,
    pub S_addr: u32,
}
impl IN_ADDR_0 {}
impl ::std::default::Default for IN_ADDR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN_ADDR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN_ADDR_0 {}
unsafe impl ::windows::runtime::Abi for IN_ADDR_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_ADDR_0_0 {
    pub s_b1: u8,
    pub s_b2: u8,
    pub s_b3: u8,
    pub s_b4: u8,
}
impl IN_ADDR_0_0 {}
impl ::std::default::Default for IN_ADDR_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IN_ADDR_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_S_un_b_e__Struct").field("s_b1", &self.s_b1).field("s_b2", &self.s_b2).field("s_b3", &self.s_b3).field("s_b4", &self.s_b4).finish()
    }
}
impl ::std::cmp::PartialEq for IN_ADDR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.s_b1 == other.s_b1 && self.s_b2 == other.s_b2 && self.s_b3 == other.s_b3 && self.s_b4 == other.s_b4
    }
}
impl ::std::cmp::Eq for IN_ADDR_0_0 {}
unsafe impl ::windows::runtime::Abi for IN_ADDR_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_ADDR_0_1 {
    pub s_w1: u16,
    pub s_w2: u16,
}
impl IN_ADDR_0_1 {}
impl ::std::default::Default for IN_ADDR_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IN_ADDR_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_S_un_w_e__Struct").field("s_w1", &self.s_w1).field("s_w2", &self.s_w2).finish()
    }
}
impl ::std::cmp::PartialEq for IN_ADDR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.s_w1 == other.s_w1 && self.s_w2 == other.s_w2
    }
}
impl ::std::cmp::Eq for IN_ADDR_0_1 {}
unsafe impl ::windows::runtime::Abi for IN_ADDR_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSA_HOST: u32 = 16777215u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSA_MAX: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSA_NET: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSA_NSHIFT: u32 = 24u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSB_HOST: u32 = 65535u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSB_MAX: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSB_NET: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSB_NSHIFT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSC_HOST: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSC_NET: u32 = 4294967040u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSC_NSHIFT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSD_HOST: u32 = 268435455u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSD_NET: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IN_CLASSD_NSHIFT: u32 = 28u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_PKTINFO {
    pub ipi_addr: IN_ADDR,
    pub ipi_ifindex: u32,
}
impl IN_PKTINFO {}
impl ::std::default::Default for IN_PKTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN_PKTINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN_PKTINFO {}
unsafe impl ::windows::runtime::Abi for IN_PKTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_PKTINFO_EX {
    pub pkt_info: IN_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl IN_PKTINFO_EX {}
impl ::std::default::Default for IN_PKTINFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IN_PKTINFO_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IN_PKTINFO_EX {}
unsafe impl ::windows::runtime::Abi for IN_PKTINFO_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IN_RECVERR {
    pub protocol: IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
impl IN_RECVERR {}
impl ::std::default::Default for IN_RECVERR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IN_RECVERR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IN_RECVERR").field("protocol", &self.protocol).field("info", &self.info).field("r#type", &self.r#type).field("code", &self.code).finish()
    }
}
impl ::std::cmp::PartialEq for IN_RECVERR {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol && self.info == other.info && self.r#type == other.r#type && self.code == other.code
    }
}
impl ::std::cmp::Eq for IN_RECVERR {}
unsafe impl ::windows::runtime::Abi for IN_RECVERR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOCPARM_MASK: u32 = 127u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_IN: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_OUT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_PROTOCOL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_UNIX: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_VENDOR: u32 = 402653184u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_VOID: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IOC_WS2: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP6T_SO_ORIGINAL_DST: u32 = 12303u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_BIFFUDP: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_CHARGEN: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_CMDSERVER: u32 = 514u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_DAYTIME: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_DISCARD: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_DYNAMIC_MAX: u32 = 65535u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_DYNAMIC_MIN: u32 = 49152u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_ECHO: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_EFSSERVER: u32 = 520u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_EPMAP: u32 = 135u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_EXECSERVER: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_FINGER: u32 = 79u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_FTP: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_FTP_DATA: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_HTTPS: u32 = 443u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_IMAP: u32 = 143u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_IMAP3: u32 = 220u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_LDAP: u32 = 389u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_LOGINSERVER: u32 = 513u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_MICROSOFT_DS: u32 = 445u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_MSP: u32 = 18u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_MTP: u32 = 57u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NAMESERVER: u32 = 42u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NETBIOS_DGM: u32 = 138u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NETBIOS_NS: u32 = 137u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NETBIOS_SSN: u32 = 139u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NETSTAT: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_NTP: u32 = 123u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_POP3: u32 = 110u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_QOTD: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_REGISTERED_MAX: u32 = 49151u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_REGISTERED_MIN: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_RESERVED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_RJE: u32 = 77u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_ROUTESERVER: u32 = 520u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_SMTP: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_SNMP: u32 = 161u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_SNMP_TRAP: u32 = 162u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_SUPDUP: u32 = 95u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_SYSTAT: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_TCPMUX: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_TELNET: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_TFTP: u32 = 69u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_TIMESERVER: u32 = 37u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_TTYLINK: u32 = 87u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_WHOIS: u32 = 43u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPORT_WHOSERVER: u32 = 513u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IPPROTO(pub i32);
pub const IPPROTO_HOPOPTS: IPPROTO = IPPROTO(0i32);
pub const IPPROTO_ICMP: IPPROTO = IPPROTO(1i32);
pub const IPPROTO_IGMP: IPPROTO = IPPROTO(2i32);
pub const IPPROTO_GGP: IPPROTO = IPPROTO(3i32);
pub const IPPROTO_IPV4: IPPROTO = IPPROTO(4i32);
pub const IPPROTO_ST: IPPROTO = IPPROTO(5i32);
pub const IPPROTO_TCP: IPPROTO = IPPROTO(6i32);
pub const IPPROTO_CBT: IPPROTO = IPPROTO(7i32);
pub const IPPROTO_EGP: IPPROTO = IPPROTO(8i32);
pub const IPPROTO_IGP: IPPROTO = IPPROTO(9i32);
pub const IPPROTO_PUP: IPPROTO = IPPROTO(12i32);
pub const IPPROTO_UDP: IPPROTO = IPPROTO(17i32);
pub const IPPROTO_IDP: IPPROTO = IPPROTO(22i32);
pub const IPPROTO_RDP: IPPROTO = IPPROTO(27i32);
pub const IPPROTO_IPV6: IPPROTO = IPPROTO(41i32);
pub const IPPROTO_ROUTING: IPPROTO = IPPROTO(43i32);
pub const IPPROTO_FRAGMENT: IPPROTO = IPPROTO(44i32);
pub const IPPROTO_ESP: IPPROTO = IPPROTO(50i32);
pub const IPPROTO_AH: IPPROTO = IPPROTO(51i32);
pub const IPPROTO_ICMPV6: IPPROTO = IPPROTO(58i32);
pub const IPPROTO_NONE: IPPROTO = IPPROTO(59i32);
pub const IPPROTO_DSTOPTS: IPPROTO = IPPROTO(60i32);
pub const IPPROTO_ND: IPPROTO = IPPROTO(77i32);
pub const IPPROTO_ICLFXBM: IPPROTO = IPPROTO(78i32);
pub const IPPROTO_PIM: IPPROTO = IPPROTO(103i32);
pub const IPPROTO_PGM: IPPROTO = IPPROTO(113i32);
pub const IPPROTO_L2TP: IPPROTO = IPPROTO(115i32);
pub const IPPROTO_SCTP: IPPROTO = IPPROTO(132i32);
pub const IPPROTO_RAW: IPPROTO = IPPROTO(255i32);
pub const IPPROTO_MAX: IPPROTO = IPPROTO(256i32);
pub const IPPROTO_RESERVED_RAW: IPPROTO = IPPROTO(257i32);
pub const IPPROTO_RESERVED_IPSEC: IPPROTO = IPPROTO(258i32);
pub const IPPROTO_RESERVED_IPSECOFFLOAD: IPPROTO = IPPROTO(259i32);
pub const IPPROTO_RESERVED_WNV: IPPROTO = IPPROTO(260i32);
pub const IPPROTO_RESERVED_MAX: IPPROTO = IPPROTO(261i32);
impl ::std::convert::From<i32> for IPPROTO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPPROTO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPROTO_IP: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPPROTO_RM: u32 = 113u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_ADD_IFLIST: u32 = 29u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_ADD_MEMBERSHIP: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_CHECKSUM: u32 = 26u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_DEL_IFLIST: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_DONTFRAG: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_DROP_MEMBERSHIP: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_ECN: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_GET_IFLIST: u32 = 33u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_HDRINCL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_HOPLIMIT: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_HOPOPTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_IFLIST: u32 = 28u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_JOIN_GROUP: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_LEAVE_GROUP: u32 = 13u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: IN6_ADDR,
    pub ipv6mr_interface: u32,
}
impl IPV6_MREQ {}
impl ::std::default::Default for IPV6_MREQ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_MREQ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_MREQ {}
unsafe impl ::windows::runtime::Abi for IPV6_MREQ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_MTU: u32 = 72u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_MTU_DISCOVER: u32 = 71u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_MULTICAST_HOPS: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_MULTICAST_IF: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_MULTICAST_LOOP: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_NRT_INTERFACE: u32 = 74u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_PKTINFO: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_PKTINFO_EX: u32 = 51u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_PROTECTION_LEVEL: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVDSTADDR: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVECN: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVERR: u32 = 75u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVIF: u32 = 24u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVRTHDR: u32 = 38u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RECVTCLASS: u32 = 40u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_RTHDR: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_TCLASS: u32 = 39u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_UNICAST_HOPS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_UNICAST_IF: u32 = 31u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_USER_MTU: u32 = 76u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_V6ONLY: u32 = 27u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_ADDRESS: u32 = 16391u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct IPX_ADDRESS_DATA {
    pub adapternum: i32,
    pub netnum: [u8; 4],
    pub nodenum: [u8; 6],
    pub wan: super::super::Foundation::BOOLEAN,
    pub status: super::super::Foundation::BOOLEAN,
    pub maxpkt: i32,
    pub linkspeed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl IPX_ADDRESS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPX_ADDRESS_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPX_ADDRESS_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPX_ADDRESS_DATA").field("adapternum", &self.adapternum).field("netnum", &self.netnum).field("nodenum", &self.nodenum).field("wan", &self.wan).field("status", &self.status).field("maxpkt", &self.maxpkt).field("linkspeed", &self.linkspeed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPX_ADDRESS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.adapternum == other.adapternum && self.netnum == other.netnum && self.nodenum == other.nodenum && self.wan == other.wan && self.status == other.status && self.maxpkt == other.maxpkt && self.linkspeed == other.linkspeed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPX_ADDRESS_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPX_ADDRESS_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_ADDRESS_NOTIFY: u32 = 16396u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_DSTYPE: u32 = 16386u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_EXTENDED_ADDRESS: u32 = 16388u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_FILTERPTYPE: u32 = 16385u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_GETNETINFO: u32 = 16392u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_GETNETINFO_NORIP: u32 = 16393u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_IMMEDIATESPXACK: u32 = 16400u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_MAXSIZE: u32 = 16390u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_MAX_ADAPTER_NUM: u32 = 16397u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IPX_NETNUM_DATA {
    pub netnum: [u8; 4],
    pub hopcount: u16,
    pub netdelay: u16,
    pub cardnum: i32,
    pub router: [u8; 6],
}
impl IPX_NETNUM_DATA {}
impl ::std::default::Default for IPX_NETNUM_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPX_NETNUM_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPX_NETNUM_DATA").field("netnum", &self.netnum).field("hopcount", &self.hopcount).field("netdelay", &self.netdelay).field("cardnum", &self.cardnum).field("router", &self.router).finish()
    }
}
impl ::std::cmp::PartialEq for IPX_NETNUM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.netnum == other.netnum && self.hopcount == other.hopcount && self.netdelay == other.netdelay && self.cardnum == other.cardnum && self.router == other.router
    }
}
impl ::std::cmp::Eq for IPX_NETNUM_DATA {}
unsafe impl ::windows::runtime::Abi for IPX_NETNUM_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_PTYPE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_RECEIVE_BROADCAST: u32 = 16399u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_RECVHDR: u32 = 16389u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_RERIPNETNUMBER: u32 = 16398u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IPX_SPXCONNSTATUS_DATA {
    pub ConnectionState: u8,
    pub WatchDogActive: u8,
    pub LocalConnectionId: u16,
    pub RemoteConnectionId: u16,
    pub LocalSequenceNumber: u16,
    pub LocalAckNumber: u16,
    pub LocalAllocNumber: u16,
    pub RemoteAckNumber: u16,
    pub RemoteAllocNumber: u16,
    pub LocalSocket: u16,
    pub ImmediateAddress: [u8; 6],
    pub RemoteNetwork: [u8; 4],
    pub RemoteNode: [u8; 6],
    pub RemoteSocket: u16,
    pub RetransmissionCount: u16,
    pub EstimatedRoundTripDelay: u16,
    pub RetransmittedPackets: u16,
    pub SuppressedPacket: u16,
}
impl IPX_SPXCONNSTATUS_DATA {}
impl ::std::default::Default for IPX_SPXCONNSTATUS_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPX_SPXCONNSTATUS_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPX_SPXCONNSTATUS_DATA")
            .field("ConnectionState", &self.ConnectionState)
            .field("WatchDogActive", &self.WatchDogActive)
            .field("LocalConnectionId", &self.LocalConnectionId)
            .field("RemoteConnectionId", &self.RemoteConnectionId)
            .field("LocalSequenceNumber", &self.LocalSequenceNumber)
            .field("LocalAckNumber", &self.LocalAckNumber)
            .field("LocalAllocNumber", &self.LocalAllocNumber)
            .field("RemoteAckNumber", &self.RemoteAckNumber)
            .field("RemoteAllocNumber", &self.RemoteAllocNumber)
            .field("LocalSocket", &self.LocalSocket)
            .field("ImmediateAddress", &self.ImmediateAddress)
            .field("RemoteNetwork", &self.RemoteNetwork)
            .field("RemoteNode", &self.RemoteNode)
            .field("RemoteSocket", &self.RemoteSocket)
            .field("RetransmissionCount", &self.RetransmissionCount)
            .field("EstimatedRoundTripDelay", &self.EstimatedRoundTripDelay)
            .field("RetransmittedPackets", &self.RetransmittedPackets)
            .field("SuppressedPacket", &self.SuppressedPacket)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPX_SPXCONNSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionState == other.ConnectionState
            && self.WatchDogActive == other.WatchDogActive
            && self.LocalConnectionId == other.LocalConnectionId
            && self.RemoteConnectionId == other.RemoteConnectionId
            && self.LocalSequenceNumber == other.LocalSequenceNumber
            && self.LocalAckNumber == other.LocalAckNumber
            && self.LocalAllocNumber == other.LocalAllocNumber
            && self.RemoteAckNumber == other.RemoteAckNumber
            && self.RemoteAllocNumber == other.RemoteAllocNumber
            && self.LocalSocket == other.LocalSocket
            && self.ImmediateAddress == other.ImmediateAddress
            && self.RemoteNetwork == other.RemoteNetwork
            && self.RemoteNode == other.RemoteNode
            && self.RemoteSocket == other.RemoteSocket
            && self.RetransmissionCount == other.RetransmissionCount
            && self.EstimatedRoundTripDelay == other.EstimatedRoundTripDelay
            && self.RetransmittedPackets == other.RetransmittedPackets
            && self.SuppressedPacket == other.SuppressedPacket
    }
}
impl ::std::cmp::Eq for IPX_SPXCONNSTATUS_DATA {}
unsafe impl ::windows::runtime::Abi for IPX_SPXCONNSTATUS_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_SPXGETCONNECTIONSTATUS: u32 = 16395u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IPX_STOPFILTERPTYPE: u32 = 16387u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_ADD_IFLIST: u32 = 29u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_ADD_MEMBERSHIP: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_BLOCK_SOURCE: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DEL_IFLIST: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DONTFRAGMENT: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DROP_MEMBERSHIP: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_ECN: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_GET_IFLIST: u32 = 33u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_HDRINCL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_HOPLIMIT: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_IFLIST: u32 = 28u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MAX_MEMBERSHIPS: u32 = 20u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IP_MREQ {
    pub imr_multiaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl IP_MREQ {}
impl ::std::default::Default for IP_MREQ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IP_MREQ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IP_MREQ {}
unsafe impl ::windows::runtime::Abi for IP_MREQ {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IP_MREQ_SOURCE {
    pub imr_multiaddr: IN_ADDR,
    pub imr_sourceaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl IP_MREQ_SOURCE {}
impl ::std::default::Default for IP_MREQ_SOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IP_MREQ_SOURCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IP_MREQ_SOURCE {}
unsafe impl ::windows::runtime::Abi for IP_MREQ_SOURCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct IP_MSFILTER {
    pub imsf_multiaddr: IN_ADDR,
    pub imsf_interface: IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [IN_ADDR; 1],
}
impl IP_MSFILTER {}
impl ::std::default::Default for IP_MSFILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IP_MSFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IP_MSFILTER {}
unsafe impl ::windows::runtime::Abi for IP_MSFILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MTU: u32 = 73u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MTU_DISCOVER: u32 = 71u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MULTICAST_IF: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MULTICAST_LOOP: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_MULTICAST_TTL: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_NRT_INTERFACE: u32 = 74u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_OPTIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_PKTINFO: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_PKTINFO_EX: u32 = 51u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_PROTECTION_LEVEL: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECEIVE_BROADCAST: u32 = 22u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVDSTADDR: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVECN: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVERR: u32 = 75u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVIF: u32 = 24u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVRTHDR: u32 = 38u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVTCLASS: u32 = 40u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVTOS: u32 = 40u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RECVTTL: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_RTHDR: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_TCLASS: u32 = 39u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_TOS: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_TTL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_UNBLOCK_SOURCE: u32 = 18u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_UNICAST_IF: u32 = 31u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_UNSPECIFIED_USER_MTU: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_USER_MTU: u32 = 76u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRDA_PROTO_SOCK_STREAM: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_9WIRE_MODE: u32 = 22u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_DISCOVERY_MODE: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_ENUMDEVICES: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_EXCLUSIVE_MODE: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_IAS_QUERY: u32 = 18u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_IAS_SET: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_IRLPT_MODE: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_PARAMETERS: u32 = 24u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_SEND_PDU_LEN: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_SHARP_MODE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const IRLMP_TINYTP_MODE: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_CLNP: u32 = 31u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_CLTP: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_ESIS: u32 = 34u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_INACT_NL: u32 = 33u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_INTRAISIS: u32 = 35u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP: u32 = 29u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP0: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP1: u32 = 26u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP2: u32 = 27u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP3: u32 = 28u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_TP4: u32 = 29u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISOPROTO_X25: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISO_EXP_DATA_NUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISO_EXP_DATA_USE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISO_HIERARCHICAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISO_MAX_ADDR_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const ISO_NON_HIERARCHICAL: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InetNtopW(family: i32, paddr: *const ::std::ffi::c_void, pstringbuf: super::super::Foundation::PWSTR, stringbufsize: usize) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InetNtopW(family: i32, paddr: *const ::std::ffi::c_void, pstringbuf: super::super::Foundation::PWSTR, stringbufsize: usize) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(InetNtopW(::std::mem::transmute(family), ::std::mem::transmute(paddr), ::std::mem::transmute(pstringbuf), ::std::mem::transmute(stringbufsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InetPtonW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(family: i32, pszaddrstring: Param1, paddrbuf: *mut ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InetPtonW(family: i32, pszaddrstring: super::super::Foundation::PWSTR, paddrbuf: *mut ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(InetPtonW(::std::mem::transmute(family), pszaddrstring.into_param().abi(), ::std::mem::transmute(paddrbuf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const JL_BOTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const JL_RECEIVER_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const JL_SENDER_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LAYERED_PROTOCOL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LITTLEENDIAN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_115200: u32 = 115200u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_1152K: u32 = 1152000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_1200: u32 = 1200u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_16M: u32 = 16000000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_19200: u32 = 19200u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_2400: u32 = 2400u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_38400: u32 = 38400u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_4M: u32 = 4000000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_57600: u32 = 57600u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_576K: u32 = 576000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_BAUD_9600: u32 = 9600u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_Computer: i32 = 4i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_Fax: i32 = 32i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_LANAccess: i32 = 64i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_Modem: i32 = 16i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_PDA_Palmtop: i32 = 2i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_PnP: i32 = 1i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB1_Printer: i32 = 8i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB2_FileServer: i32 = 2i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB2_Telephony: i32 = 1i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LM_HB_Extension: i32 = 128i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct LM_IRPARMS {
    pub nTXDataBytes: u32,
    pub nRXDataBytes: u32,
    pub nBaudRate: u32,
    pub thresholdTime: u32,
    pub discTime: u32,
    pub nMSLinkTurn: u16,
    pub nTXPackets: u8,
    pub nRXPackets: u8,
}
impl LM_IRPARMS {}
impl ::std::default::Default for LM_IRPARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LM_IRPARMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LM_IRPARMS")
            .field("nTXDataBytes", &self.nTXDataBytes)
            .field("nRXDataBytes", &self.nRXDataBytes)
            .field("nBaudRate", &self.nBaudRate)
            .field("thresholdTime", &self.thresholdTime)
            .field("discTime", &self.discTime)
            .field("nMSLinkTurn", &self.nMSLinkTurn)
            .field("nTXPackets", &self.nTXPackets)
            .field("nRXPackets", &self.nRXPackets)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LM_IRPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.nTXDataBytes == other.nTXDataBytes && self.nRXDataBytes == other.nRXDataBytes && self.nBaudRate == other.nBaudRate && self.thresholdTime == other.thresholdTime && self.discTime == other.discTime && self.nMSLinkTurn == other.nMSLinkTurn && self.nTXPackets == other.nTXPackets && self.nRXPackets == other.nRXPackets
    }
}
impl ::std::cmp::Eq for LM_IRPARMS {}
unsafe impl ::windows::runtime::Abi for LM_IRPARMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LOG2_BITS_PER_BYTE: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPBLOCKINGCALLBACK = unsafe extern "system" fn(dwcontext: usize) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPCONDITIONPROC = unsafe extern "system" fn(lpcallerid: *mut WSABUF, lpcallerdata: *mut WSABUF, lpsqos: *mut super::super::NetworkManagement::QoS::QOS, lpgqos: *mut super::super::NetworkManagement::QoS::QOS, lpcalleeid: *mut WSABUF, lpcalleedata: *mut WSABUF, g: *mut u32, dwcallbackdata: usize) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_ACCEPTEX = unsafe extern "system" fn(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_CONNECTEX = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpsendbuffer: *const ::std::ffi::c_void, dwsenddatalength: u32, lpdwbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_DISCONNECTEX = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_GETACCEPTEXSOCKADDRS = unsafe extern "system" fn(lpoutputbuffer: *const ::std::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
pub type LPFN_NSPAPI = unsafe extern "system" fn() -> u32;
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = unsafe extern "system" fn(cq: *const RIO_CQ_t);
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOCREATECOMPLETIONQUEUE = unsafe extern "system" fn(queuesize: u32, notificationcompletion: *const RIO_NOTIFICATION_COMPLETION) -> *mut RIO_CQ_t;
pub type LPFN_RIOCREATEREQUESTQUEUE = unsafe extern "system" fn(socket: SOCKET, maxoutstandingreceive: u32, maxreceivedatabuffers: u32, maxoutstandingsend: u32, maxsenddatabuffers: u32, receivecq: *const RIO_CQ_t, sendcq: *const RIO_CQ_t, socketcontext: *const ::std::ffi::c_void) -> *mut RIO_RQ_t;
pub type LPFN_RIODEQUEUECOMPLETION = unsafe extern "system" fn(cq: *const RIO_CQ_t, array: *mut RIORESULT, arraysize: u32) -> u32;
pub type LPFN_RIODEREGISTERBUFFER = unsafe extern "system" fn(bufferid: *const RIO_BUFFERID_t);
pub type LPFN_RIONOTIFY = unsafe extern "system" fn(cq: *const RIO_CQ_t) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORECEIVE = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
pub type LPFN_RIORECEIVEEX = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::std::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOREGISTERBUFFER = unsafe extern "system" fn(databuffer: super::super::Foundation::PSTR, datalength: u32) -> *mut RIO_BUFFERID_t;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORESIZECOMPLETIONQUEUE = unsafe extern "system" fn(cq: *const RIO_CQ_t, queuesize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORESIZEREQUESTQUEUE = unsafe extern "system" fn(rq: *const RIO_RQ_t, maxoutstandingreceive: u32, maxoutstandingsend: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOSEND = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOSENDEX = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_TRANSMITFILE = unsafe extern "system" fn(hsocket: SOCKET, hfile: super::super::Foundation::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_TRANSMITPACKETS = unsafe extern "system" fn(hsocket: SOCKET, lppacketarray: *const TRANSMIT_PACKETS_ELEMENT, nelementcount: u32, nsendsize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32) -> super::super::Foundation::BOOL;
pub type LPFN_WSAPOLL = unsafe extern "system" fn(fdarray: *mut WSAPOLLFD, nfds: u32, timeout: i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_WSARECVMSG = unsafe extern "system" fn(s: SOCKET, lpmsg: *mut WSAMSG, lpdwnumberofbytesrecvd: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_WSASENDMSG = unsafe extern "system" fn(s: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED);
pub type LPNSPCLEANUP = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPGETSERVICECLASSINFO = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpdwbufsize: *const u32, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPINSTALLSERVICECLASS = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPNSPIOCTL = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const ::std::mem::ManuallyDrop<WSACOMPLETION>, lpthreadid: *const WSATHREADID) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPLOOKUPSERVICEBEGIN = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpqsrestrictions: *const WSAQUERYSETW, lpserviceclassinfo: *const WSASERVICECLASSINFOW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPLOOKUPSERVICEEND = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPLOOKUPSERVICENEXT = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
pub type LPNSPREMOVESERVICECLASS = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPSETSERVICE = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW, lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
pub type LPNSPSTARTUP = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpnsproutines: *mut ::std::mem::ManuallyDrop<NSP_ROUTINE>) -> i32;
pub type LPNSPV2CLEANUP = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, pvclientsessionarg: *const ::std::ffi::c_void) -> i32;
pub type LPNSPV2CLIENTSESSIONRUNDOWN = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, pvclientsessionarg: *const ::std::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2LOOKUPSERVICEBEGIN = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpqsrestrictions: *const WSAQUERYSET2W, dwcontrolflags: u32, lpvclientsessionarg: *const ::std::ffi::c_void, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPV2LOOKUPSERVICEEND = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2LOOKUPSERVICENEXTEX = unsafe extern "system" fn(hasynccall: super::super::Foundation::HANDLE, hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *const u32, lpqsresults: *mut WSAQUERYSET2W);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2SETSERVICEEX = unsafe extern "system" fn(hasynccall: super::super::Foundation::HANDLE, lpproviderid: *const ::windows::runtime::GUID, lpqsreginfo: *const WSAQUERYSET2W, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32, lpvclientsessionarg: *const ::std::ffi::c_void);
pub type LPNSPV2STARTUP = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, ppvclientsessionarg: *mut *mut ::std::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_CALLBACK_PROC = unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, hasynctaskhandle: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCLOSEEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
pub type LPWPUCLOSESOCKETHANDLE = unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCLOSETHREAD = unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWPUCOMPLETEOVERLAPPEDREQUEST = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCREATEEVENT = unsafe extern "system" fn(lperrno: *mut i32) -> super::super::Foundation::HANDLE;
pub type LPWPUCREATESOCKETHANDLE = unsafe extern "system" fn(dwcatalogentryid: u32, dwcontext: usize, lperrno: *mut i32) -> SOCKET;
pub type LPWPUFDISSET = unsafe extern "system" fn(s: SOCKET, fdset: *const fd_set) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUGETPROVIDERPATH = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
pub type LPWPUMODIFYIFSHANDLE = unsafe extern "system" fn(dwcatalogentryid: u32, proposedhandle: SOCKET, lperrno: *mut i32) -> SOCKET;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUOPENCURRENTTHREAD = unsafe extern "system" fn(lpthreadid: *mut WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUPOSTMESSAGE = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUQUERYBLOCKINGCALLBACK = unsafe extern "system" fn(dwcatalogentryid: u32, lplpfncallback: *mut ::windows::runtime::RawPtr, lpdwcontext: *mut usize, lperrno: *mut i32) -> i32;
pub type LPWPUQUERYSOCKETHANDLECONTEXT = unsafe extern "system" fn(s: SOCKET, lpcontext: *mut usize, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUQUEUEAPC = unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lpfnuserapc: ::windows::runtime::RawPtr, dwcontext: usize, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPURESETEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUSETEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "system" fn(dwerror: u32, cbtransferred: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32);
pub type LPWSAUSERAPC = unsafe extern "system" fn(dwcontext: usize);
pub type LPWSCDEINSTALLPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCENABLENSPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> i32;
pub type LPWSCENUMPROTOCOLS = unsafe extern "system" fn(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCGETPROVIDERPATH = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCINSTALLNAMESPACE = unsafe extern "system" fn(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCINSTALLPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
pub type LPWSCUNINSTALLNAMESPACE = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCUPDATEPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
pub type LPWSCWRITENAMESPACEORDER = unsafe extern "system" fn(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32;
pub type LPWSCWRITEPROVIDERORDER = unsafe extern "system" fn(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPACCEPT = unsafe extern "system" fn(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: ::windows::runtime::RawPtr, dwcallbackdata: usize, lperrno: *mut i32) -> SOCKET;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPADDRESSTOSTRING = unsafe extern "system" fn(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPASYNCSELECT = unsafe extern "system" fn(s: SOCKET, hwnd: super::super::Foundation::HWND, wmsg: u32, levent: i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPBIND = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lperrno: *mut i32) -> i32;
pub type LPWSPCANCELBLOCKINGCALL = unsafe extern "system" fn(lperrno: *mut i32) -> i32;
pub type LPWSPCLEANUP = unsafe extern "system" fn(lperrno: *mut i32) -> i32;
pub type LPWSPCLOSESOCKET = unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPCONNECT = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, lperrno: *mut i32) -> i32;
pub type LPWSPDUPLICATESOCKET = unsafe extern "system" fn(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPENUMNETWORKEVENTS = unsafe extern "system" fn(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPEVENTSELECT = unsafe extern "system" fn(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lnetworkevents: i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPGETOVERLAPPEDRESULT = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: super::super::Foundation::BOOL, lpdwflags: *mut u32, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETPEERNAME = unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPGETQOSBYNAME = unsafe extern "system" fn(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETSOCKNAME = unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETSOCKOPT = unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPIOCTL = unsafe extern "system" fn(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPJOINLEAF = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32, lperrno: *mut i32) -> SOCKET;
pub type LPWSPLISTEN = unsafe extern "system" fn(s: SOCKET, backlog: i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPRECV = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpthreadid: *const WSATHREADID, lperrno: *const i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPRECVDISCONNECT = unsafe extern "system" fn(s: SOCKET, lpinbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPRECVFROM = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
pub type LPWSPSELECT = unsafe extern "system" fn(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPSEND = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSENDDISCONNECT = unsafe extern "system" fn(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPSENDTO = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSETSOCKOPT = unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: i32, lperrno: *mut i32) -> i32;
pub type LPWSPSHUTDOWN = unsafe extern "system" fn(s: SOCKET, how: i32, lperrno: *mut i32) -> i32;
pub type LPWSPSOCKET = unsafe extern "system" fn(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32, lperrno: *mut i32) -> SOCKET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
pub type LPWSPSTARTUP = unsafe extern "system" fn(wversionrequested: u16, lpwspdata: *const WSPData, lpprotocolinfo: *const WSAPROTOCOL_INFOW, upcalltable: ::std::mem::ManuallyDrop<WSPUPCALLTABLE>, lpproctable: *mut ::std::mem::ManuallyDrop<WSPPROC_TABLE>) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSTRINGTOADDRESS = unsafe extern "system" fn(addressstring: super::super::Foundation::PWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32, lperrno: *mut i32) -> i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_CRYPTO_COMPRESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_FIREWALL: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_INBOUND_MODIFY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_INSPECTOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_LOCAL_CACHE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_OUTBOUND_MODIFY: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_PROXY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_REDIRECTOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LSP_SYSTEM: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_ADDRCONFIG: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_API_ANSI: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_CONTAINERS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_DEEP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_DISABLE_IDN_ENCODING: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_DNS_ONLY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_DUAL_ADDR: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_EXTENDED_QUERYSET: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_FILESERVER: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_FLUSHCACHE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_FLUSHPREVIOUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_FORCE_CLEAR_TEXT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_NEAREST: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_NOCONTAINERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_NON_AUTHORITATIVE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_REQUIRE_SECURE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RES_SERVICE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_ADDR: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_ALIASES: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_ALL: u32 = 4080u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_BLOB: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_COMMENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_NAME: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_PREFERRED_NAMES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_QUERY_STRING: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_RESPONSE_FLAGS: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_TTL: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_TYPE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_RETURN_VERSION: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_SECURE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LUP_SECURE_WITH_FALLBACK: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetASCII: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_3: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_4: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_5: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_6: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_7: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_8: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetISO_8859_9: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const LmCharSetUNICODE: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MAXGETHOSTSTRUCT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MAX_MCAST_TTL: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MAX_PROTOCOL_CHAIN: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MAX_WINDOW_INCREMENT_PERCENTAGE: u32 = 25u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_BLOCK_SOURCE: u32 = 43u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_JOIN_GROUP: u32 = 41u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_LEAVE_GROUP: u32 = 42u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MCAST_UNBLOCK_SOURCE: u32 = 44u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_BCAST: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_CTRUNC: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_ERRQUEUE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_INTERRUPT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_MAXIOVLEN: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_MCAST: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_PARTIAL: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_PEEK: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_PUSH_IMMEDIATE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_TRUNC: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const MSG_WAITALL: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MULTICAST_MODE_TYPE(pub i32);
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(0i32);
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(1i32);
impl ::std::convert::From<i32> for MULTICAST_MODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MULTICAST_MODE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NAPI_DOMAIN_DESCRIPTION_BLOB {
    pub AuthLevel: u32,
    pub cchDomainName: u32,
    pub OffsetNextDomainDescription: u32,
    pub OffsetThisDomainName: u32,
}
impl NAPI_DOMAIN_DESCRIPTION_BLOB {}
impl ::std::default::Default for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NAPI_DOMAIN_DESCRIPTION_BLOB").field("AuthLevel", &self.AuthLevel).field("cchDomainName", &self.cchDomainName).field("OffsetNextDomainDescription", &self.OffsetNextDomainDescription).field("OffsetThisDomainName", &self.OffsetThisDomainName).finish()
    }
}
impl ::std::cmp::PartialEq for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.AuthLevel == other.AuthLevel && self.cchDomainName == other.cchDomainName && self.OffsetNextDomainDescription == other.OffsetNextDomainDescription && self.OffsetThisDomainName == other.OffsetThisDomainName
    }
}
impl ::std::cmp::Eq for NAPI_DOMAIN_DESCRIPTION_BLOB {}
unsafe impl ::windows::runtime::Abi for NAPI_DOMAIN_DESCRIPTION_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NAPI_PROVIDER_INSTALLATION_BLOB {
    pub dwVersion: u32,
    pub dwProviderType: u32,
    pub fSupportsWildCard: u32,
    pub cDomains: u32,
    pub OffsetFirstDomain: u32,
}
impl NAPI_PROVIDER_INSTALLATION_BLOB {}
impl ::std::default::Default for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NAPI_PROVIDER_INSTALLATION_BLOB").field("dwVersion", &self.dwVersion).field("dwProviderType", &self.dwProviderType).field("fSupportsWildCard", &self.fSupportsWildCard).field("cDomains", &self.cDomains).field("OffsetFirstDomain", &self.OffsetFirstDomain).finish()
    }
}
impl ::std::cmp::PartialEq for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwProviderType == other.dwProviderType && self.fSupportsWildCard == other.fSupportsWildCard && self.cDomains == other.cDomains && self.OffsetFirstDomain == other.OffsetFirstDomain
    }
}
impl ::std::cmp::Eq for NAPI_PROVIDER_INSTALLATION_BLOB {}
unsafe impl ::windows::runtime::Abi for NAPI_PROVIDER_INSTALLATION_BLOB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NAPI_PROVIDER_LEVEL(pub i32);
pub const ProviderLevel_None: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(0i32);
pub const ProviderLevel_Secondary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(1i32);
pub const ProviderLevel_Primary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(2i32);
impl ::std::convert::From<i32> for NAPI_PROVIDER_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NAPI_PROVIDER_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NAPI_PROVIDER_TYPE(pub i32);
pub const ProviderType_Application: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(1i32);
pub const ProviderType_Service: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(2i32);
impl ::std::convert::From<i32> for NAPI_PROVIDER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NAPI_PROVIDER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NETBIOS_GROUP_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NETBIOS_NAME_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NETBIOS_TYPE_QUICK_GROUP: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NETBIOS_TYPE_QUICK_UNIQUE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NETBIOS_UNIQUE_NAME: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NETRESOURCE2A {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: super::super::Foundation::PSTR,
    pub lpRemoteName: super::super::Foundation::PSTR,
    pub lpComment: super::super::Foundation::PSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: ::windows::runtime::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NETRESOURCE2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETRESOURCE2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETRESOURCE2A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETRESOURCE2A")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwUsage", &self.dwUsage)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("ns_info", &self.ns_info)
            .field("ServiceType", &self.ServiceType)
            .field("dwProtocols", &self.dwProtocols)
            .field("lpiProtocols", &self.lpiProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETRESOURCE2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwUsage == other.dwUsage && self.dwDisplayType == other.dwDisplayType && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.ns_info == other.ns_info && self.ServiceType == other.ServiceType && self.dwProtocols == other.dwProtocols && self.lpiProtocols == other.lpiProtocols
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETRESOURCE2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETRESOURCE2A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NETRESOURCE2W {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: super::super::Foundation::PWSTR,
    pub lpRemoteName: super::super::Foundation::PWSTR,
    pub lpComment: super::super::Foundation::PWSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: ::windows::runtime::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NETRESOURCE2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETRESOURCE2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETRESOURCE2W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETRESOURCE2W")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwUsage", &self.dwUsage)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("ns_info", &self.ns_info)
            .field("ServiceType", &self.ServiceType)
            .field("dwProtocols", &self.dwProtocols)
            .field("lpiProtocols", &self.lpiProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETRESOURCE2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwUsage == other.dwUsage && self.dwDisplayType == other.dwDisplayType && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.ns_info == other.ns_info && self.ServiceType == other.ServiceType && self.dwProtocols == other.dwProtocols && self.lpiProtocols == other.lpiProtocols
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETRESOURCE2W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETRESOURCE2W {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_DGRAM: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_MAXHOST: u32 = 1025u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_MAXSERV: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_NAMEREQD: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_NOFQDN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_NUMERICHOST: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NI_NUMERICSERV: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NLA_ALLUSERS_NETWORK: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NLA_BLOB {
    pub header: NLA_BLOB_1,
    pub data: NLA_BLOB_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NLA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLA_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLA_BLOB {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLA_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union NLA_BLOB_0 {
    pub rawData: [super::super::Foundation::CHAR; 1],
    pub interfaceData: NLA_BLOB_0_2,
    pub locationData: NLA_BLOB_0_3,
    pub connectivity: NLA_BLOB_0_1,
    pub ICS: NLA_BLOB_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NLA_BLOB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLA_BLOB_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLA_BLOB_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLA_BLOB_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NLA_BLOB_0_0 {
    pub remote: NLA_BLOB_0_0_0,
}
impl NLA_BLOB_0_0 {}
impl ::std::default::Default for NLA_BLOB_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLA_BLOB_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ICS_e__Struct").field("remote", &self.remote).finish()
    }
}
impl ::std::cmp::PartialEq for NLA_BLOB_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.remote == other.remote
    }
}
impl ::std::cmp::Eq for NLA_BLOB_0_0 {}
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NLA_BLOB_0_0_0 {
    pub speed: u32,
    pub r#type: u32,
    pub state: u32,
    pub machineName: [u16; 256],
    pub sharedAdapterName: [u16; 256],
}
impl NLA_BLOB_0_0_0 {}
impl ::std::default::Default for NLA_BLOB_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLA_BLOB_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_remote_e__Struct").field("speed", &self.speed).field("r#type", &self.r#type).field("state", &self.state).field("machineName", &self.machineName).field("sharedAdapterName", &self.sharedAdapterName).finish()
    }
}
impl ::std::cmp::PartialEq for NLA_BLOB_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed && self.r#type == other.r#type && self.state == other.state && self.machineName == other.machineName && self.sharedAdapterName == other.sharedAdapterName
    }
}
impl ::std::cmp::Eq for NLA_BLOB_0_0_0 {}
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NLA_BLOB_0_1 {
    pub r#type: NLA_CONNECTIVITY_TYPE,
    pub internet: NLA_INTERNET,
}
impl NLA_BLOB_0_1 {}
impl ::std::default::Default for NLA_BLOB_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLA_BLOB_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_connectivity_e__Struct").field("r#type", &self.r#type).field("internet", &self.internet).finish()
    }
}
impl ::std::cmp::PartialEq for NLA_BLOB_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.internet == other.internet
    }
}
impl ::std::cmp::Eq for NLA_BLOB_0_1 {}
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NLA_BLOB_0_2 {
    pub dwType: u32,
    pub dwSpeed: u32,
    pub adapterName: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl NLA_BLOB_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLA_BLOB_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NLA_BLOB_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_interfaceData_e__Struct").field("dwType", &self.dwType).field("dwSpeed", &self.dwSpeed).field("adapterName", &self.adapterName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLA_BLOB_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.dwSpeed == other.dwSpeed && self.adapterName == other.adapterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLA_BLOB_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NLA_BLOB_0_3 {
    pub information: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl NLA_BLOB_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLA_BLOB_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NLA_BLOB_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_locationData_e__Struct").field("information", &self.information).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLA_BLOB_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.information == other.information
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLA_BLOB_0_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLA_BLOB_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NLA_BLOB_1 {
    pub r#type: NLA_BLOB_DATA_TYPE,
    pub dwSize: u32,
    pub nextOffset: u32,
}
impl NLA_BLOB_1 {}
impl ::std::default::Default for NLA_BLOB_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLA_BLOB_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_header_e__Struct").field("r#type", &self.r#type).field("dwSize", &self.dwSize).field("nextOffset", &self.nextOffset).finish()
    }
}
impl ::std::cmp::PartialEq for NLA_BLOB_1 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.dwSize == other.dwSize && self.nextOffset == other.nextOffset
    }
}
impl ::std::cmp::Eq for NLA_BLOB_1 {}
unsafe impl ::windows::runtime::Abi for NLA_BLOB_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLA_BLOB_DATA_TYPE(pub i32);
pub const NLA_RAW_DATA: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(0i32);
pub const NLA_INTERFACE: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(1i32);
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(2i32);
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(3i32);
pub const NLA_ICS: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(4i32);
impl ::std::convert::From<i32> for NLA_BLOB_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLA_BLOB_DATA_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLA_CONNECTIVITY_TYPE(pub i32);
pub const NLA_NETWORK_AD_HOC: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(0i32);
pub const NLA_NETWORK_MANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(1i32);
pub const NLA_NETWORK_UNMANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(2i32);
pub const NLA_NETWORK_UNKNOWN: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(3i32);
impl ::std::convert::From<i32> for NLA_CONNECTIVITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLA_CONNECTIVITY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NLA_FRIENDLY_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLA_INTERNET(pub i32);
pub const NLA_INTERNET_UNKNOWN: NLA_INTERNET = NLA_INTERNET(0i32);
pub const NLA_INTERNET_NO: NLA_INTERNET = NLA_INTERNET(1i32);
pub const NLA_INTERNET_YES: NLA_INTERNET = NLA_INTERNET(2i32);
impl ::std::convert::From<i32> for NLA_INTERNET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLA_INTERNET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_ADDRESS_TYPE(pub i32);
pub const NlatUnspecified: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(0i32);
pub const NlatUnicast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(1i32);
pub const NlatAnycast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(2i32);
pub const NlatMulticast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(3i32);
pub const NlatBroadcast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(4i32);
pub const NlatInvalid: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(5i32);
impl ::std::convert::From<i32> for NL_ADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_ADDRESS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_BANDWIDTH_FLAG(pub i32);
pub const NlbwDisabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(0i32);
pub const NlbwEnabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(1i32);
pub const NlbwUnchanged: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(-1i32);
impl ::std::convert::From<i32> for NL_BANDWIDTH_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_BANDWIDTH_FLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NL_BANDWIDTH_INFORMATION {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl NL_BANDWIDTH_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NL_BANDWIDTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NL_BANDWIDTH_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NL_BANDWIDTH_INFORMATION").field("Bandwidth", &self.Bandwidth).field("Instability", &self.Instability).field("BandwidthPeaked", &self.BandwidthPeaked).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NL_BANDWIDTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth && self.Instability == other.Instability && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NL_BANDWIDTH_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NL_BANDWIDTH_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_DAD_STATE(pub i32);
pub const NldsInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const NldsTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const NldsDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const NldsDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const NldsPreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
pub const IpDadStateInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const IpDadStateTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const IpDadStateDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const IpDadStateDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const IpDadStatePreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
impl ::std::convert::From<i32> for NL_DAD_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_DAD_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_INTERFACE_NETWORK_CATEGORY_STATE(pub i32);
pub const NlincCategoryUnknown: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(0i32);
pub const NlincPublic: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(1i32);
pub const NlincPrivate: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(2i32);
pub const NlincDomainAuthenticated: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(3i32);
pub const NlincCategoryStateMax: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(4i32);
impl ::std::convert::From<i32> for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct NL_INTERFACE_OFFLOAD_ROD {
    pub _bitfield: u8,
}
impl NL_INTERFACE_OFFLOAD_ROD {}
impl ::std::default::Default for NL_INTERFACE_OFFLOAD_ROD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NL_INTERFACE_OFFLOAD_ROD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NL_INTERFACE_OFFLOAD_ROD").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NL_INTERFACE_OFFLOAD_ROD {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NL_INTERFACE_OFFLOAD_ROD {}
unsafe impl ::windows::runtime::Abi for NL_INTERFACE_OFFLOAD_ROD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_LINK_LOCAL_ADDRESS_BEHAVIOR(pub i32);
pub const LinkLocalAlwaysOff: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(0i32);
pub const LinkLocalDelayed: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(1i32);
pub const LinkLocalAlwaysOn: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(2i32);
pub const LinkLocalUnchanged: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(-1i32);
impl ::std::convert::From<i32> for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_NEIGHBOR_STATE(pub i32);
pub const NlnsUnreachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(0i32);
pub const NlnsIncomplete: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(1i32);
pub const NlnsProbe: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(2i32);
pub const NlnsDelay: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(3i32);
pub const NlnsStale: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(4i32);
pub const NlnsReachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(5i32);
pub const NlnsPermanent: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(6i32);
pub const NlnsMaximum: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(7i32);
impl ::std::convert::From<i32> for NL_NEIGHBOR_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_NEIGHBOR_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_NETWORK_CATEGORY(pub i32);
pub const NetworkCategoryPublic: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(0i32);
pub const NetworkCategoryPrivate: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(1i32);
pub const NetworkCategoryDomainAuthenticated: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(2i32);
pub const NetworkCategoryUnchanged: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
pub const NetworkCategoryUnknown: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
impl ::std::convert::From<i32> for NL_NETWORK_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_NETWORK_CATEGORY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_NETWORK_CONNECTIVITY_COST_HINT(pub i32);
pub const NetworkConnectivityCostHintUnknown: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(0i32);
pub const NetworkConnectivityCostHintUnrestricted: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(1i32);
pub const NetworkConnectivityCostHintFixed: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(2i32);
pub const NetworkConnectivityCostHintVariable: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(3i32);
impl ::std::convert::From<i32> for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_NETWORK_CONNECTIVITY_COST_HINT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NL_NETWORK_CONNECTIVITY_HINT {
    pub ConnectivityLevel: NL_NETWORK_CONNECTIVITY_LEVEL_HINT,
    pub ConnectivityCost: NL_NETWORK_CONNECTIVITY_COST_HINT,
    pub ApproachingDataLimit: super::super::Foundation::BOOLEAN,
    pub OverDataLimit: super::super::Foundation::BOOLEAN,
    pub Roaming: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl NL_NETWORK_CONNECTIVITY_HINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NL_NETWORK_CONNECTIVITY_HINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NL_NETWORK_CONNECTIVITY_HINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NL_NETWORK_CONNECTIVITY_HINT").field("ConnectivityLevel", &self.ConnectivityLevel).field("ConnectivityCost", &self.ConnectivityCost).field("ApproachingDataLimit", &self.ApproachingDataLimit).field("OverDataLimit", &self.OverDataLimit).field("Roaming", &self.Roaming).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NL_NETWORK_CONNECTIVITY_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectivityLevel == other.ConnectivityLevel && self.ConnectivityCost == other.ConnectivityCost && self.ApproachingDataLimit == other.ApproachingDataLimit && self.OverDataLimit == other.OverDataLimit && self.Roaming == other.Roaming
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NL_NETWORK_CONNECTIVITY_HINT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NL_NETWORK_CONNECTIVITY_HINT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_NETWORK_CONNECTIVITY_LEVEL_HINT(pub i32);
pub const NetworkConnectivityLevelHintUnknown: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(0i32);
pub const NetworkConnectivityLevelHintNone: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(1i32);
pub const NetworkConnectivityLevelHintLocalAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(2i32);
pub const NetworkConnectivityLevelHintInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(3i32);
pub const NetworkConnectivityLevelHintConstrainedInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(4i32);
pub const NetworkConnectivityLevelHintHidden: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(5i32);
impl ::std::convert::From<i32> for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NL_PATH_BANDWIDTH_ROD {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl NL_PATH_BANDWIDTH_ROD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NL_PATH_BANDWIDTH_ROD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NL_PATH_BANDWIDTH_ROD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NL_PATH_BANDWIDTH_ROD").field("Bandwidth", &self.Bandwidth).field("Instability", &self.Instability).field("BandwidthPeaked", &self.BandwidthPeaked).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NL_PATH_BANDWIDTH_ROD {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth && self.Instability == other.Instability && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NL_PATH_BANDWIDTH_ROD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NL_PATH_BANDWIDTH_ROD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_PREFIX_ORIGIN(pub i32);
pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(0i32);
pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(1i32);
pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(2i32);
pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(3i32);
pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(4i32);
pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(16i32);
impl ::std::convert::From<i32> for NL_PREFIX_ORIGIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_PREFIX_ORIGIN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_ROUTER_DISCOVERY_BEHAVIOR(pub i32);
pub const RouterDiscoveryDisabled: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(0i32);
pub const RouterDiscoveryEnabled: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(1i32);
pub const RouterDiscoveryDhcp: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(2i32);
pub const RouterDiscoveryUnchanged: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(-1i32);
impl ::std::convert::From<i32> for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_ROUTER_DISCOVERY_BEHAVIOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_ROUTE_ORIGIN(pub i32);
pub const NlroManual: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(0i32);
pub const NlroWellKnown: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(1i32);
pub const NlroDHCP: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(2i32);
pub const NlroRouterAdvertisement: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(3i32);
pub const Nlro6to4: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(4i32);
impl ::std::convert::From<i32> for NL_ROUTE_ORIGIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_ROUTE_ORIGIN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_ROUTE_PROTOCOL(pub i32);
pub const RouteProtocolOther: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const RouteProtocolLocal: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const RouteProtocolNetMgmt: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const RouteProtocolIcmp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const RouteProtocolEgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const RouteProtocolGgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const RouteProtocolHello: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const RouteProtocolRip: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const RouteProtocolIsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const RouteProtocolEsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const RouteProtocolCisco: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const RouteProtocolBbn: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const RouteProtocolOspf: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const RouteProtocolBgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const RouteProtocolIdpr: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const RouteProtocolEigrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const RouteProtocolDvmrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const RouteProtocolRpl: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const RouteProtocolDhcp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const PROTO_IP_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const MIB_IPPROTO_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const PROTO_IP_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const MIB_IPPROTO_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const PROTO_IP_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const MIB_IPPROTO_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const PROTO_IP_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const MIB_IPPROTO_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const PROTO_IP_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const MIB_IPPROTO_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const PROTO_IP_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const MIB_IPPROTO_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const PROTO_IP_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const MIB_IPPROTO_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const PROTO_IP_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const MIB_IPPROTO_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const PROTO_IP_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const MIB_IPPROTO_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const PROTO_IP_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const MIB_IPPROTO_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const PROTO_IP_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const MIB_IPPROTO_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const PROTO_IP_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const MIB_IPPROTO_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const PROTO_IP_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const MIB_IPPROTO_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const PROTO_IP_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const MIB_IPPROTO_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const PROTO_IP_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const MIB_IPPROTO_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const PROTO_IP_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const MIB_IPPROTO_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const PROTO_IP_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const MIB_IPPROTO_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const PROTO_IP_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const MIB_IPPROTO_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const PROTO_IP_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const PROTO_IP_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const MIB_IPPROTO_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const PROTO_IP_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const MIB_IPPROTO_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
pub const PROTO_IP_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
impl ::std::convert::From<i32> for NL_ROUTE_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_ROUTE_PROTOCOL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NL_SUFFIX_ORIGIN(pub i32);
pub const NlsoOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const NlsoManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const NlsoDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const NlsoRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(16i32);
impl ::std::convert::From<i32> for NL_SUFFIX_ORIGIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NL_SUFFIX_ORIGIN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSPROTO_IPX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSPROTO_SPX: u32 = 1256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSPROTO_SPXII: u32 = 1257u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct NSPV2_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPv2Startup: ::std::option::Option<LPNSPV2STARTUP>,
    pub NSPv2Cleanup: ::std::option::Option<LPNSPV2CLEANUP>,
    pub NSPv2LookupServiceBegin: ::std::option::Option<LPNSPV2LOOKUPSERVICEBEGIN>,
    pub NSPv2LookupServiceNextEx: ::std::option::Option<LPNSPV2LOOKUPSERVICENEXTEX>,
    pub NSPv2LookupServiceEnd: ::std::option::Option<LPNSPV2LOOKUPSERVICEEND>,
    pub NSPv2SetServiceEx: ::std::option::Option<LPNSPV2SETSERVICEEX>,
    pub NSPv2ClientSessionRundown: ::std::option::Option<LPNSPV2CLIENTSESSIONRUNDOWN>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl NSPV2_ROUTINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for NSPV2_ROUTINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for NSPV2_ROUTINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NSPV2_ROUTINE").field("cbSize", &self.cbSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for NSPV2_ROUTINE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.NSPv2Startup.map(|f| f as usize) == other.NSPv2Startup.map(|f| f as usize)
            && self.NSPv2Cleanup.map(|f| f as usize) == other.NSPv2Cleanup.map(|f| f as usize)
            && self.NSPv2LookupServiceBegin.map(|f| f as usize) == other.NSPv2LookupServiceBegin.map(|f| f as usize)
            && self.NSPv2LookupServiceNextEx.map(|f| f as usize) == other.NSPv2LookupServiceNextEx.map(|f| f as usize)
            && self.NSPv2LookupServiceEnd.map(|f| f as usize) == other.NSPv2LookupServiceEnd.map(|f| f as usize)
            && self.NSPv2SetServiceEx.map(|f| f as usize) == other.NSPv2SetServiceEx.map(|f| f as usize)
            && self.NSPv2ClientSessionRundown.map(|f| f as usize) == other.NSPv2ClientSessionRundown.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for NSPV2_ROUTINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for NSPV2_ROUTINE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
pub struct NSP_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPCleanup: ::std::option::Option<LPNSPCLEANUP>,
    pub NSPLookupServiceBegin: ::std::option::Option<LPNSPLOOKUPSERVICEBEGIN>,
    pub NSPLookupServiceNext: ::std::option::Option<LPNSPLOOKUPSERVICENEXT>,
    pub NSPLookupServiceEnd: ::std::option::Option<LPNSPLOOKUPSERVICEEND>,
    pub NSPSetService: ::std::option::Option<LPNSPSETSERVICE>,
    pub NSPInstallServiceClass: ::std::option::Option<LPNSPINSTALLSERVICECLASS>,
    pub NSPRemoveServiceClass: ::std::option::Option<LPNSPREMOVESERVICECLASS>,
    pub NSPGetServiceClassInfo: ::std::option::Option<LPNSPGETSERVICECLASSINFO>,
    pub NSPIoctl: ::std::option::Option<LPNSPIOCTL>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl NSP_ROUTINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl ::std::default::Default for NSP_ROUTINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for NSP_ROUTINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NSP_ROUTINE").field("cbSize", &self.cbSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for NSP_ROUTINE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.NSPCleanup.map(|f| f as usize) == other.NSPCleanup.map(|f| f as usize)
            && self.NSPLookupServiceBegin.map(|f| f as usize) == other.NSPLookupServiceBegin.map(|f| f as usize)
            && self.NSPLookupServiceNext.map(|f| f as usize) == other.NSPLookupServiceNext.map(|f| f as usize)
            && self.NSPLookupServiceEnd.map(|f| f as usize) == other.NSPLookupServiceEnd.map(|f| f as usize)
            && self.NSPSetService.map(|f| f as usize) == other.NSPSetService.map(|f| f as usize)
            && self.NSPInstallServiceClass.map(|f| f as usize) == other.NSPInstallServiceClass.map(|f| f as usize)
            && self.NSPRemoveServiceClass.map(|f| f as usize) == other.NSPRemoveServiceClass.map(|f| f as usize)
            && self.NSPGetServiceClassInfo.map(|f| f as usize) == other.NSPGetServiceClassInfo.map(|f| f as usize)
            && self.NSPIoctl.map(|f| f as usize) == other.NSPIoctl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for NSP_ROUTINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for NSP_ROUTINE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSTYPE_DYNAMIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSTYPE_ENUMERABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSTYPE_HIERARCHICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NSTYPE_WORKGROUP: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_ALL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_DHCP: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_DNS: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_EMAIL: u32 = 37u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NS_INFOA {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NS_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NS_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NS_INFOA").field("dwNameSpace", &self.dwNameSpace).field("dwNameSpaceFlags", &self.dwNameSpaceFlags).field("lpNameSpace", &self.lpNameSpace).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwNameSpaceFlags == other.dwNameSpaceFlags && self.lpNameSpace == other.lpNameSpace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NS_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct NS_INFOW {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NS_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NS_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NS_INFOW").field("dwNameSpace", &self.dwNameSpace).field("dwNameSpaceFlags", &self.dwNameSpaceFlags).field("lpNameSpace", &self.lpNameSpace).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwNameSpaceFlags == other.dwNameSpaceFlags && self.lpNameSpace == other.lpNameSpace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NS_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_LOCALNAME: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_MS: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NBP: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NDS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NETBT: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NETDES: u32 = 60u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NIS: u32 = 41u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NISPLUS: u32 = 42u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NLA: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_NTDS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_PEER_BROWSE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_SAP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct NS_SERVICE_INFOA {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl NS_SERVICE_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for NS_SERVICE_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for NS_SERVICE_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NS_SERVICE_INFOA").field("dwNameSpace", &self.dwNameSpace).field("ServiceInfo", &self.ServiceInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for NS_SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for NS_SERVICE_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for NS_SERVICE_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct NS_SERVICE_INFOW {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl NS_SERVICE_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for NS_SERVICE_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for NS_SERVICE_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NS_SERVICE_INFOW").field("dwNameSpace", &self.dwNameSpace).field("ServiceInfo", &self.ServiceInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for NS_SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for NS_SERVICE_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for NS_SERVICE_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_SLP: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_STDA: u32 = 31u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_TCPIP_HOSTS: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_TCPIP_LOCAL: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_VNS: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_WINS: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_WRQ: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const NS_X500: u32 = 40u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PFL_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PFL_MATCHES_PROTOCOL_ZERO: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PFL_MULTIPLE_PROTO_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PFL_NETWORKDIRECT_PROVIDER: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PFL_RECOMMENDED_PROTO_ENTRY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_APPLETALK: u16 = 16u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_ATM: u16 = 22u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_BAN: u16 = 21u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_CCITT: u16 = 10u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_CHAOS: u16 = 5u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_DATAKIT: u16 = 9u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_DECnet: u16 = 12u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_DLI: u16 = 13u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_ECMA: u16 = 8u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_FIREFOX: u16 = 19u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_HYLINK: u16 = 15u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_IMPLINK: u16 = 3u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_IPX: u16 = 6u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_IRDA: u16 = 26u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_ISO: u16 = 7u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_LAT: u16 = 14u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_MAX: u16 = 29u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_NS: u16 = 6u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_OSI: u16 = 7u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_PUP: u16 = 4u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_SNA: u16 = 11u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_UNIX: u16 = 1u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_UNKNOWN1: u16 = 20u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PF_VOICEVIEW: u16 = 18u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PI_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PI_NUMBER_NOT_AVAILABLE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PI_RESTRICTED: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PMTUD_STATE(pub i32);
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = PMTUD_STATE(0i32);
pub const IP_PMTUDISC_DO: PMTUD_STATE = PMTUD_STATE(1i32);
pub const IP_PMTUDISC_DONT: PMTUD_STATE = PMTUD_STATE(2i32);
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = PMTUD_STATE(3i32);
pub const IP_PMTUDISC_MAX: PMTUD_STATE = PMTUD_STATE(4i32);
impl ::std::convert::From<i32> for PMTUD_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PMTUD_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLERR: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLHUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLNVAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLOUT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLPRI: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLRDBAND: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLRDNORM: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLWRBAND: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const POLLWRNORM: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct PRIORITY_STATUS {
    pub Sender: SOCKET_PRIORITY_HINT,
    pub Receiver: SOCKET_PRIORITY_HINT,
}
impl PRIORITY_STATUS {}
impl ::std::default::Default for PRIORITY_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRIORITY_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRIORITY_STATUS").field("Sender", &self.Sender).field("Receiver", &self.Receiver).finish()
    }
}
impl ::std::cmp::PartialEq for PRIORITY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Sender == other.Sender && self.Receiver == other.Receiver
    }
}
impl ::std::cmp::Eq for PRIORITY_STATUS {}
unsafe impl ::windows::runtime::Abi for PRIORITY_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_ADDRESSES: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_ALL: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_COMMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_DISPLAY_HINT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_LOCALE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_MACHINE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_SD: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_START_TIME: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROP_VERSION: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROTECTION_LEVEL_DEFAULT: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct PROTOCOL_INFOA {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROTOCOL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROTOCOL_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROTOCOL_INFOA")
            .field("dwServiceFlags", &self.dwServiceFlags)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("lpProtocol", &self.lpProtocol)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags && self.iAddressFamily == other.iAddressFamily && self.iMaxSockAddr == other.iMaxSockAddr && self.iMinSockAddr == other.iMinSockAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol && self.dwMessageSize == other.dwMessageSize && self.lpProtocol == other.lpProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROTOCOL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROTOCOL_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct PROTOCOL_INFOW {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROTOCOL_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROTOCOL_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROTOCOL_INFOW")
            .field("dwServiceFlags", &self.dwServiceFlags)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("lpProtocol", &self.lpProtocol)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags && self.iAddressFamily == other.iAddressFamily && self.iMaxSockAddr == other.iMaxSockAddr && self.iMinSockAddr == other.iMinSockAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol && self.dwMessageSize == other.dwMessageSize && self.lpProtocol == other.lpProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROTOCOL_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROTOCOL_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const PVD_CONFIG: u32 = 12289u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn ProcessSocketNotifications<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(completionport: Param0, registrationcount: u32, registrationinfos: *mut SOCK_NOTIFY_REGISTRATION, timeoutms: u32, completioncount: u32, completionportentries: *mut super::super::System::IO::OVERLAPPED_ENTRY, receivedentrycount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessSocketNotifications(completionport: super::super::Foundation::HANDLE, registrationcount: u32, registrationinfos: *mut SOCK_NOTIFY_REGISTRATION, timeoutms: u32, completioncount: u32, completionportentries: *mut super::super::System::IO::OVERLAPPED_ENTRY, receivedentrycount: *mut u32) -> u32;
        }
        ::std::mem::transmute(ProcessSocketNotifications(
            completionport.into_param().abi(),
            ::std::mem::transmute(registrationcount),
            ::std::mem::transmute(registrationinfos),
            ::std::mem::transmute(timeoutms),
            ::std::mem::transmute(completioncount),
            ::std::mem::transmute(completionportentries),
            ::std::mem::transmute(receivedentrycount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct Q2931_IE {
    pub IEType: Q2931_IE_TYPE,
    pub IELength: u32,
    pub IE: [u8; 1],
}
impl Q2931_IE {}
impl ::std::default::Default for Q2931_IE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Q2931_IE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Q2931_IE").field("IEType", &self.IEType).field("IELength", &self.IELength).field("IE", &self.IE).finish()
    }
}
impl ::std::cmp::PartialEq for Q2931_IE {
    fn eq(&self, other: &Self) -> bool {
        self.IEType == other.IEType && self.IELength == other.IELength && self.IE == other.IE
    }
}
impl ::std::cmp::Eq for Q2931_IE {}
unsafe impl ::windows::runtime::Abi for Q2931_IE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Q2931_IE_TYPE(pub i32);
pub const IE_AALParameters: Q2931_IE_TYPE = Q2931_IE_TYPE(0i32);
pub const IE_TrafficDescriptor: Q2931_IE_TYPE = Q2931_IE_TYPE(1i32);
pub const IE_BroadbandBearerCapability: Q2931_IE_TYPE = Q2931_IE_TYPE(2i32);
pub const IE_BHLI: Q2931_IE_TYPE = Q2931_IE_TYPE(3i32);
pub const IE_BLLI: Q2931_IE_TYPE = Q2931_IE_TYPE(4i32);
pub const IE_CalledPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(5i32);
pub const IE_CalledPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(6i32);
pub const IE_CallingPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(7i32);
pub const IE_CallingPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(8i32);
pub const IE_Cause: Q2931_IE_TYPE = Q2931_IE_TYPE(9i32);
pub const IE_QOSClass: Q2931_IE_TYPE = Q2931_IE_TYPE(10i32);
pub const IE_TransitNetworkSelection: Q2931_IE_TYPE = Q2931_IE_TYPE(11i32);
impl ::std::convert::From<i32> for Q2931_IE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Q2931_IE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const QOS_CLASS0: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const QOS_CLASS1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const QOS_CLASS2: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const QOS_CLASS3: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const QOS_CLASS4: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RCVALL_IF {
    pub Mode: RCVALL_VALUE,
    pub Interface: u32,
}
impl RCVALL_IF {}
impl ::std::default::Default for RCVALL_IF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RCVALL_IF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RCVALL_IF").field("Mode", &self.Mode).field("Interface", &self.Interface).finish()
    }
}
impl ::std::cmp::PartialEq for RCVALL_IF {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Interface == other.Interface
    }
}
impl ::std::cmp::Eq for RCVALL_IF {}
unsafe impl ::windows::runtime::Abi for RCVALL_IF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RCVALL_VALUE(pub i32);
pub const RCVALL_OFF: RCVALL_VALUE = RCVALL_VALUE(0i32);
pub const RCVALL_ON: RCVALL_VALUE = RCVALL_VALUE(1i32);
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = RCVALL_VALUE(2i32);
pub const RCVALL_IPLEVEL: RCVALL_VALUE = RCVALL_VALUE(3i32);
impl ::std::convert::From<i32> for RCVALL_VALUE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RCVALL_VALUE {
    type Abi = Self;
}
pub const REAL_TIME_NOTIFICATION_CAPABILITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1801027994, 23726, 18733, [169, 1, 42, 60, 44, 80, 22, 79]);
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1749277187, 5450, 17942, [165, 8, 68, 55, 18, 149, 249, 107]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: ::windows::runtime::GUID,
}
impl REAL_TIME_NOTIFICATION_SETTING_INPUT {}
impl ::std::default::Default for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventGuid", &self.BrokerEventGuid).finish()
    }
}
impl ::std::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventGuid == other.BrokerEventGuid
    }
}
impl ::std::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT {}
unsafe impl ::windows::runtime::Abi for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: ::windows::runtime::GUID,
    pub Unmark: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT_EX").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventGuid", &self.BrokerEventGuid).field("Unmark", &self.Unmark).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventGuid == other.BrokerEventGuid && self.Unmark == other.Unmark
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    pub ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}
impl REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
impl ::std::default::Default for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REAL_TIME_NOTIFICATION_SETTING_OUTPUT").field("ChannelStatus", &self.ChannelStatus).finish()
    }
}
impl ::std::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatus == other.ChannelStatus
    }
}
impl ::std::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
unsafe impl ::windows::runtime::Abi for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RESOURCE_DISPLAY_TYPE(pub u32);
pub const RESOURCEDISPLAYTYPE_DOMAIN: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(1u32);
pub const RESOURCEDISPLAYTYPE_FILE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(4u32);
pub const RESOURCEDISPLAYTYPE_GENERIC: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(0u32);
pub const RESOURCEDISPLAYTYPE_GROUP: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(5u32);
pub const RESOURCEDISPLAYTYPE_SERVER: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(2u32);
pub const RESOURCEDISPLAYTYPE_SHARE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(3u32);
pub const RESOURCEDISPLAYTYPE_TREE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(10u32);
impl ::std::convert::From<u32> for RESOURCE_DISPLAY_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESOURCE_DISPLAY_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for RESOURCE_DISPLAY_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RESOURCE_DISPLAY_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RESOURCE_DISPLAY_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RESOURCE_DISPLAY_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RESOURCE_DISPLAY_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RESULT_IS_ADDED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RESULT_IS_ALIAS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RESULT_IS_CHANGED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RESULT_IS_DELETED: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RES_FIND_MULTIPLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RES_FLUSH_CACHE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RES_SERVICE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RES_SOFT_SEARCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RES_UNUSED_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RIORESULT {
    pub Status: i32,
    pub BytesTransferred: u32,
    pub SocketContext: u64,
    pub RequestContext: u64,
}
impl RIORESULT {}
impl ::std::default::Default for RIORESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RIORESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RIORESULT").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).field("SocketContext", &self.SocketContext).field("RequestContext", &self.RequestContext).finish()
    }
}
impl ::std::cmp::PartialEq for RIORESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred && self.SocketContext == other.SocketContext && self.RequestContext == other.RequestContext
    }
}
impl ::std::cmp::Eq for RIORESULT {}
unsafe impl ::windows::runtime::Abi for RIORESULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RIO_BUF {
    pub BufferId: *mut RIO_BUFFERID_t,
    pub Offset: u32,
    pub Length: u32,
}
impl RIO_BUF {}
impl ::std::default::Default for RIO_BUF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RIO_BUF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RIO_BUF").field("BufferId", &self.BufferId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for RIO_BUF {
    fn eq(&self, other: &Self) -> bool {
        self.BufferId == other.BufferId && self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for RIO_BUF {}
unsafe impl ::windows::runtime::Abi for RIO_BUF {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct RIO_BUFFERID_t(pub u8);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
impl RIO_CMSG_BUFFER {}
impl ::std::default::Default for RIO_CMSG_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RIO_CMSG_BUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RIO_CMSG_BUFFER").field("TotalLength", &self.TotalLength).finish()
    }
}
impl ::std::cmp::PartialEq for RIO_CMSG_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.TotalLength == other.TotalLength
    }
}
impl ::std::cmp::Eq for RIO_CMSG_BUFFER {}
unsafe impl ::windows::runtime::Abi for RIO_CMSG_BUFFER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_CORRUPT_CQ: u32 = 4294967295u32;
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct RIO_CQ_t(pub u8);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RIO_EXTENSION_FUNCTION_TABLE {
    pub cbSize: u32,
    pub RIOReceive: ::std::option::Option<LPFN_RIORECEIVE>,
    pub RIOReceiveEx: ::std::option::Option<LPFN_RIORECEIVEEX>,
    pub RIOSend: ::std::option::Option<LPFN_RIOSEND>,
    pub RIOSendEx: ::std::option::Option<LPFN_RIOSENDEX>,
    pub RIOCloseCompletionQueue: ::std::option::Option<LPFN_RIOCLOSECOMPLETIONQUEUE>,
    pub RIOCreateCompletionQueue: ::std::option::Option<LPFN_RIOCREATECOMPLETIONQUEUE>,
    pub RIOCreateRequestQueue: ::std::option::Option<LPFN_RIOCREATEREQUESTQUEUE>,
    pub RIODequeueCompletion: ::std::option::Option<LPFN_RIODEQUEUECOMPLETION>,
    pub RIODeregisterBuffer: ::std::option::Option<LPFN_RIODEREGISTERBUFFER>,
    pub RIONotify: ::std::option::Option<LPFN_RIONOTIFY>,
    pub RIORegisterBuffer: ::std::option::Option<LPFN_RIOREGISTERBUFFER>,
    pub RIOResizeCompletionQueue: ::std::option::Option<LPFN_RIORESIZECOMPLETIONQUEUE>,
    pub RIOResizeRequestQueue: ::std::option::Option<LPFN_RIORESIZEREQUESTQUEUE>,
}
#[cfg(feature = "Win32_Foundation")]
impl RIO_EXTENSION_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RIO_EXTENSION_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RIO_EXTENSION_FUNCTION_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RIO_EXTENSION_FUNCTION_TABLE").field("cbSize", &self.cbSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RIO_EXTENSION_FUNCTION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.RIOReceive.map(|f| f as usize) == other.RIOReceive.map(|f| f as usize)
            && self.RIOReceiveEx.map(|f| f as usize) == other.RIOReceiveEx.map(|f| f as usize)
            && self.RIOSend.map(|f| f as usize) == other.RIOSend.map(|f| f as usize)
            && self.RIOSendEx.map(|f| f as usize) == other.RIOSendEx.map(|f| f as usize)
            && self.RIOCloseCompletionQueue.map(|f| f as usize) == other.RIOCloseCompletionQueue.map(|f| f as usize)
            && self.RIOCreateCompletionQueue.map(|f| f as usize) == other.RIOCreateCompletionQueue.map(|f| f as usize)
            && self.RIOCreateRequestQueue.map(|f| f as usize) == other.RIOCreateRequestQueue.map(|f| f as usize)
            && self.RIODequeueCompletion.map(|f| f as usize) == other.RIODequeueCompletion.map(|f| f as usize)
            && self.RIODeregisterBuffer.map(|f| f as usize) == other.RIODeregisterBuffer.map(|f| f as usize)
            && self.RIONotify.map(|f| f as usize) == other.RIONotify.map(|f| f as usize)
            && self.RIORegisterBuffer.map(|f| f as usize) == other.RIORegisterBuffer.map(|f| f as usize)
            && self.RIOResizeCompletionQueue.map(|f| f as usize) == other.RIOResizeCompletionQueue.map(|f| f as usize)
            && self.RIOResizeRequestQueue.map(|f| f as usize) == other.RIOResizeRequestQueue.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RIO_EXTENSION_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RIO_EXTENSION_FUNCTION_TABLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_MAX_CQ_SIZE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_MSG_COMMIT_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_MSG_DEFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_MSG_DONT_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RIO_MSG_WAITALL: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RIO_NOTIFICATION_COMPLETION {
    pub Type: RIO_NOTIFICATION_COMPLETION_TYPE,
    pub Anonymous: RIO_NOTIFICATION_COMPLETION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RIO_NOTIFICATION_COMPLETION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RIO_NOTIFICATION_COMPLETION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RIO_NOTIFICATION_COMPLETION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RIO_NOTIFICATION_COMPLETION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union RIO_NOTIFICATION_COMPLETION_0 {
    pub Event: RIO_NOTIFICATION_COMPLETION_0_0,
    pub Iocp: RIO_NOTIFICATION_COMPLETION_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl RIO_NOTIFICATION_COMPLETION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RIO_NOTIFICATION_COMPLETION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RIO_NOTIFICATION_COMPLETION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RIO_NOTIFICATION_COMPLETION_0_0 {
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyReset: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl RIO_NOTIFICATION_COMPLETION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Event_e__Struct").field("EventHandle", &self.EventHandle).field("NotifyReset", &self.NotifyReset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.EventHandle == other.EventHandle && self.NotifyReset == other.NotifyReset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RIO_NOTIFICATION_COMPLETION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RIO_NOTIFICATION_COMPLETION_0_1 {
    pub IocpHandle: super::super::Foundation::HANDLE,
    pub CompletionKey: *mut ::std::ffi::c_void,
    pub Overlapped: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl RIO_NOTIFICATION_COMPLETION_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iocp_e__Struct").field("IocpHandle", &self.IocpHandle).field("CompletionKey", &self.CompletionKey).field("Overlapped", &self.Overlapped).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.IocpHandle == other.IocpHandle && self.CompletionKey == other.CompletionKey && self.Overlapped == other.Overlapped
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RIO_NOTIFICATION_COMPLETION_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RIO_NOTIFICATION_COMPLETION_TYPE(pub i32);
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = RIO_NOTIFICATION_COMPLETION_TYPE(1i32);
pub const RIO_IOCP_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = RIO_NOTIFICATION_COMPLETION_TYPE(2i32);
impl ::std::convert::From<i32> for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RIO_NOTIFICATION_COMPLETION_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct RIO_RQ_t(pub u8);
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_ADD_RECEIVE_IF: u32 = 1008u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_DEL_RECEIVE_IF: u32 = 1009u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RM_FEC_INFO {
    pub FECBlockSize: u16,
    pub FECProActivePackets: u16,
    pub FECGroupSize: u8,
    pub fFECOnDemandParityEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl RM_FEC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RM_FEC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RM_FEC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_FEC_INFO").field("FECBlockSize", &self.FECBlockSize).field("FECProActivePackets", &self.FECProActivePackets).field("FECGroupSize", &self.FECGroupSize).field("fFECOnDemandParityEnabled", &self.fFECOnDemandParityEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RM_FEC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FECBlockSize == other.FECBlockSize && self.FECProActivePackets == other.FECProActivePackets && self.FECGroupSize == other.FECGroupSize && self.fFECOnDemandParityEnabled == other.fFECOnDemandParityEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RM_FEC_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RM_FEC_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_FLUSHCACHE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_HIGH_SPEED_INTRANET_OPT: u32 = 1014u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_LATEJOIN: u32 = 1006u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_OPTIONSBASE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_RATE_WINDOW_SIZE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_RECEIVER_STATISTICS: u32 = 1013u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RM_RECEIVER_STATS {
    pub NumODataPacketsReceived: u64,
    pub NumRDataPacketsReceived: u64,
    pub NumDuplicateDataPackets: u64,
    pub DataBytesReceived: u64,
    pub TotalBytesReceived: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub AverageSequencesInWindow: u64,
    pub MinSequencesInWindow: u64,
    pub MaxSequencesInWindow: u64,
    pub FirstNakSequenceNumber: u64,
    pub NumPendingNaks: u64,
    pub NumOutstandingNaks: u64,
    pub NumDataPacketsBuffered: u64,
    pub TotalSelectiveNaksSent: u64,
    pub TotalParityNaksSent: u64,
}
impl RM_RECEIVER_STATS {}
impl ::std::default::Default for RM_RECEIVER_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RM_RECEIVER_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_RECEIVER_STATS")
            .field("NumODataPacketsReceived", &self.NumODataPacketsReceived)
            .field("NumRDataPacketsReceived", &self.NumRDataPacketsReceived)
            .field("NumDuplicateDataPackets", &self.NumDuplicateDataPackets)
            .field("DataBytesReceived", &self.DataBytesReceived)
            .field("TotalBytesReceived", &self.TotalBytesReceived)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("AverageSequencesInWindow", &self.AverageSequencesInWindow)
            .field("MinSequencesInWindow", &self.MinSequencesInWindow)
            .field("MaxSequencesInWindow", &self.MaxSequencesInWindow)
            .field("FirstNakSequenceNumber", &self.FirstNakSequenceNumber)
            .field("NumPendingNaks", &self.NumPendingNaks)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumDataPacketsBuffered", &self.NumDataPacketsBuffered)
            .field("TotalSelectiveNaksSent", &self.TotalSelectiveNaksSent)
            .field("TotalParityNaksSent", &self.TotalParityNaksSent)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RM_RECEIVER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumODataPacketsReceived == other.NumODataPacketsReceived
            && self.NumRDataPacketsReceived == other.NumRDataPacketsReceived
            && self.NumDuplicateDataPackets == other.NumDuplicateDataPackets
            && self.DataBytesReceived == other.DataBytesReceived
            && self.TotalBytesReceived == other.TotalBytesReceived
            && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall
            && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast
            && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId
            && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId
            && self.AverageSequencesInWindow == other.AverageSequencesInWindow
            && self.MinSequencesInWindow == other.MinSequencesInWindow
            && self.MaxSequencesInWindow == other.MaxSequencesInWindow
            && self.FirstNakSequenceNumber == other.FirstNakSequenceNumber
            && self.NumPendingNaks == other.NumPendingNaks
            && self.NumOutstandingNaks == other.NumOutstandingNaks
            && self.NumDataPacketsBuffered == other.NumDataPacketsBuffered
            && self.TotalSelectiveNaksSent == other.TotalSelectiveNaksSent
            && self.TotalParityNaksSent == other.TotalParityNaksSent
    }
}
impl ::std::cmp::Eq for RM_RECEIVER_STATS {}
unsafe impl ::windows::runtime::Abi for RM_RECEIVER_STATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SENDER_STATISTICS: u32 = 1005u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RM_SENDER_STATS {
    pub DataBytesSent: u64,
    pub TotalBytesSent: u64,
    pub NaksReceived: u64,
    pub NaksReceivedTooLate: u64,
    pub NumOutstandingNaks: u64,
    pub NumNaksAfterRData: u64,
    pub RepairPacketsSent: u64,
    pub BufferSpaceAvailable: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TotalODataPacketsSent: u64,
}
impl RM_SENDER_STATS {}
impl ::std::default::Default for RM_SENDER_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RM_SENDER_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_SENDER_STATS")
            .field("DataBytesSent", &self.DataBytesSent)
            .field("TotalBytesSent", &self.TotalBytesSent)
            .field("NaksReceived", &self.NaksReceived)
            .field("NaksReceivedTooLate", &self.NaksReceivedTooLate)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumNaksAfterRData", &self.NumNaksAfterRData)
            .field("RepairPacketsSent", &self.RepairPacketsSent)
            .field("BufferSpaceAvailable", &self.BufferSpaceAvailable)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TotalODataPacketsSent", &self.TotalODataPacketsSent)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RM_SENDER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytesSent == other.DataBytesSent
            && self.TotalBytesSent == other.TotalBytesSent
            && self.NaksReceived == other.NaksReceived
            && self.NaksReceivedTooLate == other.NaksReceivedTooLate
            && self.NumOutstandingNaks == other.NumOutstandingNaks
            && self.NumNaksAfterRData == other.NumNaksAfterRData
            && self.RepairPacketsSent == other.RepairPacketsSent
            && self.BufferSpaceAvailable == other.BufferSpaceAvailable
            && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId
            && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId
            && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall
            && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast
            && self.TotalODataPacketsSent == other.TotalODataPacketsSent
    }
}
impl ::std::cmp::Eq for RM_SENDER_STATS {}
unsafe impl ::windows::runtime::Abi for RM_SENDER_STATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: u32 = 1004u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct RM_SEND_WINDOW {
    pub RateKbitsPerSec: u32,
    pub WindowSizeInMSecs: u32,
    pub WindowSizeInBytes: u32,
}
impl RM_SEND_WINDOW {}
impl ::std::default::Default for RM_SEND_WINDOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RM_SEND_WINDOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_SEND_WINDOW").field("RateKbitsPerSec", &self.RateKbitsPerSec).field("WindowSizeInMSecs", &self.WindowSizeInMSecs).field("WindowSizeInBytes", &self.WindowSizeInBytes).finish()
    }
}
impl ::std::cmp::PartialEq for RM_SEND_WINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.RateKbitsPerSec == other.RateKbitsPerSec && self.WindowSizeInMSecs == other.WindowSizeInMSecs && self.WindowSizeInBytes == other.WindowSizeInBytes
    }
}
impl ::std::cmp::Eq for RM_SEND_WINDOW {}
unsafe impl ::windows::runtime::Abi for RM_SEND_WINDOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SEND_WINDOW_ADV_RATE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SET_MCAST_TTL: u32 = 1012u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SET_MESSAGE_BOUNDARY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_SET_SEND_IF: u32 = 1007u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const RM_USE_FEC: u32 = 1011u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct RSS_SCALABILITY_INFO {
    pub RssEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl RSS_SCALABILITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RSS_SCALABILITY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RSS_SCALABILITY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RSS_SCALABILITY_INFO").field("RssEnabled", &self.RssEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RSS_SCALABILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RssEnabled == other.RssEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RSS_SCALABILITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RSS_SCALABILITY_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
#[inline]
pub unsafe fn RtlEthernetAddressToStringA(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlEthernetAddressToStringA(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(RtlEthernetAddressToStringA(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
#[inline]
pub unsafe fn RtlEthernetAddressToStringW(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlEthernetAddressToStringW(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(RtlEthernetAddressToStringW(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
#[inline]
pub unsafe fn RtlEthernetStringToAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(s: Param0, terminator: *mut super::super::Foundation::PSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlEthernetStringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
        }
        ::std::mem::transmute(RtlEthernetStringToAddressA(s.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
#[inline]
pub unsafe fn RtlEthernetStringToAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(s: Param0, terminator: *mut super::super::Foundation::PWSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlEthernetStringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
        }
        ::std::mem::transmute(RtlEthernetStringToAddressW(s.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4AddressToStringA(addr: *const IN_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4AddressToStringA(addr: *const IN_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(RtlIpv4AddressToStringA(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4AddressToStringExA(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4AddressToStringExA(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(RtlIpv4AddressToStringExA(::std::mem::transmute(address), ::std::mem::transmute(port), ::std::mem::transmute(addressstring), ::std::mem::transmute(addressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4AddressToStringExW(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4AddressToStringExW(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(RtlIpv4AddressToStringExW(::std::mem::transmute(address), ::std::mem::transmute(port), ::std::mem::transmute(addressstring), ::std::mem::transmute(addressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4AddressToStringW(addr: *const IN_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4AddressToStringW(addr: *const IN_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(RtlIpv4AddressToStringW(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4StringToAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(s: Param0, strict: Param1, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN_ADDR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4StringToAddressA(s: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN_ADDR) -> i32;
        }
        ::std::mem::transmute(RtlIpv4StringToAddressA(s.into_param().abi(), strict.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4StringToAddressExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(addressstring: Param0, strict: Param1, address: *mut IN_ADDR, port: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4StringToAddressExA(addressstring: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
        }
        ::std::mem::transmute(RtlIpv4StringToAddressExA(addressstring.into_param().abi(), strict.into_param().abi(), ::std::mem::transmute(address), ::std::mem::transmute(port)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4StringToAddressExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(addressstring: Param0, strict: Param1, address: *mut IN_ADDR, port: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4StringToAddressExW(addressstring: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
        }
        ::std::mem::transmute(RtlIpv4StringToAddressExW(addressstring.into_param().abi(), strict.into_param().abi(), ::std::mem::transmute(address), ::std::mem::transmute(port)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv4StringToAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(s: Param0, strict: Param1, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN_ADDR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv4StringToAddressW(s: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN_ADDR) -> i32;
        }
        ::std::mem::transmute(RtlIpv4StringToAddressW(s.into_param().abi(), strict.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6AddressToStringA(addr: *const IN6_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6AddressToStringA(addr: *const IN6_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(RtlIpv6AddressToStringA(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6AddressToStringExA(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6AddressToStringExA(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(RtlIpv6AddressToStringExA(::std::mem::transmute(address), ::std::mem::transmute(scopeid), ::std::mem::transmute(port), ::std::mem::transmute(addressstring), ::std::mem::transmute(addressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6AddressToStringExW(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6AddressToStringExW(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(RtlIpv6AddressToStringExW(::std::mem::transmute(address), ::std::mem::transmute(scopeid), ::std::mem::transmute(port), ::std::mem::transmute(addressstring), ::std::mem::transmute(addressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6AddressToStringW(addr: *const IN6_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6AddressToStringW(addr: *const IN6_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(RtlIpv6AddressToStringW(::std::mem::transmute(addr), ::std::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6StringToAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(s: Param0, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN6_ADDR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6StringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN6_ADDR) -> i32;
        }
        ::std::mem::transmute(RtlIpv6StringToAddressA(s.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6StringToAddressExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(addressstring: Param0, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6StringToAddressExA(addressstring: super::super::Foundation::PSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
        }
        ::std::mem::transmute(RtlIpv6StringToAddressExA(addressstring.into_param().abi(), ::std::mem::transmute(address), ::std::mem::transmute(scopeid), ::std::mem::transmute(port)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6StringToAddressExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(addressstring: Param0, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6StringToAddressExW(addressstring: super::super::Foundation::PWSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
        }
        ::std::mem::transmute(RtlIpv6StringToAddressExW(addressstring.into_param().abi(), ::std::mem::transmute(address), ::std::mem::transmute(scopeid), ::std::mem::transmute(port)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RtlIpv6StringToAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(s: Param0, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN6_ADDR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIpv6StringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN6_ADDR) -> i32;
        }
        ::std::mem::transmute(RtlIpv6StringToAddressW(s.into_param().abi(), ::std::mem::transmute(terminator), ::std::mem::transmute(addr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SAP_FIELD_ABSENT: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SAP_FIELD_ANY: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SAP_FIELD_ANY_AESA_REST: u32 = 4294967291u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SAP_FIELD_ANY_AESA_SEL: u32 = 4294967290u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SCOPE_ID {
    pub Anonymous: SCOPE_ID_0,
}
impl SCOPE_ID {}
impl ::std::default::Default for SCOPE_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SCOPE_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SCOPE_ID {}
unsafe impl ::windows::runtime::Abi for SCOPE_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union SCOPE_ID_0 {
    pub Anonymous: SCOPE_ID_0_0,
    pub Value: u32,
}
impl SCOPE_ID_0 {}
impl ::std::default::Default for SCOPE_ID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SCOPE_ID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SCOPE_ID_0 {}
unsafe impl ::windows::runtime::Abi for SCOPE_ID_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
impl SCOPE_ID_0_0 {}
impl ::std::default::Default for SCOPE_ID_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCOPE_ID_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for SCOPE_ID_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for SCOPE_ID_0_0 {}
unsafe impl ::windows::runtime::Abi for SCOPE_ID_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCOPE_LEVEL(pub i32);
pub const ScopeLevelInterface: SCOPE_LEVEL = SCOPE_LEVEL(1i32);
pub const ScopeLevelLink: SCOPE_LEVEL = SCOPE_LEVEL(2i32);
pub const ScopeLevelSubnet: SCOPE_LEVEL = SCOPE_LEVEL(3i32);
pub const ScopeLevelAdmin: SCOPE_LEVEL = SCOPE_LEVEL(4i32);
pub const ScopeLevelSite: SCOPE_LEVEL = SCOPE_LEVEL(5i32);
pub const ScopeLevelOrganization: SCOPE_LEVEL = SCOPE_LEVEL(8i32);
pub const ScopeLevelGlobal: SCOPE_LEVEL = SCOPE_LEVEL(14i32);
pub const ScopeLevelCount: SCOPE_LEVEL = SCOPE_LEVEL(16i32);
impl ::std::convert::From<i32> for SCOPE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCOPE_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SD_BOTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SD_RECEIVE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SD_SEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SECURITY_PROTOCOL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SENDER_DEFAULT_LATE_JOINER_PERCENTAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SENDER_DEFAULT_RATE_KBITS_PER_SEC: u32 = 56u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SENDER_MAX_LATE_JOINER_PERCENTAGE: u32 = 75u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SEND_FLAGS(pub u32);
pub const MSG_DONTROUTE: SEND_FLAGS = SEND_FLAGS(4u32);
pub const MSG_OOB: SEND_FLAGS = SEND_FLAGS(1u32);
impl ::std::convert::From<u32> for SEND_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SEND_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for SEND_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SEND_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SEND_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SEND_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SEND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SERVICE_ADDRESS {
    pub dwAddressType: u32,
    pub dwAddressFlags: u32,
    pub dwAddressLength: u32,
    pub dwPrincipalLength: u32,
    pub lpAddress: *mut u8,
    pub lpPrincipal: *mut u8,
}
impl SERVICE_ADDRESS {}
impl ::std::default::Default for SERVICE_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_ADDRESS")
            .field("dwAddressType", &self.dwAddressType)
            .field("dwAddressFlags", &self.dwAddressFlags)
            .field("dwAddressLength", &self.dwAddressLength)
            .field("dwPrincipalLength", &self.dwPrincipalLength)
            .field("lpAddress", &self.lpAddress)
            .field("lpPrincipal", &self.lpPrincipal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressType == other.dwAddressType && self.dwAddressFlags == other.dwAddressFlags && self.dwAddressLength == other.dwAddressLength && self.dwPrincipalLength == other.dwPrincipalLength && self.lpAddress == other.lpAddress && self.lpPrincipal == other.lpPrincipal
    }
}
impl ::std::cmp::Eq for SERVICE_ADDRESS {}
unsafe impl ::windows::runtime::Abi for SERVICE_ADDRESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SERVICE_ADDRESSES {
    pub dwAddressCount: u32,
    pub Addresses: [SERVICE_ADDRESS; 1],
}
impl SERVICE_ADDRESSES {}
impl ::std::default::Default for SERVICE_ADDRESSES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_ADDRESSES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_ADDRESSES").field("dwAddressCount", &self.dwAddressCount).field("Addresses", &self.Addresses).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressCount == other.dwAddressCount && self.Addresses == other.Addresses
    }
}
impl ::std::cmp::Eq for SERVICE_ADDRESSES {}
unsafe impl ::windows::runtime::Abi for SERVICE_ADDRESSES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_ADDRESS_FLAG_RPC_CN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_ADDRESS_FLAG_RPC_DG: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_ADDRESS_FLAG_RPC_NB: u32 = 4u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SERVICE_ASYNC_INFO {
    pub lpServiceCallbackProc: ::std::option::Option<LPSERVICE_CALLBACK_PROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub hAsyncTaskHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_ASYNC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_ASYNC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_ASYNC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_ASYNC_INFO").field("lParam", &self.lParam).field("hAsyncTaskHandle", &self.hAsyncTaskHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_ASYNC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceCallbackProc.map(|f| f as usize) == other.lpServiceCallbackProc.map(|f| f as usize) && self.lParam == other.lParam && self.hAsyncTaskHandle == other.hAsyncTaskHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_ASYNC_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_ASYNC_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_FLAG_DEFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_FLAG_HARD: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct SERVICE_INFOA {
    pub lpServiceType: *mut ::windows::runtime::GUID,
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpComment: super::super::Foundation::PSTR,
    pub lpLocale: super::super::Foundation::PSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: super::super::Foundation::PSTR,
    pub lpServiceAddress: *mut SERVICE_ADDRESSES,
    pub ServiceSpecificInfo: super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SERVICE_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for SERVICE_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for SERVICE_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_INFOA")
            .field("lpServiceType", &self.lpServiceType)
            .field("lpServiceName", &self.lpServiceName)
            .field("lpComment", &self.lpComment)
            .field("lpLocale", &self.lpLocale)
            .field("dwDisplayHint", &self.dwDisplayHint)
            .field("dwVersion", &self.dwVersion)
            .field("dwTime", &self.dwTime)
            .field("lpMachineName", &self.lpMachineName)
            .field("lpServiceAddress", &self.lpServiceAddress)
            .field("ServiceSpecificInfo", &self.ServiceSpecificInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType && self.lpServiceName == other.lpServiceName && self.lpComment == other.lpComment && self.lpLocale == other.lpLocale && self.dwDisplayHint == other.dwDisplayHint && self.dwVersion == other.dwVersion && self.dwTime == other.dwTime && self.lpMachineName == other.lpMachineName && self.lpServiceAddress == other.lpServiceAddress && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for SERVICE_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for SERVICE_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct SERVICE_INFOW {
    pub lpServiceType: *mut ::windows::runtime::GUID,
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpComment: super::super::Foundation::PWSTR,
    pub lpLocale: super::super::Foundation::PWSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: super::super::Foundation::PWSTR,
    pub lpServiceAddress: *mut SERVICE_ADDRESSES,
    pub ServiceSpecificInfo: super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SERVICE_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for SERVICE_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for SERVICE_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_INFOW")
            .field("lpServiceType", &self.lpServiceType)
            .field("lpServiceName", &self.lpServiceName)
            .field("lpComment", &self.lpComment)
            .field("lpLocale", &self.lpLocale)
            .field("dwDisplayHint", &self.dwDisplayHint)
            .field("dwVersion", &self.dwVersion)
            .field("dwTime", &self.dwTime)
            .field("lpMachineName", &self.lpMachineName)
            .field("lpServiceAddress", &self.lpServiceAddress)
            .field("ServiceSpecificInfo", &self.ServiceSpecificInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType && self.lpServiceName == other.lpServiceName && self.lpComment == other.lpComment && self.lpLocale == other.lpLocale && self.dwDisplayHint == other.dwDisplayHint && self.dwVersion == other.dwVersion && self.dwTime == other.dwTime && self.lpMachineName == other.lpMachineName && self.lpServiceAddress == other.lpServiceAddress && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for SERVICE_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for SERVICE_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_LOCAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_MULTIPLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SERVICE_SERVICE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SERVICE_TYPE_INFO {
    pub dwTypeNameOffset: u32,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE; 1],
}
impl SERVICE_TYPE_INFO {}
impl ::std::default::Default for SERVICE_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TYPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_INFO").field("dwTypeNameOffset", &self.dwTypeNameOffset).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTypeNameOffset == other.dwTypeNameOffset && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
impl ::std::cmp::Eq for SERVICE_TYPE_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SERVICE_TYPE_INFO_ABSA {
    pub lpTypeName: super::super::Foundation::PSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TYPE_INFO_ABSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TYPE_INFO_ABSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TYPE_INFO_ABSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_INFO_ABSA").field("lpTypeName", &self.lpTypeName).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TYPE_INFO_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TYPE_INFO_ABSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_INFO_ABSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SERVICE_TYPE_INFO_ABSW {
    pub lpTypeName: super::super::Foundation::PWSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TYPE_INFO_ABSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TYPE_INFO_ABSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TYPE_INFO_ABSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_INFO_ABSW").field("lpTypeName", &self.lpTypeName).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TYPE_INFO_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TYPE_INFO_ABSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_INFO_ABSW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SERVICE_TYPE_VALUE {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub dwValueNameOffset: u32,
    pub dwValueOffset: u32,
}
impl SERVICE_TYPE_VALUE {}
impl ::std::default::Default for SERVICE_TYPE_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TYPE_VALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_VALUE").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("dwValueNameOffset", &self.dwValueNameOffset).field("dwValueOffset", &self.dwValueOffset).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TYPE_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.dwValueNameOffset == other.dwValueNameOffset && self.dwValueOffset == other.dwValueOffset
    }
}
impl ::std::cmp::Eq for SERVICE_TYPE_VALUE {}
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_VALUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SERVICE_TYPE_VALUE_ABSA {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: super::super::Foundation::PSTR,
    pub lpValue: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TYPE_VALUE_ABSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TYPE_VALUE_ABSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TYPE_VALUE_ABSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_VALUE_ABSA").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValueName", &self.lpValueName).field("lpValue", &self.lpValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValueName == other.lpValueName && self.lpValue == other.lpValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TYPE_VALUE_ABSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_VALUE_ABSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SERVICE_TYPE_VALUE_ABSW {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: super::super::Foundation::PWSTR,
    pub lpValue: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TYPE_VALUE_ABSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TYPE_VALUE_ABSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TYPE_VALUE_ABSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TYPE_VALUE_ABSW").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValueName", &self.lpValueName).field("lpValue", &self.lpValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValueName == other.lpValueName && self.lpValue == other.lpValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TYPE_VALUE_ABSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TYPE_VALUE_ABSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SET_SERVICE_OPERATION(pub u32);
pub const SERVICE_REGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(1u32);
pub const SERVICE_DEREGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(2u32);
pub const SERVICE_FLUSH: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(3u32);
pub const SERVICE_ADD_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(4u32);
pub const SERVICE_DELETE_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(5u32);
impl ::std::convert::From<u32> for SET_SERVICE_OPERATION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SET_SERVICE_OPERATION {
    type Abi = Self;
}
impl ::std::ops::BitOr for SET_SERVICE_OPERATION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SET_SERVICE_OPERATION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SET_SERVICE_OPERATION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SET_SERVICE_OPERATION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SET_SERVICE_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SET_SERVICE_PARTIAL_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SG_CONSTRAINED_GROUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SG_UNCONSTRAINED_GROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SIO_ASSOCIATE_PVC: u32 = 2417360899u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SIO_GET_ATM_ADDRESS: u32 = 3491102722u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SIO_GET_ATM_CONNECTION_ID: u32 = 1343619076u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SIO_GET_NUMBER_OF_ATM_DEVICES: u32 = 1343619073u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SI_NETWORK: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SI_USER_FAILED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SI_USER_NOT_SCREENED: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SI_USER_PASSED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKADDR {
    pub sa_family: u16,
    pub sa_data: [super::super::Foundation::CHAR; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKADDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR").field("sa_family", &self.sa_family).field("sa_data", &self.sa_data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family && self.sa_data == other.sa_data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKADDR_DL {
    pub sdl_family: u16,
    pub sdl_data: [u8; 8],
    pub sdl_zero: [u8; 4],
}
impl SOCKADDR_DL {}
impl ::std::default::Default for SOCKADDR_DL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKADDR_DL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_DL").field("sdl_family", &self.sdl_family).field("sdl_data", &self.sdl_data).field("sdl_zero", &self.sdl_zero).finish()
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_DL {
    fn eq(&self, other: &Self) -> bool {
        self.sdl_family == other.sdl_family && self.sdl_data == other.sdl_data && self.sdl_zero == other.sdl_zero
    }
}
impl ::std::cmp::Eq for SOCKADDR_DL {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_DL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKADDR_IN {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: IN_ADDR,
    pub sin_zero: [super::super::Foundation::CHAR; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR_IN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR_IN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR_IN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR_IN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKADDR_IN6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub Anonymous: SOCKADDR_IN6_0,
}
impl SOCKADDR_IN6 {}
impl ::std::default::Default for SOCKADDR_IN6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_IN6 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SOCKADDR_IN6 {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_IN6 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union SOCKADDR_IN6_0 {
    pub sin6_scope_id: u32,
    pub sin6_scope_struct: SCOPE_ID,
}
impl SOCKADDR_IN6_0 {}
impl ::std::default::Default for SOCKADDR_IN6_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_IN6_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SOCKADDR_IN6_0 {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_IN6_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKADDR_IN6_PAIR {
    pub SourceAddress: *mut SOCKADDR_IN6,
    pub DestinationAddress: *mut SOCKADDR_IN6,
}
impl SOCKADDR_IN6_PAIR {}
impl ::std::default::Default for SOCKADDR_IN6_PAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKADDR_IN6_PAIR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_IN6_PAIR").field("SourceAddress", &self.SourceAddress).field("DestinationAddress", &self.DestinationAddress).finish()
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_IN6_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.SourceAddress == other.SourceAddress && self.DestinationAddress == other.DestinationAddress
    }
}
impl ::std::cmp::Eq for SOCKADDR_IN6_PAIR {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_IN6_PAIR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKADDR_IN6_W2KSP1 {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub sin6_scope_id: u32,
}
impl SOCKADDR_IN6_W2KSP1 {}
impl ::std::default::Default for SOCKADDR_IN6_W2KSP1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_IN6_W2KSP1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SOCKADDR_IN6_W2KSP1 {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_IN6_W2KSP1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union SOCKADDR_INET {
    pub Ipv4: SOCKADDR_IN,
    pub Ipv6: SOCKADDR_IN6,
    pub si_family: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR_INET {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR_INET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR_INET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR_INET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR_INET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKADDR_IRDA {
    pub irdaAddressFamily: u16,
    pub irdaDeviceID: [u8; 4],
    pub irdaServiceName: [super::super::Foundation::CHAR; 25],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR_IRDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR_IRDA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKADDR_IRDA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_IRDA").field("irdaAddressFamily", &self.irdaAddressFamily).field("irdaDeviceID", &self.irdaDeviceID).field("irdaServiceName", &self.irdaServiceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR_IRDA {
    fn eq(&self, other: &Self) -> bool {
        self.irdaAddressFamily == other.irdaAddressFamily && self.irdaDeviceID == other.irdaDeviceID && self.irdaServiceName == other.irdaServiceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR_IRDA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR_IRDA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKADDR_STORAGE {
    pub ss_family: u16,
    pub __ss_pad1: [super::super::Foundation::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [super::super::Foundation::CHAR; 112],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR_STORAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR_STORAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKADDR_STORAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_STORAGE").field("ss_family", &self.ss_family).field("__ss_pad1", &self.__ss_pad1).field("__ss_align", &self.__ss_align).field("__ss_pad2", &self.__ss_pad2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR_STORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family && self.__ss_pad1 == other.__ss_pad1 && self.__ss_align == other.__ss_align && self.__ss_pad2 == other.__ss_pad2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR_STORAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR_STORAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKADDR_STORAGE_XP {
    pub ss_family: i16,
    pub __ss_pad1: [super::super::Foundation::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [super::super::Foundation::CHAR; 112],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKADDR_STORAGE_XP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKADDR_STORAGE_XP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKADDR_STORAGE_XP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_STORAGE_XP").field("ss_family", &self.ss_family).field("__ss_pad1", &self.__ss_pad1).field("__ss_align", &self.__ss_align).field("__ss_pad2", &self.__ss_pad2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKADDR_STORAGE_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family && self.__ss_pad1 == other.__ss_pad1 && self.__ss_align == other.__ss_align && self.__ss_pad2 == other.__ss_pad2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKADDR_STORAGE_XP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKADDR_STORAGE_XP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct SOCKET(pub usize);
impl ::std::default::Default for SOCKET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for SOCKET {}
unsafe impl ::windows::runtime::Abi for SOCKET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: *mut SOCKADDR,
    pub iSockaddrLength: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKET_ADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKET_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKET_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_ADDRESS").field("lpSockaddr", &self.lpSockaddr).field("iSockaddrLength", &self.iSockaddrLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.lpSockaddr == other.lpSockaddr && self.iSockaddrLength == other.iSockaddrLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKET_ADDRESS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKET_ADDRESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKET_ADDRESS_LIST {
    pub iAddressCount: i32,
    pub Address: [SOCKET_ADDRESS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKET_ADDRESS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKET_ADDRESS_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_ADDRESS_LIST").field("iAddressCount", &self.iAddressCount).field("Address", &self.Address).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKET_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressCount == other.iAddressCount && self.Address == other.Address
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKET_ADDRESS_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKET_ADDRESS_LIST {
    type Abi = Self;
}
pub const SOCKET_DEFAULT2_QM_POLICY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2932010908, 14925, 19774, [136, 66, 35, 153, 66, 227, 154, 71]);
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_ERROR: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_INFO_CONNECTION_SECURED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKET_PEER_TARGET_NAME {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTargetNameStringLen: u32,
    pub AllStrings: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKET_PEER_TARGET_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKET_PEER_TARGET_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKET_PEER_TARGET_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_PEER_TARGET_NAME").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTargetNameStringLen", &self.PeerTargetNameStringLen).field("AllStrings", &self.AllStrings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKET_PEER_TARGET_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTargetNameStringLen == other.PeerTargetNameStringLen && self.AllStrings == other.AllStrings
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKET_PEER_TARGET_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKET_PEER_TARGET_NAME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SOCKET_PRIORITY_HINT(pub i32);
pub const SocketPriorityHintVeryLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(0i32);
pub const SocketPriorityHintLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(1i32);
pub const SocketPriorityHintNormal: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(2i32);
pub const SocketMaximumPriorityHintType: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(3i32);
impl ::std::convert::From<i32> for SOCKET_PRIORITY_HINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SOCKET_PRIORITY_HINT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_System_Kernel`*"]
pub struct SOCKET_PROCESSOR_AFFINITY {
    pub Processor: super::super::System::Kernel::PROCESSOR_NUMBER,
    pub NumaNodeId: u16,
    pub Reserved: u16,
}
#[cfg(feature = "Win32_System_Kernel")]
impl SOCKET_PROCESSOR_AFFINITY {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::default::Default for SOCKET_PROCESSOR_AFFINITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::fmt::Debug for SOCKET_PROCESSOR_AFFINITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_PROCESSOR_AFFINITY").field("Processor", &self.Processor).field("NumaNodeId", &self.NumaNodeId).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::PartialEq for SOCKET_PROCESSOR_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.NumaNodeId == other.NumaNodeId && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::Eq for SOCKET_PROCESSOR_AFFINITY {}
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::runtime::Abi for SOCKET_PROCESSOR_AFFINITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SOCKET_SECURITY_PROTOCOL(pub i32);
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(0i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(1i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(2i32);
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(3i32);
impl ::std::convert::From<i32> for SOCKET_SECURITY_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_PROTOCOL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKET_SECURITY_QUERY_INFO {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
}
impl SOCKET_SECURITY_QUERY_INFO {}
impl ::std::default::Default for SOCKET_SECURITY_QUERY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKET_SECURITY_QUERY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_QUERY_INFO").field("SecurityProtocol", &self.SecurityProtocol).field("Flags", &self.Flags).field("PeerApplicationAccessTokenHandle", &self.PeerApplicationAccessTokenHandle).field("PeerMachineAccessTokenHandle", &self.PeerMachineAccessTokenHandle).finish()
    }
}
impl ::std::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.Flags == other.Flags && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle
    }
}
impl ::std::cmp::Eq for SOCKET_SECURITY_QUERY_INFO {}
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_QUERY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
    pub MmSaId: u64,
    pub QmSaId: u64,
    pub NegotiationWinerr: u32,
    pub SaLookupContext: ::windows::runtime::GUID,
}
impl SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
impl ::std::default::Default for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_QUERY_INFO_IPSEC2")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("Flags", &self.Flags)
            .field("PeerApplicationAccessTokenHandle", &self.PeerApplicationAccessTokenHandle)
            .field("PeerMachineAccessTokenHandle", &self.PeerMachineAccessTokenHandle)
            .field("MmSaId", &self.MmSaId)
            .field("QmSaId", &self.QmSaId)
            .field("NegotiationWinerr", &self.NegotiationWinerr)
            .field("SaLookupContext", &self.SaLookupContext)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.Flags == other.Flags && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle && self.MmSaId == other.MmSaId && self.QmSaId == other.QmSaId && self.NegotiationWinerr == other.NegotiationWinerr && self.SaLookupContext == other.SaLookupContext
    }
}
impl ::std::cmp::Eq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKET_SECURITY_QUERY_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTokenAccessMask", &self.PeerTokenAccessMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTokenAccessMask == other.PeerTokenAccessMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_QUERY_TEMPLATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
    pub Flags: u32,
    pub FieldMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTokenAccessMask", &self.PeerTokenAccessMask).field("Flags", &self.Flags).field("FieldMask", &self.FieldMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTokenAccessMask == other.PeerTokenAccessMask && self.Flags == other.Flags && self.FieldMask == other.FieldMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKET_SECURITY_SETTINGS {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
}
impl SOCKET_SECURITY_SETTINGS {}
impl ::std::default::Default for SOCKET_SECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKET_SECURITY_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_SETTINGS").field("SecurityProtocol", &self.SecurityProtocol).field("SecurityFlags", &self.SecurityFlags).finish()
    }
}
impl ::std::cmp::PartialEq for SOCKET_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.SecurityFlags == other.SecurityFlags
    }
}
impl ::std::cmp::Eq for SOCKET_SECURITY_SETTINGS {}
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_SETTINGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCKET_SECURITY_SETTINGS_IPSEC {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
    pub IpsecFlags: u32,
    pub AuthipMMPolicyKey: ::windows::runtime::GUID,
    pub AuthipQMPolicyKey: ::windows::runtime::GUID,
    pub Reserved: ::windows::runtime::GUID,
    pub Reserved2: u64,
    pub UserNameStringLen: u32,
    pub DomainNameStringLen: u32,
    pub PasswordStringLen: u32,
    pub AllStrings: [u16; 1],
}
impl SOCKET_SECURITY_SETTINGS_IPSEC {}
impl ::std::default::Default for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKET_SECURITY_SETTINGS_IPSEC")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("IpsecFlags", &self.IpsecFlags)
            .field("AuthipMMPolicyKey", &self.AuthipMMPolicyKey)
            .field("AuthipQMPolicyKey", &self.AuthipQMPolicyKey)
            .field("Reserved", &self.Reserved)
            .field("Reserved2", &self.Reserved2)
            .field("UserNameStringLen", &self.UserNameStringLen)
            .field("DomainNameStringLen", &self.DomainNameStringLen)
            .field("PasswordStringLen", &self.PasswordStringLen)
            .field("AllStrings", &self.AllStrings)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.SecurityFlags == other.SecurityFlags
            && self.IpsecFlags == other.IpsecFlags
            && self.AuthipMMPolicyKey == other.AuthipMMPolicyKey
            && self.AuthipQMPolicyKey == other.AuthipQMPolicyKey
            && self.Reserved == other.Reserved
            && self.Reserved2 == other.Reserved2
            && self.UserNameStringLen == other.UserNameStringLen
            && self.DomainNameStringLen == other.DomainNameStringLen
            && self.PasswordStringLen == other.PasswordStringLen
            && self.AllStrings == other.AllStrings
    }
}
impl ::std::cmp::Eq for SOCKET_SECURITY_SETTINGS_IPSEC {}
unsafe impl ::windows::runtime::Abi for SOCKET_SECURITY_SETTINGS_IPSEC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_ALLOW_INSECURE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SOCKET_USAGE_TYPE(pub i32);
pub const SYSTEM_CRITICAL_SOCKET: SOCKET_USAGE_TYPE = SOCKET_USAGE_TYPE(1i32);
impl ::std::convert::From<i32> for SOCKET_USAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SOCKET_USAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_DGRAM: u16 = 2u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_EVENT_ERR: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_EVENT_HANGUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_EVENT_IN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_EVENT_OUT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_EVENT_REMOVE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_OP_DISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_OP_ENABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_OP_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_OP_REMOVE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct SOCK_NOTIFY_REGISTRATION {
    pub socket: SOCKET,
    pub completionKey: *mut ::std::ffi::c_void,
    pub eventFilter: u16,
    pub operation: u8,
    pub triggerFlags: u8,
    pub registrationResult: u32,
}
impl SOCK_NOTIFY_REGISTRATION {}
impl ::std::default::Default for SOCK_NOTIFY_REGISTRATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCK_NOTIFY_REGISTRATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCK_NOTIFY_REGISTRATION").field("socket", &self.socket).field("completionKey", &self.completionKey).field("eventFilter", &self.eventFilter).field("operation", &self.operation).field("triggerFlags", &self.triggerFlags).field("registrationResult", &self.registrationResult).finish()
    }
}
impl ::std::cmp::PartialEq for SOCK_NOTIFY_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.socket == other.socket && self.completionKey == other.completionKey && self.eventFilter == other.eventFilter && self.operation == other.operation && self.triggerFlags == other.triggerFlags && self.registrationResult == other.registrationResult
    }
}
impl ::std::cmp::Eq for SOCK_NOTIFY_REGISTRATION {}
unsafe impl ::windows::runtime::Abi for SOCK_NOTIFY_REGISTRATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_TRIGGER_EDGE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_TRIGGER_LEVEL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_RAW: u16 = 3u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_RDM: u16 = 4u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_SEQPACKET: u16 = 5u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOCK_STREAM: u16 = 1u16;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOL_IRLMP: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOL_SOCKET: u32 = 65535u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SOMAXCONN: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_ACCEPTCONN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_BROADCAST: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_BSP_STATE: u32 = 4105u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_COMPARTMENT_ID: u32 = 12292u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONDITIONAL_ACCEPT: u32 = 12290u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONNDATA: u32 = 28672u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONNDATALEN: u32 = 28676u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONNECT_TIME: u32 = 28684u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONNOPT: u32 = 28673u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_CONNOPTLEN: u32 = 28677u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DEBUG: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DISCDATA: u32 = 28674u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DISCDATALEN: u32 = 28678u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DISCOPT: u32 = 28675u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DISCOPTLEN: u32 = 28679u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_DONTROUTE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_ERROR: u32 = 4103u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_GROUP_ID: u32 = 8193u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_GROUP_PRIORITY: u32 = 8194u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_KEEPALIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_LINGER: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_MAXDG: u32 = 28681u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_MAXPATHDG: u32 = 28682u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_MAX_MSG_SIZE: u32 = 8195u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_OOBINLINE: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_OPENTYPE: u32 = 28680u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_ORIGINAL_DST: u32 = 12303u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_PAUSE_ACCEPT: u32 = 12291u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_PORT_SCALABILITY: u32 = 12294u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_PROTOCOL_INFO: u32 = 8197u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_PROTOCOL_INFOA: u32 = 8196u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_PROTOCOL_INFOW: u32 = 8197u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_RANDOMIZE_PORT: u32 = 12293u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_RCVBUF: u32 = 4098u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_RCVLOWAT: u32 = 4100u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_RCVTIMEO: u32 = 4102u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_REUSEADDR: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_REUSE_MULTICASTPORT: u32 = 12296u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_REUSE_UNICASTPORT: u32 = 12295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_SNDBUF: u32 = 4097u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_SNDLOWAT: u32 = 4099u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_SNDTIMEO: u32 = 4101u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_SYNCHRONOUS_ALERT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_SYNCHRONOUS_NONALERT: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_TIMESTAMP: u32 = 12298u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_TIMESTAMP_ID: u32 = 12299u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_TYPE: u32 = 4104u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_UPDATE_ACCEPT_CONTEXT: u32 = 28683u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_UPDATE_CONNECT_CONTEXT: u32 = 28688u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const SO_USELOOPBACK: u32 = 64u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn SetAddrInfoExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    pname: Param0,
    pservicename: Param1,
    paddresses: *const SOCKET_ADDRESS,
    dwaddresscount: u32,
    lpblob: *const super::super::System::Com::BLOB,
    dwflags: u32,
    dwnamespace: u32,
    lpnspid: *const ::windows::runtime::GUID,
    timeout: *const timeval,
    lpoverlapped: *const super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<LPLOOKUPSERVICE_COMPLETION_ROUTINE>,
    lpnamehandle: *mut super::super::Foundation::HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(SetAddrInfoExA(
            pname.into_param().abi(),
            pservicename.into_param().abi(),
            ::std::mem::transmute(paddresses),
            ::std::mem::transmute(dwaddresscount),
            ::std::mem::transmute(lpblob),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpnspid),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
            ::std::mem::transmute(lpnamehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn SetAddrInfoExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    pname: Param0,
    pservicename: Param1,
    paddresses: *const SOCKET_ADDRESS,
    dwaddresscount: u32,
    lpblob: *const super::super::System::Com::BLOB,
    dwflags: u32,
    dwnamespace: u32,
    lpnspid: *const ::windows::runtime::GUID,
    timeout: *const timeval,
    lpoverlapped: *const super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<LPLOOKUPSERVICE_COMPLETION_ROUTINE>,
    lpnamehandle: *mut super::super::Foundation::HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(SetAddrInfoExW(
            pname.into_param().abi(),
            pservicename.into_param().abi(),
            ::std::mem::transmute(paddresses),
            ::std::mem::transmute(dwaddresscount),
            ::std::mem::transmute(lpblob),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwnamespace),
            ::std::mem::transmute(lpnspid),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
            ::std::mem::transmute(lpnamehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn SetServiceA(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOA, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceA(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOA, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpdwstatusflags: *mut u32) -> i32;
        }
        ::std::mem::transmute(SetServiceA(::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwoperation), ::std::mem::transmute(dwflags), ::std::mem::transmute(lpserviceinfo), ::std::mem::transmute(lpserviceasyncinfo), ::std::mem::transmute(lpdwstatusflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn SetServiceW(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOW, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceW(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOW, lpserviceasyncinfo: *const ::std::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpdwstatusflags: *mut u32) -> i32;
        }
        ::std::mem::transmute(SetServiceW(::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwoperation), ::std::mem::transmute(dwflags), ::std::mem::transmute(lpserviceinfo), ::std::mem::transmute(lpserviceasyncinfo), ::std::mem::transmute(lpdwstatusflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SetSocketMediaStreamingMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(value: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSocketMediaStreamingMode(value: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        SetSocketMediaStreamingMode(value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TCPSTATE(pub i32);
pub const TCPSTATE_CLOSED: TCPSTATE = TCPSTATE(0i32);
pub const TCPSTATE_LISTEN: TCPSTATE = TCPSTATE(1i32);
pub const TCPSTATE_SYN_SENT: TCPSTATE = TCPSTATE(2i32);
pub const TCPSTATE_SYN_RCVD: TCPSTATE = TCPSTATE(3i32);
pub const TCPSTATE_ESTABLISHED: TCPSTATE = TCPSTATE(4i32);
pub const TCPSTATE_FIN_WAIT_1: TCPSTATE = TCPSTATE(5i32);
pub const TCPSTATE_FIN_WAIT_2: TCPSTATE = TCPSTATE(6i32);
pub const TCPSTATE_CLOSE_WAIT: TCPSTATE = TCPSTATE(7i32);
pub const TCPSTATE_CLOSING: TCPSTATE = TCPSTATE(8i32);
pub const TCPSTATE_LAST_ACK: TCPSTATE = TCPSTATE(9i32);
pub const TCPSTATE_TIME_WAIT: TCPSTATE = TCPSTATE(10i32);
pub const TCPSTATE_MAX: TCPSTATE = TCPSTATE(11i32);
impl ::std::convert::From<i32> for TCPSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TCPSTATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TCP_ACK_FREQUENCY_PARAMETERS {
    pub TcpDelayedAckFrequency: u8,
}
impl TCP_ACK_FREQUENCY_PARAMETERS {}
impl ::std::default::Default for TCP_ACK_FREQUENCY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TCP_ACK_FREQUENCY_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCP_ACK_FREQUENCY_PARAMETERS").field("TcpDelayedAckFrequency", &self.TcpDelayedAckFrequency).finish()
    }
}
impl ::std::cmp::PartialEq for TCP_ACK_FREQUENCY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.TcpDelayedAckFrequency == other.TcpDelayedAckFrequency
    }
}
impl ::std::cmp::Eq for TCP_ACK_FREQUENCY_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for TCP_ACK_FREQUENCY_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_ATMARK: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_BSDURGENT: u32 = 28672u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_CONGESTION_ALGORITHM: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_DELAY_FIN_ACK: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_EXPEDITED_1122: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_FASTOPEN: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_ICMP_ERROR_INFO: u32 = 19u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TCP_ICW_LEVEL(pub i32);
pub const TCP_ICW_LEVEL_DEFAULT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(0i32);
pub const TCP_ICW_LEVEL_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(1i32);
pub const TCP_ICW_LEVEL_VERY_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(2i32);
pub const TCP_ICW_LEVEL_AGGRESSIVE: TCP_ICW_LEVEL = TCP_ICW_LEVEL(3i32);
pub const TCP_ICW_LEVEL_EXPERIMENTAL: TCP_ICW_LEVEL = TCP_ICW_LEVEL(4i32);
pub const TCP_ICW_LEVEL_COMPAT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(254i32);
pub const TCP_ICW_LEVEL_MAX: TCP_ICW_LEVEL = TCP_ICW_LEVEL(255i32);
impl ::std::convert::From<i32> for TCP_ICW_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TCP_ICW_LEVEL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TCP_ICW_PARAMETERS {
    pub Level: TCP_ICW_LEVEL,
}
impl TCP_ICW_PARAMETERS {}
impl ::std::default::Default for TCP_ICW_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TCP_ICW_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCP_ICW_PARAMETERS").field("Level", &self.Level).finish()
    }
}
impl ::std::cmp::PartialEq for TCP_ICW_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::std::cmp::Eq for TCP_ICW_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for TCP_ICW_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct TCP_INFO_v0 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: super::super::Foundation::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl TCP_INFO_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCP_INFO_v0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCP_INFO_v0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCP_INFO_v0")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCP_INFO_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.Mss == other.Mss
            && self.ConnectionTimeMs == other.ConnectionTimeMs
            && self.TimestampsEnabled == other.TimestampsEnabled
            && self.RttUs == other.RttUs
            && self.MinRttUs == other.MinRttUs
            && self.BytesInFlight == other.BytesInFlight
            && self.Cwnd == other.Cwnd
            && self.SndWnd == other.SndWnd
            && self.RcvWnd == other.RcvWnd
            && self.RcvBuf == other.RcvBuf
            && self.BytesOut == other.BytesOut
            && self.BytesIn == other.BytesIn
            && self.BytesReordered == other.BytesReordered
            && self.BytesRetrans == other.BytesRetrans
            && self.FastRetrans == other.FastRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.TimeoutEpisodes == other.TimeoutEpisodes
            && self.SynRetrans == other.SynRetrans
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCP_INFO_v0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCP_INFO_v0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct TCP_INFO_v1 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: super::super::Foundation::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: u64,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: u64,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl TCP_INFO_v1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCP_INFO_v1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCP_INFO_v1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCP_INFO_v1")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .field("SndLimTransRwin", &self.SndLimTransRwin)
            .field("SndLimTimeRwin", &self.SndLimTimeRwin)
            .field("SndLimBytesRwin", &self.SndLimBytesRwin)
            .field("SndLimTransCwnd", &self.SndLimTransCwnd)
            .field("SndLimTimeCwnd", &self.SndLimTimeCwnd)
            .field("SndLimBytesCwnd", &self.SndLimBytesCwnd)
            .field("SndLimTransSnd", &self.SndLimTransSnd)
            .field("SndLimTimeSnd", &self.SndLimTimeSnd)
            .field("SndLimBytesSnd", &self.SndLimBytesSnd)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCP_INFO_v1 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.Mss == other.Mss
            && self.ConnectionTimeMs == other.ConnectionTimeMs
            && self.TimestampsEnabled == other.TimestampsEnabled
            && self.RttUs == other.RttUs
            && self.MinRttUs == other.MinRttUs
            && self.BytesInFlight == other.BytesInFlight
            && self.Cwnd == other.Cwnd
            && self.SndWnd == other.SndWnd
            && self.RcvWnd == other.RcvWnd
            && self.RcvBuf == other.RcvBuf
            && self.BytesOut == other.BytesOut
            && self.BytesIn == other.BytesIn
            && self.BytesReordered == other.BytesReordered
            && self.BytesRetrans == other.BytesRetrans
            && self.FastRetrans == other.FastRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.TimeoutEpisodes == other.TimeoutEpisodes
            && self.SynRetrans == other.SynRetrans
            && self.SndLimTransRwin == other.SndLimTransRwin
            && self.SndLimTimeRwin == other.SndLimTimeRwin
            && self.SndLimBytesRwin == other.SndLimBytesRwin
            && self.SndLimTransCwnd == other.SndLimTransCwnd
            && self.SndLimTimeCwnd == other.SndLimTimeCwnd
            && self.SndLimBytesCwnd == other.SndLimBytesCwnd
            && self.SndLimTransSnd == other.SndLimTransSnd
            && self.SndLimTimeSnd == other.SndLimTimeSnd
            && self.SndLimBytesSnd == other.SndLimBytesSnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCP_INFO_v1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCP_INFO_v1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_INITIAL_RTO_DEFAULT_RTT: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TCP_INITIAL_RTO_PARAMETERS {
    pub Rtt: u16,
    pub MaxSynRetransmissions: u8,
}
impl TCP_INITIAL_RTO_PARAMETERS {}
impl ::std::default::Default for TCP_INITIAL_RTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TCP_INITIAL_RTO_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCP_INITIAL_RTO_PARAMETERS").field("Rtt", &self.Rtt).field("MaxSynRetransmissions", &self.MaxSynRetransmissions).finish()
    }
}
impl ::std::cmp::PartialEq for TCP_INITIAL_RTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Rtt == other.Rtt && self.MaxSynRetransmissions == other.MaxSynRetransmissions
    }
}
impl ::std::cmp::Eq for TCP_INITIAL_RTO_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for TCP_INITIAL_RTO_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_KEEPALIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_KEEPCNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_KEEPIDLE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_KEEPINTVL: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_MAXRT: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_MAXRTMS: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_MAXSEG: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_NODELAY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_NOSYNRETRIES: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_NOURG: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_OFFLOAD_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_STDURG: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TCP_TIMESTAMPS: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_DISCONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_REUSE_SOCKET: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_USE_DEFAULT_WORKER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_USE_KERNEL_APC: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_USE_SYSTEM_THREAD: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TF_WRITE_BEHIND: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TH_NETDEV: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TH_TAPI: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TIMESTAMPING_CONFIG {
    pub Flags: u32,
    pub TxTimestampsBuffered: u16,
}
impl TIMESTAMPING_CONFIG {}
impl ::std::default::Default for TIMESTAMPING_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TIMESTAMPING_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TIMESTAMPING_CONFIG").field("Flags", &self.Flags).field("TxTimestampsBuffered", &self.TxTimestampsBuffered).finish()
    }
}
impl ::std::cmp::PartialEq for TIMESTAMPING_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TxTimestampsBuffered == other.TxTimestampsBuffered
    }
}
impl ::std::cmp::Eq for TIMESTAMPING_CONFIG {}
unsafe impl ::windows::runtime::Abi for TIMESTAMPING_CONFIG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TIMESTAMPING_FLAG_RX: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TIMESTAMPING_FLAG_TX: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TNS_PLAN_CARRIER_ID_CODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TNS_TYPE_NATIONAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_DISCONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_ELEMENT_EOP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_ELEMENT_FILE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_ELEMENT_MEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_REUSE_SOCKET: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_USE_DEFAULT_WORKER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_USE_KERNEL_APC: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TP_USE_SYSTEM_THREAD: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TRANSMIT_FILE_BUFFERS {
    pub Head: *mut ::std::ffi::c_void,
    pub HeadLength: u32,
    pub Tail: *mut ::std::ffi::c_void,
    pub TailLength: u32,
}
impl TRANSMIT_FILE_BUFFERS {}
impl ::std::default::Default for TRANSMIT_FILE_BUFFERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSMIT_FILE_BUFFERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSMIT_FILE_BUFFERS").field("Head", &self.Head).field("HeadLength", &self.HeadLength).field("Tail", &self.Tail).field("TailLength", &self.TailLength).finish()
    }
}
impl ::std::cmp::PartialEq for TRANSMIT_FILE_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        self.Head == other.Head && self.HeadLength == other.HeadLength && self.Tail == other.Tail && self.TailLength == other.TailLength
    }
}
impl ::std::cmp::Eq for TRANSMIT_FILE_BUFFERS {}
unsafe impl ::windows::runtime::Abi for TRANSMIT_FILE_BUFFERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct TRANSMIT_PACKETS_ELEMENT {
    pub dwElFlags: u32,
    pub cLength: u32,
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl TRANSMIT_PACKETS_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRANSMIT_PACKETS_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRANSMIT_PACKETS_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRANSMIT_PACKETS_ELEMENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union TRANSMIT_PACKETS_ELEMENT_0 {
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0_0,
    pub pBuffer: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl TRANSMIT_PACKETS_ELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRANSMIT_PACKETS_ELEMENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRANSMIT_PACKETS_ELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRANSMIT_PACKETS_ELEMENT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct TRANSMIT_PACKETS_ELEMENT_0_0 {
    pub nFileOffset: i64,
    pub hFile: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TRANSMIT_PACKETS_ELEMENT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("nFileOffset", &self.nFileOffset).field("hFile", &self.hFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.nFileOffset == other.nFileOffset && self.hFile == other.hFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRANSMIT_PACKETS_ELEMENT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRANSMIT_PACKETS_ELEMENT_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct TRANSPORT_SETTING_ID {
    pub Guid: ::windows::runtime::GUID,
}
impl TRANSPORT_SETTING_ID {}
impl ::std::default::Default for TRANSPORT_SETTING_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSPORT_SETTING_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSPORT_SETTING_ID").field("Guid", &self.Guid).finish()
    }
}
impl ::std::cmp::PartialEq for TRANSPORT_SETTING_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid
    }
}
impl ::std::cmp::Eq for TRANSPORT_SETTING_ID {}
unsafe impl ::windows::runtime::Abi for TRANSPORT_SETTING_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TR_END_TO_END: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TR_NOIND: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TR_NO_END_TO_END: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TT_CBR: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TT_NOIND: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const TT_VBR: u32 = 8u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn TransmitFile<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsocket: Param0, hfile: Param1, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TransmitFile(hsocket: SOCKET, hfile: super::super::Foundation::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TransmitFile(hsocket.into_param().abi(), hfile.into_param().abi(), ::std::mem::transmute(nnumberofbytestowrite), ::std::mem::transmute(nnumberofbytespersend), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lptransmitbuffers), ::std::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UDP_CHECKSUM_COVERAGE: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UDP_COALESCED_INFO: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UDP_NOCHECKSUM: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UDP_SEND_MSG_SIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UNIX_PATH_MAX: u32 = 108u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UP_P2MP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const UP_P2P: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const VNSPROTO_IPC: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const VNSPROTO_RELIABLE_IPC: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const VNSPROTO_SPP: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WCE_AF_IRDA: u32 = 22u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WCE_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WCE_IRDA_DEVICE_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WCE_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WCE_DEVICELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WCE_DEVICELIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCE_DEVICELIST").field("numDevice", &self.numDevice).field("Device", &self.Device).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WCE_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WCE_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WCE_DEVICELIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WCE_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [super::super::Foundation::CHAR; 22],
    pub Reserved: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl WCE_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WCE_IRDA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WCE_IRDA_DEVICE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCE_IRDA_DEVICE_INFO").field("irdaDeviceID", &self.irdaDeviceID).field("irdaDeviceName", &self.irdaDeviceName).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WCE_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID && self.irdaDeviceName == other.irdaDeviceName && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WCE_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WCE_IRDA_DEVICE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WCE_PF_IRDA: u32 = 22u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WINDOWS_AF_IRDA: u32 = 26u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WINDOWS_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WINDOWS_IRDA_DEVICE_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WINDOWS_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINDOWS_DEVICELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINDOWS_DEVICELIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINDOWS_DEVICELIST").field("numDevice", &self.numDevice).field("Device", &self.Device).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINDOWS_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINDOWS_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINDOWS_DEVICELIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WINDOWS_IAS_QUERY {
    pub irdaDeviceID: [u8; 4],
    pub irdaClassName: [super::super::Foundation::CHAR; 64],
    pub irdaAttribName: [super::super::Foundation::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_QUERY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINDOWS_IAS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINDOWS_IAS_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINDOWS_IAS_QUERY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINDOWS_IAS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_QUERY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union WINDOWS_IAS_QUERY_0 {
    pub irdaAttribInt: i32,
    pub irdaAttribOctetSeq: WINDOWS_IAS_QUERY_0_0,
    pub irdaAttribUsrStr: WINDOWS_IAS_QUERY_0_1,
}
impl WINDOWS_IAS_QUERY_0 {}
impl ::std::default::Default for WINDOWS_IAS_QUERY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_QUERY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_QUERY_0 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_QUERY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WINDOWS_IAS_QUERY_0_0 {
    pub Len: u32,
    pub OctetSeq: [u8; 1024],
}
impl WINDOWS_IAS_QUERY_0_0 {}
impl ::std::default::Default for WINDOWS_IAS_QUERY_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINDOWS_IAS_QUERY_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_irdaAttribOctetSeq_e__Struct").field("Len", &self.Len).field("OctetSeq", &self.OctetSeq).finish()
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_QUERY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.OctetSeq == other.OctetSeq
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_QUERY_0_0 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_QUERY_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WINDOWS_IAS_QUERY_0_1 {
    pub Len: u32,
    pub CharSet: u32,
    pub UsrStr: [u8; 256],
}
impl WINDOWS_IAS_QUERY_0_1 {}
impl ::std::default::Default for WINDOWS_IAS_QUERY_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINDOWS_IAS_QUERY_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_irdaAttribUsrStr_e__Struct").field("Len", &self.Len).field("CharSet", &self.CharSet).field("UsrStr", &self.UsrStr).finish()
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_QUERY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.CharSet == other.CharSet && self.UsrStr == other.UsrStr
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_QUERY_0_1 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_QUERY_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WINDOWS_IAS_SET {
    pub irdaClassName: [super::super::Foundation::CHAR; 64],
    pub irdaAttribName: [super::super::Foundation::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_SET_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINDOWS_IAS_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINDOWS_IAS_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINDOWS_IAS_SET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINDOWS_IAS_SET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub union WINDOWS_IAS_SET_0 {
    pub irdaAttribInt: i32,
    pub irdaAttribOctetSeq: WINDOWS_IAS_SET_0_0,
    pub irdaAttribUsrStr: WINDOWS_IAS_SET_0_1,
}
impl WINDOWS_IAS_SET_0 {}
impl ::std::default::Default for WINDOWS_IAS_SET_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_SET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_SET_0 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_SET_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WINDOWS_IAS_SET_0_0 {
    pub Len: u16,
    pub OctetSeq: [u8; 1024],
}
impl WINDOWS_IAS_SET_0_0 {}
impl ::std::default::Default for WINDOWS_IAS_SET_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINDOWS_IAS_SET_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_irdaAttribOctetSeq_e__Struct").field("Len", &self.Len).field("OctetSeq", &self.OctetSeq).finish()
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_SET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.OctetSeq == other.OctetSeq
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_SET_0_0 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_SET_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WINDOWS_IAS_SET_0_1 {
    pub Len: u8,
    pub CharSet: u8,
    pub UsrStr: [u8; 256],
}
impl WINDOWS_IAS_SET_0_1 {}
impl ::std::default::Default for WINDOWS_IAS_SET_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINDOWS_IAS_SET_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_irdaAttribUsrStr_e__Struct").field("Len", &self.Len).field("CharSet", &self.CharSet).field("UsrStr", &self.UsrStr).finish()
    }
}
impl ::std::cmp::PartialEq for WINDOWS_IAS_SET_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.CharSet == other.CharSet && self.UsrStr == other.UsrStr
    }
}
impl ::std::cmp::Eq for WINDOWS_IAS_SET_0_1 {}
unsafe impl ::windows::runtime::Abi for WINDOWS_IAS_SET_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WINDOWS_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [super::super::Foundation::CHAR; 22],
    pub irdaDeviceHints1: u8,
    pub irdaDeviceHints2: u8,
    pub irdaCharSet: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WINDOWS_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINDOWS_IRDA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINDOWS_IRDA_DEVICE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINDOWS_IRDA_DEVICE_INFO").field("irdaDeviceID", &self.irdaDeviceID).field("irdaDeviceName", &self.irdaDeviceName).field("irdaDeviceHints1", &self.irdaDeviceHints1).field("irdaDeviceHints2", &self.irdaDeviceHints2).field("irdaCharSet", &self.irdaCharSet).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINDOWS_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID && self.irdaDeviceName == other.irdaDeviceName && self.irdaDeviceHints1 == other.irdaDeviceHints1 && self.irdaDeviceHints2 == other.irdaDeviceHints2 && self.irdaCharSet == other.irdaCharSet
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINDOWS_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINDOWS_IRDA_DEVICE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WINDOWS_PF_IRDA: u32 = 26u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WPUCompleteOverlappedRequest<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WPUCompleteOverlappedRequest(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WPUCompleteOverlappedRequest(s.into_param().abi(), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(dwerror), ::std::mem::transmute(cbtransferred), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
#[inline]
pub unsafe fn WSAAccept<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: ::std::option::Option<LPCONDITIONPROC>, dwcallbackdata: usize) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAccept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: ::windows::runtime::RawPtr, dwcallbackdata: usize) -> SOCKET;
        }
        ::std::mem::transmute(WSAAccept(s.into_param().abi(), ::std::mem::transmute(addr), ::std::mem::transmute(addrlen), ::std::mem::transmute(lpfncondition), ::std::mem::transmute(dwcallbackdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAddressToStringA(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpszaddressstring: super::super::Foundation::PSTR, lpdwaddressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAddressToStringA(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpszaddressstring: super::super::Foundation::PSTR, lpdwaddressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAAddressToStringA(::std::mem::transmute(lpsaaddress), ::std::mem::transmute(dwaddresslength), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(lpszaddressstring), ::std::mem::transmute(lpdwaddressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAddressToStringW(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAddressToStringW(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAAddressToStringW(::std::mem::transmute(lpsaaddress), ::std::mem::transmute(dwaddresslength), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(lpszaddressstring), ::std::mem::transmute(lpdwaddressstringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSAAdvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID, pnspv2routine: *const NSPV2_ROUTINE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAdvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID, pnspv2routine: *const ::std::mem::ManuallyDrop<NSPV2_ROUTINE>) -> i32;
        }
        ::std::mem::transmute(WSAAdvertiseProvider(::std::mem::transmute(puuidproviderid), ::std::mem::transmute(pnspv2routine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetHostByAddr<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, wmsg: u32, addr: Param2, len: i32, r#type: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetHostByAddr(hwnd: super::super::Foundation::HWND, wmsg: u32, addr: super::super::Foundation::PSTR, len: i32, r#type: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetHostByAddr(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), addr.into_param().abi(), ::std::mem::transmute(len), ::std::mem::transmute(r#type), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetHostByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, wmsg: u32, name: Param2, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetHostByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetHostByName(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), name.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetProtoByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, wmsg: u32, name: Param2, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetProtoByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetProtoByName(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), name.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetProtoByNumber<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, wmsg: u32, number: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetProtoByNumber(hwnd: super::super::Foundation::HWND, wmsg: u32, number: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetProtoByNumber(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), ::std::mem::transmute(number), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetServByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, wmsg: u32, name: Param2, proto: Param3, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetServByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetServByName(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), name.into_param().abi(), proto.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncGetServByPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, wmsg: u32, port: i32, proto: Param3, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncGetServByPort(hwnd: super::super::Foundation::HWND, wmsg: u32, port: i32, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSAAsyncGetServByPort(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), ::std::mem::transmute(port), proto.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAAsyncSelect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(s: Param0, hwnd: Param1, wmsg: u32, levent: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAAsyncSelect(s: SOCKET, hwnd: super::super::Foundation::HWND, wmsg: u32, levent: i32) -> i32;
        }
        ::std::mem::transmute(WSAAsyncSelect(s.into_param().abi(), hwnd.into_param().abi(), ::std::mem::transmute(wmsg), ::std::mem::transmute(levent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSABUF {
    pub len: u32,
    pub buf: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSABUF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSABUF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSABUF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSABUF").field("len", &self.len).field("buf", &self.buf).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSABUF {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.buf == other.buf
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSABUF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSABUF {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::clone::Clone for WSACOMPLETION {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    pub Parameters: WSACOMPLETION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSACOMPLETION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSACOMPLETION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSACOMPLETION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSACOMPLETION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::clone::Clone for WSACOMPLETION_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub union WSACOMPLETION_0 {
    pub WindowMessage: WSACOMPLETION_0_3,
    pub Event: WSACOMPLETION_0_1,
    pub Apc: ::std::mem::ManuallyDrop<WSACOMPLETION_0_0>,
    pub Port: WSACOMPLETION_0_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSACOMPLETION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSACOMPLETION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSACOMPLETION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSACOMPLETION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WSACOMPLETION_0_0 {
    pub lpOverlapped: *mut super::super::System::IO::OVERLAPPED,
    pub lpfnCompletionProc: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSACOMPLETION_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSACOMPLETION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WSACOMPLETION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Apc_e__Struct").field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSACOMPLETION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped && self.lpfnCompletionProc.map(|f| f as usize) == other.lpfnCompletionProc.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSACOMPLETION_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WSACOMPLETION_0_1 {
    pub lpOverlapped: *mut super::super::System::IO::OVERLAPPED,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSACOMPLETION_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSACOMPLETION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WSACOMPLETION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Event_e__Struct").field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSACOMPLETION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSACOMPLETION_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WSACOMPLETION_0_2 {
    pub lpOverlapped: *mut super::super::System::IO::OVERLAPPED,
    pub hPort: super::super::Foundation::HANDLE,
    pub Key: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSACOMPLETION_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSACOMPLETION_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WSACOMPLETION_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Port_e__Struct").field("lpOverlapped", &self.lpOverlapped).field("hPort", &self.hPort).field("Key", &self.Key).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSACOMPLETION_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped && self.hPort == other.hPort && self.Key == other.Key
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSACOMPLETION_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSACOMPLETION_0_3 {
    pub hWnd: super::super::Foundation::HWND,
    pub uMsg: u32,
    pub context: super::super::Foundation::WPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl WSACOMPLETION_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSACOMPLETION_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSACOMPLETION_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_WindowMessage_e__Struct").field("hWnd", &self.hWnd).field("uMsg", &self.uMsg).field("context", &self.context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSACOMPLETION_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.uMsg == other.uMsg && self.context == other.context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSACOMPLETION_0_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSACOMPLETION_0_3 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSACOMPLETIONTYPE(pub i32);
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(0i32);
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(1i32);
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(2i32);
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(3i32);
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(4i32);
impl ::std::convert::From<i32> for WSACOMPLETIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSACOMPLETIONTYPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSACancelAsyncRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hasynctaskhandle: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSACancelAsyncRequest(hasynctaskhandle: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(WSACancelAsyncRequest(hasynctaskhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSACancelBlockingCall() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSACancelBlockingCall() -> i32;
        }
        ::std::mem::transmute(WSACancelBlockingCall())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSACleanup() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSACleanup() -> i32;
        }
        ::std::mem::transmute(WSACleanup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSACloseEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSACloseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSACloseEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
#[inline]
pub unsafe fn WSAConnect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAConnect(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS) -> i32;
        }
        ::std::mem::transmute(WSAConnect(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen), ::std::mem::transmute(lpcallerdata), ::std::mem::transmute(lpcalleedata), ::std::mem::transmute(lpsqos), ::std::mem::transmute(lpgqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAConnectByList<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, socketaddress: *const SOCKET_ADDRESS_LIST, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAConnectByList(s: SOCKET, socketaddress: *const SOCKET_ADDRESS_LIST, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAConnectByList(
            s.into_param().abi(),
            ::std::mem::transmute(socketaddress),
            ::std::mem::transmute(localaddresslength),
            ::std::mem::transmute(localaddress),
            ::std::mem::transmute(remoteaddresslength),
            ::std::mem::transmute(remoteaddress),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAConnectByNameA<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    s: Param0,
    nodename: Param1,
    servicename: Param2,
    localaddresslength: *mut u32,
    localaddress: *mut SOCKADDR,
    remoteaddresslength: *mut u32,
    remoteaddress: *mut SOCKADDR,
    timeout: *const timeval,
    reserved: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAConnectByNameA(s: SOCKET, nodename: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAConnectByNameA(
            s.into_param().abi(),
            nodename.into_param().abi(),
            servicename.into_param().abi(),
            ::std::mem::transmute(localaddresslength),
            ::std::mem::transmute(localaddress),
            ::std::mem::transmute(remoteaddresslength),
            ::std::mem::transmute(remoteaddress),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAConnectByNameW<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    s: Param0,
    nodename: Param1,
    servicename: Param2,
    localaddresslength: *mut u32,
    localaddress: *mut SOCKADDR,
    remoteaddresslength: *mut u32,
    remoteaddress: *mut SOCKADDR,
    timeout: *const timeval,
    reserved: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAConnectByNameW(s: SOCKET, nodename: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAConnectByNameW(
            s.into_param().abi(),
            nodename.into_param().abi(),
            servicename.into_param().abi(),
            ::std::mem::transmute(localaddresslength),
            ::std::mem::transmute(localaddress),
            ::std::mem::transmute(remoteaddresslength),
            ::std::mem::transmute(remoteaddress),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSACreateEvent() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSACreateEvent() -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(WSACreateEvent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSADESCRIPTION_LEN: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: super::super::Foundation::PSTR,
    pub szDescription: [super::super::Foundation::CHAR; 257],
    pub szSystemStatus: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl WSAData {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSAData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSAData {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAData")
            .field("wVersion", &self.wVersion)
            .field("wHighVersion", &self.wHighVersion)
            .field("iMaxSockets", &self.iMaxSockets)
            .field("iMaxUdpDg", &self.iMaxUdpDg)
            .field("lpVendorInfo", &self.lpVendorInfo)
            .field("szDescription", &self.szDescription)
            .field("szSystemStatus", &self.szSystemStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSAData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.iMaxSockets == other.iMaxSockets && self.iMaxUdpDg == other.iMaxUdpDg && self.lpVendorInfo == other.lpVendorInfo && self.szDescription == other.szDescription && self.szSystemStatus == other.szSystemStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSAData {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSAData {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [super::super::Foundation::CHAR; 257],
    pub szSystemStatus: [super::super::Foundation::CHAR; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSAData {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSAData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSAData {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAData")
            .field("wVersion", &self.wVersion)
            .field("wHighVersion", &self.wHighVersion)
            .field("szDescription", &self.szDescription)
            .field("szSystemStatus", &self.szSystemStatus)
            .field("iMaxSockets", &self.iMaxSockets)
            .field("iMaxUdpDg", &self.iMaxUdpDg)
            .field("lpVendorInfo", &self.lpVendorInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSAData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.szDescription == other.szDescription && self.szSystemStatus == other.szSystemStatus && self.iMaxSockets == other.iMaxSockets && self.iMaxUdpDg == other.iMaxUdpDg && self.lpVendorInfo == other.lpVendorInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSAData {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSAData {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSADeleteSocketPeerTargetName<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(socket: Param0, peeraddr: *const SOCKADDR, peeraddrlen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSADeleteSocketPeerTargetName(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSADeleteSocketPeerTargetName(socket.into_param().abi(), ::std::mem::transmute(peeraddr), ::std::mem::transmute(peeraddrlen), ::std::mem::transmute(overlapped), ::std::mem::transmute(completionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSADuplicateSocketA<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSADuplicateSocketA(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32;
        }
        ::std::mem::transmute(WSADuplicateSocketA(s.into_param().abi(), ::std::mem::transmute(dwprocessid), ::std::mem::transmute(lpprotocolinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSADuplicateSocketW<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSADuplicateSocketW(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32;
        }
        ::std::mem::transmute(WSADuplicateSocketW(s.into_param().abi(), ::std::mem::transmute(dwprocessid), ::std::mem::transmute(lpprotocolinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSAECOMPARATOR(pub i32);
pub const COMP_EQUAL: WSAECOMPARATOR = WSAECOMPARATOR(0i32);
pub const COMP_NOTLESS: WSAECOMPARATOR = WSAECOMPARATOR(1i32);
impl ::std::convert::From<i32> for WSAECOMPARATOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSAECOMPARATOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSAESETSERVICEOP(pub i32);
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(0i32);
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(1i32);
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = WSAESETSERVICEOP(2i32);
impl ::std::convert::From<i32> for WSAESETSERVICEOP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSAESETSERVICEOP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32;
        }
        ::std::mem::transmute(WSAEnumNameSpaceProvidersA(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32;
        }
        ::std::mem::transmute(WSAEnumNameSpaceProvidersExA(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
        }
        ::std::mem::transmute(WSAEnumNameSpaceProvidersExW(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
        }
        ::std::mem::transmute(WSAEnumNameSpaceProvidersW(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAEnumNetworkEvents<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(s: Param0, heventobject: Param1, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumNetworkEvents(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32;
        }
        ::std::mem::transmute(WSAEnumNetworkEvents(s.into_param().abi(), heventobject.into_param().abi(), ::std::mem::transmute(lpnetworkevents)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAEnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOA, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOA, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAEnumProtocolsA(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAEnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAEnumProtocolsW(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAEventSelect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(s: Param0, heventobject: Param1, lnetworkevents: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAEventSelect(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lnetworkevents: i32) -> i32;
        }
        ::std::mem::transmute(WSAEventSelect(s.into_param().abi(), heventobject.into_param().abi(), ::std::mem::transmute(lnetworkevents)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAGetLastError() -> WSA_ERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetLastError() -> WSA_ERROR;
        }
        ::std::mem::transmute(WSAGetLastError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAGetOverlappedResult<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(s: Param0, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: Param3, lpdwflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: super::super::Foundation::BOOL, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAGetOverlappedResult(s.into_param().abi(), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lpcbtransfer), fwait.into_param().abi(), ::std::mem::transmute(lpdwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
#[inline]
pub unsafe fn WSAGetQOSByName<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetQOSByName(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAGetQOSByName(s.into_param().abi(), ::std::mem::transmute(lpqosname), ::std::mem::transmute(lpqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAGetServiceClassInfoA(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetServiceClassInfoA(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32;
        }
        ::std::mem::transmute(WSAGetServiceClassInfoA(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpserviceclassid), ::std::mem::transmute(lpdwbufsize), ::std::mem::transmute(lpserviceclassinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAGetServiceClassInfoW(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetServiceClassInfoW(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32;
        }
        ::std::mem::transmute(WSAGetServiceClassInfoW(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpserviceclassid), ::std::mem::transmute(lpdwbufsize), ::std::mem::transmute(lpserviceclassinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAGetServiceClassNameByClassIdA(::std::mem::transmute(lpserviceclassid), ::std::mem::transmute(lpszserviceclassname), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAGetServiceClassNameByClassIdW(::std::mem::transmute(lpserviceclassid), ::std::mem::transmute(lpszserviceclassname), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAHtonl<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, hostlong: u32, lpnetlong: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAHtonl(s: SOCKET, hostlong: u32, lpnetlong: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSAHtonl(s.into_param().abi(), ::std::mem::transmute(hostlong), ::std::mem::transmute(lpnetlong)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAHtons<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, hostshort: u16, lpnetshort: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAHtons(s: SOCKET, hostshort: u16, lpnetshort: *mut u16) -> i32;
        }
        ::std::mem::transmute(WSAHtons(s.into_param().abi(), ::std::mem::transmute(hostshort), ::std::mem::transmute(lpnetshort)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAImpersonateSocketPeer<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(socket: Param0, peeraddr: *const SOCKADDR, peeraddrlen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAImpersonateSocketPeer(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32) -> i32;
        }
        ::std::mem::transmute(WSAImpersonateSocketPeer(socket.into_param().abi(), ::std::mem::transmute(peeraddr), ::std::mem::transmute(peeraddrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32;
        }
        ::std::mem::transmute(WSAInstallServiceClassA(::std::mem::transmute(lpserviceclassinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
        }
        ::std::mem::transmute(WSAInstallServiceClassW(::std::mem::transmute(lpserviceclassinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAIoctl<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, dwiocontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAIoctl(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSAIoctl(
            s.into_param().abi(),
            ::std::mem::transmute(dwiocontrolcode),
            ::std::mem::transmute(lpvinbuffer),
            ::std::mem::transmute(cbinbuffer),
            ::std::mem::transmute(lpvoutbuffer),
            ::std::mem::transmute(cboutbuffer),
            ::std::mem::transmute(lpcbbytesreturned),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAIsBlocking() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAIsBlocking() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAIsBlocking())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
#[inline]
pub unsafe fn WSAJoinLeaf<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAJoinLeaf(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32) -> SOCKET;
        }
        ::std::mem::transmute(WSAJoinLeaf(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen), ::std::mem::transmute(lpcallerdata), ::std::mem::transmute(lpcalleedata), ::std::mem::transmute(lpsqos), ::std::mem::transmute(lpgqos), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(WSALookupServiceBeginA(::std::mem::transmute(lpqsrestrictions), ::std::mem::transmute(dwcontrolflags), ::std::mem::transmute(lphlookup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(WSALookupServiceBeginW(::std::mem::transmute(lpqsrestrictions), ::std::mem::transmute(dwcontrolflags), ::std::mem::transmute(lphlookup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSALookupServiceEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hlookup: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSALookupServiceEnd(hlookup: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(WSALookupServiceEnd(hlookup.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSALookupServiceNextA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hlookup: Param0, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSALookupServiceNextA(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32;
        }
        ::std::mem::transmute(WSALookupServiceNextA(hlookup.into_param().abi(), ::std::mem::transmute(dwcontrolflags), ::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpqsresults)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSALookupServiceNextW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hlookup: Param0, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSALookupServiceNextW(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
        }
        ::std::mem::transmute(WSALookupServiceNextW(hlookup.into_param().abi(), ::std::mem::transmute(dwcontrolflags), ::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpqsresults)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSAMSG {
    pub name: *mut SOCKADDR,
    pub namelen: i32,
    pub lpBuffers: *mut WSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WSAMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSAMSG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSAMSG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAMSG").field("name", &self.name).field("namelen", &self.namelen).field("lpBuffers", &self.lpBuffers).field("dwBufferCount", &self.dwBufferCount).field("Control", &self.Control).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSAMSG {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.namelen == other.namelen && self.lpBuffers == other.lpBuffers && self.dwBufferCount == other.dwBufferCount && self.Control == other.Control && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSAMSG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSAMSG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: ::windows::runtime::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSANAMESPACE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSANAMESPACE_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSANAMESPACE_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANAMESPACE_INFOA").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSANAMESPACE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSANAMESPACE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSANAMESPACE_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: ::windows::runtime::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: super::super::Foundation::PSTR,
    pub ProviderSpecific: super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSANAMESPACE_INFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSANAMESPACE_INFOEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSANAMESPACE_INFOEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANAMESPACE_INFOEXA").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).field("ProviderSpecific", &self.ProviderSpecific).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSANAMESPACE_INFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier && self.ProviderSpecific == other.ProviderSpecific
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSANAMESPACE_INFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSANAMESPACE_INFOEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: ::windows::runtime::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: super::super::Foundation::PWSTR,
    pub ProviderSpecific: super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSANAMESPACE_INFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSANAMESPACE_INFOEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSANAMESPACE_INFOEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANAMESPACE_INFOEXW").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).field("ProviderSpecific", &self.ProviderSpecific).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSANAMESPACE_INFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier && self.ProviderSpecific == other.ProviderSpecific
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSANAMESPACE_INFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSANAMESPACE_INFOEXW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: ::windows::runtime::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSANAMESPACE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSANAMESPACE_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSANAMESPACE_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANAMESPACE_INFOW").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSANAMESPACE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSANAMESPACE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSANAMESPACE_INFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSANETWORKEVENTS {
    pub lNetworkEvents: i32,
    pub iErrorCode: [i32; 10],
}
impl WSANETWORKEVENTS {}
impl ::std::default::Default for WSANETWORKEVENTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSANETWORKEVENTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANETWORKEVENTS").field("lNetworkEvents", &self.lNetworkEvents).field("iErrorCode", &self.iErrorCode).finish()
    }
}
impl ::std::cmp::PartialEq for WSANETWORKEVENTS {
    fn eq(&self, other: &Self) -> bool {
        self.lNetworkEvents == other.lNetworkEvents && self.iErrorCode == other.iErrorCode
    }
}
impl ::std::cmp::Eq for WSANETWORKEVENTS {}
unsafe impl ::windows::runtime::Abi for WSANETWORKEVENTS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSANSCLASSINFOA {
    pub lpszName: super::super::Foundation::PSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSANSCLASSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSANSCLASSINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSANSCLASSINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANSCLASSINFOA").field("lpszName", &self.lpszName).field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValue", &self.lpValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSANSCLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName && self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValue == other.lpValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSANSCLASSINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSANSCLASSINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSANSCLASSINFOW {
    pub lpszName: super::super::Foundation::PWSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSANSCLASSINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSANSCLASSINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSANSCLASSINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSANSCLASSINFOW").field("lpszName", &self.lpszName).field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValue", &self.lpValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSANSCLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName && self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValue == other.lpValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSANSCLASSINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSANSCLASSINFOW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSANSPIoctl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hlookup: Param0, dwcontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const WSACOMPLETION) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSANSPIoctl(hlookup: super::super::Foundation::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const ::std::mem::ManuallyDrop<WSACOMPLETION>) -> i32;
        }
        ::std::mem::transmute(WSANSPIoctl(hlookup.into_param().abi(), ::std::mem::transmute(dwcontrolcode), ::std::mem::transmute(lpvinbuffer), ::std::mem::transmute(cbinbuffer), ::std::mem::transmute(lpvoutbuffer), ::std::mem::transmute(cboutbuffer), ::std::mem::transmute(lpcbbytesreturned), ::std::mem::transmute(lpcompletion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSANtohl<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, netlong: u32, lphostlong: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSANtohl(s: SOCKET, netlong: u32, lphostlong: *mut u32) -> i32;
        }
        ::std::mem::transmute(WSANtohl(s.into_param().abi(), ::std::mem::transmute(netlong), ::std::mem::transmute(lphostlong)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSANtohs<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, netshort: u16, lphostshort: *mut u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSANtohs(s: SOCKET, netshort: u16, lphostshort: *mut u16) -> i32;
        }
        ::std::mem::transmute(WSANtohs(s.into_param().abi(), ::std::mem::transmute(netshort), ::std::mem::transmute(lphostshort)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSAPOLLDATA {
    pub result: i32,
    pub fds: u32,
    pub timeout: i32,
    pub fdArray: [WSAPOLLFD; 1],
}
impl WSAPOLLDATA {}
impl ::std::default::Default for WSAPOLLDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSAPOLLDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAPOLLDATA").field("result", &self.result).field("fds", &self.fds).field("timeout", &self.timeout).field("fdArray", &self.fdArray).finish()
    }
}
impl ::std::cmp::PartialEq for WSAPOLLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.fds == other.fds && self.timeout == other.timeout && self.fdArray == other.fdArray
    }
}
impl ::std::cmp::Eq for WSAPOLLDATA {}
unsafe impl ::windows::runtime::Abi for WSAPOLLDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSAPOLLFD {
    pub fd: SOCKET,
    pub events: i16,
    pub revents: i16,
}
impl WSAPOLLFD {}
impl ::std::default::Default for WSAPOLLFD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSAPOLLFD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAPOLLFD").field("fd", &self.fd).field("events", &self.events).field("revents", &self.revents).finish()
    }
}
impl ::std::cmp::PartialEq for WSAPOLLFD {
    fn eq(&self, other: &Self) -> bool {
        self.fd == other.fd && self.events == other.events && self.revents == other.revents
    }
}
impl ::std::cmp::Eq for WSAPOLLFD {}
unsafe impl ::windows::runtime::Abi for WSAPOLLFD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: i32,
    pub ChainEntries: [u32; 7],
}
impl WSAPROTOCOLCHAIN {}
impl ::std::default::Default for WSAPROTOCOLCHAIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSAPROTOCOLCHAIN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAPROTOCOLCHAIN").field("ChainLen", &self.ChainLen).field("ChainEntries", &self.ChainEntries).finish()
    }
}
impl ::std::cmp::PartialEq for WSAPROTOCOLCHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.ChainLen == other.ChainLen && self.ChainEntries == other.ChainEntries
    }
}
impl ::std::cmp::Eq for WSAPROTOCOLCHAIN {}
unsafe impl ::windows::runtime::Abi for WSAPROTOCOLCHAIN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: ::windows::runtime::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl WSAPROTOCOL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSAPROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSAPROTOCOL_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAPROTOCOL_INFOA")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSAPROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSAPROTOCOL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSAPROTOCOL_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: ::windows::runtime::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [u16; 256],
}
impl WSAPROTOCOL_INFOW {}
impl ::std::default::Default for WSAPROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSAPROTOCOL_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAPROTOCOL_INFOW")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WSAPROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
impl ::std::cmp::Eq for WSAPROTOCOL_INFOW {}
unsafe impl ::windows::runtime::Abi for WSAPROTOCOL_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSAPROTOCOL_LEN: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32;
        }
        ::std::mem::transmute(WSAPoll(::std::mem::transmute(fdarray), ::std::mem::transmute(fds), ::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAProviderCompleteAsyncCall<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hasynccall: Param0, iretcode: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAProviderCompleteAsyncCall(hasynccall: super::super::Foundation::HANDLE, iretcode: i32) -> i32;
        }
        ::std::mem::transmute(WSAProviderCompleteAsyncCall(hasynccall.into_param().abi(), ::std::mem::transmute(iretcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAProviderConfigChange(lpnotificationhandle: *mut super::super::Foundation::HANDLE, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAProviderConfigChange(lpnotificationhandle: *mut super::super::Foundation::HANDLE, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSAProviderConfigChange(::std::mem::transmute(lpnotificationhandle), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lpcompletionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSAQUERYSET2A {
    pub dwSize: u32,
    pub lpszServiceInstanceName: super::super::Foundation::PSTR,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: super::super::Foundation::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows::runtime::GUID,
    pub lpszContext: super::super::Foundation::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: super::super::Foundation::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSAQUERYSET2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSAQUERYSET2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSAQUERYSET2A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAQUERYSET2A")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSAQUERYSET2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSAQUERYSET2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSAQUERYSET2A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSAQUERYSET2W {
    pub dwSize: u32,
    pub lpszServiceInstanceName: super::super::Foundation::PWSTR,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: super::super::Foundation::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows::runtime::GUID,
    pub lpszContext: super::super::Foundation::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: super::super::Foundation::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSAQUERYSET2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSAQUERYSET2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSAQUERYSET2W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAQUERYSET2W")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSAQUERYSET2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSAQUERYSET2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSAQUERYSET2W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSAQUERYSETA {
    pub dwSize: u32,
    pub lpszServiceInstanceName: super::super::Foundation::PSTR,
    pub lpServiceClassId: *mut ::windows::runtime::GUID,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: super::super::Foundation::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows::runtime::GUID,
    pub lpszContext: super::super::Foundation::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: super::super::Foundation::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSAQUERYSETA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSAQUERYSETA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSAQUERYSETA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAQUERYSETA")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSAQUERYSETA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpServiceClassId == other.lpServiceClassId
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSAQUERYSETA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSAQUERYSETA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct WSAQUERYSETW {
    pub dwSize: u32,
    pub lpszServiceInstanceName: super::super::Foundation::PWSTR,
    pub lpServiceClassId: *mut ::windows::runtime::GUID,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: super::super::Foundation::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows::runtime::GUID,
    pub lpszContext: super::super::Foundation::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: super::super::Foundation::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut super::super::System::Com::BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl WSAQUERYSETW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for WSAQUERYSETW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for WSAQUERYSETW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAQUERYSETW")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for WSAQUERYSETW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpServiceClassId == other.lpServiceClassId
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for WSAQUERYSETW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for WSAQUERYSETW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSAQuerySocketSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(socket: Param0, securityquerytemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE, securityquerytemplatelen: u32, securityqueryinfo: *mut SOCKET_SECURITY_QUERY_INFO, securityqueryinfolen: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAQuerySocketSecurity(socket: SOCKET, securityquerytemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE, securityquerytemplatelen: u32, securityqueryinfo: *mut SOCKET_SECURITY_QUERY_INFO, securityqueryinfolen: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSAQuerySocketSecurity(
            socket.into_param().abi(),
            ::std::mem::transmute(securityquerytemplate),
            ::std::mem::transmute(securityquerytemplatelen),
            ::std::mem::transmute(securityqueryinfo),
            ::std::mem::transmute(securityqueryinfolen),
            ::std::mem::transmute(overlapped),
            ::std::mem::transmute(completionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSARecv<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARecv(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSARecv(s.into_param().abi(), ::std::mem::transmute(lpbuffers), ::std::mem::transmute(dwbuffercount), ::std::mem::transmute(lpnumberofbytesrecvd), ::std::mem::transmute(lpflags), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lpcompletionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSARecvDisconnect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpinbounddisconnectdata: *const WSABUF) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: *const WSABUF) -> i32;
        }
        ::std::mem::transmute(WSARecvDisconnect(s.into_param().abi(), ::std::mem::transmute(lpinbounddisconnectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSARecvEx<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, buf: super::super::Foundation::PSTR, len: i32, flags: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARecvEx(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSARecvEx(s.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(len), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSARecvFrom<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARecvFrom(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSARecvFrom(
            s.into_param().abi(),
            ::std::mem::transmute(lpbuffers),
            ::std::mem::transmute(dwbuffercount),
            ::std::mem::transmute(lpnumberofbytesrecvd),
            ::std::mem::transmute(lpflags),
            ::std::mem::transmute(lpfrom),
            ::std::mem::transmute(lpfromlen),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSARemoveServiceClass(lpserviceclassid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARemoveServiceClass(lpserviceclassid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSARemoveServiceClass(::std::mem::transmute(lpserviceclassid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAResetEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSAResetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSARevertImpersonation() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSARevertImpersonation() -> i32;
        }
        ::std::mem::transmute(WSARevertImpersonation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WSASENDMSG {
    pub lpMsg: *mut WSAMSG,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: *mut u32,
    pub lpOverlapped: *mut super::super::System::IO::OVERLAPPED,
    pub lpCompletionRoutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WSASENDMSG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSASENDMSG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WSASENDMSG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSASENDMSG").field("lpMsg", &self.lpMsg).field("dwFlags", &self.dwFlags).field("lpNumberOfBytesSent", &self.lpNumberOfBytesSent).field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSASENDMSG {
    fn eq(&self, other: &Self) -> bool {
        self.lpMsg == other.lpMsg && self.dwFlags == other.dwFlags && self.lpNumberOfBytesSent == other.lpNumberOfBytesSent && self.lpOverlapped == other.lpOverlapped && self.lpCompletionRoutine.map(|f| f as usize) == other.lpCompletionRoutine.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSASENDMSG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSASENDMSG {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: *mut ::windows::runtime::GUID,
    pub lpszServiceClassName: super::super::Foundation::PSTR,
    pub dwCount: u32,
    pub lpClassInfos: *mut WSANSCLASSINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl WSASERVICECLASSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSASERVICECLASSINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSASERVICECLASSINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSASERVICECLASSINFOA").field("lpServiceClassId", &self.lpServiceClassId).field("lpszServiceClassName", &self.lpszServiceClassName).field("dwCount", &self.dwCount).field("lpClassInfos", &self.lpClassInfos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSASERVICECLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId && self.lpszServiceClassName == other.lpszServiceClassName && self.dwCount == other.dwCount && self.lpClassInfos == other.lpClassInfos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSASERVICECLASSINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSASERVICECLASSINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: *mut ::windows::runtime::GUID,
    pub lpszServiceClassName: super::super::Foundation::PWSTR,
    pub dwCount: u32,
    pub lpClassInfos: *mut WSANSCLASSINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl WSASERVICECLASSINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSASERVICECLASSINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSASERVICECLASSINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSASERVICECLASSINFOW").field("lpServiceClassId", &self.lpServiceClassId).field("lpszServiceClassName", &self.lpszServiceClassName).field("dwCount", &self.dwCount).field("lpClassInfos", &self.lpClassInfos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSASERVICECLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId && self.lpszServiceClassName == other.lpszServiceClassName && self.dwCount == other.dwCount && self.lpClassInfos == other.lpClassInfos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSASERVICECLASSINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSASERVICECLASSINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSASYS_STATUS_LEN: u32 = 128u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSASend<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASend(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSASend(s.into_param().abi(), ::std::mem::transmute(lpbuffers), ::std::mem::transmute(dwbuffercount), ::std::mem::transmute(lpnumberofbytessent), ::std::mem::transmute(dwflags), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lpcompletionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSASendDisconnect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpoutbounddisconnectdata: *const WSABUF) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF) -> i32;
        }
        ::std::mem::transmute(WSASendDisconnect(s.into_param().abi(), ::std::mem::transmute(lpoutbounddisconnectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSASendMsg<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(handle: Param0, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASendMsg(handle: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSASendMsg(handle.into_param().abi(), ::std::mem::transmute(lpmsg), ::std::mem::transmute(dwflags), ::std::mem::transmute(lpnumberofbytessent), ::std::mem::transmute(lpoverlapped), ::std::mem::transmute(lpcompletionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSASendTo<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASendTo(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSASendTo(
            s.into_param().abi(),
            ::std::mem::transmute(lpbuffers),
            ::std::mem::transmute(dwbuffercount),
            ::std::mem::transmute(lpnumberofbytessent),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpto),
            ::std::mem::transmute(itolen),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSASetBlockingHook(lpblockfunc: ::std::option::Option<super::super::Foundation::FARPROC>) -> ::std::option::Option<super::super::Foundation::FARPROC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetBlockingHook(lpblockfunc: ::windows::runtime::RawPtr) -> ::std::option::Option<super::super::Foundation::FARPROC>;
        }
        ::std::mem::transmute(WSASetBlockingHook(::std::mem::transmute(lpblockfunc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSASetEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WSASetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSASetLastError(ierror: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetLastError(ierror: i32);
        }
        ::std::mem::transmute(WSASetLastError(::std::mem::transmute(ierror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
        }
        ::std::mem::transmute(WSASetServiceA(::std::mem::transmute(lpqsreginfo), ::std::mem::transmute(essoperation), ::std::mem::transmute(dwcontrolflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
        }
        ::std::mem::transmute(WSASetServiceW(::std::mem::transmute(lpqsreginfo), ::std::mem::transmute(essoperation), ::std::mem::transmute(dwcontrolflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSASetSocketPeerTargetName<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(socket: Param0, peertargetname: *const SOCKET_PEER_TARGET_NAME, peertargetnamelen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetSocketPeerTargetName(socket: SOCKET, peertargetname: *const SOCKET_PEER_TARGET_NAME, peertargetnamelen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSASetSocketPeerTargetName(socket.into_param().abi(), ::std::mem::transmute(peertargetname), ::std::mem::transmute(peertargetnamelen), ::std::mem::transmute(overlapped), ::std::mem::transmute(completionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn WSASetSocketSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(socket: Param0, securitysettings: *const SOCKET_SECURITY_SETTINGS, securitysettingslen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASetSocketSecurity(socket: SOCKET, securitysettings: *const SOCKET_SECURITY_SETTINGS, securitysettingslen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(WSASetSocketSecurity(socket.into_param().abi(), ::std::mem::transmute(securitysettings), ::std::mem::transmute(securitysettingslen), ::std::mem::transmute(overlapped), ::std::mem::transmute(completionroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, g: u32, dwflags: u32) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, g: u32, dwflags: u32) -> SOCKET;
        }
        ::std::mem::transmute(WSASocketA(::std::mem::transmute(af), ::std::mem::transmute(r#type), ::std::mem::transmute(protocol), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(g), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32) -> SOCKET;
        }
        ::std::mem::transmute(WSASocketW(::std::mem::transmute(af), ::std::mem::transmute(r#type), ::std::mem::transmute(protocol), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(g), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSAData) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSAData) -> i32;
        }
        ::std::mem::transmute(WSAStartup(::std::mem::transmute(wversionrequested), ::std::mem::transmute(lpwsadata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAStringToAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(addressstring: Param0, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAStringToAddressA(addressstring: super::super::Foundation::PSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSAStringToAddressA(addressstring.into_param().abi(), ::std::mem::transmute(addressfamily), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(lpaddress), ::std::mem::transmute(lpaddresslength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAStringToAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(addressstring: Param0, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAStringToAddressW(addressstring: super::super::Foundation::PWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSAStringToAddressW(addressstring.into_param().abi(), ::std::mem::transmute(addressfamily), ::std::mem::transmute(lpprotocolinfo), ::std::mem::transmute(lpaddress), ::std::mem::transmute(lpaddresslength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSATHREADID {
    pub ThreadHandle: super::super::Foundation::HANDLE,
    pub Reserved: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl WSATHREADID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSATHREADID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSATHREADID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSATHREADID").field("ThreadHandle", &self.ThreadHandle).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSATHREADID {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadHandle == other.ThreadHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSATHREADID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSATHREADID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAUnadvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAUnadvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSAUnadvertiseProvider(::std::mem::transmute(puuidproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSAUnhookBlockingHook() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAUnhookBlockingHook() -> i32;
        }
        ::std::mem::transmute(WSAUnhookBlockingHook())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSAVERSION {
    pub dwVersion: u32,
    pub ecHow: WSAECOMPARATOR,
}
impl WSAVERSION {}
impl ::std::default::Default for WSAVERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSAVERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSAVERSION").field("dwVersion", &self.dwVersion).field("ecHow", &self.ecHow).finish()
    }
}
impl ::std::cmp::PartialEq for WSAVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.ecHow == other.ecHow
    }
}
impl ::std::cmp::Eq for WSAVERSION {}
unsafe impl ::windows::runtime::Abi for WSAVERSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSAWaitForMultipleEvents<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(cevents: u32, lphevents: *const super::super::Foundation::HANDLE, fwaitall: Param2, dwtimeout: u32, falertable: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSAWaitForMultipleEvents(cevents: u32, lphevents: *const super::super::Foundation::HANDLE, fwaitall: super::super::Foundation::BOOL, dwtimeout: u32, falertable: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(WSAWaitForMultipleEvents(::std::mem::transmute(cevents), ::std::mem::transmute(lphevents), fwaitall.into_param().abi(), ::std::mem::transmute(dwtimeout), falertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSA_COMPATIBILITY_BEHAVIOR_ID(pub i32);
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(0i32);
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(1i32);
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(2i32);
impl ::std::convert::From<i32> for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSA_COMPATIBILITY_BEHAVIOR_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: u32,
}
impl WSA_COMPATIBILITY_MODE {}
impl ::std::default::Default for WSA_COMPATIBILITY_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSA_COMPATIBILITY_MODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSA_COMPATIBILITY_MODE").field("BehaviorId", &self.BehaviorId).field("TargetOsVersion", &self.TargetOsVersion).finish()
    }
}
impl ::std::cmp::PartialEq for WSA_COMPATIBILITY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BehaviorId == other.BehaviorId && self.TargetOsVersion == other.TargetOsVersion
    }
}
impl ::std::cmp::Eq for WSA_COMPATIBILITY_MODE {}
unsafe impl ::windows::runtime::Abi for WSA_COMPATIBILITY_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSA_ERROR(pub i32);
pub const WSA_IO_PENDING: WSA_ERROR = WSA_ERROR(997i32);
pub const WSA_IO_INCOMPLETE: WSA_ERROR = WSA_ERROR(996i32);
pub const WSA_INVALID_HANDLE: WSA_ERROR = WSA_ERROR(6i32);
pub const WSA_INVALID_PARAMETER: WSA_ERROR = WSA_ERROR(87i32);
pub const WSA_NOT_ENOUGH_MEMORY: WSA_ERROR = WSA_ERROR(8i32);
pub const WSA_OPERATION_ABORTED: WSA_ERROR = WSA_ERROR(995i32);
pub const WSABASEERR: WSA_ERROR = WSA_ERROR(10000i32);
pub const WSAEINTR: WSA_ERROR = WSA_ERROR(10004i32);
pub const WSAEBADF: WSA_ERROR = WSA_ERROR(10009i32);
pub const WSAEACCES: WSA_ERROR = WSA_ERROR(10013i32);
pub const WSAEFAULT: WSA_ERROR = WSA_ERROR(10014i32);
pub const WSAEINVAL: WSA_ERROR = WSA_ERROR(10022i32);
pub const WSAEMFILE: WSA_ERROR = WSA_ERROR(10024i32);
pub const WSAEWOULDBLOCK: WSA_ERROR = WSA_ERROR(10035i32);
pub const WSAEINPROGRESS: WSA_ERROR = WSA_ERROR(10036i32);
pub const WSAEALREADY: WSA_ERROR = WSA_ERROR(10037i32);
pub const WSAENOTSOCK: WSA_ERROR = WSA_ERROR(10038i32);
pub const WSAEDESTADDRREQ: WSA_ERROR = WSA_ERROR(10039i32);
pub const WSAEMSGSIZE: WSA_ERROR = WSA_ERROR(10040i32);
pub const WSAEPROTOTYPE: WSA_ERROR = WSA_ERROR(10041i32);
pub const WSAENOPROTOOPT: WSA_ERROR = WSA_ERROR(10042i32);
pub const WSAEPROTONOSUPPORT: WSA_ERROR = WSA_ERROR(10043i32);
pub const WSAESOCKTNOSUPPORT: WSA_ERROR = WSA_ERROR(10044i32);
pub const WSAEOPNOTSUPP: WSA_ERROR = WSA_ERROR(10045i32);
pub const WSAEPFNOSUPPORT: WSA_ERROR = WSA_ERROR(10046i32);
pub const WSAEAFNOSUPPORT: WSA_ERROR = WSA_ERROR(10047i32);
pub const WSAEADDRINUSE: WSA_ERROR = WSA_ERROR(10048i32);
pub const WSAEADDRNOTAVAIL: WSA_ERROR = WSA_ERROR(10049i32);
pub const WSAENETDOWN: WSA_ERROR = WSA_ERROR(10050i32);
pub const WSAENETUNREACH: WSA_ERROR = WSA_ERROR(10051i32);
pub const WSAENETRESET: WSA_ERROR = WSA_ERROR(10052i32);
pub const WSAECONNABORTED: WSA_ERROR = WSA_ERROR(10053i32);
pub const WSAECONNRESET: WSA_ERROR = WSA_ERROR(10054i32);
pub const WSAENOBUFS: WSA_ERROR = WSA_ERROR(10055i32);
pub const WSAEISCONN: WSA_ERROR = WSA_ERROR(10056i32);
pub const WSAENOTCONN: WSA_ERROR = WSA_ERROR(10057i32);
pub const WSAESHUTDOWN: WSA_ERROR = WSA_ERROR(10058i32);
pub const WSAETOOMANYREFS: WSA_ERROR = WSA_ERROR(10059i32);
pub const WSAETIMEDOUT: WSA_ERROR = WSA_ERROR(10060i32);
pub const WSAECONNREFUSED: WSA_ERROR = WSA_ERROR(10061i32);
pub const WSAELOOP: WSA_ERROR = WSA_ERROR(10062i32);
pub const WSAENAMETOOLONG: WSA_ERROR = WSA_ERROR(10063i32);
pub const WSAEHOSTDOWN: WSA_ERROR = WSA_ERROR(10064i32);
pub const WSAEHOSTUNREACH: WSA_ERROR = WSA_ERROR(10065i32);
pub const WSAENOTEMPTY: WSA_ERROR = WSA_ERROR(10066i32);
pub const WSAEPROCLIM: WSA_ERROR = WSA_ERROR(10067i32);
pub const WSAEUSERS: WSA_ERROR = WSA_ERROR(10068i32);
pub const WSAEDQUOT: WSA_ERROR = WSA_ERROR(10069i32);
pub const WSAESTALE: WSA_ERROR = WSA_ERROR(10070i32);
pub const WSAEREMOTE: WSA_ERROR = WSA_ERROR(10071i32);
pub const WSASYSNOTREADY: WSA_ERROR = WSA_ERROR(10091i32);
pub const WSAVERNOTSUPPORTED: WSA_ERROR = WSA_ERROR(10092i32);
pub const WSANOTINITIALISED: WSA_ERROR = WSA_ERROR(10093i32);
pub const WSAEDISCON: WSA_ERROR = WSA_ERROR(10101i32);
pub const WSAENOMORE: WSA_ERROR = WSA_ERROR(10102i32);
pub const WSAECANCELLED: WSA_ERROR = WSA_ERROR(10103i32);
pub const WSAEINVALIDPROCTABLE: WSA_ERROR = WSA_ERROR(10104i32);
pub const WSAEINVALIDPROVIDER: WSA_ERROR = WSA_ERROR(10105i32);
pub const WSAEPROVIDERFAILEDINIT: WSA_ERROR = WSA_ERROR(10106i32);
pub const WSASYSCALLFAILURE: WSA_ERROR = WSA_ERROR(10107i32);
pub const WSASERVICE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10108i32);
pub const WSATYPE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10109i32);
pub const WSA_E_NO_MORE: WSA_ERROR = WSA_ERROR(10110i32);
pub const WSA_E_CANCELLED: WSA_ERROR = WSA_ERROR(10111i32);
pub const WSAEREFUSED: WSA_ERROR = WSA_ERROR(10112i32);
pub const WSAHOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11001i32);
pub const WSATRY_AGAIN: WSA_ERROR = WSA_ERROR(11002i32);
pub const WSANO_RECOVERY: WSA_ERROR = WSA_ERROR(11003i32);
pub const WSANO_DATA: WSA_ERROR = WSA_ERROR(11004i32);
pub const WSA_QOS_RECEIVERS: WSA_ERROR = WSA_ERROR(11005i32);
pub const WSA_QOS_SENDERS: WSA_ERROR = WSA_ERROR(11006i32);
pub const WSA_QOS_NO_SENDERS: WSA_ERROR = WSA_ERROR(11007i32);
pub const WSA_QOS_NO_RECEIVERS: WSA_ERROR = WSA_ERROR(11008i32);
pub const WSA_QOS_REQUEST_CONFIRMED: WSA_ERROR = WSA_ERROR(11009i32);
pub const WSA_QOS_ADMISSION_FAILURE: WSA_ERROR = WSA_ERROR(11010i32);
pub const WSA_QOS_POLICY_FAILURE: WSA_ERROR = WSA_ERROR(11011i32);
pub const WSA_QOS_BAD_STYLE: WSA_ERROR = WSA_ERROR(11012i32);
pub const WSA_QOS_BAD_OBJECT: WSA_ERROR = WSA_ERROR(11013i32);
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: WSA_ERROR = WSA_ERROR(11014i32);
pub const WSA_QOS_GENERIC_ERROR: WSA_ERROR = WSA_ERROR(11015i32);
pub const WSA_QOS_ESERVICETYPE: WSA_ERROR = WSA_ERROR(11016i32);
pub const WSA_QOS_EFLOWSPEC: WSA_ERROR = WSA_ERROR(11017i32);
pub const WSA_QOS_EPROVSPECBUF: WSA_ERROR = WSA_ERROR(11018i32);
pub const WSA_QOS_EFILTERSTYLE: WSA_ERROR = WSA_ERROR(11019i32);
pub const WSA_QOS_EFILTERTYPE: WSA_ERROR = WSA_ERROR(11020i32);
pub const WSA_QOS_EFILTERCOUNT: WSA_ERROR = WSA_ERROR(11021i32);
pub const WSA_QOS_EOBJLENGTH: WSA_ERROR = WSA_ERROR(11022i32);
pub const WSA_QOS_EFLOWCOUNT: WSA_ERROR = WSA_ERROR(11023i32);
pub const WSA_QOS_EUNKOWNPSOBJ: WSA_ERROR = WSA_ERROR(11024i32);
pub const WSA_QOS_EPOLICYOBJ: WSA_ERROR = WSA_ERROR(11025i32);
pub const WSA_QOS_EFLOWDESC: WSA_ERROR = WSA_ERROR(11026i32);
pub const WSA_QOS_EPSFLOWSPEC: WSA_ERROR = WSA_ERROR(11027i32);
pub const WSA_QOS_EPSFILTERSPEC: WSA_ERROR = WSA_ERROR(11028i32);
pub const WSA_QOS_ESDMODEOBJ: WSA_ERROR = WSA_ERROR(11029i32);
pub const WSA_QOS_ESHAPERATEOBJ: WSA_ERROR = WSA_ERROR(11030i32);
pub const WSA_QOS_RESERVED_PETYPE: WSA_ERROR = WSA_ERROR(11031i32);
pub const WSA_SECURE_HOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11032i32);
pub const WSA_IPSEC_NAME_POLICY_ERROR: WSA_ERROR = WSA_ERROR(11033i32);
impl ::std::convert::From<i32> for WSA_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSA_ERROR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_MULTIPOINT_C_LEAF: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_MULTIPOINT_C_ROOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_MULTIPOINT_D_LEAF: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_MULTIPOINT_D_ROOT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_OVERLAPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_FLAG_REGISTERED_IO: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_INFINITE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_MAXIMUM_WAIT_EVENTS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_WAIT_EVENT_0: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_WAIT_FAILED: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSA_WAIT_IO_COMPLETION: u32 = 192u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCDeinstallProvider(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCDeinstallProvider(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCDeinstallProvider(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCDeinstallProvider32(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCDeinstallProvider32(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCDeinstallProvider32(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCEnableNSProvider<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpproviderid: *const ::windows::runtime::GUID, fenable: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnableNSProvider(lpproviderid: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(WSCEnableNSProvider(::std::mem::transmute(lpproviderid), fenable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCEnableNSProvider32<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpproviderid: *const ::windows::runtime::GUID, fenable: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnableNSProvider32(lpproviderid: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(WSCEnableNSProvider32(::std::mem::transmute(lpproviderid), fenable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCEnumNameSpaceProviders32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnumNameSpaceProviders32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
        }
        ::std::mem::transmute(WSCEnumNameSpaceProviders32(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSCEnumNameSpaceProvidersEx32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnumNameSpaceProvidersEx32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
        }
        ::std::mem::transmute(WSCEnumNameSpaceProvidersEx32(::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lpnspbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCEnumProtocols(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnumProtocols(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCEnumProtocols(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCEnumProtocols32(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCEnumProtocols32(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCEnumProtocols32(::std::mem::transmute(lpiprotocols), ::std::mem::transmute(lpprotocolbuffer), ::std::mem::transmute(lpdwbufferlength), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCGetApplicationCategory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(path: Param0, pathlength: u32, extra: Param2, extralength: u32, ppermittedlspcategories: *mut u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCGetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, ppermittedlspcategories: *mut u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCGetApplicationCategory(path.into_param().abi(), ::std::mem::transmute(pathlength), extra.into_param().abi(), ::std::mem::transmute(extralength), ::std::mem::transmute(ppermittedlspcategories), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCGetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCGetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCGetProviderInfo(::std::mem::transmute(lpproviderid), ::std::mem::transmute(infotype), ::std::mem::transmute(info), ::std::mem::transmute(infosize), ::std::mem::transmute(flags), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCGetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCGetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCGetProviderInfo32(::std::mem::transmute(lpproviderid), ::std::mem::transmute(infotype), ::std::mem::transmute(info), ::std::mem::transmute(infosize), ::std::mem::transmute(flags), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCGetProviderPath(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCGetProviderPath(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCGetProviderPath(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpszproviderdllpath), ::std::mem::transmute(lpproviderdllpathlen), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCGetProviderPath32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCGetProviderPath32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCGetProviderPath32(::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpszproviderdllpath), ::std::mem::transmute(lpproviderdllpathlen), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCInstallNameSpace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszidentifier: Param0, lpszpathname: Param1, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallNameSpace(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSCInstallNameSpace(lpszidentifier.into_param().abi(), lpszpathname.into_param().abi(), ::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwversion), ::std::mem::transmute(lpproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCInstallNameSpace32<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszidentifier: Param0, lpszpathname: Param1, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallNameSpace32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSCInstallNameSpace32(lpszidentifier.into_param().abi(), lpszpathname.into_param().abi(), ::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwversion), ::std::mem::transmute(lpproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSCInstallNameSpaceEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszidentifier: Param0, lpszpathname: Param1, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallNameSpaceEx(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
        }
        ::std::mem::transmute(WSCInstallNameSpaceEx(lpszidentifier.into_param().abi(), lpszpathname.into_param().abi(), ::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwversion), ::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpproviderspecific)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn WSCInstallNameSpaceEx32<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszidentifier: Param0, lpszpathname: Param1, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallNameSpaceEx32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
        }
        ::std::mem::transmute(WSCInstallNameSpaceEx32(lpszidentifier.into_param().abi(), lpszpathname.into_param().abi(), ::std::mem::transmute(dwnamespace), ::std::mem::transmute(dwversion), ::std::mem::transmute(lpproviderid), ::std::mem::transmute(lpproviderspecific)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCInstallProvider<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: Param1, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallProvider(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCInstallProvider(::std::mem::transmute(lpproviderid), lpszproviderdllpath.into_param().abi(), ::std::mem::transmute(lpprotocolinfolist), ::std::mem::transmute(dwnumberofentries), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCInstallProvider64_32<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: Param1, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallProvider64_32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCInstallProvider64_32(::std::mem::transmute(lpproviderid), lpszproviderdllpath.into_param().abi(), ::std::mem::transmute(lpprotocolinfolist), ::std::mem::transmute(dwnumberofentries), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCInstallProviderAndChains64_32<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    lpproviderid: *const ::windows::runtime::GUID,
    lpszproviderdllpath: Param1,
    lpszproviderdllpath32: Param2,
    lpszlspname: Param3,
    dwserviceflags: u32,
    lpprotocolinfolist: *mut WSAPROTOCOL_INFOW,
    dwnumberofentries: u32,
    lpdwcatalogentryid: *mut u32,
    lperrno: *mut i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCInstallProviderAndChains64_32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpszproviderdllpath32: super::super::Foundation::PWSTR, lpszlspname: super::super::Foundation::PWSTR, dwserviceflags: u32, lpprotocolinfolist: *mut WSAPROTOCOL_INFOW, dwnumberofentries: u32, lpdwcatalogentryid: *mut u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCInstallProviderAndChains64_32(
            ::std::mem::transmute(lpproviderid),
            lpszproviderdllpath.into_param().abi(),
            lpszproviderdllpath32.into_param().abi(),
            lpszlspname.into_param().abi(),
            ::std::mem::transmute(dwserviceflags),
            ::std::mem::transmute(lpprotocolinfolist),
            ::std::mem::transmute(dwnumberofentries),
            ::std::mem::transmute(lpdwcatalogentryid),
            ::std::mem::transmute(lperrno),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCSetApplicationCategory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(path: Param0, pathlength: u32, extra: Param2, extralength: u32, permittedlspcategories: u32, pprevpermlspcat: *mut u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCSetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, permittedlspcategories: u32, pprevpermlspcat: *mut u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCSetApplicationCategory(path.into_param().abi(), ::std::mem::transmute(pathlength), extra.into_param().abi(), ::std::mem::transmute(extralength), ::std::mem::transmute(permittedlspcategories), ::std::mem::transmute(pprevpermlspcat), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCSetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCSetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCSetProviderInfo(::std::mem::transmute(lpproviderid), ::std::mem::transmute(infotype), ::std::mem::transmute(info), ::std::mem::transmute(infosize), ::std::mem::transmute(flags), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCSetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCSetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCSetProviderInfo32(::std::mem::transmute(lpproviderid), ::std::mem::transmute(infotype), ::std::mem::transmute(info), ::std::mem::transmute(infosize), ::std::mem::transmute(flags), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCUnInstallNameSpace(lpproviderid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCUnInstallNameSpace(lpproviderid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSCUnInstallNameSpace(::std::mem::transmute(lpproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCUnInstallNameSpace32(lpproviderid: *const ::windows::runtime::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCUnInstallNameSpace32(lpproviderid: *const ::windows::runtime::GUID) -> i32;
        }
        ::std::mem::transmute(WSCUnInstallNameSpace32(::std::mem::transmute(lpproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCUpdateProvider<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: Param1, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCUpdateProvider(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCUpdateProvider(::std::mem::transmute(lpproviderid), lpszproviderdllpath.into_param().abi(), ::std::mem::transmute(lpprotocolinfolist), ::std::mem::transmute(dwnumberofentries), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WSCUpdateProvider32<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: Param1, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCUpdateProvider32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
        }
        ::std::mem::transmute(WSCUpdateProvider32(::std::mem::transmute(lpproviderid), lpszproviderdllpath.into_param().abi(), ::std::mem::transmute(lpprotocolinfolist), ::std::mem::transmute(dwnumberofentries), ::std::mem::transmute(lperrno)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCWriteNameSpaceOrder(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCWriteNameSpaceOrder(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32;
        }
        ::std::mem::transmute(WSCWriteNameSpaceOrder(::std::mem::transmute(lpproviderid), ::std::mem::transmute(dwnumberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCWriteNameSpaceOrder32(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCWriteNameSpaceOrder32(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32;
        }
        ::std::mem::transmute(WSCWriteNameSpaceOrder32(::std::mem::transmute(lpproviderid), ::std::mem::transmute(dwnumberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCWriteProviderOrder(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCWriteProviderOrder(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
        }
        ::std::mem::transmute(WSCWriteProviderOrder(::std::mem::transmute(lpwdcatalogentryid), ::std::mem::transmute(dwnumberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn WSCWriteProviderOrder32(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSCWriteProviderOrder32(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
        }
        ::std::mem::transmute(WSCWriteProviderOrder32(::std::mem::transmute(lpwdcatalogentryid), ::std::mem::transmute(dwnumberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSC_PROVIDER_AUDIT_INFO {
    pub RecordSize: u32,
    pub Reserved: *mut ::std::ffi::c_void,
}
impl WSC_PROVIDER_AUDIT_INFO {}
impl ::std::default::Default for WSC_PROVIDER_AUDIT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSC_PROVIDER_AUDIT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSC_PROVIDER_AUDIT_INFO").field("RecordSize", &self.RecordSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for WSC_PROVIDER_AUDIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RecordSize == other.RecordSize && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WSC_PROVIDER_AUDIT_INFO {}
unsafe impl ::windows::runtime::Abi for WSC_PROVIDER_AUDIT_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSC_PROVIDER_INFO_TYPE(pub i32);
pub const ProviderInfoLspCategories: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(0i32);
pub const ProviderInfoAudit: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(1i32);
impl ::std::convert::From<i32> for WSC_PROVIDER_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSC_PROVIDER_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSK_SO_BASE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSPDESCRIPTION_LEN: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct WSPData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [u16; 256],
}
impl WSPData {}
impl ::std::default::Default for WSPData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSPData {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSPData").field("wVersion", &self.wVersion).field("wHighVersion", &self.wHighVersion).field("szDescription", &self.szDescription).finish()
    }
}
impl ::std::cmp::PartialEq for WSPData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.szDescription == other.szDescription
    }
}
impl ::std::cmp::Eq for WSPData {}
unsafe impl ::windows::runtime::Abi for WSPData {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`, `Win32_System_IO`*"]
pub struct WSPPROC_TABLE {
    pub lpWSPAccept: ::std::option::Option<LPWSPACCEPT>,
    pub lpWSPAddressToString: ::std::option::Option<LPWSPADDRESSTOSTRING>,
    pub lpWSPAsyncSelect: ::std::option::Option<LPWSPASYNCSELECT>,
    pub lpWSPBind: ::std::option::Option<LPWSPBIND>,
    pub lpWSPCancelBlockingCall: ::std::option::Option<LPWSPCANCELBLOCKINGCALL>,
    pub lpWSPCleanup: ::std::option::Option<LPWSPCLEANUP>,
    pub lpWSPCloseSocket: ::std::option::Option<LPWSPCLOSESOCKET>,
    pub lpWSPConnect: ::std::option::Option<LPWSPCONNECT>,
    pub lpWSPDuplicateSocket: ::std::option::Option<LPWSPDUPLICATESOCKET>,
    pub lpWSPEnumNetworkEvents: ::std::option::Option<LPWSPENUMNETWORKEVENTS>,
    pub lpWSPEventSelect: ::std::option::Option<LPWSPEVENTSELECT>,
    pub lpWSPGetOverlappedResult: ::std::option::Option<LPWSPGETOVERLAPPEDRESULT>,
    pub lpWSPGetPeerName: ::std::option::Option<LPWSPGETPEERNAME>,
    pub lpWSPGetSockName: ::std::option::Option<LPWSPGETSOCKNAME>,
    pub lpWSPGetSockOpt: ::std::option::Option<LPWSPGETSOCKOPT>,
    pub lpWSPGetQOSByName: ::std::option::Option<LPWSPGETQOSBYNAME>,
    pub lpWSPIoctl: ::std::option::Option<LPWSPIOCTL>,
    pub lpWSPJoinLeaf: ::std::option::Option<LPWSPJOINLEAF>,
    pub lpWSPListen: ::std::option::Option<LPWSPLISTEN>,
    pub lpWSPRecv: ::std::option::Option<LPWSPRECV>,
    pub lpWSPRecvDisconnect: ::std::option::Option<LPWSPRECVDISCONNECT>,
    pub lpWSPRecvFrom: ::std::option::Option<LPWSPRECVFROM>,
    pub lpWSPSelect: ::std::option::Option<LPWSPSELECT>,
    pub lpWSPSend: ::std::option::Option<LPWSPSEND>,
    pub lpWSPSendDisconnect: ::std::option::Option<LPWSPSENDDISCONNECT>,
    pub lpWSPSendTo: ::std::option::Option<LPWSPSENDTO>,
    pub lpWSPSetSockOpt: ::std::option::Option<LPWSPSETSOCKOPT>,
    pub lpWSPShutdown: ::std::option::Option<LPWSPSHUTDOWN>,
    pub lpWSPSocket: ::std::option::Option<LPWSPSOCKET>,
    pub lpWSPStringToAddress: ::std::option::Option<LPWSPSTRINGTOADDRESS>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
impl WSPPROC_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
impl ::std::default::Default for WSPPROC_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WSPPROC_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSPPROC_TABLE").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WSPPROC_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.lpWSPAccept.map(|f| f as usize) == other.lpWSPAccept.map(|f| f as usize)
            && self.lpWSPAddressToString.map(|f| f as usize) == other.lpWSPAddressToString.map(|f| f as usize)
            && self.lpWSPAsyncSelect.map(|f| f as usize) == other.lpWSPAsyncSelect.map(|f| f as usize)
            && self.lpWSPBind.map(|f| f as usize) == other.lpWSPBind.map(|f| f as usize)
            && self.lpWSPCancelBlockingCall.map(|f| f as usize) == other.lpWSPCancelBlockingCall.map(|f| f as usize)
            && self.lpWSPCleanup.map(|f| f as usize) == other.lpWSPCleanup.map(|f| f as usize)
            && self.lpWSPCloseSocket.map(|f| f as usize) == other.lpWSPCloseSocket.map(|f| f as usize)
            && self.lpWSPConnect.map(|f| f as usize) == other.lpWSPConnect.map(|f| f as usize)
            && self.lpWSPDuplicateSocket.map(|f| f as usize) == other.lpWSPDuplicateSocket.map(|f| f as usize)
            && self.lpWSPEnumNetworkEvents.map(|f| f as usize) == other.lpWSPEnumNetworkEvents.map(|f| f as usize)
            && self.lpWSPEventSelect.map(|f| f as usize) == other.lpWSPEventSelect.map(|f| f as usize)
            && self.lpWSPGetOverlappedResult.map(|f| f as usize) == other.lpWSPGetOverlappedResult.map(|f| f as usize)
            && self.lpWSPGetPeerName.map(|f| f as usize) == other.lpWSPGetPeerName.map(|f| f as usize)
            && self.lpWSPGetSockName.map(|f| f as usize) == other.lpWSPGetSockName.map(|f| f as usize)
            && self.lpWSPGetSockOpt.map(|f| f as usize) == other.lpWSPGetSockOpt.map(|f| f as usize)
            && self.lpWSPGetQOSByName.map(|f| f as usize) == other.lpWSPGetQOSByName.map(|f| f as usize)
            && self.lpWSPIoctl.map(|f| f as usize) == other.lpWSPIoctl.map(|f| f as usize)
            && self.lpWSPJoinLeaf.map(|f| f as usize) == other.lpWSPJoinLeaf.map(|f| f as usize)
            && self.lpWSPListen.map(|f| f as usize) == other.lpWSPListen.map(|f| f as usize)
            && self.lpWSPRecv.map(|f| f as usize) == other.lpWSPRecv.map(|f| f as usize)
            && self.lpWSPRecvDisconnect.map(|f| f as usize) == other.lpWSPRecvDisconnect.map(|f| f as usize)
            && self.lpWSPRecvFrom.map(|f| f as usize) == other.lpWSPRecvFrom.map(|f| f as usize)
            && self.lpWSPSelect.map(|f| f as usize) == other.lpWSPSelect.map(|f| f as usize)
            && self.lpWSPSend.map(|f| f as usize) == other.lpWSPSend.map(|f| f as usize)
            && self.lpWSPSendDisconnect.map(|f| f as usize) == other.lpWSPSendDisconnect.map(|f| f as usize)
            && self.lpWSPSendTo.map(|f| f as usize) == other.lpWSPSendTo.map(|f| f as usize)
            && self.lpWSPSetSockOpt.map(|f| f as usize) == other.lpWSPSetSockOpt.map(|f| f as usize)
            && self.lpWSPShutdown.map(|f| f as usize) == other.lpWSPShutdown.map(|f| f as usize)
            && self.lpWSPSocket.map(|f| f as usize) == other.lpWSPSocket.map(|f| f as usize)
            && self.lpWSPStringToAddress.map(|f| f as usize) == other.lpWSPStringToAddress.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WSPPROC_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WSPPROC_TABLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct WSPUPCALLTABLE {
    pub lpWPUCloseEvent: ::std::option::Option<LPWPUCLOSEEVENT>,
    pub lpWPUCloseSocketHandle: ::std::option::Option<LPWPUCLOSESOCKETHANDLE>,
    pub lpWPUCreateEvent: ::std::option::Option<LPWPUCREATEEVENT>,
    pub lpWPUCreateSocketHandle: ::std::option::Option<LPWPUCREATESOCKETHANDLE>,
    pub lpWPUFDIsSet: ::std::option::Option<LPWPUFDISSET>,
    pub lpWPUGetProviderPath: ::std::option::Option<LPWPUGETPROVIDERPATH>,
    pub lpWPUModifyIFSHandle: ::std::option::Option<LPWPUMODIFYIFSHANDLE>,
    pub lpWPUPostMessage: ::std::option::Option<LPWPUPOSTMESSAGE>,
    pub lpWPUQueryBlockingCallback: ::std::option::Option<LPWPUQUERYBLOCKINGCALLBACK>,
    pub lpWPUQuerySocketHandleContext: ::std::option::Option<LPWPUQUERYSOCKETHANDLECONTEXT>,
    pub lpWPUQueueApc: ::std::option::Option<LPWPUQUEUEAPC>,
    pub lpWPUResetEvent: ::std::option::Option<LPWPURESETEVENT>,
    pub lpWPUSetEvent: ::std::option::Option<LPWPUSETEVENT>,
    pub lpWPUOpenCurrentThread: ::std::option::Option<LPWPUOPENCURRENTTHREAD>,
    pub lpWPUCloseThread: ::std::option::Option<LPWPUCLOSETHREAD>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSPUPCALLTABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSPUPCALLTABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSPUPCALLTABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSPUPCALLTABLE").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSPUPCALLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.lpWPUCloseEvent.map(|f| f as usize) == other.lpWPUCloseEvent.map(|f| f as usize)
            && self.lpWPUCloseSocketHandle.map(|f| f as usize) == other.lpWPUCloseSocketHandle.map(|f| f as usize)
            && self.lpWPUCreateEvent.map(|f| f as usize) == other.lpWPUCreateEvent.map(|f| f as usize)
            && self.lpWPUCreateSocketHandle.map(|f| f as usize) == other.lpWPUCreateSocketHandle.map(|f| f as usize)
            && self.lpWPUFDIsSet.map(|f| f as usize) == other.lpWPUFDIsSet.map(|f| f as usize)
            && self.lpWPUGetProviderPath.map(|f| f as usize) == other.lpWPUGetProviderPath.map(|f| f as usize)
            && self.lpWPUModifyIFSHandle.map(|f| f as usize) == other.lpWPUModifyIFSHandle.map(|f| f as usize)
            && self.lpWPUPostMessage.map(|f| f as usize) == other.lpWPUPostMessage.map(|f| f as usize)
            && self.lpWPUQueryBlockingCallback.map(|f| f as usize) == other.lpWPUQueryBlockingCallback.map(|f| f as usize)
            && self.lpWPUQuerySocketHandleContext.map(|f| f as usize) == other.lpWPUQuerySocketHandleContext.map(|f| f as usize)
            && self.lpWPUQueueApc.map(|f| f as usize) == other.lpWPUQueueApc.map(|f| f as usize)
            && self.lpWPUResetEvent.map(|f| f as usize) == other.lpWPUResetEvent.map(|f| f as usize)
            && self.lpWPUSetEvent.map(|f| f as usize) == other.lpWPUSetEvent.map(|f| f as usize)
            && self.lpWPUOpenCurrentThread.map(|f| f as usize) == other.lpWPUOpenCurrentThread.map(|f| f as usize)
            && self.lpWPUCloseThread.map(|f| f as usize) == other.lpWPUCloseThread.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSPUPCALLTABLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSPUPCALLTABLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const WSS_OPERATION_IN_PROGRESS: i32 = 259i32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_CONNECTIONLESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_CONNECT_DATA: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_DISCONNECT_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_EXPEDITED_DATA: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_GRACEFUL_CLOSE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_GUARANTEED_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_GUARANTEED_ORDER: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_IFS_HANDLES: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_INTERRUPT: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_MESSAGE_ORIENTED: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_MULTIPOINT_CONTROL_PLANE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_MULTIPOINT_DATA_PLANE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_PARTIAL_MESSAGE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_PSEUDO_STREAM: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_QOS_SUPPORTED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_SAN_SUPPORT_SDP: u32 = 524288u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_SUPPORT_BROADCAST: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_SUPPORT_MULTIPOINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_UNI_RECV: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP1_UNI_SEND: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_BANDWIDTH_ALLOCATION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_CONNECTIONLESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_CONNECT_DATA: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_DISCONNECT_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_ENCRYPTS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_EXPEDITED_DATA: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_FRAGMENTATION: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_GRACEFUL_CLOSE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_GUARANTEED_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_GUARANTEED_ORDER: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_MESSAGE_ORIENTED: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_PSEUDO_STREAM: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_SUPPORTS_BROADCAST: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const XP_SUPPORTS_MULTICAST: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub const _SS_MAXSIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn __WSAFDIsSet<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(fd: Param0, param1: *mut fd_set) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32;
        }
        ::std::mem::transmute(__WSAFDIsSet(fd.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn accept<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn accept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET;
        }
        ::std::mem::transmute(accept(s.into_param().abi(), ::std::mem::transmute(addr), ::std::mem::transmute(addrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut addrinfoW,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoW")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfo_dns_server {
    pub ai_servertype: u32,
    pub ai_flags: u64,
    pub ai_addrlen: u32,
    pub ai_addr: *mut SOCKADDR,
    pub Anonymous: addrinfo_dns_server_0,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfo_dns_server {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfo_dns_server {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfo_dns_server {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfo_dns_server {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfo_dns_server {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union addrinfo_dns_server_0 {
    pub ai_template: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfo_dns_server_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfo_dns_server_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfo_dns_server_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfo_dns_server_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfo_dns_server_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex2A {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex2A,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex2A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex2A")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex2A {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex2A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex2W {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex2W,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex2W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex2W")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex2W {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex2W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex2W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex3 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex3,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PWSTR,
    pub ai_interfaceindex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex3")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex3 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex4 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex4,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex4")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex4 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex4 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex4 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex5 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex5,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
    pub ai_ttl: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex5")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex5 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
            && self.ai_ttl == other.ai_ttl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex5 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex5 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoex6 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoex5,
    pub ai_version: i32,
    pub ai_fqdn: super::super::Foundation::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut addrinfo_dns_server,
    pub ai_responseflags: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoex6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoex6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoex6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoex6")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .field("ai_numservers", &self.ai_numservers)
            .field("ai_servers", &self.ai_servers)
            .field("ai_responseflags", &self.ai_responseflags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoex6 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
            && self.ai_ttl == other.ai_ttl
            && self.ai_numservers == other.ai_numservers
            && self.ai_servers == other.ai_servers
            && self.ai_responseflags == other.ai_responseflags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoex6 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoex6 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoexA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoexA,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoexA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoexA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoexA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoexA")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoexA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoexA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoexA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct addrinfoexW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: super::super::Foundation::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::std::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows::runtime::GUID,
    pub ai_next: *mut addrinfoexW,
}
#[cfg(feature = "Win32_Foundation")]
impl addrinfoexW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for addrinfoexW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for addrinfoexW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("addrinfoexW")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for addrinfoexW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for addrinfoexW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for addrinfoexW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn bind<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *const SOCKADDR, namelen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
        }
        ::std::mem::transmute(bind(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn closesocket<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn closesocket(s: SOCKET) -> i32;
        }
        ::std::mem::transmute(closesocket(s.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
impl cmsghdr {}
impl ::std::default::Default for cmsghdr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for cmsghdr {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("cmsghdr").field("cmsg_len", &self.cmsg_len).field("cmsg_level", &self.cmsg_level).field("cmsg_type", &self.cmsg_type).finish()
    }
}
impl ::std::cmp::PartialEq for cmsghdr {
    fn eq(&self, other: &Self) -> bool {
        self.cmsg_len == other.cmsg_len && self.cmsg_level == other.cmsg_level && self.cmsg_type == other.cmsg_type
    }
}
impl ::std::cmp::Eq for cmsghdr {}
unsafe impl ::windows::runtime::Abi for cmsghdr {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn connect<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *const SOCKADDR, namelen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
        }
        ::std::mem::transmute(connect(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct eWINDOW_ADVANCE_METHOD(pub i32);
pub const E_WINDOW_ADVANCE_BY_TIME: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(1i32);
pub const E_WINDOW_USE_AS_DATA_CACHE: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(2i32);
impl ::std::convert::From<i32> for eWINDOW_ADVANCE_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for eWINDOW_ADVANCE_METHOD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct fd_set {
    pub fd_count: u32,
    pub fd_array: [SOCKET; 64],
}
impl fd_set {}
impl ::std::default::Default for fd_set {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for fd_set {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("fd_set").field("fd_count", &self.fd_count).field("fd_array", &self.fd_array).finish()
    }
}
impl ::std::cmp::PartialEq for fd_set {
    fn eq(&self, other: &Self) -> bool {
        self.fd_count == other.fd_count && self.fd_array == other.fd_array
    }
}
impl ::std::cmp::Eq for fd_set {}
unsafe impl ::windows::runtime::Abi for fd_set {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn freeaddrinfo(paddrinfo: *const ADDRINFOA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn freeaddrinfo(paddrinfo: *const ADDRINFOA);
        }
        ::std::mem::transmute(freeaddrinfo(::std::mem::transmute(paddrinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getaddrinfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pnodename: Param0, pservicename: Param1, phints: *const ADDRINFOA, ppresult: *mut *mut ADDRINFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getaddrinfo(pnodename: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, phints: *const ADDRINFOA, ppresult: *mut *mut ADDRINFOA) -> i32;
        }
        ::std::mem::transmute(getaddrinfo(pnodename.into_param().abi(), pservicename.into_param().abi(), ::std::mem::transmute(phints), ::std::mem::transmute(ppresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn gethostbyaddr<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(addr: Param0, len: i32, r#type: i32) -> *mut hostent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gethostbyaddr(addr: super::super::Foundation::PSTR, len: i32, r#type: i32) -> *mut hostent;
        }
        ::std::mem::transmute(gethostbyaddr(addr.into_param().abi(), ::std::mem::transmute(len), ::std::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn gethostbyname<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(name: Param0) -> *mut hostent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gethostbyname(name: super::super::Foundation::PSTR) -> *mut hostent;
        }
        ::std::mem::transmute(gethostbyname(name.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn gethostname(name: super::super::Foundation::PSTR, namelen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gethostname(name: super::super::Foundation::PSTR, namelen: i32) -> i32;
        }
        ::std::mem::transmute(gethostname(::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PSTR, servicebuffersize: u32, flags: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PSTR, servicebuffersize: u32, flags: i32) -> i32;
        }
        ::std::mem::transmute(getnameinfo(::std::mem::transmute(psockaddr), ::std::mem::transmute(sockaddrlength), ::std::mem::transmute(pnodebuffer), ::std::mem::transmute(nodebuffersize), ::std::mem::transmute(pservicebuffer), ::std::mem::transmute(servicebuffersize), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getpeername<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *mut SOCKADDR, namelen: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getpeername(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
        }
        ::std::mem::transmute(getpeername(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getprotobyname<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(name: Param0) -> *mut protoent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getprotobyname(name: super::super::Foundation::PSTR) -> *mut protoent;
        }
        ::std::mem::transmute(getprotobyname(name.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getprotobynumber(number: i32) -> *mut protoent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getprotobynumber(number: i32) -> *mut protoent;
        }
        ::std::mem::transmute(getprotobynumber(::std::mem::transmute(number)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getservbyname<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(name: Param0, proto: Param1) -> *mut servent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getservbyname(name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR) -> *mut servent;
        }
        ::std::mem::transmute(getservbyname(name.into_param().abi(), proto.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getservbyport<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(port: i32, proto: Param1) -> *mut servent {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getservbyport(port: i32, proto: super::super::Foundation::PSTR) -> *mut servent;
        }
        ::std::mem::transmute(getservbyport(::std::mem::transmute(port), proto.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getsockname<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, name: *mut SOCKADDR, namelen: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getsockname(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
        }
        ::std::mem::transmute(getsockname(s.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(namelen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn getsockopt<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn getsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32) -> i32;
        }
        ::std::mem::transmute(getsockopt(s.into_param().abi(), ::std::mem::transmute(level), ::std::mem::transmute(optname), ::std::mem::transmute(optval), ::std::mem::transmute(optlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct hostent {
    pub h_name: super::super::Foundation::PSTR,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i16,
    pub h_length: i16,
    pub h_addr_list: *mut *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl hostent {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for hostent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for hostent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("hostent").field("h_name", &self.h_name).field("h_aliases", &self.h_aliases).field("h_addrtype", &self.h_addrtype).field("h_length", &self.h_length).field("h_addr_list", &self.h_addr_list).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for hostent {
    fn eq(&self, other: &Self) -> bool {
        self.h_name == other.h_name && self.h_aliases == other.h_aliases && self.h_addrtype == other.h_addrtype && self.h_length == other.h_length && self.h_addr_list == other.h_addr_list
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for hostent {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for hostent {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn htonl(hostlong: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn htonl(hostlong: u32) -> u32;
        }
        ::std::mem::transmute(htonl(::std::mem::transmute(hostlong)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn htons(hostshort: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn htons(hostshort: u16) -> u16;
        }
        ::std::mem::transmute(htons(::std::mem::transmute(hostshort)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct in6_pktinfo_ex {
    pub pkt_info: IN6_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl in6_pktinfo_ex {}
impl ::std::default::Default for in6_pktinfo_ex {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for in6_pktinfo_ex {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for in6_pktinfo_ex {}
unsafe impl ::windows::runtime::Abi for in6_pktinfo_ex {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn inet_addr<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(cp: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn inet_addr(cp: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(inet_addr(cp.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn inet_ntoa<'a, Param0: ::windows::runtime::IntoParam<'a, IN_ADDR>>(r#in: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn inet_ntoa(r#in: IN_ADDR) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(inet_ntoa(r#in.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn inet_ntop(family: i32, paddr: *const ::std::ffi::c_void, pstringbuf: super::super::Foundation::PSTR, stringbufsize: usize) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn inet_ntop(family: i32, paddr: *const ::std::ffi::c_void, pstringbuf: super::super::Foundation::PSTR, stringbufsize: usize) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(inet_ntop(::std::mem::transmute(family), ::std::mem::transmute(paddr), ::std::mem::transmute(pstringbuf), ::std::mem::transmute(stringbufsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn inet_pton<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(family: i32, pszaddrstring: Param1, paddrbuf: *mut ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn inet_pton(family: i32, pszaddrstring: super::super::Foundation::PSTR, paddrbuf: *mut ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(inet_pton(::std::mem::transmute(family), pszaddrstring.into_param().abi(), ::std::mem::transmute(paddrbuf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn ioctlsocket<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, cmd: i32, argp: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
        }
        ::std::mem::transmute(ioctlsocket(s.into_param().abi(), ::std::mem::transmute(cmd), ::std::mem::transmute(argp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct linger {
    pub l_onoff: u16,
    pub l_linger: u16,
}
impl linger {}
impl ::std::default::Default for linger {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for linger {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("linger").field("l_onoff", &self.l_onoff).field("l_linger", &self.l_linger).finish()
    }
}
impl ::std::cmp::PartialEq for linger {
    fn eq(&self, other: &Self) -> bool {
        self.l_onoff == other.l_onoff && self.l_linger == other.l_linger
    }
}
impl ::std::cmp::Eq for linger {}
unsafe impl ::windows::runtime::Abi for linger {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn listen<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, backlog: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn listen(s: SOCKET, backlog: i32) -> i32;
        }
        ::std::mem::transmute(listen(s.into_param().abi(), ::std::mem::transmute(backlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct netent {
    pub n_name: super::super::Foundation::PSTR,
    pub n_aliases: *mut *mut i8,
    pub n_addrtype: i16,
    pub n_net: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl netent {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for netent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for netent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("netent").field("n_name", &self.n_name).field("n_aliases", &self.n_aliases).field("n_addrtype", &self.n_addrtype).field("n_net", &self.n_net).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for netent {
    fn eq(&self, other: &Self) -> bool {
        self.n_name == other.n_name && self.n_aliases == other.n_aliases && self.n_addrtype == other.n_addrtype && self.n_net == other.n_net
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for netent {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for netent {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn ntohl(netlong: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ntohl(netlong: u32) -> u32;
        }
        ::std::mem::transmute(ntohl(::std::mem::transmute(netlong)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn ntohs(netshort: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ntohs(netshort: u16) -> u16;
        }
        ::std::mem::transmute(ntohs(::std::mem::transmute(netshort)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct protoent {
    pub p_name: super::super::Foundation::PSTR,
    pub p_aliases: *mut *mut i8,
    pub p_proto: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl protoent {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for protoent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for protoent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("protoent").field("p_name", &self.p_name).field("p_aliases", &self.p_aliases).field("p_proto", &self.p_proto).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for protoent {
    fn eq(&self, other: &Self) -> bool {
        self.p_name == other.p_name && self.p_aliases == other.p_aliases && self.p_proto == other.p_proto
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for protoent {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for protoent {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn recv<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, buf: super::super::Foundation::PSTR, len: i32, flags: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn recv(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32) -> i32;
        }
        ::std::mem::transmute(recv(s.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(len), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn recvfrom<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, buf: super::super::Foundation::PSTR, len: i32, flags: i32, from: *mut SOCKADDR, fromlen: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn recvfrom(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, from: *mut SOCKADDR, fromlen: *mut i32) -> i32;
        }
        ::std::mem::transmute(recvfrom(s.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(len), ::std::mem::transmute(flags), ::std::mem::transmute(from), ::std::mem::transmute(fromlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn select(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn select(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval) -> i32;
        }
        ::std::mem::transmute(select(::std::mem::transmute(nfds), ::std::mem::transmute(readfds), ::std::mem::transmute(writefds), ::std::mem::transmute(exceptfds), ::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn send<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(s: Param0, buf: Param1, len: i32, flags: SEND_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn send(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: SEND_FLAGS) -> i32;
        }
        ::std::mem::transmute(send(s.into_param().abi(), buf.into_param().abi(), ::std::mem::transmute(len), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn sendto<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(s: Param0, buf: Param1, len: i32, flags: i32, to: *const SOCKADDR, tolen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sendto(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, to: *const SOCKADDR, tolen: i32) -> i32;
        }
        ::std::mem::transmute(sendto(s.into_param().abi(), buf.into_param().abi(), ::std::mem::transmute(len), ::std::mem::transmute(flags), ::std::mem::transmute(to), ::std::mem::transmute(tolen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct servent {
    pub s_name: super::super::Foundation::PSTR,
    pub s_aliases: *mut *mut i8,
    pub s_proto: super::super::Foundation::PSTR,
    pub s_port: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl servent {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for servent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for servent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("servent").field("s_name", &self.s_name).field("s_aliases", &self.s_aliases).field("s_proto", &self.s_proto).field("s_port", &self.s_port).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for servent {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name && self.s_aliases == other.s_aliases && self.s_proto == other.s_proto && self.s_port == other.s_port
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for servent {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for servent {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct servent {
    pub s_name: super::super::Foundation::PSTR,
    pub s_aliases: *mut *mut i8,
    pub s_port: i16,
    pub s_proto: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl servent {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for servent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for servent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("servent").field("s_name", &self.s_name).field("s_aliases", &self.s_aliases).field("s_port", &self.s_port).field("s_proto", &self.s_proto).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for servent {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name && self.s_aliases == other.s_aliases && self.s_port == other.s_port && self.s_proto == other.s_proto
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for servent {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for servent {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn setsockopt<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(s: Param0, level: i32, optname: i32, optval: Param3, optlen: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: i32) -> i32;
        }
        ::std::mem::transmute(setsockopt(s.into_param().abi(), ::std::mem::transmute(level), ::std::mem::transmute(optname), optval.into_param().abi(), ::std::mem::transmute(optlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn shutdown<'a, Param0: ::windows::runtime::IntoParam<'a, SOCKET>>(s: Param0, how: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn shutdown(s: SOCKET, how: i32) -> i32;
        }
        ::std::mem::transmute(shutdown(s.into_param().abi(), ::std::mem::transmute(how)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct sockaddr_atm {
    pub satm_family: u16,
    pub satm_number: ATM_ADDRESS,
    pub satm_blli: ATM_BLLI,
    pub satm_bhli: ATM_BHLI,
}
impl sockaddr_atm {}
impl ::std::default::Default for sockaddr_atm {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for sockaddr_atm {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_atm").field("satm_family", &self.satm_family).field("satm_number", &self.satm_number).field("satm_blli", &self.satm_blli).field("satm_bhli", &self.satm_bhli).finish()
    }
}
impl ::std::cmp::PartialEq for sockaddr_atm {
    fn eq(&self, other: &Self) -> bool {
        self.satm_family == other.satm_family && self.satm_number == other.satm_number && self.satm_blli == other.satm_blli && self.satm_bhli == other.satm_bhli
    }
}
impl ::std::cmp::Eq for sockaddr_atm {}
unsafe impl ::windows::runtime::Abi for sockaddr_atm {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub union sockaddr_gen {
    pub Address: SOCKADDR,
    pub AddressIn: SOCKADDR_IN,
    pub AddressIn6: sockaddr_in6_old,
}
#[cfg(feature = "Win32_Foundation")]
impl sockaddr_gen {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for sockaddr_gen {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for sockaddr_gen {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for sockaddr_gen {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for sockaddr_gen {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct sockaddr_in6_old {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
}
impl sockaddr_in6_old {}
impl ::std::default::Default for sockaddr_in6_old {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for sockaddr_in6_old {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for sockaddr_in6_old {}
unsafe impl ::windows::runtime::Abi for sockaddr_in6_old {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct sockaddr_ipx {
    pub sa_family: i16,
    pub sa_netnum: [super::super::Foundation::CHAR; 4],
    pub sa_nodenum: [super::super::Foundation::CHAR; 6],
    pub sa_socket: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl sockaddr_ipx {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for sockaddr_ipx {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for sockaddr_ipx {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_ipx").field("sa_family", &self.sa_family).field("sa_netnum", &self.sa_netnum).field("sa_nodenum", &self.sa_nodenum).field("sa_socket", &self.sa_socket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for sockaddr_ipx {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family && self.sa_netnum == other.sa_netnum && self.sa_nodenum == other.sa_nodenum && self.sa_socket == other.sa_socket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for sockaddr_ipx {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for sockaddr_ipx {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct sockaddr_nb {
    pub snb_family: i16,
    pub snb_type: u16,
    pub snb_name: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl sockaddr_nb {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for sockaddr_nb {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for sockaddr_nb {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_nb").field("snb_family", &self.snb_family).field("snb_type", &self.snb_type).field("snb_name", &self.snb_name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for sockaddr_nb {
    fn eq(&self, other: &Self) -> bool {
        self.snb_family == other.snb_family && self.snb_type == other.snb_type && self.snb_name == other.snb_name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for sockaddr_nb {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for sockaddr_nb {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct sockaddr_tp {
    pub tp_family: u16,
    pub tp_addr_type: u16,
    pub tp_taddr_len: u16,
    pub tp_tsel_len: u16,
    pub tp_addr: [u8; 64],
}
impl sockaddr_tp {}
impl ::std::default::Default for sockaddr_tp {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for sockaddr_tp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_tp").field("tp_family", &self.tp_family).field("tp_addr_type", &self.tp_addr_type).field("tp_taddr_len", &self.tp_taddr_len).field("tp_tsel_len", &self.tp_tsel_len).field("tp_addr", &self.tp_addr).finish()
    }
}
impl ::std::cmp::PartialEq for sockaddr_tp {
    fn eq(&self, other: &Self) -> bool {
        self.tp_family == other.tp_family && self.tp_addr_type == other.tp_addr_type && self.tp_taddr_len == other.tp_taddr_len && self.tp_tsel_len == other.tp_tsel_len && self.tp_addr == other.tp_addr
    }
}
impl ::std::cmp::Eq for sockaddr_tp {}
unsafe impl ::windows::runtime::Abi for sockaddr_tp {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [super::super::Foundation::CHAR; 108],
}
#[cfg(feature = "Win32_Foundation")]
impl sockaddr_un {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for sockaddr_un {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for sockaddr_un {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_un").field("sun_family", &self.sun_family).field("sun_path", &self.sun_path).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for sockaddr_un {
    fn eq(&self, other: &Self) -> bool {
        self.sun_family == other.sun_family && self.sun_path == other.sun_path
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for sockaddr_un {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for sockaddr_un {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct sockaddr_vns {
    pub sin_family: u16,
    pub net_address: [u8; 4],
    pub subnet_addr: [u8; 2],
    pub port: [u8; 2],
    pub hops: u8,
    pub filler: [u8; 5],
}
impl sockaddr_vns {}
impl ::std::default::Default for sockaddr_vns {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for sockaddr_vns {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockaddr_vns").field("sin_family", &self.sin_family).field("net_address", &self.net_address).field("subnet_addr", &self.subnet_addr).field("port", &self.port).field("hops", &self.hops).field("filler", &self.filler).finish()
    }
}
impl ::std::cmp::PartialEq for sockaddr_vns {
    fn eq(&self, other: &Self) -> bool {
        self.sin_family == other.sin_family && self.net_address == other.net_address && self.subnet_addr == other.subnet_addr && self.port == other.port && self.hops == other.hops && self.filler == other.filler
    }
}
impl ::std::cmp::Eq for sockaddr_vns {}
unsafe impl ::windows::runtime::Abi for sockaddr_vns {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
#[inline]
pub unsafe fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET;
        }
        ::std::mem::transmute(socket(::std::mem::transmute(af), ::std::mem::transmute(r#type), ::std::mem::transmute(protocol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct sockproto {
    pub sp_family: u16,
    pub sp_protocol: u16,
}
impl sockproto {}
impl ::std::default::Default for sockproto {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for sockproto {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("sockproto").field("sp_family", &self.sp_family).field("sp_protocol", &self.sp_protocol).finish()
    }
}
impl ::std::cmp::PartialEq for sockproto {
    fn eq(&self, other: &Self) -> bool {
        self.sp_family == other.sp_family && self.sp_protocol == other.sp_protocol
    }
}
impl ::std::cmp::Eq for sockproto {}
unsafe impl ::windows::runtime::Abi for sockproto {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct tcp_keepalive {
    pub onoff: u32,
    pub keepalivetime: u32,
    pub keepaliveinterval: u32,
}
impl tcp_keepalive {}
impl ::std::default::Default for tcp_keepalive {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_keepalive {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_keepalive").field("onoff", &self.onoff).field("keepalivetime", &self.keepalivetime).field("keepaliveinterval", &self.keepaliveinterval).finish()
    }
}
impl ::std::cmp::PartialEq for tcp_keepalive {
    fn eq(&self, other: &Self) -> bool {
        self.onoff == other.onoff && self.keepalivetime == other.keepalivetime && self.keepaliveinterval == other.keepaliveinterval
    }
}
impl ::std::cmp::Eq for tcp_keepalive {}
unsafe impl ::windows::runtime::Abi for tcp_keepalive {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_WinSock`*"]
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl timeval {}
impl ::std::default::Default for timeval {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for timeval {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("timeval").field("tv_sec", &self.tv_sec).field("tv_usec", &self.tv_usec).finish()
    }
}
impl ::std::cmp::PartialEq for timeval {
    fn eq(&self, other: &Self) -> bool {
        self.tv_sec == other.tv_sec && self.tv_usec == other.tv_usec
    }
}
impl ::std::cmp::Eq for timeval {}
unsafe impl ::windows::runtime::Abi for timeval {
    type Abi = Self;
}
