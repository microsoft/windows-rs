use super::*;

fn virtual_relation(kind: ObjectType) -> Option<RelationId> {
    relation_contracts(kind)
        .iter()
        .find(|contract| {
            contract.realization == Realization::Container
                && contract.child == ObjectCategory::Visual
        })
        .map(|contract| contract.id)
}

fn virtual_keys(items: &VirtualItems) -> Vec<Key> {
    let mut keys = Vec::with_capacity(items.len());
    let mut unique = HashSet::with_capacity(items.len());
    for index in 0..items.len() {
        let key = items.key(index).unwrap();
        assert!(unique.insert(key.clone()), "duplicate key {key:?}");
        keys.push(key);
    }
    keys
}

pub(super) fn retain_virtual_items(declaration: &Declaration) -> Option<Box<RetainedVirtualItems>> {
    let relation = virtual_relation(declaration.kind)?;
    let declared = declaration
        .virtual_items
        .as_deref()
        .cloned()
        .unwrap_or(DeclaredVirtualItems {
            relation,
            owner: declaration.component,
            items: VirtualItems::Eager(Rc::default()),
        });
    assert!(
        declared.relation == relation,
        "invalid relation {:?} for {:?}",
        declared.relation,
        declaration.kind
    );
    let keys = virtual_keys(&declared.items);
    Some(Box::new(RetainedVirtualItems {
        relation,
        owner: declared.owner.or(declaration.component),
        items: declared.items,
        keys: Rc::new(keys),
        source_revision: 0,
        lease_revision: 0,
        active: HashMap::new(),
        containers: HashMap::new(),
    }))
}

impl RetainedGraph {
    pub(crate) fn virtual_realization(
        &self,
        collection: ObjectId,
        container: RealizedContainer,
    ) -> Option<ObjectId> {
        self.get(collection)?
            .virtual_items
            .as_ref()?
            .containers
            .get(&container)?
            .child
    }

    /// Returns the key of the row a container currently holds.
    pub(crate) fn virtual_row_key(
        &self,
        collection: ObjectId,
        container: RealizedContainer,
    ) -> Option<Key> {
        self.get(collection)?
            .virtual_items
            .as_ref()?
            .containers
            .get(&container)
            .map(|realization| realization.key.clone())
    }

    /// Returns whether a realization must wait: it touches a row whose batch is being applied, or
    /// it renders a row whose effects are pending. Recycling or replacing a pending row retires it
    /// and discards its prepared effects, so those requests proceed.
    fn realization_busy(
        &self,
        request: RealizationRequest,
        applying: &[(ObjectId, Key)],
        pending: &[(ObjectId, Key)],
    ) -> bool {
        let (collection, container, index) = match request {
            RealizationRequest::Realize {
                collection,
                container,
                index,
                ..
            } => (collection, container, Some(index)),
            RealizationRequest::Recycle {
                collection,
                container,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                container,
                ..
            } => (collection, container, None),
        };
        let Some(items) = self
            .get(collection)
            .and_then(|object| object.virtual_items.as_ref())
        else {
            return false;
        };
        let contains = |rows: &[(ObjectId, Key)], key: Option<&Key>| {
            key.is_some_and(|key| rows.iter().any(|row| row.0 == collection && row.1 == *key))
        };
        let rendered = index.and_then(|index| items.keys.get(index));
        let contained = items
            .containers
            .get(&container)
            .map(|realization| &realization.key);
        contains(applying, rendered) || contains(applying, contained) || contains(pending, rendered)
    }

    fn validate_realization_request(&self, request: RealizationRequest) -> bool {
        let (collection, source_revision, index) = match request {
            RealizationRequest::Realize {
                collection,
                source_revision,
                index,
                ..
            } => (collection, source_revision, Some(index)),
            RealizationRequest::Recycle {
                collection,
                source_revision,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                source_revision,
                ..
            } => (collection, source_revision, None),
        };
        let Some(object) = self.get(collection) else {
            return false;
        };
        let Some(items) = &object.virtual_items else {
            panic!("{collection:?} does not realize virtual items");
        };
        if items.source_revision != source_revision {
            return false;
        }
        if let Some(index) = index {
            assert!(
                index < items.keys.len(),
                "invalid realization {index} for {collection:?}"
            );
        }
        true
    }

