pub trait IBindableIterableImpl: Sized {
    fn First(&self) -> ::windows::core::Result<IBindableIterator>;
}
pub trait IBindableIteratorImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
}
pub trait IBindableObservableVectorImpl: Sized + IBindableIterableImpl + IBindableVectorImpl {
    fn VectorChanged(&self, handler: &::core::option::Option<BindableVectorChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IBindableVectorImpl: Sized + IBindableIterableImpl {
    fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IBindableVectorView>;
    fn IndexOf(&self, value: &::core::option::Option<::windows::core::IInspectable>, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
pub trait IBindableVectorViewImpl: Sized + IBindableIterableImpl {
    fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &::core::option::Option<::windows::core::IInspectable>, index: &mut u32) -> ::windows::core::Result<bool>;
}
pub trait INotifyCollectionChangedImpl: Sized {
    fn CollectionChanged(&self, handler: &::core::option::Option<NotifyCollectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyCollectionChangedEventArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<NotifyCollectionChangedAction>;
    fn NewItems(&self) -> ::windows::core::Result<IBindableVector>;
    fn OldItems(&self) -> ::windows::core::Result<IBindableVector>;
    fn NewStartingIndex(&self) -> ::windows::core::Result<i32>;
    fn OldStartingIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyCollectionChangedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithAllParameters(&self, action: NotifyCollectionChangedAction, newitems: &::core::option::Option<IBindableVector>, olditems: &::core::option::Option<IBindableVector>, newindex: i32, oldindex: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NotifyCollectionChangedEventArgs>;
}
