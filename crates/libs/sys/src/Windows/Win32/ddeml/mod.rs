windows_link::link!("user32.dll" "system" fn DdeAbandonTransaction(idinst : u32, hconv : HCONV, idtransaction : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn DdeAccessData(hdata : HDDEDATA, pcbdatasize : *mut u32) -> super::LPBYTE);
windows_link::link!("user32.dll" "system" fn DdeAddData(hdata : HDDEDATA, psrc : *const u8, cb : u32, cboff : u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeClientTransaction(pdata : *const u8, cbdata : u32, hconv : HCONV, hszitem : HSZ, wfmt : u32, wtype : u32, dwtimeout : u32, pdwresult : *mut u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeCmpStringHandles(hsz1 : HSZ, hsz2 : HSZ) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn DdeConnect(idinst : u32, hszservice : HSZ, hsztopic : HSZ, pcc : *const CONVCONTEXT) -> HCONV);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn DdeConnectList(idinst : u32, hszservice : HSZ, hsztopic : HSZ, hconvlist : HCONVLIST, pcc : *const CONVCONTEXT) -> HCONVLIST);
windows_link::link!("user32.dll" "system" fn DdeCreateDataHandle(idinst : u32, psrc : *const u8, cb : u32, cboff : u32, hszitem : HSZ, wfmt : u32, afcmd : u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeCreateStringHandleA(idinst : u32, psz : windows_sys::core::PCSTR, icodepage : i32) -> HSZ);
windows_link::link!("user32.dll" "system" fn DdeCreateStringHandleW(idinst : u32, psz : windows_sys::core::PCWSTR, icodepage : i32) -> HSZ);
windows_link::link!("user32.dll" "system" fn DdeDisconnect(hconv : HCONV) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeDisconnectList(hconvlist : HCONVLIST) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeEnableCallback(idinst : u32, hconv : HCONV, wcmd : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeFreeDataHandle(hdata : HDDEDATA) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeFreeStringHandle(idinst : u32, hsz : HSZ) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeGetData(hdata : HDDEDATA, pdst : *mut u8, cbmax : u32, cboff : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeGetLastError(idinst : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeImpersonateClient(hconv : HCONV) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeInitializeA(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : u32, ulres : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeInitializeW(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : u32, ulres : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeKeepStringHandle(idinst : u32, hsz : HSZ) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeNameService(idinst : u32, hsz1 : HSZ, hsz2 : HSZ, afcmd : u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdePostAdvise(idinst : u32, hsztopic : HSZ, hszitem : HSZ) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DdeQueryConvInfo(hconv : HCONV, idtransaction : u32, pconvinfo : *mut CONVINFO) -> u32);
windows_link::link!("user32.dll" "system" fn DdeQueryNextServer(hconvlist : HCONVLIST, hconvprev : HCONV) -> HCONV);
windows_link::link!("user32.dll" "system" fn DdeQueryStringA(idinst : u32, hsz : HSZ, psz : windows_sys::core::PSTR, cchmax : u32, icodepage : i32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeQueryStringW(idinst : u32, hsz : HSZ, psz : windows_sys::core::PWSTR, cchmax : u32, icodepage : i32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeReconnect(hconv : HCONV) -> HCONV);
windows_link::link!("user32.dll" "system" fn DdeSetUserHandle(hconv : HCONV, id : u32, huser : usize) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeUnaccessData(hdata : HDDEDATA) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeUninitialize(idinst : u32) -> windows_sys::core::BOOL);
pub const APPCLASS_MASK: i32 = 15;
pub const APPCLASS_MONITOR: i32 = 1;
pub const APPCLASS_STANDARD: i32 = 0;
pub const APPCMD_CLIENTONLY: i32 = 16;
pub const APPCMD_FILTERINITS: i32 = 32;
pub const APPCMD_MASK: i32 = 4080;
pub const CADV_LATEACK: i32 = 65535;
pub const CBF_FAIL_ADVISES: i32 = 16384;
pub const CBF_FAIL_ALLSVRXACTIONS: i32 = 258048;
pub const CBF_FAIL_CONNECTIONS: i32 = 8192;
pub const CBF_FAIL_EXECUTES: i32 = 32768;
pub const CBF_FAIL_POKES: i32 = 65536;
pub const CBF_FAIL_REQUESTS: i32 = 131072;
pub const CBF_FAIL_SELFCONNECTIONS: i32 = 4096;
pub const CBF_SKIP_ALLNOTIFICATIONS: i32 = 3932160;
pub const CBF_SKIP_CONNECT_CONFIRMS: i32 = 262144;
pub const CBF_SKIP_DISCONNECTS: i32 = 2097152;
pub const CBF_SKIP_REGISTRATIONS: i32 = 524288;
pub const CBF_SKIP_UNREGISTRATIONS: i32 = 1048576;
pub const CBR_BLOCK: HDDEDATA = -1 as _;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::SECURITY_QUALITY_OF_SERVICE,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CONVINFO {
    pub cb: u32,
    pub hUser: usize,
    pub hConvPartner: HCONV,
    pub hszSvcPartner: HSZ,
    pub hszServiceReq: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub wType: u32,
    pub wStatus: u32,
    pub wConvst: u32,
    pub wLastError: u32,
    pub hConvList: HCONVLIST,
    pub ConvCtxt: CONVCONTEXT,
    pub hwnd: super::HWND,
    pub hwndPartner: super::HWND,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CONVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CP_WINANSI: i32 = 1004;
pub const CP_WINNEUTRAL: i32 = 1004;
pub const CP_WINUNICODE: i32 = 1200;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const DDE_FACK: i32 = 32768;
pub const DDE_FACKREQ: i32 = 32768;
pub const DDE_FACKRESERVED: i32 = -49408;
pub const DDE_FADVRESERVED: i32 = -49153;
pub const DDE_FAPPSTATUS: i32 = 255;
pub const DDE_FBUSY: i32 = 16384;
pub const DDE_FDATRESERVED: i32 = -45057;
pub const DDE_FDEFERUPD: i32 = 16384;
pub const DDE_FNOTPROCESSED: i32 = 0;
pub const DDE_FPOKRESERVED: i32 = -8193;
pub const DDE_FRELEASE: i32 = 8192;
pub const DDE_FREQUESTED: i32 = 4096;
pub const DMLERR_ADVACKTIMEOUT: i32 = 16384;
pub const DMLERR_BUSY: i32 = 16385;
pub const DMLERR_DATAACKTIMEOUT: i32 = 16386;
pub const DMLERR_DLL_NOT_INITIALIZED: i32 = 16387;
pub const DMLERR_DLL_USAGE: i32 = 16388;
pub const DMLERR_EXECACKTIMEOUT: i32 = 16389;
pub const DMLERR_FIRST: i32 = 16384;
pub const DMLERR_INVALIDPARAMETER: i32 = 16390;
pub const DMLERR_LAST: i32 = 16401;
pub const DMLERR_LOW_MEMORY: i32 = 16391;
pub const DMLERR_MEMORY_ERROR: i32 = 16392;
pub const DMLERR_NOTPROCESSED: i32 = 16393;
pub const DMLERR_NO_CONV_ESTABLISHED: i32 = 16394;
pub const DMLERR_NO_ERROR: i32 = 0;
pub const DMLERR_POKEACKTIMEOUT: i32 = 16395;
pub const DMLERR_POSTMSG_FAILED: i32 = 16396;
pub const DMLERR_REENTRANCY: i32 = 16397;
pub const DMLERR_SERVER_DIED: i32 = 16398;
pub const DMLERR_SYS_ERROR: i32 = 16399;
pub const DMLERR_UNADVACKTIMEOUT: i32 = 16400;
pub const DMLERR_UNFOUND_QUEUE_ID: i32 = 16401;
pub const DNS_FILTEROFF: i32 = 8;
pub const DNS_FILTERON: i32 = 4;
pub const DNS_REGISTER: i32 = 1;
pub const DNS_UNREGISTER: i32 = 2;
pub const EC_DISABLE: i32 = 8;
pub const EC_ENABLEALL: i32 = 0;
pub const EC_ENABLEONE: i32 = 128;
pub const EC_QUERYWAITING: i32 = 2;
pub type FNCALLBACK = Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;
pub type HCONV = *mut core::ffi::c_void;
pub type HCONVLIST = *mut core::ffi::c_void;
pub const HDATA_APPOWNED: i32 = 1;
pub type HDDEDATA = *mut core::ffi::c_void;
pub type HSZ = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HSZPAIR {
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
}
impl Default for HSZPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_MONITORS: i32 = 4;
pub const MF_CALLBACKS: i32 = 134217728;
pub const MF_CONV: i32 = 1073741824;
pub const MF_ERRORS: i32 = 268435456;
pub const MF_HSZ_INFO: i32 = 16777216;
pub const MF_LINKS: i32 = 536870912;
pub const MF_MASK: u32 = 4278190080;
pub const MF_POSTMSGS: i32 = 67108864;
pub const MF_SENDMSGS: i32 = 33554432;
pub const MH_CLEANUP: i32 = 4;
pub const MH_CREATE: i32 = 1;
pub const MH_DELETE: i32 = 3;
pub const MH_KEEP: i32 = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONCBSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::HANDLE,
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
#[cfg(feature = "winnt")]
impl Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: windows_sys::core::BOOL,
    pub dwTime: u32,
    pub hTask: super::HANDLE,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hConvClient: HCONV,
    pub hConvServer: HCONV,
}
#[cfg(feature = "winnt")]
impl Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONERRSTRUCT {
    pub cb: u32,
    pub wLastError: u32,
    pub dwTime: u32,
    pub hTask: super::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type MONHSZSTRUCT = MONHSZSTRUCTA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: windows_sys::core::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::HANDLE,
    pub str: [i8; 1],
}
#[cfg(feature = "winnt")]
impl Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: windows_sys::core::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::HANDLE,
    pub str: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::HANDLE,
    pub fEstablished: windows_sys::core::BOOL,
    pub fNoData: windows_sys::core::BOOL,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub fServer: windows_sys::core::BOOL,
    pub hConvServer: HCONV,
    pub hConvClient: HCONV,
}
#[cfg(feature = "winnt")]
impl Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MONMSGSTRUCT {
    pub cb: u32,
    pub hwndTo: super::HWND,
    pub dwTime: u32,
    pub hTask: super::HANDLE,
    pub wMsg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub dmhd: DDEML_MSG_HOOK_DATA,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSGF_DDEMGR: i32 = 32769;
#[cfg(feature = "winnt")]
pub type PCONVCONTEXT = *mut CONVCONTEXT;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PCONVINFO = *mut CONVINFO;
pub type PDDEML_MSG_HOOK_DATA = *mut DDEML_MSG_HOOK_DATA;
pub type PFNCALLBACK = Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;
pub type PHSZPAIR = *mut HSZPAIR;
#[cfg(feature = "winnt")]
pub type PMONCBSTRUCT = *mut MONCBSTRUCT;
#[cfg(feature = "winnt")]
pub type PMONCONVSTRUCT = *mut MONCONVSTRUCT;
#[cfg(feature = "winnt")]
pub type PMONERRSTRUCT = *mut MONERRSTRUCT;
#[cfg(feature = "winnt")]
pub type PMONHSZSTRUCT = PMONHSZSTRUCTA;
#[cfg(feature = "winnt")]
pub type PMONHSZSTRUCTA = *mut MONHSZSTRUCTA;
#[cfg(feature = "winnt")]
pub type PMONHSZSTRUCTW = *mut MONHSZSTRUCTW;
#[cfg(feature = "winnt")]
pub type PMONLINKSTRUCT = *mut MONLINKSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PMONMSGSTRUCT = *mut MONMSGSTRUCT;
pub const QID_SYNC: u32 = 4294967295;
pub const ST_ADVISE: i32 = 2;
pub const ST_BLOCKED: i32 = 8;
pub const ST_BLOCKNEXT: i32 = 128;
pub const ST_CLIENT: i32 = 16;
pub const ST_CONNECTED: i32 = 1;
pub const ST_INLIST: i32 = 64;
pub const ST_ISLOCAL: i32 = 4;
pub const ST_ISSELF: i32 = 256;
pub const ST_TERMINATED: i32 = 32;
pub const SZDDESYS_ITEM_FORMATS: windows_sys::core::PCSTR = windows_sys::core::s!("Formats");
pub const SZDDESYS_ITEM_HELP: windows_sys::core::PCSTR = windows_sys::core::s!("Help");
pub const SZDDESYS_ITEM_RTNMSG: windows_sys::core::PCSTR = windows_sys::core::s!("ReturnMessage");
pub const SZDDESYS_ITEM_STATUS: windows_sys::core::PCSTR = windows_sys::core::s!("Status");
pub const SZDDESYS_ITEM_SYSITEMS: windows_sys::core::PCSTR = windows_sys::core::s!("SysItems");
pub const SZDDESYS_ITEM_TOPICS: windows_sys::core::PCSTR = windows_sys::core::s!("Topics");
pub const SZDDESYS_TOPIC: windows_sys::core::PCSTR = windows_sys::core::s!("System");
pub const SZDDE_ITEM_ITEMLIST: windows_sys::core::PCSTR = windows_sys::core::s!("TopicItemList");
pub const TIMEOUT_ASYNC: u32 = 4294967295;
pub const XCLASS_BOOL: i32 = 4096;
pub const XCLASS_DATA: i32 = 8192;
pub const XCLASS_FLAGS: i32 = 16384;
pub const XCLASS_MASK: i32 = 64512;
pub const XCLASS_NOTIFICATION: i32 = 32768;
pub const XST_ADVACKRCVD: i32 = 13;
pub const XST_ADVDATAACKRCVD: i32 = 16;
pub const XST_ADVDATASENT: i32 = 15;
pub const XST_ADVSENT: i32 = 11;
pub const XST_CONNECTED: i32 = 2;
pub const XST_DATARCVD: i32 = 6;
pub const XST_EXECACKRCVD: i32 = 10;
pub const XST_EXECSENT: i32 = 9;
pub const XST_INCOMPLETE: i32 = 1;
pub const XST_INIT1: i32 = 3;
pub const XST_INIT2: i32 = 4;
pub const XST_NULL: i32 = 0;
pub const XST_POKEACKRCVD: i32 = 8;
pub const XST_POKESENT: i32 = 7;
pub const XST_REQSENT: i32 = 5;
pub const XST_UNADVACKRCVD: i32 = 14;
pub const XST_UNADVSENT: i32 = 12;
pub const XTYPF_ACKREQ: i32 = 8;
pub const XTYPF_NOBLOCK: i32 = 2;
pub const XTYPF_NODATA: i32 = 4;
pub const XTYP_ADVDATA: i32 = 16400;
pub const XTYP_ADVREQ: i32 = 8226;
pub const XTYP_ADVSTART: i32 = 4144;
pub const XTYP_ADVSTOP: i32 = 32832;
pub const XTYP_CONNECT: i32 = 4194;
pub const XTYP_CONNECT_CONFIRM: i32 = 32882;
pub const XTYP_DISCONNECT: i32 = 32962;
pub const XTYP_ERROR: i32 = 32770;
pub const XTYP_EXECUTE: i32 = 16464;
pub const XTYP_MASK: i32 = 240;
pub const XTYP_MONITOR: i32 = 33010;
pub const XTYP_POKE: i32 = 16528;
pub const XTYP_REGISTER: i32 = 32930;
pub const XTYP_REQUEST: i32 = 8368;
pub const XTYP_SHIFT: i32 = 4;
pub const XTYP_UNREGISTER: i32 = 32978;
pub const XTYP_WILDCONNECT: i32 = 8418;
pub const XTYP_XACT_COMPLETE: i32 = 32896;
