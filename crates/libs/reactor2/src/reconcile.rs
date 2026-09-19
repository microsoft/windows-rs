use super::*;
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
    SetProperties {
        object: ObjectId,
        set: Rc<[Property]>,
        clear: Rc<[PropertyId]>,
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
    relations: Vec<RetainedRelation>,
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

    pub fn properties(&self, object: ObjectId) -> Option<&[Property]> {
        self.get(object).map(|object| object.properties.as_slice())
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
        let Some(next) = remaining.checked_sub(1) else {
            return Err(GraphError::SizeExceeded);
        };
        *remaining = next;
        let Some(current) = self.get(object) else {
            return Ok(false);
        };
        if current.kind != declaration.kind
            || current.key != declaration.key
            || current.properties.as_slice() != declaration.properties.as_slice()
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
                            if self.matches_declaration(*previous, next, remaining)? => {}
                        _ => return Ok(false),
                    }
                }
                (RetainedRelationValue::Many(previous), Some(RelationValue::Many(next))) => {
                    if previous.len() != next.len() {
                        return Ok(false);
                    }
                    for (previous, next) in previous.iter().zip(next.iter()) {
                        if !self.matches_declaration(*previous, next, remaining)? {
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

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error>;
    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error>;
}

pub struct Runtime<A> {
    graph: RetainedGraph,
    adapter: A,
    mutations: Vec<Mutation>,
    poisoned: bool,
}

impl<A: Adapter> Runtime<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            graph: RetainedGraph::default(),
            adapter,
            mutations: Vec::new(),
            poisoned: false,
        }
    }

    pub fn graph(&self) -> &RetainedGraph {
        &self.graph
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
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
        validate_declaration(&declaration.0).map_err(UpdateError::Graph)?;
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
    fn mount(&mut self, declaration: &Declaration) -> Result<ObjectId, GraphError> {
        let object = self.retained.allocate(RetainedObject {
            kind: declaration.kind,
            key: declaration.key.clone(),
            properties: declaration.properties.clone(),
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
