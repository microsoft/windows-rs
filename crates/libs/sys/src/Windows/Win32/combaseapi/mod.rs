#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : super::LPCOLESTR, lpclsid : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : super::LPCOLESTR, pclsid : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> super::ULONG);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : super::DWORD, ultimeout : super::ULONG) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : *mut core::ffi::c_void, ppcopy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : super::LPUNKNOWN, ppunkmarshal : *mut super::LPUNKNOWN) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const super::IID, punkouter : super::LPUNKNOWN, dwclscontext : super::DWORD, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, pserverinfo : *const super::COSERVERINFO, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, reserved : super::PVOID, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoDecodeProxy(dwclientpid : super::DWORD, ui64proxyaddress : super::UINT64, pserverinformation : PServerInformation) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoDisconnectObject(punk : super::LPUNKNOWN, dwreserved : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : super::LPVOID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : super::DWORD, dwreserved : super::DWORD));
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut super::APTTYPE, paptqualifier : *mut super::APTTYPEQUALIFIER) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const super::IID, ppinterface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : super::DWORD, iid : *const super::IID, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const super::IID, dwclscontext : super::DWORD, pvreserved : super::LPVOID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut super::ULONG_PTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> super::DWORD);
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::APTTYPE, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetInterfaceAndReleaseStream(pstm : super::LPSTREAM, iid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : super::DWORD, ppmalloc : *mut super::LPMALLOC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetMarshalSizeMax(pulsize : *mut super::ULONG, riid : *const super::IID, punk : super::LPUNKNOWN, dwdestcontext : super::DWORD, pvdestcontext : super::LPVOID, mshlflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const super::IID, pclsid : *mut super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetStandardMarshal(riid : *const super::IID, punk : super::LPUNKNOWN, dwdestcontext : super::DWORD, pvdestcontext : super::LPVOID, mshlflags : super::DWORD, ppmarshal : *mut super::LPMARSHAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetStdMarshalEx(punkouter : super::LPUNKNOWN, smexflags : super::DWORD, ppunkinner : *mut super::LPUNKNOWN) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const super::IID, pclsidnew : super::LPCLSID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : super::LPVOID, dwcoinit : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::PSECURITY_DESCRIPTOR, cauthsvc : super::LONG, asauthsvc : *const super::SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : super::DWORD, dwimplevel : super::DWORD, pauthlist : *const core::ffi::c_void, dwcapabilities : super::DWORD, preserved3 : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : super::LPUNKNOWN) -> windows_sys::core::BOOL);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : super::LPUNKNOWN, flock : windows_sys::core::BOOL, flastunlockreleases : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoMarshalHresult(pstm : super::LPSTREAM, hresult : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoMarshalInterThreadInterfaceInStream(riid : *const super::IID, punk : super::LPUNKNOWN, ppstm : *mut super::LPSTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoMarshalInterface(pstm : super::LPSTREAM, riid : *const super::IID, punk : super::LPUNKNOWN, dwdestcontext : super::DWORD, pvdestcontext : super::LPVOID, mshlflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut super::DWORD, asauthsvc : *mut *mut super::SOLE_AUTHENTICATION_SERVICE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut super::DWORD, pauthzsvc : *mut super::DWORD, pserverprincname : *mut super::LPOLESTR, pauthnlevel : *mut super::DWORD, pimplevel : *mut super::DWORD, pprivs : *mut super::RPC_AUTHZ_HANDLE, pcapabilities : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : *mut core::ffi::c_void, pwauthnsvc : *mut super::DWORD, pauthzsvc : *mut super::DWORD, pserverprincname : *mut super::LPOLESTR, pauthnlevel : *mut super::DWORD, pimplevel : *mut super::DWORD, pauthinfo : *mut super::RPC_AUTH_IDENTITY_HANDLE, pcapabilites : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const super::IID, punk : super::LPUNKNOWN, dwclscontext : super::DWORD, flags : super::DWORD, lpdwregister : super::LPDWORD) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_sys::core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const super::IID, rclsid : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : super::LPSURROGATE) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoReleaseMarshalData(pstm : super::LPSTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> super::ULONG);
windows_link::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSetCancelObject(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : *mut core::ffi::c_void, dwauthnsvc : super::DWORD, dwauthzsvc : super::DWORD, pserverprincname : *const super::OLECHAR, dwauthnlevel : super::DWORD, dwimplevel : super::DWORD, pauthinfo : super::RPC_AUTH_IDENTITY_HANDLE, dwcapabilities : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : *mut core::ffi::c_void, ppoldobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : super::SIZE_T) -> super::LPVOID);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoTaskMemFree(pv : super::LPVOID));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : super::LPVOID, cb : super::SIZE_T) -> super::LPVOID);
windows_link::link!("ole32.dll" "system" fn CoTestCancel() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoUninitialize());
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoUnmarshalHresult(pstm : super::LPSTREAM, phresult : *mut windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn CoUnmarshalInterface(pstm : super::LPSTREAM, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : super::DWORD, dwtimeout : super::DWORD, chandles : super::ULONG, phandles : super::LPHANDLE, lpdwindex : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : super::DWORD, dwtimeout : super::DWORD, chandles : super::ULONG, phandles : *const super::HANDLE, lpdwindex : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn CreateStreamOnHGlobal(hglobal : super::HGLOBAL, fdeleteonrelease : windows_sys::core::BOOL, ppstm : *mut super::LPSTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn GetHGlobalFromStream(pstm : super::LPSTREAM, phglobal : *mut super::HGLOBAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn IIDFromString(lpsz : super::LPCOLESTR, lpiid : super::LPIID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const super::IID, lplpszprogid : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const super::IID, punk : *mut core::ffi::c_void, ppagilereference : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const super::IID, lplpsz : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_sys::core::GUID, lpsz : super::LPOLESTR, cchmax : i32) -> i32);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const super::IID, lplpsz : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = 0;
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = 1;
pub type AgileReferenceOptions = i32;
#[cfg(feature = "wtypesbase")]
pub const CLSCTX_ALL: super::CLSCTX = 23;
#[cfg(feature = "wtypesbase")]
pub const CLSCTX_INPROC: super::CLSCTX = 3;
#[cfg(feature = "wtypesbase")]
pub const CLSCTX_SERVER: super::CLSCTX = 21;
pub type COINITBASE = i32;
pub const COINITBASE_MULTITHREADED: COINITBASE = 0;
pub const COM_RIGHTS_ACTIVATE_LOCAL: i32 = 8;
pub const COM_RIGHTS_ACTIVATE_REMOTE: i32 = 16;
pub const COM_RIGHTS_EXECUTE: i32 = 1;
pub const COM_RIGHTS_EXECUTE_LOCAL: i32 = 2;
pub const COM_RIGHTS_EXECUTE_REMOTE: i32 = 4;
pub const COM_RIGHTS_RESERVED1: i32 = 32;
pub const COM_RIGHTS_RESERVED2: i32 = 64;
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2;
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0;
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8;
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16;
pub type COWAIT_FLAGS = i32;
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4;
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1;
pub type CO_DEVICE_CATALOG_COOKIE = *mut CO_DEVICE_CATALOG_COOKIE__;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct CO_DEVICE_CATALOG_COOKIE__ {
    pub unused: i32,
}
pub type CO_MTA_USAGE_COOKIE = *mut CO_MTA_USAGE_COOKIE__;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct CO_MTA_USAGE_COOKIE__ {
    pub unused: i32,
}
pub const CWMO_DEFAULT: CWMO_FLAGS = 0;
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2;
pub type CWMO_FLAGS = i32;
pub const CWMO_MAX_HANDLES: i32 = 56;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const super::IID, param1: *const super::IID, param2: *mut super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(feature = "shared")]
pub type PServerInformation = *mut ServerInformation;
pub type REGCLS = i32;
pub const REGCLS_AGILE: REGCLS = 16;
pub const REGCLS_MULTIPLEUSE: REGCLS = 1;
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2;
pub const REGCLS_SINGLEUSE: REGCLS = 0;
pub const REGCLS_SURROGATE: REGCLS = 8;
pub const REGCLS_SUSPENDED: REGCLS = 4;
pub const SMEXF_HANDLER: STDMSHLFLAGS = 2;
pub const SMEXF_SERVER: STDMSHLFLAGS = 1;
pub type STDMSHLFLAGS = i32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ServerInformation {
    pub dwServerPid: super::DWORD,
    pub dwServerTid: super::DWORD,
    pub ui64ServerAddress: super::UINT64,
}
#[cfg(target_arch = "x86")]
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("_");
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("");
