use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use storage::*;
use windows_core::*;

struct StockMapView<K: RuntimeType + 'static, S: Storage>
where
    K::Default: Ord,
{
    map: BTreeMap<K::Default, S::Stored>,
}

// SAFETY: immutable keys and values retain the factory's threading contract, including in snapshots.
unsafe impl<K: RuntimeType, S: Storage> Send for StockMapView<K, S> where K::Default: Ord {}
unsafe impl<K: RuntimeType, S: Storage> Sync for StockMapView<K, S> where K::Default: Ord {}

implement_decl! {
    impl<K, S> StockMapView as StockMapView_Impl: [
        IMapView<K, S::Value>,
        IIterable<IKeyValuePair<K, S::Value>>,
    ]
    where K: RuntimeType + 'static, S: Storage, K::Default: Ord
}

pub(crate) fn from_storage<K: RuntimeType, S: Storage>(
    map: BTreeMap<K::Default, S::Stored>,
) -> IMapView<K, S::Value>
where
    K::Default: Ord,
{
    ComObject::new(StockMapView::<K, S> { map }).into_interface()
}

impl<K: RuntimeType, S: Storage> IIterable_Impl<IKeyValuePair<K, S::Value>>
    for StockMapView_Impl<K, S>
where
    K::Default: Ord,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, S::Value>>> {
        Ok(iterator::<K, S>(
            self.map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        ))
    }
}

impl<K: RuntimeType, S: Storage> IMapView_Impl<K, S::Value> for StockMapView_Impl<K, S>
where
    K::Default: Ord,
{
    fn Lookup(&self, key: Ref<K>) -> Result<S::Value> {
        S::get(
            self.map
                .get(ref_as_default::<K>(&key))
                .ok_or_else(|| Error::from(E_BOUNDS))?,
        )
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.len().try_into()?)
    }

    fn HasKey(&self, key: Ref<K>) -> Result<bool> {
        Ok(self.map.contains_key(ref_as_default::<K>(&key)))
    }

    fn Split(
        &self,
        first: OutRef<IMapView<K, S::Value>>,
        second: OutRef<IMapView<K, S::Value>>,
    ) -> Result<()> {
        _ = first.write(None);
        _ = second.write(None);
        Ok(())
    }
}

struct StockMapIterator<K: RuntimeType + 'static, S: Storage> {
    snapshot: Vec<(K::Default, S::Stored)>,
    current: AtomicUsize,
}

// SAFETY: the immutable snapshot retains its source's threading contract; the cursor is atomic.
unsafe impl<K: RuntimeType, S: Storage> Send for StockMapIterator<K, S> {}
unsafe impl<K: RuntimeType, S: Storage> Sync for StockMapIterator<K, S> {}

implement_decl! {
    impl<K, S> StockMapIterator as StockMapIterator_Impl: [
        IIterator<IKeyValuePair<K, S::Value>>,
    ]
    where K: RuntimeType + 'static, S: Storage
}

pub(crate) fn iterator<K: RuntimeType, S: Storage>(
    snapshot: Vec<(K::Default, S::Stored)>,
) -> IIterator<IKeyValuePair<K, S::Value>> {
    ComObject::new(StockMapIterator::<K, S> {
        snapshot,
        current: AtomicUsize::new(0),
    })
    .into_interface()
}

impl<K: RuntimeType, S: Storage> IIterator_Impl<IKeyValuePair<K, S::Value>>
    for StockMapIterator_Impl<K, S>
{
    fn Current(&self) -> Result<IKeyValuePair<K, S::Value>> {
        let (key, value) = self
            .snapshot
            .get(self.current.load(Ordering::Relaxed))
            .ok_or_else(|| Error::from(E_BOUNDS))?;
        Ok(ComObject::new(key_value_pair::StockKeyValuePair::<K, S> {
            key: key.clone(),
            value: value.clone(),
        })
        .into_interface())
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(self.current.load(Ordering::Relaxed) < self.snapshot.len())
    }

    fn MoveNext(&self) -> Result<bool> {
        Ok(move_next(&self.current, || self.snapshot.len()))
    }

    fn GetMany(&self, items: &mut [Option<IKeyValuePair<K, S::Value>>]) -> Result<u32> {
        // Pairs retain storage without resolving values, so both policies can reserve first.
        let Some(range) = reserve(&self.current, self.snapshot.len(), items.len()) else {
            return Ok(0);
        };
        let count = range.len();
        for (item, (key, value)) in items.iter_mut().zip(&self.snapshot[range]) {
            *item = Some(
                ComObject::new(key_value_pair::StockKeyValuePair::<K, S> {
                    key: key.clone(),
                    value: value.clone(),
                })
                .into_interface(),
            );
        }
        Ok(count as u32)
    }
}

impl<K: RuntimeType, V: RuntimeType> From<BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K::Default: Ord + Send + Sync,
    V::Default: Send + Sync,
{
    /// Creates a read-only map with directly stored, thread-safe keys and values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IMapView;
    /// use windows_core::IInspectable;
    /// let _ = IMapView::<u32, IInspectable>::from(std::collections::BTreeMap::new());
    /// ```
    fn from(map: BTreeMap<K::Default, V::Default>) -> Self {
        unsafe { Self::from_unchecked(map) }
    }
}

impl<K: RuntimeType, V: RuntimeType> IMapView<K, V>
where
    K::Default: Ord,
{
    /// Creates a read-only map with directly stored keys and values.
    ///
    /// # Safety
    ///
    /// All retained keys and values must support cloning, comparison, access, and release on every
    /// thread and in every apartment where this collection or its derived objects are used. The
    /// caller must enforce this for their complete lifetime. `IAgileObject` does not prove it.
    pub unsafe fn from_unchecked(map: BTreeMap<K::Default, V::Default>) -> Self {
        from_storage::<K, Direct<V>>(map)
    }
}

impl<K: RuntimeType, V: Interface + RuntimeType<Default = Option<V>>> IMapView<K, V>
where
    K::Default: Ord + Send + Sync,
{
    /// Creates a read-only map retaining interface values through runtime marshaling.
    ///
    /// Keys are stored directly; null values are preserved. Creation and access can fail. COM
    /// initialization and apartment dispatch remain the caller's responsibility.
    /// Non-null values require Windows.
    pub fn from_marshaled(map: BTreeMap<K::Default, Option<V>>) -> Result<Self> {
        let map = map
            .into_iter()
            .map(|(key, value)| Ok((key, Marshaled::<V>::prepare(value)?)))
            .collect::<Result<_>>()?;
        Ok(from_storage::<K, Marshaled<V>>(map))
    }
}
