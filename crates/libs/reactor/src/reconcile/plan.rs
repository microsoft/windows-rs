use super::*;

impl Planner<'_> {
    pub(super) fn replace_object(&mut self, object: ObjectId, declaration: &Declaration) {
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
        let previous_tooltip = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.tooltip.take());
        if let Some(tooltip) = &previous_tooltip {
            self.mutations.push(Mutation::SetTooltip {
                target: object,
                tooltip: None,
                placement: tooltip.placement,
            });
        }
        let previous_flyout = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.flyout.take());
        if let Some(flyout) = &previous_flyout {
            self.mutations.push(Mutation::SetFlyout {
                target: object,
                content: None,
                placement: flyout.placement,
            });
        }
        let previous_menu = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.menu.take());
        if let Some(menu) = &previous_menu {
            self.mutations.push(Mutation::SetMenu {
                target: object,
                menu: None,
                revision: menu.revision.wrapping_add(1),
            });
        }
        let previous_command_bar_flyout = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.command_bar_flyout.take());
        if let Some(flyout) = &previous_command_bar_flyout {
            self.mutations.push(Mutation::SetCommandBarFlyout {
                target: object,
                flyout: None,
                revision: flyout.revision.wrapping_add(1),
            });
        }
        let previous_content_dialog = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.content_dialog.take());
        if let Some(dialog) = &previous_content_dialog {
            if dialog.open {
                self.mutations.push(Mutation::SetContentDialogOpen {
                    dialog: dialog.object,
                    open: false,
                });
            }
            self.mutations.push(Mutation::SetContentDialog {
                owner: object,
                dialog: None,
            });
        }
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
        // Ending the replaced control's subscriptions drops its pending occurrences, which would
        // otherwise reach the replacement through the preserved ID.
        if let Some(events) = retained.events.take() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from([]),
                clear: events.iter().map(|event| event.id).collect(),
            });
        }
        if let Some(reference) = &declaration.reference {
            self.references.push(ReferenceChange::Set {
                reference: reference.clone(),
                object,
            });
        }
        let virtual_items = virtualization::retain_virtual_items(declaration);
        *self.retained.get_mut(object) = RetainedObject {
            kind: declaration.kind,
            key,
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            window_title_bar: declaration.window_title_bar,
            attachments: None,
            properties: declaration.properties.clone(),
            baselines: Vec::new(),
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
        let set = properties_before_relations(declaration.kind, declaration.properties.as_slice());
        if !set.is_empty() {
            self.mutations.push(Mutation::SetProperties {
                object,
                set,
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
            self.mount_relation(object, declaration, contract);
        }
        self.apply_index(object, declaration, None, false);
        if let Some(height) = declaration.window_title_bar {
            self.retained.set_window_title_bar(object, height);
            self.mutations
                .push(Mutation::SetWindowTitleBar { object, height });
        }
        self.reconcile_tooltip(
            object,
            previous_tooltip,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.tooltip.as_ref()),
            true,
        );
        self.reconcile_flyout(
            object,
            previous_flyout,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.flyout.as_ref()),
            true,
        );
        self.reconcile_menu(
            object,
            previous_menu,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.menu.as_ref()),
            true,
        );
        self.reconcile_command_bar_flyout(
            object,
            previous_command_bar_flyout,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.command_bar_flyout.as_ref()),
            true,
        );
        self.reconcile_content_dialog(
            object,
            previous_content_dialog,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.content_dialog.as_ref()),
            true,
        );
    }

    pub(super) fn mount(&mut self, declaration: &Declaration) -> ObjectId {
        let virtual_items = virtualization::retain_virtual_items(declaration);
        let object = self.retained.allocate(RetainedObject {
            kind: declaration.kind,
            key: declaration.key.clone(),
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            window_title_bar: declaration.window_title_bar,
            attachments: None,
            properties: declaration.properties.clone(),
            baselines: Vec::new(),
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
        });
        if let Some(height) = declaration.window_title_bar {
            self.retained.set_window_title_bar(object, height);
        }
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
        let set = properties_before_relations(declaration.kind, declaration.properties.as_slice());
        if !set.is_empty() {
            self.mutations.push(Mutation::SetProperties {
                object,
                set,
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
            self.mount_relation(object, declaration, contract);
        }
        self.apply_index(object, declaration, None, false);
        if let Some(height) = declaration.window_title_bar {
            self.mutations
                .push(Mutation::SetWindowTitleBar { object, height });
        }
        if let Some(attachments) = &declaration.attachments {
            if let Some(tooltip) = &attachments.tooltip {
                self.mount_tooltip(object, tooltip);
            }
            if let Some(flyout) = &attachments.flyout {
                self.mount_flyout(object, flyout);
            }
            if let Some(menu) = &attachments.menu {
                self.mount_menu(object, menu);
            }
            if let Some(flyout) = &attachments.command_bar_flyout {
                self.mount_command_bar_flyout(object, flyout);
            }
            if let Some(dialog) = &attachments.content_dialog {
                self.mount_content_dialog(object, dialog);
            }
        }
        object
    }

    pub(super) fn reconcile_object(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
    ) -> ObjectId {
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
        let previous_title_bar = self.retained.get(object).unwrap().window_title_bar;
        if previous_title_bar != declaration.window_title_bar {
            match (previous_title_bar, declaration.window_title_bar) {
                (Some(_), None) => {
                    self.retained.clear_window_title_bar(object);
                    self.mutations
                        .push(Mutation::ClearWindowTitleBar { object });
                }
                (_, Some(height)) => {
                    self.retained.set_window_title_bar(object, height);
                    self.mutations
                        .push(Mutation::SetWindowTitleBar { object, height });
                }
                (None, None) => {}
            }
            self.retained.get_mut(object).window_title_bar = declaration.window_title_bar;
        }
        if self.retained.get(object).unwrap().attachments.is_some()
            || declaration.attachments.is_some()
        {
            let previous_tooltip = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.tooltip.take());
            self.reconcile_tooltip(
                object,
                previous_tooltip,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.tooltip.as_ref()),
                false,
            );
            let previous_flyout = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.flyout.take());
            self.reconcile_flyout(
                object,
                previous_flyout,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.flyout.as_ref()),
                false,
            );
            let previous_menu = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.menu.take());
            self.reconcile_menu(
                object,
                previous_menu,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.menu.as_ref()),
                false,
            );
            let previous_command_bar_flyout = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.command_bar_flyout.take());
            self.reconcile_command_bar_flyout(
                object,
                previous_command_bar_flyout,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.command_bar_flyout.as_ref()),
                false,
            );
            let previous_content_dialog = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.content_dialog.take());
            self.reconcile_content_dialog(
                object,
                previous_content_dialog,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.content_dialog.as_ref()),
                false,
            );
        }
        {
            let RetainedObject {
                properties: previous,
                baselines,
                ..
            } = self.retained.get_mut(object);
            if !baselines.is_empty() {
                baselines.retain(|baseline| {
                    !previous.iter().any(|property| property.id == baseline.id)
                        && !declaration
                            .properties
                            .iter()
                            .any(|property| property.id == baseline.id)
                });
            }
        }
        let properties = declaration.properties.as_slice();
        let index = index_contract(declaration.kind);
        let (previous_index, items_property_changed) = {
            let previous = &self.retained.get(object).unwrap().properties;
            let find = |properties: &[Property], id| {
                properties
                    .iter()
                    .find(|property| property.id == id)
                    .cloned()
            };
            match index {
                Some(contract) => (
                    find(previous.as_slice(), contract.property),
                    match contract.items {
                        IndexItems::Property(items) => {
                            find(previous.as_slice(), items) != find(properties, items)
                        }
                        IndexItems::Relation(_) => false,
                    },
                ),
                None => (None, false),
            }
        };
        let index = index.map(|contract| contract.property);
        if self.retained.get(object).unwrap().properties.as_slice() != properties {
            let previous = self.retained.get(object).unwrap().properties.clone();
            let changed = |property: &Property| {
                previous.iter().find(|current| current.id == property.id) != Some(property)
            };
            let clear = previous
                .iter()
                .filter(|property| {
                    Some(property.id) != index
                        && !properties.iter().any(|current| current.id == property.id)
                })
                .map(|property| property.id)
                .collect::<Rc<[_]>>();
            // A native control coerces a value to its range without always reporting it, so a
            // declared value is written again after any change to a property that coerces it.
            let coerced = properties
                .iter()
                .filter(|property| changed(property))
                .map(|property| property.id)
                .chain(clear.iter().copied())
                .filter_map(|id| coerced_property(declaration.kind, id))
                .collect::<Vec<_>>();
            let set = properties
                .iter()
                .filter(|property| {
                    Some(property.id) != index
                        && (changed(property) || coerced.contains(&property.id))
                })
                .cloned()
                .collect::<Rc<[_]>>();
            self.retained.get_mut(object).properties = declaration.properties.clone();
            if !set.is_empty() || !clear.is_empty() {
                self.mutations
                    .push(Mutation::SetProperties { object, set, clear });
            }
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
        self.reconcile_virtual_items(object, declaration);
        let start = self.mutations.len();
        for contract in relation_contracts(declaration.kind) {
            self.reconcile_relation(object, declaration, contract);
        }
        let items_changed = items_property_changed
            || index_contract(declaration.kind).is_some_and(|contract| {
                self.mutations[start..]
                    .iter()
                    .any(|mutation| match mutation {
                        Mutation::Insert {
                            parent, relation, ..
                        }
                        | Mutation::Remove {
                            parent, relation, ..
                        }
                        | Mutation::Reorder {
                            parent, relation, ..
                        } => *parent == object && contract.items == IndexItems::Relation(*relation),
                        _ => false,
                    })
            });
        self.apply_index(object, declaration, previous_index.as_ref(), items_changed);
        object
    }
}
