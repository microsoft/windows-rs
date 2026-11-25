use super::*;
use std::collections::BTreeMap;
use storage::*;
use windows_core::*;

struct StockMap<K: RuntimeType + 'static, S: Storage>
where
    K::Default: Ord,
{
    map: std::sync::RwLock<BTreeMap<K::Default, S::Stored>>,
}

// SAFETY: retained keys and values follow the factory's threading contract; access is locked.
unsafe impl<K: RuntimeType, S: Storage> Send for StockMap<K, S> where K::Default: Ord {}
unsafe impl<K: RuntimeType, S: Storage> Sync for StockMap<K, S> where K::Default: Ord {}

implement_decl! {
    impl<K, S> StockMap as StockMap_Impl: [
        IMap<K, S::Value>,
        IIterable<IKeyValuePair<K, S::Value>>,
    ]
    where K: RuntimeType + 'static, S: Storage, K::Default: Ord
}

impl<K: RuntimeType, S: Storage> IIterable_Impl<IKeyValuePair<K, S::Value>> for StockMap_Impl<K, S>
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

impl<K: RuntimeType, S: Storage> IMap_Impl<K, S::Value> for StockMap_Impl<K, S>
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
        Ok(replaced)
    }

    fn Remove(&self, key: Ref<K>) -> Result<()> {
        let retired = self
            .map
            .write()
            .unwrap()
            .remove_entry(ref_as_default::<K>(&key));
        if retired.is_none() {
            return Err(Error::from(E_BOUNDS));
        }
        drop(retired);
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        let retired = core::mem::take(&mut *self.map.write().unwrap());
        drop(retired);
        Ok(())
    }
}

impl<K: RuntimeType, V: RuntimeType> From<BTreeMap<K::Default, V::Default>> for IMap<K, V>
where
    K::Default: Ord + Send + Sync,
    V::Default: Send + Sync,
{
    /// Creates a mutable map with directly stored, thread-safe keys and values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IMap;
    /// use windows_core::IInspectable;
    /// let _ = IMap::<u32, IInspectable>::from(std::collections::BTreeMap::new());
    /// ```
    ///
    /// Keys must be `Sync` as well as `Send`.
    ///
    /// ```compile_fail
    /// use windows_collections::IMap;
    /// use windows_core::{RuntimeType, imp};
    /// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    /// struct Key(std::cell::Cell<u32>);
    /// impl imp::TypeKind for Key { type TypeKind = imp::CloneType; }
    /// impl RuntimeType for Key {
    ///     const SIGNATURE: imp::ConstBuffer =
    ///         imp::ConstBuffer::from_slice(b"struct(Example.Key;u4)");
    /// }
    /// fn assert_send<T: Send>() {}
    /// assert_send::<Key>();
    /// let _ = IMap::<Key, u32>::from(std::collections::BTreeMap::new());
    /// ```
    ///
    /// Keys must also be `Send`, even when they are `Sync`.
    ///
    /// ```compile_fail
    /// use windows_collections::IMap;
    /// use windows_core::{RuntimeType, imp};
    /// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    /// struct Key(u32, std::marker::PhantomData<std::sync::MutexGuard<'static, ()>>);
    /// impl imp::TypeKind for Key { type TypeKind = imp::CloneType; }
    /// impl RuntimeType for Key {
    ///     const SIGNATURE: imp::ConstBuffer =
    ///         imp::ConstBuffer::from_slice(b"struct(Example.Key;u4)");
    /// }
    /// fn assert_sync<T: Sync>() {}
    /// assert_sync::<Key>();
    /// let _ = IMap::<Key, u32>::from(std::collections::BTreeMap::new());
    /// ```
    fn from(map: BTreeMap<K::Default, V::Default>) -> Self {
        unsafe { Self::from_unchecked(map) }
    }
}

impl<K: RuntimeType, V: RuntimeType> IMap<K, V>
where
    K::Default: Ord,
{
    /// Creates a mutable map with directly stored keys and values.
    ///
    /// # Safety
    ///
    /// All retained keys and values must support cloning, comparison, access, and release on every
    /// thread and in every apartment where this collection or its derived objects are used. This
    /// includes all future insertions and replacements through any native or Rust writer. The
    /// caller must enforce this for their complete lifetime. `IAgileObject` does not prove it.
    pub unsafe fn from_unchecked(map: BTreeMap<K::Default, V::Default>) -> Self {
        ComObject::new(StockMap::<K, Direct<V>> {
            map: std::sync::RwLock::new(map),
        })
        .into_interface()
    }
}

impl<K: RuntimeType, V: Interface + RuntimeType<Default = Option<V>>> IMap<K, V>
where
    K::Default: Ord + Send + Sync,
{
    /// Creates a mutable map retaining interface values through runtime marshaling.
    ///
    /// Keys are stored directly; null values are preserved. Creation and access can fail. COM
    /// initialization and apartment dispatch remain the caller's responsibility.
    /// Non-null values require Windows. Every inserted value is marshaled before changing the map.
    pub fn from_marshaled(map: BTreeMap<K::Default, Option<V>>) -> Result<Self> {
        let map = map
            .into_iter()
            .map(|(key, value)| Ok((key, Marshaled::<V>::prepare(value)?)))
            .collect::<Result<_>>()?;
        Ok(ComObject::new(StockMap::<K, Marshaled<V>> {
            map: std::sync::RwLock::new(map),
        })
        .into_interface())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_tests::*;

    fn check(mutation: Mutation) {
        let map = ComObject::new(StockMap::<u32, Direct<IInspectable>> {
            map: Default::default(),
        });
        check_release(
            map.clone(),
            move || map.map.try_write().is_ok(),
            mutation,
            false,
        );
    }

    fn check_key(mutation: Mutation) {
        let map = ComObject::new(StockMap::<OrderedKey, Direct<u32>> {
            map: Default::default(),
        });
        check_key_release(
            map.clone(),
            move || map.map.try_write().is_ok(),
            mutation,
            false,
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
