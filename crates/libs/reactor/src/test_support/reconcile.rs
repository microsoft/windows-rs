use super::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RetainedMemory {
    pub graph_size: usize,
    pub object_id_size: usize,
    pub object_type_size: usize,
    pub key_size: usize,
    pub optional_key_size: usize,
    pub optional_reference_size: usize,
    pub optional_transition_size: usize,
    pub property_size: usize,
    pub event_size: usize,
    pub property_list_size: usize,
    pub event_list_size: usize,
    pub retained_event_list_size: usize,
    pub relation_list_size: usize,
    pub optional_virtual_items_size: usize,
    pub slot_size: usize,
    pub object_size: usize,
    pub relation_size: usize,
    pub relation_value_size: usize,
    pub virtual_items_size: usize,
    pub retirement_size: usize,
    pub hash_map_size: usize,
    pub slot_len: usize,
    pub slot_capacity: usize,
    pub slot_bytes: usize,
    pub live_objects: usize,
    pub keyed_objects: usize,
    pub property_lists: usize,
    pub property_entries: usize,
    pub event_lists: usize,
    pub event_entries: usize,
    pub relation_entries: usize,
    pub relation_capacity: usize,
    pub relation_bytes: usize,
    pub child_entries: usize,
    pub child_capacity: usize,
    pub child_bytes: usize,
    pub virtual_items: usize,
    pub virtual_bytes: usize,
    pub retirement_entries: usize,
    pub retirement_node_capacity: usize,
    pub retirement_node_bytes: usize,
    pub free_len: usize,
    pub free_capacity: usize,
    pub free_bytes: usize,
}

#[cfg(test)]
impl PartialEq for ReferenceScanCounter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[cfg(test)]
impl ObjectId {
    pub(crate) const fn test(index: u32) -> Self {
        Self {
            index,
            generation: 0,
        }
    }
}

#[cfg(test)]
impl UpdateStats {
    pub fn as_slice(&self) -> &[Mutation] {
        &self.details
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Mutation> {
        self.details.iter()
    }

    pub fn first(&self) -> Option<&Mutation> {
        self.details.first()
    }
}

impl EventDispatch {
    pub(crate) fn object(&self) -> ObjectId {
        self.object
    }
}

impl NativeEvent {
    /// Replaces a reorder observation and its payload with the order `read` reports.
    pub(crate) fn read_reorder(
        mut self,
        read: impl FnOnce(ObjectId, RelationId) -> Option<Vec<ObjectId>>,
    ) -> Self {
        let (observation, event) = match &mut self {
            Self::Observation(observation) | Self::Record(observation) => (Some(observation), None),
            Self::Event { observation, event } => (observation.as_mut(), Some(event)),
            Self::Retirement(_) | Self::Realization(_) => (None, None),
        };
        if let Some(Observation::Reorder {
            object,
            relation,
            children,
        }) = observation
            && let Some(native) = read(*object, *relation)
        {
            if let Some(EventDispatch {
                call: EventCall::Items(_, items),
                ..
            }) = event
            {
                items.clone_from(&native);
            }
            *children = native;
        }
        self
    }

