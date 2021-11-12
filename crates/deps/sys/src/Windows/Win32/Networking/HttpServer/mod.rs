#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpAddFragmentToCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, datachunk: *mut HTTP_DATA_CHUNK, cachepolicy: *mut HTTP_CACHE_POLICY, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrlToUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, urlcontext: u64, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpCancelHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCloseRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCloseServerSession(serversessionid: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCloseUrlGroup(urlgroupid: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HttpCreateRequestQueue(version: HTTPAPI_VERSION, name: super::super::Foundation::PWSTR, securityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCreateServerSession(version: HTTPAPI_VERSION, serversessionid: *mut u64, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDeclarePush(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, verb: HTTP_VERB, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PSTR, headers: *const HTTP_REQUEST_HEADERS) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDelegateRequestEx(requestqueuehandle: super::super::Foundation::HANDLE, delegatequeuehandle: super::super::Foundation::HANDLE, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpDeleteServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpFindUrlGroupId(fullyqualifiedurl: super::super::Foundation::PWSTR, requestqueuehandle: super::super::Foundation::HANDLE, urlgroupid: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpFlushResponseCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpGetExtension(version: HTTPAPI_VERSION, extension: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpInitialize(version: HTTPAPI_VERSION, flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpPrepareUrl(reserved: *mut ::core::ffi::c_void, flags: u32, url: super::super::Foundation::PWSTR, preparedurl: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, returnlength: *mut u32, reserved2: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpQueryServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pinput: *const ::core::ffi::c_void, inputlength: u32, poutput: *mut ::core::ffi::c_void, outputlength: u32, preturnlength: *mut u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *mut ::core::ffi::c_void, propertyinformationlength: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReadFragmentFromCache(requestqueuehandle: super::super::Foundation::HANDLE, urlprefix: super::super::Foundation::PWSTR, byterange: *mut HTTP_BYTE_RANGE, buffer: *mut ::core::ffi::c_void, bufferlength: u32, bytesread: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveClientCertificate(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
    pub fn HttpReceiveHttpRequest(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveRequestEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitybuffer: *mut ::core::ffi::c_void, entitybufferlength: u32, bytesreturned: *mut u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrl(requestqueuehandle: super::super::Foundation::HANDLE, fullyqualifiedurl: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrlFromUrlGroup(urlgroupid: u64, pfullyqualifiedurl: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendHttpResponse(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, httpresponse: *mut HTTP_RESPONSE_V2, cachepolicy: *mut HTTP_CACHE_POLICY, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendResponseEntityBody(requestqueuehandle: super::super::Foundation::HANDLE, requestid: u64, flags: u32, entitychunkcount: u16, entitychunks: *const HTTP_DATA_CHUNK, bytessent: *mut u32, reserved1: *mut ::core::ffi::c_void, reserved2: u32, overlapped: *mut super::super::System::IO::OVERLAPPED, logdata: *mut HTTP_LOG_DATA) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetRequestProperty(requestqueuehandle: super::super::Foundation::HANDLE, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: *const ::core::ffi::c_void, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSetRequestQueueProperty(requestqueuehandle: super::super::Foundation::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetServiceConfiguration(servicehandle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const ::core::ffi::c_void, configinformationlength: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const ::core::ffi::c_void, propertyinformationlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpShutdownRequestQueue(requestqueuehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpUpdateServiceConfiguration(handle: super::super::Foundation::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const ::core::ffi::c_void, configinfolength: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDemandStart(requestqueuehandle: super::super::Foundation::HANDLE, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnect(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnectEx(requestqueuehandle: super::super::Foundation::HANDLE, connectionid: u64, reserved: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
}
pub struct HTTP2_SETTINGS_LIMITS_PARAM(i32);
pub struct HTTP2_WINDOW_SIZE_PARAM(i32);
pub struct HTTPAPI_VERSION(i32);
pub struct HTTP_503_RESPONSE_VERBOSITY(i32);
pub struct HTTP_AUTHENTICATION_HARDENING_LEVELS(i32);
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
pub struct HTTP_AUTH_STATUS(i32);
pub struct HTTP_BANDWIDTH_LIMIT_INFO(i32);
pub struct HTTP_BINDING_INFO(i32);
pub struct HTTP_BYTE_RANGE(i32);
pub struct HTTP_CACHE_POLICY(i32);
pub struct HTTP_CACHE_POLICY_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
pub struct HTTP_CHANNEL_BIND_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
pub struct HTTP_CONNECTION_LIMIT_INFO(i32);
pub struct HTTP_COOKED_URL(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(i32);
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO(i32);
pub struct HTTP_DATA_CHUNK(i32);
pub struct HTTP_DATA_CHUNK_TYPE(i32);
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_ID(i32);
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_DEMAND_CBT: u32 = 4u32;
pub struct HTTP_ENABLED_STATE(i32);
pub struct HTTP_ERROR_HEADERS_PARAM(i32);
pub struct HTTP_FEATURE_ID(i32);
pub struct HTTP_FLOWRATE_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
pub struct HTTP_HEADER_ID(i32);
pub struct HTTP_INITIALIZE(i32);
pub struct HTTP_KNOWN_HEADER(i32);
pub struct HTTP_LISTEN_ENDPOINT_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
pub struct HTTP_LOGGING_INFO(i32);
pub struct HTTP_LOGGING_ROLLOVER_TYPE(i32);
pub struct HTTP_LOGGING_TYPE(i32);
pub struct HTTP_LOG_DATA(i32);
pub struct HTTP_LOG_DATA_TYPE(i32);
pub struct HTTP_LOG_FIELDS_DATA(i32);
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
pub struct HTTP_MULTIPLE_KNOWN_HEADERS(i32);
pub struct HTTP_PERFORMANCE_PARAM(i32);
pub struct HTTP_PERFORMANCE_PARAM_TYPE(i32);
pub struct HTTP_PROPERTY_FLAGS(i32);
pub struct HTTP_PROTECTION_LEVEL_INFO(i32);
pub struct HTTP_PROTECTION_LEVEL_TYPE(i32);
pub struct HTTP_QOS_SETTING_INFO(i32);
pub struct HTTP_QOS_SETTING_TYPE(i32);
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC(i32);
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP(i32);
pub struct HTTP_QUIC_API_TIMINGS(i32);
pub struct HTTP_QUIC_CONNECTION_API_TIMINGS(i32);
pub struct HTTP_QUIC_STREAM_API_TIMINGS(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
pub struct HTTP_RECEIVE_HTTP_REQUEST_FLAGS(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
pub struct HTTP_REQUEST_AUTH_INFO(i32);
pub struct HTTP_REQUEST_AUTH_TYPE(i32);
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
pub struct HTTP_REQUEST_HEADERS(i32);
pub struct HTTP_REQUEST_INFO(i32);
pub struct HTTP_REQUEST_INFO_TYPE(i32);
pub struct HTTP_REQUEST_PROPERTY(i32);
pub struct HTTP_REQUEST_PROPERTY_SNI(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR(i32);
pub struct HTTP_REQUEST_SIZING_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
pub struct HTTP_REQUEST_SIZING_TYPE(i32);
pub struct HTTP_REQUEST_TIMING_INFO(i32);
pub struct HTTP_REQUEST_TIMING_TYPE(i32);
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO(i32);
pub struct HTTP_REQUEST_V1(i32);
pub struct HTTP_REQUEST_V2(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
pub struct HTTP_RESPONSE_HEADERS(i32);
pub struct HTTP_RESPONSE_INFO(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
pub struct HTTP_RESPONSE_INFO_TYPE(i32);
pub struct HTTP_RESPONSE_V1(i32);
pub struct HTTP_RESPONSE_V2(i32);
pub struct HTTP_SCHEME(i32);
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
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS(i32);
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS(i32);
pub struct HTTP_SERVER_AUTHENTICATION_INFO(i32);
pub struct HTTP_SERVER_PROPERTY(i32);
pub struct HTTP_SERVICE_BINDING_A(i32);
pub struct HTTP_SERVICE_BINDING_BASE(i32);
pub struct HTTP_SERVICE_BINDING_TYPE(i32);
pub struct HTTP_SERVICE_BINDING_W(i32);
pub struct HTTP_SERVICE_CONFIG_CACHE_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_CACHE_SET(i32);
pub struct HTTP_SERVICE_CONFIG_ID(i32);
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM(i32);
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY(i32);
pub struct HTTP_SERVICE_CONFIG_QUERY_TYPE(i32);
pub struct HTTP_SERVICE_CONFIG_SETTING_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_SETTING_SET(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX(i32);
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
pub struct HTTP_SERVICE_CONFIG_SSL_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SET(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET(i32);
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX(i32);
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET(i32);
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY(i32);
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM(i32);
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY(i32);
pub struct HTTP_SERVICE_CONFIG_URLACL_SET(i32);
pub struct HTTP_SSL_CLIENT_CERT_INFO(i32);
pub struct HTTP_SSL_INFO(i32);
pub struct HTTP_SSL_PROTOCOL_INFO(i32);
pub struct HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(i32);
pub struct HTTP_STATE_INFO(i32);
pub struct HTTP_TIMEOUT_LIMIT_INFO(i32);
pub struct HTTP_TLS_RESTRICTIONS_PARAM(i32);
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM(i32);
pub struct HTTP_TRANSPORT_ADDRESS(i32);
pub struct HTTP_UNKNOWN_HEADER(i32);
#[doc = "*Required features: `Win32_Networking_HttpServer`*"]
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
pub struct HTTP_VERB(i32);
pub struct HTTP_VERSION(i32);
pub struct HTTP_WSK_API_TIMINGS(i32);
