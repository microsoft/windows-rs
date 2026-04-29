use windows::{core::*, Win32::Foundation::E_BOUNDS};
use windows_collections::*;

#[test]
fn primitive() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);
    assert!(!(v.IndexOf(0, &mut 0)?));
    assert_eq!(v.GetMany(0, &mut [0; 5])?, 0);

    let v = IObservableVector::<i32>::from(vec![1, 2, 3]);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);
    assert_eq!(v.GetAt(3).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 3);

    let mut index = 0;
    assert!(!(v.IndexOf(0, &mut index)?));
    assert_eq!(index, 0);
    assert!(v.IndexOf(1, &mut index)?);
    assert_eq!(index, 0);
    assert!(v.IndexOf(2, &mut index)?);
    assert_eq!(index, 1);
    assert!(v.IndexOf(3, &mut index)?);
    assert_eq!(index, 2);

    let mut values = [0; 5];
    assert_eq!(v.GetMany(0, &mut values)?, 3);
    assert_eq!(values, [1, 2, 3, 0, 0]);

    let able: IIterable<i32> = v.cast()?;
    let v2: IObservableVector<i32> = able.cast()?;
    assert_eq!(v, v2);

    let vec: IVector<i32> = v.cast()?;
    let v3: IObservableVector<i32> = vec.cast()?;
    assert_eq!(v, v3);

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![]);
    let iter = v.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    assert_eq!(iter.GetMany(&mut [0; 5])?, 0);

    let v = IObservableVector::<i32>::from(vec![1, 2, 3]);
    let iter = v.First()?;

    assert_eq!(iter.Current()?, 1);
    assert_eq!(iter.Current()?, 1);

    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?, 2);
    assert_eq!(iter.Current()?, 2);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?, 3);
    assert_eq!(iter.Current()?, 3);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    let iter = v.First()?;
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert_eq!(values, [1, 2, 3, 0, 0]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = v.First()?;
    let mut values = [0; 1];
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert_eq!(values, [1]);
    let mut values = [0; 2];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values, [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    // MoveNext followed by GetMany reads from the advanced position
    let iter = v.First()?;
    assert!(iter.MoveNext()?);
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values[..2], [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

#[test]
fn primitive_mutable() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![]);

    // Append
    v.Append(1)?;
    v.Append(2)?;
    v.Append(3)?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);

    // SetAt
    v.SetAt(1, 20)?;
    assert_eq!(v.GetAt(1)?, 20);
    assert_eq!(v.SetAt(3, 0).unwrap_err().code(), E_BOUNDS);

    // InsertAt
    v.InsertAt(1, 10)?;
    assert_eq!(v.Size()?, 4);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 10);
    assert_eq!(v.GetAt(2)?, 20);
    assert_eq!(v.GetAt(3)?, 3);
    assert_eq!(v.InsertAt(5, 0).unwrap_err().code(), E_BOUNDS);

    // RemoveAt
    v.RemoveAt(0)?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.RemoveAt(3).unwrap_err().code(), E_BOUNDS);

    // RemoveAtEnd
    v.RemoveAtEnd()?;
    assert_eq!(v.Size()?, 2);
    assert_eq!(v.GetAt(0)?, 10);
    assert_eq!(v.GetAt(1)?, 20);

    // Clear
    v.Clear()?;
    assert_eq!(v.Size()?, 0);
    assert_eq!(v.RemoveAtEnd().unwrap_err().code(), E_BOUNDS);

    // ReplaceAll
    v.ReplaceAll(&[7, 8, 9])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 7);
    assert_eq!(v.GetAt(1)?, 8);
    assert_eq!(v.GetAt(2)?, 9);

    // ReplaceAll with empty clears the vector
    v.ReplaceAll(&[])?;
    assert_eq!(v.Size()?, 0);

    Ok(())
}

#[test]
fn get_view() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![1, 2, 3]);

    // GetView returns a snapshot
    let view = v.GetView()?;
    assert_eq!(view.Size()?, 3);
    assert_eq!(view.GetAt(0)?, 1);
    assert_eq!(view.GetAt(1)?, 2);
    assert_eq!(view.GetAt(2)?, 3);

    // Mutating the vector after GetView does not affect the snapshot
    v.Append(4)?;
    assert_eq!(v.Size()?, 4);
    assert_eq!(view.Size()?, 3);

    Ok(())
}