    pub(super) fn virtual_work(&self, request: RealizationRequest) -> Option<VirtualWork> {
        if !self.validate_realization_request(request) {
            return None;
        }
        match request {
            RealizationRequest::Realize {
                collection,
                container,
                index,
                ..
            } => {
                let items = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap();
                let key = items.keys[index].clone();
                let revision = match items.active.get(&key) {
                    Some((revision, _)) => *revision,
                    None => items.lease_revision.checked_add(1).unwrap(),
                };
                Some(VirtualWork::Realize {
                    lease: RealizationLease {
                        collection,
                        container,
                        key,
                        revision,
                    },
                    index,
                    view: Box::new(items.items.view(index).unwrap()),
                    owner: items.owner,
                })
            }
            RealizationRequest::Recycle {
                collection,
                container,
                ..
            } => {
                let items = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap();
                items
                    .containers
                    .get(&container)
                    .map(|realization| VirtualWork::Recycle {
                        lease: RealizationLease {
                            collection,
                            container,
                            key: realization.key.clone(),
                            revision: realization.revision,
                        },
                    })
            }
            RealizationRequest::Cancel {
                collection,
                container,
                ..
            } => {
                let relation = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap()
                    .relation;
                Some(VirtualWork::Cancel {
                    collection,
                    relation,
                    container,
                })
            }
        }
    }
}

impl<A: Adapter> Runtime<A> {
    /// Returns the virtual work for a realization at the front of the native queue, after applying
    /// the observations ahead of it. Returns `None` when a native event comes first, when the
    /// realization would replace one of the `busy` rows, or when a native event is being
    /// dispatched and a user observation would change the state it reads.
    pub(crate) fn next_realization(
        &mut self,
        applying: &[(ObjectId, Key)],
        pending: &[(ObjectId, Key)],
    ) -> Option<VirtualWork> {
        loop {
            let next = self.adapter.peek_native_event()?;
            if next.event.is_some() {
                return None;
            }
            match next.realization {
                Some(request) if self.graph.realization_busy(request, applying, pending) => {
                    return None;
                }
                Some(_) => {}
                None if self.native_event_active.get() && !next.record => return None,
                None => {}
            }
            match self.pop_native_work()? {
                NativeWork::Virtual(work) => return Some(work),
                NativeWork::Maintenance => {}
                NativeWork::Event(_) => unreachable!(),
            }
        }
    }

