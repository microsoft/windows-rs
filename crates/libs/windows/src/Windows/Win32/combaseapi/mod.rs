#[inline]
pub unsafe fn CLSIDFromProgID<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CLSIDFromProgIDEx<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CLSIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> u32);
    unsafe { CoAddRefServerProcess() }
}
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoAllowUnmarshalerCLSID(clsid) }
}
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : u32, ultimeout : u32) -> windows_core::HRESULT);
    unsafe { CoCancelCall(dwthreadid, ultimeout) }
}
#[inline]
pub unsafe fn CoCopyProxy<P0>(pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : *mut core::ffi::c_void, ppcopy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCopyProxy(pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<P0>(punkouter: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("combase.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : *mut core::ffi::c_void, ppunkmarshal : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoCreateGuid() -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCreateGuid(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoCreateInstance<P1, T>(rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: u32) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoCreateInstance(rclsid, punkouter.param().abi(), dwclscontext, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn CoCreateInstanceEx<P1>(clsid: *const windows_core::GUID, punkouter: P1, dwclsctx: u32, pserverinfo: Option<*const super::objidlbase::COSERVERINFO>, presults: &mut [super::objidlbase::MULTI_QI]) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, pserverinfo : *const super::objidlbase::COSERVERINFO, dwcount : u32, presults : *mut super::objidlbase::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoCreateInstanceEx(clsid, punkouter.param().abi(), dwclsctx, pserverinfo.unwrap_or(core::mem::zeroed()) as _, presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<P1>(clsid: *const windows_core::GUID, punkouter: P1, dwclsctx: u32, reserved: Option<*const core::ffi::c_void>, presults: &mut [super::objidlbase::MULTI_QI]) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, reserved : *const core::ffi::c_void, dwcount : u32, presults : *mut super::objidlbase::MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoCreateInstanceFromApp(clsid, punkouter.param().abi(), dwclsctx, reserved.unwrap_or(core::mem::zeroed()) as _, presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())) }
}
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> windows_core::Result<ServerInformation> {
    windows_core::link!("ole32.dll" "system" fn CoDecodeProxy(dwclientpid : u32, ui64proxyaddress : u64, pserverinformation : *mut ServerInformation) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoDecodeProxy(dwclientpid, ui64proxyaddress, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    unsafe { CoDecrementMTAUsage(cookie) }
}
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoDisableCallCancellation(preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : u32) -> windows_core::HRESULT);
    unsafe { CoDisconnectContext(dwtimeout) }
}
#[inline]
pub unsafe fn CoDisconnectObject<P0>(punk: P0, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoDisconnectObject(punk : *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoDisconnectObject(punk.param().abi(), dwreserved) }
}
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoEnableCallCancellation(preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn CoFileTimeNow() -> windows_core::Result<super::minwindef::FILETIME> {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::minwindef::FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoFileTimeNow(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    windows_core::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
    unsafe { CoFreeUnusedLibraries() }
}
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    windows_core::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : u32, dwreserved : u32));
    unsafe { CoFreeUnusedLibrariesEx(dwunloaddelay, dwreserved) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut super::objidlbase::APTTYPE, paptqualifier: *mut super::objidlbase::APTTYPEQUALIFIER) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut super::objidlbase::APTTYPE, paptqualifier : *mut super::objidlbase::APTTYPEQUALIFIER) -> windows_core::HRESULT);
    unsafe { CoGetApartmentType(papttype as _, paptqualifier as _) }
}
#[inline]
pub unsafe fn CoGetCallContext(riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const windows_core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetCallContext(riid, ppinterface as _) }
}
#[inline]
pub unsafe fn CoGetCallerTID() -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCallerTID(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : u32, iid : *const windows_core::GUID, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetCancelObject(dwthreadid, iid, ppunk as _) }
}
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const windows_core::GUID, dwclscontext: u32, pvreserved: Option<*const core::ffi::c_void>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const windows_core::GUID, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetClassObject(rclsid, dwclscontext, pvreserved.unwrap_or(core::mem::zeroed()) as _, riid, ppv as _) }
}
#[inline]
pub unsafe fn CoGetContextToken() -> windows_core::Result<usize> {
    windows_core::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetContextToken(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCurrentLogicalThreadId(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> u32);
    unsafe { CoGetCurrentProcess() }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::objidlbase::APTTYPE, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::objidlbase::APTTYPE, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetDefaultContext(apttype, riid, ppv as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<P0, T>(pstm: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInterfaceAndReleaseStream(pstm : *mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetInterfaceAndReleaseStream(pstm.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> windows_core::Result<super::objidlbase::IMalloc> {
    windows_core::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : u32, ppmalloc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetMalloc(dwmemcontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoGetMarshalSizeMax<P2>(pulsize: *mut u32, riid: *const windows_core::GUID, punk: P2, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetMarshalSizeMax(pulsize : *mut u32, riid : *const windows_core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_core::HRESULT);
    unsafe { CoGetMarshalSizeMax(pulsize as _, riid, punk.param().abi(), dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags) }
}
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetObjectContext(riid, ppv as _) }
}
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const windows_core::GUID, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetPSClsid(riid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoGetStandardMarshal<P1>(riid: *const windows_core::GUID, punk: P1, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<super::objidlbase::IMarshal>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetStandardMarshal(riid : *const windows_core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32, ppmarshal : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetStandardMarshal(riid, punk.param().abi(), dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoGetStdMarshalEx<P0>(punkouter: P0, smexflags: u32) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetStdMarshalEx(punkouter : *mut core::ffi::c_void, smexflags : u32, ppunkinner : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetStdMarshalEx(punkouter.param().abi(), smexflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const windows_core::GUID, pclsidnew : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetTreatAsClass(clsidold, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoImpersonateClient() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_core::HRESULT);
    unsafe { CoImpersonateClient() }
}
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> windows_core::Result<CO_MTA_USAGE_COOKIE> {
    windows_core::link!("combase.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoIncrementMTAUsage(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: Option<*const core::ffi::c_void>, dwcoinit: u32) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: Option<super::winnt::PSECURITY_DESCRIPTOR>, asauthsvc: Option<&[super::objidlbase::SOLE_AUTHENTICATION_SERVICE]>, preserved1: Option<*const core::ffi::c_void>, dwauthnlevel: u32, dwimplevel: u32, pauthlist: Option<*const core::ffi::c_void>, dwcapabilities: u32, preserved3: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::winnt::PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const super::objidlbase::SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : u32, dwimplevel : u32, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInitializeSecurity(psecdesc.unwrap_or(core::mem::zeroed()) as _, asauthsvc.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(asauthsvc.map_or(core::ptr::null(), |slice| slice.as_ptr())), preserved1.unwrap_or(core::mem::zeroed()) as _, dwauthnlevel, dwimplevel, pauthlist.unwrap_or(core::mem::zeroed()) as _, dwcapabilities, preserved3.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<P0>(pszmachinename: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CoInvalidateRemoteMachineBindings(pszmachinename.param().abi()) }
}
#[inline]
pub unsafe fn CoIsHandlerConnected<P0>(punk: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CoIsHandlerConnected(punk.param().abi()) }
}
#[inline]
pub unsafe fn CoLockObjectExternal<P0>(punk: P0, flock: bool, flastunlockreleases: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : *mut core::ffi::c_void, flock : windows_core::BOOL, flastunlockreleases : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CoLockObjectExternal(punk.param().abi(), flock.into(), flastunlockreleases.into()) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoMarshalHresult<P0>(pstm: P0, hresult: windows_core::HRESULT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn CoMarshalHresult(pstm : *mut core::ffi::c_void, hresult : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { CoMarshalHresult(pstm.param().abi(), hresult) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<P1>(riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoMarshalInterThreadInterfaceInStream(riid : *const windows_core::GUID, punk : *mut core::ffi::c_void, ppstm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoMarshalInterThreadInterfaceInStream(riid, punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoMarshalInterface<P0, P2>(pstm: P0, riid: *const windows_core::GUID, punk: P2, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoMarshalInterface(pstm : *mut core::ffi::c_void, riid : *const windows_core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_core::HRESULT);
    unsafe { CoMarshalInterface(pstm.param().abi(), riid, punk.param().abi(), dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut super::objidlbase::SOLE_AUTHENTICATION_SERVICE) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut u32, asauthsvc : *mut *mut super::objidlbase::SOLE_AUTHENTICATION_SERVICE) -> windows_core::HRESULT);
    unsafe { CoQueryAuthenticationServices(pcauthsvc as _, asauthsvc as _) }
}
#[cfg(feature = "Win32_rpcdce")]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: *mut windows_core::PWSTR, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: *mut super::rpcdce::RPC_AUTHZ_HANDLE, pcapabilities: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pprivs : *mut super::rpcdce::RPC_AUTHZ_HANDLE, pcapabilities : *mut u32) -> windows_core::HRESULT);
    unsafe { CoQueryClientBlanket(pauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pprivs as _, pcapabilities.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpcdce")]
#[inline]
pub unsafe fn CoQueryProxyBlanket<P0>(pproxy: P0, pwauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: *mut windows_core::PWSTR, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pauthinfo: Option<*mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, pcapabilites: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : *mut core::ffi::c_void, pwauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pauthinfo : *mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, pcapabilites : *mut u32) -> windows_core::HRESULT);
    unsafe { CoQueryProxyBlanket(pproxy.param().abi(), pwauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pauthinfo.unwrap_or(core::mem::zeroed()) as _, pcapabilites.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoRegisterActivationFilter<P0>(pactivationfilter: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IActivationFilter>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterActivationFilter(pactivationfilter.param().abi()) }
}
#[inline]
pub unsafe fn CoRegisterClassObject<P1>(rclsid: *const windows_core::GUID, punk: P1, dwclscontext: u32, flags: u32) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_core::GUID, punk : *mut core::ffi::c_void, dwclscontext : u32, flags : u32, lpdwregister : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterClassObject(rclsid, punk.param().abi(), dwclscontext, flags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<P0>(deviceinstanceid: P0) -> windows_core::Result<CO_DEVICE_CATALOG_COOKIE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const windows_core::GUID, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const windows_core::GUID, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoRegisterPSClsid(riid, rclsid) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoRegisterSurrogate<P0>(psurrogate: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::ISurrogate>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterSurrogate(psurrogate.param().abi()) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoReleaseMarshalData<P0>(pstm: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn CoReleaseMarshalData(pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoReleaseMarshalData(pstm.param().abi()) }
}
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> u32);
    unsafe { CoReleaseServerProcess() }
}
#[inline]
pub unsafe fn CoResumeClassObjects() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_core::HRESULT);
    unsafe { CoResumeClassObjects() }
}
#[inline]
pub unsafe fn CoRevertToSelf() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_core::HRESULT);
    unsafe { CoRevertToSelf() }
}
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_core::HRESULT);
    unsafe { CoRevokeClassObject(dwregister) }
}
#[inline]
pub unsafe fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    unsafe { CoRevokeDeviceCatalog(cookie) }
}
#[inline]
pub unsafe fn CoSetCancelObject<P0>(punk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoSetCancelObject(punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoSetCancelObject(punk.param().abi()) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn CoSetProxyBlanket<P0>(pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Option<*const super::wtypesbase::OLECHAR>, dwauthnlevel: u32, dwimplevel: u32, pauthinfo: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, dwcapabilities: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : *mut core::ffi::c_void, dwauthnsvc : u32, dwauthzsvc : u32, pserverprincname : *const super::wtypesbase::OLECHAR, dwauthnlevel : u32, dwimplevel : u32, pauthinfo : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, dwcapabilities : u32) -> windows_core::HRESULT);
    unsafe { CoSetProxyBlanket(pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.unwrap_or(core::mem::zeroed()) as _, dwauthnlevel, dwimplevel, pauthinfo.unwrap_or(core::mem::zeroed()) as _, dwcapabilities) }
}
#[inline]
pub unsafe fn CoSuspendClassObjects() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_core::HRESULT);
    unsafe { CoSuspendClassObjects() }
}
#[inline]
pub unsafe fn CoSwitchCallContext<P0>(pnewobject: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : *mut core::ffi::c_void, ppoldobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoSwitchCallContext(pnewobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("combase.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemAlloc(cb) }
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: *mut core::ffi::c_void) {
    windows_core::link!("combase.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
    unsafe { CoTaskMemFree(pv as _) }
}
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: *mut core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : *mut core::ffi::c_void, cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemRealloc(pv as _, cb) }
}
#[inline]
pub unsafe fn CoTestCancel() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoTestCancel() -> windows_core::HRESULT);
    unsafe { CoTestCancel() }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoUnmarshalHresult<P0>(pstm: P0) -> windows_core::Result<windows_core::HRESULT>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn CoUnmarshalHresult(pstm : *mut core::ffi::c_void, phresult : *mut windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoUnmarshalHresult(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CoUnmarshalInterface<P0, T>(pstm: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoUnmarshalInterface(pstm : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoUnmarshalInterface(pstm.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[super::winnt::HANDLE]) -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::winnt::HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoWaitForMultipleHandles(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[super::winnt::HANDLE]) -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::winnt::HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoWaitForMultipleObjects(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateStreamOnHGlobal(hglobal: super::minwindef::HGLOBAL, fdeleteonrelease: bool) -> windows_core::Result<super::objidlbase::IStream> {
    windows_core::link!("ole32.dll" "system" fn CreateStreamOnHGlobal(hglobal : super::minwindef::HGLOBAL, fdeleteonrelease : windows_core::BOOL, ppstm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateStreamOnHGlobal(hglobal, fdeleteonrelease.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [super::propidlbase::PROPVARIANT]) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn FreePropVariantArray(cvariants : u32, rgvars : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { FreePropVariantArray(rgvars.len().try_into().unwrap(), core::mem::transmute(rgvars.as_ptr())) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetHGlobalFromStream<P0>(pstm: P0) -> windows_core::Result<super::minwindef::HGLOBAL>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn GetHGlobalFromStream(pstm : *mut core::ffi::c_void, phglobal : *mut super::minwindef::HGLOBAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetHGlobalFromStream(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn IIDFromString(lpsz : windows_core::PCWSTR, lpiid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const windows_core::GUID, lplpszprogid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ProgIDFromCLSID(clsid, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantClear(core::mem::transmute(pvar)) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut super::propidlbase::PROPVARIANT, pvarsrc: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut super::propidlbase::PROPVARIANT, pvarsrc : *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantCopy(core::mem::transmute(pvardest), core::mem::transmute(pvarsrc)) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn RoGetAgileReference<P2>(options: AgileReferenceOptions, riid: *const windows_core::GUID, punk: P2) -> windows_core::Result<super::objidlbase::IAgileReference>
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("combase.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_core::GUID, punk : *mut core::ffi::c_void, ppagilereference : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetAgileReference(options, riid, punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StringFromCLSID(rclsid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const windows_core::GUID, lpsz: &mut [u16]) -> i32 {
    windows_core::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_core::GUID, lpsz : windows_core::PWSTR, cchmax : i32) -> i32);
    unsafe { StringFromGUID2(rguid, core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StringFromIID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StringFromIID(rclsid, &mut result__).map(|| result__)
    }
}
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = 0;
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = 1;
pub type AgileReferenceOptions = i32;
pub const CLSCTX_ALL: u32 = 23;
pub const CLSCTX_INPROC: u32 = 3;
pub const CLSCTX_SERVER: u32 = 21;
pub type COINITBASE = i32;
pub const COINITBASE_MULTITHREADED: COINITBASE = 0;
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16;
pub const COM_RIGHTS_EXECUTE: u32 = 1;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4;
pub const COM_RIGHTS_RESERVED1: u32 = 32;
pub const COM_RIGHTS_RESERVED2: u32 = 64;
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2;
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0;
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8;
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16;
pub type COWAIT_FLAGS = u32;
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4;
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub *mut core::ffi::c_void);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub *mut core::ffi::c_void);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CWMO_DEFAULT: CWMO_FLAGS = 0;
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2;
pub type CWMO_FLAGS = u32;
pub const CWMO_MAX_HANDLES: u32 = 56;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PServerInformation(pub *mut ServerInformation);
impl PServerInformation {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PServerInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REGCLS = u32;
pub const REGCLS_AGILE: REGCLS = 16;
pub const REGCLS_MULTIPLEUSE: REGCLS = 1;
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2;
pub const REGCLS_SINGLEUSE: REGCLS = 0;
pub const REGCLS_SURROGATE: REGCLS = 8;
pub const REGCLS_SUSPENDED: REGCLS = 4;
pub const SMEXF_HANDLER: STDMSHLFLAGS = 2;
pub const SMEXF_SERVER: STDMSHLFLAGS = 1;
pub type STDMSHLFLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
