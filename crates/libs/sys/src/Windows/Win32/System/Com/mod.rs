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
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn BindMoniker(pmk: IMoniker, grfopt: u32, iidresult: *const ::windows_sys::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CLSIDFromProgID(lpszprogid: ::windows_sys::core::PCWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CLSIDFromProgIDEx(lpszprogid: ::windows_sys::core::PCWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CLSIDFromString(lpsz: ::windows_sys::core::PCWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoAddRefServerProcess() -> u32;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoAllowSetForegroundWindow(punk: ::windows_sys::core::IUnknown, lpvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoAllowUnmarshalerCLSID(clsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoBuildVersion() -> u32;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCopyProxy(pproxy: ::windows_sys::core::IUnknown, ppcopy: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCreateFreeThreadedMarshaler(punkouter: ::windows_sys::core::IUnknown, ppunkmarshal: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCreateGuid(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCreateInstance(rclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCreateInstanceEx(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoCreateInstanceFromApp(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoDisconnectContext(dwtimeout: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoDisconnectObject(punk: ::windows_sys::core::IUnknown, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoFreeAllLibraries();
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoFreeUnusedLibraries();
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetCallContext(riid: *const ::windows_sys::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetClassObject(rclsid: *const ::windows_sys::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetContextToken(ptoken: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetCurrentProcess() -> u32;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetObject(pszname: ::windows_sys::core::PCWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetObjectContext(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetPSClsid(riid: *const ::windows_sys::core::GUID, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut super::super::Security::PSECURITY_DESCRIPTOR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoGetTreatAsClass(clsidold: *const ::windows_sys::core::GUID, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoImpersonateClient() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn CoInitializeSecurity(psecdesc: super::super::Security::PSECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoInstall(pbc: IBindCtx, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoInvalidateRemoteMachineBindings(pszmachinename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsHandlerConnected(punk: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsOle1Class(rclsid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLoadLibrary(lpszlibname: ::windows_sys::core::PCWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLockObjectExternal(punk: ::windows_sys::core::IUnknown, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_sys::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoQueryProxyBlanket(pproxy: ::windows_sys::core::IUnknown, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_sys::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterActivationFilter(pactivationfilter: IActivationFilter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterChannelHook(extensionuuid: *const ::windows_sys::core::GUID, pchannelhook: IChannelHook) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterClassObject(rclsid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, flags: REGCLS, lpdwregister: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterDeviceCatalog(deviceinstanceid: ::windows_sys::core::PCWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterInitializeSpy(pspy: IInitializeSpy, pulicookie: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterMallocSpy(pmallocspy: IMallocSpy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterPSClsid(riid: *const ::windows_sys::core::GUID, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRegisterSurrogate(psurrogate: ISurrogate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoReleaseServerProcess() -> u32;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoResumeClassObjects() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRevertToSelf() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRevokeClassObject(dwregister: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoRevokeMallocSpy() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoSetCancelObject(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoSetProxyBlanket(pproxy: ::windows_sys::core::IUnknown, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows_sys::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoSuspendClassObjects() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoSwitchCallContext(pnewobject: ::windows_sys::core::IUnknown, ppoldobject: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoTestCancel() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoTreatAsClass(clsidold: *const ::windows_sys::core::GUID, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CoUninitialize();
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateAntiMoniker(ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateBindCtx(reserved: u32, ppbc: *mut IBindCtx) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateClassMoniker(rclsid: *const ::windows_sys::core::GUID, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateDataAdviseHolder(ppdaholder: *mut IDataAdviseHolder) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateDataCache(punkouter: ::windows_sys::core::IUnknown, rclsid: *const ::windows_sys::core::GUID, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateFileMoniker(lpszpathname: ::windows_sys::core::PCWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateGenericComposite(pmkfirst: IMoniker, pmkrest: IMoniker, ppmkcomposite: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateIUriBuilder(piuri: IUri, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut IUriBuilder) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateItemMoniker(lpszdelim: ::windows_sys::core::PCWSTR, lpszitem: ::windows_sys::core::PCWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateObjrefMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreatePointerMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: ::windows_sys::core::PCWSTR, pibsccaller: IBindStatusCallback, ppibsc: *mut IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateUri(pwzuri: ::windows_sys::core::PCWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateUriFromMultiByteString(pszansiinputuri: ::windows_sys::core::PCSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn CreateUriWithFragment(pwzuri: ::windows_sys::core::PCWSTR, pwzfragment: ::windows_sys::core::PCWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn GetClassFile(szfilename: ::windows_sys::core::PCWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut IErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn GetRunningObjectTable(reserved: u32, pprot: *mut IRunningObjectTable) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn IIDFromString(lpsz: ::windows_sys::core::PCWSTR, lpiid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn MkParseDisplayName(pbc: IBindCtx, szusername: ::windows_sys::core::PCWSTR, pcheaten: *mut u32, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn MonikerCommonPrefixWith(pmkthis: IMoniker, pmkother: IMoniker, ppmkcommon: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonikerRelativePathTo(pmksrc: IMoniker, pmkdest: IMoniker, ppmkrelpath: *mut IMoniker, dwreserved: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn ProgIDFromCLSID(clsid: *const ::windows_sys::core::GUID, lplpszprogid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn SetErrorInfo(dwreserved: u32, perrinfo: IErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn StringFromCLSID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn StringFromGUID2(rguid: *const ::windows_sys::core::GUID, lpsz: ::windows_sys::core::PWSTR, cchmax: i32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub fn StringFromIID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type ADVF = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_NODATA: ADVF = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_PRIMEFIRST: ADVF = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_ONLYONCE: ADVF = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_DATAONSTOP: ADVF = 64i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_NOHANDLER: ADVF = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_ONSAVE: ADVF = 32i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type APTTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_CURRENT: APTTYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_STA: APTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MTA: APTTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_NA: APTTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MAINSTA: APTTYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type APTTYPEQUALIFIER = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type ApplicationType = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ServerApplication: ApplicationType = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LibraryApplication: ApplicationType = 1i32;
pub type AsyncIAdviseSink = *mut ::core::ffi::c_void;
pub type AsyncIAdviseSink2 = *mut ::core::ffi::c_void;
pub type AsyncIMultiQI = *mut ::core::ffi::c_void;
pub type AsyncIPipeByte = *mut ::core::ffi::c_void;
pub type AsyncIPipeDouble = *mut ::core::ffi::c_void;
pub type AsyncIPipeLong = *mut ::core::ffi::c_void;
pub type AsyncIUnknown = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: ::windows_sys::core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: ::windows_sys::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: ::windows_sys::core::GUID,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type BINDINFOF = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ITypeComp,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type BIND_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl ::core::marker::Copy for BIND_OPTS2 {}
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type CALLCONV = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FASTCALL: CALLCONV = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_CDECL: CALLCONV = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MSCPASCAL: CALLCONV = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_PASCAL: CALLCONV = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MACPASCAL: CALLCONV = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_STDCALL: CALLCONV = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FPFASTCALL: CALLCONV = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_SYSCALL: CALLCONV = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWCDECL: CALLCONV = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWPASCAL: CALLCONV = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MAX: CALLCONV = 9i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type CALLTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_NESTED: CALLTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC: CALLTYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CATEGORYINFO {
    pub catid: ::windows_sys::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type CLSCTX = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED1: CLSCTX = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED2: CLSCTX = 128u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED3: CLSCTX = 256u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED4: CLSCTX = 512u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED5: CLSCTX = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED6: CLSCTX = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ALL: CLSCTX = 23u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_SERVER: CLSCTX = 21u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: ::windows_sys::core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl ::core::marker::Copy for COAUTHINFO {}
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type COINIT = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_APARTMENTTHREADED: COINIT = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_MULTITHREADED: COINIT = 0u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type COINITBASE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINITBASE_MULTITHREADED: COINITBASE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type COMSD = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHPERMISSIONS: COMSD = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSPERMISSIONS: COMSD = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSRESTRICTIONS: COMSD = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CONNECTDATA {
    pub pUnk: ::windows_sys::core::IUnknown,
    pub dwCookie: u32,
}
impl ::core::marker::Copy for CONNECTDATA {}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for COSERVERINFO {}
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type COWAIT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16i32;
pub type CO_DEVICE_CATALOG_COOKIE = isize;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type CO_MARSHALING_CONTEXT_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483647i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483646i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483645i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483644i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483643i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483642i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483641i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483640i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483639i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483638i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483637i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483636i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483635i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483634i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483633i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483632i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483631i32;
pub type CO_MTA_USAGE_COOKIE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows_sys::core::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type CWMO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DEFAULT: CWMO_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type DATADIR = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_GET: DATADIR = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_SET: DATADIR = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type DCOM_CALL_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_NONE: DCOM_CALL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type DESCKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_NONE: DESCKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_FUNCDESC: DESCKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_VARDESC: DESCKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_TYPECOMP: DESCKIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_MAX: DESCKIND = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type DVASPECT = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_CONTENT: DVASPECT = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_THUMBNAIL: DVASPECT = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_ICON: DVASPECT = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_DOCPRINT: DVASPECT = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type EOLE_AUTHENTICATION_CAPABILITIES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 32i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = 128i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 256i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = 2048i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 512i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = 1024i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = 4096i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = 8192i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = 16384i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type EXTCONN = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_STRONG: EXTCONN = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_WEAK: EXTCONN = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_CALLABLE: EXTCONN = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type FUNCKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_VIRTUAL: FUNCKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_NONVIRTUAL: FUNCKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_STATIC: FUNCKIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_DISPATCH: FUNCKIND = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type GLOBALOPT_EH_VALUES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type GLOBALOPT_PROPERTIES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type GLOBALOPT_RO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type GLOBALOPT_RPCTP_VALUES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type GLOBALOPT_UNMARSHALING_POLICY_VALUES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IActivationFilter = *mut ::core::ffi::c_void;
pub type IAddrExclusionControl = *mut ::core::ffi::c_void;
pub type IAddrTrackingControl = *mut ::core::ffi::c_void;
pub type IAdviseSink = *mut ::core::ffi::c_void;
pub type IAdviseSink2 = *mut ::core::ffi::c_void;
pub type IAgileObject = *mut ::core::ffi::c_void;
pub type IAsyncManager = *mut ::core::ffi::c_void;
pub type IAsyncRpcChannelBuffer = *mut ::core::ffi::c_void;
pub type IAuthenticate = *mut ::core::ffi::c_void;
pub type IAuthenticateEx = *mut ::core::ffi::c_void;
pub type IBindCtx = *mut ::core::ffi::c_void;
pub type IBindHost = *mut ::core::ffi::c_void;
pub type IBindStatusCallback = *mut ::core::ffi::c_void;
pub type IBindStatusCallbackEx = *mut ::core::ffi::c_void;
pub type IBinding = *mut ::core::ffi::c_void;
pub type IBlockingLock = *mut ::core::ffi::c_void;
pub type ICallFactory = *mut ::core::ffi::c_void;
pub type ICancelMethodCalls = *mut ::core::ffi::c_void;
pub type ICatInformation = *mut ::core::ffi::c_void;
pub type ICatRegister = *mut ::core::ffi::c_void;
pub type IChannelHook = *mut ::core::ffi::c_void;
pub type IClassActivator = *mut ::core::ffi::c_void;
pub type IClassFactory = *mut ::core::ffi::c_void;
pub type IClientSecurity = *mut ::core::ffi::c_void;
pub type IComThreadingInfo = *mut ::core::ffi::c_void;
pub type IConnectionPoint = *mut ::core::ffi::c_void;
pub type IConnectionPointContainer = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IContext(pub u8);
pub type IContextCallback = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IDataAdviseHolder = *mut ::core::ffi::c_void;
pub type IDataObject = *mut ::core::ffi::c_void;
pub type IDispatch = *mut ::core::ffi::c_void;
pub type IEnumCATEGORYINFO = *mut ::core::ffi::c_void;
pub type IEnumConnectionPoints = *mut ::core::ffi::c_void;
pub type IEnumConnections = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEnumContextProps(pub u8);
pub type IEnumFORMATETC = *mut ::core::ffi::c_void;
pub type IEnumGUID = *mut ::core::ffi::c_void;
pub type IEnumMoniker = *mut ::core::ffi::c_void;
pub type IEnumSTATDATA = *mut ::core::ffi::c_void;
pub type IEnumString = *mut ::core::ffi::c_void;
pub type IEnumUnknown = *mut ::core::ffi::c_void;
pub type IErrorInfo = *mut ::core::ffi::c_void;
pub type IErrorLog = *mut ::core::ffi::c_void;
pub type IExternalConnection = *mut ::core::ffi::c_void;
pub type IFastRundown = *mut ::core::ffi::c_void;
pub type IForegroundTransfer = *mut ::core::ffi::c_void;
pub type IGlobalInterfaceTable = *mut ::core::ffi::c_void;
pub type IGlobalOptions = *mut ::core::ffi::c_void;
pub type IInitializeSpy = *mut ::core::ffi::c_void;
pub type IInternalUnknown = *mut ::core::ffi::c_void;
pub type IMachineGlobalObjectTable = *mut ::core::ffi::c_void;
pub type IMalloc = *mut ::core::ffi::c_void;
pub type IMallocSpy = *mut ::core::ffi::c_void;
pub type IMoniker = *mut ::core::ffi::c_void;
pub type IMultiQI = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct INTERFACEINFO {
    pub pUnk: ::windows_sys::core::IUnknown,
    pub iid: ::windows_sys::core::GUID,
    pub wMethod: u16,
}
impl ::core::marker::Copy for INTERFACEINFO {}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type INVOKEKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_FUNC: INVOKEKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8i32;
pub type INoMarshal = *mut ::core::ffi::c_void;
pub type IOplockStorage = *mut ::core::ffi::c_void;
pub type IPSFactoryBuffer = *mut ::core::ffi::c_void;
pub type IPersist = *mut ::core::ffi::c_void;
pub type IPersistFile = *mut ::core::ffi::c_void;
pub type IPersistMemory = *mut ::core::ffi::c_void;
pub type IPersistStream = *mut ::core::ffi::c_void;
pub type IPersistStreamInit = *mut ::core::ffi::c_void;
pub type IPipeByte = *mut ::core::ffi::c_void;
pub type IPipeDouble = *mut ::core::ffi::c_void;
pub type IPipeLong = *mut ::core::ffi::c_void;
pub type IProcessInitControl = *mut ::core::ffi::c_void;
pub type IProcessLock = *mut ::core::ffi::c_void;
pub type IProgressNotify = *mut ::core::ffi::c_void;
pub type IROTData = *mut ::core::ffi::c_void;
pub type IReleaseMarshalBuffers = *mut ::core::ffi::c_void;
pub type IRpcChannelBuffer = *mut ::core::ffi::c_void;
pub type IRpcChannelBuffer2 = *mut ::core::ffi::c_void;
pub type IRpcChannelBuffer3 = *mut ::core::ffi::c_void;
pub type IRpcHelper = *mut ::core::ffi::c_void;
pub type IRpcOptions = *mut ::core::ffi::c_void;
pub type IRpcProxyBuffer = *mut ::core::ffi::c_void;
pub type IRpcStubBuffer = *mut ::core::ffi::c_void;
pub type IRpcSyntaxNegotiate = *mut ::core::ffi::c_void;
pub type IRunnableObject = *mut ::core::ffi::c_void;
pub type IRunningObjectTable = *mut ::core::ffi::c_void;
pub type ISequentialStream = *mut ::core::ffi::c_void;
pub type IServerSecurity = *mut ::core::ffi::c_void;
pub type IServiceProvider = *mut ::core::ffi::c_void;
pub type IStdMarshalInfo = *mut ::core::ffi::c_void;
pub type IStream = *mut ::core::ffi::c_void;
pub type ISupportErrorInfo = *mut ::core::ffi::c_void;
pub type ISurrogate = *mut ::core::ffi::c_void;
pub type ISurrogateService = *mut ::core::ffi::c_void;
pub type ISynchronize = *mut ::core::ffi::c_void;
pub type ISynchronizeContainer = *mut ::core::ffi::c_void;
pub type ISynchronizeEvent = *mut ::core::ffi::c_void;
pub type ISynchronizeHandle = *mut ::core::ffi::c_void;
pub type ISynchronizeMutex = *mut ::core::ffi::c_void;
pub type ITimeAndNoticeControl = *mut ::core::ffi::c_void;
pub type ITypeComp = *mut ::core::ffi::c_void;
pub type ITypeInfo = *mut ::core::ffi::c_void;
pub type ITypeInfo2 = *mut ::core::ffi::c_void;
pub type ITypeLib = *mut ::core::ffi::c_void;
pub type ITypeLib2 = *mut ::core::ffi::c_void;
pub type ITypeLibRegistration = *mut ::core::ffi::c_void;
pub type ITypeLibRegistrationReader = *mut ::core::ffi::c_void;
pub type IUri = *mut ::core::ffi::c_void;
pub type IUriBuilder = *mut ::core::ffi::c_void;
pub type IUrlMon = *mut ::core::ffi::c_void;
pub type IWaitMultiple = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl ::core::marker::Copy for LONG_SIZEDARR {}
impl ::core::clone::Clone for LONG_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = ::core::option::Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNCANUNLOADNOW = ::core::option::Option<unsafe extern "system" fn() -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNGETCLASSOBJECT = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows_sys::core::GUID, param1: *const ::windows_sys::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type MEMCTX = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_TASK: MEMCTX = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SHARED: MEMCTX = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_MACSYSTEM: MEMCTX = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_UNKNOWN: MEMCTX = -1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SAME: MEMCTX = -2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type MKREDUCE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ONE: MKREDUCE = 196608i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_TOUSER: MKREDUCE = 131072i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = 65536i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ALL: MKREDUCE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type MKSYS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_NONE: MKSYS = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_FILEMONIKER: MKSYS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ANTIMONIKER: MKSYS = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ITEMMONIKER: MKSYS = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_POINTERMONIKER: MKSYS = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_CLASSMONIKER: MKSYS = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_OBJREFMONIKER: MKSYS = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_SESSIONMONIKER: MKSYS = 9i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_LUAMONIKER: MKSYS = 10i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type MSHCTX = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_LOCAL: MSHCTX = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_NOSHAREDMEM: MSHCTX = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_INPROC: MSHCTX = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CROSSCTX: MSHCTX = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CONTAINER: MSHCTX = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type MSHLFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NOPING: MSHLFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = 64i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MULTI_QI {
    pub pIID: *const ::windows_sys::core::GUID,
    pub pItf: ::windows_sys::core::IUnknown,
    pub hr: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for MULTI_QI {}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type PENDINGMSG = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type PENDINGTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type PFNCONTEXTCALL = ::core::option::Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type REGCLS = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SINGLEUSE: REGCLS = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTIPLEUSE: REGCLS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SUSPENDED: REGCLS = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SURROGATE: REGCLS = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_AGILE: REGCLS = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type RPCOPT_PROPERTIES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type RPCOPT_SERVER_LOCALITY_VALUES = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type RPC_C_AUTHN_LEVEL = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = 3u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = 5u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = 6u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type RPC_C_IMP_LEVEL = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = 3u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows_sys::core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows_sys::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type SERVERCALL = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_REJECTED: SERVERCALL = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl ::core::marker::Copy for SHORT_SIZEDARR {}
impl ::core::clone::Clone for SHORT_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: ::windows_sys::core::PWSTR,
    pub hr: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: IAdviseSink,
    pub dwConnection: u32,
}
impl ::core::marker::Copy for STATDATA {}
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: ::windows_sys::core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows_sys::core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::windows_sys::core::IUnknown,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: ::windows_sys::core::PWSTR,
    pub pstm: IStream,
    pub pstg: StructuredStorage::IStorage,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type STGTY = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STORAGE: STGTY = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STREAM: STGTY = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_LOCKBYTES: STGTY = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_PROPERTY: STGTY = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_TOEND: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type STREAM_SEEK = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_SET: STREAM_SEEK = 0u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_CUR: STREAM_SEEK = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_END: STREAM_SEEK = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type SYSKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN16: SYSKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN32: SYSKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_MAC: SYSKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN64: SYSKIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type ShutdownType = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IdleShutdown: ShutdownType = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ForcedShutdown: ShutdownType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: ::windows_sys::core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl ::core::marker::Copy for StorageLayout {}
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type THDTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct TLIBATTR {
    pub guid: ::windows_sys::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type TYMED = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_HGLOBAL: TYMED = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_FILE: TYMED = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTREAM: TYMED = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTORAGE: TYMED = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_GDI: TYMED = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_MFPICT: TYMED = 32i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ENHMF: TYMED = 64i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_NULL: TYMED = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEATTR {
    pub guid: ::windows_sys::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: ::windows_sys::core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type TYPEKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ENUM: TYPEKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_RECORD: TYPEKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MODULE: TYPEKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_INTERFACE: TYPEKIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_DISPATCH: TYPEKIND = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_COCLASS: TYPEKIND = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ALIAS: TYPEKIND = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_UNION: TYPEKIND = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MAX: TYPEKIND = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type TYSPEC = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_CLSID: TYSPEC = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILEEXT: TYSPEC = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_MIMETYPE: TYSPEC = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILENAME: TYSPEC = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PROGID: TYSPEC = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PACKAGENAME: TYSPEC = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_OBJECTID: TYSPEC = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type URI_CREATE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type Uri_PROPERTY = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = 3i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = 4i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = 5i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = 6i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = 7i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = 8i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = 9i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = 10i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = 11i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = 12i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = 13i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = 14i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = 14i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = 15i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = 15i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = 16i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = 17i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = 18i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = 18i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: ::windows_sys::core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: super::super::Foundation::BSTR,
    pub punkVal: ::windows_sys::core::IUnknown,
    pub pdispVal: IDispatch,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::windows_sys::core::IUnknown,
    pub ppdispVal: *mut IDispatch,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows_sys::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: super::Ole::IRecordInfo,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type VARKIND = i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_PERINSTANCE: VARKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_STATIC: VARKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_CONST: VARKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_DISPATCH: VARKIND = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: &str = "_";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl ::core::marker::Copy for uCLSSPEC {}
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union uCLSSPEC_0 {
    pub clsid: ::windows_sys::core::GUID,
    pub pFileExt: ::windows_sys::core::PWSTR,
    pub pMimeType: ::windows_sys::core::PWSTR,
    pub pProgId: ::windows_sys::core::PWSTR,
    pub pFileName: ::windows_sys::core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl ::core::marker::Copy for uCLSSPEC_0 {}
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: ::windows_sys::core::PWSTR,
    pub PolicyId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows_sys::core::GUID,
    pub PolicyId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::marker::Copy for userFLAG_STGMEDIUM {}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for userSTGMEDIUM {}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: ::windows_sys::core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
