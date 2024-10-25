pub const APPCLASS_MASK: i32 = 15i32;
pub const APPCLASS_MONITOR: DDE_INITIALIZE_COMMAND = 1u32;
pub const APPCLASS_STANDARD: DDE_INITIALIZE_COMMAND = 0u32;
pub const APPCMD_CLIENTONLY: DDE_INITIALIZE_COMMAND = 16u32;
pub const APPCMD_FILTERINITS: DDE_INITIALIZE_COMMAND = 32u32;
pub const APPCMD_MASK: i32 = 4080i32;
pub const CADV_LATEACK: u32 = 65535u32;
pub const CBF_FAIL_ADVISES: DDE_INITIALIZE_COMMAND = 16384u32;
pub const CBF_FAIL_ALLSVRXACTIONS: DDE_INITIALIZE_COMMAND = 258048u32;
pub const CBF_FAIL_CONNECTIONS: DDE_INITIALIZE_COMMAND = 8192u32;
pub const CBF_FAIL_EXECUTES: DDE_INITIALIZE_COMMAND = 32768u32;
pub const CBF_FAIL_POKES: DDE_INITIALIZE_COMMAND = 65536u32;
pub const CBF_FAIL_REQUESTS: DDE_INITIALIZE_COMMAND = 131072u32;
pub const CBF_FAIL_SELFCONNECTIONS: DDE_INITIALIZE_COMMAND = 4096u32;
pub const CBF_SKIP_ALLNOTIFICATIONS: DDE_INITIALIZE_COMMAND = 3932160u32;
pub const CBF_SKIP_CONNECT_CONFIRMS: DDE_INITIALIZE_COMMAND = 262144u32;
pub const CBF_SKIP_DISCONNECTS: DDE_INITIALIZE_COMMAND = 2097152u32;
pub const CBF_SKIP_REGISTRATIONS: DDE_INITIALIZE_COMMAND = 524288u32;
pub const CBF_SKIP_UNREGISTRATIONS: DDE_INITIALIZE_COMMAND = 1048576u32;
pub const CP_WINANSI: i32 = 1004i32;
pub const CP_WINNEUTRAL: i32 = 1200i32;
pub const CP_WINUNICODE: i32 = 1200i32;
pub const DDE_FACK: u32 = 32768u32;
pub const DDE_FACKREQ: u32 = 32768u32;
pub const DDE_FAPPSTATUS: u32 = 255u32;
pub const DDE_FBUSY: u32 = 16384u32;
pub const DDE_FDEFERUPD: u32 = 16384u32;
pub const DDE_FNOTPROCESSED: u32 = 0u32;
pub const DDE_FRELEASE: u32 = 8192u32;
pub const DDE_FREQUESTED: u32 = 4096u32;
pub const DMLERR_ADVACKTIMEOUT: u32 = 16384u32;
pub const DMLERR_BUSY: u32 = 16385u32;
pub const DMLERR_DATAACKTIMEOUT: u32 = 16386u32;
pub const DMLERR_DLL_NOT_INITIALIZED: u32 = 16387u32;
pub const DMLERR_DLL_USAGE: u32 = 16388u32;
pub const DMLERR_EXECACKTIMEOUT: u32 = 16389u32;
pub const DMLERR_FIRST: u32 = 16384u32;
pub const DMLERR_INVALIDPARAMETER: u32 = 16390u32;
pub const DMLERR_LAST: u32 = 16401u32;
pub const DMLERR_LOW_MEMORY: u32 = 16391u32;
pub const DMLERR_MEMORY_ERROR: u32 = 16392u32;
pub const DMLERR_NOTPROCESSED: u32 = 16393u32;
pub const DMLERR_NO_CONV_ESTABLISHED: u32 = 16394u32;
pub const DMLERR_NO_ERROR: u32 = 0u32;
pub const DMLERR_POKEACKTIMEOUT: u32 = 16395u32;
pub const DMLERR_POSTMSG_FAILED: u32 = 16396u32;
pub const DMLERR_REENTRANCY: u32 = 16397u32;
pub const DMLERR_SERVER_DIED: u32 = 16398u32;
pub const DMLERR_SYS_ERROR: u32 = 16399u32;
pub const DMLERR_UNADVACKTIMEOUT: u32 = 16400u32;
pub const DMLERR_UNFOUND_QUEUE_ID: u32 = 16401u32;
pub const DNS_FILTEROFF: DDE_NAME_SERVICE_CMD = 8u32;
pub const DNS_FILTERON: DDE_NAME_SERVICE_CMD = 4u32;
pub const DNS_REGISTER: DDE_NAME_SERVICE_CMD = 1u32;
pub const DNS_UNREGISTER: DDE_NAME_SERVICE_CMD = 2u32;
pub const EC_DISABLE: DDE_ENABLE_CALLBACK_CMD = 8u32;
pub const EC_ENABLEALL: DDE_ENABLE_CALLBACK_CMD = 0u32;
pub const EC_ENABLEONE: DDE_ENABLE_CALLBACK_CMD = 128u32;
pub const EC_QUERYWAITING: DDE_ENABLE_CALLBACK_CMD = 2u32;
pub const HDATA_APPOWNED: u32 = 1u32;
pub const MAX_MONITORS: u32 = 4u32;
pub const MF_CALLBACKS: DDE_INITIALIZE_COMMAND = 134217728u32;
pub const MF_CONV: DDE_INITIALIZE_COMMAND = 1073741824u32;
pub const MF_ERRORS: DDE_INITIALIZE_COMMAND = 268435456u32;
pub const MF_HSZ_INFO: DDE_INITIALIZE_COMMAND = 16777216u32;
pub const MF_LINKS: DDE_INITIALIZE_COMMAND = 536870912u32;
pub const MF_MASK: u32 = 4278190080u32;
pub const MF_POSTMSGS: DDE_INITIALIZE_COMMAND = 67108864u32;
pub const MF_SENDMSGS: DDE_INITIALIZE_COMMAND = 33554432u32;
pub const MH_CLEANUP: u32 = 4u32;
pub const MH_CREATE: u32 = 1u32;
pub const MH_DELETE: u32 = 3u32;
pub const MH_KEEP: u32 = 2u32;
pub const MSGF_DDEMGR: u32 = 32769u32;
pub const QID_SYNC: u32 = 4294967295u32;
pub const ST_ADVISE: CONVINFO_STATUS = 2u32;
pub const ST_BLOCKED: CONVINFO_STATUS = 8u32;
pub const ST_BLOCKNEXT: CONVINFO_STATUS = 128u32;
pub const ST_CLIENT: CONVINFO_STATUS = 16u32;
pub const ST_CONNECTED: CONVINFO_STATUS = 1u32;
pub const ST_INLIST: CONVINFO_STATUS = 64u32;
pub const ST_ISLOCAL: CONVINFO_STATUS = 4u32;
pub const ST_ISSELF: CONVINFO_STATUS = 256u32;
pub const ST_TERMINATED: CONVINFO_STATUS = 32u32;
pub const SZDDESYS_ITEM_FORMATS: windows_core::PCWSTR = windows_core::w!("Formats");
pub const SZDDESYS_ITEM_HELP: windows_core::PCWSTR = windows_core::w!("Help");
pub const SZDDESYS_ITEM_RTNMSG: windows_core::PCWSTR = windows_core::w!("ReturnMessage");
pub const SZDDESYS_ITEM_STATUS: windows_core::PCWSTR = windows_core::w!("Status");
pub const SZDDESYS_ITEM_SYSITEMS: windows_core::PCWSTR = windows_core::w!("SysItems");
pub const SZDDESYS_ITEM_TOPICS: windows_core::PCWSTR = windows_core::w!("Topics");
pub const SZDDESYS_TOPIC: windows_core::PCWSTR = windows_core::w!("System");
pub const SZDDE_ITEM_ITEMLIST: windows_core::PCWSTR = windows_core::w!("TopicItemList");
pub const TIMEOUT_ASYNC: u32 = 4294967295u32;
pub const WM_DDE_ACK: u32 = 996u32;
pub const WM_DDE_ADVISE: u32 = 994u32;
pub const WM_DDE_DATA: u32 = 997u32;
pub const WM_DDE_EXECUTE: u32 = 1000u32;
pub const WM_DDE_FIRST: u32 = 992u32;
pub const WM_DDE_INITIATE: u32 = 992u32;
pub const WM_DDE_LAST: u32 = 1000u32;
pub const WM_DDE_POKE: u32 = 999u32;
pub const WM_DDE_REQUEST: u32 = 998u32;
pub const WM_DDE_TERMINATE: u32 = 993u32;
pub const WM_DDE_UNADVISE: u32 = 995u32;
pub const XCLASS_BOOL: u32 = 4096u32;
pub const XCLASS_DATA: u32 = 8192u32;
pub const XCLASS_FLAGS: u32 = 16384u32;
pub const XCLASS_MASK: u32 = 64512u32;
pub const XCLASS_NOTIFICATION: u32 = 32768u32;
pub const XST_ADVACKRCVD: CONVINFO_CONVERSATION_STATE = 13u32;
pub const XST_ADVDATAACKRCVD: CONVINFO_CONVERSATION_STATE = 16u32;
pub const XST_ADVDATASENT: CONVINFO_CONVERSATION_STATE = 15u32;
pub const XST_ADVSENT: CONVINFO_CONVERSATION_STATE = 11u32;
pub const XST_CONNECTED: CONVINFO_CONVERSATION_STATE = 2u32;
pub const XST_DATARCVD: CONVINFO_CONVERSATION_STATE = 6u32;
pub const XST_EXECACKRCVD: CONVINFO_CONVERSATION_STATE = 10u32;
pub const XST_EXECSENT: CONVINFO_CONVERSATION_STATE = 9u32;
pub const XST_INCOMPLETE: CONVINFO_CONVERSATION_STATE = 1u32;
pub const XST_INIT1: CONVINFO_CONVERSATION_STATE = 3u32;
pub const XST_INIT2: CONVINFO_CONVERSATION_STATE = 4u32;
pub const XST_NULL: CONVINFO_CONVERSATION_STATE = 0u32;
pub const XST_POKEACKRCVD: CONVINFO_CONVERSATION_STATE = 8u32;
pub const XST_POKESENT: CONVINFO_CONVERSATION_STATE = 7u32;
pub const XST_REQSENT: CONVINFO_CONVERSATION_STATE = 5u32;
pub const XST_UNADVACKRCVD: CONVINFO_CONVERSATION_STATE = 14u32;
pub const XST_UNADVSENT: CONVINFO_CONVERSATION_STATE = 12u32;
pub const XTYPF_ACKREQ: u32 = 8u32;
pub const XTYPF_NOBLOCK: u32 = 2u32;
pub const XTYPF_NODATA: u32 = 4u32;
pub const XTYP_ADVDATA: DDE_CLIENT_TRANSACTION_TYPE = 16400u32;
pub const XTYP_ADVREQ: DDE_CLIENT_TRANSACTION_TYPE = 8226u32;
pub const XTYP_ADVSTART: DDE_CLIENT_TRANSACTION_TYPE = 4144u32;
pub const XTYP_ADVSTOP: DDE_CLIENT_TRANSACTION_TYPE = 32832u32;
pub const XTYP_CONNECT: DDE_CLIENT_TRANSACTION_TYPE = 4194u32;
pub const XTYP_CONNECT_CONFIRM: DDE_CLIENT_TRANSACTION_TYPE = 32882u32;
pub const XTYP_DISCONNECT: DDE_CLIENT_TRANSACTION_TYPE = 32962u32;
pub const XTYP_EXECUTE: DDE_CLIENT_TRANSACTION_TYPE = 16464u32;
pub const XTYP_MASK: u32 = 240u32;
pub const XTYP_MONITOR: DDE_CLIENT_TRANSACTION_TYPE = 33010u32;
pub const XTYP_POKE: DDE_CLIENT_TRANSACTION_TYPE = 16528u32;
pub const XTYP_REGISTER: DDE_CLIENT_TRANSACTION_TYPE = 32930u32;
pub const XTYP_REQUEST: DDE_CLIENT_TRANSACTION_TYPE = 8368u32;
pub const XTYP_SHIFT: u32 = 4u32;
pub const XTYP_UNREGISTER: DDE_CLIENT_TRANSACTION_TYPE = 32978u32;
pub const XTYP_WILDCONNECT: DDE_CLIENT_TRANSACTION_TYPE = 8418u32;
pub const XTYP_XACT_COMPLETE: DDE_CLIENT_TRANSACTION_TYPE = 32896u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CONVINFO_CONVERSATION_STATE(pub u32);
impl windows_core::TypeKind for CONVINFO_CONVERSATION_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CONVINFO_STATUS(pub u32);
impl windows_core::TypeKind for CONVINFO_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DDE_CLIENT_TRANSACTION_TYPE(pub u32);
impl windows_core::TypeKind for DDE_CLIENT_TRANSACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DDE_ENABLE_CALLBACK_CMD(pub u32);
impl windows_core::TypeKind for DDE_ENABLE_CALLBACK_CMD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DDE_INITIALIZE_COMMAND(pub u32);
impl windows_core::TypeKind for DDE_INITIALIZE_COMMAND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DDE_NAME_SERVICE_CMD(pub u32);
impl windows_core::TypeKind for DDE_NAME_SERVICE_CMD {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::super::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(feature = "Win32_Security")]
impl Default for CONVCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CONVCONTEXT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONVINFO {
    pub cb: u32,
    pub hUser: usize,
    pub hConvPartner: HCONV,
    pub hszSvcPartner: HSZ,
    pub hszServiceReq: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub wType: DDE_CLIENT_TRANSACTION_TYPE,
    pub wStatus: CONVINFO_STATUS,
    pub wConvst: CONVINFO_CONVERSATION_STATE,
    pub wLastError: u32,
    pub hConvList: HCONVLIST,
    pub ConvCtxt: CONVCONTEXT,
    pub hwnd: super::super::Foundation::HWND,
    pub hwndPartner: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Security")]
impl Default for CONVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CONVINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COPYDATASTRUCT {
    pub dwData: usize,
    pub cbData: u32,
    pub lpData: *mut core::ffi::c_void,
}
impl Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COPYDATASTRUCT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl Default for DDEACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEACK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl Default for DDEADVISE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEADVISE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl Default for DDEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl Default for DDELN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDELN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEML_MSG_HOOK_DATA {
    pub uiLo: usize,
    pub uiHi: usize,
    pub cbData: u32,
    pub Data: [u32; 8],
}
impl Default for DDEML_MSG_HOOK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEML_MSG_HOOK_DATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEPOKE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl Default for DDEUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDEUP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSZPAIR {
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
}
impl Default for HSZPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HSZPAIR {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct METAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: super::super::Graphics::Gdi::HMETAFILE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for METAFILEPICT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONCBSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub dwRet: u32,
    pub wType: u32,
    pub wFmt: u32,
    pub hConv: HCONV,
    pub hsz1: HSZ,
    pub hsz2: HSZ,
    pub hData: HDDEDATA,
    pub dwData1: usize,
    pub dwData2: usize,
    pub cc: CONVCONTEXT,
    pub cbData: u32,
    pub Data: [u32; 8],
}
#[cfg(feature = "Win32_Security")]
impl Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for MONCBSTRUCT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hConvClient: HCONV,
    pub hConvServer: HCONV,
}
impl Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONCONVSTRUCT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONERRSTRUCT {
    pub cb: u32,
    pub wLastError: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
}
impl Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONERRSTRUCT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [i8; 1],
}
impl Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONHSZSTRUCTA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [u16; 1],
}
impl Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONHSZSTRUCTW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub fEstablished: super::super::Foundation::BOOL,
    pub fNoData: super::super::Foundation::BOOL,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub fServer: super::super::Foundation::BOOL,
    pub hConvServer: HCONV,
    pub hConvClient: HCONV,
}
impl Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONLINKSTRUCT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONMSGSTRUCT {
    pub cb: u32,
    pub hwndTo: super::super::Foundation::HWND,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub wMsg: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dmhd: DDEML_MSG_HOOK_DATA,
}
impl Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MONMSGSTRUCT {
    type TypeKind = windows_core::CloneType;
}
pub type PFNCALLBACK = Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;
