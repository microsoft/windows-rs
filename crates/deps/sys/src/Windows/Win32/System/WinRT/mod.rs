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
    fn CoDecodeProxy();
    fn CreateDispatcherQueueController();
    fn CreateRandomAccessStreamOnFile();
    fn CreateRandomAccessStreamOverStream();
    fn CreateStreamOverRandomAccessStream();
    fn GetRestrictedErrorInfo();
    fn HSTRING_UserFree();
    fn HSTRING_UserFree64();
    fn HSTRING_UserMarshal();
    fn HSTRING_UserMarshal64();
    fn HSTRING_UserSize();
    fn HSTRING_UserSize64();
    fn HSTRING_UserUnmarshal();
    fn HSTRING_UserUnmarshal64();
    fn IsErrorPropagationEnabled();
    fn MetaDataGetDispenser();
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
    fn SetRestrictedErrorInfo();
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
}
