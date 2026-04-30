use std::collections::BTreeMap;
use windows_collections::*;
use windows_core::*;

const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as _);

#[test]
fn primitive() -> Result<()> {
    let m = IMap::<i32, u64>::from(BTreeMap::from([]));
    assert_eq!(m.Lookup(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(0)?));

    let m = IMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20)]));
    assert_eq!(m.Lookup(1i32)?, 10u64);
    assert_eq!(m.Lookup(2)?, 20);
    assert_eq!(m.Size()?, 2);
    assert!(m.HasKey(2)?);
    assert!(!(m.HasKey(99)?));

    let able: IIterable<IKeyValuePair<i32, u64>> = m.cast()?;
    let m2: IMap<i32, u64> = able.cast()?;
    assert_eq!(m, m2);

    Ok(())
}

#[test]
fn primitive_mutable() -> Result<()> {
    let m = IMap::<i32, u64>::from(BTreeMap::new());
    assert_eq!(m.Size()?, 0);

    // Insert new keys
    assert!(!(m.Insert(1, 10u64)?)); // returns false: key did not exist
    assert!(!(m.Insert(2, 20u64)?));
    assert!(!(m.Insert(3, 30u64)?));
    assert_eq!(m.Size()?, 3);
    assert_eq!(m.Lookup(1)?, 10u64);
    assert_eq!(m.Lookup(2)?, 20u64);
    assert_eq!(m.Lookup(3)?, 30u64);

    // Replace an existing key
    assert!(m.Insert(2, 200u64)?); // returns true: key was replaced
    assert_eq!(m.Lookup(2)?, 200u64);
    assert_eq!(m.Size()?, 3);

    // Remove an existing key
    m.Remove(1)?;
    assert_eq!(m.Size()?, 2);
    assert!(!(m.HasKey(1)?));

    // Remove a non-existing key returns E_BOUNDS
    assert_eq!(m.Remove(99).unwrap_err().code(), E_BOUNDS);

    // Clear
    m.Clear()?;
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(2)?));
    assert!(!(m.HasKey(3)?));

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let m = IMap::<i32, u64>::from(BTreeMap::from([]));
    let iter = m.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let m = IMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20), (3, 30)]));
    let iter = m.First()?;

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

    let iter = m.First()?;
    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert!(compare_with(&values[0], &1, &10)?);
    assert!(compare_with(&values[1], &2, &20)?);
    assert!(compare_with(&values[2], &3, &30)?);
    assert!(values[3].is_none());
    assert!(values[4].is_none());
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = m.First()?;
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

    // MoveNext followed by GetMany reads from the advanced position
    let iter = m.First()?;
    assert!(iter.MoveNext()?);
    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert!(compare_with(&values[0], &2, &20)?);
    assert!(compare_with(&values[1], &3, &30)?);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

#[test]
fn get_view() -> Result<()> {
    let m = IMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20)]));

    // GetView returns a snapshot
    let view = m.GetView()?;
    assert_eq!(view.Size()?, 2);
    assert_eq!(view.Lookup(1)?, 10u64);
    assert_eq!(view.Lookup(2)?, 20u64);

    // Mutating the map after GetView does not affect the snapshot
    m.Insert(3, 30u64)?;
    assert_eq!(m.Size()?, 3);
    assert_eq!(view.Size()?, 2);

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let m = IMap::<HSTRING, i32>::from(BTreeMap::new());
    assert_eq!(m.Lookup(h!("missing")).unwrap_err().code(), E_BOUNDS);
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(h!("missing"))?));

    assert!(!(m.Insert(h!("one"), 1)?));
    assert!(!(m.Insert(h!("two"), 2)?));
    assert_eq!(m.Size()?, 2);
    assert_eq!(m.Lookup(h!("one"))?, 1);
    assert_eq!(m.Lookup(h!("two"))?, 2);
    assert!(m.HasKey(h!("one"))?);
    assert!(!(m.HasKey(h!("three"))?));

    // Replace
    assert!(m.Insert(h!("one"), 100)?);
    assert_eq!(m.Lookup(h!("one"))?, 100);

    m.Remove(h!("one"))?;
    assert!(!(m.HasKey(h!("one"))?));
    assert_eq!(m.Size()?, 1);

    m.Clear()?;
    assert_eq!(m.Size()?, 0);

    let able: IIterable<IKeyValuePair<HSTRING, i32>> = m.cast()?;
    let m2: IMap<HSTRING, i32> = able.cast()?;
    assert_eq!(m, m2);

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
