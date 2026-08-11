use super::logical_tree::MountedLogicalTree;
use super::templated::MountedTemplatedTree;
use super::*;
use crate::reference::NativeElementRef;

#[derive(Default)]
pub(super) struct MountedTree {
    children: FxHashMap<ControlId, Vec<ControlId>>,
    // Sparse because almost every native node projects into its parent's visual collection.
    owned_only: rustc_hash::FxHashSet<ControlId>,
    logical_children: FxHashMap<ControlId, Vec<MountedOutput>>,
    nodes: FxHashMap<ControlId, MountedNativeNode>,
    headers: FxHashMap<ControlId, MountedOutput>,
    panes: FxHashMap<ControlId, MountedOutput>,
    before_unmount: FxHashMap<ControlId, BeforeUnmount>,
    pub(super) templated: MountedTemplatedTree,
    pub(super) logical: MountedLogicalTree,
}

struct MountedNativeNode {
    kind: Option<ControlKind>,
    parent: Option<ControlId>,
}

pub(super) struct BeforeUnmount {
    pub(super) reference: Option<NativeElementRef>,
    pub(super) callback: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl MountedTree {
    #[cfg(any(debug_assertions, feature = "test"))]
    pub(super) fn assert_consistent(&self) {
        let mut owned = rustc_hash::FxHashSet::default();
        let mut record = |parent: ControlId, child: ControlId| {
            assert!(
                owned.insert(child),
                "native control {child:?} has more than one owner"
            );
            assert_eq!(
                self.parent(child),
                Some(parent),
                "native control {child:?} disagrees with its owner"
            );
        };

        for (parent, outputs) in &self.logical_children {
            let native: Vec<_> = outputs.iter().filter_map(|output| output.native).collect();
            assert_eq!(
                self.children(*parent),
                native.as_slice(),
                "logical child mirror disagrees with native children"
            );
            for output in outputs {
                if let Some(node_id) = output.logical {
                    assert!(
                        self.logical.contains_node(node_id),
                        "logical child output has no mounted node"
                    );
                    assert_eq!(
                        self.logical.node_native_root(node_id),
                        output.native,
                        "logical child output native root disagrees with node"
                    );
                }
            }
        }

        for (parent, children) in &self.children {
            for child in children {
                record(*parent, *child);
            }
        }
        for (parent, header) in &self.headers {
            if let Some(header) = header.native {
                record(*parent, header);
            }
        }
        for (parent, pane) in &self.panes {
            if let Some(pane) = pane.native {
                record(*parent, pane);
            }
        }
        for (parent, state) in &self.templated.lists {
            for row in state.rows.values() {
                if let Some(content_id) = row.output.native {
                    record(*parent, content_id);
                }
            }
        }

        for (id, node) in &self.nodes {
            assert_eq!(
                self.owned_only.contains(id),
                matches!(node.kind, Some(ControlKind::ContentDialog)),
                "native control {id:?} has the wrong child projection"
            );
            if node.parent.is_some() {
                assert!(
                    owned.contains(id),
                    "native control {id:?} has a parent but is absent from its owner's children"
                );
            }
        }
        for id in &self.owned_only {
            assert!(
                self.nodes.contains_key(id),
                "owned-only projection {id:?} has no mounted native node"
            );
        }
        for id in self.before_unmount.keys() {
            assert!(
                self.nodes.contains_key(id),
                "pre-unmount callback {id:?} has no mounted native node"
            );
        }
    }

    pub(super) fn contains_native(&self, id: ControlId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub(super) fn native_roots(&self) -> Vec<ControlId> {
        let mut roots: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(id, node)| node.parent.is_none().then_some(*id))
            .collect();
        roots.sort_unstable_by_key(|id| id.0);
        roots
    }

    pub(super) fn register(&mut self, id: ControlId, kind: Option<ControlKind>) {
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.clear_parent(child, id);
            }
        }
        self.logical_children.remove(&id);
        if let Some(header) = self.headers.remove(&id)
            && let Some(header) = header.native
        {
            self.clear_parent(header, id);
        }
        if let Some(pane) = self.panes.remove(&id)
            && let Some(pane) = pane.native
        {
            self.clear_parent(pane, id);
        }
        self.before_unmount.remove(&id);
        if matches!(kind, Some(ControlKind::ContentDialog)) {
            self.owned_only.insert(id);
        } else {
            self.owned_only.remove(&id);
        }
        self.nodes
            .insert(id, MountedNativeNode { kind, parent: None });
    }

    pub(super) fn kind(&self, id: ControlId) -> Option<ControlKind> {
        self.nodes.get(&id).and_then(|node| node.kind)
    }

    pub(super) fn projects_as_child(&self, id: ControlId) -> bool {
        !self.owned_only.contains(&id)
    }

    pub(super) fn parent(&self, id: ControlId) -> Option<ControlId> {
        self.nodes.get(&id).and_then(|node| node.parent)
    }

    pub(super) fn set_parent(&mut self, child: ControlId, parent: ControlId) {
        let node = self
            .nodes
            .get_mut(&child)
            .expect("mounted child missing native node");
        debug_assert!(
            node.parent.is_none() || node.parent == Some(parent),
            "native control {child:?} already owned by {:?}",
            node.parent
        );
        node.parent = Some(parent);
    }

    pub(super) fn clear_parent(&mut self, child: ControlId, parent: ControlId) {
        if let Some(node) = self.nodes.get_mut(&child)
            && node.parent == Some(parent)
        {
            node.parent = None;
        } else if !self.nodes.contains_key(&child) {
            self.owned_only.remove(&child);
        }
    }

    pub(super) fn set_header(&mut self, parent: ControlId, header: Option<MountedOutput>) {
        if let Some(old) = self.headers.remove(&parent)
            && let Some(old) = old.native
        {
            self.clear_parent(old, parent);
        }
        if let Some(header) = header
            && let Some(native) = header.native
        {
            self.set_parent(native, parent);
            self.headers.insert(parent, header);
        } else if let Some(header) = header {
            self.headers.insert(parent, header);
        }
    }

    pub(super) fn header(&self, parent: ControlId) -> Option<MountedOutput> {
        self.headers.get(&parent).copied()
    }

    pub(super) fn set_pane(&mut self, parent: ControlId, pane: Option<MountedOutput>) {
        if let Some(old) = self.panes.remove(&parent)
            && let Some(old) = old.native
        {
            self.clear_parent(old, parent);
        }
        if let Some(pane) = pane
            && let Some(native) = pane.native
        {
            self.set_parent(native, parent);
            self.panes.insert(parent, pane);
        } else if let Some(pane) = pane {
            self.panes.insert(parent, pane);
        }
    }

    pub(super) fn pane(&self, parent: ControlId) -> Option<MountedOutput> {
        self.panes.get(&parent).copied()
    }

    pub(super) fn set_before_unmount(
        &mut self,
        id: ControlId,
        reference: Option<NativeElementRef>,
        callback: Option<Callback<Option<windows_core::IInspectable>>>,
    ) {
        debug_assert!(self.nodes.contains_key(&id));
        if reference.is_some() || callback.is_some() {
            self.before_unmount.insert(
                id,
                BeforeUnmount {
                    reference,
                    callback,
                },
            );
        } else {
            self.before_unmount.remove(&id);
        }
    }

    pub(super) fn take_before_unmount(&mut self, id: ControlId) -> Option<BeforeUnmount> {
        self.before_unmount.remove(&id)
    }

    pub(super) fn children(&self, parent: ControlId) -> &[ControlId] {
        self.children.get(&parent).map_or(&[], Vec::as_slice)
    }

    pub(super) fn logical_children(&self, parent: ControlId) -> &[MountedOutput] {
        self.logical_children
            .get(&parent)
            .map_or(&[], Vec::as_slice)
    }

    pub(super) fn logical_child(&self, parent: ControlId, index: usize) -> Option<MountedOutput> {
        self.logical_children(parent).get(index).copied()
    }

    pub(super) fn logical_owner(&self, node_id: LogicalNodeId) -> Option<ControlId> {
        self.logical_children
            .iter()
            .find_map(|(parent, children)| {
                children
                    .iter()
                    .any(|output| output.logical == Some(node_id))
                    .then_some(*parent)
            })
            .or_else(|| {
                self.headers.iter().find_map(|(parent, output)| {
                    (output.logical == Some(node_id)).then_some(*parent)
                })
            })
            .or_else(|| {
                self.panes.iter().find_map(|(parent, output)| {
                    (output.logical == Some(node_id)).then_some(*parent)
                })
            })
            .or_else(|| {
                self.templated.lists.iter().find_map(|(parent, state)| {
                    state
                        .rows
                        .values()
                        .any(|row| row.output.logical == Some(node_id))
                        .then_some(*parent)
                })
            })
    }

    pub(super) fn native_index(&self, parent: ControlId, logical_index: usize) -> usize {
        self.logical_children(parent)[..logical_index]
            .iter()
            .filter(|output| output.native.is_some())
            .count()
    }

    pub(super) fn projected_index(&self, parent: ControlId, native_index: usize) -> usize {
        if self.owned_only.is_empty() {
            return native_index;
        }
        self.children(parent)[..native_index]
            .iter()
            .filter(|child| self.projects_as_child(**child))
            .count()
    }

    pub(super) fn child_is_projected(&self, parent: ControlId, index: usize) -> bool {
        self.children(parent)
            .get(index)
            .is_some_and(|child| self.projects_as_child(*child))
    }

    pub(super) fn append_logical_child(&mut self, parent: ControlId, output: MountedOutput) {
        self.logical_children
            .entry(parent)
            .or_default()
            .push(output);
    }

    pub(super) fn insert_logical_child(
        &mut self,
        parent: ControlId,
        index: usize,
        output: MountedOutput,
    ) -> usize {
        let list = self.logical_children.entry(parent).or_default();
        let index = index.min(list.len());
        list.insert(index, output);
        index
    }

    pub(super) fn remove_logical_child(
        &mut self,
        parent: ControlId,
        index: usize,
    ) -> Option<MountedOutput> {
        self.logical_children
            .get_mut(&parent)
            .and_then(|list| (index < list.len()).then(|| list.remove(index)))
    }

    pub(super) fn replace_logical_child(
        &mut self,
        parent: ControlId,
        index: usize,
        output: MountedOutput,
    ) -> Option<MountedOutput> {
        self.logical_children.get_mut(&parent).and_then(|list| {
            (index < list.len()).then(|| std::mem::replace(&mut list[index], output))
        })
    }

    pub(super) fn move_logical_child(&mut self, parent: ControlId, from: usize, to: usize) {
        if from == to {
            return;
        }
        if let Some(list) = self.logical_children.get_mut(&parent)
            && from < list.len()
            && to < list.len()
        {
            let item = list.remove(from);
            list.insert(to, item);
        }
    }

    pub(super) fn permute_logical_children(
        &mut self,
        parent: ControlId,
        start: usize,
        new_to_old: &[i32],
        visited: &mut [bool],
    ) {
        let children = self.logical_children.get_mut(&parent).unwrap();
        debug_assert_eq!(visited.len(), new_to_old.len());
        debug_assert!(
            start
                .checked_add(new_to_old.len())
                .is_some_and(|end| end <= children.len()),
            "logical child permutation exceeds the mounted child range"
        );
        #[cfg(debug_assertions)]
        {
            let mut seen = vec![false; new_to_old.len()];
            for old in new_to_old {
                debug_assert!(*old >= 0, "logical child permutation contains a sentinel");
                let old = *old as usize;
                debug_assert!(
                    old < new_to_old.len(),
                    "logical child permutation index is out of range"
                );
                debug_assert!(
                    !seen[old],
                    "logical child permutation contains a duplicate index"
                );
                seen[old] = true;
            }
        }
        visited.fill(false);
        for cycle_start in 0..new_to_old.len() {
            if visited[cycle_start] {
                continue;
            }
            let saved = children[start + cycle_start];
            let mut current = cycle_start;
            loop {
                visited[current] = true;
                let next = new_to_old[current] as usize;
                if next == cycle_start {
                    children[start + current] = saved;
                    break;
                }
                children[start + current] = children[start + next];
                current = next;
            }
        }
    }

    pub(super) fn extend_owned_children(&self, parent: ControlId, children: &mut Vec<ControlId>) {
        children.extend_from_slice(self.children(parent));
        if let Some(header) = self.header(parent).and_then(|output| output.native) {
            children.push(header);
        }
        if let Some(pane) = self.pane(parent).and_then(|output| output.native) {
            children.push(pane);
        }
        if let Some(state) = self.templated.lists.get(&parent) {
            children.extend(state.rows.values().filter_map(|row| row.output.native));
        }
    }

    pub(super) fn extend_owned_logical_roots(
        &self,
        parent: ControlId,
        logical_roots: &mut Vec<LogicalNodeId>,
    ) {
        logical_roots.extend(
            self.logical_children(parent)
                .iter()
                .filter_map(|output| output.native.is_none().then_some(output.logical))
                .flatten(),
        );
        if let Some(output) = self.header(parent)
            && output.native.is_none()
            && let Some(logical) = output.logical
        {
            logical_roots.push(logical);
        }
        if let Some(output) = self.pane(parent)
            && output.native.is_none()
            && let Some(logical) = output.logical
        {
            logical_roots.push(logical);
        }
        if let Some(state) = self.templated.lists.get(&parent) {
            logical_roots.extend(
                state
                    .rows
                    .values()
                    .filter_map(|row| row.output.native.is_none().then_some(row.output.logical))
                    .flatten(),
            );
        }
    }

    pub(super) fn child(&self, parent: ControlId, index: usize) -> Option<ControlId> {
        self.children(parent).get(index).copied()
    }

    pub(super) fn child_position(&self, parent: ControlId, child: ControlId) -> Option<usize> {
        self.children(parent).iter().position(|id| *id == child)
    }

    pub(super) fn append_child(&mut self, parent: ControlId, child: ControlId) {
        self.set_parent(child, parent);
        self.children.entry(parent).or_default().push(child);
    }

    pub(super) fn remove_child(&mut self, parent: ControlId, index: usize) -> Option<ControlId> {
        let removed = self
            .children
            .get_mut(&parent)
            .and_then(|list| (index < list.len()).then(|| list.remove(index)));
        if let Some(child) = removed {
            self.clear_parent(child, parent);
        }
        removed
    }

    pub(super) fn replace_child(
        &mut self,
        parent: ControlId,
        index: usize,
        new: ControlId,
    ) -> Option<ControlId> {
        let replaced = self.children.get_mut(&parent).and_then(|list| {
            (index < list.len()).then(|| {
                let old = list[index];
                list[index] = new;
                old
            })
        });
        if let Some(old) = replaced {
            self.clear_parent(old, parent);
            self.set_parent(new, parent);
        }
        replaced
    }

    pub(super) fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        if from == to {
            return;
        }
        if let Some(list) = self.children.get_mut(&parent)
            && from < list.len()
            && to < list.len()
        {
            let item = list.remove(from);
            list.insert(to, item);
        }
    }

    pub(super) fn insert_child(
        &mut self,
        parent: ControlId,
        index: usize,
        child: ControlId,
    ) -> usize {
        self.set_parent(child, parent);
        let list = self.children.entry(parent).or_default();
        let index = index.min(list.len());
        list.insert(index, child);
        index
    }

    pub(super) fn remove_node(&mut self, id: ControlId) {
        let owned = self
            .nodes
            .get(&id)
            .is_some_and(|node| node.parent.is_some());
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.clear_parent(child, id);
            }
        }
        self.logical_children.remove(&id);
        if let Some(header) = self.headers.remove(&id)
            && let Some(header) = header.native
        {
            self.clear_parent(header, id);
        }
        if let Some(pane) = self.panes.remove(&id)
            && let Some(pane) = pane.native
        {
            self.clear_parent(pane, id);
        }
        self.before_unmount.remove(&id);
        self.nodes.remove(&id);
        if !owned {
            self.owned_only.remove(&id);
        }
    }
}
