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
