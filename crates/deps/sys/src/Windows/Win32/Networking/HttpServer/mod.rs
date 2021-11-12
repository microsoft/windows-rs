#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct HTTP2_SETTINGS_LIMITS_PARAM(i32);
#[repr(C)]
pub struct HTTP2_WINDOW_SIZE_PARAM(i32);
#[repr(C)]
pub struct HTTPAPI_VERSION(i32);
#[repr(transparent)]
pub struct HTTP_503_RESPONSE_VERBOSITY(pub i32);
pub const Http503ResponseVerbosityBasic: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(0i32);
pub const Http503ResponseVerbosityLimited: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(1i32);
pub const Http503ResponseVerbosityFull: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(2i32);
#[repr(transparent)]
pub struct HTTP_AUTHENTICATION_HARDENING_LEVELS(pub i32);
pub const HttpAuthenticationHardeningLegacy: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(0i32);
pub const HttpAuthenticationHardeningMedium: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(1i32);
pub const HttpAuthenticationHardeningStrict: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(2i32);
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1u32;
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2u32;
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16u32;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8u32;
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4u32;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2u32;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1u32;
#[repr(transparent)]
pub struct HTTP_AUTH_STATUS(pub i32);
pub const HttpAuthStatusSuccess: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(0i32);
pub const HttpAuthStatusNotAuthenticated: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(1i32);
pub const HttpAuthStatusFailure: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(2i32);
#[repr(C)]
pub struct HTTP_BANDWIDTH_LIMIT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_BINDING_INFO(i32);
#[repr(C)]
pub struct HTTP_BYTE_RANGE(i32);
#[repr(C)]
pub struct HTTP_CACHE_POLICY(i32);
#[repr(transparent)]
pub struct HTTP_CACHE_POLICY_TYPE(pub i32);
pub const HttpCachePolicyNocache: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(0i32);
pub const HttpCachePolicyUserInvalidates: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(1i32);
pub const HttpCachePolicyTimeToLive: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(2i32);
pub const HttpCachePolicyMaximum: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(3i32);
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
#[repr(C)]
pub struct HTTP_CHANNEL_BIND_INFO(i32);
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
#[repr(C)]
pub struct HTTP_CONNECTION_LIMIT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_COOKED_URL(i32);
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
#[repr(transparent)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(pub i32);
pub const CreateRequestQueueExternalIdProperty: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(1i32);
pub const CreateRequestQueueMax: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(2i32);
#[repr(C)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_DATA_CHUNK(i32);
#[repr(transparent)]
pub struct HTTP_DATA_CHUNK_TYPE(pub i32);
pub const HttpDataChunkFromMemory: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(0i32);
pub const HttpDataChunkFromFileHandle: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(1i32);
pub const HttpDataChunkFromFragmentCache: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(2i32);
pub const HttpDataChunkFromFragmentCacheEx: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(3i32);
pub const HttpDataChunkTrailers: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(4i32);
pub const HttpDataChunkMaximum: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(5i32);
#[repr(transparent)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_ID(pub i32);
pub const DelegateRequestReservedProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(0i32);
pub const DelegateRequestDelegateUrlProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(1i32);
#[repr(C)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO(i32);
pub const HTTP_DEMAND_CBT: u32 = 4u32;
#[repr(transparent)]
pub struct HTTP_ENABLED_STATE(pub i32);
pub const HttpEnabledStateActive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(0i32);
pub const HttpEnabledStateInactive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_ERROR_HEADERS_PARAM(i32);
#[repr(transparent)]
pub struct HTTP_FEATURE_ID(pub i32);
pub const HttpFeatureUnknown: HTTP_FEATURE_ID = HTTP_FEATURE_ID(0i32);
pub const HttpFeatureResponseTrailers: HTTP_FEATURE_ID = HTTP_FEATURE_ID(1i32);
pub const HttpFeatureApiTimings: HTTP_FEATURE_ID = HTTP_FEATURE_ID(2i32);
pub const HttpFeatureDelegateEx: HTTP_FEATURE_ID = HTTP_FEATURE_ID(3i32);
pub const HttpFeatureHttp3: HTTP_FEATURE_ID = HTTP_FEATURE_ID(4i32);
pub const HttpFeaturemax: HTTP_FEATURE_ID = HTTP_FEATURE_ID(-1i32);
#[repr(C)]
pub struct HTTP_FLOWRATE_INFO(i32);
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
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
#[repr(transparent)]
pub struct HTTP_INITIALIZE(pub u32);
pub const HTTP_INITIALIZE_CONFIG: HTTP_INITIALIZE = HTTP_INITIALIZE(2u32);
pub const HTTP_INITIALIZE_SERVER: HTTP_INITIALIZE = HTTP_INITIALIZE(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_KNOWN_HEADER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_LISTEN_ENDPOINT_INFO(i32);
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct HTTP_LOGGING_INFO(i32);
#[repr(transparent)]
pub struct HTTP_LOGGING_ROLLOVER_TYPE(pub i32);
pub const HttpLoggingRolloverSize: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(0i32);
pub const HttpLoggingRolloverDaily: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(1i32);
pub const HttpLoggingRolloverWeekly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(2i32);
pub const HttpLoggingRolloverMonthly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(3i32);
pub const HttpLoggingRolloverHourly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(4i32);
#[repr(transparent)]
pub struct HTTP_LOGGING_TYPE(pub i32);
pub const HttpLoggingTypeW3C: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(0i32);
pub const HttpLoggingTypeIIS: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(1i32);
pub const HttpLoggingTypeNCSA: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(2i32);
pub const HttpLoggingTypeRaw: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(3i32);
#[repr(C)]
pub struct HTTP_LOG_DATA(i32);
#[repr(transparent)]
pub struct HTTP_LOG_DATA_TYPE(pub i32);
pub const HttpLogDataTypeFields: HTTP_LOG_DATA_TYPE = HTTP_LOG_DATA_TYPE(0i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_LOG_FIELDS_DATA(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS(i32);
#[repr(C)]
pub struct HTTP_PERFORMANCE_PARAM(i32);
#[repr(transparent)]
pub struct HTTP_PERFORMANCE_PARAM_TYPE(pub i32);
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(0i32);
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(1i32);
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(2i32);
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(3i32);
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(4i32);
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(5i32);
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(6i32);
#[repr(C)]
pub struct HTTP_PROPERTY_FLAGS(i32);
#[repr(C)]
pub struct HTTP_PROTECTION_LEVEL_INFO(i32);
#[repr(transparent)]
pub struct HTTP_PROTECTION_LEVEL_TYPE(pub i32);
pub const HttpProtectionLevelUnrestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(0i32);
pub const HttpProtectionLevelEdgeRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(1i32);
pub const HttpProtectionLevelRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(2i32);
#[repr(C)]
pub struct HTTP_QOS_SETTING_INFO(i32);
#[repr(transparent)]
pub struct HTTP_QOS_SETTING_TYPE(pub i32);
pub const HttpQosSettingTypeBandwidth: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(0i32);
pub const HttpQosSettingTypeConnectionLimit: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(1i32);
pub const HttpQosSettingTypeFlowRate: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(2i32);
#[repr(C)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC(i32);
#[repr(C)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP(i32);
#[repr(C)]
pub struct HTTP_QUIC_API_TIMINGS(i32);
#[repr(C)]
pub struct HTTP_QUIC_CONNECTION_API_TIMINGS(i32);
#[repr(C)]
pub struct HTTP_QUIC_STREAM_API_TIMINGS(i32);
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
#[repr(transparent)]
pub struct HTTP_RECEIVE_HTTP_REQUEST_FLAGS(pub u32);
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(1u32);
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(2u32);
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_REQUEST_AUTH_INFO(i32);
#[repr(transparent)]
pub struct HTTP_REQUEST_AUTH_TYPE(pub i32);
pub const HttpRequestAuthTypeNone: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(0i32);
pub const HttpRequestAuthTypeBasic: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(1i32);
pub const HttpRequestAuthTypeDigest: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(2i32);
pub const HttpRequestAuthTypeNTLM: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(3i32);
pub const HttpRequestAuthTypeNegotiate: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(4i32);
pub const HttpRequestAuthTypeKerberos: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(5i32);
#[repr(C)]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS(i32);
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_REQUEST_HEADERS(i32);
#[repr(C)]
pub struct HTTP_REQUEST_INFO(i32);
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
#[repr(C)]
pub struct HTTP_REQUEST_PROPERTY_SNI(i32);
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR(i32);
#[repr(C)]
pub struct HTTP_REQUEST_SIZING_INFO(i32);
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
#[repr(transparent)]
pub struct HTTP_REQUEST_SIZING_TYPE(pub i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(0i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(1i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(2i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(3i32);
pub const HttpRequestSizingTypeHeaders: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(4i32);
pub const HttpRequestSizingTypeMax: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(5i32);
#[repr(C)]
pub struct HTTP_REQUEST_TIMING_INFO(i32);
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
#[repr(C)]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_REQUEST_V1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_REQUEST_V2(i32);
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_RESPONSE_HEADERS(i32);
#[repr(C)]
pub struct HTTP_RESPONSE_INFO(i32);
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
#[repr(transparent)]
pub struct HTTP_RESPONSE_INFO_TYPE(pub i32);
pub const HttpResponseInfoTypeMultipleKnownHeaders: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(0i32);
pub const HttpResponseInfoTypeAuthenticationProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(1i32);
pub const HttpResponseInfoTypeQoSProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(2i32);
pub const HttpResponseInfoTypeChannelBind: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_RESPONSE_V1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_RESPONSE_V2(i32);
#[repr(transparent)]
pub struct HTTP_SCHEME(pub i32);
pub const HttpSchemeHttp: HTTP_SCHEME = HTTP_SCHEME(0i32);
pub const HttpSchemeHttps: HTTP_SCHEME = HTTP_SCHEME(1i32);
pub const HttpSchemeMaximum: HTTP_SCHEME = HTTP_SCHEME(2i32);
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4u32;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1u32;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8u32;
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256u32;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2u32;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64u32;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVER_AUTHENTICATION_INFO(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_BINDING_A(i32);
#[repr(C)]
pub struct HTTP_SERVICE_BINDING_BASE(i32);
#[repr(transparent)]
pub struct HTTP_SERVICE_BINDING_TYPE(pub i32);
pub const HttpServiceBindingTypeNone: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(0i32);
pub const HttpServiceBindingTypeW: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(1i32);
pub const HttpServiceBindingTypeA: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_BINDING_W(i32);
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_CACHE_KEY(pub i32);
pub const MaxCacheResponseSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(0i32);
pub const CacheRangeChunkSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(1i32);
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET(i32);
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY(i32);
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_QUERY_TYPE(pub i32);
pub const HttpServiceConfigQueryExact: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(0i32);
pub const HttpServiceConfigQueryNext: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(1i32);
pub const HttpServiceConfigQueryMax: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(2i32);
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_SETTING_KEY(pub i32);
pub const HttpNone: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(0i32);
pub const HttpTlsThrottle: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(1i32);
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX(i32);
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX(i32);
#[repr(transparent)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_KEY(pub i32);
pub const IdleConnectionTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(0i32);
pub const HeaderWaitTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(1i32);
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SSL_CLIENT_CERT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_SSL_INFO(i32);
#[repr(C)]
pub struct HTTP_SSL_PROTOCOL_INFO(i32);
#[repr(transparent)]
pub struct HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(pub i32);
pub const ExParamTypeHttp2Window: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(0i32);
pub const ExParamTypeHttp2SettingsLimits: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(1i32);
pub const ExParamTypeHttpPerformance: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(2i32);
pub const ExParamTypeTlsRestrictions: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(3i32);
pub const ExParamTypeErrorHeaders: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(4i32);
pub const ExParamTypeTlsSessionTicketKeys: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(5i32);
pub const ExParamTypeMax: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(6i32);
#[repr(C)]
pub struct HTTP_STATE_INFO(i32);
#[repr(C)]
pub struct HTTP_TIMEOUT_LIMIT_INFO(i32);
#[repr(C)]
pub struct HTTP_TLS_RESTRICTIONS_PARAM(i32);
#[repr(C)]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct HTTP_TRANSPORT_ADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HTTP_UNKNOWN_HEADER(i32);
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
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
#[repr(C)]
pub struct HTTP_VERSION(i32);
#[repr(C)]
pub struct HTTP_WSK_API_TIMINGS(i32);
