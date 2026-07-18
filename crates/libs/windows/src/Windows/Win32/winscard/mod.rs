#[cfg(all(feature = "guiddef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32 {
    windows_core::link!("scarddlg.dll" "system" fn GetOpenCardNameA(param0 : *mut OPENCARDNAMEA) -> i32);
    unsafe { GetOpenCardNameA(param0 as _) }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32 {
    windows_core::link!("scarddlg.dll" "system" fn GetOpenCardNameW(param0 : *mut OPENCARDNAMEW) -> i32);
    unsafe { GetOpenCardNameW(param0 as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> super::HANDLE {
    windows_core::link!("winscard.dll" "system" fn SCardAccessStartedEvent() -> super::HANDLE);
    unsafe { SCardAccessStartedEvent() }
}
#[inline]
pub unsafe fn SCardAddReaderToGroupA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardAddReaderToGroupA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardAddReaderToGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardAddReaderToGroupW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardAddReaderToGroupW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardAddReaderToGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardAudit(hcontext: SCARDCONTEXT, dwevent: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardAudit(hcontext : SCARDCONTEXT, dwevent : u32) -> i32);
    unsafe { SCardAudit(hcontext, dwevent) }
}
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: SCARDHANDLE) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardBeginTransaction(hcard : SCARDHANDLE) -> i32);
    unsafe { SCardBeginTransaction(hcard) }
}
#[inline]
pub unsafe fn SCardCancel(hcontext: SCARDCONTEXT) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardCancel(hcontext : SCARDCONTEXT) -> i32);
    unsafe { SCardCancel(hcontext) }
}
#[inline]
pub unsafe fn SCardConnectA<P1>(hcontext: SCARDCONTEXT, szreader: P1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut SCARDHANDLE, pdwactiveprotocol: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardConnectA(hcontext : SCARDCONTEXT, szreader : windows_core::PCSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut SCARDHANDLE, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardConnectA(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[inline]
pub unsafe fn SCardConnectW<P1>(hcontext: SCARDCONTEXT, szreader: P1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut SCARDHANDLE, pdwactiveprotocol: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardConnectW(hcontext : SCARDCONTEXT, szreader : windows_core::PCWSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut SCARDHANDLE, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardConnectW(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[inline]
pub unsafe fn SCardControl(hcard: SCARDHANDLE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardControl(hcard : SCARDHANDLE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> i32);
    unsafe { SCardControl(hcard, dwcontrolcode, lpinbuffer, cbinbuffersize, lpoutbuffer as _, cboutbuffersize, lpbytesreturned as _) }
}
#[inline]
pub unsafe fn SCardDisconnect(hcard: SCARDHANDLE, dwdisposition: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardDisconnect(hcard : SCARDHANDLE, dwdisposition : u32) -> i32);
    unsafe { SCardDisconnect(hcard, dwdisposition) }
}
#[inline]
pub unsafe fn SCardDlgExtendedError() -> i32 {
    windows_core::link!("scarddlg.dll" "system" fn SCardDlgExtendedError() -> i32);
    unsafe { SCardDlgExtendedError() }
}
#[inline]
pub unsafe fn SCardEndTransaction(hcard: SCARDHANDLE, dwdisposition: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardEndTransaction(hcard : SCARDHANDLE, dwdisposition : u32) -> i32);
    unsafe { SCardEndTransaction(hcard, dwdisposition) }
}
#[inline]
pub unsafe fn SCardEstablishContext(dwscope: u32, pvreserved1: Option<*const core::ffi::c_void>, pvreserved2: Option<*const core::ffi::c_void>, phcontext: *mut SCARDCONTEXT) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardEstablishContext(dwscope : u32, pvreserved1 : *const core::ffi::c_void, pvreserved2 : *const core::ffi::c_void, phcontext : *mut SCARDCONTEXT) -> i32);
    unsafe { SCardEstablishContext(dwscope, pvreserved1.unwrap_or(core::mem::zeroed()) as _, pvreserved2.unwrap_or(core::mem::zeroed()) as _, phcontext as _) }
}
#[inline]
pub unsafe fn SCardForgetCardTypeA<P1>(hcontext: SCARDCONTEXT, szcardname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetCardTypeA(hcontext, szcardname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetCardTypeW<P1>(hcontext: SCARDCONTEXT, szcardname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetCardTypeW(hcontext, szcardname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderA<P1>(hcontext: SCARDCONTEXT, szreadername: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetReaderA(hcontext, szreadername.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderGroupA<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderGroupW<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderW<P1>(hcontext: SCARDCONTEXT, szreadername: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardForgetReaderW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetReaderW(hcontext, szreadername.param().abi()) }
}
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: SCARDCONTEXT, pvmem: *const core::ffi::c_void) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardFreeMemory(hcontext : SCARDCONTEXT, pvmem : *const core::ffi::c_void) -> i32);
    unsafe { SCardFreeMemory(hcontext, pvmem) }
}
#[inline]
pub unsafe fn SCardGetAttrib(hcard: SCARDHANDLE, dwattrid: u32, pbattr: Option<*mut u8>, pcbattrlen: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardGetAttrib(hcard : SCARDHANDLE, dwattrid : u32, pbattr : *mut u8, pcbattrlen : *mut u32) -> i32);
    unsafe { SCardGetAttrib(hcard, dwattrid, pbattr.unwrap_or(core::mem::zeroed()) as _, pcbattrlen as _) }
}
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<P1>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: u32, szprovider: *mut i8, pcchprovider: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, dwproviderid : u32, szprovider : *mut i8, pcchprovider : *mut u32) -> i32);
    unsafe { SCardGetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, szprovider as _, pcchprovider as _) }
}
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<P1>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: u32, szprovider: *mut u16, pcchprovider: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, dwproviderid : u32, szprovider : *mut u16, pcchprovider : *mut u32) -> i32);
    unsafe { SCardGetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, szprovider as _, pcchprovider as _) }
}
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pdwdevicetypeid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, pdwdevicetypeid : *mut u32) -> i32);
    unsafe { SCardGetDeviceTypeIdA(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pdwdevicetypeid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, pdwdevicetypeid : *mut u32) -> i32);
    unsafe { SCardGetDeviceTypeIdW(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[inline]
pub unsafe fn SCardGetProviderIdA<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidproviderid: *mut windows_core::GUID) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetProviderIdA(hcontext : SCARDCONTEXT, szcard : windows_core::PCSTR, pguidproviderid : *mut windows_core::GUID) -> i32);
    unsafe { SCardGetProviderIdA(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[inline]
pub unsafe fn SCardGetProviderIdW<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidproviderid: *mut windows_core::GUID) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetProviderIdW(hcontext : SCARDCONTEXT, szcard : windows_core::PCWSTR, pguidproviderid : *mut windows_core::GUID) -> i32);
    unsafe { SCardGetProviderIdW(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdeviceinstanceid: P2, pcchdeviceinstanceid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szdeviceinstanceid : windows_core::PCSTR, pcchdeviceinstanceid : *mut u32) -> i32);
    unsafe { SCardGetReaderDeviceInstanceIdA(hcontext, szreadername.param().abi(), szdeviceinstanceid.param().abi(), pcchdeviceinstanceid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdeviceinstanceid: P2, pcchdeviceinstanceid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szdeviceinstanceid : windows_core::PCWSTR, pcchdeviceinstanceid : *mut u32) -> i32);
    unsafe { SCardGetReaderDeviceInstanceIdW(hcontext, szreadername.param().abi(), szdeviceinstanceid.param().abi(), pcchdeviceinstanceid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderIconA<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderIconA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
    unsafe { SCardGetReaderIconA(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[inline]
pub unsafe fn SCardGetReaderIconW<P1>(hcontext: SCARDCONTEXT, szreadername: P1, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardGetReaderIconW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
    unsafe { SCardGetReaderIconW(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[inline]
pub unsafe fn SCardGetStatusChangeA(hcontext: SCARDCONTEXT, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardGetStatusChangeA(hcontext : SCARDCONTEXT, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardGetStatusChangeA(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardGetStatusChangeW(hcontext: SCARDCONTEXT, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardGetStatusChangeW(hcontext : SCARDCONTEXT, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardGetStatusChangeW(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: SCARDHANDLE, pctransmitcount: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardGetTransmitCount(hcard : SCARDHANDLE, pctransmitcount : *mut u32) -> i32);
    unsafe { SCardGetTransmitCount(hcard, pctransmitcount as _) }
}
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<P1>(hcontext: SCARDCONTEXT, szcardname: P1, pguidprimaryprovider: Option<*const windows_core::GUID>, rgguidinterfaces: Option<*const windows_core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceCardTypeA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, pguidprimaryprovider : *const windows_core::GUID, rgguidinterfaces : *const windows_core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
    unsafe { SCardIntroduceCardTypeA(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<P1>(hcontext: SCARDCONTEXT, szcardname: P1, pguidprimaryprovider: Option<*const windows_core::GUID>, rgguidinterfaces: Option<*const windows_core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceCardTypeW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, pguidprimaryprovider : *const windows_core::GUID, rgguidinterfaces : *const windows_core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
    unsafe { SCardIntroduceCardTypeW(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdevicename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szdevicename : windows_core::PCSTR) -> i32);
    unsafe { SCardIntroduceReaderA(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupA(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardIntroduceReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<P1>(hcontext: SCARDCONTEXT, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupW(hcontext : SCARDCONTEXT, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardIntroduceReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szdevicename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardIntroduceReaderW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szdevicename : windows_core::PCWSTR) -> i32);
    unsafe { SCardIntroduceReaderW(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: SCARDCONTEXT) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardIsValidContext(hcontext : SCARDCONTEXT) -> i32);
    unsafe { SCardIsValidContext(hcontext) }
}
#[inline]
pub unsafe fn SCardListCardsA(hcontext: SCARDCONTEXT, pbatr: Option<*const u8>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: *mut i8, pcchcards: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardListCardsA(hcontext : SCARDCONTEXT, pbatr : *const u8, rgquidinterfaces : *const windows_core::GUID, cguidinterfacecount : u32, mszcards : *mut i8, pcchcards : *mut u32) -> i32);
    unsafe { SCardListCardsA(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, rgquidinterfaces.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgquidinterfaces.map_or(0, |slice| slice.len().try_into().unwrap()), mszcards as _, pcchcards as _) }
}
#[inline]
pub unsafe fn SCardListCardsW(hcontext: SCARDCONTEXT, pbatr: Option<*const u8>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: *mut u16, pcchcards: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardListCardsW(hcontext : SCARDCONTEXT, pbatr : *const u8, rgquidinterfaces : *const windows_core::GUID, cguidinterfacecount : u32, mszcards : *mut u16, pcchcards : *mut u32) -> i32);
    unsafe { SCardListCardsW(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, rgquidinterfaces.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgquidinterfaces.map_or(0, |slice| slice.len().try_into().unwrap()), mszcards as _, pcchcards as _) }
}
#[inline]
pub unsafe fn SCardListInterfacesA<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidinterfaces: *mut windows_core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListInterfacesA(hcontext : SCARDCONTEXT, szcard : windows_core::PCSTR, pguidinterfaces : *mut windows_core::GUID, pcguidinterfaces : *mut u32) -> i32);
    unsafe { SCardListInterfacesA(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[inline]
pub unsafe fn SCardListInterfacesW<P1>(hcontext: SCARDCONTEXT, szcard: P1, pguidinterfaces: *mut windows_core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListInterfacesW(hcontext : SCARDCONTEXT, szcard : windows_core::PCWSTR, pguidinterfaces : *mut windows_core::GUID, pcguidinterfaces : *mut u32) -> i32);
    unsafe { SCardListInterfacesW(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[inline]
pub unsafe fn SCardListReaderGroupsA(hcontext: SCARDCONTEXT, mszgroups: Option<windows_core::PSTR>, pcchgroups: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardListReaderGroupsA(hcontext : SCARDCONTEXT, mszgroups : windows_core::PSTR, pcchgroups : *mut u32) -> i32);
    unsafe { SCardListReaderGroupsA(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[inline]
pub unsafe fn SCardListReaderGroupsW(hcontext: SCARDCONTEXT, mszgroups: Option<windows_core::PWSTR>, pcchgroups: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardListReaderGroupsW(hcontext : SCARDCONTEXT, mszgroups : windows_core::PWSTR, pcchgroups : *mut u32) -> i32);
    unsafe { SCardListReaderGroupsW(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[inline]
pub unsafe fn SCardListReadersA<P1, P2>(hcontext: SCARDCONTEXT, mszgroups: P1, mszreaders: P2, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersA(hcontext : SCARDCONTEXT, mszgroups : windows_core::PCSTR, mszreaders : windows_core::PCSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersA(hcontext, mszgroups.param().abi(), mszreaders.param().abi(), pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersW<P1, P2>(hcontext: SCARDCONTEXT, mszgroups: P1, mszreaders: P2, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersW(hcontext : SCARDCONTEXT, mszgroups : windows_core::PCWSTR, mszreaders : windows_core::PCWSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersW(hcontext, mszgroups.param().abi(), mszreaders.param().abi(), pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<P1, P2>(hcontext: SCARDCONTEXT, szdeviceinstanceid: P1, mszreaders: P2, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdA(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_core::PCSTR, mszreaders : windows_core::PCSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersWithDeviceInstanceIdA(hcontext, szdeviceinstanceid.param().abi(), mszreaders.param().abi(), pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<P1, P2>(hcontext: SCARDCONTEXT, szdeviceinstanceid: P1, mszreaders: P2, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdW(hcontext : SCARDCONTEXT, szdeviceinstanceid : windows_core::PCWSTR, mszreaders : windows_core::PCWSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersWithDeviceInstanceIdW(hcontext, szdeviceinstanceid.param().abi(), mszreaders.param().abi(), pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardLocateCardsA<P1>(hcontext: SCARDCONTEXT, mszcards: P1, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsA(hcontext : SCARDCONTEXT, mszcards : windows_core::PCSTR, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardLocateCardsA(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsByATRA(hcontext: SCARDCONTEXT, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsByATRA(hcontext : SCARDCONTEXT, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardLocateCardsByATRA(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsByATRW(hcontext: SCARDCONTEXT, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsByATRW(hcontext : SCARDCONTEXT, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardLocateCardsByATRW(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsW<P1>(hcontext: SCARDCONTEXT, mszcards: P1, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardLocateCardsW(hcontext : SCARDCONTEXT, mszcards : windows_core::PCWSTR, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardLocateCardsW(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardReadCacheA<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: *mut u8, datalen: *mut u32) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardReadCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCSTR, data : *mut u8, datalen : *mut u32) -> i32);
    unsafe { SCardReadCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[inline]
pub unsafe fn SCardReadCacheW<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: *mut u8, datalen: *mut u32) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardReadCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCWSTR, data : *mut u8, datalen : *mut u32) -> i32);
    unsafe { SCardReadCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[inline]
pub unsafe fn SCardReconnect(hcard: SCARDHANDLE, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: Option<*mut u32>) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardReconnect(hcard : SCARDHANDLE, dwsharemode : u32, dwpreferredprotocols : u32, dwinitialization : u32, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardReconnect(hcard, dwsharemode, dwpreferredprotocols, dwinitialization, pdwactiveprotocol.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: SCARDCONTEXT) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardReleaseContext(hcontext : SCARDCONTEXT) -> i32);
    unsafe { SCardReleaseContext(hcontext) }
}
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    windows_core::link!("winscard.dll" "system" fn SCardReleaseStartedEvent());
    unsafe { SCardReleaseStartedEvent() }
}
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupA(hcontext : SCARDCONTEXT, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardRemoveReaderFromGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<P1, P2>(hcontext: SCARDCONTEXT, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupW(hcontext : SCARDCONTEXT, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardRemoveReaderFromGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardSetAttrib(hcard: SCARDHANDLE, dwattrid: u32, pbattr: &[u8]) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardSetAttrib(hcard : SCARDHANDLE, dwattrid : u32, pbattr : *const u8, cbattrlen : u32) -> i32);
    unsafe { SCardSetAttrib(hcard, dwattrid, pbattr.as_ptr(), pbattr.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<P1, P3>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: u32, szprovider: P3) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameA(hcontext : SCARDCONTEXT, szcardname : windows_core::PCSTR, dwproviderid : u32, szprovider : windows_core::PCSTR) -> i32);
    unsafe { SCardSetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<P1, P3>(hcontext: SCARDCONTEXT, szcardname: P1, dwproviderid: u32, szprovider: P3) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameW(hcontext : SCARDCONTEXT, szcardname : windows_core::PCWSTR, dwproviderid : u32, szprovider : windows_core::PCWSTR) -> i32);
    unsafe { SCardSetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[inline]
pub unsafe fn SCardState(hcard: SCARDHANDLE, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardState(hcard : SCARDHANDLE, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardState(hcard, pdwstate as _, pdwprotocol as _, pbatr as _, pcbatrlen as _) }
}
#[inline]
pub unsafe fn SCardStatusA<P1>(hcard: SCARDHANDLE, mszreadernames: P1, pcchreaderlen: Option<*mut u32>, pdwstate: Option<*mut u32>, pdwprotocol: Option<*mut u32>, pbatr: *mut u8, pcbatrlen: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardStatusA(hcard : SCARDHANDLE, mszreadernames : windows_core::PCSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardStatusA(hcard, mszreadernames.param().abi(), pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SCardStatusW<P1>(hcard: SCARDHANDLE, mszreadernames: P1, pcchreaderlen: Option<*mut u32>, pdwstate: Option<*mut u32>, pdwprotocol: Option<*mut u32>, pbatr: *mut u8, pcbatrlen: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardStatusW(hcard : SCARDHANDLE, mszreadernames : windows_core::PCWSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardStatusW(hcard, mszreadernames.param().abi(), pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winsmcrd")]
#[inline]
pub unsafe fn SCardTransmit(hcard: SCARDHANDLE, piosendpci: *const super::SCARD_IO_REQUEST, pbsendbuffer: &[u8], piorecvpci: Option<*mut super::SCARD_IO_REQUEST>, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32 {
    windows_core::link!("winscard.dll" "system" fn SCardTransmit(hcard : SCARDHANDLE, piosendpci : *const super::SCARD_IO_REQUEST, pbsendbuffer : *const u8, cbsendlength : u32, piorecvpci : *mut super::SCARD_IO_REQUEST, pbrecvbuffer : *mut u8, pcbrecvlength : *mut u32) -> i32);
    unsafe { SCardTransmit(hcard, piosendpci, pbsendbuffer.as_ptr(), pbsendbuffer.len().try_into().unwrap(), piorecvpci.unwrap_or(core::mem::zeroed()) as _, pbrecvbuffer as _, pcbrecvlength as _) }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32 {
    windows_core::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardA(param0 : *mut OPENCARDNAME_EXA) -> i32);
    unsafe { SCardUIDlgSelectCardA(param0 as _) }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32 {
    windows_core::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardW(param0 : *mut OPENCARDNAME_EXW) -> i32);
    unsafe { SCardUIDlgSelectCardW(param0 as _) }
}
#[inline]
pub unsafe fn SCardWriteCacheA<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: &[u8]) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardWriteCacheA(hcontext : SCARDCONTEXT, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCSTR, data : *const u8, datalen : u32) -> i32);
    unsafe { SCardWriteCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data.as_ptr(), data.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SCardWriteCacheW<P3>(hcontext: SCARDCONTEXT, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: &[u8]) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winscard.dll" "system" fn SCardWriteCacheW(hcontext : SCARDCONTEXT, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCWSTR, data : *const u8, datalen : u32) -> i32);
    unsafe { SCardWriteCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data.as_ptr(), data.len().try_into().unwrap()) }
}
pub type LPCBYTE = *const u8;
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: SCARDHANDLE, param2: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *const core::ffi::c_void) -> SCARDHANDLE>;
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: SCARDCONTEXT, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *const core::ffi::c_void) -> SCARDHANDLE>;
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCSTR,
    pub lpstrSearchDesc: windows_core::PCSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: super::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCWSTR,
    pub lpstrSearchDesc: windows_core::PCWSTR,
    pub hIcon: super::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: super::LPCGUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3;
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1;
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCARDCONTEXT(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCARDHANDLE(pub usize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_core::PCWSTR,
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
