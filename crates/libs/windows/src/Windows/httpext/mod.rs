#[inline]
pub unsafe fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> windows_core::BOOL {
    windows_core::link!("rpcproxy.dll" "system" fn GetExtensionVersion(pver : *mut HSE_VERSION_INFO) -> windows_core::BOOL);
    unsafe { GetExtensionVersion(pver as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32 {
    windows_core::link!("rpcproxy.dll" "system" fn HttpExtensionProc(pecb : *const EXTENSION_CONTROL_BLOCK) -> u32);
    unsafe { HttpExtensionProc(pecb) }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::wincrypt::CERT_CONTEXT,
    pub cbAllocated: u32,
    pub dwCertificateFlags: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: HCONN,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [i8; 80],
    pub lpszMethod: windows_core::PSTR,
    pub lpszQueryString: windows_core::PSTR,
    pub lpszPathInfo: windows_core::PSTR,
    pub lpszPathTranslated: windows_core::PSTR,
    pub cbTotalBytes: u32,
    pub cbAvailable: u32,
    pub lpbData: super::minwindef::LPBYTE,
    pub lpszContentType: windows_core::PSTR,
    pub GetServerVariable: *mut u8,
    pub WriteClient: *mut u8,
    pub ReadClient: *mut u8,
    pub ServerSupportFunction: *mut u8,
}
#[cfg(feature = "minwindef")]
impl Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCONN(pub *mut core::ffi::c_void);
impl Default for HCONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003;
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0;
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1;
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: *mut i8,
    pub uHttpSubError: u16,
    pub fAsync: windows_core::BOOL,
}
impl Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: windows_core::PWSTR,
    pub pszMethod: windows_core::PSTR,
    pub pszChildHeaders: windows_core::PSTR,
    pub pUserInfo: LPHSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: LPHSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::winnt::HANDLE,
    pub pszCustomUserName: windows_core::PWSTR,
    pub pszCustomAuthType: windows_core::PSTR,
}
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_EXEC_URL_ENTITY_INFO {
    pub cbAvailable: u32,
    pub lpbData: *mut core::ffi::c_void,
}
impl Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: u32 = 128;
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: u32 = 4;
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: windows_core::PSTR,
    pub pszMethod: windows_core::PSTR,
    pub pszChildHeaders: windows_core::PSTR,
    pub pUserInfo: LPHSE_EXEC_URL_USER_INFO,
    pub pEntity: LPHSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2;
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: u16,
    pub uHttpSubStatus: u16,
    pub dwWin32Error: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::winnt::HANDLE,
    pub pszCustomUserName: windows_core::PSTR,
    pub pszCustomAuthType: windows_core::PSTR,
}
pub const HSE_IO_ASYNC: u32 = 2;
pub const HSE_IO_CACHE_RESPONSE: u32 = 32;
pub const HSE_IO_DISCONNECT_AFTER_SEND: u32 = 4;
pub const HSE_IO_FINAL_SEND: u32 = 16;
pub const HSE_IO_NODELAY: u32 = 4096;
pub const HSE_IO_SEND_HEADERS: u32 = 8;
pub const HSE_IO_SYNC: u32 = 1;
pub const HSE_IO_TRY_SKIP_CUSTOM_ERRORS: u32 = 64;
pub const HSE_LOG_BUFFER_LEN: u32 = 80;
pub const HSE_MAX_EXT_DLL_NAME_LEN: u32 = 256;
pub const HSE_REQ_ABORTIVE_CLOSE: u32 = 1014;
pub const HSE_REQ_ASYNC_READ_CLIENT: u32 = 1010;
pub const HSE_REQ_BASE: u32 = 0;
pub const HSE_REQ_CANCEL_IO: u32 = 1049;
pub const HSE_REQ_CLOSE_CONNECTION: u32 = 1017;
pub const HSE_REQ_DONE_WITH_SESSION: u32 = 4;
pub const HSE_REQ_END_RESERVED: u32 = 1000;
pub const HSE_REQ_EXEC_UNICODE_URL: u32 = 1025;
pub const HSE_REQ_EXEC_URL: u32 = 1026;
pub const HSE_REQ_GET_ANONYMOUS_TOKEN: u32 = 1038;
pub const HSE_REQ_GET_CACHE_INVALIDATION_CALLBACK: u32 = 1040;
pub const HSE_REQ_GET_CERT_INFO_EX: u32 = 1015;
pub const HSE_REQ_GET_CHANNEL_BINDING_TOKEN: u32 = 1050;
pub const HSE_REQ_GET_CONFIG_OBJECT: u32 = 1046;
pub const HSE_REQ_GET_EXEC_URL_STATUS: u32 = 1027;
pub const HSE_REQ_GET_IMPERSONATION_TOKEN: u32 = 1011;
pub const HSE_REQ_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK: u32 = 1048;
pub const HSE_REQ_GET_SSPI_INFO: u32 = 1002;
pub const HSE_REQ_GET_TRACE_INFO: u32 = 1042;
pub const HSE_REQ_GET_TRACE_INFO_EX: u32 = 1044;
pub const HSE_REQ_GET_UNICODE_ANONYMOUS_TOKEN: u32 = 1041;
pub const HSE_REQ_GET_WORKER_PROCESS_SETTINGS: u32 = 1047;
pub const HSE_REQ_IO_COMPLETION: u32 = 1005;
pub const HSE_REQ_IS_CONNECTED: u32 = 1018;
pub const HSE_REQ_IS_IN_PROCESS: u32 = 1030;
pub const HSE_REQ_IS_KEEP_CONN: u32 = 1008;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH: u32 = 1023;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH_EX: u32 = 1024;
pub const HSE_REQ_MAP_URL_TO_PATH: u32 = 1001;
pub const HSE_REQ_MAP_URL_TO_PATH_EX: u32 = 1012;
pub const HSE_REQ_NORMALIZE_URL: u32 = 1033;
pub const HSE_REQ_RAISE_TRACE_EVENT: u32 = 1045;
pub const HSE_REQ_REFRESH_ISAPI_ACL: u32 = 1007;
pub const HSE_REQ_REPORT_UNHEALTHY: u32 = 1032;
pub const HSE_REQ_SEND_CUSTOM_ERROR: u32 = 1028;
pub const HSE_REQ_SEND_RESPONSE_HEADER: u32 = 3;
pub const HSE_REQ_SEND_RESPONSE_HEADER_EX: u32 = 1016;
pub const HSE_REQ_SEND_URL: u32 = 2;
pub const HSE_REQ_SEND_URL_REDIRECT_RESP: u32 = 1;
pub const HSE_REQ_SET_FLUSH_FLAG: u32 = 1043;
pub const HSE_REQ_TRANSMIT_FILE: u32 = 1006;
pub const HSE_REQ_VECTOR_SEND: u32 = 1037;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: u32,
    pub pszStatus: windows_core::PSTR,
    pub pszHeaders: windows_core::PSTR,
    pub nElementCount: u32,
    pub lpElementArray: LPHSE_VECTOR_ELEMENT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: windows_core::PCSTR,
    pub pszHeader: windows_core::PCSTR,
    pub cchStatus: u32,
    pub cchHeader: u32,
    pub fKeepConn: windows_core::BOOL,
}
pub const HSE_STATUS_ERROR: u32 = 4;
pub const HSE_STATUS_PENDING: u32 = 3;
pub const HSE_STATUS_SUCCESS: u32 = 1;
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: u32 = 2;
pub const HSE_TERM_ADVISORY_UNLOAD: u32 = 1;
pub const HSE_TERM_MUST_UNLOAD: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct HSE_TF_INFO {
    pub pfnHseIO: PFN_HSE_IO_COMPLETION,
    pub pContext: *mut core::ffi::c_void,
    pub hFile: super::winnt::HANDLE,
    pub pszStatusCode: windows_core::PCSTR,
    pub BytesToWrite: u32,
    pub Offset: u32,
    pub pHead: *mut core::ffi::c_void,
    pub HeadLength: u32,
    pub pTail: *mut core::ffi::c_void,
    pub TailLength: u32,
    pub dwFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: windows_core::BOOL,
    pub TraceContextId: [u8; 16],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_UNICODE_URL_MAPEX_INFO {
    pub lpszPath: [u16; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
}
impl Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_URL_FLAGS_DONT_CACHE: u32 = 16;
pub const HSE_URL_FLAGS_EXECUTE: u32 = 4;
pub const HSE_URL_FLAGS_MAP_CERT: u32 = 128;
pub const HSE_URL_FLAGS_MASK: u32 = 1023;
pub const HSE_URL_FLAGS_NEGO_CERT: u32 = 32;
pub const HSE_URL_FLAGS_READ: u32 = 1;
pub const HSE_URL_FLAGS_REQUIRE_CERT: u32 = 64;
pub const HSE_URL_FLAGS_SCRIPT: u32 = 512;
pub const HSE_URL_FLAGS_SSL: u32 = 8;
pub const HSE_URL_FLAGS_SSL128: u32 = 256;
pub const HSE_URL_FLAGS_WRITE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [i8; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_VECTOR_ELEMENT {
    pub ElementType: u32,
    pub pvContext: *mut core::ffi::c_void,
    pub cbOffset: u64,
    pub cbSize: u64,
}
impl Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: u32 = 1;
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: u32 = 0;
pub const HSE_VERSION: u32 = 524288;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: u32,
    pub lpszExtensionDesc: [i8; 256],
}
impl Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_VERSION_MAJOR: u32 = 8;
pub const HSE_VERSION_MINOR: u32 = 0;
#[cfg(feature = "minwindef")]
pub type LPEXTENSION_CONTROL_BLOCK = *mut EXTENSION_CONTROL_BLOCK;
pub type LPHSE_CUSTOM_ERROR_INFO = *mut HSE_CUSTOM_ERROR_INFO;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_UNICODE_URL_INFO = *mut HSE_EXEC_UNICODE_URL_INFO;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_UNICODE_URL_USER_INFO = *mut HSE_EXEC_UNICODE_URL_USER_INFO;
pub type LPHSE_EXEC_URL_ENTITY_INFO = *mut HSE_EXEC_URL_ENTITY_INFO;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_URL_INFO = *mut HSE_EXEC_URL_INFO;
pub type LPHSE_EXEC_URL_STATUS = *mut HSE_EXEC_URL_STATUS;
#[cfg(feature = "winnt")]
pub type LPHSE_EXEC_URL_USER_INFO = *mut HSE_EXEC_URL_USER_INFO;
pub type LPHSE_RESPONSE_VECTOR = *mut HSE_RESPONSE_VECTOR;
pub type LPHSE_SEND_HEADER_EX_INFO = *mut HSE_SEND_HEADER_EX_INFO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPHSE_TF_INFO = *mut HSE_TF_INFO;
pub type LPHSE_TRACE_INFO = *mut HSE_TRACE_INFO;
pub type LPHSE_UNICODE_URL_MAPEX_INFO = *mut HSE_UNICODE_URL_MAPEX_INFO;
pub type LPHSE_URL_MAPEX_INFO = *mut HSE_URL_MAPEX_INFO;
pub type LPHSE_VECTOR_ELEMENT = *mut HSE_VECTOR_ELEMENT;
pub type LPHSE_VERSION_INFO = *mut HSE_VERSION_INFO;
pub type PFN_GETEXTENSIONVERSION = Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> windows_core::BOOL>;
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = Option<unsafe extern "system" fn(pszurl: *mut u16) -> windows_core::HRESULT>;
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = Option<unsafe extern "system" fn(pszprotocolmanagerdll: windows_core::PCWSTR, pszprotocolmanagerdllinitfunction: windows_core::PCWSTR, dwcustominterfaceid: u32, ppcustominterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "minwindef")]
pub type PFN_HSE_IO_COMPLETION = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: *mut core::ffi::c_void, cbio: u32, dwerror: u32)>;
#[cfg(feature = "minwindef")]
pub type PFN_HTTPEXTENSIONPROC = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> u32>;
pub type PFN_TERMINATEEXTENSION = Option<unsafe extern "system" fn(dwflags: u32) -> windows_core::BOOL>;
