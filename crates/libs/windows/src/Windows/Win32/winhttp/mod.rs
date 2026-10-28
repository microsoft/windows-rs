#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpAddRequestHeaders(hrequest: super::HINTERNET, lpszheaders: windows_core::PCWSTR, dwheaderslength: super::DWORD, dwmodifiers: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpAddRequestHeaders(hrequest : super::HINTERNET, lpszheaders : windows_core::PCWSTR, dwheaderslength : super::DWORD, dwmodifiers : super::DWORD) -> windows_core::BOOL);
    unsafe { WinHttpAddRequestHeaders(hrequest as _, lpszheaders, dwheaderslength, dwmodifiers) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpAddRequestHeadersEx(hrequest: super::HINTERNET, dwmodifiers: super::DWORD, ullflags: super::ULONGLONG, ullextra: super::ULONGLONG, pheaders: &[WINHTTP_EXTENDED_HEADER]) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpAddRequestHeadersEx(hrequest : super::HINTERNET, dwmodifiers : super::DWORD, ullflags : super::ULONGLONG, ullextra : super::ULONGLONG, cheaders : super::DWORD, pheaders : *const WINHTTP_EXTENDED_HEADER) -> super::DWORD);
    unsafe { WinHttpAddRequestHeadersEx(hrequest as _, dwmodifiers, ullflags, ullextra, pheaders.len().try_into().unwrap(), pheaders.as_ptr()) }
}
#[inline]
pub unsafe fn WinHttpCheckPlatform() -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCheckPlatform() -> windows_core::BOOL);
    unsafe { WinHttpCheckPlatform() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpCloseHandle(hinternet: super::HINTERNET) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCloseHandle(hinternet : super::HINTERNET) -> windows_core::BOOL);
    unsafe { WinHttpCloseHandle(hinternet as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpConnect<P1>(hsession: super::HINTERNET, pswzservername: P1, nserverport: super::INTERNET_PORT, dwreserved: super::DWORD) -> super::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpConnect(hsession : super::HINTERNET, pswzservername : windows_core::PCWSTR, nserverport : super::INTERNET_PORT, dwreserved : super::DWORD) -> super::HINTERNET);
    unsafe { WinHttpConnect(hsession as _, pswzservername.param().abi(), nserverport, dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpCrackUrl(pwszurl: &[u16], dwflags: super::DWORD, lpurlcomponents: LPURL_COMPONENTS) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCrackUrl(pwszurl : windows_core::PCWSTR, dwurllength : super::DWORD, dwflags : super::DWORD, lpurlcomponents : LPURL_COMPONENTS) -> windows_core::BOOL);
    unsafe { WinHttpCrackUrl(core::mem::transmute(pwszurl.as_ptr()), pwszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpCreateProxyResolver(hsession: super::HINTERNET, phresolver: *mut super::HINTERNET) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCreateProxyResolver(hsession : super::HINTERNET, phresolver : *mut super::HINTERNET) -> super::DWORD);
    unsafe { WinHttpCreateProxyResolver(hsession, phresolver as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpCreateUrl(lpurlcomponents: LPURL_COMPONENTS, dwflags: super::DWORD, pwszurl: Option<windows_core::PWSTR>, pdwurllength: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCreateUrl(lpurlcomponents : LPURL_COMPONENTS, dwflags : super::DWORD, pwszurl : windows_core::PWSTR, pdwurllength : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpCreateUrl(lpurlcomponents, dwflags, pwszurl.unwrap_or(core::mem::zeroed()) as _, pdwurllength as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags: super::DWORD, ppwstrautoconfigurl: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags : super::DWORD, ppwstrautoconfigurl : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WinHttpDetectAutoProxyConfigUrl(dwautodetectflags, ppwstrautoconfigurl as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpFreeProxyResult(pproxyresult: *mut WINHTTP_PROXY_RESULT) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxyResult(pproxyresult : *mut WINHTTP_PROXY_RESULT));
    unsafe { WinHttpFreeProxyResult(pproxyresult as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpFreeProxyResultEx(pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxyResultEx(pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX));
    unsafe { WinHttpFreeProxyResultEx(pproxyresultex as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpFreeProxySettings(pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxySettings(pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS));
    unsafe { WinHttpFreeProxySettings(pwinhttpproxysettings) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpFreeProxySettingsEx(proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex: super::PVOID) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxySettingsEx(proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex : super::PVOID) -> super::DWORD);
    unsafe { WinHttpFreeProxySettingsEx(proxysettingstype, pproxysettingsex) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpFreeQueryConnectionGroupResult(presult: *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeQueryConnectionGroupResult(presult : *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT));
    unsafe { WinHttpFreeQueryConnectionGroupResult(presult as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpGetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpGetDefaultProxyConfiguration(pproxyinfo as _) }
}
#[inline]
pub unsafe fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig : *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> windows_core::BOOL);
    unsafe { WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpGetProxyForUrl<P1>(hsession: super::HINTERNET, lpcwszurl: P1, pautoproxyoptions: *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrl(hsession : super::HINTERNET, lpcwszurl : windows_core::PCWSTR, pautoproxyoptions : *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpGetProxyForUrl(hsession as _, lpcwszurl.param().abi(), pautoproxyoptions as _, pproxyinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx<P1>(hresolver: super::HINTERNET, pcwszurl: P1, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pcontext: Option<super::DWORD_PTR>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx(hresolver : super::HINTERNET, pcwszurl : windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, pcontext : super::DWORD_PTR) -> super::DWORD);
    unsafe { WinHttpGetProxyForUrlEx(hresolver, pcwszurl.param().abi(), pautoproxyoptions, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx2<P1>(hresolver: super::HINTERNET, pcwszurl: P1, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pinterfaceselectioncontext: Option<&[u8]>, pcontext: Option<super::DWORD_PTR>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx2(hresolver : super::HINTERNET, pcwszurl : windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, cbinterfaceselectioncontext : super::DWORD, pinterfaceselectioncontext : *const super::BYTE, pcontext : super::DWORD_PTR) -> super::DWORD);
    unsafe { WinHttpGetProxyForUrlEx2(hresolver, pcwszurl.param().abi(), pautoproxyoptions, pinterfaceselectioncontext.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pinterfaceselectioncontext.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpGetProxyResult(hresolver: super::HINTERNET, pproxyresult: *mut WINHTTP_PROXY_RESULT) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyResult(hresolver : super::HINTERNET, pproxyresult : *mut WINHTTP_PROXY_RESULT) -> super::DWORD);
    unsafe { WinHttpGetProxyResult(hresolver, pproxyresult as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpGetProxyResultEx(hresolver: super::HINTERNET, pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyResultEx(hresolver : super::HINTERNET, pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX) -> super::DWORD);
    unsafe { WinHttpGetProxyResultEx(hresolver, pproxyresultex as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpGetProxySettingsEx(hresolver: super::HINTERNET, proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam: Option<PWINHTTP_PROXY_SETTINGS_PARAM>, pcontext: Option<super::DWORD_PTR>) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsEx(hresolver : super::HINTERNET, proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam : PWINHTTP_PROXY_SETTINGS_PARAM, pcontext : super::DWORD_PTR) -> super::DWORD);
    unsafe { WinHttpGetProxySettingsEx(hresolver, proxysettingstype, pproxysettingsparam.unwrap_or(core::mem::zeroed()) as _, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpGetProxySettingsResultEx(hresolver: super::HINTERNET, pproxysettingsex: super::PVOID) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsResultEx(hresolver : super::HINTERNET, pproxysettingsex : super::PVOID) -> super::DWORD);
    unsafe { WinHttpGetProxySettingsResultEx(hresolver, pproxysettingsex as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpGetProxySettingsVersion(hsession: super::HINTERNET, pdwproxysettingsversion: *mut super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsVersion(hsession : super::HINTERNET, pdwproxysettingsversion : *mut super::DWORD) -> super::DWORD);
    unsafe { WinHttpGetProxySettingsVersion(hsession, pdwproxysettingsversion as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpOpen<P0, P2, P3>(pszagentw: P0, dwaccesstype: super::DWORD, pszproxyw: P2, pszproxybypassw: P3, dwflags: super::DWORD) -> super::HINTERNET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpOpen(pszagentw : windows_core::PCWSTR, dwaccesstype : super::DWORD, pszproxyw : windows_core::PCWSTR, pszproxybypassw : windows_core::PCWSTR, dwflags : super::DWORD) -> super::HINTERNET);
    unsafe { WinHttpOpen(pszagentw.param().abi(), dwaccesstype, pszproxyw.param().abi(), pszproxybypassw.param().abi(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpOpenRequest<P1, P2, P3, P4>(hconnect: super::HINTERNET, pwszverb: P1, pwszobjectname: P2, pwszversion: P3, pwszreferrer: P4, ppwszaccepttypes: *mut windows_core::PCWSTR, dwflags: super::DWORD) -> super::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpOpenRequest(hconnect : super::HINTERNET, pwszverb : windows_core::PCWSTR, pwszobjectname : windows_core::PCWSTR, pwszversion : windows_core::PCWSTR, pwszreferrer : windows_core::PCWSTR, ppwszaccepttypes : *mut windows_core::PCWSTR, dwflags : super::DWORD) -> super::HINTERNET);
    unsafe { WinHttpOpenRequest(hconnect as _, pwszverb.param().abi(), pwszobjectname.param().abi(), pwszversion.param().abi(), pwszreferrer.param().abi(), ppwszaccepttypes as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpProtocolCompleteUpgrade(hrequest: super::HINTERNET, dwcontext: Option<super::DWORD_PTR>) -> super::HINTERNET {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolCompleteUpgrade(hrequest : super::HINTERNET, dwcontext : super::DWORD_PTR) -> super::HINTERNET);
    unsafe { WinHttpProtocolCompleteUpgrade(hrequest, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpProtocolReceive(protocolhandle: super::HINTERNET, flags: super::ULONGLONG, pvbuffer: super::PVOID, dwbufferlength: super::DWORD, pdwbytesread: *mut super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolReceive(protocolhandle : super::HINTERNET, flags : super::ULONGLONG, pvbuffer : super::PVOID, dwbufferlength : super::DWORD, pdwbytesread : *mut super::DWORD) -> super::DWORD);
    unsafe { WinHttpProtocolReceive(protocolhandle, flags, pvbuffer as _, dwbufferlength, pdwbytesread as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpProtocolSend(protocolhandle: super::HINTERNET, flags: super::ULONGLONG, pvbuffer: Option<&[u8]>) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolSend(protocolhandle : super::HINTERNET, flags : super::ULONGLONG, pvbuffer : super::PVOID, dwbufferlength : super::DWORD) -> super::DWORD);
    unsafe { WinHttpProtocolSend(protocolhandle, flags, core::mem::transmute(pvbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpQueryAuthSchemes(hrequest: super::HINTERNET, lpdwsupportedschemes: super::LPDWORD, lpdwfirstscheme: super::LPDWORD, pdwauthtarget: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryAuthSchemes(hrequest : super::HINTERNET, lpdwsupportedschemes : super::LPDWORD, lpdwfirstscheme : super::LPDWORD, pdwauthtarget : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpQueryAuthSchemes(hrequest as _, lpdwsupportedschemes as _, lpdwfirstscheme as _, pdwauthtarget as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpQueryConnectionGroup(hinternet: super::HINTERNET, pguidconnection: Option<*const windows_core::GUID>, ullflags: super::ULONGLONG, ppresult: *mut PWINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryConnectionGroup(hinternet : super::HINTERNET, pguidconnection : *const windows_core::GUID, ullflags : super::ULONGLONG, ppresult : *mut PWINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> super::DWORD);
    unsafe { WinHttpQueryConnectionGroup(hinternet, pguidconnection.unwrap_or(core::mem::zeroed()) as _, ullflags, ppresult as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpQueryDataAvailable(hrequest: super::HINTERNET, lpdwnumberofbytesavailable: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryDataAvailable(hrequest : super::HINTERNET, lpdwnumberofbytesavailable : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpQueryDataAvailable(hrequest as _, lpdwnumberofbytesavailable as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpQueryHeaders<P2>(hrequest: super::HINTERNET, dwinfolevel: super::DWORD, pwszname: P2, lpbuffer: Option<super::LPVOID>, lpdwbufferlength: super::LPDWORD, lpdwindex: super::LPDWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryHeaders(hrequest : super::HINTERNET, dwinfolevel : super::DWORD, pwszname : windows_core::PCWSTR, lpbuffer : super::LPVOID, lpdwbufferlength : super::LPDWORD, lpdwindex : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpQueryHeaders(hrequest as _, dwinfolevel, pwszname.param().abi(), lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpQueryHeadersEx(hrequest: super::HINTERNET, dwinfolevel: super::DWORD, ullflags: super::ULONGLONG, uicodepage: super::UINT, pdwindex: Option<super::PDWORD>, pheadername: Option<PWINHTTP_HEADER_NAME>, pbuffer: Option<super::PVOID>, pdwbufferlength: super::PDWORD, ppheaders: Option<*mut PWINHTTP_EXTENDED_HEADER>, pdwheaderscount: super::PDWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryHeadersEx(hrequest : super::HINTERNET, dwinfolevel : super::DWORD, ullflags : super::ULONGLONG, uicodepage : super::UINT, pdwindex : super::PDWORD, pheadername : PWINHTTP_HEADER_NAME, pbuffer : super::PVOID, pdwbufferlength : super::PDWORD, ppheaders : *mut PWINHTTP_EXTENDED_HEADER, pdwheaderscount : super::PDWORD) -> super::DWORD);
    unsafe { WinHttpQueryHeadersEx(hrequest, dwinfolevel, ullflags, uicodepage, pdwindex.unwrap_or(core::mem::zeroed()) as _, pheadername.unwrap_or(core::mem::zeroed()) as _, pbuffer.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength as _, ppheaders.unwrap_or(core::mem::zeroed()) as _, pdwheaderscount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpQueryOption(hinternet: super::HINTERNET, dwoption: super::DWORD, lpbuffer: Option<super::LPVOID>, lpdwbufferlength: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryOption(hinternet : super::HINTERNET, dwoption : super::DWORD, lpbuffer : super::LPVOID, lpdwbufferlength : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpQueryOption(hinternet as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpReadData(hrequest: super::HINTERNET, lpbuffer: super::LPVOID, dwnumberofbytestoread: super::DWORD, lpdwnumberofbytesread: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadData(hrequest : super::HINTERNET, lpbuffer : super::LPVOID, dwnumberofbytestoread : super::DWORD, lpdwnumberofbytesread : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpReadData(hrequest as _, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpReadDataEx(hrequest: super::HINTERNET, lpbuffer: super::LPVOID, dwnumberofbytestoread: super::DWORD, lpdwnumberofbytesread: super::LPDWORD, ullflags: super::ULONGLONG, cbproperty: super::DWORD, pvproperty: Option<super::PVOID>) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadDataEx(hrequest : super::HINTERNET, lpbuffer : super::LPVOID, dwnumberofbytestoread : super::DWORD, lpdwnumberofbytesread : super::LPDWORD, ullflags : super::ULONGLONG, cbproperty : super::DWORD, pvproperty : super::PVOID) -> super::DWORD);
    unsafe { WinHttpReadDataEx(hrequest as _, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _, ullflags, cbproperty, pvproperty.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpReadProxySettings<P1>(hsession: super::HINTERNET, pcwszconnectionname: P1, ffallbacktodefaultsettings: bool, fsetautodiscoverfordefaultsettings: bool, pdwsettingsversion: *mut super::DWORD, pfdefaultsettingsarereturned: *mut windows_core::BOOL, pwinhttpproxysettings: *mut WINHTTP_PROXY_SETTINGS) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadProxySettings(hsession : super::HINTERNET, pcwszconnectionname : windows_core::PCWSTR, ffallbacktodefaultsettings : windows_core::BOOL, fsetautodiscoverfordefaultsettings : windows_core::BOOL, pdwsettingsversion : *mut super::DWORD, pfdefaultsettingsarereturned : *mut windows_core::BOOL, pwinhttpproxysettings : *mut WINHTTP_PROXY_SETTINGS) -> super::DWORD);
    unsafe { WinHttpReadProxySettings(hsession, pcwszconnectionname.param().abi(), ffallbacktodefaultsettings.into(), fsetautodiscoverfordefaultsettings.into(), pdwsettingsversion as _, pfdefaultsettingsarereturned as _, pwinhttpproxysettings as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpReceiveResponse(hrequest: super::HINTERNET, lpreserved: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReceiveResponse(hrequest : super::HINTERNET, lpreserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { WinHttpReceiveResponse(hrequest as _, lpreserved as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpRegisterProxyChangeNotification(ullflags: super::ULONGLONG, pfncallback: WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext: super::PVOID, hregistration: *mut WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpRegisterProxyChangeNotification(ullflags : super::ULONGLONG, pfncallback : WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext : super::PVOID, hregistration : *mut WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> super::DWORD);
    unsafe { WinHttpRegisterProxyChangeNotification(ullflags, pfncallback, pvcontext, hregistration as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpResetAutoProxy(hsession: super::HINTERNET, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpResetAutoProxy(hsession : super::HINTERNET, dwflags : super::DWORD) -> super::DWORD);
    unsafe { WinHttpResetAutoProxy(hsession, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpSendRequest(hrequest: super::HINTERNET, lpszheaders: Option<&[u16]>, lpoptional: Option<super::LPVOID>, dwoptionallength: super::DWORD, dwtotallength: super::DWORD, dwcontext: super::DWORD_PTR) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSendRequest(hrequest : super::HINTERNET, lpszheaders : windows_core::PCWSTR, dwheaderslength : super::DWORD, lpoptional : super::LPVOID, dwoptionallength : super::DWORD, dwtotallength : super::DWORD, dwcontext : super::DWORD_PTR) -> windows_core::BOOL);
    unsafe { WinHttpSendRequest(hrequest as _, core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength, dwtotallength, dwcontext) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpSetCredentials<P3, P4>(hrequest: super::HINTERNET, authtargets: super::DWORD, authscheme: super::DWORD, pwszusername: P3, pwszpassword: P4, pauthparams: super::LPVOID) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetCredentials(hrequest : super::HINTERNET, authtargets : super::DWORD, authscheme : super::DWORD, pwszusername : windows_core::PCWSTR, pwszpassword : windows_core::PCWSTR, pauthparams : super::LPVOID) -> windows_core::BOOL);
    unsafe { WinHttpSetCredentials(hrequest as _, authtargets, authscheme, pwszusername.param().abi(), pwszpassword.param().abi(), pauthparams as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpSetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpSetDefaultProxyConfiguration(pproxyinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpSetOption(hinternet: Option<super::HINTERNET>, dwoption: super::DWORD, lpbuffer: super::LPVOID, dwbufferlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetOption(hinternet : super::HINTERNET, dwoption : super::DWORD, lpbuffer : super::LPVOID, dwbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { WinHttpSetOption(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer as _, dwbufferlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser: bool) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser : windows_core::BOOL) -> super::DWORD);
    unsafe { WinHttpSetProxySettingsPerUser(fproxysettingsperuser.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpSetStatusCallback(hinternet: super::HINTERNET, lpfninternetcallback: WINHTTP_STATUS_CALLBACK, dwnotificationflags: super::DWORD, dwreserved: super::DWORD_PTR) -> WINHTTP_STATUS_CALLBACK {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetStatusCallback(hinternet : super::HINTERNET, lpfninternetcallback : WINHTTP_STATUS_CALLBACK, dwnotificationflags : super::DWORD, dwreserved : super::DWORD_PTR) -> WINHTTP_STATUS_CALLBACK);
    unsafe { WinHttpSetStatusCallback(hinternet as _, lpfninternetcallback, dwnotificationflags, dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpSetTimeouts(hinternet: super::HINTERNET, nresolvetimeout: i32, nconnecttimeout: i32, nsendtimeout: i32, nreceivetimeout: i32) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetTimeouts(hinternet : super::HINTERNET, nresolvetimeout : i32, nconnecttimeout : i32, nsendtimeout : i32, nreceivetimeout : i32) -> windows_core::BOOL);
    unsafe { WinHttpSetTimeouts(hinternet as _, nresolvetimeout, nconnecttimeout, nsendtimeout, nreceivetimeout) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpTimeFromSystemTime(pst: *const super::SYSTEMTIME, pwsztime: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpTimeFromSystemTime(pst : *const super::SYSTEMTIME, pwsztime : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WinHttpTimeFromSystemTime(pst, pwsztime) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpTimeToSystemTime<P0>(pwsztime: P0, pst: *mut super::SYSTEMTIME) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpTimeToSystemTime(pwsztime : windows_core::PCWSTR, pst : *mut super::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { WinHttpTimeToSystemTime(pwsztime.param().abi(), pst as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpUnregisterProxyChangeNotification(hregistration: WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpUnregisterProxyChangeNotification(hregistration : WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> super::DWORD);
    unsafe { WinHttpUnregisterProxyChangeNotification(hregistration) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpWebSocketClose(hwebsocket: super::HINTERNET, usstatus: super::USHORT, pvreason: Option<super::PVOID>, dwreasonlength: super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketClose(hwebsocket : super::HINTERNET, usstatus : super::USHORT, pvreason : super::PVOID, dwreasonlength : super::DWORD) -> super::DWORD);
    unsafe { WinHttpWebSocketClose(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinHttpWebSocketCompleteUpgrade(hrequest: super::HINTERNET, pcontext: Option<super::DWORD_PTR>) -> super::HINTERNET {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketCompleteUpgrade(hrequest : super::HINTERNET, pcontext : super::DWORD_PTR) -> super::HINTERNET);
    unsafe { WinHttpWebSocketCompleteUpgrade(hrequest, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpWebSocketQueryCloseStatus(hwebsocket: super::HINTERNET, pusstatus: *mut super::USHORT, pvreason: Option<super::PVOID>, dwreasonlength: super::DWORD, pdwreasonlengthconsumed: *mut super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketQueryCloseStatus(hwebsocket : super::HINTERNET, pusstatus : *mut super::USHORT, pvreason : super::PVOID, dwreasonlength : super::DWORD, pdwreasonlengthconsumed : *mut super::DWORD) -> super::DWORD);
    unsafe { WinHttpWebSocketQueryCloseStatus(hwebsocket, pusstatus as _, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength, pdwreasonlengthconsumed as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpWebSocketReceive(hwebsocket: super::HINTERNET, pvbuffer: super::PVOID, dwbufferlength: super::DWORD, pdwbytesread: *mut super::DWORD, pebuffertype: *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketReceive(hwebsocket : super::HINTERNET, pvbuffer : super::PVOID, dwbufferlength : super::DWORD, pdwbytesread : *mut super::DWORD, pebuffertype : *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> super::DWORD);
    unsafe { WinHttpWebSocketReceive(hwebsocket, pvbuffer as _, dwbufferlength, pdwbytesread as _, pebuffertype as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpWebSocketSend(hwebsocket: super::HINTERNET, ebuffertype: WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: Option<&[u8]>) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketSend(hwebsocket : super::HINTERNET, ebuffertype : WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer : super::PVOID, dwbufferlength : super::DWORD) -> super::DWORD);
    unsafe { WinHttpWebSocketSend(hwebsocket, ebuffertype, core::mem::transmute(pvbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WinHttpWebSocketShutdown(hwebsocket: super::HINTERNET, usstatus: super::USHORT, pvreason: Option<super::PVOID>, dwreasonlength: super::DWORD) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketShutdown(hwebsocket : super::HINTERNET, usstatus : super::USHORT, pvreason : super::PVOID, dwreasonlength : super::DWORD) -> super::DWORD);
    unsafe { WinHttpWebSocketShutdown(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpWriteData(hrequest: super::HINTERNET, lpbuffer: Option<super::LPCVOID>, dwnumberofbytestowrite: super::DWORD, lpdwnumberofbyteswritten: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWriteData(hrequest : super::HINTERNET, lpbuffer : super::LPCVOID, dwnumberofbytestowrite : super::DWORD, lpdwnumberofbyteswritten : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WinHttpWriteData(hrequest as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwnumberofbytestowrite, lpdwnumberofbyteswritten as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WinHttpWriteProxySettings(hsession: super::HINTERNET, fforceupdate: bool, pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) -> super::DWORD {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWriteProxySettings(hsession : super::HINTERNET, fforceupdate : windows_core::BOOL, pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS) -> super::DWORD);
    unsafe { WinHttpWriteProxySettings(hsession, fforceupdate.into(), pwinhttpproxysettings) }
}
pub const API_GET_PROXY_FOR_URL: i32 = 6;
pub const API_GET_PROXY_SETTINGS: i32 = 7;
pub const API_QUERY_DATA_AVAILABLE: i32 = 2;
pub const API_READ_DATA: i32 = 3;
pub const API_RECEIVE_RESPONSE: i32 = 1;
pub const API_SEND_REQUEST: i32 = 5;
pub const API_WRITE_DATA: i32 = 4;
pub const ERROR_WINHTTP_AUTODETECTION_FAILED: i32 = 12180;
pub const ERROR_WINHTTP_AUTO_PROXY_SERVICE_ERROR: i32 = 12178;
pub const ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT: i32 = 12166;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_OPEN: i32 = 12103;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_SEND: i32 = 12102;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_OPEN: i32 = 12100;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_SEND: i32 = 12101;
pub const ERROR_WINHTTP_CANNOT_CONNECT: i32 = 12029;
pub const ERROR_WINHTTP_CHUNKED_ENCODING_HEADER_SIZE_OVERFLOW: i32 = 12183;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED: i32 = 12044;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED_PROXY: i32 = 12187;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_ACCESS_PRIVATE_KEY: i32 = 12186;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_PRIVATE_KEY: i32 = 12185;
pub const ERROR_WINHTTP_CONNECTION_ERROR: i32 = 12030;
pub const ERROR_WINHTTP_FAST_FORWARDING_NOT_SUPPORTED: i32 = 12193;
pub const ERROR_WINHTTP_FEATURE_DISABLED: i32 = 12192;
pub const ERROR_WINHTTP_GLOBAL_CALLBACK_FAILED: i32 = 12191;
pub const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: i32 = 12155;
pub const ERROR_WINHTTP_HEADER_COUNT_EXCEEDED: i32 = 12181;
pub const ERROR_WINHTTP_HEADER_NOT_FOUND: i32 = 12150;
pub const ERROR_WINHTTP_HEADER_SIZE_OVERFLOW: i32 = 12182;
pub const ERROR_WINHTTP_HTTP_PROTOCOL_MISMATCH: i32 = 12190;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: i32 = 12019;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: i32 = 12018;
pub const ERROR_WINHTTP_INTERNAL_ERROR: i32 = 12004;
pub const ERROR_WINHTTP_INVALID_HEADER: i32 = 12153;
pub const ERROR_WINHTTP_INVALID_OPTION: i32 = 12009;
pub const ERROR_WINHTTP_INVALID_QUERY_REQUEST: i32 = 12154;
pub const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: i32 = 12152;
pub const ERROR_WINHTTP_INVALID_URL: i32 = 12005;
pub const ERROR_WINHTTP_LOGIN_FAILURE: i32 = 12015;
pub const ERROR_WINHTTP_NAME_NOT_RESOLVED: i32 = 12007;
pub const ERROR_WINHTTP_NOT_INITIALIZED: i32 = 12172;
pub const ERROR_WINHTTP_OPERATION_CANCELLED: i32 = 12017;
pub const ERROR_WINHTTP_OPTION_NOT_SETTABLE: i32 = 12011;
pub const ERROR_WINHTTP_OUT_OF_HANDLES: i32 = 12001;
pub const ERROR_WINHTTP_REDIRECT_FAILED: i32 = 12156;
pub const ERROR_WINHTTP_RESEND_REQUEST: i32 = 12032;
pub const ERROR_WINHTTP_RESERVED_189: i32 = 12189;
pub const ERROR_WINHTTP_RESPONSE_DRAIN_OVERFLOW: i32 = 12184;
pub const ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR: i32 = 12177;
pub const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: i32 = 12038;
pub const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: i32 = 12037;
pub const ERROR_WINHTTP_SECURE_CERT_REVOKED: i32 = 12170;
pub const ERROR_WINHTTP_SECURE_CERT_REV_FAILED: i32 = 12057;
pub const ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE: i32 = 12179;
pub const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: i32 = 12157;
pub const ERROR_WINHTTP_SECURE_FAILURE: i32 = 12175;
pub const ERROR_WINHTTP_SECURE_FAILURE_PROXY: i32 = 12188;
pub const ERROR_WINHTTP_SECURE_INVALID_CA: i32 = 12045;
pub const ERROR_WINHTTP_SECURE_INVALID_CERT: i32 = 12169;
pub const ERROR_WINHTTP_SHUTDOWN: i32 = 12012;
pub const ERROR_WINHTTP_TIMEOUT: i32 = 12002;
pub const ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT: i32 = 12167;
pub const ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE: i32 = 12176;
pub const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: i32 = 12006;
pub const HTTP_STATUS_WEBDAV_MULTI_STATUS: i32 = 207;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION_INFO {
    pub dwMajorVersion: super::DWORD,
    pub dwMinorVersion: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION_INFO {
    pub dwMajorVersion: super::DWORD,
    pub dwMinorVersion: super::DWORD,
}
pub const ICU_ESCAPE_AUTHORITY: i32 = 8192;
pub const ICU_INCLUDE_DEFAULT_PORT: i32 = 32768;
pub const ICU_REJECT_USERPWD: i32 = 16384;
pub const INTERNET_DEFAULT_PORT: i32 = 0;
pub type INTERNET_SCHEME = i32;
pub const INTERNET_SCHEME_FTP: i32 = 3;
pub const INTERNET_SCHEME_HTTP: i32 = 1;
pub const INTERNET_SCHEME_HTTPS: i32 = 2;
pub const INTERNET_SCHEME_SOCKS: i32 = 4;
#[cfg(feature = "shared")]
pub type LPHTTP_VERSION_INFO = *mut HTTP_VERSION_INFO;
pub type LPINTERNET_SCHEME = *mut i32;
#[cfg(feature = "shared")]
pub type LPURL_COMPONENTS = *mut URL_COMPONENTS;
#[cfg(feature = "shared")]
pub type LPURL_COMPONENTSW = *mut URL_COMPONENTS;
#[cfg(feature = "shared")]
pub type LPWINHTTP_ASYNC_RESULT = *mut WINHTTP_ASYNC_RESULT;
#[cfg(feature = "shared")]
pub type LPWINHTTP_PROXY_INFO = *mut WINHTTP_PROXY_INFO;
#[cfg(feature = "shared")]
pub type LPWINHTTP_PROXY_INFOW = *mut WINHTTP_PROXY_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPWINHTTP_STATUS_CALLBACK = *mut WINHTTP_STATUS_CALLBACK;
pub const NETWORKING_KEY_BUFSIZE: i32 = 128;
#[cfg(feature = "shared")]
pub type PHTTP_VERSION_INFO = *mut HTTP_VERSION_INFO;
#[cfg(feature = "shared")]
pub type PWINHTTP_ASYNC_RESULT = *mut WINHTTP_ASYNC_RESULT;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINHTTP_AUTOPROXY_OPTIONS = *mut WINHTTP_AUTOPROXY_OPTIONS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINHTTP_CERTIFICATE_INFO = *mut WINHTTP_CERTIFICATE_INFO;
#[cfg(feature = "shared")]
pub type PWINHTTP_CONNECTION_GROUP = *mut WINHTTP_CONNECTION_GROUP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PWINHTTP_CONNECTION_INFO = *mut WINHTTP_CONNECTION_INFO;
#[cfg(feature = "shared")]
pub type PWINHTTP_CREDS = *mut WINHTTP_CREDS;
#[cfg(feature = "shared")]
pub type PWINHTTP_CREDS_EX = *mut WINHTTP_CREDS_EX;
pub type PWINHTTP_CURRENT_USER_IE_PROXY_CONFIG = *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG;
pub type PWINHTTP_EXTENDED_HEADER = *mut WINHTTP_EXTENDED_HEADER;
#[cfg(feature = "shared")]
pub type PWINHTTP_FAILED_CONNECTION_RETRIES = *mut WINHTTP_FAILED_CONNECTION_RETRIES;
pub type PWINHTTP_FAST_FORWARDING_STATE = *mut WINHTTP_FAST_FORWARDING_STATE;
#[cfg(feature = "shared")]
pub type PWINHTTP_FAST_FORWARDING_STATUS = *mut WINHTTP_FAST_FORWARDING_STATUS;
pub type PWINHTTP_HEADER_NAME = *mut WINHTTP_HEADER_NAME;
#[cfg(feature = "shared")]
pub type PWINHTTP_HOST_CONNECTION_GROUP = *mut WINHTTP_HOST_CONNECTION_GROUP;
#[cfg(feature = "shared")]
pub type PWINHTTP_HTTP2_RECEIVE_WINDOW = *mut WINHTTP_HTTP2_RECEIVE_WINDOW;
#[cfg(feature = "shared")]
pub type PWINHTTP_MATCH_CONNECTION_GUID = *mut WINHTTP_MATCH_CONNECTION_GUID;
#[cfg(feature = "shared")]
pub type PWINHTTP_PROXY_INFO = *mut WINHTTP_PROXY_INFO;
pub type PWINHTTP_PROXY_NETWORKING_KEY = *mut WINHTTP_PROXY_NETWORKING_KEY;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINHTTP_PROXY_SETTINGS = *mut WINHTTP_PROXY_SETTINGS;
#[cfg(feature = "shared")]
pub type PWINHTTP_PROXY_SETTINGS_EX = *mut WINHTTP_PROXY_SETTINGS_EX;
#[cfg(feature = "shared")]
pub type PWINHTTP_PROXY_SETTINGS_PARAM = *mut WINHTTP_PROXY_SETTINGS_PARAM;
pub type PWINHTTP_PROXY_SETTINGS_TYPE = *mut WINHTTP_PROXY_SETTINGS_TYPE;
#[cfg(feature = "shared")]
pub type PWINHTTP_QUERY_CONNECTION_GROUP_RESULT = *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT;
#[cfg(feature = "shared")]
pub type PWINHTTP_REQUEST_STATS = *mut WINHTTP_REQUEST_STATS;
#[cfg(feature = "shared")]
pub type PWINHTTP_REQUEST_TIMES = *mut WINHTTP_REQUEST_TIMES;
#[cfg(feature = "shared")]
pub type PWINHTTP_RESOLVER_CACHE_CONFIG = *mut WINHTTP_RESOLVER_CACHE_CONFIG;
pub const SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS: i32 = 13056;
pub const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: i32 = 512;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct URL_COMPONENTS {
    pub dwStructSize: super::DWORD,
    pub lpszScheme: windows_core::PWSTR,
    pub dwSchemeLength: super::DWORD,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: windows_core::PWSTR,
    pub dwHostNameLength: super::DWORD,
    pub nPort: super::INTERNET_PORT,
    pub lpszUserName: windows_core::PWSTR,
    pub dwUserNameLength: super::DWORD,
    pub lpszPassword: windows_core::PWSTR,
    pub dwPasswordLength: super::DWORD,
    pub lpszUrlPath: windows_core::PWSTR,
    pub dwUrlPathLength: super::DWORD,
    pub lpszExtraInfo: windows_core::PWSTR,
    pub dwExtraInfoLength: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct URL_COMPONENTS {
    pub dwStructSize: super::DWORD,
    pub lpszScheme: windows_core::PWSTR,
    pub dwSchemeLength: super::DWORD,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: windows_core::PWSTR,
    pub dwHostNameLength: super::DWORD,
    pub nPort: super::INTERNET_PORT,
    pub lpszUserName: windows_core::PWSTR,
    pub dwUserNameLength: super::DWORD,
    pub lpszPassword: windows_core::PWSTR,
    pub dwPasswordLength: super::DWORD,
    pub lpszUrlPath: windows_core::PWSTR,
    pub dwUrlPathLength: super::DWORD,
    pub lpszExtraInfo: windows_core::PWSTR,
    pub dwExtraInfoLength: super::DWORD,
}
#[cfg(feature = "shared")]
pub type URL_COMPONENTSW = URL_COMPONENTS;
pub const WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY: i32 = 4;
pub const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: i32 = 0;
pub const WINHTTP_ACCESS_TYPE_NAMED_PROXY: i32 = 3;
pub const WINHTTP_ACCESS_TYPE_NO_PROXY: i32 = 1;
pub const WINHTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760;
pub const WINHTTP_ADDREQ_FLAG_ADD: i32 = 536870912;
pub const WINHTTP_ADDREQ_FLAG_ADD_IF_NEW: i32 = 268435456;
pub const WINHTTP_ADDREQ_FLAG_COALESCE: i32 = 1073741824;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: i32 = 1073741824;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: i32 = 16777216;
pub const WINHTTP_ADDREQ_FLAG_REPLACE: u32 = 2147483648;
pub const WINHTTP_ADDREQ_INDEX_MASK: i32 = 65535;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_ASYNC_RESULT {
    pub dwResult: super::DWORD_PTR,
    pub dwError: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_ASYNC_RESULT {
    pub dwResult: super::DWORD_PTR,
    pub dwError: super::DWORD,
}
pub const WINHTTP_AUTH_SCHEME_BASIC: i32 = 1;
pub const WINHTTP_AUTH_SCHEME_DIGEST: i32 = 8;
pub const WINHTTP_AUTH_SCHEME_NEGOTIATE: i32 = 16;
pub const WINHTTP_AUTH_SCHEME_NTLM: i32 = 2;
pub const WINHTTP_AUTH_SCHEME_PASSPORT: i32 = 4;
pub const WINHTTP_AUTH_TARGET_PROXY: i32 = 1;
pub const WINHTTP_AUTH_TARGET_SERVER: i32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_DEFAULT: i32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH: i32 = 2;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_LOW: i32 = 1;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MAX: i32 = 3;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MEDIUM: i32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_PROXY_ONLY: i32 = 3;
pub const WINHTTP_AUTOPROXY_ALLOW_AUTOCONFIG: i32 = 256;
pub const WINHTTP_AUTOPROXY_ALLOW_CM: i32 = 1024;
pub const WINHTTP_AUTOPROXY_ALLOW_STATIC: i32 = 512;
pub const WINHTTP_AUTOPROXY_AUTO_DETECT: i32 = 1;
pub const WINHTTP_AUTOPROXY_CONFIG_URL: i32 = 2;
pub const WINHTTP_AUTOPROXY_HOST_KEEPCASE: i32 = 4;
pub const WINHTTP_AUTOPROXY_HOST_LOWERCASE: i32 = 8;
pub const WINHTTP_AUTOPROXY_NO_CACHE_CLIENT: i32 = 524288;
pub const WINHTTP_AUTOPROXY_NO_CACHE_SVC: i32 = 1048576;
pub const WINHTTP_AUTOPROXY_NO_DIRECTACCESS: i32 = 262144;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dwFlags: super::DWORD,
    pub dwAutoDetectFlags: super::DWORD,
    pub lpszAutoConfigUrl: windows_core::PCWSTR,
    pub lpvReserved: super::LPVOID,
    pub dwReserved: super::DWORD,
    pub fAutoLogonIfChallenged: windows_core::BOOL,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dwFlags: super::DWORD,
    pub dwAutoDetectFlags: super::DWORD,
    pub lpszAutoConfigUrl: windows_core::PCWSTR,
    pub lpvReserved: super::LPVOID,
    pub dwReserved: super::DWORD,
    pub fAutoLogonIfChallenged: windows_core::BOOL,
}
pub const WINHTTP_AUTOPROXY_RUN_INPROCESS: i32 = 65536;
pub const WINHTTP_AUTOPROXY_RUN_OUTPROCESS_ONLY: i32 = 131072;
pub const WINHTTP_AUTOPROXY_SORT_RESULTS: i32 = 4194304;
pub const WINHTTP_AUTOPROXY_USE_INTERFACE_CONFIG: i32 = 2048;
pub const WINHTTP_AUTO_DETECT_TYPE_DHCP: i32 = 1;
pub const WINHTTP_AUTO_DETECT_TYPE_DNS_A: i32 = 2;
pub const WINHTTP_CALLBACK_FLAG_ALL_COMPLETIONS: i32 = 159252480;
pub const WINHTTP_CALLBACK_FLAG_ALL_NOTIFICATIONS: u32 = 4294967295;
pub const WINHTTP_CALLBACK_FLAG_CLOSE_CONNECTION: i32 = 768;
pub const WINHTTP_CALLBACK_FLAG_CONNECT_TO_SERVER: i32 = 12;
pub const WINHTTP_CALLBACK_FLAG_DATA_AVAILABLE: i32 = 262144;
pub const WINHTTP_CALLBACK_FLAG_DETECTING_PROXY: i32 = 4096;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYFORURL_COMPLETE: i32 = 16777216;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYSETTINGS_COMPLETE: i32 = 134217728;
pub const WINHTTP_CALLBACK_FLAG_HANDLES: i32 = 3072;
pub const WINHTTP_CALLBACK_FLAG_HEADERS_AVAILABLE: i32 = 131072;
pub const WINHTTP_CALLBACK_FLAG_INTERMEDIATE_RESPONSE: i32 = 32768;
pub const WINHTTP_CALLBACK_FLAG_READ_COMPLETE: i32 = 524288;
pub const WINHTTP_CALLBACK_FLAG_RECEIVE_RESPONSE: i32 = 192;
pub const WINHTTP_CALLBACK_FLAG_REDIRECT: i32 = 16384;
pub const WINHTTP_CALLBACK_FLAG_REQUEST_ERROR: i32 = 2097152;
pub const WINHTTP_CALLBACK_FLAG_RESOLVE_NAME: i32 = 3;
pub const WINHTTP_CALLBACK_FLAG_SECURE_FAILURE: i32 = 65536;
pub const WINHTTP_CALLBACK_FLAG_SENDREQUEST_COMPLETE: i32 = 4194304;
pub const WINHTTP_CALLBACK_FLAG_SEND_REQUEST: i32 = 48;
pub const WINHTTP_CALLBACK_FLAG_WRITE_COMPLETE: i32 = 1048576;
pub const WINHTTP_CALLBACK_STATUS_CLOSE_COMPLETE: i32 = 33554432;
pub const WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION: i32 = 256;
pub const WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER: i32 = 8;
pub const WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER: i32 = 4;
pub const WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED: i32 = 512;
pub const WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE: i32 = 262144;
pub const WINHTTP_CALLBACK_STATUS_DETECTING_PROXY: i32 = 4096;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_CN_INVALID: i32 = 16;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_DATE_INVALID: i32 = 32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REVOKED: i32 = 4;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REV_FAILED: i32 = 1;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_WRONG_USAGE: i32 = 64;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CA: i32 = 8;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CERT: i32 = 2;
pub const WINHTTP_CALLBACK_STATUS_FLAG_SECURITY_CHANNEL_ERROR: u32 = 2147483648;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE: i32 = 16777216;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE: i32 = 134217728;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING: i32 = 2048;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CREATED: i32 = 1024;
pub const WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE: i32 = 131072;
pub const WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE: i32 = 32768;
pub const WINHTTP_CALLBACK_STATUS_NAME_RESOLVED: i32 = 2;
pub const WINHTTP_CALLBACK_STATUS_READ_COMPLETE: i32 = 524288;
pub const WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE: i32 = 64;
pub const WINHTTP_CALLBACK_STATUS_REDIRECT: i32 = 16384;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_ERROR: i32 = 2097152;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_SENT: i32 = 32;
pub const WINHTTP_CALLBACK_STATUS_RESOLVING_NAME: i32 = 1;
pub const WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED: i32 = 128;
pub const WINHTTP_CALLBACK_STATUS_SECURE_FAILURE: i32 = 65536;
pub const WINHTTP_CALLBACK_STATUS_SENDING_REQUEST: i32 = 16;
pub const WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE: i32 = 4194304;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_READ_COMPLETE: i32 = 536870912;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_WRITE_COMPLETE: i32 = 268435456;
pub const WINHTTP_CALLBACK_STATUS_SHUTDOWN_COMPLETE: i32 = 67108864;
pub const WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE: i32 = 1048576;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CERTIFICATE_INFO {
    pub ftExpiry: super::FILETIME,
    pub ftStart: super::FILETIME,
    pub lpszSubjectInfo: windows_core::PWSTR,
    pub lpszIssuerInfo: windows_core::PWSTR,
    pub lpszProtocolName: windows_core::PWSTR,
    pub lpszSignatureAlgName: windows_core::PWSTR,
    pub lpszEncryptionAlgName: windows_core::PWSTR,
    pub dwKeySize: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CERTIFICATE_INFO {
    pub ftExpiry: super::FILETIME,
    pub ftStart: super::FILETIME,
    pub lpszSubjectInfo: windows_core::PWSTR,
    pub lpszIssuerInfo: windows_core::PWSTR,
    pub lpszProtocolName: windows_core::PWSTR,
    pub lpszSignatureAlgName: windows_core::PWSTR,
    pub lpszEncryptionAlgName: windows_core::PWSTR,
    pub dwKeySize: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_GROUP {
    pub cConnections: super::ULONG,
    pub guidGroup: windows_core::GUID,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_GROUP {
    pub cConnections: super::ULONG,
    pub guidGroup: windows_core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: super::DWORD,
    pub LocalAddress: super::SOCKADDR_STORAGE,
    pub RemoteAddress: super::SOCKADDR_STORAGE,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: super::DWORD,
    pub LocalAddress: super::SOCKADDR_STORAGE,
    pub RemoteAddress: super::SOCKADDR_STORAGE,
}
pub const WINHTTP_CONNECTION_RETRY_CONDITION_408: i32 = 1;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_MASK: i32 = 7;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE: i32 = 2;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION: i32 = 4;
pub const WINHTTP_CONNS_PER_SERVER_UNLIMITED: u32 = 4294967295;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CREDS {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: super::DWORD,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CREDS {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: super::DWORD,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CREDS_EX {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: super::DWORD,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: super::DWORD,
    pub lpszUrl: windows_core::PSTR,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CREDS_EX {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: super::DWORD,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: super::DWORD,
    pub lpszUrl: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub fAutoDetect: windows_core::BOOL,
    pub lpszAutoConfigUrl: windows_core::PWSTR,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub fAutoDetect: windows_core::BOOL,
    pub lpszAutoConfigUrl: windows_core::PWSTR,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
pub const WINHTTP_DECOMPRESSION_FLAG_ALL: i32 = 3;
pub const WINHTTP_DECOMPRESSION_FLAG_DEFLATE: i32 = 2;
pub const WINHTTP_DECOMPRESSION_FLAG_GZIP: i32 = 1;
pub const WINHTTP_DEFAULT_ACCEPT_TYPES: i32 = 0;
pub const WINHTTP_DISABLE_AUTHENTICATION: i32 = 4;
pub const WINHTTP_DISABLE_COOKIES: i32 = 1;
pub const WINHTTP_DISABLE_KEEP_ALIVE: i32 = 8;
pub const WINHTTP_DISABLE_PASSPORT_AUTH: i32 = 0;
pub const WINHTTP_DISABLE_PASSPORT_KEYRING: i32 = 536870912;
pub const WINHTTP_DISABLE_REDIRECTS: i32 = 2;
pub const WINHTTP_DISABLE_SPN_SERVER_PORT: i32 = 0;
pub const WINHTTP_ENABLE_PASSPORT_AUTH: i32 = 268435456;
pub const WINHTTP_ENABLE_PASSPORT_KEYRING: i32 = 1073741824;
pub const WINHTTP_ENABLE_SPN_SERVER_PORT: i32 = 1;
pub const WINHTTP_ENABLE_SSL_REVERT_IMPERSONATION: i32 = 2;
pub const WINHTTP_ENABLE_SSL_REVOCATION: i32 = 1;
pub const WINHTTP_ERROR_BASE: i32 = 12000;
pub const WINHTTP_ERROR_LAST: i32 = 12193;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_EXTENDED_HEADER {
    pub Anonymous: WINHTTP_EXTENDED_HEADER_0,
    pub Anonymous2: WINHTTP_EXTENDED_HEADER_1,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_EXTENDED_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_0 {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_EXTENDED_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_1 {
    pub pwszValue: windows_core::PCWSTR,
    pub pszValue: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_EXTENDED_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WINHTTP_EXTENDED_HEADER {
    pub Anonymous: WINHTTP_EXTENDED_HEADER_0,
    pub Anonymous2: WINHTTP_EXTENDED_HEADER_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_EXTENDED_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_0 {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_EXTENDED_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_1 {
    pub pwszValue: windows_core::PCWSTR,
    pub pszValue: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_EXTENDED_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_EXTENDED_HEADER_FLAG_UNICODE: i32 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAILED_CONNECTION_RETRIES {
    pub dwMaxRetries: super::DWORD,
    pub dwAllowedRetryConditions: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAILED_CONNECTION_RETRIES {
    pub dwMaxRetries: super::DWORD,
    pub dwAllowedRetryConditions: super::DWORD,
}
pub type WINHTTP_FAST_FORWARDING_STATE = i32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAST_FORWARDING_STATUS {
    pub TransferState: WINHTTP_FAST_FORWARDING_STATE,
    pub NtStatus: super::LONG,
    pub dwError: super::DWORD,
    pub ullBytesTransferred: super::ULONGLONG,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAST_FORWARDING_STATUS {
    pub TransferState: WINHTTP_FAST_FORWARDING_STATE,
    pub NtStatus: super::LONG,
    pub dwError: super::DWORD,
    pub ullBytesTransferred: super::ULONGLONG,
}
pub const WINHTTP_FEATURE_ADD_REQUEST_HEADERS_EX: i32 = 46;
pub const WINHTTP_FEATURE_BACKGROUND_CONNECTIONS: i32 = 34;
pub const WINHTTP_FEATURE_CONNECTION_GUID: i32 = 38;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V0: i32 = 3;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V1: i32 = 12;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V2: i32 = 89;
pub const WINHTTP_FEATURE_DISABLE_AIA_FLAG: i32 = 91;
pub const WINHTTP_FEATURE_DISABLE_CERT_CHAIN_BUILDING: i32 = 33;
pub const WINHTTP_FEATURE_DISABLE_GLOBAL_POOLING: i32 = 76;
pub const WINHTTP_FEATURE_DISABLE_PROXY_AUTH_SCHEMES: i32 = 74;
pub const WINHTTP_FEATURE_DISABLE_SECURE_PROTOCOL_FALLBACK: i32 = 6;
pub const WINHTTP_FEATURE_DISABLE_STREAM_QUEUE: i32 = 1;
pub const WINHTTP_FEATURE_DSCP_TAG: i32 = 92;
pub const WINHTTP_FEATURE_ENABLE_HTTP2_PLUS_CLIENT_CERT: i32 = 23;
pub const WINHTTP_FEATURE_ERROR_LOG_GUID: i32 = 83;
pub const WINHTTP_FEATURE_EXPIRE_CONNECTION: i32 = 5;
pub const WINHTTP_FEATURE_EXTENDED_HEADER_FLAG_UNICODE: i32 = 54;
pub const WINHTTP_FEATURE_FAILED_CONNECTION_RETRIES: i32 = 24;
pub const WINHTTP_FEATURE_FAST_FORWARD_RESPONSE: i32 = 90;
pub const WINHTTP_FEATURE_FIRST_AVAILABLE_CONNECTION: i32 = 35;
pub const WINHTTP_FEATURE_FLAG_AUTOMATIC_CHUNKING: i32 = 59;
pub const WINHTTP_FEATURE_FLAG_SECURE_DEFAULTS: i32 = 53;
pub const WINHTTP_FEATURE_FREE_QUERY_CONNECTION_GROUP_RESULT: i32 = 51;
pub const WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX: i32 = 77;
pub const WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX_XBOX: i32 = 95;
pub const WINHTTP_FEATURE_HTTP11_DOWNGRADE_TTL: i32 = 93;
pub const WINHTTP_FEATURE_HTTP2_KEEPALIVE: i32 = 26;
pub const WINHTTP_FEATURE_HTTP2_PLUS_TRANSFER_ENCODING: i32 = 31;
pub const WINHTTP_FEATURE_HTTP2_RECEIVE_WINDOW: i32 = 43;
pub const WINHTTP_FEATURE_HTTP3_HANDSHAKE_TIMEOUT: i32 = 70;
pub const WINHTTP_FEATURE_HTTP3_INITIAL_RTT: i32 = 71;
pub const WINHTTP_FEATURE_HTTP3_KEEPALIVE: i32 = 69;
pub const WINHTTP_FEATURE_HTTP3_STREAM_ERROR_CODE: i32 = 72;
pub const WINHTTP_FEATURE_HTTP_PROTOCOL_REQUIRED: i32 = 7;
pub const WINHTTP_FEATURE_IGNORE_CERT_REVOCATION_OFFLINE: i32 = 17;
pub const WINHTTP_FEATURE_IPV6_FAST_FALLBACK: i32 = 2;
pub const WINHTTP_FEATURE_IS_FEATURE_SUPPORTED: i32 = 44;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID: i32 = 39;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: i32 = 61;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP: i32 = 50;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP_FLAG_INSECURE: i32 = 60;
pub const WINHTTP_FEATURE_QUERY_EX_ALL_HEADERS: i32 = 62;
pub const WINHTTP_FEATURE_QUERY_FLAG_TRAILERS: i32 = 55;
pub const WINHTTP_FEATURE_QUERY_FLAG_WIRE_ENCODING: i32 = 56;
pub const WINHTTP_FEATURE_QUERY_HEADERS_EX: i32 = 49;
pub const WINHTTP_FEATURE_QUIC_STATS: i32 = 66;
pub const WINHTTP_FEATURE_QUIC_STATS_V2: i32 = 79;
pub const WINHTTP_FEATURE_QUIC_STREAM_STATS: i32 = 81;
pub const WINHTTP_FEATURE_READ_DATA_EX: i32 = 48;
pub const WINHTTP_FEATURE_READ_DATA_EX_FLAG_FILL_BUFFER: i32 = 63;
pub const WINHTTP_FEATURE_REFERER_TOKEN_BINDING_HOSTNAME: i32 = 30;
pub const WINHTTP_FEATURE_REQUEST_ANNOTATION: i32 = 73;
pub const WINHTTP_FEATURE_REQUEST_STATS: i32 = 8;
pub const WINHTTP_FEATURE_REQUEST_TIMES: i32 = 4;
pub const WINHTTP_FEATURE_REQUIRE_STREAM_END: i32 = 22;
pub const WINHTTP_FEATURE_RESOLUTION_HOSTNAME: i32 = 27;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG: i32 = 32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: i32 = 58;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: i32 = 65;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: i32 = 57;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: i32 = 64;
pub const WINHTTP_FEATURE_REVERT_IMPERSONATION_SERVER_CERT: i32 = 75;
pub const WINHTTP_FEATURE_SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS: i32 = 52;
pub const WINHTTP_FEATURE_SECURITY_INFO: i32 = 13;
pub const WINHTTP_FEATURE_SERVER_CERT_CHAIN_CONTEXT: i32 = 9;
pub const WINHTTP_FEATURE_SESSION_ERROR_LOG_GUID: i32 = 94;
pub const WINHTTP_FEATURE_SESSION_SCH_CRED: i32 = 78;
pub const WINHTTP_FEATURE_SET_PROXY_SETINGS_PER_USER: i32 = 47;
pub const WINHTTP_FEATURE_SET_TOKEN_BINDING: i32 = 28;
pub const WINHTTP_FEATURE_STREAM_ERROR_CODE: i32 = 21;
pub const WINHTTP_FEATURE_TCP_FAST_OPEN: i32 = 15;
pub const WINHTTP_FEATURE_TCP_KEEPALIVE: i32 = 14;
pub const WINHTTP_FEATURE_TCP_PRIORITY_STATUS: i32 = 37;
pub const WINHTTP_FEATURE_TLS_FALSE_START: i32 = 16;
pub const WINHTTP_FEATURE_TLS_PROTOCOL_INSECURE_FALLBACK: i32 = 20;
pub const WINHTTP_FEATURE_TOKEN_BINDING_PUBLIC_KEY: i32 = 29;
pub const WINHTTP_FEATURE_UPGRADE_TO_PROTOCOL: i32 = 88;
pub const WINHTTP_FEATURE_URL_INCLUDE_DEFAULT_PORT: i32 = 80;
pub const WINHTTP_FEATURE_USE_LOOKASIDE: i32 = 82;
pub const WINHTTP_FIRST_OPTION: i32 = 1;
pub const WINHTTP_FLAG_ASYNC: i32 = 268435456;
pub const WINHTTP_FLAG_AUTOMATIC_CHUNKING: i32 = 512;
pub const WINHTTP_FLAG_BYPASS_PROXY_CACHE: i32 = 256;
pub const WINHTTP_FLAG_ESCAPE_DISABLE: i32 = 64;
pub const WINHTTP_FLAG_ESCAPE_DISABLE_QUERY: i32 = 128;
pub const WINHTTP_FLAG_ESCAPE_PERCENT: i32 = 4;
pub const WINHTTP_FLAG_NULL_CODEPAGE: i32 = 8;
pub const WINHTTP_FLAG_REFRESH: i32 = 256;
pub const WINHTTP_FLAG_SECURE: i32 = 8388608;
pub const WINHTTP_FLAG_SECURE_DEFAULTS: i32 = 805306368;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_ALL: i32 = 168;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL2: i32 = 8;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL3: i32 = 32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1: i32 = 128;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_1: i32 = 512;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_2: i32 = 2048;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_3: i32 = 8192;
pub const WINHTTP_HANDLE_TYPE_CONNECT: i32 = 2;
pub const WINHTTP_HANDLE_TYPE_PROTOCOL: i32 = 6;
pub const WINHTTP_HANDLE_TYPE_PROXY_RESOLVER: i32 = 4;
pub const WINHTTP_HANDLE_TYPE_REQUEST: i32 = 3;
pub const WINHTTP_HANDLE_TYPE_SESSION: i32 = 1;
pub const WINHTTP_HANDLE_TYPE_WEBSOCKET: i32 = 5;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union WINHTTP_HEADER_NAME {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WINHTTP_HEADER_NAME {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_HEADER_NAME_BY_INDEX: i32 = 0;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_HOST_CONNECTION_GROUP {
    pub pwszHost: windows_core::PCWSTR,
    pub cConnectionGroups: super::ULONG,
    pub pConnectionGroups: PWINHTTP_CONNECTION_GROUP,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_HOST_CONNECTION_GROUP {
    pub pwszHost: windows_core::PCWSTR,
    pub cConnectionGroups: super::ULONG,
    pub pConnectionGroups: PWINHTTP_CONNECTION_GROUP,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_HTTP2_RECEIVE_WINDOW {
    pub ulStreamWindow: super::ULONG,
    pub ulStreamWindowUpdateDelta: super::ULONG,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_HTTP2_RECEIVE_WINDOW {
    pub ulStreamWindow: super::ULONG,
    pub ulStreamWindowUpdateDelta: super::ULONG,
}
pub const WINHTTP_IGNORE_REQUEST_TOTAL_LENGTH: i32 = 0;
pub const WINHTTP_LAST_OPTION: i32 = 212;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_core::GUID,
    pub ullFlags: super::ULONGLONG,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_core::GUID,
    pub ullFlags: super::ULONGLONG,
}
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAGS_MASK: i32 = 1;
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: i32 = 1;
pub const WINHTTP_NO_ADDITIONAL_HEADERS: i32 = 0;
pub const WINHTTP_NO_CLIENT_CERT_CONTEXT: i32 = 0;
pub const WINHTTP_NO_HEADER_INDEX: i32 = 0;
pub const WINHTTP_NO_OUTPUT_BUFFER: i32 = 0;
pub const WINHTTP_NO_PROXY_BYPASS: i32 = 0;
pub const WINHTTP_NO_PROXY_NAME: i32 = 0;
pub const WINHTTP_NO_REFERER: i32 = 0;
pub const WINHTTP_NO_REQUEST_DATA: i32 = 0;
pub const WINHTTP_OPTION_ASSURED_NON_BLOCKING_CALLBACKS: i32 = 111;
pub const WINHTTP_OPTION_AUTOLOGON_POLICY: i32 = 77;
pub const WINHTTP_OPTION_BACKGROUND_CONNECTIONS: i32 = 172;
pub const WINHTTP_OPTION_CALLBACK: i32 = 1;
pub const WINHTTP_OPTION_CLIENT_CERT_CONTEXT: i32 = 47;
pub const WINHTTP_OPTION_CLIENT_CERT_ISSUER_LIST: i32 = 94;
pub const WINHTTP_OPTION_CODEPAGE: i32 = 68;
pub const WINHTTP_OPTION_CONFIGURE_PASSPORT_AUTH: i32 = 83;
pub const WINHTTP_OPTION_CONNECTION_FILTER: i32 = 131;
pub const WINHTTP_OPTION_CONNECTION_GUID: i32 = 178;
pub const WINHTTP_OPTION_CONNECTION_INFO: i32 = 93;
pub const WINHTTP_OPTION_CONNECTION_STATS_V0: i32 = 141;
pub const WINHTTP_OPTION_CONNECTION_STATS_V1: i32 = 150;
pub const WINHTTP_OPTION_CONNECTION_STATS_V2: i32 = 208;
pub const WINHTTP_OPTION_CONNECT_RETRIES: i32 = 4;
pub const WINHTTP_OPTION_CONNECT_TIMEOUT: i32 = 3;
pub const WINHTTP_OPTION_CONTEXT_VALUE: i32 = 45;
pub const WINHTTP_OPTION_DECOMPRESSION: i32 = 118;
pub const WINHTTP_OPTION_DISABLE_CERT_CHAIN_BUILDING: i32 = 171;
pub const WINHTTP_OPTION_DISABLE_FEATURE: i32 = 63;
pub const WINHTTP_OPTION_DISABLE_GLOBAL_POOLING: i32 = 195;
pub const WINHTTP_OPTION_DISABLE_PROXY_AUTH_SCHEMES: i32 = 193;
pub const WINHTTP_OPTION_DISABLE_SECURE_PROTOCOL_FALLBACK: i32 = 144;
pub const WINHTTP_OPTION_DISABLE_STREAM_QUEUE: i32 = 139;
pub const WINHTTP_OPTION_DSCP_TAG: i32 = 210;
pub const WINHTTP_OPTION_ENABLETRACING: i32 = 85;
pub const WINHTTP_OPTION_ENABLE_FAST_FORWARDING: i32 = 205;
pub const WINHTTP_OPTION_ENABLE_FEATURE: i32 = 79;
pub const WINHTTP_OPTION_ENABLE_HTTP2_PLUS_CLIENT_CERT: i32 = 161;
pub const WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL: i32 = 133;
pub const WINHTTP_OPTION_ENCODE_EXTRA: i32 = 138;
pub const WINHTTP_OPTION_ERROR_LOG_GUID: i32 = 204;
pub const WINHTTP_OPTION_EXPIRE_CONNECTION: i32 = 143;
pub const WINHTTP_OPTION_EXTENDED_ERROR: i32 = 24;
pub const WINHTTP_OPTION_FAILED_CONNECTION_RETRIES: i32 = 162;
pub const WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_DATA: i32 = 206;
pub const WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_STATUS: i32 = 209;
pub const WINHTTP_OPTION_FEATURE_SUPPORTED: i32 = 184;
pub const WINHTTP_OPTION_FIRST_AVAILABLE_CONNECTION: i32 = 173;
pub const WINHTTP_OPTION_GLOBAL_PROXY_CREDS: i32 = 97;
pub const WINHTTP_OPTION_GLOBAL_SERVER_CREDS: i32 = 98;
pub const WINHTTP_OPTION_HANDLE_TYPE: i32 = 9;
pub const WINHTTP_OPTION_HTTP11_DOWNGRADE_TTL: i32 = 211;
pub const WINHTTP_OPTION_HTTP2_KEEPALIVE: i32 = 164;
pub const WINHTTP_OPTION_HTTP2_PLUS_TRANSFER_ENCODING: i32 = 169;
pub const WINHTTP_OPTION_HTTP2_RECEIVE_WINDOW: i32 = 183;
pub const WINHTTP_OPTION_HTTP3_HANDSHAKE_TIMEOUT: i32 = 189;
pub const WINHTTP_OPTION_HTTP3_INITIAL_RTT: i32 = 190;
pub const WINHTTP_OPTION_HTTP3_KEEPALIVE: i32 = 188;
pub const WINHTTP_OPTION_HTTP3_STREAM_ERROR_CODE: i32 = 191;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_REQUIRED: i32 = 145;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_USED: i32 = 134;
pub const WINHTTP_OPTION_HTTP_VERSION: i32 = 59;
pub const WINHTTP_OPTION_IGNORE_CERT_REVOCATION_OFFLINE: i32 = 155;
pub const WINHTTP_OPTION_IPV6_FAST_FALLBACK: i32 = 140;
pub const WINHTTP_OPTION_IS_PROXY_CONNECT_RESPONSE: i32 = 104;
pub const WINHTTP_OPTION_KDC_PROXY_SETTINGS: i32 = 136;
pub const WINHTTP_OPTION_MATCH_CONNECTION_GUID: i32 = 179;
pub const WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER: i32 = 74;
pub const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: i32 = 73;
pub const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: i32 = 89;
pub const WINHTTP_OPTION_MAX_HTTP_STATUS_CONTINUE: i32 = 90;
pub const WINHTTP_OPTION_MAX_RESPONSE_DRAIN_SIZE: i32 = 92;
pub const WINHTTP_OPTION_MAX_RESPONSE_HEADER_SIZE: i32 = 91;
pub const WINHTTP_OPTION_NETWORK_INTERFACE_AFFINITY: i32 = 105;
pub const WINHTTP_OPTION_PARENT_HANDLE: i32 = 21;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_TEXT: i32 = 81;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_URL: i32 = 82;
pub const WINHTTP_OPTION_PASSPORT_RETURN_URL: i32 = 87;
pub const WINHTTP_OPTION_PASSPORT_SIGN_OUT: i32 = 86;
pub const WINHTTP_OPTION_PASSWORD: i32 = 4097;
pub const WINHTTP_OPTION_PROXY: i32 = 38;
pub const WINHTTP_OPTION_PROXY_DISABLE_SERVICE_CALLS: i32 = 137;
pub const WINHTTP_OPTION_PROXY_PASSWORD: i32 = 4099;
pub const WINHTTP_OPTION_PROXY_RESULT_ENTRY: i32 = 39;
pub const WINHTTP_OPTION_PROXY_SPN_USED: i32 = 107;
pub const WINHTTP_OPTION_PROXY_USERNAME: i32 = 4098;
pub const WINHTTP_OPTION_QUIC_STATS: i32 = 185;
pub const WINHTTP_OPTION_QUIC_STATS_V2: i32 = 200;
pub const WINHTTP_OPTION_QUIC_STREAM_STATS: i32 = 202;
pub const WINHTTP_OPTION_READ_BUFFER_SIZE: i32 = 12;
pub const WINHTTP_OPTION_RECEIVE_PROXY_CONNECT_RESPONSE: i32 = 103;
pub const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: i32 = 7;
pub const WINHTTP_OPTION_RECEIVE_TIMEOUT: i32 = 6;
pub const WINHTTP_OPTION_REDIRECT_POLICY: i32 = 88;
pub const WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS: i32 = 2;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DEFAULT: i32 = 1;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP: i32 = 1;
pub const WINHTTP_OPTION_REDIRECT_POLICY_LAST: i32 = 2;
pub const WINHTTP_OPTION_REDIRECT_POLICY_NEVER: i32 = 0;
pub const WINHTTP_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: i32 = 168;
pub const WINHTTP_OPTION_REJECT_USERPWD_IN_URL: i32 = 100;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION: i32 = 192;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: i32 = 64000;
pub const WINHTTP_OPTION_REQUEST_PRIORITY: i32 = 58;
pub const WINHTTP_OPTION_REQUEST_STATS: i32 = 146;
pub const WINHTTP_OPTION_REQUEST_TIMES: i32 = 142;
pub const WINHTTP_OPTION_REQUIRE_STREAM_END: i32 = 160;
pub const WINHTTP_OPTION_RESOLUTION_HOSTNAME: i32 = 165;
pub const WINHTTP_OPTION_RESOLVER_CACHE_CONFIG: i32 = 170;
pub const WINHTTP_OPTION_RESOLVE_TIMEOUT: i32 = 2;
pub const WINHTTP_OPTION_REVERT_IMPERSONATION_SERVER_CERT: i32 = 194;
pub const WINHTTP_OPTION_SECURE_PROTOCOLS: i32 = 84;
pub const WINHTTP_OPTION_SECURITY_CERTIFICATE_STRUCT: i32 = 32;
pub const WINHTTP_OPTION_SECURITY_FLAGS: i32 = 31;
pub const WINHTTP_OPTION_SECURITY_INFO: i32 = 151;
pub const WINHTTP_OPTION_SECURITY_KEY_BITNESS: i32 = 36;
pub const WINHTTP_OPTION_SEND_TIMEOUT: i32 = 5;
pub const WINHTTP_OPTION_SERVER_CBT: i32 = 108;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_CACHE_ONLY: i32 = 199;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_FLAGS: i32 = 148;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_CONTEXT: i32 = 147;
pub const WINHTTP_OPTION_SERVER_CERT_CONTEXT: i32 = 78;
pub const WINHTTP_OPTION_SERVER_SPN_USED: i32 = 106;
pub const WINHTTP_OPTION_SESSION_ERROR_LOG_GUID: i32 = 212;
pub const WINHTTP_OPTION_SET_TOKEN_BINDING: i32 = 166;
pub const WINHTTP_OPTION_SPN: i32 = 96;
pub const WINHTTP_OPTION_SPN_MASK: i32 = 1;
pub const WINHTTP_OPTION_STREAM_ERROR_CODE: i32 = 159;
pub const WINHTTP_OPTION_TCP_FAST_OPEN: i32 = 153;
pub const WINHTTP_OPTION_TCP_KEEPALIVE: i32 = 152;
pub const WINHTTP_OPTION_TCP_PRIORITY_HINT: i32 = 128;
pub const WINHTTP_OPTION_TCP_PRIORITY_STATUS: i32 = 177;
pub const WINHTTP_OPTION_TLS_FALSE_START: i32 = 154;
pub const WINHTTP_OPTION_TLS_PROTOCOL_INSECURE_FALLBACK: i32 = 158;
pub const WINHTTP_OPTION_TOKEN_BINDING_PUBLIC_KEY: i32 = 167;
pub const WINHTTP_OPTION_UNLOAD_NOTIFY_EVENT: i32 = 99;
pub const WINHTTP_OPTION_UNSAFE_HEADER_PARSING: i32 = 110;
pub const WINHTTP_OPTION_UPGRADE_TO_PROTOCOL: i32 = 207;
pub const WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET: i32 = 114;
pub const WINHTTP_OPTION_URL: i32 = 34;
pub const WINHTTP_OPTION_USERNAME: i32 = 4096;
pub const WINHTTP_OPTION_USER_AGENT: i32 = 41;
pub const WINHTTP_OPTION_USE_GLOBAL_SERVER_CREDENTIALS: i32 = 101;
pub const WINHTTP_OPTION_USE_LOOKASIDE: i32 = 203;
pub const WINHTTP_OPTION_USE_SESSION_SCH_CRED: i32 = 196;
pub const WINHTTP_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: i32 = 115;
pub const WINHTTP_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: i32 = 116;
pub const WINHTTP_OPTION_WEB_SOCKET_RECEIVE_BUFFER_SIZE: i32 = 122;
pub const WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE: i32 = 123;
pub const WINHTTP_OPTION_WORKER_THREAD_COUNT: i32 = 80;
pub const WINHTTP_OPTION_WRITE_BUFFER_SIZE: i32 = 13;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROTOCOL_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_PROTOCOL_OPERATION,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROTOCOL_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_PROTOCOL_OPERATION,
}
pub const WINHTTP_PROTOCOL_FLAG_HTTP2: i32 = 1;
pub const WINHTTP_PROTOCOL_FLAG_HTTP3: i32 = 2;
pub const WINHTTP_PROTOCOL_MASK: i32 = 3;
pub type WINHTTP_PROTOCOL_OPERATION = i32;
pub const WINHTTP_PROTOCOL_RECEIVE_OPERATION: WINHTTP_PROTOCOL_OPERATION = 1;
pub const WINHTTP_PROTOCOL_SEND_OPERATION: WINHTTP_PROTOCOL_OPERATION = 0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WINHTTP_PROXY_CHANGE_CALLBACK = Option<unsafe extern "system" fn(ullflags: super::ULONGLONG, pvcontext: super::PVOID)>;
pub type WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE = *mut core::ffi::c_void;
pub const WINHTTP_PROXY_DISABLE_AUTH_LOCAL_SERVICE: i32 = 256;
pub const WINHTTP_PROXY_DISABLE_SCHEME_BASIC: i32 = 1;
pub const WINHTTP_PROXY_DISABLE_SCHEME_DIGEST: i32 = 2;
pub const WINHTTP_PROXY_DISABLE_SCHEME_KERBEROS: i32 = 8;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NEGOTIATE: i32 = 16;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NTLM: i32 = 4;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_INFO {
    pub dwAccessType: super::DWORD,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_INFO {
    pub dwAccessType: super::DWORD,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
#[cfg(feature = "shared")]
pub type WINHTTP_PROXY_INFOW = WINHTTP_PROXY_INFO;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_NETWORKING_KEY {
    pub pbBuffer: [u8; 128],
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_NETWORKING_KEY {
    pub pbBuffer: [u8; 128],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_PROXY_NOTIFY_CHANGE: i32 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT {
    pub cEntries: super::DWORD,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT {
    pub cEntries: super::DWORD,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT_ENTRY {
    pub fProxy: windows_core::BOOL,
    pub fBypass: windows_core::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: windows_core::PWSTR,
    pub ProxyPort: super::INTERNET_PORT,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT_ENTRY {
    pub fProxy: windows_core::BOOL,
    pub fBypass: windows_core::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: windows_core::PWSTR,
    pub ProxyPort: super::INTERNET_PORT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT_EX {
    pub cEntries: super::DWORD,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
    pub hProxyDetectionHandle: super::HANDLE,
    pub dwProxyInterfaceAffinity: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_RESULT_EX {
    pub cEntries: super::DWORD,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
    pub hProxyDetectionHandle: super::HANDLE,
    pub dwProxyInterfaceAffinity: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS {
    pub dwStructSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCurrentSettingsVersion: super::DWORD,
    pub pwszConnectionName: windows_core::PWSTR,
    pub pwszProxy: windows_core::PWSTR,
    pub pwszProxyBypass: windows_core::PWSTR,
    pub pwszAutoconfigUrl: windows_core::PWSTR,
    pub pwszAutoconfigSecondaryUrl: windows_core::PWSTR,
    pub dwAutoDiscoveryFlags: super::DWORD,
    pub pwszLastKnownGoodAutoConfigUrl: windows_core::PWSTR,
    pub dwAutoconfigReloadDelayMins: super::DWORD,
    pub ftLastKnownDetectTime: super::FILETIME,
    pub dwDetectedInterfaceIpCount: super::DWORD,
    pub pdwDetectedInterfaceIp: super::PDWORD,
    pub cNetworkKeys: super::DWORD,
    pub pNetworkKeys: PWINHTTP_PROXY_NETWORKING_KEY,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS {
    pub dwStructSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCurrentSettingsVersion: super::DWORD,
    pub pwszConnectionName: windows_core::PWSTR,
    pub pwszProxy: windows_core::PWSTR,
    pub pwszProxyBypass: windows_core::PWSTR,
    pub pwszAutoconfigUrl: windows_core::PWSTR,
    pub pwszAutoconfigSecondaryUrl: windows_core::PWSTR,
    pub dwAutoDiscoveryFlags: super::DWORD,
    pub pwszLastKnownGoodAutoConfigUrl: windows_core::PWSTR,
    pub dwAutoconfigReloadDelayMins: super::DWORD,
    pub ftLastKnownDetectTime: super::FILETIME,
    pub dwDetectedInterfaceIpCount: super::DWORD,
    pub pdwDetectedInterfaceIp: super::PDWORD,
    pub cNetworkKeys: super::DWORD,
    pub pNetworkKeys: PWINHTTP_PROXY_NETWORKING_KEY,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: super::ULONGLONG,
    pub ullFlags: super::ULONGLONG,
    pub pcwszAutoconfigUrl: windows_core::PCWSTR,
    pub pcwszProxy: windows_core::PCWSTR,
    pub pcwszSecureProxy: windows_core::PCWSTR,
    pub cProxyBypasses: super::DWORD,
    pub rgpcwszProxyBypasses: *mut windows_core::PCWSTR,
    pub dwInterfaceIndex: super::DWORD,
    pub pcwszConnectionName: windows_core::PCWSTR,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: super::ULONGLONG,
    pub ullFlags: super::ULONGLONG,
    pub pcwszAutoconfigUrl: windows_core::PCWSTR,
    pub pcwszProxy: windows_core::PCWSTR,
    pub pcwszSecureProxy: windows_core::PCWSTR,
    pub cProxyBypasses: super::DWORD,
    pub rgpcwszProxyBypasses: *mut windows_core::PCWSTR,
    pub dwInterfaceIndex: super::DWORD,
    pub pcwszConnectionName: windows_core::PCWSTR,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: super::ULONGLONG,
    pub pcwszConnectionName: windows_core::PCWSTR,
    pub pcwszProbeHost: windows_core::PCWSTR,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: super::ULONGLONG,
    pub pcwszConnectionName: windows_core::PCWSTR,
    pub pcwszProbeHost: windows_core::PCWSTR,
}
pub type WINHTTP_PROXY_SETTINGS_TYPE = i32;
pub const WINHTTP_PROXY_TYPE_AUTO_DETECT: i32 = 8;
pub const WINHTTP_PROXY_TYPE_AUTO_PROXY_URL: i32 = 4;
pub const WINHTTP_PROXY_TYPE_DIRECT: i32 = 1;
pub const WINHTTP_PROXY_TYPE_PROXY: i32 = 2;
pub const WINHTTP_QUERY_ACCEPT: i32 = 24;
pub const WINHTTP_QUERY_ACCEPT_CHARSET: i32 = 25;
pub const WINHTTP_QUERY_ACCEPT_ENCODING: i32 = 26;
pub const WINHTTP_QUERY_ACCEPT_LANGUAGE: i32 = 27;
pub const WINHTTP_QUERY_ACCEPT_RANGES: i32 = 42;
pub const WINHTTP_QUERY_AGE: i32 = 48;
pub const WINHTTP_QUERY_ALLOW: i32 = 7;
pub const WINHTTP_QUERY_AUTHENTICATION_INFO: i32 = 76;
pub const WINHTTP_QUERY_AUTHORIZATION: i32 = 28;
pub const WINHTTP_QUERY_CACHE_CONTROL: i32 = 49;
pub const WINHTTP_QUERY_CONNECTION: i32 = 23;
pub const WINHTTP_QUERY_CONNECTION_GROUP_FLAG_INSECURE: u64 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    pub cHosts: super::ULONG,
    pub pHostConnectionGroups: PWINHTTP_HOST_CONNECTION_GROUP,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    pub cHosts: super::ULONG,
    pub pHostConnectionGroups: PWINHTTP_HOST_CONNECTION_GROUP,
}
pub const WINHTTP_QUERY_CONTENT_BASE: i32 = 50;
pub const WINHTTP_QUERY_CONTENT_DESCRIPTION: i32 = 4;
pub const WINHTTP_QUERY_CONTENT_DISPOSITION: i32 = 47;
pub const WINHTTP_QUERY_CONTENT_ENCODING: i32 = 29;
pub const WINHTTP_QUERY_CONTENT_ID: i32 = 3;
pub const WINHTTP_QUERY_CONTENT_LANGUAGE: i32 = 6;
pub const WINHTTP_QUERY_CONTENT_LENGTH: i32 = 5;
pub const WINHTTP_QUERY_CONTENT_LOCATION: i32 = 51;
pub const WINHTTP_QUERY_CONTENT_MD5: i32 = 52;
pub const WINHTTP_QUERY_CONTENT_RANGE: i32 = 53;
pub const WINHTTP_QUERY_CONTENT_TRANSFER_ENCODING: i32 = 2;
pub const WINHTTP_QUERY_CONTENT_TYPE: i32 = 1;
pub const WINHTTP_QUERY_COOKIE: i32 = 44;
pub const WINHTTP_QUERY_COST: i32 = 15;
pub const WINHTTP_QUERY_CUSTOM: i32 = 65535;
pub const WINHTTP_QUERY_DATE: i32 = 9;
pub const WINHTTP_QUERY_DERIVED_FROM: i32 = 14;
pub const WINHTTP_QUERY_ETAG: i32 = 54;
pub const WINHTTP_QUERY_EXPECT: i32 = 68;
pub const WINHTTP_QUERY_EXPIRES: i32 = 10;
pub const WINHTTP_QUERY_EX_ALL_HEADERS: i32 = 21;
pub const WINHTTP_QUERY_FLAG_NUMBER: i32 = 536870912;
pub const WINHTTP_QUERY_FLAG_NUMBER64: i32 = 134217728;
pub const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648;
pub const WINHTTP_QUERY_FLAG_SYSTEMTIME: i32 = 1073741824;
pub const WINHTTP_QUERY_FLAG_TRAILERS: i32 = 33554432;
pub const WINHTTP_QUERY_FLAG_WIRE_ENCODING: i32 = 16777216;
pub const WINHTTP_QUERY_FORWARDED: i32 = 30;
pub const WINHTTP_QUERY_FROM: i32 = 31;
pub const WINHTTP_QUERY_HOST: i32 = 55;
pub const WINHTTP_QUERY_IF_MATCH: i32 = 56;
pub const WINHTTP_QUERY_IF_MODIFIED_SINCE: i32 = 32;
pub const WINHTTP_QUERY_IF_NONE_MATCH: i32 = 57;
pub const WINHTTP_QUERY_IF_RANGE: i32 = 58;
pub const WINHTTP_QUERY_IF_UNMODIFIED_SINCE: i32 = 59;
pub const WINHTTP_QUERY_LAST_MODIFIED: i32 = 11;
pub const WINHTTP_QUERY_LINK: i32 = 16;
pub const WINHTTP_QUERY_LOCATION: i32 = 33;
pub const WINHTTP_QUERY_MAX: i32 = 78;
pub const WINHTTP_QUERY_MAX_FORWARDS: i32 = 60;
pub const WINHTTP_QUERY_MESSAGE_ID: i32 = 12;
pub const WINHTTP_QUERY_MIME_VERSION: i32 = 0;
pub const WINHTTP_QUERY_ORIG_URI: i32 = 34;
pub const WINHTTP_QUERY_PASSPORT_CONFIG: i32 = 78;
pub const WINHTTP_QUERY_PASSPORT_URLS: i32 = 77;
pub const WINHTTP_QUERY_PRAGMA: i32 = 17;
pub const WINHTTP_QUERY_PROXY_AUTHENTICATE: i32 = 41;
pub const WINHTTP_QUERY_PROXY_AUTHORIZATION: i32 = 61;
pub const WINHTTP_QUERY_PROXY_CONNECTION: i32 = 69;
pub const WINHTTP_QUERY_PROXY_SUPPORT: i32 = 75;
pub const WINHTTP_QUERY_PUBLIC: i32 = 8;
pub const WINHTTP_QUERY_RANGE: i32 = 62;
pub const WINHTTP_QUERY_RAW_HEADERS: i32 = 21;
pub const WINHTTP_QUERY_RAW_HEADERS_CRLF: i32 = 22;
pub const WINHTTP_QUERY_REFERER: i32 = 35;
pub const WINHTTP_QUERY_REFRESH: i32 = 46;
pub const WINHTTP_QUERY_REQUEST_METHOD: i32 = 45;
pub const WINHTTP_QUERY_RETRY_AFTER: i32 = 36;
pub const WINHTTP_QUERY_SERVER: i32 = 37;
pub const WINHTTP_QUERY_SET_COOKIE: i32 = 43;
pub const WINHTTP_QUERY_STATUS_CODE: i32 = 19;
pub const WINHTTP_QUERY_STATUS_TEXT: i32 = 20;
pub const WINHTTP_QUERY_TITLE: i32 = 38;
pub const WINHTTP_QUERY_TRANSFER_ENCODING: i32 = 63;
pub const WINHTTP_QUERY_UNLESS_MODIFIED_SINCE: i32 = 70;
pub const WINHTTP_QUERY_UPGRADE: i32 = 64;
pub const WINHTTP_QUERY_URI: i32 = 13;
pub const WINHTTP_QUERY_USER_AGENT: i32 = 39;
pub const WINHTTP_QUERY_VARY: i32 = 65;
pub const WINHTTP_QUERY_VERSION: i32 = 18;
pub const WINHTTP_QUERY_VIA: i32 = 66;
pub const WINHTTP_QUERY_WARNING: i32 = 67;
pub const WINHTTP_QUERY_WWW_AUTHENTICATE: i32 = 40;
pub const WINHTTP_READ_DATA_EX_FLAG_FILL_BUFFER: u64 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: super::ULONGLONG,
    pub ulIndex: super::ULONG,
    pub cStats: super::ULONG,
    pub rgullStats: [super::ULONGLONG; 32],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: super::ULONGLONG,
    pub ulIndex: super::ULONG,
    pub cStats: super::ULONG,
    pub rgullStats: [super::ULONGLONG; 32],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINHTTP_REQUEST_STAT_ENTRY = i32;
pub const WINHTTP_REQUEST_STAT_FLAG_FIRST_REQUEST: i32 = 32;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_FALSE_START: i32 = 16;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_SESSION_RESUMPTION: i32 = 8;
pub const WINHTTP_REQUEST_STAT_FLAG_TCP_FAST_OPEN: i32 = 1;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_FALSE_START: i32 = 4;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_SESSION_RESUMPTION: i32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: super::ULONG,
    pub rgullTimes: [super::ULONGLONG; 64],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: super::ULONG,
    pub rgullTimes: [super::ULONGLONG; 64],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINHTTP_REQUEST_TIME_ENTRY = i32;
pub const WINHTTP_RESET_ALL: i32 = 65535;
pub const WINHTTP_RESET_DISCARD_RESOLVERS: i32 = 262144;
pub const WINHTTP_RESET_NOTIFY_NETWORK_CHANGED: i32 = 65536;
pub const WINHTTP_RESET_OUT_OF_PROC: i32 = 131072;
pub const WINHTTP_RESET_SCRIPT_CACHE: i32 = 8;
pub const WINHTTP_RESET_STATE: i32 = 1;
pub const WINHTTP_RESET_SWPAD_ALL: i32 = 4;
pub const WINHTTP_RESET_SWPAD_CURRENT_NETWORK: i32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: super::ULONG,
    pub ulMaxCacheEntryAge: super::ULONG,
    pub ulMinCacheEntryTtl: super::ULONG,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: super::ULONGLONG,
    pub ullFlags: super::ULONGLONG,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: super::ULONG,
    pub ulMaxCacheEntryAge: super::ULONG,
    pub ulMinCacheEntryTtl: super::ULONG,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: super::ULONGLONG,
    pub ullFlags: super::ULONGLONG,
}
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: i32 = 2;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: i32 = 8;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: i32 = 1;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: i32 = 4;
pub type WINHTTP_SECURE_DNS_SETTING = i32;
pub const WINHTTP_SERVER_CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: i32 = 4;
pub const WINHTTP_SERVER_CERT_CHAIN_DISABLE_AIA: i32 = 8192;
pub const WINHTTP_SERVER_CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type WINHTTP_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: super::HINTERNET, dwcontext: super::DWORD_PTR, dwinternetstatus: super::DWORD, lpvstatusinformation: super::LPVOID, dwstatusinformationlength: super::DWORD)>;
pub const WINHTTP_TIME_FORMAT_BUFSIZE: i32 = 62;
pub const WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1006;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_WEB_SOCKET_OPERATION,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_WEB_SOCKET_OPERATION,
}
pub const WINHTTP_WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = 1;
pub const WINHTTP_WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = 0;
pub type WINHTTP_WEB_SOCKET_BUFFER_TYPE = i32;
pub const WINHTTP_WEB_SOCKET_CLOSE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = 4;
pub const WINHTTP_WEB_SOCKET_CLOSE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = 2;
pub type WINHTTP_WEB_SOCKET_CLOSE_STATUS = i32;
pub const WINHTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1005;
pub const WINHTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1001;
pub const WINHTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1003;
pub const WINHTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1007;
pub const WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: i32 = 123;
pub const WINHTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1009;
pub const WINHTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: i32 = 15000;
pub type WINHTTP_WEB_SOCKET_OPERATION = i32;
pub const WINHTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1008;
pub const WINHTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1002;
pub const WINHTTP_WEB_SOCKET_RECEIVE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = 1;
pub const WINHTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1015;
pub const WINHTTP_WEB_SOCKET_SEND_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = 0;
pub const WINHTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1011;
pub const WINHTTP_WEB_SOCKET_SHUTDOWN_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = 3;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_WEB_SOCKET_STATUS {
    pub dwBytesTransferred: super::DWORD,
    pub eBufferType: WINHTTP_WEB_SOCKET_BUFFER_TYPE,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_WEB_SOCKET_STATUS {
    pub dwBytesTransferred: super::DWORD,
    pub eBufferType: WINHTTP_WEB_SOCKET_BUFFER_TYPE,
}
pub const WINHTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1000;
pub const WINHTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1010;
pub const WINHTTP_WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = 3;
pub const WINHTTP_WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = 2;
pub const WinHttpConnectFailureCount: WINHTTP_REQUEST_STAT_ENTRY = 0;
pub const WinHttpConnectionAcquireEnd: WINHTTP_REQUEST_TIME_ENTRY = 4;
pub const WinHttpConnectionAcquireStart: WINHTTP_REQUEST_TIME_ENTRY = 2;
pub const WinHttpConnectionAcquireWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = 3;
pub const WinHttpConnectionEstablishmentEnd: WINHTTP_REQUEST_TIME_ENTRY = 8;
pub const WinHttpConnectionEstablishmentStart: WINHTTP_REQUEST_TIME_ENTRY = 7;
pub const WinHttpFastForwardingStateClientSideFailed: WINHTTP_FAST_FORWARDING_STATE = 2;
pub const WinHttpFastForwardingStateInProgress: WINHTTP_FAST_FORWARDING_STATE = 0;
pub const WinHttpFastForwardingStateServerSideFailed: WINHTTP_FAST_FORWARDING_STATE = 3;
pub const WinHttpFastForwardingStateSucceeded: WINHTTP_FAST_FORWARDING_STATE = 1;
pub const WinHttpNameResolutionEnd: WINHTTP_REQUEST_TIME_ENTRY = 6;
pub const WinHttpNameResolutionStart: WINHTTP_REQUEST_TIME_ENTRY = 5;
pub const WinHttpProxyDetectionEnd: WINHTTP_REQUEST_TIME_ENTRY = 1;
pub const WinHttpProxyDetectionStart: WINHTTP_REQUEST_TIME_ENTRY = 0;
pub const WinHttpProxyFailureCount: WINHTTP_REQUEST_STAT_ENTRY = 1;
pub const WinHttpProxySettingsTypeUnknown: WINHTTP_PROXY_SETTINGS_TYPE = 0;
pub const WinHttpProxySettingsTypeWsa: WINHTTP_PROXY_SETTINGS_TYPE = 2;
pub const WinHttpProxySettingsTypeWsl: WINHTTP_PROXY_SETTINGS_TYPE = 1;
pub const WinHttpProxySettingsTypeXBox: WINHTTP_PROXY_SETTINGS_TYPE = 3;
pub const WinHttpProxyTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = 31;
pub const WinHttpProxyTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = 12;
pub const WinHttpProxyTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = 30;
pub const WinHttpProxyTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = 33;
pub const WinHttpProxyTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = 14;
pub const WinHttpProxyTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = 32;
pub const WinHttpProxyTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = 35;
pub const WinHttpProxyTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = 34;
pub const WinHttpProxyTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = 13;
pub const WinHttpProxyTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = 15;
pub const WinHttpProxyTunnelEnd: WINHTTP_REQUEST_TIME_ENTRY = 29;
pub const WinHttpProxyTunnelStart: WINHTTP_REQUEST_TIME_ENTRY = 28;
pub const WinHttpReceiveResponseBodyDecompressionDelta: WINHTTP_REQUEST_TIME_ENTRY = 26;
pub const WinHttpReceiveResponseEnd: WINHTTP_REQUEST_TIME_ENTRY = 27;
pub const WinHttpReceiveResponseHeadersDecompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = 24;
pub const WinHttpReceiveResponseHeadersDecompressionStart: WINHTTP_REQUEST_TIME_ENTRY = 23;
pub const WinHttpReceiveResponseHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = 25;
pub const WinHttpReceiveResponseStart: WINHTTP_REQUEST_TIME_ENTRY = 22;
pub const WinHttpRequestHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = 7;
pub const WinHttpRequestHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = 6;
pub const WinHttpRequestStatLast: WINHTTP_REQUEST_STAT_ENTRY = 16;
pub const WinHttpRequestStatMax: WINHTTP_REQUEST_STAT_ENTRY = 32;
pub const WinHttpRequestTimeLast: WINHTTP_REQUEST_TIME_ENTRY = 36;
pub const WinHttpRequestTimeMax: WINHTTP_REQUEST_TIME_ENTRY = 64;
pub const WinHttpResponseBodyCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = 11;
pub const WinHttpResponseBodySize: WINHTTP_REQUEST_STAT_ENTRY = 10;
pub const WinHttpResponseHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = 9;
pub const WinHttpResponseHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = 8;
pub const WinHttpSecureDnsSettingDefault: WINHTTP_SECURE_DNS_SETTING = 0;
pub const WinHttpSecureDnsSettingForcePlaintext: WINHTTP_SECURE_DNS_SETTING = 1;
pub const WinHttpSecureDnsSettingMax: WINHTTP_SECURE_DNS_SETTING = 4;
pub const WinHttpSecureDnsSettingRequireEncryption: WINHTTP_SECURE_DNS_SETTING = 2;
pub const WinHttpSecureDnsSettingTryEncryptionWithFallback: WINHTTP_SECURE_DNS_SETTING = 3;
pub const WinHttpSendRequestEnd: WINHTTP_REQUEST_TIME_ENTRY = 21;
pub const WinHttpSendRequestHeadersCompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = 19;
pub const WinHttpSendRequestHeadersCompressionStart: WINHTTP_REQUEST_TIME_ENTRY = 18;
pub const WinHttpSendRequestHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = 20;
pub const WinHttpSendRequestStart: WINHTTP_REQUEST_TIME_ENTRY = 17;
pub const WinHttpStreamWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = 16;
pub const WinHttpStreamWaitStart: WINHTTP_REQUEST_TIME_ENTRY = 15;
pub const WinHttpTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = 10;
pub const WinHttpTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = 2;
pub const WinHttpTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = 9;
pub const WinHttpTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = 12;
pub const WinHttpTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = 4;
pub const WinHttpTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = 11;
pub const WinHttpTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = 14;
pub const WinHttpTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = 13;
pub const WinHttpTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = 3;
pub const WinHttpTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = 5;
