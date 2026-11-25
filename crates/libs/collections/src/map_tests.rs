use super::*;
use std::sync::{Arc, Mutex};
use windows_core::*;

struct ReleaseProbe {
    key: u32,
    release: Box<dyn Fn() + Send + Sync>,
}

implement_decl! {
    impl ReleaseProbe as ReleaseProbe_Impl: [IKeyValuePair<u32, u32>]
}

impl IKeyValuePair_Impl<u32, u32> for ReleaseProbe_Impl {
    fn Key(&self) -> Result<u32> {
        Ok(self.key)
    }

    fn Value(&self) -> Result<u32> {
        Ok(0)
    }
}

impl Drop for ReleaseProbe {
    fn drop(&mut self) {
        (self.release)();
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(super) struct OrderedKey(IKeyValuePair<u32, u32>);

// This wrapper retains the interface ABI and adds value-based ordering for map keys.
unsafe impl Interface for OrderedKey {
    type Vtable = <IKeyValuePair<u32, u32> as Interface>::Vtable;
    const IID: GUID = <IKeyValuePair<u32, u32> as Interface>::IID;
    const INSPECTABLE: bool = true;
}

impl RuntimeType for OrderedKey {
    const SIGNATURE: imp::ConstBuffer = <IKeyValuePair<u32, u32> as RuntimeType>::SIGNATURE;
}

impl OrderedKey {
    fn new(key: u32, release: impl Fn() + Send + Sync + 'static) -> Self {
        Self(
            ReleaseProbe {
                key,
                release: Box::new(release),
            }
            .into(),
        )
    }
}

impl PartialEq for OrderedKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for OrderedKey {}

impl PartialOrd for OrderedKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.Key().unwrap().cmp(&other.0.Key().unwrap())
    }
}

#[derive(Clone, Copy)]
pub(super) enum Mutation {
    Replace,
    Remove,
    Clear,
}

pub(super) fn check_release<T>(
    owner: ComObject<T>,
    unlocked: impl Fn() -> bool + Send + Sync + 'static,
    mutation: Mutation,
    observable: bool,
) where
    T: ComObjectInner + Send + Sync + 'static,
    T::Outer: ComObjectInterface<IMap<u32, IInspectable>>,
{
    let map: IMap<u32, IInspectable> = owner.to_interface();
    let observations = Arc::new(Mutex::new(Vec::new()));
    let unlocked = Arc::new(unlocked);
    let count = if matches!(mutation, Mutation::Clear) {
        2
    } else {
        1
    };
    for key in 7..7 + count {
        let map_for_drop = owner.clone();
        let observations = observations.clone();
        let unlocked = unlocked.clone();
        let value: IInspectable = ReleaseProbe {
            key,
            release: Box::new(move || {
                // Detect a held lock without hanging the test or unwinding through COM Release.
                let result = unlocked().then(|| {
                    let result = (|| -> Result<_> {
                        let map_for_drop: IMap<u32, IInspectable> = map_for_drop.to_interface();
                        let size = map_for_drop.Size()?;
                        let present = map_for_drop.HasKey(key)?;
                        let replaced = map_for_drop.Insert(99, None)?;
                        Ok((size, present, replaced))
                    })();
                    result.map_err(|error| error.code())
                });
                observations.lock().unwrap().push(result);
            }),
        }
        .into();
        assert!(!map.Insert(key, &value).unwrap());
    }

    let events = Arc::new(Mutex::new(Vec::new()));
    let _subscription = observable.then(|| {
        let capture = events.clone();
        map.cast::<IObservableMap<u32, IInspectable>>()
            .unwrap()
            .MapChanged(move |_, args: Ref<IMapChangedEventArgs<u32>>| {
                let args = args.unwrap();
                let change = args.CollectionChange().unwrap();
                let key = (change != CollectionChange::Reset).then(|| args.Key().unwrap());
                capture.lock().unwrap().push((change, key));
            })
            .unwrap()
    });

    let expected = match mutation {
        Mutation::Replace => {
            assert!(map.Insert(7, None).unwrap());
            vec![Some(Ok((1, true, false)))]
        }
        Mutation::Remove => {
            map.Remove(7).unwrap();
            vec![Some(Ok((0, false, false)))]
        }
        Mutation::Clear => {
            map.Clear().unwrap();
            vec![Some(Ok((0, false, false))), Some(Ok((1, false, true)))]
        }
    };
    assert_eq!(*observations.lock().unwrap(), expected);
    assert!(map.HasKey(99).unwrap());

    if observable {
        assert_eq!(*events.lock().unwrap(), expected_events(mutation));
    }
}

