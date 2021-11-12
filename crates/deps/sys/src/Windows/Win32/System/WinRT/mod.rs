#![allow(non_snake_case, non_camel_case_types)]
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
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `System`*"]
    #[cfg(feature = "System")]
    pub fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut super::super::super::System::DispatcherQueueController) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRandomAccessStreamOnFile(filepath: super::super::Foundation::PWSTR, accessmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateRandomAccessStreamOverStream(stream: super::Com::IStream, options: BSOS_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn CreateStreamOverRandomAccessStream(randomaccessstream: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn MetaDataGetDispenser(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoActivateInstance(activatableclassid: ::windows_sys::core::HSTRING, instance: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoCaptureErrorContext(hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoClearError();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFailFastWithErrorContext(hrerror: ::windows_sys::core::HRESULT);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetActivationFactory(activatableclassid: ::windows_sys::core::HSTRING, iid: *const ::windows_sys::core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, ppagilereference: *mut IAgileReference) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com_Marshal`*"]
    #[cfg(feature = "Win32_System_Com_Marshal")]
    pub fn RoGetBufferMarshaler(buffermarshaler: *mut super::Com::Marshal::IMarshal) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows_sys::core::HRESULT, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: IRoMetaDataLocator, iid: *mut ::windows_sys::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetServerActivatableClasses(servername: ::windows_sys::core::HSTRING, activatableclassids: *mut *mut ::windows_sys::core::HSTRING, count: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateError(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateErrorW(error: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateLanguageException(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING, languageexception: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterActivationFactories(activatableclassids: *const ::windows_sys::core::HSTRING, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterForApartmentShutdown(callbackobject: IApartmentShutdown, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportFailedDelegate(punkdelegate: ::windows_sys::core::IUnknown, prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportUnhandledError(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoResolveRestrictedErrorInfoReference(reference: super::super::Foundation::PWSTR, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRevokeActivationFactories(cookie: isize);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoSetErrorReportingFlags(flags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformError(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformErrorW(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUninitialize();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn SetRestrictedErrorInfo(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsCompareStringOrdinal(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, result: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsConcatString(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateString(sourcestring: super::super::Foundation::PWSTR, length: u32, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateStringReference(sourcestring: super::super::Foundation::PWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteString(string: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDuplicateString(string: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsGetStringLen(string: ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsGetStringRawBuffer(string: ::windows_sys::core::HSTRING, length: *mut u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsIsStringEmpty(string: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsReplaceString(string: ::windows_sys::core::HSTRING, stringreplaced: ::windows_sys::core::HSTRING, stringreplacewith: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsStringHasEmbeddedNull(string: ::windows_sys::core::HSTRING, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstring(string: ::windows_sys::core::HSTRING, startindex: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstringWithSpecifiedLength(string: ::windows_sys::core::HSTRING, startindex: u32, length: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringEnd(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringStart(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
}
pub struct ACTIVATIONTYPE(i32);
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(i32);
pub struct AgileReferenceOptions(i32);
pub struct BSOS_OPTIONS(i32);
pub struct CASTING_CONNECTION_ERROR_STATUS(i32);
pub struct CASTING_CONNECTION_STATE(i32);
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_CastingTypes: &'static str = "CastingTypes";
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &'static str = "PreferredSourceUriScheme";
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_ProtectedMedia: &'static str = "ProtectedMedia";
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(i32);
pub struct DISPATCHERQUEUE_THREAD_TYPE(i32);
pub struct DispatcherQueueOptions(i32);
pub struct EventRegistrationToken(i32);
pub struct HSTRING_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HSTRING_HEADER(i32);
pub struct IAccountsSettingsPaneInterop(i32);
pub struct IActivationFactory(i32);
pub struct IAgileReference(i32);
pub struct IApartmentShutdown(i32);
pub struct IAppServiceConnectionExtendedExecution(i32);
pub struct IBufferByteAccess(i32);
pub struct ICastingController(i32);
pub struct ICastingEventHandler(i32);
pub struct ICastingSourceInfo(i32);
pub struct ICorrelationVectorInformation(i32);
pub struct ICorrelationVectorSource(i32);
pub struct IDragDropManagerInterop(i32);
pub struct IHolographicSpaceInterop(i32);
pub struct IInputPaneInterop(i32);
pub struct ILanguageExceptionErrorInfo(i32);
pub struct ILanguageExceptionErrorInfo2(i32);
pub struct ILanguageExceptionStackBackTrace(i32);
pub struct ILanguageExceptionTransform(i32);
pub struct IMemoryBufferByteAccess(i32);
pub struct IMessageDispatcher(i32);
pub struct IPlayToManagerInterop(i32);
pub struct IRestrictedErrorInfo(i32);
pub struct IRoMetaDataLocator(i32);
pub struct IRoSimpleMetaDataBuilder(i32);
pub struct IShareWindowCommandEventArgsInterop(i32);
pub struct IShareWindowCommandSourceInterop(i32);
pub struct ISpatialInteractionManagerInterop(i32);
pub struct ISystemMediaTransportControlsInterop(i32);
pub struct IUIViewSettingsInterop(i32);
pub struct IUserActivityInterop(i32);
pub struct IUserActivityRequestManagerInterop(i32);
pub struct IUserActivitySourceHostInterop(i32);
pub struct IUserConsentVerifierInterop(i32);
pub struct IWeakReference(i32);
pub struct IWeakReferenceSource(i32);
pub struct IWebAuthenticationCoreManagerInterop(i32);
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
pub struct PINSPECT_HSTRING_CALLBACK(i32);
pub struct PINSPECT_HSTRING_CALLBACK2(i32);
pub struct PINSPECT_MEMORY_CALLBACK(i32);
pub struct ROPARAMIIDHANDLE(i32);
pub struct RO_ERROR_REPORTING_FLAGS(i32);
pub struct RO_INIT_TYPE(i32);
pub struct ServerInformation(i32);
pub struct TrustLevel(i32);
pub struct _RO_REGISTRATION_COOKIE(i32);
