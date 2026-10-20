use super::*;
use std::sync::{Arc, Mutex};
use windows_collections::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "ObservableCollections.Widget")
        .then(activatable::<Widget, ObservableCollections::Widget>)
}

#[implement(ObservableCollections::Widget)]
#[derive(Default)]
struct Widget;

impl ObservableCollections::IWidget_Impl for Widget_Impl {
    fn Numbers(&self) -> Result<IObservableVector<i32>> {
        Ok(IObservableVector::from(vec![]))
    }

    fn Names(&self) -> Result<IObservableMap<HSTRING, i32>> {
        Ok(IObservableMap::from(std::collections::BTreeMap::new()))
    }

    fn ChangeNumbers(&self, numbers: Ref<IObservableVector<i32>>) -> Result<()> {
        let numbers = numbers.ok()?;
        check(numbers.Size()? == 0)?;
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

        check(
            *changes.lock().unwrap()
                == [
                    (true, CollectionChange::ItemInserted, 0),
                    (true, CollectionChange::ItemInserted, 1),
                    (true, CollectionChange::ItemInserted, 1),
                    (true, CollectionChange::ItemChanged, 0),
                    (true, CollectionChange::ItemRemoved, 1),
                    (true, CollectionChange::ItemRemoved, 1),
                    (true, CollectionChange::Reset, 0),
                ],
        )?;

        check(numbers.Size()? == 1)?;
        check(numbers.GetAt(0)? == 4)
    }

    fn ChangeNames(&self, names: Ref<IObservableMap<HSTRING, i32>>) -> Result<()> {
        let names = names.ok()?;
        check(names.Size()? == 0)?;
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

        check(!names.Insert(h!("one"), 1)?)?;
        check(!names.Insert(h!("two"), 2)?)?;
        names.Remove(h!("two"))?;
        names.Clear()?;
        drop(revoker);
        check(!names.Insert(h!("three"), 3)?)?;

        // Unlike Windows maps such as `PropertySet`, C++/WinRT reports replacing a value as
        // `ItemInserted` rather than `ItemChanged`, so only check the result here.
        check(names.Insert(h!("three"), 30)?)?;

        check(
            *changes.lock().unwrap()
                == [
                    (true, CollectionChange::ItemInserted, h!("one").clone()),
                    (true, CollectionChange::ItemInserted, h!("two").clone()),
                    (true, CollectionChange::ItemRemoved, h!("two").clone()),
                    (true, CollectionChange::Reset, HSTRING::new()),
                ],
        )?;

        check(names.Size()? == 1)?;
        check(names.Lookup(h!("three"))? == 30)
    }
}
