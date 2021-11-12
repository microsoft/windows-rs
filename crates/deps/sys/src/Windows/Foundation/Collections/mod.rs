#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CollectionChange(i32);
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
