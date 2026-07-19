#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn BindMoniker<P0>(pmk: P0, grfopt: u32, iidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn BindMoniker(pmk : *mut core::ffi::c_void, grfopt : u32, iidresult : *const windows_core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BindMoniker(pmk.param().abi(), grfopt, iidresult, ppvresult as _) }
}
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<P0>(punk: P0, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoAllowSetForegroundWindow(punk.param().abi(), lpvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
    unsafe { CoBuildVersion() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::FILETIME) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::FILETIME) -> windows_core::BOOL);
    unsafe { CoDosDateTimeToFileTime(ndosdate, ndostime, lpfiletime as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpdosdate : *mut u16, lpdostime : *mut u16) -> windows_core::BOOL);
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
#[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoGetInstanceFromFile<P2>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const windows_core::GUID>, punkouter: P2, dwclsctx: u32, grfmode: u32, pwszname: *const super::OLECHAR, presults: &mut [super::MULTI_QI]) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super::COSERVERINFO, pclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, grfmode : u32, pwszname : *const super::OLECHAR, dwcount : u32, presults : *mut super::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromFile(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, grfmode, pwszname, presults.len().try_into().unwrap(), presults.as_mut_ptr()) }
}
#[cfg(all(feature = "objidl", feature = "objidlbase", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<P2, P4>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const windows_core::GUID>, punkouter: P2, dwclsctx: u32, pstg: P4, presults: &mut [super::MULTI_QI]) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super::COSERVERINFO, pclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, pstg : *mut core::ffi::c_void, dwcount : u32, presults : *mut super::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromIStorage(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, pstg.param().abi(), presults.len().try_into().unwrap(), presults.as_mut_ptr()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoGetObject<P0, T>(pszname: P0, pbindoptions: Option<*const super::BIND_OPTS>) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_core::PCWSTR, pbindoptions : *const super::BIND_OPTS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetObject(pszname.param().abi(), pbindoptions.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD) -> windows_core::Result<super::PSECURITY_DESCRIPTOR> {
    windows_core::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetSystemSecurityPermissions(comsdtype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoInitialize(pvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInitialize(pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "objidl", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn CoInstall<P0, P4>(pbc: P0, dwflags: u32, pclassspec: *const super::uCLSSPEC, pquery: *const super::QUERYCONTEXT, pszcodebase: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IBindCtx>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : u32, pclassspec : *const super::uCLSSPEC, pquery : *const super::QUERYCONTEXT, pszcodebase : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CoInstall(pbc.param().abi(), dwflags, pclassspec, pquery, pszcodebase.param().abi()) }
}
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_core::GUID) -> windows_core::BOOL);
    unsafe { CoIsOle1Class(rclsid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoLoadLibrary<P0>(lpszlibname: P0, bautofree: bool) -> super::HINSTANCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : windows_core::PCWSTR, bautofree : windows_core::BOOL) -> super::HINSTANCE);
    unsafe { CoLoadLibrary(lpszlibname.param().abi(), bautofree.into()) }
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
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<P0>(pspy: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::IInitializeSpy>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoRegisterMallocSpy<P0>(pmallocspy: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMallocSpy>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterMallocSpy(pmallocspy.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoRegisterMessageFilter<P0>(lpmessagefilter: P0) -> windows_core::Result<super::IMessageFilter>
where
    P0: windows_core::Param<super::IMessageFilter>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : *mut core::ffi::c_void, lplpmessagefilter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterMessageFilter(lpmessagefilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : u64) -> windows_core::HRESULT);
    unsafe { CoRevokeInitializeSpy(ulicookie) }
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_core::HRESULT);
    unsafe { CoRevokeMallocSpy() }
}
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const windows_core::GUID, clsidnew: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_core::GUID, clsidnew : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoTreatAsClass(clsidold, clsidnew) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateAntiMoniker() -> windows_core::Result<super::IMoniker> {
    windows_core::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAntiMoniker(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> windows_core::Result<super::IBindCtx> {
    windows_core::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateBindCtx(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const windows_core::GUID) -> windows_core::Result<super::IMoniker> {
    windows_core::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_core::GUID, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateClassMoniker(rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> windows_core::Result<super::IDataAdviseHolder> {
    windows_core::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateDataCache<P0, T>(punkouter: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CreateDataCache(punkouter : *mut core::ffi::c_void, rclsid : *const windows_core::GUID, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDataCache(punkouter.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateFileMoniker<P0>(lpszpathname: P0) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : windows_core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateFileMoniker(lpszpathname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateGenericComposite<P0, P1>(pmkfirst: P0, pmkrest: P1) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : *mut core::ffi::c_void, pmkrest : *mut core::ffi::c_void, ppmkcomposite : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateGenericComposite(pmkfirst.param().abi(), pmkrest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateItemMoniker<P0, P1>(lpszdelim: P0, lpszitem: P1) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : windows_core::PCWSTR, lpszitem : windows_core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateItemMoniker(lpszdelim.param().abi(), lpszitem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateObjrefMoniker<P0>(punk: P0) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateObjrefMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreatePointerMoniker<P0>(punk: P0) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePointerMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "urlmon", feature = "windef"))]
#[inline]
pub unsafe fn CreateStdProgressIndicator<P1, P2>(hwndparent: super::HWND, psztitle: P1, pibsccaller: P2) -> windows_core::Result<super::IBindStatusCallback>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_core::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::HWND, psztitle : windows_core::PCWSTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent, psztitle.param().abi(), pibsccaller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: Option<*const core::ffi::c_void>, pulreserved: Option<*const u32>, appshr: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { DcomChannelSetHResult(pvreserved.unwrap_or(core::mem::zeroed()) as _, pulreserved.unwrap_or(core::mem::zeroed()) as _, appshr) }
}
#[inline]
pub unsafe fn GetClassFile<P0>(szfilename: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn GetClassFile(szfilename : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetClassFile(szfilename.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> windows_core::Result<super::IRunningObjectTable> {
    windows_core::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRunningObjectTable(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MkParseDisplayName<P0, P1>(pbc: P0, szusername: P1, pcheaten: *mut u32, ppmk: *mut Option<super::IMoniker>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : *mut core::ffi::c_void, szusername : windows_core::PCWSTR, pcheaten : *mut u32, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MkParseDisplayName(pbc.param().abi(), szusername.param().abi(), pcheaten as _, core::mem::transmute(ppmk)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<P0, P1>(pmkthis: P0, pmkother: P1) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : *mut core::ffi::c_void, pmkother : *mut core::ffi::c_void, ppmkcommon : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.param().abi(), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MonikerRelativePathTo<P0, P1>(pmksrc: P0, pmkdest: P1, ppmkrelpath: *mut Option<super::IMoniker>, dwreserved: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : *mut core::ffi::c_void, pmkdest : *mut core::ffi::c_void, ppmkrelpath : *mut *mut core::ffi::c_void, dwreserved : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { MonikerRelativePathTo(pmksrc.param().abi(), pmkdest.param().abi(), core::mem::transmute(ppmkrelpath), dwreserved.into()) }
}
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile(pwcsname: *const super::OLECHAR) -> windows_core::Result<super::IFillLockBytes> {
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : *const super::OLECHAR, ppflb : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgGetIFillLockBytesOnFile(pwcsname, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<P0>(pilb: P0) -> windows_core::Result<super::IFillLockBytes>
where
    P0: windows_core::Param<super::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : *mut core::ffi::c_void, ppflb : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgGetIFillLockBytesOnILockBytes(pilb.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<P0>(pflb: P0, grfmode: u32, asyncflags: u32) -> windows_core::Result<super::IStorage>
where
    P0: windows_core::Param<super::IFillLockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : *mut core::ffi::c_void, grfmode : u32, asyncflags : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenAsyncDocfileOnIFillLockBytes(pflb.param().abi(), grfmode, asyncflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgOpenLayoutDocfile(pwcsdfname: *const super::OLECHAR, grfmode: u32, reserved: u32) -> windows_core::Result<super::IStorage> {
    windows_core::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : *const super::OLECHAR, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenLayoutDocfile(pwcsdfname, grfmode, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const ASYNC_MODE_COMPATIBILITY: u32 = 1;
pub const ASYNC_MODE_DEFAULT: u32 = 0;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
pub type COMSD = i32;
pub const MARSHALINTERFACE_MIN: u32 = 500;
pub const SD_ACCESSPERMISSIONS: COMSD = 1;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2;
pub const STGTY_REPEAT: u32 = 256;
pub const STG_LAYOUT_INTERLEAVED: u32 = 1;
pub const STG_LAYOUT_SEQUENTIAL: u32 = 0;
pub const STG_TOEND: u32 = 4294967295;
