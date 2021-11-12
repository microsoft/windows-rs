#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeClipboardChain(hwndremove: super::super::Foundation::HWND, hwndnewnext: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClipboard() -> super::super::Foundation::BOOL;
    pub fn CountClipboardFormats() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeAbandonTransaction(idinst: u32, hconv: HCONV, idtransaction: u32) -> super::super::Foundation::BOOL;
    pub fn DdeAccessData(hdata: HDDEDATA, pcbdatasize: *mut u32) -> *mut u8;
    pub fn DdeAddData(hdata: HDDEDATA, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA;
    pub fn DdeClientTransaction(pdata: *const u8, cbdata: u32, hconv: HCONV, hszitem: HSZ, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA;
    pub fn DdeCmpStringHandles(hsz1: HSZ, hsz2: HSZ) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnect(idinst: u32, hszservice: HSZ, hsztopic: HSZ, pcc: *const CONVCONTEXT) -> HCONV;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnectList(idinst: u32, hszservice: HSZ, hsztopic: HSZ, hconvlist: HCONVLIST, pcc: *const CONVCONTEXT) -> HCONVLIST;
    pub fn DdeCreateDataHandle(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: HSZ, wfmt: u32, afcmd: u32) -> HDDEDATA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleA(idinst: u32, psz: super::super::Foundation::PSTR, icodepage: i32) -> HSZ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleW(idinst: u32, psz: super::super::Foundation::PWSTR, icodepage: i32) -> HSZ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnect(hconv: HCONV) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnectList(hconvlist: HCONVLIST) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeEnableCallback(idinst: u32, hconv: HCONV, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeDataHandle(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    pub fn DdeGetData(hdata: HDDEDATA, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32;
    pub fn DdeGetLastError(idinst: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeImpersonateClient(hconv: HCONV) -> super::super::Foundation::BOOL;
    pub fn DdeInitializeA(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    pub fn DdeInitializeW(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeKeepStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    pub fn DdeNameService(idinst: u32, hsz1: HSZ, hsz2: HSZ, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdePostAdvise(idinst: u32, hsztopic: HSZ, hszitem: HSZ) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeQueryConvInfo(hconv: HCONV, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32;
    pub fn DdeQueryNextServer(hconvlist: HCONVLIST, hconvprev: HCONV) -> HCONV;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringA(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PSTR, cchmax: u32, icodepage: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringW(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PWSTR, cchmax: u32, icodepage: i32) -> u32;
    pub fn DdeReconnect(hconv: HCONV) -> HCONV;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeSetQualityOfService(hwndclient: super::super::Foundation::HWND, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeSetUserHandle(hconv: HCONV, id: u32, huser: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUnaccessData(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL;
    pub fn DeleteAtom(natom: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EmptyClipboard() -> super::super::Foundation::BOOL;
    pub fn EnumClipboardFormats(format: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardData(uformat: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameA(format: u32, lpszformatname: super::super::Foundation::PSTR, cchmaxcount: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameW(format: u32, lpszformatname: super::super::Foundation::PWSTR, cchmaxcount: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardOwner() -> super::super::Foundation::HWND;
    pub fn GetClipboardSequenceNumber() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardViewer() -> super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenClipboardWindow() -> super::super::Foundation::HWND;
    pub fn GetPriorityClipboardFormat(paformatprioritylist: *const u32, cformats: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExA(lpstring: super::super::Foundation::PSTR, flags: u32) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExW(lpstring: super::super::Foundation::PWSTR, flags: u32) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    pub fn GlobalDeleteAtom(natom: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateDdeClientWindow(hwndclient: super::super::Foundation::HWND, hwndserver: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClipboard(hwndnewowner: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatA(lpszformat: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatW(lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReuseDDElParam(lparam: super::super::Foundation::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardData(uformat: u32, hmem: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardViewer(hwndnewviewer: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetWinMetaFileBits(nsize: u32, lpmeta16data: *const u8, hdcref: super::super::Graphics::Gdi::HDC, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnpackDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL;
}
pub const APPCLASS_MASK: i32 = 15i32;
pub const APPCMD_MASK: i32 = 4080i32;
pub const CADV_LATEACK: u32 = 65535u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct CONVCONTEXT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct CONVINFO(i32);
#[repr(C)]
pub struct CONVINFO_CONVERSATION_STATE(i32);
#[repr(C)]
pub struct CONVINFO_STATUS(i32);
#[repr(C)]
pub struct COPYDATASTRUCT(i32);
pub const CP_WINANSI: i32 = 1004i32;
pub const CP_WINNEUTRAL: i32 = 1200i32;
pub const CP_WINUNICODE: i32 = 1200i32;
#[repr(C)]
pub struct DDEACK(i32);
#[repr(C)]
pub struct DDEADVISE(i32);
#[repr(C)]
pub struct DDEDATA(i32);
#[repr(C)]
pub struct DDELN(i32);
#[repr(C)]
pub struct DDEML_MSG_HOOK_DATA(i32);
#[repr(C)]
pub struct DDEPOKE(i32);
#[repr(C)]
pub struct DDEUP(i32);
#[repr(C)]
pub struct DDE_CLIENT_TRANSACTION_TYPE(i32);
#[repr(C)]
pub struct DDE_ENABLE_CALLBACK_CMD(i32);
pub const DDE_FACK: u32 = 32768u32;
pub const DDE_FACKREQ: u32 = 32768u32;
pub const DDE_FAPPSTATUS: u32 = 255u32;
pub const DDE_FBUSY: u32 = 16384u32;
pub const DDE_FDEFERUPD: u32 = 16384u32;
pub const DDE_FNOTPROCESSED: u32 = 0u32;
pub const DDE_FRELEASE: u32 = 8192u32;
pub const DDE_FREQUESTED: u32 = 4096u32;
#[repr(C)]
pub struct DDE_INITIALIZE_COMMAND(i32);
#[repr(C)]
pub struct DDE_NAME_SERVICE_CMD(i32);
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
#[repr(C)]
pub struct HCONV(i32);
#[repr(C)]
pub struct HCONVLIST(i32);
pub const HDATA_APPOWNED: u32 = 1u32;
#[repr(C)]
pub struct HDDEDATA(i32);
#[repr(C)]
pub struct HSZ(i32);
#[repr(C)]
pub struct HSZPAIR(i32);
pub const MAX_MONITORS: u32 = 4u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct METAFILEPICT(i32);
pub const MF_MASK: u32 = 4278190080u32;
pub const MH_CLEANUP: u32 = 4u32;
pub const MH_CREATE: u32 = 1u32;
pub const MH_DELETE: u32 = 3u32;
pub const MH_KEEP: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct MONCBSTRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONCONVSTRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONERRSTRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONHSZSTRUCTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONHSZSTRUCTW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONLINKSTRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MONMSGSTRUCT(i32);
pub const MSGF_DDEMGR: u32 = 32769u32;
#[repr(C)]
pub struct PFNCALLBACK(i32);
pub const QID_SYNC: u32 = 4294967295u32;
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
pub const XTYPF_ACKREQ: u32 = 8u32;
pub const XTYPF_NOBLOCK: u32 = 2u32;
pub const XTYPF_NODATA: u32 = 4u32;
pub const XTYP_MASK: u32 = 240u32;
pub const XTYP_SHIFT: u32 = 4u32;
