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
    pub(crate) call: EventCall,
}

impl EventDispatch {
    pub fn new(object: ObjectId, event: EventId, call: EventCall) -> Self {
        Self {
            object,
            event,
            call,
        }
    }
}

impl EventCall {
    /// The event contract value type of a call. Menu and command bar keys have no event contract,
    /// and `Keys` is only produced by validation.
    fn value_type(&self) -> Option<ValueType> {
        Some(match self {
            Self::Bool(..) => ValueType::Bool,
            Self::Color(..) => ValueType::Color,
            Self::ContentDialogResult(..) => ValueType::ContentDialogResult,
            #[cfg(any(test, feature = "test"))]
            Self::CharacterEventInfo(..) => ValueType::CharacterEventInfo,
            Self::String(..) => ValueType::String,
            Self::F64(..) => ValueType::F64,
            Self::FocusEventInfo(..) => ValueType::FocusEventInfo,
            Self::DragKind(..) => ValueType::DragKind,
            Self::DroppedData(..) => ValueType::DroppedData,
            Self::OptionalBool(..) => ValueType::OptionalBool,
            Self::OptionalDateTime(..) => ValueType::OptionalDateTime,
            Self::OptionalF64(..) => ValueType::OptionalF64,
            Self::OptionalTimeSpan(..) => ValueType::OptionalTimeSpan,
            Self::NavigationViewDisplayMode(..) => ValueType::NavigationViewDisplayMode,
            Self::PointerEventInfo(..) => ValueType::PointerEventInfo,
            #[cfg(any(test, feature = "test"))]
            Self::KeyEventInfo(..) => ValueType::KeyEventInfo,
            Self::Item(..) => ValueType::Key,
            Self::Items(..) => ValueType::Keys,
            Self::Selection(..) => ValueType::Selection,
            Self::SelectionIndex(..) => ValueType::SelectionIndex,
            Self::Unit(..) => ValueType::Unit,
            Self::Key(..) | Self::Keys(..) => return None,
        })
    }

    /// Calls the callback. Validation resolves native items to keys first.
    pub(super) fn call(self) {
        match self {
            Self::Bool(callback, value) => callback.call(value),
            Self::Color(callback, value) => callback.call(value),
            Self::ContentDialogResult(callback, value) => callback.call(value),
            #[cfg(any(test, feature = "test"))]
            Self::CharacterEventInfo(callback, value) => _ = callback.call(value),
            Self::String(callback, value) => callback.call(value),
            Self::F64(callback, value) => callback.call(value),
            Self::FocusEventInfo(callback, value) => callback.call(value),
            Self::DragKind(callback, value) => callback.call(value),
            Self::DroppedData(callback, value) => callback.call(value),
            Self::OptionalBool(callback, value) => callback.call(value),
            Self::OptionalDateTime(callback, value) => callback.call(value),
            Self::OptionalF64(callback, value) => callback.call(value),
            Self::OptionalTimeSpan(callback, value) => callback.call(value),
            Self::NavigationViewDisplayMode(callback, value) => callback.call(value),
            Self::PointerEventInfo(callback, value) => callback.call(value),
            #[cfg(any(test, feature = "test"))]
            Self::KeyEventInfo(callback, value) => _ = callback.call(value),
            Self::Key(callback, value) => callback.call(value),
            Self::Keys(callback, value) => callback.call(value),
            Self::Selection(callback, value) => callback.call(value.value),
            Self::SelectionIndex(callback, value) => callback.call(value),
            Self::Unit(callback) => callback.call(()),
            Self::Item(..) | Self::Items(..) => unreachable!("native items resolve to keys"),
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

    /// Validates a native event dispatch and resolves native items to keys. Returns `None` when
    /// the occurrence no longer applies.
    pub(super) fn validate_event_dispatch(&self, dispatch: EventDispatch) -> Option<EventCall> {
        let EventDispatch {
            object: id,
            event,
            call,
        } = dispatch;
        let object = self.get(id)?;
        let attachments = object.attachments.as_ref();
        let current = match event {
            EventId::MenuItemInvoked => Some(
                &attachments
                    .and_then(|attachments| attachments.menu.as_ref())?
                    .menu
                    .on_click,
            ),
            EventId::CommandInvoked => Some(
                &attachments
                    .and_then(|attachments| attachments.command_bar_flyout.as_ref())?
                    .flyout
                    .on_click,
            ),
            _ => None,
        };
        if let Some(current) = current {
            let EventCall::Key(callback, _) = &call else {
                panic!("invalid payload for event {event:?}");
            };
            return (callback == current).then_some(call);
        }
        let Some(contract) = event_contracts(object.kind)
            .iter()
            .find(|contract| contract.id == event)
        else {
            panic!("invalid event {event:?} for {:?}", object.kind);
        };
        assert!(
            call.value_type() == Some(contract.value),
            "invalid payload for event {event:?}"
        );
        let call = match call {
            EventCall::Item(callback, item) => {
                let child = object.relations.iter().any(|relation| {
                    matches!(&relation.value, RetainedRelationValue::Many(children) if children.contains(&item))
                });
                if !child {
                    return None;
                }
                EventCall::Key(callback, self.get(item)?.key.clone()?)
            }
            EventCall::Items(callback, items) => {
                let Some(contract) = reorder_contracts(object.kind)
                    .iter()
                    .find(|contract| contract.event == event)
                else {
                    panic!("invalid payload for event {event:?}");
                };
                // A stale payload no longer describes the retained order.
                let retained = self.children(id, contract.relation).unwrap_or_default();
                let keys = self
                    .reordered_children(id, contract.relation, &items)
                    .filter(|children| children == retained)?
                    .iter()
                    .map(|child| self.get(*child)?.key.clone())
                    .collect::<Option<Vec<_>>>()?;
                EventCall::Keys(callback, keys)
            }
            EventCall::Selection(
                _,
                SelectionChange {
                    item: Some(item), ..
                },
            ) => {
                let Some(contract) = selection_contract(object.kind) else {
                    panic!("{:?} does not support selection", object.kind);
                };
                let current = contract.relations.iter().any(|relation| {
                    self.children(id, *relation)
                        .is_some_and(|children| children.contains(&item))
                });
                if !current {
                    return None;
                }
                call
            }
            call => call,
        };
        // An occurrence reaches the callback subscribed when it fired, even if a render has since
        // replaced that callback. Removing the subscription drops occurrences still queued.
        let subscribed = object
            .events
            .as_deref()
            .into_iter()
            .flatten()
            .any(|retained| retained.id == event);
        subscribed.then_some(call)
    }
}
