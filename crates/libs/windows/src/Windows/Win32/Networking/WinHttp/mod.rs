#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpAddRequestHeaders(hrequest: *mut ::core::ffi::c_void, lpszheaders: &[u16], dwmodifiers: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpAddRequestHeaders(hrequest : *mut ::core::ffi::c_void, lpszheaders : ::windows_core::PCWSTR, dwheaderslength : u32, dwmodifiers : u32) -> super::super::Foundation:: BOOL);
    WinHttpAddRequestHeaders(hrequest, ::core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len().try_into().unwrap(), dwmodifiers).ok()
}
#[inline]
pub unsafe fn WinHttpAddRequestHeadersEx(hrequest: *mut ::core::ffi::c_void, dwmodifiers: u32, ullflags: u64, ullextra: u64, pheaders: &[WINHTTP_EXTENDED_HEADER]) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpAddRequestHeadersEx(hrequest : *mut ::core::ffi::c_void, dwmodifiers : u32, ullflags : u64, ullextra : u64, cheaders : u32, pheaders : *const WINHTTP_EXTENDED_HEADER) -> u32);
    WinHttpAddRequestHeadersEx(hrequest, dwmodifiers, ullflags, ullextra, pheaders.len().try_into().unwrap(), ::core::mem::transmute(pheaders.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpCheckPlatform() -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpCheckPlatform() -> super::super::Foundation:: BOOL);
    WinHttpCheckPlatform().ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpCloseHandle(hinternet: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpCloseHandle(hinternet : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WinHttpCloseHandle(hinternet).ok()
}
#[inline]
pub unsafe fn WinHttpConnect<P0>(hsession: *mut ::core::ffi::c_void, pswzservername: P0, nserverport: u16, dwreserved: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpConnect(hsession : *mut ::core::ffi::c_void, pswzservername : ::windows_core::PCWSTR, nserverport : u16, dwreserved : u32) -> *mut ::core::ffi::c_void);
    WinHttpConnect(hsession, pswzservername.into_param().abi(), nserverport, dwreserved)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpCrackUrl(pwszurl: &[u16], dwflags: u32, lpurlcomponents: *mut URL_COMPONENTS) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpCrackUrl(pwszurl : ::windows_core::PCWSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : *mut URL_COMPONENTS) -> super::super::Foundation:: BOOL);
    WinHttpCrackUrl(::core::mem::transmute(pwszurl.as_ptr()), pwszurl.len().try_into().unwrap(), dwflags, lpurlcomponents).ok()
}
#[inline]
pub unsafe fn WinHttpCreateProxyResolver(hsession: *const ::core::ffi::c_void, phresolver: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpCreateProxyResolver(hsession : *const ::core::ffi::c_void, phresolver : *mut *mut ::core::ffi::c_void) -> u32);
    WinHttpCreateProxyResolver(hsession, phresolver)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpCreateUrl(lpurlcomponents: *const URL_COMPONENTS, dwflags: WIN_HTTP_CREATE_URL_FLAGS, pwszurl: ::windows_core::PWSTR, pdwurllength: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpCreateUrl(lpurlcomponents : *const URL_COMPONENTS, dwflags : WIN_HTTP_CREATE_URL_FLAGS, pwszurl : ::windows_core::PWSTR, pdwurllength : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpCreateUrl(lpurlcomponents, dwflags, ::core::mem::transmute(pwszurl), pdwurllength).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags: u32, ppwstrautoconfigurl: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags : u32, ppwstrautoconfigurl : *mut ::windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    WinHttpDetectAutoProxyConfigUrl(dwautodetectflags, ppwstrautoconfigurl).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpFreeProxyResult(pproxyresult: *mut WINHTTP_PROXY_RESULT) {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpFreeProxyResult(pproxyresult : *mut WINHTTP_PROXY_RESULT) -> ());
    WinHttpFreeProxyResult(pproxyresult)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpFreeProxyResultEx(pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpFreeProxyResultEx(pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX) -> ());
    WinHttpFreeProxyResultEx(pproxyresultex)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpFreeProxySettings(pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpFreeProxySettings(pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS) -> ());
    WinHttpFreeProxySettings(pwinhttpproxysettings)
}
#[inline]
pub unsafe fn WinHttpFreeProxySettingsEx(proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpFreeProxySettingsEx(proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex : *const ::core::ffi::c_void) -> u32);
    WinHttpFreeProxySettingsEx(proxysettingstype, pproxysettingsex)
}
#[inline]
pub unsafe fn WinHttpFreeQueryConnectionGroupResult(presult: *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpFreeQueryConnectionGroupResult(presult : *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> ());
    WinHttpFreeQueryConnectionGroupResult(presult)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> super::super::Foundation:: BOOL);
    WinHttpGetDefaultProxyConfiguration(pproxyinfo).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig : *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> super::super::Foundation:: BOOL);
    WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetProxyForUrl<P0>(hsession: *mut ::core::ffi::c_void, lpcwszurl: P0, pautoproxyoptions: *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo: *mut WINHTTP_PROXY_INFO) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrl(hsession : *mut ::core::ffi::c_void, lpcwszurl : ::windows_core::PCWSTR, pautoproxyoptions : *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo : *mut WINHTTP_PROXY_INFO) -> super::super::Foundation:: BOOL);
    WinHttpGetProxyForUrl(hsession, lpcwszurl.into_param().abi(), pautoproxyoptions, pproxyinfo).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx<P0>(hresolver: *const ::core::ffi::c_void, pcwszurl: P0, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pcontext: usize) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx(hresolver : *const ::core::ffi::c_void, pcwszurl : ::windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, pcontext : usize) -> u32);
    WinHttpGetProxyForUrlEx(hresolver, pcwszurl.into_param().abi(), pautoproxyoptions, pcontext)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx2<P0>(hresolver: *const ::core::ffi::c_void, pcwszurl: P0, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pinterfaceselectioncontext: ::core::option::Option<&[u8]>, pcontext: usize) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx2(hresolver : *const ::core::ffi::c_void, pcwszurl : ::windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, cbinterfaceselectioncontext : u32, pinterfaceselectioncontext : *const u8, pcontext : usize) -> u32);
    WinHttpGetProxyForUrlEx2(hresolver, pcwszurl.into_param().abi(), pautoproxyoptions, pinterfaceselectioncontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pinterfaceselectioncontext.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcontext)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetProxyResult(hresolver: *const ::core::ffi::c_void, pproxyresult: *mut WINHTTP_PROXY_RESULT) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxyResult(hresolver : *const ::core::ffi::c_void, pproxyresult : *mut WINHTTP_PROXY_RESULT) -> u32);
    WinHttpGetProxyResult(hresolver, pproxyresult)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpGetProxyResultEx(hresolver: *const ::core::ffi::c_void, pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxyResultEx(hresolver : *const ::core::ffi::c_void, pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX) -> u32);
    WinHttpGetProxyResultEx(hresolver, pproxyresultex)
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsEx(hresolver: *const ::core::ffi::c_void, proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam: ::core::option::Option<*const WINHTTP_PROXY_SETTINGS_PARAM>, pcontext: usize) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsEx(hresolver : *const ::core::ffi::c_void, proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam : *const WINHTTP_PROXY_SETTINGS_PARAM, pcontext : usize) -> u32);
    WinHttpGetProxySettingsEx(hresolver, proxysettingstype, ::core::mem::transmute(pproxysettingsparam.unwrap_or(::std::ptr::null())), pcontext)
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsResultEx(hresolver: *const ::core::ffi::c_void, pproxysettingsex: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsResultEx(hresolver : *const ::core::ffi::c_void, pproxysettingsex : *mut ::core::ffi::c_void) -> u32);
    WinHttpGetProxySettingsResultEx(hresolver, pproxysettingsex)
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsVersion(hsession: *const ::core::ffi::c_void, pdwproxysettingsversion: *mut u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsVersion(hsession : *const ::core::ffi::c_void, pdwproxysettingsversion : *mut u32) -> u32);
    WinHttpGetProxySettingsVersion(hsession, pdwproxysettingsversion)
}
#[inline]
pub unsafe fn WinHttpOpen<P0, P1, P2>(pszagentw: P0, dwaccesstype: WINHTTP_ACCESS_TYPE, pszproxyw: P1, pszproxybypassw: P2, dwflags: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpOpen(pszagentw : ::windows_core::PCWSTR, dwaccesstype : WINHTTP_ACCESS_TYPE, pszproxyw : ::windows_core::PCWSTR, pszproxybypassw : ::windows_core::PCWSTR, dwflags : u32) -> *mut ::core::ffi::c_void);
    WinHttpOpen(pszagentw.into_param().abi(), dwaccesstype, pszproxyw.into_param().abi(), pszproxybypassw.into_param().abi(), dwflags)
}
#[inline]
pub unsafe fn WinHttpOpenRequest<P0, P1, P2, P3>(hconnect: *mut ::core::ffi::c_void, pwszverb: P0, pwszobjectname: P1, pwszversion: P2, pwszreferrer: P3, ppwszaccepttypes: *const ::windows_core::PCWSTR, dwflags: WINHTTP_OPEN_REQUEST_FLAGS) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpOpenRequest(hconnect : *mut ::core::ffi::c_void, pwszverb : ::windows_core::PCWSTR, pwszobjectname : ::windows_core::PCWSTR, pwszversion : ::windows_core::PCWSTR, pwszreferrer : ::windows_core::PCWSTR, ppwszaccepttypes : *const ::windows_core::PCWSTR, dwflags : WINHTTP_OPEN_REQUEST_FLAGS) -> *mut ::core::ffi::c_void);
    WinHttpOpenRequest(hconnect, pwszverb.into_param().abi(), pwszobjectname.into_param().abi(), pwszversion.into_param().abi(), pwszreferrer.into_param().abi(), ppwszaccepttypes, dwflags)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpQueryAuthSchemes(hrequest: *mut ::core::ffi::c_void, lpdwsupportedschemes: *mut u32, lpdwfirstscheme: *mut u32, pdwauthtarget: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryAuthSchemes(hrequest : *mut ::core::ffi::c_void, lpdwsupportedschemes : *mut u32, lpdwfirstscheme : *mut u32, pdwauthtarget : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpQueryAuthSchemes(hrequest, lpdwsupportedschemes, lpdwfirstscheme, pdwauthtarget).ok()
}
#[inline]
pub unsafe fn WinHttpQueryConnectionGroup(hinternet: *const ::core::ffi::c_void, pguidconnection: ::core::option::Option<*const ::windows_core::GUID>, ullflags: u64, ppresult: *mut *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryConnectionGroup(hinternet : *const ::core::ffi::c_void, pguidconnection : *const ::windows_core::GUID, ullflags : u64, ppresult : *mut *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32);
    WinHttpQueryConnectionGroup(hinternet, ::core::mem::transmute(pguidconnection.unwrap_or(::std::ptr::null())), ullflags, ppresult)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpQueryDataAvailable(hrequest: *mut ::core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryDataAvailable(hrequest : *mut ::core::ffi::c_void, lpdwnumberofbytesavailable : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpQueryDataAvailable(hrequest, lpdwnumberofbytesavailable).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpQueryHeaders<P0>(hrequest: *mut ::core::ffi::c_void, dwinfolevel: u32, pwszname: P0, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryHeaders(hrequest : *mut ::core::ffi::c_void, dwinfolevel : u32, pwszname : ::windows_core::PCWSTR, lpbuffer : *mut ::core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpQueryHeaders(hrequest, dwinfolevel, pwszname.into_param().abi(), ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength, lpdwindex).ok()
}
#[inline]
pub unsafe fn WinHttpQueryHeadersEx(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, ullflags: u64, uicodepage: u32, pdwindex: ::core::option::Option<*mut u32>, pheadername: ::core::option::Option<*const WINHTTP_HEADER_NAME>, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, pdwbufferlength: *mut u32, ppheaders: ::core::option::Option<*mut *mut WINHTTP_EXTENDED_HEADER>, pdwheaderscount: *mut u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryHeadersEx(hrequest : *const ::core::ffi::c_void, dwinfolevel : u32, ullflags : u64, uicodepage : u32, pdwindex : *mut u32, pheadername : *const WINHTTP_HEADER_NAME, pbuffer : *mut ::core::ffi::c_void, pdwbufferlength : *mut u32, ppheaders : *mut *mut WINHTTP_EXTENDED_HEADER, pdwheaderscount : *mut u32) -> u32);
    WinHttpQueryHeadersEx(hrequest, dwinfolevel, ullflags, uicodepage, ::core::mem::transmute(pdwindex.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pheadername.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), pdwbufferlength, ::core::mem::transmute(ppheaders.unwrap_or(::std::ptr::null_mut())), pdwheaderscount)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpQueryOption(hinternet: *mut ::core::ffi::c_void, dwoption: u32, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpQueryOption(hinternet : *mut ::core::ffi::c_void, dwoption : u32, lpbuffer : *mut ::core::ffi::c_void, lpdwbufferlength : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpQueryOption(hinternet, dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpReadData(hrequest: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpReadData(hrequest : *mut ::core::ffi::c_void, lpbuffer : *mut ::core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpReadData(hrequest, lpbuffer, dwnumberofbytestoread, lpdwnumberofbytesread).ok()
}
#[inline]
pub unsafe fn WinHttpReadDataEx(hrequest: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32, ullflags: u64, cbproperty: u32, pvproperty: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpReadDataEx(hrequest : *mut ::core::ffi::c_void, lpbuffer : *mut ::core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32, ullflags : u64, cbproperty : u32, pvproperty : *const ::core::ffi::c_void) -> u32);
    WinHttpReadDataEx(hrequest, lpbuffer, dwnumberofbytestoread, lpdwnumberofbytesread, ullflags, cbproperty, ::core::mem::transmute(pvproperty.unwrap_or(::std::ptr::null())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpReadProxySettings<P0, P1, P2>(hsession: *const ::core::ffi::c_void, pcwszconnectionname: P0, ffallbacktodefaultsettings: P1, fsetautodiscoverfordefaultsettings: P2, pdwsettingsversion: *mut u32, pfdefaultsettingsarereturned: *mut super::super::Foundation::BOOL, pwinhttpproxysettings: *mut WINHTTP_PROXY_SETTINGS) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpReadProxySettings(hsession : *const ::core::ffi::c_void, pcwszconnectionname : ::windows_core::PCWSTR, ffallbacktodefaultsettings : super::super::Foundation:: BOOL, fsetautodiscoverfordefaultsettings : super::super::Foundation:: BOOL, pdwsettingsversion : *mut u32, pfdefaultsettingsarereturned : *mut super::super::Foundation:: BOOL, pwinhttpproxysettings : *mut WINHTTP_PROXY_SETTINGS) -> u32);
    WinHttpReadProxySettings(hsession, pcwszconnectionname.into_param().abi(), ffallbacktodefaultsettings.into_param().abi(), fsetautodiscoverfordefaultsettings.into_param().abi(), pdwsettingsversion, pfdefaultsettingsarereturned, pwinhttpproxysettings)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpReceiveResponse(hrequest: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpReceiveResponse(hrequest : *mut ::core::ffi::c_void, lpreserved : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WinHttpReceiveResponse(hrequest, lpreserved).ok()
}
#[inline]
pub unsafe fn WinHttpRegisterProxyChangeNotification(ullflags: u64, pfncallback: WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext: *const ::core::ffi::c_void, hregistration: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpRegisterProxyChangeNotification(ullflags : u64, pfncallback : WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext : *const ::core::ffi::c_void, hregistration : *mut *mut ::core::ffi::c_void) -> u32);
    WinHttpRegisterProxyChangeNotification(ullflags, pfncallback, pvcontext, hregistration)
}
#[inline]
pub unsafe fn WinHttpResetAutoProxy(hsession: *const ::core::ffi::c_void, dwflags: u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpResetAutoProxy(hsession : *const ::core::ffi::c_void, dwflags : u32) -> u32);
    WinHttpResetAutoProxy(hsession, dwflags)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSendRequest(hrequest: *mut ::core::ffi::c_void, lpszheaders: ::core::option::Option<&[u16]>, lpoptional: ::core::option::Option<*const ::core::ffi::c_void>, dwoptionallength: u32, dwtotallength: u32, dwcontext: usize) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSendRequest(hrequest : *mut ::core::ffi::c_void, lpszheaders : ::windows_core::PCWSTR, dwheaderslength : u32, lpoptional : *const ::core::ffi::c_void, dwoptionallength : u32, dwtotallength : u32, dwcontext : usize) -> super::super::Foundation:: BOOL);
    WinHttpSendRequest(hrequest, ::core::mem::transmute(lpszheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(lpoptional.unwrap_or(::std::ptr::null())), dwoptionallength, dwtotallength, dwcontext).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSetCredentials<P0, P1>(hrequest: *mut ::core::ffi::c_void, authtargets: u32, authscheme: u32, pwszusername: P0, pwszpassword: P1, pauthparams: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetCredentials(hrequest : *mut ::core::ffi::c_void, authtargets : u32, authscheme : u32, pwszusername : ::windows_core::PCWSTR, pwszpassword : ::windows_core::PCWSTR, pauthparams : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WinHttpSetCredentials(hrequest, authtargets, authscheme, pwszusername.into_param().abi(), pwszpassword.into_param().abi(), pauthparams).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> super::super::Foundation:: BOOL);
    WinHttpSetDefaultProxyConfiguration(pproxyinfo).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSetOption(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetOption(hinternet : *const ::core::ffi::c_void, dwoption : u32, lpbuffer : *const ::core::ffi::c_void, dwbufferlength : u32) -> super::super::Foundation:: BOOL);
    WinHttpSetOption(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSetProxySettingsPerUser<P0>(fproxysettingsperuser: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser : super::super::Foundation:: BOOL) -> u32);
    WinHttpSetProxySettingsPerUser(fproxysettingsperuser.into_param().abi())
}
#[inline]
pub unsafe fn WinHttpSetStatusCallback(hinternet: *mut ::core::ffi::c_void, lpfninternetcallback: WINHTTP_STATUS_CALLBACK, dwnotificationflags: u32, dwreserved: usize) -> WINHTTP_STATUS_CALLBACK {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetStatusCallback(hinternet : *mut ::core::ffi::c_void, lpfninternetcallback : WINHTTP_STATUS_CALLBACK, dwnotificationflags : u32, dwreserved : usize) -> WINHTTP_STATUS_CALLBACK);
    WinHttpSetStatusCallback(hinternet, lpfninternetcallback, dwnotificationflags, dwreserved)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpSetTimeouts(hinternet: *mut ::core::ffi::c_void, nresolvetimeout: i32, nconnecttimeout: i32, nsendtimeout: i32, nreceivetimeout: i32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpSetTimeouts(hinternet : *mut ::core::ffi::c_void, nresolvetimeout : i32, nconnecttimeout : i32, nsendtimeout : i32, nreceivetimeout : i32) -> super::super::Foundation:: BOOL);
    WinHttpSetTimeouts(hinternet, nresolvetimeout, nconnecttimeout, nsendtimeout, nreceivetimeout).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, pwsztime: &mut [u16; 62]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpTimeFromSystemTime(pst : *const super::super::Foundation:: SYSTEMTIME, pwsztime : ::windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    WinHttpTimeFromSystemTime(pst, ::core::mem::transmute(pwsztime.as_ptr())).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpTimeToSystemTime<P0>(pwsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpTimeToSystemTime(pwsztime : ::windows_core::PCWSTR, pst : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    WinHttpTimeToSystemTime(pwsztime.into_param().abi(), pst).ok()
}
#[inline]
pub unsafe fn WinHttpUnregisterProxyChangeNotification(hregistration: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpUnregisterProxyChangeNotification(hregistration : *const ::core::ffi::c_void) -> u32);
    WinHttpUnregisterProxyChangeNotification(hregistration)
}
#[inline]
pub unsafe fn WinHttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: ::core::option::Option<*const ::core::ffi::c_void>, dwreasonlength: u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketClose(hwebsocket : *const ::core::ffi::c_void, usstatus : u16, pvreason : *const ::core::ffi::c_void, dwreasonlength : u32) -> u32);
    WinHttpWebSocketClose(hwebsocket, usstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null())), dwreasonlength)
}
#[inline]
pub unsafe fn WinHttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, pcontext: usize) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketCompleteUpgrade(hrequest : *const ::core::ffi::c_void, pcontext : usize) -> *mut ::core::ffi::c_void);
    WinHttpWebSocketCompleteUpgrade(hrequest, pcontext)
}
#[inline]
pub unsafe fn WinHttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: ::core::option::Option<*mut ::core::ffi::c_void>, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketQueryCloseStatus(hwebsocket : *const ::core::ffi::c_void, pusstatus : *mut u16, pvreason : *mut ::core::ffi::c_void, dwreasonlength : u32, pdwreasonlengthconsumed : *mut u32) -> u32);
    WinHttpWebSocketQueryCloseStatus(hwebsocket, pusstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null_mut())), dwreasonlength, pdwreasonlengthconsumed)
}
#[inline]
pub unsafe fn WinHttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pebuffertype: *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketReceive(hwebsocket : *const ::core::ffi::c_void, pvbuffer : *mut ::core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32, pebuffertype : *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32);
    WinHttpWebSocketReceive(hwebsocket, pvbuffer, dwbufferlength, pdwbytesread, pebuffertype)
}
#[inline]
pub unsafe fn WinHttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, ebuffertype: WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketSend(hwebsocket : *const ::core::ffi::c_void, ebuffertype : WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer : *const ::core::ffi::c_void, dwbufferlength : u32) -> u32);
    WinHttpWebSocketSend(hwebsocket, ebuffertype, ::core::mem::transmute(pvbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn WinHttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: ::core::option::Option<*const ::core::ffi::c_void>, dwreasonlength: u32) -> u32 {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWebSocketShutdown(hwebsocket : *const ::core::ffi::c_void, usstatus : u16, pvreason : *const ::core::ffi::c_void, dwreasonlength : u32) -> u32);
    WinHttpWebSocketShutdown(hwebsocket, usstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null())), dwreasonlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpWriteData(hrequest: *mut ::core::ffi::c_void, lpbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWriteData(hrequest : *mut ::core::ffi::c_void, lpbuffer : *const ::core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WinHttpWriteData(hrequest, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null())), dwnumberofbytestowrite, lpdwnumberofbyteswritten).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinHttpWriteProxySettings<P0>(hsession: *const ::core::ffi::c_void, fforceupdate: P0, pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("winhttp.dll" "system" fn WinHttpWriteProxySettings(hsession : *const ::core::ffi::c_void, fforceupdate : super::super::Foundation:: BOOL, pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS) -> u32);
    WinHttpWriteProxySettings(hsession, fforceupdate.into_param().abi(), pwinhttpproxysettings)
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWinHttpRequest(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWinHttpRequest {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProxy(&self, proxysetting: i32, proxyserver: super::super::System::Variant::VARIANT, bypasslist: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxy)(::windows_core::Interface::as_raw(self), proxysetting, ::core::mem::transmute(proxyserver), ::core::mem::transmute(bypasslist)).ok()
    }
    pub unsafe fn SetCredentials<P0, P1>(&self, username: P0, password: P1, flags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), username.into_param().abi(), password.into_param().abi(), flags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open<P0, P1>(&self, method: P0, url: P1, r#async: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), method.into_param().abi(), url.into_param().abi(), ::core::mem::transmute(r#async)).ok()
    }
    pub unsafe fn SetRequestHeader<P0, P1>(&self, header: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRequestHeader)(::windows_core::Interface::as_raw(self), header.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetResponseHeader<P0>(&self, header: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResponseHeader)(::windows_core::Interface::as_raw(self), header.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAllResponseHeaders(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAllResponseHeaders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send(&self, body: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(body)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ResponseText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResponseText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResponseBody(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResponseBody)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResponseStream(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResponseStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Option(&self, option: WinHttpRequestOption) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Option)(::windows_core::Interface::as_raw(self), option, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_Option(&self, option: WinHttpRequestOption, value: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_Option)(::windows_core::Interface::as_raw(self), option, ::core::mem::transmute(value)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn WaitForResponse(&self, timeout: super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WaitForResponse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(timeout), &mut result__).from_abi(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTimeouts)(::windows_core::Interface::as_raw(self), resolvetimeout, connecttimeout, sendtimeout, receivetimeout).ok()
    }
    pub unsafe fn SetClientCertificate<P0>(&self, clientcertificate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientCertificate)(::windows_core::Interface::as_raw(self), clientcertificate.into_param().abi()).ok()
    }
    pub unsafe fn SetAutoLogonPolicy(&self, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoLogonPolicy)(::windows_core::Interface::as_raw(self), autologonpolicy).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWinHttpRequest, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWinHttpRequest {
    type Vtable = IWinHttpRequest_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWinHttpRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x016fe2ec_b2c8_45f8_b23b_39e53a75396b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWinHttpRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proxysetting: i32, proxyserver: super::super::System::Variant::VARIANT, bypasslist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProxy: usize,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::std::mem::MaybeUninit<::windows_core::BSTR>, url: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#async: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Open: usize,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetResponseHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetAllResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ResponseText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResponseBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResponseBody: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResponseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResponseStream: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Option: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Option: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_Option: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_Option: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub WaitForResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::super::System::Variant::VARIANT, succeeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    WaitForResponse: usize,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTimeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::HRESULT,
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientcertificate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetAutoLogonPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWinHttpRequestEvents(::windows_core::IUnknown);
impl IWinHttpRequestEvents {
    pub unsafe fn OnResponseStart<P0>(&self, status: i32, contenttype: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnResponseStart)(::windows_core::Interface::as_raw(self), status, contenttype.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnResponseDataAvailable(&self, data: *const *const super::super::System::Com::SAFEARRAY) {
        (::windows_core::Interface::vtable(self).OnResponseDataAvailable)(::windows_core::Interface::as_raw(self), data)
    }
    pub unsafe fn OnResponseFinished(&self) {
        (::windows_core::Interface::vtable(self).OnResponseFinished)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnError<P0>(&self, errornumber: i32, errordescription: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), errornumber, errordescription.into_param().abi())
    }
}
::windows_core::imp::interface_hierarchy!(IWinHttpRequestEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWinHttpRequestEvents {
    type Vtable = IWinHttpRequestEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWinHttpRequestEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf97f4e15_b787_4212_80d1_d380cbbf982e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinHttpRequestEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnResponseStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: i32, contenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    #[cfg(feature = "Win32_System_Com")]
    pub OnResponseDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *const *const super::super::System::Com::SAFEARRAY),
    #[cfg(not(feature = "Win32_System_Com"))]
    OnResponseDataAvailable: usize,
    pub OnResponseFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errornumber: i32, errordescription: ::std::mem::MaybeUninit<::windows_core::BSTR>),
}
pub const API_GET_PROXY_FOR_URL: u32 = 6u32;
pub const API_GET_PROXY_SETTINGS: u32 = 7u32;
pub const API_QUERY_DATA_AVAILABLE: u32 = 2u32;
pub const API_READ_DATA: u32 = 3u32;
pub const API_RECEIVE_RESPONSE: u32 = 1u32;
pub const API_SEND_REQUEST: u32 = 5u32;
pub const API_WRITE_DATA: u32 = 4u32;
pub const AutoLogonPolicy_Always: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(0i32);
pub const AutoLogonPolicy_Never: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(2i32);
pub const AutoLogonPolicy_OnlyIfBypassProxy: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(1i32);
pub const ERROR_WINHTTP_AUTODETECTION_FAILED: u32 = 12180u32;
pub const ERROR_WINHTTP_AUTO_PROXY_SERVICE_ERROR: u32 = 12178u32;
pub const ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT: u32 = 12166u32;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_OPEN: u32 = 12103u32;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_SEND: u32 = 12102u32;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_OPEN: u32 = 12100u32;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_SEND: u32 = 12101u32;
pub const ERROR_WINHTTP_CANNOT_CONNECT: u32 = 12029u32;
pub const ERROR_WINHTTP_CHUNKED_ENCODING_HEADER_SIZE_OVERFLOW: u32 = 12183u32;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED: u32 = 12044u32;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187u32;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_ACCESS_PRIVATE_KEY: u32 = 12186u32;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_PRIVATE_KEY: u32 = 12185u32;
pub const ERROR_WINHTTP_CONNECTION_ERROR: u32 = 12030u32;
pub const ERROR_WINHTTP_FEATURE_DISABLED: u32 = 12192u32;
pub const ERROR_WINHTTP_GLOBAL_CALLBACK_FAILED: u32 = 12191u32;
pub const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: u32 = 12155u32;
pub const ERROR_WINHTTP_HEADER_COUNT_EXCEEDED: u32 = 12181u32;
pub const ERROR_WINHTTP_HEADER_NOT_FOUND: u32 = 12150u32;
pub const ERROR_WINHTTP_HEADER_SIZE_OVERFLOW: u32 = 12182u32;
pub const ERROR_WINHTTP_HTTP_PROTOCOL_MISMATCH: u32 = 12190u32;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: u32 = 12019u32;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: u32 = 12018u32;
pub const ERROR_WINHTTP_INTERNAL_ERROR: u32 = 12004u32;
pub const ERROR_WINHTTP_INVALID_HEADER: u32 = 12153u32;
pub const ERROR_WINHTTP_INVALID_OPTION: u32 = 12009u32;
pub const ERROR_WINHTTP_INVALID_QUERY_REQUEST: u32 = 12154u32;
pub const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: u32 = 12152u32;
pub const ERROR_WINHTTP_INVALID_URL: u32 = 12005u32;
pub const ERROR_WINHTTP_LOGIN_FAILURE: u32 = 12015u32;
pub const ERROR_WINHTTP_NAME_NOT_RESOLVED: u32 = 12007u32;
pub const ERROR_WINHTTP_NOT_INITIALIZED: u32 = 12172u32;
pub const ERROR_WINHTTP_OPERATION_CANCELLED: u32 = 12017u32;
pub const ERROR_WINHTTP_OPTION_NOT_SETTABLE: u32 = 12011u32;
pub const ERROR_WINHTTP_OUT_OF_HANDLES: u32 = 12001u32;
pub const ERROR_WINHTTP_REDIRECT_FAILED: u32 = 12156u32;
pub const ERROR_WINHTTP_RESEND_REQUEST: u32 = 12032u32;
pub const ERROR_WINHTTP_RESERVED_189: u32 = 12189u32;
pub const ERROR_WINHTTP_RESPONSE_DRAIN_OVERFLOW: u32 = 12184u32;
pub const ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR: u32 = 12177u32;
pub const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: u32 = 12038u32;
pub const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: u32 = 12037u32;
pub const ERROR_WINHTTP_SECURE_CERT_REVOKED: u32 = 12170u32;
pub const ERROR_WINHTTP_SECURE_CERT_REV_FAILED: u32 = 12057u32;
pub const ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE: u32 = 12179u32;
pub const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: u32 = 12157u32;
pub const ERROR_WINHTTP_SECURE_FAILURE: u32 = 12175u32;
pub const ERROR_WINHTTP_SECURE_FAILURE_PROXY: u32 = 12188u32;
pub const ERROR_WINHTTP_SECURE_INVALID_CA: u32 = 12045u32;
pub const ERROR_WINHTTP_SECURE_INVALID_CERT: u32 = 12169u32;
pub const ERROR_WINHTTP_SHUTDOWN: u32 = 12012u32;
pub const ERROR_WINHTTP_TIMEOUT: u32 = 12002u32;
pub const ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167u32;
pub const ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE: u32 = 12176u32;
pub const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: u32 = 12006u32;
pub const HTTPREQUEST_PROXYSETTING_DEFAULT: u32 = 0u32;
pub const HTTPREQUEST_PROXYSETTING_DIRECT: u32 = 1u32;
pub const HTTPREQUEST_PROXYSETTING_PRECONFIG: u32 = 0u32;
pub const HTTPREQUEST_PROXYSETTING_PROXY: u32 = 2u32;
pub const HTTPREQUEST_SETCREDENTIALS_FOR_PROXY: u32 = 1u32;
pub const HTTPREQUEST_SETCREDENTIALS_FOR_SERVER: u32 = 0u32;
pub const HTTP_STATUS_ACCEPTED: u32 = 202u32;
pub const HTTP_STATUS_AMBIGUOUS: u32 = 300u32;
pub const HTTP_STATUS_BAD_GATEWAY: u32 = 502u32;
pub const HTTP_STATUS_BAD_METHOD: u32 = 405u32;
pub const HTTP_STATUS_BAD_REQUEST: u32 = 400u32;
pub const HTTP_STATUS_CONFLICT: u32 = 409u32;
pub const HTTP_STATUS_CONTINUE: u32 = 100u32;
pub const HTTP_STATUS_CREATED: u32 = 201u32;
pub const HTTP_STATUS_DENIED: u32 = 401u32;
pub const HTTP_STATUS_FIRST: u32 = 100u32;
pub const HTTP_STATUS_FORBIDDEN: u32 = 403u32;
pub const HTTP_STATUS_GATEWAY_TIMEOUT: u32 = 504u32;
pub const HTTP_STATUS_GONE: u32 = 410u32;
pub const HTTP_STATUS_LAST: u32 = 505u32;
pub const HTTP_STATUS_LENGTH_REQUIRED: u32 = 411u32;
pub const HTTP_STATUS_MOVED: u32 = 301u32;
pub const HTTP_STATUS_NONE_ACCEPTABLE: u32 = 406u32;
pub const HTTP_STATUS_NOT_FOUND: u32 = 404u32;
pub const HTTP_STATUS_NOT_MODIFIED: u32 = 304u32;
pub const HTTP_STATUS_NOT_SUPPORTED: u32 = 501u32;
pub const HTTP_STATUS_NO_CONTENT: u32 = 204u32;
pub const HTTP_STATUS_OK: u32 = 200u32;
pub const HTTP_STATUS_PARTIAL: u32 = 203u32;
pub const HTTP_STATUS_PARTIAL_CONTENT: u32 = 206u32;
pub const HTTP_STATUS_PAYMENT_REQ: u32 = 402u32;
pub const HTTP_STATUS_PERMANENT_REDIRECT: u32 = 308u32;
pub const HTTP_STATUS_PRECOND_FAILED: u32 = 412u32;
pub const HTTP_STATUS_PROXY_AUTH_REQ: u32 = 407u32;
pub const HTTP_STATUS_REDIRECT: u32 = 302u32;
pub const HTTP_STATUS_REDIRECT_KEEP_VERB: u32 = 307u32;
pub const HTTP_STATUS_REDIRECT_METHOD: u32 = 303u32;
pub const HTTP_STATUS_REQUEST_TIMEOUT: u32 = 408u32;
pub const HTTP_STATUS_REQUEST_TOO_LARGE: u32 = 413u32;
pub const HTTP_STATUS_RESET_CONTENT: u32 = 205u32;
pub const HTTP_STATUS_RETRY_WITH: u32 = 449u32;
pub const HTTP_STATUS_SERVER_ERROR: u32 = 500u32;
pub const HTTP_STATUS_SERVICE_UNAVAIL: u32 = 503u32;
pub const HTTP_STATUS_SWITCH_PROTOCOLS: u32 = 101u32;
pub const HTTP_STATUS_UNSUPPORTED_MEDIA: u32 = 415u32;
pub const HTTP_STATUS_URI_TOO_LONG: u32 = 414u32;
pub const HTTP_STATUS_USE_PROXY: u32 = 305u32;
pub const HTTP_STATUS_VERSION_NOT_SUP: u32 = 505u32;
pub const HTTP_STATUS_WEBDAV_MULTI_STATUS: u32 = 207u32;
pub const ICU_BROWSER_MODE: u32 = 33554432u32;
pub const ICU_DECODE: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(268435456u32);
pub const ICU_ENCODE_PERCENT: u32 = 4096u32;
pub const ICU_ENCODE_SPACES_ONLY: u32 = 67108864u32;
pub const ICU_ESCAPE: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(2147483648u32);
pub const ICU_ESCAPE_AUTHORITY: u32 = 8192u32;
pub const ICU_NO_ENCODE: u32 = 536870912u32;
pub const ICU_NO_META: u32 = 134217728u32;
pub const ICU_REJECT_USERPWD: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(16384u32);
pub const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443u16;
pub const INTERNET_DEFAULT_HTTP_PORT: u16 = 80u16;
pub const INTERNET_DEFAULT_PORT: u16 = 0u16;
pub const NETWORKING_KEY_BUFSIZE: u32 = 128u32;
pub const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096u32;
pub const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192u32;
pub const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: u32 = 512u32;
pub const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 256u32;
pub const SECURITY_FLAG_SECURE: u32 = 1u32;
pub const SECURITY_FLAG_STRENGTH_MEDIUM: u32 = 1073741824u32;
pub const SECURITY_FLAG_STRENGTH_STRONG: u32 = 536870912u32;
pub const SECURITY_FLAG_STRENGTH_WEAK: u32 = 268435456u32;
pub const SecureProtocol_ALL: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(168i32);
pub const SecureProtocol_SSL2: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(8i32);
pub const SecureProtocol_SSL3: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(32i32);
pub const SecureProtocol_TLS1: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(128i32);
pub const SecureProtocol_TLS1_1: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(512i32);
pub const SecureProtocol_TLS1_2: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(2048i32);
pub const SslErrorFlag_CertCNInvalid: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(4096i32);
pub const SslErrorFlag_CertDateInvalid: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(8192i32);
pub const SslErrorFlag_CertWrongUsage: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(512i32);
pub const SslErrorFlag_Ignore_All: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(13056i32);
pub const SslErrorFlag_UnknownCA: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(256i32);
pub const WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(4u32);
pub const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(0u32);
pub const WINHTTP_ACCESS_TYPE_NAMED_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(3u32);
pub const WINHTTP_ACCESS_TYPE_NO_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(1u32);
pub const WINHTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760u32;
pub const WINHTTP_ADDREQ_FLAG_ADD: u32 = 536870912u32;
pub const WINHTTP_ADDREQ_FLAG_ADD_IF_NEW: u32 = 268435456u32;
pub const WINHTTP_ADDREQ_FLAG_COALESCE: u32 = 1073741824u32;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: u32 = 1073741824u32;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: u32 = 16777216u32;
pub const WINHTTP_ADDREQ_FLAG_REPLACE: u32 = 2147483648u32;
pub const WINHTTP_ADDREQ_INDEX_MASK: u32 = 65535u32;
pub const WINHTTP_AUTH_SCHEME_BASIC: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(1u32);
pub const WINHTTP_AUTH_SCHEME_DIGEST: u32 = 8u32;
pub const WINHTTP_AUTH_SCHEME_NEGOTIATE: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(16u32);
pub const WINHTTP_AUTH_SCHEME_NTLM: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(2u32);
pub const WINHTTP_AUTH_SCHEME_PASSPORT: u32 = 4u32;
pub const WINHTTP_AUTH_TARGET_PROXY: u32 = 1u32;
pub const WINHTTP_AUTH_TARGET_SERVER: u32 = 0u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_DEFAULT: u32 = 0u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH: u32 = 2u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_LOW: u32 = 1u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MAX: u32 = 3u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MEDIUM: u32 = 0u32;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_PROXY_ONLY: u32 = 3u32;
pub const WINHTTP_AUTOPROXY_ALLOW_AUTOCONFIG: u32 = 256u32;
pub const WINHTTP_AUTOPROXY_ALLOW_CM: u32 = 1024u32;
pub const WINHTTP_AUTOPROXY_ALLOW_STATIC: u32 = 512u32;
pub const WINHTTP_AUTOPROXY_AUTO_DETECT: u32 = 1u32;
pub const WINHTTP_AUTOPROXY_CONFIG_URL: u32 = 2u32;
pub const WINHTTP_AUTOPROXY_HOST_KEEPCASE: u32 = 4u32;
pub const WINHTTP_AUTOPROXY_HOST_LOWERCASE: u32 = 8u32;
pub const WINHTTP_AUTOPROXY_NO_CACHE_CLIENT: u32 = 524288u32;
pub const WINHTTP_AUTOPROXY_NO_CACHE_SVC: u32 = 1048576u32;
pub const WINHTTP_AUTOPROXY_NO_DIRECTACCESS: u32 = 262144u32;
pub const WINHTTP_AUTOPROXY_RUN_INPROCESS: u32 = 65536u32;
pub const WINHTTP_AUTOPROXY_RUN_OUTPROCESS_ONLY: u32 = 131072u32;
pub const WINHTTP_AUTOPROXY_SORT_RESULTS: u32 = 4194304u32;
pub const WINHTTP_AUTOPROXY_USE_INTERFACE_CONFIG: u32 = 2048u32;
pub const WINHTTP_AUTO_DETECT_TYPE_DHCP: u32 = 1u32;
pub const WINHTTP_AUTO_DETECT_TYPE_DNS_A: u32 = 2u32;
pub const WINHTTP_CALLBACK_FLAG_ALL_NOTIFICATIONS: u32 = 4294967295u32;
pub const WINHTTP_CALLBACK_FLAG_DATA_AVAILABLE: u32 = 262144u32;
pub const WINHTTP_CALLBACK_FLAG_DETECTING_PROXY: u32 = 4096u32;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYFORURL_COMPLETE: u32 = 16777216u32;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYSETTINGS_COMPLETE: u32 = 134217728u32;
pub const WINHTTP_CALLBACK_FLAG_HEADERS_AVAILABLE: u32 = 131072u32;
pub const WINHTTP_CALLBACK_FLAG_INTERMEDIATE_RESPONSE: u32 = 32768u32;
pub const WINHTTP_CALLBACK_FLAG_READ_COMPLETE: u32 = 524288u32;
pub const WINHTTP_CALLBACK_FLAG_REDIRECT: u32 = 16384u32;
pub const WINHTTP_CALLBACK_FLAG_REQUEST_ERROR: u32 = 2097152u32;
pub const WINHTTP_CALLBACK_FLAG_SECURE_FAILURE: u32 = 65536u32;
pub const WINHTTP_CALLBACK_FLAG_SENDREQUEST_COMPLETE: u32 = 4194304u32;
pub const WINHTTP_CALLBACK_FLAG_WRITE_COMPLETE: u32 = 1048576u32;
pub const WINHTTP_CALLBACK_STATUS_CLOSE_COMPLETE: u32 = 33554432u32;
pub const WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION: u32 = 256u32;
pub const WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER: u32 = 8u32;
pub const WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER: u32 = 4u32;
pub const WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED: u32 = 512u32;
pub const WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE: u32 = 262144u32;
pub const WINHTTP_CALLBACK_STATUS_DETECTING_PROXY: u32 = 4096u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_CN_INVALID: u32 = 16u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_DATE_INVALID: u32 = 32u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REVOKED: u32 = 4u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REV_FAILED: u32 = 1u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_WRONG_USAGE: u32 = 64u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CA: u32 = 8u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CERT: u32 = 2u32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_SECURITY_CHANNEL_ERROR: u32 = 2147483648u32;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE: u32 = 16777216u32;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE: u32 = 134217728u32;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING: u32 = 2048u32;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CREATED: u32 = 1024u32;
pub const WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE: u32 = 131072u32;
pub const WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE: u32 = 32768u32;
pub const WINHTTP_CALLBACK_STATUS_NAME_RESOLVED: u32 = 2u32;
pub const WINHTTP_CALLBACK_STATUS_READ_COMPLETE: u32 = 524288u32;
pub const WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE: u32 = 64u32;
pub const WINHTTP_CALLBACK_STATUS_REDIRECT: u32 = 16384u32;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_ERROR: u32 = 2097152u32;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_SENT: u32 = 32u32;
pub const WINHTTP_CALLBACK_STATUS_RESOLVING_NAME: u32 = 1u32;
pub const WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED: u32 = 128u32;
pub const WINHTTP_CALLBACK_STATUS_SECURE_FAILURE: u32 = 65536u32;
pub const WINHTTP_CALLBACK_STATUS_SENDING_REQUEST: u32 = 16u32;
pub const WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE: u32 = 4194304u32;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_READ_COMPLETE: u32 = 536870912u32;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_WRITE_COMPLETE: u32 = 268435456u32;
pub const WINHTTP_CALLBACK_STATUS_SHUTDOWN_COMPLETE: u32 = 67108864u32;
pub const WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE: u32 = 1048576u32;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_408: u32 = 1u32;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE: u32 = 2u32;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION: u32 = 4u32;
pub const WINHTTP_CONNS_PER_SERVER_UNLIMITED: u32 = 4294967295u32;
pub const WINHTTP_DECOMPRESSION_FLAG_DEFLATE: u32 = 2u32;
pub const WINHTTP_DECOMPRESSION_FLAG_GZIP: u32 = 1u32;
pub const WINHTTP_DISABLE_AUTHENTICATION: u32 = 4u32;
pub const WINHTTP_DISABLE_COOKIES: u32 = 1u32;
pub const WINHTTP_DISABLE_KEEP_ALIVE: u32 = 8u32;
pub const WINHTTP_DISABLE_PASSPORT_AUTH: u32 = 0u32;
pub const WINHTTP_DISABLE_PASSPORT_KEYRING: u32 = 536870912u32;
pub const WINHTTP_DISABLE_REDIRECTS: u32 = 2u32;
pub const WINHTTP_DISABLE_SPN_SERVER_PORT: u32 = 0u32;
pub const WINHTTP_ENABLE_PASSPORT_AUTH: u32 = 268435456u32;
pub const WINHTTP_ENABLE_PASSPORT_KEYRING: u32 = 1073741824u32;
pub const WINHTTP_ENABLE_SPN_SERVER_PORT: u32 = 1u32;
pub const WINHTTP_ENABLE_SSL_REVERT_IMPERSONATION: u32 = 2u32;
pub const WINHTTP_ENABLE_SSL_REVOCATION: u32 = 1u32;
pub const WINHTTP_ERROR_BASE: u32 = 12000u32;
pub const WINHTTP_ERROR_LAST: u32 = 12192u32;
pub const WINHTTP_EXTENDED_HEADER_FLAG_UNICODE: u32 = 1u32;
pub const WINHTTP_FEATURE_ADD_REQUEST_HEADERS_EX: u32 = 46u32;
pub const WINHTTP_FEATURE_BACKGROUND_CONNECTIONS: u32 = 34u32;
pub const WINHTTP_FEATURE_CONNECTION_GUID: u32 = 38u32;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V0: u32 = 3u32;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V1: u32 = 12u32;
pub const WINHTTP_FEATURE_DISABLE_CERT_CHAIN_BUILDING: u32 = 33u32;
pub const WINHTTP_FEATURE_DISABLE_PROXY_AUTH_SCHEMES: u32 = 74u32;
pub const WINHTTP_FEATURE_DISABLE_SECURE_PROTOCOL_FALLBACK: u32 = 6u32;
pub const WINHTTP_FEATURE_DISABLE_STREAM_QUEUE: u32 = 1u32;
pub const WINHTTP_FEATURE_ENABLE_HTTP2_PLUS_CLIENT_CERT: u32 = 23u32;
pub const WINHTTP_FEATURE_EXPIRE_CONNECTION: u32 = 5u32;
pub const WINHTTP_FEATURE_EXTENDED_HEADER_FLAG_UNICODE: u32 = 54u32;
pub const WINHTTP_FEATURE_FAILED_CONNECTION_RETRIES: u32 = 24u32;
pub const WINHTTP_FEATURE_FIRST_AVAILABLE_CONNECTION: u32 = 35u32;
pub const WINHTTP_FEATURE_FLAG_AUTOMATIC_CHUNKING: u32 = 59u32;
pub const WINHTTP_FEATURE_FLAG_SECURE_DEFAULTS: u32 = 53u32;
pub const WINHTTP_FEATURE_FREE_QUERY_CONNECTION_GROUP_RESULT: u32 = 51u32;
pub const WINHTTP_FEATURE_HTTP2_KEEPALIVE: u32 = 26u32;
pub const WINHTTP_FEATURE_HTTP2_PLUS_TRANSFER_ENCODING: u32 = 31u32;
pub const WINHTTP_FEATURE_HTTP2_RECEIVE_WINDOW: u32 = 43u32;
pub const WINHTTP_FEATURE_HTTP3_HANDSHAKE_TIMEOUT: u32 = 70u32;
pub const WINHTTP_FEATURE_HTTP3_INITIAL_RTT: u32 = 71u32;
pub const WINHTTP_FEATURE_HTTP3_KEEPALIVE: u32 = 69u32;
pub const WINHTTP_FEATURE_HTTP3_STREAM_ERROR_CODE: u32 = 72u32;
pub const WINHTTP_FEATURE_HTTP_PROTOCOL_REQUIRED: u32 = 7u32;
pub const WINHTTP_FEATURE_IGNORE_CERT_REVOCATION_OFFLINE: u32 = 17u32;
pub const WINHTTP_FEATURE_IPV6_FAST_FALLBACK: u32 = 2u32;
pub const WINHTTP_FEATURE_IS_FEATURE_SUPPORTED: u32 = 44u32;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID: u32 = 39u32;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: u32 = 61u32;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP: u32 = 50u32;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP_FLAG_INSECURE: u32 = 60u32;
pub const WINHTTP_FEATURE_QUERY_EX_ALL_HEADERS: u32 = 62u32;
pub const WINHTTP_FEATURE_QUERY_FLAG_TRAILERS: u32 = 55u32;
pub const WINHTTP_FEATURE_QUERY_FLAG_WIRE_ENCODING: u32 = 56u32;
pub const WINHTTP_FEATURE_QUERY_HEADERS_EX: u32 = 49u32;
pub const WINHTTP_FEATURE_QUIC_STATS: u32 = 66u32;
pub const WINHTTP_FEATURE_READ_DATA_EX: u32 = 48u32;
pub const WINHTTP_FEATURE_READ_DATA_EX_FLAG_FILL_BUFFER: u32 = 63u32;
pub const WINHTTP_FEATURE_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 30u32;
pub const WINHTTP_FEATURE_REQUEST_ANNOTATION: u32 = 73u32;
pub const WINHTTP_FEATURE_REQUEST_STATS: u32 = 8u32;
pub const WINHTTP_FEATURE_REQUEST_TIMES: u32 = 4u32;
pub const WINHTTP_FEATURE_REQUIRE_STREAM_END: u32 = 22u32;
pub const WINHTTP_FEATURE_RESOLUTION_HOSTNAME: u32 = 27u32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG: u32 = 32u32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: u32 = 58u32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: u32 = 65u32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: u32 = 57u32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: u32 = 64u32;
pub const WINHTTP_FEATURE_REVERT_IMPERSONATION_SERVER_CERT: u32 = 75u32;
pub const WINHTTP_FEATURE_SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS: u32 = 52u32;
pub const WINHTTP_FEATURE_SECURITY_INFO: u32 = 13u32;
pub const WINHTTP_FEATURE_SERVER_CERT_CHAIN_CONTEXT: u32 = 9u32;
pub const WINHTTP_FEATURE_SET_PROXY_SETINGS_PER_USER: u32 = 47u32;
pub const WINHTTP_FEATURE_SET_TOKEN_BINDING: u32 = 28u32;
pub const WINHTTP_FEATURE_STREAM_ERROR_CODE: u32 = 21u32;
pub const WINHTTP_FEATURE_TCP_FAST_OPEN: u32 = 15u32;
pub const WINHTTP_FEATURE_TCP_KEEPALIVE: u32 = 14u32;
pub const WINHTTP_FEATURE_TCP_PRIORITY_STATUS: u32 = 37u32;
pub const WINHTTP_FEATURE_TLS_FALSE_START: u32 = 16u32;
pub const WINHTTP_FEATURE_TLS_PROTOCOL_INSECURE_FALLBACK: u32 = 20u32;
pub const WINHTTP_FEATURE_TOKEN_BINDING_PUBLIC_KEY: u32 = 29u32;
pub const WINHTTP_FLAG_ASYNC: u32 = 268435456u32;
pub const WINHTTP_FLAG_AUTOMATIC_CHUNKING: u32 = 512u32;
pub const WINHTTP_FLAG_BYPASS_PROXY_CACHE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(256u32);
pub const WINHTTP_FLAG_ESCAPE_DISABLE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(64u32);
pub const WINHTTP_FLAG_ESCAPE_DISABLE_QUERY: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(128u32);
pub const WINHTTP_FLAG_ESCAPE_PERCENT: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(4u32);
pub const WINHTTP_FLAG_NULL_CODEPAGE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(8u32);
pub const WINHTTP_FLAG_REFRESH: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(256u32);
pub const WINHTTP_FLAG_SECURE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(8388608u32);
pub const WINHTTP_FLAG_SECURE_DEFAULTS: u32 = 805306368u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL2: u32 = 8u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL3: u32 = 32u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1: u32 = 128u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_1: u32 = 512u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_2: u32 = 2048u32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_3: u32 = 8192u32;
pub const WINHTTP_HANDLE_TYPE_CONNECT: u32 = 2u32;
pub const WINHTTP_HANDLE_TYPE_PROXY_RESOLVER: u32 = 4u32;
pub const WINHTTP_HANDLE_TYPE_REQUEST: u32 = 3u32;
pub const WINHTTP_HANDLE_TYPE_SESSION: u32 = 1u32;
pub const WINHTTP_HANDLE_TYPE_WEBSOCKET: u32 = 5u32;
pub const WINHTTP_IGNORE_REQUEST_TOTAL_LENGTH: u32 = 0u32;
pub const WINHTTP_INTERNET_SCHEME_FTP: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(3i32);
pub const WINHTTP_INTERNET_SCHEME_HTTP: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(1i32);
pub const WINHTTP_INTERNET_SCHEME_HTTPS: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(2i32);
pub const WINHTTP_INTERNET_SCHEME_SOCKS: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(4i32);
pub const WINHTTP_LAST_OPTION: u32 = 196u32;
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAGS_MASK: u32 = 1u32;
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: u32 = 1u32;
pub const WINHTTP_OPTION_ASSURED_NON_BLOCKING_CALLBACKS: u32 = 111u32;
pub const WINHTTP_OPTION_AUTOLOGON_POLICY: u32 = 77u32;
pub const WINHTTP_OPTION_BACKGROUND_CONNECTIONS: u32 = 172u32;
pub const WINHTTP_OPTION_CALLBACK: u32 = 1u32;
pub const WINHTTP_OPTION_CLIENT_CERT_CONTEXT: u32 = 47u32;
pub const WINHTTP_OPTION_CLIENT_CERT_ISSUER_LIST: u32 = 94u32;
pub const WINHTTP_OPTION_CODEPAGE: u32 = 68u32;
pub const WINHTTP_OPTION_CONFIGURE_PASSPORT_AUTH: u32 = 83u32;
pub const WINHTTP_OPTION_CONNECTION_FILTER: u32 = 131u32;
pub const WINHTTP_OPTION_CONNECTION_GUID: u32 = 178u32;
pub const WINHTTP_OPTION_CONNECTION_INFO: u32 = 93u32;
pub const WINHTTP_OPTION_CONNECTION_STATS_V0: u32 = 141u32;
pub const WINHTTP_OPTION_CONNECTION_STATS_V1: u32 = 150u32;
pub const WINHTTP_OPTION_CONNECT_RETRIES: u32 = 4u32;
pub const WINHTTP_OPTION_CONNECT_TIMEOUT: u32 = 3u32;
pub const WINHTTP_OPTION_CONTEXT_VALUE: u32 = 45u32;
pub const WINHTTP_OPTION_DECOMPRESSION: u32 = 118u32;
pub const WINHTTP_OPTION_DISABLE_CERT_CHAIN_BUILDING: u32 = 171u32;
pub const WINHTTP_OPTION_DISABLE_FEATURE: u32 = 63u32;
pub const WINHTTP_OPTION_DISABLE_GLOBAL_POOLING: u32 = 195u32;
pub const WINHTTP_OPTION_DISABLE_PROXY_AUTH_SCHEMES: u32 = 193u32;
pub const WINHTTP_OPTION_DISABLE_SECURE_PROTOCOL_FALLBACK: u32 = 144u32;
pub const WINHTTP_OPTION_DISABLE_STREAM_QUEUE: u32 = 139u32;
pub const WINHTTP_OPTION_ENABLETRACING: u32 = 85u32;
pub const WINHTTP_OPTION_ENABLE_FEATURE: u32 = 79u32;
pub const WINHTTP_OPTION_ENABLE_HTTP2_PLUS_CLIENT_CERT: u32 = 161u32;
pub const WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 133u32;
pub const WINHTTP_OPTION_ENCODE_EXTRA: u32 = 138u32;
pub const WINHTTP_OPTION_EXPIRE_CONNECTION: u32 = 143u32;
pub const WINHTTP_OPTION_EXTENDED_ERROR: u32 = 24u32;
pub const WINHTTP_OPTION_FAILED_CONNECTION_RETRIES: u32 = 162u32;
pub const WINHTTP_OPTION_FEATURE_SUPPORTED: u32 = 184u32;
pub const WINHTTP_OPTION_FIRST_AVAILABLE_CONNECTION: u32 = 173u32;
pub const WINHTTP_OPTION_GLOBAL_PROXY_CREDS: u32 = 97u32;
pub const WINHTTP_OPTION_GLOBAL_SERVER_CREDS: u32 = 98u32;
pub const WINHTTP_OPTION_HANDLE_TYPE: u32 = 9u32;
pub const WINHTTP_OPTION_HTTP2_KEEPALIVE: u32 = 164u32;
pub const WINHTTP_OPTION_HTTP2_PLUS_TRANSFER_ENCODING: u32 = 169u32;
pub const WINHTTP_OPTION_HTTP2_RECEIVE_WINDOW: u32 = 183u32;
pub const WINHTTP_OPTION_HTTP3_HANDSHAKE_TIMEOUT: u32 = 189u32;
pub const WINHTTP_OPTION_HTTP3_INITIAL_RTT: u32 = 190u32;
pub const WINHTTP_OPTION_HTTP3_KEEPALIVE: u32 = 188u32;
pub const WINHTTP_OPTION_HTTP3_STREAM_ERROR_CODE: u32 = 191u32;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_REQUIRED: u32 = 145u32;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_USED: u32 = 134u32;
pub const WINHTTP_OPTION_HTTP_VERSION: u32 = 59u32;
pub const WINHTTP_OPTION_IGNORE_CERT_REVOCATION_OFFLINE: u32 = 155u32;
pub const WINHTTP_OPTION_IPV6_FAST_FALLBACK: u32 = 140u32;
pub const WINHTTP_OPTION_IS_PROXY_CONNECT_RESPONSE: u32 = 104u32;
pub const WINHTTP_OPTION_KDC_PROXY_SETTINGS: u32 = 136u32;
pub const WINHTTP_OPTION_MATCH_CONNECTION_GUID: u32 = 179u32;
pub const WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74u32;
pub const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: u32 = 73u32;
pub const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: u32 = 89u32;
pub const WINHTTP_OPTION_MAX_HTTP_STATUS_CONTINUE: u32 = 90u32;
pub const WINHTTP_OPTION_MAX_RESPONSE_DRAIN_SIZE: u32 = 92u32;
pub const WINHTTP_OPTION_MAX_RESPONSE_HEADER_SIZE: u32 = 91u32;
pub const WINHTTP_OPTION_NETWORK_INTERFACE_AFFINITY: u32 = 105u32;
pub const WINHTTP_OPTION_PARENT_HANDLE: u32 = 21u32;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_TEXT: u32 = 81u32;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_URL: u32 = 82u32;
pub const WINHTTP_OPTION_PASSPORT_RETURN_URL: u32 = 87u32;
pub const WINHTTP_OPTION_PASSPORT_SIGN_OUT: u32 = 86u32;
pub const WINHTTP_OPTION_PASSWORD: u32 = 4097u32;
pub const WINHTTP_OPTION_PROXY: u32 = 38u32;
pub const WINHTTP_OPTION_PROXY_DISABLE_SERVICE_CALLS: u32 = 137u32;
pub const WINHTTP_OPTION_PROXY_PASSWORD: u32 = 4099u32;
pub const WINHTTP_OPTION_PROXY_RESULT_ENTRY: u32 = 39u32;
pub const WINHTTP_OPTION_PROXY_SPN_USED: u32 = 107u32;
pub const WINHTTP_OPTION_PROXY_USERNAME: u32 = 4098u32;
pub const WINHTTP_OPTION_QUIC_STATS: u32 = 185u32;
pub const WINHTTP_OPTION_READ_BUFFER_SIZE: u32 = 12u32;
pub const WINHTTP_OPTION_RECEIVE_PROXY_CONNECT_RESPONSE: u32 = 103u32;
pub const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: u32 = 7u32;
pub const WINHTTP_OPTION_RECEIVE_TIMEOUT: u32 = 6u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY: u32 = 88u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS: u32 = 2u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DEFAULT: u32 = 1u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP: u32 = 1u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY_LAST: u32 = 2u32;
pub const WINHTTP_OPTION_REDIRECT_POLICY_NEVER: u32 = 0u32;
pub const WINHTTP_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 168u32;
pub const WINHTTP_OPTION_REJECT_USERPWD_IN_URL: u32 = 100u32;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION: u32 = 192u32;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: u32 = 64000u32;
pub const WINHTTP_OPTION_REQUEST_PRIORITY: u32 = 58u32;
pub const WINHTTP_OPTION_REQUEST_STATS: u32 = 146u32;
pub const WINHTTP_OPTION_REQUEST_TIMES: u32 = 142u32;
pub const WINHTTP_OPTION_REQUIRE_STREAM_END: u32 = 160u32;
pub const WINHTTP_OPTION_RESOLUTION_HOSTNAME: u32 = 165u32;
pub const WINHTTP_OPTION_RESOLVER_CACHE_CONFIG: u32 = 170u32;
pub const WINHTTP_OPTION_RESOLVE_TIMEOUT: u32 = 2u32;
pub const WINHTTP_OPTION_REVERT_IMPERSONATION_SERVER_CERT: u32 = 194u32;
pub const WINHTTP_OPTION_SECURE_PROTOCOLS: u32 = 84u32;
pub const WINHTTP_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32u32;
pub const WINHTTP_OPTION_SECURITY_FLAGS: u32 = 31u32;
pub const WINHTTP_OPTION_SECURITY_INFO: u32 = 151u32;
pub const WINHTTP_OPTION_SECURITY_KEY_BITNESS: u32 = 36u32;
pub const WINHTTP_OPTION_SEND_TIMEOUT: u32 = 5u32;
pub const WINHTTP_OPTION_SERVER_CBT: u32 = 108u32;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 147u32;
pub const WINHTTP_OPTION_SERVER_CERT_CONTEXT: u32 = 78u32;
pub const WINHTTP_OPTION_SERVER_SPN_USED: u32 = 106u32;
pub const WINHTTP_OPTION_SET_TOKEN_BINDING: u32 = 166u32;
pub const WINHTTP_OPTION_SPN: u32 = 96u32;
pub const WINHTTP_OPTION_SPN_MASK: u32 = 1u32;
pub const WINHTTP_OPTION_STREAM_ERROR_CODE: u32 = 159u32;
pub const WINHTTP_OPTION_TCP_FAST_OPEN: u32 = 153u32;
pub const WINHTTP_OPTION_TCP_KEEPALIVE: u32 = 152u32;
pub const WINHTTP_OPTION_TCP_PRIORITY_HINT: u32 = 128u32;
pub const WINHTTP_OPTION_TCP_PRIORITY_STATUS: u32 = 177u32;
pub const WINHTTP_OPTION_TLS_FALSE_START: u32 = 154u32;
pub const WINHTTP_OPTION_TLS_PROTOCOL_INSECURE_FALLBACK: u32 = 158u32;
pub const WINHTTP_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 167u32;
pub const WINHTTP_OPTION_UNLOAD_NOTIFY_EVENT: u32 = 99u32;
pub const WINHTTP_OPTION_UNSAFE_HEADER_PARSING: u32 = 110u32;
pub const WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET: u32 = 114u32;
pub const WINHTTP_OPTION_URL: u32 = 34u32;
pub const WINHTTP_OPTION_USERNAME: u32 = 4096u32;
pub const WINHTTP_OPTION_USER_AGENT: u32 = 41u32;
pub const WINHTTP_OPTION_USE_GLOBAL_SERVER_CREDENTIALS: u32 = 101u32;
pub const WINHTTP_OPTION_USE_SESSION_SCH_CRED: u32 = 196u32;
pub const WINHTTP_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: u32 = 115u32;
pub const WINHTTP_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: u32 = 116u32;
pub const WINHTTP_OPTION_WEB_SOCKET_RECEIVE_BUFFER_SIZE: u32 = 122u32;
pub const WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE: u32 = 123u32;
pub const WINHTTP_OPTION_WORKER_THREAD_COUNT: u32 = 80u32;
pub const WINHTTP_OPTION_WRITE_BUFFER_SIZE: u32 = 13u32;
pub const WINHTTP_PROTOCOL_FLAG_HTTP2: u32 = 1u32;
pub const WINHTTP_PROTOCOL_FLAG_HTTP3: u32 = 2u32;
pub const WINHTTP_PROXY_DISABLE_AUTH_LOCAL_SERVICE: u32 = 256u32;
pub const WINHTTP_PROXY_DISABLE_SCHEME_BASIC: u32 = 1u32;
pub const WINHTTP_PROXY_DISABLE_SCHEME_DIGEST: u32 = 2u32;
pub const WINHTTP_PROXY_DISABLE_SCHEME_KERBEROS: u32 = 8u32;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NEGOTIATE: u32 = 16u32;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NTLM: u32 = 4u32;
pub const WINHTTP_PROXY_NOTIFY_CHANGE: u32 = 1u32;
pub const WINHTTP_PROXY_TYPE_AUTO_DETECT: u32 = 8u32;
pub const WINHTTP_PROXY_TYPE_AUTO_PROXY_URL: u32 = 4u32;
pub const WINHTTP_PROXY_TYPE_DIRECT: u32 = 1u32;
pub const WINHTTP_PROXY_TYPE_PROXY: u32 = 2u32;
pub const WINHTTP_QUERY_ACCEPT: u32 = 24u32;
pub const WINHTTP_QUERY_ACCEPT_CHARSET: u32 = 25u32;
pub const WINHTTP_QUERY_ACCEPT_ENCODING: u32 = 26u32;
pub const WINHTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27u32;
pub const WINHTTP_QUERY_ACCEPT_RANGES: u32 = 42u32;
pub const WINHTTP_QUERY_AGE: u32 = 48u32;
pub const WINHTTP_QUERY_ALLOW: u32 = 7u32;
pub const WINHTTP_QUERY_AUTHENTICATION_INFO: u32 = 76u32;
pub const WINHTTP_QUERY_AUTHORIZATION: u32 = 28u32;
pub const WINHTTP_QUERY_CACHE_CONTROL: u32 = 49u32;
pub const WINHTTP_QUERY_CONNECTION: u32 = 23u32;
pub const WINHTTP_QUERY_CONTENT_BASE: u32 = 50u32;
pub const WINHTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4u32;
pub const WINHTTP_QUERY_CONTENT_DISPOSITION: u32 = 47u32;
pub const WINHTTP_QUERY_CONTENT_ENCODING: u32 = 29u32;
pub const WINHTTP_QUERY_CONTENT_ID: u32 = 3u32;
pub const WINHTTP_QUERY_CONTENT_LANGUAGE: u32 = 6u32;
pub const WINHTTP_QUERY_CONTENT_LENGTH: u32 = 5u32;
pub const WINHTTP_QUERY_CONTENT_LOCATION: u32 = 51u32;
pub const WINHTTP_QUERY_CONTENT_MD5: u32 = 52u32;
pub const WINHTTP_QUERY_CONTENT_RANGE: u32 = 53u32;
pub const WINHTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2u32;
pub const WINHTTP_QUERY_CONTENT_TYPE: u32 = 1u32;
pub const WINHTTP_QUERY_COOKIE: u32 = 44u32;
pub const WINHTTP_QUERY_COST: u32 = 15u32;
pub const WINHTTP_QUERY_CUSTOM: u32 = 65535u32;
pub const WINHTTP_QUERY_DATE: u32 = 9u32;
pub const WINHTTP_QUERY_DERIVED_FROM: u32 = 14u32;
pub const WINHTTP_QUERY_ETAG: u32 = 54u32;
pub const WINHTTP_QUERY_EXPECT: u32 = 68u32;
pub const WINHTTP_QUERY_EXPIRES: u32 = 10u32;
pub const WINHTTP_QUERY_EX_ALL_HEADERS: u32 = 21u32;
pub const WINHTTP_QUERY_FLAG_NUMBER: u32 = 536870912u32;
pub const WINHTTP_QUERY_FLAG_NUMBER64: u32 = 134217728u32;
pub const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648u32;
pub const WINHTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824u32;
pub const WINHTTP_QUERY_FLAG_TRAILERS: u32 = 33554432u32;
pub const WINHTTP_QUERY_FLAG_WIRE_ENCODING: u32 = 16777216u32;
pub const WINHTTP_QUERY_FORWARDED: u32 = 30u32;
pub const WINHTTP_QUERY_FROM: u32 = 31u32;
pub const WINHTTP_QUERY_HOST: u32 = 55u32;
pub const WINHTTP_QUERY_IF_MATCH: u32 = 56u32;
pub const WINHTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32u32;
pub const WINHTTP_QUERY_IF_NONE_MATCH: u32 = 57u32;
pub const WINHTTP_QUERY_IF_RANGE: u32 = 58u32;
pub const WINHTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59u32;
pub const WINHTTP_QUERY_LAST_MODIFIED: u32 = 11u32;
pub const WINHTTP_QUERY_LINK: u32 = 16u32;
pub const WINHTTP_QUERY_LOCATION: u32 = 33u32;
pub const WINHTTP_QUERY_MAX: u32 = 78u32;
pub const WINHTTP_QUERY_MAX_FORWARDS: u32 = 60u32;
pub const WINHTTP_QUERY_MESSAGE_ID: u32 = 12u32;
pub const WINHTTP_QUERY_MIME_VERSION: u32 = 0u32;
pub const WINHTTP_QUERY_ORIG_URI: u32 = 34u32;
pub const WINHTTP_QUERY_PASSPORT_CONFIG: u32 = 78u32;
pub const WINHTTP_QUERY_PASSPORT_URLS: u32 = 77u32;
pub const WINHTTP_QUERY_PRAGMA: u32 = 17u32;
pub const WINHTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41u32;
pub const WINHTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61u32;
pub const WINHTTP_QUERY_PROXY_CONNECTION: u32 = 69u32;
pub const WINHTTP_QUERY_PROXY_SUPPORT: u32 = 75u32;
pub const WINHTTP_QUERY_PUBLIC: u32 = 8u32;
pub const WINHTTP_QUERY_RANGE: u32 = 62u32;
pub const WINHTTP_QUERY_RAW_HEADERS: u32 = 21u32;
pub const WINHTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22u32;
pub const WINHTTP_QUERY_REFERER: u32 = 35u32;
pub const WINHTTP_QUERY_REFRESH: u32 = 46u32;
pub const WINHTTP_QUERY_REQUEST_METHOD: u32 = 45u32;
pub const WINHTTP_QUERY_RETRY_AFTER: u32 = 36u32;
pub const WINHTTP_QUERY_SERVER: u32 = 37u32;
pub const WINHTTP_QUERY_SET_COOKIE: u32 = 43u32;
pub const WINHTTP_QUERY_STATUS_CODE: u32 = 19u32;
pub const WINHTTP_QUERY_STATUS_TEXT: u32 = 20u32;
pub const WINHTTP_QUERY_TITLE: u32 = 38u32;
pub const WINHTTP_QUERY_TRANSFER_ENCODING: u32 = 63u32;
pub const WINHTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70u32;
pub const WINHTTP_QUERY_UPGRADE: u32 = 64u32;
pub const WINHTTP_QUERY_URI: u32 = 13u32;
pub const WINHTTP_QUERY_USER_AGENT: u32 = 39u32;
pub const WINHTTP_QUERY_VARY: u32 = 65u32;
pub const WINHTTP_QUERY_VERSION: u32 = 18u32;
pub const WINHTTP_QUERY_VIA: u32 = 66u32;
pub const WINHTTP_QUERY_WARNING: u32 = 67u32;
pub const WINHTTP_QUERY_WWW_AUTHENTICATE: u32 = 40u32;
pub const WINHTTP_REQUEST_STAT_FLAG_FIRST_REQUEST: u32 = 32u32;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_FALSE_START: u32 = 16u32;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_SESSION_RESUMPTION: u32 = 8u32;
pub const WINHTTP_REQUEST_STAT_FLAG_TCP_FAST_OPEN: u32 = 1u32;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_FALSE_START: u32 = 4u32;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
pub const WINHTTP_RESET_ALL: u32 = 65535u32;
pub const WINHTTP_RESET_DISCARD_RESOLVERS: u32 = 262144u32;
pub const WINHTTP_RESET_NOTIFY_NETWORK_CHANGED: u32 = 65536u32;
pub const WINHTTP_RESET_OUT_OF_PROC: u32 = 131072u32;
pub const WINHTTP_RESET_SCRIPT_CACHE: u32 = 8u32;
pub const WINHTTP_RESET_STATE: u32 = 1u32;
pub const WINHTTP_RESET_SWPAD_ALL: u32 = 4u32;
pub const WINHTTP_RESET_SWPAD_CURRENT_NETWORK: u32 = 2u32;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: u32 = 2u32;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: u32 = 8u32;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: u32 = 1u32;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: u32 = 4u32;
pub const WINHTTP_TIME_FORMAT_BUFSIZE: u32 = 62u32;
pub const WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1006i32);
pub const WINHTTP_WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(1i32);
pub const WINHTTP_WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(0i32);
pub const WINHTTP_WEB_SOCKET_CLOSE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(4i32);
pub const WINHTTP_WEB_SOCKET_CLOSE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(2i32);
pub const WINHTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const WINHTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1001i32);
pub const WINHTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const WINHTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
pub const WINHTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const WINHTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: u32 = 15000u32;
pub const WINHTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1008i32);
pub const WINHTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const WINHTTP_WEB_SOCKET_RECEIVE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(1i32);
pub const WINHTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1015i32);
pub const WINHTTP_WEB_SOCKET_SEND_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(0i32);
pub const WINHTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const WINHTTP_WEB_SOCKET_SHUTDOWN_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(3i32);
pub const WINHTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const WINHTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const WINHTTP_WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(3i32);
pub const WINHTTP_WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(2i32);
pub const WinHttpConnectFailureCount: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(0i32);
pub const WinHttpConnectionAcquireEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(4i32);
pub const WinHttpConnectionAcquireStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(2i32);
pub const WinHttpConnectionAcquireWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(3i32);
pub const WinHttpConnectionEstablishmentEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(8i32);
pub const WinHttpConnectionEstablishmentStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(7i32);
pub const WinHttpNameResolutionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(6i32);
pub const WinHttpNameResolutionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(5i32);
pub const WinHttpProxyDetectionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(1i32);
pub const WinHttpProxyDetectionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(0i32);
pub const WinHttpProxyFailureCount: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(1i32);
pub const WinHttpProxySettingsTypeUnknown: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(0i32);
pub const WinHttpProxySettingsTypeWsa: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(2i32);
pub const WinHttpProxySettingsTypeWsl: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(1i32);
pub const WinHttpProxyTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(31i32);
pub const WinHttpProxyTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(12i32);
pub const WinHttpProxyTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(30i32);
pub const WinHttpProxyTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(33i32);
pub const WinHttpProxyTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(14i32);
pub const WinHttpProxyTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(32i32);
pub const WinHttpProxyTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(35i32);
pub const WinHttpProxyTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(34i32);
pub const WinHttpProxyTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(13i32);
pub const WinHttpProxyTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(15i32);
pub const WinHttpProxyTunnelEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(29i32);
pub const WinHttpProxyTunnelStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(28i32);
pub const WinHttpReceiveResponseBodyDecompressionDelta: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(26i32);
pub const WinHttpReceiveResponseEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(27i32);
pub const WinHttpReceiveResponseHeadersDecompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(24i32);
pub const WinHttpReceiveResponseHeadersDecompressionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(23i32);
pub const WinHttpReceiveResponseHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(25i32);
pub const WinHttpReceiveResponseStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(22i32);
pub const WinHttpRequest: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2087c2f4_2cef_4953_a8ab_66779b670495);
pub const WinHttpRequestHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(7i32);
pub const WinHttpRequestHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(6i32);
pub const WinHttpRequestOption_EnableCertificateRevocationCheck: WinHttpRequestOption = WinHttpRequestOption(18i32);
pub const WinHttpRequestOption_EnableHttp1_1: WinHttpRequestOption = WinHttpRequestOption(17i32);
pub const WinHttpRequestOption_EnableHttpsToHttpRedirects: WinHttpRequestOption = WinHttpRequestOption(12i32);
pub const WinHttpRequestOption_EnablePassportAuthentication: WinHttpRequestOption = WinHttpRequestOption(13i32);
pub const WinHttpRequestOption_EnableRedirects: WinHttpRequestOption = WinHttpRequestOption(6i32);
pub const WinHttpRequestOption_EnableTracing: WinHttpRequestOption = WinHttpRequestOption(10i32);
pub const WinHttpRequestOption_EscapePercentInURL: WinHttpRequestOption = WinHttpRequestOption(3i32);
pub const WinHttpRequestOption_MaxAutomaticRedirects: WinHttpRequestOption = WinHttpRequestOption(14i32);
pub const WinHttpRequestOption_MaxResponseDrainSize: WinHttpRequestOption = WinHttpRequestOption(16i32);
pub const WinHttpRequestOption_MaxResponseHeaderSize: WinHttpRequestOption = WinHttpRequestOption(15i32);
pub const WinHttpRequestOption_RejectUserpwd: WinHttpRequestOption = WinHttpRequestOption(19i32);
pub const WinHttpRequestOption_RevertImpersonationOverSsl: WinHttpRequestOption = WinHttpRequestOption(11i32);
pub const WinHttpRequestOption_SecureProtocols: WinHttpRequestOption = WinHttpRequestOption(9i32);
pub const WinHttpRequestOption_SelectCertificate: WinHttpRequestOption = WinHttpRequestOption(5i32);
pub const WinHttpRequestOption_SslErrorIgnoreFlags: WinHttpRequestOption = WinHttpRequestOption(4i32);
pub const WinHttpRequestOption_URL: WinHttpRequestOption = WinHttpRequestOption(1i32);
pub const WinHttpRequestOption_URLCodePage: WinHttpRequestOption = WinHttpRequestOption(2i32);
pub const WinHttpRequestOption_UrlEscapeDisable: WinHttpRequestOption = WinHttpRequestOption(7i32);
pub const WinHttpRequestOption_UrlEscapeDisableQuery: WinHttpRequestOption = WinHttpRequestOption(8i32);
pub const WinHttpRequestOption_UserAgentString: WinHttpRequestOption = WinHttpRequestOption(0i32);
pub const WinHttpRequestStatLast: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(16i32);
pub const WinHttpRequestStatMax: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(32i32);
pub const WinHttpRequestTimeLast: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(36i32);
pub const WinHttpRequestTimeMax: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(64i32);
pub const WinHttpResponseBodyCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(11i32);
pub const WinHttpResponseBodySize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(10i32);
pub const WinHttpResponseHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(9i32);
pub const WinHttpResponseHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(8i32);
pub const WinHttpSecureDnsSettingDefault: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(0i32);
pub const WinHttpSecureDnsSettingForcePlaintext: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(1i32);
pub const WinHttpSecureDnsSettingMax: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(4i32);
pub const WinHttpSecureDnsSettingRequireEncryption: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(2i32);
pub const WinHttpSecureDnsSettingTryEncryptionWithFallback: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(3i32);
pub const WinHttpSendRequestEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(21i32);
pub const WinHttpSendRequestHeadersCompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(19i32);
pub const WinHttpSendRequestHeadersCompressionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(18i32);
pub const WinHttpSendRequestHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(20i32);
pub const WinHttpSendRequestStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(17i32);
pub const WinHttpStreamWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(16i32);
pub const WinHttpStreamWaitStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(15i32);
pub const WinHttpTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(10i32);
pub const WinHttpTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(2i32);
pub const WinHttpTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(9i32);
pub const WinHttpTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(12i32);
pub const WinHttpTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(4i32);
pub const WinHttpTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(11i32);
pub const WinHttpTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(14i32);
pub const WinHttpTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(13i32);
pub const WinHttpTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(3i32);
pub const WinHttpTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(5i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_ACCESS_TYPE(pub u32);
impl ::core::marker::Copy for WINHTTP_ACCESS_TYPE {}
impl ::core::clone::Clone for WINHTTP_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_ACCESS_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_CREDS_AUTHSCHEME(pub u32);
impl ::core::marker::Copy for WINHTTP_CREDS_AUTHSCHEME {}
impl ::core::clone::Clone for WINHTTP_CREDS_AUTHSCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_CREDS_AUTHSCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_CREDS_AUTHSCHEME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_CREDS_AUTHSCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_CREDS_AUTHSCHEME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_INTERNET_SCHEME(pub i32);
impl ::core::marker::Copy for WINHTTP_INTERNET_SCHEME {}
impl ::core::clone::Clone for WINHTTP_INTERNET_SCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_INTERNET_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_INTERNET_SCHEME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_INTERNET_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_INTERNET_SCHEME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_OPEN_REQUEST_FLAGS(pub u32);
impl ::core::marker::Copy for WINHTTP_OPEN_REQUEST_FLAGS {}
impl ::core::clone::Clone for WINHTTP_OPEN_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_OPEN_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_OPEN_REQUEST_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_OPEN_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_OPEN_REQUEST_FLAGS").field(&self.0).finish()
    }
}
impl WINHTTP_OPEN_REQUEST_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_PROXY_SETTINGS_TYPE(pub i32);
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS_TYPE {}
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_PROXY_SETTINGS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_PROXY_SETTINGS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_REQUEST_STAT_ENTRY(pub i32);
impl ::core::marker::Copy for WINHTTP_REQUEST_STAT_ENTRY {}
impl ::core::clone::Clone for WINHTTP_REQUEST_STAT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_REQUEST_STAT_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_REQUEST_STAT_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_REQUEST_STAT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_REQUEST_STAT_ENTRY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_REQUEST_TIME_ENTRY(pub i32);
impl ::core::marker::Copy for WINHTTP_REQUEST_TIME_ENTRY {}
impl ::core::clone::Clone for WINHTTP_REQUEST_TIME_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_REQUEST_TIME_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_REQUEST_TIME_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_REQUEST_TIME_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_REQUEST_TIME_ENTRY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_SECURE_DNS_SETTING(pub i32);
impl ::core::marker::Copy for WINHTTP_SECURE_DNS_SETTING {}
impl ::core::clone::Clone for WINHTTP_SECURE_DNS_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_SECURE_DNS_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_SECURE_DNS_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_SECURE_DNS_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_SECURE_DNS_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_WEB_SOCKET_BUFFER_TYPE(pub i32);
impl ::core::marker::Copy for WINHTTP_WEB_SOCKET_BUFFER_TYPE {}
impl ::core::clone::Clone for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_WEB_SOCKET_CLOSE_STATUS(pub i32);
impl ::core::marker::Copy for WINHTTP_WEB_SOCKET_CLOSE_STATUS {}
impl ::core::clone::Clone for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINHTTP_WEB_SOCKET_OPERATION(pub i32);
impl ::core::marker::Copy for WINHTTP_WEB_SOCKET_OPERATION {}
impl ::core::clone::Clone for WINHTTP_WEB_SOCKET_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINHTTP_WEB_SOCKET_OPERATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIN_HTTP_CREATE_URL_FLAGS(pub u32);
impl ::core::marker::Copy for WIN_HTTP_CREATE_URL_FLAGS {}
impl ::core::clone::Clone for WIN_HTTP_CREATE_URL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN_HTTP_CREATE_URL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WIN_HTTP_CREATE_URL_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WIN_HTTP_CREATE_URL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN_HTTP_CREATE_URL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WinHttpRequestAutoLogonPolicy(pub i32);
impl ::core::marker::Copy for WinHttpRequestAutoLogonPolicy {}
impl ::core::clone::Clone for WinHttpRequestAutoLogonPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WinHttpRequestAutoLogonPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WinHttpRequestAutoLogonPolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WinHttpRequestAutoLogonPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WinHttpRequestAutoLogonPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WinHttpRequestOption(pub i32);
impl ::core::marker::Copy for WinHttpRequestOption {}
impl ::core::clone::Clone for WinHttpRequestOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WinHttpRequestOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WinHttpRequestOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WinHttpRequestOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WinHttpRequestOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WinHttpRequestSecureProtocols(pub i32);
impl ::core::marker::Copy for WinHttpRequestSecureProtocols {}
impl ::core::clone::Clone for WinHttpRequestSecureProtocols {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WinHttpRequestSecureProtocols {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WinHttpRequestSecureProtocols {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WinHttpRequestSecureProtocols {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WinHttpRequestSecureProtocols").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WinHttpRequestSslErrorFlags(pub i32);
impl ::core::marker::Copy for WinHttpRequestSslErrorFlags {}
impl ::core::clone::Clone for WinHttpRequestSslErrorFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WinHttpRequestSslErrorFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WinHttpRequestSslErrorFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WinHttpRequestSslErrorFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WinHttpRequestSslErrorFlags").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct HTTP_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
impl ::core::marker::Copy for HTTP_VERSION_INFO {}
impl ::core::clone::Clone for HTTP_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_VERSION_INFO").field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::windows_core::TypeKind for HTTP_VERSION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
impl ::core::cmp::Eq for HTTP_VERSION_INFO {}
impl ::core::default::Default for HTTP_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct URL_COMPONENTS {
    pub dwStructSize: u32,
    pub lpszScheme: ::windows_core::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: WINHTTP_INTERNET_SCHEME,
    pub lpszHostName: ::windows_core::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: ::windows_core::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: ::windows_core::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: ::windows_core::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: ::windows_core::PWSTR,
    pub dwExtraInfoLength: u32,
}
impl ::core::marker::Copy for URL_COMPONENTS {}
impl ::core::clone::Clone for URL_COMPONENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for URL_COMPONENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTS")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
impl ::windows_core::TypeKind for URL_COMPONENTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for URL_COMPONENTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTS {}
impl ::core::default::Default for URL_COMPONENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
impl ::core::marker::Copy for WINHTTP_ASYNC_RESULT {}
impl ::core::clone::Clone for WINHTTP_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_ASYNC_RESULT").field("dwResult", &self.dwResult).field("dwError", &self.dwError).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_ASYNC_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwResult == other.dwResult && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for WINHTTP_ASYNC_RESULT {}
impl ::core::default::Default for WINHTTP_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dwFlags: u32,
    pub dwAutoDetectFlags: u32,
    pub lpszAutoConfigUrl: ::windows_core::PCWSTR,
    pub lpvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub fAutoLogonIfChallenged: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_AUTOPROXY_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_AUTOPROXY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_AUTOPROXY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_AUTOPROXY_OPTIONS").field("dwFlags", &self.dwFlags).field("dwAutoDetectFlags", &self.dwAutoDetectFlags).field("lpszAutoConfigUrl", &self.lpszAutoConfigUrl).field("lpvReserved", &self.lpvReserved).field("dwReserved", &self.dwReserved).field("fAutoLogonIfChallenged", &self.fAutoLogonIfChallenged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_AUTOPROXY_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_AUTOPROXY_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwAutoDetectFlags == other.dwAutoDetectFlags && self.lpszAutoConfigUrl == other.lpszAutoConfigUrl && self.lpvReserved == other.lpvReserved && self.dwReserved == other.dwReserved && self.fAutoLogonIfChallenged == other.fAutoLogonIfChallenged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_AUTOPROXY_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_AUTOPROXY_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_CERTIFICATE_INFO {
    pub ftExpiry: super::super::Foundation::FILETIME,
    pub ftStart: super::super::Foundation::FILETIME,
    pub lpszSubjectInfo: ::windows_core::PWSTR,
    pub lpszIssuerInfo: ::windows_core::PWSTR,
    pub lpszProtocolName: ::windows_core::PWSTR,
    pub lpszSignatureAlgName: ::windows_core::PWSTR,
    pub lpszEncryptionAlgName: ::windows_core::PWSTR,
    pub dwKeySize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_CERTIFICATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_CERTIFICATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CERTIFICATE_INFO").field("ftExpiry", &self.ftExpiry).field("ftStart", &self.ftStart).field("lpszSubjectInfo", &self.lpszSubjectInfo).field("lpszIssuerInfo", &self.lpszIssuerInfo).field("lpszProtocolName", &self.lpszProtocolName).field("lpszSignatureAlgName", &self.lpszSignatureAlgName).field("lpszEncryptionAlgName", &self.lpszEncryptionAlgName).field("dwKeySize", &self.dwKeySize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_CERTIFICATE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpiry == other.ftExpiry && self.ftStart == other.ftStart && self.lpszSubjectInfo == other.lpszSubjectInfo && self.lpszIssuerInfo == other.lpszIssuerInfo && self.lpszProtocolName == other.lpszProtocolName && self.lpszSignatureAlgName == other.lpszSignatureAlgName && self.lpszEncryptionAlgName == other.lpszEncryptionAlgName && self.dwKeySize == other.dwKeySize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_CONNECTION_GROUP {
    pub cConnections: u32,
    pub guidGroup: ::windows_core::GUID,
}
impl ::core::marker::Copy for WINHTTP_CONNECTION_GROUP {}
impl ::core::clone::Clone for WINHTTP_CONNECTION_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_CONNECTION_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CONNECTION_GROUP").field("cConnections", &self.cConnections).field("guidGroup", &self.guidGroup).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_CONNECTION_GROUP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_CONNECTION_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.cConnections == other.cConnections && self.guidGroup == other.guidGroup
    }
}
impl ::core::cmp::Eq for WINHTTP_CONNECTION_GROUP {}
impl ::core::default::Default for WINHTTP_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
    pub RemoteAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WINHTTP_CONNECTION_INFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WINHTTP_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for WINHTTP_CONNECTION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WINHTTP_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
    pub RemoteAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WINHTTP_CONNECTION_INFO {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WINHTTP_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for WINHTTP_CONNECTION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WINHTTP_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_CREDS {
    pub lpszUserName: ::windows_core::PSTR,
    pub lpszPassword: ::windows_core::PSTR,
    pub lpszRealm: ::windows_core::PSTR,
    pub dwAuthScheme: WINHTTP_CREDS_AUTHSCHEME,
    pub lpszHostName: ::windows_core::PSTR,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WINHTTP_CREDS {}
impl ::core::clone::Clone for WINHTTP_CREDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_CREDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CREDS").field("lpszUserName", &self.lpszUserName).field("lpszPassword", &self.lpszPassword).field("lpszRealm", &self.lpszRealm).field("dwAuthScheme", &self.dwAuthScheme).field("lpszHostName", &self.lpszHostName).field("dwPort", &self.dwPort).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_CREDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_CREDS {
    fn eq(&self, other: &Self) -> bool {
        self.lpszUserName == other.lpszUserName && self.lpszPassword == other.lpszPassword && self.lpszRealm == other.lpszRealm && self.dwAuthScheme == other.dwAuthScheme && self.lpszHostName == other.lpszHostName && self.dwPort == other.dwPort
    }
}
impl ::core::cmp::Eq for WINHTTP_CREDS {}
impl ::core::default::Default for WINHTTP_CREDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_CREDS_EX {
    pub lpszUserName: ::windows_core::PSTR,
    pub lpszPassword: ::windows_core::PSTR,
    pub lpszRealm: ::windows_core::PSTR,
    pub dwAuthScheme: WINHTTP_CREDS_AUTHSCHEME,
    pub lpszHostName: ::windows_core::PSTR,
    pub dwPort: u32,
    pub lpszUrl: ::windows_core::PSTR,
}
impl ::core::marker::Copy for WINHTTP_CREDS_EX {}
impl ::core::clone::Clone for WINHTTP_CREDS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_CREDS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CREDS_EX").field("lpszUserName", &self.lpszUserName).field("lpszPassword", &self.lpszPassword).field("lpszRealm", &self.lpszRealm).field("dwAuthScheme", &self.dwAuthScheme).field("lpszHostName", &self.lpszHostName).field("dwPort", &self.dwPort).field("lpszUrl", &self.lpszUrl).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_CREDS_EX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_CREDS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.lpszUserName == other.lpszUserName && self.lpszPassword == other.lpszPassword && self.lpszRealm == other.lpszRealm && self.dwAuthScheme == other.dwAuthScheme && self.lpszHostName == other.lpszHostName && self.dwPort == other.dwPort && self.lpszUrl == other.lpszUrl
    }
}
impl ::core::cmp::Eq for WINHTTP_CREDS_EX {}
impl ::core::default::Default for WINHTTP_CREDS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub fAutoDetect: super::super::Foundation::BOOL,
    pub lpszAutoConfigUrl: ::windows_core::PWSTR,
    pub lpszProxy: ::windows_core::PWSTR,
    pub lpszProxyBypass: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CURRENT_USER_IE_PROXY_CONFIG").field("fAutoDetect", &self.fAutoDetect).field("lpszAutoConfigUrl", &self.lpszAutoConfigUrl).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.fAutoDetect == other.fAutoDetect && self.lpszAutoConfigUrl == other.lpszAutoConfigUrl && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_EXTENDED_HEADER {
    pub Anonymous1: WINHTTP_EXTENDED_HEADER_0,
    pub Anonymous2: WINHTTP_EXTENDED_HEADER_1,
}
impl ::core::marker::Copy for WINHTTP_EXTENDED_HEADER {}
impl ::core::clone::Clone for WINHTTP_EXTENDED_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WINHTTP_EXTENDED_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WINHTTP_EXTENDED_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WINHTTP_EXTENDED_HEADER_0 {
    pub pwszName: ::windows_core::PCWSTR,
    pub pszName: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for WINHTTP_EXTENDED_HEADER_0 {}
impl ::core::clone::Clone for WINHTTP_EXTENDED_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WINHTTP_EXTENDED_HEADER_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WINHTTP_EXTENDED_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WINHTTP_EXTENDED_HEADER_1 {
    pub pwszValue: ::windows_core::PCWSTR,
    pub pszValue: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for WINHTTP_EXTENDED_HEADER_1 {}
impl ::core::clone::Clone for WINHTTP_EXTENDED_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WINHTTP_EXTENDED_HEADER_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WINHTTP_EXTENDED_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_FAILED_CONNECTION_RETRIES {
    pub dwMaxRetries: u32,
    pub dwAllowedRetryConditions: u32,
}
impl ::core::marker::Copy for WINHTTP_FAILED_CONNECTION_RETRIES {}
impl ::core::clone::Clone for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_FAILED_CONNECTION_RETRIES").field("dwMaxRetries", &self.dwMaxRetries).field("dwAllowedRetryConditions", &self.dwAllowedRetryConditions).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_FAILED_CONNECTION_RETRIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxRetries == other.dwMaxRetries && self.dwAllowedRetryConditions == other.dwAllowedRetryConditions
    }
}
impl ::core::cmp::Eq for WINHTTP_FAILED_CONNECTION_RETRIES {}
impl ::core::default::Default for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WINHTTP_HEADER_NAME {
    pub pwszName: ::windows_core::PCWSTR,
    pub pszName: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for WINHTTP_HEADER_NAME {}
impl ::core::clone::Clone for WINHTTP_HEADER_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WINHTTP_HEADER_NAME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_HOST_CONNECTION_GROUP {
    pub pwszHost: ::windows_core::PCWSTR,
    pub cConnectionGroups: u32,
    pub pConnectionGroups: *mut WINHTTP_CONNECTION_GROUP,
}
impl ::core::marker::Copy for WINHTTP_HOST_CONNECTION_GROUP {}
impl ::core::clone::Clone for WINHTTP_HOST_CONNECTION_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_HOST_CONNECTION_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_HOST_CONNECTION_GROUP").field("pwszHost", &self.pwszHost).field("cConnectionGroups", &self.cConnectionGroups).field("pConnectionGroups", &self.pConnectionGroups).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_HOST_CONNECTION_GROUP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_HOST_CONNECTION_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.pwszHost == other.pwszHost && self.cConnectionGroups == other.cConnectionGroups && self.pConnectionGroups == other.pConnectionGroups
    }
}
impl ::core::cmp::Eq for WINHTTP_HOST_CONNECTION_GROUP {}
impl ::core::default::Default for WINHTTP_HOST_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_HTTP2_RECEIVE_WINDOW {
    pub ulStreamWindow: u32,
    pub ulStreamWindowUpdateDelta: u32,
}
impl ::core::marker::Copy for WINHTTP_HTTP2_RECEIVE_WINDOW {}
impl ::core::clone::Clone for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_HTTP2_RECEIVE_WINDOW").field("ulStreamWindow", &self.ulStreamWindow).field("ulStreamWindowUpdateDelta", &self.ulStreamWindowUpdateDelta).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_HTTP2_RECEIVE_WINDOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.ulStreamWindow == other.ulStreamWindow && self.ulStreamWindowUpdateDelta == other.ulStreamWindowUpdateDelta
    }
}
impl ::core::cmp::Eq for WINHTTP_HTTP2_RECEIVE_WINDOW {}
impl ::core::default::Default for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: ::windows_core::GUID,
    pub ullFlags: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_MATCH_CONNECTION_GUID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_MATCH_CONNECTION_GUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_MATCH_CONNECTION_GUID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_MATCH_CONNECTION_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: ::windows_core::GUID,
    pub ullFlags: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_MATCH_CONNECTION_GUID {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_MATCH_CONNECTION_GUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_MATCH_CONNECTION_GUID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_MATCH_CONNECTION_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_PROXY_INFO {
    pub dwAccessType: WINHTTP_ACCESS_TYPE,
    pub lpszProxy: ::windows_core::PWSTR,
    pub lpszProxyBypass: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WINHTTP_PROXY_INFO {}
impl ::core::clone::Clone for WINHTTP_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_INFO").field("dwAccessType", &self.dwAccessType).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_PROXY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAccessType == other.dwAccessType && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
impl ::core::cmp::Eq for WINHTTP_PROXY_INFO {}
impl ::core::default::Default for WINHTTP_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_PROXY_NETWORKING_KEY {
    pub pbBuffer: [u8; 128],
}
impl ::core::marker::Copy for WINHTTP_PROXY_NETWORKING_KEY {}
impl ::core::clone::Clone for WINHTTP_PROXY_NETWORKING_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_PROXY_NETWORKING_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_NETWORKING_KEY").field("pbBuffer", &self.pbBuffer).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_PROXY_NETWORKING_KEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_PROXY_NETWORKING_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pbBuffer == other.pbBuffer
    }
}
impl ::core::cmp::Eq for WINHTTP_PROXY_NETWORKING_KEY {}
impl ::core::default::Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_PROXY_RESULT {
    pub cEntries: u32,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_PROXY_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_PROXY_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_PROXY_RESULT_ENTRY {
    pub fProxy: super::super::Foundation::BOOL,
    pub fBypass: super::super::Foundation::BOOL,
    pub ProxyScheme: WINHTTP_INTERNET_SCHEME,
    pub pwszProxy: ::windows_core::PWSTR,
    pub ProxyPort: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_PROXY_RESULT_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_PROXY_RESULT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT_ENTRY").field("fProxy", &self.fProxy).field("fBypass", &self.fBypass).field("ProxyScheme", &self.ProxyScheme).field("pwszProxy", &self.pwszProxy).field("ProxyPort", &self.ProxyPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_RESULT_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.fProxy == other.fProxy && self.fBypass == other.fBypass && self.ProxyScheme == other.ProxyScheme && self.pwszProxy == other.pwszProxy && self.ProxyPort == other.ProxyPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_PROXY_RESULT_EX {
    pub cEntries: u32,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
    pub hProxyDetectionHandle: super::super::Foundation::HANDLE,
    pub dwProxyInterfaceAffinity: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_PROXY_RESULT_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_PROXY_RESULT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT_EX").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).field("hProxyDetectionHandle", &self.hProxyDetectionHandle).field("dwProxyInterfaceAffinity", &self.dwProxyInterfaceAffinity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_RESULT_EX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries && self.hProxyDetectionHandle == other.hProxyDetectionHandle && self.dwProxyInterfaceAffinity == other.dwProxyInterfaceAffinity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINHTTP_PROXY_SETTINGS {
    pub dwStructSize: u32,
    pub dwFlags: u32,
    pub dwCurrentSettingsVersion: u32,
    pub pwszConnectionName: ::windows_core::PWSTR,
    pub pwszProxy: ::windows_core::PWSTR,
    pub pwszProxyBypass: ::windows_core::PWSTR,
    pub pwszAutoconfigUrl: ::windows_core::PWSTR,
    pub pwszAutoconfigSecondaryUrl: ::windows_core::PWSTR,
    pub dwAutoDiscoveryFlags: u32,
    pub pwszLastKnownGoodAutoConfigUrl: ::windows_core::PWSTR,
    pub dwAutoconfigReloadDelayMins: u32,
    pub ftLastKnownDetectTime: super::super::Foundation::FILETIME,
    pub dwDetectedInterfaceIpCount: u32,
    pub pdwDetectedInterfaceIp: *mut u32,
    pub cNetworkKeys: u32,
    pub pNetworkKeys: *mut WINHTTP_PROXY_NETWORKING_KEY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_SETTINGS")
            .field("dwStructSize", &self.dwStructSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwCurrentSettingsVersion", &self.dwCurrentSettingsVersion)
            .field("pwszConnectionName", &self.pwszConnectionName)
            .field("pwszProxy", &self.pwszProxy)
            .field("pwszProxyBypass", &self.pwszProxyBypass)
            .field("pwszAutoconfigUrl", &self.pwszAutoconfigUrl)
            .field("pwszAutoconfigSecondaryUrl", &self.pwszAutoconfigSecondaryUrl)
            .field("dwAutoDiscoveryFlags", &self.dwAutoDiscoveryFlags)
            .field("pwszLastKnownGoodAutoConfigUrl", &self.pwszLastKnownGoodAutoConfigUrl)
            .field("dwAutoconfigReloadDelayMins", &self.dwAutoconfigReloadDelayMins)
            .field("ftLastKnownDetectTime", &self.ftLastKnownDetectTime)
            .field("dwDetectedInterfaceIpCount", &self.dwDetectedInterfaceIpCount)
            .field("pdwDetectedInterfaceIp", &self.pdwDetectedInterfaceIp)
            .field("cNetworkKeys", &self.cNetworkKeys)
            .field("pNetworkKeys", &self.pNetworkKeys)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.dwFlags == other.dwFlags
            && self.dwCurrentSettingsVersion == other.dwCurrentSettingsVersion
            && self.pwszConnectionName == other.pwszConnectionName
            && self.pwszProxy == other.pwszProxy
            && self.pwszProxyBypass == other.pwszProxyBypass
            && self.pwszAutoconfigUrl == other.pwszAutoconfigUrl
            && self.pwszAutoconfigSecondaryUrl == other.pwszAutoconfigSecondaryUrl
            && self.dwAutoDiscoveryFlags == other.dwAutoDiscoveryFlags
            && self.pwszLastKnownGoodAutoConfigUrl == other.pwszLastKnownGoodAutoConfigUrl
            && self.dwAutoconfigReloadDelayMins == other.dwAutoconfigReloadDelayMins
            && self.ftLastKnownDetectTime == other.ftLastKnownDetectTime
            && self.dwDetectedInterfaceIpCount == other.dwDetectedInterfaceIpCount
            && self.pdwDetectedInterfaceIp == other.pdwDetectedInterfaceIp
            && self.cNetworkKeys == other.cNetworkKeys
            && self.pNetworkKeys == other.pNetworkKeys
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: u64,
    pub ullFlags: u64,
    pub pcwszAutoconfigUrl: ::windows_core::PCWSTR,
    pub pcwszProxy: ::windows_core::PCWSTR,
    pub pcwszSecureProxy: ::windows_core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *const ::windows_core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: ::windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS_EX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: u64,
    pub ullFlags: u64,
    pub pcwszAutoconfigUrl: ::windows_core::PCWSTR,
    pub pcwszProxy: ::windows_core::PCWSTR,
    pub pcwszSecureProxy: ::windows_core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *const ::windows_core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: ::windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS_EX {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS_EX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: ::windows_core::PCWSTR,
    pub pcwszProbeHost: ::windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS_PARAM {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: ::windows_core::PCWSTR,
    pub pcwszProbeHost: ::windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_PROXY_SETTINGS_PARAM {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_PROXY_SETTINGS_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_PROXY_SETTINGS_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    pub cHosts: u32,
    pub pHostConnectionGroups: *mut WINHTTP_HOST_CONNECTION_GROUP,
}
impl ::core::marker::Copy for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {}
impl ::core::clone::Clone for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_QUERY_CONNECTION_GROUP_RESULT").field("cHosts", &self.cHosts).field("pHostConnectionGroups", &self.pHostConnectionGroups).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.cHosts == other.cHosts && self.pHostConnectionGroups == other.pHostConnectionGroups
    }
}
impl ::core::cmp::Eq for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {}
impl ::core::default::Default for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: u64,
    pub ulIndex: u32,
    pub cStats: u32,
    pub rgullStats: [u64; 32],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_REQUEST_STATS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_REQUEST_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_REQUEST_STATS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: u64,
    pub ulIndex: u32,
    pub cStats: u32,
    pub rgullStats: [u64; 32],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_REQUEST_STATS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_REQUEST_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_REQUEST_STATS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgullTimes: [u64; 64],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_REQUEST_TIMES {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_REQUEST_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_REQUEST_TIMES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgullTimes: [u64; 64],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_REQUEST_TIMES {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_REQUEST_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_REQUEST_TIMES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: u32,
    pub ulMaxCacheEntryAge: u32,
    pub ulMinCacheEntryTtl: u32,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: u64,
    pub ullFlags: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WINHTTP_RESOLVER_CACHE_CONFIG {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for WINHTTP_RESOLVER_CACHE_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: u32,
    pub ulMaxCacheEntryAge: u32,
    pub ulMinCacheEntryTtl: u32,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: u64,
    pub ullFlags: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WINHTTP_RESOLVER_CACHE_CONFIG {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for WINHTTP_RESOLVER_CACHE_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_WEB_SOCKET_OPERATION,
}
impl ::core::marker::Copy for WINHTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::clone::Clone for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_WEB_SOCKET_ASYNC_RESULT").field("AsyncResult", &self.AsyncResult).field("Operation", &self.Operation).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.AsyncResult == other.AsyncResult && self.Operation == other.Operation
    }
}
impl ::core::cmp::Eq for WINHTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WINHTTP_WEB_SOCKET_STATUS {
    pub dwBytesTransferred: u32,
    pub eBufferType: WINHTTP_WEB_SOCKET_BUFFER_TYPE,
}
impl ::core::marker::Copy for WINHTTP_WEB_SOCKET_STATUS {}
impl ::core::clone::Clone for WINHTTP_WEB_SOCKET_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_WEB_SOCKET_STATUS").field("dwBytesTransferred", &self.dwBytesTransferred).field("eBufferType", &self.eBufferType).finish()
    }
}
impl ::windows_core::TypeKind for WINHTTP_WEB_SOCKET_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WINHTTP_WEB_SOCKET_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwBytesTransferred == other.dwBytesTransferred && self.eBufferType == other.eBufferType
    }
}
impl ::core::cmp::Eq for WINHTTP_WEB_SOCKET_STATUS {}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WINHTTP_PROXY_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(ullflags: u64, pvcontext: *const ::core::ffi::c_void) -> ()>;
pub type WINHTTP_STATUS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hinternet: *mut ::core::ffi::c_void, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *mut ::core::ffi::c_void, dwstatusinformationlength: u32) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
