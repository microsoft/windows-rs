#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
    pub fn BindMoniker(pmk: IMoniker, grfopt: u32, iidresult: *const ::windows_sys::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoAddRefServerProcess() -> u32;
    pub fn CoAllowSetForegroundWindow(punk: ::windows_sys::core::IUnknown, lpvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoAllowUnmarshalerCLSID(clsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoBuildVersion() -> u32;
    pub fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoCopyProxy(pproxy: ::windows_sys::core::IUnknown, ppcopy: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn CoCreateFreeThreadedMarshaler(punkouter: ::windows_sys::core::IUnknown, ppunkmarshal: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn CoCreateGuid(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoCreateInstance(rclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoCreateInstanceEx(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    pub fn CoCreateInstanceFromApp(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_sys::core::HRESULT;
    pub fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoDisconnectContext(dwtimeout: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoDisconnectObject(punk: ::windows_sys::core::IUnknown, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    pub fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
    pub fn CoFreeAllLibraries();
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
    pub fn CoFreeUnusedLibraries();
    pub fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
    pub fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows_sys::core::HRESULT;
    pub fn CoGetCallContext(riid: *const ::windows_sys::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetClassObject(rclsid: *const ::windows_sys::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetContextToken(ptoken: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoGetCurrentProcess() -> u32;
    pub fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut IMalloc) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetObjectContext(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetPSClsid(riid: *const ::windows_sys::core::GUID, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows_sys::core::HRESULT;
    pub fn CoGetTreatAsClass(clsidold: *const ::windows_sys::core::GUID, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoImpersonateClient() -> ::windows_sys::core::HRESULT;
    pub fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInstall(pbc: IBindCtx, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsHandlerConnected(punk: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsOle1Class(rclsid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLockObjectExternal(punk: ::windows_sys::core::IUnknown, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryProxyBlanket(pproxy: ::windows_sys::core::IUnknown, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterActivationFilter(pactivationfilter: IActivationFilter) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterChannelHook(extensionuuid: *const ::windows_sys::core::GUID, pchannelhook: IChannelHook) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterClassObject(rclsid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterInitializeSpy(pspy: IInitializeSpy, pulicookie: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterMallocSpy(pmallocspy: IMallocSpy) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterPSClsid(riid: *const ::windows_sys::core::GUID, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterSurrogate(psurrogate: ISurrogate) -> ::windows_sys::core::HRESULT;
    pub fn CoReleaseServerProcess() -> u32;
    pub fn CoResumeClassObjects() -> ::windows_sys::core::HRESULT;
    pub fn CoRevertToSelf() -> ::windows_sys::core::HRESULT;
    pub fn CoRevokeClassObject(dwregister: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows_sys::core::HRESULT;
    pub fn CoRevokeMallocSpy() -> ::windows_sys::core::HRESULT;
    pub fn CoSetCancelObject(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoSetProxyBlanket(pproxy: ::windows_sys::core::IUnknown, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_sys::core::HRESULT;
    pub fn CoSuspendClassObjects() -> ::windows_sys::core::HRESULT;
    pub fn CoSwitchCallContext(pnewobject: ::windows_sys::core::IUnknown, ppoldobject: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    pub fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
    pub fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    pub fn CoTestCancel() -> ::windows_sys::core::HRESULT;
    pub fn CoTreatAsClass(clsidold: *const ::windows_sys::core::GUID, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn CoUninitialize();
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn CreateAntiMoniker(ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreateBindCtx(reserved: u32, ppbc: *mut IBindCtx) -> ::windows_sys::core::HRESULT;
    pub fn CreateClassMoniker(rclsid: *const ::windows_sys::core::GUID, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreateDataAdviseHolder(ppdaholder: *mut IDataAdviseHolder) -> ::windows_sys::core::HRESULT;
    pub fn CreateDataCache(punkouter: ::windows_sys::core::IUnknown, rclsid: *const ::windows_sys::core::GUID, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreateGenericComposite(pmkfirst: IMoniker, pmkrest: IMoniker, ppmkcomposite: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreateIUriBuilder(piuri: IUri, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut IUriBuilder) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreateObjrefMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn CreatePointerMoniker(punk: ::windows_sys::core::IUnknown, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: IBindStatusCallback, ppibsc: *mut IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut IUri) -> ::windows_sys::core::HRESULT;
    pub fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut IErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn GetRunningObjectTable(reserved: u32, pprot: *mut IRunningObjectTable) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayName(pbc: IBindCtx, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn MonikerCommonPrefixWith(pmkthis: IMoniker, pmkother: IMoniker, ppmkcommon: *mut IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonikerRelativePathTo(pmksrc: IMoniker, pmkdest: IMoniker, ppmkrelpath: *mut IMoniker, dwreserved: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProgIDFromCLSID(clsid: *const ::windows_sys::core::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn SetErrorInfo(dwreserved: u32, perrinfo: IErrorInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromCLSID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromGUID2(rguid: *const ::windows_sys::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromIID(rclsid: *const ::windows_sys::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub const ADVF_NODATA: i32 = 1i32;
pub const ADVF_PRIMEFIRST: i32 = 2i32;
pub const ADVF_ONLYONCE: i32 = 4i32;
pub const ADVF_DATAONSTOP: i32 = 64i32;
pub const ADVFCACHE_NOHANDLER: i32 = 8i32;
pub const ADVFCACHE_FORCEBUILTIN: i32 = 16i32;
pub const ADVFCACHE_ONSAVE: i32 = 32i32;
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
pub const APTTYPE_CURRENT: i32 = -1i32;
pub const APTTYPE_STA: i32 = 0i32;
pub const APTTYPE_MTA: i32 = 1i32;
pub const APTTYPE_NA: i32 = 2i32;
pub const APTTYPE_MAINSTA: i32 = 3i32;
pub const APTTYPEQUALIFIER_NONE: i32 = 0i32;
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: i32 = 1i32;
pub const APTTYPEQUALIFIER_NA_ON_MTA: i32 = 2i32;
pub const APTTYPEQUALIFIER_NA_ON_STA: i32 = 3i32;
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: i32 = 4i32;
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: i32 = 5i32;
pub const APTTYPEQUALIFIER_APPLICATION_STA: i32 = 6i32;
pub const APTTYPEQUALIFIER_RESERVED_1: i32 = 7i32;
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[repr(C)]
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
pub const ServerApplication: i32 = 0i32;
pub const LibraryApplication: i32 = 1i32;
#[repr(transparent)]
pub struct AsyncIAdviseSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIAdviseSink {}
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIAdviseSink2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIAdviseSink2 {}
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIMultiQI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIMultiQI {}
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIPipeByte(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIPipeByte {}
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIPipeDouble(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIPipeDouble {}
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIPipeLong(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIPipeLong {}
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIUnknown(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIUnknown {}
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: super::super::Foundation::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: super::super::Foundation::PWSTR,
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
pub const BINDINFOF_URLENCODESTGMEDDATA: i32 = 1i32;
pub const BINDINFOF_URLENCODEDEXTRAINFO: i32 = 2i32;
#[repr(C)]
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
pub const BIND_MAYBOTHERUSER: i32 = 1i32;
pub const BIND_JUSTTESTEXISTENCE: i32 = 2i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const CC_FASTCALL: i32 = 0i32;
pub const CC_CDECL: i32 = 1i32;
pub const CC_MSCPASCAL: i32 = 2i32;
pub const CC_PASCAL: i32 = 2i32;
pub const CC_MACPASCAL: i32 = 3i32;
pub const CC_STDCALL: i32 = 4i32;
pub const CC_FPFASTCALL: i32 = 5i32;
pub const CC_SYSCALL: i32 = 6i32;
pub const CC_MPWCDECL: i32 = 7i32;
pub const CC_MPWPASCAL: i32 = 8i32;
pub const CC_MAX: i32 = 9i32;
pub const CALLTYPE_TOPLEVEL: i32 = 1i32;
pub const CALLTYPE_NESTED: i32 = 2i32;
pub const CALLTYPE_ASYNC: i32 = 3i32;
pub const CALLTYPE_TOPLEVEL_CALLPENDING: i32 = 4i32;
pub const CALLTYPE_ASYNC_CALLPENDING: i32 = 5i32;
#[repr(C)]
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
pub const CLSCTX_INPROC_SERVER: u32 = 1u32;
pub const CLSCTX_INPROC_HANDLER: u32 = 2u32;
pub const CLSCTX_LOCAL_SERVER: u32 = 4u32;
pub const CLSCTX_INPROC_SERVER16: u32 = 8u32;
pub const CLSCTX_REMOTE_SERVER: u32 = 16u32;
pub const CLSCTX_INPROC_HANDLER16: u32 = 32u32;
pub const CLSCTX_RESERVED1: u32 = 64u32;
pub const CLSCTX_RESERVED2: u32 = 128u32;
pub const CLSCTX_RESERVED3: u32 = 256u32;
pub const CLSCTX_RESERVED4: u32 = 512u32;
pub const CLSCTX_NO_CODE_DOWNLOAD: u32 = 1024u32;
pub const CLSCTX_RESERVED5: u32 = 2048u32;
pub const CLSCTX_NO_CUSTOM_MARSHAL: u32 = 4096u32;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: u32 = 8192u32;
pub const CLSCTX_NO_FAILURE_LOG: u32 = 16384u32;
pub const CLSCTX_DISABLE_AAA: u32 = 32768u32;
pub const CLSCTX_ENABLE_AAA: u32 = 65536u32;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: u32 = 131072u32;
pub const CLSCTX_ACTIVATE_X86_SERVER: u32 = 262144u32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: u32 = 262144u32;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: u32 = 524288u32;
pub const CLSCTX_ENABLE_CLOAKING: u32 = 1048576u32;
pub const CLSCTX_APPCONTAINER: u32 = 4194304u32;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: u32 = 8388608u32;
pub const CLSCTX_RESERVED6: u32 = 16777216u32;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: u32 = 33554432u32;
pub const CLSCTX_PS_DLL: u32 = 2147483648u32;
pub const CLSCTX_ALL: u32 = 23u32;
pub const CLSCTX_SERVER: u32 = 21u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: super::super::Foundation::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const COINIT_APARTMENTTHREADED: u32 = 2u32;
pub const COINIT_MULTITHREADED: u32 = 0u32;
pub const COINIT_DISABLE_OLE1DDE: u32 = 4u32;
pub const COINIT_SPEED_OVER_MEMORY: u32 = 8u32;
pub const COINITBASE_MULTITHREADED: i32 = 0i32;
pub const SD_LAUNCHPERMISSIONS: i32 = 0i32;
pub const SD_ACCESSPERMISSIONS: i32 = 1i32;
pub const SD_LAUNCHRESTRICTIONS: i32 = 2i32;
pub const SD_ACCESSRESTRICTIONS: i32 = 3i32;
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const COWAIT_DEFAULT: i32 = 0i32;
pub const COWAIT_WAITALL: i32 = 1i32;
pub const COWAIT_ALERTABLE: i32 = 2i32;
pub const COWAIT_INPUTAVAILABLE: i32 = 4i32;
pub const COWAIT_DISPATCH_CALLS: i32 = 8i32;
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: i32 = 16i32;
pub type CO_DEVICE_CATALOG_COOKIE = isize;
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: i32 = 0i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: i32 = -2147483648i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: i32 = -2147483647i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: i32 = -2147483646i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: i32 = -2147483645i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: i32 = -2147483644i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: i32 = -2147483643i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: i32 = -2147483642i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: i32 = -2147483641i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: i32 = -2147483640i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: i32 = -2147483639i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: i32 = -2147483638i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: i32 = -2147483637i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: i32 = -2147483636i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: i32 = -2147483635i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: i32 = -2147483634i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: i32 = -2147483633i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: i32 = -2147483632i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: i32 = -2147483631i32;
pub type CO_MTA_USAGE_COOKIE = isize;
#[repr(C)]
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
pub const CWMO_DEFAULT: i32 = 0i32;
pub const CWMO_DISPATCH_CALLS: i32 = 1i32;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: i32 = 2i32;
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
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
pub const DATADIR_GET: i32 = 1i32;
pub const DATADIR_SET: i32 = 2i32;
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
pub const DCOM_NONE: i32 = 0i32;
pub const DCOM_CALL_COMPLETE: i32 = 1i32;
pub const DCOM_CALL_CANCELED: i32 = 2i32;
pub const DESCKIND_NONE: i32 = 0i32;
pub const DESCKIND_FUNCDESC: i32 = 1i32;
pub const DESCKIND_VARDESC: i32 = 2i32;
pub const DESCKIND_TYPECOMP: i32 = 3i32;
pub const DESCKIND_IMPLICITAPPOBJ: i32 = 4i32;
pub const DESCKIND_MAX: i32 = 5i32;
#[repr(C)]
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
pub const DMUS_ERRBASE: u32 = 4096u32;
pub const DVASPECT_CONTENT: i32 = 1i32;
pub const DVASPECT_THUMBNAIL: i32 = 2i32;
pub const DVASPECT_ICON: i32 = 4i32;
pub const DVASPECT_DOCPRINT: i32 = 8i32;
#[repr(C)]
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
pub const EOAC_NONE: i32 = 0i32;
pub const EOAC_MUTUAL_AUTH: i32 = 1i32;
pub const EOAC_STATIC_CLOAKING: i32 = 32i32;
pub const EOAC_DYNAMIC_CLOAKING: i32 = 64i32;
pub const EOAC_ANY_AUTHORITY: i32 = 128i32;
pub const EOAC_MAKE_FULLSIC: i32 = 256i32;
pub const EOAC_DEFAULT: i32 = 2048i32;
pub const EOAC_SECURE_REFS: i32 = 2i32;
pub const EOAC_ACCESS_CONTROL: i32 = 4i32;
pub const EOAC_APPID: i32 = 8i32;
pub const EOAC_DYNAMIC: i32 = 16i32;
pub const EOAC_REQUIRE_FULLSIC: i32 = 512i32;
pub const EOAC_AUTO_IMPERSONATE: i32 = 1024i32;
pub const EOAC_DISABLE_AAA: i32 = 4096i32;
pub const EOAC_NO_CUSTOM_MARSHAL: i32 = 8192i32;
pub const EOAC_RESERVED1: i32 = 16384i32;
#[repr(C)]
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
pub const EXTCONN_STRONG: i32 = 1i32;
pub const EXTCONN_WEAK: i32 = 2i32;
pub const EXTCONN_CALLABLE: i32 = 4i32;
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const FUNC_VIRTUAL: i32 = 0i32;
pub const FUNC_PUREVIRTUAL: i32 = 1i32;
pub const FUNC_NONVIRTUAL: i32 = 2i32;
pub const FUNC_STATIC: i32 = 3i32;
pub const FUNC_DISPATCH: i32 = 4i32;
#[repr(C)]
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
pub const COMGLB_EXCEPTION_HANDLE: i32 = 0i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: i32 = 1i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE: i32 = 1i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: i32 = 2i32;
pub const COMGLB_EXCEPTION_HANDLING: i32 = 1i32;
pub const COMGLB_APPID: i32 = 2i32;
pub const COMGLB_RPC_THREADPOOL_SETTING: i32 = 3i32;
pub const COMGLB_RO_SETTINGS: i32 = 4i32;
pub const COMGLB_UNMARSHALING_POLICY: i32 = 5i32;
pub const COMGLB_PROPERTIES_RESERVED1: i32 = 6i32;
pub const COMGLB_PROPERTIES_RESERVED2: i32 = 7i32;
pub const COMGLB_PROPERTIES_RESERVED3: i32 = 8i32;
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: i32 = 1i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: i32 = 2i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: i32 = 4i32;
pub const COMGLB_FAST_RUNDOWN: i32 = 8i32;
pub const COMGLB_RESERVED1: i32 = 16i32;
pub const COMGLB_RESERVED2: i32 = 32i32;
pub const COMGLB_RESERVED3: i32 = 64i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: i32 = 128i32;
pub const COMGLB_RESERVED4: i32 = 256i32;
pub const COMGLB_RESERVED5: i32 = 512i32;
pub const COMGLB_RESERVED6: i32 = 1024i32;
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: i32 = 0i32;
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: i32 = 1i32;
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: i32 = 0i32;
pub const COMGLB_UNMARSHALING_POLICY_STRONG: i32 = 1i32;
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: i32 = 2i32;
#[repr(C)]
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
#[repr(transparent)]
pub struct IActivationFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivationFilter {}
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAddrExclusionControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAddrExclusionControl {}
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAddrTrackingControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAddrTrackingControl {}
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdviseSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdviseSink {}
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdviseSink2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdviseSink2 {}
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAgileObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAgileObject {}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsyncManager {}
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncRpcChannelBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsyncRpcChannelBuffer {}
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAuthenticate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAuthenticate {}
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAuthenticateEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAuthenticateEx {}
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindCtx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindCtx {}
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindHost {}
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindStatusCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindStatusCallback {}
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindStatusCallbackEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindStatusCallbackEx {}
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinding {}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlockingLock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockingLock {}
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallFactory {}
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICancelMethodCalls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICancelMethodCalls {}
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICatInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatInformation {}
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICatRegister(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatRegister {}
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChannelHook(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChannelHook {}
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClassActivator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClassActivator {}
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClassFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClassFactory {}
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClientSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClientSecurity {}
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComThreadingInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComThreadingInfo {}
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectionPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectionPoint {}
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectionPointContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectionPointContainer {}
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IContext(pub u8);
#[repr(transparent)]
pub struct IContextCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextCallback {}
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(transparent)]
pub struct IDataAdviseHolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataAdviseHolder {}
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataObject {}
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatch {}
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumCATEGORYINFO(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumCATEGORYINFO {}
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumConnectionPoints(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumConnectionPoints {}
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumConnections(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumConnections {}
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IEnumContextProps(pub u8);
#[repr(transparent)]
pub struct IEnumFORMATETC(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumFORMATETC {}
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumGUID(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumGUID {}
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumMoniker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumMoniker {}
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumSTATDATA(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumSTATDATA {}
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumString {}
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumUnknown(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumUnknown {}
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IErrorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IErrorInfo {}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IErrorLog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IErrorLog {}
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExternalConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExternalConnection {}
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFastRundown(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFastRundown {}
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IForegroundTransfer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IForegroundTransfer {}
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalInterfaceTable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalInterfaceTable {}
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalOptions {}
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitializeSpy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitializeSpy {}
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternalUnknown(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternalUnknown {}
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMachineGlobalObjectTable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMachineGlobalObjectTable {}
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMalloc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMalloc {}
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMallocSpy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMallocSpy {}
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMoniker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMoniker {}
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultiQI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultiQI {}
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const INVOKE_FUNC: i32 = 1i32;
pub const INVOKE_PROPERTYGET: i32 = 2i32;
pub const INVOKE_PROPERTYPUT: i32 = 4i32;
pub const INVOKE_PROPERTYPUTREF: i32 = 8i32;
#[repr(transparent)]
pub struct INoMarshal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INoMarshal {}
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOplockStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOplockStorage {}
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPSFactoryBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPSFactoryBuffer {}
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersist(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersist {}
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistFile {}
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistMemory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistMemory {}
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistStream {}
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistStreamInit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistStreamInit {}
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPipeByte(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPipeByte {}
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPipeDouble(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPipeDouble {}
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPipeLong(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPipeLong {}
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessInitControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessInitControl {}
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessLock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessLock {}
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressNotify(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressNotify {}
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IROTData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IROTData {}
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReleaseMarshalBuffers(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReleaseMarshalBuffers {}
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcChannelBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcChannelBuffer {}
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcChannelBuffer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcChannelBuffer2 {}
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcChannelBuffer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcChannelBuffer3 {}
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcHelper {}
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcOptions {}
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcProxyBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcProxyBuffer {}
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcStubBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcStubBuffer {}
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRpcSyntaxNegotiate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRpcSyntaxNegotiate {}
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRunnableObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRunnableObject {}
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRunningObjectTable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRunningObjectTable {}
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISequentialStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISequentialStream {}
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerSecurity {}
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceProvider {}
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStdMarshalInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStdMarshalInfo {}
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStream {}
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISupportErrorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISupportErrorInfo {}
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurrogate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurrogate {}
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurrogateService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurrogateService {}
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISynchronize(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISynchronize {}
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISynchronizeContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISynchronizeContainer {}
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISynchronizeEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISynchronizeEvent {}
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISynchronizeHandle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISynchronizeHandle {}
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISynchronizeMutex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISynchronizeMutex {}
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeAndNoticeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeAndNoticeControl {}
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeComp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeComp {}
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeInfo {}
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeInfo2 {}
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeLib(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeLib {}
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeLib2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeLib2 {}
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeLibRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeLibRegistration {}
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypeLibRegistrationReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypeLibRegistrationReader {}
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUri(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUri {}
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriBuilder {}
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUrlMon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUrlMon {}
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWaitMultiple(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWaitMultiple {}
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows_sys::core::HRESULT;
pub type LPFNCANUNLOADNOW = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
pub type LPFNGETCLASSOBJECT = unsafe extern "system" fn(param0: *const ::windows_sys::core::GUID, param1: *const ::windows_sys::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
pub const MEMCTX_TASK: i32 = 1i32;
pub const MEMCTX_SHARED: i32 = 2i32;
pub const MEMCTX_MACSYSTEM: i32 = 3i32;
pub const MEMCTX_UNKNOWN: i32 = -1i32;
pub const MEMCTX_SAME: i32 = -2i32;
pub const MKRREDUCE_ONE: i32 = 196608i32;
pub const MKRREDUCE_TOUSER: i32 = 131072i32;
pub const MKRREDUCE_THROUGHUSER: i32 = 65536i32;
pub const MKRREDUCE_ALL: i32 = 0i32;
pub const MKSYS_NONE: i32 = 0i32;
pub const MKSYS_GENERICCOMPOSITE: i32 = 1i32;
pub const MKSYS_FILEMONIKER: i32 = 2i32;
pub const MKSYS_ANTIMONIKER: i32 = 3i32;
pub const MKSYS_ITEMMONIKER: i32 = 4i32;
pub const MKSYS_POINTERMONIKER: i32 = 5i32;
pub const MKSYS_CLASSMONIKER: i32 = 7i32;
pub const MKSYS_OBJREFMONIKER: i32 = 8i32;
pub const MKSYS_SESSIONMONIKER: i32 = 9i32;
pub const MKSYS_LUAMONIKER: i32 = 10i32;
pub const MSHCTX_LOCAL: i32 = 0i32;
pub const MSHCTX_NOSHAREDMEM: i32 = 1i32;
pub const MSHCTX_DIFFERENTMACHINE: i32 = 2i32;
pub const MSHCTX_INPROC: i32 = 3i32;
pub const MSHCTX_CROSSCTX: i32 = 4i32;
pub const MSHCTX_CONTAINER: i32 = 5i32;
pub const MSHLFLAGS_NORMAL: i32 = 0i32;
pub const MSHLFLAGS_TABLESTRONG: i32 = 1i32;
pub const MSHLFLAGS_TABLEWEAK: i32 = 2i32;
pub const MSHLFLAGS_NOPING: i32 = 4i32;
pub const MSHLFLAGS_RESERVED1: i32 = 8i32;
pub const MSHLFLAGS_RESERVED2: i32 = 16i32;
pub const MSHLFLAGS_RESERVED3: i32 = 32i32;
pub const MSHLFLAGS_RESERVED4: i32 = 64i32;
#[repr(C)]
pub struct MULTI_QI {
    pub pIID: *mut ::windows_sys::core::GUID,
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
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PENDINGMSG_CANCELCALL: i32 = 0i32;
pub const PENDINGMSG_WAITNOPROCESS: i32 = 1i32;
pub const PENDINGMSG_WAITDEFPROCESS: i32 = 2i32;
pub const PENDINGTYPE_TOPLEVEL: i32 = 1i32;
pub const PENDINGTYPE_NESTED: i32 = 2i32;
pub type PFNCONTEXTCALL = unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub const REGCLS_SINGLEUSE: i32 = 0i32;
pub const REGCLS_MULTIPLEUSE: i32 = 1i32;
pub const REGCLS_MULTI_SEPARATE: i32 = 2i32;
pub const REGCLS_SUSPENDED: i32 = 4i32;
pub const REGCLS_SURROGATE: i32 = 8i32;
pub const REGCLS_AGILE: i32 = 16i32;
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(C)]
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
pub const COMBND_RPCTIMEOUT: i32 = 1i32;
pub const COMBND_SERVER_LOCALITY: i32 = 2i32;
pub const COMBND_RESERVED1: i32 = 4i32;
pub const COMBND_RESERVED2: i32 = 5i32;
pub const COMBND_RESERVED3: i32 = 8i32;
pub const COMBND_RESERVED4: i32 = 16i32;
pub const SERVER_LOCALITY_PROCESS_LOCAL: i32 = 0i32;
pub const SERVER_LOCALITY_MACHINE_LOCAL: i32 = 1i32;
pub const SERVER_LOCALITY_REMOTE: i32 = 2i32;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: u32 = 0u32;
pub const RPC_C_AUTHN_LEVEL_NONE: u32 = 1u32;
pub const RPC_C_AUTHN_LEVEL_CONNECT: u32 = 2u32;
pub const RPC_C_AUTHN_LEVEL_CALL: u32 = 3u32;
pub const RPC_C_AUTHN_LEVEL_PKT: u32 = 4u32;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 5u32;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: u32 = 6u32;
pub const RPC_C_IMP_LEVEL_DEFAULT: u32 = 0u32;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: u32 = 1u32;
pub const RPC_C_IMP_LEVEL_IDENTIFY: u32 = 2u32;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: u32 = 3u32;
pub const RPC_C_IMP_LEVEL_DELEGATE: u32 = 4u32;
#[repr(C)]
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
pub const SERVERCALL_ISHANDLED: i32 = 0i32;
pub const SERVERCALL_REJECTED: i32 = 1i32;
pub const SERVERCALL_RETRYLATER: i32 = 2i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: super::super::Foundation::PWSTR,
    pub hr: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: super::super::Foundation::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::windows_sys::core::IUnknown,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: IStream,
    pub pstg: StructuredStorage::IStorage,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STGTY_STORAGE: i32 = 1i32;
pub const STGTY_STREAM: i32 = 2i32;
pub const STGTY_LOCKBYTES: i32 = 3i32;
pub const STGTY_PROPERTY: i32 = 4i32;
pub const STGTY_REPEAT: i32 = 256i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
pub const STREAM_SEEK_SET: u32 = 0u32;
pub const STREAM_SEEK_CUR: u32 = 1u32;
pub const STREAM_SEEK_END: u32 = 2u32;
pub const SYS_WIN16: i32 = 0i32;
pub const SYS_WIN32: i32 = 1i32;
pub const SYS_MAC: i32 = 2i32;
pub const SYS_WIN64: i32 = 3i32;
pub const IdleShutdown: i32 = 0i32;
pub const ForcedShutdown: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: super::super::Foundation::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
pub const THDTYPE_BLOCKMESSAGES: i32 = 0i32;
pub const THDTYPE_PROCESSMESSAGES: i32 = 1i32;
#[repr(C)]
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
pub const TYMED_HGLOBAL: i32 = 1i32;
pub const TYMED_FILE: i32 = 2i32;
pub const TYMED_ISTREAM: i32 = 4i32;
pub const TYMED_ISTORAGE: i32 = 8i32;
pub const TYMED_GDI: i32 = 16i32;
pub const TYMED_MFPICT: i32 = 32i32;
pub const TYMED_ENHMF: i32 = 64i32;
pub const TYMED_NULL: i32 = 0i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct TYPEATTR {
    pub guid: ::windows_sys::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const TKIND_ENUM: i32 = 0i32;
pub const TKIND_RECORD: i32 = 1i32;
pub const TKIND_MODULE: i32 = 2i32;
pub const TKIND_INTERFACE: i32 = 3i32;
pub const TKIND_DISPATCH: i32 = 4i32;
pub const TKIND_COCLASS: i32 = 5i32;
pub const TKIND_ALIAS: i32 = 6i32;
pub const TKIND_UNION: i32 = 7i32;
pub const TKIND_MAX: i32 = 8i32;
pub const TYSPEC_CLSID: i32 = 0i32;
pub const TYSPEC_FILEEXT: i32 = 1i32;
pub const TYSPEC_MIMETYPE: i32 = 2i32;
pub const TYSPEC_FILENAME: i32 = 3i32;
pub const TYSPEC_PROGID: i32 = 4i32;
pub const TYSPEC_PACKAGENAME: i32 = 5i32;
pub const TYSPEC_OBJECTID: i32 = 6i32;
pub const Uri_CREATE_ALLOW_RELATIVE: u32 = 1u32;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: u32 = 2u32;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: u32 = 4u32;
pub const Uri_CREATE_NOFRAG: u32 = 8u32;
pub const Uri_CREATE_NO_CANONICALIZE: u32 = 16u32;
pub const Uri_CREATE_CANONICALIZE: u32 = 256u32;
pub const Uri_CREATE_FILE_USE_DOS_PATH: u32 = 32u32;
pub const Uri_CREATE_DECODE_EXTRA_INFO: u32 = 64u32;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: u32 = 128u32;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: u32 = 512u32;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: u32 = 1024u32;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: u32 = 2048u32;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: u32 = 4096u32;
pub const Uri_CREATE_IE_SETTINGS: u32 = 8192u32;
pub const Uri_CREATE_NO_IE_SETTINGS: u32 = 16384u32;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: u32 = 32768u32;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: u32 = 65536u32;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: u32 = 131072u32;
pub const Uri_PROPERTY_ABSOLUTE_URI: i32 = 0i32;
pub const Uri_PROPERTY_STRING_START: i32 = 0i32;
pub const Uri_PROPERTY_AUTHORITY: i32 = 1i32;
pub const Uri_PROPERTY_DISPLAY_URI: i32 = 2i32;
pub const Uri_PROPERTY_DOMAIN: i32 = 3i32;
pub const Uri_PROPERTY_EXTENSION: i32 = 4i32;
pub const Uri_PROPERTY_FRAGMENT: i32 = 5i32;
pub const Uri_PROPERTY_HOST: i32 = 6i32;
pub const Uri_PROPERTY_PASSWORD: i32 = 7i32;
pub const Uri_PROPERTY_PATH: i32 = 8i32;
pub const Uri_PROPERTY_PATH_AND_QUERY: i32 = 9i32;
pub const Uri_PROPERTY_QUERY: i32 = 10i32;
pub const Uri_PROPERTY_RAW_URI: i32 = 11i32;
pub const Uri_PROPERTY_SCHEME_NAME: i32 = 12i32;
pub const Uri_PROPERTY_USER_INFO: i32 = 13i32;
pub const Uri_PROPERTY_USER_NAME: i32 = 14i32;
pub const Uri_PROPERTY_STRING_LAST: i32 = 14i32;
pub const Uri_PROPERTY_HOST_TYPE: i32 = 15i32;
pub const Uri_PROPERTY_DWORD_START: i32 = 15i32;
pub const Uri_PROPERTY_PORT: i32 = 16i32;
pub const Uri_PROPERTY_SCHEME: i32 = 17i32;
pub const Uri_PROPERTY_ZONE: i32 = 18i32;
pub const Uri_PROPERTY_DWORD_LAST: i32 = 18i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
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
    pub pcVal: super::super::Foundation::PSTR,
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
pub const VAR_PERINSTANCE: i32 = 0i32;
pub const VAR_STATIC: i32 = 1i32;
pub const VAR_CONST: i32 = 2i32;
pub const VAR_DISPATCH: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union uCLSSPEC_0 {
    pub clsid: ::windows_sys::core::GUID,
    pub pFileExt: super::super::Foundation::PWSTR,
    pub pMimeType: super::super::Foundation::PWSTR,
    pub pProgId: super::super::Foundation::PWSTR,
    pub pFileName: super::super::Foundation::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::super::Foundation::PWSTR,
    pub PolicyId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows_sys::core::GUID,
    pub PolicyId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
