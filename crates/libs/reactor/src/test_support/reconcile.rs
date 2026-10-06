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
    pub(crate) fn realization_request(&self) -> Option<RealizationRequest> {
        self.realization
    }

    pub(crate) fn object(&self) -> Option<ObjectId> {
        self.event.as_ref().map(EventDispatch::object).or_else(|| {
            self.observation
                .as_ref()
                .map(|observation| match observation {
                    Observation::SetProperty { object, .. }
                    | Observation::SetSelection { object, .. } => *object,
                })
                .or_else(|| self.retirement.map(|completion| completion.root))
                .or_else(|| {
                    self.realization.map(|request| match request {
                        RealizationRequest::Realize { collection, .. }
                        | RealizationRequest::Recycle { collection, .. }
                        | RealizationRequest::Cancel { collection, .. } => collection,
                    })
                })
        })
    }
}

impl RetainedGraph {
    pub(crate) fn menu(&self, target: ObjectId) -> Option<(&Menu, u64)> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.menu.as_ref())
            .map(|menu| (&menu.menu, menu.revision))
    }

    pub(crate) fn command_bar_flyout(&self, target: ObjectId) -> Option<(&CommandBarFlyout, u64)> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.command_bar_flyout.as_ref())
            .map(|flyout| (&flyout.flyout, flyout.revision))
    }

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

    #[cfg(feature = "test")]
    pub fn virtual_source_revision(&self, object: ObjectId) -> Option<u64> {
        self.get(object)
            .and_then(|object| object.virtual_items.as_ref())
            .map(|items| items.source_revision)
    }
}

#[cfg(feature = "test")]
impl<A: Adapter> Runtime<A> {
    pub fn release_test_scratch(&mut self) {
        self.mutations = Vec::new();
        self.native_events = Vec::new();
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

#[cfg(test)]
pub(crate) fn panic_during_transaction(graph: &mut RetainedGraph, object: ObjectId) {
    let mut transaction = GraphTransaction::new(graph);
    transaction.get_mut(object).exit_transition = ExitTransition::fade(Duration::from_millis(1));
    panic!("test planning unwind");
}

impl UpdateStats {
    pub const fn is_empty(&self) -> bool {
        self.mutations == 0
    }
}

impl NativeEvent {
    pub fn observation(observation: Observation) -> Self {
        Self::new(Some(observation), None)
    }

    pub fn event(event: EventDispatch) -> Self {
        Self::new(None, Some(event))
    }
}

impl RetainedGraph {
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

impl<A: Adapter> Runtime<A> {
    pub fn focus<T>(&mut self, reference: &ElementRef<T>) -> Result<bool, UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        let object = reference
            .get()
            .ok_or(UpdateError::Graph(GraphError::ReferenceUnavailable))?;
        let kind = self
            .graph
            .kind(object)
            .ok_or(UpdateError::Graph(GraphError::StaleObject(object)))?;
        if !focus_capable(kind) {
            return Err(UpdateError::Graph(GraphError::InvalidFocus(kind)));
        }
        self.adapter.focus(object).map_err(UpdateError::Adapter)
    }

    pub fn next_native_event(
        &mut self,
    ) -> Result<Option<NativeEventDispatch>, UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        if self.native_event_active.get() {
            return Err(UpdateError::NativeEventInProgress);
        }
        loop {
            self.preview_native_events()?;
            let Some(next) = self.native_events.first() else {
                return Ok(None);
            };
            if next.realization.is_some() {
                return Err(UpdateError::PendingNativeEvent);
            }
            if let Some(event) = self.pop_native_event()? {
                self.native_event_active.set(true);
                return Ok(Some(NativeEventDispatch::new(
                    event,
                    Rc::clone(&self.native_event_active),
                )));
            }
        }
    }

    pub fn dispatch_native_events(&mut self) -> Result<usize, UpdateError<A::Error>> {
        let mut dispatched = 0;
        while let Some(work) = self.next_native_work()? {
            match work {
                NativeWork::Event(mut event) => {
                    event.invoke();
                    dispatched += 1;
                }
                NativeWork::Virtual(VirtualWork::Realize {
                    lease, index, view, ..
                }) => {
                    self.realize_virtual(&lease, index, *view)?;
                }
                NativeWork::Virtual(VirtualWork::Recycle { lease }) => {
                    self.recycle_virtual(&lease)?;
                }
                NativeWork::Virtual(VirtualWork::Cancel {
                    collection,
                    relation,
                    container,
                }) => {
                    self.cancel_virtual(collection, relation, container)?;
                }
                NativeWork::Maintenance => {}
            }
        }
        Ok(dispatched)
    }

    pub(crate) fn recycle_virtual(
        &mut self,
        lease: &RealizationLease,
    ) -> Result<UpdateStats, UpdateError<A::Error>> {
        self.recycle_virtual_before_apply(lease, || {})
    }

    pub fn update_subtree(
        &mut self,
        object: ObjectId,
        root: impl Into<View>,
    ) -> Result<UpdateStats, UpdateError<A::Error>> {
        self.update_subtree_before_apply(object, root, || {})
    }

    pub fn remove_child(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<UpdateStats, UpdateError<A::Error>> {
        self.prepare_update()?;
        self.mutations.clear();
        let index = self
            .graph
            .relation(parent, relation)
            .and_then(|relation| match &relation.value {
                RetainedRelationValue::Many(children) => {
                    children.iter().position(|current| *current == child)
                }
                RetainedRelationValue::One(_) => None,
            })
            .ok_or(UpdateError::Graph(GraphError::MissingChild(
                relation, child,
            )))?;
        let poison_references = self.graph.references();
        let mut transaction = GraphTransaction::new(&mut self.graph);
        let mut references = Vec::new();
        let plan = (|| {
            let mut planner = Planner {
                retained: &mut transaction,
                mutations: &mut self.mutations,
                references: &mut references,
                virtual_refreshes: None,
            };
            if !planner.retire_with_transition(parent, relation, child)? {
                planner.mutations.push(Mutation::Remove {
                    parent,
                    relation,
                    child,
                    index,
                });
                planner.retire(child);
            }
            Ok::<(), GraphError>(())
        })();
        let plan = plan.and_then(|()| transaction.window_title_bar().map(|_| ()));
        if let Err(error) = plan {
            transaction.rollback();
            self.mutations.clear();
            return Err(UpdateError::Graph(error));
        }
        let RetainedRelationValue::Many(children) =
            &mut transaction.relation_mut(parent, relation).value
        else {
            unreachable!()
        };
        children.remove(index);
        defer_window_title_bar_sets(&mut self.mutations);
        self.poisoned = true;
        if let Err(error) = validate_adapter(&self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        if let Err(error) = apply_adapter(&mut self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        transaction.commit();
        self.poisoned = false;
        apply_reference_changes(&self.references, references);
        Ok(self.finish_update())
    }
}
