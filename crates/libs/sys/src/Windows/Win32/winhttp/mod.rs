windows_link::link!("winhttp.dll" "system" fn WinHttpAddRequestHeaders(hrequest : HINTERNET, lpszheaders : windows_sys::core::PCWSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpAddRequestHeadersEx(hrequest : HINTERNET, dwmodifiers : u32, ullflags : u64, ullextra : u64, cheaders : u32, pheaders : *const WINHTTP_EXTENDED_HEADER) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpCheckPlatform() -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpCloseHandle(hinternet : HINTERNET) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpConnect(hsession : HINTERNET, pswzservername : windows_sys::core::PCWSTR, nserverport : INTERNET_PORT, dwreserved : u32) -> HINTERNET);
windows_link::link!("winhttp.dll" "system" fn WinHttpCrackUrl(pwszurl : windows_sys::core::PCWSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : *mut URL_COMPONENTS) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpCreateProxyResolver(hsession : HINTERNET, phresolver : *mut HINTERNET) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpCreateUrl(lpurlcomponents : *const URL_COMPONENTS, dwflags : u32, pwszurl : windows_sys::core::PWSTR, pdwurllength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpDetectAutoProxyConfigUrl(dwautodetectflags : u32, ppwstrautoconfigurl : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpFreeProxyResult(pproxyresult : *mut WINHTTP_PROXY_RESULT));
#[cfg(feature = "winnt")]
windows_link::link!("winhttp.dll" "system" fn WinHttpFreeProxyResultEx(pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX));
#[cfg(feature = "minwindef")]
windows_link::link!("winhttp.dll" "system" fn WinHttpFreeProxySettings(pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS));
windows_link::link!("winhttp.dll" "system" fn WinHttpFreeProxySettingsEx(proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsex : *const core::ffi::c_void) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpFreeQueryConnectionGroupResult(presult : *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT));
windows_link::link!("winhttp.dll" "system" fn WinHttpGetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetIEProxyConfigForCurrentUser(pproxyconfig : *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrl(hsession : HINTERNET, lpcwszurl : windows_sys::core::PCWSTR, pautoproxyoptions : *mut WINHTTP_AUTOPROXY_OPTIONS, pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx(hresolver : HINTERNET, pcwszurl : windows_sys::core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, pcontext : usize) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxyForUrlEx2(hresolver : HINTERNET, pcwszurl : windows_sys::core::PCWSTR, pautoproxyoptions : *const WINHTTP_AUTOPROXY_OPTIONS, cbinterfaceselectioncontext : u32, pinterfaceselectioncontext : *const u8, pcontext : usize) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxyResult(hresolver : HINTERNET, pproxyresult : *mut WINHTTP_PROXY_RESULT) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxyResultEx(hresolver : HINTERNET, pproxyresultex : *mut WINHTTP_PROXY_RESULT_EX) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsEx(hresolver : HINTERNET, proxysettingstype : WINHTTP_PROXY_SETTINGS_TYPE, pproxysettingsparam : *const WINHTTP_PROXY_SETTINGS_PARAM, pcontext : usize) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsResultEx(hresolver : HINTERNET, pproxysettingsex : *mut core::ffi::c_void) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpGetProxySettingsVersion(hsession : HINTERNET, pdwproxysettingsversion : *mut u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpOpen(pszagentw : windows_sys::core::PCWSTR, dwaccesstype : u32, pszproxyw : windows_sys::core::PCWSTR, pszproxybypassw : windows_sys::core::PCWSTR, dwflags : u32) -> HINTERNET);
windows_link::link!("winhttp.dll" "system" fn WinHttpOpenRequest(hconnect : HINTERNET, pwszverb : windows_sys::core::PCWSTR, pwszobjectname : windows_sys::core::PCWSTR, pwszversion : windows_sys::core::PCWSTR, pwszreferrer : windows_sys::core::PCWSTR, ppwszaccepttypes : *mut windows_sys::core::PCWSTR, dwflags : u32) -> HINTERNET);
windows_link::link!("winhttp.dll" "system" fn WinHttpProtocolCompleteUpgrade(hrequest : HINTERNET, dwcontext : usize) -> HINTERNET);
windows_link::link!("winhttp.dll" "system" fn WinHttpProtocolReceive(protocolhandle : HINTERNET, flags : u64, pvbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpProtocolSend(protocolhandle : HINTERNET, flags : u64, pvbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryAuthSchemes(hrequest : HINTERNET, lpdwsupportedschemes : *mut u32, lpdwfirstscheme : *mut u32, pdwauthtarget : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryConnectionGroup(hinternet : HINTERNET, pguidconnection : *const windows_sys::core::GUID, ullflags : u64, ppresult : *mut PWINHTTP_QUERY_CONNECTION_GROUP_RESULT) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryDataAvailable(hrequest : HINTERNET, lpdwnumberofbytesavailable : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryHeaders(hrequest : HINTERNET, dwinfolevel : u32, pwszname : windows_sys::core::PCWSTR, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryHeadersEx(hrequest : HINTERNET, dwinfolevel : u32, ullflags : u64, uicodepage : u32, pdwindex : *mut u32, pheadername : *const WINHTTP_HEADER_NAME, pbuffer : *mut core::ffi::c_void, pdwbufferlength : *mut u32, ppheaders : *mut PWINHTTP_EXTENDED_HEADER, pdwheaderscount : *mut u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpQueryOption(hinternet : HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpReadData(hrequest : HINTERNET, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpReadDataEx(hrequest : HINTERNET, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32, ullflags : u64, cbproperty : u32, pvproperty : *const core::ffi::c_void) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("winhttp.dll" "system" fn WinHttpReadProxySettings(hsession : HINTERNET, pcwszconnectionname : windows_sys::core::PCWSTR, ffallbacktodefaultsettings : windows_sys::core::BOOL, fsetautodiscoverfordefaultsettings : windows_sys::core::BOOL, pdwsettingsversion : *mut u32, pfdefaultsettingsarereturned : *mut windows_sys::core::BOOL, pwinhttpproxysettings : *mut WINHTTP_PROXY_SETTINGS) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpReceiveResponse(hrequest : HINTERNET, lpreserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpRegisterProxyChangeNotification(ullflags : u64, pfncallback : WINHTTP_PROXY_CHANGE_CALLBACK, pvcontext : *const core::ffi::c_void, hregistration : *mut WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpResetAutoProxy(hsession : HINTERNET, dwflags : u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpSendRequest(hrequest : HINTERNET, lpszheaders : windows_sys::core::PCWSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32, dwtotallength : u32, dwcontext : usize) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetCredentials(hrequest : HINTERNET, authtargets : u32, authscheme : u32, pwszusername : windows_sys::core::PCWSTR, pwszpassword : windows_sys::core::PCWSTR, pauthparams : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetDefaultProxyConfiguration(pproxyinfo : *mut WINHTTP_PROXY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetOption(hinternet : HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, dwbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetProxySettingsPerUser(fproxysettingsperuser : windows_sys::core::BOOL) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetStatusCallback(hinternet : HINTERNET, lpfninternetcallback : WINHTTP_STATUS_CALLBACK, dwnotificationflags : u32, dwreserved : usize) -> WINHTTP_STATUS_CALLBACK);
windows_link::link!("winhttp.dll" "system" fn WinHttpSetTimeouts(hinternet : HINTERNET, nresolvetimeout : i32, nconnecttimeout : i32, nsendtimeout : i32, nreceivetimeout : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("winhttp.dll" "system" fn WinHttpTimeFromSystemTime(pst : *const super::SYSTEMTIME, pwsztime : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("winhttp.dll" "system" fn WinHttpTimeToSystemTime(pwsztime : windows_sys::core::PCWSTR, pst : *mut super::SYSTEMTIME) -> windows_sys::core::BOOL);
windows_link::link!("winhttp.dll" "system" fn WinHttpUnregisterProxyChangeNotification(hregistration : WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketClose(hwebsocket : HINTERNET, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketCompleteUpgrade(hrequest : HINTERNET, pcontext : usize) -> HINTERNET);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketQueryCloseStatus(hwebsocket : HINTERNET, pusstatus : *mut u16, pvreason : *mut core::ffi::c_void, dwreasonlength : u32, pdwreasonlengthconsumed : *mut u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketReceive(hwebsocket : HINTERNET, pvbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32, pebuffertype : *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketSend(hwebsocket : HINTERNET, ebuffertype : WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWebSocketShutdown(hwebsocket : HINTERNET, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> u32);
windows_link::link!("winhttp.dll" "system" fn WinHttpWriteData(hrequest : HINTERNET, lpbuffer : *const core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("winhttp.dll" "system" fn WinHttpWriteProxySettings(hsession : HINTERNET, fforceupdate : windows_sys::core::BOOL, pwinhttpproxysettings : *const WINHTTP_PROXY_SETTINGS) -> u32);
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
pub type HINTERNET = *mut core::ffi::c_void;
pub const HTTP_STATUS_ACCEPTED: i32 = 202;
pub const HTTP_STATUS_AMBIGUOUS: i32 = 300;
pub const HTTP_STATUS_BAD_GATEWAY: i32 = 502;
pub const HTTP_STATUS_BAD_METHOD: i32 = 405;
pub const HTTP_STATUS_BAD_REQUEST: i32 = 400;
pub const HTTP_STATUS_CONFLICT: i32 = 409;
pub const HTTP_STATUS_CONTINUE: i32 = 100;
pub const HTTP_STATUS_CREATED: i32 = 201;
pub const HTTP_STATUS_DENIED: i32 = 401;
pub const HTTP_STATUS_FIRST: i32 = 100;
pub const HTTP_STATUS_FORBIDDEN: i32 = 403;
pub const HTTP_STATUS_GATEWAY_TIMEOUT: i32 = 504;
pub const HTTP_STATUS_GONE: i32 = 410;
pub const HTTP_STATUS_LAST: i32 = 505;
pub const HTTP_STATUS_LENGTH_REQUIRED: i32 = 411;
pub const HTTP_STATUS_MOVED: i32 = 301;
pub const HTTP_STATUS_NONE_ACCEPTABLE: i32 = 406;
pub const HTTP_STATUS_NOT_FOUND: i32 = 404;
pub const HTTP_STATUS_NOT_MODIFIED: i32 = 304;
pub const HTTP_STATUS_NOT_SUPPORTED: i32 = 501;
pub const HTTP_STATUS_NO_CONTENT: i32 = 204;
pub const HTTP_STATUS_OK: i32 = 200;
pub const HTTP_STATUS_PARTIAL: i32 = 203;
pub const HTTP_STATUS_PARTIAL_CONTENT: i32 = 206;
pub const HTTP_STATUS_PAYMENT_REQ: i32 = 402;
pub const HTTP_STATUS_PERMANENT_REDIRECT: i32 = 308;
pub const HTTP_STATUS_PRECOND_FAILED: i32 = 412;
pub const HTTP_STATUS_PROXY_AUTH_REQ: i32 = 407;
pub const HTTP_STATUS_REDIRECT: i32 = 302;
pub const HTTP_STATUS_REDIRECT_KEEP_VERB: i32 = 307;
pub const HTTP_STATUS_REDIRECT_METHOD: i32 = 303;
pub const HTTP_STATUS_REQUEST_TIMEOUT: i32 = 408;
pub const HTTP_STATUS_REQUEST_TOO_LARGE: i32 = 413;
pub const HTTP_STATUS_RESET_CONTENT: i32 = 205;
pub const HTTP_STATUS_RETRY_WITH: i32 = 449;
pub const HTTP_STATUS_SERVER_ERROR: i32 = 500;
pub const HTTP_STATUS_SERVICE_UNAVAIL: i32 = 503;
pub const HTTP_STATUS_SWITCH_PROTOCOLS: i32 = 101;
pub const HTTP_STATUS_UNSUPPORTED_MEDIA: i32 = 415;
pub const HTTP_STATUS_URI_TOO_LONG: i32 = 414;
pub const HTTP_STATUS_USE_PROXY: i32 = 305;
pub const HTTP_STATUS_VERSION_NOT_SUP: i32 = 505;
pub const HTTP_STATUS_WEBDAV_MULTI_STATUS: i32 = 207;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
pub const ICU_BROWSER_MODE: i32 = 33554432;
pub const ICU_DECODE: i32 = 268435456;
pub const ICU_ENCODE_PERCENT: i32 = 4096;
pub const ICU_ENCODE_SPACES_ONLY: i32 = 67108864;
pub const ICU_ESCAPE: u32 = 2147483648;
pub const ICU_ESCAPE_AUTHORITY: i32 = 8192;
pub const ICU_INCLUDE_DEFAULT_PORT: i32 = 32768;
pub const ICU_NO_ENCODE: i32 = 536870912;
pub const ICU_NO_META: i32 = 134217728;
pub const ICU_REJECT_USERPWD: i32 = 16384;
pub const INTERNET_DEFAULT_HTTPS_PORT: i32 = 443;
pub const INTERNET_DEFAULT_HTTP_PORT: i32 = 80;
pub const INTERNET_DEFAULT_PORT: i32 = 0;
pub type INTERNET_PORT = u16;
pub type INTERNET_SCHEME = i32;
pub const INTERNET_SCHEME_FTP: i32 = 3;
pub const INTERNET_SCHEME_HTTP: i32 = 1;
pub const INTERNET_SCHEME_HTTPS: i32 = 2;
pub const INTERNET_SCHEME_SOCKS: i32 = 4;
pub type LPHINTERNET = *mut HINTERNET;
pub type LPHTTP_VERSION_INFO = *mut HTTP_VERSION_INFO;
pub type LPINTERNET_PORT = *mut INTERNET_PORT;
pub type LPINTERNET_SCHEME = *mut i32;
pub type LPURL_COMPONENTS = *mut URL_COMPONENTS;
pub type LPURL_COMPONENTSW = LPURL_COMPONENTS;
pub type LPWINHTTP_ASYNC_RESULT = *mut WINHTTP_ASYNC_RESULT;
pub type LPWINHTTP_PROXY_INFO = *mut WINHTTP_PROXY_INFO;
pub type LPWINHTTP_PROXY_INFOW = LPWINHTTP_PROXY_INFO;
pub type LPWINHTTP_STATUS_CALLBACK = *mut WINHTTP_STATUS_CALLBACK;
pub const NETWORKING_KEY_BUFSIZE: i32 = 128;
pub type PHTTP_VERSION_INFO = *mut HTTP_VERSION_INFO;
pub type PWINHTTP_ASYNC_RESULT = *mut WINHTTP_ASYNC_RESULT;
pub type PWINHTTP_AUTOPROXY_OPTIONS = *mut WINHTTP_AUTOPROXY_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PWINHTTP_CERTIFICATE_INFO = *mut WINHTTP_CERTIFICATE_INFO;
pub type PWINHTTP_CONNECTION_GROUP = *mut WINHTTP_CONNECTION_GROUP;
#[cfg(feature = "ws2")]
pub type PWINHTTP_CONNECTION_INFO = *mut WINHTTP_CONNECTION_INFO;
pub type PWINHTTP_CREDS = *mut WINHTTP_CREDS;
pub type PWINHTTP_CREDS_EX = *mut WINHTTP_CREDS_EX;
pub type PWINHTTP_CURRENT_USER_IE_PROXY_CONFIG = *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG;
pub type PWINHTTP_EXTENDED_HEADER = *mut WINHTTP_EXTENDED_HEADER;
pub type PWINHTTP_FAILED_CONNECTION_RETRIES = *mut WINHTTP_FAILED_CONNECTION_RETRIES;
pub type PWINHTTP_FAST_FORWARDING_STATE = *mut WINHTTP_FAST_FORWARDING_STATE;
pub type PWINHTTP_FAST_FORWARDING_STATUS = *mut WINHTTP_FAST_FORWARDING_STATUS;
pub type PWINHTTP_HEADER_NAME = *mut WINHTTP_HEADER_NAME;
pub type PWINHTTP_HOST_CONNECTION_GROUP = *mut WINHTTP_HOST_CONNECTION_GROUP;
pub type PWINHTTP_HTTP2_RECEIVE_WINDOW = *mut WINHTTP_HTTP2_RECEIVE_WINDOW;
pub type PWINHTTP_MATCH_CONNECTION_GUID = *mut WINHTTP_MATCH_CONNECTION_GUID;
pub type PWINHTTP_PROXY_INFO = *mut WINHTTP_PROXY_INFO;
pub type PWINHTTP_PROXY_NETWORKING_KEY = *mut WINHTTP_PROXY_NETWORKING_KEY;
#[cfg(feature = "minwindef")]
pub type PWINHTTP_PROXY_SETTINGS = *mut WINHTTP_PROXY_SETTINGS;
pub type PWINHTTP_PROXY_SETTINGS_EX = *mut WINHTTP_PROXY_SETTINGS_EX;
pub type PWINHTTP_PROXY_SETTINGS_PARAM = *mut WINHTTP_PROXY_SETTINGS_PARAM;
pub type PWINHTTP_PROXY_SETTINGS_TYPE = *mut WINHTTP_PROXY_SETTINGS_TYPE;
pub type PWINHTTP_QUERY_CONNECTION_GROUP_RESULT = *mut WINHTTP_QUERY_CONNECTION_GROUP_RESULT;
pub type PWINHTTP_REQUEST_STATS = *mut WINHTTP_REQUEST_STATS;
pub type PWINHTTP_REQUEST_TIMES = *mut WINHTTP_REQUEST_TIMES;
pub type PWINHTTP_RESOLVER_CACHE_CONFIG = *mut WINHTTP_RESOLVER_CACHE_CONFIG;
pub const SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS: i32 = 13056;
pub const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: i32 = 4096;
pub const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: i32 = 8192;
pub const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: i32 = 512;
pub const SECURITY_FLAG_IGNORE_UNKNOWN_CA: i32 = 256;
pub const SECURITY_FLAG_SECURE: i32 = 1;
pub const SECURITY_FLAG_STRENGTH_MEDIUM: i32 = 1073741824;
pub const SECURITY_FLAG_STRENGTH_STRONG: i32 = 536870912;
pub const SECURITY_FLAG_STRENGTH_WEAK: i32 = 268435456;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct URL_COMPONENTS {
    pub dwStructSize: u32,
    pub lpszScheme: windows_sys::core::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: windows_sys::core::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: INTERNET_PORT,
    pub lpszUserName: windows_sys::core::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_sys::core::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_sys::core::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_sys::core::PWSTR,
    pub dwExtraInfoLength: u32,
}
impl Default for URL_COMPONENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dwFlags: u32,
    pub dwAutoDetectFlags: u32,
    pub lpszAutoConfigUrl: windows_sys::core::PCWSTR,
    pub lpvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub fAutoLogonIfChallenged: windows_sys::core::BOOL,
}
impl Default for WINHTTP_AUTOPROXY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct WINHTTP_CERTIFICATE_INFO {
    pub ftExpiry: super::FILETIME,
    pub ftStart: super::FILETIME,
    pub lpszSubjectInfo: windows_sys::core::PWSTR,
    pub lpszIssuerInfo: windows_sys::core::PWSTR,
    pub lpszProtocolName: windows_sys::core::PWSTR,
    pub lpszSignatureAlgName: windows_sys::core::PWSTR,
    pub lpszEncryptionAlgName: windows_sys::core::PWSTR,
    pub dwKeySize: u32,
}
#[cfg(feature = "minwindef")]
impl Default for WINHTTP_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_GROUP {
    pub cConnections: u32,
    pub guidGroup: windows_sys::core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::SOCKADDR_STORAGE,
    pub RemoteAddress: super::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_CONNECTION_INFO {
    pub cbSize: u32,
    pub LocalAddress: super::SOCKADDR_STORAGE,
    pub RemoteAddress: super::SOCKADDR_STORAGE,
}
pub const WINHTTP_CONNECTION_RETRY_CONDITION_408: i32 = 1;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_MASK: i32 = 7;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE: i32 = 2;
pub const WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION: i32 = 4;
pub const WINHTTP_CONNS_PER_SERVER_UNLIMITED: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_CREDS {
    pub lpszUserName: windows_sys::core::PSTR,
    pub lpszPassword: windows_sys::core::PSTR,
    pub lpszRealm: windows_sys::core::PSTR,
    pub dwAuthScheme: u32,
    pub lpszHostName: windows_sys::core::PSTR,
    pub dwPort: u32,
}
impl Default for WINHTTP_CREDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_CREDS_EX {
    pub lpszUserName: windows_sys::core::PSTR,
    pub lpszPassword: windows_sys::core::PSTR,
    pub lpszRealm: windows_sys::core::PSTR,
    pub dwAuthScheme: u32,
    pub lpszHostName: windows_sys::core::PSTR,
    pub dwPort: u32,
    pub lpszUrl: windows_sys::core::PSTR,
}
impl Default for WINHTTP_CREDS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub fAutoDetect: windows_sys::core::BOOL,
    pub lpszAutoConfigUrl: windows_sys::core::PWSTR,
    pub lpszProxy: windows_sys::core::PWSTR,
    pub lpszProxyBypass: windows_sys::core::PWSTR,
}
impl Default for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_EXTENDED_HEADER {
    pub Anonymous: WINHTTP_EXTENDED_HEADER_0,
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
    pub pwszName: windows_sys::core::PCWSTR,
    pub pszName: windows_sys::core::PCSTR,
}
impl Default for WINHTTP_EXTENDED_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINHTTP_EXTENDED_HEADER_1 {
    pub pwszValue: windows_sys::core::PCWSTR,
    pub pszValue: windows_sys::core::PCSTR,
}
impl Default for WINHTTP_EXTENDED_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_EXTENDED_HEADER_FLAG_UNICODE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAILED_CONNECTION_RETRIES {
    pub dwMaxRetries: u32,
    pub dwAllowedRetryConditions: u32,
}
pub type WINHTTP_FAST_FORWARDING_STATE = i32;
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
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_FAST_FORWARDING_STATUS {
    pub TransferState: WINHTTP_FAST_FORWARDING_STATE,
    pub NtStatus: i32,
    pub dwError: u32,
    pub ullBytesTransferred: u64,
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
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINHTTP_HEADER_NAME {
    pub pwszName: windows_sys::core::PCWSTR,
    pub pszName: windows_sys::core::PCSTR,
}
impl Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_HEADER_NAME_BY_INDEX: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_HOST_CONNECTION_GROUP {
    pub pwszHost: windows_sys::core::PCWSTR,
    pub cConnectionGroups: u32,
    pub pConnectionGroups: PWINHTTP_CONNECTION_GROUP,
}
impl Default for WINHTTP_HOST_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_HTTP2_RECEIVE_WINDOW {
    pub ulStreamWindow: u32,
    pub ulStreamWindowUpdateDelta: u32,
}
pub const WINHTTP_IGNORE_REQUEST_TOTAL_LENGTH: i32 = 0;
pub const WINHTTP_LAST_OPTION: i32 = 212;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_sys::core::GUID,
    pub ullFlags: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_MATCH_CONNECTION_GUID {
    pub ConnectionGuid: windows_sys::core::GUID,
    pub ullFlags: u64,
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
#[repr(C)]
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
pub type WINHTTP_PROXY_CHANGE_CALLBACK = Option<unsafe extern "system" fn(ullflags: u64, pvcontext: *const core::ffi::c_void)>;
pub type WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE = *mut core::ffi::c_void;
pub const WINHTTP_PROXY_DISABLE_AUTH_LOCAL_SERVICE: i32 = 256;
pub const WINHTTP_PROXY_DISABLE_SCHEME_BASIC: i32 = 1;
pub const WINHTTP_PROXY_DISABLE_SCHEME_DIGEST: i32 = 2;
pub const WINHTTP_PROXY_DISABLE_SCHEME_KERBEROS: i32 = 8;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NEGOTIATE: i32 = 16;
pub const WINHTTP_PROXY_DISABLE_SCHEME_NTLM: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_INFO {
    pub dwAccessType: u32,
    pub lpszProxy: windows_sys::core::PWSTR,
    pub lpszProxyBypass: windows_sys::core::PWSTR,
}
impl Default for WINHTTP_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINHTTP_PROXY_INFOW = WINHTTP_PROXY_INFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_NETWORKING_KEY {
    pub pbBuffer: [u8; 128],
}
impl Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINHTTP_PROXY_NOTIFY_CHANGE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_RESULT_ENTRY {
    pub fProxy: windows_sys::core::BOOL,
    pub fBypass: windows_sys::core::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: windows_sys::core::PWSTR,
    pub ProxyPort: INTERNET_PORT,
}
impl Default for WINHTTP_PROXY_RESULT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_RESULT_EX {
    pub cEntries: u32,
    pub pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
    pub hProxyDetectionHandle: super::HANDLE,
    pub dwProxyInterfaceAffinity: u32,
}
#[cfg(feature = "winnt")]
impl Default for WINHTTP_PROXY_RESULT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_SETTINGS {
    pub dwStructSize: u32,
    pub dwFlags: u32,
    pub dwCurrentSettingsVersion: u32,
    pub pwszConnectionName: windows_sys::core::PWSTR,
    pub pwszProxy: windows_sys::core::PWSTR,
    pub pwszProxyBypass: windows_sys::core::PWSTR,
    pub pwszAutoconfigUrl: windows_sys::core::PWSTR,
    pub pwszAutoconfigSecondaryUrl: windows_sys::core::PWSTR,
    pub dwAutoDiscoveryFlags: u32,
    pub pwszLastKnownGoodAutoConfigUrl: windows_sys::core::PWSTR,
    pub dwAutoconfigReloadDelayMins: u32,
    pub ftLastKnownDetectTime: super::FILETIME,
    pub dwDetectedInterfaceIpCount: u32,
    pub pdwDetectedInterfaceIp: super::PDWORD,
    pub cNetworkKeys: u32,
    pub pNetworkKeys: PWINHTTP_PROXY_NETWORKING_KEY,
}
#[cfg(feature = "minwindef")]
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
    pub pcwszAutoconfigUrl: windows_sys::core::PCWSTR,
    pub pcwszProxy: windows_sys::core::PCWSTR,
    pub pcwszSecureProxy: windows_sys::core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *mut windows_sys::core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_SETTINGS_EX {
    pub ullGenerationId: u64,
    pub ullFlags: u64,
    pub pcwszAutoconfigUrl: windows_sys::core::PCWSTR,
    pub pcwszProxy: windows_sys::core::PCWSTR,
    pub pcwszSecureProxy: windows_sys::core::PCWSTR,
    pub cProxyBypasses: u32,
    pub rgpcwszProxyBypasses: *mut windows_sys::core::PCWSTR,
    pub dwInterfaceIndex: u32,
    pub pcwszConnectionName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_PROXY_SETTINGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: windows_sys::core::PCWSTR,
    pub pcwszProbeHost: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for WINHTTP_PROXY_SETTINGS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WINHTTP_PROXY_SETTINGS_PARAM {
    pub ullFlags: u64,
    pub pcwszConnectionName: windows_sys::core::PCWSTR,
    pub pcwszProbeHost: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WINHTTP_PROXY_SETTINGS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    pub cHosts: u32,
    pub pHostConnectionGroups: PWINHTTP_HOST_CONNECTION_GROUP,
}
impl Default for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
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
pub type WINHTTP_REQUEST_STAT_ENTRY = i32;
pub const WINHTTP_REQUEST_STAT_FLAG_FIRST_REQUEST: i32 = 32;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_FALSE_START: i32 = 16;
pub const WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_SESSION_RESUMPTION: i32 = 8;
pub const WINHTTP_REQUEST_STAT_FLAG_TCP_FAST_OPEN: i32 = 1;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_FALSE_START: i32 = 4;
pub const WINHTTP_REQUEST_STAT_FLAG_TLS_SESSION_RESUMPTION: i32 = 2;
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_RESOLVER_CACHE_CONFIG {
    pub ulMaxResolverCacheEntries: u32,
    pub ulMaxCacheEntryAge: u32,
    pub ulMinCacheEntryTtl: u32,
    pub SecureDnsSetting: WINHTTP_SECURE_DNS_SETTING,
    pub ullConnResolutionWaitTime: u64,
    pub ullFlags: u64,
}
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE: i32 = 2;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL: i32 = 8;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT: i32 = 1;
pub const WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL: i32 = 4;
pub type WINHTTP_SECURE_DNS_SETTING = i32;
pub const WINHTTP_SERVER_CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: i32 = 4;
pub const WINHTTP_SERVER_CERT_CHAIN_DISABLE_AIA: i32 = 8192;
pub const WINHTTP_SERVER_CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648;
pub type WINHTTP_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: HINTERNET, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *mut core::ffi::c_void, dwstatusinformationlength: u32)>;
pub const WINHTTP_TIME_FORMAT_BUFSIZE: i32 = 62;
pub const WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: WINHTTP_WEB_SOCKET_CLOSE_STATUS = 1006;
#[repr(C)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINHTTP_WEB_SOCKET_STATUS {
    pub dwBytesTransferred: u32,
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
