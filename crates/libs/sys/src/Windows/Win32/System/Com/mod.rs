#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
windows_link::link!("ole32.dll" "system" fn BindMoniker(pmk : *mut core::ffi::c_void, grfopt : u32, iidresult : *const windows_sys::core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : windows_sys::core::PCWSTR, lpclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : windows_sys::core::PCWSTR, lpclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> u32);
windows_link::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
windows_link::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : u32, ultimeout : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : *mut core::ffi::c_void, ppcopy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("combase.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : *mut core::ffi::c_void, ppunkmarshal : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : CLSCTX, pserverinfo : *const COSERVERINFO, dwcount : u32, presults : *mut MULTI_QI) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : CLSCTX, reserved : *const core::ffi::c_void, dwcount : u32, presults : *mut MULTI_QI) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDisconnectObject(punk : *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::super::Foundation::FILETIME) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::super::Foundation::FILETIME) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::super::Foundation::FILETIME, lpdosdate : *mut u16, lpdostime : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoFreeAllLibraries());
windows_link::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::super::Foundation::HINSTANCE));
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
windows_link::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : u32, dwreserved : u32));
windows_link::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut APTTYPE, paptqualifier : *mut APTTYPEQUALIFIER) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const windows_sys::core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : u32, iid : *const windows_sys::core::GUID, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const windows_sys::core::GUID, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut usize) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> u32);
windows_link::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : u32, ppmalloc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_sys::core::PCWSTR, pbindoptions : *const BIND_OPTS, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const windows_sys::core::GUID, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_link::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::super::Security::PSECURITY_DESCRIPTOR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const windows_sys::core::GUID, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_sys::core::HRESULT);
windows_link::link!("combase.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_link::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::super::Security::PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : u32, pclassspec : *const uCLSSPEC, pquery : *const QUERYCONTEXT, pszcodebase : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_sys::core::GUID) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : windows_sys::core::PCWSTR, bautofree : windows_sys::core::BOOL) -> super::super::Foundation::HINSTANCE);
windows_link::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : *mut core::ffi::c_void, flock : windows_sys::core::BOOL, flastunlockreleases : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut u32, asauthsvc : *mut *mut SOLE_AUTHENTICATION_SERVICE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_sys::core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pprivs : *mut *mut core::ffi::c_void, pcapabilities : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : *mut core::ffi::c_void, pwauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_sys::core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pauthinfo : *mut *mut core::ffi::c_void, pcapabilites : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_sys::core::GUID, pchannelhook : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void, dwclscontext : CLSCTX, flags : u32, lpdwregister : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_sys::core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const windows_sys::core::GUID, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> u32);
windows_link::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : u64) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSetCancelObject(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : *mut core::ffi::c_void, dwauthnsvc : u32, dwauthzsvc : u32, pserverprincname : windows_sys::core::PCWSTR, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthinfo : *const core::ffi::c_void, dwcapabilities : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : *mut core::ffi::c_void, ppoldobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("combase.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_link::link!("combase.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
windows_link::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : *const core::ffi::c_void, cb : usize) -> *mut core::ffi::c_void);
windows_link::link!("ole32.dll" "system" fn CoTestCancel() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoUninitialize());
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation::HANDLE, lpdwindex : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation::HANDLE, lpdwindex : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_sys::core::GUID, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateDataCache(punkouter : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID, iid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : *mut core::ffi::c_void, pmkrest : *mut core::ffi::c_void, ppmkcomposite : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateIUriBuilder(piuri : *mut core::ffi::c_void, dwflags : u32, dwreserved : usize, ppiuribuilder : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : windows_sys::core::PCWSTR, lpszitem : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::super::Foundation::HWND, psztitle : windows_sys::core::PCWSTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUri(pwzuri : windows_sys::core::PCWSTR, dwflags : URI_CREATE_FLAGS, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUriFromMultiByteString(pszansiinputuri : windows_sys::core::PCSTR, dwencodingflags : u32, dwcodepage : u32, dwcreateflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUriWithFragment(pwzuri : windows_sys::core::PCWSTR, pwzfragment : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn GetClassFile(szfilename : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn IIDFromString(lpsz : windows_sys::core::PCWSTR, lpiid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : *mut core::ffi::c_void, szusername : windows_sys::core::PCWSTR, pcheaten : *mut u32, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : *mut core::ffi::c_void, pmkother : *mut core::ffi::c_void, ppmkcommon : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : *mut core::ffi::c_void, pmkdest : *mut core::ffi::c_void, ppmkrelpath : *mut *mut core::ffi::c_void, dwreserved : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const windows_sys::core::GUID, lplpszprogid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const windows_sys::core::GUID, lplpsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_sys::core::GUID, lpsz : windows_sys::core::PWSTR, cchmax : i32) -> i32);
windows_link::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const windows_sys::core::GUID, lplpsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type ADVANCED_FEATURE_FLAGS = u16;
pub type ADVF = i32;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16;
pub const ADVFCACHE_NOHANDLER: ADVF = 8;
pub const ADVFCACHE_ONSAVE: ADVF = 32;
pub const ADVF_DATAONSTOP: ADVF = 64;
pub const ADVF_NODATA: ADVF = 1;
pub const ADVF_ONLYONCE: ADVF = 4;
pub const ADVF_PRIMEFIRST: ADVF = 2;
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64;
pub const APPIDREGFLAGS_RESERVED10: u32 = 32768;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2;
pub type APTTYPE = i32;
pub type APTTYPEQUALIFIER = i32;
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = 6;
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = 1;
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = 4;
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = 5;
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = 2;
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = 3;
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = 0;
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = 7;
pub const APTTYPE_CURRENT: APTTYPE = -1;
pub const APTTYPE_MAINSTA: APTTYPE = 3;
pub const APTTYPE_MTA: APTTYPE = 1;
pub const APTTYPE_NA: APTTYPE = 2;
pub const APTTYPE_STA: APTTYPE = 0;
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1;
pub const ASYNC_MODE_DEFAULT: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
pub type ApplicationType = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_sys::core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_sys::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for BINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BINDINFOF = i32;
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = 2;
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BIND_FLAGS = i32;
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2;
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
impl Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLCONV = i32;
pub type CALLTYPE = i32;
pub const CALLTYPE_ASYNC: CALLTYPE = 3;
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5;
pub const CALLTYPE_NESTED: CALLTYPE = 2;
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1;
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CATEGORYINFO {
    pub catid: windows_sys::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CC_CDECL: CALLCONV = 1;
pub const CC_FASTCALL: CALLCONV = 0;
pub const CC_FPFASTCALL: CALLCONV = 5;
pub const CC_MACPASCAL: CALLCONV = 3;
pub const CC_MAX: CALLCONV = 9;
pub const CC_MPWCDECL: CALLCONV = 7;
pub const CC_MPWPASCAL: CALLCONV = 8;
pub const CC_MSCPASCAL: CALLCONV = 2;
pub const CC_PASCAL: CALLCONV = 2;
pub const CC_STDCALL: CALLCONV = 4;
pub const CC_SYSCALL: CALLCONV = 6;
pub type CLSCTX = u32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432;
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144;
pub const CLSCTX_ALL: CLSCTX = 23;
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = 67108864;
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304;
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768;
pub const CLSCTX_DO_NOT_ELEVATE_SERVER: CLSCTX = 268435456;
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536;
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072;
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2;
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8;
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4;
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024;
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096;
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384;
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648;
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16;
pub const CLSCTX_RESERVED1: CLSCTX = 64;
pub const CLSCTX_RESERVED2: CLSCTX = 128;
pub const CLSCTX_RESERVED3: CLSCTX = 256;
pub const CLSCTX_RESERVED4: CLSCTX = 512;
pub const CLSCTX_RESERVED5: CLSCTX = 2048;
pub const CLSCTX_RESERVED6: CLSCTX = 16777216;
pub const CLSCTX_SERVER: CLSCTX = 21;
pub const CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE: CLSCTX = 134217728;
pub const CLSID_GlobalOptions: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0000034b_0000_0000_c000_000000000046);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: windows_sys::core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COINIT = i32;
pub type COINITBASE = i32;
pub const COINITBASE_MULTITHREADED: COINITBASE = 0;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
pub const COLE_DEFAULT_AUTHINFO: i32 = -1;
pub const COLE_DEFAULT_PRINCIPAL: windows_sys::core::PCWSTR = -1 as _;
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = 4;
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = 5;
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = 8;
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = 16;
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = 1;
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = 2;
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = 2;
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = 1;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = 2;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = 1;
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = 0;
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = 1;
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = 8;
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = 6;
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = 7;
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = 8;
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = 16;
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = 32;
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = 64;
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = 256;
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = 512;
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = 1024;
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = 4;
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = 3;
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = 0;
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = 1;
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = 1;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 4;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 2;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = 128;
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = 5;
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 2;
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 0;
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 1;
pub type COMSD = i32;
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16;
pub const COM_RIGHTS_EXECUTE: u32 = 1;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4;
pub const COM_RIGHTS_RESERVED1: u32 = 32;
pub const COM_RIGHTS_RESERVED2: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONNECTDATA {
    pub pUnk: *mut core::ffi::c_void,
    pub dwCookie: u32,
}
impl Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: windows_sys::core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2;
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0;
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8;
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16;
pub type COWAIT_FLAGS = i32;
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4;
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1;
pub type CO_DEVICE_CATALOG_COOKIE = *mut core::ffi::c_void;
pub type CO_MARSHALING_CONTEXT_ATTRIBUTES = i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483648;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483639;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483638;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483637;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483636;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483635;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483634;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483633;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483632;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483631;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483647;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483646;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483645;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483644;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483643;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483642;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483641;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483640;
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = 0;
pub type CO_MTA_USAGE_COOKIE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct CUSTDATAITEM {
    pub guid: windows_sys::core::GUID,
    pub varValue: super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CWMO_DEFAULT: CWMO_FLAGS = 0;
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2;
pub type CWMO_FLAGS = i32;
pub const CWMO_MAX_HANDLES: u32 = 56;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut core::ffi::c_void,
}
impl Default for ComCallData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContextProperty {
    pub policyId: windows_sys::core::GUID,
    pub flags: u32,
    pub pUnk: *mut core::ffi::c_void,
}
impl Default for ContextProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DATADIR = i32;
pub const DATADIR_GET: DATADIR = 1;
pub const DATADIR_SET: DATADIR = 2;
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4;
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = 2;
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = 1;
pub type DCOM_CALL_STATE = i32;
pub const DCOM_NONE: DCOM_CALL_STATE = 0;
pub type DESCKIND = i32;
pub const DESCKIND_FUNCDESC: DESCKIND = 1;
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4;
pub const DESCKIND_MAX: DESCKIND = 5;
pub const DESCKIND_NONE: DESCKIND = 0;
pub const DESCKIND_TYPECOMP: DESCKIND = 3;
pub const DESCKIND_VARDESC: DESCKIND = 2;
pub type DISPATCH_FLAGS = u16;
pub const DISPATCH_METHOD: DISPATCH_FLAGS = 1;
pub const DISPATCH_PROPERTYGET: DISPATCH_FLAGS = 2;
pub const DISPATCH_PROPERTYPUT: DISPATCH_FLAGS = 4;
pub const DISPATCH_PROPERTYPUTREF: DISPATCH_FLAGS = 8;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct DISPPARAMS {
    pub rgvarg: *mut super::Variant::VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMUS_ERRBASE: u32 = 4096;
pub type DVASPECT = u32;
pub const DVASPECT_CONTENT: DVASPECT = 1;
pub const DVASPECT_DOCPRINT: DVASPECT = 8;
pub const DVASPECT_ICON: DVASPECT = 4;
pub const DVASPECT_OPAQUE: DVASPECT = 16;
pub const DVASPECT_THUMBNAIL: DVASPECT = 2;
pub const DVASPECT_TRANSPARENT: DVASPECT = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl Default for DWORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = 4;
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = 128;
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = 8;
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = 1024;
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = 2048;
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = 4096;
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = 16;
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64;
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 256;
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = 1;
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = 0;
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = 8192;
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 512;
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = 16384;
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = 2;
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 32;
pub type EOLE_AUTHENTICATION_CAPABILITIES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: windows_sys::core::BSTR,
    pub bstrDescription: windows_sys::core::BSTR,
    pub bstrHelpFile: windows_sys::core::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXTCONN = i32;
pub const EXTCONN_CALLABLE: EXTCONN = 4;
pub const EXTCONN_STRONG: EXTCONN = 1;
pub const EXTCONN_WEAK: EXTCONN = 2;
pub const FADF_AUTO: ADVANCED_FEATURE_FLAGS = 1;
pub const FADF_BSTR: ADVANCED_FEATURE_FLAGS = 256;
pub const FADF_DISPATCH: ADVANCED_FEATURE_FLAGS = 1024;
pub const FADF_EMBEDDED: ADVANCED_FEATURE_FLAGS = 4;
pub const FADF_FIXEDSIZE: ADVANCED_FEATURE_FLAGS = 16;
pub const FADF_HAVEIID: ADVANCED_FEATURE_FLAGS = 64;
pub const FADF_HAVEVARTYPE: ADVANCED_FEATURE_FLAGS = 128;
pub const FADF_RECORD: ADVANCED_FEATURE_FLAGS = 32;
pub const FADF_RESERVED: ADVANCED_FEATURE_FLAGS = 61448;
pub const FADF_STATIC: ADVANCED_FEATURE_FLAGS = 2;
pub const FADF_UNKNOWN: ADVANCED_FEATURE_FLAGS = 512;
pub const FADF_VARIANT: ADVANCED_FEATURE_FLAGS = 2048;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl Default for FORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
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
    pub wFuncFlags: FUNCFLAGS,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FUNCFLAGS = u16;
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4;
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32;
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256;
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16;
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64;
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096;
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024;
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048;
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8;
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1;
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2;
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512;
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128;
pub type FUNCKIND = i32;
pub const FUNC_DISPATCH: FUNCKIND = 4;
pub const FUNC_NONVIRTUAL: FUNCKIND = 2;
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1;
pub const FUNC_STATIC: FUNCKIND = 3;
pub const FUNC_VIRTUAL: FUNCKIND = 0;
pub const ForcedShutdown: ShutdownType = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GLOBALOPT_EH_VALUES = i32;
pub type GLOBALOPT_PROPERTIES = i32;
pub type GLOBALOPT_RO_FLAGS = i32;
pub type GLOBALOPT_RPCTP_VALUES = i32;
pub type GLOBALOPT_UNMARSHALING_POLICY_VALUES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
pub type IDLFLAGS = u16;
pub const IDLFLAG_FIN: IDLFLAGS = 1;
pub const IDLFLAG_FLCID: IDLFLAGS = 4;
pub const IDLFLAG_FOUT: IDLFLAGS = 2;
pub const IDLFLAG_FRETVAL: IDLFLAGS = 8;
pub const IDLFLAG_NONE: IDLFLAGS = 0;
pub type IMPLTYPEFLAGS = i32;
pub const IMPLTYPEFLAG_FDEFAULT: IMPLTYPEFLAGS = 1;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: IMPLTYPEFLAGS = 8;
pub const IMPLTYPEFLAG_FRESTRICTED: IMPLTYPEFLAGS = 4;
pub const IMPLTYPEFLAG_FSOURCE: IMPLTYPEFLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERFACEINFO {
    pub pUnk: *mut core::ffi::c_void,
    pub iid: windows_sys::core::GUID,
    pub wMethod: u16,
}
impl Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type INVOKEKIND = i32;
pub const INVOKE_FUNC: INVOKEKIND = 1;
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2;
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4;
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8;
pub const IdleShutdown: ShutdownType = 0;
pub type LOCKTYPE = i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2;
pub const LOCK_ONLYONCE: LOCKTYPE = 4;
pub const LOCK_WRITE: LOCKTYPE = 1;
pub type LPEXCEPFINO_DEFERRED_FILLIN = Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> windows_sys::core::HRESULT>;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_sys::core::GUID, param1: *const windows_sys::core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub const LibraryApplication: ApplicationType = 1;
pub const MARSHALINTERFACE_MIN: u32 = 500;
pub const MAXLSN: u64 = 9223372036854775807;
pub type MEMCTX = i32;
pub const MEMCTX_MACSYSTEM: MEMCTX = 3;
pub const MEMCTX_SAME: MEMCTX = -2;
pub const MEMCTX_SHARED: MEMCTX = 2;
pub const MEMCTX_TASK: MEMCTX = 1;
pub const MEMCTX_UNKNOWN: MEMCTX = -1;
pub type MKRREDUCE = i32;
pub const MKRREDUCE_ALL: MKRREDUCE = 0;
pub const MKRREDUCE_ONE: MKRREDUCE = 196608;
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = 65536;
pub const MKRREDUCE_TOUSER: MKRREDUCE = 131072;
pub type MKSYS = i32;
pub const MKSYS_ANTIMONIKER: MKSYS = 3;
pub const MKSYS_CLASSMONIKER: MKSYS = 7;
pub const MKSYS_FILEMONIKER: MKSYS = 2;
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1;
pub const MKSYS_ITEMMONIKER: MKSYS = 4;
pub const MKSYS_LUAMONIKER: MKSYS = 10;
pub const MKSYS_NONE: MKSYS = 0;
pub const MKSYS_OBJREFMONIKER: MKSYS = 8;
pub const MKSYS_POINTERMONIKER: MKSYS = 5;
pub const MKSYS_SESSIONMONIKER: MKSYS = 9;
pub type MSHCTX = i32;
pub const MSHCTX_CONTAINER: MSHCTX = 5;
pub const MSHCTX_CROSSCTX: MSHCTX = 4;
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = 2;
pub const MSHCTX_INPROC: MSHCTX = 3;
pub const MSHCTX_LOCAL: MSHCTX = 0;
pub const MSHCTX_NOSHAREDMEM: MSHCTX = 1;
pub type MSHLFLAGS = i32;
pub const MSHLFLAGS_NOPING: MSHLFLAGS = 4;
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = 0;
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = 8;
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = 16;
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = 32;
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = 64;
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = 1;
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MULTI_QI {
    pub pIID: *const windows_sys::core::GUID,
    pub pItf: *mut core::ffi::c_void,
    pub hr: windows_sys::core::HRESULT,
}
impl Default for MULTI_QI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MachineGlobalObjectTableRegistrationToken = *mut core::ffi::c_void;
pub type PENDINGMSG = i32;
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0;
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2;
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1;
pub type PENDINGTYPE = i32;
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2;
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1;
pub type PFNCONTEXTCALL = Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
pub type REGCLS = i32;
pub const REGCLS_AGILE: REGCLS = 16;
pub const REGCLS_MULTIPLEUSE: REGCLS = 1;
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2;
pub const REGCLS_SINGLEUSE: REGCLS = 0;
pub const REGCLS_SURROGATE: REGCLS = 8;
pub const REGCLS_SUSPENDED: REGCLS = 4;
pub const ROTFLAGS_ALLOWANYCLIENT: ROT_FLAGS = 2;
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: ROT_FLAGS = 1;
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1;
pub type ROT_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPCOPT_PROPERTIES = i32;
pub type RPCOPT_SERVER_LOCALITY_VALUES = i32;
pub type RPC_C_AUTHN_LEVEL = u32;
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = 3;
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = 2;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = 0;
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = 1;
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = 4;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = 5;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = 6;
pub type RPC_C_IMP_LEVEL = u32;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = 1;
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = 0;
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = 4;
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = 2;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SChannelHookCallInfo {
    pub iid: windows_sys::core::GUID,
    pub cbSize: u32,
    pub uCausality: windows_sys::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut core::ffi::c_void,
}
impl Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SD_ACCESSPERMISSIONS: COMSD = 1;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2;
pub type SERVERCALL = i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0;
pub const SERVERCALL_REJECTED: SERVERCALL = 1;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2;
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1;
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0;
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut core::ffi::c_void,
}
impl Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: windows_sys::core::PWSTR,
    pub hr: windows_sys::core::HRESULT,
}
impl Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: *mut core::ffi::c_void,
    pub dwConnection: u32,
}
impl Default for STATDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STATFLAG = i32;
pub const STATFLAG_DEFAULT: STATFLAG = 0;
pub const STATFLAG_NONAME: STATFLAG = 1;
pub const STATFLAG_NOOPEN: STATFLAG = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STATSTG {
    pub pwcsName: windows_sys::core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: STGM,
    pub grfLocksSupported: u32,
    pub clsid: windows_sys::core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl Default for STATSTG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STGC = i32;
pub const STGC_CONSOLIDATE: STGC = 8;
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4;
pub const STGC_DEFAULT: STGC = 0;
pub const STGC_ONLYIFCURRENT: STGC = 2;
pub const STGC_OVERWRITE: STGC = 1;
pub type STGM = u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub u: STGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: super::super::Foundation::HGLOBAL,
    pub lpszFileName: windows_sys::core::PWSTR,
    pub pstm: *mut core::ffi::c_void,
    pub pstg: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STGM_CONVERT: STGM = 131072;
pub const STGM_CREATE: STGM = 4096;
pub const STGM_DELETEONRELEASE: STGM = 67108864;
pub const STGM_DIRECT: STGM = 0;
pub const STGM_DIRECT_SWMR: STGM = 4194304;
pub const STGM_FAILIFTHERE: STGM = 0;
pub const STGM_NOSCRATCH: STGM = 1048576;
pub const STGM_NOSNAPSHOT: STGM = 2097152;
pub const STGM_PRIORITY: STGM = 262144;
pub const STGM_READ: STGM = 0;
pub const STGM_READWRITE: STGM = 2;
pub const STGM_SHARE_DENY_NONE: STGM = 64;
pub const STGM_SHARE_DENY_READ: STGM = 48;
pub const STGM_SHARE_DENY_WRITE: STGM = 32;
pub const STGM_SHARE_EXCLUSIVE: STGM = 16;
pub const STGM_SIMPLE: STGM = 134217728;
pub const STGM_TRANSACTED: STGM = 65536;
pub const STGM_WRITE: STGM = 1;
pub type STGTY = i32;
pub const STGTY_LOCKBYTES: STGTY = 3;
pub const STGTY_PROPERTY: STGTY = 4;
pub const STGTY_REPEAT: i32 = 256;
pub const STGTY_STORAGE: STGTY = 1;
pub const STGTY_STREAM: STGTY = 2;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0;
pub const STG_TOEND: i32 = -1;
pub type STREAM_SEEK = u32;
pub const STREAM_SEEK_CUR: STREAM_SEEK = 1;
pub const STREAM_SEEK_END: STREAM_SEEK = 2;
pub const STREAM_SEEK_SET: STREAM_SEEK = 0;
pub type SYSKIND = i32;
pub const SYS_MAC: SYSKIND = 2;
pub const SYS_WIN16: SYSKIND = 0;
pub const SYS_WIN32: SYSKIND = 1;
pub const SYS_WIN64: SYSKIND = 3;
pub const ServerApplication: ApplicationType = 0;
pub type ShutdownType = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: windows_sys::core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl Default for StorageLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type THDTYPE = i32;
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = 0;
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = 1;
pub const TKIND_ALIAS: TYPEKIND = 6;
pub const TKIND_COCLASS: TYPEKIND = 5;
pub const TKIND_DISPATCH: TYPEKIND = 4;
pub const TKIND_ENUM: TYPEKIND = 0;
pub const TKIND_INTERFACE: TYPEKIND = 3;
pub const TKIND_MAX: TYPEKIND = 8;
pub const TKIND_MODULE: TYPEKIND = 2;
pub const TKIND_RECORD: TYPEKIND = 1;
pub const TKIND_UNION: TYPEKIND = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TLIBATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
pub type TYMED = i32;
pub const TYMED_ENHMF: TYMED = 64;
pub const TYMED_FILE: TYMED = 2;
pub const TYMED_GDI: TYMED = 16;
pub const TYMED_HGLOBAL: TYMED = 1;
pub const TYMED_ISTORAGE: TYMED = 8;
pub const TYMED_ISTREAM: TYMED = 4;
pub const TYMED_MFPICT: TYMED = 32;
pub const TYMED_NULL: TYMED = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: windows_sys::core::PWSTR,
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
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::Variant::VARENUM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TYPEKIND = i32;
pub type TYSPEC = i32;
pub const TYSPEC_CLSID: TYSPEC = 0;
pub const TYSPEC_FILEEXT: TYSPEC = 1;
pub const TYSPEC_FILENAME: TYSPEC = 3;
pub const TYSPEC_MIMETYPE: TYSPEC = 2;
pub const TYSPEC_OBJECTID: TYSPEC = 6;
pub const TYSPEC_PACKAGENAME: TYSPEC = 5;
pub const TYSPEC_PROGID: TYSPEC = 4;
pub type URI_CREATE_FLAGS = u32;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = 4;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = 2;
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = 1;
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = 256;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = 131072;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 512;
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 64;
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = 32;
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = 8192;
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = 8;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = 65536;
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = 16;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 1024;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 128;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = 32768;
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = 16384;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 4096;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 2048;
pub type Uri_PROPERTY = i32;
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = 0;
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = 1;
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = 2;
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = 3;
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = 18;
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = 15;
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = 4;
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = 5;
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = 6;
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = 15;
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = 7;
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = 8;
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = 9;
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = 16;
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = 10;
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = 11;
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = 17;
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = 12;
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = 14;
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = 0;
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = 13;
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = 14;
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = 18;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: windows_sys::core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VARFLAGS = u16;
pub const VARFLAG_FBINDABLE: VARFLAGS = 4;
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32;
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256;
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16;
pub const VARFLAG_FHIDDEN: VARFLAGS = 64;
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096;
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024;
pub const VARFLAG_FREADONLY: VARFLAGS = 1;
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048;
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8;
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128;
pub const VARFLAG_FSOURCE: VARFLAGS = 2;
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512;
pub type VARKIND = i32;
pub const VAR_CONST: VARKIND = 2;
pub const VAR_DISPATCH: VARKIND = 3;
pub const VAR_PERINSTANCE: VARKIND = 0;
pub const VAR_STATIC: VARKIND = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl Default for WORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union uCLSSPEC_0 {
    pub clsid: windows_sys::core::GUID,
    pub pFileExt: windows_sys::core::PWSTR,
    pub pMimeType: windows_sys::core::PWSTR,
    pub pProgId: windows_sys::core::PWSTR,
    pub pFileName: windows_sys::core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: windows_sys::core::PWSTR,
    pub PolicyId: windows_sys::core::GUID,
}
impl Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_sys::core::GUID,
    pub PolicyId: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM {
    pub u: userSTGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: windows_sys::core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