    pub(crate) fn plan_realize_virtual(
        &mut self,
        lease: &RealizationLease,
        index: usize,
        root: impl Into<View>,
    ) -> NativeBatch {
        self.mutations.clear();
        let mut declaration = root.into().0.object();
        declaration.key = Some(lease.key.clone());
        self.validator.validate(&declaration);
        let items = self
            .graph
            .get(lease.collection)
            .and_then(|object| object.virtual_items.as_ref())
            .unwrap();
        assert!(
            items.keys.get(index) == Some(&lease.key),
            "invalid realization {index} for {:?}",
            lease.collection
        );
        let relation_id = items.relation;
        let mut child = items
            .active
            .get(&lease.key)
            .and_then(|(_, container)| items.containers.get(container))
            .and_then(|active| active.child);
        let replaced_container = items
            .containers
            .get(&lease.container)
            .filter(|current| current.key != lease.key || current.revision != lease.revision)
            .map(|current| {
                (
                    current.child,
                    items
                        .active
                        .get(&current.key)
                        .is_some_and(|(_, container)| *container == lease.container),
                )
            });
        let expected_revision = match items.active.get(&lease.key) {
            Some((revision, _)) => Some(*revision),
            None => items.lease_revision.checked_add(1),
        };
        assert!(
            expected_revision == Some(lease.revision),
            "invalid realization {index} for {:?}",
            lease.collection
        );
        let moved_container = match items.active.get(&lease.key) {
            Some((_, container)) => (*container != lease.container).then(|| {
                (
                    *container,
                    items
                        .containers
                        .get(container)
                        .and_then(|active| active.child),
                )
            }),
            None => None,
        };
        let mut references = Vec::new();
        if let Some((previous, retire)) = replaced_container {
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container: lease.container,
                child: previous,
            });
            if retire && let Some(previous) = previous {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|current| *current != previous);
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.retire(previous);
            }
        }
        if let Some((container, previous_child)) = moved_container {
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container,
                child: previous_child,
            });
        }
        if let Some(current) = child {
            if self.graph.get(current).unwrap().kind == declaration.kind {
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.reconcile_object(current, &declaration);
            } else {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|candidate| *candidate != current);
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.retire(current);
                child = None;
            }
        }
        let child = if let Some(child) = child {
            child
        } else {
            let mut planner = Planner {
                retained: &mut self.graph,
                mutations: &mut self.mutations,
                references: &mut references,
                virtual_refreshes: None,
            };
            planner.mount(&declaration)
        };
        let items = self
            .graph
            .get_mut(lease.collection)
            .virtual_items
            .as_mut()
            .unwrap();
        if !items.active.contains_key(&lease.key) {
            items.lease_revision = lease.revision;
        }
        if let Some((_, previous)) = items
            .active
            .insert(lease.key.clone(), (lease.revision, lease.container))
        {
            items.containers.remove(&previous);
        }
        if let Some(previous) = items.containers.insert(
            lease.container,
            RetainedRealization {
                key: lease.key.clone(),
                index,
                revision: lease.revision,
                child: Some(child),
            },
        ) && previous.key != lease.key
        {
            items.active.remove(&previous.key);
        }
        let ordered = {
            let mut active = items
                .containers
                .values()
                .filter_map(|realization| realization.child.map(|child| (realization.index, child)))
                .collect::<Vec<_>>();
            active.sort_unstable_by_key(|(index, _)| *index);
            active.into_iter().map(|(_, child)| child).collect()
        };
        let relation = self.graph.relation_mut(lease.collection, relation_id);
        let RetainedRelationValue::Many(children) = &mut relation.value else {
            unreachable!();
        };
        *children = ordered;
        self.mutations.push(Mutation::Realize {
            parent: lease.collection,
            relation: relation_id,
            container: lease.container,
            index,
            child,
        });
        self.take_batch(references)
    }

    pub(crate) fn plan_recycle_virtual(&mut self, lease: &RealizationLease) -> NativeBatch {
        self.mutations.clear();
        let items = self
            .graph
            .get(lease.collection)
            .and_then(|object| object.virtual_items.as_ref())
            .unwrap();
        let mut references = Vec::new();
        if let Some(realization) = items.containers.get(&lease.container)
            && realization.key == lease.key
            && realization.revision == lease.revision
        {
            let relation_id = items.relation;
            let child = realization.child;
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container: lease.container,
                child,
            });
            let items = self
                .graph
                .get_mut(lease.collection)
                .virtual_items
                .as_mut()
                .unwrap();
            items.containers.remove(&lease.container);
            if items
                .active
                .get(&lease.key)
                .is_some_and(|(_, container)| *container == lease.container)
            {
                items.active.remove(&lease.key);
            }
            if let Some(child) = child {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|current| *current != child);
                Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                }
                .retire(child);
            }
        }
        self.take_batch(references)
    }

    pub(crate) fn plan_cancel_virtual(
        &mut self,
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    ) -> NativeBatch {
        self.mutations.clear();
        self.mutations.push(Mutation::Recycle {
            parent: collection,
            relation,
            container,
            child: None,
        });
        self.take_batch(Vec::new())
    }

    pub(crate) fn virtual_lease_active(&self, collection: ObjectId, key: &Key) -> bool {
        self.graph
            .get(collection)
            .and_then(|object| object.virtual_items.as_ref())
            .is_some_and(|items| items.active.contains_key(key))
    }
}

