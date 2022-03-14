#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP2_SETTINGS_LIMITS_PARAM {
    pub Http2MaxSettingsPerFrame: u32,
    pub Http2MaxSettingsPerMinute: u32,
}
impl ::core::marker::Copy for HTTP2_SETTINGS_LIMITS_PARAM {}
impl ::core::clone::Clone for HTTP2_SETTINGS_LIMITS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP2_SETTINGS_LIMITS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP2_SETTINGS_LIMITS_PARAM").field("Http2MaxSettingsPerFrame", &self.Http2MaxSettingsPerFrame).field("Http2MaxSettingsPerMinute", &self.Http2MaxSettingsPerMinute).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP2_SETTINGS_LIMITS_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP2_SETTINGS_LIMITS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP2_SETTINGS_LIMITS_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP2_SETTINGS_LIMITS_PARAM {}
impl ::core::default::Default for HTTP2_SETTINGS_LIMITS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: u32,
}
impl ::core::marker::Copy for HTTP2_WINDOW_SIZE_PARAM {}
impl ::core::clone::Clone for HTTP2_WINDOW_SIZE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP2_WINDOW_SIZE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP2_WINDOW_SIZE_PARAM").field("Http2ReceiveWindowSize", &self.Http2ReceiveWindowSize).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP2_WINDOW_SIZE_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP2_WINDOW_SIZE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP2_WINDOW_SIZE_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP2_WINDOW_SIZE_PARAM {}
impl ::core::default::Default for HTTP2_WINDOW_SIZE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: u16,
    pub HttpApiMinorVersion: u16,
}
impl ::core::marker::Copy for HTTPAPI_VERSION {}
impl ::core::clone::Clone for HTTPAPI_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTPAPI_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTPAPI_VERSION").field("HttpApiMajorVersion", &self.HttpApiMajorVersion).field("HttpApiMinorVersion", &self.HttpApiMinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTPAPI_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTPAPI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTPAPI_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTPAPI_VERSION {}
impl ::core::default::Default for HTTPAPI_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_503_RESPONSE_VERBOSITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const Http503ResponseVerbosityBasic: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const Http503ResponseVerbosityLimited: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const Http503ResponseVerbosityFull: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(2i32);
impl ::core::marker::Copy for HTTP_503_RESPONSE_VERBOSITY {}
impl ::core::clone::Clone for HTTP_503_RESPONSE_VERBOSITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_503_RESPONSE_VERBOSITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_503_RESPONSE_VERBOSITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_503_RESPONSE_VERBOSITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_503_RESPONSE_VERBOSITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_AUTHENTICATION_HARDENING_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthenticationHardeningLegacy: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthenticationHardeningMedium: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthenticationHardeningStrict: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(2i32);
impl ::core::marker::Copy for HTTP_AUTHENTICATION_HARDENING_LEVELS {}
impl ::core::clone::Clone for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_AUTHENTICATION_HARDENING_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_AUTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthStatusSuccess: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthStatusNotAuthenticated: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpAuthStatusFailure: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(2i32);
impl ::core::marker::Copy for HTTP_AUTH_STATUS {}
impl ::core::clone::Clone for HTTP_AUTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_AUTH_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_AUTH_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
}
impl ::core::marker::Copy for HTTP_BANDWIDTH_LIMIT_INFO {}
impl ::core::clone::Clone for HTTP_BANDWIDTH_LIMIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_BANDWIDTH_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BANDWIDTH_LIMIT_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_BANDWIDTH_LIMIT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_BANDWIDTH_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_BANDWIDTH_LIMIT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_BANDWIDTH_LIMIT_INFO {}
impl ::core::default::Default for HTTP_BANDWIDTH_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_BINDING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_BINDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BINDING_INFO").field("Flags", &self.Flags).field("RequestQueueHandle", &self.RequestQueueHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_BINDING_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_BINDING_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_BINDING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for HTTP_BYTE_RANGE {}
impl ::core::clone::Clone for HTTP_BYTE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_BYTE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BYTE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_BYTE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_BYTE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_BYTE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_BYTE_RANGE {}
impl ::core::default::Default for HTTP_BYTE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: u32,
}
impl ::core::marker::Copy for HTTP_CACHE_POLICY {}
impl ::core::clone::Clone for HTTP_CACHE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_CACHE_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CACHE_POLICY").field("Policy", &self.Policy).field("SecondsToLive", &self.SecondsToLive).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_CACHE_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_CACHE_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_CACHE_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_CACHE_POLICY {}
impl ::core::default::Default for HTTP_CACHE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_CACHE_POLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpCachePolicyNocache: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpCachePolicyUserInvalidates: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpCachePolicyTimeToLive: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpCachePolicyMaximum: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(3i32);
impl ::core::marker::Copy for HTTP_CACHE_POLICY_TYPE {}
impl ::core::clone::Clone for HTTP_CACHE_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_CACHE_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_CACHE_POLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_CACHE_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_CACHE_POLICY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: u32,
    pub ServiceNames: *mut *mut HTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: u32,
}
impl ::core::marker::Copy for HTTP_CHANNEL_BIND_INFO {}
impl ::core::clone::Clone for HTTP_CHANNEL_BIND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_CHANNEL_BIND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CHANNEL_BIND_INFO").field("Hardening", &self.Hardening).field("Flags", &self.Flags).field("ServiceNames", &self.ServiceNames).field("NumberOfServiceNames", &self.NumberOfServiceNames).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_CHANNEL_BIND_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_CHANNEL_BIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_CHANNEL_BIND_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_CHANNEL_BIND_INFO {}
impl ::core::default::Default for HTTP_CHANNEL_BIND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: u32,
}
impl ::core::marker::Copy for HTTP_CONNECTION_LIMIT_INFO {}
impl ::core::clone::Clone for HTTP_CONNECTION_LIMIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_CONNECTION_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CONNECTION_LIMIT_INFO").field("Flags", &self.Flags).field("MaxConnections", &self.MaxConnections).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_CONNECTION_LIMIT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_CONNECTION_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_CONNECTION_LIMIT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_CONNECTION_LIMIT_INFO {}
impl ::core::default::Default for HTTP_CONNECTION_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: u16,
    pub HostLength: u16,
    pub AbsPathLength: u16,
    pub QueryStringLength: u16,
    pub pFullUrl: ::windows::core::PCWSTR,
    pub pHost: ::windows::core::PCWSTR,
    pub pAbsPath: ::windows::core::PCWSTR,
    pub pQueryString: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for HTTP_COOKED_URL {}
