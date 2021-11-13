#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl ::core::marker::Copy for CollectionChange {}
impl ::core::clone::Clone for CollectionChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIterable<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IIterable<T> {}
impl<T> ::core::clone::Clone for IIterable<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIterator<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IIterator<T> {}
impl<T> ::core::clone::Clone for IIterator<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyValuePair<K, V>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>);
impl<K, V> ::core::marker::Copy for IKeyValuePair<K, V> {}
impl<K, V> ::core::clone::Clone for IKeyValuePair<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMap<K, V>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>);
impl<K, V> ::core::marker::Copy for IMap<K, V> {}
impl<K, V> ::core::clone::Clone for IMap<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapChangedEventArgs<K>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>);
impl<K> ::core::marker::Copy for IMapChangedEventArgs<K> {}
impl<K> ::core::clone::Clone for IMapChangedEventArgs<K> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapView<K, V>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>);
impl<K, V> ::core::marker::Copy for IMapView<K, V> {}
impl<K, V> ::core::clone::Clone for IMapView<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObservableMap<K, V>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>);
impl<K, V> ::core::marker::Copy for IObservableMap<K, V> {}
impl<K, V> ::core::clone::Clone for IObservableMap<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObservableVector<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IObservableVector<T> {}
impl<T> ::core::clone::Clone for IObservableVector<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySet {}
impl ::core::clone::Clone for IPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IVector<T> {}
impl<T> ::core::clone::Clone for IVector<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVectorChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVectorChangedEventArgs {}
impl ::core::clone::Clone for IVectorChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVectorView<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for IVectorView<T> {}
impl<T> ::core::clone::Clone for IVectorView<T> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapChangedEventHandler<K, V>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>);
impl<K, V> ::core::marker::Copy for MapChangedEventHandler<K, V> {}
impl<K, V> ::core::clone::Clone for MapChangedEventHandler<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertySet {}
impl ::core::clone::Clone for PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StringMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StringMap {}
impl ::core::clone::Clone for StringMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ValueSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ValueSet {}
impl ::core::clone::Clone for ValueSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VectorChangedEventHandler<T>(pub *mut ::core::ffi::c_void, ::core::marker::PhantomData<T>);
impl<T> ::core::marker::Copy for VectorChangedEventHandler<T> {}
impl<T> ::core::clone::Clone for VectorChangedEventHandler<T> {
    fn clone(&self) -> Self {
        *self
    }
}
