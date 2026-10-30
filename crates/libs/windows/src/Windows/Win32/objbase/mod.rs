#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn BindMoniker<P0>(pmk: P0, grfopt: super::DWORD, iidresult: *const super::IID, ppvresult: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("ole32.dll" "system" fn BindMoniker(pmk : *mut core::ffi::c_void, grfopt : super::DWORD, iidresult : *const super::IID, ppvresult : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { BindMoniker(pmk.param().abi(), grfopt, iidresult, ppvresult as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<P0>(punk: P0, lpvreserved: Option<super::LPVOID>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : super::LPVOID) -> windows_core::HRESULT);
    unsafe { CoAllowSetForegroundWindow(punk.param().abi(), lpvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CoBuildVersion() -> super::DWORD {
    windows_core::link!("ole32.dll" "system" fn CoBuildVersion() -> super::DWORD);
    unsafe { CoBuildVersion() }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: super::WORD, ndostime: super::WORD, lpfiletime: *mut super::FILETIME) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : super::WORD, ndostime : super::WORD, lpfiletime : *mut super::FILETIME) -> windows_core::BOOL);
    unsafe { CoDosDateTimeToFileTime(ndosdate, ndostime, lpfiletime as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::FILETIME, lpdosdate: super::LPWORD, lpdostime: super::LPWORD) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpdosdate : super::LPWORD, lpdostime : super::LPWORD) -> windows_core::BOOL);
    unsafe { CoFileTimeToDosDateTime(lpfiletime, lpdosdate as _, lpdostime as _) }
}
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    windows_core::link!("ole32.dll" "system" fn CoFreeAllLibraries());
    unsafe { CoFreeAllLibraries() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoFreeLibrary(hinst: super::HINSTANCE) {
    windows_core::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::HINSTANCE));
    unsafe { CoFreeLibrary(hinst) }
}
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoGetInstanceFromFile<P2>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const super::CLSID>, punkouter: P2, dwclsctx: super::DWORD, grfmode: super::DWORD, pwszname: *const super::OLECHAR, presults: &mut [super::MULTI_QI]) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super::COSERVERINFO, pclsid : *const super::CLSID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, grfmode : super::DWORD, pwszname : *const super::OLECHAR, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromFile(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, grfmode, pwszname, presults.len().try_into().unwrap(), presults.as_mut_ptr()) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<P2, P4>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const super::CLSID>, punkouter: P2, dwclsctx: super::DWORD, pstg: P4, presults: &mut [super::MULTI_QI]) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super::COSERVERINFO, pclsid : *const super::CLSID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, pstg : *mut core::ffi::c_void, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromIStorage(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, pstg.param().abi(), presults.len().try_into().unwrap(), presults.as_mut_ptr()) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn CoGetObject<P0>(pszname: P0, pbindoptions: Option<*const super::BIND_OPTS>, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_core::PCWSTR, pbindoptions : *const super::BIND_OPTS, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetObject(pszname.param().abi(), pbindoptions.unwrap_or(core::mem::zeroed()) as _, riid, ppv as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut super::PSECURITY_DESCRIPTOR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    unsafe { CoGetSystemSecurityPermissions(comsdtype, ppsd as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoInitialize(pvreserved: Option<super::LPVOID>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitialize(pvreserved : super::LPVOID) -> windows_core::HRESULT);
    unsafe { CoInitialize(pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoInstall<P0, P4>(pbc: P0, dwflags: super::DWORD, pclassspec: *const super::uCLSSPEC, pquery: *const super::QUERYCONTEXT, pszcodebase: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IBindCtx>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : super::DWORD, pclassspec : *const super::uCLSSPEC, pquery : *const super::QUERYCONTEXT, pszcodebase : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CoInstall(pbc.param().abi(), dwflags, pclassspec, pquery, pszcodebase.param().abi()) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const super::IID) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const super::IID) -> windows_core::BOOL);
    unsafe { CoIsOle1Class(rclsid) }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoLoadLibrary(lpszlibname: super::LPOLESTR, bautofree: bool) -> super::HINSTANCE {
    windows_core::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : super::LPOLESTR, bautofree : windows_core::BOOL) -> super::HINSTANCE);
    unsafe { CoLoadLibrary(lpszlibname, bautofree.into()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn CoRegisterChannelHook<P1>(extensionuuid: *const windows_core::GUID, pchannelhook: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IChannelHook>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_core::GUID, pchannelhook : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterChannelHook(extensionuuid, pchannelhook.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<P0>(pspy: P0, pulicookie: *mut super::ULARGE_INTEGER) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IInitializeSpy>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut super::ULARGE_INTEGER) -> windows_core::HRESULT);
    unsafe { CoRegisterInitializeSpy(pspy.param().abi(), pulicookie as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoRegisterMallocSpy<P0>(pmallocspy: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMALLOCSPY>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterMallocSpy(pmallocspy.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoRegisterMessageFilter<P0>(lpmessagefilter: P0, lplpmessagefilter: Option<*mut Option<super::LPMESSAGEFILTER>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMESSAGEFILTER>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : *mut core::ffi::c_void, lplpmessagefilter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterMessageFilter(lpmessagefilter.param().abi(), lplpmessagefilter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: super::ULARGE_INTEGER) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : super::ULARGE_INTEGER) -> windows_core::HRESULT);
    unsafe { CoRevokeInitializeSpy(ulicookie) }
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_core::HRESULT);
    unsafe { CoRevokeMallocSpy() }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const super::IID, clsidnew: *const super::IID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const super::IID, clsidnew : *const super::IID) -> windows_core::HRESULT);
    unsafe { CoTreatAsClass(clsidold, clsidnew) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateAntiMoniker(ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateAntiMoniker(core::mem::transmute(ppmk)) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn CreateBindCtx(reserved: super::DWORD, ppbc: *mut Option<super::LPBC>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CreateBindCtx(reserved : super::DWORD, ppbc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateBindCtx(reserved, core::mem::transmute(ppbc)) }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const super::IID, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const super::IID, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateClassMoniker(rclsid, core::mem::transmute(ppmk)) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn CreateDataCache<P0>(punkouter: P0, rclsid: *const super::IID, iid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn CreateDataCache(punkouter : *mut core::ffi::c_void, rclsid : *const super::IID, iid : *const super::IID, ppv : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { CreateDataCache(punkouter.param().abi(), rclsid, iid, ppv as _) }
}
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateFileMoniker(lpszpathname: super::LPCOLESTR, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : super::LPCOLESTR, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateFileMoniker(lpszpathname, core::mem::transmute(ppmk)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateGenericComposite<P0, P1>(pmkfirst: P0, pmkrest: P1, ppmkcomposite: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P1: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : *mut core::ffi::c_void, pmkrest : *mut core::ffi::c_void, ppmkcomposite : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateGenericComposite(pmkfirst.param().abi(), pmkrest.param().abi(), core::mem::transmute(ppmkcomposite)) }
}
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateItemMoniker(lpszdelim: super::LPCOLESTR, lpszitem: super::LPCOLESTR, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : super::LPCOLESTR, lpszitem : super::LPCOLESTR, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateItemMoniker(lpszdelim, lpszitem, core::mem::transmute(ppmk)) }
}
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
#[inline]
pub unsafe fn CreateObjrefMoniker<P0>(punk: P0, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateObjrefMoniker(punk.param().abi(), core::mem::transmute(ppmk)) }
}
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
#[inline]
pub unsafe fn CreatePointerMoniker<P0>(punk: P0, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreatePointerMoniker(punk.param().abi(), core::mem::transmute(ppmk)) }
}
#[cfg(all(feature = "urlmon", feature = "windef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateStdProgressIndicator<P2>(hwndparent: super::HWND, psztitle: super::LPCOLESTR, pibsccaller: P2, ppibsc: *mut Option<super::IBindStatusCallback>) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_core::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::HWND, psztitle : super::LPCOLESTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateStdProgressIndicator(hwndparent, psztitle, pibsccaller.param().abi(), core::mem::transmute(ppibsc)) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: Option<super::LPVOID>, pulreserved: Option<*const super::ULONG>, appshr: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : super::LPVOID, pulreserved : *const super::ULONG, appshr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { DcomChannelSetHResult(pvreserved.unwrap_or(core::mem::zeroed()) as _, pulreserved.unwrap_or(core::mem::zeroed()) as _, appshr) }
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn GetClassFile(szfilename: super::LPCOLESTR, pclsid: *mut super::CLSID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn GetClassFile(szfilename : super::LPCOLESTR, pclsid : *mut super::CLSID) -> windows_core::HRESULT);
    unsafe { GetClassFile(szfilename, pclsid as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: super::DWORD, pprot: *mut Option<super::LPRUNNINGOBJECTTABLE>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : super::DWORD, pprot : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetRunningObjectTable(reserved, core::mem::transmute(pprot)) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MkParseDisplayName<P0>(pbc: P0, szusername: super::LPCOLESTR, pcheaten: *mut super::ULONG, ppmk: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPBC>,
{
    windows_core::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : *mut core::ffi::c_void, szusername : super::LPCOLESTR, pcheaten : *mut super::ULONG, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MkParseDisplayName(pbc.param().abi(), szusername, pcheaten as _, core::mem::transmute(ppmk)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<P0, P1>(pmkthis: P0, pmkother: P1, ppmkcommon: *mut Option<super::LPMONIKER>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P1: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : *mut core::ffi::c_void, pmkother : *mut core::ffi::c_void, ppmkcommon : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MonikerCommonPrefixWith(pmkthis.param().abi(), pmkother.param().abi(), core::mem::transmute(ppmkcommon)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MonikerRelativePathTo<P0, P1>(pmksrc: P0, pmkdest: P1, ppmkrelpath: *mut Option<super::LPMONIKER>, dwreserved: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P1: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : *mut core::ffi::c_void, pmkdest : *mut core::ffi::c_void, ppmkrelpath : *mut *mut core::ffi::c_void, dwreserved : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { MonikerRelativePathTo(pmksrc.param().abi(), pmkdest.param().abi(), core::mem::transmute(ppmkrelpath), dwreserved.into()) }
}
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile(pwcsname: *const super::OLECHAR, ppflb: *mut Option<super::IFillLockBytes>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : *const super::OLECHAR, ppflb : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgGetIFillLockBytesOnFile(pwcsname, core::mem::transmute(ppflb)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<P0>(pilb: P0, ppflb: *mut Option<super::IFillLockBytes>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : *mut core::ffi::c_void, ppflb : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgGetIFillLockBytesOnILockBytes(pilb.param().abi(), core::mem::transmute(ppflb)) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<P0>(pflb: P0, grfmode: super::DWORD, asyncflags: super::DWORD, ppstgopen: *mut Option<super::IStorage>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IFillLockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : *mut core::ffi::c_void, grfmode : super::DWORD, asyncflags : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenAsyncDocfileOnIFillLockBytes(pflb.param().abi(), grfmode, asyncflags, core::mem::transmute(ppstgopen)) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgOpenLayoutDocfile(pwcsdfname: *const super::OLECHAR, grfmode: super::DWORD, reserved: super::DWORD, ppstgopen: *mut Option<super::IStorage>) -> windows_core::HRESULT {
    windows_core::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : *const super::OLECHAR, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenLayoutDocfile(pwcsdfname, grfmode, reserved, core::mem::transmute(ppstgopen)) }
}
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1;
pub const ASYNC_MODE_DEFAULT: i32 = 0;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
pub type COMSD = i32;
pub const MARSHALINTERFACE_MIN: i32 = 500;
pub const SD_ACCESSPERMISSIONS: COMSD = 1;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2;
pub const STGTY_REPEAT: i32 = 256;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0;
pub const STG_TOEND: u32 = 4294967295;