    pub(crate) fn object(&self) -> ObjectId {
        match self {
            Self::Event { event, .. } => event.object(),
            Self::Observation(observation) | Self::Record(observation) => observation.object(),
            Self::Retirement(completion) => completion.root,
            Self::Realization(
                RealizationRequest::Realize { collection, .. }
                | RealizationRequest::Recycle { collection, .. }
                | RealizationRequest::Cancel { collection, .. },
            ) => *collection,
        }
    }
}

impl RetainedGraph {
    pub fn retained_memory(&self) -> RetainedMemory {
        let mut memory = RetainedMemory {
            graph_size: size_of::<Self>(),
            object_id_size: size_of::<ObjectId>(),
            object_type_size: size_of::<ObjectType>(),
            key_size: size_of::<Key>(),
            optional_key_size: size_of::<Option<Key>>(),
            optional_reference_size: size_of::<Option<ElementRef>>(),
            optional_transition_size: size_of::<Option<ExitTransition>>(),
            property_size: size_of::<Property>(),
            event_size: size_of::<Event>(),
            property_list_size: size_of::<SharedList<Property>>(),
            event_list_size: size_of::<SharedList<Event>>(),
            retained_event_list_size: size_of::<Option<Rc<Vec<Event>>>>(),
            relation_list_size: size_of::<Vec<RetainedRelation>>(),
            optional_virtual_items_size: size_of::<Option<Box<RetainedVirtualItems>>>(),
            slot_size: size_of::<RetainedSlot>(),
            object_size: size_of::<RetainedObject>(),
            relation_size: size_of::<RetainedRelation>(),
            relation_value_size: size_of::<RetainedRelationValue>(),
            virtual_items_size: size_of::<RetainedVirtualItems>(),
            retirement_size: size_of::<RetainedRetirement>(),
            hash_map_size: size_of::<HashMap<Key, (u64, RealizedContainer)>>(),
            slot_len: self.objects.len(),
            slot_capacity: self.objects.capacity(),
            slot_bytes: self.objects.capacity() * size_of::<RetainedSlot>(),
            free_len: self.free.len(),
            free_capacity: self.free.capacity(),
            free_bytes: self.free.capacity() * size_of::<u32>(),
            retirement_entries: self.retirements.len(),
            ..Default::default()
        };
        for slot in &self.objects {
            let Some(object) = &slot.object else {
                continue;
            };
            memory.live_objects += 1;
            memory.keyed_objects += usize::from(object.key.is_some());
            memory.property_lists += usize::from(!object.properties.as_slice().is_empty());
            memory.property_entries += object.properties.as_slice().len();
            memory.event_lists += usize::from(object.events.is_some());
            memory.event_entries += retained_events(&object.events).len();
            memory.relation_entries += object.relations.len();
            memory.relation_capacity += object.relations.capacity();
            memory.relation_bytes += object.relations.capacity() * size_of::<RetainedRelation>();
            for relation in &object.relations {
                if let RetainedRelationValue::Many(children) = &relation.value {
                    memory.child_entries += children.len();
                    memory.child_capacity += children.capacity();
                    memory.child_bytes += children.capacity() * size_of::<ObjectId>();
                }
            }
            if object.virtual_items.is_some() {
                memory.virtual_items += 1;
                memory.virtual_bytes += size_of::<RetainedVirtualItems>();
            }
        }
        for retirement in self.retirements.values() {
            memory.retirement_node_capacity += retirement.nodes.capacity();
            memory.retirement_node_bytes += retirement.nodes.capacity() * size_of::<ObjectId>();
        }
        memory
    }

    #[cfg(test)]
    pub(crate) fn full_reference_scan_count(&self) -> usize {
        self.full_reference_scans.0.get()
    }

    pub fn objects(&self) -> impl Iterator<Item = ObjectId> + '_ {
        self.objects.iter().enumerate().filter_map(|(index, slot)| {
            (slot.object.is_some() && !slot.retiring).then_some(ObjectId {
                index: index.try_into().unwrap(),
                generation: slot.generation,
            })
        })
    }
}

#[cfg(feature = "test")]
impl<A: Adapter> Runtime<A> {
    pub fn release_test_scratch(&mut self) {
        self.mutations = Vec::new();
        self.virtual_refreshes = VecDeque::new();
        self.validator = DeclarationValidator::default();
    }
}

#[cfg(test)]
impl std::ops::Index<usize> for UpdateStats {
    type Output = Mutation;

    fn index(&self, index: usize) -> &Self::Output {
        &self.details[index]
    }
}

#[cfg(test)]
impl IntoIterator for UpdateStats {
    type Item = Mutation;
    type IntoIter = std::vec::IntoIter<Mutation>;

    fn into_iter(self) -> Self::IntoIter {
        self.details.into_iter()
    }
}

#[cfg(test)]
impl PartialEq<Vec<Mutation>> for UpdateStats {
    fn eq(&self, other: &Vec<Mutation>) -> bool {
        self.details == *other
    }
}

#[cfg(test)]
impl<const N: usize> PartialEq<[Mutation; N]> for UpdateStats {
    fn eq(&self, other: &[Mutation; N]) -> bool {
        self.details == *other
    }
}

impl UpdateStats {
    pub const fn is_empty(&self) -> bool {
        self.mutations == 0
    }
}

impl RetainedGraph {
    pub fn window_title_bar(&self) -> Option<(ObjectId, WindowTitleBarHeight)> {
        self.window_title_bar
    }

