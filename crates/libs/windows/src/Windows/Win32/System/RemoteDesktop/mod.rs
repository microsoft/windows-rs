#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AAAccountingData {
    pub userName: super::super::Foundation::BSTR,
    pub clientName: super::super::Foundation::BSTR,
    pub authType: AAAuthSchemes,
    pub resourceName: super::super::Foundation::BSTR,
    pub portNumber: i32,
    pub protocolName: super::super::Foundation::BSTR,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: super::super::Foundation::BSTR,
    pub mainSessionId: ::windows::core::GUID,
    pub subSessionId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AAAccountingData {
    fn clone(&self) -> Self {
        Self {
            userName: self.userName.clone(),
            clientName: self.clientName.clone(),
            authType: self.authType,
            resourceName: self.resourceName.clone(),
            portNumber: self.portNumber,
            protocolName: self.protocolName.clone(),
            numberOfBytesReceived: self.numberOfBytesReceived,
            numberOfBytesTransfered: self.numberOfBytesTransfered,
            reasonForDisconnect: self.reasonForDisconnect.clone(),
            mainSessionId: self.mainSessionId,
            subSessionId: self.subSessionId,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AAAccountingData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAAccountingData")
            .field("userName", &self.userName)
            .field("clientName", &self.clientName)
            .field("authType", &self.authType)
            .field("resourceName", &self.resourceName)
            .field("portNumber", &self.portNumber)
            .field("protocolName", &self.protocolName)
            .field("numberOfBytesReceived", &self.numberOfBytesReceived)
            .field("numberOfBytesTransfered", &self.numberOfBytesTransfered)
            .field("reasonForDisconnect", &self.reasonForDisconnect)
            .field("mainSessionId", &self.mainSessionId)
            .field("subSessionId", &self.subSessionId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AAAccountingData {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AAAccountingData {
    fn eq(&self, other: &Self) -> bool {
        self.userName == other.userName && self.clientName == other.clientName && self.authType == other.authType && self.resourceName == other.resourceName && self.portNumber == other.portNumber && self.protocolName == other.protocolName && self.numberOfBytesReceived == other.numberOfBytesReceived && self.numberOfBytesTransfered == other.numberOfBytesTransfered && self.reasonForDisconnect == other.reasonForDisconnect && self.mainSessionId == other.mainSessionId && self.subSessionId == other.subSessionId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AAAccountingData {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AAAccountingData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AAAccountingDataType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(3i32);
impl ::core::marker::Copy for AAAccountingDataType {}
impl ::core::clone::Clone for AAAccountingDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AAAccountingDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AAAccountingDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AAAccountingDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAccountingDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AAAuthSchemes(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_MIN: AAAuthSchemes = AAAuthSchemes(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_BASIC: AAAuthSchemes = AAAuthSchemes(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_NTLM: AAAuthSchemes = AAAuthSchemes(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_SC: AAAuthSchemes = AAAuthSchemes(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = AAAuthSchemes(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = AAAuthSchemes(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_ANY: AAAuthSchemes = AAAuthSchemes(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_COOKIE: AAAuthSchemes = AAAuthSchemes(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_DIGEST: AAAuthSchemes = AAAuthSchemes(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_ORGID: AAAuthSchemes = AAAuthSchemes(9i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_CONID: AAAuthSchemes = AAAuthSchemes(10i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = AAAuthSchemes(11i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_MAX: AAAuthSchemes = AAAuthSchemes(12i32);
impl ::core::marker::Copy for AAAuthSchemes {}
impl ::core::clone::Clone for AAAuthSchemes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AAAuthSchemes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AAAuthSchemes {
    type Abi = Self;
}
impl ::core::fmt::Debug for AAAuthSchemes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAuthSchemes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AATrustClassID(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_UNTRUSTED: AATrustClassID = AATrustClassID(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = AATrustClassID(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = AATrustClassID(2i32);
impl ::core::marker::Copy for AATrustClassID {}
impl ::core::clone::Clone for AATrustClassID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AATrustClassID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AATrustClassID {
    type Abi = Self;
}
impl ::core::fmt::Debug for AATrustClassID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AATrustClassID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2e9cae6_1e7b_4b8e_babd_e9bf6292ac29);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl ::core::marker::Copy for AE_CURRENT_POSITION {}
impl ::core::clone::Clone for AE_CURRENT_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CURRENT_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CURRENT_POSITION").field("u64DevicePosition", &self.u64DevicePosition).field("u64StreamPosition", &self.u64StreamPosition).field("u64PaddingFrames", &self.u64PaddingFrames).field("hnsQPCPosition", &self.hnsQPCPosition).field("f32FramesPerSecond", &self.f32FramesPerSecond).field("Flag", &self.Flag).finish()
    }
}
unsafe impl ::windows::core::Abi for AE_CURRENT_POSITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_CURRENT_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_CURRENT_POSITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_CURRENT_POSITION {}
impl ::core::default::Default for AE_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AE_POSITION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_INVALID: AE_POSITION_FLAGS = AE_POSITION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = AE_POSITION_FLAGS(4i32);
impl ::core::marker::Copy for AE_POSITION_FLAGS {}
impl ::core::clone::Clone for AE_POSITION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AE_POSITION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AE_POSITION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AE_POSITION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AE_POSITION_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl ::core::marker::Copy for BITMAP_RENDERER_STATISTICS {}
impl ::core::clone::Clone for BITMAP_RENDERER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAP_RENDERER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP_RENDERER_STATISTICS").field("dwFramesDelivered", &self.dwFramesDelivered).field("dwFramesDropped", &self.dwFramesDropped).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAP_RENDERER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAP_RENDERER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAP_RENDERER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAP_RENDERER_STATISTICS {}
impl ::core::default::Default for BITMAP_RENDERER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_DEF {
    pub name: [super::super::Foundation::CHAR; 8],
    pub options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANNEL_DEF {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANNEL_DEF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANNEL_DEF>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_ENTRY_POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANNEL_ENTRY_POINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_ENTRY_POINTS").field("cbSize", &self.cbSize).field("protocolVersion", &self.protocolVersion).field("pVirtualChannelInit", &self.pVirtualChannelInit.map(|f| f as usize)).field("pVirtualChannelOpen", &self.pVirtualChannelOpen.map(|f| f as usize)).field("pVirtualChannelClose", &self.pVirtualChannelClose.map(|f| f as usize)).field("pVirtualChannelWrite", &self.pVirtualChannelWrite.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANNEL_ENTRY_POINTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANNEL_ENTRY_POINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANNEL_ENTRY_POINTS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_ENTRY_POINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_NAME_LEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl ::core::marker::Copy for CHANNEL_PDU_HEADER {}
impl ::core::clone::Clone for CHANNEL_PDU_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANNEL_PDU_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_PDU_HEADER").field("length", &self.length).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_PDU_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANNEL_PDU_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANNEL_PDU_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANNEL_PDU_HEADER {}
impl ::core::default::Default for CHANNEL_PDU_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for CLIENT_DISPLAY {}
impl ::core::clone::Clone for CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
unsafe impl ::windows::core::Abi for CLIENT_DISPLAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIENT_DISPLAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLIENT_DISPLAY {}
impl ::core::default::Default for CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLIENT_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(2i32);
impl ::core::marker::Copy for CLIENT_MESSAGE_TYPE {}
impl ::core::clone::Clone for CLIENT_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLIENT_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLIENT_MESSAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLIENT_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIENT_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONNECTION_CHANGE_NOTIFICATION(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(8i32);
impl ::core::marker::Copy for CONNECTION_CHANGE_NOTIFICATION {}
impl ::core::clone::Clone for CONNECTION_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONNECTION_CHANGE_NOTIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONNECTION_CHANGE_NOTIFICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONNECTION_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONNECTION_CHANGE_NOTIFICATION").field(&self.0).finish()
    }
}
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b150580_fea4_4d3c_9de4_7433a66618f7);
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693f7ff5_0c4e_4d17_b8e0_1f70325e5d58);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_CONNECTED: u32 = 751u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_CONNECTING: u32 = 750u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DOMAIN_LENGTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FORCE_REJOIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HwtsVirtualChannelHandle(pub isize);
impl HwtsVirtualChannelHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HwtsVirtualChannelHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HwtsVirtualChannelHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HwtsVirtualChannelHandle {}
impl ::core::fmt::Debug for HwtsVirtualChannelHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HwtsVirtualChannelHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HwtsVirtualChannelHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsTSUserEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTSUserEx {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TerminalServicesProfilePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TerminalServicesProfilePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTerminalServicesProfilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTerminalServicesProfilePath)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TerminalServicesHomeDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TerminalServicesHomeDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTerminalServicesHomeDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTerminalServicesHomeDirectory)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TerminalServicesHomeDrive(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TerminalServicesHomeDrive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTerminalServicesHomeDrive<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTerminalServicesHomeDrive)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AllowLogon(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetAllowLogon(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn EnableRemoteControl(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableRemoteControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetEnableRemoteControl(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableRemoteControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn MaxDisconnectionTime(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxDisconnectionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetMaxDisconnectionTime(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxDisconnectionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn MaxConnectionTime(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxConnectionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetMaxConnectionTime(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxConnectionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn MaxIdleTime(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxIdleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetMaxIdleTime(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxIdleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ReconnectionAction(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReconnectionAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetReconnectionAction(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReconnectionAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn BrokenConnectionAction(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BrokenConnectionAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetBrokenConnectionAction(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBrokenConnectionAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ConnectClientDrivesAtLogon(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ConnectClientDrivesAtLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectClientDrivesAtLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ConnectClientPrintersAtLogon(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ConnectClientPrintersAtLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectClientPrintersAtLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DefaultToMainPrinter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DefaultToMainPrinter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetDefaultToMainPrinter(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultToMainPrinter)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TerminalServicesWorkDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TerminalServicesWorkDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTerminalServicesWorkDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTerminalServicesWorkDirectory)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TerminalServicesInitialProgram(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TerminalServicesInitialProgram)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTerminalServicesInitialProgram<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTerminalServicesInitialProgram)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IADsTSUserEx> for ::windows::core::IUnknown {
    fn from(value: IADsTSUserEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IADsTSUserEx> for ::windows::core::IUnknown {
    fn from(value: &IADsTSUserEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IADsTSUserEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IADsTSUserEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IADsTSUserEx> for super::Com::IDispatch {
    fn from(value: IADsTSUserEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IADsTSUserEx> for super::Com::IDispatch {
    fn from(value: &IADsTSUserEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IADsTSUserEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IADsTSUserEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsTSUserEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsTSUserEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsTSUserEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsTSUserEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsTSUserEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsTSUserEx {
    type Vtable = IADsTSUserEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4930e79_2989_4462_8a60_2fcf2f2955ef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTSUserEx_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesProfilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesProfilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesProfilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesProfilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesHomeDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesHomeDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesHomeDrive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesHomeDrive: usize,
    pub AllowLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAllowLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub EnableRemoteControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetEnableRemoteControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub MaxDisconnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxDisconnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub MaxConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub MaxIdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxIdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub ReconnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT,
    pub SetReconnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub BrokenConnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT,
    pub SetBrokenConnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub ConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT,
    pub SetConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub ConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub DefaultToMainPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetDefaultToMainPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesWorkDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesWorkDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesInitialProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesInitialProgram: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioDeviceEndpoint(::windows::core::IUnknown);
impl IAudioDeviceEndpoint {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxperiod), ::core::mem::transmute(u32latencycoefficient)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRTCaps(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRTCaps)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEventDrivenCapable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEventDrivenCapable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteExclusiveModeParametersToSharedMemory)(::core::mem::transmute_copy(self), ::core::mem::transmute(htargetprocess), ::core::mem::transmute(hnsperiod), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(u32latencycoefficient), ::core::mem::transmute(pu32sharedmemorysize), ::core::mem::transmute(phsharedmemory)).ok()
    }
}
impl ::core::convert::From<IAudioDeviceEndpoint> for ::windows::core::IUnknown {
    fn from(value: IAudioDeviceEndpoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioDeviceEndpoint> for ::windows::core::IUnknown {
    fn from(value: &IAudioDeviceEndpoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioDeviceEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioDeviceEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioDeviceEndpoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioDeviceEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioDeviceEndpoint {}
impl ::core::fmt::Debug for IAudioDeviceEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioDeviceEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioDeviceEndpoint {
    type Vtable = IAudioDeviceEndpoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4952f5a_a0b2_4cc4_8b82_9358488dd8ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceEndpoint_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRTCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRTCaps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventDrivenCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventDrivenCapable: usize,
    pub WriteExclusiveModeParametersToSharedMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioEndpoint(::windows::core::IUnknown);
impl IAudioEndpoint {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFrameFormat(&self) -> ::windows::core::Result<*mut super::super::Media::Audio::WAVEFORMATEX> {
        let mut result__: *mut super::super::Media::Audio::WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::Media::Audio::WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetFramesPerPacket(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFramesPerPacket)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetStreamFlags(&self, streamflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventHandle)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioEndpoint> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpoint> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpoint {}
impl ::core::fmt::Debug for IAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpoint {
    type Vtable = IAudioEndpoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30a99515_1527_4451_af9f_00c5f0234daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpoint_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFrameFormat: usize,
    pub GetFramesPerPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows::core::HRESULT,
    pub SetStreamFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointControl(::windows::core::IUnknown);
impl IAudioEndpointControl {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointControl> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEndpointControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEndpointControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointControl {}
impl ::core::fmt::Debug for IAudioEndpointControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointControl {
    type Vtable = IAudioEndpointControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc684b72a_6df4_4774_bdf9_76b77509b653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointRT(::windows::core::IUnknown);
impl IAudioEndpointRT {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
        (::windows::core::Interface::vtable(self).GetCurrentPadding)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppadding), ::core::mem::transmute(paecurrentposition))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ProcessingComplete(&self) {
        (::windows::core::Interface::vtable(self).ProcessingComplete)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetPinInactive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPinInactive)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetPinActive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPinActive)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointRT> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointRT> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointRT {}
impl ::core::fmt::Debug for IAudioEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointRT {
    type Vtable = IAudioEndpointRT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfd2005f_a6e5_4d39_a265_939ada9fbb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointRT_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION),
    pub ProcessingComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SetPinInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPinActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioInputEndpointRT(::windows::core::IUnknown);
impl IAudioInputEndpointRT {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Media_Audio_Apo\"`*"]
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
        (::windows::core::Interface::vtable(self).GetInputDataPointer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pconnectionproperty), ::core::mem::transmute(paetimestamp))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize) {
        (::windows::core::Interface::vtable(self).ReleaseInputDataPointer)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32framecount), ::core::mem::transmute(pdatapointer))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn PulseEndpoint(&self) {
        (::windows::core::Interface::vtable(self).PulseEndpoint)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IAudioInputEndpointRT> for ::windows::core::IUnknown {
    fn from(value: IAudioInputEndpointRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputEndpointRT> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputEndpointRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioInputEndpointRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputEndpointRT {}
impl ::core::fmt::Debug for IAudioInputEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputEndpointRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioInputEndpointRT {
    type Vtable = IAudioInputEndpointRT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8026ab61_92b2_43c1_a1df_5c37ebd08d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputEndpointRT_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub GetInputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    GetInputDataPointer: usize,
    pub ReleaseInputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize),
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IAudioOutputEndpointRT(::windows::core::IUnknown);
impl IAudioOutputEndpointRT {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetOutputDataPointer)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32framecount), ::core::mem::transmute(paetimestamp)))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Media_Audio_Apo\"`*"]
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).ReleaseOutputDataPointer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pconnectionproperty))
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn PulseEndpoint(&self) {
        (::windows::core::Interface::vtable(self).PulseEndpoint)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IAudioOutputEndpointRT> for ::windows::core::IUnknown {
    fn from(value: IAudioOutputEndpointRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioOutputEndpointRT> for ::windows::core::IUnknown {
    fn from(value: &IAudioOutputEndpointRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioOutputEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioOutputEndpointRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioOutputEndpointRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioOutputEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputEndpointRT {}
impl ::core::fmt::Debug for IAudioOutputEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputEndpointRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioOutputEndpointRT {
    type Vtable = IAudioOutputEndpointRT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa906e4_c31c_4e31_932e_19a66385e9aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputEndpointRT_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOutputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointer: usize,
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteDesktopClient(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClient {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Reconnect(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reconnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> ::windows::core::Result<IRemoteDesktopClientSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Settings)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRemoteDesktopClientSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Actions(&self) -> ::windows::core::Result<IRemoteDesktopClientActions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Actions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRemoteDesktopClientActions>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TouchPointer(&self) -> ::windows::core::Result<IRemoteDesktopClientTouchPointer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TouchPointer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRemoteDesktopClientTouchPointer>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSavedCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteSavedCredentials)(::core::mem::transmute_copy(self), servername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateSessionDisplaySettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn attachEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, eventname: Param0, callback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).attachEvent)(::core::mem::transmute_copy(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn detachEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, eventname: Param0, callback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).detachEvent)(::core::mem::transmute_copy(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClient> for ::windows::core::IUnknown {
    fn from(value: IRemoteDesktopClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClient> for ::windows::core::IUnknown {
    fn from(value: &IRemoteDesktopClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteDesktopClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteDesktopClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClient> for super::Com::IDispatch {
    fn from(value: IRemoteDesktopClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClient> for super::Com::IDispatch {
    fn from(value: &IRemoteDesktopClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteDesktopClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteDesktopClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRemoteDesktopClient {
    type Vtable = IRemoteDesktopClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d25668_625a_4905_be4e_304caa13f89c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClient_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TouchPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, touchpointer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TouchPointer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSavedCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSavedCredentials: usize,
    pub UpdateSessionDisplaySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub attachEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    attachEvent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub detachEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    detachEvent: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteDesktopClientActions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientActions {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SuspendScreenUpdates(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendScreenUpdates)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ResumeScreenUpdates(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeScreenUpdates)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExecuteRemoteAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(remoteaction)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSnapshot)(::core::mem::transmute_copy(self), ::core::mem::transmute(snapshotencoding), ::core::mem::transmute(snapshotformat), ::core::mem::transmute(snapshotwidth), ::core::mem::transmute(snapshotheight), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientActions> for ::windows::core::IUnknown {
    fn from(value: IRemoteDesktopClientActions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientActions> for ::windows::core::IUnknown {
    fn from(value: &IRemoteDesktopClientActions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientActions> for super::Com::IDispatch {
    fn from(value: IRemoteDesktopClientActions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientActions> for super::Com::IDispatch {
    fn from(value: &IRemoteDesktopClientActions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteDesktopClientActions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientActions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientActions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientActions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientActions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRemoteDesktopClientActions {
    type Vtable = IRemoteDesktopClientActions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d54bc4e_1028_45d4_8b0a_b9b6bffba176);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientActions_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub SuspendScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecuteRemoteAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapshot: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteDesktopClientSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientSettings {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rdpfilecontents: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplySettings)(::core::mem::transmute_copy(self), rdpfilecontents.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RetrieveSettings(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RetrieveSettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRdpProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRdpProperty)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetRdpProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, propertyname: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRdpProperty)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientSettings> for ::windows::core::IUnknown {
    fn from(value: IRemoteDesktopClientSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientSettings> for ::windows::core::IUnknown {
    fn from(value: &IRemoteDesktopClientSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientSettings> for super::Com::IDispatch {
    fn from(value: IRemoteDesktopClientSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientSettings> for super::Com::IDispatch {
    fn from(value: &IRemoteDesktopClientSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteDesktopClientSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRemoteDesktopClientSettings {
    type Vtable = IRemoteDesktopClientSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48a0f2a7_2713_431f_bbac_6f4558e7d64d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientSettings_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplySettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RetrieveSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RetrieveSettings: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRdpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRdpProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetRdpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetRdpProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteDesktopClientTouchPointer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientTouchPointer {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetEventsEnabled(&self, eventsenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventsEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventsenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn EventsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventsEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetPointerSpeed(&self, pointerspeed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPointerSpeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerspeed)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn PointerSpeed(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PointerSpeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientTouchPointer> for ::windows::core::IUnknown {
    fn from(value: IRemoteDesktopClientTouchPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientTouchPointer> for ::windows::core::IUnknown {
    fn from(value: &IRemoteDesktopClientTouchPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteDesktopClientTouchPointer> for super::Com::IDispatch {
    fn from(value: IRemoteDesktopClientTouchPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteDesktopClientTouchPointer> for super::Com::IDispatch {
    fn from(value: &IRemoteDesktopClientTouchPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteDesktopClientTouchPointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientTouchPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientTouchPointer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientTouchPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientTouchPointer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRemoteDesktopClientTouchPointer {
    type Vtable = IRemoteDesktopClientTouchPointer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260ec22d_8cbc_44b5_9e88_2a37f6c93ae9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientTouchPointer_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEventsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsenabled: i16) -> ::windows::core::HRESULT,
    pub EventsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetPointerSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows::core::HRESULT,
    pub PointerSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IRemoteSystemAdditionalInfoProvider(::windows::core::IUnknown);
impl IRemoteSystemAdditionalInfoProvider {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetAdditionalInfo<T: ::windows::core::Interface>(&self, deduplicationid: *mut ::windows::core::HSTRING) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetAdditionalInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(deduplicationid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IRemoteSystemAdditionalInfoProvider> for ::windows::core::IUnknown {
    fn from(value: IRemoteSystemAdditionalInfoProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRemoteSystemAdditionalInfoProvider> for ::windows::core::IUnknown {
    fn from(value: &IRemoteSystemAdditionalInfoProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteSystemAdditionalInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteSystemAdditionalInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRemoteSystemAdditionalInfoProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRemoteSystemAdditionalInfoProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteSystemAdditionalInfoProvider {}
impl ::core::fmt::Debug for IRemoteSystemAdditionalInfoProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteSystemAdditionalInfoProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteSystemAdditionalInfoProvider {
    type Vtable = IRemoteSystemAdditionalInfoProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaa3d5f_ec63_4d27_af38_e86b1d7292cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAdditionalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGAccountingEngine(::windows::core::IUnknown);
impl ITSGAccountingEngine {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoAccounting<'a, Param1: ::windows::core::IntoParam<'a, AAAccountingData>>(&self, accountingdatatype: AAAccountingDataType, accountingdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoAccounting)(::core::mem::transmute_copy(self), ::core::mem::transmute(accountingdatatype), accountingdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITSGAccountingEngine> for ::windows::core::IUnknown {
    fn from(value: ITSGAccountingEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGAccountingEngine> for ::windows::core::IUnknown {
    fn from(value: &ITSGAccountingEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGAccountingEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGAccountingEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGAccountingEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGAccountingEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAccountingEngine {}
impl ::core::fmt::Debug for ITSGAccountingEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAccountingEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGAccountingEngine {
    type Vtable = ITSGAccountingEngine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce2a0c9_e874_4f1a_86f4_06bbb9115338);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAccountingEngine_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DoAccounting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoAccounting: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGAuthenticateUserSink(::windows::core::IUnknown);
impl ITSGAuthenticateUserSink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUserAuthenticated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, username: Param0, userdomain: Param1, context: usize, usertoken: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUserAuthenticated)(::core::mem::transmute_copy(self), username.into_param().abi(), userdomain.into_param().abi(), ::core::mem::transmute(context), usertoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUserAuthenticationFailed)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), ::core::mem::transmute(genericerrorcode), ::core::mem::transmute(specificerrorcode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ReauthenticateUser(&self, context: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReauthenticateUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DisconnectUser(&self, context: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
}
impl ::core::convert::From<ITSGAuthenticateUserSink> for ::windows::core::IUnknown {
    fn from(value: ITSGAuthenticateUserSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGAuthenticateUserSink> for ::windows::core::IUnknown {
    fn from(value: &ITSGAuthenticateUserSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGAuthenticateUserSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGAuthenticateUserSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGAuthenticateUserSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGAuthenticateUserSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthenticateUserSink {}
impl ::core::fmt::Debug for ITSGAuthenticateUserSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthenticateUserSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGAuthenticateUserSink {
    type Vtable = ITSGAuthenticateUserSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3e2e73_a782_47f9_8dfb_77ee1ed27a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticateUserSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUserAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUserAuthenticated: usize,
    pub OnUserAuthenticationFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ReauthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT,
    pub DisconnectUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGAuthenticationEngine(::windows::core::IUnknown);
impl ITSGAuthenticationEngine {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AuthenticateUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ITSGAuthenticateUserSink>>(&self, mainsessionid: Param0, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AuthenticateUser)(::core::mem::transmute_copy(self), mainsessionid.into_param().abi(), ::core::mem::transmute(cookiedata), ::core::mem::transmute(numcookiebytes), ::core::mem::transmute(context), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn CancelAuthentication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, mainsessionid: Param0, context: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelAuthentication)(::core::mem::transmute_copy(self), mainsessionid.into_param().abi(), ::core::mem::transmute(context)).ok()
    }
}
impl ::core::convert::From<ITSGAuthenticationEngine> for ::windows::core::IUnknown {
    fn from(value: ITSGAuthenticationEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGAuthenticationEngine> for ::windows::core::IUnknown {
    fn from(value: &ITSGAuthenticationEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGAuthenticationEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGAuthenticationEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGAuthenticationEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGAuthenticationEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthenticationEngine {}
impl ::core::fmt::Debug for ITSGAuthenticationEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthenticationEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGAuthenticationEngine {
    type Vtable = ITSGAuthenticationEngine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ee3e5bf_04ab_4691_998c_d7f622321a56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticationEngine_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CancelAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, context: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGAuthorizeConnectionSink(::windows::core::IUnknown);
impl ITSGAuthorizeConnectionSink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnConnectionAuthorized<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, hrin: ::windows::core::HRESULT, mainsessionid: Param1, pbsohresponse: &[u8], idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionAuthorized)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrin), mainsessionid.into_param().abi(), pbsohresponse.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbsohresponse)), ::core::mem::transmute(idletimeout), ::core::mem::transmute(sessiontimeout), ::core::mem::transmute(sessiontimeoutaction), ::core::mem::transmute(trustclass), ::core::mem::transmute(policyattributes)).ok()
    }
}
impl ::core::convert::From<ITSGAuthorizeConnectionSink> for ::windows::core::IUnknown {
    fn from(value: ITSGAuthorizeConnectionSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGAuthorizeConnectionSink> for ::windows::core::IUnknown {
    fn from(value: &ITSGAuthorizeConnectionSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGAuthorizeConnectionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGAuthorizeConnectionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGAuthorizeConnectionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGAuthorizeConnectionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthorizeConnectionSink {}
impl ::core::fmt::Debug for ITSGAuthorizeConnectionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthorizeConnectionSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGAuthorizeConnectionSink {
    type Vtable = ITSGAuthorizeConnectionSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc27ece33_7781_4318_98ef_1cf2da7b7005);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeConnectionSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnConnectionAuthorized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGAuthorizeResourceSink(::windows::core::IUnknown);
impl ITSGAuthorizeResourceSink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnChannelAuthorized<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, hrin: ::windows::core::HRESULT, mainsessionid: Param1, subsessionid: i32, allowedresourcenames: &[super::super::Foundation::BSTR], failedresourcenames: &[super::super::Foundation::BSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChannelAuthorized)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrin), mainsessionid.into_param().abi(), ::core::mem::transmute(subsessionid), ::core::mem::transmute(::windows::core::as_ptr_or_null(allowedresourcenames)), allowedresourcenames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(failedresourcenames)), failedresourcenames.len() as _).ok()
    }
}
impl ::core::convert::From<ITSGAuthorizeResourceSink> for ::windows::core::IUnknown {
    fn from(value: ITSGAuthorizeResourceSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGAuthorizeResourceSink> for ::windows::core::IUnknown {
    fn from(value: &ITSGAuthorizeResourceSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGAuthorizeResourceSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGAuthorizeResourceSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGAuthorizeResourceSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGAuthorizeResourceSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthorizeResourceSink {}
impl ::core::fmt::Debug for ITSGAuthorizeResourceSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthorizeResourceSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGAuthorizeResourceSink {
    type Vtable = ITSGAuthorizeResourceSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeddfcd4_fa12_4435_ae55_7ad1a9779af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeResourceSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnChannelAuthorized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnChannelAuthorized: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITSGPolicyEngine(::windows::core::IUnknown);
impl ITSGPolicyEngine {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthorizeConnection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param10: ::windows::core::IntoParam<'a, ITSGAuthorizeConnectionSink>>(&self, mainsessionid: Param0, username: Param1, authtype: AAAuthSchemes, clientmachineip: Param3, clientmachinename: Param4, sohdata: &[u8], cookiedata: &[u8], usertoken: Param9, psink: Param10) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AuthorizeConnection)(::core::mem::transmute_copy(self), mainsessionid.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(authtype), clientmachineip.into_param().abi(), clientmachinename.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(sohdata)), sohdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(cookiedata)), cookiedata.len() as _, usertoken.into_param().abi(), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthorizeResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param11: ::windows::core::IntoParam<'a, ITSGAuthorizeResourceSink>>(&self, mainsessionid: Param0, subsessionid: i32, username: Param2, resourcenames: &[super::super::Foundation::BSTR], alternateresourcenames: &[super::super::Foundation::BSTR], portnumber: u32, operation: Param8, cookie: &[u8], psink: Param11) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AuthorizeResource)(
            ::core::mem::transmute_copy(self),
            mainsessionid.into_param().abi(),
            ::core::mem::transmute(subsessionid),
            username.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(resourcenames)),
            resourcenames.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(alternateresourcenames)),
            alternateresourcenames.len() as _,
            ::core::mem::transmute(portnumber),
            operation.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(cookie)),
            cookie.len() as _,
            psink.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsQuarantineEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsQuarantineEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITSGPolicyEngine> for ::windows::core::IUnknown {
    fn from(value: ITSGPolicyEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITSGPolicyEngine> for ::windows::core::IUnknown {
    fn from(value: &ITSGPolicyEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITSGPolicyEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITSGPolicyEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITSGPolicyEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITSGPolicyEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGPolicyEngine {}
impl ::core::fmt::Debug for ITSGPolicyEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGPolicyEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITSGPolicyEngine {
    type Vtable = ITSGPolicyEngine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bc24f08_6223_42f4_a5b4_8e37cd135bbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGPolicyEngine_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthorizeConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthorizeConnection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthorizeResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, subsessionid: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthorizeResource: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsQuarantineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsQuarantineEnabled: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbBaseNotifySink(::windows::core::IUnknown);
impl ITsSbBaseNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
}
impl ::core::convert::From<ITsSbBaseNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbBaseNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbBaseNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbBaseNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbBaseNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbBaseNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbBaseNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbBaseNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbBaseNotifySink {}
impl ::core::fmt::Debug for ITsSbBaseNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbBaseNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbBaseNotifySink {
    type Vtable = ITsSbBaseNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x808a6537_1282_4989_9e09_f43938b71722);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbBaseNotifySink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbClientConnection(::windows::core::IUnknown);
impl ITsSbClientConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitialProgram(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialProgram)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn LoadBalanceResult(&self) -> ::windows::core::Result<ITsSbLoadBalanceResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadBalanceResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbLoadBalanceResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FarmName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FarmName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, contextid: Param0, context: Param1) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PutContext)(::core::mem::transmute_copy(self), contextid.into_param().abi(), context.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, contextid: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContext)(::core::mem::transmute_copy(self), contextid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Environment(&self) -> ::windows::core::Result<ITsSbEnvironment> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Environment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ConnectionError(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionError)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SamUserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SamUserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn ClientConnectionPropertySet(&self) -> ::windows::core::Result<ITsSbClientConnectionPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClientConnectionPropertySet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbClientConnectionPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFirstAssignment(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstAssignment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RdFarmType(&self) -> ::windows::core::Result<RD_FARM_TYPE> {
        let mut result__: RD_FARM_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RdFarmType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<RD_FARM_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn UserSidString(&self) -> ::windows::core::Result<*mut i8> {
        let mut result__: *mut i8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserSidString)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut i8>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetDisconnectedSession(&self) -> ::windows::core::Result<ITsSbSession> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisconnectedSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbSession>(result__)
    }
}
impl ::core::convert::From<ITsSbClientConnection> for ::windows::core::IUnknown {
    fn from(value: ITsSbClientConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbClientConnection> for ::windows::core::IUnknown {
    fn from(value: &ITsSbClientConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbClientConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbClientConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbClientConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbClientConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbClientConnection {}
impl ::core::fmt::Debug for ITsSbClientConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbClientConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbClientConnection {
    type Vtable = ITsSbClientConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18857499_ad61_4b1b_b7df_cbcd41fb8338);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitialProgram: usize,
    pub LoadBalanceResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FarmName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetContext: usize,
    pub Environment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ConnectionError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SamUserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SamUserAccount: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ClientConnectionPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ClientConnectionPropertySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFirstAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFirstAssignment: usize,
    pub RdFarmType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows::core::HRESULT,
    pub UserSidString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows::core::HRESULT,
    pub GetDisconnectedSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
pub struct ITsSbClientConnectionPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnectionPropertySet {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Write)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbClientConnectionPropertySet> for ::windows::core::IUnknown {
    fn from(value: ITsSbClientConnectionPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbClientConnectionPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ITsSbClientConnectionPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbClientConnectionPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: ITsSbClientConnectionPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbClientConnectionPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: &ITsSbClientConnectionPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &'a ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbClientConnectionPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbClientConnectionPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbClientConnectionPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbClientConnectionPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for &'a ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for ITsSbClientConnectionPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbClientConnectionPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbClientConnectionPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbClientConnectionPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbClientConnectionPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Interface for ITsSbClientConnectionPropertySet {
    type Vtable = ITsSbClientConnectionPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe51995b0_46d6_11dd_aa21_cedc55d89593);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnectionPropertySet_Vtbl {
    pub base: ITsSbPropertySet_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbEnvironment(::windows::core::IUnknown);
impl ITsSbEnvironment {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ServerWeight(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ServerWeight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnvironmentPropertySet(&self) -> ::windows::core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnvironmentPropertySet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbEnvironmentPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetEnvironmentPropertySet<'a, Param0: ::windows::core::IntoParam<'a, ITsSbEnvironmentPropertySet>>(&self, pval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironmentPropertySet)(::core::mem::transmute_copy(self), pval.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbEnvironment> for ::windows::core::IUnknown {
    fn from(value: ITsSbEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbEnvironment> for ::windows::core::IUnknown {
    fn from(value: &ITsSbEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbEnvironment {}
impl ::core::fmt::Debug for ITsSbEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbEnvironment {
    type Vtable = ITsSbEnvironment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c87f7f7_bf51_4a5c_87bf_8e94fb6e2256);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironment_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub ServerWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnvironmentPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnvironmentPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetEnvironmentPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetEnvironmentPropertySet: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
pub struct ITsSbEnvironmentPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironmentPropertySet {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Write)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbEnvironmentPropertySet> for ::windows::core::IUnknown {
    fn from(value: ITsSbEnvironmentPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbEnvironmentPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ITsSbEnvironmentPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbEnvironmentPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: ITsSbEnvironmentPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbEnvironmentPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: &ITsSbEnvironmentPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &'a ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbEnvironmentPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbEnvironmentPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbEnvironmentPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbEnvironmentPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for &'a ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for ITsSbEnvironmentPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbEnvironmentPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbEnvironmentPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbEnvironmentPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbEnvironmentPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Interface for ITsSbEnvironmentPropertySet {
    type Vtable = ITsSbEnvironmentPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0d1bf7e_7acf_11dd_a243_e51156d89593);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironmentPropertySet_Vtbl {
    pub base: ITsSbPropertySet_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbFilterPluginStore(::windows::core::IUnknown);
impl ITsSbFilterPluginStore {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveProperties<'a, Param0: ::windows::core::IntoParam<'a, ITsSbPropertySet>>(&self, ppropertyset: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveProperties)(::core::mem::transmute_copy(self), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnumerateProperties(&self) -> ::windows::core::Result<ITsSbPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumerateProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteProperties)(::core::mem::transmute_copy(self), propertyname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbFilterPluginStore> for ::windows::core::IUnknown {
    fn from(value: ITsSbFilterPluginStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbFilterPluginStore> for ::windows::core::IUnknown {
    fn from(value: &ITsSbFilterPluginStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbFilterPluginStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbFilterPluginStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbFilterPluginStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbFilterPluginStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbFilterPluginStore {}
impl ::core::fmt::Debug for ITsSbFilterPluginStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbFilterPluginStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbFilterPluginStore {
    type Vtable = ITsSbFilterPluginStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85b44b0f_ed78_413f_9702_fa6d3b5ee755);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbFilterPluginStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnumerateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnumerateProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteProperties: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbGenericNotifySink(::windows::core::IUnknown);
impl ITsSbGenericNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnCompleted(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCompleted)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWaitTimeout(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWaitTimeout)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
impl ::core::convert::From<ITsSbGenericNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbGenericNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbGenericNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbGenericNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbGenericNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbGenericNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbGenericNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbGenericNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbGenericNotifySink {}
impl ::core::fmt::Debug for ITsSbGenericNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbGenericNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbGenericNotifySink {
    type Vtable = ITsSbGenericNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c4c8c4f_300b_46ad_9164_8468a7e7568c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGenericNotifySink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWaitTimeout: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbGlobalStore(::windows::core::IUnknown);
impl ITsSbGlobalStore {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, targetname: Param1, farmname: Param2) -> ::windows::core::Result<ITsSbTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryTarget)(::core::mem::transmute_copy(self), providername.into_param().abi(), targetname.into_param().abi(), farmname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbTarget>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QuerySessionBySessionId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, dwsessionid: u32, targetname: Param2) -> ::windows::core::Result<ITsSbSession> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuerySessionBySessionId)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(dwsessionid), targetname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbSession>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumerateFarms<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateFarms)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateTargets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, farmname: Param1, envname: Param2, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateTargets)(::core::mem::transmute_copy(self), providername.into_param().abi(), farmname.into_param().abi(), envname.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateEnvironmentsByProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateEnvironmentsByProvider)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(ppval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateSessions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, targetname: Param1, username: Param2, userdomain: Param3, poolname: Param4, initialprogram: Param5, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateSessions)(::core::mem::transmute_copy(self), providername.into_param().abi(), targetname.into_param().abi(), username.into_param().abi(), userdomain.into_param().abi(), poolname.into_param().abi(), initialprogram.into_param().abi(), ::core::mem::transmute(psessionstate), ::core::mem::transmute(pdwcount), ::core::mem::transmute(ppval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFarmProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, propertyname: Param1, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFarmProperty)(::core::mem::transmute_copy(self), farmname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
}
impl ::core::convert::From<ITsSbGlobalStore> for ::windows::core::IUnknown {
    fn from(value: ITsSbGlobalStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbGlobalStore> for ::windows::core::IUnknown {
    fn from(value: &ITsSbGlobalStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbGlobalStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbGlobalStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbGlobalStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbGlobalStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbGlobalStore {}
impl ::core::fmt::Debug for ITsSbGlobalStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbGlobalStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbGlobalStore {
    type Vtable = ITsSbGlobalStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ab60f7b_bd72_4d9f_8a3a_a0ea5574e635);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGlobalStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QuerySessionBySessionId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumerateFarms: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateTargets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateEnvironmentsByProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateEnvironmentsByProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateSessions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFarmProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbLoadBalanceResult(::windows::core::IUnknown);
impl ITsSbLoadBalanceResult {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITsSbLoadBalanceResult> for ::windows::core::IUnknown {
    fn from(value: ITsSbLoadBalanceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbLoadBalanceResult> for ::windows::core::IUnknown {
    fn from(value: &ITsSbLoadBalanceResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbLoadBalanceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbLoadBalanceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbLoadBalanceResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalanceResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalanceResult {}
impl ::core::fmt::Debug for ITsSbLoadBalanceResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalanceResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbLoadBalanceResult {
    type Vtable = ITsSbLoadBalanceResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24fdb7ac_fea6_11dc_9672_9a8956d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalanceResult_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbLoadBalancing(::windows::core::IUnknown);
impl ITsSbLoadBalancing {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetMostSuitableTarget<'a, Param0: ::windows::core::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::core::IntoParam<'a, ITsSbLoadBalancingNotifySink>>(&self, pconnection: Param0, plbsink: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMostSuitableTarget)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), plbsink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbLoadBalancing> for ::windows::core::IUnknown {
    fn from(value: ITsSbLoadBalancing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbLoadBalancing> for ::windows::core::IUnknown {
    fn from(value: &ITsSbLoadBalancing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbLoadBalancing> for ITsSbPlugin {
    fn from(value: ITsSbLoadBalancing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbLoadBalancing> for ITsSbPlugin {
    fn from(value: &ITsSbLoadBalancing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbLoadBalancing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalancing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalancing {}
impl ::core::fmt::Debug for ITsSbLoadBalancing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalancing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbLoadBalancing {
    type Vtable = ITsSbLoadBalancing_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24329274_9eb7_11dc_ae98_f2b456d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancing_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
    pub GetMostSuitableTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, plbsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbLoadBalancingNotifySink(::windows::core::IUnknown);
impl ITsSbLoadBalancingNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnGetMostSuitableTarget<'a, Param0: ::windows::core::IntoParam<'a, ITsSbLoadBalanceResult>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, plbresult: Param0, fisnewconnection: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnGetMostSuitableTarget)(::core::mem::transmute_copy(self), plbresult.into_param().abi(), fisnewconnection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbLoadBalancingNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbLoadBalancingNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbLoadBalancingNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbLoadBalancingNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbLoadBalancingNotifySink> for ITsSbBaseNotifySink {
    fn from(value: ITsSbLoadBalancingNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbLoadBalancingNotifySink> for ITsSbBaseNotifySink {
    fn from(value: &ITsSbLoadBalancingNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for &'a ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbLoadBalancingNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalancingNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalancingNotifySink {}
impl ::core::fmt::Debug for ITsSbLoadBalancingNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalancingNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbLoadBalancingNotifySink {
    type Vtable = ITsSbLoadBalancingNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f8a8297_3244_4e6a_958a_27c822c1e141);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancingNotifySink_Vtbl {
    pub base: ITsSbBaseNotifySink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnGetMostSuitableTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbresult: ::windows::core::RawPtr, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnGetMostSuitableTarget: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbOrchestration(::windows::core::IUnknown);
impl ITsSbOrchestration {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn PrepareTargetForConnect<'a, Param0: ::windows::core::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::core::IntoParam<'a, ITsSbOrchestrationNotifySink>>(&self, pconnection: Param0, porchestrationnotifysink: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareTargetForConnect)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), porchestrationnotifysink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbOrchestration> for ::windows::core::IUnknown {
    fn from(value: ITsSbOrchestration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbOrchestration> for ::windows::core::IUnknown {
    fn from(value: &ITsSbOrchestration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbOrchestration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbOrchestration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbOrchestration> for ITsSbPlugin {
    fn from(value: ITsSbOrchestration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbOrchestration> for ITsSbPlugin {
    fn from(value: &ITsSbOrchestration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbOrchestration {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbOrchestration {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbOrchestration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbOrchestration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbOrchestration {}
impl ::core::fmt::Debug for ITsSbOrchestration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbOrchestration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbOrchestration {
    type Vtable = ITsSbOrchestration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64fc1172_9eb7_11dc_8b00_3aba56d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestration_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
    pub PrepareTargetForConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, porchestrationnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbOrchestrationNotifySink(::windows::core::IUnknown);
impl ITsSbOrchestrationNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReadyToConnect<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTarget>>(&self, ptarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReadyToConnect)(::core::mem::transmute_copy(self), ptarget.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbOrchestrationNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbOrchestrationNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbOrchestrationNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbOrchestrationNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbOrchestrationNotifySink> for ITsSbBaseNotifySink {
    fn from(value: ITsSbOrchestrationNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbOrchestrationNotifySink> for ITsSbBaseNotifySink {
    fn from(value: &ITsSbOrchestrationNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for &'a ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbOrchestrationNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbOrchestrationNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbOrchestrationNotifySink {}
impl ::core::fmt::Debug for ITsSbOrchestrationNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbOrchestrationNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbOrchestrationNotifySink {
    type Vtable = ITsSbOrchestrationNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36c37d61_926b_442f_bca5_118c6d50dcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestrationNotifySink_Vtbl {
    pub base: ITsSbBaseNotifySink_Vtbl,
    pub OnReadyToConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbPlacement(::windows::core::IUnknown);
impl ITsSbPlacement {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn QueryEnvironmentForTarget<'a, Param0: ::windows::core::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::core::IntoParam<'a, ITsSbPlacementNotifySink>>(&self, pconnection: Param0, pplacementsink: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryEnvironmentForTarget)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), pplacementsink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbPlacement> for ::windows::core::IUnknown {
    fn from(value: ITsSbPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPlacement> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbPlacement> for ITsSbPlugin {
    fn from(value: ITsSbPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPlacement> for ITsSbPlugin {
    fn from(value: &ITsSbPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbPlacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlacement {}
impl ::core::fmt::Debug for ITsSbPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlacement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbPlacement {
    type Vtable = ITsSbPlacement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaadee5f_6d32_480e_9e36_ddab2329f06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacement_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
    pub QueryEnvironmentForTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pplacementsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbPlacementNotifySink(::windows::core::IUnknown);
impl ITsSbPlacementNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnQueryEnvironmentCompleted<'a, Param0: ::windows::core::IntoParam<'a, ITsSbEnvironment>>(&self, penvironment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQueryEnvironmentCompleted)(::core::mem::transmute_copy(self), penvironment.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbPlacementNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbPlacementNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPlacementNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPlacementNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbPlacementNotifySink> for ITsSbBaseNotifySink {
    fn from(value: ITsSbPlacementNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPlacementNotifySink> for ITsSbBaseNotifySink {
    fn from(value: &ITsSbPlacementNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for &'a ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbPlacementNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbPlacementNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlacementNotifySink {}
impl ::core::fmt::Debug for ITsSbPlacementNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlacementNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbPlacementNotifySink {
    type Vtable = ITsSbPlacementNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68a0c487_2b4f_46c2_94a1_6ce685183634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacementNotifySink_Vtbl {
    pub base: ITsSbBaseNotifySink_Vtbl,
    pub OnQueryEnvironmentCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbPlugin(::windows::core::IUnknown);
impl ITsSbPlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
impl ::core::convert::From<ITsSbPlugin> for ::windows::core::IUnknown {
    fn from(value: ITsSbPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPlugin> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlugin {}
impl ::core::fmt::Debug for ITsSbPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbPlugin {
    type Vtable = ITsSbPlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48cd7406_caab_465f_a5d6_baa863b9ea4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlugin_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pnotifysink: ::windows::core::RawPtr, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbPluginNotifySink(::windows::core::IUnknown);
impl ITsSbPluginNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnInitialized(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInitialized)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnTerminated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTerminated)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITsSbPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbPluginNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPluginNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbPluginNotifySink> for ITsSbBaseNotifySink {
    fn from(value: ITsSbPluginNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPluginNotifySink> for ITsSbBaseNotifySink {
    fn from(value: &ITsSbPluginNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for &'a ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbPluginNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPluginNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbPluginNotifySink {
    type Vtable = ITsSbPluginNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44dfe30b_c3be_40f5_bf82_7a95bb795adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginNotifySink_Vtbl {
    pub base: ITsSbBaseNotifySink_Vtbl,
    pub OnInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnTerminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
pub struct ITsSbPluginPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPluginPropertySet {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Write)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbPluginPropertySet> for ::windows::core::IUnknown {
    fn from(value: ITsSbPluginPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbPluginPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPluginPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbPluginPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: ITsSbPluginPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbPluginPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: &ITsSbPluginPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &'a ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbPluginPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbPluginPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbPluginPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbPluginPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for &'a ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for ITsSbPluginPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbPluginPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbPluginPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbPluginPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPluginPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Interface for ITsSbPluginPropertySet {
    type Vtable = ITsSbPluginPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95006e34_7eff_4b6c_bb40_49a4fda7cea6);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginPropertySet_Vtbl {
    pub base: ITsSbPropertySet_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
pub struct ITsSbPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPropertySet {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Read)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Write)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbPropertySet> for ::windows::core::IUnknown {
    fn from(value: ITsSbPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ITsSbPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: ITsSbPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: &ITsSbPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &'a ITsSbPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for ITsSbPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Interface for ITsSbPropertySet {
    type Vtable = ITsSbPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c025171_bb1e_4baf_a212_6d5e9774b33b);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPropertySet_Vtbl {
    pub base: super::Com::StructuredStorage::IPropertyBag_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbProvider(::windows::core::IUnknown);
impl ITsSbProvider {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, environmentname: Param1) -> ::windows::core::Result<ITsSbTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTargetObject)(::core::mem::transmute_copy(self), targetname.into_param().abi(), environmentname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbTarget>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLoadBalanceResultObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0) -> ::windows::core::Result<ITsSbLoadBalanceResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLoadBalanceResultObject)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbLoadBalanceResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSessionObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, username: Param1, domain: Param2, sessionid: u32) -> ::windows::core::Result<ITsSbSession> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSessionObject)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbSession>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreatePluginPropertySet(&self) -> ::windows::core::Result<ITsSbPluginPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePluginPropertySet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbPluginPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateTargetPropertySetObject(&self) -> ::windows::core::Result<ITsSbTargetPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTargetPropertySetObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbTargetPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateEnvironmentObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, serverweight: u32) -> ::windows::core::Result<ITsSbEnvironment> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEnvironmentObject)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(serverweight), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResourcePluginStore(&self) -> ::windows::core::Result<ITsSbResourcePluginStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResourcePluginStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbResourcePluginStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetFilterPluginStore(&self) -> ::windows::core::Result<ITsSbFilterPluginStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFilterPluginStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbFilterPluginStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterForNotification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ITsSbResourceNotification>>(&self, notificationtype: u32, resourcetomonitor: Param1, ppluginnotification: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterForNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), resourcetomonitor.into_param().abi(), ppluginnotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnRegisterForNotification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, notificationtype: u32, resourcetomonitor: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnRegisterForNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), resourcetomonitor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetInstanceOfGlobalStore(&self) -> ::windows::core::Result<ITsSbGlobalStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInstanceOfGlobalStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbGlobalStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateEnvironmentPropertySetObject(&self) -> ::windows::core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEnvironmentPropertySetObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbEnvironmentPropertySet>(result__)
    }
}
impl ::core::convert::From<ITsSbProvider> for ::windows::core::IUnknown {
    fn from(value: ITsSbProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbProvider> for ::windows::core::IUnknown {
    fn from(value: &ITsSbProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvider {}
impl ::core::fmt::Debug for ITsSbProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbProvider {
    type Vtable = ITsSbProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a4098f_6d7b_44dd_bc17_8ce44e370d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLoadBalanceResultObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLoadBalanceResultObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionObject: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreatePluginPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreatePluginPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateTargetPropertySetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateTargetPropertySetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateEnvironmentObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateEnvironmentObject: usize,
    pub GetResourcePluginStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFilterPluginStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterForNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterForNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnRegisterForNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnRegisterForNotification: usize,
    pub GetInstanceOfGlobalStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppglobalstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEnvironmentPropertySetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEnvironmentPropertySetObject: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbProvisioning(::windows::core::IUnknown);
impl ITsSbProvisioning {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualMachines<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVirtualMachines)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PatchVirtualMachines<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PatchVirtualMachines)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(pvmpatchinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteVirtualMachines<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteVirtualMachines)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CancelJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, jobguid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelJob)(::core::mem::transmute_copy(self), jobguid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbProvisioning> for ::windows::core::IUnknown {
    fn from(value: ITsSbProvisioning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbProvisioning> for ::windows::core::IUnknown {
    fn from(value: &ITsSbProvisioning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbProvisioning> for ITsSbPlugin {
    fn from(value: ITsSbProvisioning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbProvisioning> for ITsSbPlugin {
    fn from(value: &ITsSbProvisioning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvisioning {}
impl ::core::fmt::Debug for ITsSbProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvisioning").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbProvisioning {
    type Vtable = ITsSbProvisioning_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f6f0dbb_9e4f_462b_9c3f_fccc3dcb6232);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioning_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PatchVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PatchVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CancelJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CancelJob: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbProvisioningPluginNotifySink(::windows::core::IUnknown);
impl ITsSbProvisioningPluginNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnJobCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVirtualMachineStatusChanged<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnVirtualMachineStatusChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyentry), ::core::mem::transmute(vmnotifystatus), ::core::mem::transmute(errorcode), errordescr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnJobCompleted<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, resultcode: ::windows::core::HRESULT, resultdescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnJobCompleted)(::core::mem::transmute_copy(self), ::core::mem::transmute(resultcode), resultdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnJobCancelled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnJobCancelled)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockVirtualMachine)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyentry)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVirtualMachineHostStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, vmhost: Param0, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnVirtualMachineHostStatusChanged)(::core::mem::transmute_copy(self), vmhost.into_param().abi(), ::core::mem::transmute(vmhostnotifystatus), ::core::mem::transmute(errorcode), errordescr.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbProvisioningPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbProvisioningPluginNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbProvisioningPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbProvisioningPluginNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbProvisioningPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbProvisioningPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbProvisioningPluginNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbProvisioningPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvisioningPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbProvisioningPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvisioningPluginNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbProvisioningPluginNotifySink {
    type Vtable = ITsSbProvisioningPluginNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaca87a8e_818b_4581_a032_49c3dfb9c701);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioningPluginNotifySink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnJobCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVirtualMachineStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVirtualMachineStatusChanged: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnJobCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultcode: ::windows::core::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnJobCompleted: usize,
    pub OnJobCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LockVirtualMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVirtualMachineHostStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVirtualMachineHostStatusChanged: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbResourceNotification(::windows::core::IUnknown);
impl ITsSbResourceNotification {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionChange<'a, Param1: ::windows::core::IntoParam<'a, ITsSbSession>>(&self, changetype: TSSESSION_STATE, psession: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), psession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyTargetChange<'a, Param1: ::windows::core::IntoParam<'a, ITsSbTarget>>(&self, targetchangetype: u32, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyTargetChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetchangetype), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyClientConnectionStateChange<'a, Param1: ::windows::core::IntoParam<'a, ITsSbClientConnection>>(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyClientConnectionStateChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), pconnection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbResourceNotification> for ::windows::core::IUnknown {
    fn from(value: ITsSbResourceNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbResourceNotification> for ::windows::core::IUnknown {
    fn from(value: &ITsSbResourceNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbResourceNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbResourceNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbResourceNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbResourceNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourceNotification {}
impl ::core::fmt::Debug for ITsSbResourceNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourceNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbResourceNotification {
    type Vtable = ITsSbResourceNotification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65d3e85a_c39b_11dc_b92d_3cd255d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotification_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NotifySessionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyTargetChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyClientConnectionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbResourceNotificationEx(::windows::core::IUnknown);
impl ITsSbResourceNotificationEx {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifySessionChangeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, username: Param1, domain: Param2, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionChangeEx)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(sessionstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifyTargetChangeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, targetchangetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyTargetChangeEx)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(targetchangetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifyClientConnectionStateChangeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, username: Param0, domain: Param1, initialprogram: Param2, poolname: Param3, targetname: Param4, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyClientConnectionStateChangeEx)(::core::mem::transmute_copy(self), username.into_param().abi(), domain.into_param().abi(), initialprogram.into_param().abi(), poolname.into_param().abi(), targetname.into_param().abi(), ::core::mem::transmute(connectionchangetype)).ok()
    }
}
impl ::core::convert::From<ITsSbResourceNotificationEx> for ::windows::core::IUnknown {
    fn from(value: ITsSbResourceNotificationEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbResourceNotificationEx> for ::windows::core::IUnknown {
    fn from(value: &ITsSbResourceNotificationEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbResourceNotificationEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbResourceNotificationEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbResourceNotificationEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbResourceNotificationEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourceNotificationEx {}
impl ::core::fmt::Debug for ITsSbResourceNotificationEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourceNotificationEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbResourceNotificationEx {
    type Vtable = ITsSbResourceNotificationEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8a47fde_ca91_44d2_b897_3aa28a43b2b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotificationEx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySessionChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySessionChangeEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyTargetChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyTargetChangeEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyClientConnectionStateChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyClientConnectionStateChangeEx: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbResourcePlugin(::windows::core::IUnknown);
impl ITsSbResourcePlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
impl ::core::convert::From<ITsSbResourcePlugin> for ::windows::core::IUnknown {
    fn from(value: ITsSbResourcePlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbResourcePlugin> for ::windows::core::IUnknown {
    fn from(value: &ITsSbResourcePlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbResourcePlugin> for ITsSbPlugin {
    fn from(value: ITsSbResourcePlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbResourcePlugin> for ITsSbPlugin {
    fn from(value: &ITsSbResourcePlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbResourcePlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbResourcePlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourcePlugin {}
impl ::core::fmt::Debug for ITsSbResourcePlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourcePlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbResourcePlugin {
    type Vtable = ITsSbResourcePlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea8db42c_98ed_4535_a88b_2a164f35490f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePlugin_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbResourcePluginStore(::windows::core::IUnknown);
impl ITsSbResourcePluginStore {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, farmname: Param1) -> ::windows::core::Result<ITsSbTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryTarget)(::core::mem::transmute_copy(self), targetname.into_param().abi(), farmname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbTarget>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QuerySessionBySessionId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dwsessionid: u32, targetname: Param1) -> ::windows::core::Result<ITsSbSession> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuerySessionBySessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsessionid), targetname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbSession>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AddTargetToStore<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTarget>>(&self, ptarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTargetToStore)(::core::mem::transmute_copy(self), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AddSessionToStore<'a, Param0: ::windows::core::IntoParam<'a, ITsSbSession>>(&self, psession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSessionToStore)(::core::mem::transmute_copy(self), psession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AddEnvironmentToStore<'a, Param0: ::windows::core::IntoParam<'a, ITsSbEnvironment>>(&self, penvironment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddEnvironmentToStore)(::core::mem::transmute_copy(self), penvironment.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveEnvironmentFromStore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, environmentname: Param0, bignoreowner: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveEnvironmentFromStore)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), bignoreowner.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateFarms)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryEnvironment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, environmentname: Param0) -> ::windows::core::Result<ITsSbEnvironment> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryEnvironment)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateEnvironments)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveTarget<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTarget>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ptarget: Param0, bforcewrite: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveTarget)(::core::mem::transmute_copy(self), ptarget.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveEnvironment<'a, Param0: ::windows::core::IntoParam<'a, ITsSbEnvironment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, penvironment: Param0, bforcewrite: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveEnvironment)(::core::mem::transmute_copy(self), penvironment.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SaveSession<'a, Param0: ::windows::core::IntoParam<'a, ITsSbSession>>(&self, psession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveSession)(::core::mem::transmute_copy(self), psession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTargetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetProperty)(::core::mem::transmute_copy(self), targetname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEnvironmentProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, environmentname: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironmentProperty)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, newstate: TARGET_STATE) -> ::windows::core::Result<TARGET_STATE> {
        let mut result__: TARGET_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetTargetState)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(newstate), ::core::mem::transmute(&mut result__)).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetSessionState<'a, Param0: ::windows::core::IntoParam<'a, ITsSbSession>>(&self, sbsession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionState)(::core::mem::transmute_copy(self), sbsession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateTargets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, envname: Param1, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: Param3, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateTargets)(::core::mem::transmute_copy(self), farmname.into_param().abi(), envname.into_param().abi(), ::core::mem::transmute(sortbyfieldid), sortybypropname.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateSessions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, username: Param1, userdomain: Param2, poolname: Param3, initialprogram: Param4, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateSessions)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), userdomain.into_param().abi(), poolname.into_param().abi(), initialprogram.into_param().abi(), ::core::mem::transmute(psessionstate), ::core::mem::transmute(pdwcount), ::core::mem::transmute(ppval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFarmProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, propertyname: Param1, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFarmProperty)(::core::mem::transmute_copy(self), farmname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, hostname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteTarget)(::core::mem::transmute_copy(self), targetname.into_param().abi(), hostname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTargetPropertyWithVersionCheck<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTarget>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, ptarget: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetPropertyWithVersionCheck)(::core::mem::transmute_copy(self), ptarget.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEnvironmentPropertyWithVersionCheck<'a, Param0: ::windows::core::IntoParam<'a, ITsSbEnvironment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, penvironment: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironmentPropertyWithVersionCheck)(::core::mem::transmute_copy(self), penvironment.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireTargetLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, dwtimeout: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AcquireTargetLock)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ReleaseTargetLock<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseTargetLock)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TestAndSetServerState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, serverfqdn: Param1, newstate: TARGET_STATE, teststate: TARGET_STATE) -> ::windows::core::Result<TARGET_STATE> {
        let mut result__: TARGET_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TestAndSetServerState)(::core::mem::transmute_copy(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), ::core::mem::transmute(newstate), ::core::mem::transmute(teststate), ::core::mem::transmute(&mut result__)).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServerWaitingToStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, servername: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetServerWaitingToStart)(::core::mem::transmute_copy(self), poolname.into_param().abi(), servername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServerState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, serverfqdn: Param1) -> ::windows::core::Result<TARGET_STATE> {
        let mut result__: TARGET_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetServerState)(::core::mem::transmute_copy(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServerDrainMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serverfqdn: Param0, drainmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetServerDrainMode)(::core::mem::transmute_copy(self), serverfqdn.into_param().abi(), ::core::mem::transmute(drainmode)).ok()
    }
}
impl ::core::convert::From<ITsSbResourcePluginStore> for ::windows::core::IUnknown {
    fn from(value: ITsSbResourcePluginStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbResourcePluginStore> for ::windows::core::IUnknown {
    fn from(value: &ITsSbResourcePluginStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbResourcePluginStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbResourcePluginStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbResourcePluginStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbResourcePluginStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourcePluginStore {}
impl ::core::fmt::Debug for ITsSbResourcePluginStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourcePluginStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbResourcePluginStore {
    type Vtable = ITsSbResourcePluginStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c38f65f_bcf1_4036_a6bf_9e3cccae0b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePluginStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QuerySessionBySessionId: usize,
    pub AddTargetToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddSessionToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddEnvironmentToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveEnvironmentFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveEnvironmentFromStore: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryEnvironment: usize,
    pub EnumerateEnvironments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveEnvironment: usize,
    pub SaveSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTargetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEnvironmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEnvironmentProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetState: usize,
    pub SetSessionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateTargets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateSessions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFarmProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTargetPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTargetPropertyWithVersionCheck: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEnvironmentPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEnvironmentPropertyWithVersionCheck: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireTargetLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireTargetLock: usize,
    pub ReleaseTargetLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TestAndSetServerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestAndSetServerState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServerWaitingToStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServerWaitingToStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetServerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetServerState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServerDrainMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServerDrainMode: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbServiceNotification(::windows::core::IUnknown);
impl ITsSbServiceNotification {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyServiceFailure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyServiceFailure)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyServiceSuccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyServiceSuccess)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITsSbServiceNotification> for ::windows::core::IUnknown {
    fn from(value: ITsSbServiceNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbServiceNotification> for ::windows::core::IUnknown {
    fn from(value: &ITsSbServiceNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbServiceNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbServiceNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbServiceNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbServiceNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbServiceNotification {}
impl ::core::fmt::Debug for ITsSbServiceNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbServiceNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbServiceNotification {
    type Vtable = ITsSbServiceNotification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86cb68ae_86e0_4f57_8a64_bb7406bc5550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbServiceNotification_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NotifyServiceFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyServiceSuccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbSession(::windows::core::IUnknown);
impl ITsSbSession {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SessionId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetName)(::core::mem::transmute_copy(self), targetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Username(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Username)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn State(&self) -> ::windows::core::Result<TSSESSION_STATE> {
        let mut result__: TSSESSION_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TSSESSION_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetState(&self, state: TSSESSION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreateTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, time: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreateTime)(::core::mem::transmute_copy(self), time.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisconnectTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisconnectTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, time: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisconnectTime)(::core::mem::transmute_copy(self), time.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitialProgram(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialProgram)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInitialProgram<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, application: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialProgram)(::core::mem::transmute_copy(self), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ClientDisplay(&self) -> ::windows::core::Result<CLIENT_DISPLAY> {
        let mut result__: CLIENT_DISPLAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClientDisplay)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CLIENT_DISPLAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetClientDisplay<'a, Param0: ::windows::core::IntoParam<'a, CLIENT_DISPLAY>>(&self, pclientdisplay: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientDisplay)(::core::mem::transmute_copy(self), pclientdisplay.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ProtocolType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProtocolType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetProtocolType(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtocolType)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
}
impl ::core::convert::From<ITsSbSession> for ::windows::core::IUnknown {
    fn from(value: ITsSbSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbSession> for ::windows::core::IUnknown {
    fn from(value: &ITsSbSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbSession {}
impl ::core::fmt::Debug for ITsSbSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbSession {
    type Vtable = ITsSbSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd453aac7_b1d8_4c5e_ba34_9afb4c8c5510);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbSession_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Username: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisconnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisconnectTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, app: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitialProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInitialProgram: usize,
    pub ClientDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows::core::HRESULT,
    pub SetClientDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows::core::HRESULT,
    pub ProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    pub SetProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbTarget(::windows::core::IUnknown);
impl ITsSbTarget {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetName)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FarmName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FarmName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFarmName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFarmName)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetFQDN(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetFQDN)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetFQDN<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetFQDN)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetNetbios(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetNetbios)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetNetbios<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetNetbios)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IpAddresses)(::core::mem::transmute_copy(self), ::core::mem::transmute(sockaddr), ::core::mem::transmute(numaddresses)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetIpAddresses(&self, sockaddr: &[TSSD_ConnectionPoint]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpAddresses)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(sockaddr)), sockaddr.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn TargetState(&self) -> ::windows::core::Result<TARGET_STATE> {
        let mut result__: TARGET_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetTargetState(&self, state: TARGET_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn TargetPropertySet(&self) -> ::windows::core::Result<ITsSbTargetPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetPropertySet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITsSbTargetPropertySet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetTargetPropertySet<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTargetPropertySet>>(&self, pval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetPropertySet)(::core::mem::transmute_copy(self), pval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnvironmentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnvironmentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnvironmentName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironmentName)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NumSessions(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NumSessions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NumPendingConnections(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NumPendingConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn TargetLoad(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetLoad)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITsSbTarget> for ::windows::core::IUnknown {
    fn from(value: ITsSbTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTarget> for ::windows::core::IUnknown {
    fn from(value: &ITsSbTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTarget {}
impl ::core::fmt::Debug for ITsSbTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbTarget {
    type Vtable = ITsSbTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16616ecc_272d_411d_b324_126893033856);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTarget_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FarmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFarmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetFQDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetfqdnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetFQDN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetFQDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetFQDN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetNetbios: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetNetbios: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetNetbios: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetNetbios: usize,
    pub IpAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::HRESULT,
    pub SetIpAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::core::HRESULT,
    pub TargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT,
    pub SetTargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub TargetPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    TargetPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetTargetPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetTargetPropertySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnvironmentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnvironmentName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnvironmentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnvironmentName: usize,
    pub NumSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows::core::HRESULT,
    pub NumPendingConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows::core::HRESULT,
    pub TargetLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
pub struct ITsSbTargetPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTargetPropertySet {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Write)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbTargetPropertySet> for ::windows::core::IUnknown {
    fn from(value: ITsSbTargetPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbTargetPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ITsSbTargetPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbTargetPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: ITsSbTargetPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbTargetPropertySet> for super::Com::StructuredStorage::IPropertyBag {
    fn from(value: &ITsSbTargetPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &'a ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<ITsSbTargetPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbTargetPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::convert::From<&ITsSbTargetPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbTargetPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::core::IntoParam<'a, ITsSbPropertySet> for &'a ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPropertySet> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for ITsSbTargetPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbTargetPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbTargetPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbTargetPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTargetPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Interface for ITsSbTargetPropertySet {
    type Vtable = ITsSbTargetPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7bda5d6_994c_4e11_a079_2763b61830ac);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTargetPropertySet_Vtbl {
    pub base: ITsSbPropertySet_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbTaskInfo(::windows::core::IUnknown);
impl ITsSbTaskInfo {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TargetId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StartTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EndTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Deadline)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Identifier(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Identifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Context(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Context)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Plugin(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Plugin)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<RDV_TASK_STATUS> {
        let mut result__: RDV_TASK_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<RDV_TASK_STATUS>(result__)
    }
}
impl ::core::convert::From<ITsSbTaskInfo> for ::windows::core::IUnknown {
    fn from(value: ITsSbTaskInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTaskInfo> for ::windows::core::IUnknown {
    fn from(value: &ITsSbTaskInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbTaskInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbTaskInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbTaskInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskInfo {}
impl ::core::fmt::Debug for ITsSbTaskInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbTaskInfo {
    type Vtable = ITsSbTaskInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x523d1083_89be_48dd_99ea_04e82ffa7265);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deadline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Identifier: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Context: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Plugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplugin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Plugin: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbTaskPlugin(::windows::core::IUnknown);
impl ITsSbTaskPlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ITsSbProvider>, Param1: ::windows::core::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::core::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn InitializeTaskPlugin<'a, Param0: ::windows::core::IntoParam<'a, ITsSbTaskPluginNotifySink>>(&self, pitssbtaskpluginnotifysink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeTaskPlugin)(::core::mem::transmute_copy(self), pitssbtaskpluginnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pszhostname: Param0, pitssbtaskinfo: &[::core::option::Option<ITsSbTaskInfo>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskQueue)(::core::mem::transmute_copy(self), pszhostname.into_param().abi(), pitssbtaskinfo.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pitssbtaskinfo))).ok()
    }
}
impl ::core::convert::From<ITsSbTaskPlugin> for ::windows::core::IUnknown {
    fn from(value: ITsSbTaskPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTaskPlugin> for ::windows::core::IUnknown {
    fn from(value: &ITsSbTaskPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbTaskPlugin> for ITsSbPlugin {
    fn from(value: ITsSbTaskPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTaskPlugin> for ITsSbPlugin {
    fn from(value: &ITsSbTaskPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbPlugin> for &'a ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbTaskPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskPlugin {}
impl ::core::fmt::Debug for ITsSbTaskPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbTaskPlugin {
    type Vtable = ITsSbTaskPlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa22ef0f_8705_41be_93bc_44bdbcf1c9c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPlugin_Vtbl {
    pub base: ITsSbPlugin_Vtbl,
    pub InitializeTaskPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ITsSbTaskPluginNotifySink(::windows::core::IUnknown);
impl ITsSbTaskPluginNotifySink {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnError)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnSetTaskTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        sztargetname: Param0,
        taskstarttime: Param1,
        taskendtime: Param2,
        taskdeadline: Param3,
        sztasklabel: Param4,
        sztaskidentifier: Param5,
        sztaskplugin: Param6,
        dwtaskstatus: u32,
        sacontext: *const super::Com::SAFEARRAY,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetTaskTime)(::core::mem::transmute_copy(self), sztargetname.into_param().abi(), taskstarttime.into_param().abi(), taskendtime.into_param().abi(), taskdeadline.into_param().abi(), sztasklabel.into_param().abi(), sztaskidentifier.into_param().abi(), sztaskplugin.into_param().abi(), ::core::mem::transmute(dwtaskstatus), ::core::mem::transmute(sacontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDeleteTaskTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sztargetname: Param0, sztaskidentifier: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDeleteTaskTime)(::core::mem::transmute_copy(self), sztargetname.into_param().abi(), sztaskidentifier.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUpdateTaskStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sztargetname: Param0, taskidentifier: Param1, taskstatus: RDV_TASK_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUpdateTaskStatus)(::core::mem::transmute_copy(self), sztargetname.into_param().abi(), taskidentifier.into_param().abi(), ::core::mem::transmute(taskstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnReportTasks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, szhostname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReportTasks)(::core::mem::transmute_copy(self), szhostname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITsSbTaskPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITsSbTaskPluginNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTaskPluginNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITsSbTaskPluginNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITsSbTaskPluginNotifySink> for ITsSbBaseNotifySink {
    fn from(value: ITsSbTaskPluginNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTaskPluginNotifySink> for ITsSbBaseNotifySink {
    fn from(value: &ITsSbTaskPluginNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITsSbBaseNotifySink> for &'a ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ITsSbBaseNotifySink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITsSbTaskPluginNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbTaskPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskPluginNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITsSbTaskPluginNotifySink {
    type Vtable = ITsSbTaskPluginNotifySink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aaf899e_c2ec_45ee_aa37_45e60895261a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPluginNotifySink_Vtbl {
    pub base: ITsSbBaseNotifySink_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnSetTaskTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnSetTaskTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDeleteTaskTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDeleteTaskTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUpdateTaskStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUpdateTaskStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnReportTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnReportTasks: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsEnhancedFastReconnectArbitrator(::windows::core::IUnknown);
impl IWRdsEnhancedFastReconnectArbitrator {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSessionForEnhancedFastReconnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(dwsessioncount), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IWRdsEnhancedFastReconnectArbitrator> for ::windows::core::IUnknown {
    fn from(value: IWRdsEnhancedFastReconnectArbitrator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsEnhancedFastReconnectArbitrator> for ::windows::core::IUnknown {
    fn from(value: &IWRdsEnhancedFastReconnectArbitrator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsEnhancedFastReconnectArbitrator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsEnhancedFastReconnectArbitrator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsEnhancedFastReconnectArbitrator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsEnhancedFastReconnectArbitrator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsEnhancedFastReconnectArbitrator {}
impl ::core::fmt::Debug for IWRdsEnhancedFastReconnectArbitrator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsEnhancedFastReconnectArbitrator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsEnhancedFastReconnectArbitrator {
    type Vtable = IWRdsEnhancedFastReconnectArbitrator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5718ae9b_47f2_499f_b634_d8175bd51131);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSessionForEnhancedFastReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsGraphicsChannel(::windows::core::IUnknown);
impl IWRdsGraphicsChannel {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Write<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, cbsize: u32, pbuffer: *const u8, pcontext: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, IWRdsGraphicsChannelEvents>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pchannelevents: Param0, popencontext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), pchannelevents.into_param().abi(), popencontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWRdsGraphicsChannel> for ::windows::core::IUnknown {
    fn from(value: IWRdsGraphicsChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannel> for ::windows::core::IUnknown {
    fn from(value: &IWRdsGraphicsChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsGraphicsChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsGraphicsChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsGraphicsChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannel {}
impl ::core::fmt::Debug for IWRdsGraphicsChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsGraphicsChannel {
    type Vtable = IWRdsGraphicsChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x684b7a0b_edff_43ad_d5a2_4a8d5388f401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannel_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannelevents: ::windows::core::RawPtr, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsGraphicsChannelEvents(::windows::core::IUnknown);
impl IWRdsGraphicsChannelEvents {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataReceived)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClose)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnChannelOpened<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, openresult: ::windows::core::HRESULT, popencontext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChannelOpened)(::core::mem::transmute_copy(self), ::core::mem::transmute(openresult), popencontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDataSent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwritecontext: Param0, bcancelled: Param1, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataSent)(::core::mem::transmute_copy(self), pwritecontext.into_param().abi(), bcancelled.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMetricsUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(bandwidth), ::core::mem::transmute(rtt), ::core::mem::transmute(lastsentbyteindex)).ok()
    }
}
impl ::core::convert::From<IWRdsGraphicsChannelEvents> for ::windows::core::IUnknown {
    fn from(value: IWRdsGraphicsChannelEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannelEvents> for ::windows::core::IUnknown {
    fn from(value: &IWRdsGraphicsChannelEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsGraphicsChannelEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsGraphicsChannelEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsGraphicsChannelEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannelEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannelEvents {}
impl ::core::fmt::Debug for IWRdsGraphicsChannelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannelEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsGraphicsChannelEvents {
    type Vtable = IWRdsGraphicsChannelEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67f2368c_d674_4fae_66a5_d20628a640d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnChannelOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openresult: ::windows::core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDataSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDataSent: usize,
    pub OnMetricsUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsGraphicsChannelManager(::windows::core::IUnknown);
impl IWRdsGraphicsChannelManager {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> ::windows::core::Result<IWRdsGraphicsChannel> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateChannel)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszchannelname), ::core::mem::transmute(channeltype), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsGraphicsChannel>(result__)
    }
}
impl ::core::convert::From<IWRdsGraphicsChannelManager> for ::windows::core::IUnknown {
    fn from(value: IWRdsGraphicsChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannelManager> for ::windows::core::IUnknown {
    fn from(value: &IWRdsGraphicsChannelManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsGraphicsChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsGraphicsChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsGraphicsChannelManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannelManager {}
impl ::core::fmt::Debug for IWRdsGraphicsChannelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannelManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsGraphicsChannelManager {
    type Vtable = IWRdsGraphicsChannelManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd57159_e83e_476a_a8b9_4a7976e71e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolConnection(::windows::core::IUnknown);
impl IWRdsProtocolConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows::core::Result<IWRdsProtocolLogonErrorRedirector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLogonErrorRedirector)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsProtocolLogonErrorRedirector>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AcceptConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcceptConnection)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientData(&self) -> ::windows::core::Result<WTS_CLIENT_DATA> {
        let mut result__: WTS_CLIENT_DATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClientData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_CLIENT_DATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClientMonitorData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnummonitors), ::core::mem::transmute(pprimarymonitor)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetUserCredentials(&self) -> ::windows::core::Result<WTS_USER_CREDENTIAL> {
        let mut result__: WTS_USER_CREDENTIAL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserCredentials)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_USER_CREDENTIAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLicenseConnection(&self) -> ::windows::core::Result<IWRdsProtocolLicenseConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLicenseConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsProtocolLicenseConnection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AuthenticateClientToSession(&self) -> ::windows::core::Result<WTS_SESSION_ID> {
        let mut result__: WTS_SESSION_ID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticateClientToSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_SESSION_ID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifySessionId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), sessionhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputHandles)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkeyboardhandle), ::core::mem::transmute(pmousehandle), ::core::mem::transmute(pbeephandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVideoHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__: super::super::Foundation::HANDLE_PTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVideoHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE_PTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectNotify)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserAllowedToLogon<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, sessionid: u32, usertoken: Param1, pdomainname: Param2, pusername: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsUserAllowedToLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionArbitrationEnumeration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, husertoken: Param0, bsinglesessionperuserenabled: Param1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionArbitrationEnumeration)(::core::mem::transmute_copy(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(pdwsessionidentifiercount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogonNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hclienttoken: Param0, wszusername: Param1, wszdomainname: Param2, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogonNotify)(::core::mem::transmute_copy(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(pwrdsconnectionsettings)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn PreDisconnect(&self, disconnectreason: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreDisconnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(disconnectreason)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DisconnectNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectNotify)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetProtocolStatus(&self) -> ::windows::core::Result<WTS_PROTOCOL_STATUS> {
        let mut result__: WTS_PROTOCOL_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProtocolStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_PROTOCOL_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLastInputTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastInputTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualChannel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, szendpointname: Param0, bstatic: Param1, requestedpriority: u32) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualChannel)(::core::mem::transmute_copy(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), ::core::mem::transmute(requestedpriority), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn QueryProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, querytype: Param0, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryProperty)(::core::mem::transmute_copy(self), querytype.into_param().abi(), ppropertyentriesin.len() as _, ppropertyentriesout.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppropertyentriesin)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppropertyentriesout))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetShadowConnection(&self) -> ::windows::core::Result<IWRdsProtocolShadowConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetShadowConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsProtocolShadowConnection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyCommandProcessCreated(&self, sessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyCommandProcessCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolConnection> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolConnection> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnection {}
impl ::core::fmt::Debug for IWRdsProtocolConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolConnection {
    type Vtable = IWRdsProtocolConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324ed94f_fdaf_4ff6_81a8_42abe755830b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetClientMonitorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::HRESULT,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySessionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputHandles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVideoHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVideoHandle: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows::core::PCWSTR, pusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows::core::PCWSTR, wszdomainname: ::windows::core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    pub PreDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows::core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szendpointname: ::windows::core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyCommandProcessCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolConnectionCallback(::windows::core::IUnknown);
impl IWRdsProtocolConnectionCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReady(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReady)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BrokenConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(reason), ::core::mem::transmute(source)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopScreenUpdates(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopScreenUpdates)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RedrawWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetConnectionId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWRdsProtocolConnectionCallback> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolConnectionCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolConnectionCallback> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolConnectionCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolConnectionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolConnectionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolConnectionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnectionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnectionCallback {}
impl ::core::fmt::Debug for IWRdsProtocolConnectionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnectionCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolConnectionCallback {
    type Vtable = IWRdsProtocolConnectionCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1d70332_d070_4ef1_a088_78313536c2d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolConnectionSettings(::windows::core::IUnknown);
impl IWRdsProtocolConnectionSettings {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetConnectionSetting<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, propertyid: Param0, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectionSetting)(::core::mem::transmute_copy(self), propertyid.into_param().abi(), ::core::mem::transmute(ppropertyentriesin)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetConnectionSetting<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, propertyid: Param0) -> ::windows::core::Result<WTS_PROPERTY_VALUE> {
        let mut result__: WTS_PROPERTY_VALUE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionSetting)(::core::mem::transmute_copy(self), propertyid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WTS_PROPERTY_VALUE>(result__)
    }
}
impl ::core::convert::From<IWRdsProtocolConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnectionSettings {}
impl ::core::fmt::Debug for IWRdsProtocolConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolConnectionSettings {
    type Vtable = IWRdsProtocolConnectionSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83fcf5d3_f6f4_ea94_9cd2_32f280e1e510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionSettings_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetConnectionSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    pub GetConnectionSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolLicenseConnection(::windows::core::IUnknown);
impl IWRdsProtocolLicenseConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestLicensingCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplicensecapabilities), ::core::mem::transmute(pcblicensecapabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendClientLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pclientlicense)), pclientlicense.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestClientLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(reserve1)), reserve1.len() as _, ::core::mem::transmute(ppclientlicense), ::core::mem::transmute(pcbclientlicense)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProtocolComplete)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcomplete)).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolLicenseConnection> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolLicenseConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolLicenseConnection> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolLicenseConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolLicenseConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolLicenseConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolLicenseConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolLicenseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolLicenseConnection {}
impl ::core::fmt::Debug for IWRdsProtocolLicenseConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolLicenseConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolLicenseConnection {
    type Vtable = IWRdsProtocolLicenseConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d6a145f_d095_4424_957a_407fae822d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLicenseConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolListener(::windows::core::IUnknown);
impl IWRdsProtocolListener {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> ::windows::core::Result<WRDS_LISTENER_SETTINGS> {
        let mut result__: WRDS_LISTENER_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrdslistenersettinglevel), ::core::mem::transmute(&mut result__)).from_abi::<WRDS_LISTENER_SETTINGS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StartListen<'a, Param0: ::windows::core::IntoParam<'a, IWRdsProtocolListenerCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartListen)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopListen(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopListen)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolListener> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolListener> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolListener {}
impl ::core::fmt::Debug for IWRdsProtocolListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolListener {
    type Vtable = IWRdsProtocolListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcbc131b_c686_451d_a773_e279e230f540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListener_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows::core::HRESULT,
    pub StartListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolListenerCallback(::windows::core::IUnknown);
impl IWRdsProtocolListenerCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnConnected<'a, Param0: ::windows::core::IntoParam<'a, IWRdsProtocolConnection>>(&self, pconnection: Param0, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<IWRdsProtocolConnectionCallback> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OnConnected)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), ::core::mem::transmute(pwrdsconnectionsettings), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsProtocolConnectionCallback>(result__)
    }
}
impl ::core::convert::From<IWRdsProtocolListenerCallback> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolListenerCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolListenerCallback> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolListenerCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolListenerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolListenerCallback {}
impl ::core::fmt::Debug for IWRdsProtocolListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolListenerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolListenerCallback {
    type Vtable = IWRdsProtocolListenerCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab27e5b_4449_4dc1_b74a_91621d4fe984);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListenerCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnConnected: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolLogonErrorRedirector(::windows::core::IUnknown);
impl IWRdsProtocolLogonErrorRedirector {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnBeginPainting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBeginPainting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmessage: Param0) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectStatus)(::core::mem::transmute_copy(self), pszmessage.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszcaption: Param0, pszmessage: Param1, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectMessage)(::core::mem::transmute_copy(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectLogonError<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: Param2, pszmessage: Param3, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectLogonError)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntsstatus), ::core::mem::transmute(ntssubstatus), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
}
impl ::core::convert::From<IWRdsProtocolLogonErrorRedirector> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolLogonErrorRedirector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolLogonErrorRedirector> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolLogonErrorRedirector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolLogonErrorRedirector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolLogonErrorRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolLogonErrorRedirector {}
impl ::core::fmt::Debug for IWRdsProtocolLogonErrorRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolLogonErrorRedirector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolLogonErrorRedirector {
    type Vtable = IWRdsProtocolLogonErrorRedirector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x519fe83b_142a_4120_a3d5_a405d315281a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmessage: ::windows::core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows::core::PCWSTR, pszmessage: ::windows::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows::core::PCWSTR, pszmessage: ::windows::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolManager(::windows::core::IUnknown);
impl IWRdsProtocolManager {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWRdsProtocolSettings>>(&self, piwrdssettings: Param0, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), piwrdssettings.into_param().abi(), ::core::mem::transmute(pwrdssettings)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn CreateListener<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wszlistenername: Param0) -> ::windows::core::Result<IWRdsProtocolListener> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateListener)(::core::mem::transmute_copy(self), wszlistenername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWRdsProtocolListener>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyServiceStateChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptsservicestatechange)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionOfServiceStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionOfServiceStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionStateChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), ::core::mem::transmute(eventid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySettingsChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwrdssettings)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Uninitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolManager> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolManager> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolManager {}
impl ::core::fmt::Debug for IWRdsProtocolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolManager {
    type Vtable = IWRdsProtocolManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc796967_3abb_40cd_a446_105276b58950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwrdssettings: ::windows::core::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlistenername: ::windows::core::PCWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySettingsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySettingsChange: usize,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolSettings(::windows::core::IUnknown);
impl IWRdsProtocolSettings {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL) -> ::windows::core::Result<WRDS_SETTINGS> {
        let mut result__: WRDS_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrdssettingtype), ::core::mem::transmute(wrdssettinglevel), ::core::mem::transmute(&mut result__)).from_abi::<WRDS_SETTINGS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MergeSettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwrdssettings), ::core::mem::transmute(wrdsconnectionsettinglevel), ::core::mem::transmute(pwrdsconnectionsettings)).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolSettings> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolSettings> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolSettings {}
impl ::core::fmt::Debug for IWRdsProtocolSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolSettings {
    type Vtable = IWRdsProtocolSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x654a5a6a_2550_47eb_b6f7_ebd637475265);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolSettings_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MergeSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MergeSettings: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolShadowCallback(::windows::core::IUnknown);
impl IWRdsProtocolShadowCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopShadow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopShadow)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn InvokeTargetShadow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param10: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ptargetservername: Param0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: Param10) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvokeTargetShadow)(
            ::core::mem::transmute_copy(self),
            ptargetservername.into_param().abi(),
            ::core::mem::transmute(targetsessionid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam1)),
            pparam1.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam2)),
            pparam2.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam3)),
            pparam3.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam4)),
            pparam4.len() as _,
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IWRdsProtocolShadowCallback> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolShadowCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolShadowCallback> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolShadowCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolShadowCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolShadowCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolShadowCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolShadowCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolShadowCallback {}
impl ::core::fmt::Debug for IWRdsProtocolShadowCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolShadowCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolShadowCallback {
    type Vtable = IWRdsProtocolShadowCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0667ce0_0372_40d6_adb2_a0f3322674d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StopShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows::core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsProtocolShadowConnection(::windows::core::IUnknown);
impl IWRdsProtocolShadowConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, IWRdsProtocolShadowCallback>>(&self, ptargetservername: Param0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self), ptargetservername.into_param().abi(), ::core::mem::transmute(targetsessionid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers), pshadowcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DoTarget<'a, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam1)), pparam1.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam2)), pparam2.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam3)), pparam3.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam4)), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWRdsProtocolShadowConnection> for ::windows::core::IUnknown {
    fn from(value: IWRdsProtocolShadowConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsProtocolShadowConnection> for ::windows::core::IUnknown {
    fn from(value: &IWRdsProtocolShadowConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsProtocolShadowConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsProtocolShadowConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsProtocolShadowConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolShadowConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolShadowConnection {}
impl ::core::fmt::Debug for IWRdsProtocolShadowConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolShadowConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsProtocolShadowConnection {
    type Vtable = IWRdsProtocolShadowConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ae85ce6_cade_4548_8feb_99016597f60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows::core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWRdsWddmIddProps(::windows::core::IUnknown);
impl IWRdsWddmIddProps {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHardwareId)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdisplaydriverhardwareid)), pdisplaydriverhardwareid.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDriverLoad<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, sessionid: u32, driverhandle: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDriverLoad)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), driverhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDriverUnload)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableWddmIdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enabled: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableWddmIdd)(::core::mem::transmute_copy(self), enabled.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWRdsWddmIddProps> for ::windows::core::IUnknown {
    fn from(value: IWRdsWddmIddProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWRdsWddmIddProps> for ::windows::core::IUnknown {
    fn from(value: &IWRdsWddmIddProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWRdsWddmIddProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWRdsWddmIddProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWRdsWddmIddProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWRdsWddmIddProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsWddmIddProps {}
impl ::core::fmt::Debug for IWRdsWddmIddProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsWddmIddProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWRdsWddmIddProps {
    type Vtable = IWRdsWddmIddProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1382df4d_a289_43d1_a184_144726f9af90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsWddmIddProps_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: ::windows::core::PCWSTR, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDriverLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDriverLoad: usize,
    pub OnDriverUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableWddmIdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableWddmIdd: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSBitmapRenderService(::windows::core::IUnknown);
impl IWTSBitmapRenderService {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetMappedRenderer<'a, Param1: ::windows::core::IntoParam<'a, IWTSBitmapRendererCallback>>(&self, mappingid: u64, pmappedrenderercallback: Param1) -> ::windows::core::Result<IWTSBitmapRenderer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMappedRenderer)(::core::mem::transmute_copy(self), ::core::mem::transmute(mappingid), pmappedrenderercallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWTSBitmapRenderer>(result__)
    }
}
impl ::core::convert::From<IWTSBitmapRenderService> for ::windows::core::IUnknown {
    fn from(value: IWTSBitmapRenderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSBitmapRenderService> for ::windows::core::IUnknown {
    fn from(value: &IWTSBitmapRenderService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSBitmapRenderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSBitmapRenderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSBitmapRenderService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRenderService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRenderService {}
impl ::core::fmt::Debug for IWTSBitmapRenderService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRenderService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSBitmapRenderService {
    type Vtable = IWTSBitmapRenderService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetMappedRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: ::windows::core::RawPtr, ppmappedrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSBitmapRenderer(::windows::core::IUnknown);
impl IWTSBitmapRenderer {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Render<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, imageformat: Param0, dwwidth: u32, dwheight: u32, cbstride: i32, pimagebuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Render)(::core::mem::transmute_copy(self), imageformat.into_param().abi(), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(cbstride), pimagebuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pimagebuffer))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetRendererStatistics(&self) -> ::windows::core::Result<BITMAP_RENDERER_STATISTICS> {
        let mut result__: BITMAP_RENDERER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRendererStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<BITMAP_RENDERER_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RemoveMapping(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMapping)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWTSBitmapRenderer> for ::windows::core::IUnknown {
    fn from(value: IWTSBitmapRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSBitmapRenderer> for ::windows::core::IUnknown {
    fn from(value: &IWTSBitmapRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSBitmapRenderer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSBitmapRenderer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSBitmapRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRenderer {}
impl ::core::fmt::Debug for IWTSBitmapRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRenderer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSBitmapRenderer {
    type Vtable = IWTSBitmapRenderer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageformat: ::windows::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows::core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSBitmapRendererCallback(::windows::core::IUnknown);
impl IWTSBitmapRendererCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTargetSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(&self, rcnewsize: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTargetSizeChanged)(::core::mem::transmute_copy(self), rcnewsize.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWTSBitmapRendererCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSBitmapRendererCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSBitmapRendererCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSBitmapRendererCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSBitmapRendererCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSBitmapRendererCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSBitmapRendererCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRendererCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRendererCallback {}
impl ::core::fmt::Debug for IWTSBitmapRendererCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRendererCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSBitmapRendererCallback {
    type Vtable = IWTSBitmapRendererCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRendererCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTargetSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTargetSizeChanged: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSListener(::windows::core::IUnknown);
impl IWTSListener {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetConfiguration(&self) -> ::windows::core::Result<super::Com::StructuredStorage::IPropertyBag> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::StructuredStorage::IPropertyBag>(result__)
    }
}
impl ::core::convert::From<IWTSListener> for ::windows::core::IUnknown {
    fn from(value: IWTSListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSListener> for ::windows::core::IUnknown {
    fn from(value: &IWTSListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSListener {}
impl ::core::fmt::Debug for IWTSListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSListener {
    type Vtable = IWTSListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListener_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetConfiguration: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSListenerCallback(::windows::core::IUnknown);
impl IWTSListenerCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnNewChannelConnection<'a, Param0: ::windows::core::IntoParam<'a, IWTSVirtualChannel>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pchannel: Param0, data: Param1, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::core::option::Option<IWTSVirtualChannelCallback>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNewChannelConnection)(::core::mem::transmute_copy(self), pchannel.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(pbaccept), ::core::mem::transmute(ppcallback)).ok()
    }
}
impl ::core::convert::From<IWTSListenerCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSListenerCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSListenerCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSListenerCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSListenerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSListenerCallback {}
impl ::core::fmt::Debug for IWTSListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSListenerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSListenerCallback {
    type Vtable = IWTSListenerCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListenerCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNewChannelConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNewChannelConnection: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSPlugin(::windows::core::IUnknown);
impl IWTSPlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWTSVirtualChannelManager>>(&self, pchannelmgr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), pchannelmgr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Connected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connected)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdisconnectcode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminated)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWTSPlugin> for ::windows::core::IUnknown {
    fn from(value: IWTSPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSPlugin> for ::windows::core::IUnknown {
    fn from(value: &IWTSPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSPlugin {}
impl ::core::fmt::Debug for IWTSPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSPlugin {
    type Vtable = IWTSPlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230201_1439_4e62_a414_190d0ac3d40e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPlugin_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannelmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows::core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSPluginServiceProvider(::windows::core::IUnknown);
impl IWTSPluginServiceProvider {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetService<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceid: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetService)(::core::mem::transmute_copy(self), serviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IWTSPluginServiceProvider> for ::windows::core::IUnknown {
    fn from(value: IWTSPluginServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSPluginServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &IWTSPluginServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSPluginServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSPluginServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSPluginServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSPluginServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSPluginServiceProvider {}
impl ::core::fmt::Debug for IWTSPluginServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSPluginServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSPluginServiceProvider {
    type Vtable = IWTSPluginServiceProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPluginServiceProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolConnection(::windows::core::IUnknown);
impl IWTSProtocolConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows::core::Result<IWTSProtocolLogonErrorRedirector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLogonErrorRedirector)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWTSProtocolLogonErrorRedirector>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendPolicyData)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppolicydata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AcceptConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcceptConnection)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientData(&self) -> ::windows::core::Result<WTS_CLIENT_DATA> {
        let mut result__: WTS_CLIENT_DATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClientData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_CLIENT_DATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetUserCredentials(&self) -> ::windows::core::Result<WTS_USER_CREDENTIAL> {
        let mut result__: WTS_USER_CREDENTIAL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserCredentials)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_USER_CREDENTIAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLicenseConnection(&self) -> ::windows::core::Result<IWTSProtocolLicenseConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLicenseConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWTSProtocolLicenseConnection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AuthenticateClientToSession(&self) -> ::windows::core::Result<WTS_SESSION_ID> {
        let mut result__: WTS_SESSION_ID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticateClientToSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_SESSION_ID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProtocolHandles)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkeyboardhandle), ::core::mem::transmute(pmousehandle), ::core::mem::transmute(pbeephandle), ::core::mem::transmute(pvideohandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectNotify)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserAllowedToLogon<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, sessionid: u32, usertoken: Param1, pdomainname: Param2, pusername: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsUserAllowedToLogon)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionArbitrationEnumeration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, husertoken: Param0, bsinglesessionperuserenabled: Param1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionArbitrationEnumeration)(::core::mem::transmute_copy(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(pdwsessionidentifiercount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogonNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hclienttoken: Param0, wszusername: Param1, wszdomainname: Param2, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogonNotify)(::core::mem::transmute_copy(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserData)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppolicydata), ::core::mem::transmute(pclientdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DisconnectNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectNotify)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetProtocolStatus(&self) -> ::windows::core::Result<WTS_PROTOCOL_STATUS> {
        let mut result__: WTS_PROTOCOL_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProtocolStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WTS_PROTOCOL_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetLastInputTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastInputTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SendBeep(&self, frequency: u32, duration: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendBeep)(::core::mem::transmute_copy(self), ::core::mem::transmute(frequency), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualChannel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, szendpointname: Param0, bstatic: Param1, requestedpriority: u32) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualChannel)(::core::mem::transmute_copy(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), ::core::mem::transmute(requestedpriority), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn QueryProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, querytype: Param0, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryProperty)(::core::mem::transmute_copy(self), querytype.into_param().abi(), ppropertyentriesin.len() as _, ppropertyentriesout.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppropertyentriesin)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppropertyentriesout))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetShadowConnection(&self) -> ::windows::core::Result<IWTSProtocolShadowConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetShadowConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWTSProtocolShadowConnection>(result__)
    }
}
impl ::core::convert::From<IWTSProtocolConnection> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolConnection> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolConnection {}
impl ::core::fmt::Debug for IWTSProtocolConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolConnection {
    type Vtable = IWTSProtocolConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23083765_9095_4648_98bf_ef81c914032d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendPolicyData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendPolicyData: usize,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT,
    pub NotifySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProtocolHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProtocolHandles: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows::core::PCWSTR, pusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows::core::PCWSTR, wszdomainname: ::windows::core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserData: usize,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT,
    pub SendBeep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szendpointname: ::windows::core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolConnectionCallback(::windows::core::IUnknown);
impl IWTSProtocolConnectionCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnReady(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReady)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BrokenConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(reason), ::core::mem::transmute(source)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopScreenUpdates(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopScreenUpdates)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RedrawWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayIOCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(displayioctl)).ok()
    }
}
impl ::core::convert::From<IWTSProtocolConnectionCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolConnectionCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolConnectionCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolConnectionCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolConnectionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolConnectionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolConnectionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolConnectionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolConnectionCallback {}
impl ::core::fmt::Debug for IWTSProtocolConnectionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolConnectionCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolConnectionCallback {
    type Vtable = IWTSProtocolConnectionCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23083765_75eb_41fe_b4fb_e086242afa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnectionCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT,
    pub DisplayIOCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolLicenseConnection(::windows::core::IUnknown);
impl IWTSProtocolLicenseConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestLicensingCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplicensecapabilities), ::core::mem::transmute(pcblicensecapabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendClientLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pclientlicense)), pclientlicense.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestClientLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(reserve1)), reserve1.len() as _, ::core::mem::transmute(ppclientlicense), ::core::mem::transmute(pcbclientlicense)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProtocolComplete)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcomplete)).ok()
    }
}
impl ::core::convert::From<IWTSProtocolLicenseConnection> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolLicenseConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolLicenseConnection> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolLicenseConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolLicenseConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolLicenseConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolLicenseConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolLicenseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolLicenseConnection {}
impl ::core::fmt::Debug for IWTSProtocolLicenseConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolLicenseConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolLicenseConnection {
    type Vtable = IWTSProtocolLicenseConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23083765_178c_4079_8e4a_fea6496a4d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLicenseConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolListener(::windows::core::IUnknown);
impl IWTSProtocolListener {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StartListen<'a, Param0: ::windows::core::IntoParam<'a, IWTSProtocolListenerCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartListen)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopListen(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopListen)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWTSProtocolListener> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolListener> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolListener {}
impl ::core::fmt::Debug for IWTSProtocolListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolListener {
    type Vtable = IWTSProtocolListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23083765_45f0_4394_8f69_32b2bc0ef4ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListener_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StartListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolListenerCallback(::windows::core::IUnknown);
impl IWTSProtocolListenerCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnConnected<'a, Param0: ::windows::core::IntoParam<'a, IWTSProtocolConnection>>(&self, pconnection: Param0) -> ::windows::core::Result<IWTSProtocolConnectionCallback> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OnConnected)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWTSProtocolConnectionCallback>(result__)
    }
}
impl ::core::convert::From<IWTSProtocolListenerCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolListenerCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolListenerCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolListenerCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolListenerCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolListenerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolListenerCallback {}
impl ::core::fmt::Debug for IWTSProtocolListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolListenerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolListenerCallback {
    type Vtable = IWTSProtocolListenerCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23083765_1a2d_4de2_97de_4a35f260f0b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListenerCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolLogonErrorRedirector(::windows::core::IUnknown);
impl IWTSProtocolLogonErrorRedirector {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnBeginPainting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBeginPainting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmessage: Param0) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectStatus)(::core::mem::transmute_copy(self), pszmessage.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszcaption: Param0, pszmessage: Param1, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectMessage)(::core::mem::transmute_copy(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RedirectLogonError<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: Param2, pszmessage: Param3, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RedirectLogonError)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntsstatus), ::core::mem::transmute(ntssubstatus), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), ::core::mem::transmute(&mut result__)).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
}
impl ::core::convert::From<IWTSProtocolLogonErrorRedirector> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolLogonErrorRedirector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolLogonErrorRedirector> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolLogonErrorRedirector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolLogonErrorRedirector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolLogonErrorRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolLogonErrorRedirector {}
impl ::core::fmt::Debug for IWTSProtocolLogonErrorRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolLogonErrorRedirector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolLogonErrorRedirector {
    type Vtable = IWTSProtocolLogonErrorRedirector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd9b61a7_2916_4627_8dee_4328711ad6cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLogonErrorRedirector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmessage: ::windows::core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows::core::PCWSTR, pszmessage: ::windows::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows::core::PCWSTR, pszmessage: ::windows::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolManager(::windows::core::IUnknown);
impl IWTSProtocolManager {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn CreateListener<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wszlistenername: Param0) -> ::windows::core::Result<IWTSProtocolListener> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateListener)(::core::mem::transmute_copy(self), wszlistenername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWTSProtocolListener>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyServiceStateChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptsservicestatechange)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionOfServiceStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionOfServiceStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifySessionStateChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), ::core::mem::transmute(eventid)).ok()
    }
}
impl ::core::convert::From<IWTSProtocolManager> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolManager> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolManager {}
impl ::core::fmt::Debug for IWTSProtocolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolManager {
    type Vtable = IWTSProtocolManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9eaf6cc_ed79_4f01_821d_1f881b9f66cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlistenername: ::windows::core::PCWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolShadowCallback(::windows::core::IUnknown);
impl IWTSProtocolShadowCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn StopShadow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopShadow)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn InvokeTargetShadow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param10: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ptargetservername: Param0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: Param10) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvokeTargetShadow)(
            ::core::mem::transmute_copy(self),
            ptargetservername.into_param().abi(),
            ::core::mem::transmute(targetsessionid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam1)),
            pparam1.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam2)),
            pparam2.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam3)),
            pparam3.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam4)),
            pparam4.len() as _,
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IWTSProtocolShadowCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolShadowCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolShadowCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolShadowCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolShadowCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolShadowCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolShadowCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolShadowCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolShadowCallback {}
impl ::core::fmt::Debug for IWTSProtocolShadowCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolShadowCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolShadowCallback {
    type Vtable = IWTSProtocolShadowCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x503a2504_aae5_4ab1_93e0_6d1c4bc6f71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StopShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows::core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSProtocolShadowConnection(::windows::core::IUnknown);
impl IWTSProtocolShadowConnection {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, IWTSProtocolShadowCallback>>(&self, ptargetservername: Param0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self), ptargetservername.into_param().abi(), ::core::mem::transmute(targetsessionid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers), pshadowcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DoTarget<'a, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam1)), pparam1.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam2)), pparam2.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam3)), pparam3.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pparam4)), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWTSProtocolShadowConnection> for ::windows::core::IUnknown {
    fn from(value: IWTSProtocolShadowConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSProtocolShadowConnection> for ::windows::core::IUnknown {
    fn from(value: &IWTSProtocolShadowConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSProtocolShadowConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSProtocolShadowConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSProtocolShadowConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolShadowConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolShadowConnection {}
impl ::core::fmt::Debug for IWTSProtocolShadowConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolShadowConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSProtocolShadowConnection {
    type Vtable = IWTSProtocolShadowConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3b0c14_37fb_456b_bab3_6d6cd51e13bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows::core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSSBPlugin(::windows::core::IUnknown);
impl IWTSSBPlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WTSSBX_MachineChangeNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), ::core::mem::transmute(machineid), ::core::mem::transmute(pmachineinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, sessioninfo: &[WTSSBX_SESSION_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WTSSBX_SessionChangeNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), ::core::mem::transmute(machineid), sessioninfo.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(sessioninfo))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn WTSSBX_GetMostSuitableServer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, username: Param0, domainname: Param1, applicationtype: Param2, farmname: Param3, pmachineid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WTSSBX_GetMostSuitableServer)(::core::mem::transmute_copy(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), farmname.into_param().abi(), ::core::mem::transmute(pmachineid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Terminated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminated)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn WTSSBX_GetUserExternalSession<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, username: Param0, domainname: Param1, applicationtype: Param2, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WTSSBX_GetUserExternalSession)(::core::mem::transmute_copy(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), ::core::mem::transmute(redirectorinternalip), ::core::mem::transmute(psessionid), ::core::mem::transmute(pmachineconnectinfo)).ok()
    }
}
impl ::core::convert::From<IWTSSBPlugin> for ::windows::core::IUnknown {
    fn from(value: IWTSSBPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSSBPlugin> for ::windows::core::IUnknown {
    fn from(value: &IWTSSBPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSSBPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSSBPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSSBPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSSBPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSSBPlugin {}
impl ::core::fmt::Debug for IWTSSBPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSSBPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSSBPlugin {
    type Vtable = IWTSSBPlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc44be78_b18d_4399_b210_641bf67a002c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSSBPlugin_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub WTSSBX_MachineChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WTSSBX_SessionChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WTSSBX_SessionChangeNotification: usize,
    pub WTSSBX_GetMostSuitableServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::windows::core::PCWSTR, domainname: ::windows::core::PCWSTR, applicationtype: ::windows::core::PCWSTR, farmname: ::windows::core::PCWSTR, pmachineid: *mut i32) -> ::windows::core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WTSSBX_GetUserExternalSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::windows::core::PCWSTR, domainname: ::windows::core::PCWSTR, applicationtype: ::windows::core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSVirtualChannel(::windows::core::IUnknown);
impl IWTSVirtualChannel {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Write<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pbuffer: &[u8], preserved: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), pbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer)), preserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWTSVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: IWTSVirtualChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: &IWTSVirtualChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSVirtualChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannel {}
impl ::core::fmt::Debug for IWTSVirtualChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSVirtualChannel {
    type Vtable = IWTSVirtualChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannel_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSVirtualChannelCallback(::windows::core::IUnknown);
impl IWTSVirtualChannelCallback {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnDataReceived(&self, pbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataReceived)(::core::mem::transmute_copy(self), pbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn OnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClose)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWTSVirtualChannelCallback> for ::windows::core::IUnknown {
    fn from(value: IWTSVirtualChannelCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSVirtualChannelCallback> for ::windows::core::IUnknown {
    fn from(value: &IWTSVirtualChannelCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSVirtualChannelCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSVirtualChannelCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSVirtualChannelCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannelCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannelCallback {}
impl ::core::fmt::Debug for IWTSVirtualChannelCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannelCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSVirtualChannelCallback {
    type Vtable = IWTSVirtualChannelCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWTSVirtualChannelManager(::windows::core::IUnknown);
impl IWTSVirtualChannelManager {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn CreateListener<'a, Param2: ::windows::core::IntoParam<'a, IWTSListenerCallback>>(&self, pszchannelname: *const u8, uflags: u32, plistenercallback: Param2) -> ::windows::core::Result<IWTSListener> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateListener)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszchannelname), ::core::mem::transmute(uflags), plistenercallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWTSListener>(result__)
    }
}
impl ::core::convert::From<IWTSVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: IWTSVirtualChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWTSVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: &IWTSVirtualChannelManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWTSVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWTSVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWTSVirtualChannelManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannelManager {}
impl ::core::fmt::Debug for IWTSVirtualChannelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannelManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWTSVirtualChannelManager {
    type Vtable = IWTSVirtualChannelManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, uflags: u32, plistenercallback: ::windows::core::RawPtr, pplistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspace(::windows::core::IUnknown);
impl IWorkspace {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWorkspaceNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartRemoteApplication)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProcessId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWorkspace> for ::windows::core::IUnknown {
    fn from(value: IWorkspace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace> for ::windows::core::IUnknown {
    fn from(value: &IWorkspace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace {}
impl ::core::fmt::Debug for IWorkspace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWorkspaceNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWorkspaceNames: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub StartRemoteApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    StartRemoteApplication: usize,
    pub GetProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspace2(::windows::core::IUnknown);
impl IWorkspace2 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWorkspaceNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartRemoteApplication)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProcessId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplicationEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrrequestingappid: Param1, bstrrequestingappfamilyname: Param2, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: Param4, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartRemoteApplicationEx)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrrequestingappid.into_param().abi(), bstrrequestingappfamilyname.into_param().abi(), ::core::mem::transmute(blaunchintoimmersiveclient), bstrimmersiveclientactivationcontext.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
}
impl ::core::convert::From<IWorkspace2> for ::windows::core::IUnknown {
    fn from(value: IWorkspace2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace2> for ::windows::core::IUnknown {
    fn from(value: &IWorkspace2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspace2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspace2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWorkspace2> for IWorkspace {
    fn from(value: IWorkspace2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace2> for IWorkspace {
    fn from(value: &IWorkspace2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace> for IWorkspace2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace> for &'a IWorkspace2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspace2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspace2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace2 {}
impl ::core::fmt::Debug for IWorkspace2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspace2 {
    type Vtable = IWorkspace2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96d8d7cf_783e_4286_834c_ebc0e95f783c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace2_Vtbl {
    pub base: IWorkspace_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub StartRemoteApplicationEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    StartRemoteApplicationEx: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspace3(::windows::core::IUnknown);
impl IWorkspace3 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetWorkspaceNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StartRemoteApplication)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetProcessId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplicationEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrrequestingappid: Param1, bstrrequestingappfamilyname: Param2, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: Param4, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartRemoteApplicationEx)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrrequestingappid.into_param().abi(), bstrrequestingappfamilyname.into_param().abi(), ::core::mem::transmute(blaunchintoimmersiveclient), bstrimmersiveclientactivationcontext.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClaimsToken2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(&self, bstrclaimshint: Param0, bstruserhint: Param1, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: Param4) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClaimsToken2)(::core::mem::transmute_copy(self), bstrclaimshint.into_param().abi(), bstruserhint.into_param().abi(), ::core::mem::transmute(claimcookie), ::core::mem::transmute(hwndcreduiparent), rectcreduiparent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClaimsToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccesstoken: Param0, ullaccesstokenexpiration: u64, bstrrefreshtoken: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClaimsToken)(::core::mem::transmute_copy(self), bstraccesstoken.into_param().abi(), ::core::mem::transmute(ullaccesstokenexpiration), bstrrefreshtoken.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWorkspace3> for ::windows::core::IUnknown {
    fn from(value: IWorkspace3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace3> for ::windows::core::IUnknown {
    fn from(value: &IWorkspace3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWorkspace3> for IWorkspace {
    fn from(value: IWorkspace3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace3> for IWorkspace {
    fn from(value: &IWorkspace3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace> for IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace> for &'a IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWorkspace3> for IWorkspace2 {
    fn from(value: IWorkspace3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspace3> for IWorkspace2 {
    fn from(value: &IWorkspace3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace2> for IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspace2> for &'a IWorkspace3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspace2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspace3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspace3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace3 {}
impl ::core::fmt::Debug for IWorkspace3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspace3 {
    type Vtable = IWorkspace3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1becbe4a_d654_423b_afeb_be8d532c13c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace3_Vtbl {
    pub base: IWorkspace2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClaimsToken2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClaimsToken2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClaimsToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClaimsToken: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspaceClientExt(::windows::core::IUnknown);
impl IWorkspaceClientExt {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn IssueDisconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IssueDisconnect)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWorkspaceClientExt> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceClientExt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceClientExt> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceClientExt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceClientExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceClientExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspaceClientExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspaceClientExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceClientExt {}
impl ::core::fmt::Debug for IWorkspaceClientExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceClientExt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspaceClientExt {
    type Vtable = IWorkspaceClientExt_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12b952f4_41ca_4f21_a829_a6d07d9a16e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceClientExt_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceDisplayName: usize,
    pub IssueDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspaceRegistration(::windows::core::IUnknown);
impl IWorkspaceRegistration {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IWorkspaceClientExt>>(&self, punk: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddResource)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection)).ok()
    }
}
impl ::core::convert::From<IWorkspaceRegistration> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceRegistration> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspaceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspaceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceRegistration {}
impl ::core::fmt::Debug for IWorkspaceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspaceRegistration {
    type Vtable = IWorkspaceRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspaceRegistration2(::windows::core::IUnknown);
impl IWorkspaceRegistration2 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IWorkspaceClientExt>>(&self, punk: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AddResource)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddResourceEx<'a, Param0: ::windows::core::IntoParam<'a, IWorkspaceClientExt>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, punk: Param0, bstreventloguploadaddress: Param1, pdwcookie: *mut u32, correlationid: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddResourceEx)(::core::mem::transmute_copy(self), punk.into_param().abi(), bstreventloguploadaddress.into_param().abi(), ::core::mem::transmute(pdwcookie), correlationid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn RemoveResourceEx<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, dwcookieconnection: u32, correlationid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveResourceEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection), correlationid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWorkspaceRegistration2> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceRegistration2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceRegistration2> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceRegistration2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWorkspaceRegistration2> for IWorkspaceRegistration {
    fn from(value: IWorkspaceRegistration2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceRegistration2> for IWorkspaceRegistration {
    fn from(value: &IWorkspaceRegistration2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceRegistration> for IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceRegistration> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceRegistration> for &'a IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceRegistration> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspaceRegistration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspaceRegistration2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceRegistration2 {}
impl ::core::fmt::Debug for IWorkspaceRegistration2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceRegistration2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspaceRegistration2 {
    type Vtable = IWorkspaceRegistration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf59f654_39bb_44d8_94d0_4635728957e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration2_Vtbl {
    pub base: IWorkspaceRegistration_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResourceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResourceEx: usize,
    pub RemoveResourceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct IWorkspaceReportMessage(::windows::core::IUnknown);
impl IWorkspaceReportMessage {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterErrorLogMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterErrorLogMessage)(::core::mem::transmute_copy(self), bstrmessage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsErrorMessageRegistered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrwkspid: Param0, dwerrortype: u32, bstrerrormessagetype: Param2, dwerrorcode: u32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsErrorMessageRegistered)(::core::mem::transmute_copy(self), bstrwkspid.into_param().abi(), ::core::mem::transmute(dwerrortype), bstrerrormessagetype.into_param().abi(), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterErrorEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrwkspid: Param0, dwerrortype: u32, bstrerrormessagetype: Param2, dwerrorcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterErrorEvent)(::core::mem::transmute_copy(self), bstrwkspid.into_param().abi(), ::core::mem::transmute(dwerrortype), bstrerrormessagetype.into_param().abi(), ::core::mem::transmute(dwerrorcode)).ok()
    }
}
impl ::core::convert::From<IWorkspaceReportMessage> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceReportMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceReportMessage> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceReportMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceReportMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceReportMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWorkspaceReportMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWorkspaceReportMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceReportMessage {}
impl ::core::fmt::Debug for IWorkspaceReportMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceReportMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWorkspaceReportMessage {
    type Vtable = IWorkspaceReportMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7c06739_500f_4e8c_99a8_2bd6955899eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceReportMessage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterErrorLogMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterErrorLogMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsErrorMessageRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsErrorMessageRegistered: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterErrorEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterErrorEvent: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWorkspaceResTypeRegistry(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceResTypeRegistry {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddResourceType<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1, bstrlauncher: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddResourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteResourceType<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteResourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRegisteredFileExtensions(&self, fmachinewide: i16) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisteredFileExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceTypeInfo<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModifyResourceType<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1, bstrlauncher: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyResourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceResTypeRegistry> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceResTypeRegistry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceResTypeRegistry> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceResTypeRegistry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceResTypeRegistry> for super::Com::IDispatch {
    fn from(value: IWorkspaceResTypeRegistry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceResTypeRegistry> for super::Com::IDispatch {
    fn from(value: &IWorkspaceResTypeRegistry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWorkspaceResTypeRegistry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceResTypeRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceResTypeRegistry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceResTypeRegistry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceResTypeRegistry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWorkspaceResTypeRegistry {
    type Vtable = IWorkspaceResTypeRegistry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d428c79_6e2e_4351_a361_c0401a03a0ba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceResTypeRegistry_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResourceType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteResourceType: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredFileExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceTypeInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModifyResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModifyResourceType: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWorkspaceScriptable(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1, bstrpassword: Param2, bstrworkspaceparams: Param3, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsWorkspaceCredentialSpecified)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsWorkspaceSSOEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearWorkspaceCredential)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAuthenticated)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectWorkspaceByFriendlyName)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceScriptable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceScriptable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceScriptable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceScriptable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable> for super::Com::IDispatch {
    fn from(value: IWorkspaceScriptable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable> for super::Com::IDispatch {
    fn from(value: &IWorkspaceScriptable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWorkspaceScriptable {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWorkspaceScriptable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWorkspaceScriptable {
    type Vtable = IWorkspaceScriptable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefea49a2_dda5_429d_8f42_b23b92c4c347);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectWorkspace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectWorkspace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkspaceCredentialSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkspaceCredentialSpecified: usize,
    pub IsWorkspaceSSOEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbssoenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearWorkspaceCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearWorkspaceCredential: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectWorkspaceByFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectWorkspaceByFriendlyName: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWorkspaceScriptable2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisconnectWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1, bstrpassword: Param2, bstrworkspaceparams: Param3, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsWorkspaceCredentialSpecified)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsWorkspaceSSOEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ClearWorkspaceCredential)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnAuthenticated)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisconnectWorkspaceByFriendlyName)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspaceEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1, bstrredirectorname: Param2, bstrusername: Param3, bstrpassword: Param4, bstrappcontainer: Param5, bstrworkspaceparams: Param6, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartWorkspaceEx)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResourceDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResourceDismissed)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable2> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceScriptable2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable2> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceScriptable2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable2> for super::Com::IDispatch {
    fn from(value: IWorkspaceScriptable2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable2> for super::Com::IDispatch {
    fn from(value: &IWorkspaceScriptable2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable2> for IWorkspaceScriptable {
    fn from(value: IWorkspaceScriptable2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable2> for IWorkspaceScriptable {
    fn from(value: &IWorkspaceScriptable2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable> for &'a IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWorkspaceScriptable2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWorkspaceScriptable2 {
    type Vtable = IWorkspaceScriptable2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefea49a2_dda5_429d_8f42_b33ba2c4c348);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable2_Vtbl {
    pub base: IWorkspaceScriptable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspaceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspaceEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResourceDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResourceDismissed: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWorkspaceScriptable3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DisconnectWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1, bstrpassword: Param2, bstrworkspaceparams: Param3, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StartWorkspace)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsWorkspaceCredentialSpecified)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsWorkspaceSSOEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ClearWorkspaceCredential)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OnAuthenticated)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DisconnectWorkspaceByFriendlyName)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspaceEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1, bstrredirectorname: Param2, bstrusername: Param3, bstrpassword: Param4, bstrappcontainer: Param5, bstrworkspaceparams: Param6, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartWorkspaceEx)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResourceDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResourceDismissed)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartWorkspaceEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(
        &self,
        bstrworkspaceid: Param0,
        bstrworkspacefriendlyname: Param1,
        bstrredirectorname: Param2,
        bstrusername: Param3,
        bstrpassword: Param4,
        bstrappcontainer: Param5,
        bstrworkspaceparams: Param6,
        ltimeout: i32,
        lflags: i32,
        bstreventloguploadaddress: Param9,
        correlationid: Param10,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartWorkspaceEx2)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags), bstreventloguploadaddress.into_param().abi(), correlationid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable3> for ::windows::core::IUnknown {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable3> for ::windows::core::IUnknown {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable3> for super::Com::IDispatch {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable3> for super::Com::IDispatch {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable3> for IWorkspaceScriptable {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable3> for IWorkspaceScriptable {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable> for &'a IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWorkspaceScriptable3> for IWorkspaceScriptable2 {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWorkspaceScriptable3> for IWorkspaceScriptable2 {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable2> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWorkspaceScriptable2> for &'a IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWorkspaceScriptable2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWorkspaceScriptable3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWorkspaceScriptable3 {
    type Vtable = IWorkspaceScriptable3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x531e6512_2cbf_4bd2_80a5_d90a71636a9a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable3_Vtbl {
    pub base: IWorkspaceScriptable2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspaceEx2:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspaceEx2: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ItsPubPlugin(::windows::core::IUnknown);
impl ItsPubPlugin {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResourceList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResourceList)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::core::Result<pluginResource> {
        let mut result__: pluginResource = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResource)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<pluginResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCacheLastUpdateTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn pluginName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).pluginName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn pluginVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).pluginVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ResolveResource<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, resourcetype: *mut u32, resourcelocation: ::windows::core::PWSTR, endpointname: ::windows::core::PWSTR, userid: Param3, alias: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResolveResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcetype), ::core::mem::transmute(resourcelocation), ::core::mem::transmute(endpointname), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ItsPubPlugin> for ::windows::core::IUnknown {
    fn from(value: ItsPubPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItsPubPlugin> for ::windows::core::IUnknown {
    fn from(value: &ItsPubPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItsPubPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItsPubPlugin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ItsPubPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItsPubPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItsPubPlugin {}
impl ::core::fmt::Debug for ItsPubPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItsPubPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ItsPubPlugin {
    type Vtable = ItsPubPlugin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c04b05_f347_412b_822f_36c99c54ca45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetResourceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows::core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alias: ::windows::core::PCWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::core::HRESULT,
    pub GetCacheLastUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub pluginName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    pluginName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub pluginVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    pluginVersion: usize,
    pub ResolveResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: ::windows::core::PWSTR, endpointname: ::windows::core::PWSTR, userid: ::windows::core::PCWSTR, alias: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
pub struct ItsPubPlugin2(::windows::core::IUnknown);
impl ItsPubPlugin2 {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResourceList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetResourceList)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::core::Result<pluginResource> {
        let mut result__: pluginResource = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetResource)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<pluginResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCacheLastUpdateTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn pluginName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.pluginName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn pluginVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.pluginVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ResolveResource<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, resourcetype: *mut u32, resourcelocation: ::windows::core::PWSTR, endpointname: ::windows::core::PWSTR, userid: Param3, alias: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResolveResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcetype), ::core::mem::transmute(resourcelocation), ::core::mem::transmute(endpointname), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResource2List<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResource2List)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn GetResource2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::core::Result<pluginResource2> {
        let mut result__: pluginResource2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResource2)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<pluginResource2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn ResolvePersonalDesktop<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, userid: Param0, poolid: Param1, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResolvePersonalDesktop)(::core::mem::transmute_copy(self), userid.into_param().abi(), poolid.into_param().abi(), ::core::mem::transmute(epdresolutiontype), ::core::mem::transmute(ppdassignmenttype), ::core::mem::transmute(endpointname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub unsafe fn DeletePersonalDesktopAssignment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, userid: Param0, poolid: Param1, endpointname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePersonalDesktopAssignment)(::core::mem::transmute_copy(self), userid.into_param().abi(), poolid.into_param().abi(), endpointname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ItsPubPlugin2> for ::windows::core::IUnknown {
    fn from(value: ItsPubPlugin2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItsPubPlugin2> for ::windows::core::IUnknown {
    fn from(value: &ItsPubPlugin2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItsPubPlugin2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItsPubPlugin2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ItsPubPlugin2> for ItsPubPlugin {
    fn from(value: ItsPubPlugin2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItsPubPlugin2> for ItsPubPlugin {
    fn from(value: &ItsPubPlugin2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ItsPubPlugin> for ItsPubPlugin2 {
    fn into_param(self) -> ::windows::core::Param<'a, ItsPubPlugin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ItsPubPlugin> for &'a ItsPubPlugin2 {
    fn into_param(self) -> ::windows::core::Param<'a, ItsPubPlugin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ItsPubPlugin2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItsPubPlugin2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItsPubPlugin2 {}
impl ::core::fmt::Debug for ItsPubPlugin2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItsPubPlugin2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ItsPubPlugin2 {
    type Vtable = ItsPubPlugin2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa4ce418_aad7_4ec6_bad1_0a321ba465d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin2_Vtbl {
    pub base: ItsPubPlugin_Vtbl,
    pub GetResource2List: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows::core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::HRESULT,
    pub GetResource2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alias: ::windows::core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows::core::HRESULT,
    pub ResolvePersonalDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows::core::PCWSTR, poolid: ::windows::core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub DeletePersonalDesktopAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows::core::PCWSTR, poolid: ::windows::core::PCWSTR, endpointname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyCombinationType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationHome: KeyCombinationType = KeyCombinationType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationLeft: KeyCombinationType = KeyCombinationType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationUp: KeyCombinationType = KeyCombinationType(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationRight: KeyCombinationType = KeyCombinationType(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationDown: KeyCombinationType = KeyCombinationType(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationScroll: KeyCombinationType = KeyCombinationType(5i32);
impl ::core::marker::Copy for KeyCombinationType {}
impl ::core::clone::Clone for KeyCombinationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCombinationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyCombinationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCombinationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCombinationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxAppName_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxDomainName_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxFQDN_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxFarm_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxNetBiosName_Len: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxNumOfExposed_IPs: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxUserName_Len: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PCHANNEL_INIT_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PCHANNEL_OPEN_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PLUGIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POLICY_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TASK_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(64i32);
impl ::core::marker::Copy for PLUGIN_TYPE {}
impl ::core::clone::Clone for PLUGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PLUGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PLUGIN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PLUGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLUGIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cdfd28e_d0b9_4c1f_a5eb_6d1f6c6535b9);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2c3fda_338d_4d3f_81a3_e767310d908e);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6212d757_0043_4862_99c3_9f3059ac2a3b);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x197c427a_0135_4b6d_9c5e_e6579a0ab625);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELCLOSE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELENTRY = ::core::option::Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELINIT = ::core::option::Option<unsafe extern "system" fn(ppinithandle: *mut *mut ::core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELOPEN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, popenhandle: *mut u32, pchannelname: ::windows::core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELWRITE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, puserdata: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PasswordEncodingType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF8: PasswordEncodingType = PasswordEncodingType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF16LE: PasswordEncodingType = PasswordEncodingType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF16BE: PasswordEncodingType = PasswordEncodingType(2i32);
impl ::core::marker::Copy for PasswordEncodingType {}
impl ::core::clone::Clone for PasswordEncodingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PasswordEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PasswordEncodingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PasswordEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordEncodingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PolicyAttributeType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const EnableAllRedirections: PolicyAttributeType = PolicyAttributeType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DisableAllRedirections: PolicyAttributeType = PolicyAttributeType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DriveRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PrinterRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PortRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ClipboardRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PnpRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AllowOnlySDRServers: PolicyAttributeType = PolicyAttributeType(7i32);
impl ::core::marker::Copy for PolicyAttributeType {}
impl ::core::clone::Clone for PolicyAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PolicyAttributeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PolicyAttributeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PolicyAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyAttributeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ProcessIdToSessionId(::core::mem::transmute(dwprocessid), ::core::mem::transmute(psessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RDV_TASK_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = RDV_TASK_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = RDV_TASK_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = RDV_TASK_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = RDV_TASK_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = RDV_TASK_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = RDV_TASK_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = RDV_TASK_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = RDV_TASK_STATUS(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = RDV_TASK_STATUS(8i32);
impl ::core::marker::Copy for RDV_TASK_STATUS {}
impl ::core::clone::Clone for RDV_TASK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDV_TASK_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDV_TASK_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDV_TASK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDV_TASK_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RD_FARM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = RD_FARM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = RD_FARM_TYPE(-1i32);
impl ::core::marker::Copy for RD_FARM_TYPE {}
impl ::core::clone::Clone for RD_FARM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RD_FARM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RD_FARM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RD_FARM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RD_FARM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_CLEAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_VIDEO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MONITOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RFX_GFX_MONITOR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RFX_GFX_MONITOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MONITOR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MONITOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_DESKTOP_INPUT_RESET>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_DESKTOP_RESEND_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_DISCONNECT_NOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::default::Default for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for RFX_GFX_MSG_HEADER {}
impl ::core::clone::Clone for RFX_GFX_MSG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_HEADER {}
impl ::core::default::Default for RFX_GFX_MSG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl ::core::marker::Copy for RFX_GFX_MSG_RDP_DATA {}
impl ::core::clone::Clone for RFX_GFX_MSG_RDP_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_MSG_RDP_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_RDP_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_MSG_RDP_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_RDP_DATA {}
impl ::core::default::Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RFX_GFX_RECT {}
impl ::core::clone::Clone for RFX_GFX_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RFX_GFX_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RFX_GFX_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RFX_GFX_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RFX_GFX_RECT {}
impl ::core::default::Default for RFX_GFX_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteActionType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionCharms: RemoteActionType = RemoteActionType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionAppbar: RemoteActionType = RemoteActionType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionSnap: RemoteActionType = RemoteActionType(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionStartScreen: RemoteActionType = RemoteActionType(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionAppSwitch: RemoteActionType = RemoteActionType(4i32);
impl ::core::marker::Copy for RemoteActionType {}
impl ::core::clone::Clone for RemoteActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteActionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SESSION_TIMEOUT_ACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(1i32);
impl ::core::marker::Copy for SESSION_TIMEOUT_ACTION_TYPE {}
impl ::core::clone::Clone for SESSION_TIMEOUT_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SESSION_TIMEOUT_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SESSION_TIMEOUT_ACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SESSION_TIMEOUT_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_TIMEOUT_ACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SINGLE_SESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SnapshotEncodingType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotEncodingDataUri: SnapshotEncodingType = SnapshotEncodingType(0i32);
impl ::core::marker::Copy for SnapshotEncodingType {}
impl ::core::clone::Clone for SnapshotEncodingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapshotEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SnapshotEncodingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SnapshotEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotEncodingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SnapshotFormatType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatPng: SnapshotFormatType = SnapshotFormatType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatJpeg: SnapshotFormatType = SnapshotFormatType(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatBmp: SnapshotFormatType = SnapshotFormatType(2i32);
impl ::core::marker::Copy for SnapshotFormatType {}
impl ::core::clone::Clone for SnapshotFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapshotFormatType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SnapshotFormatType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SnapshotFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotFormatType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TARGET_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(128i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(256i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(512i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1024i32);
impl ::core::marker::Copy for TARGET_CHANGE_TYPE {}
impl ::core::clone::Clone for TARGET_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TARGET_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TARGET_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TARGET_OWNER(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_UNKNOWN: TARGET_OWNER = TARGET_OWNER(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = TARGET_OWNER(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = TARGET_OWNER(2i32);
impl ::core::marker::Copy for TARGET_OWNER {}
impl ::core::clone::Clone for TARGET_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_OWNER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TARGET_OWNER {
    type Abi = Self;
}
impl ::core::fmt::Debug for TARGET_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_OWNER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TARGET_PATCH_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = TARGET_PATCH_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = TARGET_PATCH_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(4i32);
impl ::core::marker::Copy for TARGET_PATCH_STATE {}
impl ::core::clone::Clone for TARGET_PATCH_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_PATCH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TARGET_PATCH_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TARGET_PATCH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_PATCH_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TARGET_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_UNKNOWN: TARGET_STATE = TARGET_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INITIALIZING: TARGET_STATE = TARGET_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_RUNNING: TARGET_STATE = TARGET_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_DOWN: TARGET_STATE = TARGET_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_HIBERNATED: TARGET_STATE = TARGET_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_CHECKED_OUT: TARGET_STATE = TARGET_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STOPPED: TARGET_STATE = TARGET_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INVALID: TARGET_STATE = TARGET_STATE(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STARTING: TARGET_STATE = TARGET_STATE(9i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STOPPING: TARGET_STATE = TARGET_STATE(10i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_MAXSTATE: TARGET_STATE = TARGET_STATE(11i32);
impl ::core::marker::Copy for TARGET_STATE {}
impl ::core::clone::Clone for TARGET_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TARGET_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TARGET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TARGET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const UNKNOWN: TARGET_TYPE = TARGET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FARM: TARGET_TYPE = TARGET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NONFARM: TARGET_TYPE = TARGET_TYPE(2i32);
impl ::core::marker::Copy for TARGET_TYPE {}
impl ::core::clone::Clone for TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TARGET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(1i32);
impl ::core::marker::Copy for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {}
impl ::core::clone::Clone for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(1i32);
impl ::core::marker::Copy for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {}
impl ::core::clone::Clone for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_RESOLUTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TSSB_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(4i32);
impl ::core::marker::Copy for TSSB_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for TSSB_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSB_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TSSB_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TSSB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TSSD_AddrV46Type(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = TSSD_AddrV46Type(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = TSSD_AddrV46Type(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = TSSD_AddrV46Type(6i32);
impl ::core::marker::Copy for TSSD_AddrV46Type {}
impl ::core::clone::Clone for TSSD_AddrV46Type {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSD_AddrV46Type {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TSSD_AddrV46Type {
    type Abi = Self;
}
impl ::core::fmt::Debug for TSSD_AddrV46Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSD_AddrV46Type").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl ::core::marker::Copy for TSSD_ConnectionPoint {}
impl ::core::clone::Clone for TSSD_ConnectionPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TSSD_ConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TSSD_ConnectionPoint").field("ServerAddressB", &self.ServerAddressB).field("AddressType", &self.AddressType).field("PortNumber", &self.PortNumber).field("AddressScope", &self.AddressScope).finish()
    }
}
unsafe impl ::windows::core::Abi for TSSD_ConnectionPoint {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TSSD_ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TSSD_ConnectionPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for TSSD_ConnectionPoint {}
impl ::core::default::Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TSSESSION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_INVALID: TSSESSION_STATE = TSSESSION_STATE(-1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_ACTIVE: TSSESSION_STATE = TSSESSION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_CONNECTED: TSSESSION_STATE = TSSESSION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_CONNECTQUERY: TSSESSION_STATE = TSSESSION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_SHADOW: TSSESSION_STATE = TSSESSION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_DISCONNECTED: TSSESSION_STATE = TSSESSION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_IDLE: TSSESSION_STATE = TSSESSION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_LISTEN: TSSESSION_STATE = TSSESSION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_RESET: TSSESSION_STATE = TSSESSION_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_DOWN: TSSESSION_STATE = TSSESSION_STATE(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_INIT: TSSESSION_STATE = TSSESSION_STATE(9i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_MAX: TSSESSION_STATE = TSSESSION_STATE(10i32);
impl ::core::marker::Copy for TSSESSION_STATE {}
impl ::core::clone::Clone for TSSESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TSSESSION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TSSESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSESSION_STATE").field(&self.0).finish()
    }
}
pub const TSUserExInterfaces: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0910dd01_df8c_11d1_ae27_00c04fa35813);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TS_SB_SORT_BY(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = TS_SB_SORT_BY(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = TS_SB_SORT_BY(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = TS_SB_SORT_BY(2i32);
impl ::core::marker::Copy for TS_SB_SORT_BY {}
impl ::core::clone::Clone for TS_SB_SORT_BY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TS_SB_SORT_BY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TS_SB_SORT_BY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TS_SB_SORT_BY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TS_SB_SORT_BY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const USERNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VM_HOST_NOTIFY_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(3i32);
impl ::core::marker::Copy for VM_HOST_NOTIFY_STATUS {}
impl ::core::clone::Clone for VM_HOST_NOTIFY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VM_HOST_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VM_HOST_NOTIFY_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VM_HOST_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_HOST_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl ::core::marker::Copy for VM_NOTIFY_ENTRY {}
impl ::core::clone::Clone for VM_NOTIFY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_ENTRY").field("VmName", &self.VmName).field("VmHost", &self.VmHost).finish()
    }
}
unsafe impl ::windows::core::Abi for VM_NOTIFY_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VM_NOTIFY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VM_NOTIFY_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_ENTRY {}
impl ::core::default::Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl ::core::marker::Copy for VM_NOTIFY_INFO {}
impl ::core::clone::Clone for VM_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_INFO").field("dwNumEntries", &self.dwNumEntries).field("ppVmEntries", &self.ppVmEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for VM_NOTIFY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VM_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VM_NOTIFY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_INFO {}
impl ::core::default::Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VM_NOTIFY_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(4i32);
impl ::core::marker::Copy for VM_NOTIFY_STATUS {}
impl ::core::clone::Clone for VM_NOTIFY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VM_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VM_NOTIFY_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VM_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VM_PATCH_INFO {}
impl ::core::clone::Clone for VM_PATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_PATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_PATCH_INFO").field("dwNumEntries", &self.dwNumEntries).field("pVmNames", &self.pVmNames).finish()
    }
}
unsafe impl ::windows::core::Abi for VM_PATCH_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VM_PATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VM_PATCH_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for VM_PATCH_INFO {}
impl ::core::default::Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_CONNECTION_SETTING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_CONNECTION_SETTING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_CONNECTION_SETTINGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_CONNECTION_SETTINGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: super::super::Foundation::BOOLEAN,
    pub fInheritColorDepth: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOLEAN,
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fResetBroken: super::super::Foundation::BOOLEAN,
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub HRes: u16,
    pub VRes: u16,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub KeyboardLayout: u32,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub PerformanceFlags: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ActiveInputLocale: u32,
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientBuildNumber: u32,
    pub ClientSessionId: u32,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserName: [u16; 256],
    pub Domain: [u16; 256],
    pub Password: [u16; 256],
    pub ProtocolName: [u16; 9],
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub imeFileName: [u16; 33],
    pub AudioDriverName: [u16; 9],
    pub ClientName: [u16; 21],
    pub ClientAddress: [u16; 31],
    pub ClientDirectory: [u16; 257],
    pub ClientDigProductId: [u16; 33],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub WRdsListenerSettings: WRDS_LISTENER_SETTINGS,
    pub EventLogActivityId: ::windows::core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_CONNECTION_SETTINGS_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_CONNECTION_SETTINGS_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRDS_CONNECTION_SETTING_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(1i32);
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_CONNECTION_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRDS_CONNECTION_SETTING_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRDS_CONNECTION_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_CONNECTION_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: u16,
}
impl ::core::marker::Copy for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_DYNAMIC_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).field("TimeZoneKeyName", &self.TimeZoneKeyName).field("DynamicDaylightTimeDisabled", &self.DynamicDaylightTimeDisabled).finish()
    }
}
unsafe impl ::windows::core::Abi for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_DYNAMIC_TIME_ZONE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTING {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WRDS_LISTENER_SETTING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_LISTENER_SETTING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTING {}
impl ::core::default::Default for WRDS_LISTENER_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WRDS_LISTENER_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_LISTENER_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS {}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_LISTENER_SETTINGS_1").field("MaxProtocolListenerConnectionCount", &self.MaxProtocolListenerConnectionCount).field("SecurityDescriptorSize", &self.SecurityDescriptorSize).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for WRDS_LISTENER_SETTINGS_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_LISTENER_SETTINGS_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRDS_LISTENER_SETTING_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(1i32);
impl ::core::marker::Copy for WRDS_LISTENER_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRDS_LISTENER_SETTING_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_LISTENER_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2993f4d_02cf_4280_8c48_1624b44f8706);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_SETTING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_SETTING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_SETTINGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_SETTINGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS_1 {
    pub WRdsDisableClipStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableClipValue: u32,
    pub WRdsDisableLPTStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableLPTValue: u32,
    pub WRdsDisableCcmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCcmValue: u32,
    pub WRdsDisableCdmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCdmValue: u32,
    pub WRdsDisableCpmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCpmValue: u32,
    pub WRdsDisablePnpStatus: WRDS_SETTING_STATUS,
    pub WRdsDisablePnpValue: u32,
    pub WRdsEncryptionLevelStatus: WRDS_SETTING_STATUS,
    pub WRdsEncryptionValue: u32,
    pub WRdsColorDepthStatus: WRDS_SETTING_STATUS,
    pub WRdsColorDepthValue: u32,
    pub WRdsDisableAutoReconnecetStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableAutoReconnecetValue: u32,
    pub WRdsDisableEncryptionStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableEncryptionValue: u32,
    pub WRdsResetBrokenStatus: WRDS_SETTING_STATUS,
    pub WRdsResetBrokenValue: u32,
    pub WRdsMaxIdleTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxIdleTimeValue: u32,
    pub WRdsMaxDisconnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxDisconnectTimeValue: u32,
    pub WRdsMaxConnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxConnectTimeValue: u32,
    pub WRdsKeepAliveStatus: WRDS_SETTING_STATUS,
    pub WRdsKeepAliveStartValue: super::super::Foundation::BOOLEAN,
    pub WRdsKeepAliveIntervalValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WRDS_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_SETTINGS_1")
            .field("WRdsDisableClipStatus", &self.WRdsDisableClipStatus)
            .field("WRdsDisableClipValue", &self.WRdsDisableClipValue)
            .field("WRdsDisableLPTStatus", &self.WRdsDisableLPTStatus)
            .field("WRdsDisableLPTValue", &self.WRdsDisableLPTValue)
            .field("WRdsDisableCcmStatus", &self.WRdsDisableCcmStatus)
            .field("WRdsDisableCcmValue", &self.WRdsDisableCcmValue)
            .field("WRdsDisableCdmStatus", &self.WRdsDisableCdmStatus)
            .field("WRdsDisableCdmValue", &self.WRdsDisableCdmValue)
            .field("WRdsDisableCpmStatus", &self.WRdsDisableCpmStatus)
            .field("WRdsDisableCpmValue", &self.WRdsDisableCpmValue)
            .field("WRdsDisablePnpStatus", &self.WRdsDisablePnpStatus)
            .field("WRdsDisablePnpValue", &self.WRdsDisablePnpValue)
            .field("WRdsEncryptionLevelStatus", &self.WRdsEncryptionLevelStatus)
            .field("WRdsEncryptionValue", &self.WRdsEncryptionValue)
            .field("WRdsColorDepthStatus", &self.WRdsColorDepthStatus)
            .field("WRdsColorDepthValue", &self.WRdsColorDepthValue)
            .field("WRdsDisableAutoReconnecetStatus", &self.WRdsDisableAutoReconnecetStatus)
            .field("WRdsDisableAutoReconnecetValue", &self.WRdsDisableAutoReconnecetValue)
            .field("WRdsDisableEncryptionStatus", &self.WRdsDisableEncryptionStatus)
            .field("WRdsDisableEncryptionValue", &self.WRdsDisableEncryptionValue)
            .field("WRdsResetBrokenStatus", &self.WRdsResetBrokenStatus)
            .field("WRdsResetBrokenValue", &self.WRdsResetBrokenValue)
            .field("WRdsMaxIdleTimeStatus", &self.WRdsMaxIdleTimeStatus)
            .field("WRdsMaxIdleTimeValue", &self.WRdsMaxIdleTimeValue)
            .field("WRdsMaxDisconnectTimeStatus", &self.WRdsMaxDisconnectTimeStatus)
            .field("WRdsMaxDisconnectTimeValue", &self.WRdsMaxDisconnectTimeValue)
            .field("WRdsMaxConnectTimeStatus", &self.WRdsMaxConnectTimeStatus)
            .field("WRdsMaxConnectTimeValue", &self.WRdsMaxConnectTimeValue)
            .field("WRdsKeepAliveStatus", &self.WRdsKeepAliveStatus)
            .field("WRdsKeepAliveStartValue", &self.WRdsKeepAliveStartValue)
            .field("WRdsKeepAliveIntervalValue", &self.WRdsKeepAliveIntervalValue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WRDS_SETTINGS_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRDS_SETTINGS_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRDS_SETTING_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(1i32);
impl ::core::marker::Copy for WRDS_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRDS_SETTING_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRDS_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRDS_SETTING_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(-1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(2i32);
impl ::core::marker::Copy for WRDS_SETTING_STATUS {}
impl ::core::clone::Clone for WRDS_SETTING_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRDS_SETTING_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRDS_SETTING_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRDS_SETTING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(3i32);
impl ::core::marker::Copy for WRDS_SETTING_TYPE {}
impl ::core::clone::Clone for WRDS_SETTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRDS_SETTING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRDS_SETTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRdsGraphicsChannelType(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(1i32);
impl ::core::marker::Copy for WRdsGraphicsChannelType {}
impl ::core::clone::Clone for WRdsGraphicsChannelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRdsGraphicsChannelType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRdsGraphicsChannelType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRdsGraphicsChannelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRdsGraphicsChannelType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCLIENTA {
    pub ClientName: [super::super::Foundation::CHAR; 21],
    pub Domain: [super::super::Foundation::CHAR; 18],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [super::super::Foundation::CHAR; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCLIENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCLIENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTA")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSCLIENTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSCLIENTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSCLIENTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSCLIENTW {
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u16; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u16; 261],
}
impl ::core::marker::Copy for WTSCLIENTW {}
impl ::core::clone::Clone for WTSCLIENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCLIENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTW")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSCLIENTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSCLIENTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSCLIENTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSCLIENTW {}
impl ::core::default::Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub ApplicationName: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCONFIGINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCONFIGINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOA")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSCONFIGINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSCONFIGINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSCONFIGINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSCONFIGINFOW {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub ApplicationName: [u16; 261],
}
impl ::core::marker::Copy for WTSCONFIGINFOW {}
impl ::core::clone::Clone for WTSCONFIGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCONFIGINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOW")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSCONFIGINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSCONFIGINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSCONFIGINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSCONFIGINFOW {}
impl ::core::default::Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCloseServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCloseServer(hserver: super::super::Foundation::HANDLE);
        }
        WTSCloseServer(hserver.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(logonid: u32, targetlogonid: u32, ppassword: Param2, bwait: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSConnectSessionA(logonid: u32, targetlogonid: u32, ppassword: ::windows::core::PCSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSConnectSessionA(::core::mem::transmute(logonid), ::core::mem::transmute(targetlogonid), ppassword.into_param().abi(), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(logonid: u32, targetlogonid: u32, ppassword: Param2, bwait: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSConnectSessionW(logonid: u32, targetlogonid: u32, ppassword: ::windows::core::PCWSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSConnectSessionW(::core::mem::transmute(logonid), ::core::mem::transmute(targetlogonid), ppassword.into_param().abi(), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCreateListenerA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCSTR, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSCreateListenerA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCreateListenerW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCWSTR, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSCreateListenerW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSDisconnectSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, bwait: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSDisconnectSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSDisconnectSession(hserver.into_param().abi(), ::core::mem::transmute(sessionid), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnableChildSessions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(benable: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnableChildSessions(benable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnableChildSessions(benable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateListenersA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateListenersA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), ::core::mem::transmute(plisteners), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateListenersW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateListenersW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), ::core::mem::transmute(plisteners), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesA(hserver.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows::core::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows::core::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesExA(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(sessionid), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows::core::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows::core::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesExW(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(sessionid), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesW(hserver.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pdomainname: Param0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateServersA(pdomainname: ::windows::core::PCSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateServersA(pdomainname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppserverinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pdomainname: Param0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateServersW(pdomainname: ::windows::core::PCWSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateServersW(pdomainname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppserverinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateSessionsA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateSessionsA(hserver.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppsessioninfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateSessionsExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateSessionsExA(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(filter), ::core::mem::transmute(ppsessioninfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateSessionsExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateSessionsExW(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(filter), ::core::mem::transmute(ppsessioninfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateSessionsW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateSessionsW(hserver.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppsessioninfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void);
        }
        WTSFreeMemory(::core::mem::transmute(pmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSFreeMemoryExA(::core::mem::transmute(wtstypeclass), ::core::mem::transmute(pmemory), ::core::mem::transmute(numberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSFreeMemoryExW(::core::mem::transmute(wtstypeclass), ::core::mem::transmute(pmemory), ::core::mem::transmute(numberofentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[inline]
pub unsafe fn WTSGetActiveConsoleSessionId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetActiveConsoleSessionId() -> u32;
        }
        ::core::mem::transmute(WTSGetActiveConsoleSessionId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSGetChildSessionId(::core::mem::transmute(psessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSGetListenerSecurityA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor), ::core::mem::transmute(nlength), ::core::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCWSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSGetListenerSecurityW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor), ::core::mem::transmute(nlength), ::core::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [super::super::Foundation::CHAR; 32],
    pub Domain: [super::super::Foundation::CHAR; 17],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOA")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBy", &self.OutgoingCompressedBy)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSINFOEXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl ::core::marker::Copy for WTSINFOEXW {}
impl ::core::clone::Clone for WTSINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTSINFOEXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSINFOEXW {}
impl ::core::default::Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [super::super::Foundation::CHAR; 33],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub DomainName: [super::super::Foundation::CHAR; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_A")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSINFOEX_LEVEL1_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEX_LEVEL1_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_W")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSINFOEX_LEVEL1_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEX_LEVEL1_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_W {}
impl ::core::default::Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSINFOEX_LEVEL_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEX_LEVEL_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTSINFOEX_LEVEL_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOEX_LEVEL_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL_W {}
impl ::core::default::Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
    pub WinStationName: [u16; 32],
    pub Domain: [u16; 17],
    pub UserName: [u16; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl ::core::marker::Copy for WTSINFOW {}
impl ::core::clone::Clone for WTSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOW")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSINFOW {}
impl ::core::default::Default for WTSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSIsChildSessionsEnabled(::core::mem::transmute(pbenabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSLISTENERCONFIGA {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [super::super::Foundation::CHAR; 61],
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSLISTENERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSLISTENERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGA")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSLISTENERCONFIGA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSLISTENERCONFIGA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSLISTENERCONFIGW {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u16; 61],
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
}
impl ::core::marker::Copy for WTSLISTENERCONFIGW {}
impl ::core::clone::Clone for WTSLISTENERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSLISTENERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGW")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSLISTENERCONFIGW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSLISTENERCONFIGW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSLISTENERCONFIGW {}
impl ::core::default::Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSLogoffSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, bwait: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSLogoffSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSLogoffSession(hserver.into_param().abi(), ::core::mem::transmute(sessionid), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerA(pservername: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerA(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerExA(pservername: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerExA(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerExW(pservername: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerExW(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerW(pservername: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerW(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryListenerConfigA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCSTR, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryListenerConfigA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryListenerConfigW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCWSTR, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryListenerConfigW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQuerySessionInformationA(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQuerySessionInformationA(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(wtsinfoclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQuerySessionInformationW(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQuerySessionInformationW(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(wtsinfoclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryUserConfigA(pservername: ::windows::core::PCSTR, pusername: ::windows::core::PCSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryUserConfigW(pservername: ::windows::core::PCWSTR, pusername: ::windows::core::PCWSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryUserToken(::core::mem::transmute(sessionid), ::core::mem::transmute(phtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSRegisterSessionNotification(hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSRegisterSessionNotification(hwnd.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hserver: Param0, hwnd: Param1, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSRegisterSessionNotificationEx(hserver.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_ADDRESS_FAMILY(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(4i32);
impl ::core::marker::Copy for WTSSBX_ADDRESS_FAMILY {}
impl ::core::clone::Clone for WTSSBX_ADDRESS_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_ADDRESS_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_ADDRESS_FAMILY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_ADDRESS_FAMILY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl ::core::marker::Copy for WTSSBX_IP_ADDRESS {}
impl ::core::clone::Clone for WTSSBX_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_IP_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_IP_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).field("PortNumber", &self.PortNumber).field("dwScope", &self.dwScope).finish()
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_IP_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSSBX_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSSBX_IP_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSSBX_IP_ADDRESS {}
impl ::core::default::Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl ::core::marker::Copy for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_CONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_CONNECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_CONNECT_INFO").field("wczMachineFQDN", &self.wczMachineFQDN).field("wczMachineNetBiosName", &self.wczMachineNetBiosName).field("dwNumOfIPAddr", &self.dwNumOfIPAddr).field("IPaddr", &self.IPaddr).finish()
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_MACHINE_CONNECT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_CONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSSBX_MACHINE_CONNECT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_MACHINE_DRAIN(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(2i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_DRAIN {}
impl ::core::clone::Clone for WTSSBX_MACHINE_DRAIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_DRAIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_MACHINE_DRAIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_DRAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_DRAIN").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_MACHINE_INFO {
    pub ClientConnectInfo: WTSSBX_MACHINE_CONNECT_INFO,
    pub wczFarmName: [u16; 257],
    pub InternalIPAddress: WTSSBX_IP_ADDRESS,
    pub dwMaxSessionsLimit: u32,
    pub ServerWeight: u32,
    pub SingleSessionMode: WTSSBX_MACHINE_SESSION_MODE,
    pub InDrain: WTSSBX_MACHINE_DRAIN,
    pub MachineState: WTSSBX_MACHINE_STATE,
}
impl ::core::marker::Copy for WTSSBX_MACHINE_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_INFO").field("ClientConnectInfo", &self.ClientConnectInfo).field("wczFarmName", &self.wczFarmName).field("InternalIPAddress", &self.InternalIPAddress).field("dwMaxSessionsLimit", &self.dwMaxSessionsLimit).field("ServerWeight", &self.ServerWeight).field("SingleSessionMode", &self.SingleSessionMode).field("InDrain", &self.InDrain).field("MachineState", &self.MachineState).finish()
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_MACHINE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSSBX_MACHINE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_MACHINE_SESSION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(2i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_SESSION_MODE {}
impl ::core::clone::Clone for WTSSBX_MACHINE_SESSION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_SESSION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_MACHINE_SESSION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_SESSION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_SESSION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_MACHINE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(2i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_STATE {}
impl ::core::clone::Clone for WTSSBX_MACHINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_MACHINE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(8i32);
impl ::core::marker::Copy for WTSSBX_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for WTSSBX_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSSBX_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSSBX_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_SESSION_INFO").field("wszUserName", &self.wszUserName).field("wszDomainName", &self.wszDomainName).field("ApplicationType", &self.ApplicationType).field("dwSessionId", &self.dwSessionId).field("CreateTime", &self.CreateTime).field("DisconnectTime", &self.DisconnectTime).field("SessionState", &self.SessionState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSSBX_SESSION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSSBX_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSSBX_SESSION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTSSBX_SESSION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(2i32);
impl ::core::marker::Copy for WTSSBX_SESSION_STATE {}
impl ::core::clone::Clone for WTSSBX_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTSSBX_SESSION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTSSBX_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl ::core::marker::Copy for WTSSESSION_NOTIFICATION {}
impl ::core::clone::Clone for WTSSESSION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSESSION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSESSION_NOTIFICATION").field("cbSize", &self.cbSize).field("dwSessionId", &self.dwSessionId).finish()
    }
}
unsafe impl ::windows::core::Abi for WTSSESSION_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSSESSION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSSESSION_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSSESSION_NOTIFICATION {}
impl ::core::default::Default for WTSSESSION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, ptitle: Param2, titlelength: u32, pmessage: Param4, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSendMessageA(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: ::windows::core::PCSTR, titlelength: u32, pmessage: ::windows::core::PCSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSendMessageA(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ptitle.into_param().abi(), ::core::mem::transmute(titlelength), pmessage.into_param().abi(), ::core::mem::transmute(messagelength), ::core::mem::transmute(style), ::core::mem::transmute(timeout), ::core::mem::transmute(presponse), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, ptitle: Param2, titlelength: u32, pmessage: Param4, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSendMessageW(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: ::windows::core::PCWSTR, titlelength: u32, pmessage: ::windows::core::PCWSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSendMessageW(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ptitle.into_param().abi(), ::core::mem::transmute(titlelength), pmessage.into_param().abi(), ::core::mem::transmute(messagelength), ::core::mem::transmute(style), ::core::mem::transmute(timeout), ::core::mem::transmute(presponse), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetListenerSecurityA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows::core::PCWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetListenerSecurityW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetRenderHint<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(prenderhintid: *mut u64, hwndowner: Param1, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows::core::HRESULT;
        }
        WTSSetRenderHint(::core::mem::transmute(prenderhintid), hwndowner.into_param().abi(), ::core::mem::transmute(renderhinttype), ::core::mem::transmute(cbhintdatalength), ::core::mem::transmute(phintdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: Param3, datalength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetUserConfigA(pservername: ::windows::core::PCSTR, pusername: ::windows::core::PCSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: ::windows::core::PCSTR, datalength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), pbuffer.into_param().abi(), ::core::mem::transmute(datalength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: Param3, datalength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetUserConfigW(pservername: ::windows::core::PCWSTR, pusername: ::windows::core::PCWSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: ::windows::core::PCWSTR, datalength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), pbuffer.into_param().abi(), ::core::mem::transmute(datalength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSShutdownSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, shutdownflag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSShutdownSystem(hserver: super::super::Foundation::HANDLE, shutdownflag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSShutdownSystem(hserver.into_param().abi(), ::core::mem::transmute(shutdownflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(ptargetservername: Param0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSStartRemoteControlSessionA(ptargetservername: ::windows::core::PCSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSStartRemoteControlSessionA(ptargetservername.into_param().abi(), ::core::mem::transmute(targetlogonid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(ptargetservername: Param0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSStartRemoteControlSessionW(ptargetservername: ::windows::core::PCWSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSStartRemoteControlSessionW(ptargetservername.into_param().abi(), ::core::mem::transmute(targetlogonid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStopRemoteControlSession(logonid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSStopRemoteControlSession(logonid: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSStopRemoteControlSession(::core::mem::transmute(logonid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSTerminateProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSTerminateProcess(hserver: super::super::Foundation::HANDLE, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSTerminateProcess(hserver.into_param().abi(), ::core::mem::transmute(processid), ::core::mem::transmute(exitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSUSERCONFIGA {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub TerminalServerProfilePath: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDir: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDirDrive: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSUSERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSUSERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGA")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTSUSERCONFIGA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSUSERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSUSERCONFIGA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSUSERCONFIGW {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u16; 261],
    pub WorkDirectory: [u16; 261],
    pub TerminalServerProfilePath: [u16; 261],
    pub TerminalServerHomeDir: [u16; 261],
    pub TerminalServerHomeDirDrive: [u16; 4],
}
impl ::core::marker::Copy for WTSUSERCONFIGW {}
impl ::core::clone::Clone for WTSUSERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSUSERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGW")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTSUSERCONFIGW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTSUSERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTSUSERCONFIGW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTSUSERCONFIGW {}
impl ::core::default::Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSUnRegisterSessionNotification(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSUnRegisterSessionNotification(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hserver: Param0, hwnd: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSUnRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSUnRegisterSessionNotificationEx(hserver.into_param().abi(), hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelClose(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelClose(hchannelhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelOpen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hserver: Param0, sessionid: u32, pvirtualname: Param2) -> HwtsVirtualChannelHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelOpen(hserver: super::super::Foundation::HANDLE, sessionid: u32, pvirtualname: ::windows::core::PCSTR) -> HwtsVirtualChannelHandle;
        }
        ::core::mem::transmute(WTSVirtualChannelOpen(hserver.into_param().abi(), ::core::mem::transmute(sessionid), pvirtualname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sessionid: u32, pvirtualname: Param1, flags: u32) -> HwtsVirtualChannelHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelOpenEx(sessionid: u32, pvirtualname: ::windows::core::PCSTR, flags: u32) -> HwtsVirtualChannelHandle;
        }
        ::core::mem::transmute(WTSVirtualChannelOpenEx(::core::mem::transmute(sessionid), pvirtualname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelPurgeInput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelPurgeInput(hchannelhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelPurgeOutput(hchannelhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelQuery(hchannelhandle: super::super::Foundation::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelQuery(hchannelhandle.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelRead<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0, timeout: u32, buffer: ::windows::core::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: ::windows::core::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelRead(hchannelhandle.into_param().abi(), ::core::mem::transmute(timeout), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(pbytesread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelWrite<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hchannelhandle: Param0, buffer: Param1, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: ::windows::core::PCSTR, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelWrite(hchannelhandle.into_param().abi(), buffer.into_param().abi(), ::core::mem::transmute(length), ::core::mem::transmute(pbyteswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSWaitSystemEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSWaitSystemEvent(hserver: super::super::Foundation::HANDLE, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSWaitSystemEvent(hserver.into_param().abi(), ::core::mem::transmute(eventmask), ::core::mem::transmute(peventflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for WTS_CACHE_STATS {}
impl ::core::clone::Clone for WTS_CACHE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_CACHE_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_CACHE_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_CACHE_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_CACHE_STATS {}
impl ::core::default::Default for WTS_CACHE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl ::core::marker::Copy for WTS_CACHE_STATS_UN {}
impl ::core::clone::Clone for WTS_CACHE_STATS_UN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_CACHE_STATS_UN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_CACHE_STATS_UN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_CACHE_STATS_UN>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_CACHE_STATS_UN {}
impl ::core::default::Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_CERT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = WTS_CERT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = WTS_CERT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = WTS_CERT_TYPE(2i32);
impl ::core::marker::Copy for WTS_CERT_TYPE {}
impl ::core::clone::Clone for WTS_CERT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CERT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_CERT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_CERT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CERT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_CLIENT_ADDRESS {}
impl ::core::clone::Clone for WTS_CLIENT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_CLIENT_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_CLIENT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_CLIENT_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_ADDRESS {}
impl ::core::default::Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOL,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fInheritInitialProgram: super::super::Foundation::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: super::super::Foundation::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub ClientName: [u16; 21],
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientDirectory: [u16; 257],
    pub ClientBuildNumber: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 33],
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_CLIENT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_CLIENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_CLIENT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for WTS_CLIENT_DISPLAY {}
impl ::core::clone::Clone for WTS_CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_CLIENT_DISPLAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_CLIENT_DISPLAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_DISPLAY {}
impl ::core::default::Default for WTS_CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_CONFIG_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(9i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(10i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(11i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(12i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(13i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(14i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(15i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(16i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(17i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(18i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(19i32);
impl ::core::marker::Copy for WTS_CONFIG_CLASS {}
impl ::core::clone::Clone for WTS_CONFIG_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONFIG_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_CONFIG_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_CONFIG_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_CONFIG_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = WTS_CONFIG_SOURCE(0i32);
impl ::core::marker::Copy for WTS_CONFIG_SOURCE {}
impl ::core::clone::Clone for WTS_CONFIG_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONFIG_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_CONFIG_SOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_CONFIG_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_CONNECTSTATE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSActive: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSListen: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSReset: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDown: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSInit: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(9i32);
impl ::core::marker::Copy for WTS_CONNECTSTATE_CLASS {}
impl ::core::clone::Clone for WTS_CONNECTSTATE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONNECTSTATE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_CONNECTSTATE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_CONNECTSTATE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONNECTSTATE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl ::core::marker::Copy for WTS_DISPLAY_IOCTL {}
impl ::core::clone::Clone for WTS_DISPLAY_IOCTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_DISPLAY_IOCTL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_DISPLAY_IOCTL").field("pDisplayIOCtlData", &self.pDisplayIOCtlData).field("cbDisplayIOCtlData", &self.cbDisplayIOCtlData).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_DISPLAY_IOCTL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_DISPLAY_IOCTL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_DISPLAY_IOCTL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_DISPLAY_IOCTL {}
impl ::core::default::Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_CONNECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LICENSE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LOGON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_RENAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSInitialProgram: WTS_INFO_CLASS = WTS_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSApplicationName: WTS_INFO_CLASS = WTS_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSWorkingDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOEMId: WTS_INFO_CLASS = WTS_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionId: WTS_INFO_CLASS = WTS_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserName: WTS_INFO_CLASS = WTS_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSWinStationName: WTS_INFO_CLASS = WTS_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDomainName: WTS_INFO_CLASS = WTS_INFO_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnectState: WTS_INFO_CLASS = WTS_INFO_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientBuildNumber: WTS_INFO_CLASS = WTS_INFO_CLASS(9i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientName: WTS_INFO_CLASS = WTS_INFO_CLASS(10i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(11i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientProductId: WTS_INFO_CLASS = WTS_INFO_CLASS(12i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientHardwareId: WTS_INFO_CLASS = WTS_INFO_CLASS(13i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientAddress: WTS_INFO_CLASS = WTS_INFO_CLASS(14i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientDisplay: WTS_INFO_CLASS = WTS_INFO_CLASS(15i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientProtocolType: WTS_INFO_CLASS = WTS_INFO_CLASS(16i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIdleTime: WTS_INFO_CLASS = WTS_INFO_CLASS(17i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSLogonTime: WTS_INFO_CLASS = WTS_INFO_CLASS(18i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIncomingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(19i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOutgoingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(20i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIncomingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(21i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOutgoingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(22i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(23i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(24i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionInfoEx: WTS_INFO_CLASS = WTS_INFO_CLASS(25i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConfigInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(26i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSValidationInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(27i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionAddressV4: WTS_INFO_CLASS = WTS_INFO_CLASS(28i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIsRemoteSession: WTS_INFO_CLASS = WTS_INFO_CLASS(29i32);
impl ::core::marker::Copy for WTS_INFO_CLASS {}
impl ::core::clone::Clone for WTS_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_INFO_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_LICENSE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_LICENSE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_LICENSE_CAPABILITIES").field("KeyExchangeAlg", &self.KeyExchangeAlg).field("ProtocolVer", &self.ProtocolVer).field("fAuthenticateServer", &self.fAuthenticateServer).field("CertType", &self.CertType).field("cbClientName", &self.cbClientName).field("rgbClientName", &self.rgbClientName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_LICENSE_CAPABILITIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_LICENSE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_LICENSE_CAPABILITIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(4i32);
impl ::core::marker::Copy for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {}
impl ::core::clone::Clone for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_LOGON_ERROR_REDIRECTOR_RESPONSE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNPRedir: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_POLICY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_POLICY_DATA")
            .field("fDisableEncryption", &self.fDisableEncryption)
            .field("fDisableAutoReconnect", &self.fDisableAutoReconnect)
            .field("ColorDepth", &self.ColorDepth)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("fDisableCpm", &self.fDisableCpm)
            .field("fDisableCdm", &self.fDisableCdm)
            .field("fDisableCcm", &self.fDisableCcm)
            .field("fDisableLPT", &self.fDisableLPT)
            .field("fDisableClip", &self.fDisableClip)
            .field("fDisablePNPRedir", &self.fDisablePNPRedir)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_POLICY_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_POLICY_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows::core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOA").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_PROCESS_INFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROCESS_INFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows::core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOW").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_PROCESS_INFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROCESS_INFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows::core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXA")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_PROCESS_INFO_EXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROCESS_INFO_EXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows::core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXW")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_PROCESS_INFO_EXW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROCESS_INFO_EXW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROPERTY_DEFAULT_CONFIG: &'static str = "DefaultConfig";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_PROPERTY_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROPERTY_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE {}
impl ::core::default::Default for WTS_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: ::windows::core::GUID,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_PROPERTY_VALUE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROPERTY_VALUE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0 {}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: ::windows::core::PSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROPERTY_VALUE_0_0").field("size", &self.size).field("pbVal", &self.pbVal).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_PROPERTY_VALUE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROPERTY_VALUE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_0 {}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_1 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROPERTY_VALUE_0_1").field("size", &self.size).field("pstrVal", &self.pstrVal).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_PROPERTY_VALUE_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROPERTY_VALUE_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_1 {}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl ::core::marker::Copy for WTS_PROTOCOL_CACHE {}
impl ::core::clone::Clone for WTS_PROTOCOL_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_CACHE").field("CacheReads", &self.CacheReads).field("CacheHits", &self.CacheHits).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_PROTOCOL_CACHE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_CACHE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROTOCOL_CACHE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_CACHE {}
impl ::core::default::Default for WTS_PROTOCOL_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_COUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: u16,
    pub Reserved: [u32; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_COUNTERS {}
impl ::core::clone::Clone for WTS_PROTOCOL_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_COUNTERS")
            .field("WdBytes", &self.WdBytes)
            .field("WdFrames", &self.WdFrames)
            .field("WaitForOutBuf", &self.WaitForOutBuf)
            .field("Frames", &self.Frames)
            .field("Bytes", &self.Bytes)
            .field("CompressedBytes", &self.CompressedBytes)
            .field("CompressFlushes", &self.CompressFlushes)
            .field("Errors", &self.Errors)
            .field("Timeouts", &self.Timeouts)
            .field("AsyncFramingError", &self.AsyncFramingError)
            .field("AsyncOverrunError", &self.AsyncOverrunError)
            .field("AsyncOverflowError", &self.AsyncOverflowError)
            .field("AsyncParityError", &self.AsyncParityError)
            .field("TdErrors", &self.TdErrors)
            .field("ProtocolType", &self.ProtocolType)
            .field("Length", &self.Length)
            .field("Specific", &self.Specific)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_PROTOCOL_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROTOCOL_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_COUNTERS {}
impl ::core::default::Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_STATUS {}
impl ::core::clone::Clone for WTS_PROTOCOL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_PROTOCOL_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_PROTOCOL_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_STATUS {}
impl ::core::default::Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77d1b30_5be1_4c6b_a0e1_bd6d2e5c9fcc);
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf4fa97_c883_4c2a_80ab_5a39c9af00db);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8e0fe7_0804_4a0e_b279_8660b1df0049);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41869ad0_6332_4dc8_95d5_db749e2f1d94);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_RCM_DRAIN_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(2i32);
impl ::core::marker::Copy for WTS_RCM_DRAIN_STATE {}
impl ::core::clone::Clone for WTS_RCM_DRAIN_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_RCM_DRAIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_RCM_DRAIN_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_RCM_DRAIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_DRAIN_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_RCM_SERVICE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(2i32);
impl ::core::marker::Copy for WTS_RCM_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_RCM_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_RCM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_RCM_SERVICE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_RCM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_CONNECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_DISCONNECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_LOGON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_MESSAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVER_INFOA {
    pub pServerName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOA {}
impl ::core::clone::Clone for WTS_SERVER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOA").field("pServerName", &self.pServerName).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SERVER_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SERVER_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOA {}
impl ::core::default::Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVER_INFOW {
    pub pServerName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOW {}
impl ::core::clone::Clone for WTS_SERVER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOW").field("pServerName", &self.pServerName).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SERVER_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SERVER_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOW {}
impl ::core::default::Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl ::core::marker::Copy for WTS_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVICE_STATE").field("RcmServiceState", &self.RcmServiceState).field("RcmDrainState", &self.RcmDrainState).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SERVICE_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SERVICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SERVICE_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SERVICE_STATE {}
impl ::core::default::Default for WTS_SERVICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_SESSION_ADDRESS {}
impl ::core::clone::Clone for WTS_SESSION_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ADDRESS {}
impl ::core::default::Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: ::windows::core::GUID,
    pub SessionId: u32,
}
impl ::core::marker::Copy for WTS_SESSION_ID {}
impl ::core::clone::Clone for WTS_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ID").field("SessionUniqueGuid", &self.SessionUniqueGuid).field("SessionId", &self.SessionId).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ID {}
impl ::core::default::Default for WTS_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: ::windows::core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOA {}
impl ::core::clone::Clone for WTS_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOA").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOA {}
impl ::core::default::Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: ::windows::core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOW {}
impl ::core::clone::Clone for WTS_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOW").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOW {}
impl ::core::default::Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows::core::PSTR,
    pub pHostName: ::windows::core::PSTR,
    pub pUserName: ::windows::core::PSTR,
    pub pDomainName: ::windows::core::PSTR,
    pub pFarmName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1A {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1A").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_INFO_1A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_INFO_1A>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1A {}
impl ::core::default::Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows::core::PWSTR,
    pub pHostName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pDomainName: ::windows::core::PWSTR,
    pub pFarmName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1W {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1W").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SESSION_INFO_1W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SESSION_INFO_1W>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1W {}
impl ::core::default::Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for WTS_SMALL_RECT {}
impl ::core::clone::Clone for WTS_SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SMALL_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SMALL_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SMALL_RECT {}
impl ::core::default::Default for WTS_SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl ::core::marker::Copy for WTS_SOCKADDR {}
impl ::core::clone::Clone for WTS_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_SOCKADDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SOCKADDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR {}
impl ::core::default::Default for WTS_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTS_SOCKADDR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SOCKADDR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0 {}
impl ::core::default::Default for WTS_SOCKADDR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SOCKADDR_0_0").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SOCKADDR_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SOCKADDR_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_0 {}
impl ::core::default::Default for WTS_SOCKADDR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_1 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SOCKADDR_0_1").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SOCKADDR_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SOCKADDR_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_1 {}
impl ::core::default::Default for WTS_SOCKADDR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WTS_SYSTEMTIME {}
impl ::core::clone::Clone for WTS_SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_SYSTEMTIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_SYSTEMTIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_SYSTEMTIME {}
impl ::core::default::Default for WTS_SYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for WTS_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WTS_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_TIME_ZONE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_TIME_ZONE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_TYPE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = WTS_TYPE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(2i32);
impl ::core::marker::Copy for WTS_TYPE_CLASS {}
impl ::core::clone::Clone for WTS_TYPE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_TYPE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_TYPE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_TYPE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_TYPE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl ::core::marker::Copy for WTS_USER_CREDENTIAL {}
impl ::core::clone::Clone for WTS_USER_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_USER_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_CREDENTIAL").field("UserName", &self.UserName).field("Password", &self.Password).field("Domain", &self.Domain).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_USER_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_USER_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_USER_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_USER_CREDENTIAL {}
impl ::core::default::Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl ::core::marker::Copy for WTS_USER_DATA {}
impl ::core::clone::Clone for WTS_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_USER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_DATA").field("WorkDirectory", &self.WorkDirectory).field("InitialProgram", &self.InitialProgram).field("UserTimeZone", &self.UserTimeZone).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_USER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_USER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_USER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_USER_DATA {}
impl ::core::default::Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: _WTS_PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONA").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WTS_VALIDATION_INFORMATIONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_VALIDATION_INFORMATIONA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: _WTS_PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONW {}
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONW").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WTS_VALIDATION_INFORMATIONW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTS_VALIDATION_INFORMATIONW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONW {}
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WTS_VIRTUAL_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(1i32);
impl ::core::marker::Copy for WTS_VIRTUAL_CLASS {}
impl ::core::clone::Clone for WTS_VIRTUAL_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_VIRTUAL_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WTS_VIRTUAL_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WTS_VIRTUAL_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_VIRTUAL_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_LOGOFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_POWEROFF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_REBOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f1dfca6_3aad_48e1_8406_4bc21a501d7c);
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _ITSWkspEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_ITSWkspEvents> for ::windows::core::IUnknown {
    fn from(value: _ITSWkspEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_ITSWkspEvents> for ::windows::core::IUnknown {
    fn from(value: &_ITSWkspEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ITSWkspEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _ITSWkspEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_ITSWkspEvents> for super::Com::IDispatch {
    fn from(value: _ITSWkspEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_ITSWkspEvents> for super::Com::IDispatch {
    fn from(value: &_ITSWkspEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _ITSWkspEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a _ITSWkspEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _ITSWkspEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _ITSWkspEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _ITSWkspEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ITSWkspEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _ITSWkspEvents {
    type Vtable = _ITSWkspEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ITSWkspEvents_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _WTS_PRODUCT_INFOA {
    pub CompanyName: [super::super::Foundation::CHAR; 256],
    pub ProductID: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _WTS_PRODUCT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _WTS_PRODUCT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_WTS_PRODUCT_INFOA").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _WTS_PRODUCT_INFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _WTS_PRODUCT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_WTS_PRODUCT_INFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _WTS_PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct _WTS_PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl ::core::marker::Copy for _WTS_PRODUCT_INFOW {}
impl ::core::clone::Clone for _WTS_PRODUCT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _WTS_PRODUCT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_WTS_PRODUCT_INFOW").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
unsafe impl ::windows::core::Abi for _WTS_PRODUCT_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _WTS_PRODUCT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_WTS_PRODUCT_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for _WTS_PRODUCT_INFOW {}
impl ::core::default::Default for _WTS_PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: ::windows::core::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource {}
impl ::core::clone::Clone for pluginResource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource")
            .field("alias", &self.alias)
            .field("name", &self.name)
            .field("resourceFileContents", &self.resourceFileContents)
            .field("fileExtension", &self.fileExtension)
            .field("resourcePluginType", &self.resourcePluginType)
            .field("isDiscoverable", &self.isDiscoverable)
            .field("resourceType", &self.resourceType)
            .field("pceIconSize", &self.pceIconSize)
            .field("iconContents", &self.iconContents)
            .field("pcePluginBlobSize", &self.pcePluginBlobSize)
            .field("blobContents", &self.blobContents)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for pluginResource {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for pluginResource {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<pluginResource>()) == 0 }
    }
}
impl ::core::cmp::Eq for pluginResource {}
impl ::core::default::Default for pluginResource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: ::windows::core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl ::core::marker::Copy for pluginResource2 {}
impl ::core::clone::Clone for pluginResource2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2").field("resourceV1", &self.resourceV1).field("pceFileAssocListSize", &self.pceFileAssocListSize).field("fileAssocList", &self.fileAssocList).field("securityDescriptor", &self.securityDescriptor).field("pceFolderListSize", &self.pceFolderListSize).field("folderList", &self.folderList).finish()
    }
}
unsafe impl ::windows::core::Abi for pluginResource2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for pluginResource2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<pluginResource2>()) == 0 }
    }
}
impl ::core::cmp::Eq for pluginResource2 {}
impl ::core::default::Default for pluginResource2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource2FileAssociation {}
impl ::core::clone::Clone for pluginResource2FileAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource2FileAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2FileAssociation").field("extName", &self.extName).field("primaryHandler", &self.primaryHandler).field("pceIconSize", &self.pceIconSize).field("iconContents", &self.iconContents).finish()
    }
}
unsafe impl ::windows::core::Abi for pluginResource2FileAssociation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for pluginResource2FileAssociation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<pluginResource2FileAssociation>()) == 0 }
    }
}
impl ::core::cmp::Eq for pluginResource2FileAssociation {}
impl ::core::default::Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
