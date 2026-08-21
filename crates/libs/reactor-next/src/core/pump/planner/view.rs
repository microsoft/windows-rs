//! Planning for [`View`] trees: component, fragment, and keyed-children
//! reconciliation and mounting, built on top of `element` and `topology`.

use super::super::*;
use super::topology::NativeAttachment;

impl<R: NativeRuntime> Pump<R> {
    pub(in super::super) fn reconcile_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        match view.into_kind() {
            ViewKind::Native(element) => {
                if matches!(element.structure(), ElementStructureRef::Virtual(_)) {
                    return Self::reconcile_virtual_collection(
                        tree, node, element, components, changes, plan,
                    );
                }
                if tree.kind(node)? != NodeKind::Native(element.kind())
                    || !tree.children(node)?.is_empty()
                    || !Self::element_structure_is_empty(&element)
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Native(element)),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_node(tree, node, element, plan)
            }
            ViewKind::Component(component) => {
                if tree.kind(node)? != NodeKind::Component
                    || tree.component_type(node)? != component.component_type()
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Component(component)),
                        components,
                        changes,
                        plan,
                    );
                }
                let token = components.token(tree.component_scope(node)?)?;
                let changed = component.apply_props(components, token)?;
                if changed {
                    changes.touched.insert(token);
                }
                if changed || changes.retry.contains(&token) {
                    if changes.deferred.contains(&token) {
                        Ok(node)
                    } else {
                        Self::recompose_component(tree, node, token, components, changes, plan)?;
                        Ok(node)
                    }
                } else {
                    Ok(node)
                }
            }
            ViewKind::Fragment(children) => {
                if tree.kind(node)? != NodeKind::Fragment {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Fragment(children)),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_fragment(tree, node, &children, components, changes, plan)?;
                Ok(node)
            }
            ViewKind::Provider { provision, child } => {
                if tree.kind(node)? != NodeKind::Provider {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Provider { provision, child }),
                        components,
                        changes,
                        plan,
                    );
                }
                let previous = tree.provision(node)?.clone();
                let affected = if previous != provision {
                    let mut affected = components
                        .context_consumers(ContextDependency {
                            id: previous.id,
                            provider: Some(node),
                        })
                        .collect::<HashSet<_>>();
                    if previous.id != provision.id {
                        for scope in components.context_consumers_for_id(provision.id) {
                            let consumer = tree
                                .component_node(scope)?
                                .ok_or(PumpError::StructureUnsupported)?;
                            if !tree.is_descendant_of(consumer, node)? {
                                continue;
                            }
                            let token = components.token(scope)?;
                            let resolved = components
                                .context_dependencies(token)?
                                .and_then(|dependencies| {
                                    dependencies
                                        .iter()
                                        .find(|dependency| dependency.id == provision.id)
                                })
                                .and_then(|dependency| dependency.provider);
                            let shadowed = match resolved {
                                Some(provider) => tree.is_descendant_of(provider, node)?,
                                None => false,
                            };
                            if !shadowed {
                                affected.insert(scope);
                            }
                        }
                    }
                    tree.set_provision(node, provision)?;
                    Some(affected)
                } else {
                    None
                };
                let [current] = tree.children(node)? else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::reconcile_planned_view(
                    tree,
                    *current,
                    View::from_kind(*child),
                    components,
                    changes,
                    plan,
                )?;
                if let Some(affected) = affected {
                    let mut ordered = Vec::with_capacity(affected.len());
                    for scope in affected {
                        if let Some(consumer) = tree.component_node(scope)?
                            && tree.is_descendant_of(consumer, node)?
                        {
                            ordered.push((tree.depth(consumer)?, consumer, scope));
                        }
                    }
                    ordered.sort_unstable_by_key(|(depth, consumer, _)| (*depth, *consumer));
                    for (_, _, scope) in ordered {
                        if let Some(consumer) = tree.component_node(scope)?
                            && tree.is_descendant_of(consumer, node)?
                        {
                            Self::recompose_component(
                                tree,
                                consumer,
                                components.token(scope)?,
                                components,
                                changes,
                                plan,
                            )?;
                        }
                    }
                }
                Ok(node)
            }
            ViewKind::Slots {
                control,
                slots: desired,
            } => {
                if !Self::control_has_role(control.kind(), ControlRole::Slots) {
                    return Err(PumpError::StructureUnsupported);
                }
                let slot_ids = slots(control.kind());
                let structure_matches = tree.kind(node)? == NodeKind::Native(control.kind())
                    && tree.children(node)?.len() == slot_ids.len()
                    && tree
                        .children(node)?
                        .iter()
                        .zip(slot_ids)
                        .all(|(node, slot)| tree.kind(*node) == Ok(NodeKind::NamedSlot(*slot)));
                if !structure_matches {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Slots {
                            control,
                            slots: desired,
                        }),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let mut desired = collect_desired_slots(desired)?;
                let slot_ids = slots(node_kind(tree, node)?);
                if desired.keys().any(|slot| !slot_ids.contains(slot)) {
                    return Err(PumpError::StructureUnsupported);
                }
                let children = tree.children(node)?.to_vec();
                if children.len() != slot_ids.len() {
                    return Err(PumpError::StructureUnsupported);
                }
                for (slot_node, slot) in children.into_iter().zip(slot_ids.iter().copied()) {
                    if tree.kind(slot_node)? != NodeKind::NamedSlot(slot) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    let [child] = tree.children(slot_node)? else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    let view = desired.remove(&slot).unwrap_or_else(View::empty);
                    let child = Self::reconcile_planned_view(
                        tree, *child, view, components, changes, plan,
                    )?;
                    if Self::native_roots(tree, child)?.len() > 1 {
                        return Err(PumpError::StructureUnsupported);
                    }
                }
                Ok(node)
            }
            ViewKind::Content { control, content } => {
                if !Self::control_has_role(control.kind(), ControlRole::Content) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Content { control, content }),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let [child] = tree.children(node)? else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::reconcile_planned_view(
                    tree,
                    *child,
                    View::from_kind(*content),
                    components,
                    changes,
                    plan,
                )?;
                Ok(node)
            }
            ViewKind::Children { control, children } => {
                if !Self::control_has_role(control.kind(), ControlRole::Children) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Children { control, children }),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let old_native = Self::native_children(tree, node)?;
                let current = tree.children(node)?.to_vec();
                let mut requires_sync = current.iter().any(|child| {
                    Self::native_roots(tree, *child).map_or(true, |roots| roots.len() != 1)
                });
                let old_keys = current
                    .iter()
                    .map(|child| {
                        tree.key(*child)?
                            .cloned()
                            .ok_or(PumpError::StructureUnsupported)
                    })
                    .collect::<Result<Vec<_>, PumpError>>()?;
                let new_keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                let operations = diff(&old_keys, &new_keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
                let mut nodes = old_keys
                    .iter()
                    .cloned()
                    .zip(current.iter().copied())
                    .collect::<HashMap<_, _>>();

                for (key, child) in old_keys.iter().zip(current.iter().copied()) {
                    if !new_key_set.contains(key) {
                        Self::collect_retired_components(tree, child, components, changes)?;
                        Self::retire_planned_subtree(tree, child, plan)?;
                    }
                }
                for child in children.iter() {
                    if let Some(child_node) = nodes.get(child.key()).copied() {
                        let old_roots = Self::native_roots(tree, child_node)?;
                        let reconciled = Self::reconcile_planned_view(
                            tree,
                            child_node,
                            child.view().clone(),
                            components,
                            changes,
                            plan,
                        )?;
                        requires_sync |= old_roots.len() != 1
                            || Self::native_roots(tree, reconciled)?.len() != 1;
                        if reconciled != child_node {
                            nodes.insert(child.key().clone(), reconciled);
                        }
                    }
                }
                for child in children.iter() {
                    if !nodes.contains_key(child.key()) {
                        let (child_node, native) = Self::mount_planned_view(
                            tree,
                            Some(node),
                            Some(child.key().clone()),
                            child.view().clone(),
                            components,
                            changes,
                            plan,
                        )?;
                        if native.len() != 1 {
                            requires_sync = true;
                        }
                        nodes.insert(child.key().clone(), child_node);
                    }
                }

                let order = new_keys
                    .iter()
                    .map(|key| {
                        nodes
                            .get(key)
                            .copied()
                            .ok_or(PumpError::StructureUnsupported)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                tree.set_children(node, order)?;
                let new_native = Self::native_children(tree, node)?;
                let dense = operations.len() >= 256;
                if (requires_sync || dense) && old_native != new_native {
                    plan.synchronize_children(node, new_native);
                } else if !requires_sync {
                    let mut key_order = old_keys;
                    for operation in operations {
                        let (key, before, inserted) = match operation {
                            KeyedOperation::Remove { key } => {
                                if let Some(index) = key_order.iter().position(|item| item == &key)
                                {
                                    key_order.remove(index);
                                }
                                continue;
                            }
                            KeyedOperation::Insert { key, before } => (key, before, true),
                            KeyedOperation::Move { key, before } => {
                                let previous = key_order
                                    .iter()
                                    .position(|item| item == &key)
                                    .ok_or(PumpError::StructureUnsupported)?;
                                key_order.remove(previous);
                                (key, before, false)
                            }
                        };
                        let index = before.as_ref().map_or(key_order.len(), |before| {
                            key_order
                                .iter()
                                .position(|item| item == before)
                                .unwrap_or(key_order.len())
                        });
                        key_order.insert(index, key.clone());
                        let child_node = nodes
                            .get(&key)
                            .copied()
                            .ok_or(PumpError::StructureUnsupported)?;
                        let child = Self::native_root(tree, child_node)?;
                        plan.push(if inserted {
                            Command::InsertChild {
                                parent: node,
                                child,
                                index,
                            }
                        } else {
                            Command::MoveChild {
                                parent: node,
                                child,
                                index,
                            }
                        });
                    }
                }
                Ok(node)
            }
        }
    }

    fn reconcile_virtual_collection(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        if tree.kind(node)? != NodeKind::VirtualCollection
            || element.kind() != MountedKind::ItemsRepeater
        {
            return Self::replace_planned_view(
                tree,
                node,
                View::native(element),
                components,
                changes,
                plan,
            );
        }
        let ElementStructure::Virtual(items) = element.into_parts().structure else {
            return Err(PumpError::StructureUnsupported);
        };
        let old_keys = tree.virtual_model(node)?.keys();
        let keys_changed = old_keys.len() != items.len()
            || old_keys
                .iter()
                .zip(items.iter())
                .any(|(old, new)| old != new.key());
        if keys_changed {
            for row in tree.children(node)?.to_vec() {
                Self::collect_retired_components(tree, row, components, changes)?;
                Self::retire_planned_subtree(tree, row, plan)?;
            }
            tree.virtual_model_mut(node)?
                .update(items.iter().map(|item| item.key().clone()))
                .map_err(TreeError::from)?;
            tree.update_virtual_items(node, items)?;
            tree.virtual_model_mut(node)?.clear();
            plan.push(Command::ResetVirtualCollection {
                node,
                item_count: tree.virtual_items(node)?.len(),
            });
            return Ok(node);
        }

        tree.update_virtual_items(node, items)?;
        let realized = tree
            .children(node)?
            .iter()
            .copied()
            .map(|logical_root| {
                let key = tree
                    .key(logical_root)?
                    .cloned()
                    .ok_or(PumpError::StructureUnsupported)?;
                let container = tree
                    .realized_container_for_logical(node, logical_root)?
                    .ok_or(PumpError::StructureUnsupported)?;
                let view = tree.virtual_item(node, &key)?.clone();
                Ok((container, logical_root, view))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        for (_, logical_root, view) in realized {
            Self::reconcile_planned_view(tree, logical_root, view, components, changes, plan)?;
        }
        Ok(node)
    }

    fn reconcile_fragment(
        tree: &mut Tree,
        node: NodeId,
        children: &[KeyedView],
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let old_native = Self::native_roots(tree, node)?;
        let current = tree.children(node)?.to_vec();
        let old_keys = current
            .iter()
            .map(|child| {
                tree.key(*child)?
                    .cloned()
                    .ok_or(PumpError::StructureUnsupported)
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        let new_keys = children
            .iter()
            .map(|child| child.key().clone())
            .collect::<Vec<_>>();
        diff(&old_keys, &new_keys)
            .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

        let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
        let mut nodes = old_keys
            .iter()
            .cloned()
            .zip(current.iter().copied())
            .collect::<HashMap<_, _>>();
        for (key, child) in old_keys.iter().zip(current) {
            if !new_key_set.contains(key) {
                Self::collect_retired_components(tree, child, components, changes)?;
                Self::retire_planned_subtree(tree, child, plan)?;
            }
        }
        let mut order = Vec::with_capacity(children.len());
        for child in children {
            let child_node = if let Some(child_node) = nodes.get(child.key()).copied() {
                let reconciled = Self::reconcile_planned_view(
                    tree,
                    child_node,
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?;
                nodes.insert(child.key().clone(), reconciled);
                reconciled
            } else {
                let mounted = Self::mount_planned_view(
                    tree,
                    Some(node),
                    Some(child.key().clone()),
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?
                .0;
                nodes.insert(child.key().clone(), mounted);
                mounted
            };
            order.push(child_node);
        }
        tree.set_children(node, order)?;

        let new_native = Self::native_roots(tree, node)?;
        if old_native != new_native {
            match Self::native_attachment(tree, node)? {
                NativeAttachment::Children { parent, .. }
                    if tree.kind(parent)? == NodeKind::VirtualCollection =>
                {
                    let [old_native] = old_native.as_slice() else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    let [new_native] = new_native.as_slice() else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    if let Some(container) = tree.realized_container(parent, *old_native)? {
                        let row = tree
                            .realized(parent, container)?
                            .ok_or(PumpError::StructureUnsupported)?;
                        tree.update_realized(parent, container, row.logical_root, *new_native)?;
                        plan.push(Command::AttachRealized {
                            collection: parent,
                            container,
                            child: *new_native,
                        });
                    } else if tree.realized_container(parent, *new_native)?.is_none() {
                        return Err(PumpError::StructureUnsupported);
                    }
                }
                NativeAttachment::Children { parent, .. } => {
                    let native = Self::native_children(tree, parent)?;
                    Self::validate_native_arity(tree, parent, &native)?;
                    plan.synchronize_children(parent, native);
                }
                NativeAttachment::Slot { parent, slot } => {
                    let child = match new_native.as_slice() {
                        [] => None,
                        [child] => Some(*child),
                        _ => return Err(PumpError::StructureUnsupported),
                    };
                    plan.push(Command::SetSlot {
                        parent,
                        slot,
                        child,
                    });
                }
            }
        }
        Ok(())
    }

    fn replace_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node)?.ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node)?.cloned();
        let index = tree
            .children(parent)?
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let attachment = Self::native_attachment(tree, node)?;
        let realized = match attachment {
            NativeAttachment::Children { parent, .. }
                if tree.kind(parent)? == NodeKind::VirtualCollection =>
            {
                let native = Self::native_root(tree, node)?;
                let container = tree
                    .realized_container(parent, native)?
                    .ok_or(PumpError::StructureUnsupported)?;
                let row = tree
                    .realized(parent, container)?
                    .ok_or(PumpError::StructureUnsupported)?;
                Some((parent, container, row))
            }
            _ => None,
        };

        Self::collect_retired_components(tree, node, components, changes)?;
        Self::retire_planned_subtree(tree, node, plan)?;
        let (replacement, native) =
            Self::mount_planned_view(tree, Some(parent), key, view, components, changes, plan)?;
        let mut children = tree.children(parent)?.to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children)?;
        match (attachment, realized) {
            (NativeAttachment::Children { .. }, Some((collection, container, row))) => {
                let [native] = native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                let logical_root = if row.logical_root == node {
                    replacement
                } else {
                    row.logical_root
                };
                if row.logical_root == node {
                    tree.set_realized(collection, container, logical_root, *native)?;
                } else {
                    tree.update_realized(collection, container, logical_root, *native)?;
                }
                plan.push(Command::AttachRealized {
                    collection,
                    container,
                    child: *native,
                });
            }
            (
                NativeAttachment::Children {
                    parent,
                    index: native_index,
                },
                None,
            ) => {
                let native_children = Self::native_children(tree, parent)?;
                Self::validate_native_arity(tree, parent, &native_children)?;
                for (index, child) in native.into_iter().enumerate() {
                    plan.push(Command::InsertChild {
                        parent,
                        child,
                        index: native_index + index,
                    });
                }
            }
            (NativeAttachment::Slot { parent, slot }, None) => {
                let child = match native.as_slice() {
                    [] => None,
                    [child] => Some(*child),
                    _ => return Err(PumpError::StructureUnsupported),
                };
                plan.push(Command::SetSlot {
                    parent,
                    slot,
                    child,
                });
            }
            (NativeAttachment::Slot { .. }, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
        }
        Ok(replacement)
    }

    pub(in super::super) fn collect_retired_components(
        tree: &Tree,
        root: NodeId,
        components: &ComponentStore,
        changes: &mut ComponentChanges,
    ) -> Result<(), PumpError> {
        for node in tree.subtree_postorder(root)? {
            if tree.kind(node)? == NodeKind::Component {
                let token = components.token(tree.component_scope(node)?)?;
                if !changes.retired.contains(&token) {
                    changes.retired.push(token);
                }
            }
        }
        Ok(())
    }

    pub(in super::super) fn recompose_component(
        tree: &mut Tree,
        node: NodeId,
        token: ComponentToken,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if !changes.composed.insert(token) {
            return Ok(());
        }
        let render = components.view(token, tree.context_snapshot(node)?)?;
        changes.context_reads.insert(token, render.dependencies);
        Self::recompose_component_view(tree, node, render.view, components, changes, plan)
    }

    pub(in super::super) fn recompose_component_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let [slot] = tree.children(node)? else {
            return Err(PumpError::StructureUnsupported);
        };
        if tree.kind(*slot)? != NodeKind::Slot {
            return Err(PumpError::StructureUnsupported);
        }
        let [child] = tree.children(*slot)? else {
            return Err(PumpError::StructureUnsupported);
        };
        Self::reconcile_planned_view(tree, *child, view, components, changes, plan).map(|_| ())
    }

    pub(in super::super) fn mount_planned_view(
        tree: &mut Tree,
        logical_parent: Option<NodeId>,
        key: Option<Key>,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(NodeId, Vec<NodeId>), PumpError> {
        match view.into_kind() {
            ViewKind::Native(element) => {
                let node = Self::mount_planned_element(tree, logical_parent, key, element, plan)?;
                Ok((node, vec![node]))
            }
            ViewKind::Component(component) => {
                let token = component.reserve(components)?;
                changes.reserved.push(token);
                let node = tree.insert_component(
                    logical_parent,
                    key,
                    token.scope(),
                    component.component_type(),
                )?;
                let slot = tree.insert(Some(node), NodeKind::Slot)?;
                let render = components.view(token, tree.context_snapshot(node)?)?;
                changes.context_reads.insert(token, render.dependencies);
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(slot),
                    None,
                    render.view,
                    components,
                    changes,
                    plan,
                )?;
                Ok((node, native))
            }
            ViewKind::Fragment(children) => {
                let node = tree.insert_fragment(logical_parent, key)?;
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                let mut native = Vec::new();
                for child in children {
                    let (key, view) = child.into_parts();
                    let (_, child_native) = Self::mount_planned_view(
                        tree,
                        Some(node),
                        Some(key),
                        view,
                        components,
                        changes,
                        plan,
                    )?;
                    native.extend(child_native);
                }
                Ok((node, native))
            }
            ViewKind::Provider { provision, child } => {
                let node = tree.insert_provider(logical_parent, key, provision)?;
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*child),
                    components,
                    changes,
                    plan,
                )?;
                Ok((node, native))
            }
            ViewKind::Slots {
                control,
                slots: desired,
            } => {
                if !Self::element_structure_is_empty(&control)
                    || !Self::control_has_role(control.kind(), ControlRole::Slots)
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let kind = control.kind();
                let mut desired = collect_desired_slots(desired)?;
                let slot_ids = slots(kind);
                if desired.len() > slot_ids.len()
                    || desired.keys().any(|slot| !slot_ids.contains(slot))
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let node = Self::mount_planned_element(tree, logical_parent, key, control, plan)?;
                for slot in slot_ids {
                    let slot_node = tree.insert(Some(node), NodeKind::NamedSlot(*slot))?;
                    let view = desired.remove(slot).unwrap_or_else(View::empty);
                    let (_, native) = Self::mount_planned_view(
                        tree,
                        Some(slot_node),
                        None,
                        view,
                        components,
                        changes,
                        plan,
                    )?;
                    let child = match native.as_slice() {
                        [] => None,
                        [child] => Some(*child),
                        _ => return Err(PumpError::StructureUnsupported),
                    };
                    if child.is_some() {
                        plan.push(Command::SetSlot {
                            parent: node,
                            slot: *slot,
                            child,
                        });
                    }
                }
                Ok((node, vec![node]))
            }
            ViewKind::Content { control, content } => {
                if !Self::element_structure_is_empty(&control)
                    || !Self::control_has_role(control.kind(), ControlRole::Content)
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let node = Self::mount_planned_element(tree, logical_parent, key, control, plan)?;
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*content),
                    components,
                    changes,
                    plan,
                )?;
                match native.as_slice() {
                    [] => {}
                    [child] => {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child: *child,
                            index: 0,
                        });
                    }
                    _ => return Err(PumpError::StructureUnsupported),
                }
                Ok((node, vec![node]))
            }
            ViewKind::Children { control, children } => {
                if !Self::element_structure_is_empty(&control)
                    || !Self::control_has_role(control.kind(), ControlRole::Children)
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let node = Self::mount_planned_element(tree, logical_parent, key, control, plan)?;
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                let mut native_index = 0;
                for child in children {
                    let (key, view) = child.into_parts();
                    let (_, native) = Self::mount_planned_view(
                        tree,
                        Some(node),
                        Some(key),
                        view,
                        components,
                        changes,
                        plan,
                    )?;
                    for child in native {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child,
                            index: native_index,
                        });
                        native_index += 1;
                    }
                }

                Ok((node, vec![node]))
            }
        }
    }
}

fn node_kind(tree: &Tree, node: NodeId) -> Result<MountedKind, PumpError> {
    match tree.kind(node)? {
        NodeKind::Native(kind) => Ok(kind),
        _ => Err(PumpError::StructureUnsupported),
    }
}

fn collect_desired_slots(slots: Rc<Vec<SlottedView>>) -> Result<HashMap<SlotId, View>, PumpError> {
    let mut desired = HashMap::new();
    for slot in Rc::unwrap_or_clone(slots) {
        if desired.insert(slot.slot, slot.view).is_some() {
            return Err(PumpError::StructureUnsupported);
        }
    }
    Ok(desired)
}
