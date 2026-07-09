#[inline]
pub unsafe fn WinHttpAddRequestHeaders(hrequest: *mut core::ffi::c_void, lpszheaders: &[u16], dwmodifiers: u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpAddRequestHeaders(hrequest : *mut core::ffi::c_void, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_core::BOOL);
    unsafe { WinHttpAddRequestHeaders(hrequest as _, core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len().try_into().unwrap(), dwmodifiers).ok() }
}
#[inline]
pub unsafe fn WinHttpAddRequestHeadersEx(hrequest: *mut core::ffi::c_void, dwmodifiers: u32, ullflags: u64, ullextra: u64, pheaders: &[WINHTTP_EXTENDED_HEADER]) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpAddRequestHeadersEx(hrequest : *mut core::ffi::c_void, dwmodifiers : u32, ullflags : u64, ullextra : u64, cheaders : u32, pheaders : *const WINHTTP_EXTENDED_HEADER) -> u32);
    unsafe { WinHttpAddRequestHeadersEx(hrequest as _, dwmodifiers, ullflags, ullextra, pheaders.len().try_into().unwrap(), core::mem::transmute(pheaders.as_ptr())) }
}
#[inline]
pub unsafe fn WinHttpCheckPlatform() -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCheckPlatform() -> windows_core::BOOL);
    unsafe { WinHttpCheckPlatform().ok() }
}
#[inline]
pub unsafe fn WinHttpCloseHandle(hinternet: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCloseHandle(hinternet : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinHttpCloseHandle(hinternet as _).ok() }
}
#[inline]
pub unsafe fn WinHttpConnect<P1>(hsession: *mut core::ffi::c_void, pswzservername: P1, nserverport: u16, dwreserved: u32) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpConnect(hsession : *mut core::ffi::c_void, pswzservername : windows_core::PCWSTR, nserverport : u16, dwreserved : u32) -> *mut core::ffi::c_void);
    unsafe { WinHttpConnect(hsession as _, pswzservername.param().abi(), nserverport, dwreserved) }
}
#[inline]
pub unsafe fn WinHttpCrackUrl(pwszurl: &[u16], dwflags: u32, lpurlcomponents: *mut URL_COMPONENTS) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCrackUrl(pwszurl : windows_core::PCWSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : *mut URL_COMPONENTS) -> windows_core::BOOL);
    unsafe { WinHttpCrackUrl(core::mem::transmute(pwszurl.as_ptr()), pwszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _).ok() }
}
#[inline]
pub unsafe fn WinHttpCreateProxyResolver(hsession: *const core::ffi::c_void, phresolver: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCreateProxyResolver(hsession : *const core::ffi::c_void, phresolver : *mut *mut core::ffi::c_void) -> u32);
    unsafe { WinHttpCreateProxyResolver(hsession, phresolver as _) }
}
#[inline]
pub unsafe fn WinHttpCreateUrl(lpurlcomponents: *const URL_COMPONENTS, dwflags: WIN_HTTP_CREATE_URL_FLAGS, pwszurl: Option<windows_core::PWSTR>, pdwurllength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpCreateUrl(lpurlcomponents : *const URL_COMPONENTS, dwflags : WIN_HTTP_CREATE_URL_FLAGS, pwszurl : windows_core::PWSTR, pdwurllength : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpCreateUrl(lpurlcomponents, dwflags, pwszurl.unwrap_or(core::mem::zeroed()) as _, pdwurllength as _).ok() }
}
#[inline]
pub unsafe fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags: u32, ppwstrautoconfigurl: *mut windows_core::PWSTR) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags : u32, ppwstrautoconfigurl : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WinHttpDetectAutoProxyConfigUrl(dwautodetectflags, ppwstrautoconfigurl as _).ok() }
}
#[inline]
pub unsafe fn WinHttpFreeProxyResult(pproxyresult: *mut WINHTTP_PROXY_RESULT) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxyResult(pproxyresult : *mut WINHTTP_PROXY_RESULT));
    unsafe { WinHttpFreeProxyResult(pproxyresult as _) }
}
#[inline]
pub unsafe fn WinHttpFreeProxyResultEx(pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxyResultEx(pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX));
    unsafe { WinHttpFreeProxyResultEx(pproxyresultex as _) }
}
#[inline]
pub unsafe fn WinHttpFreeProxySettings(pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxySettings(pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS));
    unsafe { WinHttpFreeProxySettings(pwinhttpproxysettings) }
}
#[inline]
pub unsafe fn WinHttpFreeProxySettingsEx(proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex: *const core::ffi::c_void) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeProxySettingsEx(proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex : *const core::ffi::c_void) -> u32);
    unsafe { WinHttpFreeProxySettingsEx(proxysettingstype, pproxysettingsex) }
}
#[inline]
pub unsafe fn WinHttpFreeQueryConnectionGroupResult(presult: *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) {
    windows_core::link!("winhttp.dll" "system" fn WinHttpFreeQueryConnectionGroupResult(presult : *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT));
    unsafe { WinHttpFreeQueryConnectionGroupResult(presult as _) }
}
#[inline]
pub unsafe fn WinHttpGetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpGetDefaultProxyConfiguration(pproxyinfo as _).ok() }
}
#[inline]
pub unsafe fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig : *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> windows_core::BOOL);
    unsafe { WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig as _).ok() }
}
#[inline]
pub unsafe fn WinHttpGetProxyForUrl<P1>(hsession: *mut core::ffi::c_void, lpcwszurl: P1, pautoproxyoptions: *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrl(hsession : *mut core::ffi::c_void, lpcwszurl : windows_core::PCWSTR, pautoproxyoptions : *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpGetProxyForUrl(hsession as _, lpcwszurl.param().abi(), pautoproxyoptions as _, pproxyinfo as _).ok() }
}
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx<P1>(hresolver: *const core::ffi::c_void, pcwszurl: P1, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pcontext: Option<usize>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx(hresolver : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, pcontext : usize) -> u32);
    unsafe { WinHttpGetProxyForUrlEx(hresolver, pcwszurl.param().abi(), pautoproxyoptions, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxyForUrlEx2<P1>(hresolver: *const core::ffi::c_void, pcwszurl: P1, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pinterfaceselectioncontext: Option<&[u8]>, pcontext: Option<usize>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx2(hresolver : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, cbinterfaceselectioncontext : u32, pinterfaceselectioncontext : *const u8, pcontext : usize) -> u32);
    unsafe { WinHttpGetProxyForUrlEx2(hresolver, pcwszurl.param().abi(), pautoproxyoptions, pinterfaceselectioncontext.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pinterfaceselectioncontext.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxyResult(hresolver: *const core::ffi::c_void, pproxyresult: *mut WINHTTP_PROXY_RESULT) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyResult(hresolver : *const core::ffi::c_void, pproxyresult : *mut WINHTTP_PROXY_RESULT) -> u32);
    unsafe { WinHttpGetProxyResult(hresolver, pproxyresult as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxyResultEx(hresolver: *const core::ffi::c_void, pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxyResultEx(hresolver : *const core::ffi::c_void, pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX) -> u32);
    unsafe { WinHttpGetProxyResultEx(hresolver, pproxyresultex as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsEx(hresolver: *const core::ffi::c_void, proxysettingstype: WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam: Option<*const WINHTTP_PROXY_SETTINGS_PARAM>, pcontext: Option<usize>) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsEx(hresolver : *const core::ffi::c_void, proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam : *const WINHTTP_PROXY_SETTINGS_PARAM, pcontext : usize) -> u32);
    unsafe { WinHttpGetProxySettingsEx(hresolver, proxysettingstype, pproxysettingsparam.unwrap_or(core::mem::zeroed()) as _, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsResultEx(hresolver: *const core::ffi::c_void, pproxysettingsex: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsResultEx(hresolver : *const core::ffi::c_void, pproxysettingsex : *mut core::ffi::c_void) -> u32);
    unsafe { WinHttpGetProxySettingsResultEx(hresolver, pproxysettingsex as _) }
}
#[inline]
pub unsafe fn WinHttpGetProxySettingsVersion(hsession: *const core::ffi::c_void, pdwproxysettingsversion: *mut u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsVersion(hsession : *const core::ffi::c_void, pdwproxysettingsversion : *mut u32) -> u32);
    unsafe { WinHttpGetProxySettingsVersion(hsession, pdwproxysettingsversion as _) }
}
#[inline]
pub unsafe fn WinHttpOpen<P0, P2, P3>(pszagentw: P0, dwaccesstype: WINHTTP_ACCESS_TYPE, pszproxyw: P2, pszproxybypassw: P3, dwflags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpOpen(pszagentw : windows_core::PCWSTR, dwaccesstype : WINHTTP_ACCESS_TYPE, pszproxyw : windows_core::PCWSTR, pszproxybypassw : windows_core::PCWSTR, dwflags : u32) -> *mut core::ffi::c_void);
    unsafe { WinHttpOpen(pszagentw.param().abi(), dwaccesstype, pszproxyw.param().abi(), pszproxybypassw.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn WinHttpOpenRequest<P1, P2, P3, P4>(hconnect: *mut core::ffi::c_void, pwszverb: P1, pwszobjectname: P2, pwszversion: P3, pwszreferrer: P4, ppwszaccepttypes: *const windows_core::PCWSTR, dwflags: WINHTTP_OPEN_REQUEST_FLAGS) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpOpenRequest(hconnect : *mut core::ffi::c_void, pwszverb : windows_core::PCWSTR, pwszobjectname : windows_core::PCWSTR, pwszversion : windows_core::PCWSTR, pwszreferrer : windows_core::PCWSTR, ppwszaccepttypes : *const windows_core::PCWSTR, dwflags : WINHTTP_OPEN_REQUEST_FLAGS) -> *mut core::ffi::c_void);
    unsafe { WinHttpOpenRequest(hconnect as _, pwszverb.param().abi(), pwszobjectname.param().abi(), pwszversion.param().abi(), pwszreferrer.param().abi(), ppwszaccepttypes, dwflags) }
}
#[inline]
pub unsafe fn WinHttpProtocolCompleteUpgrade(hrequest: *const core::ffi::c_void, dwcontext: Option<usize>) -> *mut core::ffi::c_void {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolCompleteUpgrade(hrequest : *const core::ffi::c_void, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { WinHttpProtocolCompleteUpgrade(hrequest, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpProtocolReceive(protocolhandle: *const core::ffi::c_void, flags: u64, pvbuffer: *mut core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolReceive(protocolhandle : *const core::ffi::c_void, flags : u64, pvbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32) -> u32);
    unsafe { WinHttpProtocolReceive(protocolhandle, flags, pvbuffer as _, dwbufferlength, pdwbytesread as _) }
}
#[inline]
pub unsafe fn WinHttpProtocolSend(protocolhandle: *const core::ffi::c_void, flags: u64, pvbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpProtocolSend(protocolhandle : *const core::ffi::c_void, flags : u64, pvbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> u32);
    unsafe { WinHttpProtocolSend(protocolhandle, flags, core::mem::transmute(pvbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn WinHttpQueryAuthSchemes(hrequest: *mut core::ffi::c_void, lpdwsupportedschemes: *mut u32, lpdwfirstscheme: *mut u32, pdwauthtarget: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryAuthSchemes(hrequest : *mut core::ffi::c_void, lpdwsupportedschemes : *mut u32, lpdwfirstscheme : *mut u32, pdwauthtarget : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpQueryAuthSchemes(hrequest as _, lpdwsupportedschemes as _, lpdwfirstscheme as _, pdwauthtarget as _).ok() }
}
#[inline]
pub unsafe fn WinHttpQueryConnectionGroup(hinternet: *const core::ffi::c_void, pguidconnection: Option<*const windows_core::GUID>, ullflags: u64, ppresult: *mut *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryConnectionGroup(hinternet : *const core::ffi::c_void, pguidconnection : *const windows_core::GUID, ullflags : u64, ppresult : *mut *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32);
    unsafe { WinHttpQueryConnectionGroup(hinternet, pguidconnection.unwrap_or(core::mem::zeroed()) as _, ullflags, ppresult as _) }
}
#[inline]
pub unsafe fn WinHttpQueryDataAvailable(hrequest: *mut core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryDataAvailable(hrequest : *mut core::ffi::c_void, lpdwnumberofbytesavailable : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpQueryDataAvailable(hrequest as _, lpdwnumberofbytesavailable as _).ok() }
}
#[inline]
pub unsafe fn WinHttpQueryHeaders<P2>(hrequest: *mut core::ffi::c_void, dwinfolevel: u32, pwszname: P2, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryHeaders(hrequest : *mut core::ffi::c_void, dwinfolevel : u32, pwszname : windows_core::PCWSTR, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpQueryHeaders(hrequest as _, dwinfolevel, pwszname.param().abi(), lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex as _).ok() }
}
#[inline]
pub unsafe fn WinHttpQueryHeadersEx(hrequest: *const core::ffi::c_void, dwinfolevel: u32, ullflags: u64, uicodepage: u32, pdwindex: Option<*mut u32>, pheadername: Option<*const WINHTTP_HEADER_NAME>, pbuffer: Option<*mut core::ffi::c_void>, pdwbufferlength: *mut u32, ppheaders: Option<*mut *mut WINHTTP_EXTENDED_HEADER>, pdwheaderscount: *mut u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryHeadersEx(hrequest : *const core::ffi::c_void, dwinfolevel : u32, ullflags : u64, uicodepage : u32, pdwindex : *mut u32, pheadername : *const WINHTTP_HEADER_NAME, pbuffer : *mut core::ffi::c_void, pdwbufferlength : *mut u32, ppheaders : *mut *mut WINHTTP_EXTENDED_HEADER, pdwheaderscount : *mut u32) -> u32);
    unsafe { WinHttpQueryHeadersEx(hrequest, dwinfolevel, ullflags, uicodepage, pdwindex.unwrap_or(core::mem::zeroed()) as _, pheadername.unwrap_or(core::mem::zeroed()) as _, pbuffer.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength as _, ppheaders.unwrap_or(core::mem::zeroed()) as _, pdwheaderscount as _) }
}
#[inline]
pub unsafe fn WinHttpQueryOption(hinternet: *mut core::ffi::c_void, dwoption: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpQueryOption(hinternet : *mut core::ffi::c_void, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpQueryOption(hinternet as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn WinHttpReadData(hrequest: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadData(hrequest : *mut core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpReadData(hrequest as _, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _).ok() }
}
#[inline]
pub unsafe fn WinHttpReadDataEx(hrequest: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32, ullflags: u64, cbproperty: u32, pvproperty: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadDataEx(hrequest : *mut core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32, ullflags : u64, cbproperty : u32, pvproperty : *const core::ffi::c_void) -> u32);
    unsafe { WinHttpReadDataEx(hrequest as _, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _, ullflags, cbproperty, pvproperty.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpReadProxySettings<P1>(hsession: *const core::ffi::c_void, pcwszconnectionname: P1, ffallbacktodefaultsettings: bool, fsetautodiscoverfordefaultsettings: bool, pdwsettingsversion: *mut u32, pfdefaultsettingsarereturned: *mut windows_core::BOOL, pwinhttpproxysettings: *mut WINHTTP_PROXY_SETTINGS) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpReadProxySettings(hsession : *const core::ffi::c_void, pcwszconnectionname : windows_core::PCWSTR, ffallbacktodefaultsettings : windows_core::BOOL, fsetautodiscoverfordefaultsettings : windows_core::BOOL, pdwsettingsversion : *mut u32, pfdefaultsettingsarereturned : *mut windows_core::BOOL, pwinhttpproxysettings : *mut WINHTTP_PROXY_SETTINGS) -> u32);
    unsafe { WinHttpReadProxySettings(hsession, pcwszconnectionname.param().abi(), ffallbacktodefaultsettings.into(), fsetautodiscoverfordefaultsettings.into(), pdwsettingsversion as _, pfdefaultsettingsarereturned as _, pwinhttpproxysettings as _) }
}
#[inline]
pub unsafe fn WinHttpReceiveResponse(hrequest: *mut core::ffi::c_void, lpreserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpReceiveResponse(hrequest : *mut core::ffi::c_void, lpreserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinHttpReceiveResponse(hrequest as _, lpreserved as _).ok() }
}
#[inline]
pub unsafe fn WinHttpRegisterProxyChangeNotification(ullflags: u64, pfncallback: WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext: *const core::ffi::c_void, hregistration: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpRegisterProxyChangeNotification(ullflags : u64, pfncallback : WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext : *const core::ffi::c_void, hregistration : *mut *mut core::ffi::c_void) -> u32);
    unsafe { WinHttpRegisterProxyChangeNotification(ullflags, pfncallback, pvcontext, hregistration as _) }
}
#[inline]
pub unsafe fn WinHttpResetAutoProxy(hsession: *const core::ffi::c_void, dwflags: u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpResetAutoProxy(hsession : *const core::ffi::c_void, dwflags : u32) -> u32);
    unsafe { WinHttpResetAutoProxy(hsession, dwflags) }
}
#[inline]
pub unsafe fn WinHttpSendRequest(hrequest: *mut core::ffi::c_void, lpszheaders: Option<&[u16]>, lpoptional: Option<*const core::ffi::c_void>, dwoptionallength: u32, dwtotallength: u32, dwcontext: usize) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSendRequest(hrequest : *mut core::ffi::c_void, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32, dwtotallength : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { WinHttpSendRequest(hrequest as _, core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength, dwtotallength, dwcontext).ok() }
}
#[inline]
pub unsafe fn WinHttpSetCredentials<P3, P4>(hrequest: *mut core::ffi::c_void, authtargets: u32, authscheme: u32, pwszusername: P3, pwszpassword: P4, pauthparams: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetCredentials(hrequest : *mut core::ffi::c_void, authtargets : u32, authscheme : u32, pwszusername : windows_core::PCWSTR, pwszpassword : windows_core::PCWSTR, pauthparams : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinHttpSetCredentials(hrequest as _, authtargets, authscheme, pwszusername.param().abi(), pwszpassword.param().abi(), pauthparams as _).ok() }
}
#[inline]
pub unsafe fn WinHttpSetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_core::BOOL);
    unsafe { WinHttpSetDefaultProxyConfiguration(pproxyinfo as _).ok() }
}
#[inline]
pub unsafe fn WinHttpSetOption(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<&[u8]>) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetOption(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { WinHttpSetOption(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, core::mem::transmute(lpbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser: bool) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser : windows_core::BOOL) -> u32);
    unsafe { WinHttpSetProxySettingsPerUser(fproxysettingsperuser.into()) }
}
#[inline]
pub unsafe fn WinHttpSetStatusCallback(hinternet: *mut core::ffi::c_void, lpfninternetcallback: WINHTTP_STATUS_CALLBACK, dwnotificationflags: u32, dwreserved: usize) -> WINHTTP_STATUS_CALLBACK {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetStatusCallback(hinternet : *mut core::ffi::c_void, lpfninternetcallback : WINHTTP_STATUS_CALLBACK, dwnotificationflags : u32, dwreserved : usize) -> WINHTTP_STATUS_CALLBACK);
    unsafe { WinHttpSetStatusCallback(hinternet as _, lpfninternetcallback, dwnotificationflags, dwreserved) }
}
#[inline]
pub unsafe fn WinHttpSetTimeouts(hinternet: *mut core::ffi::c_void, nresolvetimeout: i32, nconnecttimeout: i32, nsendtimeout: i32, nreceivetimeout: i32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpSetTimeouts(hinternet : *mut core::ffi::c_void, nresolvetimeout : i32, nconnecttimeout : i32, nsendtimeout : i32, nreceivetimeout : i32) -> windows_core::BOOL);
    unsafe { WinHttpSetTimeouts(hinternet as _, nresolvetimeout, nconnecttimeout, nsendtimeout, nreceivetimeout).ok() }
}
#[inline]
pub unsafe fn WinHttpTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, pwsztime: &mut [u16; 62]) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpTimeFromSystemTime(pst : *const super::super::Foundation::SYSTEMTIME, pwsztime : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WinHttpTimeFromSystemTime(pst, core::mem::transmute(pwsztime.as_ptr())).ok() }
}
#[inline]
pub unsafe fn WinHttpTimeToSystemTime<P0>(pwsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winhttp.dll" "system" fn WinHttpTimeToSystemTime(pwsztime : windows_core::PCWSTR, pst : *mut super::super::Foundation::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { WinHttpTimeToSystemTime(pwsztime.param().abi(), pst as _).ok() }
}
#[inline]
pub unsafe fn WinHttpUnregisterProxyChangeNotification(hregistration: *const core::ffi::c_void) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpUnregisterProxyChangeNotification(hregistration : *const core::ffi::c_void) -> u32);
    unsafe { WinHttpUnregisterProxyChangeNotification(hregistration) }
}
#[inline]
pub unsafe fn WinHttpWebSocketClose(hwebsocket: *const core::ffi::c_void, usstatus: u16, pvreason: Option<*const core::ffi::c_void>, dwreasonlength: u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketClose(hwebsocket : *const core::ffi::c_void, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> u32);
    unsafe { WinHttpWebSocketClose(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[inline]
pub unsafe fn WinHttpWebSocketCompleteUpgrade(hrequest: *const core::ffi::c_void, pcontext: Option<usize>) -> *mut core::ffi::c_void {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketCompleteUpgrade(hrequest : *const core::ffi::c_void, pcontext : usize) -> *mut core::ffi::c_void);
    unsafe { WinHttpWebSocketCompleteUpgrade(hrequest, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinHttpWebSocketQueryCloseStatus(hwebsocket: *const core::ffi::c_void, pusstatus: *mut u16, pvreason: Option<*mut core::ffi::c_void>, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketQueryCloseStatus(hwebsocket : *const core::ffi::c_void, pusstatus : *mut u16, pvreason : *mut core::ffi::c_void, dwreasonlength : u32, pdwreasonlengthconsumed : *mut u32) -> u32);
    unsafe { WinHttpWebSocketQueryCloseStatus(hwebsocket, pusstatus as _, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength, pdwreasonlengthconsumed as _) }
}
#[inline]
pub unsafe fn WinHttpWebSocketReceive(hwebsocket: *const core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pebuffertype: *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketReceive(hwebsocket : *const core::ffi::c_void, pvbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32, pebuffertype : *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32);
    unsafe { WinHttpWebSocketReceive(hwebsocket, pvbuffer as _, dwbufferlength, pdwbytesread as _, pebuffertype as _) }
}
#[inline]
pub unsafe fn WinHttpWebSocketSend(hwebsocket: *const core::ffi::c_void, ebuffertype: WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketSend(hwebsocket : *const core::ffi::c_void, ebuffertype : WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> u32);
    unsafe { WinHttpWebSocketSend(hwebsocket, ebuffertype, core::mem::transmute(pvbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn WinHttpWebSocketShutdown(hwebsocket: *const core::ffi::c_void, usstatus: u16, pvreason: Option<*const core::ffi::c_void>, dwreasonlength: u32) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWebSocketShutdown(hwebsocket : *const core::ffi::c_void, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> u32);
    unsafe { WinHttpWebSocketShutdown(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[inline]
pub unsafe fn WinHttpWriteData(hrequest: *mut core::ffi::c_void, lpbuffer: Option<*const core::ffi::c_void>, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWriteData(hrequest : *mut core::ffi::c_void, lpbuffer : *const core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WinHttpWriteData(hrequest as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwnumberofbytestowrite, lpdwnumberofbyteswritten as _).ok() }
}
#[inline]
pub unsafe fn WinHttpWriteProxySettings(hsession: *const core::ffi::c_void, fforceupdate: bool, pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) -> u32 {
    windows_core::link!("winhttp.dll" "system" fn WinHttpWriteProxySettings(hsession : *const core::ffi::c_void, fforceupdate : windows_core::BOOL, pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS) -> u32);
    unsafe { WinHttpWriteProxySettings(hsession, fforceupdate.into(), pwinhttpproxysettings) }
}
pub const API_GET_PROXY_FOR_URL: u32 = 6;
pub const API_GET_PROXY_SETTINGS: u32 = 7;
pub const API_QUERY_DATA_AVAILABLE: u32 = 2;
pub const API_READ_DATA: u32 = 3;
pub const API_RECEIVE_RESPONSE: u32 = 1;
pub const API_SEND_REQUEST: u32 = 5;
pub const API_WRITE_DATA: u32 = 4;
pub const AutoLogonPolicy_Always: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(0);
pub const AutoLogonPolicy_Never: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(2);
pub const AutoLogonPolicy_OnlyIfBypassProxy: WinHttpRequestAutoLogonPolicy = WinHttpRequestAutoLogonPolicy(1);
pub const ERROR_WINHTTP_AUTODETECTION_FAILED: u32 = 12180;
pub const ERROR_WINHTTP_AUTO_PROXY_SERVICE_ERROR: u32 = 12178;
pub const ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT: u32 = 12166;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_OPEN: u32 = 12103;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_SEND: u32 = 12102;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_OPEN: u32 = 12100;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_SEND: u32 = 12101;
pub const ERROR_WINHTTP_CANNOT_CONNECT: u32 = 12029;
pub const ERROR_WINHTTP_CHUNKED_ENCODING_HEADER_SIZE_OVERFLOW: u32 = 12183;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED: u32 = 12044;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_ACCESS_PRIVATE_KEY: u32 = 12186;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_PRIVATE_KEY: u32 = 12185;
pub const ERROR_WINHTTP_CONNECTION_ERROR: u32 = 12030;
pub const ERROR_WINHTTP_FAST_FORWARDING_NOT_SUPPORTED: u32 = 12193;
pub const ERROR_WINHTTP_FEATURE_DISABLED: u32 = 12192;
pub const ERROR_WINHTTP_GLOBAL_CALLBACK_FAILED: u32 = 12191;
pub const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: u32 = 12155;
pub const ERROR_WINHTTP_HEADER_COUNT_EXCEEDED: u32 = 12181;
pub const ERROR_WINHTTP_HEADER_NOT_FOUND: u32 = 12150;
pub const ERROR_WINHTTP_HEADER_SIZE_OVERFLOW: u32 = 12182;
pub const ERROR_WINHTTP_HTTP_PROTOCOL_MISMATCH: u32 = 12190;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: u32 = 12019;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: u32 = 12018;
pub const ERROR_WINHTTP_INTERNAL_ERROR: u32 = 12004;
pub const ERROR_WINHTTP_INVALID_HEADER: u32 = 12153;
pub const ERROR_WINHTTP_INVALID_OPTION: u32 = 12009;
pub const ERROR_WINHTTP_INVALID_QUERY_REQUEST: u32 = 12154;
pub const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: u32 = 12152;
pub const ERROR_WINHTTP_INVALID_URL: u32 = 12005;
pub const ERROR_WINHTTP_LOGIN_FAILURE: u32 = 12015;
pub const ERROR_WINHTTP_NAME_NOT_RESOLVED: u32 = 12007;
pub const ERROR_WINHTTP_NOT_INITIALIZED: u32 = 12172;
pub const ERROR_WINHTTP_OPERATION_CANCELLED: u32 = 12017;
pub const ERROR_WINHTTP_OPTION_NOT_SETTABLE: u32 = 12011;
pub const ERROR_WINHTTP_OUT_OF_HANDLES: u32 = 12001;
pub const ERROR_WINHTTP_REDIRECT_FAILED: u32 = 12156;
pub const ERROR_WINHTTP_RESEND_REQUEST: u32 = 12032;
pub const ERROR_WINHTTP_RESERVED_189: u32 = 12189;
pub const ERROR_WINHTTP_RESPONSE_DRAIN_OVERFLOW: u32 = 12184;
pub const ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR: u32 = 12177;
pub const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: u32 = 12038;
pub const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: u32 = 12037;
pub const ERROR_WINHTTP_SECURE_CERT_REVOKED: u32 = 12170;
pub const ERROR_WINHTTP_SECURE_CERT_REV_FAILED: u32 = 12057;
pub const ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE: u32 = 12179;
pub const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: u32 = 12157;
pub const ERROR_WINHTTP_SECURE_FAILURE: u32 = 12175;
pub const ERROR_WINHTTP_SECURE_FAILURE_PROXY: u32 = 12188;
pub const ERROR_WINHTTP_SECURE_INVALID_CA: u32 = 12045;
pub const ERROR_WINHTTP_SECURE_INVALID_CERT: u32 = 12169;
pub const ERROR_WINHTTP_SHUTDOWN: u32 = 12012;
pub const ERROR_WINHTTP_TIMEOUT: u32 = 12002;
pub const ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167;
pub const ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE: u32 = 12176;
pub const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: u32 = 12006;
pub const HTTPREQUEST_PROXYSETTING_DEFAULT: u32 = 0;
pub const HTTPREQUEST_PROXYSETTING_DIRECT: u32 = 1;
pub const HTTPREQUEST_PROXYSETTING_PRECONFIG: u32 = 0;
pub const HTTPREQUEST_PROXYSETTING_PROXY: u32 = 2;
pub const HTTPREQUEST_SETCREDENTIALS_FOR_PROXY: u32 = 1;
pub const HTTPREQUEST_SETCREDENTIALS_FOR_SERVER: u32 = 0;
pub const HTTP_STATUS_ACCEPTED: u32 = 202;
pub const HTTP_STATUS_AMBIGUOUS: u32 = 300;
pub const HTTP_STATUS_BAD_GATEWAY: u32 = 502;
pub const HTTP_STATUS_BAD_METHOD: u32 = 405;
pub const HTTP_STATUS_BAD_REQUEST: u32 = 400;
pub const HTTP_STATUS_CONFLICT: u32 = 409;
pub const HTTP_STATUS_CONTINUE: u32 = 100;
pub const HTTP_STATUS_CREATED: u32 = 201;
pub const HTTP_STATUS_DENIED: u32 = 401;
pub const HTTP_STATUS_FIRST: u32 = 100;
pub const HTTP_STATUS_FORBIDDEN: u32 = 403;
pub const HTTP_STATUS_GATEWAY_TIMEOUT: u32 = 504;
pub const HTTP_STATUS_GONE: u32 = 410;
pub const HTTP_STATUS_LAST: u32 = 505;
pub const HTTP_STATUS_LENGTH_REQUIRED: u32 = 411;
pub const HTTP_STATUS_MOVED: u32 = 301;
pub const HTTP_STATUS_NONE_ACCEPTABLE: u32 = 406;
pub const HTTP_STATUS_NOT_FOUND: u32 = 404;
pub const HTTP_STATUS_NOT_MODIFIED: u32 = 304;
pub const HTTP_STATUS_NOT_SUPPORTED: u32 = 501;
pub const HTTP_STATUS_NO_CONTENT: u32 = 204;
pub const HTTP_STATUS_OK: u32 = 200;
pub const HTTP_STATUS_PARTIAL: u32 = 203;
pub const HTTP_STATUS_PARTIAL_CONTENT: u32 = 206;
pub const HTTP_STATUS_PAYMENT_REQ: u32 = 402;
pub const HTTP_STATUS_PERMANENT_REDIRECT: u32 = 308;
pub const HTTP_STATUS_PRECOND_FAILED: u32 = 412;
pub const HTTP_STATUS_PROXY_AUTH_REQ: u32 = 407;
pub const HTTP_STATUS_REDIRECT: u32 = 302;
pub const HTTP_STATUS_REDIRECT_KEEP_VERB: u32 = 307;
pub const HTTP_STATUS_REDIRECT_METHOD: u32 = 303;
pub const HTTP_STATUS_REQUEST_TIMEOUT: u32 = 408;
pub const HTTP_STATUS_REQUEST_TOO_LARGE: u32 = 413;
pub const HTTP_STATUS_RESET_CONTENT: u32 = 205;
pub const HTTP_STATUS_RETRY_WITH: u32 = 449;
pub const HTTP_STATUS_SERVER_ERROR: u32 = 500;
pub const HTTP_STATUS_SERVICE_UNAVAIL: u32 = 503;
pub const HTTP_STATUS_SWITCH_PROTOCOLS: u32 = 101;
pub const HTTP_STATUS_UNSUPPORTED_MEDIA: u32 = 415;
pub const HTTP_STATUS_URI_TOO_LONG: u32 = 414;
pub const HTTP_STATUS_USE_PROXY: u32 = 305;
pub const HTTP_STATUS_VERSION_NOT_SUP: u32 = 505;
pub const HTTP_STATUS_WEBDAV_MULTI_STATUS: u32 = 207;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
pub const ICU_BROWSER_MODE: u32 = 33554432;
pub const ICU_DECODE: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(268435456);
pub const ICU_ENCODE_PERCENT: u32 = 4096;
pub const ICU_ENCODE_SPACES_ONLY: u32 = 67108864;
pub const ICU_ESCAPE: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(2147483648);
pub const ICU_ESCAPE_AUTHORITY: u32 = 8192;
pub const ICU_INCLUDE_DEFAULT_PORT: u32 = 32768;
pub const ICU_NO_ENCODE: u32 = 536870912;
pub const ICU_NO_META: u32 = 134217728;
pub const ICU_REJECT_USERPWD: WIN_HTTP_CREATE_URL_FLAGS = WIN_HTTP_CREATE_URL_FLAGS(16384);
pub const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;
pub const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
pub const INTERNET_DEFAULT_PORT: u16 = 0;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWinHttpRequest, IWinHttpRequest_Vtbl, 0x016fe2ec_b2c8_45f8_b23b_39e53a75396b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWinHttpRequest {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWinHttpRequest, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWinHttpRequest {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProxy(&self, proxysetting: i32, proxyserver: &super::super::System::Variant::VARIANT, bypasslist: &super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProxy)(windows_core::Interface::as_raw(self), proxysetting, core::mem::transmute_copy(proxyserver), core::mem::transmute_copy(bypasslist)) }
    }
    pub unsafe fn SetCredentials(&self, username: &windows_core::BSTR, password: &windows_core::BSTR, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(username), core::mem::transmute_copy(password), flags) }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open(&self, method: &windows_core::BSTR, url: &windows_core::BSTR, r#async: &super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(method), core::mem::transmute_copy(url), core::mem::transmute_copy(r#async)) }
    }
    pub unsafe fn SetRequestHeader(&self, header: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequestHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(header), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn GetResponseHeader(&self, header: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResponseHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(header), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAllResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllResponseHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send(&self, body: &super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(body)) }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ResponseText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResponseBody(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseBody)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResponseStream(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Option(&self, option: WinHttpRequestOption) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Option)(windows_core::Interface::as_raw(self), option, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_Option(&self, option: WinHttpRequestOption, value: &super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Option)(windows_core::Interface::as_raw(self), option, core::mem::transmute_copy(value)) }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn WaitForResponse(&self, timeout: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForResponse)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timeout), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimeouts)(windows_core::Interface::as_raw(self), resolvetimeout, connecttimeout, sendtimeout, receivetimeout) }
    }
    pub unsafe fn SetClientCertificate(&self, clientcertificate: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clientcertificate)) }
    }
    pub unsafe fn SetAutoLogonPolicy(&self, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoLogonPolicy)(windows_core::Interface::as_raw(self), autologonpolicy) }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWinHttpRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProxy: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProxy: usize,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Open: usize,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResponseHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllResponseHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResponseBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResponseBody: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResponseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResponseStream: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Option: unsafe extern "system" fn(*mut core::ffi::c_void, WinHttpRequestOption, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Option: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_Option: unsafe extern "system" fn(*mut core::ffi::c_void, WinHttpRequestOption, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_Option: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub WaitForResponse: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    WaitForResponse: usize,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAutoLogonPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, WinHttpRequestAutoLogonPolicy) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWinHttpRequest_Impl: super::super::System::Com::IDispatch_Impl {
    fn SetProxy(&self, proxysetting: i32, proxyserver: &super::super::System::Variant::VARIANT, bypasslist: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetCredentials(&self, username: &windows_core::BSTR, password: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn Open(&self, method: &windows_core::BSTR, url: &windows_core::BSTR, r#async: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetRequestHeader(&self, header: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetResponseHeader(&self, header: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetAllResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Send(&self, body: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Status(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResponseText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResponseBody(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ResponseStream(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_Option(&self, option: WinHttpRequestOption) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_Option(&self, option: WinHttpRequestOption, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn WaitForResponse(&self, timeout: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn SetTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::Result<()>;
    fn SetClientCertificate(&self, clientcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetAutoLogonPolicy(&self, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWinHttpRequest_Vtbl {
    pub const fn new<Identity: IWinHttpRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProxy<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxysetting: i32, proxyserver: super::super::System::Variant::VARIANT, bypasslist: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetProxy(this, core::mem::transmute_copy(&proxysetting), core::mem::transmute(&proxyserver), core::mem::transmute(&bypasslist)).into()
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::ffi::c_void, password: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetCredentials(this, core::mem::transmute(&username), core::mem::transmute(&password), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *mut core::ffi::c_void, url: *mut core::ffi::c_void, r#async: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::Open(this, core::mem::transmute(&method), core::mem::transmute(&url), core::mem::transmute(&r#async)).into()
            }
        }
        unsafe extern "system" fn SetRequestHeader<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, header: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetRequestHeader(this, core::mem::transmute(&header), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetResponseHeader<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, header: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::GetResponseHeader(this, core::mem::transmute(&header)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, headers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::GetAllResponseHeaders(this) {
                    Ok(ok__) => {
                        headers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Send<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::Send(this, core::mem::transmute(&body)).into()
            }
        }
        unsafe extern "system" fn Status<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::Status(this) {
                    Ok(ok__) => {
                        status.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::StatusText(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseText<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::ResponseText(this) {
                    Ok(ok__) => {
                        body.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseBody<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::ResponseBody(this) {
                    Ok(ok__) => {
                        body.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseStream<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::ResponseStream(this) {
                    Ok(ok__) => {
                        body.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Option<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: WinHttpRequestOption, value: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::get_Option(this, core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Option<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: WinHttpRequestOption, value: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::put_Option(this, core::mem::transmute_copy(&option), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn WaitForResponse<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: super::super::System::Variant::VARIANT, succeeded: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinHttpRequest_Impl::WaitForResponse(this, core::mem::transmute(&timeout)) {
                    Ok(ok__) => {
                        succeeded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Abort<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn SetTimeouts<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetTimeouts(this, core::mem::transmute_copy(&resolvetimeout), core::mem::transmute_copy(&connecttimeout), core::mem::transmute_copy(&sendtimeout), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificate<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientcertificate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetClientCertificate(this, core::mem::transmute(&clientcertificate)).into()
            }
        }
        unsafe extern "system" fn SetAutoLogonPolicy<Identity: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequest_Impl::SetAutoLogonPolicy(this, core::mem::transmute_copy(&autologonpolicy)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetProxy: SetProxy::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
            ResponseText: ResponseText::<Identity, OFFSET>,
            ResponseBody: ResponseBody::<Identity, OFFSET>,
            ResponseStream: ResponseStream::<Identity, OFFSET>,
            get_Option: get_Option::<Identity, OFFSET>,
            put_Option: put_Option::<Identity, OFFSET>,
            WaitForResponse: WaitForResponse::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            SetTimeouts: SetTimeouts::<Identity, OFFSET>,
            SetClientCertificate: SetClientCertificate::<Identity, OFFSET>,
            SetAutoLogonPolicy: SetAutoLogonPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinHttpRequest as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWinHttpRequest {}
windows_core::imp::define_interface!(IWinHttpRequestEvents, IWinHttpRequestEvents_Vtbl, 0xf97f4e15_b787_4212_80d1_d380cbbf982e);
windows_core::imp::interface_hierarchy!(IWinHttpRequestEvents, windows_core::IUnknown);
impl IWinHttpRequestEvents {
    pub unsafe fn OnResponseStart(&self, status: i32, contenttype: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).OnResponseStart)(windows_core::Interface::as_raw(self), status, core::mem::transmute_copy(contenttype));
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnResponseDataAvailable(&self, data: *const *const super::super::System::Com::SAFEARRAY) {
        unsafe {
            (windows_core::Interface::vtable(self).OnResponseDataAvailable)(windows_core::Interface::as_raw(self), data);
        }
    }
    pub unsafe fn OnResponseFinished(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnResponseFinished)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnError(&self, errornumber: i32, errordescription: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), errornumber, core::mem::transmute_copy(errordescription));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinHttpRequestEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnResponseStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_System_Com")]
    pub OnResponseDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const super::super::System::Com::SAFEARRAY),
    #[cfg(not(feature = "Win32_System_Com"))]
    OnResponseDataAvailable: usize,
    pub OnResponseFinished: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void),
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWinHttpRequestEvents_Impl: windows_core::IUnknownImpl {
    fn OnResponseStart(&self, status: i32, contenttype: &windows_core::BSTR);
    fn OnResponseDataAvailable(&self, data: *const *const super::super::System::Com::SAFEARRAY);
    fn OnResponseFinished(&self);
    fn OnError(&self, errornumber: i32, errordescription: &windows_core::BSTR);
}
#[cfg(feature = "Win32_System_Com")]
impl IWinHttpRequestEvents_Vtbl {
    pub const fn new<Identity: IWinHttpRequestEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnResponseStart<Identity: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: i32, contenttype: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequestEvents_Impl::OnResponseStart(this, core::mem::transmute_copy(&status), core::mem::transmute(&contenttype));
            }
        }
        unsafe extern "system" fn OnResponseDataAvailable<Identity: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *const *const super::super::System::Com::SAFEARRAY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequestEvents_Impl::OnResponseDataAvailable(this, core::mem::transmute_copy(&data));
            }
        }
        unsafe extern "system" fn OnResponseFinished<Identity: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequestEvents_Impl::OnResponseFinished(this);
            }
        }
        unsafe extern "system" fn OnError<Identity: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errornumber: i32, errordescription: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinHttpRequestEvents_Impl::OnError(this, core::mem::transmute_copy(&errornumber), core::mem::transmute(&errordescription));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResponseStart: OnResponseStart::<Identity, OFFSET>,
            OnResponseDataAvailable: OnResponseDataAvailable::<Identity, OFFSET>,
            OnResponseFinished: OnResponseFinished::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinHttpRequestEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWinHttpRequestEvents {}
pub const NETWORKING_KEY_BUFSIZE: u32 = 128;
pub const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096;
pub const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192;
pub const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: u32 = 512;
pub const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 256;
pub const SECURITY_FLAG_SECURE: u32 = 1;
pub const SECURITY_FLAG_STRENGTH_MEDIUM: u32 = 1073741824;
pub const SECURITY_FLAG_STRENGTH_STRONG: u32 = 536870912;
pub const SECURITY_FLAG_STRENGTH_WEAK: u32 = 268435456;
pub const SecureProtocol_ALL: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(168);
pub const SecureProtocol_SSL2: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(8);
pub const SecureProtocol_SSL3: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(32);
pub const SecureProtocol_TLS1: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(128);
pub const SecureProtocol_TLS1_1: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(512);
pub const SecureProtocol_TLS1_2: WinHttpRequestSecureProtocols = WinHttpRequestSecureProtocols(2048);
pub const SslErrorFlag_CertCNInvalid: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(4096);
pub const SslErrorFlag_CertDateInvalid: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(8192);
pub const SslErrorFlag_CertWrongUsage: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(512);
pub const SslErrorFlag_Ignore_All: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(13056);
pub const SslErrorFlag_UnknownCA: WinHttpRequestSslErrorFlags = WinHttpRequestSslErrorFlags(256);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URL_COMPONENTS {
    pub dwStructSize: u32,
    pub lpszScheme: windows_core::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: WINHTTP_INTERNET_SCHEME,
    pub lpszHostName: windows_core::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: windows_core::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_core::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_core::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_core::PWSTR,
    pub dwExtraInfoLength: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_ACCESS_TYPE(pub u32);
pub const WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(4);
pub const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(0);
pub const WINHTTP_ACCESS_TYPE_NAMED_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(3);
pub const WINHTTP_ACCESS_TYPE_NO_PROXY: WINHTTP_ACCESS_TYPE = WINHTTP_ACCESS_TYPE(1);
pub const WINHTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760;
pub const WINHTTP_ADDREQ_FLAG_ADD: u32 = 536870912;
pub const WINHTTP_ADDREQ_FLAG_ADD_IF_NEW: u32 = 268435456;
pub const WINHTTP_ADDREQ_FLAG_COALESCE: u32 = 1073741824;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: u32 = 1073741824;
pub const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: u32 = 16777216;
pub const WINHTTP_ADDREQ_FLAG_REPLACE: u32 = 2147483648;
pub const WINHTTP_ADDREQ_INDEX_MASK: u32 = 65535;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
pub const WINHTTP_AUTH_SCHEME_BASIC: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(1);
pub const WINHTTP_AUTH_SCHEME_DIGEST: u32 = 8;
pub const WINHTTP_AUTH_SCHEME_NEGOTIATE: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(16);
pub const WINHTTP_AUTH_SCHEME_NTLM: WINHTTP_CREDS_AUTHSCHEME = WINHTTP_CREDS_AUTHSCHEME(2);
pub const WINHTTP_AUTH_SCHEME_PASSPORT: u32 = 4;
pub const WINHTTP_AUTH_TARGET_PROXY: u32 = 1;
pub const WINHTTP_AUTH_TARGET_SERVER: u32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_DEFAULT: u32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH: u32 = 2;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_LOW: u32 = 1;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MAX: u32 = 3;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_MEDIUM: u32 = 0;
pub const WINHTTP_AUTOLOGON_SECURITY_LEVEL_PROXY_ONLY: u32 = 3;
pub const WINHTTP_AUTOPROXY_ALLOW_AUTOCONFIG: u32 = 256;
pub const WINHTTP_AUTOPROXY_ALLOW_CM: u32 = 1024;
pub const WINHTTP_AUTOPROXY_ALLOW_STATIC: u32 = 512;
pub const WINHTTP_AUTOPROXY_AUTO_DETECT: u32 = 1;
pub const WINHTTP_AUTOPROXY_CONFIG_URL: u32 = 2;
pub const WINHTTP_AUTOPROXY_HOST_KEEPCASE: u32 = 4;
pub const WINHTTP_AUTOPROXY_HOST_LOWERCASE: u32 = 8;
pub const WINHTTP_AUTOPROXY_NO_CACHE_CLIENT: u32 = 524288;
pub const WINHTTP_AUTOPROXY_NO_CACHE_SVC: u32 = 1048576;
pub const WINHTTP_AUTOPROXY_NO_DIRECTACCESS: u32 = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dwFlags: u32,
    pub dwAutoDetectFlags: u32,
    pub lpszAutoConfigUrl: windows_core::PCWSTR,
    pub lpvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub fAutoLogonIfChallenged: windows_core::BOOL,
}
impl Default for WINHTTP_AUTOPROXY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_AUTOPROXY_RUN_INPROCESS: u32 = 65536;
pub const WINHTTP_AUTOPROXY_RUN_OUTPROCESS_ONLY: u32 = 131072;
pub const WINHTTP_AUTOPROXY_SORT_RESULTS: u32 = 4194304;
pub const WINHTTP_AUTOPROXY_USE_INTERFACE_CONFIG: u32 = 2048;
pub const WINHTTP_AUTO_DETECT_TYPE_DHCP: u32 = 1;
pub const WINHTTP_AUTO_DETECT_TYPE_DNS_A: u32 = 2;
pub const WINHTTP_CALLBACK_FLAG_ALL_NOTIFICATIONS: u32 = 4294967295;
pub const WINHTTP_CALLBACK_FLAG_DATA_AVAILABLE: u32 = 262144;
pub const WINHTTP_CALLBACK_FLAG_DETECTING_PROXY: u32 = 4096;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYFORURL_COMPLETE: u32 = 16777216;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYSETTINGS_COMPLETE: u32 = 134217728;
pub const WINHTTP_CALLBACK_FLAG_HEADERS_AVAILABLE: u32 = 131072;
pub const WINHTTP_CALLBACK_FLAG_INTERMEDIATE_RESPONSE: u32 = 32768;
pub const WINHTTP_CALLBACK_FLAG_READ_COMPLETE: u32 = 524288;
pub const WINHTTP_CALLBACK_FLAG_REDIRECT: u32 = 16384;
pub const WINHTTP_CALLBACK_FLAG_REQUEST_ERROR: u32 = 2097152;
pub const WINHTTP_CALLBACK_FLAG_SECURE_FAILURE: u32 = 65536;
pub const WINHTTP_CALLBACK_FLAG_SENDREQUEST_COMPLETE: u32 = 4194304;
pub const WINHTTP_CALLBACK_FLAG_WRITE_COMPLETE: u32 = 1048576;
pub const WINHTTP_CALLBACK_STATUS_CLOSE_COMPLETE: u32 = 33554432;
pub const WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION: u32 = 256;
pub const WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER: u32 = 8;
pub const WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER: u32 = 4;
pub const WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED: u32 = 512;
pub const WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE: u32 = 262144;
pub const WINHTTP_CALLBACK_STATUS_DETECTING_PROXY: u32 = 4096;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_CN_INVALID: u32 = 16;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_DATE_INVALID: u32 = 32;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REVOKED: u32 = 4;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_REV_FAILED: u32 = 1;
pub const WINHTTP_CALLBACK_STATUS_FLAG_CERT_WRONG_USAGE: u32 = 64;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CA: u32 = 8;
pub const WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CERT: u32 = 2;
pub const WINHTTP_CALLBACK_STATUS_FLAG_SECURITY_CHANNEL_ERROR: u32 = 2147483648;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE: u32 = 16777216;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE: u32 = 134217728;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING: u32 = 2048;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CREATED: u32 = 1024;
pub const WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE: u32 = 131072;
pub const WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE: u32 = 32768;
pub const WINHTTP_CALLBACK_STATUS_NAME_RESOLVED: u32 = 2;
pub const WINHTTP_CALLBACK_STATUS_READ_COMPLETE: u32 = 524288;
pub const WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE: u32 = 64;
pub const WINHTTP_CALLBACK_STATUS_REDIRECT: u32 = 16384;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_ERROR: u32 = 2097152;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_SENT: u32 = 32;
pub const WINHTTP_CALLBACK_STATUS_RESOLVING_NAME: u32 = 1;
pub const WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED: u32 = 128;
pub const WINHTTP_CALLBACK_STATUS_SECURE_FAILURE: u32 = 65536;
pub const WINHTTP_CALLBACK_STATUS_SENDING_REQUEST: u32 = 16;
pub const WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE: u32 = 4194304;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_READ_COMPLETE: u32 = 536870912;
pub const WINHTTP_CALLBACK_STATUS_SETTINGS_WRITE_COMPLETE: u32 = 268435456;
pub const WINHTTP_CALLBACK_STATUS_SHUTDOWN_COMPLETE: u32 = 67108864;
pub const WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE: u32 = 1048576;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CERTIFICATE_INFO {
    pub ftExpiry: super::super::Foundation::FILETIME,
    pub ftStart: super::super::Foundation::FILETIME,
    pub lpszSubjectInfo: windows_core::PWSTR,
    pub lpszIssuerInfo: windows_core::PWSTR,
    pub lpszProtocolName: windows_core::PWSTR,
    pub lpszSignatureAlgName: windows_core::PWSTR,
    pub lpszEncryptionAlgName: windows_core::PWSTR,
    pub dwKeySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CONNECTION_GROUP {
    pub cConnections: u32,
    pub guidGroup: windows_core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
    pub RemoteAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
    pub RemoteAddress: super::WinSock::SOCKADDR_STORAGE,
}
pub const WINHTTP_CONNECTION_RETRY_CONDITION_408: u32 = 1;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE: u32 = 2;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION: u32 = 4;
pub const WINHTTP_CONNS_PER_SERVER_UNLIMITED: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CREDS {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: WINHTTP_CREDS_AUTHSCHEME,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_CREDS_AUTHSCHEME(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CREDS_EX {
    pub lpszUserName: windows_core::PSTR,
    pub lpszPassword: windows_core::PSTR,
    pub lpszRealm: windows_core::PSTR,
    pub dwAuthScheme: WINHTTP_CREDS_AUTHSCHEME,
    pub lpszHostName: windows_core::PSTR,
    pub dwPort: u32,
    pub lpszUrl: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub fAutoDetect: windows_core::BOOL,
    pub lpszAutoConfigUrl: windows_core::PWSTR,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
pub const WINHTTP_DECOMPRESSION_FLAG_DEFLATE: u32 = 2;
pub const WINHTTP_DECOMPRESSION_FLAG_GZIP: u32 = 1;
pub const WINHTTP_DISABLE_AUTHENTICATION: u32 = 4;
pub const WINHTTP_DISABLE_COOKIES: u32 = 1;
pub const WINHTTP_DISABLE_KEEP_ALIVE: u32 = 8;
pub const WINHTTP_DISABLE_PASSPORT_AUTH: u32 = 0;
pub const WINHTTP_DISABLE_PASSPORT_KEYRING: u32 = 536870912;
pub const WINHTTP_DISABLE_REDIRECTS: u32 = 2;
pub const WINHTTP_DISABLE_SPN_SERVER_PORT: u32 = 0;
pub const WINHTTP_ENABLE_PASSPORT_AUTH: u32 = 268435456;
pub const WINHTTP_ENABLE_PASSPORT_KEYRING: u32 = 1073741824;
pub const WINHTTP_ENABLE_SPN_SERVER_PORT: u32 = 1;
pub const WINHTTP_ENABLE_SSL_REVERT_IMPERSONATION: u32 = 2;
pub const WINHTTP_ENABLE_SSL_REVOCATION: u32 = 1;
pub const WINHTTP_ERROR_BASE: u32 = 12000;
pub const WINHTTP_ERROR_LAST: u32 = 12193;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_EXTENDED_HEADER {
    pub Anonymous1: WINHTTP_EXTENDED_HEADER_0,
    pub Anonymous2: WINHTTP_EXTENDED_HEADER_1,
}
impl Default for WINHTTP_EXTENDED_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_0 {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
impl Default for WINHTTP_EXTENDED_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_1 {
    pub pwszValue: windows_core::PCWSTR,
    pub pszValue: windows_core::PCSTR,
}
impl Default for WINHTTP_EXTENDED_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_EXTENDED_HEADER_FLAG_UNICODE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_FAILED_CONNECTION_RETRIES {
    pub dwMaxRetries: u32,
    pub dwAllowedRetryConditions: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_FAST_FORWARDING_STATE(pub i32);
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAST_FORWARDING_STATUS {
    pub TransferState: WINHTTP_FAST_FORWARDING_STATE,
    pub NtStatus: i32,
    pub dwError: u32,
    pub ullBytesTransferred: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_FAST_FORWARDING_STATUS {
    pub TransferState: WINHTTP_FAST_FORWARDING_STATE,
    pub NtStatus: i32,
    pub dwError: u32,
    pub ullBytesTransferred: u64,
}
pub const WINHTTP_FEATURE_ADD_REQUEST_HEADERS_EX: u32 = 46;
pub const WINHTTP_FEATURE_BACKGROUND_CONNECTIONS: u32 = 34;
pub const WINHTTP_FEATURE_CONNECTION_GUID: u32 = 38;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V0: u32 = 3;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V1: u32 = 12;
pub const WINHTTP_FEATURE_CONNECTION_STATS_V2: u32 = 89;
pub const WINHTTP_FEATURE_DISABLE_AIA_FLAG: u32 = 91;
pub const WINHTTP_FEATURE_DISABLE_CERT_CHAIN_BUILDING: u32 = 33;
pub const WINHTTP_FEATURE_DISABLE_GLOBAL_POOLING: u32 = 76;
pub const WINHTTP_FEATURE_DISABLE_PROXY_AUTH_SCHEMES: u32 = 74;
pub const WINHTTP_FEATURE_DISABLE_SECURE_PROTOCOL_FALLBACK: u32 = 6;
pub const WINHTTP_FEATURE_DISABLE_STREAM_QUEUE: u32 = 1;
pub const WINHTTP_FEATURE_DSCP_TAG: u32 = 92;
pub const WINHTTP_FEATURE_ENABLE_HTTP2_PLUS_CLIENT_CERT: u32 = 23;
pub const WINHTTP_FEATURE_ERROR_LOG_GUID: u32 = 83;
pub const WINHTTP_FEATURE_EXPIRE_CONNECTION: u32 = 5;
pub const WINHTTP_FEATURE_EXTENDED_HEADER_FLAG_UNICODE: u32 = 54;
pub const WINHTTP_FEATURE_FAILED_CONNECTION_RETRIES: u32 = 24;
pub const WINHTTP_FEATURE_FAST_FORWARD_RESPONSE: u32 = 90;
pub const WINHTTP_FEATURE_FIRST_AVAILABLE_CONNECTION: u32 = 35;
pub const WINHTTP_FEATURE_FLAG_AUTOMATIC_CHUNKING: u32 = 59;
pub const WINHTTP_FEATURE_FLAG_SECURE_DEFAULTS: u32 = 53;
pub const WINHTTP_FEATURE_FREE_QUERY_CONNECTION_GROUP_RESULT: u32 = 51;
pub const WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX: u32 = 77;
pub const WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX_XBOX: u32 = 95;
pub const WINHTTP_FEATURE_HTTP11_DOWNGRADE_TTL: u32 = 93;
pub const WINHTTP_FEATURE_HTTP2_KEEPALIVE: u32 = 26;
pub const WINHTTP_FEATURE_HTTP2_PLUS_TRANSFER_ENCODING: u32 = 31;
pub const WINHTTP_FEATURE_HTTP2_RECEIVE_WINDOW: u32 = 43;
pub const WINHTTP_FEATURE_HTTP3_HANDSHAKE_TIMEOUT: u32 = 70;
pub const WINHTTP_FEATURE_HTTP3_INITIAL_RTT: u32 = 71;
pub const WINHTTP_FEATURE_HTTP3_KEEPALIVE: u32 = 69;
pub const WINHTTP_FEATURE_HTTP3_STREAM_ERROR_CODE: u32 = 72;
pub const WINHTTP_FEATURE_HTTP_PROTOCOL_REQUIRED: u32 = 7;
pub const WINHTTP_FEATURE_IGNORE_CERT_REVOCATION_OFFLINE: u32 = 17;
pub const WINHTTP_FEATURE_IPV6_FAST_FALLBACK: u32 = 2;
pub const WINHTTP_FEATURE_IS_FEATURE_SUPPORTED: u32 = 44;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID: u32 = 39;
pub const WINHTTP_FEATURE_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: u32 = 61;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP: u32 = 50;
pub const WINHTTP_FEATURE_QUERY_CONNECTION_GROUP_FLAG_INSECURE: u32 = 60;
pub const WINHTTP_FEATURE_QUERY_EX_ALL_HEADERS: u32 = 62;
pub const WINHTTP_FEATURE_QUERY_FLAG_TRAILERS: u32 = 55;
pub const WINHTTP_FEATURE_QUERY_FLAG_WIRE_ENCODING: u32 = 56;
pub const WINHTTP_FEATURE_QUERY_HEADERS_EX: u32 = 49;
pub const WINHTTP_FEATURE_QUIC_STATS: u32 = 66;
pub const WINHTTP_FEATURE_QUIC_STATS_V2: u32 = 79;
pub const WINHTTP_FEATURE_QUIC_STREAM_STATS: u32 = 81;
pub const WINHTTP_FEATURE_READ_DATA_EX: u32 = 48;
pub const WINHTTP_FEATURE_READ_DATA_EX_FLAG_FILL_BUFFER: u32 = 63;
pub const WINHTTP_FEATURE_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 30;
pub const WINHTTP_FEATURE_REQUEST_ANNOTATION: u32 = 73;
pub const WINHTTP_FEATURE_REQUEST_STATS: u32 = 8;
pub const WINHTTP_FEATURE_REQUEST_TIMES: u32 = 4;
pub const WINHTTP_FEATURE_REQUIRE_STREAM_END: u32 = 22;
pub const WINHTTP_FEATURE_RESOLUTION_HOSTNAME: u32 = 27;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG: u32 = 32;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: u32 = 58;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: u32 = 65;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: u32 = 57;
pub const WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: u32 = 64;
pub const WINHTTP_FEATURE_REVERT_IMPERSONATION_SERVER_CERT: u32 = 75;
pub const WINHTTP_FEATURE_SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS: u32 = 52;
pub const WINHTTP_FEATURE_SECURITY_INFO: u32 = 13;
pub const WINHTTP_FEATURE_SERVER_CERT_CHAIN_CONTEXT: u32 = 9;
pub const WINHTTP_FEATURE_SESSION_ERROR_LOG_GUID: u32 = 94;
pub const WINHTTP_FEATURE_SESSION_SCH_CRED: u32 = 78;
pub const WINHTTP_FEATURE_SET_PROXY_SETINGS_PER_USER: u32 = 47;
pub const WINHTTP_FEATURE_SET_TOKEN_BINDING: u32 = 28;
pub const WINHTTP_FEATURE_STREAM_ERROR_CODE: u32 = 21;
pub const WINHTTP_FEATURE_TCP_FAST_OPEN: u32 = 15;
pub const WINHTTP_FEATURE_TCP_KEEPALIVE: u32 = 14;
pub const WINHTTP_FEATURE_TCP_PRIORITY_STATUS: u32 = 37;
pub const WINHTTP_FEATURE_TLS_FALSE_START: u32 = 16;
pub const WINHTTP_FEATURE_TLS_PROTOCOL_INSECURE_FALLBACK: u32 = 20;
pub const WINHTTP_FEATURE_TOKEN_BINDING_PUBLIC_KEY: u32 = 29;
pub const WINHTTP_FEATURE_UPGRADE_TO_PROTOCOL: u32 = 88;
pub const WINHTTP_FEATURE_URL_INCLUDE_DEFAULT_PORT: u32 = 80;
pub const WINHTTP_FEATURE_USE_LOOKASIDE: u32 = 82;
pub const WINHTTP_FLAG_ASYNC: u32 = 268435456;
pub const WINHTTP_FLAG_AUTOMATIC_CHUNKING: u32 = 512;
pub const WINHTTP_FLAG_BYPASS_PROXY_CACHE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(256);
pub const WINHTTP_FLAG_ESCAPE_DISABLE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(64);
pub const WINHTTP_FLAG_ESCAPE_DISABLE_QUERY: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(128);
pub const WINHTTP_FLAG_ESCAPE_PERCENT: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(4);
pub const WINHTTP_FLAG_NULL_CODEPAGE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(8);
pub const WINHTTP_FLAG_REFRESH: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(256);
pub const WINHTTP_FLAG_SECURE: WINHTTP_OPEN_REQUEST_FLAGS = WINHTTP_OPEN_REQUEST_FLAGS(8388608);
pub const WINHTTP_FLAG_SECURE_DEFAULTS: u32 = 805306368;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL2: u32 = 8;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_SSL3: u32 = 32;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1: u32 = 128;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_1: u32 = 512;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_2: u32 = 2048;
pub const WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_3: u32 = 8192;
pub const WINHTTP_HANDLE_TYPE_CONNECT: u32 = 2;
pub const WINHTTP_HANDLE_TYPE_PROTOCOL: u32 = 6;
pub const WINHTTP_HANDLE_TYPE_PROXY_RESOLVER: u32 = 4;
pub const WINHTTP_HANDLE_TYPE_REQUEST: u32 = 3;
pub const WINHTTP_HANDLE_TYPE_SESSION: u32 = 1;
pub const WINHTTP_HANDLE_TYPE_WEBSOCKET: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINHTTP_HEADER_NAME {
    pub pwszName: windows_core::PCWSTR,
    pub pszName: windows_core::PCSTR,
}
impl Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_HOST_CONNECTION_GROUP {
    pub pwszHost: windows_core::PCWSTR,
    pub cConnectionGroups: u32,
    pub pConnectionGroups: *mut WINHTTP_CONNECTION_GROUP,
}
impl Default for WINHTTP_HOST_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_HTTP2_RECEIVE_WINDOW {
    pub ulStreamWindow: u32,
    pub ulStreamWindowUpdateDelta: u32,
}
pub const WINHTTP_IGNORE_REQUEST_TOTAL_LENGTH: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_INTERNET_SCHEME(pub i32);
pub const WINHTTP_INTERNET_SCHEME_FTP: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(3);
pub const WINHTTP_INTERNET_SCHEME_HTTP: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(1);
pub const WINHTTP_INTERNET_SCHEME_HTTPS: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(2);
pub const WINHTTP_INTERNET_SCHEME_SOCKS: WINHTTP_INTERNET_SCHEME = WINHTTP_INTERNET_SCHEME(4);
pub const WINHTTP_LAST_OPTION: u32 = 212;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_core::GUID,
    pub ullFlags: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_core::GUID,
    pub ullFlags: u64,
}
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAGS_MASK: u32 = 1;
pub const WINHTTP_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_OPEN_REQUEST_FLAGS(pub u32);
impl WINHTTP_OPEN_REQUEST_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WINHTTP_OPTION_ASSURED_NON_BLOCKING_CALLBACKS: u32 = 111;
pub const WINHTTP_OPTION_AUTOLOGON_POLICY: u32 = 77;
pub const WINHTTP_OPTION_BACKGROUND_CONNECTIONS: u32 = 172;
pub const WINHTTP_OPTION_CALLBACK: u32 = 1;
pub const WINHTTP_OPTION_CLIENT_CERT_CONTEXT: u32 = 47;
pub const WINHTTP_OPTION_CLIENT_CERT_ISSUER_LIST: u32 = 94;
pub const WINHTTP_OPTION_CODEPAGE: u32 = 68;
pub const WINHTTP_OPTION_CONFIGURE_PASSPORT_AUTH: u32 = 83;
pub const WINHTTP_OPTION_CONNECTION_FILTER: u32 = 131;
pub const WINHTTP_OPTION_CONNECTION_GUID: u32 = 178;
pub const WINHTTP_OPTION_CONNECTION_INFO: u32 = 93;
pub const WINHTTP_OPTION_CONNECTION_STATS_V0: u32 = 141;
pub const WINHTTP_OPTION_CONNECTION_STATS_V1: u32 = 150;
pub const WINHTTP_OPTION_CONNECTION_STATS_V2: u32 = 208;
pub const WINHTTP_OPTION_CONNECT_RETRIES: u32 = 4;
pub const WINHTTP_OPTION_CONNECT_TIMEOUT: u32 = 3;
pub const WINHTTP_OPTION_CONTEXT_VALUE: u32 = 45;
pub const WINHTTP_OPTION_DECOMPRESSION: u32 = 118;
pub const WINHTTP_OPTION_DISABLE_CERT_CHAIN_BUILDING: u32 = 171;
pub const WINHTTP_OPTION_DISABLE_FEATURE: u32 = 63;
pub const WINHTTP_OPTION_DISABLE_GLOBAL_POOLING: u32 = 195;
pub const WINHTTP_OPTION_DISABLE_PROXY_AUTH_SCHEMES: u32 = 193;
pub const WINHTTP_OPTION_DISABLE_SECURE_PROTOCOL_FALLBACK: u32 = 144;
pub const WINHTTP_OPTION_DISABLE_STREAM_QUEUE: u32 = 139;
pub const WINHTTP_OPTION_DSCP_TAG: u32 = 210;
pub const WINHTTP_OPTION_ENABLETRACING: u32 = 85;
pub const WINHTTP_OPTION_ENABLE_FAST_FORWARDING: u32 = 205;
pub const WINHTTP_OPTION_ENABLE_FEATURE: u32 = 79;
pub const WINHTTP_OPTION_ENABLE_HTTP2_PLUS_CLIENT_CERT: u32 = 161;
pub const WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 133;
pub const WINHTTP_OPTION_ENCODE_EXTRA: u32 = 138;
pub const WINHTTP_OPTION_ERROR_LOG_GUID: u32 = 204;
pub const WINHTTP_OPTION_EXPIRE_CONNECTION: u32 = 143;
pub const WINHTTP_OPTION_EXTENDED_ERROR: u32 = 24;
pub const WINHTTP_OPTION_FAILED_CONNECTION_RETRIES: u32 = 162;
pub const WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_DATA: u32 = 206;
pub const WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_STATUS: u32 = 209;
pub const WINHTTP_OPTION_FEATURE_SUPPORTED: u32 = 184;
pub const WINHTTP_OPTION_FIRST_AVAILABLE_CONNECTION: u32 = 173;
pub const WINHTTP_OPTION_GLOBAL_PROXY_CREDS: u32 = 97;
pub const WINHTTP_OPTION_GLOBAL_SERVER_CREDS: u32 = 98;
pub const WINHTTP_OPTION_HANDLE_TYPE: u32 = 9;
pub const WINHTTP_OPTION_HTTP11_DOWNGRADE_TTL: u32 = 211;
pub const WINHTTP_OPTION_HTTP2_KEEPALIVE: u32 = 164;
pub const WINHTTP_OPTION_HTTP2_PLUS_TRANSFER_ENCODING: u32 = 169;
pub const WINHTTP_OPTION_HTTP2_RECEIVE_WINDOW: u32 = 183;
pub const WINHTTP_OPTION_HTTP3_HANDSHAKE_TIMEOUT: u32 = 189;
pub const WINHTTP_OPTION_HTTP3_INITIAL_RTT: u32 = 190;
pub const WINHTTP_OPTION_HTTP3_KEEPALIVE: u32 = 188;
pub const WINHTTP_OPTION_HTTP3_STREAM_ERROR_CODE: u32 = 191;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_REQUIRED: u32 = 145;
pub const WINHTTP_OPTION_HTTP_PROTOCOL_USED: u32 = 134;
pub const WINHTTP_OPTION_HTTP_VERSION: u32 = 59;
pub const WINHTTP_OPTION_IGNORE_CERT_REVOCATION_OFFLINE: u32 = 155;
pub const WINHTTP_OPTION_IPV6_FAST_FALLBACK: u32 = 140;
pub const WINHTTP_OPTION_IS_PROXY_CONNECT_RESPONSE: u32 = 104;
pub const WINHTTP_OPTION_KDC_PROXY_SETTINGS: u32 = 136;
pub const WINHTTP_OPTION_MATCH_CONNECTION_GUID: u32 = 179;
pub const WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74;
pub const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: u32 = 73;
pub const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: u32 = 89;
pub const WINHTTP_OPTION_MAX_HTTP_STATUS_CONTINUE: u32 = 90;
pub const WINHTTP_OPTION_MAX_RESPONSE_DRAIN_SIZE: u32 = 92;
pub const WINHTTP_OPTION_MAX_RESPONSE_HEADER_SIZE: u32 = 91;
pub const WINHTTP_OPTION_NETWORK_INTERFACE_AFFINITY: u32 = 105;
pub const WINHTTP_OPTION_PARENT_HANDLE: u32 = 21;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_TEXT: u32 = 81;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_URL: u32 = 82;
pub const WINHTTP_OPTION_PASSPORT_RETURN_URL: u32 = 87;
pub const WINHTTP_OPTION_PASSPORT_SIGN_OUT: u32 = 86;
pub const WINHTTP_OPTION_PASSWORD: u32 = 4097;
pub const WINHTTP_OPTION_PROXY: u32 = 38;
pub const WINHTTP_OPTION_PROXY_DISABLE_SERVICE_CALLS: u32 = 137;
pub const WINHTTP_OPTION_PROXY_PASSWORD: u32 = 4099;
pub const WINHTTP_OPTION_PROXY_RESULT_ENTRY: u32 = 39;
pub const WINHTTP_OPTION_PROXY_SPN_USED: u32 = 107;
pub const WINHTTP_OPTION_PROXY_USERNAME: u32 = 4098;
pub const WINHTTP_OPTION_QUIC_STATS: u32 = 185;
pub const WINHTTP_OPTION_QUIC_STATS_V2: u32 = 200;
pub const WINHTTP_OPTION_QUIC_STREAM_STATS: u32 = 202;
pub const WINHTTP_OPTION_READ_BUFFER_SIZE: u32 = 12;
pub const WINHTTP_OPTION_RECEIVE_PROXY_CONNECT_RESPONSE: u32 = 103;
pub const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: u32 = 7;
pub const WINHTTP_OPTION_RECEIVE_TIMEOUT: u32 = 6;
pub const WINHTTP_OPTION_REDIRECT_POLICY: u32 = 88;
pub const WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS: u32 = 2;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DEFAULT: u32 = 1;
pub const WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP: u32 = 1;
pub const WINHTTP_OPTION_REDIRECT_POLICY_LAST: u32 = 2;
pub const WINHTTP_OPTION_REDIRECT_POLICY_NEVER: u32 = 0;
pub const WINHTTP_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 168;
pub const WINHTTP_OPTION_REJECT_USERPWD_IN_URL: u32 = 100;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION: u32 = 192;
pub const WINHTTP_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: u32 = 64000;
pub const WINHTTP_OPTION_REQUEST_PRIORITY: u32 = 58;
pub const WINHTTP_OPTION_REQUEST_STATS: u32 = 146;
pub const WINHTTP_OPTION_REQUEST_TIMES: u32 = 142;
pub const WINHTTP_OPTION_REQUIRE_STREAM_END: u32 = 160;
pub const WINHTTP_OPTION_RESOLUTION_HOSTNAME: u32 = 165;
pub const WINHTTP_OPTION_RESOLVER_CACHE_CONFIG: u32 = 170;
pub const WINHTTP_OPTION_RESOLVE_TIMEOUT: u32 = 2;
pub const WINHTTP_OPTION_REVERT_IMPERSONATION_SERVER_CERT: u32 = 194;
pub const WINHTTP_OPTION_SECURE_PROTOCOLS: u32 = 84;
pub const WINHTTP_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32;
pub const WINHTTP_OPTION_SECURITY_FLAGS: u32 = 31;
pub const WINHTTP_OPTION_SECURITY_INFO: u32 = 151;
pub const WINHTTP_OPTION_SECURITY_KEY_BITNESS: u32 = 36;
pub const WINHTTP_OPTION_SEND_TIMEOUT: u32 = 5;
pub const WINHTTP_OPTION_SERVER_CBT: u32 = 108;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_CACHE_ONLY: u32 = 199;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_FLAGS: u32 = 148;
pub const WINHTTP_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 147;
pub const WINHTTP_OPTION_SERVER_CERT_CONTEXT: u32 = 78;
pub const WINHTTP_OPTION_SERVER_SPN_USED: u32 = 106;
pub const WINHTTP_OPTION_SESSION_ERROR_LOG_GUID: u32 = 212;
pub const WINHTTP_OPTION_SET_TOKEN_BINDING: u32 = 166;
pub const WINHTTP_OPTION_SPN: u32 = 96;
pub const WINHTTP_OPTION_SPN_MASK: u32 = 1;
pub const WINHTTP_OPTION_STREAM_ERROR_CODE: u32 = 159;
pub const WINHTTP_OPTION_TCP_FAST_OPEN: u32 = 153;
pub const WINHTTP_OPTION_TCP_KEEPALIVE: u32 = 152;
pub const WINHTTP_OPTION_TCP_PRIORITY_HINT: u32 = 128;
pub const WINHTTP_OPTION_TCP_PRIORITY_STATUS: u32 = 177;
pub const WINHTTP_OPTION_TLS_FALSE_START: u32 = 154;
pub const WINHTTP_OPTION_TLS_PROTOCOL_INSECURE_FALLBACK: u32 = 158;
pub const WINHTTP_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 167;
pub const WINHTTP_OPTION_UNLOAD_NOTIFY_EVENT: u32 = 99;
pub const WINHTTP_OPTION_UNSAFE_HEADER_PARSING: u32 = 110;
pub const WINHTTP_OPTION_UPGRADE_TO_PROTOCOL: u32 = 207;
pub const WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET: u32 = 114;
pub const WINHTTP_OPTION_URL: u32 = 34;
pub const WINHTTP_OPTION_USERNAME: u32 = 4096;
pub const WINHTTP_OPTION_USER_AGENT: u32 = 41;
pub const WINHTTP_OPTION_USE_GLOBAL_SERVER_CREDENTIALS: u32 = 101;
pub const WINHTTP_OPTION_USE_LOOKASIDE: u32 = 203;
pub const WINHTTP_OPTION_USE_SESSION_SCH_CRED: u32 = 196;
pub const WINHTTP_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: u32 = 115;
pub const WINHTTP_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: u32 = 116;
pub const WINHTTP_OPTION_WEB_SOCKET_RECEIVE_BUFFER_SIZE: u32 = 122;
pub const WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE: u32 = 123;
pub const WINHTTP_OPTION_WORKER_THREAD_COUNT: u32 = 80;
pub const WINHTTP_OPTION_WRITE_BUFFER_SIZE: u32 = 13;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_PROTOCOL_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_PROTOCOL_OPERATION,
}
pub const WINHTTP_PROTOCOL_FLAG_HTTP2: u32 = 1;
pub const WINHTTP_PROTOCOL_FLAG_HTTP3: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_PROTOCOL_OPERATION(pub i32);
pub const WINHTTP_PROTOCOL_RECEIVE_OPERATION: WINHTTP_PROTOCOL_OPERATION = WINHTTP_PROTOCOL_OPERATION(1);
pub const WINHTTP_PROTOCOL_SEND_OPERATION: WINHTTP_PROTOCOL_OPERATION = WINHTTP_PROTOCOL_OPERATION(0);
pub type WINHTTP_PROXY_CHANGE_CALLBACK = Option<unsafe extern "system" fn(ullflags: u64, pvcontext: *const core::ffi::c_void)>;
pub const WINHTTP_PROXY_DISABLE_AUTH_LOCAL_SERVICE: u32 = 256;
pub const WINHTTP_PROXY_DISABLE_SCHEME_BASIC: u32 = 1;
pub const WINHTTP_PROXY_DISABLE_SCHEME_DIGEST: u32 = 2;
pub const WINHTTP_PROXY_DISABLE_SCHEME_KERBEROS: u32 = 8;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NEGOTIATE: u32 = 16;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NTLM: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_PROXY_INFO {
    pub dwAccessType: WINHTTP_ACCESS_TYPE,
    pub lpszProxy: windows_core::PWSTR,
    pub lpszProxyBypass: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_PROXY_NETWORKING_KEY {
    pub pbBuffer: [u8; 128],
}
impl Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_PROXY_NOTIFY_CHANGE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_PROXY_RESULT {
    pub cEntries: u32,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
}
impl Default for WINHTTP_PROXY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_PROXY_RESULT_ENTRY {
    pub fProxy: windows_core::BOOL,
    pub fBypass: windows_core::BOOL,
    pub ProxyScheme: WINHTTP_INTERNET_SCHEME,
    pub pwszProxy: windows_core::PWSTR,
    pub ProxyPort: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_PROXY_RESULT_EX {
    pub cEntries: u32,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
    pub hProxyDetectionHandle: super::super::Foundation::HANDLE,
    pub dwProxyInterfaceAffinity: u32,
}
impl Default for WINHTTP_PROXY_RESULT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_PROXY_SETTINGS {
    pub dwStructSize: u32,
    pub dwFlags: u32,
    pub dwCurrentSettingsVersion: u32,
    pub pwszConnectionName: windows_core::PWSTR,
    pub pwszProxy: windows_core::PWSTR,
    pub pwszProxyBypass: windows_core::PWSTR,
    pub pwszAutoconfigUrl: windows_core::PWSTR,
    pub pwszAutoconfigSecondaryUrl: windows_core::PWSTR,
    pub dwAutoDiscoveryFlags: u32,
    pub pwszLastKnownGoodAutoConfigUrl: windows_core::PWSTR,
    pub dwAutoconfigReloadDelayMins: u32,
    pub ftLastKnownDetectTime: super::super::Foundation::FILETIME,
    pub dwDetectedInterfaceIpCount: u32,
    pub pdwDetectedInterfaceIp: *mut u32,
    pub cNetworkKeys: u32,
    pub pNetworkKeys: *mut WINHTTP_PROXY_NETWORKING_KEY,
}
impl Default for WINHTTP_PROXY_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: u64,
    pub ullFlags: u64,
    pub pcwszAutoconfigUrl: windows_core::PCWSTR,
    pub pcwszProxy: windows_core::PCWSTR,
    pub pcwszSecureProxy: windows_core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *const windows_core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: u64,
    pub ullFlags: u64,
    pub pcwszAutoconfigUrl: windows_core::PCWSTR,
    pub pcwszProxy: windows_core::PCWSTR,
    pub pcwszSecureProxy: windows_core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *const windows_core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: windows_core::PCWSTR,
    pub pcwszProbeHost: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: windows_core::PCWSTR,
    pub pcwszProbeHost: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_PROXY_SETTINGS_TYPE(pub i32);
pub const WINHTTP_PROXY_TYPE_AUTO_DETECT: u32 = 8;
pub const WINHTTP_PROXY_TYPE_AUTO_PROXY_URL: u32 = 4;
pub const WINHTTP_PROXY_TYPE_DIRECT: u32 = 1;
pub const WINHTTP_PROXY_TYPE_PROXY: u32 = 2;
pub const WINHTTP_QUERY_ACCEPT: u32 = 24;
pub const WINHTTP_QUERY_ACCEPT_CHARSET: u32 = 25;
pub const WINHTTP_QUERY_ACCEPT_ENCODING: u32 = 26;
pub const WINHTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27;
pub const WINHTTP_QUERY_ACCEPT_RANGES: u32 = 42;
pub const WINHTTP_QUERY_AGE: u32 = 48;
pub const WINHTTP_QUERY_ALLOW: u32 = 7;
pub const WINHTTP_QUERY_AUTHENTICATION_INFO: u32 = 76;
pub const WINHTTP_QUERY_AUTHORIZATION: u32 = 28;
pub const WINHTTP_QUERY_CACHE_CONTROL: u32 = 49;
pub const WINHTTP_QUERY_CONNECTION: u32 = 23;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    pub cHosts: u32,
    pub pHostConnectionGroups: *mut WINHTTP_HOST_CONNECTION_GROUP,
}
impl Default for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_QUERY_CONTENT_BASE: u32 = 50;
pub const WINHTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4;
pub const WINHTTP_QUERY_CONTENT_DISPOSITION: u32 = 47;
pub const WINHTTP_QUERY_CONTENT_ENCODING: u32 = 29;
pub const WINHTTP_QUERY_CONTENT_ID: u32 = 3;
pub const WINHTTP_QUERY_CONTENT_LANGUAGE: u32 = 6;
pub const WINHTTP_QUERY_CONTENT_LENGTH: u32 = 5;
pub const WINHTTP_QUERY_CONTENT_LOCATION: u32 = 51;
pub const WINHTTP_QUERY_CONTENT_MD5: u32 = 52;
pub const WINHTTP_QUERY_CONTENT_RANGE: u32 = 53;
pub const WINHTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2;
pub const WINHTTP_QUERY_CONTENT_TYPE: u32 = 1;
pub const WINHTTP_QUERY_COOKIE: u32 = 44;
pub const WINHTTP_QUERY_COST: u32 = 15;
pub const WINHTTP_QUERY_CUSTOM: u32 = 65535;
pub const WINHTTP_QUERY_DATE: u32 = 9;
pub const WINHTTP_QUERY_DERIVED_FROM: u32 = 14;
pub const WINHTTP_QUERY_ETAG: u32 = 54;
pub const WINHTTP_QUERY_EXPECT: u32 = 68;
pub const WINHTTP_QUERY_EXPIRES: u32 = 10;
pub const WINHTTP_QUERY_EX_ALL_HEADERS: u32 = 21;
pub const WINHTTP_QUERY_FLAG_NUMBER: u32 = 536870912;
pub const WINHTTP_QUERY_FLAG_NUMBER64: u32 = 134217728;
pub const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648;
pub const WINHTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824;
pub const WINHTTP_QUERY_FLAG_TRAILERS: u32 = 33554432;
pub const WINHTTP_QUERY_FLAG_WIRE_ENCODING: u32 = 16777216;
pub const WINHTTP_QUERY_FORWARDED: u32 = 30;
pub const WINHTTP_QUERY_FROM: u32 = 31;
pub const WINHTTP_QUERY_HOST: u32 = 55;
pub const WINHTTP_QUERY_IF_MATCH: u32 = 56;
pub const WINHTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32;
pub const WINHTTP_QUERY_IF_NONE_MATCH: u32 = 57;
pub const WINHTTP_QUERY_IF_RANGE: u32 = 58;
pub const WINHTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59;
pub const WINHTTP_QUERY_LAST_MODIFIED: u32 = 11;
pub const WINHTTP_QUERY_LINK: u32 = 16;
pub const WINHTTP_QUERY_LOCATION: u32 = 33;
pub const WINHTTP_QUERY_MAX: u32 = 78;
pub const WINHTTP_QUERY_MAX_FORWARDS: u32 = 60;
pub const WINHTTP_QUERY_MESSAGE_ID: u32 = 12;
pub const WINHTTP_QUERY_MIME_VERSION: u32 = 0;
pub const WINHTTP_QUERY_ORIG_URI: u32 = 34;
pub const WINHTTP_QUERY_PASSPORT_CONFIG: u32 = 78;
pub const WINHTTP_QUERY_PASSPORT_URLS: u32 = 77;
pub const WINHTTP_QUERY_PRAGMA: u32 = 17;
pub const WINHTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41;
pub const WINHTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61;
pub const WINHTTP_QUERY_PROXY_CONNECTION: u32 = 69;
pub const WINHTTP_QUERY_PROXY_SUPPORT: u32 = 75;
pub const WINHTTP_QUERY_PUBLIC: u32 = 8;
pub const WINHTTP_QUERY_RANGE: u32 = 62;
pub const WINHTTP_QUERY_RAW_HEADERS: u32 = 21;
pub const WINHTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
pub const WINHTTP_QUERY_REFERER: u32 = 35;
pub const WINHTTP_QUERY_REFRESH: u32 = 46;
pub const WINHTTP_QUERY_REQUEST_METHOD: u32 = 45;
pub const WINHTTP_QUERY_RETRY_AFTER: u32 = 36;
pub const WINHTTP_QUERY_SERVER: u32 = 37;
pub const WINHTTP_QUERY_SET_COOKIE: u32 = 43;
pub const WINHTTP_QUERY_STATUS_CODE: u32 = 19;
pub const WINHTTP_QUERY_STATUS_TEXT: u32 = 20;
pub const WINHTTP_QUERY_TITLE: u32 = 38;
pub const WINHTTP_QUERY_TRANSFER_ENCODING: u32 = 63;
pub const WINHTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70;
pub const WINHTTP_QUERY_UPGRADE: u32 = 64;
pub const WINHTTP_QUERY_URI: u32 = 13;
pub const WINHTTP_QUERY_USER_AGENT: u32 = 39;
pub const WINHTTP_QUERY_VARY: u32 = 65;
pub const WINHTTP_QUERY_VERSION: u32 = 18;
pub const WINHTTP_QUERY_VIA: u32 = 66;
pub const WINHTTP_QUERY_WARNING: u32 = 67;
pub const WINHTTP_QUERY_WWW_AUTHENTICATE: u32 = 40;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: u64,
    pub ulIndex: u32,
    pub cStats: u32,
    pub rgullStats: [u64; 32],
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_REQUEST_STATS {
    pub ullFlags: u64,
    pub ulIndex: u32,
    pub cStats: u32,
    pub rgullStats: [u64; 32],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_REQUEST_STAT_ENTRY(pub i32);
pub const WINHTTP_REQUEST_STAT_FLAG_FIRST_REQUEST: u32 = 32;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_FALSE_START: u32 = 16;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_SESSION_RESUMPTION: u32 = 8;
pub const WINHTTP_REQUEST_STAT_FLAG_TCP_FAST_OPEN: u32 = 1;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_FALSE_START: u32 = 4;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_SESSION_RESUMPTION: u32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgullTimes: [u64; 64],
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINHTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgullTimes: [u64; 64],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_REQUEST_TIME_ENTRY(pub i32);
pub const WINHTTP_RESET_ALL: u32 = 65535;
pub const WINHTTP_RESET_DISCARD_RESOLVERS: u32 = 262144;
pub const WINHTTP_RESET_NOTIFY_NETWORK_CHANGED: u32 = 65536;
pub const WINHTTP_RESET_OUT_OF_PROC: u32 = 131072;
pub const WINHTTP_RESET_SCRIPT_CACHE: u32 = 8;
pub const WINHTTP_RESET_STATE: u32 = 1;
pub const WINHTTP_RESET_SWPAD_ALL: u32 = 4;
pub const WINHTTP_RESET_SWPAD_CURRENT_NETWORK: u32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: u32,
    pub ulMaxCacheEntryAge: u32,
    pub ulMinCacheEntryTtl: u32,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: u64,
    pub ullFlags: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: u32,
    pub ulMaxCacheEntryAge: u32,
    pub ulMinCacheEntryTtl: u32,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: u64,
    pub ullFlags: u64,
}
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: u32 = 2;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: u32 = 8;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: u32 = 1;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_SECURE_DNS_SETTING(pub i32);
pub const WINHTTP_SERVER_CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 4;
pub const WINHTTP_SERVER_CERT_CHAIN_DISABLE_AIA: u32 = 8192;
pub const WINHTTP_SERVER_CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648;
pub type WINHTTP_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: *mut core::ffi::c_void, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *mut core::ffi::c_void, dwstatusinformationlength: u32)>;
pub const WINHTTP_TIME_FORMAT_BUFSIZE: u32 = 62;
pub const WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1006);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: WINHTTP_ASYNC_RESULT,
    pub Operation: WINHTTP_WEB_SOCKET_OPERATION,
}
pub const WINHTTP_WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(1);
pub const WINHTTP_WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_WEB_SOCKET_BUFFER_TYPE(pub i32);
pub const WINHTTP_WEB_SOCKET_CLOSE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(4);
pub const WINHTTP_WEB_SOCKET_CLOSE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(2);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_WEB_SOCKET_CLOSE_STATUS(pub i32);
pub const WINHTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1005);
pub const WINHTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1001);
pub const WINHTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1003);
pub const WINHTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1007);
pub const WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123;
pub const WINHTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1009);
pub const WINHTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: u32 = 15000;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINHTTP_WEB_SOCKET_OPERATION(pub i32);
pub const WINHTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1008);
pub const WINHTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1002);
pub const WINHTTP_WEB_SOCKET_RECEIVE_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(1);
pub const WINHTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1015);
pub const WINHTTP_WEB_SOCKET_SEND_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(0);
pub const WINHTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1011);
pub const WINHTTP_WEB_SOCKET_SHUTDOWN_OPERATION: WINHTTP_WEB_SOCKET_OPERATION = WINHTTP_WEB_SOCKET_OPERATION(3);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINHTTP_WEB_SOCKET_STATUS {
    pub dwBytesTransferred: u32,
    pub eBufferType: WINHTTP_WEB_SOCKET_BUFFER_TYPE,
}
pub const WINHTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1000);
pub const WINHTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = WINHTTP_WEB_SOCKET_CLOSE_STATUS(1010);
pub const WINHTTP_WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(3);
pub const WINHTTP_WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WINHTTP_WEB_SOCKET_BUFFER_TYPE = WINHTTP_WEB_SOCKET_BUFFER_TYPE(2);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_HTTP_CREATE_URL_FLAGS(pub u32);
pub const WinHttpConnectFailureCount: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(0);
pub const WinHttpConnectionAcquireEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(4);
pub const WinHttpConnectionAcquireStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(2);
pub const WinHttpConnectionAcquireWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(3);
pub const WinHttpConnectionEstablishmentEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(8);
pub const WinHttpConnectionEstablishmentStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(7);
pub const WinHttpFastForwardingStateClientSideFailed: WINHTTP_FAST_FORWARDING_STATE = WINHTTP_FAST_FORWARDING_STATE(2);
pub const WinHttpFastForwardingStateInProgress: WINHTTP_FAST_FORWARDING_STATE = WINHTTP_FAST_FORWARDING_STATE(0);
pub const WinHttpFastForwardingStateServerSideFailed: WINHTTP_FAST_FORWARDING_STATE = WINHTTP_FAST_FORWARDING_STATE(3);
pub const WinHttpFastForwardingStateSucceeded: WINHTTP_FAST_FORWARDING_STATE = WINHTTP_FAST_FORWARDING_STATE(1);
pub const WinHttpNameResolutionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(6);
pub const WinHttpNameResolutionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(5);
pub const WinHttpProxyDetectionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(1);
pub const WinHttpProxyDetectionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(0);
pub const WinHttpProxyFailureCount: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(1);
pub const WinHttpProxySettingsTypeUnknown: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(0);
pub const WinHttpProxySettingsTypeWsa: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(2);
pub const WinHttpProxySettingsTypeWsl: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(1);
pub const WinHttpProxySettingsTypeXBox: WINHTTP_PROXY_SETTINGS_TYPE = WINHTTP_PROXY_SETTINGS_TYPE(3);
pub const WinHttpProxyTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(31);
pub const WinHttpProxyTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(12);
pub const WinHttpProxyTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(30);
pub const WinHttpProxyTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(33);
pub const WinHttpProxyTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(14);
pub const WinHttpProxyTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(32);
pub const WinHttpProxyTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(35);
pub const WinHttpProxyTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(34);
pub const WinHttpProxyTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(13);
pub const WinHttpProxyTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(15);
pub const WinHttpProxyTunnelEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(29);
pub const WinHttpProxyTunnelStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(28);
pub const WinHttpReceiveResponseBodyDecompressionDelta: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(26);
pub const WinHttpReceiveResponseEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(27);
pub const WinHttpReceiveResponseHeadersDecompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(24);
pub const WinHttpReceiveResponseHeadersDecompressionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(23);
pub const WinHttpReceiveResponseHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(25);
pub const WinHttpReceiveResponseStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(22);
pub const WinHttpRequest: windows_core::GUID = windows_core::GUID::from_u128(0x2087c2f4_2cef_4953_a8ab_66779b670495);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WinHttpRequestAutoLogonPolicy(pub i32);
pub const WinHttpRequestHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(7);
pub const WinHttpRequestHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(6);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WinHttpRequestOption(pub i32);
pub const WinHttpRequestOption_EnableCertificateRevocationCheck: WinHttpRequestOption = WinHttpRequestOption(18);
pub const WinHttpRequestOption_EnableHttp1_1: WinHttpRequestOption = WinHttpRequestOption(17);
pub const WinHttpRequestOption_EnableHttpsToHttpRedirects: WinHttpRequestOption = WinHttpRequestOption(12);
pub const WinHttpRequestOption_EnablePassportAuthentication: WinHttpRequestOption = WinHttpRequestOption(13);
pub const WinHttpRequestOption_EnableRedirects: WinHttpRequestOption = WinHttpRequestOption(6);
pub const WinHttpRequestOption_EnableTracing: WinHttpRequestOption = WinHttpRequestOption(10);
pub const WinHttpRequestOption_EscapePercentInURL: WinHttpRequestOption = WinHttpRequestOption(3);
pub const WinHttpRequestOption_MaxAutomaticRedirects: WinHttpRequestOption = WinHttpRequestOption(14);
pub const WinHttpRequestOption_MaxResponseDrainSize: WinHttpRequestOption = WinHttpRequestOption(16);
pub const WinHttpRequestOption_MaxResponseHeaderSize: WinHttpRequestOption = WinHttpRequestOption(15);
pub const WinHttpRequestOption_RejectUserpwd: WinHttpRequestOption = WinHttpRequestOption(19);
pub const WinHttpRequestOption_RevertImpersonationOverSsl: WinHttpRequestOption = WinHttpRequestOption(11);
pub const WinHttpRequestOption_SecureProtocols: WinHttpRequestOption = WinHttpRequestOption(9);
pub const WinHttpRequestOption_SelectCertificate: WinHttpRequestOption = WinHttpRequestOption(5);
pub const WinHttpRequestOption_SslErrorIgnoreFlags: WinHttpRequestOption = WinHttpRequestOption(4);
pub const WinHttpRequestOption_URL: WinHttpRequestOption = WinHttpRequestOption(1);
pub const WinHttpRequestOption_URLCodePage: WinHttpRequestOption = WinHttpRequestOption(2);
pub const WinHttpRequestOption_UrlEscapeDisable: WinHttpRequestOption = WinHttpRequestOption(7);
pub const WinHttpRequestOption_UrlEscapeDisableQuery: WinHttpRequestOption = WinHttpRequestOption(8);
pub const WinHttpRequestOption_UserAgentString: WinHttpRequestOption = WinHttpRequestOption(0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WinHttpRequestSecureProtocols(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WinHttpRequestSslErrorFlags(pub i32);
pub const WinHttpRequestStatLast: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(16);
pub const WinHttpRequestStatMax: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(32);
pub const WinHttpRequestTimeLast: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(36);
pub const WinHttpRequestTimeMax: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(64);
pub const WinHttpResponseBodyCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(11);
pub const WinHttpResponseBodySize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(10);
pub const WinHttpResponseHeadersCompressedSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(9);
pub const WinHttpResponseHeadersSize: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(8);
pub const WinHttpSecureDnsSettingDefault: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(0);
pub const WinHttpSecureDnsSettingForcePlaintext: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(1);
pub const WinHttpSecureDnsSettingMax: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(4);
pub const WinHttpSecureDnsSettingRequireEncryption: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(2);
pub const WinHttpSecureDnsSettingTryEncryptionWithFallback: WINHTTP_SECURE_DNS_SETTING = WINHTTP_SECURE_DNS_SETTING(3);
pub const WinHttpSendRequestEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(21);
pub const WinHttpSendRequestHeadersCompressionEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(19);
pub const WinHttpSendRequestHeadersCompressionStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(18);
pub const WinHttpSendRequestHeadersEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(20);
pub const WinHttpSendRequestStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(17);
pub const WinHttpStreamWaitEnd: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(16);
pub const WinHttpStreamWaitStart: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(15);
pub const WinHttpTlsHandshakeClientLeg1End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(10);
pub const WinHttpTlsHandshakeClientLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(2);
pub const WinHttpTlsHandshakeClientLeg1Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(9);
pub const WinHttpTlsHandshakeClientLeg2End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(12);
pub const WinHttpTlsHandshakeClientLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(4);
pub const WinHttpTlsHandshakeClientLeg2Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(11);
pub const WinHttpTlsHandshakeClientLeg3End: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(14);
pub const WinHttpTlsHandshakeClientLeg3Start: WINHTTP_REQUEST_TIME_ENTRY = WINHTTP_REQUEST_TIME_ENTRY(13);
pub const WinHttpTlsHandshakeServerLeg1Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(3);
pub const WinHttpTlsHandshakeServerLeg2Size: WINHTTP_REQUEST_STAT_ENTRY = WINHTTP_REQUEST_STAT_ENTRY(5);
