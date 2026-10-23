#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpAddFragmentToCache(requestqueuehandle: super::HANDLE, urlprefix: &windows_core::PCWSTR, datachunk: PHTTP_DATA_CHUNK, cachepolicy: PHTTP_CACHE_POLICY, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpAddFragmentToCache(requestqueuehandle : super::HANDLE, urlprefix : windows_core::PCWSTR, datachunk : PHTTP_DATA_CHUNK, cachepolicy : PHTTP_CACHE_POLICY, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpAddFragmentToCache(requestqueuehandle as _, core::mem::transmute_copy(urlprefix), datachunk as _, cachepolicy as _, overlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpAddUrl(requestqueuehandle: super::HANDLE, fullyqualifiedurl: &windows_core::PCWSTR, reserved: super::PVOID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpAddUrl(requestqueuehandle : super::HANDLE, fullyqualifiedurl : windows_core::PCWSTR, reserved : super::PVOID) -> super::ULONG);
    unsafe { HttpAddUrl(requestqueuehandle as _, core::mem::transmute_copy(fullyqualifiedurl), reserved as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpAddUrlToUrlGroup(urlgroupid: HTTP_URL_GROUP_ID, pfullyqualifiedurl: &windows_core::PCWSTR, urlcontext: HTTP_URL_CONTEXT, reserved: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpAddUrlToUrlGroup(urlgroupid : HTTP_URL_GROUP_ID, pfullyqualifiedurl : windows_core::PCWSTR, urlcontext : HTTP_URL_CONTEXT, reserved : super::ULONG) -> super::ULONG);
    unsafe { HttpAddUrlToUrlGroup(urlgroupid, core::mem::transmute_copy(pfullyqualifiedurl), urlcontext, reserved) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpCancelHttpRequest(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCancelHttpRequest(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpCancelHttpRequest(requestqueuehandle as _, requestid, overlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpCloseRequestQueue(requestqueuehandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCloseRequestQueue(requestqueuehandle : super::HANDLE) -> super::ULONG);
    unsafe { HttpCloseRequestQueue(requestqueuehandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpCloseServerSession(serversessionid: HTTP_SERVER_SESSION_ID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCloseServerSession(serversessionid : HTTP_SERVER_SESSION_ID) -> super::ULONG);
    unsafe { HttpCloseServerSession(serversessionid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpCloseUrlGroup(urlgroupid: HTTP_URL_GROUP_ID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCloseUrlGroup(urlgroupid : HTTP_URL_GROUP_ID) -> super::ULONG);
    unsafe { HttpCloseUrlGroup(urlgroupid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpCreateHttpHandle(requestqueuehandle: super::PHANDLE, reserved: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCreateHttpHandle(requestqueuehandle : super::PHANDLE, reserved : super::ULONG) -> super::ULONG);
    unsafe { HttpCreateHttpHandle(requestqueuehandle as _, reserved) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpCreateRequestQueue(version: &HTTPAPI_VERSION, name: &windows_core::PCWSTR, securityattributes: super::PSECURITY_ATTRIBUTES, flags: super::ULONG, requestqueuehandle: super::PHANDLE) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCreateRequestQueue(version : HTTPAPI_VERSION, name : windows_core::PCWSTR, securityattributes : super::PSECURITY_ATTRIBUTES, flags : super::ULONG, requestqueuehandle : super::PHANDLE) -> super::ULONG);
    unsafe { HttpCreateRequestQueue(core::mem::transmute_copy(version), core::mem::transmute_copy(name), securityattributes as _, flags, requestqueuehandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpCreateServerSession(version: &HTTPAPI_VERSION, serversessionid: PHTTP_SERVER_SESSION_ID, reserved: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCreateServerSession(version : HTTPAPI_VERSION, serversessionid : PHTTP_SERVER_SESSION_ID, reserved : super::ULONG) -> super::ULONG);
    unsafe { HttpCreateServerSession(core::mem::transmute_copy(version), serversessionid as _, reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpCreateUrlGroup(serversessionid: HTTP_SERVER_SESSION_ID, purlgroupid: PHTTP_URL_GROUP_ID, reserved: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpCreateUrlGroup(serversessionid : HTTP_SERVER_SESSION_ID, purlgroupid : PHTTP_URL_GROUP_ID, reserved : super::ULONG) -> super::ULONG);
    unsafe { HttpCreateUrlGroup(serversessionid, purlgroupid as _, reserved) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpDeclarePush<P3, P4>(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, verb: HTTP_VERB, path: P3, query: P4, headers: Option<PHTTP_REQUEST_HEADERS>) -> super::ULONG
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("httpapi.dll" "system" fn HttpDeclarePush(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, verb : HTTP_VERB, path : windows_core::PCWSTR, query : windows_core::PCSTR, headers : PHTTP_REQUEST_HEADERS) -> super::ULONG);
    unsafe { HttpDeclarePush(requestqueuehandle, requestid, verb, path.param().abi(), query.param().abi(), headers.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpDelegateRequestEx(requestqueuehandle: super::HANDLE, delegatequeuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, delegateurlgroupid: HTTP_URL_GROUP_ID, propertyinfosetsize: super::ULONG, propertyinfoset: PHTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpDelegateRequestEx(requestqueuehandle : super::HANDLE, delegatequeuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, delegateurlgroupid : HTTP_URL_GROUP_ID, propertyinfosetsize : super::ULONG, propertyinfoset : PHTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> super::ULONG);
    unsafe { HttpDelegateRequestEx(requestqueuehandle, delegatequeuehandle, requestid, delegateurlgroupid, propertyinfosetsize, propertyinfoset) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpDeleteServiceConfiguration(servicehandle: super::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: super::PVOID, configinformationlength: super::ULONG, poverlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpDeleteServiceConfiguration(servicehandle : super::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : super::PVOID, configinformationlength : super::ULONG, poverlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpDeleteServiceConfiguration(servicehandle as _, configid, pconfiginformation, configinformationlength, poverlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpFindUrlGroupId<P0>(fullyqualifiedurl: P0, requestqueuehandle: super::HANDLE, urlgroupid: PHTTP_URL_GROUP_ID) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("httpapi.dll" "system" fn HttpFindUrlGroupId(fullyqualifiedurl : windows_core::PCWSTR, requestqueuehandle : super::HANDLE, urlgroupid : PHTTP_URL_GROUP_ID) -> super::ULONG);
    unsafe { HttpFindUrlGroupId(fullyqualifiedurl.param().abi(), requestqueuehandle, urlgroupid as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpFlushResponseCache(requestqueuehandle: super::HANDLE, urlprefix: &windows_core::PCWSTR, flags: super::ULONG, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpFlushResponseCache(requestqueuehandle : super::HANDLE, urlprefix : windows_core::PCWSTR, flags : super::ULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpFlushResponseCache(requestqueuehandle as _, core::mem::transmute_copy(urlprefix), flags, overlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpGetExtension(version: HTTPAPI_VERSION, extension: super::ULONG, buffer: super::PVOID, buffersize: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpGetExtension(version : HTTPAPI_VERSION, extension : super::ULONG, buffer : super::PVOID, buffersize : super::ULONG) -> super::ULONG);
    unsafe { HttpGetExtension(version, extension, buffer as _, buffersize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpInitialize(version: &HTTPAPI_VERSION, flags: super::ULONG, preserved: super::PVOID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpInitialize(version : HTTPAPI_VERSION, flags : super::ULONG, preserved : super::PVOID) -> super::ULONG);
    unsafe { HttpInitialize(core::mem::transmute_copy(version), flags, preserved as _) }
}
#[inline]
pub unsafe fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> windows_core::BOOL {
    windows_core::link!("httpapi.dll" "system" fn HttpIsFeatureSupported(featureid : HTTP_FEATURE_ID) -> windows_core::BOOL);
    unsafe { HttpIsFeatureSupported(featureid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpPrepareUrl<P2>(reserved: super::PVOID, flags: super::ULONG, url: P2, preparedurl: *mut windows_core::PWSTR) -> super::ULONG
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("httpapi.dll" "system" fn HttpPrepareUrl(reserved : super::PVOID, flags : super::ULONG, url : windows_core::PCWSTR, preparedurl : *mut windows_core::PWSTR) -> super::ULONG);
    unsafe { HttpPrepareUrl(reserved as _, flags, url.param().abi(), preparedurl as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpQueryRequestProperty(requestqueuehandle: super::HANDLE, id: HTTP_OPAQUE_ID, propertyid: HTTP_REQUEST_PROPERTY, qualifier: Option<*const core::ffi::c_void>, qualifiersize: super::ULONG, output: Option<super::PVOID>, outputbuffersize: super::ULONG, bytesreturned: Option<super::PULONG>, overlapped: Option<super::LPOVERLAPPED>) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpQueryRequestProperty(requestqueuehandle : super::HANDLE, id : HTTP_OPAQUE_ID, propertyid : HTTP_REQUEST_PROPERTY, qualifier : *const core::ffi::c_void, qualifiersize : super::ULONG, output : super::PVOID, outputbuffersize : super::ULONG, bytesreturned : super::PULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpQueryRequestProperty(requestqueuehandle, id, propertyid, qualifier.unwrap_or(core::mem::zeroed()) as _, qualifiersize, output.unwrap_or(core::mem::zeroed()) as _, outputbuffersize, bytesreturned.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpQueryRequestQueueProperty(requestqueuehandle: super::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<super::PVOID>, propertyinformationlength: super::ULONG, reserved1: super::ULONG, returnlength: Option<super::PULONG>, reserved2: super::PVOID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpQueryRequestQueueProperty(requestqueuehandle : super::HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG, reserved1 : super::ULONG, returnlength : super::PULONG, reserved2 : super::PVOID) -> super::ULONG);
    unsafe { HttpQueryRequestQueueProperty(requestqueuehandle, property, propertyinformation.unwrap_or(core::mem::zeroed()) as _, propertyinformationlength, reserved1, returnlength.unwrap_or(core::mem::zeroed()) as _, reserved2 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpQueryServerSessionProperty(serversessionid: HTTP_SERVER_SESSION_ID, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<super::PVOID>, propertyinformationlength: super::ULONG, returnlength: Option<super::PULONG>) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpQueryServerSessionProperty(serversessionid : HTTP_SERVER_SESSION_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG, returnlength : super::PULONG) -> super::ULONG);
    unsafe { HttpQueryServerSessionProperty(serversessionid, property, propertyinformation.unwrap_or(core::mem::zeroed()) as _, propertyinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpQueryServiceConfiguration(servicehandle: super::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pinput: Option<super::PVOID>, inputlength: super::ULONG, poutput: Option<super::PVOID>, outputlength: super::ULONG, preturnlength: Option<super::PULONG>, poverlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpQueryServiceConfiguration(servicehandle : super::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pinput : super::PVOID, inputlength : super::ULONG, poutput : super::PVOID, outputlength : super::ULONG, preturnlength : super::PULONG, poverlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpQueryServiceConfiguration(servicehandle as _, configid, pinput.unwrap_or(core::mem::zeroed()) as _, inputlength, poutput.unwrap_or(core::mem::zeroed()) as _, outputlength, preturnlength.unwrap_or(core::mem::zeroed()) as _, poverlapped as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpQueryUrlGroupProperty(urlgroupid: HTTP_URL_GROUP_ID, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<super::PVOID>, propertyinformationlength: super::ULONG, returnlength: Option<super::PULONG>) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpQueryUrlGroupProperty(urlgroupid : HTTP_URL_GROUP_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG, returnlength : super::PULONG) -> super::ULONG);
    unsafe { HttpQueryUrlGroupProperty(urlgroupid, property, propertyinformation.unwrap_or(core::mem::zeroed()) as _, propertyinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpReadFragmentFromCache(requestqueuehandle: super::HANDLE, urlprefix: &windows_core::PCWSTR, byterange: PHTTP_BYTE_RANGE, buffer: super::PVOID, bufferlength: super::ULONG, bytesread: Option<super::PULONG>, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpReadFragmentFromCache(requestqueuehandle : super::HANDLE, urlprefix : windows_core::PCWSTR, byterange : PHTTP_BYTE_RANGE, buffer : super::PVOID, bufferlength : super::ULONG, bytesread : super::PULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpReadFragmentFromCache(requestqueuehandle as _, core::mem::transmute_copy(urlprefix), byterange as _, buffer as _, bufferlength, bytesread.unwrap_or(core::mem::zeroed()) as _, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpReceiveClientCertificate(requestqueuehandle: super::HANDLE, connectionid: HTTP_CONNECTION_ID, flags: super::ULONG, sslclientcertinfo: PHTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: super::ULONG, bytesreceived: Option<super::PULONG>, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpReceiveClientCertificate(requestqueuehandle : super::HANDLE, connectionid : HTTP_CONNECTION_ID, flags : super::ULONG, sslclientcertinfo : PHTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize : super::ULONG, bytesreceived : super::PULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpReceiveClientCertificate(requestqueuehandle as _, connectionid, flags, sslclientcertinfo as _, sslclientcertinfosize, bytesreceived.unwrap_or(core::mem::zeroed()) as _, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn HttpReceiveHttpRequest(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, flags: super::ULONG, requestbuffer: PHTTP_REQUEST, requestbufferlength: super::ULONG, bytesreturned: Option<super::PULONG>, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpReceiveHttpRequest(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, flags : super::ULONG, requestbuffer : PHTTP_REQUEST, requestbufferlength : super::ULONG, bytesreturned : super::PULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpReceiveHttpRequest(requestqueuehandle as _, requestid, flags, requestbuffer as _, requestbufferlength, bytesreturned.unwrap_or(core::mem::zeroed()) as _, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpReceiveRequestEntityBody(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, flags: super::ULONG, entitybuffer: super::PVOID, entitybufferlength: super::ULONG, bytesreturned: Option<super::PULONG>, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpReceiveRequestEntityBody(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, flags : super::ULONG, entitybuffer : super::PVOID, entitybufferlength : super::ULONG, bytesreturned : super::PULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpReceiveRequestEntityBody(requestqueuehandle as _, requestid, flags, entitybuffer as _, entitybufferlength, bytesreturned.unwrap_or(core::mem::zeroed()) as _, overlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpRemoveUrl(requestqueuehandle: super::HANDLE, fullyqualifiedurl: &windows_core::PCWSTR) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpRemoveUrl(requestqueuehandle : super::HANDLE, fullyqualifiedurl : windows_core::PCWSTR) -> super::ULONG);
    unsafe { HttpRemoveUrl(requestqueuehandle as _, core::mem::transmute_copy(fullyqualifiedurl)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HttpRemoveUrlFromUrlGroup(urlgroupid: HTTP_URL_GROUP_ID, pfullyqualifiedurl: &windows_core::PCWSTR, flags: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpRemoveUrlFromUrlGroup(urlgroupid : HTTP_URL_GROUP_ID, pfullyqualifiedurl : windows_core::PCWSTR, flags : super::ULONG) -> super::ULONG);
    unsafe { HttpRemoveUrlFromUrlGroup(urlgroupid, core::mem::transmute_copy(pfullyqualifiedurl), flags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSendHttpResponse(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, flags: super::ULONG, httpresponse: PHTTP_RESPONSE, cachepolicy: PHTTP_CACHE_POLICY, bytessent: super::PULONG, reserved1: super::PVOID, reserved2: super::ULONG, overlapped: super::LPOVERLAPPED, logdata: PHTTP_LOG_DATA) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSendHttpResponse(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, flags : super::ULONG, httpresponse : PHTTP_RESPONSE, cachepolicy : PHTTP_CACHE_POLICY, bytessent : super::PULONG, reserved1 : super::PVOID, reserved2 : super::ULONG, overlapped : super::LPOVERLAPPED, logdata : PHTTP_LOG_DATA) -> super::ULONG);
    unsafe { HttpSendHttpResponse(requestqueuehandle as _, requestid, flags, httpresponse as _, cachepolicy as _, bytessent as _, reserved1 as _, reserved2, overlapped as _, logdata as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSendResponseEntityBody(requestqueuehandle: super::HANDLE, requestid: HTTP_REQUEST_ID, flags: super::ULONG, entitychunkcount: super::USHORT, entitychunks: Option<PHTTP_DATA_CHUNK>, bytessent: super::PULONG, reserved1: super::PVOID, reserved2: super::ULONG, overlapped: super::LPOVERLAPPED, logdata: PHTTP_LOG_DATA) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSendResponseEntityBody(requestqueuehandle : super::HANDLE, requestid : HTTP_REQUEST_ID, flags : super::ULONG, entitychunkcount : super::USHORT, entitychunks : PHTTP_DATA_CHUNK, bytessent : super::PULONG, reserved1 : super::PVOID, reserved2 : super::ULONG, overlapped : super::LPOVERLAPPED, logdata : PHTTP_LOG_DATA) -> super::ULONG);
    unsafe { HttpSendResponseEntityBody(requestqueuehandle as _, requestid, flags, entitychunkcount, entitychunks.unwrap_or(core::mem::zeroed()) as _, bytessent as _, reserved1 as _, reserved2, overlapped as _, logdata as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSetRequestProperty(requestqueuehandle: super::HANDLE, id: HTTP_OPAQUE_ID, propertyid: HTTP_REQUEST_PROPERTY, input: Option<super::PVOID>, inputpropertysize: super::ULONG, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSetRequestProperty(requestqueuehandle : super::HANDLE, id : HTTP_OPAQUE_ID, propertyid : HTTP_REQUEST_PROPERTY, input : super::PVOID, inputpropertysize : super::ULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpSetRequestProperty(requestqueuehandle, id, propertyid, input.unwrap_or(core::mem::zeroed()) as _, inputpropertysize, overlapped) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSetRequestQueueProperty(requestqueuehandle: super::HANDLE, property: HTTP_SERVER_PROPERTY, propertyinformation: super::PVOID, propertyinformationlength: super::ULONG, reserved1: super::ULONG, reserved2: super::PVOID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSetRequestQueueProperty(requestqueuehandle : super::HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG, reserved1 : super::ULONG, reserved2 : super::PVOID) -> super::ULONG);
    unsafe { HttpSetRequestQueueProperty(requestqueuehandle, property, propertyinformation, propertyinformationlength, reserved1, reserved2 as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSetServerSessionProperty(serversessionid: HTTP_SERVER_SESSION_ID, property: HTTP_SERVER_PROPERTY, propertyinformation: super::PVOID, propertyinformationlength: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSetServerSessionProperty(serversessionid : HTTP_SERVER_SESSION_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG) -> super::ULONG);
    unsafe { HttpSetServerSessionProperty(serversessionid, property, propertyinformation, propertyinformationlength) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSetServiceConfiguration(servicehandle: super::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: super::PVOID, configinformationlength: super::ULONG, poverlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSetServiceConfiguration(servicehandle : super::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : super::PVOID, configinformationlength : super::ULONG, poverlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpSetServiceConfiguration(servicehandle as _, configid, pconfiginformation, configinformationlength, poverlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpSetUrlGroupProperty(urlgroupid: HTTP_URL_GROUP_ID, property: HTTP_SERVER_PROPERTY, propertyinformation: super::PVOID, propertyinformationlength: super::ULONG) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpSetUrlGroupProperty(urlgroupid : HTTP_URL_GROUP_ID, property : HTTP_SERVER_PROPERTY, propertyinformation : super::PVOID, propertyinformationlength : super::ULONG) -> super::ULONG);
    unsafe { HttpSetUrlGroupProperty(urlgroupid, property, propertyinformation, propertyinformationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpShutdownRequestQueue(requestqueuehandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpShutdownRequestQueue(requestqueuehandle : super::HANDLE) -> super::ULONG);
    unsafe { HttpShutdownRequestQueue(requestqueuehandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpTerminate(flags: super::ULONG, preserved: super::PVOID) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpTerminate(flags : super::ULONG, preserved : super::PVOID) -> super::ULONG);
    unsafe { HttpTerminate(flags, preserved as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpUpdateServiceConfiguration(handle: super::HANDLE, configid: HTTP_SERVICE_CONFIG_ID, configinfo: super::PVOID, configinfolength: super::ULONG, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpUpdateServiceConfiguration(handle : super::HANDLE, configid : HTTP_SERVICE_CONFIG_ID, configinfo : super::PVOID, configinfolength : super::ULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpUpdateServiceConfiguration(handle as _, configid, configinfo, configinfolength, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpWaitForDemandStart(requestqueuehandle: super::HANDLE, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpWaitForDemandStart(requestqueuehandle : super::HANDLE, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpWaitForDemandStart(requestqueuehandle as _, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpWaitForDisconnect(requestqueuehandle: super::HANDLE, connectionid: HTTP_CONNECTION_ID, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpWaitForDisconnect(requestqueuehandle : super::HANDLE, connectionid : HTTP_CONNECTION_ID, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpWaitForDisconnect(requestqueuehandle as _, connectionid, overlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpWaitForDisconnectEx(requestqueuehandle: super::HANDLE, connectionid: HTTP_CONNECTION_ID, reserved: super::ULONG, overlapped: super::LPOVERLAPPED) -> super::ULONG {
    windows_core::link!("httpapi.dll" "system" fn HttpWaitForDisconnectEx(requestqueuehandle : super::HANDLE, connectionid : HTTP_CONNECTION_ID, reserved : super::ULONG, overlapped : super::LPOVERLAPPED) -> super::ULONG);
    unsafe { HttpWaitForDisconnectEx(requestqueuehandle as _, connectionid, reserved, overlapped as _) }
}
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP2_SETTINGS_LIMITS_PARAM {
    pub Http2MaxSettingsPerFrame: super::DWORD,
    pub Http2MaxSettingsPerMinute: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: super::USHORT,
    pub HttpApiMinorVersion: super::USHORT,
}
pub type HTTP_503_RESPONSE_VERBOSITY = i32;
pub type HTTP_AUTHENTICATION_HARDENING_LEVELS = i32;
pub const HTTP_AUTH_ENABLE_ALL: i32 = 31;
pub const HTTP_AUTH_ENABLE_BASIC: i32 = 1;
pub const HTTP_AUTH_ENABLE_DIGEST: i32 = 2;
pub const HTTP_AUTH_ENABLE_KERBEROS: i32 = 16;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: i32 = 8;
pub const HTTP_AUTH_ENABLE_NTLM: i32 = 4;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: i32 = 2;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: i32 = 1;
pub type HTTP_AUTH_STATUS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: super::ULARGE_INTEGER,
    pub Length: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_BYTE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const HTTP_BYTE_RANGE_TO_EOF: super::ULONGLONG = 18446744073709551615;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: super::ULONG,
}
pub type HTTP_CACHE_POLICY_TYPE = i32;
pub const HTTP_CERT_CHECK_MODE_CACHED_REVOCATION: i32 = 2;
pub const HTTP_CERT_CHECK_MODE_CACHED_URLS: i32 = 8;
pub const HTTP_CERT_CHECK_MODE_NO_AIA: i32 = 16;
pub const HTTP_CERT_CHECK_MODE_NO_REVOCATION: i32 = 1;
pub const HTTP_CERT_CHECK_MODE_NO_USAGE_CHECK: i32 = 65536;
pub const HTTP_CERT_CHECK_MODE_USE_REVOCATION_FRESHNESS: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_CERT_CONFIG_ENTRY {
    pub CertHash: [super::BYTE; 20],
    pub CertStoreName: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_CERT_CONFIG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_CERT_CONFIG_PARAM {
    pub CertConfigCount: super::ULONG,
    pub CertConfigs: PHTTP_CERT_CONFIG_ENTRY,
}
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: i32 = 16;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: super::ULONG,
    pub ServiceNames: *mut PHTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: super::ULONG,
}
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: i32 = 2;
pub const HTTP_CHANNEL_BIND_PROXY: i32 = 1;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: i32 = 32;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: i32 = 8;
pub type HTTP_CLIENT_CONNECTION_ID = u64;
pub type HTTP_CLIENT_CREDENTIAL_ID = u64;
pub type HTTP_CLIENT_REQUEST_ID = u64;
pub type HTTP_CLIENT_STREAM_ID = u64;
pub type HTTP_CONNECTION_ID = u64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: super::USHORT,
    pub HostLength: super::USHORT,
    pub AbsPathLength: super::USHORT,
    pub QueryStringLength: super::USHORT,
    pub pFullUrl: windows_core::PCWSTR,
    pub pHost: windows_core::PCWSTR,
    pub pAbsPath: windows_core::PCWSTR,
    pub pQueryString: windows_core::PCWSTR,
}
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: i32 = 2;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: i32 = 8;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: i32 = 1;
pub type HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    pub PropertyId: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID,
    pub PropertyInfoLength: super::ULONG,
    pub PropertyInfo: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub Anonymous: HTTP_DATA_CHUNK_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union HTTP_DATA_CHUNK_0 {
    pub FromMemory: HTTP_DATA_CHUNK_0_0,
    pub FromFileHandle: HTTP_DATA_CHUNK_0_1,
    pub FromFragmentCache: HTTP_DATA_CHUNK_0_2,
    pub FromFragmentCacheEx: HTTP_DATA_CHUNK_0_3,
    pub Trailers: HTTP_DATA_CHUNK_0_4,
    pub FromWinHttpFastForwarding: HTTP_DATA_CHUNK_0_5,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_DATA_CHUNK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_0 {
    pub pBuffer: super::PVOID,
    pub BufferLength: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: super::HANDLE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_DATA_CHUNK_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: super::USHORT,
    pub pFragmentName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK_0_3 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: windows_core::PCWSTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_DATA_CHUNK_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_4 {
    pub TrailerCount: super::USHORT,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_5 {
    pub WhFastForwardingData: HTTP_WINHTTP_FAST_FORWARDING_DATA,
}
pub type HTTP_DATA_CHUNK_TYPE = i32;
pub type HTTP_DELEGATE_REQUEST_PROPERTY_ID = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    pub PropertyId: HTTP_DELEGATE_REQUEST_PROPERTY_ID,
    pub PropertyInfoLength: super::ULONG,
    pub PropertyInfo: super::PVOID,
}
pub const HTTP_DEMAND_CBT: i32 = 4;
pub type HTTP_ENABLED_STATE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: super::USHORT,
    pub HeaderCount: super::USHORT,
    pub Headers: PHTTP_UNKNOWN_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FAST_FORWARD_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableFastForwarding: super::BOOLEAN,
}
pub type HTTP_FEATURE_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: super::ULONG,
    pub MaxPeakBandwidth: super::ULONG,
    pub BurstSize: super::ULONG,
}
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: i32 = 1;
pub type HTTP_HEADER_ID = i32;
pub const HTTP_INITIALIZE_CONFIG: i32 = 2;
pub const HTTP_INITIALIZE_SERVER: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: super::USHORT,
    pub pRawValue: windows_core::PCSTR,
}
#[cfg(feature = "shared")]
pub const HTTP_LIMIT_INFINITE: super::ULONG = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: super::BOOLEAN,
}
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: i32 = 1;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: i32 = 4;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: i32 = 8;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: super::ULONG,
    pub SoftwareName: windows_core::PCWSTR,
    pub SoftwareNameLength: super::USHORT,
    pub DirectoryNameLength: super::USHORT,
    pub DirectoryName: windows_core::PCWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: super::ULONG,
    pub pExtFields: super::PVOID,
    pub NumOfExtFields: super::USHORT,
    pub MaxRecordSize: super::USHORT,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: super::ULONG,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
}
pub type HTTP_LOGGING_ROLLOVER_TYPE = i32;
pub type HTTP_LOGGING_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
pub type HTTP_LOG_DATA_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_LOG_FIELDS_DATA {
    pub Base: HTTP_LOG_DATA,
    pub UserNameLength: super::USHORT,
    pub UriStemLength: super::USHORT,
    pub ClientIpLength: super::USHORT,
    pub ServerNameLength: super::USHORT,
    pub ServiceNameLength: super::USHORT,
    pub ServerIpLength: super::USHORT,
    pub MethodLength: super::USHORT,
    pub UriQueryLength: super::USHORT,
    pub HostLength: super::USHORT,
    pub UserAgentLength: super::USHORT,
    pub CookieLength: super::USHORT,
    pub ReferrerLength: super::USHORT,
    pub UserName: super::PWCHAR,
    pub UriStem: super::PWCHAR,
    pub ClientIp: super::PCHAR,
    pub ServerName: super::PCHAR,
    pub ServiceName: super::PCHAR,
    pub ServerIp: super::PCHAR,
    pub Method: super::PCHAR,
    pub UriQuery: super::PCHAR,
    pub Host: super::PCHAR,
    pub UserAgent: super::PCHAR,
    pub Cookie: super::PCHAR,
    pub Referrer: super::PCHAR,
    pub ServerPort: super::USHORT,
    pub ProtocolStatus: super::USHORT,
    pub Win32Status: super::ULONG,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: super::USHORT,
}
pub const HTTP_LOG_FIELD_BYTES_RECV: i32 = 8192;
pub const HTTP_LOG_FIELD_BYTES_SENT: i32 = 4096;
pub const HTTP_LOG_FIELD_CLIENT_IP: i32 = 4;
pub const HTTP_LOG_FIELD_CLIENT_PORT: i32 = 4194304;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: i32 = 32;
pub const HTTP_LOG_FIELD_COOKIE: i32 = 131072;
pub const HTTP_LOG_FIELD_CORRELATION_ID: i32 = 1073741824;
pub const HTTP_LOG_FIELD_DATE: i32 = 1;
pub const HTTP_LOG_FIELD_EXT_FAULT_CODE_EXT: i32 = 1;
pub const HTTP_LOG_FIELD_FAULT_CODE: u32 = 2147483648;
pub const HTTP_LOG_FIELD_HOST: i32 = 1048576;
pub const HTTP_LOG_FIELD_METHOD: i32 = 128;
pub const HTTP_LOG_FIELD_QUEUE_NAME: i32 = 67108864;
pub const HTTP_LOG_FIELD_REASON: i32 = 33554432;
pub const HTTP_LOG_FIELD_REFERER: i32 = 262144;
pub const HTTP_LOG_FIELD_SERVER_IP: i32 = 64;
pub const HTTP_LOG_FIELD_SERVER_PORT: i32 = 32768;
pub const HTTP_LOG_FIELD_SITE_ID: i32 = 16777216;
pub const HTTP_LOG_FIELD_SITE_NAME: i32 = 16;
pub const HTTP_LOG_FIELD_STATUS: i32 = 1024;
pub const HTTP_LOG_FIELD_STREAM_ID: i32 = 134217728;
pub const HTTP_LOG_FIELD_STREAM_ID_EX: i32 = 268435456;
pub const HTTP_LOG_FIELD_SUB_STATUS: i32 = 2097152;
pub const HTTP_LOG_FIELD_TIME: i32 = 2;
pub const HTTP_LOG_FIELD_TIME_TAKEN: i32 = 16384;
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: i32 = 536870912;
pub const HTTP_LOG_FIELD_URI: i32 = 8388608;
pub const HTTP_LOG_FIELD_URI_QUERY: i32 = 512;
pub const HTTP_LOG_FIELD_URI_STEM: i32 = 256;
pub const HTTP_LOG_FIELD_USER_AGENT: i32 = 65536;
pub const HTTP_LOG_FIELD_USER_NAME: i32 = 8;
pub const HTTP_LOG_FIELD_VERSION: i32 = 524288;
pub const HTTP_LOG_FIELD_WIN32_STATUS: i32 = 2048;
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: i32 = 2147483647;
#[cfg(feature = "shared")]
pub const HTTP_MIN_ALLOWED_BANDWIDTH_THROTTLING_RATE: super::ULONG = 1024;
#[cfg(feature = "shared")]
pub const HTTP_MIN_ALLOWED_LOG_FILE_ROLLOVER_SIZE: super::ULONG = 1048576;
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: super::ULONG,
    pub KnownHeaderCount: super::USHORT,
    pub KnownHeaders: PHTTP_KNOWN_HEADER,
}
pub const HTTP_NULL_ID: u64 = 0;
pub type HTTP_OPAQUE_ID = u64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_PERFORMANCE_PARAM {
    pub Type: HTTP_PERFORMANCE_PARAM_TYPE,
    pub BufferSize: super::ULONG,
    pub Buffer: super::PVOID,
}
pub type HTTP_PERFORMANCE_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl HTTP_PROPERTY_FLAGS {
    pub fn Present(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_Present(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
pub type HTTP_PROTECTION_LEVEL_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: super::PVOID,
}
pub type HTTP_QOS_SETTING_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUIC_API_TIMINGS {
    pub ConnectionTimings: HTTP_QUIC_CONNECTION_API_TIMINGS,
    pub StreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUIC_CONNECTION_API_TIMINGS {
    pub OpenTime: super::ULONGLONG,
    pub CloseTime: super::ULONGLONG,
    pub StartTime: super::ULONGLONG,
    pub ShutdownTime: super::ULONGLONG,
    pub SecConfigCreateTime: super::ULONGLONG,
    pub SecConfigDeleteTime: super::ULONGLONG,
    pub GetParamCount: super::ULONGLONG,
    pub GetParamSum: super::ULONGLONG,
    pub SetParamCount: super::ULONGLONG,
    pub SetParamSum: super::ULONGLONG,
    pub SetCallbackHandlerCount: super::ULONGLONG,
    pub SetCallbackHandlerSum: super::ULONGLONG,
    pub ControlStreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
#[cfg(feature = "shared")]
pub const HTTP_QUIC_KEEPALIVE_TIMEOUT_DISABLED: super::ULONG = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUIC_STREAM_API_TIMINGS {
    pub OpenCount: super::ULONGLONG,
    pub OpenSum: super::ULONGLONG,
    pub CloseCount: super::ULONGLONG,
    pub CloseSum: super::ULONGLONG,
    pub StartCount: super::ULONGLONG,
    pub StartSum: super::ULONGLONG,
    pub ShutdownCount: super::ULONGLONG,
    pub ShutdownSum: super::ULONGLONG,
    pub SendCount: super::ULONGLONG,
    pub SendSum: super::ULONGLONG,
    pub ReceiveSetEnabledCount: super::ULONGLONG,
    pub ReceiveSetEnabledSum: super::ULONGLONG,
    pub GetParamCount: super::ULONGLONG,
    pub GetParamSum: super::ULONGLONG,
    pub SetParamCount: super::ULONGLONG,
    pub SetParamSum: super::ULONGLONG,
    pub SetCallbackHandlerCount: super::ULONGLONG,
    pub SetCallbackHandlerSum: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_QUIC_STREAM_REQUEST_STATS {
    pub StreamWaitStart: super::ULONGLONG,
    pub StreamWaitEnd: super::ULONGLONG,
    pub RequestHeadersCompressionStart: super::ULONGLONG,
    pub RequestHeadersCompressionEnd: super::ULONGLONG,
    pub ResponseHeadersDecompressionStart: super::ULONGLONG,
    pub ResponseHeadersDecompressionEnd: super::ULONGLONG,
    pub RequestHeadersCompressedSize: super::ULONGLONG,
    pub ResponseHeadersCompressedSize: super::ULONGLONG,
}
pub type HTTP_RAW_CONNECTION_ID = u64;
pub const HTTP_RECEIVE_FULL_CHAIN: i32 = 2;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: i32 = 1;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: i32 = 1;
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: i32 = 2;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type HTTP_REQUEST = HTTP_REQUEST_V2;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: super::SECURITY_STATUS,
    pub Flags: super::ULONG,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: super::HANDLE,
    pub ContextAttributes: super::ULONG,
    pub PackedContextLength: super::ULONG,
    pub PackedContextType: super::ULONG,
    pub PackedContext: super::PVOID,
    pub MutualAuthDataLength: super::ULONG,
    pub pMutualAuthData: super::PCHAR,
    pub PackageNameLength: super::USHORT,
    pub pPackageName: windows_core::PWSTR,
}
pub type HTTP_REQUEST_AUTH_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: PHTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: super::PUCHAR,
    pub ChannelTokenSize: super::ULONG,
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_DSCP_INFO {
    pub DscpTag: super::BYTE,
}
pub const HTTP_REQUEST_FLAG_FAST_FORWARDING_ALLOWED: i32 = 16;
pub const HTTP_REQUEST_FLAG_FAST_FORWARDING_RESPONSE_ALLOWED: i32 = 16;
pub const HTTP_REQUEST_FLAG_HTTP2: i32 = 4;
pub const HTTP_REQUEST_FLAG_HTTP3: i32 = 8;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: i32 = 2;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: super::USHORT,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: super::USHORT,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
#[cfg(feature = "shared")]
impl Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_REQUEST_ID = u64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: super::ULONG,
    pub pInfo: super::PVOID,
}
pub const HTTP_REQUEST_INFO_FLAG_INITIAL_TTL: u64 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_INFO_PROPERTY_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestInfoFlags: super::ULONGLONG,
}
pub type HTTP_REQUEST_INFO_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_INITIAL_PACKET_TTL_INFO {
    pub InitialPacketTtl: super::BYTE,
}
pub type HTTP_REQUEST_PROPERTY = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_PROPERTY_SNI {
    pub Hostname: [super::WCHAR; 256],
    pub Flags: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: i32 = 2;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: i32 = 1;
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: i32 = 255;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_SIZING_INFO {
    pub Flags: super::ULONGLONG,
    pub RequestIndex: super::ULONG,
    pub RequestSizingCount: super::ULONG,
    pub RequestSizing: [super::ULONGLONG; 5],
}
#[cfg(feature = "shared")]
impl Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: i32 = 8;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: i32 = 1;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: i32 = 4;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: i32 = 2;
pub type HTTP_REQUEST_SIZING_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_TIMING_INFO {
    pub RequestTimingCount: super::ULONG,
    pub RequestTiming: [super::ULONGLONG; 30],
}
#[cfg(feature = "shared")]
impl Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_REQUEST_TIMING_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO {
    pub TokenBinding: super::PUCHAR,
    pub TokenBindingSize: super::ULONG,
    pub EKM: super::PUCHAR,
    pub EKMSize: super::ULONG,
    pub KeyType: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO {
    pub TransportIdleConnectionTimeout: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_V1 {
    pub Flags: super::ULONG,
    pub ConnectionId: HTTP_CONNECTION_ID,
    pub RequestId: HTTP_REQUEST_ID,
    pub UrlContext: HTTP_URL_CONTEXT,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: super::USHORT,
    pub RawUrlLength: super::USHORT,
    pub pUnknownVerb: windows_core::PCSTR,
    pub pRawUrl: windows_core::PCSTR,
    pub CookedUrl: HTTP_COOKED_URL,
    pub Address: HTTP_TRANSPORT_ADDRESS,
    pub Headers: HTTP_REQUEST_HEADERS,
    pub BytesReceived: super::ULONGLONG,
    pub EntityChunkCount: super::USHORT,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
    pub RawConnectionId: HTTP_RAW_CONNECTION_ID,
    pub pSslInfo: PHTTP_SSL_INFO,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_REQUEST_V2 {
    pub base: HTTP_REQUEST_V1,
    pub RequestInfoCount: super::USHORT,
    pub pRequestInfo: PHTTP_REQUEST_INFO,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type HTTP_RESPONSE = HTTP_RESPONSE_V2;
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: i32 = 2;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: super::USHORT,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: super::USHORT,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
#[cfg(feature = "shared")]
impl Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: super::ULONG,
    pub pInfo: super::PVOID,
}
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: i32 = 1;
pub type HTTP_RESPONSE_INFO_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: super::ULONG,
    pub Version: HTTP_VERSION,
    pub StatusCode: super::USHORT,
    pub ReasonLength: super::USHORT,
    pub pReason: windows_core::PCSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: super::USHORT,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_RESPONSE_V2 {
    pub base: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: super::USHORT,
    pub pResponseInfo: PHTTP_RESPONSE_INFO,
}
pub type HTTP_SCHEME = i32;
pub const HTTP_SEND_RESPONSE_FLAG_AUTOMATIC_CHUNKING: i32 = 512;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: i32 = 4;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: i32 = 1;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: i32 = 8;
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: i32 = 256;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: i32 = 2;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: i32 = 64;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: super::USHORT,
    pub Realm: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: super::USHORT,
    pub DomainName: windows_core::PWSTR,
    pub RealmLength: super::USHORT,
    pub Realm: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub AuthSchemes: super::ULONG,
    pub ReceiveMutualAuth: super::BOOLEAN,
    pub ReceiveContextHandle: super::BOOLEAN,
    pub DisableNTLMCredentialCaching: super::BOOLEAN,
    pub ExFlags: super::UCHAR,
    pub DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    pub BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}
pub type HTTP_SERVER_PROPERTY = i32;
pub type HTTP_SERVER_SESSION_ID = u64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::PCHAR,
    pub BufferSize: super::ULONG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
pub type HTTP_SERVICE_BINDING_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: super::PWCHAR,
    pub BufferSize: super::ULONG,
}
pub type HTTP_SERVICE_CONFIG_CACHE_KEY = i32;
pub type HTTP_SERVICE_CONFIG_CACHE_PARAM = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_CACHE_PARAM,
}
pub type HTTP_SERVICE_CONFIG_ID = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: super::USHORT,
    pub pAddress: super::PSOCKADDR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: super::ULONG,
    pub AddrList: [super::SOCKADDR_STORAGE; 1],
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SERVICE_CONFIG_QUERY_TYPE = i32;
pub type HTTP_SERVICE_CONFIG_SETTING_KEY = i32;
pub type HTTP_SERVICE_CONFIG_SETTING_PARAM = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SETTING_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SETTING_PARAM,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: super::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: super::DWORD,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: i32 = 16;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: i32 = 1024;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: i32 = 128;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: i32 = 32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_SESSION_ID: i32 = 16384;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: i32 = 4096;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: i32 = 64;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CACHE_CLIENT_HELLO: i32 = 32768;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: i32 = 8192;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: i32 = 2048;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: i32 = 256;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: i32 = 512;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: i32 = 2;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: i32 = 8;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: super::PSOCKADDR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    pub IpPort: super::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: super::ULONG,
    pub pSslHash: super::PVOID,
    pub AppId: windows_core::GUID,
    pub pSslCertStoreName: windows_core::PWSTR,
    pub DefaultCertCheckMode: super::DWORD,
    pub DefaultRevocationFreshnessTime: super::DWORD,
    pub DefaultRevocationUrlRetrievalTimeout: super::DWORD,
    pub pDefaultSslCtlIdentifier: windows_core::PWSTR,
    pub pDefaultSslCtlStoreName: windows_core::PWSTR,
    pub DefaultFlags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: super::ULONGLONG,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub dwToken: super::DWORD,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::SOCKADDR_STORAGE,
    pub Host: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: super::DWORD,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HTTP_SERVICE_CONFIG_TIMEOUT_KEY = i32;
pub type HTTP_SERVICE_CONFIG_TIMEOUT_PARAM = u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_TIMEOUT_PARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
pub const HTTP_SSL_CERT_SHA_HASH_LENGTH: i32 = 20;
pub const HTTP_SSL_CERT_STORE_NAME_LENGTH: i32 = 128;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: super::ULONG,
    pub CertEncodedSize: super::ULONG,
    pub pCertEncoded: super::PUCHAR,
    pub Token: super::HANDLE,
    pub CertDeniedByMapper: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: super::USHORT,
    pub ConnectionKeySize: super::USHORT,
    pub ServerCertIssuerSize: super::ULONG,
    pub ServerCertSubjectSize: super::ULONG,
    pub pServerCertIssuer: windows_core::PCSTR,
    pub pServerCertSubject: windows_core::PCSTR,
    pub pClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_SSL_PROTOCOL_INFO {
    pub Protocol: super::ULONG,
    pub CipherType: super::ULONG,
    pub CipherStrength: super::ULONG,
    pub HashType: super::ULONG,
    pub HashStrength: super::ULONG,
    pub KeyExchangeType: super::ULONG,
    pub KeyExchangeStrength: super::ULONG,
}
pub type HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: super::USHORT,
    pub DrainEntityBody: super::USHORT,
    pub RequestQueue: super::USHORT,
    pub IdleConnection: super::USHORT,
    pub HeaderWait: super::USHORT,
    pub MinSendRate: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_TLS_RESTRICTIONS_PARAM {
    pub RestrictionCount: super::ULONG,
    pub TlsRestrictions: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    pub SessionTicketKeyCount: super::ULONG,
    pub SessionTicketKeys: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: super::PSOCKADDR,
    pub pLocalAddress: super::PSOCKADDR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: super::USHORT,
    pub RawValueLength: super::USHORT,
    pub pName: windows_core::PCSTR,
    pub pRawValue: windows_core::PCSTR,
}
pub type HTTP_URL_CONTEXT = u64;
pub const HTTP_URL_FLAG_REMOVE_ALL: i32 = 1;
pub type HTTP_URL_GROUP_ID = u64;
pub type HTTP_VERB = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_WINHTTP_FAST_FORWARDING_DATA {
    pub Reserved: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for HTTP_WINHTTP_FAST_FORWARDING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_WSK_API_TIMINGS {
    pub ConnectCount: super::ULONGLONG,
    pub ConnectSum: super::ULONGLONG,
    pub DisconnectCount: super::ULONGLONG,
    pub DisconnectSum: super::ULONGLONG,
    pub SendCount: super::ULONGLONG,
    pub SendSum: super::ULONGLONG,
    pub ReceiveCount: super::ULONGLONG,
    pub ReceiveSum: super::ULONGLONG,
    pub ReleaseCount: super::ULONGLONG,
    pub ReleaseSum: super::ULONGLONG,
    pub ControlSocketCount: super::ULONGLONG,
    pub ControlSocketSum: super::ULONGLONG,
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
#[cfg(feature = "shared")]
pub type PHTTP2_SETTINGS_LIMITS_PARAM = *mut HTTP2_SETTINGS_LIMITS_PARAM;
#[cfg(feature = "shared")]
pub type PHTTP2_WINDOW_SIZE_PARAM = *mut HTTP2_WINDOW_SIZE_PARAM;
#[cfg(feature = "shared")]
pub type PHTTPAPI_VERSION = *mut HTTPAPI_VERSION;
pub type PHTTP_503_RESPONSE_VERBOSITY = *mut HTTP_503_RESPONSE_VERBOSITY;
pub type PHTTP_AUTH_STATUS = *mut HTTP_AUTH_STATUS;
#[cfg(feature = "shared")]
pub type PHTTP_BANDWIDTH_LIMIT_INFO = *mut HTTP_BANDWIDTH_LIMIT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_BINDING_INFO = *mut HTTP_BINDING_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_BYTE_RANGE = *mut HTTP_BYTE_RANGE;
#[cfg(feature = "shared")]
pub type PHTTP_CACHE_POLICY = *mut HTTP_CACHE_POLICY;
pub type PHTTP_CACHE_POLICY_TYPE = *mut HTTP_CACHE_POLICY_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_CERT_CONFIG_ENTRY = *mut HTTP_CERT_CONFIG_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_CERT_CONFIG_PARAM = *mut HTTP_CERT_CONFIG_PARAM;
#[cfg(feature = "shared")]
pub type PHTTP_CHANNEL_BIND_INFO = *mut HTTP_CHANNEL_BIND_INFO;
pub type PHTTP_CLIENT_CONNECTION_ID = *mut u64;
pub type PHTTP_CLIENT_CREDENTIAL_ID = *mut u64;
pub type PHTTP_CLIENT_REQUEST_ID = *mut u64;
pub type PHTTP_CLIENT_STREAM_ID = *mut u64;
pub type PHTTP_CONNECTION_ID = *mut u64;
#[cfg(feature = "shared")]
pub type PHTTP_CONNECTION_LIMIT_INFO = *mut HTTP_CONNECTION_LIMIT_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_COOKED_URL = *mut HTTP_COOKED_URL;
pub type PHTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = *mut HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO = *mut HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_DATA_CHUNK = *mut HTTP_DATA_CHUNK;
pub type PHTTP_DATA_CHUNK_TYPE = *mut HTTP_DATA_CHUNK_TYPE;
pub type PHTTP_DELEGATE_REQUEST_PROPERTY_ID = *mut HTTP_DELEGATE_REQUEST_PROPERTY_ID;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_DELEGATE_REQUEST_PROPERTY_INFO = *mut HTTP_DELEGATE_REQUEST_PROPERTY_INFO;
pub type PHTTP_ENABLED_STATE = *mut HTTP_ENABLED_STATE;
#[cfg(feature = "shared")]
pub type PHTTP_ERROR_HEADERS_PARAM = *mut HTTP_ERROR_HEADERS_PARAM;
#[cfg(feature = "shared")]
pub type PHTTP_FAST_FORWARD_INFO = *mut HTTP_FAST_FORWARD_INFO;
pub type PHTTP_FEATURE_ID = *mut HTTP_FEATURE_ID;
#[cfg(feature = "shared")]
pub type PHTTP_FLOWRATE_INFO = *mut HTTP_FLOWRATE_INFO;
pub type PHTTP_HEADER_ID = *mut HTTP_HEADER_ID;
#[cfg(feature = "shared")]
pub type PHTTP_KNOWN_HEADER = *mut HTTP_KNOWN_HEADER;
#[cfg(feature = "shared")]
pub type PHTTP_LISTEN_ENDPOINT_INFO = *mut HTTP_LISTEN_ENDPOINT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_LOGGING_INFO = *mut HTTP_LOGGING_INFO;
pub type PHTTP_LOGGING_ROLLOVER_TYPE = *mut HTTP_LOGGING_ROLLOVER_TYPE;
pub type PHTTP_LOGGING_TYPE = *mut HTTP_LOGGING_TYPE;
pub type PHTTP_LOG_DATA = *mut HTTP_LOG_DATA;
pub type PHTTP_LOG_DATA_TYPE = *mut HTTP_LOG_DATA_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_LOG_FIELDS_DATA = *mut HTTP_LOG_FIELDS_DATA;
#[cfg(feature = "shared")]
pub type PHTTP_MULTIPLE_KNOWN_HEADERS = *mut HTTP_MULTIPLE_KNOWN_HEADERS;
pub type PHTTP_OPAQUE_ID = *mut u64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_PERFORMANCE_PARAM = *mut HTTP_PERFORMANCE_PARAM;
pub type PHTTP_PERFORMANCE_PARAM_TYPE = *mut HTTP_PERFORMANCE_PARAM_TYPE;
#[cfg(feature = "shared")]
pub type PHTTP_PROPERTY_FLAGS = *mut HTTP_PROPERTY_FLAGS;
#[cfg(feature = "shared")]
pub type PHTTP_PROTECTION_LEVEL_INFO = *mut HTTP_PROTECTION_LEVEL_INFO;
pub type PHTTP_PROTECTION_LEVEL_TYPE = *mut HTTP_PROTECTION_LEVEL_TYPE;
#[cfg(feature = "winnt")]
pub type PHTTP_QOS_SETTING_INFO = *mut HTTP_QOS_SETTING_INFO;
pub type PHTTP_QOS_SETTING_TYPE = *mut HTTP_QOS_SETTING_TYPE;
#[cfg(feature = "shared")]
pub type PHTTP_QUERY_REQUEST_QUALIFIER_QUIC = *mut HTTP_QUERY_REQUEST_QUALIFIER_QUIC;
#[cfg(feature = "shared")]
pub type PHTTP_QUERY_REQUEST_QUALIFIER_TCP = *mut HTTP_QUERY_REQUEST_QUALIFIER_TCP;
#[cfg(feature = "shared")]
pub type PHTTP_QUIC_API_TIMINGS = *mut HTTP_QUIC_API_TIMINGS;
#[cfg(feature = "shared")]
pub type PHTTP_QUIC_CONNECTION_API_TIMINGS = *mut HTTP_QUIC_CONNECTION_API_TIMINGS;
#[cfg(feature = "shared")]
pub type PHTTP_QUIC_STREAM_API_TIMINGS = *mut HTTP_QUIC_STREAM_API_TIMINGS;
#[cfg(feature = "shared")]
pub type PHTTP_QUIC_STREAM_REQUEST_STATS = *mut HTTP_QUIC_STREAM_REQUEST_STATS;
pub type PHTTP_RAW_CONNECTION_ID = *mut u64;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_REQUEST = *mut HTTP_REQUEST_V2;
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "winnt"))]
pub type PHTTP_REQUEST_AUTH_INFO = *mut HTTP_REQUEST_AUTH_INFO;
pub type PHTTP_REQUEST_AUTH_TYPE = *mut HTTP_REQUEST_AUTH_TYPE;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PHTTP_REQUEST_CHANNEL_BIND_STATUS = *mut HTTP_REQUEST_CHANNEL_BIND_STATUS;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_DSCP_INFO = *mut HTTP_REQUEST_DSCP_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_HEADERS = *mut HTTP_REQUEST_HEADERS;
pub type PHTTP_REQUEST_ID = *mut u64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_REQUEST_INFO = *mut HTTP_REQUEST_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_INFO_PROPERTY_INFO = *mut HTTP_REQUEST_INFO_PROPERTY_INFO;
pub type PHTTP_REQUEST_INFO_TYPE = *mut HTTP_REQUEST_INFO_TYPE;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_INITIAL_PACKET_TTL_INFO = *mut HTTP_REQUEST_INITIAL_PACKET_TTL_INFO;
pub type PHTTP_REQUEST_PROPERTY = *mut HTTP_REQUEST_PROPERTY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_REQUEST_PROPERTY_SNI = *mut HTTP_REQUEST_PROPERTY_SNI;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_PROPERTY_STREAM_ERROR = *mut HTTP_REQUEST_PROPERTY_STREAM_ERROR;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_SIZING_INFO = *mut HTTP_REQUEST_SIZING_INFO;
pub type PHTTP_REQUEST_SIZING_TYPE = *mut HTTP_REQUEST_SIZING_TYPE;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_TIMING_INFO = *mut HTTP_REQUEST_TIMING_INFO;
pub type PHTTP_REQUEST_TIMING_TYPE = *mut HTTP_REQUEST_TIMING_TYPE;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PHTTP_REQUEST_TOKEN_BINDING_INFO = *mut HTTP_REQUEST_TOKEN_BINDING_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO = *mut HTTP_REQUEST_TRANSPORT_IDLE_CONNECTION_TIMEOUT_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_REQUEST_V1 = *mut HTTP_REQUEST_V1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_REQUEST_V2 = *mut HTTP_REQUEST_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_RESPONSE = *mut HTTP_RESPONSE_V2;
#[cfg(feature = "shared")]
pub type PHTTP_RESPONSE_HEADERS = *mut HTTP_RESPONSE_HEADERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_RESPONSE_INFO = *mut HTTP_RESPONSE_INFO;
pub type PHTTP_RESPONSE_INFO_TYPE = *mut HTTP_RESPONSE_INFO_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_RESPONSE_V1 = *mut HTTP_RESPONSE_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_RESPONSE_V2 = *mut HTTP_RESPONSE_V2;
#[cfg(feature = "shared")]
pub type PHTTP_SERVER_AUTHENTICATION_BASIC_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS;
#[cfg(feature = "shared")]
pub type PHTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS;
#[cfg(feature = "shared")]
pub type PHTTP_SERVER_AUTHENTICATION_INFO = *mut HTTP_SERVER_AUTHENTICATION_INFO;
pub type PHTTP_SERVER_PROPERTY = *mut HTTP_SERVER_PROPERTY;
pub type PHTTP_SERVER_SESSION_ID = *mut u64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_SERVICE_BINDING_A = *mut HTTP_SERVICE_BINDING_A;
pub type PHTTP_SERVICE_BINDING_BASE = *mut HTTP_SERVICE_BINDING_BASE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_SERVICE_BINDING_W = *mut HTTP_SERVICE_BINDING_W;
pub type PHTTP_SERVICE_CONFIG_CACHE_KEY = *mut HTTP_SERVICE_CONFIG_CACHE_KEY;
pub type PHTTP_SERVICE_CONFIG_CACHE_PARAM = *mut u32;
pub type PHTTP_SERVICE_CONFIG_CACHE_SET = *mut HTTP_SERVICE_CONFIG_CACHE_SET;
pub type PHTTP_SERVICE_CONFIG_ID = *mut HTTP_SERVICE_CONFIG_ID;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_PARAM = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_QUERY = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY;
pub type PHTTP_SERVICE_CONFIG_QUERY_TYPE = *mut HTTP_SERVICE_CONFIG_QUERY_TYPE;
pub type PHTTP_SERVICE_CONFIG_SETTING_KEY = *mut HTTP_SERVICE_CONFIG_SETTING_KEY;
pub type PHTTP_SERVICE_CONFIG_SETTING_PARAM = *mut u32;
pub type PHTTP_SERVICE_CONFIG_SETTING_SET = *mut HTTP_SERVICE_CONFIG_SETTING_SET;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_KEY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_KEY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_KEY = *mut HTTP_SERVICE_CONFIG_SSL_KEY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_KEY_EX = *mut HTTP_SERVICE_CONFIG_SSL_KEY_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM = *mut HTTP_SERVICE_CONFIG_SSL_PARAM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM_EX = *mut HTTP_SERVICE_CONFIG_SSL_PARAM_EX;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_QUERY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_QUERY_EX;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SET = *mut HTTP_SERVICE_CONFIG_SSL_SET;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_SET_EX;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_KEY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_KEY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET;
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET_EX = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_KEY = *mut HTTP_SERVICE_CONFIG_TIMEOUT_KEY;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_PARAM = *mut u16;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_SET = *mut HTTP_SERVICE_CONFIG_TIMEOUT_SET;
pub type PHTTP_SERVICE_CONFIG_URLACL_KEY = *mut HTTP_SERVICE_CONFIG_URLACL_KEY;
pub type PHTTP_SERVICE_CONFIG_URLACL_PARAM = *mut HTTP_SERVICE_CONFIG_URLACL_PARAM;
#[cfg(feature = "shared")]
pub type PHTTP_SERVICE_CONFIG_URLACL_QUERY = *mut HTTP_SERVICE_CONFIG_URLACL_QUERY;
pub type PHTTP_SERVICE_CONFIG_URLACL_SET = *mut HTTP_SERVICE_CONFIG_URLACL_SET;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_SSL_CLIENT_CERT_INFO = *mut HTTP_SSL_CLIENT_CERT_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_SSL_INFO = *mut HTTP_SSL_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_SSL_PROTOCOL_INFO = *mut HTTP_SSL_PROTOCOL_INFO;
pub type PHTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = *mut HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE;
#[cfg(feature = "shared")]
pub type PHTTP_STATE_INFO = *mut HTTP_STATE_INFO;
#[cfg(feature = "shared")]
pub type PHTTP_TIMEOUT_LIMIT_INFO = *mut HTTP_TIMEOUT_LIMIT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_TLS_RESTRICTIONS_PARAM = *mut HTTP_TLS_RESTRICTIONS_PARAM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_TLS_SESSION_TICKET_KEYS_PARAM = *mut HTTP_TLS_SESSION_TICKET_KEYS_PARAM;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PHTTP_TRANSPORT_ADDRESS = *mut HTTP_TRANSPORT_ADDRESS;
#[cfg(feature = "shared")]
pub type PHTTP_UNKNOWN_HEADER = *mut HTTP_UNKNOWN_HEADER;
pub type PHTTP_URI_SCHEME = *mut HTTP_SCHEME;
pub type PHTTP_URL_GROUP_ID = *mut u64;
pub type PHTTP_VERB = *mut HTTP_VERB;
#[cfg(feature = "shared")]
pub type PHTTP_VERSION = *mut HTTP_VERSION;
#[cfg(feature = "shared")]
pub type PHTTP_WINHTTP_FAST_FORWARDING_DATA = *mut HTTP_WINHTTP_FAST_FORWARDING_DATA;
#[cfg(feature = "shared")]
pub type PHTTP_WSK_API_TIMINGS = *mut HTTP_WSK_API_TIMINGS;
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = 1;
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = 5;
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = 6;
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = 3;
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = 4;
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = 2;
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = 0;
