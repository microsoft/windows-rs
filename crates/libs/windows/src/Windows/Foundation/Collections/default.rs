impl ::core::default::Default for CollectionChange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CollectionChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CollectionChange").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IIterable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IIterable<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IIterable<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIterable").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IIterator<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IIterator<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIterator").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IKeyValuePair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IKeyValuePair<K, V> {}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IKeyValuePair<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKeyValuePair").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IMap<K, V> {}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IMap<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMap").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IMapChangedEventArgs<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IMapChangedEventArgs<K> {}
impl<K: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IMapChangedEventArgs<K> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapChangedEventArgs").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IMapView<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IMapView<K, V> {}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IMapView<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapView").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IObservableMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IObservableMap<K, V> {}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IObservableMap<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObservableMap").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IObservableVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IObservableVector<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IObservableVector<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObservableVector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySet {}
impl ::core::fmt::Debug for IPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySet").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IVector<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IVector<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVectorChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVectorChangedEventArgs {}
impl ::core::fmt::Debug for IVectorChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVectorChangedEventArgs").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IVectorView<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IVectorView<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IVectorView<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVectorView").field(&self.0).finish()
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for MapChangedEventHandler<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for MapChangedEventHandler<K, V> {}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for MapChangedEventHandler<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertySet {}
impl ::core::fmt::Debug for PropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertySet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StringMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StringMap {}
impl ::core::fmt::Debug for StringMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StringMap").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValueSet {}
impl ::core::fmt::Debug for ValueSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueSet").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for VectorChangedEventHandler<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for VectorChangedEventHandler<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for VectorChangedEventHandler<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VectorChangedEventHandler").field(&self.0).finish()
    }
}
