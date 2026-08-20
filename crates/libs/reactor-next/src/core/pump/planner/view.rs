//! Planning for [`View`] trees: component, fragment, and keyed-children
//! reconciliation and mounting, built on top of `element` and `topology`.

use super::super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(in super::super) fn reconcile_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        match view {
            View::Native(element) => {
                if tree.kind(node)? != NodeKind::Native(element.kind())
                    || !tree.children(node)?.is_empty()
                    || !matches!(element.structure(), ElementStructureRef::None)
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Native(element),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_node(tree, node, element, plan)
            }
            View::Component(component) => {
                if tree.kind(node)? != NodeKind::Component
                    || tree.component_type(node)? != component.component_type()
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Component(component),
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
            View::Empty => {
                if tree.kind(node)? != NodeKind::Fragment {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Empty,
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_fragment(tree, node, &[], components, changes, plan)?;
                Ok(node)
            }
            View::Fragment(children) => {
                if tree.kind(node)? != NodeKind::Fragment {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Fragment(children),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_fragment(tree, node, &children, components, changes, plan)?;
                Ok(node)
            }
            View::Provider { provision, child } => {
                if tree.kind(node)? != NodeKind::Provider {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Provider { provision, child },
                        components,
                        changes,
                        plan,
                    );
                }
                let previous = tree.provision(node)?.clone();
                if previous != provision {
                    let ids = [previous.id, provision.id];
                    for descendant in tree.subtree_postorder(node)? {
                        if tree.kind(descendant)? != NodeKind::Component {
                            continue;
                        }
                        let token = components.token(tree.component_scope(descendant)?)?;
                        let affected =
                            components
                                .context_dependencies(token)?
                                .is_some_and(|dependencies| {
                                    dependencies.iter().any(|dependency| {
                                        if previous.id == provision.id {
                                            dependency.id == provision.id
                                                && dependency.provider == Some(node)
                                        } else {
                                            ids.contains(&dependency.id)
                                        }
                                    })
                                });
                        if affected {
                            let mut current = descendant;
                            while current != node {
                                if tree.kind(current)? == NodeKind::Component {
                                    changes
                                        .retry
                                        .insert(components.token(tree.component_scope(current)?)?);
                                }
                                current = tree
                                    .parent(current)?
                                    .ok_or(PumpError::StructureUnsupported)?;
                            }
                        }
                    }
                    tree.set_provision(node, provision)?;
                }
                let [current] = tree.children(node)? else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::reconcile_planned_view(tree, *current, *child, components, changes, plan)?;
                Ok(node)
            }
            View::Content { control, content } => {
                if !Self::control_has_role(control.kind(), ControlRole::Content) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Content { control, content },
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let [child] = tree.children(node)? else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::reconcile_planned_view(tree, *child, *content, components, changes, plan)?;
                Ok(node)
            }
            View::Children { control, children } => {
                if !Self::control_has_role(control.kind(), ControlRole::Children) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Children { control, children },
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
            View::VirtualItems { .. } => Err(PumpError::StructureUnsupported),
        }
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
            let (native_parent, _) = Self::native_location(tree, node)?;
            let native = Self::native_children(tree, native_parent)?;
            Self::validate_native_arity(tree, native_parent, &native)?;
            plan.synchronize_children(native_parent, native);
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
        let (native_parent, native_index) = Self::native_location(tree, node)?;
        if tree.kind(native_parent)? == NodeKind::VirtualCollection {
            return Err(PumpError::StructureUnsupported);
        }

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
        let native_children = Self::native_children(tree, native_parent)?;
        Self::validate_native_arity(tree, native_parent, &native_children)?;
        for (index, child) in native.into_iter().enumerate() {
            plan.push(Command::InsertChild {
                parent: native_parent,
                child,
                index: native_index + index,
            });
        }
        Ok(replacement)
    }

    fn collect_retired_components(
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
        match view {
            View::Native(element) => {
                let node = Self::mount_planned_element(tree, logical_parent, key, element, plan)?;
                Ok((node, vec![node]))
            }
            View::Component(component) => {
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
            View::Empty => {
                let node = tree.insert_fragment(logical_parent, key)?;
                Ok((node, Vec::new()))
            }
            View::Fragment(children) => {
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
            View::Provider { provision, child } => {
                let node = tree.insert_provider(logical_parent, key, provision)?;
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    *child,
                    components,
                    changes,
                    plan,
                )?;
                Ok((node, native))
            }
            View::Content { control, content } => {
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
                    *content,
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
            View::Children { control, children } => {
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
            View::VirtualItems { .. } => Err(PumpError::StructureUnsupported),
        }
    }
}
