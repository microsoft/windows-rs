use super::*;

impl RetainedGraph {
    pub(super) fn clear_references(&self, endpoint: &ReferenceEndpoint) {
        #[cfg(test)]
        self.full_reference_scans
            .0
            .set(self.full_reference_scans.0.get() + 1);
        for (index, slot) in self.objects.iter().enumerate() {
            let Some(object) = slot.object.as_ref() else {
                continue;
            };
            if let Some(reference) = object.reference.as_ref()
                && let Ok(index) = u32::try_from(index)
            {
                reference.clear(
                    endpoint,
                    ObjectId {
                        index,
                        generation: slot.generation,
                    },
                );
            }
        }
    }

    pub fn root(&self) -> Option<ObjectId> {
        self.root
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

    pub(super) fn get(&self, object: ObjectId) -> Option<&RetainedObject> {
        let slot = self.objects.get(object.index as usize)?;
        (slot.generation == object.generation && !slot.retiring)
            .then_some(slot.object.as_ref())
            .flatten()
    }

    /// Returns whether the adapter still owns the object, including during an exit transition.
    pub(super) fn is_allocated(&self, object: ObjectId) -> bool {
        self.objects
            .get(object.index as usize)
            .is_some_and(|slot| slot.generation == object.generation && slot.object.is_some())
    }

    pub(super) fn get_mut(&mut self, object: ObjectId) -> &mut RetainedObject {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        assert!(!slot.retiring);
        slot.object.as_mut().unwrap()
    }

    pub(super) fn relation(
        &self,
        object: ObjectId,
        relation: RelationId,
    ) -> Option<&RetainedRelation> {
        self.get(object)?
            .relations
            .iter()
            .find(|state| state.id == relation)
    }

    pub(super) fn matches_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> bool {
        self.matches_declaration_inner(object, declaration, remaining, true)
    }

    pub(super) fn matches_subtree_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> bool {
        self.matches_declaration_inner(object, declaration, remaining, false)
    }

    fn matches_declaration_inner(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
        compare_key: bool,
    ) -> bool {
        let Some(next) = remaining.checked_sub(1) else {
            panic!("declaration exceeds the maximum of {MAX_OBJECTS} objects");
        };
        *remaining = next;
        let Some(current) = self.get(object) else {
            return false;
        };
        if current.kind != declaration.kind
            || (compare_key && current.key != declaration.key)
            || current.reference != declaration.reference
            || current.exit_transition != declaration.exit_transition
            || current.window_title_bar != declaration.window_title_bar
            || current.properties.as_slice() != declaration.properties.as_slice()
            || retained_events(&current.events) != declaration.events.as_slice()
            // A baseline for a declared property records an unsettled departure; the update
            // path settles it even when the app accepted the native value.
            || current.baselines.iter().any(|baseline| {
                declaration
                    .properties
                    .iter()
                    .any(|property| property.id == baseline.id)
            })
        {
            return false;
        }
        let current_tooltip = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref());
        let declared_tooltip = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref());
        match (current_tooltip, declared_tooltip) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.placement == declared.placement
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration(),
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        let current_menu = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.menu.as_ref());
        let declared_menu = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.menu.as_ref());
        match (current_menu, declared_menu) {
            (None, None) => {}
            (Some(current), Some(declared)) if current.menu == declared.menu => {}
            _ => return false,
        }
        let current_command_bar_flyout = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.command_bar_flyout.as_ref());
        let declared_command_bar_flyout = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.command_bar_flyout.as_ref());
        match (current_command_bar_flyout, declared_command_bar_flyout) {
            (None, None) => {}
            (Some(current), Some(declared)) if current.flyout == declared.flyout => {}
            _ => return false,
        }
        let current_flyout = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref());
        let declared_flyout = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref());
        match (current_flyout, declared_flyout) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.placement == declared.placement
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration(),
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        let current_dialog = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref());
        let declared_dialog = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref());
        match (current_dialog, declared_dialog) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.open == declared.open
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration,
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        if let Some(current_virtual) = &current.virtual_items {
            let declared = declaration.virtual_items.as_ref();
            if declared.map(|declared| &declared.items) != Some(&current_virtual.items)
                || declared
                    .and_then(|declared| declared.owner)
                    .or(declaration.component)
                    != current_virtual.owner
            {
                return false;
            }
        } else if declaration.virtual_items.is_some() {
            return false;
        }
        let contracts = relation_contracts(current.kind);
        for (index, relation) in declaration.relations.iter().enumerate() {
            if !contracts.iter().any(|contract| contract.id == relation.id)
                || declaration.relations.as_slice()[..index]
                    .iter()
                    .any(|previous| previous.id == relation.id)
            {
                return false;
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
                                next.as_object(),
                                remaining,
                                true,
                            ) => {}
                        _ => return false,
                    }
                }
                (RetainedRelationValue::Many(previous), Some(RelationValue::Many(next))) => {
                    if previous.len() != next.len() {
                        return false;
                    }
                    for (previous, next) in previous.iter().zip(next.iter()) {
                        if !self.matches_declaration_inner(
                            *previous,
                            next.as_object(),
                            remaining,
                            true,
                        ) {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }
        true
    }

    pub(super) fn relation_mut(
        &mut self,
        object: ObjectId,
        relation: RelationId,
    ) -> &mut RetainedRelation {
        self.get_mut(object)
            .relations
            .iter_mut()
            .find(|state| state.id == relation)
            .unwrap()
    }

    pub(super) fn set_window_title_bar(&mut self, object: ObjectId, height: WindowTitleBarHeight) {
        assert!(
            self.window_title_bar
                .is_none_or(|(current, _)| current == object),
            "window title bar declared more than once"
        );
        self.window_title_bar = Some((object, height));
    }

    pub(super) fn clear_window_title_bar(&mut self, object: ObjectId) {
        if self
            .window_title_bar
            .is_some_and(|(current, _)| current == object)
        {
            self.window_title_bar = None;
        }
    }

    pub(super) fn allocate(&mut self, object: RetainedObject) -> ObjectId {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.objects[index as usize];
            slot.object = Some(object);
            slot.retiring = false;
            ObjectId {
                index,
                generation: slot.generation,
            }
        } else {
            let index = self.objects.len().try_into().unwrap();
            self.objects.push(RetainedSlot {
                generation: 0,
                object: Some(object),
                retiring: false,
            });
            ObjectId {
                index,
                generation: 0,
            }
        }
    }

    pub(super) fn remove(&mut self, object: ObjectId) {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        slot.object = None;
        slot.retiring = false;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(object.index);
    }
}
