#[cfg(all(feature = "guiddef", feature = "windef"))]
windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameA(param0 : *mut OPENCARDNAMEA) -> i32);
#[cfg(all(feature = "guiddef", feature = "windef"))]
windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameW(param0 : *mut OPENCARDNAMEW) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("winscard.dll" "system" fn SCardAccessStartedEvent() -> super::winnt::HANDLE);
windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szgroupname : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szgroupname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardAudit(hcontext : SCARDCONTEXT, dwevent : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardBeginTransaction(hcard : SCARDHANDLE) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardCancel(hcontext : SCARDCONTEXT) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardConnectA(hcontext : SCARDCONTEXT, szreader : windows_sys::core::PCSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut SCARDHANDLE, pdwactiveprotocol : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardConnectW(hcontext : SCARDCONTEXT, szreader : windows_sys::core::PCWSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut SCARDHANDLE, pdwactiveprotocol : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardControl(hcard : SCARDHANDLE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardDisconnect(hcard : SCARDHANDLE, dwdisposition : u32) -> i32);
windows_link::link!("scarddlg.dll" "system" fn SCardDlgExtendedError() -> i32);
windows_link::link!("winscard.dll" "system" fn SCardEndTransaction(hcard : SCARDHANDLE, dwdisposition : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardEstablishContext(dwscope : u32, pvreserved1 : *const core::ffi::c_void, pvreserved2 : *const core::ffi::c_void, phcontext : *mut SCARDCONTEXT) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardFreeMemory(hcontext : SCARDCONTEXT, pvmem : *const core::ffi::c_void) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetAttrib(hcard : SCARDHANDLE, dwattrid : u32, pbattr : *mut u8, pcbattrlen : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, dwproviderid : u32, szprovider : *mut i8, pcchprovider : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, dwproviderid : u32, szprovider : *mut u16, pcchprovider : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, pdwdevicetypeid : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, pdwdevicetypeid : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdA(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCSTR, pguidproviderid : *mut windows_sys::core::GUID) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdW(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCWSTR, pguidproviderid : *mut windows_sys::core::GUID) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szdeviceinstanceid : windows_sys::core::PCSTR, pcchdeviceinstanceid : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szdeviceinstanceid : windows_sys::core::PCWSTR, pcchdeviceinstanceid : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeA(hcontext : SCARDCONTEXT, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeW(hcontext : SCARDCONTEXT, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardGetTransmitCount(hcard : SCARDHANDLE, pctransmitcount : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, pguidprimaryprovider : *const windows_sys::core::GUID, rgguidinterfaces : *const windows_sys::core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, pguidprimaryprovider : *const windows_sys::core::GUID, rgguidinterfaces : *const windows_sys::core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szdevicename : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szdevicename : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardIsValidContext(hcontext : SCARDCONTEXT) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListCardsA(hcontext : SCARDCONTEXT, pbatr : *const u8, rgquidinterfaces : *const windows_sys::core::GUID, cguidinterfacecount : u32, mszcards : *mut i8, pcchcards : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListCardsW(hcontext : SCARDCONTEXT, pbatr : *const u8, rgquidinterfaces : *const windows_sys::core::GUID, cguidinterfacecount : u32, mszcards : *mut u16, pcchcards : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListInterfacesA(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCSTR, pguidinterfaces : *mut windows_sys::core::GUID, pcguidinterfaces : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListInterfacesW(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCWSTR, pguidinterfaces : *mut windows_sys::core::GUID, pcguidinterfaces : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsA(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PSTR, pcchgroups : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsW(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PWSTR, pcchgroups : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReadersA(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PCSTR, mszreaders : windows_sys::core::PCSTR, pcchreaders : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReadersW(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PCWSTR, mszreaders : windows_sys::core::PCWSTR, pcchreaders : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdA(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_sys::core::PCSTR, mszreaders : windows_sys::core::PCSTR, pcchreaders : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdW(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_sys::core::PCWSTR, mszreaders : windows_sys::core::PCWSTR, pcchreaders : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsA(hcontext : SCARDCONTEXT, mszcards : windows_sys::core::PCSTR, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRA(hcontext : SCARDCONTEXT, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRW(hcontext : SCARDCONTEXT, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsW(hcontext : SCARDCONTEXT, mszcards : windows_sys::core::PCWSTR, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardReadCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const windows_sys::core::GUID, freshnesscounter : u32, lookupname : windows_sys::core::PCSTR, data : *mut u8, datalen : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardReadCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const windows_sys::core::GUID, freshnesscounter : u32, lookupname : windows_sys::core::PCWSTR, data : *mut u8, datalen : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardReconnect(hcard : SCARDHANDLE, dwsharemode : u32, dwpreferredprotocols : u32, dwinitialization : u32, pdwactiveprotocol : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardReleaseContext(hcontext : SCARDCONTEXT) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardReleaseStartedEvent());
windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szgroupname : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szgroupname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardSetAttrib(hcard : SCARDHANDLE, dwattrid : u32, pbattr : *const u8, cbattrlen : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, dwproviderid : u32, szprovider : windows_sys::core::PCSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, dwproviderid : u32, szprovider : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardState(hcard : SCARDHANDLE, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardStatusA(hcard : SCARDHANDLE, mszreadernames : windows_sys::core::PCSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardStatusW(hcard : SCARDHANDLE, mszreadernames : windows_sys::core::PCWSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
#[cfg(feature = "winsmcrd")]
windows_link::link!("winscard.dll" "system" fn SCardTransmit(hcard : SCARDHANDLE, piosendpci : *const super::winsmcrd::SCARD_IO_REQUEST, pbsendbuffer : *const u8, cbsendlength : u32, piorecvpci : *mut super::winsmcrd::SCARD_IO_REQUEST, pbrecvbuffer : *mut u8, pcbrecvlength : *mut u32) -> i32);
#[cfg(all(feature = "guiddef", feature = "windef"))]
windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardA(param0 : *mut OPENCARDNAME_EXA) -> i32);
#[cfg(all(feature = "guiddef", feature = "windef"))]
windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardW(param0 : *mut OPENCARDNAME_EXW) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardWriteCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const windows_sys::core::GUID, freshnesscounter : u32, lookupname : windows_sys::core::PCSTR, data : *const u8, datalen : u32) -> i32);
windows_link::link!("winscard.dll" "system" fn SCardWriteCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const windows_sys::core::GUID, freshnesscounter : u32, lookupname : windows_sys::core::PCWSTR, data : *const u8, datalen : u32) -> i32);
pub type LPCBYTE = *const u8;
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *const core::ffi::c_void) -> SCARDHANDLE>;
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *const core::ffi::c_void) -> SCARDHANDLE>;
pub type LPOCNDSCPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: *const core::ffi::c_void)>;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAME = LPOPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAMEA = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAMEW = *mut OPENCARDNAMEW;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAME_EX = LPOPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type LPOPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
#[cfg(feature = "guiddef")]
pub type LPOPENCARD_SEARCH_CRITERIA = LPOPENCARD_SEARCH_CRITERIAA;
#[cfg(feature = "guiddef")]
pub type LPOPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(feature = "guiddef")]
pub type LPOPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARD_ATRMASK = *mut SCARD_ATRMASK;
pub type LPSCARD_READERSTATE = LPSCARD_READERSTATEA;
pub type LPSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
pub type LPSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type OPENCARDNAME = OPENCARDNAMEA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_sys::core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_sys::core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: super::guiddef::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_sys::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_sys::core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_sys::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_sys::core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: super::guiddef::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_sys::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_sys::core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type OPENCARDNAME_EX = OPENCARDNAME_EXA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub lpstrSearchDesc: windows_sys::core::PCSTR,
    pub hIcon: super::windef::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_sys::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_sys::core::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub lpstrSearchDesc: windows_sys::core::PCWSTR,
    pub hIcon: super::windef::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_sys::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_sys::core::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "guiddef")]
pub type OPENCARD_SEARCH_CRITERIA = OPENCARD_SEARCH_CRITERIAA;
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_sys::core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: super::guiddef::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_sys::core::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "guiddef")]
impl Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_sys::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: super::guiddef::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_sys::core::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "guiddef")]
impl Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAME = POPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAMEA = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAMEW = *mut OPENCARDNAMEW;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAME_EX = POPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub type POPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
#[cfg(feature = "guiddef")]
pub type POPENCARD_SEARCH_CRITERIA = POPENCARD_SEARCH_CRITERIAA;
#[cfg(feature = "guiddef")]
pub type POPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(feature = "guiddef")]
pub type POPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
pub type PREADER_SEL_REQUEST = *mut READER_SEL_REQUEST;
pub type PREADER_SEL_RESPONSE = *mut READER_SEL_RESPONSE;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type PSCARD_ATRMASK = *mut SCARD_ATRMASK;
pub type PSCARD_READERSTATE = PSCARD_READERSTATEA;
pub type PSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
pub type PSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3;
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1;
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2;
pub type SCARDCONTEXT = usize;
pub type SCARDHANDLE = usize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0;
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1;
pub const SCARD_AUTOALLOCATE: i32 = -1;
pub const SCARD_EJECT_CARD: u32 = 3;
pub const SCARD_LEAVE_CARD: u32 = 0;
pub const SCARD_PROVIDER_CSP: u32 = 2;
pub const SCARD_PROVIDER_KSP: u32 = 3;
pub const SCARD_PROVIDER_PRIMARY: u32 = 1;
pub type SCARD_READERSTATE = SCARD_READERSTATEA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_sys::core::PCSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: u32,
    pub dwEventState: u32,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_sys::core::PCWSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: u32,
    pub dwEventState: u32,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_READER_SEL_AUTH_PACKAGE: u32 = 4294966667;
pub const SCARD_RESET_CARD: u32 = 1;
pub const SCARD_SCOPE_SYSTEM: u32 = 2;
pub const SCARD_SCOPE_TERMINAL: u32 = 1;
pub const SCARD_SCOPE_USER: u32 = 0;
pub const SCARD_SHARE_DIRECT: u32 = 3;
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1;
pub const SCARD_SHARE_SHARED: u32 = 2;
pub const SCARD_STATE_ATRMATCH: u32 = 64;
pub const SCARD_STATE_CHANGED: u32 = 2;
pub const SCARD_STATE_EMPTY: u32 = 16;
pub const SCARD_STATE_EXCLUSIVE: u32 = 128;
pub const SCARD_STATE_IGNORE: u32 = 1;
pub const SCARD_STATE_INUSE: u32 = 256;
pub const SCARD_STATE_MUTE: u32 = 512;
pub const SCARD_STATE_PRESENT: u32 = 32;
pub const SCARD_STATE_UNAVAILABLE: u32 = 8;
pub const SCARD_STATE_UNAWARE: u32 = 0;
pub const SCARD_STATE_UNKNOWN: u32 = 4;
pub const SCARD_STATE_UNPOWERED: u32 = 1024;
pub const SCARD_UNPOWER_CARD: u32 = 2;
pub const SCERR_NOCARDNAME: u32 = 16384;
pub const SCERR_NOGUIDS: u32 = 32768;
pub const SC_DLG_FORCE_UI: u32 = 4;
pub const SC_DLG_MINIMAL_UI: u32 = 1;
pub const SC_DLG_NO_UI: u32 = 2;
