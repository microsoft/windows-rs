use crate::{core::*, Foundation::Collections::*};

#[windows_core::implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView_Impl<K, V>
where
    K: windows_core::RuntimeType,
    V: windows_core::RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        use windows_core::IUnknownImpl;

        Ok(windows_core::ComObject::new(StockMapViewIterator::<K, V> { _owner: self.to_object(), current: std::sync::RwLock::new(self.map.iter()) }).into_interface())
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView_Impl<K, V>
where
    K: windows_core::RuntimeType,
    V: windows_core::RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Lookup(&self, key: Ref<K>) -> windows_core::Result<V> {
        let value = self.map.get(&*key).ok_or_else(|| windows_core::Error::from(windows_core::imp::E_BOUNDS))?;
        V::from_default(value)
    }
    fn Size(&self) -> windows_core::Result<u32> {
        Ok(self.map.len().try_into()?)
    }
    fn HasKey(&self, key: Ref<K>) -> windows_core::Result<bool> {
        Ok(self.map.contains_key(&*key))
    }
    fn Split(&self, first: windows_core::OutRef<'_, IMapView<K, V>>, second: windows_core::OutRef<'_, IMapView<K, V>>) -> windows_core::Result<()> {
        _ = first.write(None);
        _ = second.write(None);
        Ok(())
    }
}

#[windows_core::implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<'a, K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    _owner: windows_core::ComObject<StockMapView<K, V>>,
    current: std::sync::RwLock<std::collections::btree_map::Iter<'a, K::Default, V::Default>>,
}

impl<'a, K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator_Impl<'a, K, V>
where
    K: windows_core::RuntimeType,
    V: windows_core::RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Current(&self) -> windows_core::Result<IKeyValuePair<K, V>> {
        let mut current = self.current.read().unwrap().clone().peekable();

        if let Some((key, value)) = current.peek() {
            Ok(windows_core::ComObject::new(StockKeyValuePair { key: (*key).clone(), value: (*value).clone() }).into_interface())
        } else {
            Err(windows_core::Error::from(windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> windows_core::Result<bool> {
        let mut current = self.current.read().unwrap().clone().peekable();

        Ok(current.peek().is_some())
    }

    fn MoveNext(&self) -> windows_core::Result<bool> {
        let mut current = self.current.write().unwrap();

        current.next();
        Ok(current.clone().peekable().peek().is_some())
    }

    fn GetMany(&self, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> windows_core::Result<u32> {
        let mut current = self.current.write().unwrap();
        let mut actual = 0;

        for pair in pairs {
            if let Some((key, value)) = current.next() {
                *pair = Some(windows_core::ComObject::new(StockKeyValuePair { key: (*key).clone(), value: (*value).clone() }).into_interface());
                actual += 1;
            } else {
                break;
            }
        }

        Ok(actual)
    }
}

#[windows_core::implement(IKeyValuePair<K, V>)]
struct StockKeyValuePair<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
    K::Default: Clone,
    V::Default: Clone,
{
    key: K::Default,
    value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair_Impl<K, V>
where
    K: windows_core::RuntimeType,
    V: windows_core::RuntimeType,
    K::Default: Clone,
    V::Default: Clone,
{
    fn Key(&self) -> windows_core::Result<K> {
        K::from_default(&self.key)
    }
    fn Value(&self) -> windows_core::Result<V> {
        V::from_default(&self.value)
    }
}

impl<K, V> From<std::collections::BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K: windows_core::RuntimeType,
    V: windows_core::RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn from(map: std::collections::BTreeMap<K::Default, V::Default>) -> Self {
        StockMapView { map }.into()
    }
}
