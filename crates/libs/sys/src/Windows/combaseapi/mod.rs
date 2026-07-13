windows_link::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : windows_sys::core::PCWSTR, lpclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : windows_sys::core::PCWSTR, lpclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> u32);
windows_link::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : u32, ultimeout : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : *mut core::ffi::c_void, ppcopy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : *mut core::ffi::c_void, ppunkmarshal : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, pserverinfo : *const super::objidlbase::COSERVERINFO, dwcount : u32, presults : *mut super::objidlbase::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, reserved : *const core::ffi::c_void, dwcount : u32, presults : *mut super::objidlbase::MULTI_QI) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDecodeProxy(dwclientpid : u32, ui64proxyaddress : u64, pserverinformation : *mut ServerInformation) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisconnectObject(punk : *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::minwindef::FILETIME) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : u32, dwreserved : u32));
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut super::objidlbase::APTTYPE, paptqualifier : *mut super::objidlbase::APTTYPEQUALIFIER) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const windows_sys::core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : u32, iid : *const windows_sys::core::GUID, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const windows_sys::core::GUID, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut usize) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> u32);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::objidlbase::APTTYPE, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetInterfaceAndReleaseStream(pstm : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : u32, ppmalloc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetMarshalSizeMax(pulsize : *mut u32, riid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const windows_sys::core::GUID, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoGetStandardMarshal(riid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32, ppmarshal : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetStdMarshalEx(punkouter : *mut core::ffi::c_void, smexflags : u32, ppunkinner : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const windows_sys::core::GUID, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::winnt::PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const super::objidlbase::SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : u32, dwimplevel : u32, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : *mut core::ffi::c_void, flock : windows_sys::core::BOOL, flastunlockreleases : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoMarshalHresult(pstm : *mut core::ffi::c_void, hresult : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoMarshalInterThreadInterfaceInStream(riid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, ppstm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoMarshalInterface(pstm : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut u32, asauthsvc : *mut *mut super::objidlbase::SOLE_AUTHENTICATION_SERVICE) -> windows_sys::core::HRESULT);
#[cfg(feature = "rpc")]
windows_link::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_sys::core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pprivs : *mut super::rpc::RPC_AUTHZ_HANDLE, pcapabilities : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "rpc")]
windows_link::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : *mut core::ffi::c_void, pwauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_sys::core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pauthinfo : *mut super::rpc::RPC_AUTH_IDENTITY_HANDLE, pcapabilites : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, dwclscontext : u32, flags : u32, lpdwregister : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_sys::core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const windows_sys::core::GUID, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoReleaseMarshalData(pstm : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> u32);
windows_link::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-com-l1-1-3.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSetCancelObject(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "rpc", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : *mut core::ffi::c_void, dwauthnsvc : u32, dwauthzsvc : u32, pserverprincname : *const super::wtypesbase::OLECHAR, dwauthnlevel : u32, dwimplevel : u32, pauthinfo : super::rpc::RPC_AUTH_IDENTITY_HANDLE, dwcapabilities : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : *mut core::ffi::c_void, ppoldobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_link::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
windows_link::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : *mut core::ffi::c_void, cb : usize) -> *mut core::ffi::c_void);
windows_link::link!("ole32.dll" "system" fn CoTestCancel() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoUninitialize());
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoUnmarshalHresult(pstm : *mut core::ffi::c_void, phresult : *mut windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoUnmarshalInterface(pstm : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::winnt::HANDLE, lpdwindex : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::winnt::HANDLE, lpdwindex : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CreateStreamOnHGlobal(hglobal : super::minwindef::HGLOBAL, fdeleteonrelease : windows_sys::core::BOOL, ppstm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn FreePropVariantArray(cvariants : u32, rgvars : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn GetHGlobalFromStream(pstm : *mut core::ffi::c_void, phglobal : *mut super::minwindef::HGLOBAL) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn IIDFromString(lpsz : windows_sys::core::PCWSTR, lpiid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const windows_sys::core::GUID, lplpszprogid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut super::propidlbase::PROPVARIANT, pvarsrc : *const super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, ppagilereference : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const windows_sys::core::GUID, lplpsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_sys::core::GUID, lpsz : windows_sys::core::PWSTR, cchmax : i32) -> i32);
windows_link::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const windows_sys::core::GUID, lplpsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
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
pub type CO_DEVICE_CATALOG_COOKIE = *mut core::ffi::c_void;
pub type CO_MTA_USAGE_COOKIE = *mut core::ffi::c_void;
pub const CWMO_DEFAULT: CWMO_FLAGS = 0;
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2;
pub type CWMO_FLAGS = u32;
pub const CWMO_MAX_HANDLES: u32 = 56;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_sys::core::GUID, param1: *const windows_sys::core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type PServerInformation = *mut ServerInformation;
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
#[derive(Clone, Copy, Default)]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
