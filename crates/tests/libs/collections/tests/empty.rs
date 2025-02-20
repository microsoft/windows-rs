use std::collections::BTreeMap;
use windows_collections::*;

#[test]
fn iterable() {
    let empty = IIterable::<i32>::from(vec![]);
    let mut count = 0;

    for _ in &empty {
        count += 1;
    }

    assert_eq!(count, 0);
    let sum: i32 = empty.into_iter().sum();
    assert_eq!(sum, 0);
}

#[test]
fn vector_view() {
    let empty = IVectorView::<i32>::from(vec![]);
    let mut count = 0;

    for _ in &empty {
        count += 1;
    }

    assert_eq!(count, 0);
    let sum: i32 = empty.into_iter().sum();
    assert_eq!(sum, 0);
}

#[test]
fn map_view() {
    let empty = IMapView::<i32, i32>::from(BTreeMap::from([]));
    let mut count = 0;

    for _ in &empty {
        count += 1;
    }

    assert_eq!(count, 0);

    let sum: i32 = empty
        .into_iter()
        .map(|pair| pair.Key().unwrap() + pair.Value().unwrap())
        .sum();

    assert_eq!(sum, 0);
}
