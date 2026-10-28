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

    fn mark_retiring(&mut self, root: ObjectId, nodes: Vec<ObjectId>, parent: ObjectId) {
        for object in &nodes {
            let slot = &mut self.objects[object.index as usize];
            assert_eq!(slot.generation, object.generation);
            assert!(!slot.retiring);
            slot.retiring = true;
        }
        assert!(
            self.retirements
                .insert(root, RetainedRetirement { nodes, parent })
                .is_none()
        );
    }

    fn complete_retirement(&mut self, root: ObjectId) {
        let Some(retirement) = self.retirements.remove(&root) else {
            return;
        };
        for object in retirement.nodes {
            self.remove(object);
        }
    }
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
        self.apply_plan(Vec::new(), || {});
        self.mutations.clear();
    }
}

impl Planner<'_> {
    pub(super) fn retire(&mut self, object: ObjectId) {
        for (root, nodes) in self.retained.retirements_for_parent(object) {
            self.mutations
                .push(Mutation::CompleteRetirement { root, nodes });
            self.retained.complete_retirement(root);
        }
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
        self.clear_content_dialog(object);
        self.clear_command_bar_flyout(object);
        self.clear_menu(object);
        self.clear_flyout(object);
        self.clear_tooltip(object);
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
            self.clear_content_dialog(object);
            self.clear_command_bar_flyout(object);
            self.clear_menu(object);
            self.clear_flyout(object);
            self.clear_tooltip(object);
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
        self.retained.mark_retiring(root, nodes, parent);
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
