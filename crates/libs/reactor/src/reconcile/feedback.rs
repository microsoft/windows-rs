use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Observation {
    SetProperty {
        object: ObjectId,
        property: Property,
    },
    SetSelection {
        object: ObjectId,
        selected: ObservedSelection,
    },
    /// The native control reordered a relation's children, listed in native order.
    Reorder {
        object: ObjectId,
        relation: RelationId,
        children: Vec<ObjectId>,
    },
}

/// The item a native selector reports as selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservedSelection {
    Cleared,
    Item(ObjectId),
    /// A selected item outside the owner's selection relations, such as a nested
    /// `NavigationViewItem` or the built-in settings item.
    Untracked,
}

impl ObservedSelection {
    fn item(self) -> Option<ObjectId> {
        match self {
            Self::Item(item) => Some(item),
            Self::Cleared | Self::Untracked => None,
        }
    }
}

impl Observation {
    pub(crate) fn object(&self) -> ObjectId {
        match self {
            Self::SetProperty { object, .. }
            | Self::SetSelection { object, .. }
            | Self::Reorder { object, .. } => *object,
        }
    }
}

/// The controlled event that reports a property, directly or through the property it coerces.
pub(crate) fn reporting_event(kind: ObjectType, property: PropertyId) -> Option<EventId> {
    controlled_event(kind, property).or_else(|| {
        coerced_property(kind, property).and_then(|coerced| controlled_event(kind, coerced))
    })
}

/// Native notifications dropped while Reactor writes the state they report.
#[derive(Default)]
pub(crate) struct FeedbackState {
    /// Depth of the nested writes suppressing each event.
    suppressed: HashMap<(ObjectId, EventId), usize>,
}

impl FeedbackState {
    /// Returns whether an event should be reported rather than dropped.
    pub(crate) fn reports(&self, object: ObjectId, event: EventId) -> bool {
        !self.suppressed.contains_key(&(object, event))
    }

    pub(crate) fn suppress(&mut self, object: ObjectId, event: EventId) {
        *self.suppressed.entry((object, event)).or_default() += 1;
    }

