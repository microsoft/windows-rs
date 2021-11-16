#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpAddFragmentToCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrlToUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, urlcontext: u64, reserved: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpCancelHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCloseRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
    pub fn HttpCloseServerSession(serversessionid: u64) -> u32;
    pub fn HttpCloseUrlGroup(urlgroupid: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HttpCreateRequestQueue(version: HTTPAPI_VERSION, name: super::super::Foundation::PWSTR, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32;
    pub fn HttpCreateServerSession(version: HTTPAPI_VERSION, serversessionid: *mut u64, reserved: u32) -> u32;
    pub fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDeclarePush(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, verb: HTTP_VERB, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PSTR, headers: *const HTTP_REQUEST_HEADERS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDelegateRequestEx(requestqueuehandle: super::super::Foundation::HANDLE, delegatequeuehandle: super::super::Foundation::HANDLE, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpDeleteServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpFindUrlGroupId(fullyqualifiedurl: super::super::Foundation::PWSTR, requestqueuehandle: super::super::Foundation::HANDLE, urlgroupid: *mut u64) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpFlushResponseCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    pub fn HttpGetExtension(version: HTTPAPI_VERSION, extension: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
    pub fn HttpInitialize(version: HTTPAPI_VERSION, flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpPrepareUrl(reserved: *mut ::core::ffi::c_void, flags: u32, url: super::super::Foundation::PWSTR, preparedurl: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::core::ffi::c_void) -> u32;
    pub fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpQueryServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::core::ffi::c_void, inputlength: u32, poutput: *mut ::core::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    pub fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReadFragmentFromCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::core::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveClientCertificate(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
    pub fn HttpReceiveHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveRequestEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitybuffer: *mut ::core::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrlFromUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendHttpResponse(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendResponseEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitychunkcount: u16, entitychunks: *const HTTP_DATA_CHUNK, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetRequestProperty(requestqueuehandle: super::super::Foundation::HANDLE, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::core::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSetRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> u32;
    pub fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    pub fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpShutdownRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
    pub fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpUpdateServiceConfiguration(handle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::core::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDemandStart(requestqueuehandle: super::super::Foundation::HANDLE, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnect(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnectEx(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
}
#[repr(C)]
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
#[repr(C)]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: u32,
}
impl ::core::marker::Copy for HTTP2_WINDOW_SIZE_PARAM {}
impl ::core::clone::Clone for HTTP2_WINDOW_SIZE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const Http503ResponseVerbosityBasic: i32 = 0i32;
pub const Http503ResponseVerbosityLimited: i32 = 1i32;
pub const Http503ResponseVerbosityFull: i32 = 2i32;
pub const HttpAuthenticationHardeningLegacy: i32 = 0i32;
pub const HttpAuthenticationHardeningMedium: i32 = 1i32;
pub const HttpAuthenticationHardeningStrict: i32 = 2i32;
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1u32;
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2u32;
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16u32;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8u32;
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4u32;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2u32;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1u32;
pub const HttpAuthStatusSuccess: i32 = 0i32;
pub const HttpAuthStatusNotAuthenticated: i32 = 1i32;
pub const HttpAuthStatusFailure: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const HttpCachePolicyNocache: i32 = 0i32;
pub const HttpCachePolicyUserInvalidates: i32 = 1i32;
pub const HttpCachePolicyTimeToLive: i32 = 2i32;
pub const HttpCachePolicyMaximum: i32 = 3i32;
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
#[repr(C)]
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
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for HTTP_COOKED_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_COOKED_URL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
pub const CreateRequestQueueExternalIdProperty: i32 = 1i32;
pub const CreateRequestQueueMax: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: u16,
    pub pFragmentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_DATA_CHUNK_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_DATA_CHUNK_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
pub const HttpDataChunkFromMemory: i32 = 0i32;
pub const HttpDataChunkFromFileHandle: i32 = 1i32;
pub const HttpDataChunkFromFragmentCache: i32 = 2i32;
pub const HttpDataChunkFromFragmentCacheEx: i32 = 3i32;
pub const HttpDataChunkTrailers: i32 = 4i32;
pub const HttpDataChunkMaximum: i32 = 5i32;
pub const DelegateRequestReservedProperty: i32 = 0i32;
pub const DelegateRequestDelegateUrlProperty: i32 = 1i32;
#[repr(C)]
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
pub const HTTP_DEMAND_CBT: u32 = 4u32;
pub const HttpEnabledStateActive: i32 = 0i32;
pub const HttpEnabledStateInactive: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: u16,
    pub HeaderCount: u16,
    pub Headers: *mut HTTP_UNKNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_ERROR_HEADERS_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_ERROR_HEADERS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HttpFeatureUnknown: i32 = 0i32;
pub const HttpFeatureResponseTrailers: i32 = 1i32;
pub const HttpFeatureApiTimings: i32 = 2i32;
pub const HttpFeatureDelegateEx: i32 = 3i32;
pub const HttpFeatureHttp3: i32 = 4i32;
pub const HttpFeaturemax: i32 = -1i32;
#[repr(C)]
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
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
pub const HttpHeaderCacheControl: i32 = 0i32;
pub const HttpHeaderConnection: i32 = 1i32;
pub const HttpHeaderDate: i32 = 2i32;
pub const HttpHeaderKeepAlive: i32 = 3i32;
pub const HttpHeaderPragma: i32 = 4i32;
pub const HttpHeaderTrailer: i32 = 5i32;
pub const HttpHeaderTransferEncoding: i32 = 6i32;
pub const HttpHeaderUpgrade: i32 = 7i32;
pub const HttpHeaderVia: i32 = 8i32;
pub const HttpHeaderWarning: i32 = 9i32;
pub const HttpHeaderAllow: i32 = 10i32;
pub const HttpHeaderContentLength: i32 = 11i32;
pub const HttpHeaderContentType: i32 = 12i32;
pub const HttpHeaderContentEncoding: i32 = 13i32;
pub const HttpHeaderContentLanguage: i32 = 14i32;
pub const HttpHeaderContentLocation: i32 = 15i32;
pub const HttpHeaderContentMd5: i32 = 16i32;
pub const HttpHeaderContentRange: i32 = 17i32;
pub const HttpHeaderExpires: i32 = 18i32;
pub const HttpHeaderLastModified: i32 = 19i32;
pub const HttpHeaderAccept: i32 = 20i32;
pub const HttpHeaderAcceptCharset: i32 = 21i32;
pub const HttpHeaderAcceptEncoding: i32 = 22i32;
pub const HttpHeaderAcceptLanguage: i32 = 23i32;
pub const HttpHeaderAuthorization: i32 = 24i32;
pub const HttpHeaderCookie: i32 = 25i32;
pub const HttpHeaderExpect: i32 = 26i32;
pub const HttpHeaderFrom: i32 = 27i32;
pub const HttpHeaderHost: i32 = 28i32;
pub const HttpHeaderIfMatch: i32 = 29i32;
pub const HttpHeaderIfModifiedSince: i32 = 30i32;
pub const HttpHeaderIfNoneMatch: i32 = 31i32;
pub const HttpHeaderIfRange: i32 = 32i32;
pub const HttpHeaderIfUnmodifiedSince: i32 = 33i32;
pub const HttpHeaderMaxForwards: i32 = 34i32;
pub const HttpHeaderProxyAuthorization: i32 = 35i32;
pub const HttpHeaderReferer: i32 = 36i32;
pub const HttpHeaderRange: i32 = 37i32;
pub const HttpHeaderTe: i32 = 38i32;
pub const HttpHeaderTranslate: i32 = 39i32;
pub const HttpHeaderUserAgent: i32 = 40i32;
pub const HttpHeaderRequestMaximum: i32 = 41i32;
pub const HttpHeaderAcceptRanges: i32 = 20i32;
pub const HttpHeaderAge: i32 = 21i32;
pub const HttpHeaderEtag: i32 = 22i32;
pub const HttpHeaderLocation: i32 = 23i32;
pub const HttpHeaderProxyAuthenticate: i32 = 24i32;
pub const HttpHeaderRetryAfter: i32 = 25i32;
pub const HttpHeaderServer: i32 = 26i32;
pub const HttpHeaderSetCookie: i32 = 27i32;
pub const HttpHeaderVary: i32 = 28i32;
pub const HttpHeaderWwwAuthenticate: i32 = 29i32;
pub const HttpHeaderResponseMaximum: i32 = 30i32;
pub const HttpHeaderMaximum: i32 = 41i32;
pub const HTTP_INITIALIZE_CONFIG: u32 = 2u32;
pub const HTTP_INITIALIZE_SERVER: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: u16,
    pub pRawValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_KNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_KNOWN_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: u32,
    pub SoftwareName: super::super::Foundation::PWSTR,
    pub SoftwareNameLength: u16,
    pub DirectoryNameLength: u16,
    pub DirectoryName: super::super::Foundation::PWSTR,
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
pub const HttpLoggingRolloverSize: i32 = 0i32;
pub const HttpLoggingRolloverDaily: i32 = 1i32;
pub const HttpLoggingRolloverWeekly: i32 = 2i32;
pub const HttpLoggingRolloverMonthly: i32 = 3i32;
pub const HttpLoggingRolloverHourly: i32 = 4i32;
pub const HttpLoggingTypeW3C: i32 = 0i32;
pub const HttpLoggingTypeIIS: i32 = 1i32;
pub const HttpLoggingTypeNCSA: i32 = 2i32;
pub const HttpLoggingTypeRaw: i32 = 3i32;
#[repr(C)]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
impl ::core::marker::Copy for HTTP_LOG_DATA {}
impl ::core::clone::Clone for HTTP_LOG_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HttpLogDataTypeFields: i32 = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for HTTP_LOG_FIELDS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_LOG_FIELDS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTTP_LOG_FIELD_BYTES_RECV: u32 = 8192u32;
pub const HTTP_LOG_FIELD_BYTES_SENT: u32 = 4096u32;
pub const HTTP_LOG_FIELD_CLIENT_IP: u32 = 4u32;
pub const HTTP_LOG_FIELD_CLIENT_PORT: u32 = 4194304u32;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: u32 = 32u32;
pub const HTTP_LOG_FIELD_COOKIE: u32 = 131072u32;
pub const HTTP_LOG_FIELD_CORRELATION_ID: u32 = 1073741824u32;
pub const HTTP_LOG_FIELD_DATE: u32 = 1u32;
pub const HTTP_LOG_FIELD_HOST: u32 = 1048576u32;
pub const HTTP_LOG_FIELD_METHOD: u32 = 128u32;
pub const HTTP_LOG_FIELD_QUEUE_NAME: u32 = 67108864u32;
pub const HTTP_LOG_FIELD_REASON: u32 = 33554432u32;
pub const HTTP_LOG_FIELD_REFERER: u32 = 262144u32;
pub const HTTP_LOG_FIELD_SERVER_IP: u32 = 64u32;
pub const HTTP_LOG_FIELD_SERVER_PORT: u32 = 32768u32;
pub const HTTP_LOG_FIELD_SITE_ID: u32 = 16777216u32;
pub const HTTP_LOG_FIELD_SITE_NAME: u32 = 16u32;
pub const HTTP_LOG_FIELD_STATUS: u32 = 1024u32;
pub const HTTP_LOG_FIELD_STREAM_ID: u32 = 134217728u32;
pub const HTTP_LOG_FIELD_STREAM_ID_EX: u32 = 268435456u32;
pub const HTTP_LOG_FIELD_SUB_STATUS: u32 = 2097152u32;
pub const HTTP_LOG_FIELD_TIME: u32 = 2u32;
pub const HTTP_LOG_FIELD_TIME_TAKEN: u32 = 16384u32;
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: u32 = 536870912u32;
pub const HTTP_LOG_FIELD_URI: u32 = 8388608u32;
pub const HTTP_LOG_FIELD_URI_QUERY: u32 = 512u32;
pub const HTTP_LOG_FIELD_URI_STEM: u32 = 256u32;
pub const HTTP_LOG_FIELD_USER_AGENT: u32 = 65536u32;
pub const HTTP_LOG_FIELD_USER_NAME: u32 = 8u32;
pub const HTTP_LOG_FIELD_VERSION: u32 = 524288u32;
pub const HTTP_LOG_FIELD_WIN32_STATUS: u32 = 2048u32;
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: u32 = 2147483647u32;
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: u32,
    pub KnownHeaderCount: u16,
    pub KnownHeaders: *mut HTTP_KNOWN_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_MULTIPLE_KNOWN_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const PerformanceParamSendBufferingFlags: i32 = 0i32;
pub const PerformanceParamAggressiveICW: i32 = 1i32;
pub const PerformanceParamMaxSendBufferSize: i32 = 2i32;
pub const PerformanceParamMaxConcurrentClientStreams: i32 = 3i32;
pub const PerformanceParamMaxReceiveBufferSize: i32 = 4i32;
pub const PerformanceParamDecryptOnSspiThread: i32 = 5i32;
pub const PerformanceParamMax: i32 = 6i32;
#[repr(C)]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for HTTP_PROPERTY_FLAGS {}
impl ::core::clone::Clone for HTTP_PROPERTY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HttpProtectionLevelUnrestricted: i32 = 0i32;
pub const HttpProtectionLevelEdgeRestricted: i32 = 1i32;
pub const HttpProtectionLevelRestricted: i32 = 2i32;
#[repr(C)]
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
pub const HttpQosSettingTypeBandwidth: i32 = 0i32;
pub const HttpQosSettingTypeConnectionLimit: i32 = 1i32;
pub const HttpQosSettingTypeFlowRate: i32 = 2i32;
#[repr(C)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: u64,
}
impl ::core::marker::Copy for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
impl ::core::clone::Clone for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: u64,
}
impl ::core::marker::Copy for HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
impl ::core::clone::Clone for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: u32 = 1u32;
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: u32 = 2u32;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
#[repr(C)]
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
    pub pMutualAuthData: super::super::Foundation::PSTR,
    pub PackageNameLength: u16,
    pub pPackageName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_REQUEST_AUTH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HttpRequestAuthTypeNone: i32 = 0i32;
pub const HttpRequestAuthTypeBasic: i32 = 1i32;
pub const HttpRequestAuthTypeDigest: i32 = 2i32;
pub const HttpRequestAuthTypeNTLM: i32 = 3i32;
pub const HttpRequestAuthTypeNegotiate: i32 = 4i32;
pub const HttpRequestAuthTypeKerberos: i32 = 5i32;
#[repr(C)]
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
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_REQUEST_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_REQUEST_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HttpRequestInfoTypeAuth: i32 = 0i32;
pub const HttpRequestInfoTypeChannelBind: i32 = 1i32;
pub const HttpRequestInfoTypeSslProtocol: i32 = 2i32;
pub const HttpRequestInfoTypeSslTokenBindingDraft: i32 = 3i32;
pub const HttpRequestInfoTypeSslTokenBinding: i32 = 4i32;
pub const HttpRequestInfoTypeRequestTiming: i32 = 5i32;
pub const HttpRequestInfoTypeTcpInfoV0: i32 = 6i32;
pub const HttpRequestInfoTypeRequestSizing: i32 = 7i32;
pub const HttpRequestInfoTypeQuicStats: i32 = 8i32;
pub const HttpRequestInfoTypeTcpInfoV1: i32 = 9i32;
pub const HttpRequestPropertyIsb: i32 = 0i32;
pub const HttpRequestPropertyTcpInfoV0: i32 = 1i32;
pub const HttpRequestPropertyQuicStats: i32 = 2i32;
pub const HttpRequestPropertyTcpInfoV1: i32 = 3i32;
pub const HttpRequestPropertySni: i32 = 4i32;
pub const HttpRequestPropertyStreamError: i32 = 5i32;
pub const HttpRequestPropertyWskApiTimings: i32 = 6i32;
pub const HttpRequestPropertyQuicApiTimings: i32 = 7i32;
#[repr(C)]
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
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: u32,
}
impl ::core::marker::Copy for HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
impl ::core::clone::Clone for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: i32 = 0i32;
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: i32 = 1i32;
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: i32 = 2i32;
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: i32 = 3i32;
pub const HttpRequestSizingTypeHeaders: i32 = 4i32;
pub const HttpRequestSizingTypeMax: i32 = 5i32;
#[repr(C)]
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
pub const HttpRequestTimingTypeConnectionStart: i32 = 0i32;
pub const HttpRequestTimingTypeDataStart: i32 = 1i32;
pub const HttpRequestTimingTypeTlsCertificateLoadStart: i32 = 2i32;
pub const HttpRequestTimingTypeTlsCertificateLoadEnd: i32 = 3i32;
pub const HttpRequestTimingTypeTlsHandshakeLeg1Start: i32 = 4i32;
pub const HttpRequestTimingTypeTlsHandshakeLeg1End: i32 = 5i32;
pub const HttpRequestTimingTypeTlsHandshakeLeg2Start: i32 = 6i32;
pub const HttpRequestTimingTypeTlsHandshakeLeg2End: i32 = 7i32;
pub const HttpRequestTimingTypeTlsAttributesQueryStart: i32 = 8i32;
pub const HttpRequestTimingTypeTlsAttributesQueryEnd: i32 = 9i32;
pub const HttpRequestTimingTypeTlsClientCertQueryStart: i32 = 10i32;
pub const HttpRequestTimingTypeTlsClientCertQueryEnd: i32 = 11i32;
pub const HttpRequestTimingTypeHttp2StreamStart: i32 = 12i32;
pub const HttpRequestTimingTypeHttp2HeaderDecodeStart: i32 = 13i32;
pub const HttpRequestTimingTypeHttp2HeaderDecodeEnd: i32 = 14i32;
pub const HttpRequestTimingTypeRequestHeaderParseStart: i32 = 15i32;
pub const HttpRequestTimingTypeRequestHeaderParseEnd: i32 = 16i32;
pub const HttpRequestTimingTypeRequestRoutingStart: i32 = 17i32;
pub const HttpRequestTimingTypeRequestRoutingEnd: i32 = 18i32;
pub const HttpRequestTimingTypeRequestQueuedForInspection: i32 = 19i32;
pub const HttpRequestTimingTypeRequestDeliveredForInspection: i32 = 20i32;
pub const HttpRequestTimingTypeRequestReturnedAfterInspection: i32 = 21i32;
pub const HttpRequestTimingTypeRequestQueuedForDelegation: i32 = 22i32;
pub const HttpRequestTimingTypeRequestDeliveredForDelegation: i32 = 23i32;
pub const HttpRequestTimingTypeRequestReturnedAfterDelegation: i32 = 24i32;
pub const HttpRequestTimingTypeRequestQueuedForIO: i32 = 25i32;
pub const HttpRequestTimingTypeRequestDeliveredForIO: i32 = 26i32;
pub const HttpRequestTimingTypeHttp3StreamStart: i32 = 27i32;
pub const HttpRequestTimingTypeHttp3HeaderDecodeStart: i32 = 28i32;
pub const HttpRequestTimingTypeHttp3HeaderDecodeEnd: i32 = 29i32;
pub const HttpRequestTimingTypeMax: i32 = 30i32;
#[repr(C)]
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
#[repr(C)]
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
impl ::core::marker::Copy for HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_REQUEST_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_RESPONSE_HEADERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_RESPONSE_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
pub const HttpResponseInfoTypeMultipleKnownHeaders: i32 = 0i32;
pub const HttpResponseInfoTypeAuthenticationProperty: i32 = 1i32;
pub const HttpResponseInfoTypeQoSProperty: i32 = 2i32;
pub const HttpResponseInfoTypeChannelBind: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_RESPONSE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HttpSchemeHttp: i32 = 0i32;
pub const HttpSchemeHttps: i32 = 1i32;
pub const HttpSchemeMaximum: i32 = 2i32;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4u32;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1u32;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8u32;
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256u32;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2u32;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64u32;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: u16,
    pub Realm: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: u16,
    pub DomainName: super::super::Foundation::PWSTR,
    pub RealmLength: u16,
    pub Realm: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const HttpServerAuthenticationProperty: i32 = 0i32;
pub const HttpServerLoggingProperty: i32 = 1i32;
pub const HttpServerQosProperty: i32 = 2i32;
pub const HttpServerTimeoutsProperty: i32 = 3i32;
pub const HttpServerQueueLengthProperty: i32 = 4i32;
pub const HttpServerStateProperty: i32 = 5i32;
pub const HttpServer503VerbosityProperty: i32 = 6i32;
pub const HttpServerBindingProperty: i32 = 7i32;
pub const HttpServerExtendedAuthenticationProperty: i32 = 8i32;
pub const HttpServerListenEndpointProperty: i32 = 9i32;
pub const HttpServerChannelBindProperty: i32 = 10i32;
pub const HttpServerProtectionLevelProperty: i32 = 11i32;
pub const HttpServerDelegationProperty: i32 = 16i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::super::Foundation::PSTR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_BASE {}
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HttpServiceBindingTypeNone: i32 = 0i32;
pub const HttpServiceBindingTypeW: i32 = 1i32;
pub const HttpServiceBindingTypeA: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::super::Foundation::PWSTR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_BINDING_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_BINDING_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MaxCacheResponseSize: i32 = 0i32;
pub const CacheRangeChunkSize: i32 = 1i32;
#[repr(C)]
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
pub const HttpServiceConfigIPListenList: i32 = 0i32;
pub const HttpServiceConfigSSLCertInfo: i32 = 1i32;
pub const HttpServiceConfigUrlAclInfo: i32 = 2i32;
pub const HttpServiceConfigTimeout: i32 = 3i32;
pub const HttpServiceConfigCache: i32 = 4i32;
pub const HttpServiceConfigSslSniCertInfo: i32 = 5i32;
pub const HttpServiceConfigSslCcsCertInfo: i32 = 6i32;
pub const HttpServiceConfigSetting: i32 = 7i32;
pub const HttpServiceConfigSslCertInfoEx: i32 = 8i32;
pub const HttpServiceConfigSslSniCertInfoEx: i32 = 9i32;
pub const HttpServiceConfigSslCcsCertInfoEx: i32 = 10i32;
pub const HttpServiceConfigSslScopedCcsCertInfo: i32 = 11i32;
pub const HttpServiceConfigSslScopedCcsCertInfoEx: i32 = 12i32;
pub const HttpServiceConfigMax: i32 = 13i32;
#[repr(C)]
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
#[repr(C)]
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
pub const HttpServiceConfigQueryExact: i32 = 0i32;
pub const HttpServiceConfigQueryNext: i32 = 1i32;
pub const HttpServiceConfigQueryMax: i32 = 2i32;
pub const HttpNone: i32 = 0i32;
pub const HttpTlsThrottle: i32 = 1i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: u32 = 16u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: u32 = 1024u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: u32 = 128u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: u32 = 32u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: u32 = 4096u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: u32 = 64u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: u32 = 8192u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: u32 = 2048u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: u32 = 256u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: u32 = 512u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: u32 = 2u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: u32 = 4u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: u32 = 8u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: u32 = 1u32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: u32,
    pub pSslHash: *mut ::core::ffi::c_void,
    pub AppId: ::windows_sys::core::GUID,
    pub pSslCertStoreName: super::super::Foundation::PWSTR,
    pub DefaultCertCheckMode: u32,
    pub DefaultRevocationFreshnessTime: u32,
    pub DefaultRevocationUrlRetrievalTimeout: u32,
    pub pDefaultSslCtlIdentifier: super::super::Foundation::PWSTR,
    pub pDefaultSslCtlStoreName: super::super::Foundation::PWSTR,
    pub DefaultFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: u64,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    pub Http2WindowSizeParam: HTTP2_WINDOW_SIZE_PARAM,
    pub Http2SettingsLimitsParam: HTTP2_SETTINGS_LIMITS_PARAM,
    pub HttpPerformanceParam: HTTP_PERFORMANCE_PARAM,
    pub HttpTlsRestrictionsParam: HTTP_TLS_RESTRICTIONS_PARAM,
    pub HttpErrorHeadersParam: HTTP_ERROR_HEADERS_PARAM,
    pub HttpTlsSessionTicketKeysParam: HTTP_TLS_SESSION_TICKET_KEYS_PARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
    pub Host: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const IdleConnectionTimeout: i32 = 0i32;
pub const HeaderWaitTimeout: i32 = 1i32;
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_KEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_SERVICE_CONFIG_URLACL_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_SSL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const ExParamTypeHttp2Window: i32 = 0i32;
pub const ExParamTypeHttp2SettingsLimits: i32 = 1i32;
pub const ExParamTypeHttpPerformance: i32 = 2i32;
pub const ExParamTypeTlsRestrictions: i32 = 3i32;
pub const ExParamTypeErrorHeaders: i32 = 4i32;
pub const ExParamTypeTlsSessionTicketKeys: i32 = 5i32;
pub const ExParamTypeMax: i32 = 6i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: u16,
    pub RawValueLength: u16,
    pub pName: super::super::Foundation::PSTR,
    pub pRawValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_UNKNOWN_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_UNKNOWN_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
pub const HttpVerbUnparsed: i32 = 0i32;
pub const HttpVerbUnknown: i32 = 1i32;
pub const HttpVerbInvalid: i32 = 2i32;
pub const HttpVerbOPTIONS: i32 = 3i32;
pub const HttpVerbGET: i32 = 4i32;
pub const HttpVerbHEAD: i32 = 5i32;
pub const HttpVerbPOST: i32 = 6i32;
pub const HttpVerbPUT: i32 = 7i32;
pub const HttpVerbDELETE: i32 = 8i32;
pub const HttpVerbTRACE: i32 = 9i32;
pub const HttpVerbCONNECT: i32 = 10i32;
pub const HttpVerbTRACK: i32 = 11i32;
pub const HttpVerbMOVE: i32 = 12i32;
pub const HttpVerbCOPY: i32 = 13i32;
pub const HttpVerbPROPFIND: i32 = 14i32;
pub const HttpVerbPROPPATCH: i32 = 15i32;
pub const HttpVerbMKCOL: i32 = 16i32;
pub const HttpVerbLOCK: i32 = 17i32;
pub const HttpVerbUNLOCK: i32 = 18i32;
pub const HttpVerbSEARCH: i32 = 19i32;
pub const HttpVerbMaximum: i32 = 20i32;
#[repr(C)]
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
#[repr(C)]
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
