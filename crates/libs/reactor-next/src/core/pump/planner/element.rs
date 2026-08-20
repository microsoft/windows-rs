//! Planning for plain [`Element`] trees: native/virtual reconciliation and
//! mounting, independent of [`View`]/component structure.

use super::super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn element_structure_is_empty(element: &Element) -> bool {
        match element.structure() {
            ElementStructureRef::None | ElementStructureRef::Content(None) => true,
            ElementStructureRef::Children(children) => children.is_empty(),
            ElementStructureRef::Virtual(_) => false,
            ElementStructureRef::Content(Some(_)) => false,
        }
    }

    pub(in super::super) fn plan_local_native_state(
        native: &NativeState,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<MountedProps, PumpError> {
        let props_changed = native.desired != parts.props;
        if props_changed || plan.retry_properties {
            parts.props.visit_properties(&mut |property, value| {
                let changed = native.properties.get(&property).map_or_else(
                    || value.is_some(),
                    |current| current != &NativePropertyState::Known(value.clone()),
                );
                if !changed {
                    return;
                }
                let command = match &value {
                    Some(value) => Command::SetProperty {
                        node,
                        property,
                        value: value.clone(),
                    },
                    None => Command::ClearProperty { node, property },
                };
                let command = plan.push(command);
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value,
                });
            });
        }
        Ok(parts.props)
    }

    pub(in super::super) fn reconcile_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let desired_kind = element.kind();
        let compatible = match tree.kind(node)? {
            NodeKind::Native(kind) => kind == desired_kind,
            NodeKind::VirtualCollection => desired_kind == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Self::replace_planned_node(tree, node, element, plan);
        }
        if !plan.retry_properties && Self::node_matches_element(tree, node, &element)? {
            return Ok(node);
        }

        let parts = element.into_parts();
        if tree.kind(node)? == NodeKind::VirtualCollection {
            let ElementStructure::Virtual(items) = parts.structure else {
                return Err(PumpError::StructureUnsupported);
            };
            let old_keys = tree.virtual_model(node)?.keys();
            let keys_changed = old_keys.len() != items.len()
                || old_keys
                    .iter()
                    .zip(items.iter())
                    .any(|(old, new)| old != new.key());
            if keys_changed {
                for child in tree.children(node)?.to_vec() {
                    Self::retire_planned_subtree(tree, child, plan)?;
                }
                let keys = items.iter().map(|item| item.key().clone());
                tree.virtual_model_mut(node)?
                    .update(keys)
                    .map_err(TreeError::from)?;
                tree.update_virtual_items(node, items)?;
                tree.virtual_model_mut(node)?.clear();
                plan.push(Command::ResetVirtualCollection {
                    node,
                    item_count: tree.virtual_items(node)?.len(),
                });
            } else {
                tree.update_virtual_items(node, items)?;
                let realized = tree
                    .children(node)?
                    .iter()
                    .copied()
                    .map(|child| {
                        let key = tree
                            .key(child)?
                            .cloned()
                            .ok_or(PumpError::StructureUnsupported)?;
                        let element = tree.virtual_item(node, &key)?.clone();
                        Ok((child, element))
                    })
                    .collect::<Result<Vec<_>, PumpError>>()?;
                for (child, element) in realized {
                    Self::reconcile_node(tree, child, element, plan)?;
                }
            }
            return Ok(node);
        }
        let NodeKind::Native(kind) = tree.kind(node)? else {
            return Err(PumpError::NotMounted);
        };
        debug_assert_eq!(kind, parts.kind);

        let props_changed = tree.native(node)?.desired != parts.props;
        if props_changed || plan.retry_properties {
            let properties = &tree.native(node)?.properties;
            parts.props.visit_properties(&mut |property, value| {
                let changed = properties.get(&property).map_or_else(
                    || value.is_some(),
                    |native| native != &NativePropertyState::Known(value.clone()),
                );
                if !changed {
                    return;
                }

                let command = match &value {
                    Some(value) => Command::SetProperty {
                        node,
                        property,
                        value: value.clone(),
                    },
                    None => Command::ClearProperty { node, property },
                };
                let command = plan.push(command);
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value,
                });
            });
        }
        if props_changed {
            Self::update_event_states(tree.native_mut(node)?, node, &parts.props, plan)?;
            tree.native_mut(node)?.desired = parts.props;
        }

        let current_children = tree.children(node)?.to_vec();
        match parts.structure {
            ElementStructure::None => {
                if !current_children.is_empty() {
                    return Err(PumpError::StructureUnsupported);
                }
            }
            ElementStructure::Content(content) => match (current_children.as_slice(), content) {
                ([], None) => {}
                ([], Some(content)) => {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
                ([child], None) => {
                    Self::retire_planned_subtree(tree, *child, plan)?;
                }
                ([child], Some(content)) => {
                    Self::reconcile_node(tree, *child, content, plan)?;
                }
                _ => return Err(PumpError::StructureUnsupported),
            },
            ElementStructure::Children(children) => {
                if current_children.len() == children.len()
                    && current_children
                        .iter()
                        .zip(children.iter())
                        .all(|(child, desired)| {
                            tree.key(*child).is_ok_and(|key| key == Some(desired.key()))
                        })
                {
                    let mut replacements = Vec::new();
                    for (index, (child, desired)) in current_children
                        .iter()
                        .copied()
                        .zip(children.iter())
                        .enumerate()
                    {
                        if !plan.retry_properties
                            && Self::node_matches_element(tree, child, desired.element())?
                        {
                        } else {
                            let reconciled =
                                Self::reconcile_node(tree, child, desired.element().clone(), plan)?;
                            if reconciled != child {
                                replacements.push((index, reconciled));
                            }
                        }
                    }
                    if !replacements.is_empty() {
                        let mut children = current_children;
                        for (index, replacement) in replacements {
                            children[index] = replacement;
                        }
                        tree.set_children(node, children)?;
                    }
                    return Ok(node);
                }

                let mut old_keys = Vec::with_capacity(current_children.len());
                let mut nodes = HashMap::with_capacity(current_children.len());
                for child in current_children.iter().copied() {
                    let key = tree
                        .key(child)?
                        .cloned()
                        .ok_or(PumpError::StructureUnsupported)?;
                    old_keys.push(key.clone());
                    nodes.insert(key, child);
                }

                let new_keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                let operations = diff(&old_keys, &new_keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

                let mut elements = children
                    .iter()
                    .map(|child| (child.key().clone(), child.element()))
                    .collect::<HashMap<_, _>>();
                let mut replacements = HashMap::new();
                for key in &new_keys {
                    if let Some(child_node) = nodes.get(key).copied() {
                        let child = elements
                            .remove(key)
                            .ok_or(PumpError::StructureUnsupported)?;
                        let reconciled = if !plan.retry_properties
                            && Self::node_matches_element(tree, child_node, child)?
                        {
                            child_node
                        } else {
                            Self::reconcile_node(tree, child_node, child.clone(), plan)?
                        };
                        if reconciled != child_node {
                            nodes.insert(key.clone(), reconciled);
                            replacements.insert(child_node, reconciled);
                        }
                    }
                }

                if operations.len() >= 256 && operations.len() * 4 > new_keys.len() {
                    let old_key_set = old_keys.iter().cloned().collect::<HashSet<_>>();
                    let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
                    if old_key_set == new_key_set {
                        let order = new_keys
                            .iter()
                            .map(|key| {
                                nodes
                                    .get(key)
                                    .copied()
                                    .ok_or(PumpError::StructureUnsupported)
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        plan.push(Command::ResetChildren { parent: node });
                        for (index, key) in new_keys.iter().enumerate() {
                            let child = nodes
                                .get(key)
                                .copied()
                                .ok_or(PumpError::StructureUnsupported)?;
                            plan.push(Command::InsertChild {
                                parent: node,
                                child,
                                index,
                            });
                        }
                        tree.set_children(node, order)?;
                        return Ok(node);
                    }
                }

                let mut order = current_children
                    .into_iter()
                    .map(|child| replacements.get(&child).copied().unwrap_or(child))
                    .collect::<Vec<_>>();
                for operation in operations {
                    let (key, before, child, moved) = match operation {
                        KeyedOperation::Move { key, before } => {
                            let child = nodes
                                .get(&key)
                                .copied()
                                .ok_or(PumpError::StructureUnsupported)?;
                            let previous = order
                                .iter()
                                .position(|item| *item == child)
                                .ok_or(PumpError::StructureUnsupported)?;
                            order.remove(previous);
                            (key, before, child, true)
                        }
                        KeyedOperation::Insert { key, before } => {
                            let element = elements
                                .remove(&key)
                                .ok_or(PumpError::StructureUnsupported)?;
                            let child = Self::mount_planned_element(
                                tree,
                                Some(node),
                                Some(key.clone()),
                                element.clone(),
                                plan,
                            )?;
                            (key, before, child, false)
                        }
                        KeyedOperation::Remove { key } => {
                            let child =
                                nodes.remove(&key).ok_or(PumpError::StructureUnsupported)?;
                            let previous = order
                                .iter()
                                .position(|item| *item == child)
                                .ok_or(PumpError::StructureUnsupported)?;
                            order.remove(previous);
                            Self::retire_planned_subtree(tree, child, plan)?;
                            continue;
                        }
                    };
                    let index = if let Some(before) = before {
                        let before = nodes
                            .get(&before)
                            .copied()
                            .ok_or(PumpError::StructureUnsupported)?;
                        order
                            .iter()
                            .position(|item| *item == before)
                            .ok_or(PumpError::StructureUnsupported)?
                    } else {
                        order.len()
                    };
                    order.insert(index, child);
                    if moved {
                        plan.push(Command::MoveChild {
                            parent: node,
                            child,
                            index,
                        });
                    } else {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child,
                            index,
                        });
                    }
                    nodes.insert(key, child);
                }
                tree.set_children(node, order)?;
            }
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
        }
        Ok(node)
    }

    fn node_matches_element(
        tree: &Tree,
        node: NodeId,
        element: &Element,
    ) -> Result<bool, PumpError> {
        let kind = tree.kind(node)?;
        let compatible = match kind {
            NodeKind::Native(mounted) => mounted == element.kind(),
            NodeKind::VirtualCollection => element.kind() == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Ok(false);
        }
        if kind == NodeKind::VirtualCollection {
            let ElementStructureRef::Virtual(items) = element.structure() else {
                return Ok(false);
            };
            return Ok(tree.virtual_items(node)? == items);
        }
        if !element.props_match(&tree.native(node)?.desired) {
            return Ok(false);
        }

        let children = tree.children(node)?;
        match element.structure() {
            ElementStructureRef::None => Ok(children.is_empty()),
            ElementStructureRef::Content(content) => match (children, content) {
                ([], None) => Ok(true),
                ([child], Some(content)) => Self::node_matches_element(tree, *child, content),
                _ => Ok(false),
            },
            ElementStructureRef::Children(desired) => {
                if children.len() != desired.len() {
                    return Ok(false);
                }
                for (child, desired) in children.iter().zip(desired) {
                    if tree.key(*child)? != Some(desired.key())
                        || !Self::node_matches_element(tree, *child, desired.element())?
                    {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ElementStructureRef::Virtual(_) => Ok(false),
        }
    }

    fn replace_planned_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node)?.ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node)?.cloned();
        let container = if tree.kind(parent)? == NodeKind::VirtualCollection {
            Some(
                tree.realized_container(parent, node)?
                    .ok_or(PumpError::StructureUnsupported)?,
            )
        } else {
            None
        };
        let index = tree
            .children(parent)?
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        Self::retire_planned_subtree(tree, node, plan)?;
        let replacement = Self::mount_planned_element(tree, Some(parent), key, element, plan)?;
        let mut children = tree.children(parent)?.to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children)?;
        if let Some(container) = container {
            tree.set_realized(parent, container, replacement)?;
            plan.push(Command::AttachRealized {
                collection: parent,
                container,
                child: replacement,
            });
        } else {
            plan.push(Command::InsertChild {
                parent,
                child: replacement,
                index,
            });
        }
        Ok(replacement)
    }

    fn update_event_states(
        native: &mut NativeState,
        node: NodeId,
        desired: &MountedProps,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let mut desired_events = Vec::new();
        desired.visit_events(&mut |event, active| {
            desired_events.push((event, active));
        });
        for (event, active) in desired_events {
            let state = native.events.entry(event).or_insert(EventState {
                revision: 0,
                active: false,
            });
            if state.active != active {
                state.revision = state
                    .revision
                    .checked_add(1)
                    .ok_or(PumpError::RevisionExhausted)?;
                state.active = active;
                if active {
                    plan.push(Command::SubscribeEvent {
                        node,
                        event,
                        revision: state.revision,
                    });
                } else {
                    plan.push(Command::UnsubscribeEvent { node, event });
                }
            }
        }
        Ok(())
    }

    pub(super) fn reconcile_shallow_control(
        tree: &mut Tree,
        node: NodeId,
        control: Element,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if !Self::element_structure_is_empty(&control) {
            return Err(PumpError::StructureUnsupported);
        }
        let parts = control.into_parts();
        Self::reconcile_shallow_parts(tree, node, parts, plan)
    }

    fn reconcile_shallow_parts(
        tree: &mut Tree,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if tree.kind(node)? != NodeKind::Native(parts.kind) {
            return Err(PumpError::StructureUnsupported);
        }
        Self::reconcile_native_state(tree.native_mut(node)?, node, parts, plan)
    }

    fn reconcile_native_state(
        native: &mut NativeState,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let props_changed = native.desired != parts.props;
        if props_changed || plan.retry_properties {
            let properties = &native.properties;
            parts.props.visit_properties(&mut |property, value| {
                let changed = properties.get(&property).map_or_else(
                    || value.is_some(),
                    |native| native != &NativePropertyState::Known(value.clone()),
                );
                if !changed {
                    return;
                }
                let command = match &value {
                    Some(value) => Command::SetProperty {
                        node,
                        property,
                        value: value.clone(),
                    },
                    None => Command::ClearProperty { node, property },
                };
                let command = plan.push(command);
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value,
                });
            });
        }
        if props_changed {
            Self::update_event_states(native, node, &parts.props, plan)?;
            native.desired = parts.props;
        }
        Ok(())
    }

    pub(in super::super) fn mount_planned_element(
        tree: &mut Tree,
        parent: Option<NodeId>,
        key: Option<Key>,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parts = element.into_parts();
        if let ElementStructure::Virtual(items) = parts.structure {
            if parts.kind != MountedKind::ItemsRepeater {
                return Err(PumpError::StructureUnsupported);
            }
            let item_count = items.len();
            let node = tree.insert_virtual_items(plan.identity, parent, key, items)?;
            plan.push(Command::CreateVirtualCollection { node, item_count });
            return Ok(node);
        }
        let node = tree.insert_native(parent, parts.kind, key, parts.props.clone())?;
        plan.push(Command::Create {
            node,
            kind: parts.kind,
        });
        parts.props.visit_properties(&mut |property, value| {
            if let Some(value) = value {
                let command = plan.push(Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                });
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value: Some(value),
                });
            }
        });
        for (event, state) in &tree.native(node)?.events {
            if state.active {
                plan.push(Command::SubscribeEvent {
                    node,
                    event: *event,
                    revision: state.revision,
                });
            }
        }

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
            }
            ElementStructure::Children(children) => {
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        Self::mount_planned_element(tree, Some(node), Some(key), child, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        child,
                        index,
                    });
                }
            }
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
        }
        Ok(node)
    }
}
