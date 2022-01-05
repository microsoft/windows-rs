pub trait IAsyncActionImpl: Sized + IAsyncInfoImpl {
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncActionCompletedHandler>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
pub trait IAsyncActionWithProgressImpl<TProgress>: Sized + IAsyncInfoImpl
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: &::core::option::Option<AsyncActionProgressHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncActionWithProgressCompletedHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&self) -> ::windows::core::Result<()>;
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncActionWithProgress<TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
pub trait IAsyncInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
pub trait IAsyncOperationImpl<TResult>: Sized + IAsyncInfoImpl
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncOperationCompletedHandler<TResult>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperation<TResult> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
pub trait IAsyncOperationWithProgressImpl<TResult, TProgress>: Sized + IAsyncInfoImpl
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: &::core::option::Option<AsyncOperationProgressHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
pub trait IClosableImpl: Sized {
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeferralImpl: Sized + IClosableImpl {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeferral {
    const NAME: &'static str = "Windows.Foundation.IDeferral";
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeferralFactoryImpl: Sized {
    fn Create(&self, handler: &::core::option::Option<DeferralCompletedHandler>) -> ::windows::core::Result<Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeferralFactory {
    const NAME: &'static str = "Windows.Foundation.IDeferralFactory";
}
pub trait IGetActivationFactoryImpl: Sized {
    fn GetActivationFactory(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidHelperStaticsImpl: Sized {
    fn CreateNewGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Empty(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Equals(&self, target: &::windows::core::GUID, value: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidHelperStatics {
    const NAME: &'static str = "Windows.Foundation.IGuidHelperStatics";
}
pub trait IMemoryBufferImpl: Sized + IClosableImpl {
    fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference>;
}
impl ::windows::core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryBufferFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<MemoryBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryBufferFactory {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferFactory";
}
pub trait IMemoryBufferReferenceImpl: Sized + IClosableImpl {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Closed(&self, handler: &::core::option::Option<TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>>) -> ::windows::core::Result<EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
pub trait IPropertyValueImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PropertyType>;
    fn IsNumericScalar(&self) -> ::windows::core::Result<bool>;
    fn GetUInt8(&self) -> ::windows::core::Result<u8>;
    fn GetInt16(&self) -> ::windows::core::Result<i16>;
    fn GetUInt16(&self) -> ::windows::core::Result<u16>;
    fn GetInt32(&self) -> ::windows::core::Result<i32>;
    fn GetUInt32(&self) -> ::windows::core::Result<u32>;
    fn GetInt64(&self) -> ::windows::core::Result<i64>;
    fn GetUInt64(&self) -> ::windows::core::Result<u64>;
    fn GetSingle(&self) -> ::windows::core::Result<f32>;
    fn GetDouble(&self) -> ::windows::core::Result<f64>;
    fn GetChar16(&self) -> ::windows::core::Result<u16>;
    fn GetBoolean(&self) -> ::windows::core::Result<bool>;
    fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDateTime(&self) -> ::windows::core::Result<DateTime>;
    fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan>;
    fn GetPoint(&self) -> ::windows::core::Result<Point>;
    fn GetSize(&self) -> ::windows::core::Result<Size>;
    fn GetRect(&self) -> ::windows::core::Result<Rect>;
    fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()>;
    fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()>;
    fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()>;
    fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()>;
    fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()>;
    fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()>;
    fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()>;
    fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()>;
    fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()>;
    fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()>;
    fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()>;
    fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()>;
    fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()>;
    fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()>;
    fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyValueStaticsImpl: Sized {
    fn CreateEmpty(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt8(&self, value: u8) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt16(&self, value: i16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt16(&self, value: u16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt32(&self, value: i32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt32(&self, value: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt64(&self, value: i64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt64(&self, value: u64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSingle(&self, value: f32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateChar16(&self, value: u16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateBoolean(&self, value: bool) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInspectable(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateGuid(&self, value: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDateTime(&self, value: &DateTime) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateTimeSpan(&self, value: &TimeSpan) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreatePoint(&self, value: &Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSize(&self, value: &Size) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateRect(&self, value: &Rect) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt8Array(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt16Array(&self, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt16Array(&self, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt32Array(&self, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt32Array(&self, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt64Array(&self, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt64Array(&self, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSingleArray(&self, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDoubleArray(&self, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateChar16Array(&self, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateBooleanArray(&self, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateStringArray(&self, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInspectableArray(&self, value: &[<::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateGuidArray(&self, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDateTimeArray(&self, value: &[<DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateTimeSpanArray(&self, value: &[<TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreatePointArray(&self, value: &[<Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSizeArray(&self, value: &[<Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateRectArray(&self, value: &[<Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyValueStatics {
    const NAME: &'static str = "Windows.Foundation.IPropertyValueStatics";
}
pub trait IReferenceImpl<T>: Sized + IPropertyValueImpl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<T>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
pub trait IReferenceArrayImpl<T>: Sized + IPropertyValueImpl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<::windows::core::Array<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
pub trait IStringableImpl: Sized {
    fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriEscapeStaticsImpl: Sized {
    fn UnescapeComponent(&self, tounescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EscapeComponent(&self, toescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriEscapeStatics {
    const NAME: &'static str = "Windows.Foundation.IUriEscapeStatics";
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassImpl: Sized {
    fn AbsoluteUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Extension(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Fragment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Query(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QueryParsed(&self) -> ::windows::core::Result<WwwFormUrlDecoder>;
    fn RawUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SchemeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Port(&self) -> ::windows::core::Result<i32>;
    fn Suspicious(&self) -> ::windows::core::Result<bool>;
    fn Equals(&self, puri: &::core::option::Option<Uri>) -> ::windows::core::Result<bool>;
    fn CombineUri(&self, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClass {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClass";
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassFactoryImpl: Sized {
    fn CreateUri(&self, uri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
    fn CreateWithRelativeUri(&self, baseuri: &::windows::core::HSTRING, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClassFactory {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassFactory";
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassWithAbsoluteCanonicalUriImpl: Sized {
    fn AbsoluteCanonicalUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayIri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
}
pub trait IWwwFormUrlDecoderEntryImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWwwFormUrlDecoderRuntimeClassImpl: Sized + IIterableImpl<IWwwFormUrlDecoderEntry> + IVectorViewImpl<IWwwFormUrlDecoderEntry> {
    fn GetFirstValueByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderRuntimeClass {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderRuntimeClass";
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwwFormUrlDecoderRuntimeClassFactoryImpl: Sized {
    fn CreateWwwFormUrlDecoder(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<WwwFormUrlDecoder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderRuntimeClassFactory {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory";
}
