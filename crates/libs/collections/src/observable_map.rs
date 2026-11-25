use super::*;
use std::collections::BTreeMap;
use storage::*;
use windows_core::*;

struct StockObservableMap<K: RuntimeType + 'static, S: Storage>
where
    K::Default: Ord,
{
    map: std::sync::RwLock<BTreeMap<K::Default, S::Stored>>,
    handlers: Event<MapChangedEventHandler<K, S::Value>>,
}

// SAFETY: keys and values follow the factory's threading contract; the map and events synchronize.
unsafe impl<K: RuntimeType, S: Storage> Send for StockObservableMap<K, S> where K::Default: Ord {}
unsafe impl<K: RuntimeType, S: Storage> Sync for StockObservableMap<K, S> where K::Default: Ord {}

implement_decl! {
    impl<K, S> StockObservableMap as StockObservableMap_Impl: [
        IObservableMap<K, S::Value>,
        IMap<K, S::Value>,
        IIterable<IKeyValuePair<K, S::Value>>,
    ]
    where K: RuntimeType + 'static, S: Storage, K::Default: Ord
}

impl<K: RuntimeType, S: Storage> IObservableMap_Impl<K, S::Value> for StockObservableMap_Impl<K, S>
where
    K::Default: Ord,
{
    fn MapChanged(&self, handler: Ref<MapChangedEventHandler<K, S::Value>>) -> Result<i64> {
        self.handlers.add(handler.ok()?)
    }

    fn RemoveMapChanged(&self, token: i64) -> Result<()> {
        self.handlers.remove(token);
        Ok(())
    }
}

impl<K: RuntimeType, S: Storage> IIterable_Impl<IKeyValuePair<K, S::Value>>
    for StockObservableMap_Impl<K, S>
where
    K::Default: Ord,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, S::Value>>> {
        let snapshot = self
            .map
            .read()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Ok(map_view::iterator::<K, S>(snapshot))
    }
}

impl<K: RuntimeType, S: Storage> IMap_Impl<K, S::Value> for StockObservableMap_Impl<K, S>
where
    K::Default: Ord,
{
    fn Lookup(&self, key: Ref<K>) -> Result<S::Value> {
        let value = self
            .map
            .read()
            .unwrap()
            .get(ref_as_default::<K>(&key))
            .cloned()
            .ok_or_else(|| Error::from(E_BOUNDS))?;
        S::get_owned(value)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.read().unwrap().len().try_into()?)
    }

    fn HasKey(&self, key: Ref<K>) -> Result<bool> {
        Ok(self
            .map
            .read()
            .unwrap()
            .contains_key(ref_as_default::<K>(&key)))
    }

    fn GetView(&self) -> Result<IMapView<K, S::Value>> {
        let snapshot = self.map.read().unwrap().clone();
        Ok(map_view::from_storage::<K, S>(snapshot))
    }

    fn Insert(&self, key: Ref<K>, value: Ref<S::Value>) -> Result<bool> {
        let key = ref_as_default::<K>(&key);
        let value = S::prepare_ref(ref_as_default::<S::Value>(&value))?;
        let retired = {
            let mut map = self.map.write().unwrap();
            if let Some(previous) = map.get_mut(key) {
                Some(core::mem::replace(previous, value))
            } else {
                map.insert(key.clone(), value)
            }
        };
        let replaced = retired.is_some();
        drop(retired);
        let change = if replaced {
            CollectionChange::ItemChanged
        } else {
            CollectionChange::ItemInserted
        };
        self.fire_changed(change, Some(key.clone()));
        Ok(replaced)
    }

    fn Remove(&self, key: Ref<K>) -> Result<()> {
        let key_clone = ref_as_default::<K>(&key).clone();
        let retired = self
            .map
            .write()
            .unwrap()
            .remove_entry(ref_as_default::<K>(&key));
        if retired.is_none() {
            return Err(Error::from(E_BOUNDS));
        }
        drop(retired);
        self.fire_changed(CollectionChange::ItemRemoved, Some(key_clone));
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        let retired = core::mem::take(&mut *self.map.write().unwrap());
        drop(retired);
        self.fire_changed(CollectionChange::Reset, None);
        Ok(())
    }
}

impl<K: RuntimeType, S: Storage> StockObservableMap_Impl<K, S>
where
    K::Default: Ord,
{
    fn fire_changed(&self, change: CollectionChange, key: Option<K::Default>) {
        let observable: IObservableMap<K, S::Value> = self.to_object().into_interface();
        let args: IMapChangedEventArgs<K> =
            ComObject::new(StockMapChangedEventArgs { change, key }).into_interface();
        self.handlers
            .call(|handler: &MapChangedEventHandler<K, S::Value>| {
                handler.Invoke(&observable, &args)
            });
    }
}

