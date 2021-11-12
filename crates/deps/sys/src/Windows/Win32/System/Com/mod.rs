#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn BindMoniker(pmk: IMoniker, grfopt: u32, iidresult: *const ::windows_sys::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAddRefServerProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowSetForegroundWindow(punk: ::windows_sys::core::IUnknown, lpvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowUnmarshalerCLSID(clsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoBuildVersion() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCopyProxy(pproxy: ::windows_sys::core::IUnknown, ppcopy: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateFreeThreadedMarshaler(punkouter: ::windows_sys::core::IUnknown, ppunkmarshal: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateGuid(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstance(rclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoCreateInstanceEx(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstanceFromApp(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectContext(dwtimeout: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectObject(punk: ::windows_sys::core::IUnknown, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeAllLibraries();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeUnusedLibraries();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallContext(riid: *const ::windows_sys::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetClassObject(rclsid: *const ::windows_sys::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetContextToken(ptoken: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetObjectContext(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetPSClsid(riid: *const ::windows_sys::core::GUID, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetTreatAsClass(clsidold: *const ::windows_sys::core::GUID, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoImpersonateClient() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInstall(pbc: IBindCtx, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsHandlerConnected(punk: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsOle1Class(rclsid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLockObjectExternal(punk: ::windows_sys::core::IUnknown, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryProxyBlanket(pproxy: ::windows_sys::core::IUnknown, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterActivationFilter(pactivationfilter: IActivationFilter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterChannelHook(extensionuuid: *const ::windows_sys::core::GUID, pchannelhook: IChannelHook) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterClassObject(rclsid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterInitializeSpy(pspy: IInitializeSpy, pulicookie: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterMallocSpy(pmallocspy: IMallocSpy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterPSClsid(riid: *const ::windows_sys::core::GUID, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterSurrogate(psurrogate: ISurrogate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoReleaseServerProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoResumeClassObjects() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevertToSelf() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeClassObject(dwregister: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeMallocSpy() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSetCancelObject(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoSetProxyBlanket(pproxy: ::windows_sys::core::IUnknown, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSuspendClassObjects() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSwitchCallContext(pnewobject: ::windows_sys::core::IUnknown, ppoldobject: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTestCancel() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTreatAsClass(clsidold: *const ::windows_sys::core::GUID, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoUninitialize();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateAntiMoniker(ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateBindCtx(reserved: u32, ppbc: *mut IBindCtx) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateClassMoniker(rclsid: *const ::windows_sys::core::GUID, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataAdviseHolder(ppdaholder: *mut IDataAdviseHolder) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataCache(punkouter: ::windows_sys::core::IUnknown, rclsid: *const ::windows_sys::core::GUID, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateGenericComposite(pmkfirst: IMoniker, pmkrest: IMoniker, ppmkcomposite: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateIUriBuilder(piuri: IUri, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut IUriBuilder) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateObjrefMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreatePointerMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: IBindStatusCallback, ppibsc: *mut IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut IErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetRunningObjectTable(reserved: u32, pprot: *mut IRunningObjectTable) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayName(pbc: IBindCtx, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn MonikerCommonPrefixWith(pmkthis: IMoniker, pmkother: IMoniker, ppmkcommon: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonikerRelativePathTo(pmksrc: IMoniker, pmkdest: IMoniker, ppmkrelpath: *mut IMoniker, dwreserved: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProgIDFromCLSID(clsid: *const ::windows_sys::core::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn SetErrorInfo(dwreserved: u32, perrinfo: IErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromCLSID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromGUID2(rguid: *const ::windows_sys::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromIID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub struct ADVF(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
pub struct APTTYPE(i32);
pub struct APTTYPEQUALIFIER(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
pub struct AUTHENTICATEINFO(i32);
pub struct ApplicationType(i32);
pub struct AsyncIAdviseSink(i32);
pub struct AsyncIAdviseSink2(i32);
pub struct AsyncIMultiQI(i32);
pub struct AsyncIPipeByte(i32);
pub struct AsyncIPipeDouble(i32);
pub struct AsyncIPipeLong(i32);
pub struct AsyncIUnknown(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO(i32);
pub struct BINDINFOF(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct BINDPTR(i32);
pub struct BIND_FLAGS(i32);
pub struct BIND_OPTS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS3(i32);
pub struct BLOB(i32);
pub struct BYTE_BLOB(i32);
pub struct BYTE_SIZEDARR(i32);
pub struct CALLCONV(i32);
pub struct CALLTYPE(i32);
pub struct CATEGORYINFO(i32);
pub struct CLSCTX(i32);
pub struct COAUTHIDENTITY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COAUTHINFO(i32);
pub struct COINIT(i32);
pub struct COINITBASE(i32);
pub struct COMSD(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
pub struct CONNECTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COSERVERINFO(i32);
pub struct COWAIT_FLAGS(i32);
pub struct CO_DEVICE_CATALOG_COOKIE(i32);
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(i32);
pub struct CO_MTA_USAGE_COOKIE(i32);
pub struct CSPLATFORM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM(i32);
pub struct CWMO_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
pub struct CY(i32);
pub struct ComCallData(i32);
pub struct DATADIR(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
pub struct DCOM_CALL_STATE(i32);
pub struct DESCKIND(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
pub struct DVASPECT(i32);
pub struct DVTARGETDEVICE(i32);
pub struct DWORD_BLOB(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC(i32);
pub struct EOLE_AUTHENTICATION_CAPABILITIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO(i32);
pub struct EXTCONN(i32);
pub struct FLAGGED_BYTE_BLOB(i32);
pub struct FLAGGED_WORD_BLOB(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM(i32);
pub struct FORMATETC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct FUNCDESC(i32);
pub struct FUNCKIND(i32);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT(i32);
pub struct GLOBALOPT_EH_VALUES(i32);
pub struct GLOBALOPT_PROPERTIES(i32);
pub struct GLOBALOPT_RO_FLAGS(i32);
pub struct GLOBALOPT_RPCTP_VALUES(i32);
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(i32);
pub struct HYPER_SIZEDARR(i32);
pub struct IActivationFilter(i32);
pub struct IAddrExclusionControl(i32);
pub struct IAddrTrackingControl(i32);
pub struct IAdviseSink(i32);
pub struct IAdviseSink2(i32);
pub struct IAgileObject(i32);
pub struct IAsyncManager(i32);
pub struct IAsyncRpcChannelBuffer(i32);
pub struct IAuthenticate(i32);
pub struct IAuthenticateEx(i32);
pub struct IBindCtx(i32);
pub struct IBindHost(i32);
pub struct IBindStatusCallback(i32);
pub struct IBindStatusCallbackEx(i32);
pub struct IBinding(i32);
pub struct IBlockingLock(i32);
pub struct ICallFactory(i32);
pub struct ICancelMethodCalls(i32);
pub struct ICatInformation(i32);
pub struct ICatRegister(i32);
pub struct IChannelHook(i32);
pub struct IClassActivator(i32);
pub struct IClassFactory(i32);
pub struct IClientSecurity(i32);
pub struct IComThreadingInfo(i32);
pub struct IConnectionPoint(i32);
pub struct IConnectionPointContainer(i32);
pub struct IContext(i32);
pub struct IContextCallback(i32);
pub struct IDLDESC(i32);
pub struct IDataAdviseHolder(i32);
pub struct IDataObject(i32);
pub struct IDispatch(i32);
pub struct IEnumCATEGORYINFO(i32);
pub struct IEnumConnectionPoints(i32);
pub struct IEnumConnections(i32);
pub struct IEnumContextProps(i32);
pub struct IEnumFORMATETC(i32);
pub struct IEnumGUID(i32);
pub struct IEnumMoniker(i32);
pub struct IEnumSTATDATA(i32);
pub struct IEnumString(i32);
pub struct IEnumUnknown(i32);
pub struct IErrorInfo(i32);
pub struct IErrorLog(i32);
pub struct IExternalConnection(i32);
pub struct IFastRundown(i32);
pub struct IForegroundTransfer(i32);
pub struct IGlobalInterfaceTable(i32);
pub struct IGlobalOptions(i32);
pub struct IInitializeSpy(i32);
pub struct IInternalUnknown(i32);
pub struct IMachineGlobalObjectTable(i32);
pub struct IMalloc(i32);
pub struct IMallocSpy(i32);
pub struct IMoniker(i32);
pub struct IMultiQI(i32);
pub struct INTERFACEINFO(i32);
pub struct INVOKEKIND(i32);
pub struct INoMarshal(i32);
pub struct IOplockStorage(i32);
pub struct IPSFactoryBuffer(i32);
pub struct IPersist(i32);
pub struct IPersistFile(i32);
pub struct IPersistMemory(i32);
pub struct IPersistStream(i32);
pub struct IPersistStreamInit(i32);
pub struct IPipeByte(i32);
pub struct IPipeDouble(i32);
pub struct IPipeLong(i32);
pub struct IProcessInitControl(i32);
pub struct IProcessLock(i32);
pub struct IProgressNotify(i32);
pub struct IROTData(i32);
pub struct IReleaseMarshalBuffers(i32);
pub struct IRpcChannelBuffer(i32);
pub struct IRpcChannelBuffer2(i32);
pub struct IRpcChannelBuffer3(i32);
pub struct IRpcHelper(i32);
pub struct IRpcOptions(i32);
pub struct IRpcProxyBuffer(i32);
pub struct IRpcStubBuffer(i32);
pub struct IRpcSyntaxNegotiate(i32);
pub struct IRunnableObject(i32);
pub struct IRunningObjectTable(i32);
pub struct ISequentialStream(i32);
pub struct IServerSecurity(i32);
pub struct IServiceProvider(i32);
pub struct IStdMarshalInfo(i32);
pub struct IStream(i32);
pub struct ISupportErrorInfo(i32);
pub struct ISurrogate(i32);
pub struct ISurrogateService(i32);
pub struct ISynchronize(i32);
pub struct ISynchronizeContainer(i32);
pub struct ISynchronizeEvent(i32);
pub struct ISynchronizeHandle(i32);
pub struct ISynchronizeMutex(i32);
pub struct ITimeAndNoticeControl(i32);
pub struct ITypeComp(i32);
pub struct ITypeInfo(i32);
pub struct ITypeInfo2(i32);
pub struct ITypeLib(i32);
pub struct ITypeLib2(i32);
pub struct ITypeLibRegistration(i32);
pub struct ITypeLibRegistrationReader(i32);
pub struct IUri(i32);
pub struct IUriBuilder(i32);
pub struct IUrlMon(i32);
pub struct IWaitMultiple(i32);
pub struct LONG_SIZEDARR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LPEXCEPFINO_DEFERRED_FILLIN(i32);
pub struct LPFNCANUNLOADNOW(i32);
pub struct LPFNGETCLASSOBJECT(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
pub struct MEMCTX(i32);
pub struct MKREDUCE(i32);
pub struct MKSYS(i32);
pub struct MSHCTX(i32);
pub struct MSHLFLAGS(i32);
pub struct MULTI_QI(i32);
pub struct MachineGlobalObjectTableRegistrationToken__(i32);
pub struct PENDINGMSG(i32);
pub struct PENDINGTYPE(i32);
pub struct PFNCONTEXTCALL(i32);
pub struct QUERYCONTEXT(i32);
pub struct REGCLS(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
pub struct RPCOLEMESSAGE(i32);
pub struct RPCOPT_PROPERTIES(i32);
pub struct RPCOPT_SERVER_LOCALITY_VALUES(i32);
pub struct RPC_C_AUTHN_LEVEL(i32);
pub struct RPC_C_IMP_LEVEL(i32);
pub struct RemSTGMEDIUM(i32);
pub struct SAFEARRAY(i32);
pub struct SAFEARRAYBOUND(i32);
pub struct SChannelHookCallInfo(i32);
pub struct SERVERCALL(i32);
pub struct SHORT_SIZEDARR(i32);
pub struct SOLE_AUTHENTICATION_INFO(i32);
pub struct SOLE_AUTHENTICATION_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SOLE_AUTHENTICATION_SERVICE(i32);
pub struct STATDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM(i32);
pub struct STGTY(i32);
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_TOEND: i32 = -1i32;
pub struct STREAM_SEEK(i32);
pub struct SYSKIND(i32);
pub struct ShutdownType(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct StorageLayout(i32);
pub struct THDTYPE(i32);
pub struct TLIBATTR(i32);
pub struct TYMED(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct TYPEATTR(i32);
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC(i32);
pub struct TYPEKIND(i32);
pub struct TYSPEC(i32);
pub struct URI_CREATE_FLAGS(i32);
pub struct Uri_PROPERTY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT(i32);
pub struct VARKIND(i32);
pub struct WORD_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC(i32);
pub struct userFLAG_STGMEDIUM(i32);
pub struct userSTGMEDIUM(i32);
