use super::*;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordingAdapter {
    objects: HashMap<ObjectId, RecordedObject>,
    owners: Rc<HashMap<ObjectId, (ObjectId, RelationId)>>,
    batches: Vec<Vec<Mutation>>,
    native_events: VecDeque<NativeEvent>,
    retirements: HashMap<ObjectId, RecordedRetirement>,
    realizations: HashMap<(ObjectId, RealizedContainer), (RelationId, usize, ObjectId)>,
    focuses: Vec<ObjectId>,
    imperatives: Vec<ImperativeRequest>,
    window_title_bar: Option<(ObjectId, WindowTitleBarHeight)>,
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
struct RecordedRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
    relation: RelationId,
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
    InvalidWindowTitleBar(ObjectId),
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
            native_events: VecDeque::new(),
            retirements: HashMap::new(),
            realizations: HashMap::new(),
            focuses: Vec::new(),
            imperatives: Vec::new(),
            window_title_bar: None,
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

    pub fn focuses(&self) -> &[ObjectId] {
        &self.focuses
    }

    pub fn imperatives(&self) -> &[ImperativeRequest] {
        &self.imperatives
    }

    pub fn window_title_bar(&self) -> Option<(ObjectId, WindowTitleBarHeight)> {
        self.window_title_bar
    }

    pub fn retirement_count(&self) -> usize {
        self.retirements.len()
    }

    pub fn complete_retirement(&mut self, root: ObjectId) -> bool {
        if !self.retirements.contains_key(&root) {
            return false;
        }
        self.queue_retirement_completion(root);
        true
    }

    pub fn queue_retirement_completion(&mut self, root: ObjectId) {
        self.native_events
            .push_back(NativeEvent::retirement(RetirementCompletion { root }));
    }

    pub fn observe(&mut self, observation: Observation) {
        self.native_events
            .push_back(NativeEvent::observation(observation));
    }

    pub fn queue_event(&mut self, event: EventDispatch) {
        self.native_events.push_back(NativeEvent::event(event));
    }

    pub fn queue_native_event(
        &mut self,
        observation: Option<Observation>,
        event: Option<EventDispatch>,
    ) {
        self.native_events
            .push_back(NativeEvent::new(observation, event));
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        let request = Self::coalesce_realization(&mut self.native_events, request);
        self.native_events
            .push_back(NativeEvent::realization(request));
    }

    pub fn realized_count(&self, object: ObjectId) -> usize {
        self.realizations
            .keys()
            .filter(|(parent, _)| *parent == object)
            .count()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn realization_count(&self) -> usize {
        self.realizations.len()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn queued_native_event_count(&self) -> usize {
        self.native_events.len()
    }

    fn coalesce_realization(
        events: &mut VecDeque<NativeEvent>,
        request: RealizationRequest,
    ) -> RealizationRequest {
        let RealizationRequest::Recycle {
            collection,
            container,
            source_revision,
        } = request
        else {
            return request;
        };
        let pending = events.iter().rposition(|event| {
            matches!(
                event.realization_request(),
                Some(RealizationRequest::Realize {
                    collection: pending_collection,
                    container: pending_container,
                    ..
                }) if pending_collection == collection && pending_container == container
            )
        });
        if let Some(pending) = pending {
            events.remove(pending);
            RealizationRequest::Cancel {
                collection,
                container,
                source_revision,
            }
        } else {
            request
        }
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
                        || !relation_accepts(contract, *kind)
                    {
                        return Err(AdapterError::InvalidReplacement(*object));
                    }
                    self.objects.insert(*object, Self::recorded_object(*kind));
                    self.native_events
                        .retain(|event| event.object() != Some(*object));
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
                Mutation::ClearWindowTitleBar { object } => {
                    if self
                        .window_title_bar
                        .is_some_and(|(current, _)| current == *object)
                    {
                        self.window_title_bar = None;
                    }
                }
                Mutation::SetWindowTitleBar { object, height } => {
                    if self
                        .objects
                        .get(object)
                        .is_none_or(|object| object.kind != ObjectType::TitleBar)
                    {
                        return Err(AdapterError::InvalidWindowTitleBar(*object));
                    }
                    self.window_title_bar = Some((*object, *height));
                }
                Mutation::SetVirtualSource { object, .. } => {
                    self.require_object(*object)?;
                }
                Mutation::Realize {
                    parent,
                    relation,
                    container,
                    index,
                    child,
                } => {
                    self.require_object(*child)?;
                    self.validate_child(*parent, *relation, *child)?;
                    if self.is_owned(*child)
                        && self.owners.get(child) != Some(&(*parent, *relation))
                    {
                        return Err(AdapterError::AlreadyOwned(*child));
                    }
                    self.realizations.retain(|(owner, _), (_, _, current)| {
                        !(*owner == *parent && *current == *child)
                    });
                    self.realizations
                        .insert((*parent, *container), (*relation, *index, *child));
                    Rc::make_mut(&mut self.owners).insert(*child, (*parent, *relation));
                    self.rebuild_realized_relation(*parent, *relation)?;
                }
                Mutation::Recycle {
                    parent,
                    relation,
                    container,
                    child,
                } => {
                    if let Some((current_relation, _, current_child)) =
                        self.realizations.remove(&(*parent, *container))
                    {
                        if current_relation != *relation
                            || child.is_some_and(|child| child != current_child)
                        {
                            return Err(AdapterError::InvalidMutation(*relation));
                        }
                        Rc::make_mut(&mut self.owners).remove(&current_child);
                        self.rebuild_realized_relation(*parent, *relation)?;
                    }
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
                    let index = self.active_index(*parent, *relation, *index)?;
                    match self.relation_mut(*parent, *relation)? {
                        RecordedRelation::Many(children) if index <= children.len() => {
                            children.insert(index, *child);
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
                } => {
                    let index = self.active_index(*parent, *relation, *index)?;
                    match self.relation_mut(*parent, *relation)? {
                        RecordedRelation::Many(children) => {
                            if children.get(index) != Some(child) {
                                return Err(AdapterError::ChildNotFound(*child));
                            }
                            children.remove(index);
                            Rc::make_mut(&mut self.owners).remove(child);
                        }
                        _ => return Err(AdapterError::InvalidMutation(*relation)),
                    }
                }
                Mutation::Reorder {
                    parent,
                    relation,
                    moves: _,
                    children,
                } => {
                    let retired = self
                        .retirements
                        .iter()
                        .filter_map(|(root, retirement)| {
                            (retirement.parent == *parent && retirement.relation == *relation)
                                .then_some(*root)
                        })
                        .collect::<Vec<_>>();
                    match self.relation_mut(*parent, *relation)? {
                        RecordedRelation::Many(current) => {
                            let mut expected = current
                                .iter()
                                .copied()
                                .filter(|child| !retired.contains(child))
                                .collect::<Vec<_>>();
                            let mut requested = children.clone();
                            expected.sort_unstable_by_key(|object| {
                                (object.index(), object.generation())
                            });
                            requested.sort_unstable_by_key(|object| {
                                (object.index(), object.generation())
                            });
                            if expected != requested {
                                return Err(AdapterError::InvalidMutation(*relation));
                            }
                            let mut requested = children.iter().copied();
                            for child in current.iter_mut() {
                                if !retired.contains(child) {
                                    let Some(next) = requested.next() else {
                                        return Err(AdapterError::InvalidMutation(*relation));
                                    };
                                    *child = next;
                                }
                            }
                            current.extend(requested);
                        }
                        _ => return Err(AdapterError::InvalidMutation(*relation)),
                    }
                }
                Mutation::Retire {
                    root,
                    nodes,
                    parent,
                    relation,
                    ..
                } => {
                    if self.owners.get(root) != Some(&(*parent, *relation))
                        || nodes.iter().any(|node| !self.objects.contains_key(node))
                        || self.retirements.contains_key(root)
                    {
                        return Err(AdapterError::MissingObject(*root));
                    }
                    self.retirements.insert(
                        *root,
                        RecordedRetirement {
                            nodes: nodes.clone(),
                            parent: *parent,
                            relation: *relation,
                        },
                    );
                }
                Mutation::CompleteRetirement { root, nodes } => {
                    let retirement = self
                        .retirements
                        .remove(root)
                        .ok_or(AdapterError::MissingObject(*root))?;
                    if retirement.nodes != *nodes {
                        return Err(AdapterError::InvalidReplacement(*root));
                    }
                    let relation = self.relation_mut(retirement.parent, retirement.relation)?;
                    let RecordedRelation::Many(children) = relation else {
                        return Err(AdapterError::InvalidMutation(retirement.relation));
                    };
                    let Some(index) = children.iter().position(|child| child == root) else {
                        return Err(AdapterError::ChildNotFound(*root));
                    };
                    children.remove(index);
                    let owners = Rc::make_mut(&mut self.owners);
                    for object in nodes {
                        owners.remove(object);
                        self.objects
                            .remove(object)
                            .ok_or(AdapterError::MissingObject(*object))?;
                        self.native_events
                            .retain(|event| event.object() != Some(*object));
                    }
                }
                Mutation::Destroy { object } => {
                    if self
                        .window_title_bar
                        .is_some_and(|(current, _)| current == *object)
                    {
                        return Err(AdapterError::InvalidWindowTitleBar(*object));
                    }
                    if self.is_owned(*object) {
                        return Err(AdapterError::StillOwned(*object));
                    }
                    self.objects
                        .remove(object)
                        .ok_or(AdapterError::MissingObject(*object))?;
                    self.realizations
                        .retain(|(parent, _), (_, _, child)| parent != object && child != object);
                }
            }
        }
        Ok(())
    }

    fn rebuild_realized_relation(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
    ) -> Result<(), AdapterError> {
        let mut children = self
            .realizations
            .iter()
            .filter_map(|((owner, _), (current_relation, index, child))| {
                (*owner == parent && *current_relation == relation).then_some((*index, *child))
            })
            .collect::<Vec<_>>();
        children.sort_unstable_by_key(|(index, _)| *index);
        let RecordedRelation::Many(current) = self.relation_mut(parent, relation)? else {
            return Err(AdapterError::InvalidMutation(relation));
        };
        *current = children.into_iter().map(|(_, child)| child).collect();
        Ok(())
    }

    fn require_object(&self, object: ObjectId) -> Result<(), AdapterError> {
        self.objects
            .contains_key(&object)
            .then_some(())
            .ok_or(AdapterError::MissingObject(object))
    }

    fn active_index(
        &self,
        parent: ObjectId,
        relation: RelationId,
        active_index: usize,
    ) -> Result<usize, AdapterError> {
        let RecordedRelation::Many(children) = self
            .objects
            .get(&parent)
            .and_then(|object| object.relations.get(&relation))
            .ok_or(AdapterError::InvalidRelation(parent, relation))?
        else {
            return Err(AdapterError::InvalidMutation(relation));
        };
        let retired = self
            .retirements
            .iter()
            .filter_map(|(root, retirement)| {
                (retirement.parent == parent && retirement.relation == relation).then_some(*root)
            })
            .collect::<Vec<_>>();
        let mut active = 0;
        for (index, child) in children.iter().enumerate() {
            if retired.contains(child) {
                continue;
            }
            if active == active_index {
                return Ok(index);
            }
            active += 1;
        }
        (active == active_index)
            .then_some(children.len())
            .ok_or(AdapterError::InvalidMutation(relation))
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

impl Runtime<RecordingAdapter> {
    pub fn record_batches(&mut self, record: bool) {
        self.adapter_mut().record_batches(record);
    }

    pub fn validate_batches(&mut self, validate: bool) {
        self.adapter_mut().validate_batches(validate);
    }

    pub fn complete_retirement(&mut self, root: ObjectId) -> bool {
        self.adapter_mut().complete_retirement(root)
    }

    pub fn queue_event(&mut self, event: EventDispatch) {
        self.adapter_mut().queue_event(event);
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        self.adapter_mut().queue_realization(request);
    }
}

impl ComponentHost<RecordingAdapter> {
    pub fn record_batches(&mut self, record: bool) {
        self.runtime_mut_internal().record_batches(record);
    }

    pub fn validate_batches(&mut self, validate: bool) {
        self.runtime_mut_internal().validate_batches(validate);
    }

    pub fn queue_event(&mut self, event: EventDispatch) {
        self.runtime_mut_internal().queue_event(event);
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        self.runtime_mut_internal().queue_realization(request);
    }
}

impl Default for RecordingAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter for RecordingAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        events.extend(self.native_events.iter().cloned());
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.native_events.pop_front()
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

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        if !self.objects.contains_key(&object) {
            return Err(AdapterError::MissingObject(object));
        }
        self.focuses.push(object);
        Ok(true)
    }

    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        if !self.objects.contains_key(&request.object()) {
            return Err(AdapterError::MissingObject(request.object()));
        }
        self.imperatives.push(request.clone());
        match request {
            ImperativeRequest::InitializeWebView2 { completion, .. } => {
                completion.call(Err(IntegrationError::Unavailable));
            }
            ImperativeRequest::RequestSwapChainPanelFrame { completion, .. }
            | ImperativeRequest::SetSwapChain { completion, .. }
            | ImperativeRequest::SetNativeImageSource { completion, .. }
            | ImperativeRequest::SetCompositionChildVisual { completion, .. } => {
                completion.call(Ok(()));
            }
            _ => {}
        }
        Ok(())
    }
}
