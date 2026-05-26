use std::collections::BTreeMap;
use windows_collections::*;
use windows_core::*;
#[cfg(windows)]
use windows_reference::IReference;

#[test]
fn vector_i32() {
    let v = IVector::<i32>::from(vec![1, 2, 3]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.Collections.IVector`1<Int32>");
}

#[test]
fn vector_string() {
    let v = IVector::<HSTRING>::from(vec![HSTRING::from("a")]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.Collections.IVector`1<String>");
}

#[test]
fn vector_view_i32() {
    let v = IVectorView::<i32>::from(vec![1, 2, 3]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.Collections.IVectorView`1<Int32>");
}

#[test]
fn iterable_i32() {
    let v = IIterable::<i32>::from(vec![1, 2, 3]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.Collections.IIterable`1<Int32>");
}

#[test]
fn map_string_i32() {
    let mut map = BTreeMap::new();
    map.insert(HSTRING::from("a"), 1i32);
    let m = IMap::<HSTRING, i32>::from(map);
    let inspectable: IInspectable = m.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.Collections.IMap`2<String, Int32>"
    );
}

#[test]
fn map_view_string_i32() {
    let mut map = BTreeMap::new();
    map.insert(HSTRING::from("a"), 1i32);
    let m = IMapView::<HSTRING, i32>::from(map);
    let inspectable: IInspectable = m.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.Collections.IMapView`2<String, Int32>"
    );
}

#[test]
fn observable_vector_i32() {
    let v = IObservableVector::<i32>::from(vec![1, 2, 3]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.Collections.IObservableVector`1<Int32>"
    );
}

#[test]
fn observable_map_string_i32() {
    let mut map = BTreeMap::new();
    map.insert(HSTRING::from("a"), 1i32);
    let m = IObservableMap::<HSTRING, i32>::from(map);
    let inspectable: IInspectable = m.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.Collections.IObservableMap`2<String, Int32>"
    );
}

#[test]
fn nested_vector_of_reference() {
    // IVector<IReference<Single>> - tests nested generic type name composition
    let v = IVector::<IReference<f32>>::from(vec![Some(IReference::<f32>::from(2.5f32))]);
    let inspectable: IInspectable = v.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.Collections.IVector`1<Windows.Foundation.IReference`1<Single>>"
    );
}
