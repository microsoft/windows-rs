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
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
#[repr(C)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl ::core::marker::Copy for DateTime {}
impl ::core::clone::Clone for DateTime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Deferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeferralCompletedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EventHandler<T>(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EventRegistrationToken {
    pub Value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl ::core::marker::Copy for Point {}
impl ::core::clone::Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
#[repr(C)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Rect {}
impl ::core::clone::Clone for Rect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Size {}
impl ::core::clone::Clone for Size {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TimeSpan {
    pub Duration: i64,
}
impl ::core::marker::Copy for TimeSpan {}
impl ::core::clone::Clone for TimeSpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TypedEventHandler<TSender, TResult>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Uri(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwwFormUrlDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
