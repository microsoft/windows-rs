#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP2_SETTINGS_LIMITS_PARAM {
    pub Http2MaxSettingsPerFrame: u32,
    pub Http2MaxSettingsPerMinute: u32,
}
impl HTTP2_SETTINGS_LIMITS_PARAM {}
impl ::std::default::Default for HTTP2_SETTINGS_LIMITS_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP2_SETTINGS_LIMITS_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP2_SETTINGS_LIMITS_PARAM").field("Http2MaxSettingsPerFrame", &self.Http2MaxSettingsPerFrame).field("Http2MaxSettingsPerMinute", &self.Http2MaxSettingsPerMinute).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP2_SETTINGS_LIMITS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Http2MaxSettingsPerFrame == other.Http2MaxSettingsPerFrame && self.Http2MaxSettingsPerMinute == other.Http2MaxSettingsPerMinute
    }
}
impl ::std::cmp::Eq for HTTP2_SETTINGS_LIMITS_PARAM {}
unsafe impl ::windows::runtime::Abi for HTTP2_SETTINGS_LIMITS_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: u32,
}
impl HTTP2_WINDOW_SIZE_PARAM {}
impl ::std::default::Default for HTTP2_WINDOW_SIZE_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP2_WINDOW_SIZE_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP2_WINDOW_SIZE_PARAM").field("Http2ReceiveWindowSize", &self.Http2ReceiveWindowSize).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP2_WINDOW_SIZE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Http2ReceiveWindowSize == other.Http2ReceiveWindowSize
    }
}
impl ::std::cmp::Eq for HTTP2_WINDOW_SIZE_PARAM {}
unsafe impl ::windows::runtime::Abi for HTTP2_WINDOW_SIZE_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: u16,
    pub HttpApiMinorVersion: u16,
}
impl HTTPAPI_VERSION {}
impl ::std::default::Default for HTTPAPI_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTPAPI_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTPAPI_VERSION").field("HttpApiMajorVersion", &self.HttpApiMajorVersion).field("HttpApiMinorVersion", &self.HttpApiMinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for HTTPAPI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.HttpApiMajorVersion == other.HttpApiMajorVersion && self.HttpApiMinorVersion == other.HttpApiMinorVersion
    }
}
impl ::std::cmp::Eq for HTTPAPI_VERSION {}
unsafe impl ::windows::runtime::Abi for HTTPAPI_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_503_RESPONSE_VERBOSITY(pub i32);
pub const Http503ResponseVerbosityBasic: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(0i32);
pub const Http503ResponseVerbosityLimited: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(1i32);
pub const Http503ResponseVerbosityFull: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(2i32);
impl ::std::convert::From<i32> for HTTP_503_RESPONSE_VERBOSITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_503_RESPONSE_VERBOSITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_AUTHENTICATION_HARDENING_LEVELS(pub i32);
pub const HttpAuthenticationHardeningLegacy: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(0i32);
pub const HttpAuthenticationHardeningMedium: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(1i32);
pub const HttpAuthenticationHardeningStrict: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(2i32);
impl ::std::convert::From<i32> for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_AUTH_STATUS(pub i32);
pub const HttpAuthStatusSuccess: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(0i32);
pub const HttpAuthStatusNotAuthenticated: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(1i32);
pub const HttpAuthStatusFailure: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(2i32);
impl ::std::convert::From<i32> for HTTP_AUTH_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_AUTH_STATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
}
impl HTTP_BANDWIDTH_LIMIT_INFO {}
impl ::std::default::Default for HTTP_BANDWIDTH_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_BANDWIDTH_LIMIT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_BANDWIDTH_LIMIT_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_BANDWIDTH_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxBandwidth == other.MaxBandwidth
    }
}
impl ::std::cmp::Eq for HTTP_BANDWIDTH_LIMIT_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_BANDWIDTH_LIMIT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_BINDING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_BINDING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_BINDING_INFO").field("Flags", &self.Flags).field("RequestQueueHandle", &self.RequestQueueHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequestQueueHandle == other.RequestQueueHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_BINDING_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_BINDING_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: u64,
    pub Length: u64,
}
impl HTTP_BYTE_RANGE {}
impl ::std::default::Default for HTTP_BYTE_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_BYTE_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_BYTE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_BYTE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for HTTP_BYTE_RANGE {}
unsafe impl ::windows::runtime::Abi for HTTP_BYTE_RANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: u32,
}
impl HTTP_CACHE_POLICY {}
impl ::std::default::Default for HTTP_CACHE_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_CACHE_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_CACHE_POLICY").field("Policy", &self.Policy).field("SecondsToLive", &self.SecondsToLive).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_CACHE_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Policy == other.Policy && self.SecondsToLive == other.SecondsToLive
    }
}
impl ::std::cmp::Eq for HTTP_CACHE_POLICY {}
unsafe impl ::windows::runtime::Abi for HTTP_CACHE_POLICY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_CACHE_POLICY_TYPE(pub i32);
pub const HttpCachePolicyNocache: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(0i32);
pub const HttpCachePolicyUserInvalidates: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(1i32);
pub const HttpCachePolicyTimeToLive: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(2i32);
pub const HttpCachePolicyMaximum: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(3i32);
impl ::std::convert::From<i32> for HTTP_CACHE_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_CACHE_POLICY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: u32,
    pub ServiceNames: *mut *mut HTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: u32,
}
impl HTTP_CHANNEL_BIND_INFO {}
impl ::std::default::Default for HTTP_CHANNEL_BIND_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_CHANNEL_BIND_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_CHANNEL_BIND_INFO").field("Hardening", &self.Hardening).field("Flags", &self.Flags).field("ServiceNames", &self.ServiceNames).field("NumberOfServiceNames", &self.NumberOfServiceNames).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_CHANNEL_BIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Hardening == other.Hardening && self.Flags == other.Flags && self.ServiceNames == other.ServiceNames && self.NumberOfServiceNames == other.NumberOfServiceNames
    }
}
impl ::std::cmp::Eq for HTTP_CHANNEL_BIND_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_CHANNEL_BIND_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: u32,
}
impl HTTP_CONNECTION_LIMIT_INFO {}
impl ::std::default::Default for HTTP_CONNECTION_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_CONNECTION_LIMIT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_CONNECTION_LIMIT_INFO").field("Flags", &self.Flags).field("MaxConnections", &self.MaxConnections).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_CONNECTION_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxConnections == other.MaxConnections
    }
}
impl ::std::cmp::Eq for HTTP_CONNECTION_LIMIT_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_CONNECTION_LIMIT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: u16,
    pub HostLength: u16,
    pub AbsPathLength: u16,
    pub QueryStringLength: u16,
    pub pFullUrl: super::super::Foundation::PWSTR,
    pub pHost: super::super::Foundation::PWSTR,
    pub pAbsPath: super::super::Foundation::PWSTR,
    pub pQueryString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_COOKED_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_COOKED_URL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_COOKED_URL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_COOKED_URL")
            .field("FullUrlLength", &self.FullUrlLength)
            .field("HostLength", &self.HostLength)
            .field("AbsPathLength", &self.AbsPathLength)
            .field("QueryStringLength", &self.QueryStringLength)
            .field("pFullUrl", &self.pFullUrl)
            .field("pHost", &self.pHost)
            .field("pAbsPath", &self.pAbsPath)
            .field("pQueryString", &self.pQueryString)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_COOKED_URL {
    fn eq(&self, other: &Self) -> bool {
        self.FullUrlLength == other.FullUrlLength && self.HostLength == other.HostLength && self.AbsPathLength == other.AbsPathLength && self.QueryStringLength == other.QueryStringLength && self.pFullUrl == other.pFullUrl && self.pHost == other.pHost && self.pAbsPath == other.pAbsPath && self.pQueryString == other.pQueryString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_COOKED_URL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_COOKED_URL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(pub i32);
pub const CreateRequestQueueExternalIdProperty: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(1i32);
pub const CreateRequestQueueMax: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(2i32);
impl ::std::convert::From<i32> for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    pub PropertyId: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut ::std::ffi::c_void,
}
impl HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {}
impl ::std::default::Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.PropertyInfoLength == other.PropertyInfoLength && self.PropertyInfo == other.PropertyInfo
    }
}
impl ::std::cmp::Eq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub Anonymous: HTTP_DATA_CHUNK_0,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub union HTTP_DATA_CHUNK_0 {
    pub FromMemory: HTTP_DATA_CHUNK_0_3,
    pub FromFileHandle: HTTP_DATA_CHUNK_0_0,
    pub FromFragmentCache: HTTP_DATA_CHUNK_0_2,
    pub FromFragmentCacheEx: HTTP_DATA_CHUNK_0_1,
    pub Trailers: HTTP_DATA_CHUNK_0_4,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_DATA_CHUNK_0_0 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_DATA_CHUNK_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FromFileHandle_e__Struct").field("ByteRange", &self.ByteRange).field("FileHandle", &self.FileHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ByteRange == other.ByteRange && self.FileHandle == other.FileHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_DATA_CHUNK_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FromFragmentCacheEx_e__Struct").field("ByteRange", &self.ByteRange).field("pFragmentName", &self.pFragmentName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ByteRange == other.ByteRange && self.pFragmentName == other.pFragmentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: u16,
    pub pFragmentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_DATA_CHUNK_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FromFragmentCache_e__Struct").field("FragmentNameLength", &self.FragmentNameLength).field("pFragmentName", &self.pFragmentName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.FragmentNameLength == other.FragmentNameLength && self.pFragmentName == other.pFragmentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_DATA_CHUNK_0_3 {
    pub pBuffer: *mut ::std::ffi::c_void,
    pub BufferLength: u32,
}
impl HTTP_DATA_CHUNK_0_3 {}
impl ::std::default::Default for HTTP_DATA_CHUNK_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_DATA_CHUNK_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FromMemory_e__Struct").field("pBuffer", &self.pBuffer).field("BufferLength", &self.BufferLength).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.BufferLength == other.BufferLength
    }
}
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0_3 {}
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_DATA_CHUNK_0_4 {
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_DATA_CHUNK_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_DATA_CHUNK_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_DATA_CHUNK_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Trailers_e__Struct").field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_DATA_CHUNK_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.TrailerCount == other.TrailerCount && self.pTrailers == other.pTrailers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_DATA_CHUNK_0_4 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_0_4 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_DATA_CHUNK_TYPE(pub i32);
pub const HttpDataChunkFromMemory: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(0i32);
pub const HttpDataChunkFromFileHandle: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(1i32);
pub const HttpDataChunkFromFragmentCache: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(2i32);
pub const HttpDataChunkFromFragmentCacheEx: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(3i32);
pub const HttpDataChunkTrailers: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(4i32);
pub const HttpDataChunkMaximum: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(5i32);
impl ::std::convert::From<i32> for HTTP_DATA_CHUNK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_DATA_CHUNK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_ID(pub i32);
pub const DelegateRequestReservedProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(0i32);
pub const DelegateRequestDelegateUrlProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(1i32);
impl ::std::convert::From<i32> for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    pub PropertyId: HTTP_DELEGATE_REQUEST_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut ::std::ffi::c_void,
}
impl HTTP_DELEGATE_REQUEST_PROPERTY_INFO {}
impl ::std::default::Default for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_DELEGATE_REQUEST_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.PropertyInfoLength == other.PropertyInfoLength && self.PropertyInfo == other.PropertyInfo
    }
}
impl ::std::cmp::Eq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_DEMAND_CBT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_ENABLED_STATE(pub i32);
pub const HttpEnabledStateActive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(0i32);
pub const HttpEnabledStateInactive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(1i32);
impl ::std::convert::From<i32> for HTTP_ENABLED_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_ENABLED_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: u16,
    pub HeaderCount: u16,
    pub Headers: *mut HTTP_UNKNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_ERROR_HEADERS_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_ERROR_HEADERS_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_ERROR_HEADERS_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_ERROR_HEADERS_PARAM").field("StatusCode", &self.StatusCode).field("HeaderCount", &self.HeaderCount).field("Headers", &self.Headers).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_ERROR_HEADERS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.StatusCode == other.StatusCode && self.HeaderCount == other.HeaderCount && self.Headers == other.Headers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_ERROR_HEADERS_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_ERROR_HEADERS_PARAM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_FEATURE_ID(pub i32);
