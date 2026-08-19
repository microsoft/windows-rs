use super::*;

impl<R: NativeRuntime> Engine<R> {
    pub(super) fn create_native_node(
        &mut self,
        native_kind: NativeKind,
        node_kind: NodeKind,
    ) -> Result<NodeId, EngineError> {
        let id = self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: node_kind,
            native_kind: Some(native_kind),
            mounted: None,
        });
        self.pending.push(Command::Create {
            id,
            kind: native_kind,
        });
        Ok(id)
    }

    pub fn attach(&mut self, parent: NodeId, child: NodeId) -> Result<(), EngineError> {
        let parent_node = self
            .arena
            .get(parent)
            .ok_or(EngineError::InvalidNode(parent))?;
        if matches!(parent_node.kind, NodeKind::VirtualHost { .. }) {
            return Err(EngineError::VirtualHostManaged(parent));
        }
        let index = parent_node.children.len();
        self.attach_at(parent, child, index)
    }

    pub(crate) fn reorder_children(
        &mut self,
        parent: NodeId,
        desired: &[NodeId],
    ) -> Result<(), EngineError> {
        let children = &self
            .arena
            .get(parent)
            .ok_or(EngineError::InvalidNode(parent))?
            .children;
        if children == desired {
            return Ok(());
        }
        assert!(
            children.len() == desired.len() && children.iter().all(|child| desired.contains(child))
        );

        let Some(native_parent) = self.nearest_native_ancestor(parent) else {
            self.arena.get_mut(parent).unwrap().children = desired.to_vec();
            return Ok(());
        };
        let mut working = self.projected_native_children(native_parent);
        self.arena.get_mut(parent).unwrap().children = desired.to_vec();
        let desired_roots = self.projected_native_children(native_parent);
        apply_minimal_reorder(&mut working, &desired_roots, |root, index| {
            self.pending.push(Command::Move {
                parent: native_parent,
                child: root,
                index,
            });
        });
        debug_assert_eq!(working, desired_roots);
        Ok(())
    }

    pub(crate) fn attach_at(
        &mut self,
        parent: NodeId,
        child: NodeId,
        index: usize,
    ) -> Result<(), EngineError> {
        self.attach_at_with_native_index(parent, child, index, None)
    }

    pub(crate) fn attach_appended(
        &mut self,
        parent: NodeId,
        children: &[NodeId],
    ) -> Result<(), EngineError> {
        let Some(native_parent) = self.nearest_native_ancestor(parent) else {
            for child in children {
                self.attach(parent, *child)?;
            }
            return Ok(());
        };
        let direct_grid_append = native_parent == parent
            && self.arena.get(parent).unwrap().native_kind == Some(NativeKind::Grid)
            && children
                .iter()
                .all(|child| !self.subtree_contains_owner_bound(*child));
        if !direct_grid_append {
            for child in children {
                self.attach(parent, *child)?;
            }
            return Ok(());
        }

        let mut native_index = self
            .arena
            .get(parent)
            .unwrap()
            .children
            .iter()
            .map(|child| self.projected_native_root_count(*child))
            .sum();
        for child in children {
            let root_count = self.projected_native_root_count(*child);
            let index = self.arena.get(parent).unwrap().children.len();
            self.attach_at_with_native_index(parent, *child, index, Some(native_index))?;
            native_index += root_count;
        }
        Ok(())
    }

    fn attach_at_with_native_index(
        &mut self,
        parent: NodeId,
        child: NodeId,
        index: usize,
        native_index: Option<usize>,
    ) -> Result<(), EngineError> {
        let child_parent = self
            .arena
            .get(child)
            .ok_or(EngineError::InvalidNode(child))?
            .parent;
        if let Some(existing) = child_parent {
            return Err(EngineError::ParentConflict {
                child,
                parent: existing,
            });
        }
        let child_count = self
            .arena
            .get(parent)
            .ok_or(EngineError::InvalidNode(parent))?
            .children
            .len();
        assert!(index <= child_count);
        let direct_panel_append = self
            .nearest_native_ancestor(parent)
            .and_then(|native_parent| {
                let node = self.arena.get(native_parent).unwrap();
                (native_parent == parent
                    && index == child_count
                    && node.native_kind == Some(NativeKind::Grid)
                    && !self.subtree_contains_owner_bound(child))
                .then(|| {
                    (
                        native_parent,
                        native_index.unwrap_or_else(|| {
                            self.arena
                                .get(parent)
                                .unwrap()
                                .children
                                .iter()
                                .map(|child| self.projected_native_root_count(*child))
                                .sum()
                        }),
                    )
                })
            });
        if let Some(native_parent) = self.nearest_native_ancestor(parent) {
            let native_parent_node = self.arena.get(native_parent).unwrap();
            let attachment_shape = native_parent_node
                .native_kind
                .map(NativeKind::attachment_shape);
            if attachment_shape == Some(AttachmentShape::Content)
                && (!self.projected_native_children(native_parent).is_empty()
                    || self.projected_native_roots(child).len() != 1)
            {
                return Err(EngineError::NativeParentRejectsChildren(native_parent));
            }
            if matches!(
                attachment_shape,
                Some(AttachmentShape::ContentPane | AttachmentShape::HeaderContent)
            ) {
                let branch = if parent == native_parent {
                    child
                } else {
                    self.branch_below(native_parent, parent)
                        .ok_or(EngineError::NativeParentRejectsChildren(native_parent))?
                };
                let NodeKind::StructuralSlot(slot) = self.arena.get(branch).unwrap().kind else {
                    return Err(EngineError::NativeParentRejectsChildren(native_parent));
                };
                let valid_slot = structural_slot_attachment(attachment_shape.unwrap(), slot);
                let root_count = if parent == native_parent {
                    self.projected_native_root_count(branch)
                } else {
                    self.projected_native_root_count(branch)
                        + self.projected_native_root_count(child)
                };
                let duplicate_slot = self.arena.get(native_parent).unwrap().children.iter().any(
                    |sibling| {
                        matches!(
                            self.arena.get(*sibling).map(|node| &node.kind),
                            Some(NodeKind::StructuralSlot(sibling_slot)) if *sibling_slot == slot
                        )
                    },
                );
                if valid_slot.is_none()
                    || root_count > 1
                    || (parent == native_parent && duplicate_slot)
                {
                    return Err(EngineError::NativeParentRejectsChildren(native_parent));
                }
            }
        }

        self.arena.get_mut(child).unwrap().parent = Some(parent);
        self.arena
            .get_mut(parent)
            .unwrap()
            .children
            .insert(index, child);

        if let Some(native_parent) = self.nearest_native_ancestor(parent) {
            let roots = self.projected_native_roots(child);
            if let Some((direct_parent, first_index)) = direct_panel_append {
                debug_assert_eq!(direct_parent, native_parent);
                for (offset, root) in roots.into_iter().enumerate() {
                    self.pending.push(Command::Attach {
                        parent: native_parent,
                        child: root,
                        attachment: Attachment::Child {
                            index: first_index + offset,
                        },
                    });
                }
                return Ok(());
            }
            let all_roots = self.projected_native_children(native_parent);
            let native_parent_node = self.arena.get(native_parent).unwrap();
            if native_parent_node.native_kind == Some(NativeKind::NavigationView) {
                let branch = self.branch_below(native_parent, child).unwrap();
                let NodeKind::NavigationSection(section) = self.arena.get(branch).unwrap().kind
                else {
                    return Err(EngineError::NativeParentRejectsChildren(native_parent));
                };
                let section_roots = self.projected_native_roots(branch);
                for root in roots {
                    let attachment = match section {
                        NavigationSection::Menu => Attachment::Item {
                            index: section_roots
                                .iter()
                                .position(|candidate| *candidate == root)
                                .unwrap(),
                        },
                        NavigationSection::Content => {
                            if section_roots.len() != 1 {
                                return Err(EngineError::NativeParentRejectsChildren(
                                    native_parent,
                                ));
                            }
                            Attachment::Content
                        }
                        NavigationSection::Footer => {
                            if section_roots.len() != 1 {
                                return Err(EngineError::NativeParentRejectsChildren(
                                    native_parent,
                                ));
                            }
                            Attachment::PaneFooter
                        }
                    };
                    self.pending.push(Command::Attach {
                        parent: native_parent,
                        child: root,
                        attachment,
                    });
                }
                return Ok(());
            }
            if matches!(
                native_parent_node.native_kind,
                Some(NativeKind::CommandBar | NativeKind::CommandBarFlyout)
            ) {
                let branch = self.branch_below(native_parent, child).unwrap();
                let NodeKind::CommandSection(section) = self.arena.get(branch).unwrap().kind else {
                    return Err(EngineError::NativeParentRejectsChildren(native_parent));
                };
                let section_roots = self.projected_native_roots(branch);
                for root in roots {
                    let index = section_roots
                        .iter()
                        .position(|candidate| *candidate == root)
                        .unwrap();
                    self.pending.push(Command::Attach {
                        parent: native_parent,
                        child: root,
                        attachment: Attachment::Command { section, index },
                    });
                }
                return Ok(());
            }
            if native_parent_node.native_kind.unwrap().attachment_shape() == AttachmentShape::Items
            {
                for root in roots {
                    let index = all_roots
                        .iter()
                        .position(|candidate| *candidate == root)
                        .unwrap();
                    self.pending.push(Command::Attach {
                        parent: native_parent,
                        child: root,
                        attachment: Attachment::Item { index },
                    });
                }
                return Ok(());
            }
            let fixed_attachment = match &native_parent_node.kind {
                NodeKind::VirtualHost { realized } => {
                    let branch = self.branch_below(native_parent, child).unwrap();
                    if self.virtual_empty.get(&native_parent) == Some(&branch) {
                        if roots.len() != 1 {
                            return Err(EngineError::NativeParentRejectsChildren(native_parent));
                        }
                        Some(Attachment::Header)
                    } else {
                        let Some((item_index, row)) =
                            realized.iter().find(|(_, row)| row.root == branch)
                        else {
                            return Err(EngineError::VirtualRowMissing(child));
                        };
                        if roots.len() != 1 {
                            return Err(EngineError::VirtualRowNativeRootCount {
                                host: native_parent,
                                index: *item_index,
                                count: roots.len(),
                            });
                        }
                        Some(Attachment::VirtualItem {
                            index: *item_index,
                            lease: row.lease,
                        })
                    }
                }
                _ => match native_parent_node.native_kind.unwrap().attachment_shape() {
                    AttachmentShape::Children => None,
                    AttachmentShape::Items => unreachable!(),
                    AttachmentShape::Content => {
                        if roots.len() != 1 {
                            return Err(EngineError::NativeParentRejectsChildren(native_parent));
                        }
                        Some(Attachment::Content)
                    }
                    AttachmentShape::ContentPane => {
                        let branch = self.branch_below(native_parent, child).unwrap();
                        let NodeKind::StructuralSlot(slot) = self.arena.get(branch).unwrap().kind
                        else {
                            return Err(EngineError::NativeParentRejectsChildren(native_parent));
                        };
                        Some(
                            structural_slot_attachment(AttachmentShape::ContentPane, slot)
                                .ok_or(EngineError::NativeParentRejectsChildren(native_parent))?,
                        )
                    }
                    AttachmentShape::HeaderContent => {
                        let branch = self.branch_below(native_parent, child).unwrap();
                        let NodeKind::StructuralSlot(slot) = self.arena.get(branch).unwrap().kind
                        else {
                            return Err(EngineError::NativeParentRejectsChildren(native_parent));
                        };
                        Some(
                            structural_slot_attachment(AttachmentShape::HeaderContent, slot)
                                .ok_or(EngineError::NativeParentRejectsChildren(native_parent))?,
                        )
                    }
                    AttachmentShape::None => {
                        return Err(EngineError::NativeParentRejectsChildren(native_parent));
                    }
                },
            };
            for root in roots {
                let attachment = fixed_attachment.unwrap_or_else(|| {
                    let index = all_roots
                        .iter()
                        .position(|candidate| *candidate == root)
                        .unwrap();
                    Attachment::Child { index }
                });
                self.pending.push(Command::Attach {
                    parent: native_parent,
                    child: root,
                    attachment,
                });
            }
        }
        Ok(())
    }

    fn branch_below(&self, ancestor: NodeId, mut node: NodeId) -> Option<NodeId> {
        loop {
            let parent = self.arena.get(node)?.parent?;
            if parent == ancestor {
                return Some(node);
            }
            node = parent;
        }
    }

    pub(super) fn valid_window_owner(&self, owner: NodeId, child: NodeId) -> bool {
        if owner == child {
            return false;
        }
        let Some(owner_node) = self.arena.get(owner) else {
            return false;
        };
        if !matches!(owner_node.kind, NodeKind::Window) {
            return false;
        }
        let Some(owned_slot) = owner_node.children.get(1).copied() else {
            return false;
        };
        if self.branch_below(owner, child) != Some(owned_slot) {
            return false;
        }
        let mut current = self.arena.get(child).and_then(|node| node.parent);
        while let Some(id) = current {
            let Some(node) = self.arena.get(id) else {
                return false;
            };
            if matches!(node.kind, NodeKind::Window) {
                return id == owner;
            }
            current = node.parent;
        }
        false
    }

    pub fn remove_subtree(&mut self, id: NodeId) -> Result<(), EngineError> {
        if let Some(parked) = self.parked_virtual_rows.remove(&id) {
            for root in parked.into_values() {
                self.remove_subtree(root)?;
            }
            self.virtual_empty.remove(&id);
        }
        if self.arena.get(id).is_some_and(|node| node.kind.is_native())
            && let Some((owner, accessory, relation)) = self.owner_bound_accessory_for_owner(id)
        {
            self.pending.push(Command::UnbindOwner {
                owner,
                accessory,
                relation,
            });
        }
        if self.arena.get(id).is_some_and(|node| {
            matches!(
                node.mounted.as_ref().map(|mounted| &mounted.kind),
                Some(MountedKind::Window(MountedWindow {
                    title_bar: Some(_),
                    ..
                }))
            )
        }) {
            self.pending.push(Command::UpdateWindow {
                id,
                update: WindowUpdate::UnbindTitleBar,
            });
        }
        while let Some(child) = self
            .arena
            .get(id)
            .ok_or(EngineError::InvalidNode(id))?
            .children
            .last()
            .copied()
        {
            self.remove_subtree(child)?;
        }

        let (parent, native, projected) = {
            let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
            (
                node.parent,
                node.kind.is_native(),
                node.kind.projects_native_root(),
            )
        };
        if let Some(parent) = parent {
            let native_parent = projected
                .then(|| self.nearest_native_attachment_parent(id, parent))
                .flatten();
            let parent_node = self.arena.get_mut(parent).unwrap();
            parent_node.children.retain(|child| *child != id);
            if let NodeKind::VirtualHost { realized } = &mut parent_node.kind {
                realized.retain(|_, row| row.root != id);
                if self.virtual_empty.get(&parent) == Some(&id) {
                    self.virtual_empty.remove(&parent);
                }
            }
            if let Some(native_parent) = native_parent {
                self.pending.push(Command::Detach {
                    parent: native_parent,
                    child: id,
                });
            }
        }
        if native {
            self.pending.push(Command::Destroy { id });
        } else if matches!(
            self.arena.get(id).ok_or(EngineError::InvalidNode(id))?.kind,
            NodeKind::Window
        ) {
            self.pending.push(Command::CloseWindow { id });
        }
        if let Some(mut node) = self.arena.remove(id) {
            self.retire_mounted(id, node.mounted.take());
        }
        Ok(())
    }

    pub(super) fn retire_subtree(&mut self, id: NodeId) {
        while let Some(child) = self.arena.get_mut(id).unwrap().children.pop() {
            self.retire_subtree(child);
        }
        let mut node = self.arena.remove(id).unwrap();
        self.retire_mounted(id, node.mounted.take());
    }

    fn retire_mounted(&mut self, id: NodeId, mounted: Option<Mounted>) {
        if let Some(mut mounted) = mounted {
            if let MountedKind::FadeTransition {
                revision,
                exiting: true,
                ..
            } = &mounted.kind
            {
                self.stop_timer(id, FADE_TRANSITION_TIMER_SLOT, *revision);
            }
            mounted.prepare_remove(id);
            if mounted.has_reference() {
                self.references -= 1;
            }
            if let Some(cleanup) = mounted.take_reference_cleanup() {
                self.retired_reference_cleanups.push(cleanup);
            }
            if mounted.has_effect_hooks() || mounted.has_resource_hooks() {
                self.retired.push(mounted);
            }
        }
    }

    fn nearest_native_ancestor(&self, mut id: NodeId) -> Option<NodeId> {
        loop {
            let node = self.arena.get(id)?;
            if node.kind.is_native() {
                return Some(id);
            }
            id = node.parent?;
        }
    }

    fn nearest_native_attachment_parent(
        &self,
        mut branch: NodeId,
        mut id: NodeId,
    ) -> Option<NodeId> {
        loop {
            let node = self.arena.get(id)?;
            if node.kind.is_native() {
                return Some(id);
            }
            if matches!(
                node.kind,
                NodeKind::OwnerBound {
                    project_accessory: false,
                    ..
                }
            ) && node.children.get(1) == Some(&branch)
            {
                return None;
            }
            branch = id;
            id = node.parent?;
        }
    }

    fn owner_bound_accessory_for_owner(
        &self,
        owner: NodeId,
    ) -> Option<(NodeId, NodeId, OwnerRelation)> {
        let mut branch = owner;
        let mut id = self.arena.get(owner)?.parent?;
        loop {
            let node = self.arena.get(id)?;
            if node.kind.is_native() {
                return None;
            }
            if let NodeKind::OwnerBound { relation, .. } = node.kind
                && node.children.first() == Some(&branch)
            {
                return Some((
                    owner,
                    self.single_projected_native_root(*node.children.get(1)?)?,
                    relation,
                ));
            }
            branch = id;
            id = node.parent?;
        }
    }

    fn projected_native_children(&self, parent: NodeId) -> Vec<NodeId> {
        self.arena
            .get(parent)
            .into_iter()
            .flat_map(|node| node.children.iter().copied())
            .flat_map(|child| self.projected_native_roots(child))
            .collect()
    }

    pub(crate) fn projected_native_roots(&self, id: NodeId) -> Vec<NodeId> {
        let Some(node) = self.arena.get(id) else {
            return Vec::new();
        };
        if matches!(node.kind, NodeKind::Application | NodeKind::Window) {
            Vec::new()
        } else if node.kind.projects_native_root() {
            vec![id]
        } else if matches!(node.kind, NodeKind::OwnedNative) {
            Vec::new()
        } else if let NodeKind::OwnerBound {
            project_accessory, ..
        } = node.kind
        {
            node.children
                .iter()
                .take(if project_accessory { 2 } else { 1 })
                .copied()
                .flat_map(|child| self.projected_native_roots(child))
                .collect()
        } else {
            node.children
                .iter()
                .copied()
                .flat_map(|child| self.projected_native_roots(child))
                .collect()
        }
    }

    pub(super) fn projected_native_root_count(&self, id: NodeId) -> usize {
        let Some(node) = self.arena.get(id) else {
            return 0;
        };
        if matches!(node.kind, NodeKind::Application | NodeKind::Window) {
            0
        } else if node.kind.projects_native_root() {
            1
        } else if matches!(node.kind, NodeKind::OwnedNative) {
            0
        } else if let NodeKind::OwnerBound {
            project_accessory, ..
        } = node.kind
        {
            node.children
                .iter()
                .take(if project_accessory { 2 } else { 1 })
                .map(|child| self.projected_native_root_count(*child))
                .sum()
        } else {
            node.children
                .iter()
                .map(|child| self.projected_native_root_count(*child))
                .sum()
        }
    }

    fn subtree_contains_owner_bound(&self, id: NodeId) -> bool {
        let Some(node) = self.arena.get(id) else {
            return false;
        };
        matches!(node.kind, NodeKind::OwnerBound { .. })
            || node
                .children
                .iter()
                .any(|child| self.subtree_contains_owner_bound(*child))
    }

    pub(crate) fn single_projected_native_root(&self, id: NodeId) -> Option<NodeId> {
        fn visit<R: NativeRuntime>(
            engine: &Engine<R>,
            id: NodeId,
            found: &mut Option<NodeId>,
        ) -> bool {
            let Some(node) = engine.arena.get(id) else {
                return true;
            };
            if matches!(node.kind, NodeKind::Application | NodeKind::Window) {
                return true;
            }
            if node.kind.projects_native_root() {
                if found.is_some() {
                    return false;
                }
                *found = Some(id);
                return true;
            }
            if matches!(node.kind, NodeKind::OwnedNative) {
                return true;
            }
            let children = if let NodeKind::OwnerBound {
                project_accessory, ..
            } = node.kind
            {
                &node.children[..node
                    .children
                    .len()
                    .min(if project_accessory { 2 } else { 1 })]
            } else {
                &node.children
            };
            children
                .iter()
                .copied()
                .all(|child| visit(engine, child, found))
        }

        let mut found = None;
        visit(self, id, &mut found).then_some(found).flatten()
    }
}

