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
    pub fn BindMoniker(pmk: ::windows::runtime::RawPtr, grfopt: u32, iidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAddRefServerProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowSetForegroundWindow(punk: ::windows::runtime::RawPtr, lpvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoBuildVersion() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCopyProxy(pproxy: ::windows::runtime::RawPtr, ppcopy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateFreeThreadedMarshaler(punkouter: ::windows::runtime::RawPtr, ppunkmarshal: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateGuid(pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstance(rclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclscontext: CLSCTX, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoCreateInstanceEx(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstanceFromApp(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectContext(dwtimeout: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectObject(punk: ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
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
    pub fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallContext(riid: *const ::windows::runtime::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetClassObject(rclsid: *const ::windows::runtime::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetContextToken(ptoken: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetObjectContext(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetPSClsid(riid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetTreatAsClass(clsidold: *const ::windows::runtime::GUID, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoImpersonateClient() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInstall(pbc: ::windows::runtime::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsHandlerConnected(punk: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsOle1Class(rclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLockObjectExternal(punk: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryProxyBlanket(pproxy: ::windows::runtime::RawPtr, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterActivationFilter(pactivationfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterChannelHook(extensionuuid: *const ::windows::runtime::GUID, pchannelhook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterClassObject(rclsid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterInitializeSpy(pspy: ::windows::runtime::RawPtr, pulicookie: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterMallocSpy(pmallocspy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterPSClsid(riid: *const ::windows::runtime::GUID, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterSurrogate(psurrogate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoReleaseServerProcess() -> u32;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoResumeClassObjects() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevertToSelf() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeClassObject(dwregister: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeMallocSpy() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSetCancelObject(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoSetProxyBlanket(pproxy: ::windows::runtime::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSuspendClassObjects() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSwitchCallContext(pnewobject: ::windows::runtime::RawPtr, ppoldobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTestCancel() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTreatAsClass(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoUninitialize();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateAntiMoniker(ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateClassMoniker(rclsid: *const ::windows::runtime::GUID, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataAdviseHolder(ppdaholder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataCache(punkouter: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateGenericComposite(pmkfirst: ::windows::runtime::RawPtr, pmkrest: ::windows::runtime::RawPtr, ppmkcomposite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateIUriBuilder(piuri: ::windows::runtime::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateObjrefMoniker(punk: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreatePointerMoniker(punk: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: ::windows::runtime::RawPtr, ppibsc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayName(pbc: ::windows::runtime::RawPtr, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn MonikerCommonPrefixWith(pmkthis: ::windows::runtime::RawPtr, pmkother: ::windows::runtime::RawPtr, ppmkcommon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonikerRelativePathTo(pmksrc: ::windows::runtime::RawPtr, pmkdest: ::windows::runtime::RawPtr, ppmkrelpath: *mut ::windows::runtime::RawPtr, dwreserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProgIDFromCLSID(clsid: *const ::windows::runtime::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn SetErrorInfo(dwreserved: u32, perrinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromCLSID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromGUID2(rguid: *const ::windows::runtime::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromIID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
}
