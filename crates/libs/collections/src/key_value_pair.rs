use super::*;
use storage::*;
use windows_core::*;

pub(super) struct StockKeyValuePair<K: RuntimeType + 'static, S: Storage> {
    pub(super) key: K::Default,
    pub(super) value: S::Stored,
}

// SAFETY: immutable pairs retain their source collection's threading contract.
unsafe impl<K: RuntimeType, S: Storage> Send for StockKeyValuePair<K, S> {}
unsafe impl<K: RuntimeType, S: Storage> Sync for StockKeyValuePair<K, S> {}

implement_decl! {
    impl<K, S> StockKeyValuePair as pub(super) StockKeyValuePair_Impl: [
        IKeyValuePair<K, S::Value>,
    ]
    where K: RuntimeType + 'static, S: Storage
}

impl<K: RuntimeType, S: Storage> IKeyValuePair_Impl<K, S::Value> for StockKeyValuePair_Impl<K, S> {
    fn Key(&self) -> Result<K> {
        K::from_default(&self.key)
    }

    fn Value(&self) -> Result<S::Value> {
        S::get(&self.value)
    }
}
