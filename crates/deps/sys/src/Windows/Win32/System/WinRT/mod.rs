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
    fn ACTIVATIONTYPE();
    fn APARTMENT_SHUTDOWN_REGISTRATION_COOKIE();
    fn AgileReferenceOptions();
    fn BSOS_OPTIONS();
    fn CASTING_CONNECTION_ERROR_STATUS();
    fn CASTING_CONNECTION_STATE();
    fn CastingSourceInfo_Property_CastingTypes();
    fn CastingSourceInfo_Property_PreferredSourceUriScheme();
    fn CastingSourceInfo_Property_ProtectedMedia();
    fn CoDecodeProxy();
    fn CreateDispatcherQueueController();
    fn CreateRandomAccessStreamOnFile();
    fn CreateRandomAccessStreamOverStream();
    fn CreateStreamOverRandomAccessStream();
    fn DISPATCHERQUEUE_THREAD_APARTMENTTYPE();
    fn DISPATCHERQUEUE_THREAD_TYPE();
    fn DispatcherQueueOptions();
    fn EventRegistrationToken();
    fn GetRestrictedErrorInfo();
    fn HSTRING_BUFFER();
    fn HSTRING_HEADER();
    fn HSTRING_UserFree();
    fn HSTRING_UserFree64();
    fn HSTRING_UserMarshal();
    fn HSTRING_UserMarshal64();
    fn HSTRING_UserSize();
    fn HSTRING_UserSize64();
    fn HSTRING_UserUnmarshal();
    fn HSTRING_UserUnmarshal64();
    fn IAccountsSettingsPaneInterop();
    fn IActivationFactory();
    fn IAgileReference();
    fn IApartmentShutdown();
    fn IAppServiceConnectionExtendedExecution();
    fn IBufferByteAccess();
    fn ICastingController();
    fn ICastingEventHandler();
    fn ICastingSourceInfo();
    fn ICorrelationVectorInformation();
    fn ICorrelationVectorSource();
    fn IDragDropManagerInterop();
    fn IHolographicSpaceInterop();
    fn IInputPaneInterop();
    fn ILanguageExceptionErrorInfo();
    fn ILanguageExceptionErrorInfo2();
    fn ILanguageExceptionStackBackTrace();
    fn ILanguageExceptionTransform();
    fn IMemoryBufferByteAccess();
    fn IMessageDispatcher();
    fn IPlayToManagerInterop();
    fn IRestrictedErrorInfo();
    fn IRoMetaDataLocator();
    fn IRoSimpleMetaDataBuilder();
    fn IShareWindowCommandEventArgsInterop();
    fn IShareWindowCommandSourceInterop();
    fn ISpatialInteractionManagerInterop();
    fn ISystemMediaTransportControlsInterop();
    fn IUIViewSettingsInterop();
    fn IUserActivityInterop();
    fn IUserActivityRequestManagerInterop();
    fn IUserActivitySourceHostInterop();
    fn IUserConsentVerifierInterop();
    fn IWeakReference();
    fn IWeakReferenceSource();
    fn IWebAuthenticationCoreManagerInterop();
    fn IsErrorPropagationEnabled();
    fn MAX_ERROR_MESSAGE_CHARS();
    fn MetaDataGetDispenser();
    fn PINSPECT_HSTRING_CALLBACK();
    fn PINSPECT_HSTRING_CALLBACK2();
    fn PINSPECT_MEMORY_CALLBACK();
    fn ROPARAMIIDHANDLE();
    fn RO_ERROR_REPORTING_FLAGS();
    fn RO_INIT_TYPE();
    fn RoActivateInstance();
    fn RoCaptureErrorContext();
    fn RoClearError();
    fn RoFailFastWithErrorContext();
    fn RoFreeParameterizedTypeExtra();
    fn RoGetActivationFactory();
    fn RoGetAgileReference();
    fn RoGetApartmentIdentifier();
    fn RoGetBufferMarshaler();
    fn RoGetErrorReportingFlags();
    fn RoGetMatchingRestrictedErrorInfo();
    fn RoGetParameterizedTypeInstanceIID();
    fn RoGetServerActivatableClasses();
    fn RoInitialize();
    fn RoInspectCapturedStackBackTrace();
    fn RoInspectThreadErrorInfo();
    fn RoOriginateError();
    fn RoOriginateErrorW();
    fn RoOriginateLanguageException();
    fn RoParameterizedTypeExtraGetTypeSignature();
    fn RoRegisterActivationFactories();
    fn RoRegisterForApartmentShutdown();
    fn RoReportFailedDelegate();
    fn RoReportUnhandledError();
    fn RoResolveRestrictedErrorInfoReference();
    fn RoRevokeActivationFactories();
    fn RoSetErrorReportingFlags();
    fn RoTransformError();
    fn RoTransformErrorW();
    fn RoUninitialize();
    fn RoUnregisterForApartmentShutdown();
    fn ServerInformation();
    fn SetRestrictedErrorInfo();
    fn TrustLevel();
    fn WindowsCompareStringOrdinal();
    fn WindowsConcatString();
    fn WindowsCreateString();
    fn WindowsCreateStringReference();
    fn WindowsDeleteString();
    fn WindowsDeleteStringBuffer();
    fn WindowsDuplicateString();
    fn WindowsGetStringLen();
    fn WindowsGetStringRawBuffer();
    fn WindowsInspectString();
    fn WindowsInspectString2();
    fn WindowsIsStringEmpty();
    fn WindowsPreallocateStringBuffer();
    fn WindowsPromoteStringBuffer();
    fn WindowsReplaceString();
    fn WindowsStringHasEmbeddedNull();
    fn WindowsSubstring();
    fn WindowsSubstringWithSpecifiedLength();
    fn WindowsTrimStringEnd();
    fn WindowsTrimStringStart();
    fn _RO_REGISTRATION_COOKIE();
}
