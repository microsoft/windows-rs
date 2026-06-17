windows_link::link!("rpcproxy.dll" "system" fn GetExtensionVersion(pver : *mut HSE_VERSION_INFO) -> windows_sys::core::BOOL);
windows_link::link!("rpcproxy.dll" "system" fn GetFilterVersion(pver : *mut HTTP_FILTER_VERSION) -> windows_sys::core::BOOL);
windows_link::link!("rpcproxy.dll" "system" fn HttpExtensionProc(pecb : *const EXTENSION_CONTROL_BLOCK) -> u32);
windows_link::link!("rpcproxy.dll" "system" fn HttpFilterProc(pfc : *mut HTTP_FILTER_CONTEXT, notificationtype : u32, pvnotification : *mut core::ffi::c_void) -> u32);
pub const ADMINDATA_MAX_NAME_LEN: u32 = 256;
pub const ALL_METADATA: METADATATYPES = 0;
pub const APPCTR_MD_ID_BEGIN_RESERVED: u32 = 57344;
pub const APPCTR_MD_ID_END_RESERVED: u32 = 61439;
pub const APPSTATUS_NOTDEFINED: u32 = 2;
pub const APPSTATUS_RUNNING: u32 = 1;
pub const APPSTATUS_STOPPED: u32 = 0;
pub const ASP_MD_ID_BEGIN_RESERVED: u32 = 28672;
pub const ASP_MD_ID_END_RESERVED: u32 = 29951;
pub const ASP_MD_SERVER_BASE: u32 = 7000;
pub const ASP_MD_UT_APP: u32 = 101;
pub const BINARY_METADATA: METADATATYPES = 3;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Default)]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::super::Security::Cryptography::CERT_CONTEXT,
    pub cbAllocated: u32,
    pub dwCertificateFlags: u32,
}
pub const CLSID_IImgCtx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3050f3d6_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_IisServiceControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8fb8621_588f_11d2_9d61_00c04f79c5fe);
pub const CLSID_MSAdminBase_W: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9e69610_b80d_11d0_b9b9_00a0c922e750);
pub const CLSID_Request: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x920c25d0_25d9_11d0_a55f_00a0c90c2091);
pub const CLSID_Response: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46e19ba0_25dd_11d0_a55f_00a0c90c2091);
pub const CLSID_ScriptingContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd97a6da0_a868_11cf_83ae_11b0c90c2bd8);
pub const CLSID_Server: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa506d160_25e0_11d0_a55f_00a0c90c2091);
pub const CLSID_Session: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x509f8f20_25de_11d0_a55f_00a0c90c2091);
pub const CLSID_WamAdmin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x61738644_f196_11d0_9953_00c04fd919c1);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONFIGURATION_ENTRY {
    pub bstrKey: windows_sys::core::BSTR,
    pub bstrValue: windows_sys::core::BSTR,
}
impl Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPID_HTTPREQUEST_ABORT: u32 = 12;
pub const DISPID_HTTPREQUEST_BASE: u32 = 1;
pub const DISPID_HTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 4;
pub const DISPID_HTTPREQUEST_GETRESPONSEHEADER: u32 = 3;
pub const DISPID_HTTPREQUEST_OPEN: u32 = 1;
pub const DISPID_HTTPREQUEST_OPTION: u32 = 6;
pub const DISPID_HTTPREQUEST_RESPONSEBODY: u32 = 10;
pub const DISPID_HTTPREQUEST_RESPONSESTREAM: u32 = 11;
pub const DISPID_HTTPREQUEST_RESPONSETEXT: u32 = 9;
pub const DISPID_HTTPREQUEST_SEND: u32 = 5;
pub const DISPID_HTTPREQUEST_SETAUTOLOGONPOLICY: u32 = 18;
pub const DISPID_HTTPREQUEST_SETCLIENTCERTIFICATE: u32 = 17;
pub const DISPID_HTTPREQUEST_SETCREDENTIALS: u32 = 14;
pub const DISPID_HTTPREQUEST_SETPROXY: u32 = 13;
pub const DISPID_HTTPREQUEST_SETREQUESTHEADER: u32 = 2;
pub const DISPID_HTTPREQUEST_SETTIMEOUTS: u32 = 16;
pub const DISPID_HTTPREQUEST_STATUS: u32 = 7;
pub const DISPID_HTTPREQUEST_STATUSTEXT: u32 = 8;
pub const DISPID_HTTPREQUEST_WAITFORRESPONSE: u32 = 15;
pub const DWN_COLORMODE: u32 = 63;
pub const DWN_DOWNLOADONLY: u32 = 64;
pub const DWN_FORCEDITHER: u32 = 128;
pub const DWN_MIRRORIMAGE: u32 = 512;
pub const DWN_RAWIMAGE: u32 = 256;
pub const DWORD_METADATA: METADATATYPES = 1;
pub const EXPANDSZ_METADATA: METADATATYPES = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: HCONN,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [i8; 80],
    pub lpszMethod: windows_sys::core::PSTR,
    pub lpszQueryString: windows_sys::core::PSTR,
    pub lpszPathInfo: windows_sys::core::PSTR,
    pub lpszPathTranslated: windows_sys::core::PSTR,
    pub cbTotalBytes: u32,
    pub cbAvailable: u32,
    pub lpbData: *mut u8,
    pub lpszContentType: windows_sys::core::PSTR,
    pub GetServerVariable: PFN_IIS_GETSERVERVARIABLE,
    pub WriteClient: PFN_IIS_WRITECLIENT,
    pub ReadClient: PFN_IIS_READCLIENT,
    pub ServerSupportFunction: PFN_IIS_SERVERSUPPORTFUNCTION,
}
impl Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FP_MD_ID_BEGIN_RESERVED: u32 = 32768;
pub const FP_MD_ID_END_RESERVED: u32 = 36863;
pub type FTP_ACCESS = i32;
pub const FTP_ACCESS_NONE: FTP_ACCESS = 0;
pub const FTP_ACCESS_READ: FTP_ACCESS = 1;
pub const FTP_ACCESS_READ_WRITE: FTP_ACCESS = 3;
pub const FTP_ACCESS_WRITE: FTP_ACCESS = 2;
pub const FTP_PROCESS_CLOSE_SESSION: FTP_PROCESS_STATUS = 1;
pub const FTP_PROCESS_CONTINUE: FTP_PROCESS_STATUS = 0;
pub const FTP_PROCESS_REJECT_COMMAND: FTP_PROCESS_STATUS = 3;
pub type FTP_PROCESS_STATUS = i32;
pub const FTP_PROCESS_TERMINATE_SESSION: FTP_PROCESS_STATUS = 2;
pub const FtpProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x70bdc667_33b2_45f0_ac52_c3ca46f7a656);
pub const GUID_IIS_ALL_TRACE_PROVIDERS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_IIS_ASPNET_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaff081fe_0247_4275_9c4e_021f3dc1da35);
pub const GUID_IIS_ASP_TRACE_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06b94d9a_b15e_456e_a4ef_37c984a2cb4b);
pub const GUID_IIS_ISAPI_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1c2040e_8840_4c31_ba11_9871031a19ea);
pub const GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd55d3bc9_cba9_44df_827e_132d3a4596c2);
pub const GUID_IIS_WWW_SERVER_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3a2a4e84_4c21_4981_ae10_3fda0d9b0f83);
pub const GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xde4649c9_15e8_4fea_9d85_1cdda520c334);
pub type HCONN = *mut core::ffi::c_void;
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003;
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0;
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1;
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: windows_sys::core::PSTR,
    pub uHttpSubError: u16,
    pub fAsync: windows_sys::core::BOOL,
}
impl Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: windows_sys::core::PWSTR,
    pub pszMethod: windows_sys::core::PSTR,
    pub pszChildHeaders: windows_sys::core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
impl Default for HSE_EXEC_UNICODE_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: windows_sys::core::PWSTR,
    pub pszCustomAuthType: windows_sys::core::PSTR,
}
impl Default for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: windows_sys::core::PSTR,
    pub pszMethod: windows_sys::core::PSTR,
    pub pszChildHeaders: windows_sys::core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
