#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CollectionChange();
    fn IIterable();
    fn IIterator();
    fn IKeyValuePair();
    fn IMap();
    fn IMapChangedEventArgs();
    fn IMapView();
    fn IObservableMap();
    fn IObservableVector();
    fn IPropertySet();
    fn IVector();
    fn IVectorChangedEventArgs();
    fn IVectorView();
    fn MapChangedEventHandler();
    fn PropertySet();
    fn StringMap();
    fn ValueSet();
    fn VectorChangedEventHandler();
}
