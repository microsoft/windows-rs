use super::*;
use ObservableCollections::*;
use std::sync::{Arc, Mutex};
use windows_collections::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    change_numbers(&widget.Numbers()?)?;
    change_names(&widget.Names()?)?;
    widget.ChangeNumbers(&IObservableVector::from(vec![]))?;
    widget.ChangeNames(&IObservableMap::from(std::collections::BTreeMap::new()))?;
    Ok(())
}

fn change_numbers(numbers: &IObservableVector<i32>) -> Result<()> {
    assert_eq!(numbers.Size()?, 0);
    let identity = numbers.cast::<IUnknown>()?.as_raw() as usize;
    let changes = Arc::new(Mutex::new(vec![]));

    let revoker = numbers.VectorChanged({
        let changes = changes.clone();
        move |sender, args| {
            let sender = sender.ok().unwrap().cast::<IUnknown>().unwrap();
            let args = args.ok().unwrap();
            let change = args.CollectionChange().unwrap();
            let index = if change == CollectionChange::Reset {
                0
            } else {
                args.Index().unwrap()
            };
            changes
                .lock()
                .unwrap()
                .push((sender.as_raw() as usize == identity, change, index));
        }
    })?;

    numbers.Append(1)?;
    numbers.Append(3)?;
    numbers.InsertAt(1, 2)?;
    numbers.SetAt(0, 10)?;
    numbers.RemoveAt(1)?;
    numbers.RemoveAtEnd()?;
    numbers.Clear()?;
    drop(revoker);
    numbers.Append(4)?;

    assert_eq!(
        *changes.lock().unwrap(),
        [
            (true, CollectionChange::ItemInserted, 0),
            (true, CollectionChange::ItemInserted, 1),
            (true, CollectionChange::ItemInserted, 1),
            (true, CollectionChange::ItemChanged, 0),
            (true, CollectionChange::ItemRemoved, 1),
            (true, CollectionChange::ItemRemoved, 1),
            (true, CollectionChange::Reset, 0),
        ]
    );

    assert_eq!(numbers.Size()?, 1);
    assert_eq!(numbers.GetAt(0)?, 4);
    Ok(())
}

fn change_names(names: &IObservableMap<HSTRING, i32>) -> Result<()> {
    assert_eq!(names.Size()?, 0);
    let identity = names.cast::<IUnknown>()?.as_raw() as usize;
    let changes = Arc::new(Mutex::new(vec![]));

    let revoker = names.MapChanged({
        let changes = changes.clone();
        move |sender, args| {
            let sender = sender.ok().unwrap().cast::<IUnknown>().unwrap();
            let args = args.ok().unwrap();
            let change = args.CollectionChange().unwrap();
            let key = if change == CollectionChange::Reset {
                HSTRING::new()
            } else {
                args.Key().unwrap()
            };
            changes
                .lock()
                .unwrap()
                .push((sender.as_raw() as usize == identity, change, key));
        }
    })?;

    assert!(!names.Insert(h!("one"), 1)?);
    assert!(!names.Insert(h!("two"), 2)?);
    assert!(names.Insert(h!("one"), 10)?);
    names.Remove(h!("two"))?;
    names.Clear()?;
    drop(revoker);
    assert!(!names.Insert(h!("three"), 3)?);

    assert_eq!(
        *changes.lock().unwrap(),
        [
            (true, CollectionChange::ItemInserted, h!("one").clone()),
            (true, CollectionChange::ItemInserted, h!("two").clone()),
            (true, CollectionChange::ItemChanged, h!("one").clone()),
            (true, CollectionChange::ItemRemoved, h!("two").clone()),
            (true, CollectionChange::Reset, HSTRING::new()),
        ]
    );

    assert_eq!(names.Size()?, 1);
    assert_eq!(names.Lookup(h!("three"))?, 3);
    Ok(())
}
