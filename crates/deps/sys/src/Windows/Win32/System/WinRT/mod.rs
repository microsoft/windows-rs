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
    pub fn CoDecodeProxy();
    #[doc = "*Required features: `Win32_System_WinRT`, `System`*"]
    #[cfg(feature = "System")]
    pub fn CreateDispatcherQueueController();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRandomAccessStreamOnFile();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateRandomAccessStreamOverStream();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn CreateStreamOverRandomAccessStream();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn GetRestrictedErrorInfo();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree64();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal64();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize64();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsErrorPropagationEnabled();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn MetaDataGetDispenser();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoActivateInstance();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoCaptureErrorContext();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoClearError();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFailFastWithErrorContext();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFreeParameterizedTypeExtra();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetActivationFactory();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetAgileReference();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetApartmentIdentifier();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com_Marshal`*"]
    #[cfg(feature = "Win32_System_Com_Marshal")]
    pub fn RoGetBufferMarshaler();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetErrorReportingFlags();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetMatchingRestrictedErrorInfo();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoGetParameterizedTypeInstanceIID();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetServerActivatableClasses();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInitialize();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectCapturedStackBackTrace();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectThreadErrorInfo();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateError();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateErrorW();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateLanguageException();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoParameterizedTypeExtraGetTypeSignature();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterActivationFactories();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterForApartmentShutdown();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportFailedDelegate();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportUnhandledError();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoResolveRestrictedErrorInfoReference();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRevokeActivationFactories();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoSetErrorReportingFlags();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformError();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformErrorW();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUninitialize();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUnregisterForApartmentShutdown();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn SetRestrictedErrorInfo();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsCompareStringOrdinal();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsConcatString();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateString();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateStringReference();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteString();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteStringBuffer();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDuplicateString();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsGetStringLen();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsGetStringRawBuffer();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString2();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsIsStringEmpty();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPreallocateStringBuffer();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPromoteStringBuffer();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsReplaceString();
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsStringHasEmbeddedNull();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstring();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstringWithSpecifiedLength();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringEnd();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringStart();
}