pub(super) fn check_key_release<T>(
    owner: ComObject<T>,
    unlocked: impl Fn() -> bool + Send + Sync + 'static,
    mutation: Mutation,
    observable: bool,
) where
    T: ComObjectInner + Send + Sync + 'static,
    T::Outer: ComObjectInterface<IMap<OrderedKey, u32>>,
{
    let map: IMap<OrderedKey, u32> = owner.to_interface();
    let observations = Arc::new(Mutex::new(Vec::new()));
    let unlocked = Arc::new(unlocked);
    let count = if matches!(mutation, Mutation::Clear) {
        2
    } else {
        1
    };
    for id in 7..7 + count {
        let map_for_drop = owner.clone();
        let observations = observations.clone();
        let unlocked = unlocked.clone();
        let key = OrderedKey::new(id, move || {
            let result = unlocked().then(|| {
                let result = (|| -> Result<_> {
                    let map_for_drop: IMap<OrderedKey, u32> = map_for_drop.to_interface();
                    let size = map_for_drop.Size()?;
                    let present = map_for_drop.HasKey(&OrderedKey::new(id, || {}))?;
                    let replaced = map_for_drop.Insert(&OrderedKey::new(99, || {}), 0)?;
                    Ok((size, present, replaced))
                })();
                result.map_err(|error| error.code())
            });
            observations.lock().unwrap().push(result);
        });
        assert!(!map.Insert(&key, 0).unwrap());
    }

    let events = Arc::new(Mutex::new(Vec::new()));
    let _subscription = observable.then(|| {
        let capture = events.clone();
        map.cast::<IObservableMap<OrderedKey, u32>>()
            .unwrap()
            .MapChanged(move |_, args: Ref<IMapChangedEventArgs<OrderedKey>>| {
                let args = args.unwrap();
                let change = args.CollectionChange().unwrap();
                let key = (change != CollectionChange::Reset)
                    .then(|| args.Key().unwrap().0.Key().unwrap());
                capture.lock().unwrap().push((change, key));
            })
            .unwrap()
    });

    let expected = match mutation {
        Mutation::Replace => unreachable!(),
        Mutation::Remove => {
            // The equal lookup key and notification must not retain the stored COM object.
            map.Remove(&OrderedKey::new(7, || {})).unwrap();
            vec![Some(Ok((0, false, false)))]
        }
        Mutation::Clear => {
            map.Clear().unwrap();
            vec![Some(Ok((0, false, false))), Some(Ok((1, false, true)))]
        }
    };
    assert_eq!(*observations.lock().unwrap(), expected);
    assert!(map.HasKey(&OrderedKey::new(99, || {})).unwrap());
    if observable {
        assert_eq!(*events.lock().unwrap(), expected_events(mutation));
    }
}

fn expected_events(mutation: Mutation) -> Vec<(CollectionChange, Option<u32>)> {
    let mut events = vec![(CollectionChange::ItemInserted, Some(99))];
    let last = match mutation {
        Mutation::Replace => (CollectionChange::ItemChanged, Some(7)),
        Mutation::Remove => (CollectionChange::ItemRemoved, Some(7)),
        Mutation::Clear => {
            events.push((CollectionChange::ItemChanged, Some(99)));
            (CollectionChange::Reset, None)
        }
    };
    events.push(last);
    events
}
