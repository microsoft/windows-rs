#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeClipboardChain(hwndremove: super::super::Foundation::HWND, hwndnewnext: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClipboard() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn CountClipboardFormats() -> i32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeAbandonTransaction(idinst: u32, hconv: HCONV, idtransaction: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeAccessData(hdata: HDDEDATA, pcbdatasize: *mut u32) -> *mut u8;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeAddData(hdata: HDDEDATA, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeClientTransaction(pdata: *const u8, cbdata: u32, hconv: HCONV, hszitem: HSZ, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeCmpStringHandles(hsz1: HSZ, hsz2: HSZ) -> i32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnect(idinst: u32, hszservice: HSZ, hsztopic: HSZ, pcc: *const CONVCONTEXT) -> HCONV;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnectList(idinst: u32, hszservice: HSZ, hsztopic: HSZ, hconvlist: HCONVLIST, pcc: *const CONVCONTEXT) -> HCONVLIST;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeCreateDataHandle(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: HSZ, wfmt: u32, afcmd: u32) -> HDDEDATA;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleA(idinst: u32, psz: super::super::Foundation::PSTR, icodepage: i32) -> HSZ;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleW(idinst: u32, psz: super::super::Foundation::PWSTR, icodepage: i32) -> HSZ;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnect(hconv: HCONV) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnectList(hconvlist: HCONVLIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeEnableCallback(idinst: u32, hconv: HCONV, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeDataHandle(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeGetData(hdata: HDDEDATA, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeGetLastError(idinst: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeImpersonateClient(hconv: HCONV) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeInitializeA(pidinst: *mut u32, pfncallback: ::windows::runtime::RawPtr, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeInitializeW(pidinst: *mut u32, pfncallback: ::windows::runtime::RawPtr, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeKeepStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeNameService(idinst: u32, hsz1: HSZ, hsz2: HSZ, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdePostAdvise(idinst: u32, hsztopic: HSZ, hszitem: HSZ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeQueryConvInfo(hconv: HCONV, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeQueryNextServer(hconvlist: HCONVLIST, hconvprev: HCONV) -> HCONV;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringA(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PSTR, cchmax: u32, icodepage: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringW(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PWSTR, cchmax: u32, icodepage: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeReconnect(hconv: HCONV) -> HCONV;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeSetQualityOfService(hwndclient: super::super::Foundation::HWND, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeSetUserHandle(hconv: HCONV, id: u32, huser: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUnaccessData(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DeleteAtom(natom: u16) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EmptyClipboard() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn EnumClipboardFormats(format: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardData(uformat: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameA(format: u32, lpszformatname: super::super::Foundation::PSTR, cchmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameW(format: u32, lpszformatname: super::super::Foundation::PWSTR, cchmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardOwner() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GetClipboardSequenceNumber() -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardViewer() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenClipboardWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GetPriorityClipboardFormat(paformatprioritylist: *const u32, cformats: i32) -> i32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExA(lpstring: super::super::Foundation::PSTR, flags: u32) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExW(lpstring: super::super::Foundation::PWSTR, flags: u32) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GlobalDeleteAtom(natom: u16) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateDdeClientWindow(hwndclient: super::super::Foundation::HWND, hwndserver: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClipboard(hwndnewowner: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatA(lpszformat: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatW(lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReuseDDElParam(lparam: super::super::Foundation::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardData(uformat: u32, hmem: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardViewer(hwndnewviewer: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetWinMetaFileBits(nsize: u32, lpmeta16data: *const u8, hdcref: super::super::Graphics::Gdi::HDC, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE;
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnpackDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL;
}
