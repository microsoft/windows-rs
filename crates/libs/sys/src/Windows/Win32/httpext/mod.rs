#[cfg(feature = "shared")]
windows_link::link!("rpcproxy.dll" "system" fn GetExtensionVersion(pver : *mut HSE_VERSION_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rpcproxy.dll" "system" fn HttpExtensionProc(pecb : *const EXTENSION_CONTROL_BLOCK) -> super::DWORD);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::CERT_CONTEXT,
    pub cbAllocated: super::DWORD,
    pub dwCertificateFlags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub ConnID: HCONN,
    pub dwHttpStatusCode: super::DWORD,
    pub lpszLogData: [super::CHAR; 80],
    pub lpszMethod: windows_sys::core::PSTR,
    pub lpszQueryString: windows_sys::core::PSTR,
    pub lpszPathInfo: windows_sys::core::PSTR,
    pub lpszPathTranslated: windows_sys::core::PSTR,
    pub cbTotalBytes: super::DWORD,
    pub cbAvailable: super::DWORD,
    pub lpbData: super::LPBYTE,
    pub lpszContentType: windows_sys::core::PSTR,
    pub GetServerVariable: Option<unsafe extern "system" fn(HCONN, windows_sys::core::PSTR, super::LPVOID, super::LPDWORD) -> windows_sys::core::BOOL>,
    pub WriteClient: Option<unsafe extern "system" fn(HCONN, super::LPVOID, super::LPDWORD, super::DWORD) -> windows_sys::core::BOOL>,
    pub ReadClient: Option<unsafe extern "system" fn(HCONN, super::LPVOID, super::LPDWORD) -> windows_sys::core::BOOL>,
    pub ServerSupportFunction: Option<unsafe extern "system" fn(HCONN, super::DWORD, super::LPVOID, super::LPDWORD, super::LPDWORD) -> windows_sys::core::BOOL>,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HCONN = *mut core::ffi::c_void;
pub const HSE_APPEND_LOG_PARAMETER: i32 = 1003;
pub const HSE_APP_FLAG_IN_PROCESS: i32 = 0;
pub const HSE_APP_FLAG_ISOLATED_OOP: i32 = 1;
pub const HSE_APP_FLAG_POOLED_OOP: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: *mut super::CHAR,
    pub uHttpSubError: super::USHORT,
    pub fAsync: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: windows_sys::core::PWSTR,
    pub pszMethod: windows_sys::core::PSTR,
    pub pszChildHeaders: windows_sys::core::PSTR,
    pub pUserInfo: LPHSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: LPHSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::HANDLE,
    pub pszCustomUserName: windows_sys::core::PWSTR,
    pub pszCustomAuthType: windows_sys::core::PSTR,
}
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_URL_ENTITY_INFO {
    pub cbAvailable: super::DWORD,
    pub lpbData: super::LPVOID,
}
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: i32 = 128;
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: i32 = 4;
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: windows_sys::core::PSTR,
    pub pszMethod: windows_sys::core::PSTR,
    pub pszChildHeaders: windows_sys::core::PSTR,
    pub pUserInfo: LPHSE_EXEC_URL_USER_INFO,
    pub pEntity: LPHSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: super::DWORD,
}
pub const HSE_EXEC_URL_NO_HEADERS: i32 = 2;
pub const HSE_EXEC_URL_SSI_CMD: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: super::USHORT,
    pub uHttpSubStatus: super::USHORT,
    pub dwWin32Error: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::HANDLE,
    pub pszCustomUserName: windows_sys::core::PSTR,
    pub pszCustomAuthType: windows_sys::core::PSTR,
}
pub const HSE_IO_ASYNC: i32 = 2;
pub const HSE_IO_CACHE_RESPONSE: i32 = 32;
pub const HSE_IO_DISCONNECT_AFTER_SEND: i32 = 4;
pub const HSE_IO_FINAL_SEND: i32 = 16;
pub const HSE_IO_NODELAY: i32 = 4096;
pub const HSE_IO_SEND_HEADERS: i32 = 8;
pub const HSE_IO_SYNC: i32 = 1;
pub const HSE_IO_TRY_SKIP_CUSTOM_ERRORS: i32 = 64;
pub const HSE_LOG_BUFFER_LEN: i32 = 80;
pub const HSE_MAX_EXT_DLL_NAME_LEN: i32 = 256;
pub const HSE_REQ_ABORTIVE_CLOSE: i32 = 1014;
pub const HSE_REQ_ASYNC_READ_CLIENT: i32 = 1010;
pub const HSE_REQ_BASE: i32 = 0;
pub const HSE_REQ_CANCEL_IO: i32 = 1049;
pub const HSE_REQ_CLOSE_CONNECTION: i32 = 1017;
pub const HSE_REQ_DONE_WITH_SESSION: i32 = 4;
pub const HSE_REQ_END_RESERVED: i32 = 1000;
pub const HSE_REQ_EXEC_UNICODE_URL: i32 = 1025;
pub const HSE_REQ_EXEC_URL: i32 = 1026;
pub const HSE_REQ_GET_ANONYMOUS_TOKEN: i32 = 1038;
pub const HSE_REQ_GET_CACHE_INVALIDATION_CALLBACK: i32 = 1040;
pub const HSE_REQ_GET_CERT_INFO_EX: i32 = 1015;
pub const HSE_REQ_GET_CHANNEL_BINDING_TOKEN: i32 = 1050;
pub const HSE_REQ_GET_CONFIG_OBJECT: i32 = 1046;
pub const HSE_REQ_GET_EXEC_URL_STATUS: i32 = 1027;
pub const HSE_REQ_GET_IMPERSONATION_TOKEN: i32 = 1011;
pub const HSE_REQ_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK: i32 = 1048;
pub const HSE_REQ_GET_SSPI_INFO: i32 = 1002;
pub const HSE_REQ_GET_TRACE_INFO: i32 = 1042;
pub const HSE_REQ_GET_TRACE_INFO_EX: i32 = 1044;
pub const HSE_REQ_GET_UNICODE_ANONYMOUS_TOKEN: i32 = 1041;
pub const HSE_REQ_GET_WORKER_PROCESS_SETTINGS: i32 = 1047;
pub const HSE_REQ_IO_COMPLETION: i32 = 1005;
pub const HSE_REQ_IS_CONNECTED: i32 = 1018;
pub const HSE_REQ_IS_IN_PROCESS: i32 = 1030;
pub const HSE_REQ_IS_KEEP_CONN: i32 = 1008;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH: i32 = 1023;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH_EX: i32 = 1024;
pub const HSE_REQ_MAP_URL_TO_PATH: i32 = 1001;
pub const HSE_REQ_MAP_URL_TO_PATH_EX: i32 = 1012;
pub const HSE_REQ_NORMALIZE_URL: i32 = 1033;
pub const HSE_REQ_RAISE_TRACE_EVENT: i32 = 1045;
pub const HSE_REQ_REFRESH_ISAPI_ACL: i32 = 1007;
pub const HSE_REQ_REPORT_UNHEALTHY: i32 = 1032;
pub const HSE_REQ_SEND_CUSTOM_ERROR: i32 = 1028;
pub const HSE_REQ_SEND_RESPONSE_HEADER: i32 = 3;
pub const HSE_REQ_SEND_RESPONSE_HEADER_EX: i32 = 1016;
pub const HSE_REQ_SEND_URL: i32 = 2;
pub const HSE_REQ_SEND_URL_REDIRECT_RESP: i32 = 1;
pub const HSE_REQ_SET_FLUSH_FLAG: i32 = 1043;
pub const HSE_REQ_TRANSMIT_FILE: i32 = 1006;
pub const HSE_REQ_VECTOR_SEND: i32 = 1037;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: super::DWORD,
    pub pszStatus: windows_sys::core::PSTR,
    pub pszHeaders: windows_sys::core::PSTR,
    pub nElementCount: super::DWORD,
    pub lpElementArray: LPHSE_VECTOR_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: windows_sys::core::PCSTR,
    pub pszHeader: windows_sys::core::PCSTR,
    pub cchStatus: super::DWORD,
    pub cchHeader: super::DWORD,
    pub fKeepConn: windows_sys::core::BOOL,
}
pub const HSE_STATUS_ERROR: i32 = 4;
pub const HSE_STATUS_PENDING: i32 = 3;
pub const HSE_STATUS_SUCCESS: i32 = 1;
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: i32 = 2;
pub const HSE_TERM_ADVISORY_UNLOAD: i32 = 1;
pub const HSE_TERM_MUST_UNLOAD: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_TF_INFO {
    pub pfnHseIO: PFN_HSE_IO_COMPLETION,
    pub pContext: super::PVOID,
    pub hFile: super::HANDLE,
    pub pszStatusCode: windows_sys::core::PCSTR,
    pub BytesToWrite: super::DWORD,
    pub Offset: super::DWORD,
    pub pHead: super::PVOID,
    pub HeadLength: super::DWORD,
    pub pTail: super::PVOID,
    pub TailLength: super::DWORD,
    pub dwFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: windows_sys::core::BOOL,
    pub TraceContextId: [super::BYTE; 16],
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HSE_UNICODE_URL_MAPEX_INFO {
    pub lpszPath: [super::WCHAR; 260],
    pub dwFlags: super::DWORD,
    pub cchMatchingPath: super::DWORD,
    pub cchMatchingURL: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_URL_FLAGS_DONT_CACHE: i32 = 16;
pub const HSE_URL_FLAGS_EXECUTE: i32 = 4;
pub const HSE_URL_FLAGS_MAP_CERT: i32 = 128;
pub const HSE_URL_FLAGS_MASK: i32 = 1023;
pub const HSE_URL_FLAGS_NEGO_CERT: i32 = 32;
pub const HSE_URL_FLAGS_READ: i32 = 1;
pub const HSE_URL_FLAGS_REQUIRE_CERT: i32 = 64;
pub const HSE_URL_FLAGS_SCRIPT: i32 = 512;
pub const HSE_URL_FLAGS_SSL: i32 = 8;
pub const HSE_URL_FLAGS_SSL128: i32 = 256;
pub const HSE_URL_FLAGS_WRITE: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [super::CHAR; 260],
    pub dwFlags: super::DWORD,
    pub cchMatchingPath: super::DWORD,
    pub cchMatchingURL: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HSE_VECTOR_ELEMENT {
    pub ElementType: super::DWORD,
    pub pvContext: super::PVOID,
    pub cbOffset: super::ULONGLONG,
    pub cbSize: super::ULONGLONG,
}
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: i32 = 1;
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: i32 = 0;
#[cfg(feature = "shared")]
pub const HSE_VERSION: super::LONG = 0x80000_u32 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: super::DWORD,
    pub lpszExtensionDesc: [super::CHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_VERSION_MAJOR: i32 = 8;
pub const HSE_VERSION_MINOR: i32 = 0;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPEXTENSION_CONTROL_BLOCK = *mut EXTENSION_CONTROL_BLOCK;
#[cfg(feature = "shared")]
pub type LPHSE_CUSTOM_ERROR_INFO = *mut HSE_CUSTOM_ERROR_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPHSE_EXEC_UNICODE_URL_INFO = *mut HSE_EXEC_UNICODE_URL_INFO;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_UNICODE_URL_USER_INFO = *mut HSE_EXEC_UNICODE_URL_USER_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPHSE_EXEC_URL_ENTITY_INFO = *mut HSE_EXEC_URL_ENTITY_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPHSE_EXEC_URL_INFO = *mut HSE_EXEC_URL_INFO;
#[cfg(feature = "shared")]
pub type LPHSE_EXEC_URL_STATUS = *mut HSE_EXEC_URL_STATUS;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_URL_USER_INFO = *mut HSE_EXEC_URL_USER_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPHSE_RESPONSE_VECTOR = *mut HSE_RESPONSE_VECTOR;
#[cfg(feature = "shared")]
pub type LPHSE_SEND_HEADER_EX_INFO = *mut HSE_SEND_HEADER_EX_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPHSE_TF_INFO = *mut HSE_TF_INFO;
#[cfg(feature = "shared")]
pub type LPHSE_TRACE_INFO = *mut HSE_TRACE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPHSE_UNICODE_URL_MAPEX_INFO = *mut HSE_UNICODE_URL_MAPEX_INFO;
#[cfg(feature = "shared")]
pub type LPHSE_URL_MAPEX_INFO = *mut HSE_URL_MAPEX_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPHSE_VECTOR_ELEMENT = *mut HSE_VECTOR_ELEMENT;
#[cfg(feature = "shared")]
pub type LPHSE_VERSION_INFO = *mut HSE_VERSION_INFO;
#[cfg(feature = "shared")]
pub type PFN_GETEXTENSIONVERSION = Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = Option<unsafe extern "system" fn(pszurl: *mut super::WCHAR) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = Option<unsafe extern "system" fn(pszprotocolmanagerdll: windows_sys::core::PCWSTR, pszprotocolmanagerdllinitfunction: windows_sys::core::PCWSTR, dwcustominterfaceid: super::DWORD, ppcustominterface: *mut super::PVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_HSE_IO_COMPLETION = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: super::PVOID, cbio: super::DWORD, dwerror: super::DWORD)>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PFN_HTTPEXTENSIONPROC = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_TERMINATEEXTENSION = Option<unsafe extern "system" fn(dwflags: super::DWORD) -> windows_sys::core::BOOL>;
