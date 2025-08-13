use std::collections::BTreeMap;
use windows::{core::*, Win32::Foundation::E_BOUNDS};
use windows_collections::*;

#[test]
fn primitive() -> Result<()> {
    let m = IMapView::<i32, u64>::from(BTreeMap::from([]));
    assert_eq!(m.Lookup(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(0)?));
    let mut left = None;
    let mut right = None;
    m.Split(&mut left, &mut right)?;

    let m = BTreeMap::from([(1, 10), (2, 20)]);
    let m: IMapView<i32, u64> = m.into();
    assert_eq!(m.Lookup(1i32)?, 10u64);
    assert_eq!(m.Lookup(2)?, 20);
    assert_eq!(m.Size()?, 2);
    assert!(m.HasKey(2)?);

    let able: IIterable<IKeyValuePair<i32, u64>> = m.cast()?;
    let m2: IMapView<i32, u64> = able.cast()?;
    assert_eq!(m, m2);

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let able = IMapView::<i32, u64>::from(BTreeMap::from([]));
    let iter = able.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let able = IMapView::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20), (3, 30)]));
    let iter = able.First()?;

    assert_eq!(iter.Current()?.Key()?, 1i32);
    assert_eq!(iter.Current()?.Value()?, 10u64);

    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?.Key()?, 2);
    assert_eq!(iter.Current()?.Value()?, 20);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?.Key()?, 3);
    assert_eq!(iter.Current()?.Value()?, 30);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert!(compare_with(&values[0], &1, &10)?);
    assert!(compare_with(&values[1], &2, &20)?);
    assert!(compare_with(&values[2], &3, &30)?);
    assert!(values[3].is_none());
    assert!(values[4].is_none());
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize_with(1, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert!(compare_with(&values[0], &1, &10)?);
    let mut values = vec![];
    values.resize_with(2, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert!(compare_with(&values[0], &2, &20)?);
    assert!(compare_with(&values[1], &3, &30)?);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

fn compare_with<K, V>(pair: &Option<IKeyValuePair<K, V>>, key: &K, value: &V) -> Result<bool>
where
    K: RuntimeType + std::cmp::PartialEq,
    V: RuntimeType + std::cmp::PartialEq,
{
    match pair {
        None => Ok(false),
        Some(pair) => Ok(&pair.Key()? == key && &pair.Value()? == value),
    }
}

#[test]
fn hstring() -> Result<()> {
    let m = IMapView::<HSTRING, i32>::from(BTreeMap::new());
    assert_eq!(m.Lookup(h!("missing")).unwrap_err().code(), E_BOUNDS);
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(h!("missing"))?));

    let m = BTreeMap::from([("one".into(), 1), ("two".into(), 2)]);
    assert!(m.contains_key(h!("one")));

    let m = IMapView::<HSTRING, i32>::from(m);
    assert_eq!(m.Lookup(h!("one"))?, 1);
    assert_eq!(m.Lookup(h!("two"))?, 2);
    assert_eq!(m.Size()?, 2);
    assert!(m.HasKey(h!("one"))?);
    assert!(!(m.HasKey(h!("three"))?));

    let able: IIterable<IKeyValuePair<HSTRING, i32>> = m.cast()?;
    let m2: IMapView<HSTRING, i32> = able.cast()?;
    assert_eq!(m, m2);

    Ok(())
}