impl ::core::clone::Clone for HTTP_COOKED_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_COOKED_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_COOKED_URL").field("FullUrlLength", &self.FullUrlLength).field("HostLength", &self.HostLength).field("AbsPathLength", &self.AbsPathLength).field("QueryStringLength", &self.QueryStringLength).field("pFullUrl", &self.pFullUrl).field("pHost", &self.pHost).field("pAbsPath", &self.pAbsPath).field("pQueryString", &self.pQueryString).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_COOKED_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_COOKED_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_COOKED_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_COOKED_URL {}
impl ::core::default::Default for HTTP_COOKED_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const CreateRequestQueueExternalIdProperty: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const CreateRequestQueueMax: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(2i32);
impl ::core::marker::Copy for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {}
impl ::core::clone::Clone for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    pub PropertyId: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {}
impl ::core::clone::Clone for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {}
impl ::core::default::Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub Anonymous: HTTP_DATA_CHUNK_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union HTTP_DATA_CHUNK_0 {
    pub FromMemory: HTTP_DATA_CHUNK_0_3,
    pub FromFileHandle: HTTP_DATA_CHUNK_0_0,
    pub FromFragmentCache: HTTP_DATA_CHUNK_0_2,
    pub FromFragmentCacheEx: HTTP_DATA_CHUNK_0_1,
    pub Trailers: HTTP_DATA_CHUNK_0_4,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_0 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DATA_CHUNK_0_0").field("ByteRange", &self.ByteRange).field("FileHandle", &self.FileHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DATA_CHUNK_0_1").field("ByteRange", &self.ByteRange).field("pFragmentName", &self.pFragmentName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: u16,
    pub pFragmentName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DATA_CHUNK_0_2").field("FragmentNameLength", &self.FragmentNameLength).field("pFragmentName", &self.pFragmentName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_3 {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub BufferLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DATA_CHUNK_0_3").field("pBuffer", &self.pBuffer).field("BufferLength", &self.BufferLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_4 {
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DATA_CHUNK_0_4").field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_0_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_DATA_CHUNK_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DATA_CHUNK_0_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_DATA_CHUNK_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_DATA_CHUNK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkFromMemory: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkFromFileHandle: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkFromFragmentCache: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkFromFragmentCacheEx: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkTrailers: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpDataChunkMaximum: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(5i32);
impl ::core::marker::Copy for HTTP_DATA_CHUNK_TYPE {}
impl ::core::clone::Clone for HTTP_DATA_CHUNK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_DATA_CHUNK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_DATA_CHUNK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_DATA_CHUNK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const DelegateRequestReservedProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const DelegateRequestDelegateUrlProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(1i32);
impl ::core::marker::Copy for HTTP_DELEGATE_REQUEST_PROPERTY_ID {}
impl ::core::clone::Clone for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_DELEGATE_REQUEST_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    pub PropertyId: HTTP_DELEGATE_REQUEST_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {}
impl ::core::clone::Clone for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DELEGATE_REQUEST_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_DELEGATE_REQUEST_PROPERTY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {}
impl ::core::default::Default for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_DEMAND_CBT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_ENABLED_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpEnabledStateActive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpEnabledStateInactive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(1i32);
impl ::core::marker::Copy for HTTP_ENABLED_STATE {}
impl ::core::clone::Clone for HTTP_ENABLED_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_ENABLED_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_ENABLED_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: u16,
    pub HeaderCount: u16,
    pub Headers: *mut HTTP_UNKNOWN_HEADER,
}
impl ::core::marker::Copy for HTTP_ERROR_HEADERS_PARAM {}
impl ::core::clone::Clone for HTTP_ERROR_HEADERS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_ERROR_HEADERS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_ERROR_HEADERS_PARAM").field("StatusCode", &self.StatusCode).field("HeaderCount", &self.HeaderCount).field("Headers", &self.Headers).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_ERROR_HEADERS_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_ERROR_HEADERS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_ERROR_HEADERS_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_ERROR_HEADERS_PARAM {}
impl ::core::default::Default for HTTP_ERROR_HEADERS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_FEATURE_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeatureUnknown: HTTP_FEATURE_ID = HTTP_FEATURE_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeatureResponseTrailers: HTTP_FEATURE_ID = HTTP_FEATURE_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeatureApiTimings: HTTP_FEATURE_ID = HTTP_FEATURE_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeatureDelegateEx: HTTP_FEATURE_ID = HTTP_FEATURE_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeatureHttp3: HTTP_FEATURE_ID = HTTP_FEATURE_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpFeaturemax: HTTP_FEATURE_ID = HTTP_FEATURE_ID(-1i32);
impl ::core::marker::Copy for HTTP_FEATURE_ID {}
impl ::core::clone::Clone for HTTP_FEATURE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_FEATURE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_FEATURE_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
    pub MaxPeakBandwidth: u32,
    pub BurstSize: u32,
}
impl ::core::marker::Copy for HTTP_FLOWRATE_INFO {}
impl ::core::clone::Clone for HTTP_FLOWRATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FLOWRATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FLOWRATE_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).field("MaxPeakBandwidth", &self.MaxPeakBandwidth).field("BurstSize", &self.BurstSize).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FLOWRATE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FLOWRATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FLOWRATE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FLOWRATE_INFO {}
impl ::core::default::Default for HTTP_FLOWRATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_HEADER_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderCacheControl: HTTP_HEADER_ID = HTTP_HEADER_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderConnection: HTTP_HEADER_ID = HTTP_HEADER_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderDate: HTTP_HEADER_ID = HTTP_HEADER_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderKeepAlive: HTTP_HEADER_ID = HTTP_HEADER_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderPragma: HTTP_HEADER_ID = HTTP_HEADER_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderTrailer: HTTP_HEADER_ID = HTTP_HEADER_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderTransferEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderUpgrade: HTTP_HEADER_ID = HTTP_HEADER_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderVia: HTTP_HEADER_ID = HTTP_HEADER_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderWarning: HTTP_HEADER_ID = HTTP_HEADER_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAllow: HTTP_HEADER_ID = HTTP_HEADER_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentLength: HTTP_HEADER_ID = HTTP_HEADER_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentType: HTTP_HEADER_ID = HTTP_HEADER_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentMd5: HTTP_HEADER_ID = HTTP_HEADER_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderContentRange: HTTP_HEADER_ID = HTTP_HEADER_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderExpires: HTTP_HEADER_ID = HTTP_HEADER_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderLastModified: HTTP_HEADER_ID = HTTP_HEADER_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAccept: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAcceptCharset: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAcceptEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAcceptLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderExpect: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderFrom: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderHost: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderIfMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderIfModifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderIfNoneMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(31i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderIfRange: HTTP_HEADER_ID = HTTP_HEADER_ID(32i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderIfUnmodifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(33i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderMaxForwards: HTTP_HEADER_ID = HTTP_HEADER_ID(34i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderProxyAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(35i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderReferer: HTTP_HEADER_ID = HTTP_HEADER_ID(36i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderRange: HTTP_HEADER_ID = HTTP_HEADER_ID(37i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderTe: HTTP_HEADER_ID = HTTP_HEADER_ID(38i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderTranslate: HTTP_HEADER_ID = HTTP_HEADER_ID(39i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderUserAgent: HTTP_HEADER_ID = HTTP_HEADER_ID(40i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderRequestMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAcceptRanges: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderAge: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderEtag: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderProxyAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderRetryAfter: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderServer: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderSetCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderVary: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderWwwAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderResponseMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpHeaderMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
impl ::core::marker::Copy for HTTP_HEADER_ID {}
impl ::core::clone::Clone for HTTP_HEADER_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_HEADER_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_HEADER_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_HEADER_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_HEADER_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_INITIALIZE(pub u32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_INITIALIZE_CONFIG: HTTP_INITIALIZE = HTTP_INITIALIZE(2u32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_INITIALIZE_SERVER: HTTP_INITIALIZE = HTTP_INITIALIZE(1u32);
impl ::core::marker::Copy for HTTP_INITIALIZE {}
impl ::core::clone::Clone for HTTP_INITIALIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_INITIALIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_INITIALIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_INITIALIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_INITIALIZE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HTTP_INITIALIZE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HTTP_INITIALIZE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HTTP_INITIALIZE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HTTP_INITIALIZE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HTTP_INITIALIZE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: u16,
    pub pRawValue: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HTTP_KNOWN_HEADER {}
impl ::core::clone::Clone for HTTP_KNOWN_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_KNOWN_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_KNOWN_HEADER").field("RawValueLength", &self.RawValueLength).field("pRawValue", &self.pRawValue).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_KNOWN_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_KNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_KNOWN_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_KNOWN_HEADER {}
impl ::core::default::Default for HTTP_KNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_LISTEN_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_LISTEN_ENDPOINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_LISTEN_ENDPOINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LISTEN_ENDPOINT_INFO").field("Flags", &self.Flags).field("EnableSharing", &self.EnableSharing).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_LISTEN_ENDPOINT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_LISTEN_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_LISTEN_ENDPOINT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_LISTEN_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_LISTEN_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: u32,
    pub SoftwareName: ::windows::core::PCWSTR,
    pub SoftwareNameLength: u16,
    pub DirectoryNameLength: u16,
    pub DirectoryName: ::windows::core::PCWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: u32,
    pub pExtFields: *mut ::core::ffi::c_void,
    pub NumOfExtFields: u16,
    pub MaxRecordSize: u16,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for HTTP_LOGGING_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for HTTP_LOGGING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for HTTP_LOGGING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOGGING_INFO")
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
unsafe impl ::windows::core::Abi for HTTP_LOGGING_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for HTTP_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_LOGGING_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for HTTP_LOGGING_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for HTTP_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_LOGGING_ROLLOVER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingRolloverSize: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingRolloverDaily: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingRolloverWeekly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingRolloverMonthly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingRolloverHourly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(4i32);
impl ::core::marker::Copy for HTTP_LOGGING_ROLLOVER_TYPE {}
impl ::core::clone::Clone for HTTP_LOGGING_ROLLOVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_LOGGING_ROLLOVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_LOGGING_ROLLOVER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_LOGGING_ROLLOVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_ROLLOVER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_LOGGING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingTypeW3C: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingTypeIIS: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingTypeNCSA: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLoggingTypeRaw: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(3i32);
impl ::core::marker::Copy for HTTP_LOGGING_TYPE {}
impl ::core::clone::Clone for HTTP_LOGGING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_LOGGING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_LOGGING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_LOGGING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
impl ::core::marker::Copy for HTTP_LOG_DATA {}
impl ::core::clone::Clone for HTTP_LOG_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_LOG_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOG_DATA").field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_LOG_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_LOG_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_LOG_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_LOG_DATA {}
impl ::core::default::Default for HTTP_LOG_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_LOG_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpLogDataTypeFields: HTTP_LOG_DATA_TYPE = HTTP_LOG_DATA_TYPE(0i32);
impl ::core::marker::Copy for HTTP_LOG_DATA_TYPE {}
impl ::core::clone::Clone for HTTP_LOG_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_LOG_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_LOG_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_LOG_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOG_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
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
    pub UserName: ::windows::core::PWSTR,
    pub UriStem: ::windows::core::PWSTR,
    pub ClientIp: ::windows::core::PSTR,
    pub ServerName: ::windows::core::PSTR,
    pub ServiceName: ::windows::core::PSTR,
    pub ServerIp: ::windows::core::PSTR,
    pub Method: ::windows::core::PSTR,
    pub UriQuery: ::windows::core::PSTR,
    pub Host: ::windows::core::PSTR,
    pub UserAgent: ::windows::core::PSTR,
    pub Cookie: ::windows::core::PSTR,
    pub Referrer: ::windows::core::PSTR,
    pub ServerPort: u16,
    pub ProtocolStatus: u16,
    pub Win32Status: u32,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: u16,
}
impl ::core::marker::Copy for HTTP_LOG_FIELDS_DATA {}
impl ::core::clone::Clone for HTTP_LOG_FIELDS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_LOG_FIELDS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOG_FIELDS_DATA")
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
unsafe impl ::windows::core::Abi for HTTP_LOG_FIELDS_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_LOG_FIELDS_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_LOG_FIELDS_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_LOG_FIELDS_DATA {}
impl ::core::default::Default for HTTP_LOG_FIELDS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_BYTES_RECV: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_BYTES_SENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_CLIENT_IP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_CLIENT_PORT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_COMPUTER_NAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_COOKIE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_CORRELATION_ID: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_HOST: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_METHOD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_QUEUE_NAME: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_REASON: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_REFERER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_SERVER_IP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_SERVER_PORT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_SITE_ID: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_SITE_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_STATUS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_STREAM_ID: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_STREAM_ID_EX: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_SUB_STATUS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_TIME_TAKEN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_URI: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_URI_QUERY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_URI_STEM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_USER_AGENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_USER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_VERSION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_LOG_FIELD_WIN32_STATUS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: u32,
    pub KnownHeaderCount: u16,
    pub KnownHeaders: *mut HTTP_KNOWN_HEADER,
}
impl ::core::marker::Copy for HTTP_MULTIPLE_KNOWN_HEADERS {}
impl ::core::clone::Clone for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_MULTIPLE_KNOWN_HEADERS").field("HeaderId", &self.HeaderId).field("Flags", &self.Flags).field("KnownHeaderCount", &self.KnownHeaderCount).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_MULTIPLE_KNOWN_HEADERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_MULTIPLE_KNOWN_HEADERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_MULTIPLE_KNOWN_HEADERS {}
impl ::core::default::Default for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_PERFORMANCE_PARAM {
    pub Type: HTTP_PERFORMANCE_PARAM_TYPE,
    pub BufferSize: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_PERFORMANCE_PARAM {}
impl ::core::clone::Clone for HTTP_PERFORMANCE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_PERFORMANCE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PERFORMANCE_PARAM").field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_PERFORMANCE_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_PERFORMANCE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_PERFORMANCE_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_PERFORMANCE_PARAM {}
impl ::core::default::Default for HTTP_PERFORMANCE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_PERFORMANCE_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(6i32);
impl ::core::marker::Copy for HTTP_PERFORMANCE_PARAM_TYPE {}
impl ::core::clone::Clone for HTTP_PERFORMANCE_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_PERFORMANCE_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_PERFORMANCE_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_PERFORMANCE_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PERFORMANCE_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for HTTP_PROPERTY_FLAGS {}
impl ::core::clone::Clone for HTTP_PROPERTY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PROPERTY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_PROPERTY_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_PROPERTY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_PROPERTY_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_PROPERTY_FLAGS {}
impl ::core::default::Default for HTTP_PROPERTY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
impl ::core::marker::Copy for HTTP_PROTECTION_LEVEL_INFO {}
impl ::core::clone::Clone for HTTP_PROTECTION_LEVEL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_PROTECTION_LEVEL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PROTECTION_LEVEL_INFO").field("Flags", &self.Flags).field("Level", &self.Level).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_PROTECTION_LEVEL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_PROTECTION_LEVEL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_PROTECTION_LEVEL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_PROTECTION_LEVEL_INFO {}
impl ::core::default::Default for HTTP_PROTECTION_LEVEL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_PROTECTION_LEVEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpProtectionLevelUnrestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpProtectionLevelEdgeRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpProtectionLevelRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(2i32);
impl ::core::marker::Copy for HTTP_PROTECTION_LEVEL_TYPE {}
impl ::core::clone::Clone for HTTP_PROTECTION_LEVEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_PROTECTION_LEVEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_PROTECTION_LEVEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_PROTECTION_LEVEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PROTECTION_LEVEL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_QOS_SETTING_INFO {}
impl ::core::clone::Clone for HTTP_QOS_SETTING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QOS_SETTING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QOS_SETTING_INFO").field("QosType", &self.QosType).field("QosSetting", &self.QosSetting).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_QOS_SETTING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QOS_SETTING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QOS_SETTING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QOS_SETTING_INFO {}
impl ::core::default::Default for HTTP_QOS_SETTING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_QOS_SETTING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpQosSettingTypeBandwidth: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpQosSettingTypeConnectionLimit: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpQosSettingTypeFlowRate: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(2i32);
impl ::core::marker::Copy for HTTP_QOS_SETTING_TYPE {}
impl ::core::clone::Clone for HTTP_QOS_SETTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_QOS_SETTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_QOS_SETTING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_QOS_SETTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_QOS_SETTING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: u64,
}
impl ::core::marker::Copy for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
impl ::core::clone::Clone for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_QUIC").field("Freshness", &self.Freshness).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QUERY_REQUEST_QUALIFIER_QUIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
impl ::core::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: u64,
}
impl ::core::marker::Copy for HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
impl ::core::clone::Clone for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_TCP").field("Freshness", &self.Freshness).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QUERY_REQUEST_QUALIFIER_TCP>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
impl ::core::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_QUIC_API_TIMINGS {
    pub ConnectionTimings: HTTP_QUIC_CONNECTION_API_TIMINGS,
    pub StreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
impl ::core::marker::Copy for HTTP_QUIC_API_TIMINGS {}
impl ::core::clone::Clone for HTTP_QUIC_API_TIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QUIC_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_API_TIMINGS").field("ConnectionTimings", &self.ConnectionTimings).field("StreamTimings", &self.StreamTimings).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_QUIC_API_TIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QUIC_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QUIC_API_TIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_API_TIMINGS {}
impl ::core::default::Default for HTTP_QUIC_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
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
impl ::core::marker::Copy for HTTP_QUIC_CONNECTION_API_TIMINGS {}
impl ::core::clone::Clone for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_CONNECTION_API_TIMINGS")
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
unsafe impl ::windows::core::Abi for HTTP_QUIC_CONNECTION_API_TIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QUIC_CONNECTION_API_TIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_CONNECTION_API_TIMINGS {}
impl ::core::default::Default for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
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
impl ::core::marker::Copy for HTTP_QUIC_STREAM_API_TIMINGS {}
impl ::core::clone::Clone for HTTP_QUIC_STREAM_API_TIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_QUIC_STREAM_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_STREAM_API_TIMINGS")
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
unsafe impl ::windows::core::Abi for HTTP_QUIC_STREAM_API_TIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_QUIC_STREAM_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_QUIC_STREAM_API_TIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_STREAM_API_TIMINGS {}
impl ::core::default::Default for HTTP_QUIC_STREAM_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_RECEIVE_HTTP_REQUEST_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(2u32);
impl ::core::marker::Copy for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {}
impl ::core::clone::Clone for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_RECEIVE_HTTP_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: i32,
    pub Flags: u32,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: super::super::Foundation::HANDLE,
    pub ContextAttributes: u32,
    pub PackedContextLength: u32,
    pub PackedContextType: u32,
    pub PackedContext: *mut ::core::ffi::c_void,
    pub MutualAuthDataLength: u32,
    pub pMutualAuthData: ::windows::core::PSTR,
    pub PackageNameLength: u16,
    pub pPackageName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_REQUEST_AUTH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_REQUEST_AUTH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_AUTH_INFO")
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
unsafe impl ::windows::core::Abi for HTTP_REQUEST_AUTH_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_REQUEST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_AUTH_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_REQUEST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_REQUEST_AUTH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeNone: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeBasic: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeDigest: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeNTLM: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeNegotiate: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestAuthTypeKerberos: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(5i32);
impl ::core::marker::Copy for HTTP_REQUEST_AUTH_TYPE {}
impl ::core::clone::Clone for HTTP_REQUEST_AUTH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_REQUEST_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_AUTH_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_REQUEST_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_AUTH_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: *mut HTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: *mut u8,
    pub ChannelTokenSize: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for HTTP_REQUEST_CHANNEL_BIND_STATUS {}
impl ::core::clone::Clone for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_CHANNEL_BIND_STATUS").field("ServiceName", &self.ServiceName).field("ChannelToken", &self.ChannelToken).field("ChannelTokenSize", &self.ChannelTokenSize).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_CHANNEL_BIND_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_CHANNEL_BIND_STATUS {}
impl ::core::default::Default for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
impl ::core::marker::Copy for HTTP_REQUEST_HEADERS {}
impl ::core::clone::Clone for HTTP_REQUEST_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_HEADERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_HEADERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_HEADERS {}
impl ::core::default::Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: u32,
    pub pInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_REQUEST_INFO {}
impl ::core::clone::Clone for HTTP_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_INFO").field("InfoType", &self.InfoType).field("InfoLength", &self.InfoLength).field("pInfo", &self.pInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_INFO {}
impl ::core::default::Default for HTTP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_REQUEST_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeAuth: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeChannelBind: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeSslProtocol: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeSslTokenBindingDraft: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeSslTokenBinding: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeRequestTiming: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeTcpInfoV0: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeRequestSizing: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeQuicStats: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestInfoTypeTcpInfoV1: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(9i32);
impl ::core::marker::Copy for HTTP_REQUEST_INFO_TYPE {}
impl ::core::clone::Clone for HTTP_REQUEST_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_REQUEST_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_REQUEST_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_REQUEST_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyIsb: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyTcpInfoV0: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyQuicStats: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyTcpInfoV1: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertySni: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyStreamError: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyWskApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestPropertyQuicApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(7i32);
impl ::core::marker::Copy for HTTP_REQUEST_PROPERTY {}
impl ::core::clone::Clone for HTTP_REQUEST_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_PROPERTY_SNI {
    pub Hostname: [u16; 256],
    pub Flags: u32,
}
impl ::core::marker::Copy for HTTP_REQUEST_PROPERTY_SNI {}
impl ::core::clone::Clone for HTTP_REQUEST_PROPERTY_SNI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY_SNI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_PROPERTY_SNI").field("Hostname", &self.Hostname).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_PROPERTY_SNI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_PROPERTY_SNI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_PROPERTY_SNI>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_PROPERTY_SNI {}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: u32,
}
impl ::core::marker::Copy for HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
impl ::core::clone::Clone for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_PROPERTY_STREAM_ERROR").field("ErrorCode", &self.ErrorCode).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_PROPERTY_STREAM_ERROR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_SIZING_INFO {
    pub Flags: u64,
    pub RequestIndex: u32,
    pub RequestSizingCount: u32,
    pub RequestSizing: [u64; 5],
}
impl ::core::marker::Copy for HTTP_REQUEST_SIZING_INFO {}
impl ::core::clone::Clone for HTTP_REQUEST_SIZING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_SIZING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_SIZING_INFO").field("Flags", &self.Flags).field("RequestIndex", &self.RequestIndex).field("RequestSizingCount", &self.RequestSizingCount).field("RequestSizing", &self.RequestSizing).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_SIZING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_SIZING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_SIZING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_SIZING_INFO {}
impl ::core::default::Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_REQUEST_SIZING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeHeaders: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestSizingTypeMax: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(5i32);
impl ::core::marker::Copy for HTTP_REQUEST_SIZING_TYPE {}
impl ::core::clone::Clone for HTTP_REQUEST_SIZING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_REQUEST_SIZING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_SIZING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_REQUEST_SIZING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_SIZING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_TIMING_INFO {
    pub RequestTimingCount: u32,
    pub RequestTiming: [u64; 30],
}
impl ::core::marker::Copy for HTTP_REQUEST_TIMING_INFO {}
impl ::core::clone::Clone for HTTP_REQUEST_TIMING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TIMING_INFO").field("RequestTimingCount", &self.RequestTimingCount).field("RequestTiming", &self.RequestTiming).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_TIMING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TIMING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_TIMING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TIMING_INFO {}
impl ::core::default::Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_REQUEST_TIMING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeConnectionStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeDataStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsCertificateLoadStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsCertificateLoadEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsHandshakeLeg1Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsHandshakeLeg1End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsHandshakeLeg2Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsHandshakeLeg2End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsAttributesQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsAttributesQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsClientCertQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeTlsClientCertQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp2StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp2HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp2HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestHeaderParseStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestHeaderParseEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestRoutingStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestRoutingEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestQueuedForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestDeliveredForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestReturnedAfterInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestQueuedForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestDeliveredForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestReturnedAfterDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestQueuedForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeRequestDeliveredForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp3StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp3HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeHttp3HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpRequestTimingTypeMax: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(30i32);
impl ::core::marker::Copy for HTTP_REQUEST_TIMING_TYPE {}
impl ::core::clone::Clone for HTTP_REQUEST_TIMING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_REQUEST_TIMING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_TIMING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_TIMING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO {
    pub TokenBinding: *mut u8,
    pub TokenBindingSize: u32,
    pub EKM: *mut u8,
    pub EKMSize: u32,
    pub KeyType: u8,
}
impl ::core::marker::Copy for HTTP_REQUEST_TOKEN_BINDING_INFO {}
impl ::core::clone::Clone for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TOKEN_BINDING_INFO").field("TokenBinding", &self.TokenBinding).field("TokenBindingSize", &self.TokenBindingSize).field("EKM", &self.EKM).field("EKMSize", &self.EKMSize).field("KeyType", &self.KeyType).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_TOKEN_BINDING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_TOKEN_BINDING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TOKEN_BINDING_INFO {}
impl ::core::default::Default for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_REQUEST_V1 {
    pub Flags: u32,
    pub ConnectionId: u64,
    pub RequestId: u64,
    pub UrlContext: u64,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: u16,
    pub RawUrlLength: u16,
    pub pUnknownVerb: ::windows::core::PCSTR,
    pub pRawUrl: ::windows::core::PCSTR,
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
impl ::core::marker::Copy for HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_REQUEST_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_REQUEST_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_V1")
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
unsafe impl ::windows::core::Abi for HTTP_REQUEST_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_REQUEST_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_REQUEST_V2 {
    pub __AnonymousBase_http_L1861_C35: HTTP_REQUEST_V1,
    pub RequestInfoCount: u16,
    pub pRequestInfo: *mut HTTP_REQUEST_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_REQUEST_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_REQUEST_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_REQUEST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_V2").field("__AnonymousBase_http_L1861_C35", &self.__AnonymousBase_http_L1861_C35).field("RequestInfoCount", &self.RequestInfoCount).field("pRequestInfo", &self.pRequestInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_REQUEST_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_REQUEST_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_REQUEST_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
impl ::core::marker::Copy for HTTP_RESPONSE_HEADERS {}
impl ::core::clone::Clone for HTTP_RESPONSE_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_RESPONSE_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_RESPONSE_HEADERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_RESPONSE_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_RESPONSE_HEADERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_RESPONSE_HEADERS {}
impl ::core::default::Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: u32,
    pub pInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_RESPONSE_INFO {}
impl ::core::clone::Clone for HTTP_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_INFO").field("Type", &self.Type).field("Length", &self.Length).field("pInfo", &self.pInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_RESPONSE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_RESPONSE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_RESPONSE_INFO {}
impl ::core::default::Default for HTTP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_RESPONSE_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpResponseInfoTypeMultipleKnownHeaders: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpResponseInfoTypeAuthenticationProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpResponseInfoTypeQoSProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpResponseInfoTypeChannelBind: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(3i32);
impl ::core::marker::Copy for HTTP_RESPONSE_INFO_TYPE {}
impl ::core::clone::Clone for HTTP_RESPONSE_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_RESPONSE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_RESPONSE_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_RESPONSE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_RESPONSE_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: u32,
    pub Version: HTTP_VERSION,
    pub StatusCode: u16,
    pub ReasonLength: u16,
    pub pReason: ::windows::core::PCSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: u16,
    pub pEntityChunks: *mut HTTP_DATA_CHUNK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_RESPONSE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_RESPONSE_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_V1").field("Flags", &self.Flags).field("Version", &self.Version).field("StatusCode", &self.StatusCode).field("ReasonLength", &self.ReasonLength).field("pReason", &self.pReason).field("Headers", &self.Headers).field("EntityChunkCount", &self.EntityChunkCount).field("pEntityChunks", &self.pEntityChunks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_RESPONSE_V1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_RESPONSE_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_RESPONSE_V1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_RESPONSE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_RESPONSE_V2 {
    pub __AnonymousBase_http_L2050_C36: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: u16,
    pub pResponseInfo: *mut HTTP_RESPONSE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_RESPONSE_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_RESPONSE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_RESPONSE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_V2").field("__AnonymousBase_http_L2050_C36", &self.__AnonymousBase_http_L2050_C36).field("ResponseInfoCount", &self.ResponseInfoCount).field("pResponseInfo", &self.pResponseInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_RESPONSE_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_RESPONSE_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_RESPONSE_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_RESPONSE_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_RESPONSE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SCHEME(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpSchemeHttp: HTTP_SCHEME = HTTP_SCHEME(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpSchemeHttps: HTTP_SCHEME = HTTP_SCHEME(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpSchemeMaximum: HTTP_SCHEME = HTTP_SCHEME(2i32);
impl ::core::marker::Copy for HTTP_SCHEME {}
impl ::core::clone::Clone for HTTP_SCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SCHEME {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SCHEME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: u16,
    pub Realm: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
impl ::core::clone::Clone for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS").field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: u16,
    pub DomainName: ::windows::core::PWSTR,
    pub RealmLength: u16,
    pub Realm: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
impl ::core::clone::Clone for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS").field("DomainNameLength", &self.DomainNameLength).field("DomainName", &self.DomainName).field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for HTTP_SERVER_AUTHENTICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVER_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_INFO").field("Flags", &self.Flags).field("AuthSchemes", &self.AuthSchemes).field("ReceiveMutualAuth", &self.ReceiveMutualAuth).field("ReceiveContextHandle", &self.ReceiveContextHandle).field("DisableNTLMCredentialCaching", &self.DisableNTLMCredentialCaching).field("ExFlags", &self.ExFlags).field("DigestParams", &self.DigestParams).field("BasicParams", &self.BasicParams).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_SERVER_AUTHENTICATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVER_AUTHENTICATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVER_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerLoggingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerQosProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerTimeoutsProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerQueueLengthProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerStateProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServer503VerbosityProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerBindingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerExtendedAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerListenEndpointProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(9i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerChannelBindProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(10i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerProtectionLevelProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(11i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServerDelegationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(16i32);
impl ::core::marker::Copy for HTTP_SERVER_PROPERTY {}
impl ::core::clone::Clone for HTTP_SERVER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVER_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVER_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVER_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: ::windows::core::PSTR,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_A {}
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_A").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_BINDING_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_BINDING_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_A {}
impl ::core::default::Default for HTTP_SERVICE_BINDING_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_BASE {}
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_BASE").field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_BINDING_BASE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_BASE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_BINDING_BASE>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_BASE {}
impl ::core::default::Default for HTTP_SERVICE_BINDING_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_BINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceBindingTypeNone: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceBindingTypeW: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceBindingTypeA: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(2i32);
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_TYPE {}
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_BINDING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_BINDING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: ::windows::core::PWSTR,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_W {}
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_W").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_BINDING_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_BINDING_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_W {}
impl ::core::default::Default for HTTP_SERVICE_BINDING_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_CONFIG_CACHE_KEY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const MaxCacheResponseSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const CacheRangeChunkSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(1i32);
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_CACHE_KEY {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_CACHE_KEY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_CACHE_KEY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_CACHE_SET {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_CACHE_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_CACHE_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_CACHE_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_CACHE_SET {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_CONFIG_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigIPListenList: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSSLCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigUrlAclInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigTimeout: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigCache: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslSniCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSetting: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslSniCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslScopedCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigSslScopedCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigMax: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(13i32);
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_ID {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: u16,
    pub pAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM").field("AddrLength", &self.AddrLength).field("pAddress", &self.pAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: u32,
    pub AddrList: [super::WinSock::SOCKADDR_STORAGE; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY").field("AddrCount", &self.AddrCount).field("AddrList", &self.AddrList).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_CONFIG_QUERY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigQueryExact: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigQueryNext: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpServiceConfigQueryMax: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(2i32);
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_QUERY_TYPE {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_QUERY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_CONFIG_SETTING_KEY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpNone: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpTlsThrottle: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(1i32);
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SETTING_KEY {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SETTING_KEY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_SETTING_KEY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SETTING_KEY,
    pub ParamDesc: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SETTING_SET {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SETTING_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SETTING_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SETTING_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SETTING_SET {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_KEY").field("LocalAddress", &self.LocalAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_KEY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_QUERY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_CCS_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_SET>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY").field("pIpPort", &self.pIpPort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_KEY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_KEY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_KEY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY_EX").field("IpPort", &self.IpPort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_KEY_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: u32,
    pub pSslHash: *mut ::core::ffi::c_void,
    pub AppId: ::windows::core::GUID,
    pub pSslCertStoreName: ::windows::core::PWSTR,
    pub DefaultCertCheckMode: u32,
    pub DefaultRevocationFreshnessTime: u32,
    pub DefaultRevocationUrlRetrievalTimeout: u32,
    pub pDefaultSslCtlIdentifier: ::windows::core::PWSTR,
    pub pDefaultSslCtlStoreName: ::windows::core::PWSTR,
    pub DefaultFlags: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_PARAM")
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
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: u64,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_PARAM_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub union HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    pub Http2WindowSizeParam: HTTP2_WINDOW_SIZE_PARAM,
    pub Http2SettingsLimitsParam: HTTP2_SETTINGS_LIMITS_PARAM,
    pub HttpPerformanceParam: HTTP_PERFORMANCE_PARAM,
    pub HttpTlsRestrictionsParam: HTTP_TLS_RESTRICTIONS_PARAM,
    pub HttpErrorHeadersParam: HTTP_ERROR_HEADERS_PARAM,
    pub HttpTlsSessionTicketKeysParam: HTTP_TLS_SESSION_TICKET_KEYS_PARAM,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_QUERY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_QUERY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_QUERY_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SET {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SET>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SET_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
    pub Host: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_KEY").field("IpPort", &self.IpPort).field("Host", &self.Host).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_KEY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_QUERY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_SET>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_KEY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const IdleConnectionTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HeaderWaitTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(1i32);
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_TIMEOUT_KEY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: u16,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_TIMEOUT_SET {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_TIMEOUT_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_TIMEOUT_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_KEY {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_KEY").field("pUrlPrefix", &self.pUrlPrefix).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_URLACL_KEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_URLACL_KEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_KEY {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_PARAM {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_PARAM").field("pStringSecurityDescriptor", &self.pStringSecurityDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_URLACL_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_PARAM {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: u32,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_QUERY {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_URLACL_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_QUERY {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_SET {}
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SERVICE_CONFIG_URLACL_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SERVICE_CONFIG_URLACL_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_SET {}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: u32,
    pub CertEncodedSize: u32,
    pub pCertEncoded: *mut u8,
    pub Token: super::super::Foundation::HANDLE,
    pub CertDeniedByMapper: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SSL_CLIENT_CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SSL_CLIENT_CERT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SSL_CLIENT_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_CLIENT_CERT_INFO").field("CertFlags", &self.CertFlags).field("CertEncodedSize", &self.CertEncodedSize).field("pCertEncoded", &self.pCertEncoded).field("Token", &self.Token).field("CertDeniedByMapper", &self.CertDeniedByMapper).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_SSL_CLIENT_CERT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SSL_CLIENT_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SSL_CLIENT_CERT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SSL_CLIENT_CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SSL_CLIENT_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: u16,
    pub ConnectionKeySize: u16,
    pub ServerCertIssuerSize: u32,
    pub ServerCertSubjectSize: u32,
    pub pServerCertIssuer: ::windows::core::PCSTR,
    pub pServerCertSubject: ::windows::core::PCSTR,
    pub pClientCertInfo: *mut HTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SSL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SSL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_INFO").field("ServerCertKeySize", &self.ServerCertKeySize).field("ConnectionKeySize", &self.ConnectionKeySize).field("ServerCertIssuerSize", &self.ServerCertIssuerSize).field("ServerCertSubjectSize", &self.ServerCertSubjectSize).field("pServerCertIssuer", &self.pServerCertIssuer).field("pServerCertSubject", &self.pServerCertSubject).field("pClientCertInfo", &self.pClientCertInfo).field("SslClientCertNegotiated", &self.SslClientCertNegotiated).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_SSL_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SSL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SSL_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SSL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_SSL_PROTOCOL_INFO {
    pub Protocol: u32,
    pub CipherType: u32,
    pub CipherStrength: u32,
    pub HashType: u32,
    pub HashStrength: u32,
    pub KeyExchangeType: u32,
    pub KeyExchangeStrength: u32,
}
impl ::core::marker::Copy for HTTP_SSL_PROTOCOL_INFO {}
impl ::core::clone::Clone for HTTP_SSL_PROTOCOL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_SSL_PROTOCOL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_PROTOCOL_INFO").field("Protocol", &self.Protocol).field("CipherType", &self.CipherType).field("CipherStrength", &self.CipherStrength).field("HashType", &self.HashType).field("HashStrength", &self.HashStrength).field("KeyExchangeType", &self.KeyExchangeType).field("KeyExchangeStrength", &self.KeyExchangeStrength).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_SSL_PROTOCOL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_SSL_PROTOCOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_SSL_PROTOCOL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_SSL_PROTOCOL_INFO {}
impl ::core::default::Default for HTTP_SSL_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeHttp2Window: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeHttp2SettingsLimits: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeHttpPerformance: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeTlsRestrictions: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeErrorHeaders: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeTlsSessionTicketKeys: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const ExParamTypeMax: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(6i32);
impl ::core::marker::Copy for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {}
impl ::core::clone::Clone for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
impl ::core::marker::Copy for HTTP_STATE_INFO {}
impl ::core::clone::Clone for HTTP_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_STATE_INFO").field("Flags", &self.Flags).field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_STATE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_STATE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_STATE_INFO {}
impl ::core::default::Default for HTTP_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: u16,
    pub DrainEntityBody: u16,
    pub RequestQueue: u16,
    pub IdleConnection: u16,
    pub HeaderWait: u16,
    pub MinSendRate: u32,
}
impl ::core::marker::Copy for HTTP_TIMEOUT_LIMIT_INFO {}
impl ::core::clone::Clone for HTTP_TIMEOUT_LIMIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_TIMEOUT_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TIMEOUT_LIMIT_INFO").field("Flags", &self.Flags).field("EntityBody", &self.EntityBody).field("DrainEntityBody", &self.DrainEntityBody).field("RequestQueue", &self.RequestQueue).field("IdleConnection", &self.IdleConnection).field("HeaderWait", &self.HeaderWait).field("MinSendRate", &self.MinSendRate).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_TIMEOUT_LIMIT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_TIMEOUT_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TIMEOUT_LIMIT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_TIMEOUT_LIMIT_INFO {}
impl ::core::default::Default for HTTP_TIMEOUT_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_TLS_RESTRICTIONS_PARAM {
    pub RestrictionCount: u32,
    pub TlsRestrictions: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_TLS_RESTRICTIONS_PARAM {}
impl ::core::clone::Clone for HTTP_TLS_RESTRICTIONS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_TLS_RESTRICTIONS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TLS_RESTRICTIONS_PARAM").field("RestrictionCount", &self.RestrictionCount).field("TlsRestrictions", &self.TlsRestrictions).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_TLS_RESTRICTIONS_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_TLS_RESTRICTIONS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TLS_RESTRICTIONS_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_TLS_RESTRICTIONS_PARAM {}
impl ::core::default::Default for HTTP_TLS_RESTRICTIONS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    pub SessionTicketKeyCount: u32,
    pub SessionTicketKeys: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {}
impl ::core::clone::Clone for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TLS_SESSION_TICKET_KEYS_PARAM").field("SessionTicketKeyCount", &self.SessionTicketKeyCount).field("SessionTicketKeys", &self.SessionTicketKeys).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TLS_SESSION_TICKET_KEYS_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {}
impl ::core::default::Default for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: *mut super::WinSock::SOCKADDR,
    pub pLocalAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_TRANSPORT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_TRANSPORT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_TRANSPORT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRANSPORT_ADDRESS").field("pRemoteAddress", &self.pRemoteAddress).field("pLocalAddress", &self.pLocalAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for HTTP_TRANSPORT_ADDRESS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_TRANSPORT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TRANSPORT_ADDRESS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_TRANSPORT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_TRANSPORT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: u16,
    pub RawValueLength: u16,
    pub pName: ::windows::core::PCSTR,
    pub pRawValue: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HTTP_UNKNOWN_HEADER {}
impl ::core::clone::Clone for HTTP_UNKNOWN_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_UNKNOWN_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_UNKNOWN_HEADER").field("NameLength", &self.NameLength).field("RawValueLength", &self.RawValueLength).field("pName", &self.pName).field("pRawValue", &self.pRawValue).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_UNKNOWN_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_UNKNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_UNKNOWN_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_UNKNOWN_HEADER {}
impl ::core::default::Default for HTTP_UNKNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_VERB(pub i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbUnparsed: HTTP_VERB = HTTP_VERB(0i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbUnknown: HTTP_VERB = HTTP_VERB(1i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbInvalid: HTTP_VERB = HTTP_VERB(2i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbOPTIONS: HTTP_VERB = HTTP_VERB(3i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbGET: HTTP_VERB = HTTP_VERB(4i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbHEAD: HTTP_VERB = HTTP_VERB(5i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbPOST: HTTP_VERB = HTTP_VERB(6i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbPUT: HTTP_VERB = HTTP_VERB(7i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbDELETE: HTTP_VERB = HTTP_VERB(8i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbTRACE: HTTP_VERB = HTTP_VERB(9i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbCONNECT: HTTP_VERB = HTTP_VERB(10i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbTRACK: HTTP_VERB = HTTP_VERB(11i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbMOVE: HTTP_VERB = HTTP_VERB(12i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbCOPY: HTTP_VERB = HTTP_VERB(13i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbPROPFIND: HTTP_VERB = HTTP_VERB(14i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbPROPPATCH: HTTP_VERB = HTTP_VERB(15i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbMKCOL: HTTP_VERB = HTTP_VERB(16i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbLOCK: HTTP_VERB = HTTP_VERB(17i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbUNLOCK: HTTP_VERB = HTTP_VERB(18i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbSEARCH: HTTP_VERB = HTTP_VERB(19i32);
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HttpVerbMaximum: HTTP_VERB = HTTP_VERB(20i32);
impl ::core::marker::Copy for HTTP_VERB {}
impl ::core::clone::Clone for HTTP_VERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_VERB {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_VERB {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_VERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_VERB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub const HTTP_VERSION: &'static str = "HTTP/1.0";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for HTTP_VERSION {}
impl ::core::clone::Clone for HTTP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_VERSION {}
impl ::core::default::Default for HTTP_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
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
impl ::core::marker::Copy for HTTP_WSK_API_TIMINGS {}
impl ::core::clone::Clone for HTTP_WSK_API_TIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_WSK_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_WSK_API_TIMINGS")
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
unsafe impl ::windows::core::Abi for HTTP_WSK_API_TIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_WSK_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_WSK_API_TIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_WSK_API_TIMINGS {}
impl ::core::default::Default for HTTP_WSK_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpAddFragmentToCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddFragmentToCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: ::windows::core::PCWSTR, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpAddFragmentToCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::core::mem::transmute(datachunk), ::core::mem::transmute(cachepolicy), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpAddUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(requestqueuehandle: Param0, fullyqualifiedurl: Param1, reserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpAddUrl(requestqueuehandle.into_param().abi(), fullyqualifiedurl.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpAddUrlToUrlGroup<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(urlgroupid: u64, pfullyqualifiedurl: Param1, urlcontext: u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddUrlToUrlGroup(urlgroupid: u64, pfullyqualifiedurl: ::windows::core::PCWSTR, urlcontext: u64, reserved: u32) -> u32;
        }
        ::core::mem::transmute(HttpAddUrlToUrlGroup(::core::mem::transmute(urlgroupid), pfullyqualifiedurl.into_param().abi(), ::core::mem::transmute(urlcontext), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpCancelHttpRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCancelHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpCancelHttpRequest(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCloseRequestQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(HttpCloseRequestQueue(requestqueuehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpCloseServerSession(serversessionid: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseServerSession(serversessionid: u64) -> u32;
        }
        ::core::mem::transmute(HttpCloseServerSession(::core::mem::transmute(serversessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpCloseUrlGroup(urlgroupid: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseUrlGroup(urlgroupid: u64) -> u32;
        }
        ::core::mem::transmute(HttpCloseUrlGroup(::core::mem::transmute(urlgroupid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32;
        }
        ::core::mem::transmute(HttpCreateHttpHandle(::core::mem::transmute(requestqueuehandle), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HttpCreateRequestQueue<'a, Param0: ::windows::core::IntoParam<'a, HTTPAPI_VERSION>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(version: Param0, name: Param1, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateRequestQueue(version: HTTPAPI_VERSION, name: ::windows::core::PCWSTR, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(HttpCreateRequestQueue(version.into_param().abi(), name.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flags), ::core::mem::transmute(requestqueuehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpCreateServerSession<'a, Param0: ::windows::core::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, serversessionid: *mut u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateServerSession(version: HTTPAPI_VERSION, serversessionid: *mut u64, reserved: u32) -> u32;
        }
        ::core::mem::transmute(HttpCreateServerSession(version.into_param().abi(), ::core::mem::transmute(serversessionid), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32;
        }
        ::core::mem::transmute(HttpCreateUrlGroup(::core::mem::transmute(serversessionid), ::core::mem::transmute(purlgroupid), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpDeclarePush<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(requestqueuehandle: Param0, requestid: u64, verb: HTTP_VERB, path: Param3, query: Param4, headers: *const HTTP_REQUEST_HEADERS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDeclarePush(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, verb: HTTP_VERB, path: ::windows::core::PCWSTR, query: ::windows::core::PCSTR, headers: *const HTTP_REQUEST_HEADERS) -> u32;
        }
        ::core::mem::transmute(HttpDeclarePush(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(verb), path.into_param().abi(), query.into_param().abi(), ::core::mem::transmute(headers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpDelegateRequestEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, delegatequeuehandle: Param1, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDelegateRequestEx(requestqueuehandle: super::super::Foundation::HANDLE, delegatequeuehandle: super::super::Foundation::HANDLE, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32;
        }
        ::core::mem::transmute(HttpDelegateRequestEx(requestqueuehandle.into_param().abi(), delegatequeuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(delegateurlgroupid), ::core::mem::transmute(propertyinfosetsize), ::core::mem::transmute(propertyinfoset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpDeleteServiceConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDeleteServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpDeleteServiceConfiguration(servicehandle.into_param().abi(), ::core::mem::transmute(configid), ::core::mem::transmute(pconfiginformation), ::core::mem::transmute(configinformationlength), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpFindUrlGroupId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(fullyqualifiedurl: Param0, requestqueuehandle: Param1, urlgroupid: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFindUrlGroupId(fullyqualifiedurl: ::windows::core::PCWSTR, requestqueuehandle: super::super::Foundation::HANDLE, urlgroupid: *mut u64) -> u32;
        }
        ::core::mem::transmute(HttpFindUrlGroupId(fullyqualifiedurl.into_param().abi(), requestqueuehandle.into_param().abi(), ::core::mem::transmute(urlgroupid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpFlushResponseCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFlushResponseCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: ::windows::core::PCWSTR, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpFlushResponseCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpGetExtension<'a, Param0: ::windows::core::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, extension: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpGetExtension(version: HTTPAPI_VERSION, extension: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(HttpGetExtension(version.into_param().abi(), ::core::mem::transmute(extension), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpInitialize<'a, Param0: ::windows::core::IntoParam<'a, HTTPAPI_VERSION>>(version: Param0, flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpInitialize(version: HTTPAPI_VERSION, flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpInitialize(version.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpIsFeatureSupported(::core::mem::transmute(featureid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpPrepareUrl<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(reserved: *mut ::core::ffi::c_void, flags: u32, url: Param2, preparedurl: *mut ::windows::core::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpPrepareUrl(reserved: *mut ::core::ffi::c_void, flags: u32, url: ::windows::core::PCWSTR, preparedurl: *mut ::windows::core::PWSTR) -> u32;
        }
        ::core::mem::transmute(HttpPrepareUrl(::core::mem::transmute(reserved), ::core::mem::transmute(flags), url.into_param().abi(), ::core::mem::transmute(preparedurl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpQueryRequestQueueProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpQueryRequestQueueProperty(requestqueuehandle.into_param().abi(), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength), ::core::mem::transmute(reserved1), ::core::mem::transmute(returnlength), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(HttpQueryServerSessionProperty(::core::mem::transmute(serversessionid), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpQueryServiceConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::core::ffi::c_void, inputlength: u32, poutput: *mut ::core::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::core::ffi::c_void, inputlength: u32, poutput: *mut ::core::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpQueryServiceConfiguration(servicehandle.into_param().abi(), ::core::mem::transmute(configid), ::core::mem::transmute(pinput), ::core::mem::transmute(inputlength), ::core::mem::transmute(poutput), ::core::mem::transmute(outputlength), ::core::mem::transmute(preturnlength), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(HttpQueryUrlGroupProperty(::core::mem::transmute(urlgroupid), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpReadFragmentFromCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(requestqueuehandle: Param0, urlprefix: Param1, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::core::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReadFragmentFromCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: ::windows::core::PCWSTR, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::core::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpReadFragmentFromCache(requestqueuehandle.into_param().abi(), urlprefix.into_param().abi(), ::core::mem::transmute(byterange), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength), ::core::mem::transmute(bytesread), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpReceiveClientCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveClientCertificate(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpReceiveClientCertificate(requestqueuehandle.into_param().abi(), ::core::mem::transmute(connectionid), ::core::mem::transmute(flags), ::core::mem::transmute(sslclientcertinfo), ::core::mem::transmute(sslclientcertinfosize), ::core::mem::transmute(bytesreceived), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpReceiveHttpRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpReceiveHttpRequest(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(flags), ::core::mem::transmute(requestbuffer), ::core::mem::transmute(requestbufferlength), ::core::mem::transmute(bytesreturned), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpReceiveRequestEntityBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, entitybuffer: *mut ::core::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpReceiveRequestEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitybuffer: *mut ::core::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpReceiveRequestEntityBody(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(flags), ::core::mem::transmute(entitybuffer), ::core::mem::transmute(entitybufferlength), ::core::mem::transmute(bytesreturned), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpRemoveUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(requestqueuehandle: Param0, fullyqualifiedurl: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpRemoveUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(HttpRemoveUrl(requestqueuehandle.into_param().abi(), fullyqualifiedurl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpRemoveUrlFromUrlGroup<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(urlgroupid: u64, pfullyqualifiedurl: Param1, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpRemoveUrlFromUrlGroup(urlgroupid: u64, pfullyqualifiedurl: ::windows::core::PCWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(HttpRemoveUrlFromUrlGroup(::core::mem::transmute(urlgroupid), pfullyqualifiedurl.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpSendHttpResponse<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendHttpResponse(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
        }
        ::core::mem::transmute(HttpSendHttpResponse(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(flags), ::core::mem::transmute(httpresponse), ::core::mem::transmute(cachepolicy), ::core::mem::transmute(bytessent), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(overlapped), ::core::mem::transmute(logdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpSendResponseEntityBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, requestid: u64, flags: u32, entitychunks: &[HTTP_DATA_CHUNK], bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendResponseEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitychunkcount: u16, entitychunks: *const HTTP_DATA_CHUNK, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
        }
        ::core::mem::transmute(HttpSendResponseEntityBody(requestqueuehandle.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(flags), entitychunks.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(entitychunks)), ::core::mem::transmute(bytessent), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(overlapped), ::core::mem::transmute(logdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpSetRequestProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::core::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetRequestProperty(requestqueuehandle: super::super::Foundation::HANDLE, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::core::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpSetRequestProperty(requestqueuehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(propertyid), ::core::mem::transmute(input), ::core::mem::transmute(inputpropertysize), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSetRequestQueueProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpSetRequestQueueProperty(requestqueuehandle.into_param().abi(), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
        }
        ::core::mem::transmute(HttpSetServerSessionProperty(::core::mem::transmute(serversessionid), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpSetServiceConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(servicehandle: Param0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpSetServiceConfiguration(servicehandle.into_param().abi(), ::core::mem::transmute(configid), ::core::mem::transmute(pconfiginformation), ::core::mem::transmute(configinformationlength), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
        }
        ::core::mem::transmute(HttpSetUrlGroupProperty(::core::mem::transmute(urlgroupid), ::core::mem::transmute(property), ::core::mem::transmute(propertyinformation), ::core::mem::transmute(propertyinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpShutdownRequestQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpShutdownRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(HttpShutdownRequestQueue(requestqueuehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`*"]
#[inline]
pub unsafe fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpTerminate(::core::mem::transmute(flags), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpUpdateServiceConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::core::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpUpdateServiceConfiguration(handle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::core::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpUpdateServiceConfiguration(handle.into_param().abi(), ::core::mem::transmute(configid), ::core::mem::transmute(configinfo), ::core::mem::transmute(configinfolength), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpWaitForDemandStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDemandStart(requestqueuehandle: super::super::Foundation::HANDLE, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpWaitForDemandStart(requestqueuehandle.into_param().abi(), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpWaitForDisconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDisconnect(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpWaitForDisconnect(requestqueuehandle.into_param().abi(), ::core::mem::transmute(connectionid), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_HttpServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpWaitForDisconnectEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(requestqueuehandle: Param0, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWaitForDisconnectEx(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(HttpWaitForDisconnectEx(requestqueuehandle.into_param().abi(), ::core::mem::transmute(connectionid), ::core::mem::transmute(reserved), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
