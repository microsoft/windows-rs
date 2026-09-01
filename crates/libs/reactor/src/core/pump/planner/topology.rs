//! Tree topology helpers shared by element and view planning: native-root
//! lookup, native parent/location/children queries, arity validation, subtree
//! and subtree retirement.

use super::super::*;

#[derive(Clone, Copy)]
pub(super) enum NativeAttachment {
    ChildList {
        parent: NodeId,
        slot: Option<SlotId>,
        index: usize,
    },
    Slot {
        parent: NodeId,
        slot: SlotId,
    },
    Tooltip {
        owner: NodeId,
        placement: TooltipPlacement,
    },
    Flyout {
        owner: NodeId,
        placement: FlyoutPlacement,
    },
    ContentDialog,
}

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn control_has_role(kind: MountedKind, role: ControlRole) -> bool {
        CONTROLS
            .iter()
            .find(|control| control.kind == kind)
            .is_some_and(|control| control.role == role)
    }

    pub(in super::super) fn native_root(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let mut root = None;
        let mut multiple = false;
        Self::visit_native_roots(tree, node, &mut |current| {
            if root.replace(current).is_some() {
                multiple = true;
            }
        })?;
        if multiple {
            return Err(PumpError::StructureUnsupported);
        }
        root.ok_or(PumpError::StructureUnsupported)
    }

    pub(super) fn native_roots(tree: &Tree, node: NodeId) -> Result<Vec<NodeId>, PumpError> {
        let mut roots = Vec::new();
        Self::visit_native_roots(tree, node, &mut |root| roots.push(root))?;
        Ok(roots)
    }

    fn visit_native_roots(
        tree: &Tree,
        node: NodeId,
        visit: &mut impl FnMut(NodeId),
    ) -> Result<(), PumpError> {
        match tree.kind(node) {
            NodeKind::Native(_) | NodeKind::VirtualCollection => visit(node),
            NodeKind::Tooltip(_) => {
                let [target, _tooltip] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::visit_native_roots(tree, *target, visit)?;
            }
            NodeKind::Flyout(_) => {
                let [target, _content] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::visit_native_roots(tree, *target, visit)?;
            }
            NodeKind::Menu(_) | NodeKind::CommandBarFlyout | NodeKind::TreeNodes => {
                let [target] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::visit_native_roots(tree, *target, visit)?;
            }
            NodeKind::ContentDialog(_) => {}
            NodeKind::Component
            | NodeKind::Fragment
            | NodeKind::Provider
            | NodeKind::Slot
            | NodeKind::NamedSlot(_) => {
                for child in tree.children(node).iter().copied() {
                    Self::visit_native_roots(tree, child, visit)?;
                }
            }
            NodeKind::Application | NodeKind::Window => {
                return Err(PumpError::StructureUnsupported);
            }
        }
        Ok(())
    }

    pub(super) fn native_root_count(tree: &Tree, node: NodeId) -> Result<usize, PumpError> {
        let mut count = 0;
        Self::visit_native_roots(tree, node, &mut |_| count += 1)?;
        Ok(count)
    }

    pub(super) fn native_roots_intersect(
        tree: &Tree,
        node: NodeId,
        first: NodeId,
        second: NodeId,
    ) -> Result<bool, PumpError> {
        let mut found = false;
        Self::visit_native_roots(tree, node, &mut |root| {
            found |= root == first || root == second;
        })?;
        Ok(found)
    }

    pub(super) fn native_container(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let mut current = node;
        loop {
            match tree.kind(current) {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => return Ok(current),
                NodeKind::Component
                | NodeKind::Fragment
                | NodeKind::Provider
                | NodeKind::Slot
                | NodeKind::NamedSlot(_)
                | NodeKind::Tooltip(_)
                | NodeKind::Flyout(_)
                | NodeKind::Menu(_)
                | NodeKind::CommandBarFlyout
                | NodeKind::TreeNodes
                | NodeKind::ContentDialog(_) => {
                    current = tree
                        .parent(current)
                        .ok_or(PumpError::StructureUnsupported)?;
                }
            }
        }
    }

    pub(super) fn native_attachment(
        tree: &Tree,
        node: NodeId,
    ) -> Result<NativeAttachment, PumpError> {
        let mut current = node;
        let mut offset = 0;
        loop {
            let parent = tree
                .parent(current)
                .ok_or(PumpError::StructureUnsupported)?;
            for sibling in tree.children(parent) {
                if *sibling == current {
                    break;
                }
                offset += Self::native_root_count(tree, *sibling)?;
            }
            match tree.kind(parent) {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => {
                    return Ok(NativeAttachment::ChildList {
                        parent,
                        slot: None,
                        index: offset,
                    });
                }
                NodeKind::NamedSlot(slot) => {
                    let native_parent = Self::native_container(tree, parent)?;
                    if !matches!(tree.kind(native_parent), NodeKind::Native(_)) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    if slot_is_collection(slot) {
                        return Ok(NativeAttachment::ChildList {
                            parent: native_parent,
                            slot: Some(slot),
                            index: offset,
                        });
                    }
                    if offset != 0 || Self::native_root_count(tree, parent)? > 1 {
                        return Err(PumpError::StructureUnsupported);
                    }
                    return Ok(NativeAttachment::Slot {
                        parent: native_parent,
                        slot,
                    });
                }
                NodeKind::Tooltip(placement) => {
                    let [target, tooltip] = tree.children(parent) else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    if current == *tooltip {
                        return Ok(NativeAttachment::Tooltip {
                            owner: parent,
                            placement,
                        });
                    }
                    if current != *target {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = parent;
                }
                NodeKind::Flyout(placement) => {
                    let [target, content] = tree.children(parent) else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    if current == *content {
                        return Ok(NativeAttachment::Flyout {
                            owner: parent,
                            placement,
                        });
                    }
                    if current != *target {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = parent;
                }
                NodeKind::Menu(_) | NodeKind::CommandBarFlyout | NodeKind::TreeNodes => {
                    let [target] = tree.children(parent) else {
                        return Err(PumpError::StructureUnsupported);
                    };
                    if current != *target {
                        return Err(PumpError::StructureUnsupported);
                    }
                    current = parent;
                }
                NodeKind::ContentDialog(_) => return Ok(NativeAttachment::ContentDialog),
                NodeKind::Component | NodeKind::Fragment | NodeKind::Provider | NodeKind::Slot => {
                    current = parent;
                }
            }
        }
    }

    pub(super) fn native_children(tree: &Tree, parent: NodeId) -> Result<Vec<NodeId>, PumpError> {
        let mut native = Vec::new();
        for child in tree.children(parent).iter().copied() {
            if matches!(tree.kind(child), NodeKind::NamedSlot(_)) {
                continue;
            }
            Self::visit_native_roots(tree, child, &mut |root| native.push(root))?;
        }
        Ok(native)
    }

    pub(super) fn collection_slot_node(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let mut current = node;
        loop {
            let parent = tree
                .parent(current)
                .ok_or(PumpError::StructureUnsupported)?;
            if let NodeKind::NamedSlot(slot) = tree.kind(parent)
                && slot_is_collection(slot)
            {
                return Ok(parent);
            }
            current = parent;
        }
    }

    pub(super) fn virtual_row_owner(
        tree: &Tree,
        node: NodeId,
    ) -> Result<Option<(NodeId, RealizedContainer, RealizedRow)>, PumpError> {
        let mut logical_root = node;
        while let Some(parent) = tree.parent(logical_root) {
            if tree.kind(parent) == NodeKind::VirtualCollection {
                let Some(container) = tree.realized_container_for_logical(parent, logical_root)
                else {
                    return Ok(None);
                };
                let row = tree
                    .realized(parent, container)
                    .ok_or(PumpError::StructureUnsupported)?;
                return Ok(Some((parent, container, row)));
            }
            logical_root = parent;
        }
        Ok(None)
    }

    pub(in crate::core::pump) fn refresh_virtual_row_attachment(
        tree: &mut Tree,
        collection: NodeId,
        container: RealizedContainer,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let row = tree
            .realized(collection, container)
            .ok_or(PumpError::StructureUnsupported)?;
        let roots = Self::native_roots(tree, row.logical_root)?;
        let next = match roots.as_slice() {
            [root] => Some(*root),
            _ => None,
        };
        if let Some(root) = next
            && let Some(owner) = tree.realized_container(collection, root)
            && owner != container
        {
            return Err(PumpError::StructureUnsupported);
        }
        if row.native_root != next {
            match (row.native_root, next) {
                (_, Some(child)) => {
                    plan.push(Command::AttachRealized {
                        collection,
                        container,
                        child,
                    });
                }
                (Some(child), None) => {
                    plan.push(Command::DetachRealized {
                        collection,
                        container,
                        child,
                    });
                }
                (None, None) => {}
            }
            tree.update_realized(collection, container, row.logical_root, next);
        }
        let key = tree
            .key(row.logical_root)
            .cloned()
            .ok_or(PumpError::StructureUnsupported)?;
        plan.diagnostics.retain(|diagnostic| {
            !matches!(
                diagnostic,
                PumpDiagnostic::VirtualRowRootCount {
                    collection: current,
                    key: current_key,
                    ..
                } if *current == collection && current_key == &key
            )
        });
        if roots.len() > 1 {
            plan.diagnostics.push(PumpDiagnostic::VirtualRowRootCount {
                collection,
                key,
                actual: roots.len(),
            });
        }
        Ok(())
    }

    pub(super) fn tooltip_owner(tree: &Tree, node: NodeId) -> Result<Option<NodeId>, PumpError> {
        let mut current = node;
        while let Some(parent) = tree.parent(current) {
            if matches!(tree.kind(parent), NodeKind::Tooltip(_)) {
                let [target, tooltip] = tree.children(parent) else {
                    return Err(PumpError::StructureUnsupported);
                };
                if current == *target || current == *tooltip {
                    return Ok(Some(parent));
                }
                return Err(PumpError::StructureUnsupported);
            }
            current = parent;
        }
        Ok(None)
    }

    pub(super) fn flyout_owner(tree: &Tree, node: NodeId) -> Result<Option<NodeId>, PumpError> {
        let mut current = node;
        while let Some(parent) = tree.parent(current) {
            if matches!(tree.kind(parent), NodeKind::Flyout(_)) {
                let [target, content] = tree.children(parent) else {
                    return Err(PumpError::StructureUnsupported);
                };
                if current == *target || current == *content {
                    return Ok(Some(parent));
                }
                return Err(PumpError::StructureUnsupported);
            }
            current = parent;
        }
        Ok(None)
    }

    pub(super) fn owned_attachment_owner(
        tree: &Tree,
        node: NodeId,
    ) -> Result<Option<NodeId>, PumpError> {
        let mut current = node;
        while let Some(parent) = tree.parent(current) {
            if matches!(
                tree.kind(parent),
                NodeKind::Menu(_) | NodeKind::CommandBarFlyout
            ) {
                let [target] = tree.children(parent) else {
                    return Err(PumpError::StructureUnsupported);
                };
                return if current == *target {
                    Ok(Some(parent))
                } else {
                    Err(PumpError::StructureUnsupported)
                };
            }
            current = parent;
        }
        Ok(None)
    }

    pub(super) fn clear_tooltip_attachment(
        tree: &mut Tree,
        owner: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let NodeKind::Tooltip(placement) = tree.kind(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        if let Some((target, _)) = tree.tooltip_attachment(owner) {
            plan.push(Command::SetTooltip {
                target,
                tooltip: None,
                placement,
            });
            tree.set_tooltip_attachment(owner, None);
        }
        Ok(())
    }

    pub(super) fn refresh_tooltip_attachment(
        tree: &mut Tree,
        owner: NodeId,
        placement: TooltipPlacement,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let NodeKind::Tooltip(previous_placement) = tree.kind(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let [target, tooltip] = tree.children(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let attachment = (
            Self::native_root(tree, *target)?,
            Self::native_root(tree, *tooltip)?,
        );
        let previous = tree.tooltip_attachment(owner);
        if let Some((previous_target, _)) = previous
            && previous_target != attachment.0
        {
            plan.push(Command::SetTooltip {
                target: previous_target,
                tooltip: None,
                placement: previous_placement,
            });
        }
        if previous != Some(attachment) || previous_placement != placement {
            plan.push(Command::SetTooltip {
                target: attachment.0,
                tooltip: Some(attachment.1),
                placement,
            });
            tree.set_tooltip_attachment(owner, Some(attachment));
        }
        tree.set_tooltip_placement(owner, placement);
        Ok(())
    }

    pub(super) fn clear_flyout_attachment(
        tree: &mut Tree,
        owner: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let NodeKind::Flyout(placement) = tree.kind(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        if let Some((target, _)) = tree.flyout_attachment(owner) {
            plan.push(Command::SetFlyout {
                target,
                content: None,
                placement,
            });
            tree.set_flyout_attachment(owner, None);
        }
        Ok(())
    }

    pub(super) fn clear_owned_attachment(
        tree: &Tree,
        owner: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let [target] = tree.children(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let target = Self::native_root(tree, *target)?;
        match tree.kind(owner) {
            NodeKind::Menu(kind) => {
                plan.push(Command::SetOwnedMenu {
                    owner,
                    target,
                    kind,
                    items: None,
                    revision: tree.owned_revision(owner),
                });
            }
            NodeKind::CommandBarFlyout => {
                plan.push(Command::SetCommandBarFlyout {
                    owner,
                    target,
                    primary: None,
                    secondary: Vec::new(),
                    revision: tree.owned_revision(owner),
                });
            }
            _ => return Err(PumpError::StructureUnsupported),
        }
        Ok(())
    }

    pub(super) fn refresh_owned_attachment(
        tree: &mut Tree,
        owner: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let [target] = tree.children(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let target = Self::native_root(tree, *target)?;
        let revision = tree.owned_revision(owner);
        match tree.kind(owner) {
            NodeKind::Menu(_) => {
                let kind = match tree.kind(target) {
                    NodeKind::Native(MountedKind::Button) => OwnedMenuKind::ButtonFlyout,
                    NodeKind::Native(MountedKind::DropDownButton) => {
                        OwnedMenuKind::DropDownButtonFlyout
                    }
                    NodeKind::Native(MountedKind::MenuBarItem) => OwnedMenuKind::MenuBarItem,
                    _ => return Err(PumpError::StructureUnsupported),
                };
                tree.set_kind(owner, NodeKind::Menu(kind));
                plan.push(Command::SetOwnedMenu {
                    owner,
                    target,
                    kind,
                    items: Some(tree.owned_menu(owner).to_vec()),
                    revision,
                });
            }
            NodeKind::CommandBarFlyout => {
                let (primary, secondary) = tree.owned_commands(owner);
                plan.push(Command::SetCommandBarFlyout {
                    owner,
                    target,
                    primary: Some(primary.clone()),
                    secondary: secondary.clone(),
                    revision,
                });
            }
            _ => return Err(PumpError::StructureUnsupported),
        };
        Ok(())
    }

    pub(super) fn refresh_flyout_attachment(
        tree: &mut Tree,
        owner: NodeId,
        placement: FlyoutPlacement,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let NodeKind::Flyout(previous_placement) = tree.kind(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let [target, content] = tree.children(owner) else {
            return Err(PumpError::StructureUnsupported);
        };
        let attachment = (
            Self::native_root(tree, *target)?,
            Self::native_root(tree, *content)?,
        );
        if !matches!(
            tree.kind(attachment.0),
            NodeKind::Native(MountedKind::Button | MountedKind::SplitButton)
        ) {
            return Err(PumpError::StructureUnsupported);
        }
        let previous = tree.flyout_attachment(owner);
        if let Some((previous_target, _)) = previous
            && previous_target != attachment.0
        {
            plan.push(Command::SetFlyout {
                target: previous_target,
                content: None,
                placement: previous_placement,
            });
        }
        if previous != Some(attachment) || previous_placement != placement {
            plan.push(Command::SetFlyout {
                target: attachment.0,
                content: Some(attachment.1),
                placement,
            });
            tree.set_flyout_attachment(owner, Some(attachment));
        }
        tree.set_flyout_placement(owner, placement);
        Ok(())
    }

    pub(super) fn validate_native_arity(
        tree: &Tree,
        parent: NodeId,
        native: &[NodeId],
    ) -> Result<(), PumpError> {
        let allows_many = match tree.kind(parent) {
            NodeKind::Native(kind) => Self::control_has_role(kind, ControlRole::Children),
            NodeKind::NamedSlot(slot) => slot_is_collection(slot),
            NodeKind::Window => false,
            _ => return Err(PumpError::StructureUnsupported),
        };
        if allows_many || native.len() <= 1 {
            Ok(())
        } else {
            Err(PumpError::StructureUnsupported)
        }
    }

    pub(in super::super) fn retire_planned_subtree(
        tree: &mut Tree,
        root: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let nodes = tree.subtree_postorder(root);
        let node_set = nodes.iter().copied().collect::<HashSet<_>>();
        let mut retained_nodes = HashSet::new();
        let mut retirements = Vec::new();
        for node in nodes.iter().copied() {
            if !matches!(tree.kind(node), NodeKind::Native(_)) {
                continue;
            }
            let Some(transition) = tree.exit_transition(node) else {
                continue;
            };
            let attachment = Self::native_attachment(tree, node)?;
            let (parent, slot) = match attachment {
                NativeAttachment::ChildList { parent, slot, .. }
                    if !node_set.contains(&parent)
                        && match slot {
                            Some(slot) => slot_is_collection(slot),
                            None => matches!(
                                tree.kind(parent),
                                NodeKind::Native(kind)
                                    if Self::control_has_role(kind, ControlRole::Children)
                            ),
                        } =>
                {
                    (parent, slot)
                }
                NativeAttachment::ChildList { parent, .. } if node_set.contains(&parent) => {
                    continue;
                }
                _ => return Err(PumpError::ExitTransitionUnsupported),
            };
            let retained = tree
                .subtree_postorder(node)
                .into_iter()
                .filter(|descendant| {
                    matches!(
                        tree.kind(*descendant),
                        NodeKind::Native(_) | NodeKind::VirtualCollection
                    )
                })
                .collect::<Vec<_>>();
            retained_nodes.extend(retained.iter().copied());
            retirements.push((node, retained, parent, slot, transition));
        }
        plan.commits.retain(|commit| !nodes.contains(&commit.node));
        plan.reference_commits
            .retain(|commit| !nodes.contains(&commit.node));
        for node in nodes.iter().copied() {
            if matches!(tree.kind(node), NodeKind::Tooltip(_)) {
                Self::clear_tooltip_attachment(tree, node, plan)?;
            }
            if matches!(tree.kind(node), NodeKind::Flyout(_)) {
                Self::clear_flyout_attachment(tree, node, plan)?;
            }
            if matches!(
                tree.kind(node),
                NodeKind::Menu(_) | NodeKind::CommandBarFlyout
            ) {
                Self::clear_owned_attachment(tree, node, plan)?;
            }
            if matches!(tree.kind(node), NodeKind::ContentDialog(true)) {
                let [dialog] = tree.children(node) else {
                    return Err(PumpError::StructureUnsupported);
                };
                plan.push(Command::SetContentDialogOpen {
                    node: *dialog,
                    owner: Self::native_container(tree, node)?,
                    open: false,
                });
            }
        }
        for node in nodes.iter().copied() {
            match tree.kind(node) {
                NodeKind::Native(_) => {
                    if let Some(reference) = tree.native(node).reference.clone() {
                        plan.reference_commits.push(ReferenceCommit {
                            node,
                            old: Some(reference),
                            new: None,
                        });
                    }
                    if !retained_nodes.contains(&node) && tree.parent(node).is_some() {
                        match Self::native_attachment(tree, node)? {
                            NativeAttachment::ChildList {
                                parent, slot: None, ..
                            } if tree.kind(parent) == NodeKind::VirtualCollection => {
                                let Some((collection, container, row)) =
                                    Self::virtual_row_owner(tree, node)?
                                else {
                                    return Err(PumpError::StructureUnsupported);
                                };
                                if row.native_root == Some(node) {
                                    plan.push(Command::DetachRealized {
                                        collection,
                                        container,
                                        child: node,
                                    });
                                    tree.update_realized(
                                        collection,
                                        container,
                                        row.logical_root,
                                        None,
                                    );
                                }
                            }
                            NativeAttachment::ChildList { parent, slot, .. } => {
                                plan.push(Command::RemoveChild {
                                    parent,
                                    slot,
                                    child: node,
                                });
                            }
                            NativeAttachment::Slot { parent, slot } => {
                                plan.push(Command::SetSlot {
                                    parent,
                                    slot,
                                    child: None,
                                });
                            }
                            NativeAttachment::Tooltip { owner, .. } => {
                                Self::clear_tooltip_attachment(tree, owner, plan)?;
                            }
                            NativeAttachment::Flyout { owner, .. } => {
                                Self::clear_flyout_attachment(tree, owner, plan)?;
                            }
                            NativeAttachment::ContentDialog => {}
                        }
                    }
                    for (event, state) in &tree.native(node).events {
                        if state.active {
                            plan.push(Command::UnsubscribeEvent {
                                node,
                                event: *event,
                            });
                        }
                    }
                    if !retained_nodes.contains(&node) {
                        plan.push(Command::Destroy { node });
                    }
                }
                NodeKind::VirtualCollection => {
                    if !retained_nodes.contains(&node) && tree.parent(node).is_some() {
                        match Self::native_attachment(tree, node)? {
                            NativeAttachment::ChildList { parent, slot, .. } => {
                                plan.push(Command::RemoveChild {
                                    parent,
                                    slot,
                                    child: node,
                                });
                            }
                            NativeAttachment::Slot { parent, slot } => {
                                plan.push(Command::SetSlot {
                                    parent,
                                    slot,
                                    child: None,
                                });
                            }
                            NativeAttachment::Tooltip { owner, .. } => {
                                Self::clear_tooltip_attachment(tree, owner, plan)?;
                            }
                            NativeAttachment::Flyout { owner, .. } => {
                                Self::clear_flyout_attachment(tree, owner, plan)?;
                            }
                            NativeAttachment::ContentDialog => {}
                        }
                    }
                    if !retained_nodes.contains(&node) {
                        plan.push(Command::Destroy { node });
                    }
                }
                NodeKind::Component
                | NodeKind::Fragment
                | NodeKind::Provider
                | NodeKind::Slot
                | NodeKind::NamedSlot(_)
                | NodeKind::Tooltip(_)
                | NodeKind::Flyout(_)
                | NodeKind::Menu(_)
                | NodeKind::CommandBarFlyout
                | NodeKind::TreeNodes
                | NodeKind::ContentDialog(_) => {}
                NodeKind::Application | NodeKind::Window => {
                    return Err(PumpError::StructureUnsupported);
                }
            }
        }
        for (root, nodes, parent, slot, transition) in retirements {
            plan.push(Command::RetireSubtree {
                root,
                nodes,
                parent,
                slot,
                transition,
            });
        }
        tree.retire_subtree(root);
        Ok(())
    }
}
