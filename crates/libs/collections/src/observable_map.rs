use super::*;
use windows_core::*;

#[implement(IObservableMap<K, V>, IMap<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockObservableMap<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    map: std::sync::RwLock<std::collections::BTreeMap<K::Default, V::Default>>,
    handlers: Event<MapChangedEventHandler<K, V>>,
}

impl<K, V> IObservableMap_Impl<K, V> for StockObservableMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn MapChanged(&self, vhnd: Ref<MapChangedEventHandler<K, V>>) -> Result<i64> {
        self.handlers.add(vhnd.ok()?)
    }

    fn RemoveMapChanged(&self, token: i64) -> Result<()> {
        self.handlers.remove(token);
        Ok(())
    }
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockObservableMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, V>>> {
        Ok(ComObject::new(StockObservableMapIterator::<K, V> {
            owner: self.to_object(),
            current: 0.into(),
        })
        .into_interface())
    }
}

impl<K, V> IMap_Impl<K, V> for StockObservableMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Lookup(&self, key: Generic<K>) -> Result<V> {
        let map = self.map.read().unwrap();
        let value = map
            .get(generic_as_default::<K>(&key))
            .ok_or_else(|| Error::from(E_BOUNDS))?;
        V::from_default(value)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.read().unwrap().len().try_into()?)
    }

    fn HasKey(&self, key: Generic<K>) -> Result<bool> {
        Ok(self
            .map
            .read()
            .unwrap()
            .contains_key(generic_as_default::<K>(&key)))
    }

    fn GetView(&self) -> Result<IMapView<K, V>> {
        let snapshot = self.map.read().unwrap().clone();
        Ok(IMapView::<K, V>::from(snapshot))
    }

    fn Insert(&self, key: Generic<K>, value: Generic<V>) -> Result<bool> {
        let replaced = {
            let mut map = self.map.write().unwrap();
            let replaced = map.contains_key(generic_as_default::<K>(&key));
            map.insert(
                generic_as_default::<K>(&key).clone(),
                generic_as_default::<V>(&value).clone(),
            );
            replaced
        };
        let change = if replaced {
            CollectionChange::ItemChanged
        } else {
            CollectionChange::ItemInserted
        };
        self.fire_changed(change, Some(generic_as_default::<K>(&key).clone()));
        Ok(replaced)
    }

    fn Remove(&self, key: Generic<K>) -> Result<()> {
        let key_clone = generic_as_default::<K>(&key).clone();
        {
            let mut map = self.map.write().unwrap();
            if map.remove(generic_as_default::<K>(&key)).is_none() {
                return Err(Error::from(E_BOUNDS));
            }
        }
        self.fire_changed(CollectionChange::ItemRemoved, Some(key_clone));
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        self.map.write().unwrap().clear();
        self.fire_changed(CollectionChange::Reset, None);
        Ok(())
    }
}

impl<K, V> StockObservableMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn fire_changed(&self, change: CollectionChange, key: Option<K::Default>) {
        let observable: IObservableMap<K, V> = self.to_object().into_interface();
        let args: IMapChangedEventArgs<K> =
            ComObject::new(StockMapChangedEventArgs { change, key }).into_interface();
        self.handlers
            .call(|handler: &MapChangedEventHandler<K, V>| handler.Invoke(&observable, &args));
    }
}

#[implement(IMapChangedEventArgs<K>)]
struct StockMapChangedEventArgs<K>
where
    K: RuntimeType + 'static,
    K::Default: Clone,
{
    change: CollectionChange,
    key: Option<K::Default>,
}

impl<K> IMapChangedEventArgs_Impl<K> for StockMapChangedEventArgs_Impl<K>
where
    K: RuntimeType,
    K::Default: Clone,
{
    fn CollectionChange(&self) -> Result<CollectionChange> {
        Ok(self.change)
    }

    fn Key(&self) -> Result<K> {
        match &self.key {
            Some(key) => K::from_default(key),
            None => Err(Error::from(E_BOUNDS)),
        }
    }
}

#[implement(IIterator<IKeyValuePair<K, V>>)]
struct StockObservableMapIterator<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    owner: ComObject<StockObservableMap<K, V>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockObservableMapIterator_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Current(&self) -> Result<IKeyValuePair<K, V>> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let map = self.owner.map.read().unwrap();
        if let Some((key, value)) = map.iter().nth(current) {
            Ok(ComObject::new(super::key_value_pair::StockKeyValuePair {
                key: key.clone(),
                value: value.clone(),
            })
            .into_interface())
        } else {
            Err(Error::from(E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let map = self.owner.map.read().unwrap();
        Ok(map.len() > current)
    }

    fn MoveNext(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let map = self.owner.map.read().unwrap();
        let len = map.len();
        drop(map);

        if current < len {
            self.current
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(len > current + 1)
    }

    fn GetMany(&self, items: &mut [Option<IKeyValuePair<K, V>>]) -> Result<u32> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let map = self.owner.map.read().unwrap();

        if current >= map.len() {
            return Ok(0);
        }

        let actual = std::cmp::min(map.len() - current, items.len());
        let (items, _) = items.split_at_mut(actual);

        for (item, (key, value)) in items.iter_mut().zip(map.iter().skip(current)) {
            *item = Some(
                ComObject::new(super::key_value_pair::StockKeyValuePair {
                    key: key.clone(),
                    value: value.clone(),
                })
                .into_interface(),
            );
        }

        self.current
            .fetch_add(actual, std::sync::atomic::Ordering::Relaxed);

        Ok(actual as u32)
    }
}

impl<K, V> From<std::collections::BTreeMap<K::Default, V::Default>> for IObservableMap<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn from(map: std::collections::BTreeMap<K::Default, V::Default>) -> Self {
        ComObject::new(StockObservableMap {
            map: std::sync::RwLock::new(map),
            handlers: Event::new(),
        })
        .into_interface()
    }
}
