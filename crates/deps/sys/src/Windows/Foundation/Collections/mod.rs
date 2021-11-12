#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CollectionChange(i32);
pub struct IIterable<T>(pub *mut ::core::ffi::c_void);
pub struct IIterator<T>(pub *mut ::core::ffi::c_void);
pub struct IKeyValuePair<K, V>(pub *mut ::core::ffi::c_void);
pub struct IMap<K, V>(pub *mut ::core::ffi::c_void);
pub struct IMapChangedEventArgs<K>(pub *mut ::core::ffi::c_void);
pub struct IMapView<K, V>(pub *mut ::core::ffi::c_void);
pub struct IObservableMap<K, V>(pub *mut ::core::ffi::c_void);
pub struct IObservableVector<T>(pub *mut ::core::ffi::c_void);
pub struct IPropertySet(pub *mut ::core::ffi::c_void);
pub struct IVector<T>(pub *mut ::core::ffi::c_void);
pub struct IVectorChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IVectorView<T>(pub *mut ::core::ffi::c_void);
pub struct MapChangedEventHandler<K, V>(pub *mut ::core::ffi::c_void);
pub struct PropertySet(i32);
pub struct StringMap(i32);
pub struct ValueSet(i32);
pub struct VectorChangedEventHandler<T>(pub *mut ::core::ffi::c_void);
