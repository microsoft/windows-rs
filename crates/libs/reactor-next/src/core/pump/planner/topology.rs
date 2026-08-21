//! Tree topology helpers shared by element and view planning: native-root
//! lookup, native parent/location/children queries, arity validation, subtree
//! and subtree retirement.

use super::super::*;

#[derive(Clone, Copy)]
pub(super) enum NativeAttachment {
    Children { parent: NodeId, index: usize },
    Slot { parent: NodeId, slot: SlotId },
}

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn control_has_role(kind: MountedKind, role: ControlRole) -> bool {
        CONTROLS
            .iter()
            .find(|control| control.kind == kind)
            .is_some_and(|control| control.role == role)
    }

    pub(in super::super) fn native_root(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let roots = Self::native_roots(tree, node)?;
        let [root] = roots.as_slice() else {
            return Err(PumpError::StructureUnsupported);
        };
        Ok(*root)
    }

    pub(super) fn native_roots(tree: &Tree, node: NodeId) -> Result<Vec<NodeId>, PumpError> {
        match tree.kind(node)? {
            NodeKind::Native(_) | NodeKind::VirtualCollection => Ok(vec![node]),
            NodeKind::Component
            | NodeKind::Fragment
            | NodeKind::Provider
            | NodeKind::Slot
            | NodeKind::NamedSlot(_) => {
                let mut roots = Vec::new();
                for child in tree.children(node)?.iter().copied() {
                    roots.extend(Self::native_roots(tree, child)?);
                }
                Ok(roots)
            }
            NodeKind::Application | NodeKind::Window => Err(PumpError::StructureUnsupported),
        }
    }

    fn native_container(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let mut current = node;
        loop {
            match tree.kind(current)? {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => return Ok(current),
                NodeKind::Component
                | NodeKind::Fragment
                | NodeKind::Provider
                | NodeKind::Slot
                | NodeKind::NamedSlot(_) => {
                    current = tree
                        .parent(current)?
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
                .parent(current)?
                .ok_or(PumpError::StructureUnsupported)?;
            for sibling in tree.children(parent)? {
                if *sibling == current {
                    break;
                }
                offset += Self::native_roots(tree, *sibling)?.len();
            }
            match tree.kind(parent)? {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => {
                    return Ok(NativeAttachment::Children {
                        parent,
                        index: offset,
                    });
                }
                NodeKind::NamedSlot(slot) => {
                    if offset != 0 || Self::native_roots(tree, parent)?.len() > 1 {
                        return Err(PumpError::StructureUnsupported);
                    }
                    let parent = Self::native_container(tree, parent)?;
                    if !matches!(tree.kind(parent)?, NodeKind::Native(_)) {
                        return Err(PumpError::StructureUnsupported);
                    }
                    return Ok(NativeAttachment::Slot { parent, slot });
                }
                NodeKind::Component | NodeKind::Fragment | NodeKind::Provider | NodeKind::Slot => {
                    current = parent;
                }
            }
        }
    }

    pub(super) fn native_children(tree: &Tree, parent: NodeId) -> Result<Vec<NodeId>, PumpError> {
        let mut native = Vec::new();
        for child in tree.children(parent)?.iter().copied() {
            if matches!(tree.kind(child)?, NodeKind::NamedSlot(_)) {
                continue;
            }
            native.extend(Self::native_roots(tree, child)?);
        }
        Ok(native)
    }

    pub(super) fn validate_native_arity(
        tree: &Tree,
        parent: NodeId,
        native: &[NodeId],
    ) -> Result<(), PumpError> {
        let allows_many = match tree.kind(parent)? {
            NodeKind::Native(kind) => Self::control_has_role(kind, ControlRole::Children),
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
        let nodes = tree.subtree_postorder(root)?;
        plan.commits.retain(|commit| !nodes.contains(&commit.node));
        plan.reference_commits
            .retain(|commit| !nodes.contains(&commit.node));
        for node in nodes {
            match tree.kind(node)? {
                NodeKind::Native(_) => {
                    if let Some(reference) = tree.native(node)?.reference.clone() {
                        plan.reference_commits.push(ReferenceCommit {
                            node,
                            old: Some(reference),
                            new: None,
                        });
                    }
                    if tree.parent(node)?.is_some() {
                        match Self::native_attachment(tree, node)? {
                            NativeAttachment::Children { parent, .. }
                                if tree.kind(parent)? == NodeKind::VirtualCollection =>
                            {
                                let container = tree
                                    .realized_container(parent, node)?
                                    .ok_or(PumpError::StructureUnsupported)?;
                                plan.push(Command::DetachRealized {
                                    collection: parent,
                                    container,
                                    child: node,
                                });
                            }
                            NativeAttachment::Children { parent, .. } => {
                                plan.push(Command::RemoveChild {
                                    parent,
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
                        }
                    }
                    for (event, state) in &tree.native(node)?.events {
                        if state.active {
                            plan.push(Command::UnsubscribeEvent {
                                node,
                                event: *event,
                            });
                        }
                    }
                    plan.push(Command::Destroy { node });
                }
                NodeKind::VirtualCollection => {
                    if tree.parent(node)?.is_some() {
                        match Self::native_attachment(tree, node)? {
                            NativeAttachment::Children { parent, .. } => {
                                plan.push(Command::RemoveChild {
                                    parent,
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
                        }
                    }
                    plan.push(Command::Destroy { node });
                }
                NodeKind::Component
                | NodeKind::Fragment
                | NodeKind::Provider
                | NodeKind::Slot
                | NodeKind::NamedSlot(_) => {}
                NodeKind::Application | NodeKind::Window => {
                    return Err(PumpError::StructureUnsupported);
                }
            }
        }
        tree.retire_subtree(root)?;
        Ok(())
    }
}