#[test]
fn vector_changed_event() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![1, 2, 3]);

    let events: std::sync::Arc<std::sync::Mutex<Vec<(CollectionChange, u32)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let events_clone = events.clone();

    let handler =
        VectorChangedEventHandler::new(move |_sender, args: Ref<IVectorChangedEventArgs>| {
            let args = args.ok()?;
            let change = args.CollectionChange()?;
            let index = args.Index()?;
            events_clone.lock().unwrap().push((change, index));
            Ok(())
        });

    let token = v.VectorChanged(&handler)?;

    // Append fires ItemInserted
    v.Append(4)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].0, CollectionChange::ItemInserted);
        assert_eq!(ev[0].1, 3);
    }

    // SetAt fires ItemChanged
    v.SetAt(0, 10)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 2);
        assert_eq!(ev[1].0, CollectionChange::ItemChanged);
        assert_eq!(ev[1].1, 0);
    }

    // RemoveAt fires ItemRemoved
    v.RemoveAt(0)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 3);
        assert_eq!(ev[2].0, CollectionChange::ItemRemoved);
        assert_eq!(ev[2].1, 0);
    }

    // RemoveAtEnd fires ItemRemoved
    v.RemoveAtEnd()?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 4);
        assert_eq!(ev[3].0, CollectionChange::ItemRemoved);
        assert_eq!(ev[3].1, 2); // index of last element before removal
    }

    // InsertAt fires ItemInserted
    v.InsertAt(0, 99)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 5);
        assert_eq!(ev[4].0, CollectionChange::ItemInserted);
        assert_eq!(ev[4].1, 0);
    }

    // Clear fires Reset
    v.Clear()?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 6);
        assert_eq!(ev[5].0, CollectionChange::Reset);
        assert_eq!(ev[5].1, 0);
    }

    // ReplaceAll fires Reset
    v.ReplaceAll(&[1, 2])?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 7);
        assert_eq!(ev[6].0, CollectionChange::Reset);
        assert_eq!(ev[6].1, 0);
    }

    // Removing the handler means no more events
    v.RemoveVectorChanged(token)?;
    v.Append(5)?;
    {
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 7); // no new events
    }

    Ok(())
}

#[test]
fn multiple_handlers() -> Result<()> {
    let v = IObservableVector::<i32>::from(vec![]);

    let count1 = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count2 = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let c1 = count1.clone();
    let handler1 = VectorChangedEventHandler::new(move |_, _| {
        c1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    });

    let c2 = count2.clone();
    let handler2 = VectorChangedEventHandler::new(move |_, _| {
        c2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    });

    let token1 = v.VectorChanged(&handler1)?;
    let token2 = v.VectorChanged(&handler2)?;

    v.Append(1)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 1);

    // Remove first handler
    v.RemoveVectorChanged(token1)?;
    v.Append(2)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1); // no change
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 2);

    v.RemoveVectorChanged(token2)?;
    v.Append(3)?;
    assert_eq!(count1.load(std::sync::atomic::Ordering::Relaxed), 1); // no change
    assert_eq!(count2.load(std::sync::atomic::Ordering::Relaxed), 2); // no change

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let v = IObservableVector::<HSTRING>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);

    v.Append(h!("one"))?;
    v.Append(h!("two"))?;
    v.Append(h!("three"))?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(&v.GetAt(0)?, h!("one"));
    assert_eq!(&v.GetAt(1)?, h!("two"));
    assert_eq!(&v.GetAt(2)?, h!("three"));

    v.SetAt(1, h!("TWO"))?;
    assert_eq!(&v.GetAt(1)?, h!("TWO"));

    v.RemoveAt(0)?;
    assert_eq!(v.Size()?, 2);
    assert_eq!(&v.GetAt(0)?, h!("TWO"));
    assert_eq!(&v.GetAt(1)?, h!("three"));

    v.Clear()?;
    assert_eq!(v.Size()?, 0);

    let able: IIterable<HSTRING> = v.cast()?;
    let v2: IObservableVector<HSTRING> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}
