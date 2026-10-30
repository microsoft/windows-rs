use super::*;

impl RetainedGraph {
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

    fn mark_retiring(
        &mut self,
        root: ObjectId,
        nodes: Vec<ObjectId>,
        parent: ObjectId,
        relation: RelationId,
    ) {
        for object in &nodes {
            let slot = &mut self.objects[object.index as usize];
            assert_eq!(slot.generation, object.generation);
            assert!(!slot.retiring);
            slot.retiring = true;
        }
        assert!(
            self.retirements
                .insert(
                    root,
                    RetainedRetirement {
                        nodes,
                        parent,
                        relation,
                    }
                )
                .is_none()
        );
    }

    fn complete_retirement(&mut self, root: ObjectId) {
        let Some(retirement) = self.retirements.remove(&root) else {
            return;
        };
        let key = (retirement.parent, retirement.relation);
        if self
            .retirements
            .values()
            .any(|other| (other.parent, other.relation) == key)
        {
            if let Some(children) = self.native_orders.get_mut(&key) {
                children.retain(|child| *child != root);
            }
        } else {
            self.native_orders.remove(&key);
        }
        for object in retirement.nodes {
            self.remove(object);
        }
    }

    pub(super) fn native_children(&self, parent: ObjectId, relation: RelationId) -> NativeChildren {
        let retiring = self
            .retirements
            .iter()
            .filter(|(_, retirement)| {
                retirement.parent == parent && retirement.relation == relation
            })
            .map(|(root, _)| *root)
            .collect::<Vec<_>>();
        let children = if retiring.is_empty() {
            self.children(parent, relation).unwrap().to_vec()
        } else {
            self.native_orders[&(parent, relation)].clone()
        };
        NativeChildren { children, retiring }
    }

    pub(super) fn set_native_children(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        native: NativeChildren,
    ) {
        if native.retiring.is_empty() {
            self.native_orders.remove(&(parent, relation));
        } else {
            self.native_orders
                .insert((parent, relation), native.children);
        }
    }
}

/// A many relation's children in native order. Children fading out keep their native slots until
/// their retirement completes, so native indices count them while the declared (active) indices
/// do not. Adapters apply native indices and moves as given.
pub(super) struct NativeChildren {
    children: Vec<ObjectId>,
    retiring: Vec<ObjectId>,
}

impl NativeChildren {
    /// The native index of the active child at `index`, or the end when `index` is the active
    /// length.
    fn slot(&self, index: usize) -> usize {
        if self.retiring.is_empty() {
            return index;
        }
        self.children
            .iter()
            .enumerate()
            .filter(|(_, child)| !self.retiring.contains(child))
            .nth(index)
            .map_or(self.children.len(), |(slot, _)| slot)
    }

    /// Removes the active child at `index` and returns its native index.
    pub(super) fn remove(&mut self, index: usize, child: ObjectId) -> usize {
        let slot = self.slot(index);
        assert_eq!(self.children.remove(slot), child);
        slot
    }

    /// Inserts a child at active `index` and returns its native index.
    pub(super) fn insert(&mut self, index: usize, child: ObjectId) -> usize {
        let slot = self.slot(index);
        self.children.insert(slot, child);
        slot
    }

    /// Keeps a child in its native slot while it fades out.
    pub(super) fn retire(&mut self, child: ObjectId) {
        debug_assert!(self.children.contains(&child));
        self.retiring.push(child);
    }

    /// Puts the active children in `next` order without moving retiring children, and returns
    /// the moves that produce the new native order.
    pub(super) fn reorder(&mut self, next: &[ObjectId]) -> Vec<Move> {
        let mut active = next.iter().copied();
        let desired = self
            .children
            .iter()
            .map(|child| {
                if self.retiring.contains(child) {
                    *child
                } else {
                    active.next().unwrap()
                }
            })
            .collect::<Vec<_>>();
        assert!(active.next().is_none());
        let moves = reorder_moves(&self.children, &desired, &self.retiring);
        self.children = desired;
        moves
    }

    pub(super) fn children(&self) -> &[ObjectId] {
        &self.children
    }
}

