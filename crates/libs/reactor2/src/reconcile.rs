use super::*;
use crate::ir::validate_property;
use std::collections::HashMap;
use std::rc::Rc;

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
    Destroy {
        object: ObjectId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Observation {
    SetProperty {
        object: ObjectId,
        property: Property,
    },
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
            (EventValue::String(callback), EventPayload::String(value)) => callback.call(value),
            (EventValue::F64(callback), EventPayload::F64(value)) => callback.call(value),
            (EventValue::Unit(callback), EventPayload::Unit) => callback.call(()),
            _ => unreachable!(),
        }
    }

    pub(crate) fn object(&self) -> ObjectId {
        self.object
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
    properties: SharedList<Property>,
    events: Option<Rc<Vec<Event>>>,
    relations: Vec<RetainedRelation>,
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
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RetainedSlot {
    generation: u32,
    object: Option<RetainedObject>,
}

impl RetainedGraph {
    pub fn root(&self) -> Option<ObjectId> {
        self.root
    }

    pub fn object_count(&self) -> usize {
        self.objects
            .iter()
            .filter(|slot| slot.object.is_some())
            .count()
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

    fn owner(&self, child: ObjectId) -> Option<(ObjectId, RelationId)> {
        self.objects.iter().enumerate().find_map(|(index, slot)| {
            let object = slot.object.as_ref()?;
            object.relations.iter().find_map(|relation| {
                let contains = match &relation.value {
                    RetainedRelationValue::One(current) => *current == Some(child),
                    RetainedRelationValue::Many(children) => children.contains(&child),
                };
                contains.then_some((
                    ObjectId {
                        index: index.try_into().unwrap(),
                        generation: slot.generation,
                    },
                    relation.id,
                ))
            })
        })
    }

    pub fn properties(&self, object: ObjectId) -> Option<&[Property]> {
        self.get(object).map(|object| object.properties.as_slice())
    }

    fn apply_observation(&mut self, observation: Observation) -> Result<(), GraphError> {
        match observation {
            Observation::SetProperty { object, property } => {
                let Some(object) = self.try_get_mut(object) else {
                    return Ok(());
                };
                validate_property(object.kind, &property)?;
                let property_id = property.id;
                object
                    .properties
                    .upsert(|current| current.id == property_id, property);
            }
        }
        Ok(())
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
            (
                ValueType::String,
                EventValue::String(_),
                EventPayload::String(_)
            ) | (ValueType::F64, EventValue::F64(_), EventPayload::F64(_))
                | (ValueType::Unit, EventValue::Unit(_), EventPayload::Unit)
        ) {
            return Err(GraphError::InvalidEventValue(dispatch.event));
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
        (slot.generation == object.generation)
            .then_some(slot.object.as_ref())
            .flatten()
    }

    fn get_mut(&mut self, object: ObjectId) -> &mut RetainedObject {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        slot.object.as_mut().unwrap()
    }

    fn try_get_mut(&mut self, object: ObjectId) -> Option<&mut RetainedObject> {
        let slot = self.objects.get_mut(object.index as usize)?;
        (slot.generation == object.generation)
            .then_some(slot.object.as_mut())
            .flatten()
    }

    fn relation(&self, object: ObjectId, relation: RelationId) -> Option<&RetainedRelation> {
        self.get(object)?
            .relations
            .iter()
            .find(|state| state.id == relation)
    }

    fn relation_mut(&mut self, object: ObjectId, relation: RelationId) -> &mut RetainedRelation {
        self.get_mut(object)
            .relations
            .iter_mut()
            .find(|state| state.id == relation)
            .unwrap()
    }

    fn allocate(&mut self, object: RetainedObject) -> ObjectId {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.objects[index as usize];
            slot.object = Some(object);
            ObjectId {
                index,
                generation: slot.generation,
            }
        } else {
            let id = ObjectId {
                index: self.objects.len() as u32,
                generation: 0,
            };
            self.objects.push(RetainedSlot {
                generation: 0,
                object: Some(object),
            });
            id
        }
    }

    fn remove(&mut self, object: ObjectId) {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        slot.object = None;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(object.index);
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
            || current.properties.as_slice() != declaration.properties.as_slice()
            || retained_events(&current.events) != declaration.events.as_slice()
        {
            return Ok(false);
        }
        for contract in relation_contracts(current.kind) {
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
                            if self
                                .matches_declaration_inner(*previous, next, remaining, true)? => {}
                        _ => return Ok(false),
                    }
                }
                (RetainedRelationValue::Many(previous), Some(RelationValue::Many(next))) => {
                    if previous.len() != next.len() {
                        return Ok(false);
                    }
                    for (previous, next) in previous.iter().zip(next.iter()) {
                        if !self.matches_declaration_inner(*previous, next, remaining, true)? {
                            return Ok(false);
                        }
                    }
                }
                _ => return Ok(false),
            }
        }
        Ok(true)
    }
}

