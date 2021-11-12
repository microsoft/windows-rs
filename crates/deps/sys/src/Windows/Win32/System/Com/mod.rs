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
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
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
#[doc = "*Required features: `Win32_System_Com`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
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
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_TOEND: i32 = -1i32;
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
