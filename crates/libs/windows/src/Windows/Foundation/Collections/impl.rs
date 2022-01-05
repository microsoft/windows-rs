pub trait IIterableImpl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn First(&self) -> ::windows::core::Result<IIterator<T>>;
}
pub trait IIteratorImpl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Current(&self) -> ::windows::core::Result<T>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
}
pub trait IKeyValuePairImpl<K, V>: Sized
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Key(&self) -> ::windows::core::Result<K>;
    fn Value(&self) -> ::windows::core::Result<V>;
}
pub trait IMapImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn GetView(&self) -> ::windows::core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as ::windows::core::DefaultType>::DefaultType, value: &<V as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Remove(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
pub trait IMapChangedEventArgsImpl<K>: Sized
where
    K: ::windows::core::RuntimeType + 'static,
{
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Key(&self) -> ::windows::core::Result<K>;
}
pub trait IMapViewImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows::core::Result<()>;
}
pub trait IObservableMapImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>> + IMapImpl<K, V>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn MapChanged(&self, vhnd: &::core::option::Option<MapChangedEventHandler<K, V>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IObservableVectorImpl<T>: Sized + IIterableImpl<T> + IVectorImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: &::core::option::Option<VectorChangedEventHandler<T>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IPropertySetImpl: Sized + IIterableImpl<IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> + IObservableMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
pub trait IVectorImpl<T>: Sized + IIterableImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as ::windows::core::DefaultType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
pub trait IVectorChangedEventArgsImpl: Sized {
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Index(&self) -> ::windows::core::Result<u32>;
}
pub trait IVectorViewImpl<T>: Sized + IIterableImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &<T as ::windows::core::DefaultType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
}
