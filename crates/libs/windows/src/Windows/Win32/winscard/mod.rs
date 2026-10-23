#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetOpenCardNameA(param0: LPOPENCARDNAMEA) -> super::LONG {
    windows_core::link!("scarddlg.dll" "system" fn GetOpenCardNameA(param0 : LPOPENCARDNAMEA) -> super::LONG);
    unsafe { GetOpenCardNameA(param0 as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetOpenCardNameW(param0: LPOPENCARDNAMEW) -> super::LONG {
    windows_core::link!("scarddlg.dll" "system" fn GetOpenCardNameW(param0 : LPOPENCARDNAMEW) -> super::LONG);
    unsafe { GetOpenCardNameW(param0 as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> super::HANDLE {
    windows_core::link!("winscard.dll" "system" fn SCardAccessStartedEvent() -> super::HANDLE);
    unsafe { SCardAccessStartedEvent() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardAddReaderToGroupA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardAddReaderToGroupA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardAddReaderToGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardAddReaderToGroupW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardAddReaderToGroupW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardAddReaderToGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardAudit(hcontext: SCARDCONTEXT, dwevent: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardAudit(hcontext : SCARDCONTEXT, dwevent : super::DWORD) -> super::LONG);
    unsafe { SCardAudit(hcontext, dwevent) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: SCARDHANDLE) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardBeginTransaction(hcard : SCARDHANDLE) -> super::LONG);
    unsafe { SCardBeginTransaction(hcard) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardCancel(hcontext: SCARDCONTEXT) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardCancel(hcontext : SCARDCONTEXT) -> super::LONG);
    unsafe { SCardCancel(hcontext) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardConnectA<P1>(hcontext: SCARDCONTEXT, szreader: P1, dwsharemode: super::DWORD, dwpreferredprotocols: super::DWORD, phcard: LPSCARDHANDLE, pdwactiveprotocol: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardConnectA(hcontext : SCARDCONTEXT, szreader : windows_core::PCSTR, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, phcard : LPSCARDHANDLE, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
    unsafe { SCardConnectA(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardConnectW<P1>(hcontext: SCARDCONTEXT, szreader: P1, dwsharemode: super::DWORD, dwpreferredprotocols: super::DWORD, phcard: LPSCARDHANDLE, pdwactiveprotocol: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardConnectW(hcontext : SCARDCONTEXT, szreader : windows_core::PCWSTR, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, phcard : LPSCARDHANDLE, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
    unsafe { SCardConnectW(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardControl(hcard: SCARDHANDLE, dwcontrolcode: super::DWORD, lpinbuffer: super::LPCVOID, cbinbuffersize: super::DWORD, lpoutbuffer: super::LPVOID, cboutbuffersize: super::DWORD, lpbytesreturned: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardControl(hcard : SCARDHANDLE, dwcontrolcode : super::DWORD, lpinbuffer : super::LPCVOID, cbinbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, cboutbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> super::LONG);
    unsafe { SCardControl(hcard, dwcontrolcode, lpinbuffer, cbinbuffersize, lpoutbuffer as _, cboutbuffersize, lpbytesreturned as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardDisconnect(hcard: SCARDHANDLE, dwdisposition: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardDisconnect(hcard : SCARDHANDLE, dwdisposition : super::DWORD) -> super::LONG);
    unsafe { SCardDisconnect(hcard, dwdisposition) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardDlgExtendedError() -> super::LONG {
    windows_core::link!("scarddlg.dll" "system" fn SCardDlgExtendedError() -> super::LONG);
    unsafe { SCardDlgExtendedError() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardEndTransaction(hcard: SCARDHANDLE, dwdisposition: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardEndTransaction(hcard : SCARDHANDLE, dwdisposition : super::DWORD) -> super::LONG);
    unsafe { SCardEndTransaction(hcard, dwdisposition) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardEstablishContext(dwscope: super::DWORD, pvreserved1: super::LPCVOID, pvreserved2: super::LPCVOID, phcontext: LPSCARDCONTEXT) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardEstablishContext(dwscope : super::DWORD, pvreserved1 : super::LPCVOID, pvreserved2 : super::LPCVOID, phcontext : LPSCARDCONTEXT) -> super::LONG);
    unsafe { SCardEstablishContext(dwscope, pvreserved1 as _, pvreserved2 as _, phcontext as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetCardTypeA<P1>(hcontext: SCARDCONTEXT, szcardname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardForgetCardTypeA(hcontext, szcardname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetCardTypeW<P1>(hcontext: SCARDCONTEXT, szcardname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardForgetCardTypeW(hcontext, szcardname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetReaderA<P1>(hcontext: SCARDCONTEXT, szreadername: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardForgetReaderA(hcontext, szreadername.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetReaderGroupA<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardForgetReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetReaderGroupW<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardForgetReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardForgetReaderW<P1>(hcontext: SCARDCONTEXT, szreadername: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardForgetReaderW(hcontext, szreadername.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: SCARDCONTEXT, pvmem: super::LPCVOID) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardFreeMemory(hcontext : SCARDCONTEXT, pvmem : super::LPCVOID) -> super::LONG);
    unsafe { SCardFreeMemory(hcontext, pvmem) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetAttrib(hcard: SCARDHANDLE, dwattrid: super::DWORD, pbattr: Option<super::LPBYTE>, pcbattrlen: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardGetAttrib(hcard : SCARDHANDLE, dwattrid : super::DWORD, pbattr : super::LPBYTE, pcbattrlen : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetAttrib(hcard, dwattrid, pbattr.unwrap_or(core::mem::zeroed()) as _, pcbattrlen as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<P1>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: super::DWORD, szprovider: *mut super::CHAR, pcchprovider: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, dwproviderid : super::DWORD, szprovider : *mut super::CHAR, pcchprovider : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, szprovider as _, pcchprovider as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<P1>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: super::DWORD, szprovider: *mut super::WCHAR, pcchprovider: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, dwproviderid : super::DWORD, szprovider : *mut super::WCHAR, pcchprovider : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, szprovider as _, pcchprovider as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pdwdevicetypeid: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, pdwdevicetypeid : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetDeviceTypeIdA(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pdwdevicetypeid: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, pdwdevicetypeid : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetDeviceTypeIdW(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetProviderIdA<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidproviderid: super::LPGUID) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetProviderIdA(hcontext : SCARDCONTEXT, szcard : windows_core::PCSTR, pguidproviderid : super::LPGUID) -> super::LONG);
    unsafe { SCardGetProviderIdA(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetProviderIdW<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidproviderid: super::LPGUID) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetProviderIdW(hcontext : SCARDCONTEXT, szcard : windows_core::PCWSTR, pguidproviderid : super::LPGUID) -> super::LONG);
    unsafe { SCardGetProviderIdW(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<P1>(hcontext: SCARDCONTEXT, szreadername: P1, szdeviceinstanceid: windows_core::PSTR, pcchdeviceinstanceid: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szdeviceinstanceid : windows_core::PSTR, pcchdeviceinstanceid : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetReaderDeviceInstanceIdA(hcontext, szreadername.param().abi(), szdeviceinstanceid, pcchdeviceinstanceid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<P1>(hcontext: SCARDCONTEXT, szreadername: P1, szdeviceinstanceid: windows_core::PWSTR, pcchdeviceinstanceid: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szdeviceinstanceid : windows_core::PWSTR, pcchdeviceinstanceid : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetReaderDeviceInstanceIdW(hcontext, szreadername.param().abi(), szdeviceinstanceid, pcchdeviceinstanceid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetReaderIconA<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pbicon: super::LPBYTE, pcbicon: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderIconA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, pbicon : super::LPBYTE, pcbicon : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetReaderIconA(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetReaderIconW<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pbicon: super::LPBYTE, pcbicon: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderIconW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, pbicon : super::LPBYTE, pcbicon : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetReaderIconW(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetStatusChangeA(hcontext: SCARDCONTEXT, dwtimeout: super::DWORD, rgreaderstates: LPSCARD_READERSTATEA, creaders: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardGetStatusChangeA(hcontext : SCARDCONTEXT, dwtimeout : super::DWORD, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardGetStatusChangeA(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetStatusChangeW(hcontext: SCARDCONTEXT, dwtimeout: super::DWORD, rgreaderstates: LPSCARD_READERSTATEW, creaders: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardGetStatusChangeW(hcontext : SCARDCONTEXT, dwtimeout : super::DWORD, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardGetStatusChangeW(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: SCARDHANDLE, pctransmitcount: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardGetTransmitCount(hcard : SCARDHANDLE, pctransmitcount : super::LPDWORD) -> super::LONG);
    unsafe { SCardGetTransmitCount(hcard, pctransmitcount as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<P1>(hcontext: SCARDCONTEXT, szcardname: P1, pguidprimaryprovider: Option<super::LPCGUID>, rgguidinterfaces: Option<super::LPCGUID>, dwinterfacecount: super::DWORD, pbatr: LPCBYTE, pbatrmask: LPCBYTE, cbatrlen: super::DWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, pguidprimaryprovider : super::LPCGUID, rgguidinterfaces : super::LPCGUID, dwinterfacecount : super::DWORD, pbatr : LPCBYTE, pbatrmask : LPCBYTE, cbatrlen : super::DWORD) -> super::LONG);
    unsafe { SCardIntroduceCardTypeA(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<P1>(hcontext: SCARDCONTEXT, szcardname: P1, pguidprimaryprovider: Option<super::LPCGUID>, rgguidinterfaces: Option<super::LPCGUID>, dwinterfacecount: super::DWORD, pbatr: LPCBYTE, pbatrmask: LPCBYTE, cbatrlen: super::DWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, pguidprimaryprovider : super::LPCGUID, rgguidinterfaces : super::LPCGUID, dwinterfacecount : super::DWORD, pbatr : LPCBYTE, pbatrmask : LPCBYTE, cbatrlen : super::DWORD) -> super::LONG);
    unsafe { SCardIntroduceCardTypeW(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardIntroduceReaderA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdevicename: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szdevicename : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardIntroduceReaderA(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardIntroduceReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardIntroduceReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardIntroduceReaderW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdevicename: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szdevicename : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardIntroduceReaderW(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: SCARDCONTEXT) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardIsValidContext(hcontext : SCARDCONTEXT) -> super::LONG);
    unsafe { SCardIsValidContext(hcontext) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListCardsA(hcontext: SCARDCONTEXT, pbatr: Option<LPCBYTE>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: *mut super::CHAR, pcchcards: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardListCardsA(hcontext : SCARDCONTEXT, pbatr : LPCBYTE, rgquidinterfaces : super::LPCGUID, cguidinterfacecount : super::DWORD, mszcards : *mut super::CHAR, pcchcards : super::LPDWORD) -> super::LONG);
    unsafe { SCardListCardsA(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgquidinterfaces.map_or(core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.map_or(0, |slice| slice.len().try_into().unwrap()), mszcards as _, pcchcards as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SCardListCardsW(hcontext: SCARDCONTEXT, pbatr: Option<LPCBYTE>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: *mut super::WCHAR, pcchcards: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardListCardsW(hcontext : SCARDCONTEXT, pbatr : LPCBYTE, rgquidinterfaces : super::LPCGUID, cguidinterfacecount : super::DWORD, mszcards : *mut super::WCHAR, pcchcards : super::LPDWORD) -> super::LONG);
    unsafe { SCardListCardsW(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgquidinterfaces.map_or(core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.map_or(0, |slice| slice.len().try_into().unwrap()), mszcards as _, pcchcards as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListInterfacesA<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidinterfaces: super::LPGUID, pcguidinterfaces: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListInterfacesA(hcontext : SCARDCONTEXT, szcard : windows_core::PCSTR, pguidinterfaces : super::LPGUID, pcguidinterfaces : super::LPDWORD) -> super::LONG);
    unsafe { SCardListInterfacesA(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListInterfacesW<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidinterfaces: super::LPGUID, pcguidinterfaces: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListInterfacesW(hcontext : SCARDCONTEXT, szcard : windows_core::PCWSTR, pguidinterfaces : super::LPGUID, pcguidinterfaces : super::LPDWORD) -> super::LONG);
    unsafe { SCardListInterfacesW(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReaderGroupsA(hcontext: SCARDCONTEXT, mszgroups: Option<windows_core::PSTR>, pcchgroups: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardListReaderGroupsA(hcontext : SCARDCONTEXT, mszgroups : windows_core::PSTR, pcchgroups : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReaderGroupsA(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReaderGroupsW(hcontext: SCARDCONTEXT, mszgroups: Option<windows_core::PWSTR>, pcchgroups: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardListReaderGroupsW(hcontext : SCARDCONTEXT, mszgroups : windows_core::PWSTR, pcchgroups : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReaderGroupsW(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReadersA<P1>(hcontext: SCARDCONTEXT, mszgroups: P1, mszreaders: windows_core::PSTR, pcchreaders: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersA(hcontext : SCARDCONTEXT, mszgroups : windows_core::PCSTR, mszreaders : windows_core::PSTR, pcchreaders : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReadersA(hcontext, mszgroups.param().abi(), mszreaders, pcchreaders as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReadersW<P1>(hcontext: SCARDCONTEXT, mszgroups: P1, mszreaders: windows_core::PWSTR, pcchreaders: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersW(hcontext : SCARDCONTEXT, mszgroups : windows_core::PCWSTR, mszreaders : windows_core::PWSTR, pcchreaders : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReadersW(hcontext, mszgroups.param().abi(), mszreaders, pcchreaders as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<P1>(hcontext: SCARDCONTEXT, szdeviceinstanceid: P1, mszreaders: windows_core::PSTR, pcchreaders: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdA(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_core::PCSTR, mszreaders : windows_core::PSTR, pcchreaders : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReadersWithDeviceInstanceIdA(hcontext, szdeviceinstanceid.param().abi(), mszreaders, pcchreaders as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<P1>(hcontext: SCARDCONTEXT, szdeviceinstanceid: P1, mszreaders: windows_core::PWSTR, pcchreaders: super::LPDWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdW(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_core::PCWSTR, mszreaders : windows_core::PWSTR, pcchreaders : super::LPDWORD) -> super::LONG);
    unsafe { SCardListReadersWithDeviceInstanceIdW(hcontext, szdeviceinstanceid.param().abi(), mszreaders, pcchreaders as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardLocateCardsA<P1>(hcontext: SCARDCONTEXT, mszcards: P1, rgreaderstates: LPSCARD_READERSTATEA, creaders: super::DWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsA(hcontext : SCARDCONTEXT, mszcards : windows_core::PCSTR, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardLocateCardsA(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardLocateCardsByATRA(hcontext: SCARDCONTEXT, rgatrmasks: LPSCARD_ATRMASK, catrs: super::DWORD, rgreaderstates: LPSCARD_READERSTATEA, creaders: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsByATRA(hcontext : SCARDCONTEXT, rgatrmasks : LPSCARD_ATRMASK, catrs : super::DWORD, rgreaderstates : LPSCARD_READERSTATEA, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardLocateCardsByATRA(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardLocateCardsByATRW(hcontext: SCARDCONTEXT, rgatrmasks: LPSCARD_ATRMASK, catrs: super::DWORD, rgreaderstates: LPSCARD_READERSTATEW, creaders: super::DWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsByATRW(hcontext : SCARDCONTEXT, rgatrmasks : LPSCARD_ATRMASK, catrs : super::DWORD, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardLocateCardsByATRW(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardLocateCardsW<P1>(hcontext: SCARDCONTEXT, mszcards: P1, rgreaderstates: LPSCARD_READERSTATEW, creaders: super::DWORD) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsW(hcontext : SCARDCONTEXT, mszcards : windows_core::PCWSTR, rgreaderstates : LPSCARD_READERSTATEW, creaders : super::DWORD) -> super::LONG);
    unsafe { SCardLocateCardsW(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
#[inline]
pub unsafe fn SCardReadCacheA<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const super::UUID, freshnesscounter: super::DWORD, lookupname: P3, data: super::PBYTE, datalen: *mut super::DWORD) -> super::LONG
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardReadCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_core::PCSTR, data : super::PBYTE, datalen : *mut super::DWORD) -> super::LONG);
    unsafe { SCardReadCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
#[inline]
pub unsafe fn SCardReadCacheW<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const super::UUID, freshnesscounter: super::DWORD, lookupname: P3, data: super::PBYTE, datalen: *mut super::DWORD) -> super::LONG
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardReadCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_core::PCWSTR, data : super::PBYTE, datalen : *mut super::DWORD) -> super::LONG);
    unsafe { SCardReadCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardReconnect(hcard: SCARDHANDLE, dwsharemode: super::DWORD, dwpreferredprotocols: super::DWORD, dwinitialization: super::DWORD, pdwactiveprotocol: Option<super::LPDWORD>) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardReconnect(hcard : SCARDHANDLE, dwsharemode : super::DWORD, dwpreferredprotocols : super::DWORD, dwinitialization : super::DWORD, pdwactiveprotocol : super::LPDWORD) -> super::LONG);
    unsafe { SCardReconnect(hcard, dwsharemode, dwpreferredprotocols, dwinitialization, pdwactiveprotocol.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: SCARDCONTEXT) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardReleaseContext(hcontext : SCARDCONTEXT) -> super::LONG);
    unsafe { SCardReleaseContext(hcontext) }
}
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    windows_core::link!("winscard.dll" "system" fn SCardReleaseStartedEvent());
    unsafe { SCardReleaseStartedEvent() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardRemoveReaderFromGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardRemoveReaderFromGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardSetAttrib(hcard: SCARDHANDLE, dwattrid: super::DWORD, pbattr: &[u8]) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardSetAttrib(hcard : SCARDHANDLE, dwattrid : super::DWORD, pbattr : LPCBYTE, cbattrlen : super::DWORD) -> super::LONG);
    unsafe { SCardSetAttrib(hcard, dwattrid, core::mem::transmute(pbattr.as_ptr()), pbattr.len().try_into().unwrap()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<P1, P3>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: super::DWORD, szprovider: P3) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, dwproviderid : super::DWORD, szprovider : windows_core::PCSTR) -> super::LONG);
    unsafe { SCardSetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<P1, P3>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: super::DWORD, szprovider: P3) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, dwproviderid : super::DWORD, szprovider : windows_core::PCWSTR) -> super::LONG);
    unsafe { SCardSetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardState(hcard: SCARDHANDLE, pdwstate: super::LPDWORD, pdwprotocol: super::LPDWORD, pbatr: super::LPBYTE, pcbatrlen: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardState(hcard : SCARDHANDLE, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
    unsafe { SCardState(hcard, pdwstate as _, pdwprotocol as _, pbatr as _, pcbatrlen as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardStatusA(hcard: SCARDHANDLE, mszreadernames: windows_core::PSTR, pcchreaderlen: Option<super::LPDWORD>, pdwstate: Option<super::LPDWORD>, pdwprotocol: Option<super::LPDWORD>, pbatr: super::LPBYTE, pcbatrlen: Option<super::LPDWORD>) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardStatusA(hcard : SCARDHANDLE, mszreadernames : windows_core::PSTR, pcchreaderlen : super::LPDWORD, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
    unsafe { SCardStatusA(hcard, mszreadernames, pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SCardStatusW(hcard: SCARDHANDLE, mszreadernames: windows_core::PWSTR, pcchreaderlen: Option<super::LPDWORD>, pdwstate: Option<super::LPDWORD>, pdwprotocol: Option<super::LPDWORD>, pbatr: super::LPBYTE, pcbatrlen: Option<super::LPDWORD>) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardStatusW(hcard : SCARDHANDLE, mszreadernames : windows_core::PWSTR, pcchreaderlen : super::LPDWORD, pdwstate : super::LPDWORD, pdwprotocol : super::LPDWORD, pbatr : super::LPBYTE, pcbatrlen : super::LPDWORD) -> super::LONG);
    unsafe { SCardStatusW(hcard, mszreadernames, pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winsmcrd"))]
#[inline]
pub unsafe fn SCardTransmit(hcard: SCARDHANDLE, piosendpci: super::LPCSCARD_IO_REQUEST, pbsendbuffer: &[u8], piorecvpci: Option<super::LPSCARD_IO_REQUEST>, pbrecvbuffer: super::LPBYTE, pcbrecvlength: super::LPDWORD) -> super::LONG {
    windows_core::link!("winscard.dll" "system" fn SCardTransmit(hcard : SCARDHANDLE, piosendpci : super::LPCSCARD_IO_REQUEST, pbsendbuffer : LPCBYTE, cbsendlength : super::DWORD, piorecvpci : super::LPSCARD_IO_REQUEST, pbrecvbuffer : super::LPBYTE, pcbrecvlength : super::LPDWORD) -> super::LONG);
    unsafe { SCardTransmit(hcard, piosendpci, core::mem::transmute(pbsendbuffer.as_ptr()), pbsendbuffer.len().try_into().unwrap(), piorecvpci.unwrap_or(core::mem::zeroed()) as _, pbrecvbuffer as _, pcbrecvlength as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: LPOPENCARDNAME_EXA) -> super::LONG {
    windows_core::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardA(param0 : LPOPENCARDNAME_EXA) -> super::LONG);
    unsafe { SCardUIDlgSelectCardA(param0 as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: LPOPENCARDNAME_EXW) -> super::LONG {
    windows_core::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardW(param0 : LPOPENCARDNAME_EXW) -> super::LONG);
    unsafe { SCardUIDlgSelectCardW(param0 as _) }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
#[inline]
pub unsafe fn SCardWriteCacheA<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const super::UUID, freshnesscounter: super::DWORD, lookupname: P3, data: &[u8]) -> super::LONG
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardWriteCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_core::PCSTR, data : super::PBYTE, datalen : super::DWORD) -> super::LONG);
    unsafe { SCardWriteCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
#[inline]
pub unsafe fn SCardWriteCacheW<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const super::UUID, freshnesscounter: super::DWORD, lookupname: P3, data: &[u8]) -> super::LONG
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardWriteCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const super::UUID, freshnesscounter : super::DWORD, lookupname : windows_core::PCWSTR, data : super::PBYTE, datalen : super::DWORD) -> super::LONG);
    unsafe { SCardWriteCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
}
pub type LPCBYTE = *const u8;
#[cfg(feature = "winnt")]
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: super::PVOID) -> SCARDHANDLE>;
#[cfg(feature = "winnt")]
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: super::PVOID) -> SCARDHANDLE>;
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
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: super::DWORD,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: super::DWORD,
    pub lpstrTitle: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: super::DWORD,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: super::DWORD,
    pub lpstrTitle: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: super::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub lpstrTitle: windows_core::PCSTR,
    pub lpstrSearchDesc: windows_core::PCSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: super::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub lpstrTitle: windows_core::PCWSTR,
    pub lpstrSearchDesc: windows_core::PCWSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: super::LPVOID,
    pub dwShareMode: super::DWORD,
    pub dwPreferredProtocols: super::DWORD,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: super::DWORD,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: super::DWORD,
    pub dwActiveProtocol: super::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type OPENCARD_SEARCH_CRITERIA = OPENCARD_SEARCH_CRITERIAA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: super::DWORD,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrCardNames: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: super::DWORD,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: super::DWORD,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: super::DWORD,
    pub lpstrCardNames: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: super::DWORD,
    pub cbSerialNumberLength: super::DWORD,
    pub dwDesiredCardModuleVersion: super::DWORD,
}
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const SCARD_ALL_READERS: windows_core::PCSTR = windows_core::s!("SCard$AllReaders\u{0}");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const SCARD_DEFAULT_READERS: windows_core::PCSTR = windows_core::s!("SCard$DefaultReaders\u{0}");
pub const SCARD_EJECT_CARD: i32 = 3;
pub const SCARD_LEAVE_CARD: i32 = 0;
pub const SCARD_LOCAL_READERS: windows_core::PCSTR = windows_core::s!("SCard$LocalReaders\u{0}");
pub const SCARD_PROVIDER_CSP: i32 = 2;
pub const SCARD_PROVIDER_KSP: i32 = 3;
pub const SCARD_PROVIDER_PRIMARY: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type SCARD_READERSTATE = SCARD_READERSTATEA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_core::PCWSTR,
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
pub const SCARD_SYSTEM_READERS: windows_core::PCSTR = windows_core::s!("SCard$SystemReaders\u{0}");
pub const SCARD_UNPOWER_CARD: i32 = 2;
pub const SCERR_NOCARDNAME: i32 = 16384;
pub const SCERR_NOGUIDS: i32 = 32768;
pub const SC_DLG_FORCE_UI: i32 = 4;
pub const SC_DLG_MINIMAL_UI: i32 = 1;
pub const SC_DLG_NO_UI: i32 = 2;