fn structural_slot_attachment(shape: AttachmentShape, slot: StructuralSlot) -> Option<Attachment> {
    match (shape, slot) {
        (AttachmentShape::ContentPane, StructuralSlot::Content)
        | (AttachmentShape::HeaderContent, StructuralSlot::Content) => Some(Attachment::Content),
        (AttachmentShape::ContentPane, StructuralSlot::Pane) => Some(Attachment::Pane),
        (AttachmentShape::HeaderContent, StructuralSlot::Header) => Some(Attachment::Header),
        _ => None,
    }
}

pub(super) fn apply_minimal_reorder(
    current: &mut Vec<NodeId>,
    desired: &[NodeId],
    mut move_node: impl FnMut(NodeId, usize),
) {
    if current == desired {
        return;
    }

    let mut positions = current
        .iter()
        .copied()
        .enumerate()
        .map(|(index, id)| (id, index))
        .collect::<Vec<_>>();
    positions.sort_unstable_by_key(|(id, _)| *id);
    let mut sequence = desired
        .iter()
        .map(|id| {
            positions
                .binary_search_by_key(id, |(candidate, _)| *candidate)
                .map(|index| positions[index].1)
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut tails = Vec::<usize>::new();
    let mut previous = vec![usize::MAX; sequence.len()];
    for (index, value) in sequence.iter().copied().enumerate() {
        let position = tails.partition_point(|tail| sequence[*tail] < value);
        if position > 0 {
            previous[index] = tails[position - 1];
        }
        if position == tails.len() {
            tails.push(index);
        } else {
            tails[position] = index;
        }
    }

    if let Some(mut index) = tails.last().copied() {
        loop {
            let predecessor = previous[index];
            sequence[index] = usize::MAX;
            if predecessor == usize::MAX {
                break;
            }
            index = predecessor;
        }
    }

    for desired_index in (0..desired.len()).rev() {
        if sequence[desired_index] == usize::MAX {
            continue;
        }
        let id = desired[desired_index];
        let source = current
            .iter()
            .position(|candidate| *candidate == id)
            .unwrap();
        let target = if desired_index + 1 == desired.len() {
            current.len() - 1
        } else {
            let anchor = current
                .iter()
                .position(|candidate| *candidate == desired[desired_index + 1])
                .unwrap();
            if source < anchor { anchor - 1 } else { anchor }
        };
        let id = current.remove(source);
        current.insert(target, id);
        move_node(id, target);
    }
    debug_assert_eq!(current, desired);
}