pub trait Adapter {
    type Error;

    fn drain_observations(&mut self, _observations: &mut Vec<Observation>) {}
    fn drain_events(&mut self, _events: &mut Vec<EventDispatch>) {}
    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error>;
    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error>;
}

pub struct Runtime<A> {
    graph: RetainedGraph,
    adapter: A,
    mutations: Vec<Mutation>,
    observations: Vec<Observation>,
    events: Vec<EventDispatch>,
    poisoned: bool,
}

impl<A: Adapter> Runtime<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            graph: RetainedGraph::default(),
            adapter,
            mutations: Vec::new(),
            observations: Vec::new(),
            events: Vec::new(),
            poisoned: false,
        }
    }

    pub fn graph(&self) -> &RetainedGraph {
        &self.graph
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    pub fn adapter_mut(&mut self) -> &mut A {
        &mut self.adapter
    }

    pub fn drain_events(
        &mut self,
        events: &mut Vec<EventDispatch>,
    ) -> Result<(), UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        self.observations.clear();
        self.adapter.drain_observations(&mut self.observations);
        for observation in self.observations.drain(..) {
            self.graph
                .apply_observation(observation)
                .map_err(UpdateError::Graph)?;
        }
        self.events.clear();
        self.adapter.drain_events(&mut self.events);
        for event in self.events.drain(..) {
            if self
                .graph
                .validate_event_dispatch(&event)
                .map_err(UpdateError::Graph)?
            {
                events.push(event);
            }
        }
        Ok(())
    }

    pub fn update(
        &mut self,
        root: impl Into<Visual>,
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        self.mutations.clear();
        let declaration = root.into();
        validate_declaration(&declaration.0).map_err(UpdateError::Graph)?;
        self.observations.clear();
        self.adapter.drain_observations(&mut self.observations);
        for observation in self.observations.drain(..) {
            self.graph
                .apply_observation(observation)
                .map_err(UpdateError::Graph)?;
        }
        if let Some(root) = self.graph.root {
            let mut remaining = MAX_OBJECTS;
            if self
                .graph
                .matches_declaration(root, &declaration.0, &mut remaining)
                .map_err(UpdateError::Graph)?
            {
                return Ok(Vec::new());
            }
        }
        if let Some(current) = self.graph.root {
            let previous = self.graph.get(current).unwrap().kind;
            let next = declaration.0.kind;
            if previous != next {
                return Err(UpdateError::Graph(GraphError::RootTypeChanged {
                    previous,
                    next,
                }));
            }
        }
        {
            let mut planner = Planner {
                retained: &mut self.graph,
                mutations: &mut self.mutations,
            };
            let root = match planner.retained.root {
                Some(current) => planner
                    .reconcile_object(current, &declaration.0)
                    .map_err(UpdateError::Graph)?,
                None => planner.mount(&declaration.0).map_err(UpdateError::Graph)?,
            };
            planner.retained.root = Some(root);
        }
        if let Err(error) = self.adapter.validate(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
        if let Err(error) = self.adapter.apply(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
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

    pub(crate) fn update_owned_subtree_before_apply(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        object: ObjectId,
        root: impl Into<Visual>,
        before_apply: impl FnOnce(),
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        self.update_subtree_inner(object, root.into(), Some((parent, relation)), before_apply)
    }

    fn update_subtree_inner(
        &mut self,
        object: ObjectId,
        declaration: Visual,
        owner: Option<(ObjectId, RelationId)>,
        before_apply: impl FnOnce(),
    ) -> Result<Vec<Mutation>, UpdateError<A::Error>> {
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
        self.mutations.clear();
        validate_declaration(&declaration.0).map_err(UpdateError::Graph)?;
        let previous = self
            .graph
            .get(object)
            .ok_or(UpdateError::Graph(GraphError::StaleObject(object)))?
            .kind;
        if previous != declaration.0.kind {
            let owner = owner.or_else(|| self.graph.owner(object));
            let Some((parent, relation)) = owner else {
                return Err(UpdateError::Graph(GraphError::RootTypeChanged {
                    previous,
                    next: declaration.0.kind,
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
                || contract.child != object_category(declaration.0.kind)
            {
                return Err(UpdateError::Graph(GraphError::InvalidChildCategory(
                    relation,
                )));
            }
        }
        let mut remaining = MAX_OBJECTS;
        if self
            .graph
            .matches_subtree_declaration(object, &declaration.0, &mut remaining)
            .map_err(UpdateError::Graph)?
        {
            before_apply();
            return Ok(Vec::new());
        }
        {
            let mut planner = Planner {
                retained: &mut self.graph,
                mutations: &mut self.mutations,
            };
            if previous == declaration.0.kind {
                planner
                    .reconcile_object(object, &declaration.0)
                    .map_err(UpdateError::Graph)?;
            } else {
                planner
                    .replace_object(object, &declaration.0)
                    .map_err(UpdateError::Graph)?;
            }
        }
        if let Err(error) = self.adapter.validate(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
        before_apply();
        if let Err(error) = self.adapter.apply(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
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
        if self.poisoned {
            return Err(UpdateError::Poisoned);
        }
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
        self.mutations.push(Mutation::Remove {
            parent,
            relation,
            child,
            index,
        });
        {
            let mut planner = Planner {
                retained: &mut self.graph,
                mutations: &mut self.mutations,
            };
            planner.retire(child);
        }
        let RetainedRelationValue::Many(children) =
            &mut self.graph.relation_mut(parent, relation).value
        else {
            unreachable!()
        };
        children.remove(index);
        if let Err(error) = self.adapter.validate(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
        if let Err(error) = self.adapter.apply(&self.mutations) {
            self.poisoned = true;
            self.graph = RetainedGraph::default();
            self.mutations.clear();
            return Err(UpdateError::Adapter(error));
        }
        let mutations = self.mutations.clone();
        if self.mutations.capacity() > 256 {
            self.mutations = Vec::with_capacity(256);
        }
        Ok(mutations)
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdateError<E> {
    Graph(GraphError),
    Adapter(E),
    Poisoned,
}

struct Planner<'a> {
    retained: &'a mut RetainedGraph,
    mutations: &'a mut Vec<Mutation>,
}

impl Planner<'_> {
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
        let key = self.retained.get_mut(object).key.take();
        *self.retained.get_mut(object) = RetainedObject {
            kind: declaration.kind,
            key,
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
        for contract in relation_contracts(declaration.kind) {
            self.mount_relation(object, declaration, contract)?;
        }
        Ok(())
    }

    fn mount(&mut self, declaration: &Declaration) -> Result<ObjectId, GraphError> {
        let object = self.retained.allocate(RetainedObject {
            kind: declaration.kind,
            key: declaration.key.clone(),
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
        });
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
                    .map(|child| self.mount(child))
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
                    let child = self.mount(child)?;
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
                let next = match (previous, desired) {
                    (Some(previous), Some(desired))
                        if self.retained.get(previous).unwrap().kind == desired.kind =>
                    {
                        Some(self.reconcile_object(previous, desired)?)
                    }
                    (previous, desired) => {
                        if let Some(previous) = previous {
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
                previous.iter().zip(desired).all(|(previous, desired)| {
                    let previous = self.retained.get(*previous).unwrap();
                    previous.kind == desired.kind && previous.key == desired.key
                })
            }
            _ => false,
        };
        if same_order {
            for (index, desired) in desired.iter().enumerate() {
                let previous = match &self.retained.relation(object, relation_id).unwrap().value {
                    RetainedRelationValue::Many(previous) => previous[index],
                    _ => unreachable!(),
                };
                self.reconcile_object(previous, desired)?;
            }
            return Ok(());
        }
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
        for desired in desired {
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
            self.mutations.push(Mutation::Remove {
                parent: object,
                relation: relation_id,
                child,
                index,
            });
            self.retire(child);
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

        fn longest_increasing_positions(
            sequence: &[(usize, usize)],
        ) -> std::collections::HashSet<usize> {
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

            let mut positions = std::collections::HashSet::with_capacity(tails.len());
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
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut next = Vec::with_capacity(desired.len());
        for (index, desired) in desired.iter().enumerate() {
            let child = if let Some(previous) = previous.get(index).copied() {
                if self.retained.get(previous).unwrap().kind == desired.kind {
                    self.reconcile_object(previous, desired)?
                } else {
                    self.mutations.push(Mutation::Remove {
                        parent: object,
                        relation: relation_id,
                        child: previous,
                        index,
                    });
                    self.retire(previous);
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
            self.mutations.push(Mutation::Remove {
                parent: object,
                relation: relation_id,
                child,
                index,
            });
            self.retire(child);
        }
        if previous != next {
            self.retained.relation_mut(object, relation_id).value =
                RetainedRelationValue::Many(next);
        }
        Ok(())
    }

    fn retire(&mut self, object: ObjectId) {
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
        self.mutations.push(Mutation::Destroy { object });
        self.retained.remove(object);
    }
}