struct StockMapChangedEventArgs<K: RuntimeType + 'static> {
    change: CollectionChange,
    key: Option<K::Default>,
}

// SAFETY: immutable event keys inherit the creating collection's threading contract.
unsafe impl<K: RuntimeType> Send for StockMapChangedEventArgs<K> {}
unsafe impl<K: RuntimeType> Sync for StockMapChangedEventArgs<K> {}

implement_decl! {
    impl<K> StockMapChangedEventArgs as StockMapChangedEventArgs_Impl: [IMapChangedEventArgs<K>]
    where K: RuntimeType + 'static
}

impl<K: RuntimeType> IMapChangedEventArgs_Impl<K> for StockMapChangedEventArgs_Impl<K> {
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

impl<K: RuntimeType, V: RuntimeType> From<BTreeMap<K::Default, V::Default>> for IObservableMap<K, V>
where
    K::Default: Ord + Send + Sync,
    V::Default: Send + Sync,
{
    /// Creates an observable map with directly stored, thread-safe keys and values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IObservableMap;
    /// use windows_core::IInspectable;
    /// let _ = IObservableMap::<u32, IInspectable>::from(std::collections::BTreeMap::new());
    /// ```
    fn from(map: BTreeMap<K::Default, V::Default>) -> Self {
        unsafe { Self::from_unchecked(map) }
    }
}

impl<K: RuntimeType, V: RuntimeType> IObservableMap<K, V>
where
    K::Default: Ord,
{
    /// Creates an observable map with directly stored keys and values.
    ///
    /// # Safety
    ///
    /// All retained keys and values must support cloning, comparison, access, and release on every
    /// thread and in every apartment where this collection or its derived objects are used. This
    /// includes all future insertions and replacements through any native or Rust writer. The
    /// caller must enforce this for their complete lifetime. `IAgileObject` does not prove it.
    pub unsafe fn from_unchecked(map: BTreeMap<K::Default, V::Default>) -> Self {
        ComObject::new(StockObservableMap::<K, Direct<V>> {
            map: std::sync::RwLock::new(map),
            handlers: Event::new(),
        })
        .into_interface()
    }
}

impl<K: RuntimeType, V: Interface + RuntimeType<Default = Option<V>>> IObservableMap<K, V>
where
    K::Default: Ord + Send + Sync,
{
    /// Creates an observable map retaining interface values through runtime marshaling.
    ///
    /// Keys are stored directly; null values are preserved. Creation and access can fail. COM
    /// initialization and apartment dispatch remain the caller's responsibility.
    /// Non-null values require Windows. Failed marshaling changes nothing and emits no
    /// notification.
    pub fn from_marshaled(map: BTreeMap<K::Default, Option<V>>) -> Result<Self> {
        let map = map
            .into_iter()
            .map(|(key, value)| Ok((key, Marshaled::<V>::prepare(value)?)))
            .collect::<Result<_>>()?;
        Ok(ComObject::new(StockObservableMap::<K, Marshaled<V>> {
            map: std::sync::RwLock::new(map),
            handlers: Event::new(),
        })
        .into_interface())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_tests::*;

    fn check(mutation: Mutation) {
        let map = ComObject::new(StockObservableMap::<u32, Direct<IInspectable>> {
            map: Default::default(),
            handlers: Event::new(),
        });
        check_release(
            map.clone(),
            move || map.map.try_write().is_ok(),
            mutation,
            true,
        );
    }

    fn check_key(mutation: Mutation) {
        let map = ComObject::new(StockObservableMap::<OrderedKey, Direct<u32>> {
            map: Default::default(),
            handlers: Event::new(),
        });
        check_key_release(
            map.clone(),
            move || map.map.try_write().is_ok(),
            mutation,
            true,
        );
    }

    #[test]
    fn replacement_release_reenters() {
        check(Mutation::Replace);
    }

    #[test]
    fn removal_release_reenters() {
        check(Mutation::Remove);
    }

    #[test]
    fn clear_release_reenters() {
        check(Mutation::Clear);
    }

    #[test]
    fn removal_key_release_reenters() {
        check_key(Mutation::Remove);
    }

    #[test]
    fn clear_key_release_reenters() {
        check_key(Mutation::Clear);
    }
}