impl Planner<'_> {
    pub(super) fn reconcile_virtual_items(&mut self, object: ObjectId, declaration: &Declaration) {
        let Some(previous) = self.retained.get(object).unwrap().virtual_items.clone() else {
            return;
        };
        let declared =
            declaration
                .virtual_items
                .as_deref()
                .cloned()
                .unwrap_or(DeclaredVirtualItems {
                    relation: previous.relation,
                    owner: declaration.component,
                    items: VirtualItems::Eager(Rc::default()),
                });
        assert!(
            declared.relation == previous.relation,
            "invalid relation {:?} for {:?}",
            declared.relation,
            declaration.kind
        );
        let keys = match (&previous.items, &declared.items) {
            (VirtualItems::Lazy(previous_source), VirtualItems::Lazy(next_source))
                if previous_source.key_revision == next_source.key_revision
                    && previous_source.len == next_source.len =>
            {
                Rc::clone(&previous.keys)
            }
            _ => Rc::new(virtual_keys(&declared.items)),
        };
        let items_changed = previous.items != declared.items;
        let keys_changed = previous.keys.as_ref() != keys.as_ref();
        let source_revision = if keys_changed {
            previous.source_revision.checked_add(1).unwrap()
        } else {
            previous.source_revision
        };
        let old_by_key = previous
            .containers
            .values()
            .map(|realization| (realization.key.clone(), realization.clone()))
            .collect::<HashMap<_, _>>();
        let represented = previous
            .containers
            .values()
            .filter_map(|realization| keys.get(realization.index))
            .cloned()
            .collect::<HashSet<_>>();
        let mut lease_revision = previous.lease_revision;
        let mut containers = HashMap::with_capacity(previous.containers.len());
        let mut active = HashMap::with_capacity(previous.active.len());
        let mut retired = HashSet::new();
        let mut previous_containers = previous.containers.iter().collect::<Vec<_>>();
        previous_containers
            .sort_unstable_by_key(|(container, realization)| (realization.index, container.0));
        for (container, current) in previous_containers {
            let Some(key) = keys.get(current.index).cloned() else {
                self.mutations.push(Mutation::Recycle {
                    parent: object,
                    relation: previous.relation,
                    container: *container,
                    child: current.child,
                });
                if let Some(child) = current.child
                    && !represented.contains(&current.key)
                    && retired.insert(child)
                {
                    self.retire(child);
                }
                continue;
            };
            let preserved = old_by_key.get(&key);
            let child = preserved.and_then(|realization| realization.child);
            let revision = if let Some(realization) = preserved {
                realization.revision
            } else {
                lease_revision = lease_revision.checked_add(1).unwrap();
                lease_revision
            };
            if current.key != key || current.child != child {
                self.mutations.push(Mutation::Recycle {
                    parent: object,
                    relation: previous.relation,
                    container: *container,
                    child: current.child,
                });
                if let Some(current_child) = current.child
                    && !represented.contains(&current.key)
                    && retired.insert(current_child)
                {
                    self.retire(current_child);
                }
            }
            containers.insert(
                *container,
                RetainedRealization {
                    key: key.clone(),
                    revision,
                    index: current.index,
                    child,
                },
            );
            active.insert(key, (revision, *container));
        }
        let children = {
            let mut children = containers
                .values()
                .filter_map(|realization| realization.child.map(|child| (realization.index, child)))
                .collect::<Vec<_>>();
            children.sort_unstable_by_key(|(index, _)| *index);
            children.into_iter().map(|(_, child)| child).collect()
        };
        let relation = self.retained.relation_mut(object, previous.relation);
        let RetainedRelationValue::Many(retained_children) = &mut relation.value else {
            unreachable!();
        };
        *retained_children = children;
        let retained = self
            .retained
            .get_mut(object)
            .virtual_items
            .as_mut()
            .unwrap();
        retained.owner = declared.owner.or(declaration.component);
        retained.items = declared.items;
        retained.keys = keys;
        retained.source_revision = source_revision;
        retained.lease_revision = lease_revision;
        retained.containers = containers;
        retained.active = active;
        if keys_changed {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: retained.keys.len(),
                source_revision,
            });
        }
        if (items_changed || keys_changed)
            && let Some(refreshes) = self.virtual_refreshes.as_mut()
        {
            let mut containers = retained.containers.iter().collect::<Vec<_>>();
            containers
                .sort_unstable_by_key(|(container, realization)| (realization.index, container.0));
            refreshes.extend(containers.into_iter().map(|(container, realization)| {
                RealizationRequest::Realize {
                    collection: object,
                    container: *container,
                    index: realization.index,
                    source_revision,
                }
            }));
        }
    }
}
