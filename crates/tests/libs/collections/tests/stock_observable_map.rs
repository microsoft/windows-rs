use std::collections::BTreeMap;
use windows_collections::*;
use windows_core::*;

const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as _);

#[test]
fn primitive() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([]));
    assert_eq!(m.Lookup(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(m.Size()?, 0);
    assert!(!(m.HasKey(0)?));

    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20)]));
    assert_eq!(m.Lookup(1i32)?, 10u64);
    assert_eq!(m.Lookup(2)?, 20);
    assert_eq!(m.Size()?, 2);
    assert!(m.HasKey(2)?);
    assert!(!(m.HasKey(99)?));

    let able: IIterable<IKeyValuePair<i32, u64>> = m.cast()?;
    let m2: IObservableMap<i32, u64> = able.cast()?;
    assert_eq!(m, m2);

    let map: IMap<i32, u64> = m.cast()?;
    let m3: IObservableMap<i32, u64> = map.cast()?;
    assert_eq!(m, m3);

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([]));
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

    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20), (3, 30)]));
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
fn primitive_mutable() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::new());
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
fn get_view() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([(1, 10), (2, 20)]));

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
fn map_changed_event() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::from([(1, 10)]));

    let events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let events_clone = events.clone();

    let handler =
        MapChangedEventHandler::new(move |_sender, args: Ref<IMapChangedEventArgs<i32>>| {
            let args = args.ok()?;
            let change = args.CollectionChange()?;
            let key = args.Key().ok();
            events_clone.lock().unwrap().push((change, key));
            Ok(())
        });

    let token = m.MapChanged(&handler)?;

    // Insert new key fires ItemInserted
    m.Insert(2, 20u64)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].0, CollectionChange::ItemInserted);
        assert_eq!(ev[0].1, Some(2));
    }

    // Replace existing key fires ItemChanged
    m.Insert(1, 100u64)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 2);
        assert_eq!(ev[1].0, CollectionChange::ItemChanged);
        assert_eq!(ev[1].1, Some(1));
    }

    // Remove fires ItemRemoved
    m.Remove(2)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 3);
        assert_eq!(ev[2].0, CollectionChange::ItemRemoved);
        assert_eq!(ev[2].1, Some(2));
    }

    // Clear fires Reset; key is unspecified for Reset
    m.Clear()?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 4);
        assert_eq!(ev[3].0, CollectionChange::Reset);
        assert_eq!(ev[3].1, None);
    }

    // Removing the handler means no more events
    m.RemoveMapChanged(token)?;
    m.Insert(5, 50u64)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 4); // no new events
    }

    Ok(())
}

#[test]
fn multiple_handlers() -> Result<()> {
    let m = IObservableMap::<i32, u64>::from(BTreeMap::new());

    let count1 = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count2 = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let c1 = count1.clone();
    let handler1 = MapChangedEventHandler::new(move |_, _| {
        c1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    });

    let c2 = count2.clone();
    let handler2 = MapChangedEventHandler::new(move |_, _| {
        c2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    });

    let token1 = m.MapChanged(&handler1)?;
    let token2 = m.MapChanged(&handler2)?;

    m.Insert(1, 10u64)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 1);

    // Remove first handler
    m.RemoveMapChanged(token1)?;
    m.Insert(2, 20u64)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1); // no change
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 2);

    m.RemoveMapChanged(token2)?;
    m.Insert(3, 30u64)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1); // no change
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 2); // no change

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let m = IObservableMap::<HSTRING, i32>::from(BTreeMap::new());
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
    let m2: IObservableMap<HSTRING, i32> = able.cast()?;
    assert_eq!(m, m2);

    Ok(())
}

#[test]
fn hstring_map_changed_event() -> Result<()> {
    let m = IObservableMap::<HSTRING, i32>::from(BTreeMap::new());

    let events: std::sync::Arc<std::sync::Mutex<Vec<(CollectionChange, HSTRING)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let events_clone = events.clone();

    let handler =
        MapChangedEventHandler::new(move |_sender, args: Ref<IMapChangedEventArgs<HSTRING>>| {
            let args = args.ok()?;
            let change = args.CollectionChange()?;
            let key = args.Key()?;
            events_clone.lock().unwrap().push((change, key));
            Ok(())
        });

    let _token = m.MapChanged(&handler)?;

    m.Insert(h!("alpha"), 1)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].0, CollectionChange::ItemInserted);
        assert_eq!(&ev[0].1, h!("alpha"));
    }

    m.Insert(h!("alpha"), 99)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 2);
        assert_eq!(ev[1].0, CollectionChange::ItemChanged);
        assert_eq!(&ev[1].1, h!("alpha"));
    }

    m.Remove(h!("alpha"))?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 3);
        assert_eq!(ev[2].0, CollectionChange::ItemRemoved);
        assert_eq!(&ev[2].1, h!("alpha"));
    }

    Ok(())
}
