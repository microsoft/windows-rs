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
    pub fn BindMoniker();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgID();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromProgIDEx();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLSIDFromString();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAddRefServerProcess();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowSetForegroundWindow();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoAllowUnmarshalerCLSID();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoBuildVersion();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCancelCall();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCopyProxy();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateFreeThreadedMarshaler();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateGuid();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstance();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoCreateInstanceEx();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoCreateInstanceFromApp();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDecrementMTAUsage();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisableCallCancellation();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectContext();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoDisconnectObject();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoDosDateTimeToFileTime();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoEnableCallCancellation();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeNow();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFileTimeToDosDateTime();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeAllLibraries();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoFreeLibrary();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeUnusedLibraries();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoFreeUnusedLibrariesEx();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetApartmentType();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallContext();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCallerTID();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCancelObject();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetClassObject();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetContextToken();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentLogicalThreadId();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetCurrentProcess();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetMalloc();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetObject();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetObjectContext();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetPSClsid();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoGetSystemSecurityPermissions();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoGetTreatAsClass();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoImpersonateClient();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoIncrementMTAUsage();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitialize();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoInitializeEx();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CoInitializeSecurity();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInstall();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInvalidateRemoteMachineBindings();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsHandlerConnected();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoIsOle1Class();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLoadLibrary();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoLockObjectExternal();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryAuthenticationServices();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryClientBlanket();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoQueryProxyBlanket();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterActivationFilter();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterChannelHook();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterClassObject();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoRegisterDeviceCatalog();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterInitializeSpy();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterMallocSpy();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterPSClsid();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRegisterSurrogate();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoReleaseServerProcess();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoResumeClassObjects();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevertToSelf();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeClassObject();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeDeviceCatalog();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeInitializeSpy();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoRevokeMallocSpy();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSetCancelObject();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoSetProxyBlanket();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSuspendClassObjects();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoSwitchCallContext();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemAlloc();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemFree();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTaskMemRealloc();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTestCancel();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoTreatAsClass();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CoUninitialize();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleHandles();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoWaitForMultipleObjects();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateAntiMoniker();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateBindCtx();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateClassMoniker();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataAdviseHolder();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateDataCache();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFileMoniker();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateGenericComposite();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateIUriBuilder();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateItemMoniker();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreateObjrefMoniker();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn CreatePointerMoniker();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdProgressIndicator();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUri();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriFromMultiByteString();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUriWithFragment();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn DcomChannelSetHResult();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFile();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetErrorInfo();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn GetRunningObjectTable();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IIDFromString();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayName();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn MonikerCommonPrefixWith();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonikerRelativePathTo();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProgIDFromCLSID();
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub fn SetErrorInfo();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromCLSID();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromGUID2();
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StringFromIID();
}
