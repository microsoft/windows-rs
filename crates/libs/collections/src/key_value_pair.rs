use super::*;
use windows_core::*;

#[implement(IKeyValuePair<K, V>)]
pub(super) struct StockKeyValuePair<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone,
    V::Default: Clone,
{
    pub(super) key: K::Default,
    pub(super) value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone,
    V::Default: Clone,
{
    fn Key(&self) -> Result<K> {
        K::from_default(&self.key)
    }

    fn Value(&self) -> Result<V> {
        V::from_default(&self.value)
    }
}