impl Default for HSE_EXEC_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2;
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: u16,
    pub uHttpSubStatus: u16,
    pub dwWin32Error: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: windows_sys::core::PSTR,
    pub pszCustomAuthType: windows_sys::core::PSTR,
}
impl Default for HSE_EXEC_URL_USER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: u32,
    pub pszStatus: windows_sys::core::PSTR,
    pub pszHeaders: windows_sys::core::PSTR,
    pub nElementCount: u32,
    pub lpElementArray: *mut HSE_VECTOR_ELEMENT,
}
impl Default for HSE_RESPONSE_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: windows_sys::core::PCSTR,
    pub pszHeader: windows_sys::core::PCSTR,
    pub cchStatus: u32,
    pub cchHeader: u32,
    pub fKeepConn: windows_sys::core::BOOL,
}
impl Default for HSE_SEND_HEADER_EX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSE_STATUS_ERROR: u32 = 4;
pub const HSE_STATUS_PENDING: u32 = 3;
pub const HSE_STATUS_SUCCESS: u32 = 1;
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: u32 = 2;
pub const HSE_TERM_ADVISORY_UNLOAD: u32 = 1;
pub const HSE_TERM_MUST_UNLOAD: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_TF_INFO {
    pub pfnHseIO: PFN_HSE_IO_COMPLETION,
    pub pContext: *mut core::ffi::c_void,
    pub hFile: super::super::Foundation::HANDLE,
    pub pszStatusCode: windows_sys::core::PCSTR,
    pub BytesToWrite: u32,
    pub Offset: u32,
    pub pHead: *mut core::ffi::c_void,
    pub HeadLength: u32,
    pub pTail: *mut core::ffi::c_void,
    pub TailLength: u32,
    pub dwFlags: u32,
}
impl Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: windows_sys::core::BOOL,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: windows_sys::core::PCSTR,
    pub pszPhysicalPath: windows_sys::core::PCSTR,
    pub dwReason: u32,
}
impl Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: windows_sys::core::PSTR,
    pub cbUserBuff: u32,
    pub pszPassword: windows_sys::core::PSTR,
    pub cbPasswordBuff: u32,
}
impl Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub GetUserToken: isize,
    pub HttpStatus: u32,
    pub fResetAuth: windows_sys::core::BOOL,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: u32,
    pub Revision: u32,
    pub ServerContext: *mut core::ffi::c_void,
    pub ulReserved: u32,
    pub fIsSecurePort: windows_sys::core::BOOL,
    pub pFilterContext: *mut core::ffi::c_void,
    pub GetServerVariable: isize,
    pub AddResponseHeaders: isize,
    pub WriteClient: isize,
    pub AllocMem: isize,
    pub ServerSupportFunction: isize,
}
impl Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: windows_sys::core::PCSTR,
    pub pszClientUserName: windows_sys::core::PCSTR,
    pub pszServerName: windows_sys::core::PCSTR,
    pub pszOperation: windows_sys::core::PCSTR,
    pub pszTarget: windows_sys::core::PCSTR,
    pub pszParameters: windows_sys::core::PCSTR,
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
#[derive(Clone, Copy, Default)]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub HttpStatus: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: windows_sys::core::PCSTR,
    pub pszPhysicalPath: windows_sys::core::PSTR,
    pub cbPathBuff: u32,
}
impl Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: windows_sys::core::PCSTR,
    pub pszPhysicalPath: windows_sys::core::PSTR,
    pub cbPathBuff: u32,
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub pszScriptMapEntry: windows_sys::core::PCSTR,
}
impl Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_TRACE_CONFIGURATION {
    pub pProviderGuid: *const windows_sys::core::GUID,
    pub dwAreas: u32,
    pub dwVerbosity: u32,
    pub fProviderEnabled: windows_sys::core::BOOL,
}
impl Default for HTTP_TRACE_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_TRACE_EVENT {
    pub pProviderGuid: *const windows_sys::core::GUID,
    pub dwArea: u32,
    pub pAreaGuid: *const windows_sys::core::GUID,
    pub dwEvent: u32,
    pub pszEventName: windows_sys::core::PCWSTR,
    pub dwEventVersion: u32,
    pub dwVerbosity: u32,
    pub pActivityGuid: *const windows_sys::core::GUID,
    pub pRelatedActivityGuid: *const windows_sys::core::GUID,
    pub dwTimeStamp: u32,
    pub dwFlags: u32,
    pub cEventItems: u32,
    pub pEventItems: *mut HTTP_TRACE_EVENT_ITEM,
}
impl Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_TRACE_EVENT_ITEM {
    pub pszName: windows_sys::core::PCWSTR,
    pub dwDataType: HTTP_TRACE_TYPE,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pszDataDescription: windows_sys::core::PCWSTR,
}
impl Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_TRACE_LEVEL_END: u32 = 7;
pub const HTTP_TRACE_LEVEL_START: u32 = 6;
pub type HTTP_TRACE_TYPE = i32;
pub const HTTP_TRACE_TYPE_BOOL: HTTP_TRACE_TYPE = 11;
pub const HTTP_TRACE_TYPE_BYTE: HTTP_TRACE_TYPE = 17;
pub const HTTP_TRACE_TYPE_CHAR: HTTP_TRACE_TYPE = 16;
pub const HTTP_TRACE_TYPE_LONG: HTTP_TRACE_TYPE = 3;
pub const HTTP_TRACE_TYPE_LONGLONG: HTTP_TRACE_TYPE = 20;
pub const HTTP_TRACE_TYPE_LPCGUID: HTTP_TRACE_TYPE = 72;
pub const HTTP_TRACE_TYPE_LPCSTR: HTTP_TRACE_TYPE = 30;
pub const HTTP_TRACE_TYPE_LPCWSTR: HTTP_TRACE_TYPE = 31;
pub const HTTP_TRACE_TYPE_SHORT: HTTP_TRACE_TYPE = 2;
pub const HTTP_TRACE_TYPE_ULONG: HTTP_TRACE_TYPE = 19;
pub const HTTP_TRACE_TYPE_ULONGLONG: HTTP_TRACE_TYPE = 21;
pub const HTTP_TRACE_TYPE_USHORT: HTTP_TRACE_TYPE = 18;
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYA: windows_sys::core::PCSTR = windows_sys::core::s!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYW: windows_sys::core::PCWSTR = windows_sys::core::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_REG_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IISADMIN_EXTENSIONS_REG_KEYA: windows_sys::core::PCSTR = windows_sys::core::s!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IISADMIN_EXTENSIONS_REG_KEYW: windows_sys::core::PCWSTR = windows_sys::core::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IIS_CLASS_CERTMAPPER: windows_sys::core::PCSTR = windows_sys::core::s!("IIsCertMapper");
pub const IIS_CLASS_CERTMAPPER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsCertMapper");
pub const IIS_CLASS_COMPRESS_SCHEME: windows_sys::core::PCSTR = windows_sys::core::s!("IIsCompressionScheme");
pub const IIS_CLASS_COMPRESS_SCHEMES: windows_sys::core::PCSTR = windows_sys::core::s!("IIsCompressionSchemes");
pub const IIS_CLASS_COMPRESS_SCHEMES_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsCompressionSchemes");
pub const IIS_CLASS_COMPRESS_SCHEME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsCompressionScheme");
pub const IIS_CLASS_COMPUTER: windows_sys::core::PCSTR = windows_sys::core::s!("IIsComputer");
pub const IIS_CLASS_COMPUTER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsComputer");
pub const IIS_CLASS_FILTER: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFilter");
pub const IIS_CLASS_FILTERS: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFilters");
pub const IIS_CLASS_FILTERS_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFilters");
pub const IIS_CLASS_FILTER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFilter");
pub const IIS_CLASS_FTP_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFtpInfo");
pub const IIS_CLASS_FTP_INFO_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFtpInfo");
pub const IIS_CLASS_FTP_SERVER: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFtpServer");
pub const IIS_CLASS_FTP_SERVER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFtpServer");
pub const IIS_CLASS_FTP_SERVICE: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFtpService");
pub const IIS_CLASS_FTP_SERVICE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFtpService");
pub const IIS_CLASS_FTP_VDIR: windows_sys::core::PCSTR = windows_sys::core::s!("IIsFtpVirtualDir");
pub const IIS_CLASS_FTP_VDIR_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsFtpVirtualDir");
pub const IIS_CLASS_LOG_MODULE: windows_sys::core::PCSTR = windows_sys::core::s!("IIsLogModule");
pub const IIS_CLASS_LOG_MODULES: windows_sys::core::PCSTR = windows_sys::core::s!("IIsLogModules");
pub const IIS_CLASS_LOG_MODULES_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsLogModules");
pub const IIS_CLASS_LOG_MODULE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsLogModule");
pub const IIS_CLASS_MIMEMAP: windows_sys::core::PCSTR = windows_sys::core::s!("IIsMimeMap");
pub const IIS_CLASS_MIMEMAP_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsMimeMap");
pub const IIS_CLASS_WEB_DIR: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebDirectory");
pub const IIS_CLASS_WEB_DIR_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebDirectory");
pub const IIS_CLASS_WEB_FILE: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebFile");
pub const IIS_CLASS_WEB_FILE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebFile");
pub const IIS_CLASS_WEB_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebInfo");
pub const IIS_CLASS_WEB_INFO_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebInfo");
pub const IIS_CLASS_WEB_SERVER: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebServer");
pub const IIS_CLASS_WEB_SERVER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebServer");
pub const IIS_CLASS_WEB_SERVICE: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebService");
pub const IIS_CLASS_WEB_SERVICE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebService");
pub const IIS_CLASS_WEB_VDIR: windows_sys::core::PCSTR = windows_sys::core::s!("IIsWebVirtualDir");
pub const IIS_CLASS_WEB_VDIR_W: windows_sys::core::PCWSTR = windows_sys::core::w!("IIsWebVirtualDir");
pub const IIS_MD_ADSI_METAID_BEGIN: u32 = 130000;
pub const IIS_MD_ADSI_SCHEMA_PATH_A: windows_sys::core::PCSTR = windows_sys::core::s!("/Schema");
pub const IIS_MD_ADSI_SCHEMA_PATH_W: windows_sys::core::PCWSTR = windows_sys::core::w!("/Schema");
pub const IIS_MD_APPPOOL_BASE: u32 = 9000;
pub const IIS_MD_APP_BASE: u32 = 9100;
pub const IIS_MD_FILE_PROP_BASE: u32 = 6000;
pub const IIS_MD_FTP_BASE: u32 = 5000;
pub const IIS_MD_GLOBAL_BASE: u32 = 9200;
pub const IIS_MD_HTTP_BASE: u32 = 2000;
pub const IIS_MD_ID_BEGIN_RESERVED: u32 = 1;
pub const IIS_MD_ID_END_RESERVED: u32 = 32767;
pub const IIS_MD_INSTANCE_ROOT: windows_sys::core::PCSTR = windows_sys::core::s!("Root");
pub const IIS_MD_ISAPI_FILTERS: windows_sys::core::PCSTR = windows_sys::core::s!("/Filters");
pub const IIS_MD_LOCAL_MACHINE_PATH: windows_sys::core::PCSTR = windows_sys::core::s!("LM");
pub const IIS_MD_LOGCUSTOM_BASE: u32 = 4500;
pub const IIS_MD_LOGCUSTOM_LAST: u32 = 4508;
pub const IIS_MD_LOG_BASE: u32 = 4000;
pub const IIS_MD_LOG_LAST: u32 = 4015;
pub const IIS_MD_SERVER_BASE: u32 = 1000;
pub const IIS_MD_SSL_BASE: u32 = 5500;
pub const IIS_MD_SVC_INFO_PATH: windows_sys::core::PCSTR = windows_sys::core::s!("Info");
pub const IIS_MD_UT_END_RESERVED: u32 = 2000;
pub const IIS_MD_UT_FILE: u32 = 2;
pub const IIS_MD_UT_SERVER: u32 = 1;
pub const IIS_MD_UT_WAM: u32 = 100;
pub const IIS_MD_VR_BASE: u32 = 3000;
pub const IIS_WEBSOCKET: windows_sys::core::PCWSTR = windows_sys::core::w!("websockets");
pub const IIS_WEBSOCKET_SERVER_VARIABLE: windows_sys::core::PCSTR = windows_sys::core::s!("IIS_WEBSOCK");
pub const IMAP_MD_ID_BEGIN_RESERVED: u32 = 49152;
pub const IMAP_MD_ID_END_RESERVED: u32 = 53247;
pub const IMGANIM_ANIMATED: u32 = 268435456;
pub const IMGANIM_MASK: u32 = 268435456;
pub const IMGBITS_MASK: u32 = 234881024;
pub const IMGBITS_NONE: u32 = 33554432;
pub const IMGBITS_PARTIAL: u32 = 67108864;
pub const IMGBITS_TOTAL: u32 = 134217728;
pub const IMGCHG_ANIMATE: u32 = 8;
pub const IMGCHG_COMPLETE: u32 = 4;
pub const IMGCHG_MASK: u32 = 15;
pub const IMGCHG_SIZE: u32 = 1;
pub const IMGCHG_VIEW: u32 = 2;
pub const IMGLOAD_COMPLETE: u32 = 16777216;
pub const IMGLOAD_ERROR: u32 = 8388608;
pub const IMGLOAD_LOADING: u32 = 2097152;
pub const IMGLOAD_MASK: u32 = 32505856;
pub const IMGLOAD_NOTLOADED: u32 = 1048576;
pub const IMGLOAD_STOPPED: u32 = 4194304;
pub const IMGTRANS_MASK: u32 = 536870912;
pub const IMGTRANS_OPAQUE: u32 = 536870912;
pub const INVALID_END_METADATA: METADATATYPES = 6;
pub const LIBID_ASPTypeLibrary: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd97a6da0_a85c_11cf_83ae_00a0c90c2bd8);
pub const LIBID_IISRSTALib: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8fb8614_588f_11d2_9d61_00c04f79c5fe);
pub const LIBID_WAMREGLib: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x29822aa8_f302_11d0_9953_00c04fd919c1);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGGING_PARAMETERS {
    pub pszSessionId: windows_sys::core::PCWSTR,
    pub pszSiteName: windows_sys::core::PCWSTR,
    pub pszUserName: windows_sys::core::PCWSTR,
    pub pszHostName: windows_sys::core::PCWSTR,
    pub pszRemoteIpAddress: windows_sys::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_sys::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: windows_sys::core::PCWSTR,
    pub pszCommandParameters: windows_sys::core::PCWSTR,
    pub pszFullPath: windows_sys::core::PCWSTR,
    pub dwElapsedMilliseconds: u32,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: windows_sys::core::HRESULT,
    pub pszInformation: windows_sys::core::PCWSTR,
}
impl Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MB_DONT_IMPERSONATE: u32 = 9033;
pub const MD_ACCESS_EXECUTE: u32 = 4;
pub const MD_ACCESS_MAP_CERT: u32 = 128;
pub const MD_ACCESS_MASK: u32 = 65535;
pub const MD_ACCESS_NEGO_CERT: u32 = 32;
pub const MD_ACCESS_NO_PHYSICAL_DIR: u32 = 32768;
pub const MD_ACCESS_NO_REMOTE_EXECUTE: u32 = 8192;
pub const MD_ACCESS_NO_REMOTE_READ: u32 = 4096;
pub const MD_ACCESS_NO_REMOTE_SCRIPT: u32 = 16384;
pub const MD_ACCESS_NO_REMOTE_WRITE: u32 = 1024;
pub const MD_ACCESS_PERM: u32 = 6016;
pub const MD_ACCESS_READ: u32 = 1;
pub const MD_ACCESS_REQUIRE_CERT: u32 = 64;
pub const MD_ACCESS_SCRIPT: u32 = 512;
pub const MD_ACCESS_SOURCE: u32 = 16;
pub const MD_ACCESS_SSL: u32 = 8;
pub const MD_ACCESS_SSL128: u32 = 256;
pub const MD_ACCESS_WRITE: u32 = 2;
pub const MD_ACR_ENUM_KEYS: u32 = 8;
pub const MD_ACR_READ: u32 = 1;
pub const MD_ACR_RESTRICTED_WRITE: u32 = 32;
pub const MD_ACR_UNSECURE_PROPS_READ: u32 = 128;
pub const MD_ACR_WRITE: u32 = 2;
pub const MD_ACR_WRITE_DAC: u32 = 262144;
pub const MD_ADMIN_ACL: u32 = 6027;
pub const MD_ADMIN_INSTANCE: u32 = 2115;
pub const MD_ADV_CACHE_TTL: u32 = 2064;
pub const MD_ADV_NOTIFY_PWD_EXP_IN_DAYS: u32 = 2063;
pub const MD_AD_CONNECTIONS_PASSWORD: u32 = 5015;
pub const MD_AD_CONNECTIONS_USERNAME: u32 = 5014;
pub const MD_ALLOW_ANONYMOUS: u32 = 5005;
pub const MD_ALLOW_KEEPALIVES: u32 = 6038;
pub const MD_ALLOW_PATH_INFO_FOR_SCRIPT_MAPPINGS: u32 = 2095;
pub const MD_ALLOW_REPLACE_ON_RENAME: u32 = 5009;
pub const MD_ANONYMOUS_ONLY: u32 = 5006;
pub const MD_ANONYMOUS_PWD: u32 = 6021;
pub const MD_ANONYMOUS_USER_NAME: u32 = 6020;
pub const MD_ANONYMOUS_USE_SUBAUTH: u32 = 6022;
pub const MD_APPPOOL_32_BIT_APP_ON_WIN64: u32 = 9040;
pub const MD_APPPOOL_ALLOW_TRANSIENT_REGISTRATION: u32 = 9202;
pub const MD_APPPOOL_APPPOOL_ID: u32 = 9201;
pub const MD_APPPOOL_AUTO_SHUTDOWN_EXE: u32 = 9035;
pub const MD_APPPOOL_AUTO_SHUTDOWN_PARAMS: u32 = 9036;
pub const MD_APPPOOL_AUTO_START: u32 = 9028;
pub const MD_APPPOOL_COMMAND: u32 = 9026;
pub const MD_APPPOOL_COMMAND_START: u32 = 1;
pub const MD_APPPOOL_COMMAND_STOP: u32 = 2;
pub const MD_APPPOOL_DISALLOW_OVERLAPPING_ROTATION: u32 = 9015;
pub const MD_APPPOOL_DISALLOW_ROTATION_ON_CONFIG_CHANGE: u32 = 9018;
pub const MD_APPPOOL_EMULATION_ON_WINARM64: u32 = 9043;
pub const MD_APPPOOL_IDENTITY_TYPE: u32 = 9021;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSERVICE: u32 = 1;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSYSTEM: u32 = 0;
pub const MD_APPPOOL_IDENTITY_TYPE_NETWORKSERVICE: u32 = 2;
pub const MD_APPPOOL_IDENTITY_TYPE_SPECIFICUSER: u32 = 3;
pub const MD_APPPOOL_IDLE_TIMEOUT: u32 = 9005;
pub const MD_APPPOOL_MANAGED_PIPELINE_MODE: u32 = 9041;
pub const MD_APPPOOL_MANAGED_RUNTIME_VERSION: u32 = 9039;
pub const MD_APPPOOL_MAX_PROCESS_COUNT: u32 = 9003;
pub const MD_APPPOOL_ORPHAN_ACTION_EXE: u32 = 9031;
pub const MD_APPPOOL_ORPHAN_ACTION_PARAMS: u32 = 9032;
pub const MD_APPPOOL_ORPHAN_PROCESSES_FOR_DEBUGGING: u32 = 9009;
pub const MD_APPPOOL_PERIODIC_RESTART_CONNECTIONS: u32 = 9104;
pub const MD_APPPOOL_PERIODIC_RESTART_MEMORY: u32 = 9024;
pub const MD_APPPOOL_PERIODIC_RESTART_PRIVATE_MEMORY: u32 = 9038;
pub const MD_APPPOOL_PERIODIC_RESTART_REQUEST_COUNT: u32 = 9002;
pub const MD_APPPOOL_PERIODIC_RESTART_SCHEDULE: u32 = 9020;
pub const MD_APPPOOL_PERIODIC_RESTART_TIME: u32 = 9001;
pub const MD_APPPOOL_PINGING_ENABLED: u32 = 9004;
pub const MD_APPPOOL_PING_INTERVAL: u32 = 9013;
pub const MD_APPPOOL_PING_RESPONSE_TIMELIMIT: u32 = 9014;
pub const MD_APPPOOL_RAPID_FAIL_PROTECTION_ENABLED: u32 = 9006;
pub const MD_APPPOOL_SHUTDOWN_TIMELIMIT: u32 = 9012;
pub const MD_APPPOOL_SMP_AFFINITIZED: u32 = 9007;
pub const MD_APPPOOL_SMP_AFFINITIZED_PROCESSOR_MASK: u32 = 9008;
pub const MD_APPPOOL_STARTUP_TIMELIMIT: u32 = 9011;
pub const MD_APPPOOL_STATE: u32 = 9027;
pub const MD_APPPOOL_STATE_STARTED: u32 = 2;
pub const MD_APPPOOL_STATE_STARTING: u32 = 1;
pub const MD_APPPOOL_STATE_STOPPED: u32 = 4;
pub const MD_APPPOOL_STATE_STOPPING: u32 = 3;
pub const MD_APPPOOL_UL_APPPOOL_QUEUE_LENGTH: u32 = 9017;
pub const MD_APP_ALLOW_TRANSIENT_REGISTRATION: u32 = 9102;
pub const MD_APP_APPPOOL_ID: u32 = 9101;
pub const MD_APP_AUTO_START: u32 = 9103;
pub const MD_APP_DEPENDENCIES: u32 = 2167;
pub const MD_APP_FRIENDLY_NAME: u32 = 2102;
pub const MD_APP_ISOLATED: u32 = 2104;
pub const MD_APP_OOP_RECOVER_LIMIT: u32 = 2110;
pub const MD_APP_PACKAGE_ID: u32 = 2106;
pub const MD_APP_PACKAGE_NAME: u32 = 2107;
pub const MD_APP_PERIODIC_RESTART_REQUESTS: u32 = 2112;
pub const MD_APP_PERIODIC_RESTART_SCHEDULE: u32 = 2113;
pub const MD_APP_PERIODIC_RESTART_TIME: u32 = 2111;
pub const MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL: u32 = 9042;
pub const MD_APP_POOL_LOG_EVENT_ON_RECYCLE: u32 = 9037;
pub const MD_APP_POOL_PROCESSMODEL_IDLE_TIMEOUT: u32 = 1;
pub const MD_APP_POOL_RECYCLE_CONFIG_CHANGE: u32 = 64;
pub const MD_APP_POOL_RECYCLE_ISAPI_UNHEALTHY: u32 = 16;
pub const MD_APP_POOL_RECYCLE_MEMORY: u32 = 8;
pub const MD_APP_POOL_RECYCLE_ON_DEMAND: u32 = 32;
pub const MD_APP_POOL_RECYCLE_PRIVATE_MEMORY: u32 = 128;
pub const MD_APP_POOL_RECYCLE_REQUESTS: u32 = 2;
pub const MD_APP_POOL_RECYCLE_SCHEDULE: u32 = 4;
pub const MD_APP_POOL_RECYCLE_TIME: u32 = 1;
pub const MD_APP_ROOT: u32 = 2103;
pub const MD_APP_SHUTDOWN_TIME_LIMIT: u32 = 2114;
pub const MD_APP_TRACE_URL_LIST: u32 = 2118;
pub const MD_APP_WAM_CLSID: u32 = 2105;
pub const MD_ASP_ALLOWOUTOFPROCCMPNTS: u32 = 7014;
pub const MD_ASP_ALLOWOUTOFPROCCOMPONENTS: u32 = 7014;
pub const MD_ASP_ALLOWSESSIONSTATE: u32 = 7011;
pub const MD_ASP_BUFFERINGON: u32 = 7000;
pub const MD_ASP_BUFFER_LIMIT: u32 = 7052;
pub const MD_ASP_CALCLINENUMBER: u32 = 7050;
pub const MD_ASP_CODEPAGE: u32 = 7016;
pub const MD_ASP_DISKTEMPLATECACHEDIRECTORY: u32 = 7036;
pub const MD_ASP_ENABLEAPPLICATIONRESTART: u32 = 7027;
pub const MD_ASP_ENABLEASPHTMLFALLBACK: u32 = 7021;
pub const MD_ASP_ENABLECHUNKEDENCODING: u32 = 7022;
pub const MD_ASP_ENABLECLIENTDEBUG: u32 = 7019;
pub const MD_ASP_ENABLEPARENTPATHS: u32 = 7008;
pub const MD_ASP_ENABLESERVERDEBUG: u32 = 7018;
pub const MD_ASP_ENABLETYPELIBCACHE: u32 = 7023;
pub const MD_ASP_ERRORSTONTLOG: u32 = 7024;
pub const MD_ASP_EXCEPTIONCATCHENABLE: u32 = 7015;
pub const MD_ASP_EXECUTEINMTA: u32 = 7041;
pub const MD_ASP_ID_LAST: u32 = 7053;
pub const MD_ASP_KEEPSESSIONIDSECURE: u32 = 7043;
pub const MD_ASP_LCID: u32 = 7042;
pub const MD_ASP_LOGERRORREQUESTS: u32 = 7001;
pub const MD_ASP_MAXDISKTEMPLATECACHEFILES: u32 = 7040;
pub const MD_ASP_MAXREQUESTENTITY: u32 = 7053;
pub const MD_ASP_MAX_REQUEST_ENTITY_ALLOWED: u32 = 7053;
pub const MD_ASP_MEMFREEFACTOR: u32 = 7009;
pub const MD_ASP_MINUSEDBLOCKS: u32 = 7010;
pub const MD_ASP_PROCESSORTHREADMAX: u32 = 7025;
pub const MD_ASP_QUEUECONNECTIONTESTTIME: u32 = 7028;
pub const MD_ASP_QUEUETIMEOUT: u32 = 7013;
pub const MD_ASP_REQEUSTQUEUEMAX: u32 = 7026;
pub const MD_ASP_RUN_ONEND_ANON: u32 = 7051;
pub const MD_ASP_SCRIPTENGINECACHEMAX: u32 = 7005;
pub const MD_ASP_SCRIPTERRORMESSAGE: u32 = 7003;
pub const MD_ASP_SCRIPTERRORSSENTTOBROWSER: u32 = 7002;
pub const MD_ASP_SCRIPTFILECACHESIZE: u32 = 7004;
pub const MD_ASP_SCRIPTLANGUAGE: u32 = 7012;
pub const MD_ASP_SCRIPTLANGUAGELIST: u32 = 7017;
pub const MD_ASP_SCRIPTTIMEOUT: u32 = 7006;
pub const MD_ASP_SERVICE_ENABLE_SXS: u32 = 2;
pub const MD_ASP_SERVICE_ENABLE_TRACKER: u32 = 1;
pub const MD_ASP_SERVICE_FLAGS: u32 = 7044;
pub const MD_ASP_SERVICE_FLAG_FUSION: u32 = 7046;
pub const MD_ASP_SERVICE_FLAG_PARTITIONS: u32 = 7047;
pub const MD_ASP_SERVICE_FLAG_TRACKER: u32 = 7045;
pub const MD_ASP_SERVICE_PARTITION_ID: u32 = 7048;
pub const MD_ASP_SERVICE_SXS_NAME: u32 = 7049;
pub const MD_ASP_SERVICE_USE_PARTITION: u32 = 4;
pub const MD_ASP_SESSIONMAX: u32 = 7029;
pub const MD_ASP_SESSIONTIMEOUT: u32 = 7007;
pub const MD_ASP_THREADGATEENABLED: u32 = 7030;
pub const MD_ASP_THREADGATELOADHIGH: u32 = 7035;
pub const MD_ASP_THREADGATELOADLOW: u32 = 7034;
pub const MD_ASP_THREADGATESLEEPDELAY: u32 = 7032;
pub const MD_ASP_THREADGATESLEEPMAX: u32 = 7033;
pub const MD_ASP_THREADGATETIMESLICE: u32 = 7031;
pub const MD_ASP_TRACKTHREADINGMODEL: u32 = 7020;
pub const MD_AUTHORIZATION: u32 = 6000;
pub const MD_AUTHORIZATION_PERSISTENCE: u32 = 6031;
pub const MD_AUTH_ADVNOTIFY_DISABLE: u32 = 4;
pub const MD_AUTH_ANONYMOUS: u32 = 1;
pub const MD_AUTH_BASIC: u32 = 2;
pub const MD_AUTH_CHANGE_DISABLE: u32 = 2;
pub const MD_AUTH_CHANGE_FLAGS: u32 = 2068;
pub const MD_AUTH_CHANGE_UNSECURE: u32 = 1;
pub const MD_AUTH_CHANGE_URL: u32 = 2060;
pub const MD_AUTH_EXPIRED_UNSECUREURL: u32 = 2067;
pub const MD_AUTH_EXPIRED_URL: u32 = 2061;
pub const MD_AUTH_MD5: u32 = 16;
pub const MD_AUTH_NT: u32 = 4;
pub const MD_AUTH_PASSPORT: u32 = 64;
pub const MD_AUTH_SINGLEREQUEST: u32 = 64;
pub const MD_AUTH_SINGLEREQUESTALWAYSIFPROXY: u32 = 256;
pub const MD_AUTH_SINGLEREQUESTIFPROXY: u32 = 128;
pub const MD_BACKUP_FORCE_BACKUP: u32 = 4;
pub const MD_BACKUP_HIGHEST_VERSION: u32 = 4294967294;
pub const MD_BACKUP_MAX_LEN: u32 = 100;
pub const MD_BACKUP_MAX_VERSION: u32 = 9999;
pub const MD_BACKUP_NEXT_VERSION: u32 = 4294967295;
pub const MD_BACKUP_OVERWRITE: u32 = 1;
pub const MD_BACKUP_SAVE_FIRST: u32 = 2;
pub const MD_BANNER_MESSAGE: u32 = 5011;
pub const MD_BINDINGS: u32 = 2022;
pub const MD_CACHE_EXTENSIONS: u32 = 6034;
pub const MD_CAL_AUTH_RESERVE_TIMEOUT: u32 = 2131;
pub const MD_CAL_SSL_RESERVE_TIMEOUT: u32 = 2132;
pub const MD_CAL_VC_PER_CONNECT: u32 = 2130;
pub const MD_CAL_W3_ERROR: u32 = 2133;
pub const MD_CC_MAX_AGE: u32 = 6042;
pub const MD_CC_NO_CACHE: u32 = 6041;
pub const MD_CC_OTHER: u32 = 6043;
pub const MD_CENTRAL_W3C_LOGGING_ENABLED: u32 = 2119;
pub const MD_CERT_CACHE_RETRIEVAL_ONLY: u32 = 2;
pub const MD_CERT_CHECK_REVOCATION_FRESHNESS_TIME: u32 = 4;
pub const MD_CERT_NO_REVOC_CHECK: u32 = 1;
pub const MD_CERT_NO_USAGE_CHECK: u32 = 65536;
pub const MD_CGI_RESTRICTION_LIST: u32 = 2164;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MD_CHANGE_OBJECT_W {
    pub pszMDPath: windows_sys::core::PWSTR,
    pub dwMDChangeType: u32,
    pub dwMDNumDataIDs: u32,
    pub pdwMDDataIDs: *mut u32,
}
impl Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MD_CHANGE_TYPE_ADD_OBJECT: u32 = 2;
pub const MD_CHANGE_TYPE_DELETE_DATA: u32 = 8;
pub const MD_CHANGE_TYPE_DELETE_OBJECT: u32 = 1;
pub const MD_CHANGE_TYPE_RENAME_OBJECT: u32 = 16;
pub const MD_CHANGE_TYPE_RESTORE: u32 = 32;
pub const MD_CHANGE_TYPE_SET_DATA: u32 = 4;
pub const MD_COMMENTS: u32 = 9990;
pub const MD_CONNECTION_TIMEOUT: u32 = 1013;
pub const MD_CPU_ACTION: u32 = 9022;
pub const MD_CPU_APP_ENABLED: u32 = 2141;
pub const MD_CPU_CGI_ENABLED: u32 = 2140;
pub const MD_CPU_CGI_LIMIT: u32 = 2148;
pub const MD_CPU_DISABLE_ALL_LOGGING: u32 = 0;
pub const MD_CPU_ENABLE_ACTIVE_PROCS: u32 = 64;
pub const MD_CPU_ENABLE_ALL_PROC_LOGGING: u32 = 1;
pub const MD_CPU_ENABLE_APP_LOGGING: u32 = 4;
pub const MD_CPU_ENABLE_CGI_LOGGING: u32 = 2;
pub const MD_CPU_ENABLE_EVENT: u32 = 1;
pub const MD_CPU_ENABLE_KERNEL_TIME: u32 = 8;
pub const MD_CPU_ENABLE_LOGGING: u32 = 2147483648;
pub const MD_CPU_ENABLE_PAGE_FAULTS: u32 = 16;
pub const MD_CPU_ENABLE_PROC_TYPE: u32 = 2;
pub const MD_CPU_ENABLE_TERMINATED_PROCS: u32 = 128;
pub const MD_CPU_ENABLE_TOTAL_PROCS: u32 = 32;
pub const MD_CPU_ENABLE_USER_TIME: u32 = 4;
pub const MD_CPU_KILL_W3WP: u32 = 1;
pub const MD_CPU_LIMIT: u32 = 9023;
pub const MD_CPU_LIMITS_ENABLED: u32 = 2143;
pub const MD_CPU_LIMIT_LOGEVENT: u32 = 2149;
pub const MD_CPU_LIMIT_PAUSE: u32 = 2152;
pub const MD_CPU_LIMIT_PRIORITY: u32 = 2150;
pub const MD_CPU_LIMIT_PROCSTOP: u32 = 2151;
pub const MD_CPU_LOGGING_INTERVAL: u32 = 2145;
pub const MD_CPU_LOGGING_MASK: u32 = 4507;
pub const MD_CPU_LOGGING_OPTIONS: u32 = 2146;
pub const MD_CPU_NO_ACTION: u32 = 0;
pub const MD_CPU_RESET_INTERVAL: u32 = 2144;
pub const MD_CPU_THROTTLE: u32 = 3;
pub const MD_CPU_TRACE: u32 = 2;
pub const MD_CREATE_PROCESS_AS_USER: u32 = 6035;
pub const MD_CREATE_PROC_NEW_CONSOLE: u32 = 6036;
pub const MD_CUSTOM_DEPLOYMENT_DATA: u32 = 6055;
pub const MD_CUSTOM_ERROR: u32 = 6008;
pub const MD_CUSTOM_ERROR_DESC: u32 = 2120;
pub const MD_DEFAULT_BACKUP_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("MDBackUp");
pub const MD_DEFAULT_LOAD_FILE: u32 = 6006;
pub const MD_DEFAULT_LOGON_DOMAIN: u32 = 6012;
pub const MD_DEMAND_START_THRESHOLD: u32 = 9207;
pub const MD_DIRBROW_ENABLED: u32 = 2147483648;
pub const MD_DIRBROW_LOADDEFAULT: u32 = 1073741824;
pub const MD_DIRBROW_LONG_DATE: u32 = 32;
pub const MD_DIRBROW_SHOW_DATE: u32 = 2;
pub const MD_DIRBROW_SHOW_EXTENSION: u32 = 16;
pub const MD_DIRBROW_SHOW_SIZE: u32 = 8;
pub const MD_DIRBROW_SHOW_TIME: u32 = 4;
pub const MD_DIRECTORY_BROWSING: u32 = 6005;
pub const MD_DISABLE_SOCKET_POOLING: u32 = 1029;
pub const MD_DONT_LOG: u32 = 6023;
pub const MD_DOWNLEVEL_ADMIN_INSTANCE: u32 = 1021;
pub const MD_DO_REVERSE_DNS: u32 = 6029;
pub const MD_ENABLEDPROTOCOLS: u32 = 2023;
pub const MD_ENABLE_URL_AUTHORIZATION: u32 = 6048;
pub const MD_ERROR_CANNOT_REMOVE_SECURE_ATTRIBUTE: i32 = -2146646008;
pub const MD_ERROR_DATA_NOT_FOUND: i32 = -2146646015;
pub const MD_ERROR_IISAO_INVALID_SCHEMA: i32 = -2146646000;
pub const MD_ERROR_INVALID_VERSION: i32 = -2146646014;
pub const MD_ERROR_NOT_INITIALIZED: i32 = -2146646016;
pub const MD_ERROR_NO_SESSION_KEY: i32 = -2146645987;
pub const MD_ERROR_READ_METABASE_FILE: i32 = -2146645991;
pub const MD_ERROR_SECURE_CHANNEL_FAILURE: i32 = -2146646010;
pub const MD_ERROR_SUB400_INVALID_CONTENT_LENGTH: u32 = 7;
pub const MD_ERROR_SUB400_INVALID_DEPTH: u32 = 2;
pub const MD_ERROR_SUB400_INVALID_DESTINATION: u32 = 1;
pub const MD_ERROR_SUB400_INVALID_IF: u32 = 3;
pub const MD_ERROR_SUB400_INVALID_LOCK_TOKEN: u32 = 9;
pub const MD_ERROR_SUB400_INVALID_OVERWRITE: u32 = 4;
pub const MD_ERROR_SUB400_INVALID_REQUEST_BODY: u32 = 6;
pub const MD_ERROR_SUB400_INVALID_TIMEOUT: u32 = 8;
pub const MD_ERROR_SUB400_INVALID_TRANSLATE: u32 = 5;
pub const MD_ERROR_SUB400_INVALID_WEBSOCKET_REQUEST: u32 = 11;
pub const MD_ERROR_SUB400_INVALID_XFF_HEADER: u32 = 10;
pub const MD_ERROR_SUB401_APPLICATION: u32 = 5;
pub const MD_ERROR_SUB401_FILTER: u32 = 4;
pub const MD_ERROR_SUB401_LOGON: u32 = 1;
pub const MD_ERROR_SUB401_LOGON_ACL: u32 = 3;
pub const MD_ERROR_SUB401_LOGON_CONFIG: u32 = 2;
pub const MD_ERROR_SUB401_URLAUTH_POLICY: u32 = 7;
pub const MD_ERROR_SUB403_ADDR_REJECT: u32 = 6;
pub const MD_ERROR_SUB403_APPPOOL_DENIED: u32 = 18;
pub const MD_ERROR_SUB403_CAL_EXCEEDED: u32 = 15;
pub const MD_ERROR_SUB403_CERT_BAD: u32 = 16;
pub const MD_ERROR_SUB403_CERT_REQUIRED: u32 = 7;
pub const MD_ERROR_SUB403_CERT_REVOKED: u32 = 13;
pub const MD_ERROR_SUB403_CERT_TIME_INVALID: u32 = 17;
pub const MD_ERROR_SUB403_DIR_LIST_DENIED: u32 = 14;
pub const MD_ERROR_SUB403_EXECUTE_ACCESS_DENIED: u32 = 1;
pub const MD_ERROR_SUB403_INFINITE_DEPTH_DENIED: u32 = 22;
pub const MD_ERROR_SUB403_INSUFFICIENT_PRIVILEGE_FOR_CGI: u32 = 19;
pub const MD_ERROR_SUB403_INVALID_CNFG: u32 = 10;
pub const MD_ERROR_SUB403_LOCK_TOKEN_REQUIRED: u32 = 23;
pub const MD_ERROR_SUB403_MAPPER_DENY_ACCESS: u32 = 12;
pub const MD_ERROR_SUB403_PASSPORT_LOGIN_FAILURE: u32 = 20;
pub const MD_ERROR_SUB403_PWD_CHANGE: u32 = 11;
pub const MD_ERROR_SUB403_READ_ACCESS_DENIED: u32 = 2;
pub const MD_ERROR_SUB403_SITE_ACCESS_DENIED: u32 = 8;
pub const MD_ERROR_SUB403_SOURCE_ACCESS_DENIED: u32 = 21;
pub const MD_ERROR_SUB403_SSL128_REQUIRED: u32 = 5;
pub const MD_ERROR_SUB403_SSL_REQUIRED: u32 = 4;
pub const MD_ERROR_SUB403_TOO_MANY_USERS: u32 = 9;
pub const MD_ERROR_SUB403_VALIDATION_FAILURE: u32 = 24;
pub const MD_ERROR_SUB403_WRITE_ACCESS_DENIED: u32 = 3;
pub const MD_ERROR_SUB404_DENIED_BY_FILTERING_RULE: u32 = 19;
pub const MD_ERROR_SUB404_DENIED_BY_MIMEMAP: u32 = 3;
pub const MD_ERROR_SUB404_DENIED_BY_POLICY: u32 = 2;
pub const MD_ERROR_SUB404_FILE_ATTRIBUTE_HIDDEN: u32 = 9;
pub const MD_ERROR_SUB404_FILE_EXTENSION_DENIED: u32 = 7;
pub const MD_ERROR_SUB404_HIDDEN_SEGMENT: u32 = 8;
pub const MD_ERROR_SUB404_NO_HANDLER: u32 = 4;
pub const MD_ERROR_SUB404_PRECONDITIONED_HANDLER: u32 = 17;
pub const MD_ERROR_SUB404_QUERY_STRING_SEQUENCE_DENIED: u32 = 18;
pub const MD_ERROR_SUB404_QUERY_STRING_TOO_LONG: u32 = 15;
pub const MD_ERROR_SUB404_SITE_NOT_FOUND: u32 = 1;
pub const MD_ERROR_SUB404_STATICFILE_DAV: u32 = 16;
pub const MD_ERROR_SUB404_TOO_MANY_URL_SEGMENTS: u32 = 20;
pub const MD_ERROR_SUB404_URL_DOUBLE_ESCAPED: u32 = 11;
pub const MD_ERROR_SUB404_URL_HAS_HIGH_BIT_CHARS: u32 = 12;
pub const MD_ERROR_SUB404_URL_SEQUENCE_DENIED: u32 = 5;
pub const MD_ERROR_SUB404_URL_TOO_LONG: u32 = 14;
pub const MD_ERROR_SUB404_VERB_DENIED: u32 = 6;
pub const MD_ERROR_SUB413_CONTENT_LENGTH_TOO_LARGE: u32 = 1;
pub const MD_ERROR_SUB423_LOCK_TOKEN_SUBMITTED: u32 = 1;
pub const MD_ERROR_SUB423_NO_CONFLICTING_LOCK: u32 = 2;
pub const MD_ERROR_SUB500_ASPNET_HANDLERS: u32 = 23;
pub const MD_ERROR_SUB500_ASPNET_IMPERSONATION: u32 = 24;
pub const MD_ERROR_SUB500_ASPNET_MODULES: u32 = 22;
pub const MD_ERROR_SUB500_BAD_METADATA: u32 = 19;
pub const MD_ERROR_SUB500_HANDLERS_MODULE: u32 = 21;
pub const MD_ERROR_SUB500_UNC_ACCESS: u32 = 16;
pub const MD_ERROR_SUB500_URLAUTH_NO_SCOPE: u32 = 20;
pub const MD_ERROR_SUB500_URLAUTH_NO_STORE: u32 = 17;
pub const MD_ERROR_SUB500_URLAUTH_STORE_ERROR: u32 = 18;
pub const MD_ERROR_SUB502_ARR_CONNECTION_ERROR: u32 = 3;
pub const MD_ERROR_SUB502_ARR_NO_SERVER: u32 = 4;
pub const MD_ERROR_SUB502_PREMATURE_EXIT: u32 = 2;
pub const MD_ERROR_SUB502_TIMEOUT: u32 = 1;
pub const MD_ERROR_SUB503_APP_CONCURRENT: u32 = 2;
pub const MD_ERROR_SUB503_ASPNET_QUEUE_FULL: u32 = 3;
pub const MD_ERROR_SUB503_CONNECTION_LIMIT: u32 = 5;
pub const MD_ERROR_SUB503_CPU_LIMIT: u32 = 1;
pub const MD_ERROR_SUB503_FASTCGI_QUEUE_FULL: u32 = 4;
pub const MD_EXIT_MESSAGE: u32 = 5001;
pub const MD_EXPORT_INHERITED: u32 = 1;
pub const MD_EXPORT_NODE_ONLY: u32 = 2;
pub const MD_EXTLOG_BYTES_RECV: u32 = 8192;
pub const MD_EXTLOG_BYTES_SENT: u32 = 4096;
pub const MD_EXTLOG_CLIENT_IP: u32 = 4;
pub const MD_EXTLOG_COMPUTER_NAME: u32 = 32;
pub const MD_EXTLOG_COOKIE: u32 = 131072;
pub const MD_EXTLOG_DATE: u32 = 1;
pub const MD_EXTLOG_HOST: u32 = 1048576;
pub const MD_EXTLOG_HTTP_STATUS: u32 = 1024;
pub const MD_EXTLOG_HTTP_SUB_STATUS: u32 = 2097152;
pub const MD_EXTLOG_METHOD: u32 = 128;
pub const MD_EXTLOG_PROTOCOL_VERSION: u32 = 524288;
pub const MD_EXTLOG_REFERER: u32 = 262144;
pub const MD_EXTLOG_SERVER_IP: u32 = 64;
pub const MD_EXTLOG_SERVER_PORT: u32 = 32768;
pub const MD_EXTLOG_SITE_NAME: u32 = 16;
pub const MD_EXTLOG_TIME: u32 = 2;
pub const MD_EXTLOG_TIME_TAKEN: u32 = 16384;
pub const MD_EXTLOG_URI_QUERY: u32 = 512;
pub const MD_EXTLOG_URI_STEM: u32 = 256;
pub const MD_EXTLOG_USERNAME: u32 = 8;
pub const MD_EXTLOG_USER_AGENT: u32 = 65536;
pub const MD_EXTLOG_WIN32_STATUS: u32 = 2048;
pub const MD_FILTER_DESCRIPTION: u32 = 2045;
pub const MD_FILTER_ENABLED: u32 = 2043;
pub const MD_FILTER_ENABLE_CACHE: u32 = 2046;
pub const MD_FILTER_FLAGS: u32 = 2044;
pub const MD_FILTER_IMAGE_PATH: u32 = 2041;
pub const MD_FILTER_LOAD_ORDER: u32 = 2040;
pub const MD_FILTER_STATE: u32 = 2042;
pub const MD_FILTER_STATE_LOADED: u32 = 1;
pub const MD_FILTER_STATE_UNLOADED: u32 = 4;
pub const MD_FOOTER_DOCUMENT: u32 = 6009;
pub const MD_FOOTER_ENABLED: u32 = 6010;
pub const MD_FRONTPAGE_WEB: u32 = 2072;
pub const MD_FTPS_128_BITS: u32 = 5053;
pub const MD_FTPS_ALLOW_CCC: u32 = 5054;
pub const MD_FTPS_SECURE_ANONYMOUS: u32 = 5052;
pub const MD_FTPS_SECURE_CONTROL_CHANNEL: u32 = 5050;
pub const MD_FTPS_SECURE_DATA_CHANNEL: u32 = 5051;
pub const MD_FTP_KEEP_PARTIAL_UPLOADS: u32 = 5019;
pub const MD_FTP_LOG_IN_UTF_8: u32 = 5013;
pub const MD_FTP_PASV_RESPONSE_IP: u32 = 5018;
pub const MD_FTP_UTF8_FILE_NAMES: u32 = 5020;
pub const MD_GLOBAL_BINARY_LOGGING_ENABLED: u32 = 4016;
pub const MD_GLOBAL_BINSCHEMATIMESTAMP: u32 = 9991;
pub const MD_GLOBAL_CHANGE_NUMBER: u32 = 9997;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MAJOR_VERSION_NUMBER: u32 = 9994;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MINOR_VERSION_NUMBER: u32 = 9993;
pub const MD_GLOBAL_LOG_IN_UTF_8: u32 = 9206;
pub const MD_GLOBAL_SESSIONKEY: u32 = 9999;
pub const MD_GLOBAL_STANDARD_APP_MODE_ENABLED: u32 = 9203;
pub const MD_GLOBAL_XMLSCHEMATIMESTAMP: u32 = 9992;
pub const MD_GREETING_MESSAGE: u32 = 5002;
pub const MD_HC_CACHE_CONTROL_HEADER: u32 = 2211;
pub const MD_HC_COMPRESSION_BUFFER_SIZE: u32 = 2223;
pub const MD_HC_COMPRESSION_DIRECTORY: u32 = 2210;
pub const MD_HC_COMPRESSION_DLL: u32 = 2237;
pub const MD_HC_CREATE_FLAGS: u32 = 2243;
pub const MD_HC_DO_DISK_SPACE_LIMITING: u32 = 2216;
pub const MD_HC_DO_DYNAMIC_COMPRESSION: u32 = 2213;
pub const MD_HC_DO_NAMESPACE_DYNAMIC_COMPRESSION: u32 = 2255;
pub const MD_HC_DO_NAMESPACE_STATIC_COMPRESSION: u32 = 2256;
pub const MD_HC_DO_ON_DEMAND_COMPRESSION: u32 = 2215;
pub const MD_HC_DO_STATIC_COMPRESSION: u32 = 2214;
pub const MD_HC_DYNAMIC_COMPRESSION_LEVEL: u32 = 2241;
pub const MD_HC_EXPIRES_HEADER: u32 = 2212;
pub const MD_HC_FILES_DELETED_PER_DISK_FREE: u32 = 2225;
pub const MD_HC_FILE_EXTENSIONS: u32 = 2238;
pub const MD_HC_IO_BUFFER_SIZE: u32 = 2222;
pub const MD_HC_MAX_DISK_SPACE_USAGE: u32 = 2221;
pub const MD_HC_MAX_QUEUE_LENGTH: u32 = 2224;
pub const MD_HC_MIME_TYPE: u32 = 2239;
pub const MD_HC_MIN_FILE_SIZE_FOR_COMP: u32 = 2226;
pub const MD_HC_NO_COMPRESSION_FOR_HTTP_10: u32 = 2217;
pub const MD_HC_NO_COMPRESSION_FOR_PROXIES: u32 = 2218;
pub const MD_HC_NO_COMPRESSION_FOR_RANGE: u32 = 2219;
pub const MD_HC_ON_DEMAND_COMP_LEVEL: u32 = 2242;
pub const MD_HC_PRIORITY: u32 = 2240;
pub const MD_HC_SCRIPT_FILE_EXTENSIONS: u32 = 2244;
pub const MD_HC_SEND_CACHE_HEADERS: u32 = 2220;
pub const MD_HEADER_WAIT_TIMEOUT: u32 = 9204;
pub const MD_HISTORY_LATEST: u32 = 1;
pub const MD_HTTPERRORS_EXISTING_RESPONSE: u32 = 6056;
pub const MD_HTTP_CUSTOM: u32 = 6004;
pub const MD_HTTP_EXPIRES: u32 = 6002;
pub const MD_HTTP_FORWARDER_CUSTOM: u32 = 6054;
pub const MD_HTTP_PICS: u32 = 6003;
pub const MD_HTTP_REDIRECT: u32 = 6011;
pub const MD_IISADMIN_EXTENSIONS: u32 = 1028;
pub const MD_IMPORT_INHERITED: u32 = 1;
pub const MD_IMPORT_MERGE: u32 = 4;
pub const MD_IMPORT_NODE_ONLY: u32 = 2;
pub const MD_INSERT_PATH_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("<%INSERT_PATH%>");
pub const MD_INSERT_PATH_STRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("<%INSERT_PATH%>");
pub const MD_IN_PROCESS_ISAPI_APPS: u32 = 2073;
pub const MD_IP_SEC: u32 = 6019;
pub const MD_ISAPI_RESTRICTION_LIST: u32 = 2163;
pub const MD_IS_CONTENT_INDEXED: u32 = 6039;
pub const MD_KEY_TYPE: u32 = 1002;
pub const MD_LEVELS_TO_SCAN: u32 = 1022;
pub const MD_LOAD_BALANCER_CAPABILITIES: u32 = 9034;
pub const MD_LOAD_BALANCER_CAPABILITIES_BASIC: u32 = 1;
pub const MD_LOAD_BALANCER_CAPABILITIES_SOPHISTICATED: u32 = 2;
pub const MD_LOCATION: u32 = 9989;
pub const MD_LOGCUSTOM_DATATYPE_DOUBLE: u32 = 5;
pub const MD_LOGCUSTOM_DATATYPE_FLOAT: u32 = 4;
pub const MD_LOGCUSTOM_DATATYPE_INT: u32 = 0;
pub const MD_LOGCUSTOM_DATATYPE_LONG: u32 = 2;
pub const MD_LOGCUSTOM_DATATYPE_LPSTR: u32 = 6;
pub const MD_LOGCUSTOM_DATATYPE_LPWSTR: u32 = 7;
pub const MD_LOGCUSTOM_DATATYPE_UINT: u32 = 1;
pub const MD_LOGCUSTOM_DATATYPE_ULONG: u32 = 3;
pub const MD_LOGCUSTOM_PROPERTY_DATATYPE: u32 = 4505;
pub const MD_LOGCUSTOM_PROPERTY_HEADER: u32 = 4502;
pub const MD_LOGCUSTOM_PROPERTY_ID: u32 = 4503;
pub const MD_LOGCUSTOM_PROPERTY_MASK: u32 = 4504;
pub const MD_LOGCUSTOM_PROPERTY_NAME: u32 = 4501;
pub const MD_LOGCUSTOM_PROPERTY_NODE_ID: u32 = 4508;
pub const MD_LOGCUSTOM_SERVICES_STRING: u32 = 4506;
pub const MD_LOGEXT_FIELD_MASK: u32 = 4013;
pub const MD_LOGEXT_FIELD_MASK2: u32 = 4014;
pub const MD_LOGFILE_DIRECTORY: u32 = 4001;
pub const MD_LOGFILE_LOCALTIME_ROLLOVER: u32 = 4015;
pub const MD_LOGFILE_PERIOD: u32 = 4003;
pub const MD_LOGFILE_PERIOD_DAILY: u32 = 1;
pub const MD_LOGFILE_PERIOD_HOURLY: u32 = 4;
pub const MD_LOGFILE_PERIOD_MAXSIZE: u32 = 0;
pub const MD_LOGFILE_PERIOD_MONTHLY: u32 = 3;
pub const MD_LOGFILE_PERIOD_NONE: u32 = 0;
pub const MD_LOGFILE_PERIOD_WEEKLY: u32 = 2;
pub const MD_LOGFILE_TRUNCATE_SIZE: u32 = 4004;
pub const MD_LOGON_BATCH: u32 = 1;
pub const MD_LOGON_INTERACTIVE: u32 = 0;
pub const MD_LOGON_METHOD: u32 = 6013;
pub const MD_LOGON_NETWORK: u32 = 2;
pub const MD_LOGON_NETWORK_CLEARTEXT: u32 = 3;
pub const MD_LOGSQL_DATA_SOURCES: u32 = 4007;
pub const MD_LOGSQL_PASSWORD: u32 = 4010;
pub const MD_LOGSQL_TABLE_NAME: u32 = 4008;
pub const MD_LOGSQL_USER_NAME: u32 = 4009;
pub const MD_LOG_ANONYMOUS: u32 = 5007;
pub const MD_LOG_NONANONYMOUS: u32 = 5008;
pub const MD_LOG_PLUGINS_AVAILABLE: u32 = 4012;
pub const MD_LOG_PLUGIN_MOD_ID: u32 = 4005;
pub const MD_LOG_PLUGIN_ORDER: u32 = 4011;
pub const MD_LOG_PLUGIN_UI_ID: u32 = 4006;
pub const MD_LOG_TYPE: u32 = 4000;
pub const MD_LOG_TYPE_DISABLED: u32 = 0;
pub const MD_LOG_TYPE_ENABLED: u32 = 1;
pub const MD_LOG_UNUSED1: u32 = 4002;
pub const MD_MAX_BANDWIDTH: u32 = 1000;
pub const MD_MAX_BANDWIDTH_BLOCKED: u32 = 1003;
pub const MD_MAX_CHANGE_ENTRIES: u32 = 100;
pub const MD_MAX_CLIENTS_MESSAGE: u32 = 5003;
pub const MD_MAX_CONNECTIONS: u32 = 1014;
pub const MD_MAX_ENDPOINT_CONNECTIONS: u32 = 1024;
pub const MD_MAX_ERROR_FILES: u32 = 9988;
pub const MD_MAX_GLOBAL_BANDWIDTH: u32 = 9201;
pub const MD_MAX_GLOBAL_CONNECTIONS: u32 = 9202;
pub const MD_MAX_REQUEST_ENTITY_ALLOWED: u32 = 6051;
pub const MD_MD_SERVER_SS_AUTH_MAPPING: u32 = 2200;
pub const MD_METADATA_ID_REGISTRATION: u32 = 1030;
pub const MD_MIME_MAP: u32 = 6015;
pub const MD_MIN_FILE_BYTES_PER_SEC: u32 = 9205;
pub const MD_MSDOS_DIR_OUTPUT: u32 = 5004;
pub const MD_NETLOGON_WKS_DNS: u32 = 2;
pub const MD_NETLOGON_WKS_IP: u32 = 1;
pub const MD_NETLOGON_WKS_NONE: u32 = 0;
pub const MD_NET_LOGON_WKS: u32 = 2065;
pub const MD_NOTIFEXAUTH_NTLMSSL: u32 = 1;
pub const MD_NOTIFY_ACCESS_DENIED: u32 = 2048;
pub const MD_NOTIFY_AUTHENTICATION: u32 = 8192;
pub const MD_NOTIFY_AUTH_COMPLETE: u32 = 67108864;
pub const MD_NOTIFY_END_OF_NET_SESSION: u32 = 256;
pub const MD_NOTIFY_END_OF_REQUEST: u32 = 128;
pub const MD_NOTIFY_LOG: u32 = 512;
pub const MD_NOTIFY_NONSECURE_PORT: u32 = 2;
pub const MD_NOTIFY_ORDER_DEFAULT: u32 = 131072;
pub const MD_NOTIFY_ORDER_HIGH: u32 = 524288;
pub const MD_NOTIFY_ORDER_LOW: u32 = 131072;
pub const MD_NOTIFY_ORDER_MEDIUM: u32 = 262144;
pub const MD_NOTIFY_PREPROC_HEADERS: u32 = 16384;
pub const MD_NOTIFY_READ_RAW_DATA: u32 = 32768;
pub const MD_NOTIFY_SECURE_PORT: u32 = 1;
pub const MD_NOTIFY_SEND_RAW_DATA: u32 = 1024;
pub const MD_NOTIFY_SEND_RESPONSE: u32 = 64;
pub const MD_NOTIFY_URL_MAP: u32 = 4096;
pub const MD_NOT_DELETABLE: u32 = 2116;
pub const MD_NTAUTHENTICATION_PROVIDERS: u32 = 6032;
pub const MD_PASSIVE_PORT_RANGE: u32 = 5016;
pub const MD_PASSPORT_NEED_MAPPING: u32 = 2;
pub const MD_PASSPORT_NO_MAPPING: u32 = 0;
pub const MD_PASSPORT_REQUIRE_AD_MAPPING: u32 = 6052;
pub const MD_PASSPORT_TRY_MAPPING: u32 = 1;
pub const MD_POOL_IDC_TIMEOUT: u32 = 6037;
pub const MD_PROCESS_NTCR_IF_LOGGED_ON: u32 = 2070;
pub const MD_PUT_READ_SIZE: u32 = 6046;
pub const MD_RAPID_FAIL_PROTECTION_INTERVAL: u32 = 9029;
pub const MD_RAPID_FAIL_PROTECTION_MAX_CRASHES: u32 = 9030;
pub const MD_REALM: u32 = 6001;
pub const MD_REDIRECT_HEADERS: u32 = 6044;
pub const MD_RESTRICTION_LIST_CUSTOM_DESC: u32 = 2165;
pub const MD_ROOT_ENABLE_EDIT_WHILE_RUNNING: u32 = 9998;
pub const MD_ROOT_ENABLE_HISTORY: u32 = 9996;
pub const MD_ROOT_MAX_HISTORY_FILES: u32 = 9995;
pub const MD_SCHEMA_METAID: u32 = 1004;
pub const MD_SCRIPTMAPFLAG_ALLOWED_ON_READ_DIR: u32 = 1;
pub const MD_SCRIPTMAPFLAG_CHECK_PATH_INFO: u32 = 4;
pub const MD_SCRIPTMAPFLAG_SCRIPT: u32 = 1;
pub const MD_SCRIPT_MAPS: u32 = 6014;
pub const MD_SCRIPT_TIMEOUT: u32 = 6033;
pub const MD_SECURE_BINDINGS: u32 = 2021;
pub const MD_SECURITY_SETUP_REQUIRED: u32 = 2166;
pub const MD_SERVER_AUTOSTART: u32 = 1017;
pub const MD_SERVER_BINDINGS: u32 = 1023;
pub const MD_SERVER_COMMAND: u32 = 1012;
pub const MD_SERVER_COMMAND_CONTINUE: u32 = 4;
pub const MD_SERVER_COMMAND_PAUSE: u32 = 3;
pub const MD_SERVER_COMMAND_START: u32 = 1;
pub const MD_SERVER_COMMAND_STOP: u32 = 2;
pub const MD_SERVER_COMMENT: u32 = 1015;
pub const MD_SERVER_CONFIGURATION_INFO: u32 = 1027;
pub const MD_SERVER_CONFIG_ALLOW_ENCRYPT: u32 = 4;
pub const MD_SERVER_CONFIG_AUTO_PW_SYNC: u32 = 8;
pub const MD_SERVER_CONFIG_SSL_128: u32 = 2;
pub const MD_SERVER_CONFIG_SSL_40: u32 = 1;
pub const MD_SERVER_LISTEN_BACKLOG: u32 = 1019;
pub const MD_SERVER_LISTEN_TIMEOUT: u32 = 1020;
pub const MD_SERVER_SIZE: u32 = 1018;
pub const MD_SERVER_SIZE_LARGE: u32 = 2;
pub const MD_SERVER_SIZE_MEDIUM: u32 = 1;
pub const MD_SERVER_SIZE_SMALL: u32 = 0;
pub const MD_SERVER_STATE: u32 = 1016;
pub const MD_SERVER_STATE_CONTINUING: u32 = 7;
pub const MD_SERVER_STATE_PAUSED: u32 = 6;
pub const MD_SERVER_STATE_PAUSING: u32 = 5;
pub const MD_SERVER_STATE_STARTED: u32 = 2;
pub const MD_SERVER_STATE_STARTING: u32 = 1;
pub const MD_SERVER_STATE_STOPPED: u32 = 4;
pub const MD_SERVER_STATE_STOPPING: u32 = 3;
pub const MD_SET_HOST_NAME: u32 = 2154;
pub const MD_SHOW_4_DIGIT_YEAR: u32 = 5010;
pub const MD_SSI_EXEC_DISABLED: u32 = 6028;
pub const MD_SSL_ACCESS_PERM: u32 = 6030;
pub const MD_SSL_ALWAYS_NEGO_CLIENT_CERT: u32 = 5521;
pub const MD_SSL_KEY_PASSWORD: u32 = 5502;
pub const MD_SSL_KEY_REQUEST: u32 = 5503;
pub const MD_SSL_PRIVATE_KEY: u32 = 5501;
pub const MD_SSL_PUBLIC_KEY: u32 = 5500;
pub const MD_SSL_USE_DS_MAPPER: u32 = 5519;
pub const MD_STOP_LISTENING: u32 = 9987;
pub const MD_SUPPRESS_DEFAULT_BANNER: u32 = 5017;
pub const MD_UPLOAD_READAHEAD_SIZE: u32 = 6045;
pub const MD_URL_AUTHORIZATION_IMPERSONATION_LEVEL: u32 = 6053;
pub const MD_URL_AUTHORIZATION_SCOPE_NAME: u32 = 6050;
pub const MD_URL_AUTHORIZATION_STORE_NAME: u32 = 6049;
pub const MD_USER_ISOLATION: u32 = 5012;
pub const MD_USER_ISOLATION_AD: u32 = 2;
pub const MD_USER_ISOLATION_BASIC: u32 = 1;
pub const MD_USER_ISOLATION_LAST: u32 = 2;
pub const MD_USER_ISOLATION_NONE: u32 = 0;
pub const MD_USE_DIGEST_SSP: u32 = 6047;
pub const MD_USE_HOST_NAME: u32 = 2066;
pub const MD_VR_IGNORE_TRANSLATE: u32 = 3008;
pub const MD_VR_NO_CACHE: u32 = 3007;
pub const MD_VR_PASSTHROUGH: u32 = 3006;
pub const MD_VR_PASSWORD: u32 = 3003;
pub const MD_VR_PATH: u32 = 3001;
pub const MD_VR_USERNAME: u32 = 3002;
pub const MD_WAM_PWD: u32 = 7502;
pub const MD_WAM_USER_NAME: u32 = 7501;
pub const MD_WARNING_DUP_NAME: i32 = 837636;
pub const MD_WARNING_INVALID_DATA: i32 = 837637;
pub const MD_WARNING_PATH_NOT_FOUND: i32 = 837635;
pub const MD_WARNING_PATH_NOT_INSERTED: i32 = 837639;
pub const MD_WARNING_SAVE_FAILED: i32 = 837641;
pub const MD_WEBDAV_MAX_ATTRIBUTES_PER_ELEMENT: u32 = 8501;
pub const MD_WEB_SVC_EXT_RESTRICTION_LIST: u32 = 2168;
pub const MD_WIN32_ERROR: u32 = 1099;
pub type METADATATYPES = i32;
pub const METADATA_DONT_EXPAND: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct METADATA_GETALL_INTERNAL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub Anonymous: METADATA_GETALL_INTERNAL_RECORD_0,
    pub dwMDDataTag: u32,
}
impl Default for METADATA_GETALL_INTERNAL_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union METADATA_GETALL_INTERNAL_RECORD_0 {
    pub dwMDDataOffset: usize,
    pub pbMDData: *mut u8,
}
impl Default for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct METADATA_GETALL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub dwMDDataOffset: u32,
    pub dwMDDataTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct METADATA_HANDLE_INFO {
    pub dwMDPermissions: u32,
    pub dwMDSystemChangeNumber: u32,
}
pub const METADATA_INHERIT: u32 = 1;
pub const METADATA_INSERT_PATH: u32 = 64;
pub const METADATA_ISINHERITED: u32 = 32;
pub const METADATA_LOCAL_MACHINE_ONLY: u32 = 128;
pub const METADATA_MASTER_ROOT_HANDLE: u32 = 0;
pub const METADATA_MAX_NAME_LEN: u32 = 256;
pub const METADATA_NON_SECURE_ONLY: u32 = 256;
pub const METADATA_NO_ATTRIBUTES: u32 = 0;
pub const METADATA_PARTIAL_PATH: u32 = 2;
pub const METADATA_PERMISSION_READ: u32 = 1;
pub const METADATA_PERMISSION_WRITE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct METADATA_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub pbMDData: *mut u8,
    pub dwMDDataTag: u32,
}
impl Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const METADATA_REFERENCE: u32 = 8;
pub const METADATA_SECURE: u32 = 4;
pub const METADATA_VOLATILE: u32 = 16;
pub const MSCS_MD_ID_BEGIN_RESERVED: u32 = 53248;
pub const MSCS_MD_ID_END_RESERVED: u32 = 57343;
pub const MULTISZ_METADATA: METADATATYPES = 5;
pub const NNTP_MD_ID_BEGIN_RESERVED: u32 = 45056;
pub const NNTP_MD_ID_END_RESERVED: u32 = 49151;
pub type PFN_GETEXTENSIONVERSION = Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> windows_sys::core::BOOL>;
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = Option<unsafe extern "system" fn(pszurl: windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT>;
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = Option<unsafe extern "system" fn(pszprotocolmanagerdll: windows_sys::core::PCWSTR, pszprotocolmanagerdllinitfunction: windows_sys::core::PCWSTR, dwcustominterfaceid: u32, ppcustominterface: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type PFN_HSE_IO_COMPLETION = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: *mut core::ffi::c_void, cbio: u32, dwerror: u32)>;
pub type PFN_HTTPEXTENSIONPROC = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> u32>;
pub type PFN_IIS_GETSERVERVARIABLE = Option<unsafe extern "system" fn(param0: HCONN, param1: windows_sys::core::PCSTR, param2: *mut core::ffi::c_void, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_IIS_READCLIENT = Option<unsafe extern "system" fn(param0: HCONN, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_IIS_SERVERSUPPORTFUNCTION = Option<unsafe extern "system" fn(param0: HCONN, param1: u32, param2: *mut core::ffi::c_void, param3: *mut u32, param4: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_IIS_WRITECLIENT = Option<unsafe extern "system" fn(param0: HCONN, param1: *mut core::ffi::c_void, param2: *mut u32, param3: u32) -> windows_sys::core::BOOL>;
pub type PFN_TERMINATEEXTENSION = Option<unsafe extern "system" fn(dwflags: u32) -> windows_sys::core::BOOL>;
pub type PFN_WEB_CORE_ACTIVATE = Option<unsafe extern "system" fn(pszapphostconfigfile: windows_sys::core::PCWSTR, pszrootwebconfigfile: windows_sys::core::PCWSTR, pszinstancename: windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT>;
pub type PFN_WEB_CORE_SET_METADATA_DLL_ENTRY = Option<unsafe extern "system" fn(pszmetadatatype: windows_sys::core::PCWSTR, pszvalue: windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT>;
pub type PFN_WEB_CORE_SHUTDOWN = Option<unsafe extern "system" fn(fimmediate: u32) -> windows_sys::core::HRESULT>;
pub const POP3_MD_ID_BEGIN_RESERVED: u32 = 40960;
pub const POP3_MD_ID_END_RESERVED: u32 = 45055;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POST_PROCESS_PARAMETERS {
    pub pszSessionId: windows_sys::core::PCWSTR,
    pub pszSiteName: windows_sys::core::PCWSTR,
    pub pszUserName: windows_sys::core::PCWSTR,
    pub pszHostName: windows_sys::core::PCWSTR,
    pub pszRemoteIpAddress: windows_sys::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_sys::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: windows_sys::core::PCWSTR,
    pub pszCommandParameters: windows_sys::core::PCWSTR,
    pub pszFullPath: windows_sys::core::PCWSTR,
    pub pszPhysicalPath: windows_sys::core::PCWSTR,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: windows_sys::core::HRESULT,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
impl Default for POST_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRE_PROCESS_PARAMETERS {
    pub pszSessionId: windows_sys::core::PCWSTR,
    pub pszSiteName: windows_sys::core::PCWSTR,
    pub pszUserName: windows_sys::core::PCWSTR,
    pub pszHostName: windows_sys::core::PCWSTR,
    pub pszRemoteIpAddress: windows_sys::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_sys::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub pszCommand: windows_sys::core::PCWSTR,
    pub pszCommandParameters: windows_sys::core::PCWSTR,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
impl Default for PRE_PROCESS_PARAMETERS {
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
pub const SMTP_MD_ID_BEGIN_RESERVED: u32 = 36864;
pub const SMTP_MD_ID_END_RESERVED: u32 = 40959;
pub const STRING_METADATA: METADATATYPES = 2;
pub const USER_MD_ID_BASE_RESERVED: u32 = 65535;
pub const WAM_MD_ID_BEGIN_RESERVED: u32 = 29952;
pub const WAM_MD_ID_END_RESERVED: u32 = 32767;
pub const WAM_MD_SERVER_BASE: u32 = 7500;
pub const WEBDAV_MD_SERVER_BASE: u32 = 8500;
pub const WEB_CORE_ACTIVATE_DLL_ENTRY: windows_sys::core::PCSTR = windows_sys::core::s!("WebCoreActivate");
pub const WEB_CORE_DLL_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("hwebcore.dll");
pub const WEB_CORE_SET_METADATA_DLL_ENTRY: windows_sys::core::PCSTR = windows_sys::core::s!("WebCoreSetMetadata");
pub const WEB_CORE_SHUTDOWN_DLL_ENTRY: windows_sys::core::PCSTR = windows_sys::core::s!("WebCoreShutdown");
