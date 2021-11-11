#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpAddRequestHeaders(hrequest: *mut ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, dwmodifiers: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpAddRequestHeadersEx(hrequest: *mut ::core::ffi::c_void, dwmodifiers: u32, ullflags: u64, ullextra: u64, cheaders: u32, pheaders: *const WINHTTP_EXTENDED_HEADER) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCheckPlatform() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCloseHandle(hinternet: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpConnect(hsession: *mut ::core::ffi::c_void, pswzservername: super::super::Foundation::PWSTR, nserverport: INTERNET_PORT, dwreserved: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCrackUrl(pwszurl: super::super::Foundation::PWSTR, dwurllength: u32, dwflags: u32, lpurlcomponents: *mut URL_COMPONENTS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpCreateProxyResolver(hsession: *const ::core::ffi::c_void, phresolver: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCreateUrl(lpurlcomponents: *const URL_COMPONENTS, dwflags: WIN_HTTP_CREATE_URL_FLAGS, pwszurl: super::super::Foundation::PWSTR, pdwurllength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags: u32, ppwstrautoconfigurl: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxyResult(pproxyresult: *mut WINHTTP_PROXY_RESULT);
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxyResultEx(pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX);
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxySettings(pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS);
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeQueryConnectionGroupResult(presult: *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT);
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrl(hsession: *mut ::core::ffi::c_void, lpcwszurl: super::super::Foundation::PWSTR, pautoproxyoptions: *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo: *mut WINHTTP_PROXY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrlEx(hresolver: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, pcontext: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrlEx2(hresolver: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pautoproxyoptions: *const WINHTTP_AUTOPROXY_OPTIONS, cbinterfaceselectioncontext: u32, pinterfaceselectioncontext: *const u8, pcontext: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyResult(hresolver: *const ::core::ffi::c_void, pproxyresult: *mut WINHTTP_PROXY_RESULT) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyResultEx(hresolver: *const ::core::ffi::c_void, pproxyresultex: *mut WINHTTP_PROXY_RESULT_EX) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpGetProxySettingsVersion(hsession: *const ::core::ffi::c_void, pdwproxysettingsversion: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpOpen(pszagentw: super::super::Foundation::PWSTR, dwaccesstype: WINHTTP_ACCESS_TYPE, pszproxyw: super::super::Foundation::PWSTR, pszproxybypassw: super::super::Foundation::PWSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpOpenRequest(hconnect: *mut ::core::ffi::c_void, pwszverb: super::super::Foundation::PWSTR, pwszobjectname: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, pwszreferrer: super::super::Foundation::PWSTR, ppwszaccepttypes: *mut super::super::Foundation::PWSTR, dwflags: WINHTTP_OPEN_REQUEST_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryAuthSchemes(hrequest: *mut ::core::ffi::c_void, lpdwsupportedschemes: *mut u32, lpdwfirstscheme: *mut u32, pdwauthtarget: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryConnectionGroup(hinternet: *const ::core::ffi::c_void, pguidconnection: *const ::windows::runtime::GUID, ullflags: u64, ppresult: *mut *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryDataAvailable(hrequest: *mut ::core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryHeaders(hrequest: *mut ::core::ffi::c_void, dwinfolevel: u32, pwszname: super::super::Foundation::PWSTR, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryHeadersEx(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, ullflags: u64, uicodepage: u32, pdwindex: *mut u32, pheadername: *const WINHTTP_HEADER_NAME, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, ppheaders: *mut *mut WINHTTP_EXTENDED_HEADER, pdwheaderscount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryOption(hinternet: *mut ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReadData(hrequest: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpReadDataEx(hrequest: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32, ullflags: u64, cbproperty: u32, pvproperty: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReadProxySettings(hsession: *const ::core::ffi::c_void, pcwszconnectionname: super::super::Foundation::PWSTR, ffallbacktodefaultsettings: super::super::Foundation::BOOL, fsetautodiscoverfordefaultsettings: super::super::Foundation::BOOL, pdwsettingsversion: *mut u32, pfdefaultsettingsarereturned: *mut super::super::Foundation::BOOL, pwinhttpproxysettings: *mut WINHTTP_PROXY_SETTINGS) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReceiveResponse(hrequest: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpResetAutoProxy(hsession: *const ::core::ffi::c_void, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSendRequest(hrequest: *mut ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32, dwtotallength: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetCredentials(hrequest: *mut ::core::ffi::c_void, authtargets: u32, authscheme: u32, pwszusername: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR, pauthparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetDefaultProxyConfiguration(pproxyinfo: *mut WINHTTP_PROXY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetOption(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpSetStatusCallback(hinternet: *mut ::core::ffi::c_void, lpfninternetcallback: ::windows::runtime::RawPtr, dwnotificationflags: u32, dwreserved: usize) -> ::core::option::Option<WINHTTP_STATUS_CALLBACK>;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetTimeouts(hinternet: *mut ::core::ffi::c_void, nresolvetimeout: i32, nconnecttimeout: i32, nsendtimeout: i32, nreceivetimeout: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, pwsztime: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpTimeToSystemTime(pwsztime: super::super::Foundation::PWSTR, pst: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, pcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: *mut ::core::ffi::c_void, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pebuffertype: *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, ebuffertype: WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpWriteData(hrequest: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpWriteProxySettings(hsession: *const ::core::ffi::c_void, fforceupdate: super::super::Foundation::BOOL, pwinhttpproxysettings: *const WINHTTP_PROXY_SETTINGS) -> u32;
}
