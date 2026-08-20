use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationLease {
    pub collection: NodeId,
    pub key: Key,
    pub revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VirtualModelError {
    DuplicateKey(Key),
    MissingIndex(usize),
    RevisionExhausted,
}

#[derive(Clone)]
pub struct VirtualModel {
    active: HashMap<Key, u64>,
    collection: NodeId,
    keys: Vec<Key>,
    revision: u64,
}

impl VirtualModel {
    pub fn new(
        collection: NodeId,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<Self, VirtualModelError> {
        let keys = keys.into_iter().collect::<Vec<_>>();
        diff(&[], &keys)
            .map_err(|KeyedError::DuplicateKey(key)| VirtualModelError::DuplicateKey(key))?;
        Ok(Self {
            active: HashMap::new(),
            collection,
            keys,
            revision: 0,
        })
    }

    pub fn update(
        &mut self,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<Vec<KeyedOperation<Key>>, VirtualModelError> {
        let keys = keys.into_iter().collect::<Vec<_>>();
        let operations = diff(&self.keys, &keys)
            .map_err(|KeyedError::DuplicateKey(key)| VirtualModelError::DuplicateKey(key))?;
        let retained = keys.iter().cloned().collect::<HashSet<_>>();
        self.active.retain(|key, _| retained.contains(key));
        self.keys = keys;
        Ok(operations)
    }

    pub fn realize(&mut self, index: usize) -> Result<RealizationLease, VirtualModelError> {
        let key = self
            .keys
            .get(index)
            .cloned()
            .ok_or(VirtualModelError::MissingIndex(index))?;
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(VirtualModelError::RevisionExhausted)?;
        self.active.insert(key.clone(), self.revision);
        Ok(RealizationLease {
            collection: self.collection,
            key,
            revision: self.revision,
        })
    }

    pub fn accepts(&self, lease: &RealizationLease) -> bool {
        lease.collection == self.collection && self.active.get(&lease.key) == Some(&lease.revision)
    }

    pub fn recycle(&mut self, lease: &RealizationLease) -> bool {
        if !self.accepts(lease) {
            return false;
        }
        self.active.remove(&lease.key);
        true
    }

    pub fn clear(&mut self) {
        self.active.clear();
    }

    pub fn active_len(&self) -> usize {
        self.active.len()
    }

    pub fn keys(&self) -> &[Key] {
        &self.keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLLECTION: NodeId = NodeId::from_parts(4, 2);

    fn keys(values: &[&str]) -> Vec<Key> {
        values.iter().copied().map(Key::from).collect()
    }

    #[test]
    fn update_uses_shared_keyed_operations_and_retains_moved_lease() {
        let mut model = VirtualModel::new(COLLECTION, keys(&["a", "b", "c"])).unwrap();
        let lease = model.realize(1).unwrap();

        let operations = model.update(keys(&["c", "b", "d"])).unwrap();

        assert_eq!(model.keys(), keys(&["c", "b", "d"]));
        assert!(operations
            .iter()
            .any(|operation| matches!(operation, KeyedOperation::Remove { key } if key == &Key::from("a"))));
        assert!(operations
            .iter()
            .any(|operation| matches!(operation, KeyedOperation::Insert { key, .. } if key == &Key::from("d"))));
        assert!(model.accepts(&lease));
    }

    #[test]
    fn removed_and_recycled_leases_are_rejected() {
        let mut model = VirtualModel::new(COLLECTION, keys(&["a", "b"])).unwrap();
        let removed = model.realize(0).unwrap();
        let recycled = model.realize(1).unwrap();

        model.update(keys(&["b"])).unwrap();

        assert!(!model.accepts(&removed));
        assert!(model.recycle(&recycled));
        assert!(!model.accepts(&recycled));
        assert!(!model.recycle(&recycled));
    }

    #[test]
    fn replacement_realization_invalidates_older_callback() {
        let mut model = VirtualModel::new(COLLECTION, keys(&["a"])).unwrap();
        let first = model.realize(0).unwrap();
        let second = model.realize(0).unwrap();

        assert!(!model.accepts(&first));
        assert!(model.accepts(&second));
    }

    #[test]
    fn collection_identity_rejects_foreign_lease() {
        let mut model = VirtualModel::new(COLLECTION, keys(&["a"])).unwrap();
        let lease = model.realize(0).unwrap();
        let foreign = RealizationLease {
            collection: NodeId::from_parts(5, 2),
            ..lease
        };

        assert!(!model.accepts(&foreign));
    }

    #[test]
    fn repeated_realize_recycle_returns_to_zero_resources() {
        let mut model = VirtualModel::new(COLLECTION, keys(&["a", "b", "c"])).unwrap();

        for _ in 0..10_000 {
            for index in 0..model.keys().len() {
                let lease = model.realize(index).unwrap();
                assert!(model.recycle(&lease));
            }
            assert_eq!(model.active_len(), 0);
        }

        model.clear();
        assert_eq!(model.active_len(), 0);
    }

    #[test]
    fn rejects_duplicate_keys_and_missing_indices() {
        assert_eq!(
            VirtualModel::new(COLLECTION, keys(&["a", "a"])).err(),
            Some(VirtualModelError::DuplicateKey(Key::from("a")))
        );
        let mut model = VirtualModel::new(COLLECTION, keys(&["a"])).unwrap();
        assert_eq!(model.realize(1), Err(VirtualModelError::MissingIndex(1)));
        assert_eq!(
            model.update(keys(&["a", "a"])),
            Err(VirtualModelError::DuplicateKey(Key::from("a")))
        );
    }
}
