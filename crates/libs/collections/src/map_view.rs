use super::*;
use windows_core::*;

#[implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, V>>> {
        Ok(ComObject::new(StockMapViewIterator::<K, V> {
            owner: self.to_object(),
            current: 0.into(),
        })
        .into_interface())
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Lookup(&self, key: Ref<K>) -> Result<V> {
        let value = self.map.get(&*key).ok_or_else(|| Error::from(E_BOUNDS))?;

        V::from_default(value)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.len().try_into()?)
    }

    fn HasKey(&self, key: Ref<K>) -> Result<bool> {
        Ok(self.map.contains_key(&*key))
    }

    fn Split(&self, first: OutRef<IMapView<K, V>>, second: OutRef<IMapView<K, V>>) -> Result<()> {
        _ = first.write(None);
        _ = second.write(None);
        Ok(())
    }
}

#[implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    owner: ComObject<StockMapView<K, V>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Current(&self) -> Result<IKeyValuePair<K, V>> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        if let Some((key, value)) = self.owner.map.iter().nth(current) {
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
        Ok(self.owner.map.len() > current)
    }

    fn MoveNext(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let len = self.owner.map.len();

        if current < len {
            self.current
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(len > current + 1)
    }

    fn GetMany(&self, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> Result<u32> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        if current >= self.owner.map.len() {
            return Ok(0);
        }

        let actual = std::cmp::min(self.owner.map.len() - current, pairs.len());
        let (pairs, _) = pairs.split_at_mut(actual);

        for (pair, (key, value)) in pairs.iter_mut().zip(self.owner.map.iter().skip(current)) {
            *pair = Some(
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

impl<K, V> From<std::collections::BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn from(map: std::collections::BTreeMap<K::Default, V::Default>) -> Self {
        ComObject::new(StockMapView { map }).into_interface()
    }
}
