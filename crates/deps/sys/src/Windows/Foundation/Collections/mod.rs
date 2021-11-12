#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CollectionChange(i32);
pub struct IIterable<T>(i32);
pub struct IIterator<T>(i32);
pub struct IKeyValuePair<K, V>(i32);
pub struct IMap<K, V>(i32);
pub struct IMapChangedEventArgs<K>(i32);
pub struct IMapView<K, V>(i32);
pub struct IObservableMap<K, V>(i32);
pub struct IObservableVector<T>(i32);
pub struct IPropertySet(i32);
pub struct IVector<T>(i32);
pub struct IVectorChangedEventArgs(i32);
pub struct IVectorView<T>(i32);
pub struct MapChangedEventHandler<K, V>(i32);
pub struct PropertySet(i32);
pub struct StringMap(i32);
pub struct ValueSet(i32);
pub struct VectorChangedEventHandler<T>(i32);
