#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameA(param0 : LPOPENCARDNAMEA) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameW(param0 : LPOPENCARDNAMEW) -> super::LONG);
#[cfg(feature = "winnt")]
windows_link::link!("winscard.dll" "system" fn SCardAccessStartedEvent() -> super::HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szgroupname : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szgroupname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardAudit(hcontext : SCARDCONTEXT, dwevent : super::DWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardBeginTransaction(hcard : SCARDHANDLE) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardCancel(hcontext : SCARDCONTEXT) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardConnectA(hcontext : SCARDCONTEXT, szreader : windows_sys::core::PCSTR, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, phcard : LPSCARDHANDLE, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardConnectW(hcontext : SCARDCONTEXT, szreader : windows_sys::core::PCWSTR, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, phcard : LPSCARDHANDLE, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardControl(hcard : SCARDHANDLE, dwcontrolcode : super::DWORD, lpinbuffer : super::LPCVOID, cbinbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, cboutbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardDisconnect(hcard : SCARDHANDLE, dwdisposition : super::DWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("scarddlg.dll" "system" fn SCardDlgExtendedError() -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardEndTransaction(hcard : SCARDHANDLE, dwdisposition : super::DWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardEstablishContext(dwscope : super::DWORD, pvreserved1 : super::LPCVOID, pvreserved2 : super::LPCVOID, phcontext : LPSCARDCONTEXT) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardForgetReaderW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardFreeMemory(hcontext : SCARDCONTEXT, pvmem : super::LPCVOID) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetAttrib(hcard : SCARDHANDLE, dwattrid : super::DWORD, pbattr : super::LPBYTE, pcbattrlen : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, dwproviderid : super::DWORD, szprovider : *mut super::CHAR, pcchprovider : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, dwproviderid : super::DWORD, szprovider : *mut super::WCHAR, pcchprovider : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, pdwdevicetypeid : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, pdwdevicetypeid : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdA(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCSTR, pguidproviderid : super::LPGUID) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdW(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCWSTR, pguidproviderid : super::LPGUID) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szdeviceinstanceid : windows_sys::core::PSTR, pcchdeviceinstanceid : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szdeviceinstanceid : windows_sys::core::PWSTR, pcchdeviceinstanceid : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, pbicon : super::LPBYTE, pcbicon : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, pbicon : super::LPBYTE, pcbicon : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeA(hcontext : SCARDCONTEXT, dwtimeout : super::DWORD, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeW(hcontext : SCARDCONTEXT, dwtimeout : super::DWORD, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardGetTransmitCount(hcard : SCARDHANDLE, pctransmitcount : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, pguidprimaryprovider : super::LPCGUID, rgguidinterfaces : super::LPCGUID, dwinterfacecount : super::DWORD, pbatr : LPCBYTE, pbatrmask : LPCBYTE, cbatrlen : super::DWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, pguidprimaryprovider : super::LPCGUID, rgguidinterfaces : super::LPCGUID, dwinterfacecount : super::DWORD, pbatr : LPCBYTE, pbatrmask : LPCBYTE, cbatrlen : super::DWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szdevicename : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szdevicename : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardIsValidContext(hcontext : SCARDCONTEXT) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListCardsA(hcontext : SCARDCONTEXT, pbatr : LPCBYTE, rgquidinterfaces : super::LPCGUID, cguidinterfacecount : super::DWORD, mszcards : *mut super::CHAR, pcchcards : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winscard.dll" "system" fn SCardListCardsW(hcontext : SCARDCONTEXT, pbatr : LPCBYTE, rgquidinterfaces : super::LPCGUID, cguidinterfacecount : super::DWORD, mszcards : *mut super::WCHAR, pcchcards : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListInterfacesA(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCSTR, pguidinterfaces : super::LPGUID, pcguidinterfaces : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListInterfacesW(hcontext : SCARDCONTEXT, szcard : windows_sys::core::PCWSTR, pguidinterfaces : super::LPGUID, pcguidinterfaces : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsA(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PSTR, pcchgroups : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsW(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PWSTR, pcchgroups : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReadersA(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PCSTR, mszreaders : windows_sys::core::PSTR, pcchreaders : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReadersW(hcontext : SCARDCONTEXT, mszgroups : windows_sys::core::PCWSTR, mszreaders : windows_sys::core::PWSTR, pcchreaders : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdA(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_sys::core::PCSTR, mszreaders : windows_sys::core::PSTR, pcchreaders : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdW(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_sys::core::PCWSTR, mszreaders : windows_sys::core::PWSTR, pcchreaders : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsA(hcontext : SCARDCONTEXT, mszcards : windows_sys::core::PCSTR, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRA(hcontext : SCARDCONTEXT, rgatrmasks : LPSCARD_ATRMASK, catrs : super::DWORD, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRW(hcontext : SCARDCONTEXT, rgatrmasks : LPSCARD_ATRMASK, catrs : super::DWORD, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardLocateCardsW(hcontext : SCARDCONTEXT, mszcards : windows_sys::core::PCWSTR, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
#[cfg(all(feature = "rpc", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardReadCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_sys::core::PCSTR, data : super::PBYTE, datalen : *mut super::DWORD) -> super::LONG);
#[cfg(all(feature = "rpc", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardReadCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_sys::core::PCWSTR, data : super::PBYTE, datalen : *mut super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardReconnect(hcard : SCARDHANDLE, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, dwinitialization : super::DWORD, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardReleaseContext(hcontext : SCARDCONTEXT) -> super::LONG);
windows_link::link!("winscard.dll" "system" fn SCardReleaseStartedEvent());
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupA(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCSTR, szgroupname : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupW(hcontext : SCARDCONTEXT, szreadername : windows_sys::core::PCWSTR, szgroupname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardSetAttrib(hcard : SCARDHANDLE, dwattrid : super::DWORD, pbattr : LPCBYTE, cbattrlen : super::DWORD) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCSTR, dwproviderid : super::DWORD, szprovider : windows_sys::core::PCSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_sys::core::PCWSTR, dwproviderid : super::DWORD, szprovider : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardState(hcard : SCARDHANDLE, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardStatusA(hcard : SCARDHANDLE, mszreadernames : windows_sys::core::PSTR, pcchreaderlen : super::LPDWORD, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardStatusW(hcard : SCARDHANDLE, mszreadernames : windows_sys::core::PWSTR, pcchreaderlen : super::LPDWORD, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winsmcrd"))]
windows_link::link!("winscard.dll" "system" fn SCardTransmit(hcard : SCARDHANDLE, piosendpci : super::LPCSCARD_IO_REQUEST, pbsendbuffer : LPCBYTE, cbsendlength : super::DWORD, piorecvpci : super::LPSCARD_IO_REQUEST, pbrecvbuffer : super::LPBYTE, pcbrecvlength : super::LPDWORD) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardA(param0 : LPOPENCARDNAME_EXA) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardW(param0 : LPOPENCARDNAME_EXW) -> super::LONG);
#[cfg(all(feature = "rpc", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardWriteCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_sys::core::PCSTR, data : super::PBYTE, datalen : super::DWORD) -> super::LONG);
#[cfg(all(feature = "rpc", feature = "shared"))]
windows_link::link!("winscard.dll" "system" fn SCardWriteCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_sys::core::PCWSTR, data : super::PBYTE, datalen : super::DWORD) -> super::LONG);
pub type LPCBYTE = *const u8;
#[cfg(feature = "winnt")]
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: super::PVOID) -> SCARDHANDLE>;
#[cfg(feature = "winnt")]
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: super::PVOID) -> SCARDHANDLE>;
#[cfg(feature = "winnt")]
pub type LPOCNDSCPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: super::PVOID)>;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAME = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAMEA = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAMEW = *mut OPENCARDNAMEW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAME_EX = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPOPENCARD_SEARCH_CRITERIA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPOPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPOPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
#[cfg(target_arch = "x86")]
pub type LPSCARDCONTEXT = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LPSCARDCONTEXT = *mut u64;
#[cfg(target_arch = "x86")]
pub type LPSCARDHANDLE = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LPSCARDHANDLE = *mut u64;
#[cfg(feature = "shared")]
pub type LPSCARD_ATRMASK = *mut SCARD_ATRMASK;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPSCARD_READERSTATE = *mut SCARD_READERSTATEA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type OPENCARDNAME = OPENCARDNAMEA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_sys::core::PSTR,
    pub nMaxGroupNames: super::DWORD,
    pub lpstrCardNames: windows_sys::core::PSTR,
    pub nMaxCardNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrRdr: windows_sys::core::PSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_sys::core::PSTR,
    pub nMaxCard: super::DWORD,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub dwFlags: super::DWORD,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_sys::core::PWSTR,
    pub nMaxGroupNames: super::DWORD,
    pub lpstrCardNames: windows_sys::core::PWSTR,
    pub nMaxCardNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrRdr: windows_sys::core::PWSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_sys::core::PWSTR,
    pub nMaxCard: super::DWORD,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub dwFlags: super::DWORD,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type OPENCARDNAME_EX = OPENCARDNAME_EXA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: super::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub lpstrSearchDesc: windows_sys::core::PCSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub lpstrRdr: windows_sys::core::PSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_sys::core::PSTR,
    pub nMaxCard: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: super::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub lpstrSearchDesc: windows_sys::core::PCWSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub lpstrRdr: windows_sys::core::PWSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_sys::core::PWSTR,
    pub nMaxCard: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type OPENCARD_SEARCH_CRITERIA = OPENCARD_SEARCH_CRITERIAA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: super::DWORD,
    pub lpstrGroupNames: windows_sys::core::PSTR,
    pub nMaxGroupNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrCardNames: windows_sys::core::PSTR,
    pub nMaxCardNames: super::DWORD,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: super::DWORD,
    pub lpstrGroupNames: windows_sys::core::PWSTR,
    pub nMaxGroupNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrCardNames: windows_sys::core::PWSTR,
    pub nMaxCardNames: super::DWORD,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAME = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAMEA = *mut OPENCARDNAMEA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAMEW = *mut OPENCARDNAMEW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAME_EX = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type POPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type POPENCARD_SEARCH_CRITERIA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type POPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type POPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
#[cfg(feature = "shared")]
pub type PREADER_SEL_REQUEST = *mut READER_SEL_REQUEST;
#[cfg(feature = "shared")]
pub type PREADER_SEL_RESPONSE = *mut READER_SEL_RESPONSE;
#[cfg(target_arch = "x86")]
pub type PSCARDCONTEXT = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCARDCONTEXT = *mut u64;
#[cfg(target_arch = "x86")]
pub type PSCARDHANDLE = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCARDHANDLE = *mut u64;
#[cfg(feature = "shared")]
pub type PSCARD_ATRMASK = *mut SCARD_ATRMASK;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PSCARD_READERSTATE = *mut SCARD_READERSTATEA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
#[cfg(feature = "shared")]
impl Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
#[cfg(feature = "shared")]
impl Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: super::DWORD,
    pub cchReaderNameLength: super::DWORD,
    pub cbContainerNameOffset: super::DWORD,
    pub cchContainerNameLength: super::DWORD,
    pub dwDesiredCardModuleVersion: super::DWORD,
    pub dwCspFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: super::DWORD,
    pub cbSerialNumberLength: super::DWORD,
    pub dwDesiredCardModuleVersion: super::DWORD,
}
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: super::DWORD,
    pub cchReaderNameLength: super::DWORD,
    pub cbCardNameOffset: super::DWORD,
    pub cchCardNameLength: super::DWORD,
}
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3;
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1;
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2;
#[cfg(target_arch = "x86")]
pub type SCARDCONTEXT = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SCARDCONTEXT = u64;
#[cfg(target_arch = "x86")]
pub type SCARDHANDLE = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SCARDHANDLE = u64;
pub const SCARD_ALL_READERS: windows_sys::core::PCSTR = windows_sys::core::s!("SCard$AllReaders\u{0}");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCARD_ATRMASK {
    pub cbAtr: super::DWORD,
    pub rgbAtr: [super::BYTE; 36],
    pub rgbMask: [super::BYTE; 36],
}
#[cfg(feature = "shared")]
impl Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_AUDIT_CHV_FAILURE: i32 = 0;
pub const SCARD_AUDIT_CHV_SUCCESS: i32 = 1;
#[cfg(feature = "shared")]
pub const SCARD_AUTOALLOCATE: super::DWORD = 4294967295;
pub const SCARD_DEFAULT_READERS: windows_sys::core::PCSTR = windows_sys::core::s!("SCard$DefaultReaders\u{0}");
pub const SCARD_EJECT_CARD: i32 = 3;
pub const SCARD_LEAVE_CARD: i32 = 0;
pub const SCARD_LOCAL_READERS: windows_sys::core::PCSTR = windows_sys::core::s!("SCard$LocalReaders\u{0}");
pub const SCARD_PROVIDER_CSP: i32 = 2;
pub const SCARD_PROVIDER_KSP: i32 = 3;
pub const SCARD_PROVIDER_PRIMARY: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type SCARD_READERSTATE = SCARD_READERSTATEA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_sys::core::PCSTR,
    pub pvUserData: super::LPVOID,
    pub dwCurrentState: super::DWORD,
    pub dwEventState: super::DWORD,
    pub cbAtr: super::DWORD,
    pub rgbAtr: [super::BYTE; 36],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_sys::core::PCWSTR,
    pub pvUserData: super::LPVOID,
    pub dwCurrentState: super::DWORD,
    pub dwEventState: super::DWORD,
    pub cbAtr: super::DWORD,
    pub rgbAtr: [super::BYTE; 36],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const SCARD_READER_SEL_AUTH_PACKAGE: super::DWORD = 4294966667;
pub const SCARD_RESET_CARD: i32 = 1;
pub const SCARD_SCOPE_SYSTEM: i32 = 2;
pub const SCARD_SCOPE_TERMINAL: i32 = 1;
pub const SCARD_SCOPE_USER: i32 = 0;
pub const SCARD_SHARE_DIRECT: i32 = 3;
pub const SCARD_SHARE_EXCLUSIVE: i32 = 1;
pub const SCARD_SHARE_SHARED: i32 = 2;
pub const SCARD_STATE_ATRMATCH: i32 = 64;
pub const SCARD_STATE_CHANGED: i32 = 2;
pub const SCARD_STATE_EMPTY: i32 = 16;
pub const SCARD_STATE_EXCLUSIVE: i32 = 128;
pub const SCARD_STATE_IGNORE: i32 = 1;
pub const SCARD_STATE_INUSE: i32 = 256;
pub const SCARD_STATE_MUTE: i32 = 512;
pub const SCARD_STATE_PRESENT: i32 = 32;
pub const SCARD_STATE_UNAVAILABLE: i32 = 8;
pub const SCARD_STATE_UNAWARE: i32 = 0;
pub const SCARD_STATE_UNKNOWN: i32 = 4;
pub const SCARD_STATE_UNPOWERED: i32 = 1024;
pub const SCARD_SYSTEM_READERS: windows_sys::core::PCSTR = windows_sys::core::s!("SCard$SystemReaders\u{0}");
pub const SCARD_UNPOWER_CARD: i32 = 2;
pub const SCERR_NOCARDNAME: i32 = 16384;
pub const SCERR_NOGUIDS: i32 = 32768;
pub const SC_DLG_FORCE_UI: i32 = 4;
pub const SC_DLG_MINIMAL_UI: i32 = 1;
pub const SC_DLG_NO_UI: i32 = 2;
