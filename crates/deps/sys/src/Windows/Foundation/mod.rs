#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: AsyncStatus = AsyncStatus(2i32);
    pub const Completed: AsyncStatus = AsyncStatus(1i32);
    pub const Error: AsyncStatus = AsyncStatus(3i32);
    pub const Started: AsyncStatus = AsyncStatus(0i32);
}
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
#[repr(transparent)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: PropertyType = PropertyType(0i32);
    pub const UInt8: PropertyType = PropertyType(1i32);
    pub const Int16: PropertyType = PropertyType(2i32);
    pub const UInt16: PropertyType = PropertyType(3i32);
    pub const Int32: PropertyType = PropertyType(4i32);
    pub const UInt32: PropertyType = PropertyType(5i32);
    pub const Int64: PropertyType = PropertyType(6i32);
    pub const UInt64: PropertyType = PropertyType(7i32);
    pub const Single: PropertyType = PropertyType(8i32);
    pub const Double: PropertyType = PropertyType(9i32);
    pub const Char16: PropertyType = PropertyType(10i32);
    pub const Boolean: PropertyType = PropertyType(11i32);
    pub const String: PropertyType = PropertyType(12i32);
    pub const Inspectable: PropertyType = PropertyType(13i32);
    pub const DateTime: PropertyType = PropertyType(14i32);
    pub const TimeSpan: PropertyType = PropertyType(15i32);
    pub const Guid: PropertyType = PropertyType(16i32);
    pub const Point: PropertyType = PropertyType(17i32);
    pub const Size: PropertyType = PropertyType(18i32);
    pub const Rect: PropertyType = PropertyType(19i32);
    pub const OtherType: PropertyType = PropertyType(20i32);
    pub const UInt8Array: PropertyType = PropertyType(1025i32);
    pub const Int16Array: PropertyType = PropertyType(1026i32);
    pub const UInt16Array: PropertyType = PropertyType(1027i32);
    pub const Int32Array: PropertyType = PropertyType(1028i32);
    pub const UInt32Array: PropertyType = PropertyType(1029i32);
    pub const Int64Array: PropertyType = PropertyType(1030i32);
    pub const UInt64Array: PropertyType = PropertyType(1031i32);
    pub const SingleArray: PropertyType = PropertyType(1032i32);
    pub const DoubleArray: PropertyType = PropertyType(1033i32);
    pub const Char16Array: PropertyType = PropertyType(1034i32);
    pub const BooleanArray: PropertyType = PropertyType(1035i32);
    pub const StringArray: PropertyType = PropertyType(1036i32);
    pub const InspectableArray: PropertyType = PropertyType(1037i32);
    pub const DateTimeArray: PropertyType = PropertyType(1038i32);
    pub const TimeSpanArray: PropertyType = PropertyType(1039i32);
    pub const GuidArray: PropertyType = PropertyType(1040i32);
    pub const PointArray: PropertyType = PropertyType(1041i32);
    pub const SizeArray: PropertyType = PropertyType(1042i32);
    pub const RectArray: PropertyType = PropertyType(1043i32);
    pub const OtherTypeArray: PropertyType = PropertyType(1044i32);
}
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
