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
extern "system" {}
pub struct AsyncActionCompletedHandler(pub *mut ::core::ffi::c_void);
pub struct AsyncActionProgressHandler<TProgress>(pub *mut ::core::ffi::c_void);
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(pub *mut ::core::ffi::c_void);
pub struct AsyncOperationCompletedHandler<TResult>(pub *mut ::core::ffi::c_void);
pub struct AsyncOperationProgressHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void);
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void);
pub struct AsyncStatus(i32);
pub struct DateTime(i32);
pub struct Deferral(i32);
pub struct DeferralCompletedHandler(pub *mut ::core::ffi::c_void);
pub struct EventHandler<T>(pub *mut ::core::ffi::c_void);
pub struct EventRegistrationToken(i32);
pub struct FoundationContract(i32);
pub struct GuidHelper(i32);
pub struct IAsyncAction(pub *mut ::core::ffi::c_void);
pub struct IAsyncActionWithProgress<TProgress>(pub *mut ::core::ffi::c_void);
pub struct IAsyncInfo(pub *mut ::core::ffi::c_void);
pub struct IAsyncOperation<TResult>(pub *mut ::core::ffi::c_void);
pub struct IAsyncOperationWithProgress<TResult, TProgress>(pub *mut ::core::ffi::c_void);
pub struct IClosable(pub *mut ::core::ffi::c_void);
pub struct IDeferral(pub *mut ::core::ffi::c_void);
pub struct IDeferralFactory(pub *mut ::core::ffi::c_void);
pub struct IGetActivationFactory(pub *mut ::core::ffi::c_void);
pub struct IGuidHelperStatics(pub *mut ::core::ffi::c_void);
pub struct IMemoryBuffer(pub *mut ::core::ffi::c_void);
pub struct IMemoryBufferFactory(pub *mut ::core::ffi::c_void);
pub struct IMemoryBufferReference(pub *mut ::core::ffi::c_void);
pub struct IPropertyValue(pub *mut ::core::ffi::c_void);
pub struct IPropertyValueStatics(pub *mut ::core::ffi::c_void);
pub struct IReference<T>(pub *mut ::core::ffi::c_void);
pub struct IReferenceArray<T>(pub *mut ::core::ffi::c_void);
pub struct IStringable(pub *mut ::core::ffi::c_void);
pub struct IUriEscapeStatics(pub *mut ::core::ffi::c_void);
pub struct IUriRuntimeClass(pub *mut ::core::ffi::c_void);
pub struct IUriRuntimeClassFactory(pub *mut ::core::ffi::c_void);
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(pub *mut ::core::ffi::c_void);
pub struct IWwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
pub struct IWwwFormUrlDecoderRuntimeClass(pub *mut ::core::ffi::c_void);
pub struct IWwwFormUrlDecoderRuntimeClassFactory(pub *mut ::core::ffi::c_void);
pub struct MemoryBuffer(i32);
pub struct Point(i32);
pub struct PropertyType(i32);
pub struct PropertyValue(i32);
pub struct Rect(i32);
pub struct Size(i32);
pub struct TimeSpan(i32);
pub struct TypedEventHandler<TSender, TResult>(pub *mut ::core::ffi::c_void);
pub struct UniversalApiContract(i32);
pub struct Uri(i32);
pub struct WwwFormUrlDecoder(i32);
pub struct WwwFormUrlDecoderEntry(i32);
