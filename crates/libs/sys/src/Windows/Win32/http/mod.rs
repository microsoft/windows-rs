#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpAddFragmentToCache(requestqueuehandle : super::winnt::HANDLE, urlprefix : windows_sys::core::PCWSTR, datachunk : *mut HTTP_DATA_CHUNK, cachepolicy : *mut HTTP_CACHE_POLICY, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpAddUrl(requestqueuehandle : super::winnt::HANDLE, fullyqualifiedurl : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpAddUrlToUrlGroup(urlgroupid : HTTP_URL_GROUP_ID, pfullyqualifiedurl : windows_sys::core::PCWSTR, urlcontext : HTTP_URL_CONTEXT, reserved : u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpCancelHttpRequest(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpCloseRequestQueue(requestqueuehandle : super::winnt::HANDLE) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpCloseServerSession(serversessionid : HTTP_SERVER_SESSION_ID) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpCloseUrlGroup(urlgroupid : HTTP_URL_GROUP_ID) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpCreateHttpHandle(requestqueuehandle : *mut super::winnt::HANDLE, reserved : u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpCreateRequestQueue(version : HTTPAPI_VERSION, name : windows_sys::core::PCWSTR, securityattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, flags : u32, requestqueuehandle : *mut super::winnt::HANDLE) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpCreateServerSession(version : HTTPAPI_VERSION, serversessionid : *mut HTTP_OPAQUE_ID, reserved : u32) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpCreateUrlGroup(serversessionid : HTTP_SERVER_SESSION_ID, purlgroupid : *mut HTTP_OPAQUE_ID, reserved : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpDeclarePush(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, verb : HTTP_VERB, path : windows_sys::core::PCWSTR, query : windows_sys::core::PCSTR, headers : *const HTTP_REQUEST_HEADERS) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpDelegateRequestEx(requestqueuehandle : super::winnt::HANDLE, delegatequeuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, delegateurlgroupid : HTTP_URL_GROUP_ID, propertyinfosetsize : u32, propertyinfoset : *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpDeleteServiceConfiguration(servicehandle : super::winnt::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : *const core::ffi::c_void, configinformationlength : u32, poverlapped : *const super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpFindUrlGroupId(fullyqualifiedurl : windows_sys::core::PCWSTR, requestqueuehandle : super::winnt::HANDLE, urlgroupid : *mut HTTP_OPAQUE_ID) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpFlushResponseCache(requestqueuehandle : super::winnt::HANDLE, urlprefix : windows_sys::core::PCWSTR, flags : u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpGetExtension(version : HTTPAPI_VERSION, extension : u32, buffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpInitialize(version : HTTPAPI_VERSION, flags : u32, preserved : *const core::ffi::c_void) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpIsFeatureSupported(featureid : HTTP_FEATURE_ID) -> windows_sys::core::BOOL);
windows_link::link!("httpapi.dll" "system" fn HttpPrepareUrl(reserved : *const core::ffi::c_void, flags : u32, url : windows_sys::core::PCWSTR, preparedurl : *mut windows_sys::core::PWSTR) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpQueryRequestProperty(requestqueuehandle : super::winnt::HANDLE, id : HTTP_OPAQUE_ID, propertyid : HTTP_REQUEST_PROPERTY, qualifier : *const core::ffi::c_void, qualifiersize : u32, output : *mut core::ffi::c_void, outputbuffersize : u32, bytesreturned : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpQueryRequestQueueProperty(requestqueuehandle : super::winnt::HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, reserved1 : u32, returnlength : *mut u32, reserved2 : *const core::ffi::c_void) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpQueryServerSessionProperty(serversessionid : HTTP_SERVER_SESSION_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, returnlength : *mut u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpQueryServiceConfiguration(servicehandle : super::winnt::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pinput : *const core::ffi::c_void, inputlength : u32, poutput : *mut core::ffi::c_void, outputlength : u32, preturnlength : *mut u32, poverlapped : *const super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpQueryUrlGroupProperty(urlgroupid : HTTP_URL_GROUP_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, returnlength : *mut u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpReadFragmentFromCache(requestqueuehandle : super::winnt::HANDLE, urlprefix : windows_sys::core::PCWSTR, byterange : *mut HTTP_BYTE_RANGE, buffer : *mut core::ffi::c_void, bufferlength : u32, bytesread : *mut u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpReceiveClientCertificate(requestqueuehandle : super::winnt::HANDLE, connectionid : HTTP_CONNECTION_ID, flags : u32, sslclientcertinfo : *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize : u32, bytesreceived : *mut u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
windows_link::link!("httpapi.dll" "system" fn HttpReceiveHttpRequest(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, flags : u32, requestbuffer : *mut HTTP_REQUEST, requestbufferlength : u32, bytesreturned : *mut u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpReceiveRequestEntityBody(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, flags : u32, entitybuffer : *mut core::ffi::c_void, entitybufferlength : u32, bytesreturned : *mut u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpRemoveUrl(requestqueuehandle : super::winnt::HANDLE, fullyqualifiedurl : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpRemoveUrlFromUrlGroup(urlgroupid : HTTP_URL_GROUP_ID, pfullyqualifiedurl : windows_sys::core::PCWSTR, flags : u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpSendHttpResponse(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, flags : u32, httpresponse : *mut HTTP_RESPONSE, cachepolicy : *mut HTTP_CACHE_POLICY, bytessent : *mut u32, reserved1 : *const core::ffi::c_void, reserved2 : u32, overlapped : *mut super::minwinbase::OVERLAPPED, logdata : *mut HTTP_LOG_DATA) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpSendResponseEntityBody(requestqueuehandle : super::winnt::HANDLE, requestid : HTTP_REQUEST_ID, flags : u32, entitychunkcount : u16, entitychunks : *const HTTP_DATA_CHUNK, bytessent : *mut u32, reserved1 : *const core::ffi::c_void, reserved2 : u32, overlapped : *mut super::minwinbase::OVERLAPPED, logdata : *mut HTTP_LOG_DATA) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpSetRequestProperty(requestqueuehandle : super::winnt::HANDLE, id : HTTP_OPAQUE_ID, propertyid : HTTP_REQUEST_PROPERTY, input : *const core::ffi::c_void, inputpropertysize : u32, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpSetRequestQueueProperty(requestqueuehandle : super::winnt::HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpSetServerSessionProperty(serversessionid : HTTP_SERVER_SESSION_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpSetServiceConfiguration(servicehandle : super::winnt::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : *const core::ffi::c_void, configinformationlength : u32, poverlapped : *const super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpSetUrlGroupProperty(urlgroupid : HTTP_URL_GROUP_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("httpapi.dll" "system" fn HttpShutdownRequestQueue(requestqueuehandle : super::winnt::HANDLE) -> u32);
windows_link::link!("httpapi.dll" "system" fn HttpTerminate(flags : u32, preserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpUpdateServiceConfiguration(handle : super::winnt::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, configinfo : *const core::ffi::c_void, configinfolength : u32, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpWaitForDemandStart(requestqueuehandle : super::winnt::HANDLE, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpWaitForDisconnect(requestqueuehandle : super::winnt::HANDLE, connectionid : HTTP_CONNECTION_ID, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("httpapi.dll" "system" fn HttpWaitForDisconnectEx(requestqueuehandle : super::winnt::HANDLE, connectionid : HTTP_CONNECTION_ID, reserved : u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
pub const CacheRangeChunkSize: HTTP_SERVICE_CONFIG_CACHE_KEY = 1;
pub const CreateRequestQueueExternalIdProperty: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = 1;
pub const CreateRequestQueueMax: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = 2;
pub const DelegateRequestDelegateUrlProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = 1;
pub const DelegateRequestReservedProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = 0;
pub const ExParamTypeCertConfig: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 6;
pub const ExParamTypeErrorHeaders: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 4;
pub const ExParamTypeHttp2SettingsLimits: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 1;
pub const ExParamTypeHttp2Window: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 0;
pub const ExParamTypeHttpPerformance: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 2;
pub const ExParamTypeMax: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 7;
pub const ExParamTypeTlsRestrictions: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 3;
pub const ExParamTypeTlsSessionTicketKeys: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP2_SETTINGS_LIMITS_PARAM {
    pub Http2MaxSettingsPerFrame: u32,
    pub Http2MaxSettingsPerMinute: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: u16,
    pub HttpApiMinorVersion: u16,
}
pub type HTTP_503_RESPONSE_VERBOSITY = i32;
pub type HTTP_AUTHENTICATION_HARDENING_LEVELS = i32;
pub const HTTP_AUTH_ENABLE_ALL: u32 = 31;
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1;
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2;
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8;
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1;
pub type HTTP_AUTH_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: u64,
    pub Length: u64,
}
pub const HTTP_BYTE_RANGE_TO_EOF: u64 = 18446744073709551615;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: u32,
}
pub type HTTP_CACHE_POLICY_TYPE = i32;
pub const HTTP_CERT_CHECK_MODE_CACHED_REVOCATION: u32 = 2;
pub const HTTP_CERT_CHECK_MODE_CACHED_URLS: u32 = 8;
pub const HTTP_CERT_CHECK_MODE_NO_AIA: u32 = 16;
pub const HTTP_CERT_CHECK_MODE_NO_REVOCATION: u32 = 1;
pub const HTTP_CERT_CHECK_MODE_NO_USAGE_CHECK: u32 = 65536;
pub const HTTP_CERT_CHECK_MODE_USE_REVOCATION_FRESHNESS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_CERT_CONFIG_ENTRY {
    pub CertHash: [u8; 20],
    pub CertStoreName: [u16; 128],
}
impl Default for HTTP_CERT_CONFIG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_CERT_CONFIG_PARAM {
    pub CertConfigCount: u32,
    pub CertConfigs: PHTTP_CERT_CONFIG_ENTRY,
}
impl Default for HTTP_CERT_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: u32,
    pub ServiceNames: *mut PHTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: u32,
}
impl Default for HTTP_CHANNEL_BIND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2;
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8;
pub type HTTP_CLIENT_CONNECTION_ID = HTTP_OPAQUE_ID;
pub type HTTP_CLIENT_CREDENTIAL_ID = HTTP_OPAQUE_ID;
pub type HTTP_CLIENT_REQUEST_ID = HTTP_OPAQUE_ID;
pub type HTTP_CLIENT_STREAM_ID = HTTP_OPAQUE_ID;
pub type HTTP_CONNECTION_ID = HTTP_OPAQUE_ID;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: u16,
    pub HostLength: u16,
    pub AbsPathLength: u16,
    pub QueryStringLength: u16,
    pub pFullUrl: windows_sys::core::PCWSTR,
    pub pHost: windows_sys::core::PCWSTR,
    pub pAbsPath: windows_sys::core::PCWSTR,
    pub pQueryString: windows_sys::core::PCWSTR,
}
impl Default for HTTP_COOKED_URL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1;
pub type HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    pub PropertyId: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut core::ffi::c_void,
}
impl Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub Anonymous: HTTP_DATA_CHUNK_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union HTTP_DATA_CHUNK_0 {
    pub FromMemory: HTTP_DATA_CHUNK_0_0,
    pub FromFileHandle: HTTP_DATA_CHUNK_0_1,
    pub FromFragmentCache: HTTP_DATA_CHUNK_0_2,
    pub FromFragmentCacheEx: HTTP_DATA_CHUNK_0_3,
    pub Trailers: HTTP_DATA_CHUNK_0_4,
    pub FromWinHttpFastForwarding: HTTP_DATA_CHUNK_0_5,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_0 {
    pub pBuffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: u16,
    pub pFragmentName: windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_3 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_4 {
    pub TrailerCount: u16,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_DATA_CHUNK_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_DATA_CHUNK_0_5 {
    pub WhFastForwardingData: HTTP_WINHTTP_FAST_FORWARDING_DATA,
}
pub type HTTP_DATA_CHUNK_TYPE = i32;
pub type HTTP_DELEGATE_REQUEST_PROPERTY_ID = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    pub PropertyId: HTTP_DELEGATE_REQUEST_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut core::ffi::c_void,
}
impl Default for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_DEMAND_CBT: u32 = 4;
pub type HTTP_ENABLED_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: u16,
    pub HeaderCount: u16,
    pub Headers: PHTTP_UNKNOWN_HEADER,
}
impl Default for HTTP_ERROR_HEADERS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_FAST_FORWARD_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableFastForwarding: bool,
}
pub type HTTP_FEATURE_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
    pub MaxPeakBandwidth: u32,
    pub BurstSize: u32,
}
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1;
pub type HTTP_HEADER_ID = i32;
pub const HTTP_INITIALIZE_CONFIG: u32 = 2;
pub const HTTP_INITIALIZE_SERVER: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: u16,
    pub pRawValue: windows_sys::core::PCSTR,
}
impl Default for HTTP_KNOWN_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_LIMIT_INFINITE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: bool,
}
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: u32,
    pub SoftwareName: windows_sys::core::PCWSTR,
    pub SoftwareNameLength: u16,
    pub DirectoryNameLength: u16,
    pub DirectoryName: windows_sys::core::PCWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: u32,
    pub pExtFields: *mut core::ffi::c_void,
    pub NumOfExtFields: u16,
    pub MaxRecordSize: u16,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: u32,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_LOGGING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_LOGGING_ROLLOVER_TYPE = i32;
pub type HTTP_LOGGING_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
pub type HTTP_LOG_DATA_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
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
    pub UserName: super::winnt::PWCHAR,
    pub UriStem: super::winnt::PWCHAR,
    pub ClientIp: super::winnt::PCHAR,
    pub ServerName: super::winnt::PCHAR,
    pub ServiceName: super::winnt::PCHAR,
    pub ServerIp: super::winnt::PCHAR,
    pub Method: super::winnt::PCHAR,
    pub UriQuery: super::winnt::PCHAR,
    pub Host: super::winnt::PCHAR,
    pub UserAgent: super::winnt::PCHAR,
    pub Cookie: super::winnt::PCHAR,
    pub Referrer: super::winnt::PCHAR,
    pub ServerPort: u16,
    pub ProtocolStatus: u16,
    pub Win32Status: u32,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: u16,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_LOG_FIELDS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_LOG_FIELD_BYTES_RECV: u32 = 8192;
pub const HTTP_LOG_FIELD_BYTES_SENT: u32 = 4096;
pub const HTTP_LOG_FIELD_CLIENT_IP: u32 = 4;
pub const HTTP_LOG_FIELD_CLIENT_PORT: u32 = 4194304;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: u32 = 32;
pub const HTTP_LOG_FIELD_COOKIE: u32 = 131072;
pub const HTTP_LOG_FIELD_CORRELATION_ID: u32 = 1073741824;
pub const HTTP_LOG_FIELD_DATE: u32 = 1;
pub const HTTP_LOG_FIELD_EXT_FAULT_CODE_EXT: u32 = 1;
pub const HTTP_LOG_FIELD_FAULT_CODE: u32 = 2147483648;
pub const HTTP_LOG_FIELD_HOST: u32 = 1048576;
pub const HTTP_LOG_FIELD_METHOD: u32 = 128;
pub const HTTP_LOG_FIELD_QUEUE_NAME: u32 = 67108864;
pub const HTTP_LOG_FIELD_REASON: u32 = 33554432;
pub const HTTP_LOG_FIELD_REFERER: u32 = 262144;
pub const HTTP_LOG_FIELD_SERVER_IP: u32 = 64;
pub const HTTP_LOG_FIELD_SERVER_PORT: u32 = 32768;
pub const HTTP_LOG_FIELD_SITE_ID: u32 = 16777216;
pub const HTTP_LOG_FIELD_SITE_NAME: u32 = 16;
pub const HTTP_LOG_FIELD_STATUS: u32 = 1024;
pub const HTTP_LOG_FIELD_STREAM_ID: u32 = 134217728;
pub const HTTP_LOG_FIELD_STREAM_ID_EX: u32 = 268435456;
pub const HTTP_LOG_FIELD_SUB_STATUS: u32 = 2097152;
pub const HTTP_LOG_FIELD_TIME: u32 = 2;
pub const HTTP_LOG_FIELD_TIME_TAKEN: u32 = 16384;
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: u32 = 536870912;
pub const HTTP_LOG_FIELD_URI: u32 = 8388608;
pub const HTTP_LOG_FIELD_URI_QUERY: u32 = 512;
pub const HTTP_LOG_FIELD_URI_STEM: u32 = 256;
pub const HTTP_LOG_FIELD_USER_AGENT: u32 = 65536;
pub const HTTP_LOG_FIELD_USER_NAME: u32 = 8;
pub const HTTP_LOG_FIELD_VERSION: u32 = 524288;
pub const HTTP_LOG_FIELD_WIN32_STATUS: u32 = 2048;
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: u32 = 2147483647;
pub const HTTP_MIN_ALLOWED_BANDWIDTH_THROTTLING_RATE: u32 = 1024;
pub const HTTP_MIN_ALLOWED_LOG_FILE_ROLLOVER_SIZE: u32 = 1048576;
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: u32,
    pub KnownHeaderCount: u16,
    pub KnownHeaders: PHTTP_KNOWN_HEADER,
}
impl Default for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_NULL_ID: u32 = 0;
pub type HTTP_OPAQUE_ID = u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_PERFORMANCE_PARAM {
    pub Type: HTTP_PERFORMANCE_PARAM_TYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
impl Default for HTTP_PERFORMANCE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_PERFORMANCE_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
pub type HTTP_PROTECTION_LEVEL_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: *mut core::ffi::c_void,
}
impl Default for HTTP_QOS_SETTING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_QOS_SETTING_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_QUIC_API_TIMINGS {
    pub ConnectionTimings: HTTP_QUIC_CONNECTION_API_TIMINGS,
    pub StreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const HTTP_QUIC_KEEPALIVE_TIMEOUT_DISABLED: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_QUIC_STREAM_REQUEST_STATS {
    pub StreamWaitStart: u64,
    pub StreamWaitEnd: u64,
    pub RequestHeadersCompressionStart: u64,
    pub RequestHeadersCompressionEnd: u64,
    pub ResponseHeadersDecompressionStart: u64,
    pub ResponseHeadersDecompressionEnd: u64,
    pub RequestHeadersCompressedSize: u64,
    pub ResponseHeadersCompressedSize: u64,
}
pub type HTTP_RAW_CONNECTION_ID = HTTP_OPAQUE_ID;
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: u32 = 1;
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: u32 = 2;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
pub type HTTP_REQUEST = HTTP_REQUEST_V2;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_ncrypt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: super::ncrypt::SECURITY_STATUS,
    pub Flags: u32,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: super::winnt::HANDLE,
    pub ContextAttributes: u32,
    pub PackedContextLength: u32,
    pub PackedContextType: u32,
    pub PackedContext: *mut core::ffi::c_void,
    pub MutualAuthDataLength: u32,
    pub pMutualAuthData: super::winnt::PCHAR,
    pub PackageNameLength: u16,
    pub pPackageName: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_ncrypt", feature = "Win32_winnt"))]
impl Default for HTTP_REQUEST_AUTH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_REQUEST_AUTH_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: PHTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: super::minwindef::PUCHAR,
    pub ChannelTokenSize: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_REQUEST_DSCP_INFO {
    pub DscpTag: u8,
}
pub const HTTP_REQUEST_FLAG_FAST_FORWARDING_ALLOWED: u32 = 16;
pub const HTTP_REQUEST_FLAG_FAST_FORWARDING_RESPONSE_ALLOWED: u32 = 16;
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4;
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
impl Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_REQUEST_ID = HTTP_OPAQUE_ID;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: u32,
    pub pInfo: *mut core::ffi::c_void,
}
impl Default for HTTP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_REQUEST_INFO_FLAG_INITIAL_TTL: u64 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_REQUEST_INFO_PROPERTY_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestInfoFlags: u64,
}
pub type HTTP_REQUEST_INFO_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_REQUEST_INITIAL_PACKET_TTL_INFO {
    pub InitialPacketTtl: u8,
}
pub type HTTP_REQUEST_PROPERTY = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_PROPERTY_SNI {
    pub Hostname: [u16; 256],
    pub Flags: u32,
}
impl Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1;
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_SIZING_INFO {
    pub Flags: u64,
    pub RequestIndex: u32,
    pub RequestSizingCount: u32,
    pub RequestSizing: [u64; 5],
}
impl Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2;
pub type HTTP_REQUEST_SIZING_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_TIMING_INFO {
    pub RequestTimingCount: u32,
    pub RequestTiming: [u64; 30],
}
impl Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_REQUEST_TIMING_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO {
    pub TokenBinding: super::minwindef::PUCHAR,
    pub TokenBindingSize: u32,
    pub EKM: super::minwindef::PUCHAR,
    pub EKMSize: u32,
    pub KeyType: u8,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO {
    pub TransportIdleConnectionTimeout: u16,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_V1 {
    pub Flags: u32,
    pub ConnectionId: HTTP_CONNECTION_ID,
    pub RequestId: HTTP_REQUEST_ID,
    pub UrlContext: HTTP_URL_CONTEXT,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: u16,
    pub RawUrlLength: u16,
    pub pUnknownVerb: windows_sys::core::PCSTR,
    pub pRawUrl: windows_sys::core::PCSTR,
    pub CookedUrl: HTTP_COOKED_URL,
    pub Address: HTTP_TRANSPORT_ADDRESS,
    pub Headers: HTTP_REQUEST_HEADERS,
    pub BytesReceived: u64,
    pub EntityChunkCount: u16,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
    pub RawConnectionId: HTTP_RAW_CONNECTION_ID,
    pub pSslInfo: PHTTP_SSL_INFO,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
impl Default for HTTP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
#[derive(Clone, Copy)]
pub struct HTTP_REQUEST_V2 {
    pub Base: HTTP_REQUEST_V1,
    pub RequestInfoCount: u16,
    pub pRequestInfo: PHTTP_REQUEST_INFO,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
impl Default for HTTP_REQUEST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
pub type HTTP_RESPONSE = HTTP_RESPONSE_V2;
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
impl Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: u32,
    pub pInfo: *mut core::ffi::c_void,
}
impl Default for HTTP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1;
pub type HTTP_RESPONSE_INFO_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: u32,
    pub Version: HTTP_VERSION,
    pub StatusCode: u16,
    pub ReasonLength: u16,
    pub pReason: windows_sys::core::PCSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: u16,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_RESPONSE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_V2 {
    pub Base: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: u16,
    pub pResponseInfo: PHTTP_RESPONSE_INFO,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_RESPONSE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SCHEME = i32;
pub const HTTP_SEND_RESPONSE_FLAG_AUTOMATIC_CHUNKING: u32 = 512;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8;
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: u16,
    pub Realm: windows_sys::core::PWSTR,
}
impl Default for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: u16,
    pub DomainName: windows_sys::core::PWSTR,
    pub RealmLength: u16,
    pub Realm: windows_sys::core::PWSTR,
}
impl Default for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVER_AUTHENTICATION_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub AuthSchemes: u32,
    pub ReceiveMutualAuth: bool,
    pub ReceiveContextHandle: bool,
    pub DisableNTLMCredentialCaching: bool,
    pub ExFlags: u8,
    pub DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    pub BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}
pub type HTTP_SERVER_PROPERTY = i32;
pub type HTTP_SERVER_SESSION_ID = HTTP_OPAQUE_ID;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::winnt::PCHAR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_SERVICE_BINDING_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
pub type HTTP_SERVICE_BINDING_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::winnt::PWCHAR,
    pub BufferSize: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HTTP_SERVICE_BINDING_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SERVICE_CONFIG_CACHE_KEY = i32;
pub type HTTP_SERVICE_CONFIG_CACHE_PARAM = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_CACHE_PARAM,
}
pub type HTTP_SERVICE_CONFIG_ID = i32;
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: u16,
    pub pAddress: super::ws2::PSOCKADDR,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: u32,
    pub AddrList: [super::ws2::SOCKADDR_STORAGE; 1],
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SERVICE_CONFIG_QUERY_TYPE = i32;
pub type HTTP_SERVICE_CONFIG_SETTING_KEY = i32;
pub type HTTP_SERVICE_CONFIG_SETTING_PARAM = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SETTING_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SETTING_PARAM,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: super::ws2::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: u32 = 16;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: u32 = 1024;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: u32 = 128;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: u32 = 32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_SESSION_ID: u32 = 16384;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: u32 = 4096;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: u32 = 64;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CACHE_CLIENT_HELLO: u32 = 32768;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: u32 = 8192;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: u32 = 2048;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: u32 = 256;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: u32 = 512;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: u32 = 2;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: u32 = 8;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: super::ws2::PSOCKADDR,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    pub IpPort: super::ws2::SOCKADDR_STORAGE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: u32,
    pub pSslHash: *mut core::ffi::c_void,
    pub AppId: windows_sys::core::GUID,
    pub pSslCertStoreName: windows_sys::core::PWSTR,
    pub DefaultCertCheckMode: u32,
    pub DefaultRevocationFreshnessTime: u32,
    pub DefaultRevocationUrlRetrievalTimeout: u32,
    pub pDefaultSslCtlIdentifier: windows_sys::core::PWSTR,
    pub pDefaultSslCtlStoreName: windows_sys::core::PWSTR,
    pub DefaultFlags: u32,
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: u64,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    pub Http2WindowSizeParam: HTTP2_WINDOW_SIZE_PARAM,
    pub Http2SettingsLimitsParam: HTTP2_SETTINGS_LIMITS_PARAM,
    pub HttpPerformanceParam: HTTP_PERFORMANCE_PARAM,
    pub HttpTlsRestrictionsParam: HTTP_TLS_RESTRICTIONS_PARAM,
    pub HttpErrorHeadersParam: HTTP_ERROR_HEADERS_PARAM,
    pub HttpTlsSessionTicketKeysParam: HTTP_TLS_SESSION_TICKET_KEYS_PARAM,
    pub HttpCertConfigParam: HTTP_CERT_CONFIG_PARAM,
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::ws2::SOCKADDR_STORAGE,
    pub Host: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SERVICE_CONFIG_TIMEOUT_KEY = i32;
pub type HTTP_SERVICE_CONFIG_TIMEOUT_PARAM = u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_TIMEOUT_PARAM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: windows_sys::core::PWSTR,
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: windows_sys::core::PWSTR,
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
pub const HTTP_SSL_CERT_SHA_HASH_LENGTH: u32 = 20;
pub const HTTP_SSL_CERT_STORE_NAME_LENGTH: u32 = 128;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: u32,
    pub CertEncodedSize: u32,
    pub pCertEncoded: super::minwindef::PUCHAR,
    pub Token: super::winnt::HANDLE,
    pub CertDeniedByMapper: bool,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for HTTP_SSL_CLIENT_CERT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: u16,
    pub ConnectionKeySize: u16,
    pub ServerCertIssuerSize: u32,
    pub ServerCertSubjectSize: u32,
    pub pServerCertIssuer: windows_sys::core::PCSTR,
    pub pServerCertSubject: windows_sys::core::PCSTR,
    pub pClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for HTTP_SSL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_SSL_PROTOCOL_INFO {
    pub Protocol: u32,
    pub CipherType: u32,
    pub CipherStrength: u32,
    pub HashType: u32,
    pub HashStrength: u32,
    pub KeyExchangeType: u32,
    pub KeyExchangeStrength: u32,
}
pub type HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: u16,
    pub DrainEntityBody: u16,
    pub RequestQueue: u16,
    pub IdleConnection: u16,
    pub HeaderWait: u16,
    pub MinSendRate: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_TLS_RESTRICTIONS_PARAM {
    pub RestrictionCount: u32,
    pub TlsRestrictions: *mut core::ffi::c_void,
}
impl Default for HTTP_TLS_RESTRICTIONS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    pub SessionTicketKeyCount: u32,
    pub SessionTicketKeys: *mut core::ffi::c_void,
}
impl Default for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy)]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: super::ws2::PSOCKADDR,
    pub pLocalAddress: super::ws2::PSOCKADDR,
}
#[cfg(feature = "Win32_ws2")]
impl Default for HTTP_TRANSPORT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: u16,
    pub RawValueLength: u16,
    pub pName: windows_sys::core::PCSTR,
    pub pRawValue: windows_sys::core::PCSTR,
}
impl Default for HTTP_UNKNOWN_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_URL_CONTEXT = u64;
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1;
pub type HTTP_URL_GROUP_ID = HTTP_OPAQUE_ID;
pub type HTTP_VERB = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_WINHTTP_FAST_FORWARDING_DATA {
    pub Reserved: [u8; 16],
}
impl Default for HTTP_WINHTTP_FAST_FORWARDING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const HeaderWaitTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = 1;
pub const Http503ResponseVerbosityBasic: HTTP_503_RESPONSE_VERBOSITY = 0;
pub const Http503ResponseVerbosityFull: HTTP_503_RESPONSE_VERBOSITY = 2;
pub const Http503ResponseVerbosityLimited: HTTP_503_RESPONSE_VERBOSITY = 1;
pub const HttpAuthStatusFailure: HTTP_AUTH_STATUS = 2;
pub const HttpAuthStatusNotAuthenticated: HTTP_AUTH_STATUS = 1;
pub const HttpAuthStatusSuccess: HTTP_AUTH_STATUS = 0;
pub const HttpAuthenticationHardeningLegacy: HTTP_AUTHENTICATION_HARDENING_LEVELS = 0;
pub const HttpAuthenticationHardeningMedium: HTTP_AUTHENTICATION_HARDENING_LEVELS = 1;
pub const HttpAuthenticationHardeningStrict: HTTP_AUTHENTICATION_HARDENING_LEVELS = 2;
pub const HttpCachePolicyMaximum: HTTP_CACHE_POLICY_TYPE = 3;
pub const HttpCachePolicyNocache: HTTP_CACHE_POLICY_TYPE = 0;
pub const HttpCachePolicyTimeToLive: HTTP_CACHE_POLICY_TYPE = 2;
pub const HttpCachePolicyUserInvalidates: HTTP_CACHE_POLICY_TYPE = 1;
pub const HttpDataChunkFromFileHandle: HTTP_DATA_CHUNK_TYPE = 1;
pub const HttpDataChunkFromFragmentCache: HTTP_DATA_CHUNK_TYPE = 2;
pub const HttpDataChunkFromFragmentCacheEx: HTTP_DATA_CHUNK_TYPE = 3;
pub const HttpDataChunkFromMemory: HTTP_DATA_CHUNK_TYPE = 0;
pub const HttpDataChunkFromWinHttpFastForwarding: HTTP_DATA_CHUNK_TYPE = 5;
pub const HttpDataChunkMaximum: HTTP_DATA_CHUNK_TYPE = 6;
pub const HttpDataChunkTrailers: HTTP_DATA_CHUNK_TYPE = 4;
pub const HttpEnabledStateActive: HTTP_ENABLED_STATE = 0;
pub const HttpEnabledStateInactive: HTTP_ENABLED_STATE = 1;
pub const HttpFeatureApiTimings: HTTP_FEATURE_ID = 2;
pub const HttpFeatureAutomaticChunkedEncoding: HTTP_FEATURE_ID = 8;
pub const HttpFeatureCacheTlsClientHello: HTTP_FEATURE_ID = 11;
pub const HttpFeatureDedicatedReqQueueDelegationType: HTTP_FEATURE_ID = 9;
pub const HttpFeatureDelegateEx: HTTP_FEATURE_ID = 3;
pub const HttpFeatureDisableAiaFlag: HTTP_FEATURE_ID = 13;
pub const HttpFeatureDisableTlsSessionId: HTTP_FEATURE_ID = 6;
pub const HttpFeatureDscp: HTTP_FEATURE_ID = 14;
pub const HttpFeatureFastForwardResponse: HTTP_FEATURE_ID = 10;
pub const HttpFeatureHttp3: HTTP_FEATURE_ID = 4;
pub const HttpFeatureIdleConnectionTimeoutRequestProperty: HTTP_FEATURE_ID = 12;
pub const HttpFeatureLast: HTTP_FEATURE_ID = 18;
pub const HttpFeatureQueryCipherInfo: HTTP_FEATURE_ID = 15;
pub const HttpFeatureQueryInitialPacketTtl: HTTP_FEATURE_ID = 16;
pub const HttpFeatureResponseTrailers: HTTP_FEATURE_ID = 1;
pub const HttpFeatureTlsDualCerts: HTTP_FEATURE_ID = 7;
pub const HttpFeatureTlsHandshakePerformanceCounters: HTTP_FEATURE_ID = 17;
pub const HttpFeatureTlsSessionTickets: HTTP_FEATURE_ID = 5;
pub const HttpFeatureUnknown: HTTP_FEATURE_ID = 0;
pub const HttpFeaturemax: HTTP_FEATURE_ID = -1;
pub const HttpHeaderAccept: HTTP_HEADER_ID = 20;
pub const HttpHeaderAcceptCharset: HTTP_HEADER_ID = 21;
pub const HttpHeaderAcceptEncoding: HTTP_HEADER_ID = 22;
pub const HttpHeaderAcceptLanguage: HTTP_HEADER_ID = 23;
pub const HttpHeaderAcceptRanges: HTTP_HEADER_ID = 20;
pub const HttpHeaderAge: HTTP_HEADER_ID = 21;
pub const HttpHeaderAllow: HTTP_HEADER_ID = 10;
pub const HttpHeaderAuthorization: HTTP_HEADER_ID = 24;
pub const HttpHeaderCacheControl: HTTP_HEADER_ID = 0;
pub const HttpHeaderConnection: HTTP_HEADER_ID = 1;
pub const HttpHeaderContentEncoding: HTTP_HEADER_ID = 13;
pub const HttpHeaderContentLanguage: HTTP_HEADER_ID = 14;
pub const HttpHeaderContentLength: HTTP_HEADER_ID = 11;
pub const HttpHeaderContentLocation: HTTP_HEADER_ID = 15;
pub const HttpHeaderContentMd5: HTTP_HEADER_ID = 16;
pub const HttpHeaderContentRange: HTTP_HEADER_ID = 17;
pub const HttpHeaderContentType: HTTP_HEADER_ID = 12;
pub const HttpHeaderCookie: HTTP_HEADER_ID = 25;
pub const HttpHeaderDate: HTTP_HEADER_ID = 2;
pub const HttpHeaderEtag: HTTP_HEADER_ID = 22;
pub const HttpHeaderExpect: HTTP_HEADER_ID = 26;
pub const HttpHeaderExpires: HTTP_HEADER_ID = 18;
pub const HttpHeaderFrom: HTTP_HEADER_ID = 27;
pub const HttpHeaderHost: HTTP_HEADER_ID = 28;
pub const HttpHeaderIfMatch: HTTP_HEADER_ID = 29;
pub const HttpHeaderIfModifiedSince: HTTP_HEADER_ID = 30;
pub const HttpHeaderIfNoneMatch: HTTP_HEADER_ID = 31;
pub const HttpHeaderIfRange: HTTP_HEADER_ID = 32;
pub const HttpHeaderIfUnmodifiedSince: HTTP_HEADER_ID = 33;
pub const HttpHeaderKeepAlive: HTTP_HEADER_ID = 3;
pub const HttpHeaderLastModified: HTTP_HEADER_ID = 19;
pub const HttpHeaderLocation: HTTP_HEADER_ID = 23;
pub const HttpHeaderMaxForwards: HTTP_HEADER_ID = 34;
pub const HttpHeaderMaximum: HTTP_HEADER_ID = 41;
pub const HttpHeaderPragma: HTTP_HEADER_ID = 4;
pub const HttpHeaderProxyAuthenticate: HTTP_HEADER_ID = 24;
pub const HttpHeaderProxyAuthorization: HTTP_HEADER_ID = 35;
pub const HttpHeaderRange: HTTP_HEADER_ID = 37;
pub const HttpHeaderReferer: HTTP_HEADER_ID = 36;
pub const HttpHeaderRequestMaximum: HTTP_HEADER_ID = 41;
pub const HttpHeaderResponseMaximum: HTTP_HEADER_ID = 30;
pub const HttpHeaderRetryAfter: HTTP_HEADER_ID = 25;
pub const HttpHeaderServer: HTTP_HEADER_ID = 26;
pub const HttpHeaderSetCookie: HTTP_HEADER_ID = 27;
pub const HttpHeaderTe: HTTP_HEADER_ID = 38;
pub const HttpHeaderTrailer: HTTP_HEADER_ID = 5;
pub const HttpHeaderTransferEncoding: HTTP_HEADER_ID = 6;
pub const HttpHeaderTranslate: HTTP_HEADER_ID = 39;
pub const HttpHeaderUpgrade: HTTP_HEADER_ID = 7;
pub const HttpHeaderUserAgent: HTTP_HEADER_ID = 40;
pub const HttpHeaderVary: HTTP_HEADER_ID = 28;
pub const HttpHeaderVia: HTTP_HEADER_ID = 8;
pub const HttpHeaderWarning: HTTP_HEADER_ID = 9;
pub const HttpHeaderWwwAuthenticate: HTTP_HEADER_ID = 29;
pub const HttpLogDataTypeFields: HTTP_LOG_DATA_TYPE = 0;
pub const HttpLoggingRolloverDaily: HTTP_LOGGING_ROLLOVER_TYPE = 1;
pub const HttpLoggingRolloverHourly: HTTP_LOGGING_ROLLOVER_TYPE = 4;
pub const HttpLoggingRolloverMonthly: HTTP_LOGGING_ROLLOVER_TYPE = 3;
pub const HttpLoggingRolloverSize: HTTP_LOGGING_ROLLOVER_TYPE = 0;
pub const HttpLoggingRolloverWeekly: HTTP_LOGGING_ROLLOVER_TYPE = 2;
pub const HttpLoggingTypeIIS: HTTP_LOGGING_TYPE = 1;
pub const HttpLoggingTypeNCSA: HTTP_LOGGING_TYPE = 2;
pub const HttpLoggingTypeRaw: HTTP_LOGGING_TYPE = 3;
pub const HttpLoggingTypeW3C: HTTP_LOGGING_TYPE = 0;
pub const HttpNone: HTTP_SERVICE_CONFIG_SETTING_KEY = 0;
pub const HttpProtectionLevelEdgeRestricted: HTTP_PROTECTION_LEVEL_TYPE = 1;
pub const HttpProtectionLevelRestricted: HTTP_PROTECTION_LEVEL_TYPE = 2;
pub const HttpProtectionLevelUnrestricted: HTTP_PROTECTION_LEVEL_TYPE = 0;
pub const HttpQosSettingTypeBandwidth: HTTP_QOS_SETTING_TYPE = 0;
pub const HttpQosSettingTypeConnectionLimit: HTTP_QOS_SETTING_TYPE = 1;
pub const HttpQosSettingTypeFlowRate: HTTP_QOS_SETTING_TYPE = 2;
pub const HttpRequestAuthTypeBasic: HTTP_REQUEST_AUTH_TYPE = 1;
pub const HttpRequestAuthTypeDigest: HTTP_REQUEST_AUTH_TYPE = 2;
pub const HttpRequestAuthTypeKerberos: HTTP_REQUEST_AUTH_TYPE = 5;
pub const HttpRequestAuthTypeNTLM: HTTP_REQUEST_AUTH_TYPE = 3;
pub const HttpRequestAuthTypeNegotiate: HTTP_REQUEST_AUTH_TYPE = 4;
pub const HttpRequestAuthTypeNone: HTTP_REQUEST_AUTH_TYPE = 0;
pub const HttpRequestInfoTypeAuth: HTTP_REQUEST_INFO_TYPE = 0;
pub const HttpRequestInfoTypeChannelBind: HTTP_REQUEST_INFO_TYPE = 1;
pub const HttpRequestInfoTypeDscpTag: HTTP_REQUEST_INFO_TYPE = 13;
pub const HttpRequestInfoTypeInitialPacketTtl: HTTP_REQUEST_INFO_TYPE = 14;
pub const HttpRequestInfoTypeQuicStats: HTTP_REQUEST_INFO_TYPE = 8;
pub const HttpRequestInfoTypeQuicStatsV2: HTTP_REQUEST_INFO_TYPE = 10;
pub const HttpRequestInfoTypeRequestSizing: HTTP_REQUEST_INFO_TYPE = 7;
pub const HttpRequestInfoTypeRequestTiming: HTTP_REQUEST_INFO_TYPE = 5;
pub const HttpRequestInfoTypeSslProtocol: HTTP_REQUEST_INFO_TYPE = 2;
pub const HttpRequestInfoTypeSslTokenBinding: HTTP_REQUEST_INFO_TYPE = 4;
pub const HttpRequestInfoTypeSslTokenBindingDraft: HTTP_REQUEST_INFO_TYPE = 3;
pub const HttpRequestInfoTypeTcpInfoV0: HTTP_REQUEST_INFO_TYPE = 6;
pub const HttpRequestInfoTypeTcpInfoV1: HTTP_REQUEST_INFO_TYPE = 9;
pub const HttpRequestInfoTypeTcpInfoV2: HTTP_REQUEST_INFO_TYPE = 11;
pub const HttpRequestInfoTypeTransportIdleConnectionTimeout: HTTP_REQUEST_INFO_TYPE = 12;
pub const HttpRequestPropertyDscpTag: HTTP_REQUEST_PROPERTY = 13;
pub const HttpRequestPropertyIsb: HTTP_REQUEST_PROPERTY = 0;
pub const HttpRequestPropertyQuicApiTimings: HTTP_REQUEST_PROPERTY = 7;
pub const HttpRequestPropertyQuicStats: HTTP_REQUEST_PROPERTY = 2;
pub const HttpRequestPropertyQuicStatsV2: HTTP_REQUEST_PROPERTY = 8;
pub const HttpRequestPropertyQuicStreamStats: HTTP_REQUEST_PROPERTY = 9;
pub const HttpRequestPropertySni: HTTP_REQUEST_PROPERTY = 4;
pub const HttpRequestPropertyStreamError: HTTP_REQUEST_PROPERTY = 5;
pub const HttpRequestPropertyTcpInfoV0: HTTP_REQUEST_PROPERTY = 1;
pub const HttpRequestPropertyTcpInfoV1: HTTP_REQUEST_PROPERTY = 3;
pub const HttpRequestPropertyTcpInfoV2: HTTP_REQUEST_PROPERTY = 10;
pub const HttpRequestPropertyTlsCipherInfo: HTTP_REQUEST_PROPERTY = 14;
pub const HttpRequestPropertyTlsClientHello: HTTP_REQUEST_PROPERTY = 11;
pub const HttpRequestPropertyTransportIdleConnectionTimeout: HTTP_REQUEST_PROPERTY = 12;
pub const HttpRequestPropertyWskApiTimings: HTTP_REQUEST_PROPERTY = 6;
pub const HttpRequestSizingTypeHeaders: HTTP_REQUEST_SIZING_TYPE = 4;
pub const HttpRequestSizingTypeMax: HTTP_REQUEST_SIZING_TYPE = 5;
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: HTTP_REQUEST_SIZING_TYPE = 0;
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: HTTP_REQUEST_SIZING_TYPE = 1;
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: HTTP_REQUEST_SIZING_TYPE = 2;
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: HTTP_REQUEST_SIZING_TYPE = 3;
pub const HttpRequestTimingTypeConnectionStart: HTTP_REQUEST_TIMING_TYPE = 0;
pub const HttpRequestTimingTypeDataStart: HTTP_REQUEST_TIMING_TYPE = 1;
pub const HttpRequestTimingTypeHttp2HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = 14;
pub const HttpRequestTimingTypeHttp2HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = 13;
pub const HttpRequestTimingTypeHttp2StreamStart: HTTP_REQUEST_TIMING_TYPE = 12;
pub const HttpRequestTimingTypeHttp3HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = 29;
pub const HttpRequestTimingTypeHttp3HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = 28;
pub const HttpRequestTimingTypeHttp3StreamStart: HTTP_REQUEST_TIMING_TYPE = 27;
pub const HttpRequestTimingTypeMax: HTTP_REQUEST_TIMING_TYPE = 30;
pub const HttpRequestTimingTypeRequestDeliveredForDelegation: HTTP_REQUEST_TIMING_TYPE = 23;
pub const HttpRequestTimingTypeRequestDeliveredForIO: HTTP_REQUEST_TIMING_TYPE = 26;
pub const HttpRequestTimingTypeRequestDeliveredForInspection: HTTP_REQUEST_TIMING_TYPE = 20;
pub const HttpRequestTimingTypeRequestHeaderParseEnd: HTTP_REQUEST_TIMING_TYPE = 16;
pub const HttpRequestTimingTypeRequestHeaderParseStart: HTTP_REQUEST_TIMING_TYPE = 15;
pub const HttpRequestTimingTypeRequestQueuedForDelegation: HTTP_REQUEST_TIMING_TYPE = 22;
pub const HttpRequestTimingTypeRequestQueuedForIO: HTTP_REQUEST_TIMING_TYPE = 25;
pub const HttpRequestTimingTypeRequestQueuedForInspection: HTTP_REQUEST_TIMING_TYPE = 19;
pub const HttpRequestTimingTypeRequestReturnedAfterDelegation: HTTP_REQUEST_TIMING_TYPE = 24;
pub const HttpRequestTimingTypeRequestReturnedAfterInspection: HTTP_REQUEST_TIMING_TYPE = 21;
pub const HttpRequestTimingTypeRequestRoutingEnd: HTTP_REQUEST_TIMING_TYPE = 18;
pub const HttpRequestTimingTypeRequestRoutingStart: HTTP_REQUEST_TIMING_TYPE = 17;
pub const HttpRequestTimingTypeTlsAttributesQueryEnd: HTTP_REQUEST_TIMING_TYPE = 9;
pub const HttpRequestTimingTypeTlsAttributesQueryStart: HTTP_REQUEST_TIMING_TYPE = 8;
pub const HttpRequestTimingTypeTlsCertificateLoadEnd: HTTP_REQUEST_TIMING_TYPE = 3;
pub const HttpRequestTimingTypeTlsCertificateLoadStart: HTTP_REQUEST_TIMING_TYPE = 2;
pub const HttpRequestTimingTypeTlsClientCertQueryEnd: HTTP_REQUEST_TIMING_TYPE = 11;
pub const HttpRequestTimingTypeTlsClientCertQueryStart: HTTP_REQUEST_TIMING_TYPE = 10;
pub const HttpRequestTimingTypeTlsHandshakeLeg1End: HTTP_REQUEST_TIMING_TYPE = 5;
pub const HttpRequestTimingTypeTlsHandshakeLeg1Start: HTTP_REQUEST_TIMING_TYPE = 4;
pub const HttpRequestTimingTypeTlsHandshakeLeg2End: HTTP_REQUEST_TIMING_TYPE = 7;
pub const HttpRequestTimingTypeTlsHandshakeLeg2Start: HTTP_REQUEST_TIMING_TYPE = 6;
pub const HttpResponseInfoTypeAuthenticationProperty: HTTP_RESPONSE_INFO_TYPE = 1;
pub const HttpResponseInfoTypeChannelBind: HTTP_RESPONSE_INFO_TYPE = 3;
pub const HttpResponseInfoTypeMultipleKnownHeaders: HTTP_RESPONSE_INFO_TYPE = 0;
pub const HttpResponseInfoTypeQoSProperty: HTTP_RESPONSE_INFO_TYPE = 2;
pub const HttpSchemeHttp: HTTP_SCHEME = 0;
pub const HttpSchemeHttps: HTTP_SCHEME = 1;
pub const HttpSchemeMaximum: HTTP_SCHEME = 2;
pub const HttpServer503VerbosityProperty: HTTP_SERVER_PROPERTY = 6;
pub const HttpServerAuthenticationProperty: HTTP_SERVER_PROPERTY = 0;
pub const HttpServerBindingProperty: HTTP_SERVER_PROPERTY = 7;
pub const HttpServerChannelBindProperty: HTTP_SERVER_PROPERTY = 10;
pub const HttpServerDelegationProperty: HTTP_SERVER_PROPERTY = 16;
pub const HttpServerExtendedAuthenticationProperty: HTTP_SERVER_PROPERTY = 8;
pub const HttpServerFastForwardingProperty: HTTP_SERVER_PROPERTY = 18;
pub const HttpServerListenEndpointProperty: HTTP_SERVER_PROPERTY = 9;
pub const HttpServerLoggingProperty: HTTP_SERVER_PROPERTY = 1;
pub const HttpServerProtectionLevelProperty: HTTP_SERVER_PROPERTY = 11;
pub const HttpServerQosProperty: HTTP_SERVER_PROPERTY = 2;
pub const HttpServerQueueLengthProperty: HTTP_SERVER_PROPERTY = 4;
pub const HttpServerRequestInfoProperty: HTTP_SERVER_PROPERTY = 19;
pub const HttpServerStateProperty: HTTP_SERVER_PROPERTY = 5;
pub const HttpServerTimeoutsProperty: HTTP_SERVER_PROPERTY = 3;
pub const HttpServiceBindingTypeA: HTTP_SERVICE_BINDING_TYPE = 2;
pub const HttpServiceBindingTypeNone: HTTP_SERVICE_BINDING_TYPE = 0;
pub const HttpServiceBindingTypeW: HTTP_SERVICE_BINDING_TYPE = 1;
pub const HttpServiceConfigCache: HTTP_SERVICE_CONFIG_ID = 4;
pub const HttpServiceConfigIPListenList: HTTP_SERVICE_CONFIG_ID = 0;
pub const HttpServiceConfigMax: HTTP_SERVICE_CONFIG_ID = 13;
pub const HttpServiceConfigQueryExact: HTTP_SERVICE_CONFIG_QUERY_TYPE = 0;
pub const HttpServiceConfigQueryMax: HTTP_SERVICE_CONFIG_QUERY_TYPE = 2;
pub const HttpServiceConfigQueryNext: HTTP_SERVICE_CONFIG_QUERY_TYPE = 1;
pub const HttpServiceConfigSSLCertInfo: HTTP_SERVICE_CONFIG_ID = 1;
pub const HttpServiceConfigSetting: HTTP_SERVICE_CONFIG_ID = 7;
pub const HttpServiceConfigSslCcsCertInfo: HTTP_SERVICE_CONFIG_ID = 6;
pub const HttpServiceConfigSslCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = 10;
pub const HttpServiceConfigSslCertInfoEx: HTTP_SERVICE_CONFIG_ID = 8;
pub const HttpServiceConfigSslScopedCcsCertInfo: HTTP_SERVICE_CONFIG_ID = 11;
pub const HttpServiceConfigSslScopedCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = 12;
pub const HttpServiceConfigSslSniCertInfo: HTTP_SERVICE_CONFIG_ID = 5;
pub const HttpServiceConfigSslSniCertInfoEx: HTTP_SERVICE_CONFIG_ID = 9;
pub const HttpServiceConfigTimeout: HTTP_SERVICE_CONFIG_ID = 3;
pub const HttpServiceConfigUrlAclInfo: HTTP_SERVICE_CONFIG_ID = 2;
pub const HttpTlsThrottle: HTTP_SERVICE_CONFIG_SETTING_KEY = 1;
pub const HttpVerbCONNECT: HTTP_VERB = 10;
pub const HttpVerbCOPY: HTTP_VERB = 13;
pub const HttpVerbDELETE: HTTP_VERB = 8;
pub const HttpVerbGET: HTTP_VERB = 4;
pub const HttpVerbHEAD: HTTP_VERB = 5;
pub const HttpVerbInvalid: HTTP_VERB = 2;
pub const HttpVerbLOCK: HTTP_VERB = 17;
pub const HttpVerbMKCOL: HTTP_VERB = 16;
pub const HttpVerbMOVE: HTTP_VERB = 12;
pub const HttpVerbMaximum: HTTP_VERB = 20;
pub const HttpVerbOPTIONS: HTTP_VERB = 3;
pub const HttpVerbPOST: HTTP_VERB = 6;
pub const HttpVerbPROPFIND: HTTP_VERB = 14;
pub const HttpVerbPROPPATCH: HTTP_VERB = 15;
pub const HttpVerbPUT: HTTP_VERB = 7;
pub const HttpVerbSEARCH: HTTP_VERB = 19;
pub const HttpVerbTRACE: HTTP_VERB = 9;
pub const HttpVerbTRACK: HTTP_VERB = 11;
pub const HttpVerbUNLOCK: HTTP_VERB = 18;
pub const HttpVerbUnknown: HTTP_VERB = 1;
pub const HttpVerbUnparsed: HTTP_VERB = 0;
pub const IdleConnectionTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = 0;
pub const MaxCacheResponseSize: HTTP_SERVICE_CONFIG_CACHE_KEY = 0;
pub type PHTTP2_SETTINGS_LIMITS_PARAM = *mut HTTP2_SETTINGS_LIMITS_PARAM;
pub type PHTTP2_WINDOW_SIZE_PARAM = *mut HTTP2_WINDOW_SIZE_PARAM;
pub type PHTTPAPI_VERSION = *mut HTTPAPI_VERSION;
pub type PHTTP_503_RESPONSE_VERBOSITY = *mut HTTP_503_RESPONSE_VERBOSITY;
pub type PHTTP_AUTH_STATUS = *mut HTTP_AUTH_STATUS;
pub type PHTTP_BANDWIDTH_LIMIT_INFO = *mut HTTP_BANDWIDTH_LIMIT_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_BINDING_INFO = *mut HTTP_BINDING_INFO;
pub type PHTTP_BYTE_RANGE = *mut HTTP_BYTE_RANGE;
pub type PHTTP_CACHE_POLICY = *mut HTTP_CACHE_POLICY;
pub type PHTTP_CACHE_POLICY_TYPE = *mut HTTP_CACHE_POLICY_TYPE;
pub type PHTTP_CERT_CONFIG_ENTRY = *mut HTTP_CERT_CONFIG_ENTRY;
pub type PHTTP_CERT_CONFIG_PARAM = *mut HTTP_CERT_CONFIG_PARAM;
pub type PHTTP_CHANNEL_BIND_INFO = *mut HTTP_CHANNEL_BIND_INFO;
pub type PHTTP_CLIENT_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_CLIENT_CREDENTIAL_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_CLIENT_REQUEST_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_CLIENT_STREAM_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_CONNECTION_LIMIT_INFO = *mut HTTP_CONNECTION_LIMIT_INFO;
pub type PHTTP_COOKED_URL = *mut HTTP_COOKED_URL;
pub type PHTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = *mut HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID;
pub type PHTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO = *mut HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_DATA_CHUNK = *mut HTTP_DATA_CHUNK;
pub type PHTTP_DATA_CHUNK_TYPE = *mut HTTP_DATA_CHUNK_TYPE;
pub type PHTTP_DELEGATE_REQUEST_PROPERTY_ID = *mut HTTP_DELEGATE_REQUEST_PROPERTY_ID;
pub type PHTTP_DELEGATE_REQUEST_PROPERTY_INFO = *mut HTTP_DELEGATE_REQUEST_PROPERTY_INFO;
pub type PHTTP_ENABLED_STATE = *mut HTTP_ENABLED_STATE;
pub type PHTTP_ERROR_HEADERS_PARAM = *mut HTTP_ERROR_HEADERS_PARAM;
pub type PHTTP_FAST_FORWARD_INFO = *mut HTTP_FAST_FORWARD_INFO;
pub type PHTTP_FEATURE_ID = *mut HTTP_FEATURE_ID;
pub type PHTTP_FLOWRATE_INFO = *mut HTTP_FLOWRATE_INFO;
pub type PHTTP_HEADER_ID = *mut HTTP_HEADER_ID;
pub type PHTTP_KNOWN_HEADER = *mut HTTP_KNOWN_HEADER;
pub type PHTTP_LISTEN_ENDPOINT_INFO = *mut HTTP_LISTEN_ENDPOINT_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_LOGGING_INFO = *mut HTTP_LOGGING_INFO;
pub type PHTTP_LOGGING_ROLLOVER_TYPE = *mut HTTP_LOGGING_ROLLOVER_TYPE;
pub type PHTTP_LOGGING_TYPE = *mut HTTP_LOGGING_TYPE;
pub type PHTTP_LOG_DATA = *mut HTTP_LOG_DATA;
pub type PHTTP_LOG_DATA_TYPE = *mut HTTP_LOG_DATA_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_LOG_FIELDS_DATA = *mut HTTP_LOG_FIELDS_DATA;
pub type PHTTP_MULTIPLE_KNOWN_HEADERS = *mut HTTP_MULTIPLE_KNOWN_HEADERS;
pub type PHTTP_OPAQUE_ID = *mut u64;
pub type PHTTP_PERFORMANCE_PARAM = *mut HTTP_PERFORMANCE_PARAM;
pub type PHTTP_PERFORMANCE_PARAM_TYPE = *mut HTTP_PERFORMANCE_PARAM_TYPE;
pub type PHTTP_PROPERTY_FLAGS = *mut HTTP_PROPERTY_FLAGS;
pub type PHTTP_PROTECTION_LEVEL_INFO = *mut HTTP_PROTECTION_LEVEL_INFO;
pub type PHTTP_PROTECTION_LEVEL_TYPE = *mut HTTP_PROTECTION_LEVEL_TYPE;
pub type PHTTP_QOS_SETTING_INFO = *mut HTTP_QOS_SETTING_INFO;
pub type PHTTP_QOS_SETTING_TYPE = *mut HTTP_QOS_SETTING_TYPE;
pub type PHTTP_QUERY_REQUEST_QUALIFIER_QUIC = *mut HTTP_QUERY_REQUEST_QUALIFIER_QUIC;
pub type PHTTP_QUERY_REQUEST_QUALIFIER_TCP = *mut HTTP_QUERY_REQUEST_QUALIFIER_TCP;
pub type PHTTP_QUIC_API_TIMINGS = *mut HTTP_QUIC_API_TIMINGS;
pub type PHTTP_QUIC_CONNECTION_API_TIMINGS = *mut HTTP_QUIC_CONNECTION_API_TIMINGS;
pub type PHTTP_QUIC_STREAM_API_TIMINGS = *mut HTTP_QUIC_STREAM_API_TIMINGS;
pub type PHTTP_QUIC_STREAM_REQUEST_STATS = *mut HTTP_QUIC_STREAM_REQUEST_STATS;
pub type PHTTP_RAW_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
pub type PHTTP_REQUEST = *mut HTTP_REQUEST;
#[cfg(all(feature = "Win32_ncrypt", feature = "Win32_winnt"))]
pub type PHTTP_REQUEST_AUTH_INFO = *mut HTTP_REQUEST_AUTH_INFO;
pub type PHTTP_REQUEST_AUTH_TYPE = *mut HTTP_REQUEST_AUTH_TYPE;
#[cfg(feature = "Win32_minwindef")]
pub type PHTTP_REQUEST_CHANNEL_BIND_STATUS = *mut HTTP_REQUEST_CHANNEL_BIND_STATUS;
pub type PHTTP_REQUEST_DSCP_INFO = *mut HTTP_REQUEST_DSCP_INFO;
pub type PHTTP_REQUEST_HEADERS = *mut HTTP_REQUEST_HEADERS;
pub type PHTTP_REQUEST_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_REQUEST_INFO = *mut HTTP_REQUEST_INFO;
pub type PHTTP_REQUEST_INFO_PROPERTY_INFO = *mut HTTP_REQUEST_INFO_PROPERTY_INFO;
pub type PHTTP_REQUEST_INFO_TYPE = *mut HTTP_REQUEST_INFO_TYPE;
pub type PHTTP_REQUEST_INITIAL_PACKET_TTL_INFO = *mut HTTP_REQUEST_INITIAL_PACKET_TTL_INFO;
pub type PHTTP_REQUEST_PROPERTY = *mut HTTP_REQUEST_PROPERTY;
pub type PHTTP_REQUEST_PROPERTY_SNI = *mut HTTP_REQUEST_PROPERTY_SNI;
pub type PHTTP_REQUEST_PROPERTY_STREAM_ERROR = *mut HTTP_REQUEST_PROPERTY_STREAM_ERROR;
pub type PHTTP_REQUEST_SIZING_INFO = *mut HTTP_REQUEST_SIZING_INFO;
pub type PHTTP_REQUEST_SIZING_TYPE = *mut HTTP_REQUEST_SIZING_TYPE;
pub type PHTTP_REQUEST_TIMING_INFO = *mut HTTP_REQUEST_TIMING_INFO;
pub type PHTTP_REQUEST_TIMING_TYPE = *mut HTTP_REQUEST_TIMING_TYPE;
#[cfg(feature = "Win32_minwindef")]
pub type PHTTP_REQUEST_TOKEN_BINDING_INFO = *mut HTTP_REQUEST_TOKEN_BINDING_INFO;
pub type PHTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO = *mut HTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
pub type PHTTP_REQUEST_V1 = *mut HTTP_REQUEST_V1;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
pub type PHTTP_REQUEST_V2 = *mut HTTP_REQUEST_V2;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_RESPONSE = *mut HTTP_RESPONSE;
pub type PHTTP_RESPONSE_HEADERS = *mut HTTP_RESPONSE_HEADERS;
pub type PHTTP_RESPONSE_INFO = *mut HTTP_RESPONSE_INFO;
pub type PHTTP_RESPONSE_INFO_TYPE = *mut HTTP_RESPONSE_INFO_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_RESPONSE_V1 = *mut HTTP_RESPONSE_V1;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_RESPONSE_V2 = *mut HTTP_RESPONSE_V2;
pub type PHTTP_SERVER_AUTHENTICATION_BASIC_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS;
pub type PHTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS;
pub type PHTTP_SERVER_AUTHENTICATION_INFO = *mut HTTP_SERVER_AUTHENTICATION_INFO;
pub type PHTTP_SERVER_PROPERTY = *mut HTTP_SERVER_PROPERTY;
pub type PHTTP_SERVER_SESSION_ID = *mut HTTP_OPAQUE_ID;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_SERVICE_BINDING_A = *mut HTTP_SERVICE_BINDING_A;
pub type PHTTP_SERVICE_BINDING_BASE = *mut HTTP_SERVICE_BINDING_BASE;
#[cfg(feature = "Win32_winnt")]
pub type PHTTP_SERVICE_BINDING_W = *mut HTTP_SERVICE_BINDING_W;
pub type PHTTP_SERVICE_CONFIG_CACHE_KEY = *mut HTTP_SERVICE_CONFIG_CACHE_KEY;
pub type PHTTP_SERVICE_CONFIG_CACHE_PARAM = *mut u32;
pub type PHTTP_SERVICE_CONFIG_CACHE_SET = *mut HTTP_SERVICE_CONFIG_CACHE_SET;
pub type PHTTP_SERVICE_CONFIG_ID = *mut HTTP_SERVICE_CONFIG_ID;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_PARAM = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_QUERY = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY;
pub type PHTTP_SERVICE_CONFIG_QUERY_TYPE = *mut HTTP_SERVICE_CONFIG_QUERY_TYPE;
pub type PHTTP_SERVICE_CONFIG_SETTING_KEY = *mut HTTP_SERVICE_CONFIG_SETTING_KEY;
pub type PHTTP_SERVICE_CONFIG_SETTING_PARAM = *mut u32;
pub type PHTTP_SERVICE_CONFIG_SETTING_SET = *mut HTTP_SERVICE_CONFIG_SETTING_SET;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_KEY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_KEY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_KEY = *mut HTTP_SERVICE_CONFIG_SSL_KEY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_KEY_EX = *mut HTTP_SERVICE_CONFIG_SSL_KEY_EX;
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM = *mut HTTP_SERVICE_CONFIG_SSL_PARAM;
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM_EX = *mut HTTP_SERVICE_CONFIG_SSL_PARAM_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_QUERY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_QUERY_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SET = *mut HTTP_SERVICE_CONFIG_SSL_SET;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_SET_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_KEY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_KEY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_KEY = *mut HTTP_SERVICE_CONFIG_TIMEOUT_KEY;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_PARAM = *mut u16;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_SET = *mut HTTP_SERVICE_CONFIG_TIMEOUT_SET;
pub type PHTTP_SERVICE_CONFIG_URLACL_KEY = *mut HTTP_SERVICE_CONFIG_URLACL_KEY;
pub type PHTTP_SERVICE_CONFIG_URLACL_PARAM = *mut HTTP_SERVICE_CONFIG_URLACL_PARAM;
pub type PHTTP_SERVICE_CONFIG_URLACL_QUERY = *mut HTTP_SERVICE_CONFIG_URLACL_QUERY;
pub type PHTTP_SERVICE_CONFIG_URLACL_SET = *mut HTTP_SERVICE_CONFIG_URLACL_SET;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PHTTP_SSL_CLIENT_CERT_INFO = *mut HTTP_SSL_CLIENT_CERT_INFO;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PHTTP_SSL_INFO = *mut HTTP_SSL_INFO;
pub type PHTTP_SSL_PROTOCOL_INFO = *mut HTTP_SSL_PROTOCOL_INFO;
pub type PHTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = *mut HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE;
pub type PHTTP_STATE_INFO = *mut HTTP_STATE_INFO;
pub type PHTTP_TIMEOUT_LIMIT_INFO = *mut HTTP_TIMEOUT_LIMIT_INFO;
pub type PHTTP_TLS_RESTRICTIONS_PARAM = *mut HTTP_TLS_RESTRICTIONS_PARAM;
pub type PHTTP_TLS_SESSION_TICKET_KEYS_PARAM = *mut HTTP_TLS_SESSION_TICKET_KEYS_PARAM;
#[cfg(feature = "Win32_ws2")]
pub type PHTTP_TRANSPORT_ADDRESS = *mut HTTP_TRANSPORT_ADDRESS;
pub type PHTTP_UNKNOWN_HEADER = *mut HTTP_UNKNOWN_HEADER;
pub type PHTTP_URI_SCHEME = *mut HTTP_SCHEME;
pub type PHTTP_URL_GROUP_ID = *mut HTTP_OPAQUE_ID;
pub type PHTTP_VERB = *mut HTTP_VERB;
pub type PHTTP_VERSION = *mut HTTP_VERSION;
pub type PHTTP_WINHTTP_FAST_FORWARDING_DATA = *mut HTTP_WINHTTP_FAST_FORWARDING_DATA;
pub type PHTTP_WSK_API_TIMINGS = *mut HTTP_WSK_API_TIMINGS;
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = 1;
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = 5;
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = 6;
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = 3;
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = 4;
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = 2;
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = 0;