pub const HttpFeatureUnknown: HTTP_FEATURE_ID = HTTP_FEATURE_ID(0i32);
pub const HttpFeatureResponseTrailers: HTTP_FEATURE_ID = HTTP_FEATURE_ID(1i32);
pub const HttpFeatureApiTimings: HTTP_FEATURE_ID = HTTP_FEATURE_ID(2i32);
pub const HttpFeatureDelegateEx: HTTP_FEATURE_ID = HTTP_FEATURE_ID(3i32);
pub const HttpFeatureHttp3: HTTP_FEATURE_ID = HTTP_FEATURE_ID(4i32);
pub const HttpFeaturemax: HTTP_FEATURE_ID = HTTP_FEATURE_ID(-1i32);
impl ::std::convert::From<i32> for HTTP_FEATURE_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_FEATURE_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
    pub MaxPeakBandwidth: u32,
    pub BurstSize: u32,
}
impl HTTP_FLOWRATE_INFO {}
impl ::std::default::Default for HTTP_FLOWRATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_FLOWRATE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FLOWRATE_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).field("MaxPeakBandwidth", &self.MaxPeakBandwidth).field("BurstSize", &self.BurstSize).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_FLOWRATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxBandwidth == other.MaxBandwidth && self.MaxPeakBandwidth == other.MaxPeakBandwidth && self.BurstSize == other.BurstSize
    }
}
impl ::std::cmp::Eq for HTTP_FLOWRATE_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_FLOWRATE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_HEADER_ID(pub i32);
pub const HttpHeaderCacheControl: HTTP_HEADER_ID = HTTP_HEADER_ID(0i32);
pub const HttpHeaderConnection: HTTP_HEADER_ID = HTTP_HEADER_ID(1i32);
pub const HttpHeaderDate: HTTP_HEADER_ID = HTTP_HEADER_ID(2i32);
pub const HttpHeaderKeepAlive: HTTP_HEADER_ID = HTTP_HEADER_ID(3i32);
pub const HttpHeaderPragma: HTTP_HEADER_ID = HTTP_HEADER_ID(4i32);
pub const HttpHeaderTrailer: HTTP_HEADER_ID = HTTP_HEADER_ID(5i32);
pub const HttpHeaderTransferEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(6i32);
pub const HttpHeaderUpgrade: HTTP_HEADER_ID = HTTP_HEADER_ID(7i32);
pub const HttpHeaderVia: HTTP_HEADER_ID = HTTP_HEADER_ID(8i32);
pub const HttpHeaderWarning: HTTP_HEADER_ID = HTTP_HEADER_ID(9i32);
pub const HttpHeaderAllow: HTTP_HEADER_ID = HTTP_HEADER_ID(10i32);
pub const HttpHeaderContentLength: HTTP_HEADER_ID = HTTP_HEADER_ID(11i32);
pub const HttpHeaderContentType: HTTP_HEADER_ID = HTTP_HEADER_ID(12i32);
pub const HttpHeaderContentEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(13i32);
pub const HttpHeaderContentLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(14i32);
pub const HttpHeaderContentLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(15i32);
pub const HttpHeaderContentMd5: HTTP_HEADER_ID = HTTP_HEADER_ID(16i32);
pub const HttpHeaderContentRange: HTTP_HEADER_ID = HTTP_HEADER_ID(17i32);
pub const HttpHeaderExpires: HTTP_HEADER_ID = HTTP_HEADER_ID(18i32);
pub const HttpHeaderLastModified: HTTP_HEADER_ID = HTTP_HEADER_ID(19i32);
pub const HttpHeaderAccept: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
pub const HttpHeaderAcceptCharset: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
pub const HttpHeaderAcceptEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
pub const HttpHeaderAcceptLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
pub const HttpHeaderAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
pub const HttpHeaderCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
pub const HttpHeaderExpect: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
pub const HttpHeaderFrom: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
pub const HttpHeaderHost: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
pub const HttpHeaderIfMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
pub const HttpHeaderIfModifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
pub const HttpHeaderIfNoneMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(31i32);
pub const HttpHeaderIfRange: HTTP_HEADER_ID = HTTP_HEADER_ID(32i32);
pub const HttpHeaderIfUnmodifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(33i32);
pub const HttpHeaderMaxForwards: HTTP_HEADER_ID = HTTP_HEADER_ID(34i32);
pub const HttpHeaderProxyAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(35i32);
pub const HttpHeaderReferer: HTTP_HEADER_ID = HTTP_HEADER_ID(36i32);
pub const HttpHeaderRange: HTTP_HEADER_ID = HTTP_HEADER_ID(37i32);
pub const HttpHeaderTe: HTTP_HEADER_ID = HTTP_HEADER_ID(38i32);
pub const HttpHeaderTranslate: HTTP_HEADER_ID = HTTP_HEADER_ID(39i32);
pub const HttpHeaderUserAgent: HTTP_HEADER_ID = HTTP_HEADER_ID(40i32);
pub const HttpHeaderRequestMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
pub const HttpHeaderAcceptRanges: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
pub const HttpHeaderAge: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
pub const HttpHeaderEtag: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
pub const HttpHeaderLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
pub const HttpHeaderProxyAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
pub const HttpHeaderRetryAfter: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
pub const HttpHeaderServer: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
pub const HttpHeaderSetCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
pub const HttpHeaderVary: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
pub const HttpHeaderWwwAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
pub const HttpHeaderResponseMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
pub const HttpHeaderMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
impl ::std::convert::From<i32> for HTTP_HEADER_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_HEADER_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_INITIALIZE(pub u32);
pub const HTTP_INITIALIZE_CONFIG: HTTP_INITIALIZE = HTTP_INITIALIZE(2u32);
pub const HTTP_INITIALIZE_SERVER: HTTP_INITIALIZE = HTTP_INITIALIZE(1u32);
impl ::std::convert::From<u32> for HTTP_INITIALIZE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_INITIALIZE {
    type Abi = Self;
}
impl ::std::ops::BitOr for HTTP_INITIALIZE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HTTP_INITIALIZE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HTTP_INITIALIZE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HTTP_INITIALIZE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HTTP_INITIALIZE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: u16,
    pub pRawValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_KNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_KNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_KNOWN_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_KNOWN_HEADER").field("RawValueLength", &self.RawValueLength).field("pRawValue", &self.pRawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_KNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RawValueLength == other.RawValueLength && self.pRawValue == other.pRawValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_KNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_KNOWN_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_LISTEN_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_LISTEN_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_LISTEN_ENDPOINT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_LISTEN_ENDPOINT_INFO").field("Flags", &self.Flags).field("EnableSharing", &self.EnableSharing).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_LISTEN_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EnableSharing == other.EnableSharing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_LISTEN_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_LISTEN_ENDPOINT_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Security`*"]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: u32,
    pub SoftwareName: super::super::Foundation::PWSTR,
    pub SoftwareNameLength: u16,
    pub DirectoryNameLength: u16,
    pub DirectoryName: super::super::Foundation::PWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: u32,
    pub pExtFields: *mut ::std::ffi::c_void,
    pub NumOfExtFields: u16,
    pub MaxRecordSize: u16,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl HTTP_LOGGING_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for HTTP_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for HTTP_LOGGING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_LOGGING_INFO")
            .field("Flags", &self.Flags)
            .field("LoggingFlags", &self.LoggingFlags)
            .field("SoftwareName", &self.SoftwareName)
            .field("SoftwareNameLength", &self.SoftwareNameLength)
            .field("DirectoryNameLength", &self.DirectoryNameLength)
            .field("DirectoryName", &self.DirectoryName)
            .field("Format", &self.Format)
            .field("Fields", &self.Fields)
            .field("pExtFields", &self.pExtFields)
            .field("NumOfExtFields", &self.NumOfExtFields)
            .field("MaxRecordSize", &self.MaxRecordSize)
            .field("RolloverType", &self.RolloverType)
            .field("RolloverSize", &self.RolloverSize)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for HTTP_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.LoggingFlags == other.LoggingFlags
            && self.SoftwareName == other.SoftwareName
            && self.SoftwareNameLength == other.SoftwareNameLength
            && self.DirectoryNameLength == other.DirectoryNameLength
            && self.DirectoryName == other.DirectoryName
            && self.Format == other.Format
            && self.Fields == other.Fields
            && self.pExtFields == other.pExtFields
            && self.NumOfExtFields == other.NumOfExtFields
            && self.MaxRecordSize == other.MaxRecordSize
            && self.RolloverType == other.RolloverType
            && self.RolloverSize == other.RolloverSize
            && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for HTTP_LOGGING_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for HTTP_LOGGING_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_LOGGING_ROLLOVER_TYPE(pub i32);
pub const HttpLoggingRolloverSize: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(0i32);
pub const HttpLoggingRolloverDaily: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(1i32);
pub const HttpLoggingRolloverWeekly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(2i32);
pub const HttpLoggingRolloverMonthly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(3i32);
pub const HttpLoggingRolloverHourly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(4i32);
impl ::std::convert::From<i32> for HTTP_LOGGING_ROLLOVER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_LOGGING_ROLLOVER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_LOGGING_TYPE(pub i32);
pub const HttpLoggingTypeW3C: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(0i32);
pub const HttpLoggingTypeIIS: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(1i32);
pub const HttpLoggingTypeNCSA: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(2i32);
pub const HttpLoggingTypeRaw: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(3i32);
impl ::std::convert::From<i32> for HTTP_LOGGING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_LOGGING_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
impl HTTP_LOG_DATA {}
impl ::std::default::Default for HTTP_LOG_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_LOG_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_LOG_DATA").field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_LOG_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::std::cmp::Eq for HTTP_LOG_DATA {}
unsafe impl ::windows::runtime::Abi for HTTP_LOG_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_LOG_DATA_TYPE(pub i32);
pub const HttpLogDataTypeFields: HTTP_LOG_DATA_TYPE = HTTP_LOG_DATA_TYPE(0i32);
impl ::std::convert::From<i32> for HTTP_LOG_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_LOG_DATA_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_LOG_FIELDS_DATA {
    pub Base: HTTP_LOG_DATA,
    pub UserNameLength: u16,
    pub UriStemLength: u16,
    pub ClientIpLength: u16,
    pub ServerNameLength: u16,
    pub ServiceNameLength: u16,
    pub ServerIpLength: u16,
    pub MethodLength: u16,
    pub UriQueryLength: u16,
    pub HostLength: u16,
    pub UserAgentLength: u16,
    pub CookieLength: u16,
    pub ReferrerLength: u16,
    pub UserName: super::super::Foundation::PWSTR,
    pub UriStem: super::super::Foundation::PWSTR,
    pub ClientIp: super::super::Foundation::PSTR,
    pub ServerName: super::super::Foundation::PSTR,
    pub ServiceName: super::super::Foundation::PSTR,
    pub ServerIp: super::super::Foundation::PSTR,
    pub Method: super::super::Foundation::PSTR,
    pub UriQuery: super::super::Foundation::PSTR,
    pub Host: super::super::Foundation::PSTR,
    pub UserAgent: super::super::Foundation::PSTR,
    pub Cookie: super::super::Foundation::PSTR,
    pub Referrer: super::super::Foundation::PSTR,
    pub ServerPort: u16,
    pub ProtocolStatus: u16,
    pub Win32Status: u32,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_LOG_FIELDS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_LOG_FIELDS_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_LOG_FIELDS_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_LOG_FIELDS_DATA")
            .field("Base", &self.Base)
            .field("UserNameLength", &self.UserNameLength)
            .field("UriStemLength", &self.UriStemLength)
            .field("ClientIpLength", &self.ClientIpLength)
            .field("ServerNameLength", &self.ServerNameLength)
            .field("ServiceNameLength", &self.ServiceNameLength)
            .field("ServerIpLength", &self.ServerIpLength)
            .field("MethodLength", &self.MethodLength)
            .field("UriQueryLength", &self.UriQueryLength)
            .field("HostLength", &self.HostLength)
            .field("UserAgentLength", &self.UserAgentLength)
            .field("CookieLength", &self.CookieLength)
            .field("ReferrerLength", &self.ReferrerLength)
            .field("UserName", &self.UserName)
            .field("UriStem", &self.UriStem)
            .field("ClientIp", &self.ClientIp)
            .field("ServerName", &self.ServerName)
            .field("ServiceName", &self.ServiceName)
            .field("ServerIp", &self.ServerIp)
            .field("Method", &self.Method)
            .field("UriQuery", &self.UriQuery)
            .field("Host", &self.Host)
            .field("UserAgent", &self.UserAgent)
            .field("Cookie", &self.Cookie)
            .field("Referrer", &self.Referrer)
            .field("ServerPort", &self.ServerPort)
            .field("ProtocolStatus", &self.ProtocolStatus)
            .field("Win32Status", &self.Win32Status)
            .field("MethodNum", &self.MethodNum)
            .field("SubStatus", &self.SubStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_LOG_FIELDS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base
            && self.UserNameLength == other.UserNameLength
            && self.UriStemLength == other.UriStemLength
            && self.ClientIpLength == other.ClientIpLength
            && self.ServerNameLength == other.ServerNameLength
            && self.ServiceNameLength == other.ServiceNameLength
            && self.ServerIpLength == other.ServerIpLength
            && self.MethodLength == other.MethodLength
            && self.UriQueryLength == other.UriQueryLength
            && self.HostLength == other.HostLength
            && self.UserAgentLength == other.UserAgentLength
            && self.CookieLength == other.CookieLength
            && self.ReferrerLength == other.ReferrerLength
            && self.UserName == other.UserName
            && self.UriStem == other.UriStem
            && self.ClientIp == other.ClientIp
            && self.ServerName == other.ServerName
            && self.ServiceName == other.ServiceName
            && self.ServerIp == other.ServerIp
            && self.Method == other.Method
            && self.UriQuery == other.UriQuery
            && self.Host == other.Host
            && self.UserAgent == other.UserAgent
            && self.Cookie == other.Cookie
            && self.Referrer == other.Referrer
            && self.ServerPort == other.ServerPort
            && self.ProtocolStatus == other.ProtocolStatus
            && self.Win32Status == other.Win32Status
            && self.MethodNum == other.MethodNum
            && self.SubStatus == other.SubStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_LOG_FIELDS_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_LOG_FIELDS_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_BYTES_RECV: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_BYTES_SENT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_CLIENT_IP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_CLIENT_PORT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_COMPUTER_NAME: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_COOKIE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_CORRELATION_ID: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_DATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_HOST: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_METHOD: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_QUEUE_NAME: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_REASON: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_REFERER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_SERVER_IP: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_SERVER_PORT: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_SITE_ID: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_SITE_NAME: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_STATUS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_STREAM_ID: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_STREAM_ID_EX: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_SUB_STATUS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_TIME: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_TIME_TAKEN: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_URI: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_URI_QUERY: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_URI_STEM: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_USER_AGENT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_USER_NAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_VERSION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOG_FIELD_WIN32_STATUS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: u32,
    pub KnownHeaderCount: u16,
    pub KnownHeaders: *mut HTTP_KNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_MULTIPLE_KNOWN_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_MULTIPLE_KNOWN_HEADERS").field("HeaderId", &self.HeaderId).field("Flags", &self.Flags).field("KnownHeaderCount", &self.KnownHeaderCount).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderId == other.HeaderId && self.Flags == other.Flags && self.KnownHeaderCount == other.KnownHeaderCount && self.KnownHeaders == other.KnownHeaders
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_MULTIPLE_KNOWN_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_MULTIPLE_KNOWN_HEADERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_PERFORMANCE_PARAM {
    pub Type: HTTP_PERFORMANCE_PARAM_TYPE,
    pub BufferSize: u32,
    pub Buffer: *mut ::std::ffi::c_void,
}
impl HTTP_PERFORMANCE_PARAM {}
impl ::std::default::Default for HTTP_PERFORMANCE_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_PERFORMANCE_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_PERFORMANCE_PARAM").field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_PERFORMANCE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::std::cmp::Eq for HTTP_PERFORMANCE_PARAM {}
unsafe impl ::windows::runtime::Abi for HTTP_PERFORMANCE_PARAM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_PERFORMANCE_PARAM_TYPE(pub i32);
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(0i32);
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(1i32);
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(2i32);
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(3i32);
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(4i32);
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(5i32);
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(6i32);
impl ::std::convert::From<i32> for HTTP_PERFORMANCE_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_PERFORMANCE_PARAM_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
impl HTTP_PROPERTY_FLAGS {}
impl ::std::default::Default for HTTP_PROPERTY_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_PROPERTY_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_PROPERTY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_PROPERTY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for HTTP_PROPERTY_FLAGS {}
unsafe impl ::windows::runtime::Abi for HTTP_PROPERTY_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
impl HTTP_PROTECTION_LEVEL_INFO {}
impl ::std::default::Default for HTTP_PROTECTION_LEVEL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_PROTECTION_LEVEL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_PROTECTION_LEVEL_INFO").field("Flags", &self.Flags).field("Level", &self.Level).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_PROTECTION_LEVEL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Level == other.Level
    }
}
impl ::std::cmp::Eq for HTTP_PROTECTION_LEVEL_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_PROTECTION_LEVEL_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_PROTECTION_LEVEL_TYPE(pub i32);
pub const HttpProtectionLevelUnrestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(0i32);
pub const HttpProtectionLevelEdgeRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(1i32);
pub const HttpProtectionLevelRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(2i32);
impl ::std::convert::From<i32> for HTTP_PROTECTION_LEVEL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_PROTECTION_LEVEL_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: *mut ::std::ffi::c_void,
}
impl HTTP_QOS_SETTING_INFO {}
impl ::std::default::Default for HTTP_QOS_SETTING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QOS_SETTING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QOS_SETTING_INFO").field("QosType", &self.QosType).field("QosSetting", &self.QosSetting).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QOS_SETTING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.QosType == other.QosType && self.QosSetting == other.QosSetting
    }
}
impl ::std::cmp::Eq for HTTP_QOS_SETTING_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_QOS_SETTING_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_QOS_SETTING_TYPE(pub i32);
pub const HttpQosSettingTypeBandwidth: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(0i32);
pub const HttpQosSettingTypeConnectionLimit: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(1i32);
pub const HttpQosSettingTypeFlowRate: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(2i32);
impl ::std::convert::From<i32> for HTTP_QOS_SETTING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_QOS_SETTING_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: u64,
}
impl HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
impl ::std::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_QUIC").field("Freshness", &self.Freshness).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn eq(&self, other: &Self) -> bool {
        self.Freshness == other.Freshness
    }
}
impl ::std::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
unsafe impl ::windows::runtime::Abi for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: u64,
}
impl HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
impl ::std::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_TCP").field("Freshness", &self.Freshness).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn eq(&self, other: &Self) -> bool {
        self.Freshness == other.Freshness
    }
}
impl ::std::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
unsafe impl ::windows::runtime::Abi for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QUIC_API_TIMINGS {
    pub ConnectionTimings: HTTP_QUIC_CONNECTION_API_TIMINGS,
    pub StreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
impl HTTP_QUIC_API_TIMINGS {}
impl ::std::default::Default for HTTP_QUIC_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QUIC_API_TIMINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QUIC_API_TIMINGS").field("ConnectionTimings", &self.ConnectionTimings).field("StreamTimings", &self.StreamTimings).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QUIC_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionTimings == other.ConnectionTimings && self.StreamTimings == other.StreamTimings
    }
}
impl ::std::cmp::Eq for HTTP_QUIC_API_TIMINGS {}
unsafe impl ::windows::runtime::Abi for HTTP_QUIC_API_TIMINGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QUIC_CONNECTION_API_TIMINGS {
    pub OpenTime: u64,
    pub CloseTime: u64,
    pub StartTime: u64,
    pub ShutdownTime: u64,
    pub SecConfigCreateTime: u64,
    pub SecConfigDeleteTime: u64,
    pub GetParamCount: u64,
    pub GetParamSum: u64,
    pub SetParamCount: u64,
    pub SetParamSum: u64,
    pub SetCallbackHandlerCount: u64,
    pub SetCallbackHandlerSum: u64,
    pub ControlStreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
impl HTTP_QUIC_CONNECTION_API_TIMINGS {}
impl ::std::default::Default for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QUIC_CONNECTION_API_TIMINGS")
            .field("OpenTime", &self.OpenTime)
            .field("CloseTime", &self.CloseTime)
            .field("StartTime", &self.StartTime)
            .field("ShutdownTime", &self.ShutdownTime)
            .field("SecConfigCreateTime", &self.SecConfigCreateTime)
            .field("SecConfigDeleteTime", &self.SecConfigDeleteTime)
            .field("GetParamCount", &self.GetParamCount)
            .field("GetParamSum", &self.GetParamSum)
            .field("SetParamCount", &self.SetParamCount)
            .field("SetParamSum", &self.SetParamSum)
            .field("SetCallbackHandlerCount", &self.SetCallbackHandlerCount)
            .field("SetCallbackHandlerSum", &self.SetCallbackHandlerSum)
            .field("ControlStreamTimings", &self.ControlStreamTimings)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.OpenTime == other.OpenTime
            && self.CloseTime == other.CloseTime
            && self.StartTime == other.StartTime
            && self.ShutdownTime == other.ShutdownTime
            && self.SecConfigCreateTime == other.SecConfigCreateTime
            && self.SecConfigDeleteTime == other.SecConfigDeleteTime
            && self.GetParamCount == other.GetParamCount
            && self.GetParamSum == other.GetParamSum
            && self.SetParamCount == other.SetParamCount
            && self.SetParamSum == other.SetParamSum
            && self.SetCallbackHandlerCount == other.SetCallbackHandlerCount
            && self.SetCallbackHandlerSum == other.SetCallbackHandlerSum
            && self.ControlStreamTimings == other.ControlStreamTimings
    }
}
impl ::std::cmp::Eq for HTTP_QUIC_CONNECTION_API_TIMINGS {}
unsafe impl ::windows::runtime::Abi for HTTP_QUIC_CONNECTION_API_TIMINGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_QUIC_STREAM_API_TIMINGS {
    pub OpenCount: u64,
    pub OpenSum: u64,
    pub CloseCount: u64,
    pub CloseSum: u64,
    pub StartCount: u64,
    pub StartSum: u64,
    pub ShutdownCount: u64,
    pub ShutdownSum: u64,
    pub SendCount: u64,
    pub SendSum: u64,
    pub ReceiveSetEnabledCount: u64,
    pub ReceiveSetEnabledSum: u64,
    pub GetParamCount: u64,
    pub GetParamSum: u64,
    pub SetParamCount: u64,
    pub SetParamSum: u64,
    pub SetCallbackHandlerCount: u64,
    pub SetCallbackHandlerSum: u64,
}
impl HTTP_QUIC_STREAM_API_TIMINGS {}
impl ::std::default::Default for HTTP_QUIC_STREAM_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_QUIC_STREAM_API_TIMINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_QUIC_STREAM_API_TIMINGS")
            .field("OpenCount", &self.OpenCount)
            .field("OpenSum", &self.OpenSum)
            .field("CloseCount", &self.CloseCount)
            .field("CloseSum", &self.CloseSum)
            .field("StartCount", &self.StartCount)
            .field("StartSum", &self.StartSum)
            .field("ShutdownCount", &self.ShutdownCount)
            .field("ShutdownSum", &self.ShutdownSum)
            .field("SendCount", &self.SendCount)
            .field("SendSum", &self.SendSum)
            .field("ReceiveSetEnabledCount", &self.ReceiveSetEnabledCount)
            .field("ReceiveSetEnabledSum", &self.ReceiveSetEnabledSum)
            .field("GetParamCount", &self.GetParamCount)
            .field("GetParamSum", &self.GetParamSum)
            .field("SetParamCount", &self.SetParamCount)
            .field("SetParamSum", &self.SetParamSum)
            .field("SetCallbackHandlerCount", &self.SetCallbackHandlerCount)
            .field("SetCallbackHandlerSum", &self.SetCallbackHandlerSum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_QUIC_STREAM_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.OpenCount == other.OpenCount
            && self.OpenSum == other.OpenSum
            && self.CloseCount == other.CloseCount
            && self.CloseSum == other.CloseSum
            && self.StartCount == other.StartCount
            && self.StartSum == other.StartSum
            && self.ShutdownCount == other.ShutdownCount
            && self.ShutdownSum == other.ShutdownSum
            && self.SendCount == other.SendCount
            && self.SendSum == other.SendSum
            && self.ReceiveSetEnabledCount == other.ReceiveSetEnabledCount
            && self.ReceiveSetEnabledSum == other.ReceiveSetEnabledSum
            && self.GetParamCount == other.GetParamCount
            && self.GetParamSum == other.GetParamSum
            && self.SetParamCount == other.SetParamCount
            && self.SetParamSum == other.SetParamSum
            && self.SetCallbackHandlerCount == other.SetCallbackHandlerCount
            && self.SetCallbackHandlerSum == other.SetCallbackHandlerSum
    }
}
impl ::std::cmp::Eq for HTTP_QUIC_STREAM_API_TIMINGS {}
unsafe impl ::windows::runtime::Abi for HTTP_QUIC_STREAM_API_TIMINGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_RECEIVE_HTTP_REQUEST_FLAGS(pub u32);
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(1u32);
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(2u32);
impl ::std::convert::From<u32> for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: i32,
    pub Flags: u32,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: super::super::Foundation::HANDLE,
    pub ContextAttributes: u32,
    pub PackedContextLength: u32,
    pub PackedContextType: u32,
    pub PackedContext: *mut ::std::ffi::c_void,
    pub MutualAuthDataLength: u32,
    pub pMutualAuthData: super::super::Foundation::PSTR,
    pub PackageNameLength: u16,
    pub pPackageName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_REQUEST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_REQUEST_AUTH_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_AUTH_INFO")
            .field("AuthStatus", &self.AuthStatus)
            .field("SecStatus", &self.SecStatus)
            .field("Flags", &self.Flags)
            .field("AuthType", &self.AuthType)
            .field("AccessToken", &self.AccessToken)
            .field("ContextAttributes", &self.ContextAttributes)
            .field("PackedContextLength", &self.PackedContextLength)
            .field("PackedContextType", &self.PackedContextType)
            .field("PackedContext", &self.PackedContext)
            .field("MutualAuthDataLength", &self.MutualAuthDataLength)
            .field("pMutualAuthData", &self.pMutualAuthData)
            .field("PackageNameLength", &self.PackageNameLength)
            .field("pPackageName", &self.pPackageName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_REQUEST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuthStatus == other.AuthStatus
            && self.SecStatus == other.SecStatus
            && self.Flags == other.Flags
            && self.AuthType == other.AuthType
            && self.AccessToken == other.AccessToken
            && self.ContextAttributes == other.ContextAttributes
            && self.PackedContextLength == other.PackedContextLength
            && self.PackedContextType == other.PackedContextType
            && self.PackedContext == other.PackedContext
            && self.MutualAuthDataLength == other.MutualAuthDataLength
            && self.pMutualAuthData == other.pMutualAuthData
            && self.PackageNameLength == other.PackageNameLength
            && self.pPackageName == other.pPackageName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_AUTH_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_REQUEST_AUTH_TYPE(pub i32);
pub const HttpRequestAuthTypeNone: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(0i32);
pub const HttpRequestAuthTypeBasic: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(1i32);
pub const HttpRequestAuthTypeDigest: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(2i32);
pub const HttpRequestAuthTypeNTLM: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(3i32);
pub const HttpRequestAuthTypeNegotiate: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(4i32);
pub const HttpRequestAuthTypeKerberos: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(5i32);
impl ::std::convert::From<i32> for HTTP_REQUEST_AUTH_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_AUTH_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: *mut HTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: *mut u8,
    pub ChannelTokenSize: u32,
    pub Flags: u32,
}
impl HTTP_REQUEST_CHANNEL_BIND_STATUS {}
impl ::std::default::Default for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_CHANNEL_BIND_STATUS").field("ServiceName", &self.ServiceName).field("ChannelToken", &self.ChannelToken).field("ChannelTokenSize", &self.ChannelTokenSize).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceName == other.ServiceName && self.ChannelToken == other.ChannelToken && self.ChannelTokenSize == other.ChannelTokenSize && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_CHANNEL_BIND_STATUS {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_REQUEST_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_REQUEST_HEADERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_REQUEST_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnknownHeaderCount == other.UnknownHeaderCount && self.pUnknownHeaders == other.pUnknownHeaders && self.TrailerCount == other.TrailerCount && self.pTrailers == other.pTrailers && self.KnownHeaders == other.KnownHeaders
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_REQUEST_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_HEADERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: u32,
    pub pInfo: *mut ::std::ffi::c_void,
}
impl HTTP_REQUEST_INFO {}
impl ::std::default::Default for HTTP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_INFO").field("InfoType", &self.InfoType).field("InfoLength", &self.InfoLength).field("pInfo", &self.pInfo).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InfoType == other.InfoType && self.InfoLength == other.InfoLength && self.pInfo == other.pInfo
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_REQUEST_INFO_TYPE(pub i32);
pub const HttpRequestInfoTypeAuth: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(0i32);
pub const HttpRequestInfoTypeChannelBind: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(1i32);
pub const HttpRequestInfoTypeSslProtocol: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(2i32);
pub const HttpRequestInfoTypeSslTokenBindingDraft: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(3i32);
pub const HttpRequestInfoTypeSslTokenBinding: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(4i32);
pub const HttpRequestInfoTypeRequestTiming: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(5i32);
pub const HttpRequestInfoTypeTcpInfoV0: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(6i32);
pub const HttpRequestInfoTypeRequestSizing: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(7i32);
pub const HttpRequestInfoTypeQuicStats: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(8i32);
pub const HttpRequestInfoTypeTcpInfoV1: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(9i32);
impl ::std::convert::From<i32> for HTTP_REQUEST_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_REQUEST_PROPERTY(pub i32);
pub const HttpRequestPropertyIsb: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(0i32);
pub const HttpRequestPropertyTcpInfoV0: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(1i32);
pub const HttpRequestPropertyQuicStats: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(2i32);
pub const HttpRequestPropertyTcpInfoV1: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(3i32);
pub const HttpRequestPropertySni: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(4i32);
pub const HttpRequestPropertyStreamError: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(5i32);
pub const HttpRequestPropertyWskApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(6i32);
pub const HttpRequestPropertyQuicApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(7i32);
impl ::std::convert::From<i32> for HTTP_REQUEST_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_PROPERTY_SNI {
    pub Hostname: [u16; 256],
    pub Flags: u32,
}
impl HTTP_REQUEST_PROPERTY_SNI {}
impl ::std::default::Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_PROPERTY_SNI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_PROPERTY_SNI").field("Hostname", &self.Hostname).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_PROPERTY_SNI {
    fn eq(&self, other: &Self) -> bool {
        self.Hostname == other.Hostname && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_PROPERTY_SNI {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_PROPERTY_SNI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: u32,
}
impl HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
impl ::std::default::Default for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_PROPERTY_STREAM_ERROR").field("ErrorCode", &self.ErrorCode).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_SIZING_INFO {
    pub Flags: u64,
    pub RequestIndex: u32,
    pub RequestSizingCount: u32,
    pub RequestSizing: [u64; 5],
}
impl HTTP_REQUEST_SIZING_INFO {}
impl ::std::default::Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_SIZING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_SIZING_INFO").field("Flags", &self.Flags).field("RequestIndex", &self.RequestIndex).field("RequestSizingCount", &self.RequestSizingCount).field("RequestSizing", &self.RequestSizing).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_SIZING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequestIndex == other.RequestIndex && self.RequestSizingCount == other.RequestSizingCount && self.RequestSizing == other.RequestSizing
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_SIZING_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_SIZING_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_REQUEST_SIZING_TYPE(pub i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(0i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(1i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(2i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(3i32);
pub const HttpRequestSizingTypeHeaders: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(4i32);
pub const HttpRequestSizingTypeMax: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(5i32);
impl ::std::convert::From<i32> for HTTP_REQUEST_SIZING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_SIZING_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_TIMING_INFO {
    pub RequestTimingCount: u32,
    pub RequestTiming: [u64; 30],
}
impl HTTP_REQUEST_TIMING_INFO {}
impl ::std::default::Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_TIMING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_TIMING_INFO").field("RequestTimingCount", &self.RequestTimingCount).field("RequestTiming", &self.RequestTiming).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_TIMING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RequestTimingCount == other.RequestTimingCount && self.RequestTiming == other.RequestTiming
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_TIMING_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_TIMING_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_REQUEST_TIMING_TYPE(pub i32);
pub const HttpRequestTimingTypeConnectionStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(0i32);
pub const HttpRequestTimingTypeDataStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(1i32);
pub const HttpRequestTimingTypeTlsCertificateLoadStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(2i32);
pub const HttpRequestTimingTypeTlsCertificateLoadEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(3i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg1Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(4i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg1End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(5i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg2Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(6i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg2End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(7i32);
pub const HttpRequestTimingTypeTlsAttributesQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(8i32);
pub const HttpRequestTimingTypeTlsAttributesQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(9i32);
pub const HttpRequestTimingTypeTlsClientCertQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(10i32);
pub const HttpRequestTimingTypeTlsClientCertQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(11i32);
pub const HttpRequestTimingTypeHttp2StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(12i32);
pub const HttpRequestTimingTypeHttp2HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(13i32);
pub const HttpRequestTimingTypeHttp2HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(14i32);
pub const HttpRequestTimingTypeRequestHeaderParseStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(15i32);
pub const HttpRequestTimingTypeRequestHeaderParseEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(16i32);
pub const HttpRequestTimingTypeRequestRoutingStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(17i32);
pub const HttpRequestTimingTypeRequestRoutingEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(18i32);
pub const HttpRequestTimingTypeRequestQueuedForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(19i32);
pub const HttpRequestTimingTypeRequestDeliveredForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(20i32);
pub const HttpRequestTimingTypeRequestReturnedAfterInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(21i32);
pub const HttpRequestTimingTypeRequestQueuedForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(22i32);
pub const HttpRequestTimingTypeRequestDeliveredForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(23i32);
pub const HttpRequestTimingTypeRequestReturnedAfterDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(24i32);
pub const HttpRequestTimingTypeRequestQueuedForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(25i32);
pub const HttpRequestTimingTypeRequestDeliveredForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(26i32);
pub const HttpRequestTimingTypeHttp3StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(27i32);
pub const HttpRequestTimingTypeHttp3HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(28i32);
pub const HttpRequestTimingTypeHttp3HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(29i32);
pub const HttpRequestTimingTypeMax: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(30i32);
impl ::std::convert::From<i32> for HTTP_REQUEST_TIMING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_TIMING_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO {
    pub TokenBinding: *mut u8,
    pub TokenBindingSize: u32,
    pub EKM: *mut u8,
    pub EKMSize: u32,
    pub KeyType: u8,
}
impl HTTP_REQUEST_TOKEN_BINDING_INFO {}
impl ::std::default::Default for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_TOKEN_BINDING_INFO").field("TokenBinding", &self.TokenBinding).field("TokenBindingSize", &self.TokenBindingSize).field("EKM", &self.EKM).field("EKMSize", &self.EKMSize).field("KeyType", &self.KeyType).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TokenBinding == other.TokenBinding && self.TokenBindingSize == other.TokenBindingSize && self.EKM == other.EKM && self.EKMSize == other.EKMSize && self.KeyType == other.KeyType
    }
}
impl ::std::cmp::Eq for HTTP_REQUEST_TOKEN_BINDING_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_TOKEN_BINDING_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_REQUEST_V1 {
    pub Flags: u32,
    pub ConnectionId: u64,
    pub RequestId: u64,
    pub UrlContext: u64,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: u16,
    pub RawUrlLength: u16,
    pub pUnknownVerb: super::super::Foundation::PSTR,
    pub pRawUrl: super::super::Foundation::PSTR,
    pub CookedUrl: HTTP_COOKED_URL,
    pub Address: HTTP_TRANSPORT_ADDRESS,
    pub Headers: HTTP_REQUEST_HEADERS,
    pub BytesReceived: u64,
    pub EntityChunkCount: u16,
    pub pEntityChunks: *mut HTTP_DATA_CHUNK,
    pub RawConnectionId: u64,
    pub pSslInfo: *mut HTTP_SSL_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_REQUEST_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_V1")
            .field("Flags", &self.Flags)
            .field("ConnectionId", &self.ConnectionId)
            .field("RequestId", &self.RequestId)
            .field("UrlContext", &self.UrlContext)
            .field("Version", &self.Version)
            .field("Verb", &self.Verb)
            .field("UnknownVerbLength", &self.UnknownVerbLength)
            .field("RawUrlLength", &self.RawUrlLength)
            .field("pUnknownVerb", &self.pUnknownVerb)
            .field("pRawUrl", &self.pRawUrl)
            .field("CookedUrl", &self.CookedUrl)
            .field("Address", &self.Address)
            .field("Headers", &self.Headers)
            .field("BytesReceived", &self.BytesReceived)
            .field("EntityChunkCount", &self.EntityChunkCount)
            .field("pEntityChunks", &self.pEntityChunks)
            .field("RawConnectionId", &self.RawConnectionId)
            .field("pSslInfo", &self.pSslInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_REQUEST_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.ConnectionId == other.ConnectionId
            && self.RequestId == other.RequestId
            && self.UrlContext == other.UrlContext
            && self.Version == other.Version
            && self.Verb == other.Verb
            && self.UnknownVerbLength == other.UnknownVerbLength
            && self.RawUrlLength == other.RawUrlLength
            && self.pUnknownVerb == other.pUnknownVerb
            && self.pRawUrl == other.pRawUrl
            && self.CookedUrl == other.CookedUrl
            && self.Address == other.Address
            && self.Headers == other.Headers
            && self.BytesReceived == other.BytesReceived
            && self.EntityChunkCount == other.EntityChunkCount
            && self.pEntityChunks == other.pEntityChunks
            && self.RawConnectionId == other.RawConnectionId
            && self.pSslInfo == other.pSslInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_REQUEST_V2 {
    pub __AnonymousBase_http_L1861_C35: HTTP_REQUEST_V1,
    pub RequestInfoCount: u16,
    pub pRequestInfo: *mut HTTP_REQUEST_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_REQUEST_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_REQUEST_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_V2").field("__AnonymousBase_http_L1861_C35", &self.__AnonymousBase_http_L1861_C35).field("RequestInfoCount", &self.RequestInfoCount).field("pRequestInfo", &self.pRequestInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_http_L1861_C35 == other.__AnonymousBase_http_L1861_C35 && self.RequestInfoCount == other.RequestInfoCount && self.pRequestInfo == other.pRequestInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_REQUEST_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_REQUEST_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_RESPONSE_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_RESPONSE_HEADERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_RESPONSE_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_RESPONSE_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnknownHeaderCount == other.UnknownHeaderCount && self.pUnknownHeaders == other.pUnknownHeaders && self.TrailerCount == other.TrailerCount && self.pTrailers == other.pTrailers && self.KnownHeaders == other.KnownHeaders
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_RESPONSE_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_RESPONSE_HEADERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: u32,
    pub pInfo: *mut ::std::ffi::c_void,
}
impl HTTP_RESPONSE_INFO {}
impl ::std::default::Default for HTTP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_RESPONSE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_RESPONSE_INFO").field("Type", &self.Type).field("Length", &self.Length).field("pInfo", &self.pInfo).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Length == other.Length && self.pInfo == other.pInfo
    }
}
impl ::std::cmp::Eq for HTTP_RESPONSE_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_RESPONSE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_RESPONSE_INFO_TYPE(pub i32);
pub const HttpResponseInfoTypeMultipleKnownHeaders: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(0i32);
pub const HttpResponseInfoTypeAuthenticationProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(1i32);
pub const HttpResponseInfoTypeQoSProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(2i32);
pub const HttpResponseInfoTypeChannelBind: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(3i32);
impl ::std::convert::From<i32> for HTTP_RESPONSE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_RESPONSE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: u32,
    pub Version: HTTP_VERSION,
    pub StatusCode: u16,
    pub ReasonLength: u16,
    pub pReason: super::super::Foundation::PSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: u16,
    pub pEntityChunks: *mut HTTP_DATA_CHUNK,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_RESPONSE_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_RESPONSE_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_RESPONSE_V1")
            .field("Flags", &self.Flags)
            .field("Version", &self.Version)
            .field("StatusCode", &self.StatusCode)
            .field("ReasonLength", &self.ReasonLength)
            .field("pReason", &self.pReason)
            .field("Headers", &self.Headers)
            .field("EntityChunkCount", &self.EntityChunkCount)
            .field("pEntityChunks", &self.pEntityChunks)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_RESPONSE_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Version == other.Version && self.StatusCode == other.StatusCode && self.ReasonLength == other.ReasonLength && self.pReason == other.pReason && self.Headers == other.Headers && self.EntityChunkCount == other.EntityChunkCount && self.pEntityChunks == other.pEntityChunks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_RESPONSE_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_RESPONSE_V2 {
    pub __AnonymousBase_http_L2050_C36: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: u16,
    pub pResponseInfo: *mut HTTP_RESPONSE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_RESPONSE_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_RESPONSE_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_RESPONSE_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_RESPONSE_V2").field("__AnonymousBase_http_L2050_C36", &self.__AnonymousBase_http_L2050_C36).field("ResponseInfoCount", &self.ResponseInfoCount).field("pResponseInfo", &self.pResponseInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_RESPONSE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_http_L2050_C36 == other.__AnonymousBase_http_L2050_C36 && self.ResponseInfoCount == other.ResponseInfoCount && self.pResponseInfo == other.pResponseInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_RESPONSE_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_RESPONSE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SCHEME(pub i32);
pub const HttpSchemeHttp: HTTP_SCHEME = HTTP_SCHEME(0i32);
pub const HttpSchemeHttps: HTTP_SCHEME = HTTP_SCHEME(1i32);
pub const HttpSchemeMaximum: HTTP_SCHEME = HTTP_SCHEME(2i32);
impl ::std::convert::From<i32> for HTTP_SCHEME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SCHEME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: u16,
    pub Realm: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS").field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.RealmLength == other.RealmLength && self.Realm == other.Realm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: u16,
    pub DomainName: super::super::Foundation::PWSTR,
    pub RealmLength: u16,
    pub Realm: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS").field("DomainNameLength", &self.DomainNameLength).field("DomainName", &self.DomainName).field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.DomainNameLength == other.DomainNameLength && self.DomainName == other.DomainName && self.RealmLength == other.RealmLength && self.Realm == other.Realm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVER_AUTHENTICATION_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub AuthSchemes: u32,
    pub ReceiveMutualAuth: super::super::Foundation::BOOLEAN,
    pub ReceiveContextHandle: super::super::Foundation::BOOLEAN,
    pub DisableNTLMCredentialCaching: super::super::Foundation::BOOLEAN,
    pub ExFlags: u8,
    pub DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    pub BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVER_AUTHENTICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVER_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVER_AUTHENTICATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVER_AUTHENTICATION_INFO")
            .field("Flags", &self.Flags)
            .field("AuthSchemes", &self.AuthSchemes)
            .field("ReceiveMutualAuth", &self.ReceiveMutualAuth)
            .field("ReceiveContextHandle", &self.ReceiveContextHandle)
            .field("DisableNTLMCredentialCaching", &self.DisableNTLMCredentialCaching)
            .field("ExFlags", &self.ExFlags)
            .field("DigestParams", &self.DigestParams)
            .field("BasicParams", &self.BasicParams)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.AuthSchemes == other.AuthSchemes && self.ReceiveMutualAuth == other.ReceiveMutualAuth && self.ReceiveContextHandle == other.ReceiveContextHandle && self.DisableNTLMCredentialCaching == other.DisableNTLMCredentialCaching && self.ExFlags == other.ExFlags && self.DigestParams == other.DigestParams && self.BasicParams == other.BasicParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVER_AUTHENTICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVER_AUTHENTICATION_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVER_PROPERTY(pub i32);
pub const HttpServerAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(0i32);
pub const HttpServerLoggingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(1i32);
pub const HttpServerQosProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(2i32);
pub const HttpServerTimeoutsProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(3i32);
pub const HttpServerQueueLengthProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(4i32);
pub const HttpServerStateProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(5i32);
pub const HttpServer503VerbosityProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(6i32);
pub const HttpServerBindingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(7i32);
pub const HttpServerExtendedAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(8i32);
pub const HttpServerListenEndpointProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(9i32);
pub const HttpServerChannelBindProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(10i32);
pub const HttpServerProtectionLevelProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(11i32);
pub const HttpServerDelegationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(16i32);
impl ::std::convert::From<i32> for HTTP_SERVER_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVER_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::super::Foundation::PSTR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_BINDING_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_BINDING_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_BINDING_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_BINDING_A").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_BINDING_A {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_BINDING_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_BINDING_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
impl HTTP_SERVICE_BINDING_BASE {}
impl ::std::default::Default for HTTP_SERVICE_BINDING_BASE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_SERVICE_BINDING_BASE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_BINDING_BASE").field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_SERVICE_BINDING_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::std::cmp::Eq for HTTP_SERVICE_BINDING_BASE {}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_BINDING_BASE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_BINDING_TYPE(pub i32);
pub const HttpServiceBindingTypeNone: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(0i32);
pub const HttpServiceBindingTypeW: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(1i32);
pub const HttpServiceBindingTypeA: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(2i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_BINDING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_BINDING_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::super::Foundation::PWSTR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_BINDING_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_BINDING_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_BINDING_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_BINDING_W").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_BINDING_W {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_BINDING_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_BINDING_W {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_CACHE_KEY(pub i32);
pub const MaxCacheResponseSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(0i32);
pub const CacheRangeChunkSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(1i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_CACHE_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: u32,
}
impl HTTP_SERVICE_CONFIG_CACHE_SET {}
impl ::std::default::Default for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_CACHE_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_CACHE_SET {}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_CACHE_SET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_ID(pub i32);
pub const HttpServiceConfigIPListenList: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(0i32);
pub const HttpServiceConfigSSLCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(1i32);
pub const HttpServiceConfigUrlAclInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(2i32);
pub const HttpServiceConfigTimeout: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(3i32);
pub const HttpServiceConfigCache: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(4i32);
pub const HttpServiceConfigSslSniCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(5i32);
pub const HttpServiceConfigSslCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(6i32);
pub const HttpServiceConfigSetting: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(7i32);
pub const HttpServiceConfigSslCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(8i32);
pub const HttpServiceConfigSslSniCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(9i32);
pub const HttpServiceConfigSslCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(10i32);
pub const HttpServiceConfigSslScopedCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(11i32);
pub const HttpServiceConfigSslScopedCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(12i32);
pub const HttpServiceConfigMax: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(13i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_CONFIG_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: u16,
    pub pAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM").field("AddrLength", &self.AddrLength).field("pAddress", &self.pAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.AddrLength == other.AddrLength && self.pAddress == other.pAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: u32,
    pub AddrList: [super::WinSock::SOCKADDR_STORAGE; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY").field("AddrCount", &self.AddrCount).field("AddrList", &self.AddrList).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.AddrCount == other.AddrCount && self.AddrList == other.AddrList
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_QUERY_TYPE(pub i32);
pub const HttpServiceConfigQueryExact: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(0i32);
pub const HttpServiceConfigQueryNext: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(1i32);
pub const HttpServiceConfigQueryMax: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(2i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_SETTING_KEY(pub i32);
pub const HttpNone: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(0i32);
pub const HttpTlsThrottle: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(1i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SETTING_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SETTING_KEY,
    pub ParamDesc: u32,
}
impl HTTP_SERVICE_CONFIG_SETTING_SET {}
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SETTING_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SETTING_SET {}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SETTING_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_CCS_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_KEY").field("LocalAddress", &self.LocalAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddress == other.LocalAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_CCS_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY").field("pIpPort", &self.pIpPort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pIpPort == other.pIpPort
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_KEY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY_EX").field("IpPort", &self.IpPort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.IpPort == other.IpPort
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: u32,
    pub pSslHash: *mut ::std::ffi::c_void,
    pub AppId: ::windows::runtime::GUID,
    pub pSslCertStoreName: super::super::Foundation::PWSTR,
    pub DefaultCertCheckMode: u32,
    pub DefaultRevocationFreshnessTime: u32,
    pub DefaultRevocationUrlRetrievalTimeout: u32,
    pub pDefaultSslCtlIdentifier: super::super::Foundation::PWSTR,
    pub pDefaultSslCtlStoreName: super::super::Foundation::PWSTR,
    pub DefaultFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_SSL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_PARAM")
            .field("SslHashLength", &self.SslHashLength)
            .field("pSslHash", &self.pSslHash)
            .field("AppId", &self.AppId)
            .field("pSslCertStoreName", &self.pSslCertStoreName)
            .field("DefaultCertCheckMode", &self.DefaultCertCheckMode)
            .field("DefaultRevocationFreshnessTime", &self.DefaultRevocationFreshnessTime)
            .field("DefaultRevocationUrlRetrievalTimeout", &self.DefaultRevocationUrlRetrievalTimeout)
            .field("pDefaultSslCtlIdentifier", &self.pDefaultSslCtlIdentifier)
            .field("pDefaultSslCtlStoreName", &self.pDefaultSslCtlStoreName)
            .field("DefaultFlags", &self.DefaultFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.SslHashLength == other.SslHashLength
            && self.pSslHash == other.pSslHash
            && self.AppId == other.AppId
            && self.pSslCertStoreName == other.pSslCertStoreName
            && self.DefaultCertCheckMode == other.DefaultCertCheckMode
            && self.DefaultRevocationFreshnessTime == other.DefaultRevocationFreshnessTime
            && self.DefaultRevocationUrlRetrievalTimeout == other.DefaultRevocationUrlRetrievalTimeout
            && self.pDefaultSslCtlIdentifier == other.pDefaultSslCtlIdentifier
            && self.pDefaultSslCtlStoreName == other.pDefaultSslCtlStoreName
            && self.DefaultFlags == other.DefaultFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: u64,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_SSL_PARAM_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub union HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    pub Http2WindowSizeParam: HTTP2_WINDOW_SIZE_PARAM,
    pub Http2SettingsLimitsParam: HTTP2_SETTINGS_LIMITS_PARAM,
    pub HttpPerformanceParam: HTTP_PERFORMANCE_PARAM,
    pub HttpTlsRestrictionsParam: HTTP_TLS_RESTRICTIONS_PARAM,
    pub HttpErrorHeadersParam: HTTP_ERROR_HEADERS_PARAM,
    pub HttpTlsSessionTicketKeysParam: HTTP_TLS_SESSION_TICKET_KEYS_PARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_QUERY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
    pub Host: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_KEY").field("IpPort", &self.IpPort).field("Host", &self.Host).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.IpPort == other.IpPort && self.Host == other.Host
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SNI_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_KEY(pub i32);
pub const IdleConnectionTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(0i32);
pub const HeaderWaitTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(1i32);
impl ::std::convert::From<i32> for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: u16,
}
impl HTTP_SERVICE_CONFIG_TIMEOUT_SET {}
impl ::std::default::Default for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_TIMEOUT_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {}
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_URLACL_KEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_URLACL_KEY").field("pUrlPrefix", &self.pUrlPrefix).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pUrlPrefix == other.pUrlPrefix
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_KEY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_URLACL_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_URLACL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_URLACL_PARAM").field("pStringSecurityDescriptor", &self.pStringSecurityDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.pStringSecurityDescriptor == other.pStringSecurityDescriptor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_URLACL_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_URLACL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_QUERY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SERVICE_CONFIG_URLACL_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SERVICE_CONFIG_URLACL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_SET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SERVICE_CONFIG_URLACL_SET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: u32,
    pub CertEncodedSize: u32,
    pub pCertEncoded: *mut u8,
    pub Token: super::super::Foundation::HANDLE,
    pub CertDeniedByMapper: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SSL_CLIENT_CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SSL_CLIENT_CERT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SSL_CLIENT_CERT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SSL_CLIENT_CERT_INFO").field("CertFlags", &self.CertFlags).field("CertEncodedSize", &self.CertEncodedSize).field("pCertEncoded", &self.pCertEncoded).field("Token", &self.Token).field("CertDeniedByMapper", &self.CertDeniedByMapper).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SSL_CLIENT_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CertFlags == other.CertFlags && self.CertEncodedSize == other.CertEncodedSize && self.pCertEncoded == other.pCertEncoded && self.Token == other.Token && self.CertDeniedByMapper == other.CertDeniedByMapper
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SSL_CLIENT_CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SSL_CLIENT_CERT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: u16,
    pub ConnectionKeySize: u16,
    pub ServerCertIssuerSize: u32,
    pub ServerCertSubjectSize: u32,
    pub pServerCertIssuer: super::super::Foundation::PSTR,
    pub pServerCertSubject: super::super::Foundation::PSTR,
    pub pClientCertInfo: *mut HTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_SSL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_SSL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SSL_INFO")
            .field("ServerCertKeySize", &self.ServerCertKeySize)
            .field("ConnectionKeySize", &self.ConnectionKeySize)
            .field("ServerCertIssuerSize", &self.ServerCertIssuerSize)
            .field("ServerCertSubjectSize", &self.ServerCertSubjectSize)
            .field("pServerCertIssuer", &self.pServerCertIssuer)
            .field("pServerCertSubject", &self.pServerCertSubject)
            .field("pClientCertInfo", &self.pClientCertInfo)
            .field("SslClientCertNegotiated", &self.SslClientCertNegotiated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_SSL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ServerCertKeySize == other.ServerCertKeySize && self.ConnectionKeySize == other.ConnectionKeySize && self.ServerCertIssuerSize == other.ServerCertIssuerSize && self.ServerCertSubjectSize == other.ServerCertSubjectSize && self.pServerCertIssuer == other.pServerCertIssuer && self.pServerCertSubject == other.pServerCertSubject && self.pClientCertInfo == other.pClientCertInfo && self.SslClientCertNegotiated == other.SslClientCertNegotiated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_SSL_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_SSL_PROTOCOL_INFO {
    pub Protocol: u32,
    pub CipherType: u32,
    pub CipherStrength: u32,
    pub HashType: u32,
    pub HashStrength: u32,
    pub KeyExchangeType: u32,
    pub KeyExchangeStrength: u32,
}
impl HTTP_SSL_PROTOCOL_INFO {}
impl ::std::default::Default for HTTP_SSL_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_SSL_PROTOCOL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_SSL_PROTOCOL_INFO")
            .field("Protocol", &self.Protocol)
            .field("CipherType", &self.CipherType)
            .field("CipherStrength", &self.CipherStrength)
            .field("HashType", &self.HashType)
            .field("HashStrength", &self.HashStrength)
            .field("KeyExchangeType", &self.KeyExchangeType)
            .field("KeyExchangeStrength", &self.KeyExchangeStrength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_SSL_PROTOCOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Protocol == other.Protocol && self.CipherType == other.CipherType && self.CipherStrength == other.CipherStrength && self.HashType == other.HashType && self.HashStrength == other.HashStrength && self.KeyExchangeType == other.KeyExchangeType && self.KeyExchangeStrength == other.KeyExchangeStrength
    }
}
impl ::std::cmp::Eq for HTTP_SSL_PROTOCOL_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_SSL_PROTOCOL_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(pub i32);
pub const ExParamTypeHttp2Window: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(0i32);
pub const ExParamTypeHttp2SettingsLimits: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(1i32);
pub const ExParamTypeHttpPerformance: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(2i32);
pub const ExParamTypeTlsRestrictions: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(3i32);
pub const ExParamTypeErrorHeaders: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(4i32);
pub const ExParamTypeTlsSessionTicketKeys: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(5i32);
pub const ExParamTypeMax: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(6i32);
impl ::std::convert::From<i32> for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
impl HTTP_STATE_INFO {}
impl ::std::default::Default for HTTP_STATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_STATE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_STATE_INFO").field("Flags", &self.Flags).field("State", &self.State).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.State == other.State
    }
}
impl ::std::cmp::Eq for HTTP_STATE_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_STATE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: u16,
    pub DrainEntityBody: u16,
    pub RequestQueue: u16,
    pub IdleConnection: u16,
    pub HeaderWait: u16,
    pub MinSendRate: u32,
}
impl HTTP_TIMEOUT_LIMIT_INFO {}
impl ::std::default::Default for HTTP_TIMEOUT_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_TIMEOUT_LIMIT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TIMEOUT_LIMIT_INFO")
            .field("Flags", &self.Flags)
            .field("EntityBody", &self.EntityBody)
            .field("DrainEntityBody", &self.DrainEntityBody)
            .field("RequestQueue", &self.RequestQueue)
            .field("IdleConnection", &self.IdleConnection)
            .field("HeaderWait", &self.HeaderWait)
            .field("MinSendRate", &self.MinSendRate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_TIMEOUT_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EntityBody == other.EntityBody && self.DrainEntityBody == other.DrainEntityBody && self.RequestQueue == other.RequestQueue && self.IdleConnection == other.IdleConnection && self.HeaderWait == other.HeaderWait && self.MinSendRate == other.MinSendRate
    }
}
impl ::std::cmp::Eq for HTTP_TIMEOUT_LIMIT_INFO {}
unsafe impl ::windows::runtime::Abi for HTTP_TIMEOUT_LIMIT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_TLS_RESTRICTIONS_PARAM {
    pub RestrictionCount: u32,
    pub TlsRestrictions: *mut ::std::ffi::c_void,
}
impl HTTP_TLS_RESTRICTIONS_PARAM {}
impl ::std::default::Default for HTTP_TLS_RESTRICTIONS_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_TLS_RESTRICTIONS_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TLS_RESTRICTIONS_PARAM").field("RestrictionCount", &self.RestrictionCount).field("TlsRestrictions", &self.TlsRestrictions).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_TLS_RESTRICTIONS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.RestrictionCount == other.RestrictionCount && self.TlsRestrictions == other.TlsRestrictions
    }
}
impl ::std::cmp::Eq for HTTP_TLS_RESTRICTIONS_PARAM {}
unsafe impl ::windows::runtime::Abi for HTTP_TLS_RESTRICTIONS_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    pub SessionTicketKeyCount: u32,
    pub SessionTicketKeys: *mut ::std::ffi::c_void,
}
impl HTTP_TLS_SESSION_TICKET_KEYS_PARAM {}
impl ::std::default::Default for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TLS_SESSION_TICKET_KEYS_PARAM").field("SessionTicketKeyCount", &self.SessionTicketKeyCount).field("SessionTicketKeys", &self.SessionTicketKeys).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.SessionTicketKeyCount == other.SessionTicketKeyCount && self.SessionTicketKeys == other.SessionTicketKeys
    }
}
impl ::std::cmp::Eq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {}
unsafe impl ::windows::runtime::Abi for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: *mut super::WinSock::SOCKADDR,
    pub pLocalAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl HTTP_TRANSPORT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for HTTP_TRANSPORT_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for HTTP_TRANSPORT_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TRANSPORT_ADDRESS").field("pRemoteAddress", &self.pRemoteAddress).field("pLocalAddress", &self.pLocalAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for HTTP_TRANSPORT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.pRemoteAddress == other.pRemoteAddress && self.pLocalAddress == other.pLocalAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for HTTP_TRANSPORT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for HTTP_TRANSPORT_ADDRESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: u16,
    pub RawValueLength: u16,
    pub pName: super::super::Foundation::PSTR,
    pub pRawValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_UNKNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_UNKNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_UNKNOWN_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_UNKNOWN_HEADER").field("NameLength", &self.NameLength).field("RawValueLength", &self.RawValueLength).field("pName", &self.pName).field("pRawValue", &self.pRawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_UNKNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NameLength == other.NameLength && self.RawValueLength == other.RawValueLength && self.pName == other.pName && self.pRawValue == other.pRawValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_UNKNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_UNKNOWN_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_VERB(pub i32);
pub const HttpVerbUnparsed: HTTP_VERB = HTTP_VERB(0i32);
pub const HttpVerbUnknown: HTTP_VERB = HTTP_VERB(1i32);
pub const HttpVerbInvalid: HTTP_VERB = HTTP_VERB(2i32);
pub const HttpVerbOPTIONS: HTTP_VERB = HTTP_VERB(3i32);
pub const HttpVerbGET: HTTP_VERB = HTTP_VERB(4i32);
pub const HttpVerbHEAD: HTTP_VERB = HTTP_VERB(5i32);
pub const HttpVerbPOST: HTTP_VERB = HTTP_VERB(6i32);
pub const HttpVerbPUT: HTTP_VERB = HTTP_VERB(7i32);
pub const HttpVerbDELETE: HTTP_VERB = HTTP_VERB(8i32);
pub const HttpVerbTRACE: HTTP_VERB = HTTP_VERB(9i32);
pub const HttpVerbCONNECT: HTTP_VERB = HTTP_VERB(10i32);
pub const HttpVerbTRACK: HTTP_VERB = HTTP_VERB(11i32);
pub const HttpVerbMOVE: HTTP_VERB = HTTP_VERB(12i32);
pub const HttpVerbCOPY: HTTP_VERB = HTTP_VERB(13i32);
pub const HttpVerbPROPFIND: HTTP_VERB = HTTP_VERB(14i32);
pub const HttpVerbPROPPATCH: HTTP_VERB = HTTP_VERB(15i32);
pub const HttpVerbMKCOL: HTTP_VERB = HTTP_VERB(16i32);
pub const HttpVerbLOCK: HTTP_VERB = HTTP_VERB(17i32);
pub const HttpVerbUNLOCK: HTTP_VERB = HTTP_VERB(18i32);
pub const HttpVerbSEARCH: HTTP_VERB = HTTP_VERB(19i32);
pub const HttpVerbMaximum: HTTP_VERB = HTTP_VERB(20i32);
impl ::std::convert::From<i32> for HTTP_VERB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_VERB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl HTTP_VERSION {}
impl ::std::default::Default for HTTP_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::std::cmp::Eq for HTTP_VERSION {}
unsafe impl ::windows::runtime::Abi for HTTP_VERSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub struct HTTP_WSK_API_TIMINGS {
    pub ConnectCount: u64,
    pub ConnectSum: u64,
    pub DisconnectCount: u64,
    pub DisconnectSum: u64,
    pub SendCount: u64,
    pub SendSum: u64,
    pub ReceiveCount: u64,
    pub ReceiveSum: u64,
    pub ReleaseCount: u64,
    pub ReleaseSum: u64,
    pub ControlSocketCount: u64,
    pub ControlSocketSum: u64,
}
impl HTTP_WSK_API_TIMINGS {}
impl ::std::default::Default for HTTP_WSK_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_WSK_API_TIMINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_WSK_API_TIMINGS")
            .field("ConnectCount", &self.ConnectCount)
            .field("ConnectSum", &self.ConnectSum)
            .field("DisconnectCount", &self.DisconnectCount)
            .field("DisconnectSum", &self.DisconnectSum)
            .field("SendCount", &self.SendCount)
            .field("SendSum", &self.SendSum)
            .field("ReceiveCount", &self.ReceiveCount)
            .field("ReceiveSum", &self.ReceiveSum)
            .field("ReleaseCount", &self.ReleaseCount)
            .field("ReleaseSum", &self.ReleaseSum)
            .field("ControlSocketCount", &self.ControlSocketCount)
            .field("ControlSocketSum", &self.ControlSocketSum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_WSK_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectCount == other.ConnectCount
            && self.ConnectSum == other.ConnectSum
            && self.DisconnectCount == other.DisconnectCount
            && self.DisconnectSum == other.DisconnectSum
            && self.SendCount == other.SendCount
            && self.SendSum == other.SendSum
            && self.ReceiveCount == other.ReceiveCount
            && self.ReceiveSum == other.ReceiveSum
            && self.ReleaseCount == other.ReleaseCount
            && self.ReleaseSum == other.ReleaseSum
            && self.ControlSocketCount == other.ControlSocketCount
            && self.ControlSocketSum == other.ControlSocketSum
    }
}
impl ::std::cmp::Eq for HTTP_WSK_API_TIMINGS {}
unsafe impl ::windows::runtime::Abi for HTTP_WSK_API_TIMINGS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpAddFragmentToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddFragmentToCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpAddFragmentToCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::std::mem::transmute(datachunk), ::std::mem::transmute(cachepolicy), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpAddUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(requestqueuehandle: Param0, fullyqualifiedurl: Param1, reserved: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR, reserved: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(HttpAddUrl(requestqueuehandle.into_param().abi(), fullyqualifiedurl.into_param().abi(), ::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpAddUrlToUrlGroup<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(urlgroupid: u64, pfullyqualifiedurl: Param1, urlcontext: u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddUrlToUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, urlcontext: u64, reserved: u32) -> u32;
        }
        ::std::mem::transmute(HttpAddUrlToUrlGroup(::std::mem::transmute(urlgroupid), pfullyqualifiedurl.into_param().abi(), ::std::mem::transmute(urlcontext), ::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpCancelHttpRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCancelHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpCancelHttpRequest(requestqueuehandle.into_param().abi(), ::std::mem::transmute(requestid), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpCloseRequestQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(HttpCloseRequestQueue(requestqueuehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpCloseServerSession(serversessionid: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseServerSession(serversessionid: u64) -> u32;
        }
        ::std::mem::transmute(HttpCloseServerSession(::std::mem::transmute(serversessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpCloseUrlGroup(urlgroupid: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseUrlGroup(urlgroupid: u64) -> u32;
        }
        ::std::mem::transmute(HttpCloseUrlGroup(::std::mem::transmute(urlgroupid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32;
        }
        ::std::mem::transmute(HttpCreateHttpHandle(::std::mem::transmute(requestqueuehandle), ::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn HttpCreateRequestQueue<'a, Param0: ::windows::runtime::IntoParam<'a, HTTPAPI_VERSION>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(version: Param0, name: Param1, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateRequestQueue(version: HTTPAPI_VERSION, name: super::super::Foundation::PWSTR, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(HttpCreateRequestQueue(version.into_param().abi(), name.into_param().abi(), ::std::mem::transmute(securityattributes), ::std::mem::transmute(flags), ::std::mem::transmute(requestqueuehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpCreateServerSession<'a, Param0: ::windows::runtime::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, serversessionid: *mut u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateServerSession(version: HTTPAPI_VERSION, serversessionid: *mut u64, reserved: u32) -> u32;
        }
        ::std::mem::transmute(HttpCreateServerSession(version.into_param().abi(), ::std::mem::transmute(serversessionid), ::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32;
        }
        ::std::mem::transmute(HttpCreateUrlGroup(::std::mem::transmute(serversessionid), ::std::mem::transmute(purlgroupid), ::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpDeclarePush<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(requestqueuehandle: Param0, requestid: u64, verb: HTTP_VERB, path: Param3, query: Param4, headers: *const HTTP_REQUEST_HEADERS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDeclarePush(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, verb: HTTP_VERB, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PSTR, headers: *const HTTP_REQUEST_HEADERS) -> u32;
        }
        ::std::mem::transmute(HttpDeclarePush(requestqueuehandle.into_param().abi(), ::std::mem::transmute(requestid), ::std::mem::transmute(verb), path.into_param().abi(), query.into_param().abi(), ::std::mem::transmute(headers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpDelegateRequestEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, delegatequeuehandle: Param1, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDelegateRequestEx(requestqueuehandle: super::super::Foundation::HANDLE, delegatequeuehandle: super::super::Foundation::HANDLE, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32;
        }
        ::std::mem::transmute(HttpDelegateRequestEx(requestqueuehandle.into_param().abi(), delegatequeuehandle.into_param().abi(), ::std::mem::transmute(requestid), ::std::mem::transmute(delegateurlgroupid), ::std::mem::transmute(propertyinfosetsize), ::std::mem::transmute(propertyinfoset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpDeleteServiceConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::std::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDeleteServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::std::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpDeleteServiceConfiguration(servicehandle.into_param().abi(), ::std::mem::transmute(configid), ::std::mem::transmute(pconfiginformation), ::std::mem::transmute(configinformationlength), ::std::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpFindUrlGroupId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(fullyqualifiedurl: Param0, requestqueuehandle: Param1, urlgroupid: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFindUrlGroupId(fullyqualifiedurl: super::super::Foundation::PWSTR, requestqueuehandle: super::super::Foundation::HANDLE, urlgroupid: *mut u64) -> u32;
        }
        ::std::mem::transmute(HttpFindUrlGroupId(fullyqualifiedurl.into_param().abi(), requestqueuehandle.into_param().abi(), ::std::mem::transmute(urlgroupid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpFlushResponseCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFlushResponseCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpFlushResponseCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpGetExtension<'a, Param0: ::windows::runtime::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, extension: u32, buffer: *mut ::std::ffi::c_void, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpGetExtension(version: HTTPAPI_VERSION, extension: u32, buffer: *mut ::std::ffi::c_void, buffersize: u32) -> u32;
        }
        ::std::mem::transmute(HttpGetExtension(version.into_param().abi(), ::std::mem::transmute(extension), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpInitialize<'a, Param0: ::windows::runtime::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, flags: HTTP_INITIALIZE, preserved: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpInitialize(version: HTTPAPI_VERSION, flags: HTTP_INITIALIZE, preserved: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(HttpInitialize(version.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HttpIsFeatureSupported(::std::mem::transmute(featureid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpPrepareUrl<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(reserved: *mut ::std::ffi::c_void, flags: u32, url: Param2, preparedurl: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpPrepareUrl(reserved: *mut ::std::ffi::c_void, flags: u32, url: super::super::Foundation::PWSTR, preparedurl: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(HttpPrepareUrl(::std::mem::transmute(reserved), ::std::mem::transmute(flags), url.into_param().abi(), ::std::mem::transmute(preparedurl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpQueryRequestQueueProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(HttpQueryRequestQueueProperty(requestqueuehandle.into_param().abi(), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength), ::std::mem::transmute(reserved1), ::std::mem::transmute(returnlength), ::std::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(HttpQueryServerSessionProperty(::std::mem::transmute(serversessionid), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpQueryServiceConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::std::ffi::c_void, inputlength: u32, poutput: *mut ::std::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::std::ffi::c_void, inputlength: u32, poutput: *mut ::std::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpQueryServiceConfiguration(
            servicehandle.into_param().abi(),
            ::std::mem::transmute(configid),
            ::std::mem::transmute(pinput),
            ::std::mem::transmute(inputlength),
            ::std::mem::transmute(poutput),
            ::std::mem::transmute(outputlength),
            ::std::mem::transmute(preturnlength),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::std::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(HttpQueryUrlGroupProperty(::std::mem::transmute(urlgroupid), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpReadFragmentFromCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::std::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReadFragmentFromCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::std::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpReadFragmentFromCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::std::mem::transmute(byterange), ::std::mem::transmute(buffer), ::std::mem::transmute(bufferlength), ::std::mem::transmute(bytesread), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpReceiveClientCertificate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveClientCertificate(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpReceiveClientCertificate(requestqueuehandle.into_param().abi(), ::std::mem::transmute(connectionid), ::std::mem::transmute(flags), ::std::mem::transmute(sslclientcertinfo), ::std::mem::transmute(sslclientcertinfosize), ::std::mem::transmute(bytesreceived), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpReceiveHttpRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpReceiveHttpRequest(requestqueuehandle.into_param().abi(), ::std::mem::transmute(requestid), ::std::mem::transmute(flags), ::std::mem::transmute(requestbuffer), ::std::mem::transmute(requestbufferlength), ::std::mem::transmute(bytesreturned), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpReceiveRequestEntityBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, entitybuffer: *mut ::std::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveRequestEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitybuffer: *mut ::std::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpReceiveRequestEntityBody(requestqueuehandle.into_param().abi(), ::std::mem::transmute(requestid), ::std::mem::transmute(flags), ::std::mem::transmute(entitybuffer), ::std::mem::transmute(entitybufferlength), ::std::mem::transmute(bytesreturned), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpRemoveUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(requestqueuehandle: Param0, fullyqualifiedurl: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpRemoveUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(HttpRemoveUrl(requestqueuehandle.into_param().abi(), fullyqualifiedurl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpRemoveUrlFromUrlGroup<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(urlgroupid: u64, pfullyqualifiedurl: Param1, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpRemoveUrlFromUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::std::mem::transmute(HttpRemoveUrlFromUrlGroup(::std::mem::transmute(urlgroupid), pfullyqualifiedurl.into_param().abi(), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpSendHttpResponse<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::std::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendHttpResponse(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::std::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
        }
        ::std::mem::transmute(HttpSendHttpResponse(
            requestqueuehandle.into_param().abi(),
            ::std::mem::transmute(requestid),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(httpresponse),
            ::std::mem::transmute(cachepolicy),
            ::std::mem::transmute(bytessent),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(reserved2),
            ::std::mem::transmute(overlapped),
            ::std::mem::transmute(logdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpSendResponseEntityBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, entitychunkcount: u16, entitychunks: *const HTTP_DATA_CHUNK, bytessent: *mut u32, reserved1: *mut ::std::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendResponseEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitychunkcount: u16, entitychunks: *const HTTP_DATA_CHUNK, bytessent: *mut u32, reserved1: *mut ::std::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
        }
        ::std::mem::transmute(HttpSendResponseEntityBody(
            requestqueuehandle.into_param().abi(),
            ::std::mem::transmute(requestid),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(entitychunkcount),
            ::std::mem::transmute(entitychunks),
            ::std::mem::transmute(bytessent),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(reserved2),
            ::std::mem::transmute(overlapped),
            ::std::mem::transmute(logdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpSetRequestProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::std::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetRequestProperty(requestqueuehandle: super::super::Foundation::HANDLE, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::std::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpSetRequestProperty(requestqueuehandle.into_param().abi(), ::std::mem::transmute(id), ::std::mem::transmute(propertyid), ::std::mem::transmute(input), ::std::mem::transmute(inputpropertysize), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpSetRequestQueueProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(HttpSetRequestQueueProperty(requestqueuehandle.into_param().abi(), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength), ::std::mem::transmute(reserved1), ::std::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32) -> u32;
        }
        ::std::mem::transmute(HttpSetServerSessionProperty(::std::mem::transmute(serversessionid), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpSetServiceConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::std::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::std::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpSetServiceConfiguration(servicehandle.into_param().abi(), ::std::mem::transmute(configid), ::std::mem::transmute(pconfiginformation), ::std::mem::transmute(configinformationlength), ::std::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::std::ffi::c_void, propertyinformationlength: u32) -> u32;
        }
        ::std::mem::transmute(HttpSetUrlGroupProperty(::std::mem::transmute(urlgroupid), ::std::mem::transmute(property), ::std::mem::transmute(propertyinformation), ::std::mem::transmute(propertyinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn HttpShutdownRequestQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpShutdownRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(HttpShutdownRequestQueue(requestqueuehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
#[inline]
pub unsafe fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(HttpTerminate(::std::mem::transmute(flags), ::std::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpUpdateServiceConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::std::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpUpdateServiceConfiguration(handle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::std::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpUpdateServiceConfiguration(handle.into_param().abi(), ::std::mem::transmute(configid), ::std::mem::transmute(configinfo), ::std::mem::transmute(configinfolength), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpWaitForDemandStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDemandStart(requestqueuehandle: super::super::Foundation::HANDLE, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpWaitForDemandStart(requestqueuehandle.into_param().abi(), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpWaitForDisconnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDisconnect(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpWaitForDisconnect(requestqueuehandle.into_param().abi(), ::std::mem::transmute(connectionid), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
#[inline]
pub unsafe fn HttpWaitForDisconnectEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDisconnectEx(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::std::mem::transmute(HttpWaitForDisconnectEx(requestqueuehandle.into_param().abi(), ::std::mem::transmute(connectionid), ::std::mem::transmute(reserved), ::std::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
