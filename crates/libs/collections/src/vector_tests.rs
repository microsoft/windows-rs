use super::*;
use std::sync::{Arc, Mutex};
use windows_core::*;

struct ReleaseProbe(Box<dyn Fn() + Send + Sync>);

implement_decl! {
    impl ReleaseProbe as ReleaseProbe_Impl: [IKeyValuePair<u32, u32>]
}

impl IKeyValuePair_Impl<u32, u32> for ReleaseProbe_Impl {
    fn Key(&self) -> Result<u32> {
        Ok(0)
    }
    fn Value(&self) -> Result<u32> {
        Ok(0)
    }
}

impl Drop for ReleaseProbe {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[derive(Clone, Copy)]
pub(super) enum Mutation {
    Set,
    Remove,
    RemoveLast,
    Clear,
    ReplaceAll,
}

pub(super) fn check_release<T>(
    owner: ComObject<T>,
    unlocked: impl Fn() -> bool + Send + Sync + 'static,
    mutation: Mutation,
    observable: bool,
) where
    T: ComObjectInner + Send + Sync + 'static,
    T::Outer: ComObjectInterface<IVector<IInspectable>>,
{
    let vector: IVector<IInspectable> = owner.to_interface();
    let observations = Arc::new(Mutex::new(Vec::new()));
    let unlocked = Arc::new(unlocked);
    let count = if matches!(mutation, Mutation::Clear | Mutation::ReplaceAll) {
        2
    } else {
        1
    };
    for _ in 0..count {
        let vector = vector.clone();
        let for_drop = owner.clone();
        let observations = observations.clone();
        let unlocked = unlocked.clone();
        let value: IInspectable = ReleaseProbe(Box::new(move || {
            // Check the actual lock before COM reentry, without unwinding through Release.
            let result = unlocked().then(|| {
                let result = (|| -> Result<_> {
                    let for_drop: IVector<IInspectable> = for_drop.to_interface();
                    let size = for_drop.Size()?;
                    let mut index = 0;
                    let found = for_drop.IndexOf(None, &mut index)?;
                    for_drop.Append(None)?;
                    Ok((size, found, index))
                })();
                result.map_err(|error| error.code())
            });
            observations.lock().unwrap().push(result);
        }))
        .into();
        vector.Append(&value).unwrap();
    }

    let events = Arc::new(Mutex::new(Vec::new()));
    let _subscription = observable.then(|| {
        let capture = events.clone();
        vector
            .cast::<IObservableVector<IInspectable>>()
            .unwrap()
            .VectorChanged(move |_, args: Ref<IVectorChangedEventArgs>| {
                let args = args.unwrap();
                capture
                    .lock()
                    .unwrap()
                    .push((args.CollectionChange().unwrap(), args.Index().unwrap()));
            })
            .unwrap()
    });
    let (expected, expected_events) = match mutation {
        Mutation::Set => {
            vector.SetAt(0, None).unwrap();
            (
                vec![Some(Ok((1, true, 0)))],
                vec![
                    (CollectionChange::ItemInserted, 1),
                    (CollectionChange::ItemChanged, 0),
                ],
            )
        }
        Mutation::Remove | Mutation::RemoveLast => {
            if matches!(mutation, Mutation::Remove) {
                vector.RemoveAt(0).unwrap();
            } else {
                vector.RemoveAtEnd().unwrap();
            }
            (
                vec![Some(Ok((0, false, 0)))],
                vec![
                    (CollectionChange::ItemInserted, 0),
                    (CollectionChange::ItemRemoved, 0),
                ],
            )
        }
        Mutation::Clear => {
            vector.Clear().unwrap();
            (
                vec![Some(Ok((0, false, 0))), Some(Ok((1, true, 0)))],
                vec![
                    (CollectionChange::ItemInserted, 0),
                    (CollectionChange::ItemInserted, 1),
                    (CollectionChange::Reset, 0),
                ],
            )
        }
        Mutation::ReplaceAll => {
            vector.ReplaceAll(&[None]).unwrap();
            (
                vec![Some(Ok((1, true, 0))), Some(Ok((2, true, 0)))],
                vec![
                    (CollectionChange::ItemInserted, 1),
                    (CollectionChange::ItemInserted, 2),
                    (CollectionChange::Reset, 0),
                ],
            )
        }
    };
    assert_eq!(*observations.lock().unwrap(), expected);
    let expected_len = match mutation {
        Mutation::Set | Mutation::Clear => 2,
        Mutation::Remove | Mutation::RemoveLast => 1,
        Mutation::ReplaceAll => 3,
    };
    assert_eq!(vector.Size().unwrap(), expected_len);
    let mut values = vec![None; expected_len as usize];
    assert_eq!(vector.GetMany(0, &mut values).unwrap(), expected_len);
    assert!(values.iter().all(Option::is_none));
    if observable {
        assert_eq!(*events.lock().unwrap(), expected_events);
    }
}
