windows_link::link!("kernel32.dll" "system" fn AddAtomA(lpstring : windows_sys::core::PCSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn AddAtomW(lpstring : windows_sys::core::PCWSTR) -> u16);
windows_link::link!("user32.dll" "system" fn AddClipboardFormatListener(hwnd : super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ChangeClipboardChain(hwndremove : super::super::Foundation::HWND, hwndnewnext : super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CloseClipboard() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CountClipboardFormats() -> i32);
windows_link::link!("user32.dll" "system" fn DdeAbandonTransaction(idinst : u32, hconv : HCONV, idtransaction : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeAccessData(hdata : HDDEDATA, pcbdatasize : *mut u32) -> *mut u8);
windows_link::link!("user32.dll" "system" fn DdeAddData(hdata : HDDEDATA, psrc : *const u8, cb : u32, cboff : u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeClientTransaction(pdata : *const u8, cbdata : u32, hconv : HCONV, hszitem : HSZ, wfmt : u32, wtype : DDE_CLIENT_TRANSACTION_TYPE, dwtimeout : u32, pdwresult : *mut u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeCmpStringHandles(hsz1 : HSZ, hsz2 : HSZ) -> i32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("user32.dll" "system" fn DdeConnect(idinst : u32, hszservice : HSZ, hsztopic : HSZ, pcc : *const CONVCONTEXT) -> HCONV);
#[cfg(feature = "Win32_Security")]
windows_link::link!("user32.dll" "system" fn DdeConnectList(idinst : u32, hszservice : HSZ, hsztopic : HSZ, hconvlist : HCONVLIST, pcc : *const CONVCONTEXT) -> HCONVLIST);
windows_link::link!("user32.dll" "system" fn DdeCreateDataHandle(idinst : u32, psrc : *const u8, cb : u32, cboff : u32, hszitem : HSZ, wfmt : u32, afcmd : u32) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdeCreateStringHandleA(idinst : u32, psz : windows_sys::core::PCSTR, icodepage : i32) -> HSZ);
windows_link::link!("user32.dll" "system" fn DdeCreateStringHandleW(idinst : u32, psz : windows_sys::core::PCWSTR, icodepage : i32) -> HSZ);
windows_link::link!("user32.dll" "system" fn DdeDisconnect(hconv : HCONV) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeDisconnectList(hconvlist : HCONVLIST) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeEnableCallback(idinst : u32, hconv : HCONV, wcmd : DDE_ENABLE_CALLBACK_CMD) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeFreeDataHandle(hdata : HDDEDATA) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeFreeStringHandle(idinst : u32, hsz : HSZ) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeGetData(hdata : HDDEDATA, pdst : *mut u8, cbmax : u32, cboff : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeGetLastError(idinst : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeImpersonateClient(hconv : HCONV) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeInitializeA(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : DDE_INITIALIZE_COMMAND, ulres : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeInitializeW(pidinst : *mut u32, pfncallback : PFNCALLBACK, afcmd : DDE_INITIALIZE_COMMAND, ulres : u32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeKeepStringHandle(idinst : u32, hsz : HSZ) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeNameService(idinst : u32, hsz1 : HSZ, hsz2 : HSZ, afcmd : DDE_NAME_SERVICE_CMD) -> HDDEDATA);
windows_link::link!("user32.dll" "system" fn DdePostAdvise(idinst : u32, hsztopic : HSZ, hszitem : HSZ) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("user32.dll" "system" fn DdeQueryConvInfo(hconv : HCONV, idtransaction : u32, pconvinfo : *mut CONVINFO) -> u32);
windows_link::link!("user32.dll" "system" fn DdeQueryNextServer(hconvlist : HCONVLIST, hconvprev : HCONV) -> HCONV);
windows_link::link!("user32.dll" "system" fn DdeQueryStringA(idinst : u32, hsz : HSZ, psz : windows_sys::core::PSTR, cchmax : u32, icodepage : i32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeQueryStringW(idinst : u32, hsz : HSZ, psz : windows_sys::core::PWSTR, cchmax : u32, icodepage : i32) -> u32);
windows_link::link!("user32.dll" "system" fn DdeReconnect(hconv : HCONV) -> HCONV);
#[cfg(feature = "Win32_Security")]
windows_link::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::super::Foundation::HWND, pqosnew : *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev : *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeSetUserHandle(hconv : HCONV, id : u32, huser : usize) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeUnaccessData(hdata : HDDEDATA) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DdeUninitialize(idinst : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteAtom(natom : u16) -> u16);
windows_link::link!("user32.dll" "system" fn EmptyClipboard() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn EnumClipboardFormats(format : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn FindAtomA(lpstring : windows_sys::core::PCSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn FindAtomW(lpstring : windows_sys::core::PCWSTR) -> u16);
windows_link::link!("user32.dll" "system" fn FreeDDElParam(msg : u32, lparam : super::super::Foundation::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetAtomNameA(natom : u16, lpbuffer : windows_sys::core::PSTR, nsize : i32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetAtomNameW(natom : u16, lpbuffer : windows_sys::core::PWSTR, nsize : i32) -> u32);
windows_link::link!("user32.dll" "system" fn GetClipboardData(uformat : u32) -> super::super::Foundation::HANDLE);
windows_link::link!("user32.dll" "system" fn GetClipboardFormatNameA(format : u32, lpszformatname : windows_sys::core::PSTR, cchmaxcount : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetClipboardFormatNameW(format : u32, lpszformatname : windows_sys::core::PWSTR, cchmaxcount : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetClipboardOwner() -> super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetClipboardSequenceNumber() -> u32);
windows_link::link!("user32.dll" "system" fn GetClipboardViewer() -> super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetOpenClipboardWindow() -> super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetPriorityClipboardFormat(paformatprioritylist : *const u32, cformats : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetUpdatedClipboardFormats(lpuiformats : *mut u32, cformats : u32, pcformatsout : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomA(lpstring : windows_sys::core::PCSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomExA(lpstring : windows_sys::core::PCSTR, flags : u32) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomExW(lpstring : windows_sys::core::PCWSTR, flags : u32) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomW(lpstring : windows_sys::core::PCWSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalDeleteAtom(natom : u16) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalFindAtomA(lpstring : windows_sys::core::PCSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalFindAtomW(lpstring : windows_sys::core::PCWSTR) -> u16);
windows_link::link!("kernel32.dll" "system" fn GlobalGetAtomNameA(natom : u16, lpbuffer : windows_sys::core::PSTR, nsize : i32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GlobalGetAtomNameW(natom : u16, lpbuffer : windows_sys::core::PWSTR, nsize : i32) -> u32);
windows_link::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::super::Foundation::HWND, hwndserver : super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn InitAtomTable(nsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsClipboardFormatAvailable(format : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn OpenClipboard(hwndnewowner : super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn PackDDElParam(msg : u32, uilo : usize, uihi : usize) -> super::super::Foundation::LPARAM);
windows_link::link!("user32.dll" "system" fn RegisterClipboardFormatA(lpszformat : windows_sys::core::PCSTR) -> u32);
windows_link::link!("user32.dll" "system" fn RegisterClipboardFormatW(lpszformat : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("user32.dll" "system" fn RemoveClipboardFormatListener(hwnd : super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::super::Foundation::LPARAM, msgin : u32, msgout : u32, uilo : usize, uihi : usize) -> super::super::Foundation::LPARAM);
windows_link::link!("user32.dll" "system" fn SetClipboardData(uformat : u32, hmem : super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE);
windows_link::link!("user32.dll" "system" fn SetClipboardViewer(hwndnewviewer : super::super::Foundation::HWND) -> super::super::Foundation::HWND);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("gdi32.dll" "system" fn SetWinMetaFileBits(nsize : u32, lpmeta16data : *const u8, hdcref : super::super::Graphics::Gdi::HDC, lpmfp : *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE);
windows_link::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::super::Foundation::LPARAM, puilo : *mut usize, puihi : *mut usize) -> windows_sys::core::BOOL);
pub const APPCLASS_MASK: i32 = 15;
pub const APPCLASS_MONITOR: DDE_INITIALIZE_COMMAND = 1;
pub const APPCLASS_STANDARD: DDE_INITIALIZE_COMMAND = 0;
pub const APPCMD_CLIENTONLY: DDE_INITIALIZE_COMMAND = 16;
pub const APPCMD_FILTERINITS: DDE_INITIALIZE_COMMAND = 32;
pub const APPCMD_MASK: i32 = 4080;
pub const CADV_LATEACK: u32 = 65535;
pub const CBF_FAIL_ADVISES: DDE_INITIALIZE_COMMAND = 16384;
pub const CBF_FAIL_ALLSVRXACTIONS: DDE_INITIALIZE_COMMAND = 258048;
pub const CBF_FAIL_CONNECTIONS: DDE_INITIALIZE_COMMAND = 8192;
pub const CBF_FAIL_EXECUTES: DDE_INITIALIZE_COMMAND = 32768;
pub const CBF_FAIL_POKES: DDE_INITIALIZE_COMMAND = 65536;
pub const CBF_FAIL_REQUESTS: DDE_INITIALIZE_COMMAND = 131072;
pub const CBF_FAIL_SELFCONNECTIONS: DDE_INITIALIZE_COMMAND = 4096;
pub const CBF_SKIP_ALLNOTIFICATIONS: DDE_INITIALIZE_COMMAND = 3932160;
pub const CBF_SKIP_CONNECT_CONFIRMS: DDE_INITIALIZE_COMMAND = 262144;
pub const CBF_SKIP_DISCONNECTS: DDE_INITIALIZE_COMMAND = 2097152;
pub const CBF_SKIP_REGISTRATIONS: DDE_INITIALIZE_COMMAND = 524288;
pub const CBF_SKIP_UNREGISTRATIONS: DDE_INITIALIZE_COMMAND = 1048576;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::super::Security::SECURITY_QUALITY_OF_SERVICE,
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
pub type CONVINFO_CONVERSATION_STATE = u32;
pub type CONVINFO_STATUS = u32;
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
pub const CP_WINANSI: i32 = 1004;
pub const CP_WINNEUTRAL: i32 = 1200;
pub const CP_WINUNICODE: i32 = 1200;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
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
pub type DDE_CLIENT_TRANSACTION_TYPE = u32;
pub type DDE_ENABLE_CALLBACK_CMD = u32;
pub const DDE_FACK: u32 = 32768;
pub const DDE_FACKREQ: u32 = 32768;
pub const DDE_FAPPSTATUS: u32 = 255;
pub const DDE_FBUSY: u32 = 16384;
pub const DDE_FDEFERUPD: u32 = 16384;
pub const DDE_FNOTPROCESSED: u32 = 0;
pub const DDE_FRELEASE: u32 = 8192;
pub const DDE_FREQUESTED: u32 = 4096;
pub type DDE_INITIALIZE_COMMAND = u32;
pub type DDE_NAME_SERVICE_CMD = u32;
pub const DMLERR_ADVACKTIMEOUT: u32 = 16384;
pub const DMLERR_BUSY: u32 = 16385;
pub const DMLERR_DATAACKTIMEOUT: u32 = 16386;
pub const DMLERR_DLL_NOT_INITIALIZED: u32 = 16387;
pub const DMLERR_DLL_USAGE: u32 = 16388;
pub const DMLERR_EXECACKTIMEOUT: u32 = 16389;
pub const DMLERR_FIRST: u32 = 16384;
pub const DMLERR_INVALIDPARAMETER: u32 = 16390;
pub const DMLERR_LAST: u32 = 16401;
pub const DMLERR_LOW_MEMORY: u32 = 16391;
pub const DMLERR_MEMORY_ERROR: u32 = 16392;
pub const DMLERR_NOTPROCESSED: u32 = 16393;
pub const DMLERR_NO_CONV_ESTABLISHED: u32 = 16394;
pub const DMLERR_NO_ERROR: u32 = 0;
pub const DMLERR_POKEACKTIMEOUT: u32 = 16395;
pub const DMLERR_POSTMSG_FAILED: u32 = 16396;
pub const DMLERR_REENTRANCY: u32 = 16397;
pub const DMLERR_SERVER_DIED: u32 = 16398;
pub const DMLERR_SYS_ERROR: u32 = 16399;
pub const DMLERR_UNADVACKTIMEOUT: u32 = 16400;
pub const DMLERR_UNFOUND_QUEUE_ID: u32 = 16401;
pub const DNS_FILTEROFF: DDE_NAME_SERVICE_CMD = 8;
pub const DNS_FILTERON: DDE_NAME_SERVICE_CMD = 4;
pub const DNS_REGISTER: DDE_NAME_SERVICE_CMD = 1;
pub const DNS_UNREGISTER: DDE_NAME_SERVICE_CMD = 2;
pub const EC_DISABLE: DDE_ENABLE_CALLBACK_CMD = 8;
pub const EC_ENABLEALL: DDE_ENABLE_CALLBACK_CMD = 0;
pub const EC_ENABLEONE: DDE_ENABLE_CALLBACK_CMD = 128;
pub const EC_QUERYWAITING: DDE_ENABLE_CALLBACK_CMD = 2;
pub type HCONV = *mut core::ffi::c_void;
pub type HCONVLIST = *mut core::ffi::c_void;
pub const HDATA_APPOWNED: u32 = 1;
pub type HDDEDATA = *mut core::ffi::c_void;
pub type HSZ = *mut core::ffi::c_void;
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
pub const MAX_MONITORS: u32 = 4;
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
pub const MF_CALLBACKS: DDE_INITIALIZE_COMMAND = 134217728;
pub const MF_CONV: DDE_INITIALIZE_COMMAND = 1073741824;
pub const MF_ERRORS: DDE_INITIALIZE_COMMAND = 268435456;
pub const MF_HSZ_INFO: DDE_INITIALIZE_COMMAND = 16777216;
pub const MF_LINKS: DDE_INITIALIZE_COMMAND = 536870912;
pub const MF_MASK: u32 = 4278190080;
pub const MF_POSTMSGS: DDE_INITIALIZE_COMMAND = 67108864;
pub const MF_SENDMSGS: DDE_INITIALIZE_COMMAND = 33554432;
pub const MH_CLEANUP: u32 = 4;
pub const MH_CREATE: u32 = 1;
pub const MH_DELETE: u32 = 3;
pub const MH_KEEP: u32 = 2;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: windows_sys::core::BOOL,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: windows_sys::core::BOOL,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: windows_sys::core::BOOL,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
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
impl Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const MSGF_DDEMGR: u32 = 32769;
pub type PFNCALLBACK = Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;
pub const QID_SYNC: u32 = 4294967295;
pub const ST_ADVISE: CONVINFO_STATUS = 2;
pub const ST_BLOCKED: CONVINFO_STATUS = 8;
pub const ST_BLOCKNEXT: CONVINFO_STATUS = 128;
pub const ST_CLIENT: CONVINFO_STATUS = 16;
pub const ST_CONNECTED: CONVINFO_STATUS = 1;
pub const ST_INLIST: CONVINFO_STATUS = 64;
pub const ST_ISLOCAL: CONVINFO_STATUS = 4;
pub const ST_ISSELF: CONVINFO_STATUS = 256;
pub const ST_TERMINATED: CONVINFO_STATUS = 32;
pub const SZDDESYS_ITEM_FORMATS: windows_sys::core::PCWSTR = windows_sys::core::w!("Formats");
pub const SZDDESYS_ITEM_HELP: windows_sys::core::PCWSTR = windows_sys::core::w!("Help");
pub const SZDDESYS_ITEM_RTNMSG: windows_sys::core::PCWSTR = windows_sys::core::w!("ReturnMessage");
pub const SZDDESYS_ITEM_STATUS: windows_sys::core::PCWSTR = windows_sys::core::w!("Status");
pub const SZDDESYS_ITEM_SYSITEMS: windows_sys::core::PCWSTR = windows_sys::core::w!("SysItems");
pub const SZDDESYS_ITEM_TOPICS: windows_sys::core::PCWSTR = windows_sys::core::w!("Topics");
pub const SZDDESYS_TOPIC: windows_sys::core::PCWSTR = windows_sys::core::w!("System");
pub const SZDDE_ITEM_ITEMLIST: windows_sys::core::PCWSTR = windows_sys::core::w!("TopicItemList");
pub const TIMEOUT_ASYNC: u32 = 4294967295;
pub const WM_DDE_ACK: u32 = 996;
pub const WM_DDE_ADVISE: u32 = 994;
pub const WM_DDE_DATA: u32 = 997;
pub const WM_DDE_EXECUTE: u32 = 1000;
pub const WM_DDE_FIRST: u32 = 992;
pub const WM_DDE_INITIATE: u32 = 992;
pub const WM_DDE_LAST: u32 = 1000;
pub const WM_DDE_POKE: u32 = 999;
pub const WM_DDE_REQUEST: u32 = 998;
pub const WM_DDE_TERMINATE: u32 = 993;
pub const WM_DDE_UNADVISE: u32 = 995;
pub const XCLASS_BOOL: u32 = 4096;
pub const XCLASS_DATA: u32 = 8192;
pub const XCLASS_FLAGS: u32 = 16384;
pub const XCLASS_MASK: u32 = 64512;
pub const XCLASS_NOTIFICATION: u32 = 32768;
pub const XST_ADVACKRCVD: CONVINFO_CONVERSATION_STATE = 13;
pub const XST_ADVDATAACKRCVD: CONVINFO_CONVERSATION_STATE = 16;
pub const XST_ADVDATASENT: CONVINFO_CONVERSATION_STATE = 15;
pub const XST_ADVSENT: CONVINFO_CONVERSATION_STATE = 11;
pub const XST_CONNECTED: CONVINFO_CONVERSATION_STATE = 2;
pub const XST_DATARCVD: CONVINFO_CONVERSATION_STATE = 6;
pub const XST_EXECACKRCVD: CONVINFO_CONVERSATION_STATE = 10;
pub const XST_EXECSENT: CONVINFO_CONVERSATION_STATE = 9;
pub const XST_INCOMPLETE: CONVINFO_CONVERSATION_STATE = 1;
pub const XST_INIT1: CONVINFO_CONVERSATION_STATE = 3;
pub const XST_INIT2: CONVINFO_CONVERSATION_STATE = 4;
pub const XST_NULL: CONVINFO_CONVERSATION_STATE = 0;
pub const XST_POKEACKRCVD: CONVINFO_CONVERSATION_STATE = 8;
pub const XST_POKESENT: CONVINFO_CONVERSATION_STATE = 7;
pub const XST_REQSENT: CONVINFO_CONVERSATION_STATE = 5;
pub const XST_UNADVACKRCVD: CONVINFO_CONVERSATION_STATE = 14;
pub const XST_UNADVSENT: CONVINFO_CONVERSATION_STATE = 12;
pub const XTYPF_ACKREQ: u32 = 8;
pub const XTYPF_NOBLOCK: u32 = 2;
pub const XTYPF_NODATA: u32 = 4;
pub const XTYP_ADVDATA: DDE_CLIENT_TRANSACTION_TYPE = 16400;
pub const XTYP_ADVREQ: DDE_CLIENT_TRANSACTION_TYPE = 8226;
pub const XTYP_ADVSTART: DDE_CLIENT_TRANSACTION_TYPE = 4144;
pub const XTYP_ADVSTOP: DDE_CLIENT_TRANSACTION_TYPE = 32832;
pub const XTYP_CONNECT: DDE_CLIENT_TRANSACTION_TYPE = 4194;
pub const XTYP_CONNECT_CONFIRM: DDE_CLIENT_TRANSACTION_TYPE = 32882;
pub const XTYP_DISCONNECT: DDE_CLIENT_TRANSACTION_TYPE = 32962;
pub const XTYP_EXECUTE: DDE_CLIENT_TRANSACTION_TYPE = 16464;
pub const XTYP_MASK: u32 = 240;
pub const XTYP_MONITOR: DDE_CLIENT_TRANSACTION_TYPE = 33010;
pub const XTYP_POKE: DDE_CLIENT_TRANSACTION_TYPE = 16528;
pub const XTYP_REGISTER: DDE_CLIENT_TRANSACTION_TYPE = 32930;
pub const XTYP_REQUEST: DDE_CLIENT_TRANSACTION_TYPE = 8368;
pub const XTYP_SHIFT: u32 = 4;
pub const XTYP_UNREGISTER: DDE_CLIENT_TRANSACTION_TYPE = 32978;
pub const XTYP_WILDCONNECT: DDE_CLIENT_TRANSACTION_TYPE = 8418;
pub const XTYP_XACT_COMPLETE: DDE_CLIENT_TRANSACTION_TYPE = 32896;
