use super::*;

impl Planner<'_> {
    pub(super) fn mount_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return;
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
                    .map(|child| self.mount(child.as_object()));
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
                    let child = self.mount(child.as_object());
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
    }

    /// Applies a declaration's index property after the items it indexes, since native controls
    /// reject an index past their items. An index out of range is applied as no selection. Native
    /// controls also move or drop the selection when the items change, so the index is applied
    /// again after Reactor changes them.
    pub(super) fn apply_index(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        previous: Option<&Property>,
        items_changed: bool,
    ) {
        let Some(contract) = index_contract(declaration.kind) else {
            return;
        };
        let declared = declaration
            .properties
            .as_slice()
            .iter()
            .find(|property| property.id == contract.property);
        let (set, clear): (Rc<[Property]>, Rc<[PropertyId]>) = match declared {
            Some(declared) if items_changed || Some(declared) != previous => {
                let len = match contract.items {
                    IndexItems::Relation(relation) => self
                        .retained
                        .children(object, relation)
                        .map_or(0, <[_]>::len),
                    IndexItems::Property(items) => declaration
                        .properties
                        .as_slice()
                        .iter()
                        .find_map(|property| match &property.value {
                            PropertyValue::StringList(values) if property.id == items => {
                                Some(values.len())
                            }
                            _ => None,
                        })
                        .unwrap_or(0),
                };
                let value = match declared.value {
                    PropertyValue::SelectionIndex(Some(index)) if index < len => {
                        declared.value.clone()
                    }
                    _ if contract.requires_selection && len > 0 => {
                        PropertyValue::SelectionIndex(Some(0))
                    }
                    _ => PropertyValue::SelectionIndex(None),
                };
                let property = Property {
                    id: contract.property,
                    value,
                };
                (Rc::from([property]), Rc::from([]))
            }
            None if previous.is_some() && !contract.requires_selection => {
                (Rc::from([]), Rc::from([contract.property]))
            }
            _ => return,
        };
        self.mutations
            .push(Mutation::SetProperties { object, set, clear });
    }

    pub(super) fn reconcile_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return;
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
                let desired = desired.map(DeclaredNode::as_object);
                let next = match (previous, desired) {
                    (Some(previous), Some(desired))
                        if self.retained.get(previous).unwrap().kind == desired.kind =>
                    {
                        Some(self.reconcile_object(previous, desired))
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
                                panic!(
                                    "exit transitions are not supported for children of relation {:?}",
                                    contract.id
                                );
                            }
                            self.mutations.push(Mutation::Detach {
                                parent: object,
                                relation: contract.id,
                                child: previous,
                            });
                            self.retire(previous);
                        }
                        let next = desired.map(|desired| self.mount(desired));
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
                    self.reconcile_keyed_many(object, contract.id, relation);
                }
                Identity::Positional => {
                    self.reconcile_positional_many(object, contract.id, relation);
                }
            },
        }
    }

    fn reconcile_keyed_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) {
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
                    let desired = desired.as_object();
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
                self.reconcile_object(previous, desired.as_object());
            }
            return;
        }
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Vec<_>>();
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
            if !self.retire_with_transition(object, relation_id, child) {
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
                self.reconcile_object(previous, desired)
            } else {
                let child = self.mount(desired);
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
            let stable = longest_increasing_positions(&retained, next.len());
            let mut moves = Vec::new();

            for next_index in (0..next.len()).rev() {
                if stable[next_index] {
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

        fn longest_increasing_positions(sequence: &[(usize, usize)], len: usize) -> Vec<bool> {
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

            let mut positions = vec![false; len];
            let mut cursor = tails.last().copied();
            while let Some(sequence_index) = cursor {
                positions[sequence[sequence_index].0] = true;
                cursor = predecessors[sequence_index];
            }
            positions
        }
    }

    fn reconcile_positional_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) {
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
                    if self.retained.get(*previous).unwrap().kind != desired.as_object().kind {
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
                self.reconcile_object(previous, desired.as_object());
            }
            return;
        }
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Vec<_>>();
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut next = Vec::with_capacity(desired.len());
        for (index, desired) in desired.iter().copied().enumerate() {
            let child = if let Some(previous) = previous.get(index).copied() {
                if self.retained.get(previous).unwrap().kind == desired.kind {
                    self.reconcile_object(previous, desired)
                } else {
                    if !self.retire_with_transition(object, relation_id, previous) {
                        self.mutations.push(Mutation::Remove {
                            parent: object,
                            relation: relation_id,
                            child: previous,
                            index,
                        });
                        self.retire(previous);
                    }
                    let child = self.mount(desired);
                    self.mutations.push(Mutation::Insert {
                        parent: object,
                        relation: relation_id,
                        child,
                        index,
                    });
                    child
                }
            } else {
                let child = self.mount(desired);
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
            if !self.retire_with_transition(object, relation_id, child) {
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
    }
}