/// Returns moves that turn `previous` into `next`, applied in order: each removes `child` and
/// inserts it before `before`, or at the end. Children outside the longest run that keeps its
/// relative order move; `fixed` children, which keep their native index, never move.
fn reorder_moves(previous: &[ObjectId], next: &[ObjectId], fixed: &[ObjectId]) -> Vec<Move> {
    if previous == next {
        return Vec::new();
    }
    let previous_indices = previous
        .iter()
        .copied()
        .enumerate()
        .map(|(index, child)| (child, index))
        .collect::<HashMap<_, _>>();
    // A child can stay in place only if the same fixed children precede it in both orders.
    // The longest run of such children then includes every fixed child.
    let fixed_before = |order: &[ObjectId]| {
        let mut count = 0;
        order
            .iter()
            .map(|child| {
                let before = count;
                count += usize::from(fixed.contains(child));
                before
            })
            .collect::<Vec<_>>()
    };
    let previous_fixed = fixed_before(previous);
    let next_fixed = fixed_before(next);
    let retained = next
        .iter()
        .enumerate()
        .filter_map(|(next_index, child)| {
            previous_indices
                .get(child)
                .filter(|previous_index| previous_fixed[**previous_index] == next_fixed[next_index])
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
        assert!(!fixed.contains(&child));
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

impl<A: Adapter> Runtime<A> {
    pub(super) fn apply_retirement_completion(&mut self, completion: RetirementCompletion) {
        self.mutations.clear();
        let Some(nodes) = self.graph.retirement_nodes(completion.root) else {
            return;
        };
        self.mutations.push(Mutation::CompleteRetirement {
            root: completion.root,
            nodes: nodes.to_vec(),
        });
        self.graph.complete_retirement(completion.root);
        let batch = self.take_batch(Vec::new());
        self.apply_batch(batch);
    }
}

impl Planner<'_> {
    /// Completes the retirements of an object's children, whose native slots go with it.
    pub(super) fn complete_child_retirements(&mut self, object: ObjectId) {
        for (root, nodes) in self.retained.retirements_for_parent(object) {
            self.mutations
                .push(Mutation::CompleteRetirement { root, nodes });
            self.retained.complete_retirement(root);
        }
    }

    pub(super) fn retire(&mut self, object: ObjectId) {
        self.complete_child_retirements(object);
        if let Some(reference) = self.retained.get_mut(object).reference.take() {
            self.references
                .push(ReferenceChange::Clear { reference, object });
        }
        if self
            .retained
            .get_mut(object)
            .window_title_bar
            .take()
            .is_some()
        {
            self.retained.clear_window_title_bar(object);
            self.mutations
                .push(Mutation::ClearWindowTitleBar { object });
        }
        self.clear_attachments(object);
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

    pub(super) fn retire_with_transition(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        root: ObjectId,
    ) -> bool {
        let Some(transition) = self.retained.get(root).unwrap().exit_transition else {
            return false;
        };
        let contract = relation_contracts(self.retained.get(parent).unwrap().kind)
            .iter()
            .find(|contract| contract.id == relation)
            .unwrap();
        // A retiring child keeps its native slot and stays selectable, so it cannot fade out of a
        // selector's items.
        let kind = self.retained.get(parent).unwrap().kind;
        if contract.cardinality != Cardinality::Many
            || contract.realization != Realization::Owned
            || index_for_items(kind, IndexItems::Relation(relation)).is_some()
            || selection_for_relation(kind, relation).is_some()
        {
            panic!("exit transitions are not supported for children of relation {relation:?}");
        }
        let mut nodes = Vec::new();
        self.collect_subtree_postorder(root, &mut nodes);
        for object in nodes.iter().copied() {
            self.clear_attachments(object);
        }
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
            if retained.window_title_bar.take().is_some() {
                self.retained.clear_window_title_bar(*object);
                self.mutations
                    .push(Mutation::ClearWindowTitleBar { object: *object });
            }
        }
        self.mutations.push(Mutation::Retire {
            root,
            nodes: nodes.clone(),
            parent,
            relation,
            duration: transition.duration(),
        });
        self.retained.mark_retiring(root, nodes, parent, relation);
        true
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
