use super::*;
use windows_core::*;

#[implement(IMap<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMap<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    map: std::sync::RwLock<std::collections::BTreeMap<K::Default, V::Default>>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, V>>> {
        Ok(ComObject::new(StockMapIterator::<K, V> {
            owner: self.to_object(),
            current: 0.into(),
        })
        .into_interface())
    }
}

impl<K, V> IMap_Impl<K, V> for StockMap_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Lookup(&self, key: Ref<K>) -> Result<V> {
        let map = self.map.read().unwrap();
        let value = map.get(&*key).ok_or_else(|| Error::from(E_BOUNDS))?;
        V::from_default(value)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.read().unwrap().len().try_into()?)
    }

    fn HasKey(&self, key: Ref<K>) -> Result<bool> {
        Ok(self.map.read().unwrap().contains_key(&*key))
    }

    fn GetView(&self) -> Result<IMapView<K, V>> {
        let snapshot = self.map.read().unwrap().clone();
        Ok(IMapView::<K, V>::from(snapshot))
    }

    fn Insert(&self, key: Ref<K>, value: Ref<V>) -> Result<bool> {
        let mut map = self.map.write().unwrap();
        let replaced = map.contains_key(&*key);
        map.insert((*key).clone(), (*value).clone());
        Ok(replaced)
    }

    fn Remove(&self, key: Ref<K>) -> Result<()> {
        let mut map = self.map.write().unwrap();
        if map.remove(&*key).is_none() {
            return Err(Error::from(E_BOUNDS));
        }
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        self.map.write().unwrap().clear();
        Ok(())
    }
}

#[implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapIterator<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    owner: ComObject<StockMap<K, V>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapIterator_Impl<K, V>
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

impl<K, V> From<std::collections::BTreeMap<K::Default, V::Default>> for IMap<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn from(map: std::collections::BTreeMap<K::Default, V::Default>) -> Self {
        ComObject::new(StockMap {
            map: std::sync::RwLock::new(map),
        })
        .into_interface()
    }
}
