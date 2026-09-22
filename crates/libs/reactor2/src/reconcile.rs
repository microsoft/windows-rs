use super::*;
use crate::ir::{DeclarationValidator, validate_property};
use std::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(any(test, feature = "test"))]
use std::mem::size_of;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectId {
    index: u32,
    generation: u32,
}

impl ObjectId {
    pub const fn index(self) -> u32 {
        self.index
    }

    pub const fn generation(self) -> u32 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    pub child: ObjectId,
    pub before: Option<ObjectId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mutation {
    Create {
        object: ObjectId,
        kind: ObjectType,
    },
    Replace {
        object: ObjectId,
        kind: ObjectType,
    },
    SetProperties {
        object: ObjectId,
        set: Rc<[Property]>,
        clear: Rc<[PropertyId]>,
    },
    SetEvents {
        object: ObjectId,
        set: Rc<[Event]>,
        clear: Rc<[EventId]>,
    },
    SetVirtualSource {
        object: ObjectId,
        item_count: usize,
        source_revision: u64,
    },
    Realize {
        parent: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
        index: usize,
        child: ObjectId,
    },
    Recycle {
        parent: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
        child: Option<ObjectId>,
    },
    Attach {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    },
    Detach {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    },
    Insert {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    },
    Remove {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    },
    Reorder {
        parent: ObjectId,
        relation: RelationId,
        moves: Vec<Move>,
        children: Vec<ObjectId>,
    },
    Retire {
        root: ObjectId,
        nodes: Vec<ObjectId>,
        parent: ObjectId,
        relation: RelationId,
        duration: Duration,
    },
    CompleteRetirement {
        root: ObjectId,
        nodes: Vec<ObjectId>,
    },
    Destroy {
        object: ObjectId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetirementCompletion {
    pub root: ObjectId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RealizedContainer(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationLease {
    pub collection: ObjectId,
    pub container: RealizedContainer,
    pub key: Key,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealizationRequest {
    Realize {
        collection: ObjectId,
        container: RealizedContainer,
        index: usize,
        source_revision: u64,
    },
    Recycle {
        collection: ObjectId,
        container: RealizedContainer,
        source_revision: u64,
    },
    Cancel {
        collection: ObjectId,
        container: RealizedContainer,
        source_revision: u64,
    },
}

#[derive(Clone)]
pub(crate) enum VirtualWork {
    Realize {
        lease: RealizationLease,
        index: usize,
        view: Visual,
        owner: Option<ComponentId>,
    },
    Recycle {
        lease: RealizationLease,
    },
    Cancel {
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    },
}

pub(crate) enum NativeWork {
    Event(NativeEventDispatch),
    Virtual(VirtualWork),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Observation {
    SetProperty {
        object: ObjectId,
        property: Property,
    },
    SetSelection {
        object: ObjectId,
        selected: Option<ObjectId>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FeedbackExpectation {
    Exact(Property),
    DeferredExact(Property),
    Normalized { observation: Option<Observation> },
    Suppressed,
}

#[derive(Default)]
pub(crate) struct FeedbackState {
    expectations: HashMap<(ObjectId, EventId), FeedbackExpectation>,
}

impl FeedbackState {
    pub(crate) fn begin(
        &mut self,
        object: ObjectId,
        event: EventId,
        expectation: FeedbackExpectation,
    ) {
        self.expectations.insert((object, event), expectation);
    }

    pub(crate) fn observe(
        &mut self,
        object: ObjectId,
        event: EventId,
        observation: Observation,
    ) -> bool {
        let key = (object, event);
        if let Some(FeedbackExpectation::DeferredExact(expected)) = self.expectations.get(&key) {
            let suppress = matches!(
                &observation,
                Observation::SetProperty { property, .. } if property == expected
            );
            if !suppress {
                self.expectations.remove(&key);
            }
            return !suppress;
        }
        let Some(expectation) = self.expectations.get_mut(&key) else {
            return true;
        };
        match expectation {
            FeedbackExpectation::Exact(expected)
                if matches!(
                    &observation,
                    Observation::SetProperty { property, .. } if property == expected
                ) =>
            {
                false
            }
            FeedbackExpectation::Normalized {
                observation: pending,
            } => {
                *pending = Some(observation);
                false
            }
            FeedbackExpectation::Suppressed => false,
            FeedbackExpectation::DeferredExact(_) => unreachable!(),
            FeedbackExpectation::Exact(_) => true,
        }
    }

    pub(crate) fn finish(&mut self, object: ObjectId, event: EventId) -> Option<Observation> {
        match self.expectations.remove(&(object, event)) {
            Some(FeedbackExpectation::Normalized { observation }) => observation,
            Some(FeedbackExpectation::DeferredExact(property)) => {
                self.expectations.insert(
                    (object, event),
                    FeedbackExpectation::DeferredExact(property),
                );
                None
            }
            Some(FeedbackExpectation::Exact(_) | FeedbackExpectation::Suppressed) | None => None,
        }
    }

    pub(crate) fn remove_object(&mut self, object: ObjectId) {
        self.expectations
            .retain(|(expected_object, _), _| *expected_object != object);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventDispatch {
    pub object: ObjectId,
    pub event: EventId,
    callback: EventValue,
    payload: EventPayload,
}

impl EventDispatch {
    pub fn new(
        object: ObjectId,
        event: EventId,
        callback: EventValue,
        payload: EventPayload,
    ) -> Self {
        Self {
            object,
            event,
            callback,
            payload,
        }
    }

    pub fn invoke(self) {
        match (self.callback, self.payload) {
            (EventValue::Bool(callback), EventPayload::Bool(value)) => callback.call(value),
            (EventValue::String(callback), EventPayload::String(value)) => callback.call(value),
            (EventValue::F64(callback), EventPayload::F64(value)) => callback.call(value),
            (EventValue::OptionalBool(callback), EventPayload::OptionalBool(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalF64(callback), EventPayload::OptionalF64(value)) => {
                callback.call(value);
            }
            (EventValue::Selection(callback), EventPayload::Selection(value)) => {
                callback.call(value.value);
            }
            (EventValue::PointerEventInfo(callback), EventPayload::PointerEventInfo(value)) => {
                callback.call(value);
            }
            (EventValue::SelectionIndex(callback), EventPayload::SelectionIndex(value)) => {
                callback.call(value);
            }
            (EventValue::Unit(callback), EventPayload::Unit) => callback.call(()),
            _ => unreachable!(),
        }
    }

    pub(crate) fn object(&self) -> ObjectId {
        self.object
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeEvent {
    observation: Option<Observation>,
    event: Option<EventDispatch>,
    retirement: Option<RetirementCompletion>,
    realization: Option<RealizationRequest>,
}

impl NativeEvent {
    pub fn new(observation: Option<Observation>, event: Option<EventDispatch>) -> Self {
        Self {
            observation,
            event,
            retirement: None,
            realization: None,
        }
    }

    pub fn observation(observation: Observation) -> Self {
        Self::new(Some(observation), None)
    }

    pub fn event(event: EventDispatch) -> Self {
        Self::new(None, Some(event))
    }

    pub fn retirement(completion: RetirementCompletion) -> Self {
        Self {
            observation: None,
            event: None,
            retirement: Some(completion),
            realization: None,
        }
    }

    pub fn realization(request: RealizationRequest) -> Self {
        Self {
            observation: None,
            event: None,
            retirement: None,
            realization: Some(request),
        }
    }

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

pub struct NativeEventDispatch {
    event: Option<EventDispatch>,
    active: Rc<Cell<bool>>,
}

struct CallbackUnwindGuard {
    active: Rc<Cell<bool>>,
    completed: bool,
}

impl Drop for CallbackUnwindGuard {
    fn drop(&mut self) {
        if !self.completed {
            self.active.set(false);
        }
    }
}

impl NativeEventDispatch {
    fn new(event: EventDispatch, active: Rc<Cell<bool>>) -> Self {
        Self {
            event: Some(event),
            active,
        }
    }

    pub fn invoke(&mut self) {
        if let Some(event) = self.event.take() {
            let mut guard = CallbackUnwindGuard {
                active: Rc::clone(&self.active),
                completed: false,
            };
            event.invoke();
            guard.completed = true;
        }
    }
}

impl Drop for NativeEventDispatch {
    fn drop(&mut self) {
        self.active.set(false);
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RetainedRelationValue {
    One(Option<ObjectId>),
    Many(Vec<ObjectId>),
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRelation {
    id: RelationId,
    value: RetainedRelationValue,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedObject {
    kind: ObjectType,
    key: Option<Key>,
    reference: Option<ElementRef>,
    exit_transition: Option<ExitTransition>,
    properties: SharedList<Property>,
    events: Option<Rc<Vec<Event>>>,
    relations: Vec<RetainedRelation>,
    virtual_items: Option<Box<RetainedVirtualItems>>,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedVirtualItems {
    relation: RelationId,
    owner: Option<ComponentId>,
    items: VirtualItems,
    keys: Rc<Vec<Key>>,
    source_revision: u64,
    lease_revision: u64,
    active: HashMap<Key, (u64, RealizedContainer)>,
    containers: HashMap<RealizedContainer, RetainedRealization>,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRealization {
    key: Key,
    revision: u64,
    index: usize,
    child: Option<ObjectId>,
}

fn virtual_relation(kind: ObjectType) -> Option<RelationId> {
    relation_contracts(kind)
        .iter()
        .find(|contract| {
            contract.realization == Realization::Container
                && contract.child == ObjectCategory::Visual
        })
        .map(|contract| contract.id)
}

fn virtual_keys(items: &VirtualItems) -> Result<Vec<Key>, GraphError> {
    let mut keys = Vec::with_capacity(items.len());
    let mut unique = HashSet::with_capacity(items.len());
    for index in 0..items.len() {
        let Some(key) = items.key(index) else {
            return Err(GraphError::SizeExceeded);
        };
        if !unique.insert(key.clone()) {
            return Err(GraphError::DuplicateKey(key));
        }
        keys.push(key);
    }
    Ok(keys)
}

fn retain_virtual_items(
    declaration: &Declaration,
) -> Result<Option<Box<RetainedVirtualItems>>, GraphError> {
    let Some(relation) = virtual_relation(declaration.kind) else {
        return Ok(None);
    };
    let declared = declaration
        .virtual_items
        .as_deref()
        .cloned()
        .unwrap_or(DeclaredVirtualItems {
            relation,
            owner: declaration.component,
            items: VirtualItems::Eager(Rc::default()),
        });
    if declared.relation != relation {
        return Err(GraphError::InvalidRelation(
            declaration.kind,
            declared.relation,
        ));
    }
    let keys = virtual_keys(&declared.items)?;
    Ok(Some(Box::new(RetainedVirtualItems {
        relation,
        owner: declared.owner.or(declaration.component),
        items: declared.items,
        keys: Rc::new(keys),
        source_revision: 0,
        lease_revision: 0,
        active: HashMap::new(),
        containers: HashMap::new(),
    })))
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
}

fn retained_events(events: &Option<Rc<Vec<Event>>>) -> &[Event] {
    events.as_deref().map_or(&[], Vec::as_slice)
}

fn retain_events(events: &SharedList<Event>) -> Option<Rc<Vec<Event>>> {
    (!events.as_slice().is_empty()).then(|| Rc::new(events.as_slice().to_vec()))
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RetainedGraph {
    root: Option<ObjectId>,
    objects: Vec<RetainedSlot>,
    free: Vec<u32>,
    retirements: HashMap<ObjectId, RetainedRetirement>,
    #[cfg(test)]
    full_reference_scans: ReferenceScanCounter,
}

#[cfg(any(test, feature = "test"))]
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
#[derive(Clone, Debug, Default)]
struct ReferenceScanCounter(Cell<usize>);

#[cfg(test)]
impl PartialEq for ReferenceScanCounter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RetainedSlot {
    generation: u32,
    object: Option<RetainedObject>,
    retiring: bool,
}

impl RetainedGraph {
    #[cfg(any(test, feature = "test"))]
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

    fn references(&self) -> Vec<(ElementRef, ObjectId)> {
        #[cfg(test)]
        self.full_reference_scans
            .0
            .set(self.full_reference_scans.0.get() + 1);
        self.objects
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                let index = u32::try_from(index).ok()?;
                slot.object.as_ref()?.reference.as_ref().map(|reference| {
                    (
                        reference.clone(),
                        ObjectId {
                            index,
                            generation: slot.generation,
                        },
                    )
                })
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn full_reference_scan_count(&self) -> usize {
        self.full_reference_scans.0.get()
    }

    fn clear_references(&mut self) {
        for (index, slot) in self.objects.iter_mut().enumerate() {
            let Some(object) = slot.object.as_mut() else {
                continue;
            };
            if let Some(reference) = object.reference.take()
                && let Ok(index) = u32::try_from(index)
            {
                reference.clear(ObjectId {
                    index,
                    generation: slot.generation,
                });
            }
        }
    }

    pub fn root(&self) -> Option<ObjectId> {
        self.root
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

    pub fn kind(&self, object: ObjectId) -> Option<ObjectType> {
        self.get(object).map(|object| object.kind)
    }

    pub fn children(&self, object: ObjectId, relation: RelationId) -> Option<&[ObjectId]> {
        match &self.relation(object, relation)?.value {
            RetainedRelationValue::Many(children) => Some(children),
            _ => None,
        }
    }

    pub fn child(&self, object: ObjectId, relation: RelationId) -> Option<ObjectId> {
        match &self.relation(object, relation)?.value {
            RetainedRelationValue::One(child) => *child,
            _ => None,
        }
    }

    pub(crate) fn owner(&self, child: ObjectId) -> Option<(ObjectId, RelationId)> {
        self.objects.iter().enumerate().find_map(|(index, slot)| {
            let object = (!slot.retiring).then_some(slot.object.as_ref()).flatten()?;
            object.relations.iter().find_map(|relation| {
                let contains = match &relation.value {
                    RetainedRelationValue::One(current) => *current == Some(child),
                    RetainedRelationValue::Many(children) => children.contains(&child),
                };
                if contains {
                    Some((
                        ObjectId {
                            index: index.try_into().ok()?,
                            generation: slot.generation,
                        },
                        relation.id,
                    ))
                } else {
                    None
                }
            })
        })
    }

    pub fn properties(&self, object: ObjectId) -> Option<&[Property]> {
        self.get(object).map(|object| object.properties.as_slice())
    }

    pub fn events(&self, object: ObjectId) -> Option<&[Event]> {
        self.get(object)
            .map(|object| retained_events(&object.events))
    }

    fn validate_event_dispatch(&self, dispatch: &EventDispatch) -> Result<bool, GraphError> {
        let Some(object) = self.get(dispatch.object) else {
            return Ok(false);
        };
        let contract = event_contracts(object.kind)
            .iter()
            .find(|contract| contract.id == dispatch.event)
            .ok_or(GraphError::InvalidEvent(object.kind, dispatch.event))?;
        if !matches!(
            (contract.value, &dispatch.callback, &dispatch.payload),
            (ValueType::Bool, EventValue::Bool(_), EventPayload::Bool(_))
                | (
                    ValueType::String,
                    EventValue::String(_),
                    EventPayload::String(_)
                )
                | (ValueType::F64, EventValue::F64(_), EventPayload::F64(_))
                | (
                    ValueType::OptionalBool,
                    EventValue::OptionalBool(_),
                    EventPayload::OptionalBool(_)
                )
                | (
                    ValueType::OptionalF64,
                    EventValue::OptionalF64(_),
                    EventPayload::OptionalF64(_)
                )
                | (
                    ValueType::Selection,
                    EventValue::Selection(_),
                    EventPayload::Selection(_)
                )
                | (
                    ValueType::PointerEventInfo,
                    EventValue::PointerEventInfo(_),
                    EventPayload::PointerEventInfo(_)
                )
                | (
                    ValueType::SelectionIndex,
                    EventValue::SelectionIndex(_),
                    EventPayload::SelectionIndex(_)
                )
                | (ValueType::Unit, EventValue::Unit(_), EventPayload::Unit)
        ) {
            return Err(GraphError::InvalidEventValue(dispatch.event));
        }
        if let EventPayload::Selection(selection) = &dispatch.payload
            && let Some(item) = selection.item
        {
            let Some(contract) = selection_contract(object.kind) else {
                return Err(GraphError::InvalidSelection(object.kind));
            };
            if !contract.relations.iter().any(|relation| {
                self.children(dispatch.object, *relation)
                    .is_some_and(|children| children.contains(&item))
            }) {
                return Ok(false);
            }
        }
        Ok(object
            .events
            .as_deref()
            .into_iter()
            .flatten()
            .find(|event| event.id == dispatch.event)
            .is_some_and(|event| event.value == dispatch.callback))
    }

    fn get(&self, object: ObjectId) -> Option<&RetainedObject> {
        let slot = self.objects.get(object.index as usize)?;
        (slot.generation == object.generation && !slot.retiring)
            .then_some(slot.object.as_ref())
            .flatten()
    }

    fn get_mut(&mut self, object: ObjectId) -> &mut RetainedObject {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        assert!(!slot.retiring);
        slot.object.as_mut().unwrap()
    }

    fn relation(&self, object: ObjectId, relation: RelationId) -> Option<&RetainedRelation> {
        self.get(object)?
            .relations
            .iter()
            .find(|state| state.id == relation)
    }

    fn retirement_nodes(&self, root: ObjectId) -> Option<&[ObjectId]> {
        self.retirements
            .get(&root)
            .map(|retirement| retirement.nodes.as_slice())
    }

    fn retirements_for_parent(&self, parent: ObjectId) -> Vec<(ObjectId, Vec<ObjectId>)> {
        self.retirements
            .iter()
            .filter(|(_, retirement)| retirement.parent == parent)
            .map(|(root, retirement)| (*root, retirement.nodes.clone()))
            .collect()
    }

    fn matches_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> Result<bool, GraphError> {
        self.matches_declaration_inner(object, declaration, remaining, true)
    }

    fn matches_subtree_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> Result<bool, GraphError> {
        self.matches_declaration_inner(object, declaration, remaining, false)
    }

    fn matches_declaration_inner(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
        compare_key: bool,
    ) -> Result<bool, GraphError> {
        let Some(next) = remaining.checked_sub(1) else {
            return Err(GraphError::SizeExceeded);
        };
        *remaining = next;
        let Some(current) = self.get(object) else {
            return Ok(false);
        };
        if current.kind != declaration.kind
            || (compare_key && current.key != declaration.key)
            || current.reference != declaration.reference
            || current.exit_transition != declaration.exit_transition
            || current.properties.as_slice() != declaration.properties.as_slice()
            || retained_events(&current.events) != declaration.events.as_slice()
        {
            return Ok(false);
        }
        if let Some(current_virtual) = &current.virtual_items {
            let declared = declaration.virtual_items.as_ref();
            if declared.map(|declared| &declared.items) != Some(&current_virtual.items)
                || declared
                    .and_then(|declared| declared.owner)
                    .or(declaration.component)
                    != current_virtual.owner
            {
                return Ok(false);
            }
        } else if declaration.virtual_items.is_some() {
            return Ok(false);
        }
        let contracts = relation_contracts(current.kind);
        for (index, relation) in declaration.relations.iter().enumerate() {
            if !contracts.iter().any(|contract| contract.id == relation.id)
                || declaration.relations.as_slice()[..index]
                    .iter()
                    .any(|previous| previous.id == relation.id)
            {
                return Ok(false);
            }
        }
        for contract in contracts {
            if contract.realization == Realization::Container && current.virtual_items.is_some() {
                continue;
            }
            let declared = declaration
                .relations
                .iter()
                .find(|relation| relation.id == contract.id);
            let retained = current
                .relations
                .iter()
                .find(|relation| relation.id == contract.id)
                .unwrap();
            match (&retained.value, declared.map(|relation| &relation.value)) {
                (RetainedRelationValue::One(None), None) => {}
                (RetainedRelationValue::Many(children), None) if children.is_empty() => {}
                (RetainedRelationValue::One(previous), Some(RelationValue::One(next))) => {
                    match (previous, next) {
                        (None, None) => {}
                        (Some(previous), Some(next))
                            if self.matches_declaration_inner(
                                *previous,
                                next.as_object()?,
                                remaining,
                                true,
                            )? => {}
                        _ => return Ok(false),
                    }
                }
                (RetainedRelationValue::Many(previous), Some(RelationValue::Many(next))) => {
                    if previous.len() != next.len() {
                        return Ok(false);
                    }
                    for (previous, next) in previous.iter().zip(next.iter()) {
                        if !self.matches_declaration_inner(
                            *previous,
                            next.as_object()?,
                            remaining,
                            true,
                        )? {
                            return Ok(false);
                        }
                    }
                }
                _ => return Ok(false),
            }
        }
        Ok(true)
    }

    fn validate_realization_request(
        &self,
        request: RealizationRequest,
    ) -> Result<bool, GraphError> {
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
            return Ok(false);
        };
        let Some(items) = &object.virtual_items else {
            return Err(GraphError::InvalidRealization(
                collection,
                index.unwrap_or_default(),
            ));
        };
        if items.source_revision != source_revision {
            return Ok(false);
        }
        if let Some(index) = index
            && index >= items.keys.len()
        {
            return Err(GraphError::InvalidRealization(collection, index));
        }
        Ok(true)
    }

    fn virtual_work(&self, request: RealizationRequest) -> Result<Option<VirtualWork>, GraphError> {
        if !self.validate_realization_request(request)? {
            return Ok(None);
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
                    None => items
                        .lease_revision
                        .checked_add(1)
                        .ok_or(GraphError::SizeExceeded)?,
                };
                Ok(Some(VirtualWork::Realize {
                    lease: RealizationLease {
                        collection,
                        container,
                        key,
                        revision,
                    },
                    index,
                    view: items.items.view(index).ok_or(GraphError::SizeExceeded)?,
                    owner: items.owner,
                }))
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
                Ok(items
                    .containers
                    .get(&container)
                    .map(|realization| VirtualWork::Recycle {
                        lease: RealizationLease {
                            collection,
                            container,
                            key: realization.key.clone(),
                            revision: realization.revision,
                        },
                    }))
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
                Ok(Some(VirtualWork::Cancel {
                    collection,
                    relation,
                    container,
                }))
            }
        }
    }
}

pub trait Adapter {
    type Error;

    fn preview_native_events(&self, _events: &mut Vec<NativeEvent>) {}
    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        None
    }
    fn take_error(&mut self) -> Option<Self::Error> {
        None
    }
    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error>;
    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error>;
    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error>;
    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        request.complete_unavailable();
        Ok(())
    }
}

enum GraphUndo {
    Root(Option<ObjectId>),
    Slot {
        index: usize,
        previous: RetainedSlot,
    },
    ObjectPushed,
    FreePopped(u32),
    FreePushed(u32),
    RetirementInserted(ObjectId),
    RetirementRemoved {
        root: ObjectId,
        retirement: RetainedRetirement,
    },
}

struct GraphTransaction<'a> {
    graph: &'a mut RetainedGraph,
    undo: Vec<GraphUndo>,
    snapshotted_slots: HashSet<usize>,
    initial_object_len: usize,
    root_snapshotted: bool,
    armed: bool,
}

impl<'a> GraphTransaction<'a> {
    fn new(graph: &'a mut RetainedGraph) -> Self {
        Self {
            initial_object_len: graph.objects.len(),
            graph,
            undo: Vec::new(),
            snapshotted_slots: HashSet::new(),
            root_snapshotted: false,
            armed: true,
        }
    }

    fn snapshot_slot(&mut self, index: usize) {
        if index < self.initial_object_len && self.snapshotted_slots.insert(index) {
            self.undo.push(GraphUndo::Slot {
                index,
                previous: self.graph.objects[index].clone(),
            });
        }
    }

    fn get_mut(&mut self, object: ObjectId) -> &mut RetainedObject {
        self.snapshot_slot(object.index as usize);
        self.graph.get_mut(object)
    }

    fn relation_mut(&mut self, object: ObjectId, relation: RelationId) -> &mut RetainedRelation {
        self.get_mut(object)
            .relations
            .iter_mut()
            .find(|state| state.id == relation)
            .unwrap()
    }

    fn set_root(&mut self, root: Option<ObjectId>) {
        if self.graph.root == root {
            return;
        }
        if !self.root_snapshotted {
            self.undo.push(GraphUndo::Root(self.graph.root));
            self.root_snapshotted = true;
        }
        self.graph.root = root;
    }

    fn allocate(&mut self, object: RetainedObject) -> Result<ObjectId, GraphError> {
        if let Some(index) = self.graph.free.last().copied() {
            self.snapshot_slot(index as usize);
            let popped = self.graph.free.pop().unwrap();
            debug_assert_eq!(popped, index);
            self.undo.push(GraphUndo::FreePopped(index));
            let slot = &mut self.graph.objects[index as usize];
            slot.object = Some(object);
            slot.retiring = false;
            Ok(ObjectId {
                index,
                generation: slot.generation,
            })
        } else {
            let index = self
                .graph
                .objects
                .len()
                .try_into()
                .map_err(|_| GraphError::SizeExceeded)?;
            let id = ObjectId {
                index,
                generation: 0,
            };
            self.undo.push(GraphUndo::ObjectPushed);
            self.graph.objects.push(RetainedSlot {
                generation: 0,
                object: Some(object),
                retiring: false,
            });
            Ok(id)
        }
    }

    fn remove(&mut self, object: ObjectId) {
        self.snapshot_slot(object.index as usize);
        let slot = &mut self.graph.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        slot.object = None;
        slot.retiring = false;
        slot.generation = slot.generation.wrapping_add(1);
        self.undo.push(GraphUndo::FreePushed(object.index));
        self.graph.free.push(object.index);
    }

    fn mark_retiring(&mut self, root: ObjectId, nodes: Vec<ObjectId>, parent: ObjectId) {
        for object in &nodes {
            self.snapshot_slot(object.index as usize);
            let slot = &mut self.graph.objects[object.index as usize];
            assert_eq!(slot.generation, object.generation);
            assert!(!slot.retiring);
            slot.retiring = true;
        }
        self.undo.push(GraphUndo::RetirementInserted(root));
        assert!(
            self.graph
                .retirements
                .insert(root, RetainedRetirement { nodes, parent })
                .is_none()
        );
    }

    fn complete_retirement(&mut self, root: ObjectId) {
        let Some(retirement) = self.graph.retirements.remove(&root) else {
            return;
        };
        self.undo.push(GraphUndo::RetirementRemoved {
            root,
            retirement: retirement.clone(),
        });
        for object in retirement.nodes {
            self.remove(object);
        }
    }

    fn apply_observation(&mut self, observation: Observation) -> Result<(), GraphError> {
        match observation {
            Observation::SetProperty { object, property } => {
                let Some(retained) = self.graph.get(object) else {
                    return Ok(());
                };
                validate_property(retained.kind, &property)?;
                let property_id = property.id;
                self.get_mut(object)
                    .properties
                    .upsert(|current| current.id == property_id, property);
            }
            Observation::SetSelection { object, selected } => {
                let Some(owner) = self.graph.get(object) else {
                    return Ok(());
                };
                let Some(contract) = selection_contract(owner.kind) else {
                    return Err(GraphError::InvalidSelection(owner.kind));
                };
                if selected.is_some_and(|selected| {
                    self.graph.kind(selected) != Some(contract.item)
                        || !contract.relations.iter().any(|relation| {
                            self.graph
                                .children(object, *relation)
                                .is_some_and(|children| children.contains(&selected))
                        })
                }) {
                    return Ok(());
                }
                let children = contract
                    .relations
                    .iter()
                    .flat_map(|relation| {
                        self.graph
                            .children(object, *relation)
                            .unwrap_or_default()
                            .iter()
                            .copied()
                    })
                    .collect::<Vec<_>>();
                for child in children {
                    let child_object = self.get_mut(child);
                    if child_object
                        .properties
                        .iter()
                        .any(|property| property.id == contract.selected_property)
                    {
                        child_object.properties.upsert(
                            |property| property.id == contract.selected_property,
                            Property {
                                id: contract.selected_property,
                                value: PropertyValue::Bool(Some(child) == selected),
                            },
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn rollback_entries(&mut self) {
        for undo in self.undo.drain(..).rev() {
            match undo {
                GraphUndo::Root(previous) => self.graph.root = previous,
                GraphUndo::Slot { index, previous } => self.graph.objects[index] = previous,
                GraphUndo::ObjectPushed => {
                    self.graph.objects.pop().unwrap();
                }
                GraphUndo::FreePopped(index) => self.graph.free.push(index),
                GraphUndo::FreePushed(index) => {
                    assert_eq!(self.graph.free.pop(), Some(index));
                }
                GraphUndo::RetirementInserted(root) => {
                    self.graph.retirements.remove(&root).unwrap();
                }
                GraphUndo::RetirementRemoved { root, retirement } => {
                    assert!(self.graph.retirements.insert(root, retirement).is_none());
                }
            }
        }
    }

    fn rollback(mut self) {
        self.rollback_entries();
        self.armed = false;
    }

    fn commit(mut self) {
        self.armed = false;
    }
}

impl std::ops::Deref for GraphTransaction<'_> {
    type Target = RetainedGraph;

    fn deref(&self) -> &Self::Target {
        self.graph
    }
}

impl Drop for GraphTransaction<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.rollback_entries();
        }
    }
}

#[cfg(test)]
pub(crate) fn panic_during_transaction(graph: &mut RetainedGraph, object: ObjectId) {
    let mut transaction = GraphTransaction::new(graph);
    transaction.get_mut(object).exit_transition = ExitTransition::fade(Duration::from_millis(1));
    panic!("test planning unwind");
}

struct ReferenceUnwindGuard {
    references: Vec<(ElementRef, ObjectId)>,
    armed: bool,
}

impl ReferenceUnwindGuard {
    fn new(references: Vec<(ElementRef, ObjectId)>) -> Self {
        Self {
            references,
            armed: true,
        }
    }

    fn disarm(mut self) {
        self.armed = false;
    }
}

impl Drop for ReferenceUnwindGuard {
    fn drop(&mut self) {
        if self.armed && std::thread::panicking() {
            for (reference, object) in &self.references {
                reference.clear(*object);
            }
        }
    }
}

fn validate_adapter<A: Adapter>(
    adapter: &A,
    mutations: &[Mutation],
    references: &[(ElementRef, ObjectId)],
) -> Result<(), A::Error> {
    let guard = ReferenceUnwindGuard::new(references.to_vec());
    let result = adapter.validate(mutations);
    guard.disarm();
    result
}

fn apply_adapter<A: Adapter>(
    adapter: &mut A,
    mutations: &[Mutation],
    references: &[(ElementRef, ObjectId)],
) -> Result<(), A::Error> {
    let guard = ReferenceUnwindGuard::new(references.to_vec());
    let result = adapter.apply(mutations);
    guard.disarm();
    result
}

/// Owns retained state and the adapter protocol.
///
/// Mutable adapter access is intentionally unavailable so callers cannot bypass reconciliation.
///
/// ```compile_fail
/// use windows_reactor2::{RecordingAdapter, Runtime};
///
/// let mut runtime = Runtime::new(RecordingAdapter::new());
/// let _: &mut RecordingAdapter = runtime.adapter_mut();
/// ```
pub struct Runtime<A> {
    graph: RetainedGraph,
    adapter: A,
    mutations: Vec<Mutation>,
    native_events: Vec<NativeEvent>,
    virtual_refreshes: VecDeque<RealizationRequest>,
    validator: DeclarationValidator,
    native_event_active: Rc<Cell<bool>>,
    references: ReferenceEndpoint,
    poisoned: bool,
}

impl<A> Drop for Runtime<A> {
    fn drop(&mut self) {
        self.graph.clear_references();
        self.references.clear();
    }
}

impl<A: Adapter> Runtime<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            graph: RetainedGraph::default(),
            adapter,
            mutations: Vec::new(),
            native_events: Vec::new(),
            virtual_refreshes: VecDeque::new(),
            validator: DeclarationValidator::default(),
            native_event_active: Rc::new(Cell::new(false)),
            references: ReferenceEndpoint::new(),
            poisoned: false,
        }
    }

    pub fn graph(&self) -> &RetainedGraph {
        &self.graph
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    #[cfg(feature = "test")]
    pub fn release_test_scratch(&mut self) {
        self.mutations = Vec::new();
        self.native_events = Vec::new();
        self.virtual_refreshes = VecDeque::new();
        self.validator = DeclarationValidator::default();
    }

    pub(crate) fn adapter_mut(&mut self) -> &mut A {
        &mut self.adapter
    }

    fn poison_and_discard(&mut self, references: &[(ElementRef, ObjectId)]) {
        self.poisoned = true;
        for (reference, object) in references {
            reference.clear(*object);
        }
        self.graph.clear_references();
        self.graph = RetainedGraph::default();
        self.mutations.clear();
        self.native_events.clear();
        self.virtual_refreshes.clear();
        self.references.clear();
    }

    pub(crate) fn poison_and_discard_all(&mut self) {
        let references = self.graph.references();
        self.poison_and_discard(&references);
    }

    pub fn focus(&mut self, reference: &ElementRef) -> Result<bool, UpdateError<A::Error>> {
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

    pub(crate) fn set_imperative_waker(&self, waker: impl Fn() + 'static) {
        self.references.set_waker(waker);
    }

    fn dispatch_imperative(&mut self) -> Result<bool, UpdateError<A::Error>> {
        let Some(queued) = self.references.pop() else {
            return Ok(false);
        };
        let revoke = matches!(queued.request, ImperativeRequest::RevokeObservation { .. });
        let available = self.graph.kind(queued.object).is_some() && (revoke || queued.is_current());
        if !available {
            queued.request.complete_unavailable();
            return Ok(true);
        }
        if let Err(error) = self.adapter.imperative(queued.request) {
            self.poison_and_discard_all();
            return Err(UpdateError::Adapter(error));
        }
        Ok(true)
    }

    fn preview_native_events(&mut self) -> Result<(), UpdateError<A::Error>> {
        if let Some(error) = self.adapter.take_error() {
            self.poison_and_discard_all();
            return Err(UpdateError::Adapter(error));
        }
        self.native_events.clear();
        self.adapter.preview_native_events(&mut self.native_events);
        if self.native_events.is_empty() {
            return Ok(());
        }
        let mut transaction = GraphTransaction::new(&mut self.graph);
        for event in &self.native_events {
            if let Some(observation) = &event.observation
                && let Err(error) = transaction.apply_observation(observation.clone())
            {
                transaction.rollback();
                self.poisoned = true;
                return Err(UpdateError::InvalidNativeEvent(error));
            }
            if let Some(completion) = event.retirement {
                transaction.complete_retirement(completion.root);
            }
            if let Some(realization) = event.realization
                && let Err(error) = transaction.validate_realization_request(realization)
            {
                transaction.rollback();
                self.poisoned = true;
                return Err(UpdateError::InvalidNativeEvent(error));
            }
        }
        transaction.rollback();
        Ok(())
    }

    fn pop_native_event(&mut self) -> Result<Option<EventDispatch>, UpdateError<A::Error>> {
        let Some(event) = self.adapter.pop_native_event() else {
            return Ok(None);
        };
        if let Some(observation) = event.observation {
            let mut transaction = GraphTransaction::new(&mut self.graph);
            if let Err(error) = transaction.apply_observation(observation) {
                transaction.rollback();
                self.poisoned = true;
                return Err(UpdateError::InvalidNativeEvent(error));
            }
            transaction.commit();
        }
        if let Some(completion) = event.retirement {
            self.apply_retirement_completion(completion)?;
        }
        if event.realization.is_some() {
            return Err(UpdateError::PendingNativeEvent);
        }
        let Some(dispatch) = event.event else {
            return Ok(None);
        };
        match self.graph.validate_event_dispatch(&dispatch) {
            Ok(true) => Ok(Some(dispatch)),
            Ok(false) => Ok(None),
            Err(error) => {
                self.poisoned = true;
                Err(UpdateError::InvalidNativeEvent(error))
            }
        }
    }

    fn pop_native_work(&mut self) -> Result<Option<NativeWork>, UpdateError<A::Error>> {
        let Some(event) = self.adapter.pop_native_event() else {
            return Ok(None);
        };
        if let Some(observation) = event.observation {
            let mut transaction = GraphTransaction::new(&mut self.graph);
            if let Err(error) = transaction.apply_observation(observation) {
                transaction.rollback();
                self.poisoned = true;
                return Err(UpdateError::InvalidNativeEvent(error));
            }
            transaction.commit();
        }
        if let Some(completion) = event.retirement {
            self.apply_retirement_completion(completion)?;
        }
        if let Some(request) = event.realization {
            if let RealizationRequest::Recycle {
                collection,
                container,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                container,
                ..
            } = request
            {
                self.virtual_refreshes.retain(|refresh| {
                    !matches!(
                        refresh,
                        RealizationRequest::Realize {
                            collection: refresh_collection,
                            container: refresh_container,
                            ..
                        } if *refresh_collection == collection && *refresh_container == container
                    )
                });
            }
            return self
                .graph
                .virtual_work(request)
                .map(|work| work.map(NativeWork::Virtual))
                .map_err(|error| {
                    self.poisoned = true;
                    UpdateError::InvalidNativeEvent(error)
                });
        }
        let Some(dispatch) = event.event else {
            return Ok(None);
        };
        match self.graph.validate_event_dispatch(&dispatch) {
            Ok(true) => {
                self.native_event_active.set(true);
                Ok(Some(NativeWork::Event(NativeEventDispatch::new(
                    dispatch,
                    Rc::clone(&self.native_event_active),
                ))))
            }
            Ok(false) => Ok(None),
            Err(error) => {
                self.poisoned = true;
                Err(UpdateError::InvalidNativeEvent(error))
            }
        }
    }

    pub(crate) fn next_native_work(&mut self) -> Result<Option<NativeWork>, UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        if self.native_event_active.get() {
            return Err(UpdateError::NativeEventInProgress);
        }
        loop {
            self.preview_native_events()?;
            if !self.native_events.is_empty() {
                if let Some(work) = self.pop_native_work()? {
                    return Ok(Some(work));
                }
                continue;
            }
            if let Some(request) = self.virtual_refreshes.pop_front()
                && let Some(work) = self
                    .graph
                    .virtual_work(request)
                    .map_err(UpdateError::Graph)?
            {
                return Ok(Some(NativeWork::Virtual(work)));
            }
            if self.virtual_refreshes.is_empty() {
                for _ in 0..64 {
                    if !self.dispatch_imperative()? {
                        return Ok(None);
                    }
                }
                self.references.wake();
                return Ok(None);
            }
        }
    }

    pub(crate) fn prepare_update(&mut self) -> Result<(), UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        if self.native_event_active.get() {
            return Ok(());
        }
        if !self.virtual_refreshes.is_empty() {
            return Err(UpdateError::PendingNativeEvent);
        }
        loop {
            self.preview_native_events()?;
            let Some(next) = self.native_events.first() else {
                return Ok(());
            };
            if next.event.is_some() || next.realization.is_some() {
                return Err(UpdateError::PendingNativeEvent);
            }
            self.pop_native_event()?;
        }
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
                    self.realize_virtual(&lease, index, view)?;
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
            }
        }
        Ok(dispatched)
    }

    pub(crate) fn realize_virtual(
        &mut self,
        lease: &RealizationLease,
        index: usize,
        root: impl Into<Visual>,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.mutations.clear();
        let poison_references = self.graph.references();
        let mut declaration = root.into().0.object().map_err(UpdateError::Graph)?;
        declaration.key = Some(lease.key.clone());
        let _ = self
            .validator
            .validate(&declaration)
            .map_err(UpdateError::Graph)?;
        let mut transaction = GraphTransaction::new(&mut self.graph);
        let mut references = Vec::new();
        let plan = (|| {
            let snapshot = transaction
                .get(lease.collection)
                .and_then(|object| object.virtual_items.as_ref())
                .cloned()
                .ok_or(GraphError::StaleObject(lease.collection))?;
            if snapshot.keys.get(index) != Some(&lease.key) {
                return Err(GraphError::InvalidRealization(lease.collection, index));
            }
            let mut child = snapshot
                .active
                .get(&lease.key)
                .and_then(|(_, container)| snapshot.containers.get(container))
                .and_then(|active| active.child);
            if let Some(current) = snapshot.containers.get(&lease.container)
                && (current.key != lease.key || current.revision != lease.revision)
            {
                self.mutations.push(Mutation::Recycle {
                    parent: lease.collection,
                    relation: snapshot.relation,
                    container: lease.container,
                    child: current.child,
                });
                if snapshot
                    .active
                    .get(&current.key)
                    .is_some_and(|(_, container)| *container == lease.container)
                    && let Some(previous) = current.child
                {
                    let relation = transaction.relation_mut(lease.collection, snapshot.relation);
                    let RetainedRelationValue::Many(children) = &mut relation.value else {
                        unreachable!();
                    };
                    children.retain(|current| *current != previous);
                    let mut planner = Planner {
                        retained: &mut transaction,
                        mutations: &mut self.mutations,
                        references: &mut references,
                        virtual_refreshes: None,
                    };
                    planner.retire(previous);
                }
            }
            if let Some((revision, container)) = snapshot.active.get(&lease.key) {
                if *revision != lease.revision {
                    return Err(GraphError::InvalidRealization(lease.collection, index));
                }
                if *container != lease.container {
                    self.mutations.push(Mutation::Recycle {
                        parent: lease.collection,
                        relation: snapshot.relation,
                        container: *container,
                        child: snapshot
                            .containers
                            .get(container)
                            .and_then(|realization| realization.child),
                    });
                }
            } else if snapshot.lease_revision.checked_add(1) != Some(lease.revision) {
                return Err(GraphError::InvalidRealization(lease.collection, index));
            }
            if let Some(current) = child {
                if transaction.get(current).unwrap().kind == declaration.kind {
                    let mut planner = Planner {
                        retained: &mut transaction,
                        mutations: &mut self.mutations,
                        references: &mut references,
                        virtual_refreshes: None,
                    };
                    planner.reconcile_object(current, &declaration)?;
                } else {
                    let relation = transaction.relation_mut(lease.collection, snapshot.relation);
                    let RetainedRelationValue::Many(children) = &mut relation.value else {
                        unreachable!();
                    };
                    children.retain(|candidate| *candidate != current);
                    let mut planner = Planner {
                        retained: &mut transaction,
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
                    retained: &mut transaction,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.mount(&declaration)?
            };
            let items = transaction
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
                    .filter_map(|realization| {
                        realization.child.map(|child| (realization.index, child))
                    })
                    .collect::<Vec<_>>();
                active.sort_unstable_by_key(|(index, _)| *index);
                active.into_iter().map(|(_, child)| child).collect()
            };
            let relation = transaction.relation_mut(lease.collection, snapshot.relation);
            let RetainedRelationValue::Many(children) = &mut relation.value else {
                unreachable!();
            };
            *children = ordered;
            self.mutations.push(Mutation::Realize {
                parent: lease.collection,
                relation: snapshot.relation,
                container: lease.container,
                index,
                child,
            });
            Ok::<(), GraphError>(())
        })();
        if let Err(error) = plan {
            transaction.rollback();
            self.mutations.clear();
            return Err(UpdateError::Graph(error));
        }
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
        Ok(self.mutations.clone())
    }

    pub(crate) fn recycle_virtual(
        &mut self,
        lease: &RealizationLease,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.recycle_virtual_before_apply(lease, || {})
    }

    pub(crate) fn recycle_virtual_before_apply(
        &mut self,
        lease: &RealizationLease,
        before_apply: impl FnOnce(),
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.mutations.clear();
        let poison_references = self.graph.references();
        let mut transaction = GraphTransaction::new(&mut self.graph);
        let mut references = Vec::new();
        let plan = (|| {
            let snapshot = transaction
                .get(lease.collection)
                .and_then(|object| object.virtual_items.as_ref())
                .cloned()
                .ok_or(GraphError::StaleObject(lease.collection))?;
            let Some(realization) = snapshot.containers.get(&lease.container) else {
                return Ok(());
            };
            if realization.key != lease.key || realization.revision != lease.revision {
                return Ok(());
            }
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: snapshot.relation,
                container: lease.container,
                child: realization.child,
            });
            let items = transaction
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
            if let Some(child) = realization.child {
                let relation = transaction.relation_mut(lease.collection, snapshot.relation);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|current| *current != child);
                let mut planner = Planner {
                    retained: &mut transaction,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.retire(child);
            }
            Ok::<(), GraphError>(())
        })();
        if let Err(error) = plan {
            transaction.rollback();
            self.mutations.clear();
            return Err(UpdateError::Graph(error));
        }
        self.poisoned = true;
        if let Err(error) = validate_adapter(&self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        before_apply();
        if let Err(error) = apply_adapter(&mut self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        transaction.commit();
        self.poisoned = false;
        apply_reference_changes(&self.references, references);
        Ok(self.mutations.clone())
    }

    pub(crate) fn cancel_virtual(
        &mut self,
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.mutations.clear();
        let poison_references = self.graph.references();
        self.mutations.push(Mutation::Recycle {
            parent: collection,
            relation,
            container,
            child: None,
        });
        self.poisoned = true;
        if let Err(error) = validate_adapter(&self.adapter, &self.mutations, &poison_references) {
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        if let Err(error) = apply_adapter(&mut self.adapter, &self.mutations, &poison_references) {
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        self.poisoned = false;
        Ok(self.mutations.clone())
    }

    pub(crate) fn virtual_lease_active(&self, collection: ObjectId, key: &Key) -> bool {
        self.graph
            .get(collection)
            .and_then(|object| object.virtual_items.as_ref())
            .is_some_and(|items| items.active.contains_key(key))
    }

    pub fn update(
        &mut self,
        root: impl Into<Visual>,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.prepare_update()?;
        self.mutations.clear();
        let declaration = root.into().0.object().map_err(UpdateError::Graph)?;
        if let Some(root) = self.graph.root {
            let mut remaining = MAX_OBJECTS;
            if self
                .graph
                .matches_declaration(root, &declaration, &mut remaining)
                .map_err(UpdateError::Graph)?
            {
                return Ok(Vec::new());
            }
        }
        let object_count = self
            .validator
            .validate(&declaration)
            .map_err(UpdateError::Graph)?;
        if self.graph.objects.is_empty() {
            self.graph.objects.reserve_exact(object_count);
        }
        if let Some(current) = self.graph.root {
            let previous = self.graph.get(current).unwrap().kind;
            let next = declaration.kind;
            if previous != next {
                return Err(UpdateError::Graph(GraphError::RootTypeChanged {
                    previous,
                    next,
                }));
            }
        }
        let poison_references = self.graph.references();
        let mut transaction = GraphTransaction::new(&mut self.graph);
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let plan = (|| {
            let mut planner = Planner {
                retained: &mut transaction,
                mutations: &mut self.mutations,
                references: &mut references,
                virtual_refreshes: Some(&mut virtual_refreshes),
            };
            let root = match planner.retained.root {
                Some(current) => planner.reconcile_object(current, &declaration)?,
                None => planner.mount(&declaration)?,
            };
            planner.retained.set_root(Some(root));
            Ok::<(), GraphError>(())
        })();
        if let Err(error) = plan {
            transaction.rollback();
            self.mutations.clear();
            return Err(UpdateError::Graph(error));
        }
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
        self.virtual_refreshes.extend(virtual_refreshes);
        let mutations = self.mutations.clone();
        if self.mutations.capacity() > 256 {
            self.mutations = Vec::with_capacity(256);
        }
        Ok(mutations)
    }

    pub fn update_subtree(
        &mut self,
        object: ObjectId,
        root: impl Into<Visual>,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.update_subtree_before_apply(object, root, || {})
    }

    pub(crate) fn update_subtree_before_apply(
        &mut self,
        object: ObjectId,
        root: impl Into<Visual>,
        before_apply: impl FnOnce(),
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.update_subtree_inner(object, root.into(), None, before_apply)
    }

    fn update_subtree_inner(
        &mut self,
        object: ObjectId,
        declaration: Visual,
        owner: Option<(ObjectId, RelationId)>,
        before_apply: impl FnOnce(),
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.prepare_update()?;
        self.mutations.clear();
        let declaration = declaration.0.object().map_err(UpdateError::Graph)?;
        let _ = self
            .validator
            .validate(&declaration)
            .map_err(UpdateError::Graph)?;
        let previous = self
            .graph
            .get(object)
            .ok_or(UpdateError::Graph(GraphError::StaleObject(object)))?
            .kind;
        if previous != declaration.kind {
            let owner = owner.or_else(|| self.graph.owner(object));
            let Some((parent, relation)) = owner else {
                return Err(UpdateError::Graph(GraphError::RootTypeChanged {
                    previous,
                    next: declaration.kind,
                }));
            };
            let owns_object =
                self.graph
                    .relation(parent, relation)
                    .is_some_and(|owned| match &owned.value {
                        RetainedRelationValue::One(child) => *child == Some(object),
                        RetainedRelationValue::Many(children) => children.contains(&object),
                    });
            if !owns_object {
                return Err(UpdateError::Graph(GraphError::MissingChild(
                    relation, object,
                )));
            }
            let contract = relation_contracts(self.graph.kind(parent).unwrap())
                .iter()
                .find(|contract| contract.id == relation)
                .unwrap();
            if contract.realization != Realization::Owned
                || !relation_accepts(contract, declaration.kind)
            {
                return Err(UpdateError::Graph(GraphError::InvalidChildCategory(
                    relation,
                )));
            }
        }
        let mut remaining = MAX_OBJECTS;
        if self
            .graph
            .matches_subtree_declaration(object, &declaration, &mut remaining)
            .map_err(UpdateError::Graph)?
        {
            before_apply();
            return Ok(Vec::new());
        }
        let mut transaction = GraphTransaction::new(&mut self.graph);
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let plan = (|| {
            let mut planner = Planner {
                retained: &mut transaction,
                mutations: &mut self.mutations,
                references: &mut references,
                virtual_refreshes: Some(&mut virtual_refreshes),
            };
            if previous == declaration.kind {
                planner.reconcile_object(object, &declaration)?;
            } else {
                planner.replace_object(object, &declaration)?;
            }
            Ok::<(), GraphError>(())
        })();
        if let Err(error) = plan {
            transaction.rollback();
            self.mutations.clear();
            return Err(UpdateError::Graph(error));
        }
        let poison_references = detached_references(&references);
        self.poisoned = true;
        if let Err(error) = validate_adapter(&self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        self.poisoned = false;
        before_apply();
        self.poisoned = true;
        if let Err(error) = apply_adapter(&mut self.adapter, &self.mutations, &poison_references) {
            transaction.commit();
            self.poison_and_discard(&poison_references);
            return Err(UpdateError::Adapter(error));
        }
        transaction.commit();
        self.poisoned = false;
        apply_reference_changes(&self.references, references);
        self.virtual_refreshes.extend(virtual_refreshes);
        let mutations = self.mutations.clone();
        if self.mutations.capacity() > 256 {
            self.mutations = Vec::with_capacity(256);
        }
        Ok(mutations)
    }

    pub fn remove_child(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
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
        let mutations = self.mutations.clone();
        if self.mutations.capacity() > 256 {
            self.mutations = Vec::with_capacity(256);
        }
        Ok(mutations)
    }

    fn apply_retirement_completion(
        &mut self,
        completion: RetirementCompletion,
    ) -> Result<(), UpdateError<A::Error>> {
        self.mutations.clear();
        let Some(nodes) = self.graph.retirement_nodes(completion.root) else {
            return Ok(());
        };
        self.mutations.push(Mutation::CompleteRetirement {
            root: completion.root,
            nodes: nodes.to_vec(),
        });
        let poison_references = self.graph.references();
        let mut transaction = GraphTransaction::new(&mut self.graph);
        transaction.complete_retirement(completion.root);
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
        self.mutations.clear();
        self.poisoned = false;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdateError<E> {
    Graph(GraphError),
    InvalidNativeEvent(GraphError),
    Adapter(E),
    PendingNativeEvent,
    NativeEventInProgress,
    Poisoned,
}

struct Planner<'a, 'graph> {
    retained: &'a mut GraphTransaction<'graph>,
    mutations: &'a mut Vec<Mutation>,
    references: &'a mut Vec<ReferenceChange>,
    virtual_refreshes: Option<&'a mut Vec<RealizationRequest>>,
}

enum ReferenceChange {
    Clear {
        reference: ElementRef,
        object: ObjectId,
    },
    Set {
        reference: ElementRef,
        object: ObjectId,
    },
}

fn detached_references(changes: &[ReferenceChange]) -> Vec<(ElementRef, ObjectId)> {
    changes
        .iter()
        .filter_map(|change| match change {
            ReferenceChange::Clear { reference, object } => Some((reference.clone(), *object)),
            ReferenceChange::Set { .. } => None,
        })
        .collect()
}

fn apply_reference_changes(endpoint: &ReferenceEndpoint, changes: Vec<ReferenceChange>) {
    for change in changes {
        match change {
            ReferenceChange::Clear { reference, object } => reference.clear(object),
            ReferenceChange::Set { reference, object } => reference.bind(endpoint.clone(), object),
        }
    }
}

impl Planner<'_, '_> {
    fn replace_object(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
    ) -> Result<(), GraphError> {
        let previous = std::mem::take(&mut self.retained.get_mut(object).relations);
        for relation in previous {
            match relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.mutations.push(Mutation::Detach {
                        parent: object,
                        relation: relation.id,
                        child,
                    });
                    self.retire(child);
                }
                RetainedRelationValue::Many(children) => {
                    for (index, child) in children.into_iter().enumerate().rev() {
                        self.mutations.push(Mutation::Remove {
                            parent: object,
                            relation: relation.id,
                            child,
                            index,
                        });
                        self.retire(child);
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        let retained = self.retained.get_mut(object);
        let key = retained.key.take();
        if let Some(reference) = retained.reference.take() {
            self.references
                .push(ReferenceChange::Clear { reference, object });
        }
        if let Some(reference) = &declaration.reference {
            self.references.push(ReferenceChange::Set {
                reference: reference.clone(),
                object,
            });
        }
        let virtual_items = retain_virtual_items(declaration)?;
        *self.retained.get_mut(object) = RetainedObject {
            kind: declaration.kind,
            key,
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            properties: declaration.properties.clone(),
            events: retain_events(&declaration.events),
            relations: relation_contracts(declaration.kind)
                .iter()
                .map(|contract| RetainedRelation {
                    id: contract.id,
                    value: match contract.cardinality {
                        Cardinality::One => RetainedRelationValue::One(None),
                        Cardinality::Many => RetainedRelationValue::Many(Vec::new()),
                    },
                })
                .collect(),
            virtual_items,
        };
        self.mutations.push(Mutation::Replace {
            object,
            kind: declaration.kind,
        });
        if !declaration.properties.as_slice().is_empty() {
            self.mutations.push(Mutation::SetProperties {
                object,
                set: Rc::from(declaration.properties.as_slice()),
                clear: Rc::from([]),
            });
        }
        if !declaration.events.as_slice().is_empty() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from(declaration.events.as_slice()),
                clear: Rc::from([]),
            });
        }
        if let Some(items) = &self.retained.get(object).unwrap().virtual_items {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: items.keys.len(),
                source_revision: items.source_revision,
            });
        }
        for contract in relation_contracts(declaration.kind) {
            self.mount_relation(object, declaration, contract)?;
        }
        Ok(())
    }

    fn mount(&mut self, declaration: &Declaration) -> Result<ObjectId, GraphError> {
        let virtual_items = retain_virtual_items(declaration)?;
        let object = self.retained.allocate(RetainedObject {
            kind: declaration.kind,
            key: declaration.key.clone(),
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            properties: declaration.properties.clone(),
            events: retain_events(&declaration.events),
            relations: relation_contracts(declaration.kind)
                .iter()
                .map(|contract| RetainedRelation {
                    id: contract.id,
                    value: match contract.cardinality {
                        Cardinality::One => RetainedRelationValue::One(None),
                        Cardinality::Many => RetainedRelationValue::Many(Vec::new()),
                    },
                })
                .collect(),
            virtual_items,
        })?;
        if let Some(reference) = &declaration.reference {
            self.references.push(ReferenceChange::Set {
                reference: reference.clone(),
                object,
            });
        }
        self.mutations.push(Mutation::Create {
            object,
            kind: declaration.kind,
        });
        if !self
            .retained
            .get(object)
            .unwrap()
            .properties
            .as_slice()
            .is_empty()
        {
            self.mutations.push(Mutation::SetProperties {
                object,
                set: Rc::from(self.retained.get(object).unwrap().properties.as_slice()),
                clear: Rc::from([]),
            });
        }
        if self.retained.get(object).unwrap().events.is_some() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from(retained_events(&self.retained.get(object).unwrap().events)),
                clear: Rc::from([]),
            });
        }
        if let Some(items) = &self.retained.get(object).unwrap().virtual_items {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: items.keys.len(),
                source_revision: items.source_revision,
            });
        }
        for contract in relation_contracts(declaration.kind) {
            self.mount_relation(object, declaration, contract)?;
        }
        Ok(object)
    }

    fn mount_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) -> Result<(), GraphError> {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return Ok(());
        }
        let relation = declaration
            .relations
            .iter()
            .find(|relation| relation.id == contract.id);
        match contract.cardinality {
            Cardinality::One => {
                let child = relation
                    .and_then(|relation| match &relation.value {
                        RelationValue::One(child) => child.as_deref(),
                        RelationValue::Many(_) => unreachable!(),
                    })
                    .map(|child| child.as_object().and_then(|child| self.mount(child)))
                    .transpose()?;
                if let Some(child) = child {
                    self.mutations.push(Mutation::Attach {
                        parent: object,
                        relation: contract.id,
                        child,
                    });
                }
                self.retained.relation_mut(object, contract.id).value =
                    RetainedRelationValue::One(child);
            }
            Cardinality::Many => {
                let children = relation
                    .map(|relation| match &relation.value {
                        RelationValue::Many(children) => children.as_slice(),
                        RelationValue::One(_) => unreachable!(),
                    })
                    .unwrap_or_default();
                let mut retained = Vec::with_capacity(children.len());
                for (index, child) in children.iter().enumerate() {
                    let child = self.mount(child.as_object()?)?;
                    self.mutations.push(Mutation::Insert {
                        parent: object,
                        relation: contract.id,
                        child,
                        index,
                    });
                    retained.push(child);
                }
                self.retained.relation_mut(object, contract.id).value =
                    RetainedRelationValue::Many(retained);
            }
        }
        Ok(())
    }

    fn reconcile_object(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
    ) -> Result<ObjectId, GraphError> {
        debug_assert_eq!(self.retained.get(object).unwrap().kind, declaration.kind);
        if self.retained.get(object).unwrap().reference != declaration.reference {
            if let Some(reference) = self.retained.get_mut(object).reference.take() {
                self.references
                    .push(ReferenceChange::Clear { reference, object });
            }
            if let Some(reference) = &declaration.reference {
                self.references.push(ReferenceChange::Set {
                    reference: reference.clone(),
                    object,
                });
            }
            self.retained
                .get_mut(object)
                .reference
                .clone_from(&declaration.reference);
        }
        if self.retained.get(object).unwrap().exit_transition != declaration.exit_transition {
            self.retained.get_mut(object).exit_transition = declaration.exit_transition;
        }
        let properties = declaration.properties.as_slice();
        if self.retained.get(object).unwrap().properties.as_slice() != properties {
            let previous = self.retained.get(object).unwrap().properties.clone();
            let set = properties
                .iter()
                .filter(|property| {
                    previous.iter().find(|current| current.id == property.id) != Some(*property)
                })
                .cloned()
                .collect::<Rc<[_]>>();
            let clear = previous
                .iter()
                .filter(|property| !properties.iter().any(|current| current.id == property.id))
                .map(|property| property.id)
                .collect::<Rc<[_]>>();
            self.retained.get_mut(object).properties = declaration.properties.clone();
            self.mutations
                .push(Mutation::SetProperties { object, set, clear });
        }
        let events = declaration.events.as_slice();
        if retained_events(&self.retained.get(object).unwrap().events) != events {
            let previous = self.retained.get(object).unwrap().events.clone();
            let set = events
                .iter()
                .filter(|event| {
                    retained_events(&previous)
                        .iter()
                        .find(|current| current.id == event.id)
                        != Some(*event)
                })
                .cloned()
                .collect::<Rc<[_]>>();
            let clear = retained_events(&previous)
                .iter()
                .filter(|event| !events.iter().any(|current| current.id == event.id))
                .map(|event| event.id)
                .collect::<Rc<[_]>>();
            self.retained.get_mut(object).events = retain_events(&declaration.events);
            self.mutations
                .push(Mutation::SetEvents { object, set, clear });
        }
        self.reconcile_virtual_items(object, declaration)?;
        for contract in relation_contracts(declaration.kind) {
            self.reconcile_relation(object, declaration, contract)?;
        }
        Ok(object)
    }

    fn reconcile_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) -> Result<(), GraphError> {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return Ok(());
        }
        let relation = declaration
            .relations
            .iter()
            .find(|relation| relation.id == contract.id);
        match contract.cardinality {
            Cardinality::One => {
                let RetainedRelationValue::One(previous) =
                    self.retained.relation(object, contract.id).unwrap().value
                else {
                    unreachable!()
                };
                let desired = relation.and_then(|relation| match &relation.value {
                    RelationValue::One(child) => child.as_deref(),
                    RelationValue::Many(_) => unreachable!(),
                });
                let desired = desired.map(DeclaredNode::as_object).transpose()?;
                let next = match (previous, desired) {
                    (Some(previous), Some(desired))
                        if self.retained.get(previous).unwrap().kind == desired.kind =>
                    {
                        Some(self.reconcile_object(previous, desired)?)
                    }
                    (previous, desired) => {
                        if let Some(previous) = previous {
                            if self
                                .retained
                                .get(previous)
                                .unwrap()
                                .exit_transition
                                .is_some()
                            {
                                return Err(GraphError::ExitTransitionUnsupported);
                            }
                            self.mutations.push(Mutation::Detach {
                                parent: object,
                                relation: contract.id,
                                child: previous,
                            });
                            self.retire(previous);
                        }
                        let next = desired.map(|desired| self.mount(desired)).transpose()?;
                        if let Some(next) = next {
                            self.mutations.push(Mutation::Attach {
                                parent: object,
                                relation: contract.id,
                                child: next,
                            });
                        }
                        next
                    }
                };
                if previous != next {
                    self.retained.relation_mut(object, contract.id).value =
                        RetainedRelationValue::One(next);
                }
            }
            Cardinality::Many => match contract.identity {
                Identity::Keyed => {
                    self.reconcile_keyed_many(object, contract.id, relation)?;
                }
                Identity::Positional => {
                    self.reconcile_positional_many(object, contract.id, relation)?;
                }
            },
        }
        Ok(())
    }

    fn reconcile_virtual_items(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
    ) -> Result<(), GraphError> {
        let Some(previous) = self.retained.get(object).unwrap().virtual_items.clone() else {
            return Ok(());
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
        if declared.relation != previous.relation {
            return Err(GraphError::InvalidRelation(
                declaration.kind,
                declared.relation,
            ));
        }
        let keys = match (&previous.items, &declared.items) {
            (VirtualItems::Lazy(previous_source), VirtualItems::Lazy(next_source))
                if previous_source.key_revision == next_source.key_revision
                    && previous_source.len == next_source.len =>
            {
                Rc::clone(&previous.keys)
            }
            _ => Rc::new(virtual_keys(&declared.items)?),
        };
        let items_changed = previous.items != declared.items;
        let keys_changed = previous.keys.as_ref() != keys.as_ref();
        let source_revision = if keys_changed {
            previous
                .source_revision
                .checked_add(1)
                .ok_or(GraphError::SizeExceeded)?
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
                lease_revision = lease_revision
                    .checked_add(1)
                    .ok_or(GraphError::SizeExceeded)?;
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
        if items_changed || keys_changed {
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
        Ok(())
    }

    fn reconcile_keyed_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) -> Result<(), GraphError> {
        let desired = relation
            .map(|relation| match &relation.value {
                RelationValue::Many(children) => children.as_slice(),
                RelationValue::One(_) => unreachable!(),
            })
            .unwrap_or_default();
        let same_order = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(previous) if previous.len() == desired.len() => {
                let mut same_order = true;
                for (previous, desired) in previous.iter().zip(desired) {
                    let previous = self.retained.get(*previous).unwrap();
                    let desired = desired.as_object()?;
                    if previous.kind != desired.kind || previous.key != desired.key {
                        same_order = false;
                        break;
                    }
                }
                same_order
            }
            _ => false,
        };
        if same_order {
            for (index, desired) in desired.iter().enumerate() {
                let previous = match &self.retained.relation(object, relation_id).unwrap().value {
                    RetainedRelationValue::Many(previous) => previous[index],
                    _ => unreachable!(),
                };
                self.reconcile_object(previous, desired.as_object()?)?;
            }
            return Ok(());
        }
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Result<Vec<_>, _>>()?;
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut by_key = HashMap::with_capacity(previous.len());
        for (index, child) in previous.iter().enumerate() {
            by_key.insert(
                self.retained.get(*child).unwrap().key.clone().unwrap(),
                (*child, index),
            );
        }

        let mut matched = Vec::with_capacity(desired.len());
        let mut removed = Vec::new();
        for desired in desired.iter().copied() {
            let key = desired.key.clone().unwrap();
            let previous = match by_key.remove(&key) {
                Some((previous, _))
                    if self.retained.get(previous).unwrap().kind == desired.kind =>
                {
                    Some(previous)
                }
                Some(previous) => {
                    removed.push(previous);
                    None
                }
                None => None,
            };
            matched.push((previous, desired));
        }
        removed.extend(by_key.into_values());
        removed.sort_unstable_by_key(|(_, index)| std::cmp::Reverse(*index));
        let mut current = previous.clone();
        for (child, index) in removed {
            if !self.retire_with_transition(object, relation_id, child)? {
                self.mutations.push(Mutation::Remove {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                self.retire(child);
            }
            current.remove(index);
        }

        let mut next = Vec::with_capacity(desired.len());
        for (index, (previous, desired)) in matched.into_iter().enumerate() {
            let child = if let Some(previous) = previous {
                self.reconcile_object(previous, desired)?
            } else {
                let child = self.mount(desired)?;
                self.mutations.push(Mutation::Insert {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                current.insert(index, child);
                child
            };
            next.push(child);
        }
        let moves = reorder_moves(&current, &next);
        if !moves.is_empty() {
            self.mutations.push(Mutation::Reorder {
                parent: object,
                relation: relation_id,
                moves,
                children: next.clone(),
            });
        }
        if previous != next {
            self.retained.relation_mut(object, relation_id).value =
                RetainedRelationValue::Many(next);
        }

        fn reorder_moves(previous: &[ObjectId], next: &[ObjectId]) -> Vec<Move> {
            if previous == next {
                return Vec::new();
            }
            let previous_indices = previous
                .iter()
                .copied()
                .enumerate()
                .map(|(index, child)| (child, index))
                .collect::<HashMap<_, _>>();
            let retained = next
                .iter()
                .enumerate()
                .filter_map(|(next_index, child)| {
                    previous_indices
                        .get(child)
                        .map(|previous_index| (next_index, *previous_index))
                })
                .collect::<Vec<_>>();
            let stable = longest_increasing_positions(&retained);
            let mut moves = Vec::new();

            for next_index in (0..next.len()).rev() {
                if stable.contains(&next_index) {
                    continue;
                }
                let child = next[next_index];
                moves.push(Move {
                    child,
                    before: next.get(next_index + 1).copied(),
                });
            }
            moves
        }

        fn longest_increasing_positions(sequence: &[(usize, usize)]) -> HashSet<usize> {
            let mut tails = Vec::<usize>::new();
            let mut predecessors = vec![None; sequence.len()];

            for (sequence_index, &(_, value)) in sequence.iter().enumerate() {
                let position = tails.partition_point(|tail| sequence[*tail].1 < value);
                if position > 0 {
                    predecessors[sequence_index] = Some(tails[position - 1]);
                }
                if position == tails.len() {
                    tails.push(sequence_index);
                } else {
                    tails[position] = sequence_index;
                }
            }

            let mut positions = HashSet::with_capacity(tails.len());
            let mut cursor = tails.last().copied();
            while let Some(sequence_index) = cursor {
                positions.insert(sequence[sequence_index].0);
                cursor = predecessors[sequence_index];
            }
            positions
        }
        Ok(())
    }

    fn reconcile_positional_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) -> Result<(), GraphError> {
        let desired = relation
            .map(|relation| match &relation.value {
                RelationValue::Many(children) => children.as_slice(),
                RelationValue::One(_) => unreachable!(),
            })
            .unwrap_or_default();
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Result<Vec<_>, _>>()?;
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut next = Vec::with_capacity(desired.len());
        for (index, desired) in desired.iter().copied().enumerate() {
            let child = if let Some(previous) = previous.get(index).copied() {
                if self.retained.get(previous).unwrap().kind == desired.kind {
                    self.reconcile_object(previous, desired)?
                } else {
                    if !self.retire_with_transition(object, relation_id, previous)? {
                        self.mutations.push(Mutation::Remove {
                            parent: object,
                            relation: relation_id,
                            child: previous,
                            index,
                        });
                        self.retire(previous);
                    }
                    let child = self.mount(desired)?;
                    self.mutations.push(Mutation::Insert {
                        parent: object,
                        relation: relation_id,
                        child,
                        index,
                    });
                    child
                }
            } else {
                let child = self.mount(desired)?;
                self.mutations.push(Mutation::Insert {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                child
            };
            next.push(child);
        }
        for (index, child) in previous
            .iter()
            .enumerate()
            .skip(desired.len())
            .rev()
            .map(|(index, child)| (index, *child))
        {
            if !self.retire_with_transition(object, relation_id, child)? {
                self.mutations.push(Mutation::Remove {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                self.retire(child);
            }
        }
        if previous != next {
            self.retained.relation_mut(object, relation_id).value =
                RetainedRelationValue::Many(next);
        }
        Ok(())
    }

    fn retire(&mut self, object: ObjectId) {
        for (root, nodes) in self.retained.retirements_for_parent(object) {
            self.mutations
                .push(Mutation::CompleteRetirement { root, nodes });
            self.retained.complete_retirement(root);
        }
        if let Some(reference) = self.retained.get_mut(object).reference.take() {
            self.references
                .push(ReferenceChange::Clear { reference, object });
        }
        let virtual_items = self.retained.get_mut(object).virtual_items.take();
        let relations = std::mem::take(&mut self.retained.get_mut(object).relations);
        for relation in relations {
            match relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.mutations.push(Mutation::Detach {
                        parent: object,
                        relation: relation.id,
                        child,
                    });
                    self.retire(child);
                }
                RetainedRelationValue::Many(children) => {
                    if let Some(items) = virtual_items
                        .as_ref()
                        .filter(|items| items.relation == relation.id)
                    {
                        let mut containers = items.containers.iter().collect::<Vec<_>>();
                        containers.sort_unstable_by_key(|(container, realization)| {
                            (realization.index, container.0)
                        });
                        for (container, realization) in containers {
                            self.mutations.push(Mutation::Recycle {
                                parent: object,
                                relation: relation.id,
                                container: *container,
                                child: realization.child,
                            });
                        }
                        for child in children.into_iter().rev() {
                            self.retire(child);
                        }
                    } else {
                        for (index, child) in children.into_iter().enumerate().rev() {
                            self.mutations.push(Mutation::Remove {
                                parent: object,
                                relation: relation.id,
                                child,
                                index,
                            });
                            self.retire(child);
                        }
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        self.mutations.push(Mutation::Destroy { object });
        self.retained.remove(object);
    }

    fn retire_with_transition(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        root: ObjectId,
    ) -> Result<bool, GraphError> {
        let Some(transition) = self.retained.get(root).unwrap().exit_transition else {
            return Ok(false);
        };
        let contract = relation_contracts(self.retained.get(parent).unwrap().kind)
            .iter()
            .find(|contract| contract.id == relation)
            .unwrap();
        if contract.cardinality != Cardinality::Many || contract.realization != Realization::Owned {
            return Err(GraphError::ExitTransitionUnsupported);
        }
        let mut nodes = Vec::new();
        self.collect_subtree_postorder(root, &mut nodes);
        for object in &nodes {
            let retained = self.retained.get_mut(*object);
            if let Some(reference) = retained.reference.take() {
                self.references.push(ReferenceChange::Clear {
                    reference,
                    object: *object,
                });
            }
            if let Some(events) = retained.events.take() {
                self.mutations.push(Mutation::SetEvents {
                    object: *object,
                    set: Rc::from([]),
                    clear: events.iter().map(|event| event.id).collect(),
                });
            }
        }
        self.mutations.push(Mutation::Retire {
            root,
            nodes: nodes.clone(),
            parent,
            relation,
            duration: transition.duration(),
        });
        self.retained.mark_retiring(root, nodes, parent);
        Ok(true)
    }

    fn collect_subtree_postorder(&self, object: ObjectId, nodes: &mut Vec<ObjectId>) {
        for relation in &self.retained.get(object).unwrap().relations {
            match &relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.collect_subtree_postorder(*child, nodes);
                }
                RetainedRelationValue::Many(children) => {
                    for child in children {
                        self.collect_subtree_postorder(*child, nodes);
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        nodes.push(object);
    }
}
