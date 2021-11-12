#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System_WinRT_Xaml")]
pub mod Xaml;
#[link(name = "windows")]
extern "system" {
    pub fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "System")]
    pub fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut super::super::super::System::DispatcherQueueController) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRandomAccessStreamOnFile(filepath: super::super::Foundation::PWSTR, accessmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateRandomAccessStreamOverStream(stream: super::Com::IStream, options: BSOS_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CreateStreamOverRandomAccessStream(randomaccessstream: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    pub fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    pub fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    pub fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    pub fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    pub fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    pub fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    pub fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
    pub fn MetaDataGetDispenser(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn RoActivateInstance(activatableclassid: ::windows_sys::core::HSTRING, instance: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
    pub fn RoCaptureErrorContext(hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    pub fn RoClearError();
    pub fn RoFailFastWithErrorContext(hrerror: ::windows_sys::core::HRESULT);
    pub fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
    pub fn RoGetActivationFactory(activatableclassid: ::windows_sys::core::HSTRING, iid: *const ::windows_sys::core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, ppagilereference: *mut IAgileReference) -> ::windows_sys::core::HRESULT;
    pub fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com_Marshal")]
    pub fn RoGetBufferMarshaler(buffermarshaler: *mut super::Com::Marshal::IMarshal) -> ::windows_sys::core::HRESULT;
    pub fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows_sys::core::HRESULT, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: IRoMetaDataLocator, iid: *mut ::windows_sys::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_sys::core::HRESULT;
    pub fn RoGetServerActivatableClasses(servername: ::windows_sys::core::HSTRING, activatableclassids: *mut *mut ::windows_sys::core::HSTRING, count: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows_sys::core::HRESULT;
    pub fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateError(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateErrorW(error: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateLanguageException(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING, languageexception: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> super::super::Foundation::PSTR;
    pub fn RoRegisterActivationFactories(activatableclassids: *const ::windows_sys::core::HSTRING, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows_sys::core::HRESULT;
    pub fn RoRegisterForApartmentShutdown(callbackobject: IApartmentShutdown, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn RoReportFailedDelegate(punkdelegate: ::windows_sys::core::IUnknown, prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn RoReportUnhandledError(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoResolveRestrictedErrorInfoReference(reference: super::super::Foundation::PWSTR, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn RoRevokeActivationFactories(cookie: isize);
    pub fn RoSetErrorReportingFlags(flags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformError(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformErrorW(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    pub fn RoUninitialize();
    pub fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    pub fn SetRestrictedErrorInfo(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn WindowsCompareStringOrdinal(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, result: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn WindowsConcatString(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateString(sourcestring: super::super::Foundation::PWSTR, length: u32, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateStringReference(sourcestring: super::super::Foundation::PWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsDeleteString(string: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    pub fn WindowsDuplicateString(string: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsGetStringLen(string: ::windows_sys::core::HSTRING) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsGetStringRawBuffer(string: ::windows_sys::core::HSTRING, length: *mut u32) -> super::super::Foundation::PWSTR;
    pub fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsIsStringEmpty(string: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    pub fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    pub fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsReplaceString(string: ::windows_sys::core::HSTRING, stringreplaced: ::windows_sys::core::HSTRING, stringreplacewith: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsStringHasEmbeddedNull(string: ::windows_sys::core::HSTRING, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn WindowsSubstring(string: ::windows_sys::core::HSTRING, startindex: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsSubstringWithSpecifiedLength(string: ::windows_sys::core::HSTRING, startindex: u32, length: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsTrimStringEnd(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    pub fn WindowsTrimStringStart(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct ACTIVATIONTYPE(pub i32);
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
#[repr(C)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(i32);
#[repr(transparent)]
pub struct AgileReferenceOptions(pub i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
#[repr(transparent)]
pub struct BSOS_OPTIONS(pub i32);
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
#[repr(transparent)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
#[repr(transparent)]
pub struct CASTING_CONNECTION_STATE(pub i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
pub const CastingSourceInfo_Property_CastingTypes: &'static str = "CastingTypes";
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &'static str = "PreferredSourceUriScheme";
pub const CastingSourceInfo_Property_ProtectedMedia: &'static str = "ProtectedMedia";
#[repr(transparent)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
#[repr(transparent)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
#[repr(C)]
pub struct DispatcherQueueOptions(i32);
#[repr(C)]
pub struct EventRegistrationToken(i32);
#[repr(C)]
pub struct HSTRING_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HSTRING_HEADER(i32);
#[repr(transparent)]
pub struct IAccountsSettingsPaneInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAgileReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApartmentShutdown(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceConnectionExtendedExecution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferByteAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingSourceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorrelationVectorInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorrelationVectorSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragDropManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageExceptionStackBackTrace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageExceptionTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryBufferByteAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageDispatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRestrictedErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoMetaDataLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoSimpleMetaDataBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareWindowCommandEventArgsInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareWindowCommandSourceInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIViewSettingsInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityRequestManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivitySourceHostInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserConsentVerifierInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWeakReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWeakReferenceSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerInterop(pub *mut ::core::ffi::c_void);
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[repr(C)]
pub struct PINSPECT_HSTRING_CALLBACK(i32);
#[repr(C)]
pub struct PINSPECT_HSTRING_CALLBACK2(i32);
#[repr(C)]
pub struct PINSPECT_MEMORY_CALLBACK(i32);
#[repr(C)]
pub struct ROPARAMIIDHANDLE(i32);
#[repr(transparent)]
pub struct RO_ERROR_REPORTING_FLAGS(pub u32);
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0u32);
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1u32);
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2u32);
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4u32);
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8u32);
#[repr(transparent)]
pub struct RO_INIT_TYPE(pub i32);
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
#[repr(C)]
pub struct ServerInformation(i32);
#[repr(transparent)]
pub struct TrustLevel(pub i32);
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
pub const FullTrust: TrustLevel = TrustLevel(2i32);
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(i32);
