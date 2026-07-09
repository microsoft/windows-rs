#[inline]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> windows_core::BOOL {
    windows_core::link!("rpcproxy.dll" "system" fn GetFilterVersion(pver : *mut HTTP_FILTER_VERSION) -> windows_core::BOOL);
    unsafe { GetFilterVersion(pver as _) }
}
#[inline]
pub unsafe fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("rpcproxy.dll" "system" fn HttpFilterProc(pfc : *mut HTTP_FILTER_CONTEXT, notificationtype : u32, pvnotification : *mut core::ffi::c_void) -> u32);
    unsafe { HttpFilterProc(pfc as _, notificationtype, pvnotification as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: *const i8,
    pub pszPhysicalPath: *const i8,
    pub dwReason: u32,
}
impl Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: *mut i8,
    pub cbUserBuff: u32,
    pub pszPassword: *mut i8,
    pub cbPasswordBuff: u32,
}
impl Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: *mut u8,
    pub SetHeader: *mut u8,
    pub AddHeader: *mut u8,
    pub GetUserToken: *mut u8,
    pub HttpStatus: u32,
    pub fResetAuth: windows_core::BOOL,
    pub dwReserved: u32,
}
impl Default for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: u32,
    pub Revision: u32,
    pub ServerContext: *mut core::ffi::c_void,
    pub ulReserved: u32,
    pub fIsSecurePort: windows_core::BOOL,
    pub pFilterContext: *mut core::ffi::c_void,
    pub GetServerVariable: *mut u8,
    pub AddResponseHeaders: *mut u8,
    pub WriteClient: *mut u8,
    pub AllocMem: *mut u8,
    pub ServerSupportFunction: *mut u8,
}
impl Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: *const i8,
    pub pszClientUserName: *const i8,
    pub pszServerName: *const i8,
    pub pszOperation: *const i8,
    pub pszTarget: *const i8,
    pub pszParameters: *const i8,
    pub dwHttpStatus: u32,
    pub dwWin32Status: u32,
    pub dwBytesSent: u32,
    pub dwBytesRecvd: u32,
    pub msTimeForProcessing: u32,
}
impl Default for HTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: *mut u8,
    pub SetHeader: *mut u8,
    pub AddHeader: *mut u8,
    pub HttpStatus: u32,
    pub dwReserved: u32,
}
impl Default for HTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_RAW_DATA {
    pub pvInData: *mut core::ffi::c_void,
    pub cbInData: u32,
    pub cbInBuffer: u32,
    pub dwReserved: u32,
}
impl Default for HTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_FILTER_REVISION: u32 = 655360;
pub type HTTP_FILTER_SEND_RESPONSE = HTTP_FILTER_PREPROC_HEADERS;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: *const i8,
    pub pszPhysicalPath: *mut i8,
    pub cbPathBuff: u32,
}
impl Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: *const i8,
    pub pszPhysicalPath: *mut i8,
    pub cbPathBuff: u32,
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub pszScriptMapEntry: *const i8,
}
impl Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: u32,
    pub dwFilterVersion: u32,
    pub lpszFilterDesc: [i8; 257],
    pub dwFlags: u32,
}
impl Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_ACCESS_DENIED(pub *mut HTTP_FILTER_ACCESS_DENIED);
impl PHTTP_FILTER_ACCESS_DENIED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_AUTHENT(pub *mut HTTP_FILTER_AUTHENT);
impl PHTTP_FILTER_AUTHENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_AUTH_COMPLETE_INFO(pub *mut HTTP_FILTER_AUTH_COMPLETE_INFO);
impl PHTTP_FILTER_AUTH_COMPLETE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_CONTEXT(pub *mut HTTP_FILTER_CONTEXT);
impl PHTTP_FILTER_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_LOG(pub *mut HTTP_FILTER_LOG);
impl PHTTP_FILTER_LOG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_PREPROC_HEADERS(pub *mut HTTP_FILTER_PREPROC_HEADERS);
impl PHTTP_FILTER_PREPROC_HEADERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_RAW_DATA(pub *mut HTTP_FILTER_RAW_DATA);
impl PHTTP_FILTER_RAW_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_SEND_RESPONSE(pub *mut HTTP_FILTER_PREPROC_HEADERS);
impl PHTTP_FILTER_SEND_RESPONSE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_SEND_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_URL_MAP(pub *mut HTTP_FILTER_URL_MAP);
impl PHTTP_FILTER_URL_MAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_URL_MAP_EX(pub *mut HTTP_FILTER_URL_MAP_EX);
impl PHTTP_FILTER_URL_MAP_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTTP_FILTER_VERSION(pub *mut HTTP_FILTER_VERSION);
impl PHTTP_FILTER_VERSION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SF_DENIED_APPLICATION: u32 = 8;
pub const SF_DENIED_BY_CONFIG: u32 = 65536;
pub const SF_DENIED_FILTER: u32 = 4;
pub const SF_DENIED_LOGON: u32 = 1;
pub const SF_DENIED_RESOURCE: u32 = 2;
pub const SF_MAX_AUTH_TYPE: u32 = 33;
pub const SF_MAX_FILTER_DESC_LEN: u32 = 257;
pub const SF_MAX_PASSWORD: u32 = 257;
pub const SF_MAX_USERNAME: u32 = 257;
pub const SF_NOTIFY_ACCESS_DENIED: u32 = 2048;
pub const SF_NOTIFY_AUTHENTICATION: u32 = 8192;
pub const SF_NOTIFY_AUTH_COMPLETE: u32 = 67108864;
pub const SF_NOTIFY_END_OF_NET_SESSION: u32 = 256;
pub const SF_NOTIFY_END_OF_REQUEST: u32 = 128;
pub const SF_NOTIFY_LOG: u32 = 512;
pub const SF_NOTIFY_NONSECURE_PORT: u32 = 2;
pub const SF_NOTIFY_ORDER_DEFAULT: u32 = 131072;
pub const SF_NOTIFY_ORDER_HIGH: u32 = 524288;
pub const SF_NOTIFY_ORDER_LOW: u32 = 131072;
pub const SF_NOTIFY_ORDER_MASK: u32 = 917504;
pub const SF_NOTIFY_ORDER_MEDIUM: u32 = 262144;
pub const SF_NOTIFY_PREPROC_HEADERS: u32 = 16384;
pub const SF_NOTIFY_READ_RAW_DATA: u32 = 32768;
pub const SF_NOTIFY_SECURE_PORT: u32 = 1;
pub const SF_NOTIFY_SEND_RAW_DATA: u32 = 1024;
pub const SF_NOTIFY_SEND_RESPONSE: u32 = 64;
pub const SF_NOTIFY_URL_MAP: u32 = 4096;
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