    pub(crate) fn release(&mut self, object: ObjectId, event: EventId) {
        let key = (object, event);
        let depth = self.suppressed.get_mut(&key).unwrap();
        *depth -= 1;
        if *depth == 0 {
            self.suppressed.remove(&key);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventDispatch {
    pub object: ObjectId,
    pub event: EventId,
    callback: EventValue,
    pub(super) payload: EventPayload,
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
            (EventValue::Bool(callback), EventPayload::Bool(value)) => callback.call(value),
            (EventValue::Color(callback), EventPayload::Color(value)) => callback.call(value),
            #[cfg(any(test, feature = "test"))]
            (EventValue::CharacterEventInfo(callback), EventPayload::CharacterEventInfo(value)) => {
                callback.call(value);
            }
            (
                EventValue::ContentDialogResult(callback),
                EventPayload::ContentDialogResult(value),
            ) => callback.call(value),
            (EventValue::String(callback), EventPayload::String(value)) => callback.call(value),
            (EventValue::F64(callback), EventPayload::F64(value)) => callback.call(value),
            (EventValue::FocusEventInfo(callback), EventPayload::FocusEventInfo(value)) => {
                callback.call(value);
            }
            (EventValue::DragKind(callback), EventPayload::DragKind(value)) => callback.call(value),
            (EventValue::DroppedData(callback), EventPayload::DroppedData(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalBool(callback), EventPayload::OptionalBool(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalDateTime(callback), EventPayload::OptionalDateTime(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalF64(callback), EventPayload::OptionalF64(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalTimeSpan(callback), EventPayload::OptionalTimeSpan(value)) => {
                callback.call(value);
            }
            (
                EventValue::NavigationViewDisplayMode(callback),
                EventPayload::NavigationViewDisplayMode(value),
            ) => callback.call(value),
            (EventValue::Selection(callback), EventPayload::Selection(value)) => {
                callback.call(value.value);
            }
            (EventValue::PointerEventInfo(callback), EventPayload::PointerEventInfo(value)) => {
                callback.call(value);
            }
            #[cfg(any(test, feature = "test"))]
            (EventValue::KeyEventInfo(callback), EventPayload::KeyEventInfo(value)) => {
                callback.call(value);
            }
            (EventValue::Key(callback), EventPayload::Key(value)) => callback.call(value),
            (EventValue::Keys(callback), EventPayload::Keys(value)) => callback.call(value),
            (EventValue::SelectionIndex(callback), EventPayload::SelectionIndex(value)) => {
                callback.call(value);
            }
            (EventValue::Unit(callback), EventPayload::Unit) => callback.call(()),
            _ => unreachable!(),
        }
    }
}

impl RetainedGraph {
    fn validate_observation(&self, observation: &Observation) -> bool {
        match observation {
            Observation::SetProperty { object, property } => {
                let Some(retained) = self.get(*object) else {
                    return false;
                };
                validate_property(retained.kind, property);
                true
            }
            Observation::SetSelection { object, selected } => {
                let Some(owner) = self.get(*object) else {
                    return false;
                };
                let Some(contract) = selection_contract(owner.kind) else {
                    panic!("{:?} does not support selection", owner.kind);
                };
                selected.item().is_none_or(|selected| {
                    self.kind(selected) == Some(contract.item)
                        && contract.relations.iter().any(|relation| {
                            self.children(*object, *relation)
                                .is_some_and(|children| children.contains(&selected))
                        })
                })
            }
            Observation::Reorder {
                object,
                relation,
                children,
            } => {
                let Some(owner) = self.get(*object) else {
                    return false;
                };
                assert!(
                    reorder_event(owner.kind, *relation).is_some(),
                    "invalid relation {relation:?} for {:?}",
                    owner.kind
                );
                self.reordered_children(*object, *relation, children)
                    .is_some()
            }
        }
    }

    /// Returns the live children in native order when they are a permutation of the retained
    /// children. Native collections may still hold children that are retiring.
    fn reordered_children(
        &self,
        object: ObjectId,
        relation: RelationId,
        native: &[ObjectId],
    ) -> Option<Vec<ObjectId>> {
        let retained = self.children(object, relation).unwrap_or_default();
        let children = native
            .iter()
            .copied()
            .filter(|child| self.get(*child).is_some())
            .collect::<Vec<_>>();
        let unique = children.iter().collect::<HashSet<_>>();
        (children.len() == retained.len()
            && unique.len() == children.len()
            && retained.iter().all(|child| unique.contains(child)))
        .then_some(children)
    }

    /// Applies a native observation. Returns `false` when the paired event must not be dispatched:
    /// a native order that already matches the retained order echoes Reactor's own mutation or
    /// a drop in place, and is not a reorder. Likewise, a selection that matches every child's
    /// retained selection flag reports nothing new, such as a control raising a deferred
    /// notification after Reactor wrote the flags.
    pub(super) fn apply_observation(&mut self, observation: Observation, record: bool) -> bool {
        if !self.validate_observation(&observation) {
            return true;
        }
        match observation {
            Observation::SetProperty { object, property } => {
                let retained = self.get_mut(object);
                let property_id = property.id;
                let baseline = retained
                    .baselines
                    .iter()
                    .position(|baseline| baseline.id == property_id);
                // Observing a native value does not make the declaration own that property.
                let Some(current) = retained
                    .properties
                    .iter()
                    .find(|current| current.id == property_id)
                    .cloned()
                else {
                    match baseline {
                        Some(index) if retained.baselines[index] == property => return false,
                        Some(index) if record => retained.baselines[index] = property,
                        Some(index) => {
                            retained.baselines.swap_remove(index);
                        }
                        None if record => retained.baselines.push(property),
                        None => {}
                    }
                    return true;
                };
                // Native state that matches the declaration is settled, whichever write or
                // notification produced it, so only a departure from the declaration is reported.
                let settled = match baseline {
                    Some(index) if retained.baselines[index] == property => {
                        retained.baselines.swap_remove(index);
                        true
                    }
                    Some(_) => false,
                    None if current == property => return false,
                    None => {
                        retained.baselines.push(current);
                        false
                    }
                };
                retained
                    .properties
                    .upsert(|current| current.id == property_id, property);
                if settled {
                    return false;
                }
            }
            Observation::SetSelection { object, selected } => {
                let contract = selection_contract(self.get(object).unwrap().kind).unwrap();
                let children = contract
                    .relations
                    .iter()
                    .flat_map(|relation| {
                        self.children(object, *relation)
                            .unwrap_or_default()
                            .iter()
                            .copied()
                    })
                    .collect::<Vec<_>>();
                let item = selected.item();
                let matches_retained = selected != ObservedSelection::Untracked
                    && children.iter().all(|child| {
                        self.get(*child).unwrap().properties.iter().any(|property| {
                            property.id == contract.selected_property
                                && property.value == PropertyValue::Bool(Some(*child) == item)
                        })
                    });
                if matches_retained {
                    return false;
                }
                for child in children {
                    let child_object = self.get_mut(child);
                    if child_object
                        .properties
                        .iter()
                        .any(|property| property.id == contract.selected_property)
                    {
                        child_object.properties.upsert(
                            |property| property.id == contract.selected_property,
                            Property {
                                id: contract.selected_property,
                                value: PropertyValue::Bool(Some(child) == item),
                            },
                        );
                    }
                }
            }
            Observation::Reorder {
                object,
                relation,
                children,
            } => {
                let children = self
                    .reordered_children(object, relation, &children)
                    .unwrap();
                if let Some(retained) = self
                    .get_mut(object)
                    .relations
                    .iter_mut()
                    .find(|retained| retained.id == relation)
                {
                    if matches!(&retained.value, RetainedRelationValue::Many(current) if *current == children)
                    {
                        return false;
                    }
                    retained.value = RetainedRelationValue::Many(children);
                }
            }
        }
        true
    }

    /// Validates a native event dispatch and resolves native item payloads to keys.
    pub(super) fn validate_event_dispatch(&self, dispatch: &mut EventDispatch) -> bool {
        let Some(object) = self.get(dispatch.object) else {
            return false;
        };
        if dispatch.event == EventId::MenuItemInvoked {
            let Some(menu) = object
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.menu.as_ref())
            else {
                return false;
            };
            return matches!(
                (&dispatch.callback, &dispatch.payload),
                (EventValue::Key(callback), EventPayload::Key(_))
                    if callback == &menu.menu.on_click
            );
        }
        if dispatch.event == EventId::CommandInvoked {
            let Some(flyout) = object
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.command_bar_flyout.as_ref())
            else {
                return false;
            };
            return matches!(
                (&dispatch.callback, &dispatch.payload),
                (EventValue::Key(callback), EventPayload::Key(_))
                    if callback == &flyout.flyout.on_click
            );
        }
        let Some(contract) = event_contracts(object.kind)
            .iter()
            .find(|contract| contract.id == dispatch.event)
        else {
            panic!("invalid event {:?} for {:?}", dispatch.event, object.kind);
        };
        let valid = match (contract.value, &dispatch.callback, &dispatch.payload) {
            #[cfg(any(test, feature = "test"))]
            (
                ValueType::CharacterEventInfo,
                EventValue::CharacterEventInfo(_),
                EventPayload::CharacterEventInfo(_),
            )
            | (
                ValueType::KeyEventInfo,
                EventValue::KeyEventInfo(_),
                EventPayload::KeyEventInfo(_),
            ) => true,
            (ValueType::Bool, EventValue::Bool(_), EventPayload::Bool(_))
            | (ValueType::Color, EventValue::Color(_), EventPayload::Color(_))
            | (
                ValueType::ContentDialogResult,
                EventValue::ContentDialogResult(_),
                EventPayload::ContentDialogResult(_),
            )
            | (ValueType::String, EventValue::String(_), EventPayload::String(_))
            | (ValueType::F64, EventValue::F64(_), EventPayload::F64(_))
            | (
                ValueType::FocusEventInfo,
                EventValue::FocusEventInfo(_),
                EventPayload::FocusEventInfo(_),
            )
            | (ValueType::DragKind, EventValue::DragKind(_), EventPayload::DragKind(_))
            | (ValueType::DroppedData, EventValue::DroppedData(_), EventPayload::DroppedData(_))
            | (
                ValueType::OptionalBool,
                EventValue::OptionalBool(_),
                EventPayload::OptionalBool(_),
            )
            | (
                ValueType::OptionalDateTime,
                EventValue::OptionalDateTime(_),
                EventPayload::OptionalDateTime(_),
            )
            | (ValueType::OptionalF64, EventValue::OptionalF64(_), EventPayload::OptionalF64(_))
            | (
                ValueType::OptionalTimeSpan,
                EventValue::OptionalTimeSpan(_),
                EventPayload::OptionalTimeSpan(_),
            )
            | (
                ValueType::NavigationViewDisplayMode,
                EventValue::NavigationViewDisplayMode(_),
                EventPayload::NavigationViewDisplayMode(_),
            )
            | (ValueType::Selection, EventValue::Selection(_), EventPayload::Selection(_))
            | (
                ValueType::PointerEventInfo,
                EventValue::PointerEventInfo(_),
                EventPayload::PointerEventInfo(_),
            )
            | (
                ValueType::SelectionIndex,
                EventValue::SelectionIndex(_),
                EventPayload::SelectionIndex(_),
            )
            | (ValueType::Key, EventValue::Key(_), EventPayload::Item(_))
            | (ValueType::Keys, EventValue::Keys(_), EventPayload::Items(_))
            | (ValueType::Unit, EventValue::Unit(_), EventPayload::Unit) => true,
            _ => false,
        };
        assert!(valid, "invalid payload for event {:?}", dispatch.event);
        if let EventPayload::Selection(selection) = &dispatch.payload
            && let Some(item) = selection.item
        {
            let Some(contract) = selection_contract(object.kind) else {
                panic!("{:?} does not support selection", object.kind);
            };
            if !contract.relations.iter().any(|relation| {
                self.children(dispatch.object, *relation)
                    .is_some_and(|children| children.contains(&item))
            }) {
                return false;
            }
        }
        let resolved = match &dispatch.payload {
            EventPayload::Item(item) => {
                let child = object.relations.iter().any(|relation| {
                    matches!(&relation.value, RetainedRelationValue::Many(children) if children.contains(item))
                });
                let Some(key) = child.then(|| self.get(*item)?.key.clone()).flatten() else {
                    return false;
                };
                Some(EventPayload::Key(key))
            }
            EventPayload::Items(items) => {
                let Some(contract) = reorder_contracts(object.kind)
                    .iter()
                    .find(|contract| contract.event == dispatch.event)
                else {
                    panic!("invalid payload for event {:?}", dispatch.event);
                };
                // A stale payload no longer describes the retained order.
                let retained = self
                    .children(dispatch.object, contract.relation)
                    .unwrap_or_default();
                let Some(keys) = self
                    .reordered_children(dispatch.object, contract.relation, items)
                    .filter(|children| children == retained)
                    .and_then(|children| {
                        children
                            .iter()
                            .map(|child| self.get(*child)?.key.clone())
                            .collect::<Option<Vec<_>>>()
                    })
                else {
                    return false;
                };
                Some(EventPayload::Keys(keys))
            }
            _ => None,
        };
        // An occurrence reaches the callback subscribed when it fired, even if a render has since
        // replaced that callback. Removing the subscription drops occurrences still queued.
        let subscribed = object
            .events
            .as_deref()
            .into_iter()
            .flatten()
            .any(|event| event.id == dispatch.event);
        if subscribed && let Some(resolved) = resolved {
            dispatch.payload = resolved;
        }
        subscribed
    }
}