    pub fn object_count(&self) -> usize {
        self.objects
            .iter()
            .filter(|slot| slot.object.is_some() && !slot.retiring)
            .count()
    }

    pub fn retired_count(&self) -> usize {
        self.retirements.len()
    }

    pub fn properties(&self, object: ObjectId) -> Option<&[Property]> {
        self.get(object).map(|object| object.properties.as_slice())
    }

    pub fn events(&self, object: ObjectId) -> Option<&[Event]> {
        self.get(object)
            .map(|object| retained_events(&object.events))
    }
}

impl Runtime<RecordingAdapter> {
    pub fn focus<T>(&mut self, reference: &ElementRef<T>) -> bool {
        let Some(object) = reference.get() else {
            return false;
        };
        assert!(focus_capable(self.graph.kind(object).unwrap()));
        self.adapter.focus(object)
    }
}

impl<A: Adapter> Runtime<A> {
    pub fn next_native_event(&mut self) -> Option<NativeEventDispatch> {
        assert!(
            !self.native_event_active.get(),
            "native event dispatch is in progress"
        );
        while let Some(next) = self.adapter.peek_native_event() {
            assert!(
                !matches!(next, NativeEvent::Realization(_)),
                "virtual realization is pending"
            );
            if let Some(NativeWork::Event(event)) = self.pop_native_work() {
                return Some(event);
            }
        }
        None
    }

    pub fn dispatch_native_events(&mut self) -> usize {
        let mut dispatched = 0;
        while let Some(work) = self.next_native_work() {
            match work {
                NativeWork::Event(mut event) => {
                    event.invoke();
                    dispatched += 1;
                }
                NativeWork::Virtual(VirtualWork::Realize {
                    lease, index, view, ..
                }) => {
                    self.realize_virtual(&lease, index, *view);
                }
                NativeWork::Virtual(VirtualWork::Recycle { lease }) => {
                    self.recycle_virtual(&lease);
                }
                NativeWork::Virtual(VirtualWork::Cancel {
                    collection,
                    relation,
                    container,
                }) => {
                    self.cancel_virtual(collection, relation, container);
                }
                NativeWork::Maintenance => {}
            }
        }
        dispatched
    }

    pub(crate) fn realize_virtual(
        &mut self,
        lease: &RealizationLease,
        index: usize,
        root: impl Into<View>,
    ) -> UpdateStats {
        let batch = self.plan_realize_virtual(lease, index, root);
        self.apply_batch(batch)
    }

    pub(crate) fn recycle_virtual(&mut self, lease: &RealizationLease) -> UpdateStats {
        let batch = self.plan_recycle_virtual(lease);
        self.apply_batch(batch)
    }

    pub(crate) fn cancel_virtual(
        &mut self,
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    ) -> UpdateStats {
        let batch = self.plan_cancel_virtual(collection, relation, container);
        self.apply_batch(batch)
    }

    pub fn update_subtree(&mut self, object: ObjectId, root: impl Into<View>) -> UpdateStats {
        let batch = self.plan_subtree_update(object, root);
        self.apply_batch(batch)
    }

    pub fn remove_child(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> UpdateStats {
        self.assert_ready();
        self.mutations.clear();
        let RetainedRelationValue::Many(children) =
            &self.graph.relation(parent, relation).unwrap().value
        else {
            panic!("{relation:?} is not a collection relation");
        };
        let index = children
            .iter()
            .position(|current| *current == child)
            .unwrap();
        let mut references = Vec::new();
        let mut planner = Planner {
            retained: &mut self.graph,
            mutations: &mut self.mutations,
            references: &mut references,
            virtual_refreshes: None,
        };
        let mut native = planner.retained.native_children(parent, relation);
        planner.remove_child(parent, relation, &mut native, index, child);
        planner
            .retained
            .set_native_children(parent, relation, native);
        let RetainedRelationValue::Many(children) =
            &mut self.graph.relation_mut(parent, relation).value
        else {
            unreachable!()
        };
        children.remove(index);
        let batch = self.take_batch(references);
        self.apply_batch(batch)
    }
}

impl<A: Adapter> Runtime<A> {
    pub(crate) fn adapter_mut(&mut self) -> &mut A {
        &mut self.adapter
    }
}

impl UpdateStats {
    pub const fn len(&self) -> usize {
        self.mutations
    }
}
