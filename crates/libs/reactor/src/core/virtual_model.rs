use rustc_hash::FxHashMap;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationLease {
    pub identity: WindowToken,
    pub collection: NodeId,
    pub container: RealizedContainer,
    pub key: Key,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RealizedContainer(pub u64);

#[derive(Clone)]
pub struct VirtualModel {
    active: HashMap<Key, (u64, RealizedContainer)>,
    collection: NodeId,
    containers: FxHashMap<RealizedContainer, (Key, u64)>,
    identity: WindowToken,
    keys: Rc<Vec<Key>>,
    revision: u64,
    source_revision: u64,
}

impl VirtualModel {
    pub fn new(
        identity: WindowToken,
        collection: NodeId,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<Self, DuplicateKeyError<Key>> {
        let keys = keys.into_iter().collect::<Vec<_>>();
        diff(&[], &keys)?;
        Ok(Self {
            active: HashMap::new(),
            collection,
            containers: FxHashMap::default(),
            identity,
            keys: Rc::new(keys),
            revision: 0,
            source_revision: 0,
        })
    }

    pub fn update(
        &mut self,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<Vec<KeyedOperation<Key>>, DuplicateKeyError<Key>> {
        let keys = keys.into_iter().collect::<Vec<_>>();
        let operations = diff(&self.keys, &keys)?;
        let source_revision = self.source_revision.checked_add(1).unwrap();
        let retained = keys.iter().cloned().collect::<HashSet<_>>();
        self.active.retain(|key, _| retained.contains(key));
        self.containers.retain(|_, (key, _)| retained.contains(key));
        self.keys = Rc::new(keys);
        self.source_revision = source_revision;
        Ok(operations)
    }

    pub fn realize(
        &mut self,
        index: usize,
        container: RealizedContainer,
    ) -> Option<RealizationLease> {
        let key = self.keys.get(index).cloned()?;
        self.revision = self.revision.checked_add(1).unwrap();
        if let Some((old_key, old_revision)) = self.containers.remove(&container)
            && self.active.get(&old_key) == Some(&(old_revision, container))
        {
            self.active.remove(&old_key);
        }
        if let Some((_, old_container)) = self.active.remove(&key) {
            self.containers.remove(&old_container);
        }
        self.active.insert(key.clone(), (self.revision, container));
        self.containers
            .insert(container, (key.clone(), self.revision));
        Some(RealizationLease {
            identity: self.identity,
            collection: self.collection,
            container,
            key,
            revision: self.revision,
        })
    }

    #[cfg(test)]
    pub fn accepts(&self, lease: &RealizationLease) -> bool {
        lease.identity == self.identity
            && lease.collection == self.collection
            && self.active.get(&lease.key) == Some(&(lease.revision, lease.container))
    }

    #[cfg(test)]
    pub fn recycle(&mut self, lease: &RealizationLease) -> bool {
        if !self.accepts(lease) {
            return false;
        }
        self.active.remove(&lease.key);
        self.containers.remove(&lease.container);
        true
    }

    pub fn recycle_container(&mut self, container: RealizedContainer) -> Option<RealizationLease> {
        let (key, revision) = self.containers.remove(&container)?;
        if self.active.get(&key) != Some(&(revision, container)) {
            return None;
        }
        self.active.remove(&key);
        Some(RealizationLease {
            identity: self.identity,
            collection: self.collection,
            container,
            key,
            revision,
        })
    }

    pub fn clear(&mut self) {
        self.active.clear();
        self.containers.clear();
    }

    #[cfg(test)]
    pub fn active_len(&self) -> usize {
        self.active.len()
    }

    pub fn keys(&self) -> &[Key] {
        &self.keys
    }

    pub fn source_revision(&self) -> u64 {
        self.source_revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLLECTION: NodeId = NodeId::from_parts(4, 2);
    const FIRST: RealizedContainer = RealizedContainer(10);
    const SECOND: RealizedContainer = RealizedContainer(11);

    fn identity() -> WindowToken {
        WindowToken::new(WindowId::allocate())
    }

    fn keys(values: &[&str]) -> Vec<Key> {
        values.iter().copied().map(Key::from).collect()
    }

    #[test]
    fn update_uses_shared_keyed_operations_and_retains_moved_lease() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a", "b", "c"])).unwrap();
        let lease = model.realize(1, FIRST).unwrap();

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
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a", "b"])).unwrap();
        let removed = model.realize(0, FIRST).unwrap();
        let recycled = model.realize(1, SECOND).unwrap();

        model.update(keys(&["b"])).unwrap();

        assert!(!model.accepts(&removed));
        assert!(model.recycle(&recycled));
        assert!(!model.accepts(&recycled));
        assert!(!model.recycle(&recycled));
    }

    #[test]
    fn replacement_realization_invalidates_older_callback() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a"])).unwrap();
        let first = model.realize(0, FIRST).unwrap();
        let second = model.realize(0, SECOND).unwrap();

        assert!(!model.accepts(&first));
        assert!(model.accepts(&second));
        assert_eq!(model.recycle_container(FIRST), None);
        assert_eq!(model.recycle_container(SECOND), Some(second));
    }

    #[test]
    fn collection_identity_rejects_foreign_lease() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a"])).unwrap();
        let lease = model.realize(0, FIRST).unwrap();
        let foreign = RealizationLease {
            collection: NodeId::from_parts(5, 2),
            ..lease
        };

        assert!(!model.accepts(&foreign));
    }

    #[test]
    fn reused_container_invalidates_its_previous_key() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a", "b"])).unwrap();
        let first = model.realize(0, FIRST).unwrap();
        let second = model.realize(1, FIRST).unwrap();

        assert!(!model.accepts(&first));
        assert!(model.accepts(&second));
        assert_eq!(model.active_len(), 1);
        assert_eq!(model.recycle_container(FIRST), Some(second));
    }

    #[test]
    fn repeated_realize_recycle_returns_to_zero_resources() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a", "b", "c"])).unwrap();

        for _ in 0..10_000 {
            for index in 0..model.keys().len() {
                let lease = model
                    .realize(index, RealizedContainer(index as u64))
                    .unwrap();
                assert!(model.recycle(&lease));
            }
            assert_eq!(model.active_len(), 0);
        }

        model.clear();
        assert_eq!(model.active_len(), 0);
    }

    #[test]
    fn rejects_duplicate_keys_and_out_of_range_realizations() {
        assert_eq!(
            VirtualModel::new(identity(), COLLECTION, keys(&["a", "a"])).err(),
            Some(DuplicateKeyError(Key::from("a")))
        );
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a"])).unwrap();
        assert_eq!(model.realize(1, FIRST), None);
        assert_eq!(
            model.update(keys(&["a", "a"])),
            Err(DuplicateKeyError(Key::from("a")))
        );
    }

    #[test]
    fn source_revision_exhaustion_panics_before_update() {
        let mut model = VirtualModel::new(identity(), COLLECTION, keys(&["a"])).unwrap();
        model.source_revision = u64::MAX;

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            _ = model.update(keys(&["b"]));
        }));

        assert!(result.is_err());
        assert_eq!(model.keys(), keys(&["a"]));
        assert_eq!(model.source_revision(), u64::MAX);
    }
}
