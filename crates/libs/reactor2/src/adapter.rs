use super::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordingAdapter {
    objects: HashMap<ObjectId, RecordedObject>,
    owners: Rc<HashMap<ObjectId, (ObjectId, RelationId)>>,
    batches: Vec<Vec<Mutation>>,
    observations: Vec<Observation>,
    events: Vec<EventDispatch>,
    record_batches: bool,
    validate_batches: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct RecordedObject {
    kind: ObjectType,
    properties: Rc<[Property]>,
    events: Rc<[Event]>,
    relations: Rc<HashMap<RelationId, RecordedRelation>>,
}

#[derive(Clone, Debug, PartialEq)]
enum RecordedRelation {
    One(Option<ObjectId>),
    Many(Vec<ObjectId>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterError {
    DuplicateObject(ObjectId),
    MissingObject(ObjectId),
    InvalidRelation(ObjectId, RelationId),
    InvalidMutation(RelationId),
    InvalidChildCategory(RelationId),
    InvalidReplacement(ObjectId),
    ChildNotFound(ObjectId),
    AlreadyOwned(ObjectId),
    StillOwned(ObjectId),
}

impl RecordingAdapter {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            owners: Rc::new(HashMap::new()),
            batches: Vec::new(),
            observations: Vec::new(),
            events: Vec::new(),
            record_batches: false,
            validate_batches: true,
        }
    }

    pub fn record_batches(&mut self, record: bool) {
        self.record_batches = record;
    }

    pub fn validate_batches(&mut self, validate: bool) {
        self.validate_batches = validate;
    }

    pub fn batches(&self) -> &[Vec<Mutation>] {
        &self.batches
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn observe(&mut self, observation: Observation) {
        self.observations.push(observation);
    }

    pub fn queue_event(&mut self, event: EventDispatch) {
        self.events.push(event);
    }

    pub fn children(&self, object: ObjectId, relation: RelationId) -> Option<&[ObjectId]> {
        match self.objects.get(&object)?.relations.get(&relation)? {
            RecordedRelation::Many(children) => Some(children),
            _ => None,
        }
    }

    fn apply_batch(&mut self, mutations: &[Mutation]) -> Result<(), AdapterError> {
        for mutation in mutations {
            match mutation {
                Mutation::Create { object, kind } => {
                    if self.objects.contains_key(object) {
                        return Err(AdapterError::DuplicateObject(*object));
                    }
                    self.objects.insert(*object, Self::recorded_object(*kind));
                }
                Mutation::Replace { object, kind } => {
                    let previous = self
                        .objects
                        .get(object)
                        .ok_or(AdapterError::MissingObject(*object))?;
                    if previous.relations.values().any(|relation| match relation {
                        RecordedRelation::One(child) => child.is_some(),
                        RecordedRelation::Many(children) => !children.is_empty(),
                    }) {
                        return Err(AdapterError::StillOwned(*object));
                    }
                    let (parent, relation) = self
                        .owners
                        .get(object)
                        .copied()
                        .ok_or(AdapterError::InvalidReplacement(*object))?;
                    let parent_object = self
                        .objects
                        .get(&parent)
                        .ok_or(AdapterError::MissingObject(parent))?;
                    let contract = relation_contracts(parent_object.kind)
                        .iter()
                        .find(|contract| contract.id == relation)
                        .ok_or(AdapterError::InvalidRelation(parent, relation))?;
                    if contract.realization != Realization::Owned
                        || contract.child != object_category(*kind)
                    {
                        return Err(AdapterError::InvalidReplacement(*object));
                    }
                    self.objects.insert(*object, Self::recorded_object(*kind));
                    self.observations.retain(|observation| {
                        !matches!(
                            observation,
                            Observation::SetProperty {
                                object: observed,
                                ..
                            } if observed == object
                        )
                    });
                    self.events.retain(|event| event.object() != *object);
                }
                Mutation::SetProperties { object, set, clear } => {
                    let object = self.object_mut(*object)?;
                    let mut properties = object.properties.to_vec();
                    properties.retain(|property| !clear.contains(&property.id));
                    for property in set.iter() {
                        if let Some(current) = properties
                            .iter_mut()
                            .find(|current| current.id == property.id)
                        {
                            current.clone_from(property);
                        } else {
                            properties.push(property.clone());
                        }
                    }
                    object.properties = properties.into();
                }
                Mutation::SetEvents { object, set, clear } => {
                    let object = self.object_mut(*object)?;
                    let mut events = object.events.to_vec();
                    events.retain(|event| !clear.contains(&event.id));
                    for event in set.iter() {
                        if let Some(current) =
                            events.iter_mut().find(|current| current.id == event.id)
                        {
                            current.clone_from(event);
                        } else {
                            events.push(event.clone());
                        }
                    }
                    object.events = events.into();
                }
                Mutation::Attach {
                    parent,
                    relation,
                    child,
                } => {
                    self.require_object(*child)?;
                    self.validate_child(*parent, *relation, *child)?;
                    if self.is_owned(*child) {
                        return Err(AdapterError::AlreadyOwned(*child));
                    }
                    match self.relation_mut(*parent, *relation)? {
                        RecordedRelation::One(slot @ None) => *slot = Some(*child),
                        _ => return Err(AdapterError::InvalidMutation(*relation)),
                    }
                    Rc::make_mut(&mut self.owners).insert(*child, (*parent, *relation));
                }
                Mutation::Detach {
                    parent,
                    relation,
                    child,
                } => match self.relation_mut(*parent, *relation)? {
                    RecordedRelation::One(slot) if *slot == Some(*child) => {
                        *slot = None;
                        Rc::make_mut(&mut self.owners).remove(child);
                    }
                    _ => return Err(AdapterError::ChildNotFound(*child)),
                },
                Mutation::Insert {
                    parent,
                    relation,
                    child,
                    index,
                } => {
                    self.require_object(*child)?;
                    self.validate_child(*parent, *relation, *child)?;
                    if self.is_owned(*child) {
                        return Err(AdapterError::AlreadyOwned(*child));
                    }
                    match self.relation_mut(*parent, *relation)? {
                        RecordedRelation::Many(children) if *index <= children.len() => {
                            children.insert(*index, *child);
                        }
                        _ => return Err(AdapterError::InvalidMutation(*relation)),
                    }
                    Rc::make_mut(&mut self.owners).insert(*child, (*parent, *relation));
                }
                Mutation::Remove {
                    parent,
                    relation,
                    child,
                    index,
                } => match self.relation_mut(*parent, *relation)? {
                    RecordedRelation::Many(children) => {
                        if children.get(*index) != Some(child) {
                            return Err(AdapterError::ChildNotFound(*child));
                        }
                        children.remove(*index);
                        Rc::make_mut(&mut self.owners).remove(child);
                    }
                    _ => return Err(AdapterError::InvalidMutation(*relation)),
                },
                Mutation::Reorder {
                    parent,
                    relation,
                    moves: _,
                    children,
                } => match self.relation_mut(*parent, *relation)? {
                    RecordedRelation::Many(current) => {
                        let mut expected = current.clone();
                        let mut requested = children.clone();
                        expected
                            .sort_unstable_by_key(|object| (object.index(), object.generation()));
                        requested
                            .sort_unstable_by_key(|object| (object.index(), object.generation()));
                        if expected != requested {
                            return Err(AdapterError::InvalidMutation(*relation));
                        }
                        current.clone_from(children);
                    }
                    _ => return Err(AdapterError::InvalidMutation(*relation)),
                },
                Mutation::Destroy { object } => {
                    if self.is_owned(*object) {
                        return Err(AdapterError::StillOwned(*object));
                    }
                    self.objects
                        .remove(object)
                        .ok_or(AdapterError::MissingObject(*object))?;
                }
            }
        }
        Ok(())
    }

    fn require_object(&self, object: ObjectId) -> Result<(), AdapterError> {
        self.objects
            .contains_key(&object)
            .then_some(())
            .ok_or(AdapterError::MissingObject(object))
    }

    fn object_mut(&mut self, object: ObjectId) -> Result<&mut RecordedObject, AdapterError> {
        self.objects
            .get_mut(&object)
            .ok_or(AdapterError::MissingObject(object))
    }

    fn relation_mut(
        &mut self,
        object: ObjectId,
        relation: RelationId,
    ) -> Result<&mut RecordedRelation, AdapterError> {
        Rc::make_mut(&mut self.object_mut(object)?.relations)
            .get_mut(&relation)
            .ok_or(AdapterError::InvalidRelation(object, relation))
    }

    fn validate_child(
        &self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), AdapterError> {
        let parent_id = parent;
        let parent = self
            .objects
            .get(&parent)
            .ok_or(AdapterError::MissingObject(parent))?;
        let child = self
            .objects
            .get(&child)
            .ok_or(AdapterError::MissingObject(child))?;
        let contract = relation_contracts(parent.kind)
            .iter()
            .find(|contract| contract.id == relation)
            .ok_or(AdapterError::InvalidRelation(parent_id, relation))?;
        if object_category(child.kind) == contract.child {
            Ok(())
        } else {
            Err(AdapterError::InvalidChildCategory(relation))
        }
    }

    fn recorded_object(kind: ObjectType) -> RecordedObject {
        RecordedObject {
            kind,
            properties: Rc::from([]),
            events: Rc::from([]),
            relations: Rc::new(
                relation_contracts(kind)
                    .iter()
                    .map(|contract| {
                        (
                            contract.id,
                            match contract.cardinality {
                                Cardinality::One => RecordedRelation::One(None),
                                Cardinality::Many => RecordedRelation::Many(Vec::new()),
                            },
                        )
                    })
                    .collect(),
            ),
        }
    }

    fn is_owned(&self, object: ObjectId) -> bool {
        self.owners.contains_key(&object)
    }
}

impl Default for RecordingAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter for RecordingAdapter {
    type Error = AdapterError;

    fn drain_observations(&mut self, observations: &mut Vec<Observation>) {
        observations.append(&mut self.observations);
    }

    fn drain_events(&mut self, events: &mut Vec<EventDispatch>) {
        events.append(&mut self.events);
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if !self.validate_batches {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.apply_batch(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.apply_batch(mutations)?;
        if self.record_batches {
            self.batches.push(mutations.to_vec());
        }
        Ok(())
    }
}
