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
#[repr(transparent)]
pub struct AsyncActionCompletedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncActionProgressHandler<TProgress>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncOperationCompletedHandler<TResult>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AsyncStatus(i32);
#[repr(C)]
pub struct DateTime(i32);
#[repr(transparent)]
pub struct Deferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeferralCompletedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EventHandler<T>(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EventRegistrationToken(i32);
#[repr(C)]
pub struct FoundationContract(i32);
#[repr(transparent)]
pub struct IAsyncAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncActionWithProgress<TProgress>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncOperation<TResult>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClosable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeferralFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetActivationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryBufferFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryBufferReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReference<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceArray<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStringable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUriEscapeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUriRuntimeClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUriRuntimeClassFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MemoryBuffer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Point(i32);
#[repr(C)]
pub struct PropertyType(i32);
#[repr(C)]
pub struct Rect(i32);
#[repr(C)]
pub struct Size(i32);
#[repr(C)]
pub struct TimeSpan(i32);
#[repr(transparent)]
pub struct TypedEventHandler<TSender, TResult>(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UniversalApiContract(i32);
#[repr(transparent)]
pub struct Uri(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwwFormUrlDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
