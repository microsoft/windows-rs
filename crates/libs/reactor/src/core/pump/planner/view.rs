//! Planning for [`View`] trees: component, fragment, and keyed-children
//! reconciliation and mounting, built on top of `element` and `topology`.

use super::super::*;
use super::topology::NativeAttachment;

fn validate_commands(commands: &[CommandBarCommand]) -> Result<(), PumpError> {
    let mut keys = HashSet::new();
    for command in commands {
        if !keys.insert(command.key()) {
            return Err(PumpError::DuplicateKey(command.key().clone()));
        }
    }
    Ok(())
}

fn validate_menu_items(items: &[MenuItem]) -> Result<(), PumpError> {
    let mut keys = HashSet::new();
    for item in items {
        if !keys.insert(item.key()) {
            return Err(PumpError::DuplicateKey(item.key().clone()));
        }
        if let MenuItem::Submenu { items, .. } = item {
            validate_menu_items(items)?;
        }
    }
    Ok(())
}

fn validate_tree_nodes(nodes: &[TreeNode]) -> Result<(), PumpError> {
    let mut keys = HashSet::new();
    for node in nodes {
        if !keys.insert(&node.key) {
            return Err(PumpError::DuplicateKey(node.key.clone()));
        }
        validate_tree_nodes(&node.children)?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct ChildListTarget {
    logical_parent: NodeId,
    native_parent: NodeId,
    slot: Option<SlotId>,
}

struct KeyedChildReconciliation {
    old_keys: Vec<Key>,
    new_keys: Vec<Key>,
    operations: Vec<KeyedOperation<Key>>,
    nodes: HashMap<Key, NodeId>,
}

impl KeyedChildReconciliation {
    fn order(&self) -> Result<Vec<NodeId>, PumpError> {
        self.new_keys
            .iter()
            .map(|key| {
                self.nodes
                    .get(key)
                    .copied()
                    .ok_or(PumpError::StructureUnsupported)
            })
            .collect()
    }
}

impl<R: NativeRuntime> Pump<R> {
    pub(in super::super) fn reconcile_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        if Self::view_uses_native_control(view.as_kind(), MountedKind::ToolTip)
            && !Self::is_tooltip_implementation(tree, node)
        {
            return Err(PumpError::StructureUnsupported);
        }
        if Self::view_uses_native_control(view.as_kind(), MountedKind::ContentDialog)
            && !Self::is_content_dialog_implementation(tree, node)
        {
            return Err(PumpError::StructureUnsupported);
        }
        if matches!(tree.kind(node), NodeKind::Tooltip(_))
            && !matches!(view.as_kind(), ViewKind::Tooltip { .. })
        {
            return Self::unwrap_tooltip(tree, node, view, components, changes, plan);
        }
        if matches!(tree.kind(node), NodeKind::Flyout(_))
            && !matches!(view.as_kind(), ViewKind::Flyout { .. })
        {
            return Self::unwrap_flyout(tree, node, view, components, changes, plan);
        }
        if matches!(
            tree.kind(node),
            NodeKind::Menu(_) | NodeKind::CommandBarFlyout | NodeKind::TreeNodes
        ) && !matches!(
            view.as_kind(),
            ViewKind::Menu { .. } | ViewKind::CommandBarFlyout { .. } | ViewKind::TreeNodes { .. }
        ) {
            return Self::replace_planned_view(tree, node, view, components, changes, plan);
        }
        if !plan.reconcile_observations
            && !matches!(view.as_kind(), ViewKind::Native(_))
            && Self::node_matches_view_kind(tree, node, view.as_kind())?
        {
            return Ok(node);
        }
        match view.into_kind() {
            ViewKind::Native(element) => {
                if matches!(element.structure(), ElementStructureRef::Virtual(_)) {
                    return Self::reconcile_virtual_collection(
                        tree, node, element, components, changes, plan,
                    );
                }
                if tree.kind(node) != NodeKind::Native(element.kind())
                    || !tree.children(node).is_empty()
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
                if tree.kind(node) != NodeKind::Component
                    || tree.component_type(node) != component.component_type()
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
                let token = components.token(tree.component_scope(node));
                let changed = component.apply_input(components, token);
                changes
                    .host_requests
                    .extend(components.take_host_requests());
                if changed {
                    changes.touched.insert(token);
                }
                if changed || changes.recompose.contains(&token) {
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
                if tree.kind(node) != NodeKind::Fragment {
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
                if tree.kind(node) != NodeKind::Provider {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Provider { provision, child }),
                        components,
                        changes,
                        plan,
                    );
                }
                let previous = tree.provision(node).clone();
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
                                .component_node(scope)
                                .ok_or(PumpError::StructureUnsupported)?;
                            if !tree.is_descendant_of(consumer, node) {
                                continue;
                            }
                            let token = components.token(scope);
                            let resolved = components
                                .context_dependencies(token)
                                .and_then(|dependencies| {
                                    dependencies
                                        .iter()
                                        .find(|dependency| dependency.id == provision.id)
                                })
                                .and_then(|dependency| dependency.provider);
                            let shadowed = match resolved {
                                Some(provider) => tree.is_descendant_of(provider, node),
                                None => false,
                            };
                            if !shadowed {
                                affected.insert(scope);
                            }
                        }
                    }
                    tree.set_provision(node, provision);
                    Some(affected)
                } else {
                    None
                };
                let [current] = tree.children(node) else {
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
                        if let Some(consumer) = tree.component_node(scope)
                            && tree.is_descendant_of(consumer, node)
                        {
                            ordered.push((tree.depth(consumer), consumer, scope));
                        }
                    }
                    ordered.sort_unstable_by_key(|(depth, consumer, _)| (*depth, *consumer));
                    for (_, _, scope) in ordered {
                        if let Some(consumer) = tree.component_node(scope)
                            && tree.is_descendant_of(consumer, node)
                        {
                            Self::recompose_component(
                                tree,
                                consumer,
                                components.token(scope),
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
                let structure_matches = tree.kind(node) == NodeKind::Native(control.kind())
                    && tree.children(node).len() == slot_ids.len()
                    && tree
                        .children(node)
                        .iter()
                        .zip(slot_ids)
                        .all(|(node, slot)| tree.kind(*node) == NodeKind::NamedSlot(*slot));
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
                let children = tree.children(node).to_vec();
                if children.len() != slot_ids.len() {
                    return Err(PumpError::StructureUnsupported);
                }
                for (slot_node, slot) in children.into_iter().zip(slot_ids.iter().copied()) {
                    if tree.kind(slot_node) != NodeKind::NamedSlot(slot) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    if slot_is_collection(slot) {
                        let desired = match desired.remove(&slot) {
                            Some(SlotContent::Collection(children)) => children,
                            Some(SlotContent::Single(_)) => {
                                return Err(PumpError::StructureUnsupported);
                            }
                            None => Rc::new(Vec::new()),
                        };
                        Self::reconcile_keyed_child_list(
                            tree,
                            ChildListTarget {
                                logical_parent: slot_node,
                                native_parent: node,
                                slot: Some(slot),
                            },
                            &desired,
                            components,
                            changes,
                            plan,
                        )?;
                    } else {
                        let [child] = tree.children(slot_node) else {
                            return Err(PumpError::StructureUnsupported);
                        };
                        let view = match desired.remove(&slot) {
                            Some(SlotContent::Single(view)) => view,
                            Some(SlotContent::Collection(_)) => {
                                return Err(PumpError::StructureUnsupported);
                            }
                            None => View::empty(),
                        };
                        let child = Self::reconcile_planned_view(
                            tree, *child, view, components, changes, plan,
                        )?;
                        if Self::native_root_count(tree, child)? > 1 {
                            return Err(PumpError::StructureUnsupported);
                        }
                    }
                }
                Ok(node)
            }
            ViewKind::Tooltip { target, tooltip } => {
                let NodeKind::Tooltip(_) = tree.kind(node) else {
                    return Self::wrap_tooltip(
                        tree,
                        node,
                        View::from_kind(*target),
                        tooltip,
                        components,
                        changes,
                        plan,
                    );
                };
                let [current_target, current_tooltip] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current_target = *current_target;
                let current_tooltip = *current_tooltip;
                Self::reconcile_planned_view(
                    tree,
                    current_target,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let content = *tooltip.content;
                Self::reconcile_planned_view(
                    tree,
                    current_tooltip,
                    ToolTip::new().content(content),
                    components,
                    changes,
                    plan,
                )?;
                Self::refresh_tooltip_attachment(tree, node, tooltip.placement, plan)?;
                Ok(node)
            }
            ViewKind::Flyout { target, flyout } => {
                let NodeKind::Flyout(_) = tree.kind(node) else {
                    return Self::wrap_flyout(
                        tree,
                        node,
                        View::from_kind(*target),
                        flyout,
                        components,
                        changes,
                        plan,
                    );
                };
                let [current_target, current_content] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current_target = *current_target;
                let current_content = *current_content;
                Self::reconcile_planned_view(
                    tree,
                    current_target,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                Self::reconcile_planned_view(
                    tree,
                    current_content,
                    *flyout.content,
                    components,
                    changes,
                    plan,
                )?;
                Self::refresh_flyout_attachment(tree, node, flyout.placement, plan)?;
                Ok(node)
            }
            ViewKind::Menu { target, menu } => {
                let NodeKind::Menu(_) = tree.kind(node) else {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::Menu { target, menu }),
                        components,
                        changes,
                        plan,
                    );
                };
                let [current_target] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current_target = *current_target;
                let current_target = Self::reconcile_planned_view(
                    tree,
                    current_target,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let target = Self::native_root(tree, current_target)?;
                let kind = match tree.kind(target) {
                    NodeKind::Native(MountedKind::Button) => OwnedMenuKind::ButtonFlyout,
                    NodeKind::Native(MountedKind::DropDownButton) => {
                        OwnedMenuKind::DropDownButtonFlyout
                    }
                    NodeKind::Native(MountedKind::MenuBarItem) => OwnedMenuKind::MenuBarItem,
                    _ => return Err(PumpError::StructureUnsupported),
                };
                tree.set_kind(node, NodeKind::Menu(kind));
                validate_menu_items(&menu.items)?;
                let revision = tree.update_menu(node, menu.clone());
                plan.push(Command::SetOwnedMenu {
                    owner: node,
                    target,
                    kind,
                    items: Some(menu.items),
                    revision,
                });
                Ok(node)
            }
            ViewKind::CommandBarFlyout { target, flyout } => {
                if tree.kind(node) != NodeKind::CommandBarFlyout {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::CommandBarFlyout { target, flyout }),
                        components,
                        changes,
                        plan,
                    );
                }
                let [current_target] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current_target = *current_target;
                let current_target = Self::reconcile_planned_view(
                    tree,
                    current_target,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let target = Self::native_root(tree, current_target)?;
                if tree.kind(target) != NodeKind::Native(MountedKind::Button) {
                    return Err(PumpError::StructureUnsupported);
                }
                validate_commands(&flyout.primary)?;
                validate_commands(&flyout.secondary)?;
                let revision = tree.update_command_bar_flyout(node, flyout.clone());
                plan.push(Command::SetCommandBarFlyout {
                    owner: node,
                    target,
                    primary: Some(flyout.primary),
                    secondary: flyout.secondary,
                    revision,
                });
                Ok(node)
            }
            ViewKind::TreeNodes {
                tree: desired,
                nodes,
            } => {
                if tree.kind(node) != NodeKind::TreeNodes {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::TreeNodes {
                            tree: desired,
                            nodes,
                        }),
                        components,
                        changes,
                        plan,
                    );
                }
                validate_tree_nodes(&nodes)?;
                let [current] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current = Self::reconcile_planned_view(
                    tree,
                    *current,
                    View::from_kind(*desired),
                    components,
                    changes,
                    plan,
                )?;
                let target = Self::native_root(tree, current)?;
                if tree.kind(target) != NodeKind::Native(MountedKind::TreeView) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.tree_nodes(node) != &nodes {
                    tree.update_tree_nodes(node, Rc::clone(&nodes));
                    plan.push(Command::SetTreeViewNodes {
                        target,
                        nodes: nodes.as_ref().clone(),
                    });
                }
                Ok(node)
            }
            ViewKind::ContentDialog { dialog, open } => {
                let NodeKind::ContentDialog(previous_open) = tree.kind(node) else {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::from_kind(ViewKind::ContentDialog { dialog, open }),
                        components,
                        changes,
                        plan,
                    );
                };
                let [current] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                let current = *current;
                let owner = Self::native_container(tree, node)?;
                let replacing =
                    !Self::content_dialog_implementation_matches(tree, current, dialog.as_ref())?;
                let previous_dialog = Self::native_root(tree, current)?;
                if replacing && previous_open {
                    plan.push(Command::SetContentDialogOpen {
                        node: previous_dialog,
                        owner,
                        open: false,
                    });
                }
                let current = Self::reconcile_planned_view(
                    tree,
                    current,
                    View::from_kind(*dialog),
                    components,
                    changes,
                    plan,
                )?;
                let dialog = Self::native_root(tree, current)?;
                if replacing {
                    tree.set_content_dialog_open(node, open);
                    if open {
                        plan.post_publish_commands
                            .push(Command::SetContentDialogOpen {
                                node: dialog,
                                owner,
                                open: true,
                            });
                    }
                } else if previous_open != open {
                    let command = Command::SetContentDialogOpen {
                        node: dialog,
                        owner,
                        open,
                    };
                    if open {
                        plan.post_publish_commands.push(command);
                    } else {
                        plan.push(command);
                    }
                    tree.set_content_dialog_open(node, open);
                }
                let [mounted_dialog] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                if Self::native_root(tree, *mounted_dialog)? != dialog {
                    return Err(PumpError::StructureUnsupported);
                }
                Ok(node)
            }
            ViewKind::Content { control, content } => {
                if !Self::control_has_role(control.kind(), ControlRole::Content) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node) != NodeKind::Native(control.kind())
                    || tree.children(node).len() != 1
                {
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
                let [child] = tree.children(node) else {
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
                if tree.kind(node) != NodeKind::Native(control.kind()) {
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
                Self::reconcile_keyed_child_list(
                    tree,
                    ChildListTarget {
                        logical_parent: node,
                        native_parent: node,
                        slot: None,
                    },
                    &children,
                    components,
                    changes,
                    plan,
                )?;
                Ok(node)
            }
        }
    }

    fn reconcile_keyed_child_list(
        tree: &mut Tree,
        target: ChildListTarget,
        desired: &[KeyedView],
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let old_native = Self::native_children(tree, target.logical_parent)?;
        let current = tree.children(target.logical_parent).to_vec();
        let mut requires_sync = current
            .iter()
            .any(|child| Self::native_root_count(tree, *child) != Ok(1));
        if target.slot.is_some() && requires_sync {
            return Err(PumpError::StructureUnsupported);
        }
        let mut reconciliation = Self::begin_keyed_child_reconciliation(
            tree, current, desired, components, changes, plan,
        )?;
        for child in desired {
            if let Some(child_node) = reconciliation.nodes.get(child.key()).copied() {
                let old_root_count = Self::native_root_count(tree, child_node)?;
                let reconciled = Self::reconcile_planned_view(
                    tree,
                    child_node,
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?;
                let invalid_roots =
                    old_root_count != 1 || Self::native_root_count(tree, reconciled)? != 1;
                if target.slot.is_some() && invalid_roots {
                    return Err(PumpError::StructureUnsupported);
                }
                requires_sync |= invalid_roots;
                if reconciled != child_node {
                    reconciliation.nodes.insert(child.key().clone(), reconciled);
                }
            }
        }
        for child in desired {
            if !reconciliation.nodes.contains_key(child.key()) {
                let (child_node, native) = Self::mount_planned_view(
                    tree,
                    Some(target.logical_parent),
                    Some(child.key().clone()),
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?;
                if target.slot.is_some() && native.len() != 1 {
                    return Err(PumpError::StructureUnsupported);
                }
                requires_sync |= native.len() != 1;
                reconciliation.nodes.insert(child.key().clone(), child_node);
            }
        }

        tree.set_children(target.logical_parent, reconciliation.order()?);
        let new_native = Self::native_children(tree, target.logical_parent)?;
        let dense = super::is_dense_keyed_update(&reconciliation.operations);
        if (requires_sync || dense) && old_native != new_native {
            plan.synchronize_children(target.native_parent, target.slot, new_native);
        } else if !requires_sync {
            Self::replay_keyed_child_list(
                tree,
                target.native_parent,
                target.slot,
                reconciliation.old_keys,
                reconciliation.operations,
                &reconciliation.nodes,
                plan,
            )?;
        }
        Ok(())
    }

    pub(super) fn replay_keyed_child_list(
        tree: &Tree,
        parent: NodeId,
        slot: Option<SlotId>,
        mut key_order: Vec<Key>,
        operations: Vec<KeyedOperation<Key>>,
        nodes: &HashMap<Key, NodeId>,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        for operation in operations {
            let (key, before, inserted) = match operation {
                KeyedOperation::Remove { key } => {
                    if let Some(index) = key_order.iter().position(|item| item == &key) {
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
                    parent,
                    slot,
                    child,
                    index,
                }
            } else {
                Command::MoveChild {
                    parent,
                    slot,
                    child,
                    index,
                }
            });
        }
        Ok(())
    }

    fn node_matches_view_kind(
        tree: &Tree,
        node: NodeId,
        view: &ViewKind,
    ) -> Result<bool, PumpError> {
        match view {
            ViewKind::Native(element) => {
                if tree.kind(node) == NodeKind::VirtualCollection {
                    Ok(false)
                } else {
                    Self::node_matches_element(tree, node, element)
                }
            }
            ViewKind::Component(_) => Ok(false),
            ViewKind::Fragment(children) => {
                if tree.kind(node) != NodeKind::Fragment {
                    return Ok(false);
                }
                Self::keyed_views_match(tree, tree.children(node), children)
            }
            ViewKind::Provider { provision, child } => {
                if tree.kind(node) != NodeKind::Provider || tree.provision(node) != provision {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, child)
            }
            ViewKind::Tooltip { target, tooltip } => {
                if tree.kind(node) != NodeKind::Tooltip(tooltip.placement) {
                    return Ok(false);
                }
                let [current_target, current_tooltip] = tree.children(node) else {
                    return Ok(false);
                };
                if !Self::node_matches_view_kind(tree, *current_target, target)?
                    || tree.kind(*current_tooltip) != NodeKind::Native(MountedKind::ToolTip)
                {
                    return Ok(false);
                }
                let [content] = tree.children(*current_tooltip) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *content, tooltip.content.as_kind())
            }
            ViewKind::Flyout { target, flyout } => {
                if tree.kind(node) != NodeKind::Flyout(flyout.placement) {
                    return Ok(false);
                }
                let [current_target, current_content] = tree.children(node) else {
                    return Ok(false);
                };
                Ok(Self::node_matches_view_kind(tree, *current_target, target)?
                    && Self::node_matches_view_kind(
                        tree,
                        *current_content,
                        flyout.content.as_kind(),
                    )?)
            }
            ViewKind::Menu { target, menu } => {
                if !matches!(tree.kind(node), NodeKind::Menu(_))
                    || tree.owned_menu(node) != menu.items
                    || tree.owned_callback(node) != &menu.on_click
                {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, target)
            }
            ViewKind::CommandBarFlyout { target, flyout } => {
                if tree.kind(node) != NodeKind::CommandBarFlyout
                    || tree.owned_commands(node)
                        != &(flyout.primary.clone(), flyout.secondary.clone())
                    || tree.owned_callback(node) != &flyout.on_click
                {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, target)
            }
            ViewKind::TreeNodes {
                tree: desired,
                nodes,
            } => {
                if tree.kind(node) != NodeKind::TreeNodes || tree.tree_nodes(node) != nodes {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, desired)
            }
            ViewKind::ContentDialog { dialog, open } => {
                if tree.kind(node) != NodeKind::ContentDialog(*open) {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, dialog.as_ref())
            }
            ViewKind::Content { control, content } => {
                if !Self::control_has_role(control.kind(), ControlRole::Content)
                    || !Self::shallow_control_matches(tree, node, control)?
                {
                    return Ok(false);
                }
                let [current] = tree.children(node) else {
                    return Ok(false);
                };
                Self::node_matches_view_kind(tree, *current, content)
            }
            ViewKind::Children { control, children } => {
                if !Self::control_has_role(control.kind(), ControlRole::Children)
                    || !Self::shallow_control_matches(tree, node, control)?
                {
                    return Ok(false);
                }
                Self::keyed_views_match(tree, tree.children(node), children)
            }
            ViewKind::Slots {
                control,
                slots: desired,
            } => {
                if !Self::control_has_role(control.kind(), ControlRole::Slots)
                    || !Self::shallow_control_matches(tree, node, control)?
                {
                    return Ok(false);
                }
                let slot_ids = slots(control.kind());
                let children = tree.children(node);
                if children.len() != slot_ids.len()
                    || desired.iter().any(|candidate| {
                        !slot_ids.contains(&candidate.slot)
                            || desired
                                .iter()
                                .filter(|other| other.slot == candidate.slot)
                                .count()
                                != 1
                    })
                {
                    return Ok(false);
                }
                for (slot_node, slot) in children.iter().zip(slot_ids) {
                    if tree.kind(*slot_node) != NodeKind::NamedSlot(*slot) {
                        return Ok(false);
                    }
                    let desired = desired.iter().find(|candidate| candidate.slot == *slot);
                    if slot_is_collection(*slot) {
                        let desired = match desired {
                            Some(SlottedView {
                                content: SlotContent::Collection(children),
                                ..
                            }) => children.as_slice(),
                            Some(_) => return Ok(false),
                            None => &[],
                        };
                        if !Self::keyed_views_match(tree, tree.children(*slot_node), desired)? {
                            return Ok(false);
                        }
                    } else {
                        let [current] = tree.children(*slot_node) else {
                            return Ok(false);
                        };
                        match desired {
                            Some(SlottedView {
                                content: SlotContent::Single(view),
                                ..
                            }) => {
                                if !Self::node_matches_view_kind(tree, *current, view.as_kind())? {
                                    return Ok(false);
                                }
                            }
                            Some(_) => return Ok(false),
                            None => {
                                if tree.kind(*current) != NodeKind::Fragment
                                    || !tree.children(*current).is_empty()
                                {
                                    return Ok(false);
                                }
                            }
                        }
                    }
                }
                Ok(true)
            }
        }
    }

    fn shallow_control_matches(
        tree: &Tree,
        node: NodeId,
        control: &Element,
    ) -> Result<bool, PumpError> {
        if tree.kind(node) != NodeKind::Native(control.kind())
            || !Self::element_structure_is_empty(control)
        {
            return Ok(false);
        }
        let native = tree.native(node);
        Ok(control.props_match(&native.desired)
            && native.reference.as_ref() == control.reference()
            && tree.node_window_title_bar(node) == control.window_title_bar()
            && Self::native_properties_match(native, &native.desired, control.element_state()))
    }

    fn keyed_views_match(
        tree: &Tree,
        mounted: &[NodeId],
        desired: &[KeyedView],
    ) -> Result<bool, PumpError> {
        if mounted.len() != desired.len() {
            return Ok(false);
        }
        for (mounted, desired) in mounted.iter().zip(desired) {
            if tree.key(*mounted) != Some(desired.key())
                || !Self::node_matches_view_kind(tree, *mounted, desired.view().as_kind())?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn reconcile_virtual_collection(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        if tree.kind(node) != NodeKind::VirtualCollection
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
        let ElementParts {
            props,
            reference,
            element_state,
            window_title_bar,
            structure: ElementStructure::Virtual(items),
            ..
        } = element.into_parts()
        else {
            return Err(PumpError::StructureUnsupported);
        };
        Self::reconcile_native_values(
            tree,
            node,
            props,
            reference,
            element_state,
            window_title_bar,
            plan,
        )?;
        let changed_keys =
            items.changed_keys(tree.virtual_items(node), tree.virtual_model(node).keys());
        if let Some(keys) = changed_keys {
            for row in tree.children(node).to_vec() {
                Self::collect_retired_components(tree, row, components, changes);
                Self::retire_planned_subtree(tree, row, plan)?;
            }
            let source_revision = {
                let model = tree.virtual_model_mut(node);
                model
                    .update(keys)
                    .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
                model.source_revision()
            };
            tree.update_virtual_items(node, items);
            tree.virtual_model_mut(node).clear();
            plan.push(Command::ResetVirtualCollection {
                node,
                item_count: tree.virtual_items(node).len(),
                source_revision,
            });
            return Ok(node);
        }

        tree.update_virtual_items(node, items);
        let realized = tree
            .realized_rows(node)
            .map(|(container, row)| {
                Ok((
                    container,
                    row.logical_root,
                    tree.virtual_view_at(node, row.index),
                ))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        for (_, logical_root, view) in realized {
            Self::reconcile_planned_view(tree, logical_root, view, components, changes, plan)?;
        }
        Ok(node)
    }

    fn begin_keyed_child_reconciliation(
        tree: &mut Tree,
        current: Vec<NodeId>,
        desired: &[KeyedView],
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<KeyedChildReconciliation, PumpError> {
        let old_keys = current
            .iter()
            .map(|child| {
                tree.key(*child)
                    .cloned()
                    .ok_or(PumpError::StructureUnsupported)
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        let new_keys = desired
            .iter()
            .map(|child| child.key().clone())
            .collect::<Vec<_>>();
        let operations = diff(&old_keys, &new_keys)
            .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
        let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
        let nodes = old_keys
            .iter()
            .cloned()
            .zip(current.iter().copied())
            .collect::<HashMap<_, _>>();

        for (key, child) in old_keys.iter().zip(current) {
            if !new_key_set.contains(key) {
                Self::collect_retired_components(tree, child, components, changes);
                Self::retire_planned_subtree(tree, child, plan)?;
            }
        }
        Ok(KeyedChildReconciliation {
            old_keys,
            new_keys,
            operations,
            nodes,
        })
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
        let current = tree.children(node).to_vec();
        let mut reconciliation = Self::begin_keyed_child_reconciliation(
            tree, current, children, components, changes, plan,
        )?;
        for child in children {
            if let Some(child_node) = reconciliation.nodes.get(child.key()).copied() {
                let reconciled = Self::reconcile_planned_view(
                    tree,
                    child_node,
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?;
                reconciliation.nodes.insert(child.key().clone(), reconciled);
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
                reconciliation.nodes.insert(child.key().clone(), mounted);
            }
        }
        tree.set_children(node, reconciliation.order()?);

        let new_native = Self::native_roots(tree, node)?;
        if old_native != new_native {
            match Self::native_attachment(tree, node)? {
                NativeAttachment::ChildList {
                    parent, slot: None, ..
                } if tree.kind(parent) == NodeKind::VirtualCollection => {
                    let Some((collection, container, _)) = Self::virtual_row_owner(tree, node)?
                    else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    Self::refresh_virtual_row_attachment(tree, collection, container, plan)?;
                }
                NativeAttachment::ChildList { parent, slot, .. } => {
                    let list_node =
                        slot.map_or(Ok(parent), |_| Self::collection_slot_node(tree, node))?;
                    let native = Self::native_children(tree, list_node)?;
                    Self::validate_native_arity(tree, list_node, &native)?;
                    plan.synchronize_children(parent, slot, native);
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
                NativeAttachment::Tooltip {
                    owner, placement, ..
                } => {
                    Self::refresh_tooltip_attachment(tree, owner, placement, plan)?;
                }
                NativeAttachment::Flyout { owner, placement } => {
                    Self::refresh_flyout_attachment(tree, owner, placement, plan)?;
                }
                NativeAttachment::ContentDialog => {}
            }
        }
        Ok(())
    }

    fn view_uses_native_control(view: &ViewKind, expected: MountedKind) -> bool {
        match view {
            ViewKind::Native(control)
            | ViewKind::Content { control, .. }
            | ViewKind::Children { control, .. }
            | ViewKind::Slots { control, .. } => control.kind() == expected,
            ViewKind::Component(_)
            | ViewKind::Fragment(_)
            | ViewKind::Provider { .. }
            | ViewKind::Tooltip { .. }
            | ViewKind::Flyout { .. }
            | ViewKind::Menu { .. }
            | ViewKind::CommandBarFlyout { .. }
            | ViewKind::TreeNodes { .. }
            | ViewKind::ContentDialog { .. } => false,
        }
    }

    fn content_dialog_implementation_matches(
        tree: &Tree,
        node: NodeId,
        view: &ViewKind,
    ) -> Result<bool, PumpError> {
        let (control, expected_children) = match view {
            ViewKind::Native(control) => (control, 0),
            ViewKind::Content { control, .. } => (control, 1),
            _ => return Ok(false),
        };
        Ok(control.kind() == MountedKind::ContentDialog
            && tree.kind(node) == NodeKind::Native(MountedKind::ContentDialog)
            && tree.children(node).len() == expected_children)
    }

    fn is_content_dialog_implementation(tree: &Tree, node: NodeId) -> bool {
        tree.parent(node)
            .is_some_and(|parent| matches!(tree.kind(parent), NodeKind::ContentDialog(_)))
    }

    fn content_dialog_implementation_mount_allowed(tree: &Tree, parent: Option<NodeId>) -> bool {
        let Some(parent) = parent else {
            return false;
        };
        matches!(tree.kind(parent), NodeKind::ContentDialog(_)) && tree.children(parent).is_empty()
    }

    fn content_dialog_owner_allowed(tree: &Tree, mut parent: Option<NodeId>) -> bool {
        while let Some(node) = parent {
            match tree.kind(node) {
                NodeKind::Native(kind) => {
                    return Self::control_has_role(kind, ControlRole::Children);
                }
                NodeKind::Window => return true,
                NodeKind::Component | NodeKind::Fragment | NodeKind::Provider => {
                    parent = tree.parent(node);
                }
                NodeKind::Application
                | NodeKind::VirtualCollection
                | NodeKind::Slot
                | NodeKind::NamedSlot(_)
                | NodeKind::Tooltip(_)
                | NodeKind::Flyout(_)
                | NodeKind::Menu(_)
                | NodeKind::CommandBarFlyout
                | NodeKind::TreeNodes
                | NodeKind::ContentDialog(_) => return false,
            }
        }
        false
    }

    fn is_tooltip_implementation(tree: &Tree, node: NodeId) -> bool {
        let Some(parent) = tree.parent(node) else {
            return false;
        };
        if !matches!(tree.kind(parent), NodeKind::Tooltip(_)) {
            return false;
        }
        let [_, tooltip] = tree.children(parent) else {
            return false;
        };
        *tooltip == node
    }

    fn tooltip_implementation_mount_allowed(tree: &Tree, parent: Option<NodeId>) -> bool {
        let Some(parent) = parent else {
            return false;
        };
        matches!(tree.kind(parent), NodeKind::Tooltip(_)) && tree.children(parent).len() == 1
    }

    fn wrap_tooltip(
        tree: &mut Tree,
        node: NodeId,
        target: View,
        tooltip: Tooltip,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        if Self::tooltip_owner(tree, node)?.is_some() || Self::flyout_owner(tree, node)?.is_some() {
            return Err(PumpError::StructureUnsupported);
        }
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let target = Self::reconcile_planned_view(tree, node, target, components, changes, plan)?;
        if tree.subtree_postorder(target).into_iter().any(|child| {
            matches!(
                tree.kind(child),
                NodeKind::Tooltip(_) | NodeKind::Flyout(_) | NodeKind::ContentDialog(_)
            )
        }) {
            return Err(PumpError::StructureUnsupported);
        }

        let placement = tooltip.placement;
        let owner = tree.insert_tooltip(Some(parent), key, placement);
        tree.reparent(target, owner, None);
        let content = *tooltip.content;
        let (_, tooltip_native) = Self::mount_planned_view(
            tree,
            Some(owner),
            None,
            ToolTip::new().content(content),
            components,
            changes,
            plan,
        )?;
        let [tooltip_native] = tooltip_native.as_slice() else {
            return Err(PumpError::StructureUnsupported);
        };
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == owner)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, owner);
        tree.set_children(parent, children);
        Self::refresh_tooltip_attachment(tree, owner, placement, plan)?;
        debug_assert_eq!(
            tree.tooltip_attachment(owner),
            Some((Self::native_root(tree, target)?, *tooltip_native))
        );
        Ok(owner)
    }

    fn unwrap_tooltip(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let [target, tooltip] = tree.children(node) else {
            return Err(PumpError::StructureUnsupported);
        };
        let target = *target;
        let tooltip = *tooltip;

        Self::clear_tooltip_attachment(tree, node, plan)?;
        Self::collect_retired_components(tree, tooltip, components, changes);
        Self::retire_planned_subtree(tree, tooltip, plan)?;
        tree.reparent(target, parent, key);
        tree.retire_subtree(node);
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == target)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, target);
        tree.set_children(parent, children);
        Self::reconcile_planned_view(tree, target, view, components, changes, plan)
    }

    fn wrap_flyout(
        tree: &mut Tree,
        node: NodeId,
        target: View,
        flyout: Flyout,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        if Self::flyout_owner(tree, node)?.is_some() || Self::tooltip_owner(tree, node)?.is_some() {
            return Err(PumpError::StructureUnsupported);
        }
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let target = Self::reconcile_planned_view(tree, node, target, components, changes, plan)?;
        if tree.subtree_postorder(target).into_iter().any(|child| {
            matches!(
                tree.kind(child),
                NodeKind::Tooltip(_) | NodeKind::Flyout(_) | NodeKind::ContentDialog(_)
            )
        }) {
            return Err(PumpError::StructureUnsupported);
        }

        let placement = flyout.placement;
        let owner = tree.insert_flyout(Some(parent), key, placement);
        tree.reparent(target, owner, None);
        let (_, content_native) = Self::mount_planned_view(
            tree,
            Some(owner),
            None,
            *flyout.content,
            components,
            changes,
            plan,
        )?;
        let [content_native] = content_native.as_slice() else {
            return Err(PumpError::StructureUnsupported);
        };
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == owner)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, owner);
        tree.set_children(parent, children);
        Self::refresh_flyout_attachment(tree, owner, placement, plan)?;
        debug_assert_eq!(
            tree.flyout_attachment(owner),
            Some((Self::native_root(tree, target)?, *content_native))
        );
        Ok(owner)
    }

    fn unwrap_flyout(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let [target, content] = tree.children(node) else {
            return Err(PumpError::StructureUnsupported);
        };
        let target = *target;
        let content = *content;

        Self::clear_flyout_attachment(tree, node, plan)?;
        Self::collect_retired_components(tree, content, components, changes);
        Self::retire_planned_subtree(tree, content, plan)?;
        tree.reparent(target, parent, key);
        tree.retire_subtree(node);
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == target)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, target);
        tree.set_children(parent, children);
        Self::reconcile_planned_view(tree, target, view, components, changes, plan)
    }

    fn replace_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let attachment = Self::native_attachment(tree, node)?;
        let realized = match attachment {
            NativeAttachment::ChildList {
                parent, slot: None, ..
            } if tree.kind(parent) == NodeKind::VirtualCollection => {
                Self::virtual_row_owner(tree, node)?
            }
            _ => None,
        };
        let tooltip_owner = if let Some(owner) = Self::tooltip_owner(tree, node)? {
            let attachment = tree
                .tooltip_attachment(owner)
                .ok_or(PumpError::StructureUnsupported)?;
            Self::native_roots_intersect(tree, node, attachment.0, attachment.1)?.then_some(owner)
        } else {
            None
        };
        let flyout_owner = if let Some(owner) = Self::flyout_owner(tree, node)? {
            let attachment = tree
                .flyout_attachment(owner)
                .ok_or(PumpError::StructureUnsupported)?;
            Self::native_roots_intersect(tree, node, attachment.0, attachment.1)?.then_some(owner)
        } else {
            None
        };
        let owned_attachment_owner = Self::owned_attachment_owner(tree, node)?;
        if let Some(owner) = tooltip_owner {
            Self::clear_tooltip_attachment(tree, owner, plan)?;
        }
        if let Some(owner) = flyout_owner {
            Self::clear_flyout_attachment(tree, owner, plan)?;
        }
        if let Some(owner) = owned_attachment_owner {
            Self::clear_owned_attachment(tree, owner, plan)?;
        }

        Self::collect_retired_components(tree, node, components, changes);
        Self::retire_planned_subtree(tree, node, plan)?;
        let (replacement, native) =
            Self::mount_planned_view(tree, Some(parent), key, view, components, changes, plan)?;
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children);
        match (attachment, realized) {
            (
                NativeAttachment::ChildList { slot: None, .. },
                Some((collection, container, row)),
            ) => {
                let logical_root = if row.logical_root == node {
                    replacement
                } else {
                    row.logical_root
                };
                if row.logical_root == node {
                    tree.set_realized(collection, container, row.index, logical_root, None);
                } else {
                    tree.update_realized(collection, container, logical_root, None);
                }
                Self::refresh_virtual_row_attachment(tree, collection, container, plan)?;
            }
            (
                NativeAttachment::ChildList {
                    parent,
                    slot: None,
                    index: native_index,
                },
                None,
            ) => {
                let native_children = Self::native_children(tree, parent)?;
                Self::validate_native_arity(tree, parent, &native_children)?;
                for (index, child) in native.into_iter().enumerate() {
                    plan.push(Command::InsertChild {
                        parent,
                        slot: None,
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
            (
                NativeAttachment::Tooltip {
                    owner, placement, ..
                },
                None,
            ) => {
                Self::refresh_tooltip_attachment(tree, owner, placement, plan)?;
            }
            (NativeAttachment::Flyout { owner, placement }, None) => {
                Self::refresh_flyout_attachment(tree, owner, placement, plan)?;
            }
            (
                NativeAttachment::ChildList {
                    parent,
                    slot: Some(slot),
                    index: native_index,
                },
                None,
            ) => {
                if native.len() != 1 {
                    return Err(PumpError::StructureUnsupported);
                }
                plan.push(Command::InsertChild {
                    parent,
                    slot: Some(slot),
                    child: native[0],
                    index: native_index,
                });
            }
            (NativeAttachment::ChildList { slot: Some(_), .. }, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
            (NativeAttachment::Slot { .. }, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
            (NativeAttachment::Tooltip { .. }, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
            (NativeAttachment::Flyout { .. }, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
            (NativeAttachment::ContentDialog, None) => {}
            (NativeAttachment::ContentDialog, Some(_)) => {
                return Err(PumpError::StructureUnsupported);
            }
        }
        if let Some(owner) = tooltip_owner
            && !matches!(attachment, NativeAttachment::Tooltip { .. })
        {
            let NodeKind::Tooltip(placement) = tree.kind(owner) else {
                return Err(PumpError::StructureUnsupported);
            };
            Self::refresh_tooltip_attachment(tree, owner, placement, plan)?;
        }
        if let Some(owner) = flyout_owner
            && !matches!(attachment, NativeAttachment::Flyout { .. })
        {
            let NodeKind::Flyout(placement) = tree.kind(owner) else {
                return Err(PumpError::StructureUnsupported);
            };
            Self::refresh_flyout_attachment(tree, owner, placement, plan)?;
        }
        if let Some(owner) = owned_attachment_owner {
            Self::refresh_owned_attachment(tree, owner, plan)?;
        }
        Ok(replacement)
    }

    pub(in super::super) fn collect_retired_components(
        tree: &mut Tree,
        root: NodeId,
        components: &ComponentStore,
        changes: &mut ComponentChanges,
    ) {
        for node in tree.subtree_postorder(root) {
            if tree.kind(node) == NodeKind::Component {
                let scope = tree.component_scope(node);
                let token = components.token(scope);
                if !changes.retired.contains(&token) {
                    changes.retired.push(token);
                }
                tree.set_window_title(scope, None);
                tree.set_window_visuals(scope, None);
                tree.set_window_size_observation(scope, None);
                tree.set_color_scheme_observation(scope, None);
                tree.remove_window_declarations(scope);
            }
        }
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
        let ComponentRender {
            color_scheme_observation,
            dependencies,
            view,
            window_size_observation,
            window_title,
            window_visuals,
        } = components.view(token, tree.context_snapshot(node))?;
        Self::reconcile_component_window_title(tree, token, window_title);
        Self::reconcile_component_window_visuals(tree, token, window_visuals);
        tree.set_window_size_observation(token.scope(), window_size_observation);
        tree.set_color_scheme_observation(token.scope(), color_scheme_observation);
        changes.context_reads.insert(token, dependencies);
        Self::recompose_component_view(tree, node, view, components, changes, plan)
    }

    pub(in super::super) fn reconcile_component_window_title(
        tree: &mut Tree,
        token: ComponentToken,
        title: Option<String>,
    ) {
        let scope = token.scope();
        tree.set_window_title(scope, title.map(Into::into));
    }

    pub(in super::super) fn reconcile_component_window_visuals(
        tree: &mut Tree,
        token: ComponentToken,
        visuals: Option<WindowVisuals>,
    ) {
        tree.set_window_visuals(token.scope(), visuals);
    }

    pub(in super::super) fn recompose_component_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let [slot] = tree.children(node) else {
            return Err(PumpError::StructureUnsupported);
        };
        if tree.kind(*slot) != NodeKind::Slot {
            return Err(PumpError::StructureUnsupported);
        }
        let [child] = tree.children(*slot) else {
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
        if Self::view_uses_native_control(view.as_kind(), MountedKind::ToolTip)
            && !Self::tooltip_implementation_mount_allowed(tree, logical_parent)
        {
            return Err(PumpError::StructureUnsupported);
        }
        if Self::view_uses_native_control(view.as_kind(), MountedKind::ContentDialog)
            && !Self::content_dialog_implementation_mount_allowed(tree, logical_parent)
        {
            return Err(PumpError::StructureUnsupported);
        }
        match view.into_kind() {
            ViewKind::Native(element) => {
                let node = Self::mount_planned_element(tree, logical_parent, key, element, plan)?;
                Ok((node, vec![node]))
            }
            ViewKind::Component(component) => {
                let token = component.reserve(components);
                changes
                    .host_requests
                    .extend(components.take_host_requests());
                changes.reserved.push(token);
                let node = tree.insert_component(
                    logical_parent,
                    key,
                    token.scope(),
                    component.component_type(),
                );
                let slot = tree.insert(Some(node), NodeKind::Slot);
                let ComponentRender {
                    color_scheme_observation,
                    dependencies,
                    view,
                    window_size_observation,
                    window_title,
                    window_visuals,
                } = components.view(token, tree.context_snapshot(node))?;
                Self::reconcile_component_window_title(tree, token, window_title);
                Self::reconcile_component_window_visuals(tree, token, window_visuals);
                tree.set_window_size_observation(token.scope(), window_size_observation);
                tree.set_color_scheme_observation(token.scope(), color_scheme_observation);
                changes.context_reads.insert(token, dependencies);
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(slot),
                    None,
                    view,
                    components,
                    changes,
                    plan,
                )?;
                Ok((node, native))
            }
            ViewKind::Fragment(children) => {
                let node = tree.insert_fragment(logical_parent, key);
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys).map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
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
                let node = tree.insert_provider(logical_parent, key, provision);
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
            ViewKind::Tooltip { target, tooltip } => {
                let mut current = logical_parent;
                while let Some(parent) = current {
                    if matches!(
                        tree.kind(parent),
                        NodeKind::Tooltip(_) | NodeKind::Flyout(_) | NodeKind::ContentDialog(_)
                    ) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = tree.parent(parent);
                }
                let placement = tooltip.placement;
                let node = tree.insert_tooltip(logical_parent, key, placement);
                let (_, target_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let [target] = target_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                let (_, tooltip_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    ToolTip::new().content(*tooltip.content),
                    components,
                    changes,
                    plan,
                )?;
                let [tooltip] = tooltip_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                tree.set_tooltip_attachment(node, Some((*target, *tooltip)));
                plan.push(Command::SetTooltip {
                    target: *target,
                    tooltip: Some(*tooltip),
                    placement,
                });
                Ok((node, vec![*target]))
            }
            ViewKind::Flyout { target, flyout } => {
                let mut current = logical_parent;
                while let Some(parent) = current {
                    if matches!(
                        tree.kind(parent),
                        NodeKind::Tooltip(_) | NodeKind::Flyout(_) | NodeKind::ContentDialog(_)
                    ) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = tree.parent(parent);
                }
                let placement = flyout.placement;
                let node = tree.insert_flyout(logical_parent, key, placement);
                let (_, target_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let [target] = target_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                let (_, content_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    *flyout.content,
                    components,
                    changes,
                    plan,
                )?;
                let [content] = content_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                if !matches!(
                    tree.kind(*target),
                    NodeKind::Native(MountedKind::Button | MountedKind::SplitButton)
                ) {
                    return Err(PumpError::StructureUnsupported);
                }
                tree.set_flyout_attachment(node, Some((*target, *content)));
                plan.push(Command::SetFlyout {
                    target: *target,
                    content: Some(*content),
                    placement,
                });
                Ok((node, vec![*target]))
            }
            ViewKind::Menu { target, menu } => {
                let mut current = logical_parent;
                while let Some(parent) = current {
                    if matches!(
                        tree.kind(parent),
                        NodeKind::Tooltip(_)
                            | NodeKind::Flyout(_)
                            | NodeKind::Menu(_)
                            | NodeKind::CommandBarFlyout
                            | NodeKind::ContentDialog(_)
                    ) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = tree.parent(parent);
                }
                validate_menu_items(&menu.items)?;
                let node = tree.insert_menu(logical_parent, key, OwnedMenuKind::ButtonFlyout, menu);
                let (_, target_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let [target] = target_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                let kind = match tree.kind(*target) {
                    NodeKind::Native(MountedKind::Button) => OwnedMenuKind::ButtonFlyout,
                    NodeKind::Native(MountedKind::DropDownButton) => {
                        OwnedMenuKind::DropDownButtonFlyout
                    }
                    NodeKind::Native(MountedKind::MenuBarItem) => OwnedMenuKind::MenuBarItem,
                    _ => return Err(PumpError::StructureUnsupported),
                };
                tree.set_kind(node, NodeKind::Menu(kind));
                let menu = tree.owned_menu(node).to_vec();
                plan.push(Command::SetOwnedMenu {
                    owner: node,
                    target: *target,
                    kind,
                    items: Some(menu),
                    revision: 1,
                });
                Ok((node, vec![*target]))
            }
            ViewKind::CommandBarFlyout { target, flyout } => {
                validate_commands(&flyout.primary)?;
                validate_commands(&flyout.secondary)?;
                let primary = flyout.primary.clone();
                let secondary = flyout.secondary.clone();
                let node = tree.insert_command_bar_flyout(logical_parent, key, flyout);
                let (_, target_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*target),
                    components,
                    changes,
                    plan,
                )?;
                let [target] = target_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                if tree.kind(*target) != NodeKind::Native(MountedKind::Button) {
                    return Err(PumpError::StructureUnsupported);
                }
                plan.push(Command::SetCommandBarFlyout {
                    owner: node,
                    target: *target,
                    primary: Some(primary),
                    secondary,
                    revision: 1,
                });
                Ok((node, vec![*target]))
            }
            ViewKind::TreeNodes {
                tree: desired,
                nodes,
            } => {
                validate_tree_nodes(&nodes)?;
                let node = tree.insert_tree_nodes(logical_parent, key, Rc::clone(&nodes));
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*desired),
                    components,
                    changes,
                    plan,
                )?;
                let [target] = native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                if tree.kind(*target) != NodeKind::Native(MountedKind::TreeView) {
                    return Err(PumpError::StructureUnsupported);
                }
                plan.push(Command::SetTreeViewNodes {
                    target: *target,
                    nodes: nodes.as_ref().clone(),
                });
                Ok((node, vec![*target]))
            }
            ViewKind::ContentDialog { dialog, open } => {
                if !Self::content_dialog_owner_allowed(tree, logical_parent) {
                    return Err(PumpError::StructureUnsupported);
                }
                let mut current = logical_parent;
                while let Some(parent) = current {
                    if matches!(
                        tree.kind(parent),
                        NodeKind::Tooltip(_) | NodeKind::Flyout(_) | NodeKind::ContentDialog(_)
                    ) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = tree.parent(parent);
                }
                let node = tree.insert_content_dialog(logical_parent, key, open);
                let (_, dialog_native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    View::from_kind(*dialog),
                    components,
                    changes,
                    plan,
                )?;
                let [dialog] = dialog_native.as_slice() else {
                    return Err(PumpError::StructureUnsupported);
                };
                if tree.kind(*dialog) != NodeKind::Native(MountedKind::ContentDialog) {
                    return Err(PumpError::StructureUnsupported);
                }
                if open {
                    plan.post_publish_commands
                        .push(Command::SetContentDialogOpen {
                            node: *dialog,
                            owner: Self::native_container(tree, node)?,
                            open: true,
                        });
                }
                Ok((node, Vec::new()))
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
                    let slot_node = tree.insert(Some(node), NodeKind::NamedSlot(*slot));
                    if slot_is_collection(*slot) {
                        let children = match desired.remove(slot) {
                            Some(SlotContent::Collection(children)) => {
                                Rc::unwrap_or_clone(children)
                            }
                            Some(SlotContent::Single(_)) => {
                                return Err(PumpError::StructureUnsupported);
                            }
                            None => Vec::new(),
                        };
                        let keys = children
                            .iter()
                            .map(|child| child.key().clone())
                            .collect::<Vec<_>>();
                        diff(&[], &keys)
                            .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
                        for (index, child) in children.into_iter().enumerate() {
                            let (key, view) = child.into_parts();
                            let (_, native) = Self::mount_planned_view(
                                tree,
                                Some(slot_node),
                                Some(key),
                                view,
                                components,
                                changes,
                                plan,
                            )?;
                            let [child] = native.as_slice() else {
                                return Err(PumpError::StructureUnsupported);
                            };
                            plan.push(Command::InsertChild {
                                parent: node,
                                slot: Some(*slot),
                                child: *child,
                                index,
                            });
                        }
                    } else {
                        let view = match desired.remove(slot) {
                            Some(SlotContent::Single(view)) => view,
                            Some(SlotContent::Collection(_)) => {
                                return Err(PumpError::StructureUnsupported);
                            }
                            None => View::empty(),
                        };
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
                            slot: None,
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
                diff(&[], &keys).map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
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
                            slot: None,
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
    match tree.kind(node) {
        NodeKind::Native(kind) => Ok(kind),
        _ => Err(PumpError::StructureUnsupported),
    }
}

fn collect_desired_slots(
    slots: Rc<Vec<SlottedView>>,
) -> Result<HashMap<SlotId, SlotContent>, PumpError> {
    let mut desired = HashMap::new();
    for slot in Rc::unwrap_or_clone(slots) {
        if desired.insert(slot.slot, slot.content).is_some() {
            return Err(PumpError::StructureUnsupported);
        }
    }
    Ok(desired)
}
