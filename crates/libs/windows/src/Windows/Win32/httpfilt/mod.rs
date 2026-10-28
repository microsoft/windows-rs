#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> windows_core::BOOL {
    windows_core::link!("rpcproxy.dll" "system" fn GetFilterVersion(pver : *mut HTTP_FILTER_VERSION) -> windows_core::BOOL);
    unsafe { GetFilterVersion(pver as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: super::DWORD, pvnotification: *mut core::ffi::c_void) -> super::DWORD {
    windows_core::link!("rpcproxy.dll" "system" fn HttpFilterProc(pfc : *mut HTTP_FILTER_CONTEXT, notificationtype : super::DWORD, pvnotification : *mut core::ffi::c_void) -> super::DWORD);
    unsafe { HttpFilterProc(pfc as _, notificationtype, pvnotification as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: *const super::CHAR,
    pub pszPhysicalPath: *const super::CHAR,
    pub dwReason: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: *mut super::CHAR,
    pub cbUserBuff: super::DWORD,
    pub pszPassword: *mut super::CHAR,
    pub cbPasswordBuff: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, super::LPVOID, super::LPDWORD) -> windows_core::BOOL>,
    pub SetHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, windows_core::PSTR) -> windows_core::BOOL>,
    pub AddHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, windows_core::PSTR) -> windows_core::BOOL>,
    pub GetUserToken: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, *mut super::HANDLE) -> windows_core::BOOL>,
    pub HttpStatus: super::DWORD,
    pub fResetAuth: windows_core::BOOL,
    pub dwReserved: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: super::DWORD,
    pub Revision: super::DWORD,
    pub ServerContext: super::PVOID,
    pub ulReserved: super::DWORD,
    pub fIsSecurePort: windows_core::BOOL,
    pub pFilterContext: super::PVOID,
    pub GetServerVariable: Option<unsafe extern "system" fn(*mut Self, windows_core::PSTR, super::LPVOID, super::LPDWORD) -> windows_core::BOOL>,
    pub AddResponseHeaders: Option<unsafe extern "system" fn(*mut Self, windows_core::PSTR, super::DWORD) -> windows_core::BOOL>,
    pub WriteClient: Option<unsafe extern "system" fn(*mut Self, super::LPVOID, super::LPDWORD, super::DWORD) -> windows_core::BOOL>,
    pub AllocMem: Option<unsafe extern "system" fn(*mut Self, super::DWORD, super::DWORD) -> *mut core::ffi::c_void>,
    pub ServerSupportFunction: Option<unsafe extern "system" fn(*mut Self, SF_REQ_TYPE, super::PVOID, super::ULONG_PTR, super::ULONG_PTR) -> windows_core::BOOL>,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: *const super::CHAR,
    pub pszClientUserName: *const super::CHAR,
    pub pszServerName: *const super::CHAR,
    pub pszOperation: *const super::CHAR,
    pub pszTarget: *const super::CHAR,
    pub pszParameters: *const super::CHAR,
    pub dwHttpStatus: super::DWORD,
    pub dwWin32Status: super::DWORD,
    pub dwBytesSent: super::DWORD,
    pub dwBytesRecvd: super::DWORD,
    pub msTimeForProcessing: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, super::LPVOID, super::LPDWORD) -> windows_core::BOOL>,
    pub SetHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, windows_core::PSTR) -> windows_core::BOOL>,
    pub AddHeader: Option<unsafe extern "system" fn(*mut HTTP_FILTER_CONTEXT, windows_core::PSTR, windows_core::PSTR) -> windows_core::BOOL>,
    pub HttpStatus: super::DWORD,
    pub dwReserved: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_RAW_DATA {
    pub pvInData: super::PVOID,
    pub cbInData: super::DWORD,
    pub cbInBuffer: super::DWORD,
    pub dwReserved: super::DWORD,
}
#[cfg(feature = "shared")]
pub const HTTP_FILTER_REVISION: super::LONG = 0xA0000_u32 as _;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type HTTP_FILTER_SEND_RESPONSE = HTTP_FILTER_PREPROC_HEADERS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: *const super::CHAR,
    pub pszPhysicalPath: *mut super::CHAR,
    pub cbPathBuff: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: *const super::CHAR,
    pub pszPhysicalPath: *mut super::CHAR,
    pub cbPathBuff: super::DWORD,
    pub dwFlags: super::DWORD,
    pub cchMatchingPath: super::DWORD,
    pub cchMatchingURL: super::DWORD,
    pub pszScriptMapEntry: *const super::CHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: super::DWORD,
    pub dwFilterVersion: super::DWORD,
    pub lpszFilterDesc: [super::CHAR; 257],
    pub dwFlags: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_ACCESS_DENIED = *mut HTTP_FILTER_ACCESS_DENIED;
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_AUTHENT = *mut HTTP_FILTER_AUTHENT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_FILTER_AUTH_COMPLETE_INFO = *mut HTTP_FILTER_AUTH_COMPLETE_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_FILTER_CONTEXT = *mut HTTP_FILTER_CONTEXT;
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_LOG = *mut HTTP_FILTER_LOG;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_FILTER_PREPROC_HEADERS = *mut HTTP_FILTER_PREPROC_HEADERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHTTP_FILTER_RAW_DATA = *mut HTTP_FILTER_RAW_DATA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PHTTP_FILTER_SEND_RESPONSE = *mut HTTP_FILTER_PREPROC_HEADERS;
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_URL_MAP = *mut HTTP_FILTER_URL_MAP;
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_URL_MAP_EX = *mut HTTP_FILTER_URL_MAP_EX;
#[cfg(feature = "shared")]
pub type PHTTP_FILTER_VERSION = *mut HTTP_FILTER_VERSION;
pub const SF_DENIED_APPLICATION: i32 = 8;
pub const SF_DENIED_BY_CONFIG: i32 = 65536;
pub const SF_DENIED_FILTER: i32 = 4;
pub const SF_DENIED_LOGON: i32 = 1;
pub const SF_DENIED_RESOURCE: i32 = 2;
pub const SF_MAX_AUTH_TYPE: i32 = 33;
pub const SF_MAX_FILTER_DESC_LEN: i32 = 257;
pub const SF_MAX_PASSWORD: i32 = 257;
pub const SF_MAX_USERNAME: i32 = 257;
pub const SF_NOTIFY_ACCESS_DENIED: i32 = 2048;
pub const SF_NOTIFY_AUTHENTICATION: i32 = 8192;
pub const SF_NOTIFY_AUTH_COMPLETE: i32 = 67108864;
pub const SF_NOTIFY_END_OF_NET_SESSION: i32 = 256;
pub const SF_NOTIFY_END_OF_REQUEST: i32 = 128;
pub const SF_NOTIFY_LOG: i32 = 512;
pub const SF_NOTIFY_NONSECURE_PORT: i32 = 2;
pub const SF_NOTIFY_ORDER_DEFAULT: i32 = 131072;
pub const SF_NOTIFY_ORDER_HIGH: i32 = 524288;
pub const SF_NOTIFY_ORDER_LOW: i32 = 131072;
pub const SF_NOTIFY_ORDER_MASK: i32 = 917504;
pub const SF_NOTIFY_ORDER_MEDIUM: i32 = 262144;
pub const SF_NOTIFY_PREPROC_HEADERS: i32 = 16384;
pub const SF_NOTIFY_READ_RAW_DATA: i32 = 32768;
pub const SF_NOTIFY_SECURE_PORT: i32 = 1;
pub const SF_NOTIFY_SEND_RAW_DATA: i32 = 1024;
pub const SF_NOTIFY_SEND_RESPONSE: i32 = 64;
pub const SF_NOTIFY_URL_MAP: i32 = 4096;
pub type SF_PROPERTY_IIS = i32;
pub const SF_PROPERTY_INSTANCE_NUM_ID: SF_PROPERTY_IIS = 1;
pub const SF_PROPERTY_SSL_CTXT: SF_PROPERTY_IIS = 0;
pub const SF_REQ_ADD_HEADERS_ON_DENIAL: SF_REQ_TYPE = 1;
pub const SF_REQ_DISABLE_NOTIFICATIONS: SF_REQ_TYPE = 8;
pub const SF_REQ_GET_CONNID: SF_REQ_TYPE = 4;
pub const SF_REQ_GET_PROPERTY: SF_REQ_TYPE = 6;
pub const SF_REQ_NORMALIZE_URL: SF_REQ_TYPE = 7;
pub const SF_REQ_SEND_RESPONSE_HEADER: SF_REQ_TYPE = 0;
pub const SF_REQ_SET_CERTIFICATE_INFO: SF_REQ_TYPE = 5;
pub const SF_REQ_SET_NEXT_READ_SIZE: SF_REQ_TYPE = 2;
pub const SF_REQ_SET_PROXY_INFO: SF_REQ_TYPE = 3;
pub type SF_REQ_TYPE = i32;
pub const SF_STATUS_REQ_ERROR: SF_STATUS_TYPE = 134217732;
pub const SF_STATUS_REQ_FINISHED: SF_STATUS_TYPE = 134217728;
pub const SF_STATUS_REQ_FINISHED_KEEP_CONN: SF_STATUS_TYPE = 134217729;
pub const SF_STATUS_REQ_HANDLED_NOTIFICATION: SF_STATUS_TYPE = 134217731;
pub const SF_STATUS_REQ_NEXT_NOTIFICATION: SF_STATUS_TYPE = 134217730;
pub const SF_STATUS_REQ_READ_NEXT: SF_STATUS_TYPE = 134217733;
pub type SF_STATUS_TYPE = i32;
