#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Foundation_Collections")]
pub mod Collections;
#[cfg(feature = "Foundation_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Foundation_Metadata")]
pub mod Metadata;
#[cfg(feature = "Foundation_Numerics")]
pub mod Numerics;
#[link(name = "windows")]
extern "system" {
    fn AsyncActionCompletedHandler();
    fn AsyncActionProgressHandler();
    fn AsyncActionWithProgressCompletedHandler();
    fn AsyncOperationCompletedHandler();
    fn AsyncOperationProgressHandler();
    fn AsyncOperationWithProgressCompletedHandler();
    fn AsyncStatus();
    fn DateTime();
    fn Deferral();
    fn DeferralCompletedHandler();
    fn EventHandler();
    fn EventRegistrationToken();
    fn FoundationContract();
    fn GuidHelper();
    fn IAsyncAction();
    fn IAsyncActionWithProgress();
    fn IAsyncInfo();
    fn IAsyncOperation();
    fn IAsyncOperationWithProgress();
    fn IClosable();
    fn IDeferral();
    fn IDeferralFactory();
    fn IGetActivationFactory();
    fn IGuidHelperStatics();
    fn IMemoryBuffer();
    fn IMemoryBufferFactory();
    fn IMemoryBufferReference();
    fn IPropertyValue();
    fn IPropertyValueStatics();
    fn IReference();
    fn IReferenceArray();
    fn IStringable();
    fn IUriEscapeStatics();
    fn IUriRuntimeClass();
    fn IUriRuntimeClassFactory();
    fn IUriRuntimeClassWithAbsoluteCanonicalUri();
    fn IWwwFormUrlDecoderEntry();
    fn IWwwFormUrlDecoderRuntimeClass();
    fn IWwwFormUrlDecoderRuntimeClassFactory();
    fn MemoryBuffer();
    fn Point();
    fn PropertyType();
    fn PropertyValue();
    fn Rect();
    fn Size();
    fn TimeSpan();
    fn TypedEventHandler();
    fn UniversalApiContract();
    fn Uri();
    fn WwwFormUrlDecoder();
    fn WwwFormUrlDecoderEntry();
}
