#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
    pub mainSessionId: ::windows::runtime::GUID,
    pub subSessionId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl AAAccountingData {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AAAccountingData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AAAccountingData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AAAccountingData")
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
impl ::core::cmp::PartialEq for AAAccountingData {
    fn eq(&self, other: &Self) -> bool {
        self.userName == other.userName
            && self.clientName == other.clientName
            && self.authType == other.authType
            && self.resourceName == other.resourceName
            && self.portNumber == other.portNumber
            && self.protocolName == other.protocolName
            && self.numberOfBytesReceived == other.numberOfBytesReceived
            && self.numberOfBytesTransfered == other.numberOfBytesTransfered
            && self.reasonForDisconnect == other.reasonForDisconnect
            && self.mainSessionId == other.mainSessionId
            && self.subSessionId == other.subSessionId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AAAccountingData {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AAAccountingData {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AAAccountingDataType(pub i32);
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(0i32);
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(1i32);
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(2i32);
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(3i32);
impl ::core::convert::From<i32> for AAAccountingDataType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AAAccountingDataType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AAAuthSchemes(pub i32);
pub const AA_AUTH_MIN: AAAuthSchemes = AAAuthSchemes(0i32);
pub const AA_AUTH_BASIC: AAAuthSchemes = AAAuthSchemes(1i32);
pub const AA_AUTH_NTLM: AAAuthSchemes = AAAuthSchemes(2i32);
pub const AA_AUTH_SC: AAAuthSchemes = AAAuthSchemes(3i32);
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = AAAuthSchemes(4i32);
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = AAAuthSchemes(5i32);
pub const AA_AUTH_ANY: AAAuthSchemes = AAAuthSchemes(6i32);
pub const AA_AUTH_COOKIE: AAAuthSchemes = AAAuthSchemes(7i32);
pub const AA_AUTH_DIGEST: AAAuthSchemes = AAAuthSchemes(8i32);
pub const AA_AUTH_ORGID: AAAuthSchemes = AAAuthSchemes(9i32);
pub const AA_AUTH_CONID: AAAuthSchemes = AAAuthSchemes(10i32);
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = AAAuthSchemes(11i32);
pub const AA_AUTH_MAX: AAAuthSchemes = AAAuthSchemes(12i32);
impl ::core::convert::From<i32> for AAAuthSchemes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AAAuthSchemes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AATrustClassID(pub i32);
pub const AA_UNTRUSTED: AATrustClassID = AATrustClassID(0i32);
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = AATrustClassID(1i32);
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = AATrustClassID(2i32);
impl ::core::convert::From<i32> for AATrustClassID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AATrustClassID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3806972646, 7803, 19342, [186, 189, 233, 191, 98, 146, 172, 41]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl AE_CURRENT_POSITION {}
impl ::core::default::Default for AE_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AE_CURRENT_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AE_CURRENT_POSITION")
            .field("u64DevicePosition", &self.u64DevicePosition)
            .field("u64StreamPosition", &self.u64StreamPosition)
            .field("u64PaddingFrames", &self.u64PaddingFrames)
            .field("hnsQPCPosition", &self.hnsQPCPosition)
            .field("f32FramesPerSecond", &self.f32FramesPerSecond)
            .field("Flag", &self.Flag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AE_CURRENT_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.u64DevicePosition == other.u64DevicePosition && self.u64StreamPosition == other.u64StreamPosition && self.u64PaddingFrames == other.u64PaddingFrames && self.hnsQPCPosition == other.hnsQPCPosition && self.f32FramesPerSecond == other.f32FramesPerSecond && self.Flag == other.Flag
    }
}
impl ::core::cmp::Eq for AE_CURRENT_POSITION {}
unsafe impl ::windows::runtime::Abi for AE_CURRENT_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AE_POSITION_FLAGS(pub i32);
pub const POSITION_INVALID: AE_POSITION_FLAGS = AE_POSITION_FLAGS(0i32);
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(1i32);
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(2i32);
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = AE_POSITION_FLAGS(4i32);
impl ::core::convert::From<i32> for AE_POSITION_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AE_POSITION_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl BITMAP_RENDERER_STATISTICS {}
impl ::core::default::Default for BITMAP_RENDERER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BITMAP_RENDERER_STATISTICS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BITMAP_RENDERER_STATISTICS").field("dwFramesDelivered", &self.dwFramesDelivered).field("dwFramesDropped", &self.dwFramesDropped).finish()
    }
}
impl ::core::cmp::PartialEq for BITMAP_RENDERER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFramesDelivered == other.dwFramesDelivered && self.dwFramesDropped == other.dwFramesDropped
    }
}
impl ::core::cmp::Eq for BITMAP_RENDERER_STATISTICS {}
unsafe impl ::windows::runtime::Abi for BITMAP_RENDERER_STATISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct CHANNEL_DEF {
    pub name: [super::super::Foundation::CHAR; 8],
    pub options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANNEL_DEF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHANNEL_DEF {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: ::core::option::Option<PVIRTUALCHANNELINIT>,
    pub pVirtualChannelOpen: ::core::option::Option<PVIRTUALCHANNELOPEN>,
    pub pVirtualChannelClose: ::core::option::Option<PVIRTUALCHANNELCLOSE>,
    pub pVirtualChannelWrite: ::core::option::Option<PVIRTUALCHANNELWRITE>,
}
#[cfg(feature = "Win32_Foundation")]
impl CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_ENTRY_POINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANNEL_ENTRY_POINTS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHANNEL_ENTRY_POINTS").field("cbSize", &self.cbSize).field("protocolVersion", &self.protocolVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANNEL_ENTRY_POINTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.protocolVersion == other.protocolVersion && self.pVirtualChannelInit.map(|f| f as usize) == other.pVirtualChannelInit.map(|f| f as usize) && self.pVirtualChannelOpen.map(|f| f as usize) == other.pVirtualChannelOpen.map(|f| f as usize) && self.pVirtualChannelClose.map(|f| f as usize) == other.pVirtualChannelClose.map(|f| f as usize) && self.pVirtualChannelWrite.map(|f| f as usize) == other.pVirtualChannelWrite.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHANNEL_ENTRY_POINTS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_NAME_LEN: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl CHANNEL_PDU_HEADER {}
impl ::core::default::Default for CHANNEL_PDU_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CHANNEL_PDU_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHANNEL_PDU_HEADER").field("length", &self.length).field("flags", &self.flags).finish()
    }
}
impl ::core::cmp::PartialEq for CHANNEL_PDU_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for CHANNEL_PDU_HEADER {}
unsafe impl ::windows::runtime::Abi for CHANNEL_PDU_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CLIENTNAME_LENGTH: u32 = 20u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl CLIENT_DISPLAY {}
impl ::core::default::Default for CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CLIENT_DISPLAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::core::cmp::PartialEq for CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for CLIENT_DISPLAY {}
unsafe impl ::windows::runtime::Abi for CLIENT_DISPLAY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLIENT_MESSAGE_TYPE(pub i32);
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(0i32);
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(1i32);
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(2i32);
impl ::core::convert::From<i32> for CLIENT_MESSAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLIENT_MESSAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONNECTION_CHANGE_NOTIFICATION(pub i32);
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(0i32);
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(1i32);
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(2i32);
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(3i32);
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(4i32);
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(5i32);
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(6i32);
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(7i32);
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(8i32);
impl ::core::convert::From<i32> for CONNECTION_CHANGE_NOTIFICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONNECTION_CHANGE_NOTIFICATION {
    type Abi = Self;
}
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1259668864, 65188, 19772, [157, 228, 116, 51, 166, 102, 24, 247]);
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1765769205, 3150, 19735, [184, 224, 31, 112, 50, 94, 93, 88]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_CONNECTED: u32 = 751u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_CONNECTING: u32 = 750u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DOMAIN_LENGTH: u32 = 17u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const FORCE_REJOIN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HwtsVirtualChannelHandle(pub isize);
impl ::core::default::Default for HwtsVirtualChannelHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HwtsVirtualChannelHandle {}
unsafe impl ::windows::runtime::Abi for HwtsVirtualChannelHandle {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IADsTSUserEx(pub ::windows::runtime::IUnknown);
impl IADsTSUserEx {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TerminalServicesProfilePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTerminalServicesProfilePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TerminalServicesHomeDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTerminalServicesHomeDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TerminalServicesHomeDrive(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTerminalServicesHomeDrive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AllowLogon(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetAllowLogon(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn EnableRemoteControl(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetEnableRemoteControl(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn MaxDisconnectionTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetMaxDisconnectionTime(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn MaxConnectionTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetMaxConnectionTime(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn MaxIdleTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetMaxIdleTime(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ReconnectionAction(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetReconnectionAction(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn BrokenConnectionAction(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetBrokenConnectionAction(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ConnectClientDrivesAtLogon(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ConnectClientPrintersAtLogon(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn DefaultToMainPrinter(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetDefaultToMainPrinter(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TerminalServicesWorkDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTerminalServicesWorkDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TerminalServicesInitialProgram(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTerminalServicesInitialProgram<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pnewval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), pnewval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IADsTSUserEx {
    type Vtable = IADsTSUserEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3297971833, 10633, 17506, [138, 96, 47, 207, 47, 41, 85, 239]);
}
impl ::core::convert::From<IADsTSUserEx> for ::windows::runtime::IUnknown {
    fn from(value: IADsTSUserEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IADsTSUserEx> for ::windows::runtime::IUnknown {
    fn from(value: &IADsTSUserEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IADsTSUserEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IADsTSUserEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IADsTSUserEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IADsTSUserEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsTSUserEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioDeviceEndpoint(pub ::windows::runtime::IUnknown);
impl IAudioDeviceEndpoint {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxperiod), ::core::mem::transmute(u32latencycoefficient)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetRTCaps(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetEventDrivenCapable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(htargetprocess), ::core::mem::transmute(hnsperiod), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(u32latencycoefficient), ::core::mem::transmute(pu32sharedmemorysize), ::core::mem::transmute(phsharedmemory)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioDeviceEndpoint {
    type Vtable = IAudioDeviceEndpoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3566546778, 41138, 19652, [139, 130, 147, 88, 72, 141, 216, 172]);
}
impl ::core::convert::From<IAudioDeviceEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: IAudioDeviceEndpoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioDeviceEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioDeviceEndpoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioDeviceEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioDeviceEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceEndpoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpoint(pub ::windows::runtime::IUnknown);
impl IAudioEndpoint {
    #[cfg(feature = "Win32_Media_Audio")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Media_Audio`*"]
    pub unsafe fn GetFrameFormat(&self) -> ::windows::runtime::Result<*mut super::super::Media::Audio::WAVEFORMATEX> {
        let mut result__: <*mut super::super::Media::Audio::WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::Media::Audio::WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetFramesPerPacket(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetStreamFlags(&self, streamflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpoint {
    type Vtable = IAudioEndpoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(816420117, 5415, 17489, [175, 159, 0, 197, 240, 35, 77, 175]);
}
impl ::core::convert::From<IAudioEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_Audio")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pframesperpacket: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, platency: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointControl(pub ::windows::runtime::IUnknown);
impl IAudioEndpointControl {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointControl {
    type Vtable = IAudioEndpointControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3330586410, 28148, 18292, [189, 249, 118, 183, 117, 9, 182, 83]);
}
impl ::core::convert::From<IAudioEndpointControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointRT(pub ::windows::runtime::IUnknown);
impl IAudioEndpointRT {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppadding), ::core::mem::transmute(paecurrentposition)))
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ProcessingComplete(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetPinInactive(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetPinActive(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointRT {
    type Vtable = IAudioEndpointRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3755081823, 42725, 19769, [162, 101, 147, 154, 218, 159, 187, 77]);
}
impl ::core::convert::From<IAudioEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointRT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointRT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointRT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioInputEndpointRT(pub ::windows::runtime::IUnknown);
impl IAudioInputEndpointRT {
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pconnectionproperty), ::core::mem::transmute(paetimestamp)))
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32framecount), ::core::mem::transmute(pdatapointer)))
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn PulseEndpoint(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioInputEndpointRT {
    type Vtable = IAudioInputEndpointRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2150017889, 37554, 17345, [161, 223, 92, 55, 235, 208, 141, 130]);
}
impl ::core::convert::From<IAudioInputEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: IAudioInputEndpointRT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioInputEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioInputEndpointRT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioInputEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioInputEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputEndpointRT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_Audio_Apo")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32framecount: u32, pdatapointer: usize),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioOutputEndpointRT(pub ::windows::runtime::IUnknown);
impl IAudioOutputEndpointRT {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32framecount), ::core::mem::transmute(paetimestamp)))
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Media_Audio_Apo`*"]
    pub unsafe fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pconnectionproperty)))
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn PulseEndpoint(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioOutputEndpointRT {
    type Vtable = IAudioOutputEndpointRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2410219236, 49948, 20017, [147, 46, 25, 166, 99, 133, 233, 170]);
}
impl ::core::convert::From<IAudioOutputEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: IAudioOutputEndpointRT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioOutputEndpointRT> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioOutputEndpointRT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioOutputEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioOutputEndpointRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputEndpointRT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteDesktopClient(pub ::windows::runtime::IUnknown);
impl IRemoteDesktopClient {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Connect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Reconnect(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Settings(&self) -> ::windows::runtime::Result<IRemoteDesktopClientSettings> {
        let mut result__: <IRemoteDesktopClientSettings as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRemoteDesktopClientSettings>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Actions(&self) -> ::windows::runtime::Result<IRemoteDesktopClientActions> {
        let mut result__: <IRemoteDesktopClientActions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRemoteDesktopClientActions>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn TouchPointer(&self) -> ::windows::runtime::Result<IRemoteDesktopClientTouchPointer> {
        let mut result__: <IRemoteDesktopClientTouchPointer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRemoteDesktopClientTouchPointer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeleteSavedCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), servername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn attachEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, eventname: Param0, callback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn detachEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, eventname: Param0, callback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteDesktopClient {
    type Vtable = IRemoteDesktopClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1473402472, 25178, 18693, [190, 78, 48, 76, 170, 19, 248, 156]);
}
impl ::core::convert::From<IRemoteDesktopClient> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteDesktopClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteDesktopClient> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteDesktopClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteDesktopClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteDesktopClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IRemoteDesktopClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: u32, height: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, touchpointer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: u32, height: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteDesktopClientActions(pub ::windows::runtime::IUnknown);
impl IRemoteDesktopClientActions {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SuspendScreenUpdates(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ResumeScreenUpdates(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(remoteaction)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(snapshotencoding), ::core::mem::transmute(snapshotformat), ::core::mem::transmute(snapshotwidth), ::core::mem::transmute(snapshotheight), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteDesktopClientActions {
    type Vtable = IRemoteDesktopClientActions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2102705230, 4136, 17876, [139, 10, 185, 182, 191, 251, 161, 118]);
}
impl ::core::convert::From<IRemoteDesktopClientActions> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteDesktopClientActions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteDesktopClientActions> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteDesktopClientActions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IRemoteDesktopClientActions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientActions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaction: RemoteActionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteDesktopClientSettings(pub ::windows::runtime::IUnknown);
impl IRemoteDesktopClientSettings {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ApplySettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rdpfilecontents: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), rdpfilecontents.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RetrieveSettings(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetRdpProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetRdpProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, propertyname: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteDesktopClientSettings {
    type Vtable = IRemoteDesktopClientSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1218507431, 10003, 17183, [187, 172, 111, 69, 88, 231, 214, 77]);
}
impl ::core::convert::From<IRemoteDesktopClientSettings> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteDesktopClientSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteDesktopClientSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteDesktopClientSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IRemoteDesktopClientSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rdpfilecontents: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteDesktopClientTouchPointer(pub ::windows::runtime::IUnknown);
impl IRemoteDesktopClientTouchPointer {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetEventsEnabled(&self, eventsenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventsenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn EventsEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetPointerSpeed(&self, pointerspeed: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerspeed)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn PointerSpeed(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteDesktopClientTouchPointer {
    type Vtable = IRemoteDesktopClientTouchPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(638501421, 36028, 17589, [158, 136, 42, 55, 246, 201, 58, 233]);
}
impl ::core::convert::From<IRemoteDesktopClientTouchPointer> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteDesktopClientTouchPointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteDesktopClientTouchPointer> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteDesktopClientTouchPointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IRemoteDesktopClientTouchPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientTouchPointer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventsenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventsenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerspeed: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerspeed: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteSystemAdditionalInfoProvider(pub ::windows::runtime::IUnknown);
impl IRemoteSystemAdditionalInfoProvider {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetAdditionalInfo<T: ::windows::runtime::Interface>(&self, deduplicationid: *mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(deduplicationid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteSystemAdditionalInfoProvider {
    type Vtable = IRemoteSystemAdditionalInfoProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4004134239, 60515, 19751, [175, 56, 232, 107, 29, 114, 146, 203]);
}
impl ::core::convert::From<IRemoteSystemAdditionalInfoProvider> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteSystemAdditionalInfoProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteSystemAdditionalInfoProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteSystemAdditionalInfoProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteSystemAdditionalInfoProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteSystemAdditionalInfoProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAdditionalInfoProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deduplicationid: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, riid: *const ::windows::runtime::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGAccountingEngine(pub ::windows::runtime::IUnknown);
impl ITSGAccountingEngine {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DoAccounting<'a, Param1: ::windows::runtime::IntoParam<'a, AAAccountingData>>(&self, accountingdatatype: AAAccountingDataType, accountingdata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(accountingdatatype), accountingdata.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITSGAccountingEngine {
    type Vtable = ITSGAccountingEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1289920713, 59508, 20250, [134, 244, 6, 187, 185, 17, 83, 56]);
}
impl ::core::convert::From<ITSGAccountingEngine> for ::windows::runtime::IUnknown {
    fn from(value: ITSGAccountingEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGAccountingEngine> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGAccountingEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGAccountingEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGAccountingEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAccountingEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGAuthenticateUserSink(pub ::windows::runtime::IUnknown);
impl ITSGAuthenticateUserSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnUserAuthenticated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, username: Param0, userdomain: Param1, context: usize, usertoken: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), username.into_param().abi(), userdomain.into_param().abi(), ::core::mem::transmute(context), usertoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: ::windows::runtime::HRESULT, specificerrorcode: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), ::core::mem::transmute(genericerrorcode), ::core::mem::transmute(specificerrorcode)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ReauthenticateUser(&self, context: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn DisconnectUser(&self, context: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITSGAuthenticateUserSink {
    type Vtable = ITSGAuthenticateUserSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(742272627, 42882, 18425, [141, 251, 119, 238, 30, 210, 122, 3]);
}
impl ::core::convert::From<ITSGAuthenticateUserSink> for ::windows::runtime::IUnknown {
    fn from(value: ITSGAuthenticateUserSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGAuthenticateUserSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGAuthenticateUserSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGAuthenticateUserSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGAuthenticateUserSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticateUserSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: usize, genericerrorcode: ::windows::runtime::HRESULT, specificerrorcode: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGAuthenticationEngine(pub ::windows::runtime::IUnknown);
impl ITSGAuthenticationEngine {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AuthenticateUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, ITSGAuthenticateUserSink>>(&self, mainsessionid: Param0, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mainsessionid.into_param().abi(), ::core::mem::transmute(cookiedata), ::core::mem::transmute(numcookiebytes), ::core::mem::transmute(context), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CancelAuthentication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, mainsessionid: Param0, context: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mainsessionid.into_param().abi(), ::core::mem::transmute(context)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITSGAuthenticationEngine {
    type Vtable = ITSGAuthenticationEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2665735615, 1195, 18065, [153, 140, 215, 246, 34, 50, 26, 86]);
}
impl ::core::convert::From<ITSGAuthenticationEngine> for ::windows::runtime::IUnknown {
    fn from(value: ITSGAuthenticationEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGAuthenticationEngine> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGAuthenticationEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGAuthenticationEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGAuthenticationEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticationEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainsessionid: ::windows::runtime::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainsessionid: ::windows::runtime::GUID, context: usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGAuthorizeConnectionSink(pub ::windows::runtime::IUnknown);
impl ITSGAuthorizeConnectionSink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnConnectionAuthorized<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, hrin: ::windows::runtime::HRESULT, mainsessionid: Param1, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(hrin),
            mainsessionid.into_param().abi(),
            ::core::mem::transmute(cbsohresponse),
            ::core::mem::transmute(pbsohresponse),
            ::core::mem::transmute(idletimeout),
            ::core::mem::transmute(sessiontimeout),
            ::core::mem::transmute(sessiontimeoutaction),
            ::core::mem::transmute(trustclass),
            ::core::mem::transmute(policyattributes),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITSGAuthorizeConnectionSink {
    type Vtable = ITSGAuthorizeConnectionSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3263090227, 30593, 17176, [152, 239, 28, 242, 218, 123, 112, 5]);
}
impl ::core::convert::From<ITSGAuthorizeConnectionSink> for ::windows::runtime::IUnknown {
    fn from(value: ITSGAuthorizeConnectionSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGAuthorizeConnectionSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGAuthorizeConnectionSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGAuthorizeConnectionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGAuthorizeConnectionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeConnectionSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrin: ::windows::runtime::HRESULT, mainsessionid: ::windows::runtime::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGAuthorizeResourceSink(pub ::windows::runtime::IUnknown);
impl ITSGAuthorizeResourceSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnChannelAuthorized<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, hrin: ::windows::runtime::HRESULT, mainsessionid: Param1, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(hrin),
            mainsessionid.into_param().abi(),
            ::core::mem::transmute(subsessionid),
            ::core::mem::transmute(allowedresourcenames),
            ::core::mem::transmute(numallowedresourcenames),
            ::core::mem::transmute(failedresourcenames),
            ::core::mem::transmute(numfailedresourcenames),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITSGAuthorizeResourceSink {
    type Vtable = ITSGAuthorizeResourceSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4275961044, 64018, 17461, [174, 85, 122, 209, 169, 119, 154, 247]);
}
impl ::core::convert::From<ITSGAuthorizeResourceSink> for ::windows::runtime::IUnknown {
    fn from(value: ITSGAuthorizeResourceSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGAuthorizeResourceSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGAuthorizeResourceSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGAuthorizeResourceSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGAuthorizeResourceSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeResourceSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrin: ::windows::runtime::HRESULT, mainsessionid: ::windows::runtime::GUID, subsessionid: i32, allowedresourcenames: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numallowedresourcenames: u32, failedresourcenames: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numfailedresourcenames: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITSGPolicyEngine(pub ::windows::runtime::IUnknown);
impl ITSGPolicyEngine {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn AuthorizeConnection<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param10: ::windows::runtime::IntoParam<'a, ITSGAuthorizeConnectionSink>>(
        &self,
        mainsessionid: Param0,
        username: Param1,
        authtype: AAAuthSchemes,
        clientmachineip: Param3,
        clientmachinename: Param4,
        sohdata: *const u8,
        numsohbytes: u32,
        cookiedata: *const u8,
        numcookiebytes: u32,
        usertoken: Param9,
        psink: Param10,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            mainsessionid.into_param().abi(),
            username.into_param().abi(),
            ::core::mem::transmute(authtype),
            clientmachineip.into_param().abi(),
            clientmachinename.into_param().abi(),
            ::core::mem::transmute(sohdata),
            ::core::mem::transmute(numsohbytes),
            ::core::mem::transmute(cookiedata),
            ::core::mem::transmute(numcookiebytes),
            usertoken.into_param().abi(),
            psink.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn AuthorizeResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param11: ::windows::runtime::IntoParam<'a, ITSGAuthorizeResourceSink>>(
        &self,
        mainsessionid: Param0,
        subsessionid: i32,
        username: Param2,
        resourcenames: *const super::super::Foundation::BSTR,
        numresources: u32,
        alternateresourcenames: *const super::super::Foundation::BSTR,
        numalternateresourcename: u32,
        portnumber: u32,
        operation: Param8,
        cookie: *const u8,
        numbytesincookie: u32,
        psink: Param11,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            mainsessionid.into_param().abi(),
            ::core::mem::transmute(subsessionid),
            username.into_param().abi(),
            ::core::mem::transmute(resourcenames),
            ::core::mem::transmute(numresources),
            ::core::mem::transmute(alternateresourcenames),
            ::core::mem::transmute(numalternateresourcename),
            ::core::mem::transmute(portnumber),
            operation.into_param().abi(),
            ::core::mem::transmute(cookie),
            ::core::mem::transmute(numbytesincookie),
            psink.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsQuarantineEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITSGPolicyEngine {
    type Vtable = ITSGPolicyEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2344767240, 25123, 17140, [165, 180, 142, 55, 205, 19, 91, 189]);
}
impl ::core::convert::From<ITSGPolicyEngine> for ::windows::runtime::IUnknown {
    fn from(value: ITSGPolicyEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITSGPolicyEngine> for ::windows::runtime::IUnknown {
    fn from(value: &ITSGPolicyEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITSGPolicyEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITSGPolicyEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGPolicyEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mainsessionid: ::windows::runtime::GUID,
        username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        authtype: AAAuthSchemes,
        clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        sohdata: *const u8,
        numsohbytes: u32,
        cookiedata: *const u8,
        numcookiebytes: u32,
        usertoken: super::super::Foundation::HANDLE_PTR,
        psink: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mainsessionid: ::windows::runtime::GUID,
        subsessionid: i32,
        username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        resourcenames: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        numresources: u32,
        alternateresourcenames: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        numalternateresourcename: u32,
        portnumber: u32,
        operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        cookie: *const u8,
        numbytesincookie: u32,
        psink: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbBaseNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbBaseNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbBaseNotifySink {
    type Vtable = ITsSbBaseNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2156553527, 4738, 18825, [158, 9, 244, 57, 56, 183, 23, 34]);
}
impl ::core::convert::From<ITsSbBaseNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbBaseNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbBaseNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbBaseNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbBaseNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbBaseNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbBaseNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbClientConnection(pub ::windows::runtime::IUnknown);
impl ITsSbClientConnection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn UserName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn InitialProgram(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn LoadBalanceResult(&self) -> ::windows::runtime::Result<ITsSbLoadBalanceResult> {
        let mut result__: <ITsSbLoadBalanceResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbLoadBalanceResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn FarmName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PutContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, contextid: Param0, context: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), contextid.into_param().abi(), context.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, contextid: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), contextid.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Environment(&self) -> ::windows::runtime::Result<ITsSbEnvironment> {
        let mut result__: <ITsSbEnvironment as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ConnectionError(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SamUserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ClientConnectionPropertySet(&self) -> ::windows::runtime::Result<ITsSbClientConnectionPropertySet> {
        let mut result__: <ITsSbClientConnectionPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbClientConnectionPropertySet>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsFirstAssignment(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RdFarmType(&self) -> ::windows::runtime::Result<RD_FARM_TYPE> {
        let mut result__: <RD_FARM_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RD_FARM_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn UserSidString(&self) -> ::windows::runtime::Result<*mut i8> {
        let mut result__: <*mut i8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut i8>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetDisconnectedSession(&self) -> ::windows::runtime::Result<ITsSbSession> {
        let mut result__: <ITsSbSession as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbSession>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbClientConnection {
    type Vtable = ITsSbClientConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(411399321, 44385, 19227, [183, 223, 203, 205, 65, 251, 131, 56]);
}
impl ::core::convert::From<ITsSbClientConnection> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbClientConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbClientConnection> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbClientConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbClientConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbClientConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenvironment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszusersidstring: *mut *mut i8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbClientConnectionPropertySet(pub ::windows::runtime::IUnknown);
impl ITsSbClientConnectionPropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbClientConnectionPropertySet {
    type Vtable = ITsSbClientConnectionPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3843659184, 18134, 4573, [170, 33, 206, 220, 85, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbClientConnectionPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbClientConnectionPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbClientConnectionPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbClientConnectionPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITsSbClientConnectionPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbClientConnectionPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbClientConnectionPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbClientConnectionPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for &ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &ITsSbClientConnectionPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnectionPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbEnvironment(pub ::windows::runtime::IUnknown);
impl ITsSbEnvironment {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ServerWeight(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn EnvironmentPropertySet(&self) -> ::windows::runtime::Result<ITsSbEnvironmentPropertySet> {
        let mut result__: <ITsSbEnvironmentPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbEnvironmentPropertySet>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetEnvironmentPropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbEnvironmentPropertySet>>(&self, pval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbEnvironment {
    type Vtable = ITsSbEnvironment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2357721079, 48977, 19036, [135, 191, 142, 148, 251, 110, 34, 86]);
}
impl ::core::convert::From<ITsSbEnvironment> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbEnvironment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbEnvironment> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbEnvironment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbEnvironmentPropertySet(pub ::windows::runtime::IUnknown);
impl ITsSbEnvironmentPropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbEnvironmentPropertySet {
    type Vtable = ITsSbEnvironmentPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3503406974, 31439, 4573, [162, 67, 229, 17, 86, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbEnvironmentPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbEnvironmentPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbEnvironmentPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbEnvironmentPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITsSbEnvironmentPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbEnvironmentPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbEnvironmentPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbEnvironmentPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for &ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &ITsSbEnvironmentPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironmentPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbFilterPluginStore(pub ::windows::runtime::IUnknown);
impl ITsSbFilterPluginStore {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SaveProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbPropertySet>>(&self, ppropertyset: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn EnumerateProperties(&self) -> ::windows::runtime::Result<ITsSbPropertySet> {
        let mut result__: <ITsSbPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbPropertySet>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeleteProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), propertyname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbFilterPluginStore {
    type Vtable = ITsSbFilterPluginStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2243185423, 60792, 16703, [151, 2, 250, 109, 59, 94, 231, 85]);
}
impl ::core::convert::From<ITsSbFilterPluginStore> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbFilterPluginStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbFilterPluginStore> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbFilterPluginStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbFilterPluginStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbFilterPluginStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbFilterPluginStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbGenericNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbGenericNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnCompleted(&self, status: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetWaitTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbGenericNotifySink {
    type Vtable = ITsSbGenericNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1280085071, 12299, 18093, [145, 100, 132, 104, 167, 231, 86, 140]);
}
impl ::core::convert::From<ITsSbGenericNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbGenericNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbGenericNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbGenericNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbGenericNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbGenericNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGenericNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbGlobalStore(pub ::windows::runtime::IUnknown);
impl ITsSbGlobalStore {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QueryTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, targetname: Param1, farmname: Param2) -> ::windows::runtime::Result<ITsSbTarget> {
        let mut result__: <ITsSbTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), providername.into_param().abi(), targetname.into_param().abi(), farmname.into_param().abi(), &mut result__).from_abi::<ITsSbTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QuerySessionBySessionId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, dwsessionid: u32, targetname: Param2) -> ::windows::runtime::Result<ITsSbSession> {
        let mut result__: <ITsSbSession as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(dwsessionid), targetname.into_param().abi(), &mut result__).from_abi::<ITsSbSession>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EnumerateFarms<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateTargets<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, farmname: Param1, envname: Param2, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), providername.into_param().abi(), farmname.into_param().abi(), envname.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateEnvironmentsByProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, providername: Param0, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), providername.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(ppval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateSessions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        providername: Param0,
        targetname: Param1,
        username: Param2,
        userdomain: Param3,
        poolname: Param4,
        initialprogram: Param5,
        psessionstate: *const TSSESSION_STATE,
        pdwcount: *mut u32,
        ppval: *mut *mut ::core::option::Option<ITsSbSession>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            providername.into_param().abi(),
            targetname.into_param().abi(),
            username.into_param().abi(),
            userdomain.into_param().abi(),
            poolname.into_param().abi(),
            initialprogram.into_param().abi(),
            ::core::mem::transmute(psessionstate),
            ::core::mem::transmute(pdwcount),
            ::core::mem::transmute(ppval),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetFarmProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, propertyname: Param1, pvarvalue: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), farmname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbGlobalStore {
    type Vtable = ITsSbGlobalStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2595622779, 48498, 19871, [138, 58, 160, 234, 85, 116, 230, 53]);
}
impl ::core::convert::From<ITsSbGlobalStore> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbGlobalStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbGlobalStore> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbGlobalStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbGlobalStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbGlobalStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGlobalStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        psessionstate: *const TSSESSION_STATE,
        pdwcount: *mut u32,
        ppval: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbLoadBalanceResult(pub ::windows::runtime::IUnknown);
impl ITsSbLoadBalanceResult {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbLoadBalanceResult {
    type Vtable = ITsSbLoadBalanceResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(620607404, 65190, 4572, [150, 114, 154, 137, 86, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbLoadBalanceResult> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbLoadBalanceResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbLoadBalanceResult> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbLoadBalanceResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbLoadBalanceResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbLoadBalanceResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalanceResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbLoadBalancing(pub ::windows::runtime::IUnknown);
impl ITsSbLoadBalancing {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetMostSuitableTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::runtime::IntoParam<'a, ITsSbLoadBalancingNotifySink>>(&self, pconnection: Param0, plbsink: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), plbsink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbLoadBalancing {
    type Vtable = ITsSbLoadBalancing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607294068, 40631, 4572, [174, 152, 242, 180, 86, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbLoadBalancing> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbLoadBalancing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbLoadBalancing> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbLoadBalancing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbLoadBalancing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr, plbsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbLoadBalancingNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbLoadBalancingNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnGetMostSuitableTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbLoadBalanceResult>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, plbresult: Param0, fisnewconnection: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), plbresult.into_param().abi(), fisnewconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbLoadBalancingNotifySink {
    type Vtable = ITsSbLoadBalancingNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602912919, 12868, 20074, [149, 138, 39, 200, 34, 193, 225, 65]);
}
impl ::core::convert::From<ITsSbLoadBalancingNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbLoadBalancingNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbLoadBalancingNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbLoadBalancingNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for &ITsSbLoadBalancingNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancingNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plbresult: ::windows::runtime::RawPtr, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbOrchestration(pub ::windows::runtime::IUnknown);
impl ITsSbOrchestration {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn PrepareTargetForConnect<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::runtime::IntoParam<'a, ITsSbOrchestrationNotifySink>>(&self, pconnection: Param0, porchestrationnotifysink: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), porchestrationnotifysink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbOrchestration {
    type Vtable = ITsSbOrchestration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1694241138, 40631, 4572, [139, 0, 58, 186, 86, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbOrchestration> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbOrchestration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbOrchestration> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbOrchestration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbOrchestration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbOrchestration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbOrchestration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbOrchestration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr, porchestrationnotifysink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbOrchestrationNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbOrchestrationNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReadyToConnect<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTarget>>(&self, ptarget: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ptarget.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbOrchestrationNotifySink {
    type Vtable = ITsSbOrchestrationNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(918781281, 37483, 17455, [188, 165, 17, 140, 109, 80, 220, 242]);
}
impl ::core::convert::From<ITsSbOrchestrationNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbOrchestrationNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbOrchestrationNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbOrchestrationNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for &ITsSbOrchestrationNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestrationNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPlacement(pub ::windows::runtime::IUnknown);
impl ITsSbPlacement {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn QueryEnvironmentForTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbClientConnection>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPlacementNotifySink>>(&self, pconnection: Param0, pplacementsink: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), pplacementsink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPlacement {
    type Vtable = ITsSbPlacement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3668831839, 27954, 18446, [158, 54, 221, 171, 35, 41, 240, 109]);
}
impl ::core::convert::From<ITsSbPlacement> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPlacement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPlacement> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPlacement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPlacement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPlacement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbPlacement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbPlacement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr, pplacementsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPlacementNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbPlacementNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnQueryEnvironmentCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbEnvironment>>(&self, penvironment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), penvironment.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPlacementNotifySink {
    type Vtable = ITsSbPlacementNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1755366535, 11087, 18114, [148, 161, 108, 230, 133, 24, 54, 52]);
}
impl ::core::convert::From<ITsSbPlacementNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPlacementNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPlacementNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPlacementNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for &ITsSbPlacementNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacementNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penvironment: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPlugin(pub ::windows::runtime::IUnknown);
impl ITsSbPlugin {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPlugin {
    type Vtable = ITsSbPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1221424134, 51883, 18015, [165, 214, 186, 168, 99, 185, 234, 79]);
}
impl ::core::convert::From<ITsSbPlugin> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPluginNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbPluginNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnInitialized(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnTerminated(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPluginNotifySink {
    type Vtable = ITsSbPluginNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1155523339, 50110, 16629, [191, 130, 122, 149, 187, 121, 90, 223]);
}
impl ::core::convert::From<ITsSbPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPluginNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPluginNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for &ITsSbPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPluginPropertySet(pub ::windows::runtime::IUnknown);
impl ITsSbPluginPropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPluginPropertySet {
    type Vtable = ITsSbPluginPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2499833396, 32511, 19308, [187, 64, 73, 164, 253, 167, 206, 166]);
}
impl ::core::convert::From<ITsSbPluginPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPluginPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPluginPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPluginPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITsSbPluginPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbPluginPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbPluginPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbPluginPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for &ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &ITsSbPluginPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbPropertySet(pub ::windows::runtime::IUnknown);
impl ITsSbPropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbPropertySet {
    type Vtable = ITsSbPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1543655793, 47902, 19375, [162, 18, 109, 94, 151, 116, 179, 59]);
}
impl ::core::convert::From<ITsSbPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &ITsSbPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbProvider(pub ::windows::runtime::IUnknown);
impl ITsSbProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateTargetObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, environmentname: Param1) -> ::windows::runtime::Result<ITsSbTarget> {
        let mut result__: <ITsSbTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), targetname.into_param().abi(), environmentname.into_param().abi(), &mut result__).from_abi::<ITsSbTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateLoadBalanceResultObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0) -> ::windows::runtime::Result<ITsSbLoadBalanceResult> {
        let mut result__: <ITsSbLoadBalanceResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), targetname.into_param().abi(), &mut result__).from_abi::<ITsSbLoadBalanceResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateSessionObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, username: Param1, domain: Param2, sessionid: u32) -> ::windows::runtime::Result<ITsSbSession> {
        let mut result__: <ITsSbSession as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), ::core::mem::transmute(sessionid), &mut result__).from_abi::<ITsSbSession>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CreatePluginPropertySet(&self) -> ::windows::runtime::Result<ITsSbPluginPropertySet> {
        let mut result__: <ITsSbPluginPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbPluginPropertySet>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CreateTargetPropertySetObject(&self) -> ::windows::runtime::Result<ITsSbTargetPropertySet> {
        let mut result__: <ITsSbTargetPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbTargetPropertySet>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateEnvironmentObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, serverweight: u32) -> ::windows::runtime::Result<ITsSbEnvironment> {
        let mut result__: <ITsSbEnvironment as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(serverweight), &mut result__).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetResourcePluginStore(&self) -> ::windows::runtime::Result<ITsSbResourcePluginStore> {
        let mut result__: <ITsSbResourcePluginStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbResourcePluginStore>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetFilterPluginStore(&self) -> ::windows::runtime::Result<ITsSbFilterPluginStore> {
        let mut result__: <ITsSbFilterPluginStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbFilterPluginStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RegisterForNotification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ITsSbResourceNotification>>(&self, notificationtype: u32, resourcetomonitor: Param1, ppluginnotification: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), resourcetomonitor.into_param().abi(), ppluginnotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn UnRegisterForNotification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, notificationtype: u32, resourcetomonitor: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), resourcetomonitor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetInstanceOfGlobalStore(&self) -> ::windows::runtime::Result<ITsSbGlobalStore> {
        let mut result__: <ITsSbGlobalStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbGlobalStore>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CreateEnvironmentPropertySetObject(&self) -> ::windows::runtime::Result<ITsSbEnvironmentPropertySet> {
        let mut result__: <ITsSbEnvironmentPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbEnvironmentPropertySet>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbProvider {
    type Vtable = ITsSbProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275674511, 28027, 17629, [188, 23, 140, 228, 78, 55, 13, 82]);
}
impl ::core::convert::From<ITsSbProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppglobalstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbProvisioning(pub ::windows::runtime::IUnknown);
impl ITsSbProvisioning {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateVirtualMachines<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn PatchVirtualMachines<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(pvmpatchinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeleteVirtualMachines<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ITsSbProvisioningPluginNotifySink>>(&self, jobxmlstring: Param0, jobguid: Param1, psink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CancelJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, jobguid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), jobguid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbProvisioning {
    type Vtable = ITsSbProvisioning_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(795807163, 40527, 17963, [156, 63, 252, 204, 61, 203, 98, 50]);
}
impl ::core::convert::From<ITsSbProvisioning> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbProvisioning) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbProvisioning> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbProvisioning) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioning_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::runtime::RawPtr, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbProvisioningPluginNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbProvisioningPluginNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnVirtualMachineStatusChanged<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::runtime::HRESULT, errordescr: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyentry), ::core::mem::transmute(vmnotifystatus), ::core::mem::transmute(errorcode), errordescr.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnJobCompleted<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, resultcode: ::windows::runtime::HRESULT, resultdescription: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(resultcode), resultdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnJobCancelled(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvmnotifyentry)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnVirtualMachineHostStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, vmhost: Param0, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::runtime::HRESULT, errordescr: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), vmhost.into_param().abi(), ::core::mem::transmute(vmhostnotifystatus), ::core::mem::transmute(errorcode), errordescr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbProvisioningPluginNotifySink {
    type Vtable = ITsSbProvisioningPluginNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2896722574, 33163, 17793, [160, 50, 73, 195, 223, 185, 199, 1]);
}
impl ::core::convert::From<ITsSbProvisioningPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbProvisioningPluginNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbProvisioningPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbProvisioningPluginNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbProvisioningPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbProvisioningPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioningPluginNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::runtime::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resultcode: ::windows::runtime::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::runtime::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbResourceNotification(pub ::windows::runtime::IUnknown);
impl ITsSbResourceNotification {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionChange<'a, Param1: ::windows::runtime::IntoParam<'a, ITsSbSession>>(&self, changetype: TSSESSION_STATE, psession: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), psession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyTargetChange<'a, Param1: ::windows::runtime::IntoParam<'a, ITsSbTarget>>(&self, targetchangetype: u32, ptarget: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetchangetype), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyClientConnectionStateChange<'a, Param1: ::windows::runtime::IntoParam<'a, ITsSbClientConnection>>(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), pconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbResourceNotification {
    type Vtable = ITsSbResourceNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1708386394, 50075, 4572, [185, 45, 60, 210, 85, 216, 149, 147]);
}
impl ::core::convert::From<ITsSbResourceNotification> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbResourceNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbResourceNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbResourceNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbResourceNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbResourceNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changetype: TSSESSION_STATE, psession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetchangetype: u32, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbResourceNotificationEx(pub ::windows::runtime::IUnknown);
impl ITsSbResourceNotificationEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn NotifySessionChangeEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, username: Param1, domain: Param2, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(sessionstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn NotifyTargetChangeEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, targetchangetype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(targetchangetype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn NotifyClientConnectionStateChangeEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        username: Param0,
        domain: Param1,
        initialprogram: Param2,
        poolname: Param3,
        targetname: Param4,
        connectionchangetype: CONNECTION_CHANGE_NOTIFICATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), username.into_param().abi(), domain.into_param().abi(), initialprogram.into_param().abi(), poolname.into_param().abi(), targetname.into_param().abi(), ::core::mem::transmute(connectionchangetype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbResourceNotificationEx {
    type Vtable = ITsSbResourceNotificationEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2829352926, 51857, 17618, [184, 151, 58, 162, 138, 67, 178, 183]);
}
impl ::core::convert::From<ITsSbResourceNotificationEx> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbResourceNotificationEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbResourceNotificationEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbResourceNotificationEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbResourceNotificationEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbResourceNotificationEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotificationEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        connectionchangetype: CONNECTION_CHANGE_NOTIFICATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbResourcePlugin(pub ::windows::runtime::IUnknown);
impl ITsSbResourcePlugin {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbResourcePlugin {
    type Vtable = ITsSbResourcePlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3935155244, 39149, 17717, [168, 139, 42, 22, 79, 53, 73, 15]);
}
impl ::core::convert::From<ITsSbResourcePlugin> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbResourcePlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbResourcePlugin> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbResourcePlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbResourcePlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbResourcePluginStore(pub ::windows::runtime::IUnknown);
impl ITsSbResourcePluginStore {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QueryTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, farmname: Param1) -> ::windows::runtime::Result<ITsSbTarget> {
        let mut result__: <ITsSbTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), targetname.into_param().abi(), farmname.into_param().abi(), &mut result__).from_abi::<ITsSbTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QuerySessionBySessionId<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dwsessionid: u32, targetname: Param1) -> ::windows::runtime::Result<ITsSbSession> {
        let mut result__: <ITsSbSession as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsessionid), targetname.into_param().abi(), &mut result__).from_abi::<ITsSbSession>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AddTargetToStore<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTarget>>(&self, ptarget: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AddSessionToStore<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbSession>>(&self, psession: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psession.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AddEnvironmentToStore<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbEnvironment>>(&self, penvironment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), penvironment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RemoveEnvironmentFromStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, environmentname: Param0, bignoreowner: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), bignoreowner.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QueryEnvironment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, environmentname: Param0) -> ::windows::runtime::Result<ITsSbEnvironment> {
        let mut result__: <ITsSbEnvironment as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), &mut result__).from_abi::<ITsSbEnvironment>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SaveTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTarget>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ptarget: Param0, bforcewrite: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ptarget.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SaveEnvironment<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbEnvironment>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, penvironment: Param0, bforcewrite: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), penvironment.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SaveSession<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbSession>>(&self, psession: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psession.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetTargetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), targetname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetEnvironmentProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, environmentname: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), environmentname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, newstate: TARGET_STATE) -> ::windows::runtime::Result<TARGET_STATE> {
        let mut result__: <TARGET_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(newstate), &mut result__).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetSessionState<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbSession>>(&self, sbsession: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), sbsession.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateTargets<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, envname: Param1, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: Param3, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), farmname.into_param().abi(), envname.into_param().abi(), ::core::mem::transmute(sortbyfieldid), sortybypropname.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateSessions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        targetname: Param0,
        username: Param1,
        userdomain: Param2,
        poolname: Param3,
        initialprogram: Param4,
        psessionstate: *const TSSESSION_STATE,
        pdwcount: *mut u32,
        ppval: *mut *mut ::core::option::Option<ITsSbSession>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), targetname.into_param().abi(), username.into_param().abi(), userdomain.into_param().abi(), poolname.into_param().abi(), initialprogram.into_param().abi(), ::core::mem::transmute(psessionstate), ::core::mem::transmute(pdwcount), ::core::mem::transmute(ppval)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetFarmProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, farmname: Param0, propertyname: Param1, pvarvalue: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), farmname.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeleteTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, hostname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), targetname.into_param().abi(), hostname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetTargetPropertyWithVersionCheck<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTarget>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, ptarget: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ptarget.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetEnvironmentPropertyWithVersionCheck<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbEnvironment>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, penvironment: Param0, propertyname: Param1, pproperty: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), penvironment.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn AcquireTargetLock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0, dwtimeout: u32) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), targetname.into_param().abi(), ::core::mem::transmute(dwtimeout), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ReleaseTargetLock<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TestAndSetServerState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, serverfqdn: Param1, newstate: TARGET_STATE, teststate: TARGET_STATE) -> ::windows::runtime::Result<TARGET_STATE> {
        let mut result__: <TARGET_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), ::core::mem::transmute(newstate), ::core::mem::transmute(teststate), &mut result__).from_abi::<TARGET_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetServerWaitingToStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, servername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), poolname.into_param().abi(), servername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetServerState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, poolname: Param0, serverfqdn: Param1) -> ::windows::runtime::Result<TARGET_STATE> {
        let mut result__: <TARGET_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), &mut result__).from_abi::<TARGET_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetServerDrainMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serverfqdn: Param0, drainmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), serverfqdn.into_param().abi(), ::core::mem::transmute(drainmode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbResourcePluginStore {
    type Vtable = ITsSbResourcePluginStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1547236959, 48369, 16438, [166, 191, 158, 60, 204, 174, 11, 99]);
}
impl ::core::convert::From<ITsSbResourcePluginStore> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbResourcePluginStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbResourcePluginStore> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbResourcePluginStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbResourcePluginStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbResourcePluginStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePluginStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penvironment: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32, pval: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penvironment: ::windows::runtime::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sbsession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        psessionstate: *const TSSESSION_STATE,
        pdwcount: *mut u32,
        ppval: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penvironment: ::windows::runtime::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbServiceNotification(pub ::windows::runtime::IUnknown);
impl ITsSbServiceNotification {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyServiceFailure(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyServiceSuccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbServiceNotification {
    type Vtable = ITsSbServiceNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2261477550, 34528, 20311, [138, 100, 187, 116, 6, 188, 85, 80]);
}
impl ::core::convert::From<ITsSbServiceNotification> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbServiceNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbServiceNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbServiceNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbServiceNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbServiceNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbServiceNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbSession(pub ::windows::runtime::IUnknown);
impl ITsSbSession {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SessionId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targetname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), targetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Username(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn State(&self) -> ::windows::runtime::Result<TSSESSION_STATE> {
        let mut result__: <TSSESSION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TSSESSION_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetState(&self, state: TSSESSION_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(state)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateTime(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetCreateTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, time: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), time.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectTime(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetDisconnectTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, time: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), time.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn InitialProgram(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetInitialProgram<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, application: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ClientDisplay(&self) -> ::windows::runtime::Result<CLIENT_DISPLAY> {
        let mut result__: <CLIENT_DISPLAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CLIENT_DISPLAY>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetClientDisplay<'a, Param0: ::windows::runtime::IntoParam<'a, CLIENT_DISPLAY>>(&self, pclientdisplay: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pclientdisplay.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ProtocolType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetProtocolType(&self, val: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbSession {
    type Vtable = ITsSbSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3562252999, 45528, 19550, [186, 52, 154, 251, 76, 140, 85, 16]);
}
impl ::core::convert::From<ITsSbSession> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbSession> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, domain: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut TSSESSION_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: TSSESSION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, time: super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, time: super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, app: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientdisplay: CLIENT_DISPLAY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbTarget(pub ::windows::runtime::IUnknown);
impl ITsSbTarget {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn FarmName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetFarmName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetFQDN(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetFQDN<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetNetbios(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetNetbios<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(sockaddr), ::core::mem::transmute(numaddresses)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetIpAddresses(&self, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(sockaddr), ::core::mem::transmute(numaddresses)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn TargetState(&self) -> ::windows::runtime::Result<TARGET_STATE> {
        let mut result__: <TARGET_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TARGET_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetTargetState(&self, state: TARGET_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn TargetPropertySet(&self) -> ::windows::runtime::Result<ITsSbTargetPropertySet> {
        let mut result__: <ITsSbTargetPropertySet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITsSbTargetPropertySet>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetTargetPropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTargetPropertySet>>(&self, pval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnvironmentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetEnvironmentName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NumSessions(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NumPendingConnections(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn TargetLoad(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbTarget {
    type Vtable = ITsSbTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(375484108, 10029, 16669, [179, 36, 18, 104, 147, 3, 56, 86]);
}
impl ::core::convert::From<ITsSbTarget> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbTarget> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetfqdnname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetnetbiosname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut TARGET_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: TARGET_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertyset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumsessions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumpendingconnections: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetload: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbTargetPropertySet(pub ::windows::runtime::IUnknown);
impl ITsSbTargetPropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::Com::VARIANT, perrorlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbTargetPropertySet {
    type Vtable = ITsSbTargetPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4156401110, 39244, 19985, [160, 121, 39, 99, 182, 24, 48, 172]);
}
impl ::core::convert::From<ITsSbTargetPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbTargetPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbTargetPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbTargetPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITsSbTargetPropertySet> for ITsSbPropertySet {
    fn from(value: ITsSbTargetPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITsSbTargetPropertySet> for ITsSbPropertySet {
    fn from(value: &ITsSbTargetPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPropertySet> for &ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyBag> for &ITsSbTargetPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::StructuredStorage::IPropertyBag> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTargetPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropname: super::super::Foundation::PWSTR, pvar: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbTaskInfo(pub ::windows::runtime::IUnknown);
impl ITsSbTaskInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn TargetId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EndTime(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Identifier(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn Context(&self) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Plugin(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<RDV_TASK_STATUS> {
        let mut result__: <RDV_TASK_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RDV_TASK_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbTaskInfo {
    type Vtable = ITsSbTaskInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1379733635, 35262, 18653, [153, 234, 4, 232, 47, 250, 114, 101]);
}
impl ::core::convert::From<ITsSbTaskInfo> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbTaskInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbTaskInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbTaskInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbTaskInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbTaskInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentifier: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplugin: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut RDV_TASK_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbTaskPlugin(pub ::windows::runtime::IUnknown);
impl ITsSbTaskPlugin {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbProvider>, Param1: ::windows::runtime::IntoParam<'a, ITsSbPluginNotifySink>, Param2: ::windows::runtime::IntoParam<'a, ITsSbPluginPropertySet>>(&self, pprovider: Param0, pnotifysink: Param1, ppropertyset: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminate(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn InitializeTaskPlugin<'a, Param0: ::windows::runtime::IntoParam<'a, ITsSbTaskPluginNotifySink>>(&self, pitssbtaskpluginnotifysink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pitssbtaskpluginnotifysink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetTaskQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pszhostname: Param0, sbtaskinfosize: u32, pitssbtaskinfo: *const ::core::option::Option<ITsSbTaskInfo>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszhostname.into_param().abi(), ::core::mem::transmute(sbtaskinfosize), ::core::mem::transmute(pitssbtaskinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbTaskPlugin {
    type Vtable = ITsSbTaskPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4196593423, 34565, 16830, [147, 188, 68, 189, 188, 241, 201, 196]);
}
impl ::core::convert::From<ITsSbTaskPlugin> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbTaskPlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbTaskPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbTaskPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbPlugin> for &ITsSbTaskPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovider: ::windows::runtime::RawPtr, pnotifysink: ::windows::runtime::RawPtr, ppropertyset: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitssbtaskpluginnotifysink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITsSbTaskPluginNotifySink(pub ::windows::runtime::IUnknown);
impl ITsSbTaskPluginNotifySink {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype), ::core::mem::transmute(messageid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn OnSetTaskTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            sztargetname.into_param().abi(),
            taskstarttime.into_param().abi(),
            taskendtime.into_param().abi(),
            taskdeadline.into_param().abi(),
            sztasklabel.into_param().abi(),
            sztaskidentifier.into_param().abi(),
            sztaskplugin.into_param().abi(),
            ::core::mem::transmute(dwtaskstatus),
            ::core::mem::transmute(sacontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnDeleteTaskTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sztargetname: Param0, sztaskidentifier: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sztargetname.into_param().abi(), sztaskidentifier.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnUpdateTaskStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sztargetname: Param0, taskidentifier: Param1, taskstatus: RDV_TASK_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sztargetname.into_param().abi(), taskidentifier.into_param().abi(), ::core::mem::transmute(taskstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnReportTasks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, szhostname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), szhostname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITsSbTaskPluginNotifySink {
    type Vtable = ITsSbTaskPluginNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1789888926, 49900, 17902, [170, 55, 69, 230, 8, 149, 38, 26]);
}
impl ::core::convert::From<ITsSbTaskPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITsSbTaskPluginNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITsSbTaskPluginNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITsSbTaskPluginNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITsSbBaseNotifySink> for &ITsSbTaskPluginNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITsSbBaseNotifySink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPluginNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        taskstarttime: super::super::Foundation::FILETIME,
        taskendtime: super::super::Foundation::FILETIME,
        taskdeadline: super::super::Foundation::FILETIME,
        sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        dwtaskstatus: u32,
        sacontext: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsEnhancedFastReconnectArbitrator(pub ::windows::runtime::IUnknown);
impl IWRdsEnhancedFastReconnectArbitrator {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(dwsessioncount), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsEnhancedFastReconnectArbitrator {
    type Vtable = IWRdsEnhancedFastReconnectArbitrator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1461235355, 18418, 18847, [182, 52, 216, 23, 91, 213, 17, 49]);
}
impl ::core::convert::From<IWRdsEnhancedFastReconnectArbitrator> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsEnhancedFastReconnectArbitrator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsEnhancedFastReconnectArbitrator> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsEnhancedFastReconnectArbitrator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsEnhancedFastReconnectArbitrator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsEnhancedFastReconnectArbitrator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsEnhancedFastReconnectArbitrator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsGraphicsChannel(pub ::windows::runtime::IUnknown);
impl IWRdsGraphicsChannel {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Write<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, cbsize: u32, pbuffer: *const u8, pcontext: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, IWRdsGraphicsChannelEvents>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pchannelevents: Param0, popencontext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pchannelevents.into_param().abi(), popencontext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsGraphicsChannel {
    type Vtable = IWRdsGraphicsChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1749776907, 60927, 17325, [213, 162, 74, 141, 83, 136, 244, 1]);
}
impl ::core::convert::From<IWRdsGraphicsChannel> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsGraphicsChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannel> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsGraphicsChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsGraphicsChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsGraphicsChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbsize: u32, pbuffer: *const u8, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchannelevents: ::windows::runtime::RawPtr, popencontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsGraphicsChannelEvents(pub ::windows::runtime::IUnknown);
impl IWRdsGraphicsChannelEvents {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnClose(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnChannelOpened<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, openresult: ::windows::runtime::HRESULT, popencontext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(openresult), popencontext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnDataSent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwritecontext: Param0, bcancelled: Param1, pbuffer: *const u8, cbbuffer: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwritecontext.into_param().abi(), bcancelled.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(bandwidth), ::core::mem::transmute(rtt), ::core::mem::transmute(lastsentbyteindex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsGraphicsChannelEvents {
    type Vtable = IWRdsGraphicsChannelEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1743926924, 54900, 20398, [102, 165, 210, 6, 40, 166, 64, 210]);
}
impl ::core::convert::From<IWRdsGraphicsChannelEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsGraphicsChannelEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannelEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsGraphicsChannelEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsGraphicsChannelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsGraphicsChannelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbsize: u32, pbuffer: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, openresult: ::windows::runtime::HRESULT, popencontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwritecontext: ::windows::runtime::RawPtr, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsGraphicsChannelManager(pub ::windows::runtime::IUnknown);
impl IWRdsGraphicsChannelManager {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> ::windows::runtime::Result<IWRdsGraphicsChannel> {
        let mut result__: <IWRdsGraphicsChannel as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszchannelname), ::core::mem::transmute(channeltype), &mut result__).from_abi::<IWRdsGraphicsChannel>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsGraphicsChannelManager {
    type Vtable = IWRdsGraphicsChannelManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265646425, 59454, 18282, [168, 185, 74, 121, 118, 231, 30, 24]);
}
impl ::core::convert::From<IWRdsGraphicsChannelManager> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsGraphicsChannelManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsGraphicsChannelManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsGraphicsChannelManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsGraphicsChannelManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsGraphicsChannelManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolConnection(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolConnection {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows::runtime::Result<IWRdsProtocolLogonErrorRedirector> {
        let mut result__: <IWRdsProtocolLogonErrorRedirector as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWRdsProtocolLogonErrorRedirector>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AcceptConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetClientData(&self) -> ::windows::runtime::Result<WTS_CLIENT_DATA> {
        let mut result__: <WTS_CLIENT_DATA as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_CLIENT_DATA>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnummonitors), ::core::mem::transmute(pprimarymonitor)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetUserCredentials(&self) -> ::windows::runtime::Result<WTS_USER_CREDENTIAL> {
        let mut result__: <WTS_USER_CREDENTIAL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_USER_CREDENTIAL>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLicenseConnection(&self) -> ::windows::runtime::Result<IWRdsProtocolLicenseConnection> {
        let mut result__: <IWRdsProtocolLicenseConnection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWRdsProtocolLicenseConnection>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AuthenticateClientToSession(&self) -> ::windows::runtime::Result<WTS_SESSION_ID> {
        let mut result__: <WTS_SESSION_ID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_SESSION_ID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn NotifySessionId<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), sessionhandle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkeyboardhandle), ::core::mem::transmute(pmousehandle), ::core::mem::transmute(pbeephandle)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetVideoHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__: <super::super::Foundation::HANDLE_PTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE_PTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsUserAllowedToLogon<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: u32, usertoken: Param1, pdomainname: Param2, pusername: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SessionArbitrationEnumeration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, husertoken: Param0, bsinglesessionperuserenabled: Param1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(pdwsessionidentifiercount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn LogonNotify<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hclienttoken: Param0, wszusername: Param1, wszdomainname: Param2, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(pwrdsconnectionsettings)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn PreDisconnect(&self, disconnectreason: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(disconnectreason)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn DisconnectNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetProtocolStatus(&self) -> ::windows::runtime::Result<WTS_PROTOCOL_STATUS> {
        let mut result__: <WTS_PROTOCOL_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_PROTOCOL_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLastInputTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulerror)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateVirtualChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, szendpointname: Param0, bstatic: Param1, requestedpriority: u32) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), ::core::mem::transmute(requestedpriority), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QueryProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, querytype: Param0, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), querytype.into_param().abi(), ::core::mem::transmute(ulnumentriesin), ::core::mem::transmute(ulnumentriesout), ::core::mem::transmute(ppropertyentriesin), ::core::mem::transmute(ppropertyentriesout)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetShadowConnection(&self) -> ::windows::runtime::Result<IWRdsProtocolShadowConnection> {
        let mut result__: <IWRdsProtocolShadowConnection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWRdsProtocolShadowConnection>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyCommandProcessCreated(&self, sessionid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolConnection {
    type Vtable = IWRdsProtocolConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844028239, 64943, 20470, [129, 168, 66, 171, 231, 85, 131, 11]);
}
impl ::core::convert::From<IWRdsProtocolConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplogonerrorredir: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplicenseconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *mut WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disconnectreason: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plastinputtime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulerror: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querytype: ::windows::runtime::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppshadowconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolConnectionCallback(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolConnectionCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReady(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reason), ::core::mem::transmute(source)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopScreenUpdates(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetConnectionId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolConnectionCallback {
    type Vtable = IWRdsProtocolConnectionCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4057400114, 53360, 20209, [160, 136, 120, 49, 53, 54, 194, 214]);
}
impl ::core::convert::From<IWRdsProtocolConnectionCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolConnectionCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolConnectionCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolConnectionCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolConnectionCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolConnectionCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: u32, source: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: *const WTS_SMALL_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectionid: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolConnectionSettings(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolConnectionSettings {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetConnectionSetting<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, propertyid: Param0, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), propertyid.into_param().abi(), ::core::mem::transmute(ppropertyentriesin)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionSetting<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, propertyid: Param0) -> ::windows::runtime::Result<WTS_PROPERTY_VALUE> {
        let mut result__: <WTS_PROPERTY_VALUE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), propertyid.into_param().abi(), &mut result__).from_abi::<WTS_PROPERTY_VALUE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolConnectionSettings {
    type Vtable = IWRdsProtocolConnectionSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2214393299, 63220, 60052, [156, 210, 50, 242, 128, 225, 229, 16]);
}
impl ::core::convert::From<IWRdsProtocolConnectionSettings> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolConnectionSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolConnectionSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolConnectionSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolConnectionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolConnectionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyid: ::windows::runtime::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyid: ::windows::runtime::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolLicenseConnection(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolLicenseConnection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplicensecapabilities), ::core::mem::transmute(pcblicensecapabilities)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientlicense), ::core::mem::transmute(cbclientlicense)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserve1), ::core::mem::transmute(reserve2), ::core::mem::transmute(ppclientlicense), ::core::mem::transmute(pcbclientlicense)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcomplete)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolLicenseConnection {
    type Vtable = IWRdsProtocolLicenseConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(493491295, 53397, 17444, [149, 122, 64, 127, 174, 130, 45, 132]);
}
impl ::core::convert::From<IWRdsProtocolLicenseConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolLicenseConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolLicenseConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolLicenseConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolLicenseConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolLicenseConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLicenseConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcomplete: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolListener(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolListener {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> ::windows::runtime::Result<WRDS_LISTENER_SETTINGS> {
        let mut result__: <WRDS_LISTENER_SETTINGS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrdslistenersettinglevel), &mut result__).from_abi::<WRDS_LISTENER_SETTINGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StartListen<'a, Param0: ::windows::runtime::IntoParam<'a, IWRdsProtocolListenerCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopListen(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolListener {
    type Vtable = IWRdsProtocolListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4240184091, 50822, 17693, [167, 115, 226, 121, 226, 48, 245, 64]);
}
impl ::core::convert::From<IWRdsProtocolListener> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolListener> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolListenerCallback(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolListenerCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnConnected<'a, Param0: ::windows::runtime::IntoParam<'a, IWRdsProtocolConnection>>(&self, pconnection: Param0, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> ::windows::runtime::Result<IWRdsProtocolConnectionCallback> {
        let mut result__: <IWRdsProtocolConnectionCallback as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), ::core::mem::transmute(pwrdsconnectionsettings), &mut result__).from_abi::<IWRdsProtocolConnectionCallback>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolListenerCallback {
    type Vtable = IWRdsProtocolListenerCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(984776283, 17481, 19905, [183, 74, 145, 98, 29, 79, 233, 132]);
}
impl ::core::convert::From<IWRdsProtocolListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolListenerCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolListenerCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListenerCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolLogonErrorRedirector(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolLogonErrorRedirector {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnBeginPainting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszmessage: Param0) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszmessage.into_param().abi(), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcaption: Param0, pszmessage: Param1, utype: u32) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectLogonError<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: Param2, pszmessage: Param3, utype: u32) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntsstatus), ::core::mem::transmute(ntssubstatus), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolLogonErrorRedirector {
    type Vtable = IWRdsProtocolLogonErrorRedirector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1369434171, 5162, 16672, [163, 213, 164, 5, 211, 21, 40, 26]);
}
impl ::core::convert::From<IWRdsProtocolLogonErrorRedirector> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolLogonErrorRedirector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolLogonErrorRedirector> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolLogonErrorRedirector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLogonErrorRedirector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolManager(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IWRdsProtocolSettings>>(&self, piwrdssettings: Param0, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piwrdssettings.into_param().abi(), ::core::mem::transmute(pwrdssettings)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateListener<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszlistenername: Param0) -> ::windows::runtime::Result<IWRdsProtocolListener> {
        let mut result__: <IWRdsProtocolListener as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszlistenername.into_param().abi(), &mut result__).from_abi::<IWRdsProtocolListener>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptsservicestatechange)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), ::core::mem::transmute(eventid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwrdssettings)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Uninitialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolManager {
    type Vtable = IWRdsProtocolManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3698944359, 15035, 16589, [164, 70, 16, 82, 118, 181, 137, 80]);
}
impl ::core::convert::From<IWRdsProtocolManager> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piwrdssettings: ::windows::runtime::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolSettings(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolSettings {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL) -> ::windows::runtime::Result<WRDS_SETTINGS> {
        let mut result__: <WRDS_SETTINGS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrdssettingtype), ::core::mem::transmute(wrdssettinglevel), &mut result__).from_abi::<WRDS_SETTINGS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwrdssettings), ::core::mem::transmute(wrdsconnectionsettinglevel), ::core::mem::transmute(pwrdsconnectionsettings)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolSettings {
    type Vtable = IWRdsProtocolSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1699371626, 9552, 18411, [182, 247, 235, 214, 55, 71, 82, 101]);
}
impl ::core::convert::From<IWRdsProtocolSettings> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolShadowCallback(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolShadowCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopShadow(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn InvokeTargetShadow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptargetservername: Param0, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: Param10) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ptargetservername.into_param().abi(),
            ::core::mem::transmute(targetsessionid),
            ::core::mem::transmute(pparam1),
            ::core::mem::transmute(param1size),
            ::core::mem::transmute(pparam2),
            ::core::mem::transmute(param2size),
            ::core::mem::transmute(pparam3),
            ::core::mem::transmute(param3size),
            ::core::mem::transmute(pparam4),
            ::core::mem::transmute(param4size),
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolShadowCallback {
    type Vtable = IWRdsProtocolShadowCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3764813024, 882, 16598, [173, 178, 160, 243, 50, 38, 116, 214]);
}
impl ::core::convert::From<IWRdsProtocolShadowCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolShadowCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolShadowCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolShadowCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolShadowCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolShadowCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsProtocolShadowConnection(pub ::windows::runtime::IUnknown);
impl IWRdsProtocolShadowConnection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Start<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IWRdsProtocolShadowCallback>>(&self, ptargetservername: Param0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptargetservername.into_param().abi(), ::core::mem::transmute(targetsessionid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers), pshadowcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DoTarget<'a, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: Param8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pparam1),
            ::core::mem::transmute(param1size),
            ::core::mem::transmute(pparam2),
            ::core::mem::transmute(param2size),
            ::core::mem::transmute(pparam3),
            ::core::mem::transmute(param3size),
            ::core::mem::transmute(pparam4),
            ::core::mem::transmute(param4size),
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsProtocolShadowConnection {
    type Vtable = IWRdsProtocolShadowConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2598919398, 51934, 17736, [143, 235, 153, 1, 101, 151, 246, 10]);
}
impl ::core::convert::From<IWRdsProtocolShadowConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsProtocolShadowConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsProtocolShadowConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsProtocolShadowConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsProtocolShadowConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsProtocolShadowConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWRdsWddmIddProps(pub ::windows::runtime::IUnknown);
impl IWRdsWddmIddProps {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetHardwareId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pdisplaydriverhardwareid: Param0, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdisplaydriverhardwareid.into_param().abi(), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnDriverLoad<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, sessionid: u32, driverhandle: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), driverhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn EnableWddmIdd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), enabled.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWRdsWddmIddProps {
    type Vtable = IWRdsWddmIddProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(327343949, 41609, 17361, [161, 132, 20, 71, 38, 249, 175, 144]);
}
impl ::core::convert::From<IWRdsWddmIddProps> for ::windows::runtime::IUnknown {
    fn from(value: IWRdsWddmIddProps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWRdsWddmIddProps> for ::windows::runtime::IUnknown {
    fn from(value: &IWRdsWddmIddProps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWRdsWddmIddProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWRdsWddmIddProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsWddmIddProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaydriverhardwareid: super::super::Foundation::PWSTR, count: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSBitmapRenderService(pub ::windows::runtime::IUnknown);
impl IWTSBitmapRenderService {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetMappedRenderer<'a, Param1: ::windows::runtime::IntoParam<'a, IWTSBitmapRendererCallback>>(&self, mappingid: u64, pmappedrenderercallback: Param1) -> ::windows::runtime::Result<IWTSBitmapRenderer> {
        let mut result__: <IWTSBitmapRenderer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(mappingid), pmappedrenderercallback.into_param().abi(), &mut result__).from_abi::<IWTSBitmapRenderer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSBitmapRenderService {
    type Vtable = IWTSBitmapRenderService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929170065, 1534, 16577, [180, 156, 61, 46, 244, 98, 106, 14]);
}
impl ::core::convert::From<IWTSBitmapRenderService> for ::windows::runtime::IUnknown {
    fn from(value: IWTSBitmapRenderService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSBitmapRenderService> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSBitmapRenderService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSBitmapRenderService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSBitmapRenderService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mappingid: u64, pmappedrenderercallback: ::windows::runtime::RawPtr, ppmappedrenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSBitmapRenderer(pub ::windows::runtime::IUnknown);
impl IWTSBitmapRenderer {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Render<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, imageformat: Param0, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), imageformat.into_param().abi(), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbimagebuffer), ::core::mem::transmute(pimagebuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetRendererStatistics(&self) -> ::windows::runtime::Result<BITMAP_RENDERER_STATISTICS> {
        let mut result__: <BITMAP_RENDERER_STATISTICS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BITMAP_RENDERER_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RemoveMapping(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSBitmapRenderer {
    type Vtable = IWTSBitmapRenderer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1534774423, 62409, 18167, [140, 91, 250, 104, 93, 52, 65, 177]);
}
impl ::core::convert::From<IWTSBitmapRenderer> for ::windows::runtime::IUnknown {
    fn from(value: IWTSBitmapRenderer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSBitmapRenderer> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSBitmapRenderer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSBitmapRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSBitmapRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageformat: ::windows::runtime::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSBitmapRendererCallback(pub ::windows::runtime::IUnknown);
impl IWTSBitmapRendererCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnTargetSizeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, rcnewsize: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), rcnewsize.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSBitmapRendererCallback {
    type Vtable = IWTSBitmapRendererCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3615658638, 65102, 20087, [174, 144, 156, 208, 179, 227, 179, 83]);
}
impl ::core::convert::From<IWTSBitmapRendererCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSBitmapRendererCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSBitmapRendererCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSBitmapRendererCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSBitmapRendererCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSBitmapRendererCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRendererCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rcnewsize: super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSListener(pub ::windows::runtime::IUnknown);
impl IWTSListener {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetConfiguration(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::IPropertyBag> {
        let mut result__: <super::Com::StructuredStorage::IPropertyBag as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::IPropertyBag>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSListener {
    type Vtable = IWTSListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426054, 39481, 19800, [134, 116, 205, 180, 223, 244, 231, 59]);
}
impl ::core::convert::From<IWTSListener> for ::windows::runtime::IUnknown {
    fn from(value: IWTSListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSListener> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertybag: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSListenerCallback(pub ::windows::runtime::IUnknown);
impl IWTSListenerCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnNewChannelConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IWTSVirtualChannel>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pchannel: Param0, data: Param1, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::core::option::Option<IWTSVirtualChannelCallback>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pchannel.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(pbaccept), ::core::mem::transmute(ppcallback)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSListenerCallback {
    type Vtable = IWTSListenerCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426051, 54951, 4568, [185, 253, 0, 11, 219, 209, 241, 152]);
}
impl ::core::convert::From<IWTSListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSListenerCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSListenerCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListenerCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchannel: ::windows::runtime::RawPtr, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSPlugin(pub ::windows::runtime::IUnknown);
impl IWTSPlugin {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IWTSVirtualChannelManager>>(&self, pchannelmgr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pchannelmgr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Connected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdisconnectcode)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminated(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSPlugin {
    type Vtable = IWTSPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426049, 5177, 20066, [164, 20, 25, 13, 10, 195, 212, 14]);
}
impl ::core::convert::From<IWTSPlugin> for ::windows::runtime::IUnknown {
    fn from(value: IWTSPlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchannelmgr: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdisconnectcode: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSPluginServiceProvider(pub ::windows::runtime::IUnknown);
impl IWTSPluginServiceProvider {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetService<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, serviceid: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), serviceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSPluginServiceProvider {
    type Vtable = IWTSPluginServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3554702179, 2172, 18284, [134, 167, 219, 177, 95, 70, 221, 180]);
}
impl ::core::convert::From<IWTSPluginServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IWTSPluginServiceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSPluginServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSPluginServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSPluginServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSPluginServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPluginServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::GUID, ppunkobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolConnection(pub ::windows::runtime::IUnknown);
impl IWTSProtocolConnection {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows::runtime::Result<IWTSProtocolLogonErrorRedirector> {
        let mut result__: <IWTSProtocolLogonErrorRedirector as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWTSProtocolLogonErrorRedirector>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppolicydata)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AcceptConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetClientData(&self) -> ::windows::runtime::Result<WTS_CLIENT_DATA> {
        let mut result__: <WTS_CLIENT_DATA as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_CLIENT_DATA>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetUserCredentials(&self) -> ::windows::runtime::Result<WTS_USER_CREDENTIAL> {
        let mut result__: <WTS_USER_CREDENTIAL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_USER_CREDENTIAL>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLicenseConnection(&self) -> ::windows::runtime::Result<IWTSProtocolLicenseConnection> {
        let mut result__: <IWTSProtocolLicenseConnection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWTSProtocolLicenseConnection>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AuthenticateClientToSession(&self) -> ::windows::runtime::Result<WTS_SESSION_ID> {
        let mut result__: <WTS_SESSION_ID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_SESSION_ID>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkeyboardhandle), ::core::mem::transmute(pmousehandle), ::core::mem::transmute(pbeephandle), ::core::mem::transmute(pvideohandle)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsUserAllowedToLogon<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: u32, usertoken: Param1, pdomainname: Param2, pusername: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SessionArbitrationEnumeration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, husertoken: Param0, bsinglesessionperuserenabled: Param1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), ::core::mem::transmute(psessionidarray), ::core::mem::transmute(pdwsessionidentifiercount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn LogonNotify<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hclienttoken: Param0, wszusername: Param1, wszdomainname: Param2, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), ::core::mem::transmute(sessionid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppolicydata), ::core::mem::transmute(pclientdata)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn DisconnectNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetProtocolStatus(&self) -> ::windows::runtime::Result<WTS_PROTOCOL_STATUS> {
        let mut result__: <WTS_PROTOCOL_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WTS_PROTOCOL_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetLastInputTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulerror)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SendBeep(&self, frequency: u32, duration: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(frequency), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateVirtualChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, szendpointname: Param0, bstatic: Param1, requestedpriority: u32) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), ::core::mem::transmute(requestedpriority), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn QueryProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, querytype: Param0, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), querytype.into_param().abi(), ::core::mem::transmute(ulnumentriesin), ::core::mem::transmute(ulnumentriesout), ::core::mem::transmute(ppropertyentriesin), ::core::mem::transmute(ppropertyentriesout)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetShadowConnection(&self) -> ::windows::runtime::Result<IWTSProtocolShadowConnection> {
        let mut result__: <IWTSProtocolShadowConnection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWTSProtocolShadowConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolConnection {
    type Vtable = IWTSProtocolConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587741029, 37013, 17992, [152, 191, 239, 129, 201, 20, 3, 45]);
}
impl ::core::convert::From<IWTSProtocolConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplogonerrorredir: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplicenseconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *mut WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plastinputtime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulerror: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frequency: u32, duration: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querytype: ::windows::runtime::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppshadowconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolConnectionCallback(pub ::windows::runtime::IUnknown);
impl IWTSProtocolConnectionCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnReady(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reason), ::core::mem::transmute(source)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopScreenUpdates(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(displayioctl)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolConnectionCallback {
    type Vtable = IWTSProtocolConnectionCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587741029, 30187, 16894, [180, 251, 224, 134, 36, 42, 250, 15]);
}
impl ::core::convert::From<IWTSProtocolConnectionCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolConnectionCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolConnectionCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolConnectionCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolConnectionCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolConnectionCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnectionCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: u32, source: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: *const WTS_SMALL_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolLicenseConnection(pub ::windows::runtime::IUnknown);
impl IWTSProtocolLicenseConnection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplicensecapabilities), ::core::mem::transmute(pcblicensecapabilities)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientlicense), ::core::mem::transmute(cbclientlicense)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserve1), ::core::mem::transmute(reserve2), ::core::mem::transmute(ppclientlicense), ::core::mem::transmute(pcbclientlicense)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcomplete)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolLicenseConnection {
    type Vtable = IWTSProtocolLicenseConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587741029, 6028, 16505, [142, 74, 254, 166, 73, 106, 77, 112]);
}
impl ::core::convert::From<IWTSProtocolLicenseConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolLicenseConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolLicenseConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolLicenseConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolLicenseConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolLicenseConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLicenseConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcomplete: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolListener(pub ::windows::runtime::IUnknown);
impl IWTSProtocolListener {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StartListen<'a, Param0: ::windows::runtime::IntoParam<'a, IWTSProtocolListenerCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopListen(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolListener {
    type Vtable = IWTSProtocolListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587741029, 17904, 17300, [143, 105, 50, 178, 188, 14, 244, 202]);
}
impl ::core::convert::From<IWTSProtocolListener> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolListener> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolListenerCallback(pub ::windows::runtime::IUnknown);
impl IWTSProtocolListenerCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnConnected<'a, Param0: ::windows::runtime::IntoParam<'a, IWTSProtocolConnection>>(&self, pconnection: Param0) -> ::windows::runtime::Result<IWTSProtocolConnectionCallback> {
        let mut result__: <IWTSProtocolConnectionCallback as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pconnection.into_param().abi(), &mut result__).from_abi::<IWTSProtocolConnectionCallback>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolListenerCallback {
    type Vtable = IWTSProtocolListenerCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587741029, 6701, 19938, [151, 222, 74, 53, 242, 96, 240, 179]);
}
impl ::core::convert::From<IWTSProtocolListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolListenerCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolListenerCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolListenerCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolListenerCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListenerCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr, pcallback: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolLogonErrorRedirector(pub ::windows::runtime::IUnknown);
impl IWTSProtocolLogonErrorRedirector {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnBeginPainting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszmessage: Param0) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszmessage.into_param().abi(), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcaption: Param0, pszmessage: Param1, utype: u32) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RedirectLogonError<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: Param2, pszmessage: Param3, utype: u32) -> ::windows::runtime::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE> {
        let mut result__: <WTS_LOGON_ERROR_REDIRECTOR_RESPONSE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntsstatus), ::core::mem::transmute(ntssubstatus), pszcaption.into_param().abi(), pszmessage.into_param().abi(), ::core::mem::transmute(utype), &mut result__).from_abi::<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolLogonErrorRedirector {
    type Vtable = IWTSProtocolLogonErrorRedirector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4254818727, 10518, 17959, [141, 238, 67, 40, 113, 26, 214, 203]);
}
impl ::core::convert::From<IWTSProtocolLogonErrorRedirector> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolLogonErrorRedirector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolLogonErrorRedirector> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolLogonErrorRedirector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolLogonErrorRedirector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLogonErrorRedirector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolManager(pub ::windows::runtime::IUnknown);
impl IWTSProtocolManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateListener<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszlistenername: Param0) -> ::windows::runtime::Result<IWTSProtocolListener> {
        let mut result__: <IWTSProtocolListener as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszlistenername.into_param().abi(), &mut result__).from_abi::<IWTSProtocolListener>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptsservicestatechange)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessionid), ::core::mem::transmute(eventid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolManager {
    type Vtable = IWTSProtocolManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4192925388, 60793, 20225, [130, 29, 31, 136, 27, 159, 102, 204]);
}
impl ::core::convert::From<IWTSProtocolManager> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolShadowCallback(pub ::windows::runtime::IUnknown);
impl IWTSProtocolShadowCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn StopShadow(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn InvokeTargetShadow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptargetservername: Param0, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: Param10) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ptargetservername.into_param().abi(),
            ::core::mem::transmute(targetsessionid),
            ::core::mem::transmute(pparam1),
            ::core::mem::transmute(param1size),
            ::core::mem::transmute(pparam2),
            ::core::mem::transmute(param2size),
            ::core::mem::transmute(pparam3),
            ::core::mem::transmute(param3size),
            ::core::mem::transmute(pparam4),
            ::core::mem::transmute(param4size),
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolShadowCallback {
    type Vtable = IWTSProtocolShadowCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1345987844, 43749, 19121, [147, 224, 109, 28, 75, 198, 247, 26]);
}
impl ::core::convert::From<IWTSProtocolShadowCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolShadowCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolShadowCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolShadowCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolShadowCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolShadowCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSProtocolShadowConnection(pub ::windows::runtime::IUnknown);
impl IWTSProtocolShadowConnection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn Start<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IWTSProtocolShadowCallback>>(&self, ptargetservername: Param0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptargetservername.into_param().abi(), ::core::mem::transmute(targetsessionid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers), pshadowcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DoTarget<'a, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: Param8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pparam1),
            ::core::mem::transmute(param1size),
            ::core::mem::transmute(pparam2),
            ::core::mem::transmute(param2size),
            ::core::mem::transmute(pparam3),
            ::core::mem::transmute(param3size),
            ::core::mem::transmute(pparam4),
            ::core::mem::transmute(param4size),
            pclientname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSProtocolShadowConnection {
    type Vtable = IWTSProtocolShadowConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3996847124, 14331, 17771, [186, 179, 109, 108, 213, 30, 19, 191]);
}
impl ::core::convert::From<IWTSProtocolShadowConnection> for ::windows::runtime::IUnknown {
    fn from(value: IWTSProtocolShadowConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSProtocolShadowConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSProtocolShadowConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSProtocolShadowConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSProtocolShadowConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSSBPlugin(pub ::windows::runtime::IUnknown);
impl IWTSSBPlugin {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Initialize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), ::core::mem::transmute(machineid), ::core::mem::transmute(pmachineinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype), ::core::mem::transmute(machineid), ::core::mem::transmute(numofsessions), ::core::mem::transmute(sessioninfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn WTSSBX_GetMostSuitableServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, username: Param0, domainname: Param1, applicationtype: Param2, farmname: Param3, pmachineid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), farmname.into_param().abi(), ::core::mem::transmute(pmachineid)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Terminated(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn WTSSBX_GetUserExternalSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        username: Param0,
        domainname: Param1,
        applicationtype: Param2,
        redirectorinternalip: *const WTSSBX_IP_ADDRESS,
        psessionid: *mut u32,
        pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), ::core::mem::transmute(redirectorinternalip), ::core::mem::transmute(psessionid), ::core::mem::transmute(pmachineconnectinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSSBPlugin {
    type Vtable = IWTSSBPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3695492728, 45453, 17305, [178, 16, 100, 27, 246, 122, 0, 44]);
}
impl ::core::convert::From<IWTSSBPlugin> for ::windows::runtime::IUnknown {
    fn from(value: IWTSSBPlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSSBPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSSBPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSSBPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSSBPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSSBPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plugincapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, farmname: super::super::Foundation::PWSTR, pmachineid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSVirtualChannel(pub ::windows::runtime::IUnknown);
impl IWTSVirtualChannel {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Write<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, cbsize: u32, pbuffer: *const u8, preserved: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer), preserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSVirtualChannel {
    type Vtable = IWTSVirtualChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426055, 54951, 4568, [185, 253, 0, 11, 219, 209, 241, 152]);
}
impl ::core::convert::From<IWTSVirtualChannel> for ::windows::runtime::IUnknown {
    fn from(value: IWTSVirtualChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSVirtualChannel> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSVirtualChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSVirtualChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSVirtualChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbsize: u32, pbuffer: *const u8, preserved: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSVirtualChannelCallback(pub ::windows::runtime::IUnknown);
impl IWTSVirtualChannelCallback {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(pbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn OnClose(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWTSVirtualChannelCallback {
    type Vtable = IWTSVirtualChannelCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426052, 54951, 4568, [185, 253, 0, 11, 219, 209, 241, 152]);
}
impl ::core::convert::From<IWTSVirtualChannelCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWTSVirtualChannelCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSVirtualChannelCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSVirtualChannelCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSVirtualChannelCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSVirtualChannelCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbsize: u32, pbuffer: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWTSVirtualChannelManager(pub ::windows::runtime::IUnknown);
impl IWTSVirtualChannelManager {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn CreateListener<'a, Param2: ::windows::runtime::IntoParam<'a, IWTSListenerCallback>>(&self, pszchannelname: *const u8, uflags: u32, plistenercallback: Param2) -> ::windows::runtime::Result<IWTSListener> {
        let mut result__: <IWTSListener as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszchannelname), ::core::mem::transmute(uflags), plistenercallback.into_param().abi(), &mut result__).from_abi::<IWTSListener>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWTSVirtualChannelManager {
    type Vtable = IWTSVirtualChannelManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703426053, 54951, 4568, [185, 253, 0, 11, 219, 209, 241, 152]);
}
impl ::core::convert::From<IWTSVirtualChannelManager> for ::windows::runtime::IUnknown {
    fn from(value: IWTSVirtualChannelManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWTSVirtualChannelManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWTSVirtualChannelManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWTSVirtualChannelManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWTSVirtualChannelManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszchannelname: *const u8, uflags: u32, plistenercallback: ::windows::runtime::RawPtr, pplistener: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspace(pub ::windows::runtime::IUnknown);
impl IWorkspace {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspace {
    type Vtable = IWorkspace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3106061240, 19541, 20458, [132, 150, 190, 176, 180, 66, 133, 229]);
}
impl ::core::convert::From<IWorkspace> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspace> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprocessid: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspace2(pub ::windows::runtime::IUnknown);
impl IWorkspace2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn StartRemoteApplicationEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrworkspaceid: Param0,
        bstrrequestingappid: Param1,
        bstrrequestingappfamilyname: Param2,
        blaunchintoimmersiveclient: i16,
        bstrimmersiveclientactivationcontext: Param4,
        psaparams: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            bstrworkspaceid.into_param().abi(),
            bstrrequestingappid.into_param().abi(),
            bstrrequestingappfamilyname.into_param().abi(),
            ::core::mem::transmute(blaunchintoimmersiveclient),
            bstrimmersiveclientactivationcontext.into_param().abi(),
            ::core::mem::transmute(psaparams),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspace2 {
    type Vtable = IWorkspace2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2530793423, 30782, 17030, [131, 76, 235, 192, 233, 95, 120, 60]);
}
impl ::core::convert::From<IWorkspace2> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspace2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspace2> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspace2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace> for IWorkspace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace> for &IWorkspace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprocessid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        blaunchintoimmersiveclient: i16,
        bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        psaparams: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspace3(pub ::windows::runtime::IUnknown);
impl IWorkspace3 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn StartRemoteApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(psaparams)).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn StartRemoteApplicationEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrworkspaceid: Param0,
        bstrrequestingappid: Param1,
        bstrrequestingappfamilyname: Param2,
        blaunchintoimmersiveclient: i16,
        bstrimmersiveclientactivationcontext: Param4,
        psaparams: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            bstrworkspaceid.into_param().abi(),
            bstrrequestingappid.into_param().abi(),
            bstrrequestingappfamilyname.into_param().abi(),
            ::core::mem::transmute(blaunchintoimmersiveclient),
            bstrimmersiveclientactivationcontext.into_param().abi(),
            ::core::mem::transmute(psaparams),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetClaimsToken2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, bstrclaimshint: Param0, bstruserhint: Param1, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: Param4) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrclaimshint.into_param().abi(), bstruserhint.into_param().abi(), ::core::mem::transmute(claimcookie), ::core::mem::transmute(hwndcreduiparent), rectcreduiparent.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn SetClaimsToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccesstoken: Param0, ullaccesstokenexpiration: u64, bstrrefreshtoken: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstraccesstoken.into_param().abi(), ::core::mem::transmute(ullaccesstokenexpiration), bstrrefreshtoken.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspace3 {
    type Vtable = IWorkspace3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(468500042, 54868, 16955, [175, 235, 190, 141, 83, 44, 19, 198]);
}
impl ::core::convert::From<IWorkspace3> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspace3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspace3> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspace3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace2> for IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace2> for &IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace> for IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspace> for &IWorkspace3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspace> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprocessid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        blaunchintoimmersiveclient: i16,
        bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        psaparams: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceClientExt(pub ::windows::runtime::IUnknown);
impl IWorkspaceClientExt {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn IssueDisconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceClientExt {
    type Vtable = IWorkspaceClientExt_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(314135284, 16842, 20257, [168, 41, 166, 208, 125, 154, 22, 229]);
}
impl ::core::convert::From<IWorkspaceClientExt> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceClientExt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceClientExt> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceClientExt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceClientExt {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceClientExt {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceClientExt_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspacedisplayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceRegistration(pub ::windows::runtime::IUnknown);
impl IWorkspaceRegistration {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::runtime::IntoParam<'a, IWorkspaceClientExt>>(&self, punk: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceRegistration {
    type Vtable = IWorkspaceRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3106061240, 19541, 20458, [132, 150, 190, 176, 180, 66, 133, 230]);
}
impl ::core::convert::From<IWorkspaceRegistration> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookieconnection: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceRegistration2(pub ::windows::runtime::IUnknown);
impl IWorkspaceRegistration2 {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::runtime::IntoParam<'a, IWorkspaceClientExt>>(&self, punk: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn AddResourceEx<'a, Param0: ::windows::runtime::IntoParam<'a, IWorkspaceClientExt>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, punk: Param0, bstreventloguploadaddress: Param1, pdwcookie: *mut u32, correlationid: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi(), bstreventloguploadaddress.into_param().abi(), ::core::mem::transmute(pdwcookie), correlationid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn RemoveResourceEx<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, dwcookieconnection: u32, correlationid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookieconnection), correlationid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceRegistration2 {
    type Vtable = IWorkspaceRegistration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3478779476, 14779, 17624, [148, 208, 70, 53, 114, 137, 87, 233]);
}
impl ::core::convert::From<IWorkspaceRegistration2> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceRegistration2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceRegistration2> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceRegistration2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceRegistration> for IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceRegistration> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceRegistration> for &IWorkspaceRegistration2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceRegistration> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookieconnection: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookieconnection: u32, correlationid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceReportMessage(pub ::windows::runtime::IUnknown);
impl IWorkspaceReportMessage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RegisterErrorLogMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessage: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrmessage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsErrorMessageRegistered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrwkspid: Param0, dwerrortype: u32, bstrerrormessagetype: Param2, dwerrorcode: u32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrwkspid.into_param().abi(), ::core::mem::transmute(dwerrortype), bstrerrormessagetype.into_param().abi(), ::core::mem::transmute(dwerrorcode), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn RegisterErrorEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrwkspid: Param0, dwerrortype: u32, bstrerrormessagetype: Param2, dwerrorcode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrwkspid.into_param().abi(), ::core::mem::transmute(dwerrortype), bstrerrormessagetype.into_param().abi(), ::core::mem::transmute(dwerrorcode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceReportMessage {
    type Vtable = IWorkspaceReportMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2814404409, 20495, 20108, [153, 168, 43, 214, 149, 88, 153, 235]);
}
impl ::core::convert::From<IWorkspaceReportMessage> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceReportMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceReportMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceReportMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceReportMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceReportMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceReportMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceResTypeRegistry(pub ::windows::runtime::IUnknown);
impl IWorkspaceResTypeRegistry {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn AddResourceType<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1, bstrlauncher: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeleteResourceType<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetRegisteredFileExtensions(&self, fmachinewide: i16) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceTypeInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ModifyResourceType<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fmachinewide: i16, bstrfileextension: Param1, bstrlauncher: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmachinewide), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceResTypeRegistry {
    type Vtable = IWorkspaceResTypeRegistry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(490900601, 28206, 17233, [163, 97, 192, 64, 26, 3, 160, 186]);
}
impl ::core::convert::From<IWorkspaceResTypeRegistry> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceResTypeRegistry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceResTypeRegistry> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceResTypeRegistry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IWorkspaceResTypeRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceResTypeRegistry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceScriptable(pub ::windows::runtime::IUnknown);
impl IWorkspaceScriptable {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrworkspaceid: Param0,
        bstrusername: Param1,
        bstrpassword: Param2,
        bstrworkspaceparams: Param3,
        ltimeout: i32,
        lflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceScriptable {
    type Vtable = IWorkspaceScriptable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4025108898, 56741, 17053, [143, 66, 178, 59, 146, 196, 195, 71]);
}
impl ::core::convert::From<IWorkspaceScriptable> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceScriptable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceScriptable> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceScriptable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceScriptable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceScriptable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IWorkspaceScriptable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbssoenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceScriptable2(pub ::windows::runtime::IUnknown);
impl IWorkspaceScriptable2 {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrworkspaceid: Param0,
        bstrusername: Param1,
        bstrpassword: Param2,
        bstrworkspaceparams: Param3,
        ltimeout: i32,
        lflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspaceEx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::core::mem::transmute_copy(self),
            bstrworkspaceid.into_param().abi(),
            bstrworkspacefriendlyname.into_param().abi(),
            bstrredirectorname.into_param().abi(),
            bstrusername.into_param().abi(),
            bstrpassword.into_param().abi(),
            bstrappcontainer.into_param().abi(),
            bstrworkspaceparams.into_param().abi(),
            ::core::mem::transmute(ltimeout),
            ::core::mem::transmute(lflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ResourceDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceScriptable2 {
    type Vtable = IWorkspaceScriptable2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4025108898, 56741, 17053, [143, 66, 179, 59, 162, 196, 195, 72]);
}
impl ::core::convert::From<IWorkspaceScriptable2> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceScriptable2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceScriptable2> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceScriptable2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWorkspaceScriptable2> for IWorkspaceScriptable {
    fn from(value: IWorkspaceScriptable2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceScriptable2> for IWorkspaceScriptable {
    fn from(value: &IWorkspaceScriptable2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable> for &IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IWorkspaceScriptable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbssoenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ltimeout: i32,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWorkspaceScriptable3(pub ::windows::runtime::IUnknown);
impl IWorkspaceScriptable3 {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrworkspaceid: Param0,
        bstrusername: Param1,
        bstrpassword: Param2,
        bstrworkspaceparams: Param3,
        ltimeout: i32,
        lflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ::core::mem::transmute(ltimeout), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn IsWorkspaceCredentialSpecified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bcountunauthenticatedcredentials: i16) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), ::core::mem::transmute(bcountunauthenticatedcredentials), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ClearWorkspaceCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn OnAuthenticated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrusername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectWorkspaceByFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspacefriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspaceEx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::core::mem::transmute_copy(self),
            bstrworkspaceid.into_param().abi(),
            bstrworkspacefriendlyname.into_param().abi(),
            bstrredirectorname.into_param().abi(),
            bstrusername.into_param().abi(),
            bstrpassword.into_param().abi(),
            bstrappcontainer.into_param().abi(),
            bstrworkspaceparams.into_param().abi(),
            ::core::mem::transmute(ltimeout),
            ::core::mem::transmute(lflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ResourceDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrworkspaceid: Param0, bstrworkspacefriendlyname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn StartWorkspaceEx2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param10: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::core::mem::transmute_copy(self),
            bstrworkspaceid.into_param().abi(),
            bstrworkspacefriendlyname.into_param().abi(),
            bstrredirectorname.into_param().abi(),
            bstrusername.into_param().abi(),
            bstrpassword.into_param().abi(),
            bstrappcontainer.into_param().abi(),
            bstrworkspaceparams.into_param().abi(),
            ::core::mem::transmute(ltimeout),
            ::core::mem::transmute(lflags),
            bstreventloguploadaddress.into_param().abi(),
            correlationid.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWorkspaceScriptable3 {
    type Vtable = IWorkspaceScriptable3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394500882, 11455, 19410, [128, 165, 217, 10, 113, 99, 106, 154]);
}
impl ::core::convert::From<IWorkspaceScriptable3> for ::windows::runtime::IUnknown {
    fn from(value: IWorkspaceScriptable3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWorkspaceScriptable3> for ::windows::runtime::IUnknown {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWorkspaceScriptable3> for IWorkspaceScriptable2 {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceScriptable3> for IWorkspaceScriptable2 {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable2> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable2> for &IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWorkspaceScriptable3> for IWorkspaceScriptable {
    fn from(value: IWorkspaceScriptable3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWorkspaceScriptable3> for IWorkspaceScriptable {
    fn from(value: &IWorkspaceScriptable3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWorkspaceScriptable> for &IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWorkspaceScriptable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IWorkspaceScriptable3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbssoenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ltimeout: i32,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ltimeout: i32,
        lflags: i32,
        bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        correlationid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ItsPubPlugin(pub ::windows::runtime::IUnknown);
impl ItsPubPlugin {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::runtime::Result<pluginResource> {
        let mut result__: <pluginResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<pluginResource>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn pluginName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn pluginVersion(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ResolveResource<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: Param3, alias: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcetype), ::core::mem::transmute(resourcelocation), ::core::mem::transmute(endpointname), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ItsPubPlugin {
    type Vtable = ItsPubPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1891650309, 62279, 16683, [130, 47, 54, 201, 156, 84, 202, 69]);
}
impl ::core::convert::From<ItsPubPlugin> for ::windows::runtime::IUnknown {
    fn from(value: ItsPubPlugin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ItsPubPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &ItsPubPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ItsPubPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ItsPubPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastupdatetime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ItsPubPlugin2(pub ::windows::runtime::IUnknown);
impl ItsPubPlugin2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::runtime::Result<pluginResource> {
        let mut result__: <pluginResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<pluginResource>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn pluginName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn pluginVersion(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ResolveResource<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: Param3, alias: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcetype), ::core::mem::transmute(resourcelocation), ::core::mem::transmute(endpointname), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResource2List<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, userid: Param0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), userid.into_param().abi(), ::core::mem::transmute(pceapplistsize), ::core::mem::transmute(resourcelist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetResource2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, alias: Param0, flags: i32) -> ::windows::runtime::Result<pluginResource2> {
        let mut result__: <pluginResource2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), alias.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<pluginResource2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn ResolvePersonalDesktop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, userid: Param0, poolid: Param1, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), userid.into_param().abi(), poolid.into_param().abi(), ::core::mem::transmute(epdresolutiontype), ::core::mem::transmute(ppdassignmenttype), ::core::mem::transmute(endpointname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn DeletePersonalDesktopAssignment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, userid: Param0, poolid: Param1, endpointname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), userid.into_param().abi(), poolid.into_param().abi(), endpointname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ItsPubPlugin2 {
    type Vtable = ItsPubPlugin2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199343128, 43735, 20166, [186, 209, 10, 50, 27, 164, 101, 213]);
}
impl ::core::convert::From<ItsPubPlugin2> for ::windows::runtime::IUnknown {
    fn from(value: ItsPubPlugin2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ItsPubPlugin2> for ::windows::runtime::IUnknown {
    fn from(value: &ItsPubPlugin2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ItsPubPlugin2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ItsPubPlugin2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ItsPubPlugin> for ItsPubPlugin2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ItsPubPlugin> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ItsPubPlugin> for &ItsPubPlugin2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ItsPubPlugin> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastupdatetime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyCombinationType(pub i32);
pub const KeyCombinationHome: KeyCombinationType = KeyCombinationType(0i32);
pub const KeyCombinationLeft: KeyCombinationType = KeyCombinationType(1i32);
pub const KeyCombinationUp: KeyCombinationType = KeyCombinationType(2i32);
pub const KeyCombinationRight: KeyCombinationType = KeyCombinationType(3i32);
pub const KeyCombinationDown: KeyCombinationType = KeyCombinationType(4i32);
pub const KeyCombinationScroll: KeyCombinationType = KeyCombinationType(5i32);
impl ::core::convert::From<i32> for KeyCombinationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCombinationType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxAppName_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxDomainName_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxFQDN_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxFarm_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxNetBiosName_Len: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxNumOfExposed_IPs: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxUserName_Len: u32 = 104u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub type PCHANNEL_INIT_EVENT_FN = unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub type PCHANNEL_OPEN_EVENT_FN = unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PLUGIN_TYPE(pub i32);
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(0i32);
pub const POLICY_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(1i32);
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(2i32);
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(4i32);
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(8i32);
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(16i32);
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(32i32);
pub const TASK_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(64i32);
impl ::core::convert::From<i32> for PLUGIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PLUGIN_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(215995022, 53433, 19487, [165, 235, 109, 31, 108, 101, 53, 185]);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979100122, 13197, 19775, [129, 163, 231, 103, 49, 13, 144, 142]);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1645401943, 67, 18530, [153, 195, 159, 48, 89, 172, 42, 59]);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(427573882, 309, 19309, [156, 94, 230, 87, 154, 10, 182, 37]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub type PVIRTUALCHANNELCLOSE = unsafe extern "system" fn(openhandle: u32) -> u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELENTRY = unsafe extern "system" fn(pentrypoints: *mut ::core::mem::ManuallyDrop<CHANNEL_ENTRY_POINTS>) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELINIT = unsafe extern "system" fn(ppinithandle: *mut *mut ::core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: ::windows::runtime::RawPtr) -> u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELOPEN = unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, popenhandle: *mut u32, pchannelname: super::super::Foundation::PSTR, pchannelopeneventproc: ::windows::runtime::RawPtr) -> u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub type PVIRTUALCHANNELWRITE = unsafe extern "system" fn(openhandle: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, puserdata: *mut ::core::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PasswordEncodingType(pub i32);
pub const PasswordEncodingUTF8: PasswordEncodingType = PasswordEncodingType(0i32);
pub const PasswordEncodingUTF16LE: PasswordEncodingType = PasswordEncodingType(1i32);
pub const PasswordEncodingUTF16BE: PasswordEncodingType = PasswordEncodingType(2i32);
impl ::core::convert::From<i32> for PasswordEncodingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PasswordEncodingType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PolicyAttributeType(pub i32);
pub const EnableAllRedirections: PolicyAttributeType = PolicyAttributeType(0i32);
pub const DisableAllRedirections: PolicyAttributeType = PolicyAttributeType(1i32);
pub const DriveRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(2i32);
pub const PrinterRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(3i32);
pub const PortRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(4i32);
pub const ClipboardRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(5i32);
pub const PnpRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(6i32);
pub const AllowOnlySDRServers: PolicyAttributeType = PolicyAttributeType(7i32);
impl ::core::convert::From<i32> for PolicyAttributeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PolicyAttributeType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3838576843, 37934, 19225, [133, 4, 189, 90, 137, 167, 71, 245]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDV_TASK_STATUS(pub i32);
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = RDV_TASK_STATUS(0i32);
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = RDV_TASK_STATUS(1i32);
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = RDV_TASK_STATUS(2i32);
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = RDV_TASK_STATUS(3i32);
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = RDV_TASK_STATUS(4i32);
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = RDV_TASK_STATUS(5i32);
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = RDV_TASK_STATUS(6i32);
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = RDV_TASK_STATUS(7i32);
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = RDV_TASK_STATUS(8i32);
impl ::core::convert::From<i32> for RDV_TASK_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RDV_TASK_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RD_FARM_TYPE(pub i32);
pub const RD_FARM_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(0i32);
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = RD_FARM_TYPE(1i32);
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(2i32);
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(3i32);
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(4i32);
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(5i32);
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = RD_FARM_TYPE(-1i32);
impl ::core::convert::From<i32> for RD_FARM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RD_FARM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_CLEAR: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_VIDEO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MONITOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RFX_GFX_MONITOR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RFX_GFX_MONITOR_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::default::Default for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl RFX_GFX_MSG_HEADER {}
impl ::core::default::Default for RFX_GFX_MSG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_HEADER {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl RFX_GFX_MSG_RDP_DATA {}
impl ::core::default::Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_MSG_RDP_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_MSG_RDP_DATA {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_MSG_RDP_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl RFX_GFX_RECT {}
impl ::core::default::Default for RFX_GFX_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFX_GFX_RECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for RFX_GFX_RECT {}
unsafe impl ::windows::runtime::Abi for RFX_GFX_RECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteActionType(pub i32);
pub const RemoteActionCharms: RemoteActionType = RemoteActionType(0i32);
pub const RemoteActionAppbar: RemoteActionType = RemoteActionType(1i32);
pub const RemoteActionSnap: RemoteActionType = RemoteActionType(2i32);
pub const RemoteActionStartScreen: RemoteActionType = RemoteActionType(3i32);
pub const RemoteActionAppSwitch: RemoteActionType = RemoteActionType(4i32);
impl ::core::convert::From<i32> for RemoteActionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteActionType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SESSION_TIMEOUT_ACTION_TYPE(pub i32);
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(0i32);
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(1i32);
impl ::core::convert::From<i32> for SESSION_TIMEOUT_ACTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SESSION_TIMEOUT_ACTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const SINGLE_SESSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SnapshotEncodingType(pub i32);
pub const SnapshotEncodingDataUri: SnapshotEncodingType = SnapshotEncodingType(0i32);
impl ::core::convert::From<i32> for SnapshotEncodingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SnapshotEncodingType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SnapshotFormatType(pub i32);
pub const SnapshotFormatPng: SnapshotFormatType = SnapshotFormatType(0i32);
pub const SnapshotFormatJpeg: SnapshotFormatType = SnapshotFormatType(1i32);
pub const SnapshotFormatBmp: SnapshotFormatType = SnapshotFormatType(2i32);
impl ::core::convert::From<i32> for SnapshotFormatType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SnapshotFormatType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TARGET_CHANGE_TYPE(pub i32);
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1i32);
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(2i32);
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(4i32);
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(8i32);
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(16i32);
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(32i32);
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(64i32);
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(128i32);
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(256i32);
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(512i32);
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1024i32);
impl ::core::convert::From<i32> for TARGET_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TARGET_OWNER(pub i32);
pub const OWNER_UNKNOWN: TARGET_OWNER = TARGET_OWNER(0i32);
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = TARGET_OWNER(1i32);
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = TARGET_OWNER(2i32);
impl ::core::convert::From<i32> for TARGET_OWNER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_OWNER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TARGET_PATCH_STATE(pub i32);
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = TARGET_PATCH_STATE(0i32);
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(1i32);
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = TARGET_PATCH_STATE(2i32);
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(3i32);
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(4i32);
impl ::core::convert::From<i32> for TARGET_PATCH_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_PATCH_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TARGET_STATE(pub i32);
pub const TARGET_UNKNOWN: TARGET_STATE = TARGET_STATE(1i32);
pub const TARGET_INITIALIZING: TARGET_STATE = TARGET_STATE(2i32);
pub const TARGET_RUNNING: TARGET_STATE = TARGET_STATE(3i32);
pub const TARGET_DOWN: TARGET_STATE = TARGET_STATE(4i32);
pub const TARGET_HIBERNATED: TARGET_STATE = TARGET_STATE(5i32);
pub const TARGET_CHECKED_OUT: TARGET_STATE = TARGET_STATE(6i32);
pub const TARGET_STOPPED: TARGET_STATE = TARGET_STATE(7i32);
pub const TARGET_INVALID: TARGET_STATE = TARGET_STATE(8i32);
pub const TARGET_STARTING: TARGET_STATE = TARGET_STATE(9i32);
pub const TARGET_STOPPING: TARGET_STATE = TARGET_STATE(10i32);
pub const TARGET_MAXSTATE: TARGET_STATE = TARGET_STATE(11i32);
impl ::core::convert::From<i32> for TARGET_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TARGET_TYPE(pub i32);
pub const UNKNOWN: TARGET_TYPE = TARGET_TYPE(0i32);
pub const FARM: TARGET_TYPE = TARGET_TYPE(1i32);
pub const NONFARM: TARGET_TYPE = TARGET_TYPE(2i32);
impl ::core::convert::From<i32> for TARGET_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(pub i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(0i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(1i32);
impl ::core::convert::From<i32> for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(pub i32);
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(0i32);
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(1i32);
impl ::core::convert::From<i32> for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TSSB_NOTIFICATION_TYPE(pub i32);
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(0i32);
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(1i32);
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(2i32);
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(4i32);
impl ::core::convert::From<i32> for TSSB_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TSSB_NOTIFICATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TSSD_AddrV46Type(pub i32);
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = TSSD_AddrV46Type(0i32);
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = TSSD_AddrV46Type(4i32);
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = TSSD_AddrV46Type(6i32);
impl ::core::convert::From<i32> for TSSD_AddrV46Type {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TSSD_AddrV46Type {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl TSSD_ConnectionPoint {}
impl ::core::default::Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TSSD_ConnectionPoint {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TSSD_ConnectionPoint").field("ServerAddressB", &self.ServerAddressB).field("AddressType", &self.AddressType).field("PortNumber", &self.PortNumber).field("AddressScope", &self.AddressScope).finish()
    }
}
impl ::core::cmp::PartialEq for TSSD_ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.ServerAddressB == other.ServerAddressB && self.AddressType == other.AddressType && self.PortNumber == other.PortNumber && self.AddressScope == other.AddressScope
    }
}
impl ::core::cmp::Eq for TSSD_ConnectionPoint {}
unsafe impl ::windows::runtime::Abi for TSSD_ConnectionPoint {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TSSESSION_STATE(pub i32);
pub const STATE_INVALID: TSSESSION_STATE = TSSESSION_STATE(-1i32);
pub const STATE_ACTIVE: TSSESSION_STATE = TSSESSION_STATE(0i32);
pub const STATE_CONNECTED: TSSESSION_STATE = TSSESSION_STATE(1i32);
pub const STATE_CONNECTQUERY: TSSESSION_STATE = TSSESSION_STATE(2i32);
pub const STATE_SHADOW: TSSESSION_STATE = TSSESSION_STATE(3i32);
pub const STATE_DISCONNECTED: TSSESSION_STATE = TSSESSION_STATE(4i32);
pub const STATE_IDLE: TSSESSION_STATE = TSSESSION_STATE(5i32);
pub const STATE_LISTEN: TSSESSION_STATE = TSSESSION_STATE(6i32);
pub const STATE_RESET: TSSESSION_STATE = TSSESSION_STATE(7i32);
pub const STATE_DOWN: TSSESSION_STATE = TSSESSION_STATE(8i32);
pub const STATE_INIT: TSSESSION_STATE = TSSESSION_STATE(9i32);
pub const STATE_MAX: TSSESSION_STATE = TSSESSION_STATE(10i32);
impl ::core::convert::From<i32> for TSSESSION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TSSESSION_STATE {
    type Abi = Self;
}
pub const TSUserExInterfaces: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(152100097, 57228, 4561, [174, 39, 0, 192, 79, 163, 88, 19]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TS_SB_SORT_BY(pub i32);
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = TS_SB_SORT_BY(0i32);
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = TS_SB_SORT_BY(1i32);
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = TS_SB_SORT_BY(2i32);
impl ::core::convert::From<i32> for TS_SB_SORT_BY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TS_SB_SORT_BY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const USERNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VM_HOST_NOTIFY_STATUS(pub i32);
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(0i32);
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(1i32);
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(2i32);
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(3i32);
impl ::core::convert::From<i32> for VM_HOST_NOTIFY_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VM_HOST_NOTIFY_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl VM_NOTIFY_ENTRY {}
impl ::core::default::Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VM_NOTIFY_ENTRY").field("VmName", &self.VmName).field("VmHost", &self.VmHost).finish()
    }
}
impl ::core::cmp::PartialEq for VM_NOTIFY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VmName == other.VmName && self.VmHost == other.VmHost
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_ENTRY {}
unsafe impl ::windows::runtime::Abi for VM_NOTIFY_ENTRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl VM_NOTIFY_INFO {}
impl ::core::default::Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VM_NOTIFY_INFO").field("dwNumEntries", &self.dwNumEntries).field("ppVmEntries", &self.ppVmEntries).finish()
    }
}
impl ::core::cmp::PartialEq for VM_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.ppVmEntries == other.ppVmEntries
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_INFO {}
unsafe impl ::windows::runtime::Abi for VM_NOTIFY_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VM_NOTIFY_STATUS(pub i32);
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(0i32);
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(1i32);
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(2i32);
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(3i32);
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(4i32);
impl ::core::convert::From<i32> for VM_NOTIFY_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VM_NOTIFY_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl VM_PATCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VM_PATCH_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VM_PATCH_INFO").field("dwNumEntries", &self.dwNumEntries).field("pVmNames", &self.pVmNames).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VM_PATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.pVmNames == other.pVmNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VM_PATCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VM_PATCH_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTING {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_CONNECTION_SETTING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTINGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_CONNECTION_SETTINGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
    pub EventLogActivityId: ::windows::runtime::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_CONNECTION_SETTINGS_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_CONNECTION_SETTINGS_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRDS_CONNECTION_SETTING_LEVEL(pub i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(0i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(1i32);
impl ::core::convert::From<i32> for WRDS_CONNECTION_SETTING_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRDS_CONNECTION_SETTING_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WRDS_DYNAMIC_TIME_ZONE_INFORMATION")
            .field("Bias", &self.Bias)
            .field("StandardName", &self.StandardName)
            .field("StandardDate", &self.StandardDate)
            .field("StandardBias", &self.StandardBias)
            .field("DaylightName", &self.DaylightName)
            .field("DaylightDate", &self.DaylightDate)
            .field("DaylightBias", &self.DaylightBias)
            .field("TimeZoneKeyName", &self.TimeZoneKeyName)
            .field("DynamicDaylightTimeDisabled", &self.DynamicDaylightTimeDisabled)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias && self.TimeZoneKeyName == other.TimeZoneKeyName && self.DynamicDaylightTimeDisabled == other.DynamicDaylightTimeDisabled
    }
}
impl ::core::cmp::Eq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl WRDS_LISTENER_SETTING {}
impl ::core::default::Default for WRDS_LISTENER_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTING {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTING {}
unsafe impl ::windows::runtime::Abi for WRDS_LISTENER_SETTING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl WRDS_LISTENER_SETTINGS {}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS {}
unsafe impl ::windows::runtime::Abi for WRDS_LISTENER_SETTINGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl WRDS_LISTENER_SETTINGS_1 {}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTINGS_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WRDS_LISTENER_SETTINGS_1").field("MaxProtocolListenerConnectionCount", &self.MaxProtocolListenerConnectionCount).field("SecurityDescriptorSize", &self.SecurityDescriptorSize).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxProtocolListenerConnectionCount == other.MaxProtocolListenerConnectionCount && self.SecurityDescriptorSize == other.SecurityDescriptorSize && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS_1 {}
unsafe impl ::windows::runtime::Abi for WRDS_LISTENER_SETTINGS_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRDS_LISTENER_SETTING_LEVEL(pub i32);
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(0i32);
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(1i32);
impl ::core::convert::From<i32> for WRDS_LISTENER_SETTING_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRDS_LISTENER_SETTING_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3533258573, 719, 17024, [140, 72, 22, 36, 180, 79, 135, 6]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTING {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_SETTING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTINGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_SETTINGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WRDS_SETTINGS_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WRDS_SETTINGS_1")
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
impl ::core::cmp::PartialEq for WRDS_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.WRdsDisableClipStatus == other.WRdsDisableClipStatus
            && self.WRdsDisableClipValue == other.WRdsDisableClipValue
            && self.WRdsDisableLPTStatus == other.WRdsDisableLPTStatus
            && self.WRdsDisableLPTValue == other.WRdsDisableLPTValue
            && self.WRdsDisableCcmStatus == other.WRdsDisableCcmStatus
            && self.WRdsDisableCcmValue == other.WRdsDisableCcmValue
            && self.WRdsDisableCdmStatus == other.WRdsDisableCdmStatus
            && self.WRdsDisableCdmValue == other.WRdsDisableCdmValue
            && self.WRdsDisableCpmStatus == other.WRdsDisableCpmStatus
            && self.WRdsDisableCpmValue == other.WRdsDisableCpmValue
            && self.WRdsDisablePnpStatus == other.WRdsDisablePnpStatus
            && self.WRdsDisablePnpValue == other.WRdsDisablePnpValue
            && self.WRdsEncryptionLevelStatus == other.WRdsEncryptionLevelStatus
            && self.WRdsEncryptionValue == other.WRdsEncryptionValue
            && self.WRdsColorDepthStatus == other.WRdsColorDepthStatus
            && self.WRdsColorDepthValue == other.WRdsColorDepthValue
            && self.WRdsDisableAutoReconnecetStatus == other.WRdsDisableAutoReconnecetStatus
            && self.WRdsDisableAutoReconnecetValue == other.WRdsDisableAutoReconnecetValue
            && self.WRdsDisableEncryptionStatus == other.WRdsDisableEncryptionStatus
            && self.WRdsDisableEncryptionValue == other.WRdsDisableEncryptionValue
            && self.WRdsResetBrokenStatus == other.WRdsResetBrokenStatus
            && self.WRdsResetBrokenValue == other.WRdsResetBrokenValue
            && self.WRdsMaxIdleTimeStatus == other.WRdsMaxIdleTimeStatus
            && self.WRdsMaxIdleTimeValue == other.WRdsMaxIdleTimeValue
            && self.WRdsMaxDisconnectTimeStatus == other.WRdsMaxDisconnectTimeStatus
            && self.WRdsMaxDisconnectTimeValue == other.WRdsMaxDisconnectTimeValue
            && self.WRdsMaxConnectTimeStatus == other.WRdsMaxConnectTimeStatus
            && self.WRdsMaxConnectTimeValue == other.WRdsMaxConnectTimeValue
            && self.WRdsKeepAliveStatus == other.WRdsKeepAliveStatus
            && self.WRdsKeepAliveStartValue == other.WRdsKeepAliveStartValue
            && self.WRdsKeepAliveIntervalValue == other.WRdsKeepAliveIntervalValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WRDS_SETTINGS_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRDS_SETTING_LEVEL(pub i32);
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(0i32);
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(1i32);
impl ::core::convert::From<i32> for WRDS_SETTING_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRDS_SETTING_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRDS_SETTING_STATUS(pub i32);
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(-1i32);
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(0i32);
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(1i32);
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(2i32);
impl ::core::convert::From<i32> for WRDS_SETTING_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRDS_SETTING_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRDS_SETTING_TYPE(pub i32);
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(0i32);
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(1i32);
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(2i32);
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(3i32);
impl ::core::convert::From<i32> for WRDS_SETTING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRDS_SETTING_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WRdsGraphicsChannelType(pub i32);
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(0i32);
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(1i32);
impl ::core::convert::From<i32> for WRdsGraphicsChannelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WRdsGraphicsChannelType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCLIENTA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSCLIENTA")
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
impl ::core::cmp::PartialEq for WTSCLIENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSCLIENTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSCLIENTW {}
impl ::core::default::Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSCLIENTW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSCLIENTW")
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
impl ::core::cmp::PartialEq for WTSCLIENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for WTSCLIENTW {}
unsafe impl ::windows::runtime::Abi for WTSCLIENTW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCONFIGINFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSCONFIGINFOA")
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
impl ::core::cmp::PartialEq for WTSCONFIGINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon
            && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.ShadowSettings == other.ShadowSettings
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.ApplicationName == other.ApplicationName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSCONFIGINFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSCONFIGINFOW {}
impl ::core::default::Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSCONFIGINFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSCONFIGINFOW")
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
impl ::core::cmp::PartialEq for WTSCONFIGINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon
            && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.ShadowSettings == other.ShadowSettings
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.ApplicationName == other.ApplicationName
    }
}
impl ::core::cmp::Eq for WTSCONFIGINFOW {}
unsafe impl ::windows::runtime::Abi for WTSCONFIGINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCloseServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCloseServer(hserver: super::super::Foundation::HANDLE);
        }
        ::core::mem::transmute(WTSCloseServer(hserver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(logonid: u32, targetlogonid: u32, ppassword: Param2, bwait: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSConnectSessionA(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSConnectSessionA(::core::mem::transmute(logonid), ::core::mem::transmute(targetlogonid), ppassword.into_param().abi(), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(logonid: u32, targetlogonid: u32, ppassword: Param2, bwait: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSConnectSessionW(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PWSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSConnectSessionW(::core::mem::transmute(logonid), ::core::mem::transmute(targetlogonid), ppassword.into_param().abi(), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCreateListenerA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSCreateListenerA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSCreateListenerW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSCreateListenerW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSDisconnectSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, bwait: Param2) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnableChildSessions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(benable: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesExA(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(sessionid), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateProcessesExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateProcessesExW(hserver.into_param().abi(), ::core::mem::transmute(plevel), ::core::mem::transmute(sessionid), ::core::mem::transmute(ppprocessinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pdomainname: Param0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateServersA(pdomainname: super::super::Foundation::PSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateServersA(pdomainname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppserverinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pdomainname: Param0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSEnumerateServersW(pdomainname: super::super::Foundation::PWSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSEnumerateServersW(pdomainname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(version), ::core::mem::transmute(ppserverinfo), ::core::mem::transmute(pcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(WTSFreeMemory(::core::mem::transmute(pmemory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSGetListenerSecurityA(
            hserver.into_param().abi(),
            ::core::mem::transmute(preserved),
            ::core::mem::transmute(reserved),
            plistenername.into_param().abi(),
            ::core::mem::transmute(securityinformation),
            ::core::mem::transmute(psecuritydescriptor),
            ::core::mem::transmute(nlength),
            ::core::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSGetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSGetListenerSecurityW(
            hserver.into_param().abi(),
            ::core::mem::transmute(preserved),
            ::core::mem::transmute(reserved),
            plistenername.into_param().abi(),
            ::core::mem::transmute(securityinformation),
            ::core::mem::transmute(psecuritydescriptor),
            ::core::mem::transmute(nlength),
            ::core::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSINFOA")
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
impl ::core::cmp::PartialEq for WTSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.SessionId == other.SessionId
            && self.IncomingBytes == other.IncomingBytes
            && self.OutgoingBytes == other.OutgoingBytes
            && self.IncomingFrames == other.IncomingFrames
            && self.OutgoingFrames == other.OutgoingFrames
            && self.IncomingCompressedBytes == other.IncomingCompressedBytes
            && self.OutgoingCompressedBy == other.OutgoingCompressedBy
            && self.WinStationName == other.WinStationName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.ConnectTime == other.ConnectTime
            && self.DisconnectTime == other.DisconnectTime
            && self.LastInputTime == other.LastInputTime
            && self.LogonTime == other.LogonTime
            && self.CurrentTime == other.CurrentTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSINFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
#[cfg(feature = "Win32_Foundation")]
impl WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEXA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSINFOEXA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl WTSINFOEXW {}
impl ::core::default::Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSINFOEXW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTSINFOEXW {}
unsafe impl ::windows::runtime::Abi for WTSINFOEXW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSINFOEX_LEVEL1_A")
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
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_A {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.SessionState == other.SessionState
            && self.SessionFlags == other.SessionFlags
            && self.WinStationName == other.WinStationName
            && self.UserName == other.UserName
            && self.DomainName == other.DomainName
            && self.LogonTime == other.LogonTime
            && self.ConnectTime == other.ConnectTime
            && self.DisconnectTime == other.DisconnectTime
            && self.LastInputTime == other.LastInputTime
            && self.CurrentTime == other.CurrentTime
            && self.IncomingBytes == other.IncomingBytes
            && self.OutgoingBytes == other.OutgoingBytes
            && self.IncomingFrames == other.IncomingFrames
            && self.OutgoingFrames == other.OutgoingFrames
            && self.IncomingCompressedBytes == other.IncomingCompressedBytes
            && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSINFOEX_LEVEL1_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSINFOEX_LEVEL1_W {}
impl ::core::default::Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSINFOEX_LEVEL1_W")
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
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_W {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.SessionState == other.SessionState
            && self.SessionFlags == other.SessionFlags
            && self.WinStationName == other.WinStationName
            && self.UserName == other.UserName
            && self.DomainName == other.DomainName
            && self.LogonTime == other.LogonTime
            && self.ConnectTime == other.ConnectTime
            && self.DisconnectTime == other.DisconnectTime
            && self.LastInputTime == other.LastInputTime
            && self.CurrentTime == other.CurrentTime
            && self.IncomingBytes == other.IncomingBytes
            && self.OutgoingBytes == other.OutgoingBytes
            && self.IncomingFrames == other.IncomingFrames
            && self.OutgoingFrames == other.OutgoingFrames
            && self.IncomingCompressedBytes == other.IncomingCompressedBytes
            && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_W {}
unsafe impl ::windows::runtime::Abi for WTSINFOEX_LEVEL1_W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
#[cfg(feature = "Win32_Foundation")]
impl WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSINFOEX_LEVEL_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl WTSINFOEX_LEVEL_W {}
impl ::core::default::Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL_W {}
unsafe impl ::windows::runtime::Abi for WTSINFOEX_LEVEL_W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSINFOW {}
impl ::core::default::Default for WTSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSINFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSINFOW")
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
impl ::core::cmp::PartialEq for WTSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.SessionId == other.SessionId
            && self.IncomingBytes == other.IncomingBytes
            && self.OutgoingBytes == other.OutgoingBytes
            && self.IncomingFrames == other.IncomingFrames
            && self.OutgoingFrames == other.OutgoingFrames
            && self.IncomingCompressedBytes == other.IncomingCompressedBytes
            && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
            && self.WinStationName == other.WinStationName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.ConnectTime == other.ConnectTime
            && self.DisconnectTime == other.DisconnectTime
            && self.LastInputTime == other.LastInputTime
            && self.LogonTime == other.LogonTime
            && self.CurrentTime == other.CurrentTime
    }
}
impl ::core::cmp::Eq for WTSINFOW {}
unsafe impl ::windows::runtime::Abi for WTSINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSLISTENERCONFIGA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSLISTENERCONFIGA")
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
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSLISTENERCONFIGA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSLISTENERCONFIGW {}
impl ::core::default::Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSLISTENERCONFIGW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSLISTENERCONFIGW")
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
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
impl ::core::cmp::Eq for WTSLISTENERCONFIGW {}
unsafe impl ::windows::runtime::Abi for WTSLISTENERCONFIGW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSLogoffSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hserver: Param0, sessionid: u32, bwait: Param2) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerA(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerExA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerExA(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerExW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerExW(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pservername: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSOpenServerW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WTSOpenServerW(pservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryListenerConfigA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryListenerConfigA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryListenerConfigW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryListenerConfigW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQuerySessionInformationA(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQuerySessionInformationA(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(wtsinfoclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQuerySessionInformationW(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQuerySessionInformationW(hserver.into_param().abi(), ::core::mem::transmute(sessionid), ::core::mem::transmute(wtsinfoclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSQueryUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSQueryUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, dwflags: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hserver: Param0, hwnd: Param1, dwflags: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_ADDRESS_FAMILY(pub i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(0i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(1i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(2i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(3i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(4i32);
impl ::core::convert::From<i32> for WTSSBX_ADDRESS_FAMILY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_ADDRESS_FAMILY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl WTSSBX_IP_ADDRESS {}
impl ::core::default::Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSSBX_IP_ADDRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSSBX_IP_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).field("PortNumber", &self.PortNumber).field("dwScope", &self.dwScope).finish()
    }
}
impl ::core::cmp::PartialEq for WTSSBX_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address && self.PortNumber == other.PortNumber && self.dwScope == other.dwScope
    }
}
impl ::core::cmp::Eq for WTSSBX_IP_ADDRESS {}
unsafe impl ::windows::runtime::Abi for WTSSBX_IP_ADDRESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_CONNECT_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSSBX_MACHINE_CONNECT_INFO").field("wczMachineFQDN", &self.wczMachineFQDN).field("wczMachineNetBiosName", &self.wczMachineNetBiosName).field("dwNumOfIPAddr", &self.dwNumOfIPAddr).field("IPaddr", &self.IPaddr).finish()
    }
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_CONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wczMachineFQDN == other.wczMachineFQDN && self.wczMachineNetBiosName == other.wczMachineNetBiosName && self.dwNumOfIPAddr == other.dwNumOfIPAddr && self.IPaddr == other.IPaddr
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_CONNECT_INFO {}
unsafe impl ::windows::runtime::Abi for WTSSBX_MACHINE_CONNECT_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_MACHINE_DRAIN(pub i32);
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(0i32);
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(1i32);
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(2i32);
impl ::core::convert::From<i32> for WTSSBX_MACHINE_DRAIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_MACHINE_DRAIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSSBX_MACHINE_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSSBX_MACHINE_INFO")
            .field("ClientConnectInfo", &self.ClientConnectInfo)
            .field("wczFarmName", &self.wczFarmName)
            .field("InternalIPAddress", &self.InternalIPAddress)
            .field("dwMaxSessionsLimit", &self.dwMaxSessionsLimit)
            .field("ServerWeight", &self.ServerWeight)
            .field("SingleSessionMode", &self.SingleSessionMode)
            .field("InDrain", &self.InDrain)
            .field("MachineState", &self.MachineState)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientConnectInfo == other.ClientConnectInfo && self.wczFarmName == other.wczFarmName && self.InternalIPAddress == other.InternalIPAddress && self.dwMaxSessionsLimit == other.dwMaxSessionsLimit && self.ServerWeight == other.ServerWeight && self.SingleSessionMode == other.SingleSessionMode && self.InDrain == other.InDrain && self.MachineState == other.MachineState
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_INFO {}
unsafe impl ::windows::runtime::Abi for WTSSBX_MACHINE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_MACHINE_SESSION_MODE(pub i32);
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(0i32);
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(1i32);
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(2i32);
impl ::core::convert::From<i32> for WTSSBX_MACHINE_SESSION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_MACHINE_SESSION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_MACHINE_STATE(pub i32);
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(0i32);
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(1i32);
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(2i32);
impl ::core::convert::From<i32> for WTSSBX_MACHINE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_MACHINE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_NOTIFICATION_TYPE(pub i32);
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(1i32);
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(2i32);
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(4i32);
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(8i32);
impl ::core::convert::From<i32> for WTSSBX_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_NOTIFICATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSSBX_SESSION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSSBX_SESSION_INFO")
            .field("wszUserName", &self.wszUserName)
            .field("wszDomainName", &self.wszDomainName)
            .field("ApplicationType", &self.ApplicationType)
            .field("dwSessionId", &self.dwSessionId)
            .field("CreateTime", &self.CreateTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("SessionState", &self.SessionState)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSSBX_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wszUserName == other.wszUserName && self.wszDomainName == other.wszDomainName && self.ApplicationType == other.ApplicationType && self.dwSessionId == other.dwSessionId && self.CreateTime == other.CreateTime && self.DisconnectTime == other.DisconnectTime && self.SessionState == other.SessionState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSSBX_SESSION_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTSSBX_SESSION_STATE(pub i32);
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(0i32);
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(1i32);
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(2i32);
impl ::core::convert::From<i32> for WTSSBX_SESSION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTSSBX_SESSION_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl WTSSESSION_NOTIFICATION {}
impl ::core::default::Default for WTSSESSION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSSESSION_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSSESSION_NOTIFICATION").field("cbSize", &self.cbSize).field("dwSessionId", &self.dwSessionId).finish()
    }
}
impl ::core::cmp::PartialEq for WTSSESSION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSessionId == other.dwSessionId
    }
}
impl ::core::cmp::Eq for WTSSESSION_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for WTSSESSION_NOTIFICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    hserver: Param0,
    sessionid: u32,
    ptitle: Param2,
    titlelength: u32,
    pmessage: Param4,
    messagelength: u32,
    style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,
    timeout: u32,
    presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT,
    bwait: Param9,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSendMessageA(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PSTR, titlelength: u32, pmessage: super::super::Foundation::PSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSendMessageA(
            hserver.into_param().abi(),
            ::core::mem::transmute(sessionid),
            ptitle.into_param().abi(),
            ::core::mem::transmute(titlelength),
            pmessage.into_param().abi(),
            ::core::mem::transmute(messagelength),
            ::core::mem::transmute(style),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(presponse),
            bwait.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    hserver: Param0,
    sessionid: u32,
    ptitle: Param2,
    titlelength: u32,
    pmessage: Param4,
    messagelength: u32,
    style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,
    timeout: u32,
    presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT,
    bwait: Param9,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSendMessageW(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PWSTR, titlelength: u32, pmessage: super::super::Foundation::PWSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSendMessageW(
            hserver.into_param().abi(),
            ::core::mem::transmute(sessionid),
            ptitle.into_param().abi(),
            ::core::mem::transmute(titlelength),
            pmessage.into_param().abi(),
            ::core::mem::transmute(messagelength),
            ::core::mem::transmute(style),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(presponse),
            bwait.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetListenerSecurityA(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hserver: Param0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: Param3, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetListenerSecurityW(hserver.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(reserved), plistenername.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetRenderHint<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(prenderhintid: *mut u64, hwndowner: Param1, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows::runtime::HRESULT;
        }
        WTSSetRenderHint(::core::mem::transmute(prenderhintid), hwndowner.into_param().abi(), ::core::mem::transmute(renderhinttype), ::core::mem::transmute(cbhintdatalength), ::core::mem::transmute(phintdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: Param3, datalength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PSTR, datalength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), pbuffer.into_param().abi(), ::core::mem::transmute(datalength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pservername: Param0, pusername: Param1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: Param3, datalength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSSetUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PWSTR, datalength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSSetUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), ::core::mem::transmute(wtsconfigclass), pbuffer.into_param().abi(), ::core::mem::transmute(datalength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSShutdownSystem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, shutdownflag: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ptargetservername: Param0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSStartRemoteControlSessionA(ptargetservername: super::super::Foundation::PSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSStartRemoteControlSessionA(ptargetservername.into_param().abi(), ::core::mem::transmute(targetlogonid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ptargetservername: Param0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSStartRemoteControlSessionW(ptargetservername: super::super::Foundation::PWSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSStartRemoteControlSessionW(ptargetservername.into_param().abi(), ::core::mem::transmute(targetlogonid), ::core::mem::transmute(hotkeyvk), ::core::mem::transmute(hotkeymodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSTerminateProcess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSUSERCONFIGA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSUSERCONFIGA")
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
impl ::core::cmp::PartialEq for WTSUSERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTSUSERCONFIGA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTSUSERCONFIGW {}
impl ::core::default::Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTSUSERCONFIGW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTSUSERCONFIGW")
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
impl ::core::cmp::PartialEq for WTSUSERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
impl ::core::cmp::Eq for WTSUSERCONFIGW {}
unsafe impl ::windows::runtime::Abi for WTSUSERCONFIGW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hserver: Param0, hwnd: Param1) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hserver: Param0, sessionid: u32, pvirtualname: Param2) -> HwtsVirtualChannelHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelOpen(hserver: super::super::Foundation::HANDLE, sessionid: u32, pvirtualname: super::super::Foundation::PSTR) -> HwtsVirtualChannelHandle;
        }
        ::core::mem::transmute(WTSVirtualChannelOpen(hserver.into_param().abi(), ::core::mem::transmute(sessionid), pvirtualname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(sessionid: u32, pvirtualname: Param1, flags: u32) -> HwtsVirtualChannelHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelOpenEx(sessionid: u32, pvirtualname: super::super::Foundation::PSTR, flags: u32) -> HwtsVirtualChannelHandle;
        }
        ::core::mem::transmute(WTSVirtualChannelOpenEx(::core::mem::transmute(sessionid), pvirtualname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelRead<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hchannelhandle: Param0, timeout: u32, buffer: super::super::Foundation::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: super::super::Foundation::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelRead(hchannelhandle.into_param().abi(), ::core::mem::transmute(timeout), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(pbytesread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelWrite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hchannelhandle: Param0, buffer: Param1, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: super::super::Foundation::PSTR, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTSVirtualChannelWrite(hchannelhandle.into_param().abi(), buffer.into_param().abi(), ::core::mem::transmute(length), ::core::mem::transmute(pbyteswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSWaitSystemEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hserver: Param0, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl WTS_CACHE_STATS {}
impl ::core::default::Default for WTS_CACHE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_CACHE_STATS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTS_CACHE_STATS {}
unsafe impl ::windows::runtime::Abi for WTS_CACHE_STATS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl WTS_CACHE_STATS_UN {}
impl ::core::default::Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_CACHE_STATS_UN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTS_CACHE_STATS_UN {}
unsafe impl ::windows::runtime::Abi for WTS_CACHE_STATS_UN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_CERT_TYPE(pub i32);
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = WTS_CERT_TYPE(0i32);
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = WTS_CERT_TYPE(1i32);
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = WTS_CERT_TYPE(2i32);
impl ::core::convert::From<i32> for WTS_CERT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_CERT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl WTS_CLIENT_ADDRESS {}
impl ::core::default::Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_ADDRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_CLIENT_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_CLIENT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_ADDRESS {}
unsafe impl ::windows::runtime::Abi for WTS_CLIENT_ADDRESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_CLIENT_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_CLIENT_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl WTS_CLIENT_DISPLAY {}
impl ::core::default::Default for WTS_CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_DISPLAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_DISPLAY {}
unsafe impl ::windows::runtime::Abi for WTS_CLIENT_DISPLAY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_CONFIG_CLASS(pub i32);
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(0i32);
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(1i32);
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(2i32);
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(3i32);
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(4i32);
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(5i32);
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(6i32);
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(7i32);
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(8i32);
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(9i32);
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(10i32);
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(11i32);
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(12i32);
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(13i32);
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(14i32);
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(15i32);
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(16i32);
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(17i32);
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(18i32);
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(19i32);
impl ::core::convert::From<i32> for WTS_CONFIG_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_CONFIG_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_CONFIG_SOURCE(pub i32);
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = WTS_CONFIG_SOURCE(0i32);
impl ::core::convert::From<i32> for WTS_CONFIG_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_CONFIG_SOURCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_CONNECTSTATE_CLASS(pub i32);
pub const WTSActive: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(0i32);
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(1i32);
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(2i32);
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(3i32);
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(4i32);
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(5i32);
pub const WTSListen: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(6i32);
pub const WTSReset: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(7i32);
pub const WTSDown: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(8i32);
pub const WTSInit: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(9i32);
impl ::core::convert::From<i32> for WTS_CONNECTSTATE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_CONNECTSTATE_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl WTS_DISPLAY_IOCTL {}
impl ::core::default::Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_DISPLAY_IOCTL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_DISPLAY_IOCTL").field("pDisplayIOCtlData", &self.pDisplayIOCtlData).field("cbDisplayIOCtlData", &self.cbDisplayIOCtlData).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_DISPLAY_IOCTL {
    fn eq(&self, other: &Self) -> bool {
        self.pDisplayIOCtlData == other.pDisplayIOCtlData && self.cbDisplayIOCtlData == other.cbDisplayIOCtlData
    }
}
impl ::core::cmp::Eq for WTS_DISPLAY_IOCTL {}
unsafe impl ::windows::runtime::Abi for WTS_DISPLAY_IOCTL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_CONNECT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_CREATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LICENSE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LOGON: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_RENAME: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_INFO_CLASS(pub i32);
pub const WTSInitialProgram: WTS_INFO_CLASS = WTS_INFO_CLASS(0i32);
pub const WTSApplicationName: WTS_INFO_CLASS = WTS_INFO_CLASS(1i32);
pub const WTSWorkingDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(2i32);
pub const WTSOEMId: WTS_INFO_CLASS = WTS_INFO_CLASS(3i32);
pub const WTSSessionId: WTS_INFO_CLASS = WTS_INFO_CLASS(4i32);
pub const WTSUserName: WTS_INFO_CLASS = WTS_INFO_CLASS(5i32);
pub const WTSWinStationName: WTS_INFO_CLASS = WTS_INFO_CLASS(6i32);
pub const WTSDomainName: WTS_INFO_CLASS = WTS_INFO_CLASS(7i32);
pub const WTSConnectState: WTS_INFO_CLASS = WTS_INFO_CLASS(8i32);
pub const WTSClientBuildNumber: WTS_INFO_CLASS = WTS_INFO_CLASS(9i32);
pub const WTSClientName: WTS_INFO_CLASS = WTS_INFO_CLASS(10i32);
pub const WTSClientDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(11i32);
pub const WTSClientProductId: WTS_INFO_CLASS = WTS_INFO_CLASS(12i32);
pub const WTSClientHardwareId: WTS_INFO_CLASS = WTS_INFO_CLASS(13i32);
pub const WTSClientAddress: WTS_INFO_CLASS = WTS_INFO_CLASS(14i32);
pub const WTSClientDisplay: WTS_INFO_CLASS = WTS_INFO_CLASS(15i32);
pub const WTSClientProtocolType: WTS_INFO_CLASS = WTS_INFO_CLASS(16i32);
pub const WTSIdleTime: WTS_INFO_CLASS = WTS_INFO_CLASS(17i32);
pub const WTSLogonTime: WTS_INFO_CLASS = WTS_INFO_CLASS(18i32);
pub const WTSIncomingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(19i32);
pub const WTSOutgoingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(20i32);
pub const WTSIncomingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(21i32);
pub const WTSOutgoingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(22i32);
pub const WTSClientInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(23i32);
pub const WTSSessionInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(24i32);
pub const WTSSessionInfoEx: WTS_INFO_CLASS = WTS_INFO_CLASS(25i32);
pub const WTSConfigInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(26i32);
pub const WTSValidationInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(27i32);
pub const WTSSessionAddressV4: WTS_INFO_CLASS = WTS_INFO_CLASS(28i32);
pub const WTSIsRemoteSession: WTS_INFO_CLASS = WTS_INFO_CLASS(29i32);
impl ::core::convert::From<i32> for WTS_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_INFO_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_LICENSE_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_LICENSE_CAPABILITIES")
            .field("KeyExchangeAlg", &self.KeyExchangeAlg)
            .field("ProtocolVer", &self.ProtocolVer)
            .field("fAuthenticateServer", &self.fAuthenticateServer)
            .field("CertType", &self.CertType)
            .field("cbClientName", &self.cbClientName)
            .field("rgbClientName", &self.rgbClientName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_LICENSE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.KeyExchangeAlg == other.KeyExchangeAlg && self.ProtocolVer == other.ProtocolVer && self.fAuthenticateServer == other.fAuthenticateServer && self.CertType == other.CertType && self.cbClientName == other.cbClientName && self.rgbClientName == other.rgbClientName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_LICENSE_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_CREATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(pub i32);
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(0i32);
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(1i32);
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(2i32);
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(3i32);
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(4i32);
impl ::core::convert::From<i32> for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
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
impl WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_POLICY_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_POLICY_DATA")
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
impl ::core::cmp::PartialEq for WTS_POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.fDisableEncryption == other.fDisableEncryption && self.fDisableAutoReconnect == other.fDisableAutoReconnect && self.ColorDepth == other.ColorDepth && self.MinEncryptionLevel == other.MinEncryptionLevel && self.fDisableCpm == other.fDisableCpm && self.fDisableCdm == other.fDisableCdm && self.fDisableCcm == other.fDisableCcm && self.fDisableLPT == other.fDisableLPT && self.fDisableClip == other.fDisableClip && self.fDisablePNPRedir == other.fDisablePNPRedir
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_POLICY_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROCESS_INFOA").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROCESS_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROCESS_INFOW").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROCESS_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PSTR,
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
impl WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROCESS_INFO_EXA")
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
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.ProcessId == other.ProcessId
            && self.pProcessName == other.pProcessName
            && self.pUserSid == other.pUserSid
            && self.NumberOfThreads == other.NumberOfThreads
            && self.HandleCount == other.HandleCount
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.WorkingSetSize == other.WorkingSetSize
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.UserTime == other.UserTime
            && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROCESS_INFO_EXA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PWSTR,
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
impl WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROCESS_INFO_EXW")
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
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.ProcessId == other.ProcessId
            && self.pProcessName == other.pProcessName
            && self.pUserSid == other.pUserSid
            && self.NumberOfThreads == other.NumberOfThreads
            && self.HandleCount == other.HandleCount
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.WorkingSetSize == other.WorkingSetSize
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.UserTime == other.UserTime
            && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROCESS_INFO_EXW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROPERTY_VALUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROPERTY_VALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROPERTY_VALUE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROPERTY_VALUE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_bVal_e__Struct").field("size", &self.size).field("pbVal", &self.pbVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.pbVal == other.pbVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROPERTY_VALUE_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_PROPERTY_VALUE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_strVal_e__Struct").field("size", &self.size).field("pstrVal", &self.pstrVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.pstrVal == other.pstrVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_PROPERTY_VALUE_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl WTS_PROTOCOL_CACHE {}
impl ::core::default::Default for WTS_PROTOCOL_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_CACHE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROTOCOL_CACHE").field("CacheReads", &self.CacheReads).field("CacheHits", &self.CacheHits).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_CACHE {
    fn eq(&self, other: &Self) -> bool {
        self.CacheReads == other.CacheReads && self.CacheHits == other.CacheHits
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_CACHE {}
unsafe impl ::windows::runtime::Abi for WTS_PROTOCOL_CACHE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTS_PROTOCOL_COUNTERS {}
impl ::core::default::Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_COUNTERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_PROTOCOL_COUNTERS")
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
impl ::core::cmp::PartialEq for WTS_PROTOCOL_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.WdBytes == other.WdBytes
            && self.WdFrames == other.WdFrames
            && self.WaitForOutBuf == other.WaitForOutBuf
            && self.Frames == other.Frames
            && self.Bytes == other.Bytes
            && self.CompressedBytes == other.CompressedBytes
            && self.CompressFlushes == other.CompressFlushes
            && self.Errors == other.Errors
            && self.Timeouts == other.Timeouts
            && self.AsyncFramingError == other.AsyncFramingError
            && self.AsyncOverrunError == other.AsyncOverrunError
            && self.AsyncOverflowError == other.AsyncOverflowError
            && self.AsyncParityError == other.AsyncParityError
            && self.TdErrors == other.TdErrors
            && self.ProtocolType == other.ProtocolType
            && self.Length == other.Length
            && self.Specific == other.Specific
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WTS_PROTOCOL_COUNTERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl WTS_PROTOCOL_STATUS {}
impl ::core::default::Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_STATUS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_STATUS {}
unsafe impl ::windows::runtime::Abi for WTS_PROTOCOL_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3346864944, 23521, 19563, [160, 225, 189, 109, 46, 92, 159, 204]);
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2616523415, 51331, 19498, [128, 171, 90, 57, 201, 175, 0, 219]);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2341343207, 2052, 18958, [178, 121, 134, 96, 177, 223, 0, 73]);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099340496, 25394, 19912, [149, 213, 219, 116, 158, 47, 29, 148]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_RCM_DRAIN_STATE(pub i32);
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(0i32);
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(1i32);
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(2i32);
impl ::core::convert::From<i32> for WTS_RCM_DRAIN_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_RCM_DRAIN_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_RCM_SERVICE_STATE(pub i32);
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(0i32);
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(1i32);
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(2i32);
impl ::core::convert::From<i32> for WTS_RCM_SERVICE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_RCM_SERVICE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_CONNECT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_DISCONNECT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_LOGON: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_MESSAGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_RESET: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SERVER_INFOA {
    pub pServerName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SERVER_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SERVER_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SERVER_INFOA").field("pServerName", &self.pServerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SERVER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SERVER_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SERVER_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SERVER_INFOW {
    pub pServerName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SERVER_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SERVER_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SERVER_INFOW").field("pServerName", &self.pServerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SERVER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SERVER_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SERVER_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl WTS_SERVICE_STATE {}
impl ::core::default::Default for WTS_SERVICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SERVICE_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SERVICE_STATE").field("RcmServiceState", &self.RcmServiceState).field("RcmDrainState", &self.RcmDrainState).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SERVICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.RcmServiceState == other.RcmServiceState && self.RcmDrainState == other.RcmDrainState
    }
}
impl ::core::cmp::Eq for WTS_SERVICE_STATE {}
unsafe impl ::windows::runtime::Abi for WTS_SERVICE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl WTS_SESSION_ADDRESS {}
impl ::core::default::Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ADDRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ADDRESS {}
unsafe impl ::windows::runtime::Abi for WTS_SESSION_ADDRESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: ::windows::runtime::GUID,
    pub SessionId: u32,
}
impl WTS_SESSION_ID {}
impl ::core::default::Default for WTS_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_ID").field("SessionUniqueGuid", &self.SessionUniqueGuid).field("SessionId", &self.SessionId).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionUniqueGuid == other.SessionUniqueGuid && self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ID {}
unsafe impl ::windows::runtime::Abi for WTS_SESSION_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: super::super::Foundation::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SESSION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SESSION_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_INFOA").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SESSION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SESSION_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: super::super::Foundation::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SESSION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SESSION_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_INFOW").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SESSION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SESSION_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: super::super::Foundation::PSTR,
    pub pHostName: super::super::Foundation::PSTR,
    pub pUserName: super::super::Foundation::PSTR,
    pub pDomainName: super::super::Foundation::PSTR,
    pub pFarmName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SESSION_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SESSION_INFO_1A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_INFO_1A")
            .field("ExecEnvId", &self.ExecEnvId)
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("pSessionName", &self.pSessionName)
            .field("pHostName", &self.pHostName)
            .field("pUserName", &self.pUserName)
            .field("pDomainName", &self.pDomainName)
            .field("pFarmName", &self.pFarmName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SESSION_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SESSION_INFO_1A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: super::super::Foundation::PWSTR,
    pub pHostName: super::super::Foundation::PWSTR,
    pub pUserName: super::super::Foundation::PWSTR,
    pub pDomainName: super::super::Foundation::PWSTR,
    pub pFarmName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_SESSION_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_SESSION_INFO_1W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SESSION_INFO_1W")
            .field("ExecEnvId", &self.ExecEnvId)
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("pSessionName", &self.pSessionName)
            .field("pHostName", &self.pHostName)
            .field("pUserName", &self.pUserName)
            .field("pDomainName", &self.pDomainName)
            .field("pFarmName", &self.pFarmName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_SESSION_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_SESSION_INFO_1W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl WTS_SMALL_RECT {}
impl ::core::default::Default for WTS_SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SMALL_RECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for WTS_SMALL_RECT {}
unsafe impl ::windows::runtime::Abi for WTS_SMALL_RECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl WTS_SOCKADDR {}
impl ::core::default::Default for WTS_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR {}
unsafe impl ::windows::runtime::Abi for WTS_SOCKADDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl WTS_SOCKADDR_0 {}
impl ::core::default::Default for WTS_SOCKADDR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0 {}
unsafe impl ::windows::runtime::Abi for WTS_SOCKADDR_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl WTS_SOCKADDR_0_0 {}
impl ::core::default::Default for WTS_SOCKADDR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ipv4_e__Struct").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sin_port == other.sin_port && self.IN_ADDR == other.IN_ADDR && self.sin_zero == other.sin_zero
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_0 {}
unsafe impl ::windows::runtime::Abi for WTS_SOCKADDR_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl WTS_SOCKADDR_0_1 {}
impl ::core::default::Default for WTS_SOCKADDR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ipv6_e__Struct").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_port == other.sin6_port && self.sin6_flowinfo == other.sin6_flowinfo && self.sin6_addr == other.sin6_addr && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_1 {}
unsafe impl ::windows::runtime::Abi for WTS_SOCKADDR_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
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
impl WTS_SYSTEMTIME {}
impl ::core::default::Default for WTS_SYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_SYSTEMTIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_SYSTEMTIME")
            .field("wYear", &self.wYear)
            .field("wMonth", &self.wMonth)
            .field("wDayOfWeek", &self.wDayOfWeek)
            .field("wDay", &self.wDay)
            .field("wHour", &self.wHour)
            .field("wMinute", &self.wMinute)
            .field("wSecond", &self.wSecond)
            .field("wMilliseconds", &self.wMilliseconds)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WTS_SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDayOfWeek == other.wDayOfWeek && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds
    }
}
impl ::core::cmp::Eq for WTS_SYSTEMTIME {}
unsafe impl ::windows::runtime::Abi for WTS_SYSTEMTIME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl WTS_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_TIME_ZONE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_TIME_ZONE_INFORMATION")
            .field("Bias", &self.Bias)
            .field("StandardName", &self.StandardName)
            .field("StandardDate", &self.StandardDate)
            .field("StandardBias", &self.StandardBias)
            .field("DaylightName", &self.DaylightName)
            .field("DaylightDate", &self.DaylightDate)
            .field("DaylightBias", &self.DaylightBias)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WTS_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias
    }
}
impl ::core::cmp::Eq for WTS_TIME_ZONE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for WTS_TIME_ZONE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_TYPE_CLASS(pub i32);
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = WTS_TYPE_CLASS(0i32);
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(1i32);
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(2i32);
impl ::core::convert::From<i32> for WTS_TYPE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_TYPE_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl WTS_USER_CREDENTIAL {}
impl ::core::default::Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_USER_CREDENTIAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_USER_CREDENTIAL").field("UserName", &self.UserName).field("Password", &self.Password).field("Domain", &self.Domain).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_USER_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.Password == other.Password && self.Domain == other.Domain
    }
}
impl ::core::cmp::Eq for WTS_USER_CREDENTIAL {}
unsafe impl ::windows::runtime::Abi for WTS_USER_CREDENTIAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl WTS_USER_DATA {}
impl ::core::default::Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_USER_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_USER_DATA").field("WorkDirectory", &self.WorkDirectory).field("InitialProgram", &self.InitialProgram).field("UserTimeZone", &self.UserTimeZone).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_USER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.UserTimeZone == other.UserTimeZone
    }
}
impl ::core::cmp::Eq for WTS_USER_DATA {}
unsafe impl ::windows::runtime::Abi for WTS_USER_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: _WTS_PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_VALIDATION_INFORMATIONA").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WTS_VALIDATION_INFORMATIONA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: _WTS_PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl WTS_VALIDATION_INFORMATIONW {}
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WTS_VALIDATION_INFORMATIONW").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONW {}
unsafe impl ::windows::runtime::Abi for WTS_VALIDATION_INFORMATIONW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WTS_VIRTUAL_CLASS(pub i32);
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(0i32);
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(1i32);
impl ::core::convert::From<i32> for WTS_VIRTUAL_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WTS_VIRTUAL_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_LOGOFF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_POWEROFF: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_REBOOT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1327365286, 15021, 18657, [132, 6, 75, 194, 26, 80, 29, 124]);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _ITSWkspEvents(pub ::windows::runtime::IUnknown);
impl _ITSWkspEvents {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for _ITSWkspEvents {
    type Vtable = _ITSWkspEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3106061240, 19541, 20458, [132, 150, 190, 176, 180, 66, 133, 233]);
}
impl ::core::convert::From<_ITSWkspEvents> for ::windows::runtime::IUnknown {
    fn from(value: _ITSWkspEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_ITSWkspEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_ITSWkspEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _ITSWkspEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _ITSWkspEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for _ITSWkspEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &_ITSWkspEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ITSWkspEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct _WTS_PRODUCT_INFOA {
    pub CompanyName: [super::super::Foundation::CHAR; 256],
    pub ProductID: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _WTS_PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _WTS_PRODUCT_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_WTS_PRODUCT_INFOA").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _WTS_PRODUCT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _WTS_PRODUCT_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct _WTS_PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl _WTS_PRODUCT_INFOW {}
impl ::core::default::Default for _WTS_PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _WTS_PRODUCT_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_WTS_PRODUCT_INFOW").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
impl ::core::cmp::PartialEq for _WTS_PRODUCT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
impl ::core::cmp::Eq for _WTS_PRODUCT_INFOW {}
unsafe impl ::windows::runtime::Abi for _WTS_PRODUCT_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: super::super::Foundation::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl pluginResource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for pluginResource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for pluginResource {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("pluginResource")
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for pluginResource {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias && self.name == other.name && self.resourceFileContents == other.resourceFileContents && self.fileExtension == other.fileExtension && self.resourcePluginType == other.resourcePluginType && self.isDiscoverable == other.isDiscoverable && self.resourceType == other.resourceType && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents && self.pcePluginBlobSize == other.pcePluginBlobSize && self.blobContents == other.blobContents
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for pluginResource {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for pluginResource {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: super::super::Foundation::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl pluginResource2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for pluginResource2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for pluginResource2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("pluginResource2")
            .field("resourceV1", &self.resourceV1)
            .field("pceFileAssocListSize", &self.pceFileAssocListSize)
            .field("fileAssocList", &self.fileAssocList)
            .field("securityDescriptor", &self.securityDescriptor)
            .field("pceFolderListSize", &self.pceFolderListSize)
            .field("folderList", &self.folderList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for pluginResource2 {
    fn eq(&self, other: &Self) -> bool {
        self.resourceV1 == other.resourceV1 && self.pceFileAssocListSize == other.pceFileAssocListSize && self.fileAssocList == other.fileAssocList && self.securityDescriptor == other.securityDescriptor && self.pceFolderListSize == other.pceFolderListSize && self.folderList == other.folderList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for pluginResource2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for pluginResource2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl pluginResource2FileAssociation {}
impl ::core::default::Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for pluginResource2FileAssociation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("pluginResource2FileAssociation").field("extName", &self.extName).field("primaryHandler", &self.primaryHandler).field("pceIconSize", &self.pceIconSize).field("iconContents", &self.iconContents).finish()
    }
}
impl ::core::cmp::PartialEq for pluginResource2FileAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.extName == other.extName && self.primaryHandler == other.primaryHandler && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents
    }
}
impl ::core::cmp::Eq for pluginResource2FileAssociation {}
unsafe impl ::windows::runtime::Abi for pluginResource2FileAssociation {
    type Abi = Self;
}
