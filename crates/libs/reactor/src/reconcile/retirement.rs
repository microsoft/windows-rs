use super::*;

impl Planner<'_> {
    pub(super) fn retire(&mut self, object: ObjectId) {
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
}
