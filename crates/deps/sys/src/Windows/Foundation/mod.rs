#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for AsyncActionCompletedHandler {}
impl ::core::clone::Clone for AsyncActionCompletedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncActionProgressHandler<TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TProgress>);
impl<TProgress> ::core::marker::Copy for AsyncActionProgressHandler<TProgress> {}
impl<TProgress> ::core::clone::Clone for AsyncActionProgressHandler<TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TProgress>);
impl<TProgress> ::core::marker::Copy for AsyncActionWithProgressCompletedHandler<TProgress> {}
impl<TProgress> ::core::clone::Clone for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncOperationCompletedHandler<TResult>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TResult>);
impl<TResult> ::core::marker::Copy for AsyncOperationCompletedHandler<TResult> {}
impl<TResult> ::core::clone::Clone for AsyncOperationCompletedHandler<TResult> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>);
impl<TResult, TProgress> ::core::marker::Copy for AsyncOperationProgressHandler<TResult, TProgress> {}
impl<TResult, TProgress> ::core::clone::Clone for AsyncOperationProgressHandler<TResult, TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>);
impl<TResult, TProgress> ::core::marker::Copy for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {}
impl<TResult, TProgress> ::core::clone::Clone for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl ::core::marker::Copy for AsyncStatus {}
impl ::core::clone::Clone for AsyncStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for Deferral {}
impl ::core::clone::Clone for Deferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeferralCompletedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeferralCompletedHandler {}
impl ::core::clone::Clone for DeferralCompletedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EventHandler<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for EventHandler<T> {}
impl<T> ::core::clone::Clone for EventHandler<T> {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IAsyncAction {}
impl ::core::clone::Clone for IAsyncAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncActionWithProgress<TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TProgress>);
impl<TProgress> ::core::marker::Copy for IAsyncActionWithProgress<TProgress> {}
impl<TProgress> ::core::clone::Clone for IAsyncActionWithProgress<TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsyncInfo {}
impl ::core::clone::Clone for IAsyncInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncOperation<TResult>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TResult>);
impl<TResult> ::core::marker::Copy for IAsyncOperation<TResult> {}
impl<TResult> ::core::clone::Clone for IAsyncOperation<TResult> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>);
impl<TResult, TProgress> ::core::marker::Copy for IAsyncOperationWithProgress<TResult, TProgress> {}
impl<TResult, TProgress> ::core::clone::Clone for IAsyncOperationWithProgress<TResult, TProgress> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClosable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClosable {}
impl ::core::clone::Clone for IClosable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeferral {}
impl ::core::clone::Clone for IDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeferralFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeferralFactory {}
impl ::core::clone::Clone for IDeferralFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetActivationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetActivationFactory {}
impl ::core::clone::Clone for IGetActivationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidHelperStatics {}
impl ::core::clone::Clone for IGuidHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryBuffer {}
impl ::core::clone::Clone for IMemoryBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryBufferFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryBufferFactory {}
impl ::core::clone::Clone for IMemoryBufferFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryBufferReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryBufferReference {}
impl ::core::clone::Clone for IMemoryBufferReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyValue {}
impl ::core::clone::Clone for IPropertyValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyValueStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyValueStatics {}
impl ::core::clone::Clone for IPropertyValueStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReference<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IReference<T> {}
impl<T> ::core::clone::Clone for IReference<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceArray<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IReferenceArray<T> {}
impl<T> ::core::clone::Clone for IReferenceArray<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStringable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStringable {}
impl ::core::clone::Clone for IStringable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriEscapeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriEscapeStatics {}
impl ::core::clone::Clone for IUriEscapeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriRuntimeClass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriRuntimeClass {}
impl ::core::clone::Clone for IUriRuntimeClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriRuntimeClassFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriRuntimeClassFactory {}
impl ::core::clone::Clone for IUriRuntimeClassFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriRuntimeClassWithAbsoluteCanonicalUri {}
impl ::core::clone::Clone for IUriRuntimeClassWithAbsoluteCanonicalUri {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWwwFormUrlDecoderEntry {}
impl ::core::clone::Clone for IWwwFormUrlDecoderEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWwwFormUrlDecoderRuntimeClass {}
impl ::core::clone::Clone for IWwwFormUrlDecoderRuntimeClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWwwFormUrlDecoderRuntimeClassFactory {}
impl ::core::clone::Clone for IWwwFormUrlDecoderRuntimeClassFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MemoryBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MemoryBuffer {}
impl ::core::clone::Clone for MemoryBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PropertyType {}
impl ::core::clone::Clone for PropertyType {
    fn clone(&self) -> Self {
        *self
    }
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
pub struct TypedEventHandler<TSender, TResult>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<TSender>, ::core::marker::PhantomData<TResult>);
impl<TSender, TResult> ::core::marker::Copy for TypedEventHandler<TSender, TResult> {}
impl<TSender, TResult> ::core::clone::Clone for TypedEventHandler<TSender, TResult> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Uri(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Uri {}
impl ::core::clone::Clone for Uri {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WwwFormUrlDecoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WwwFormUrlDecoder {}
impl ::core::clone::Clone for WwwFormUrlDecoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WwwFormUrlDecoderEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WwwFormUrlDecoderEntry {}
impl ::core::clone::Clone for WwwFormUrlDecoderEntry {
    fn clone(&self) -> Self {
        *self
    }
}
