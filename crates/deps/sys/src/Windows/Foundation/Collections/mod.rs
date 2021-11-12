#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct IIterable<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIterator<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyValuePair<K, V>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMap<K, V>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapChangedEventArgs<K>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapView<K, V>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObservableMap<K, V>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObservableVector<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVectorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVectorView<T>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapChangedEventHandler<K, V>(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StringMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ValueSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VectorChangedEventHandler<T>(pub *mut ::core::ffi::c_void);
