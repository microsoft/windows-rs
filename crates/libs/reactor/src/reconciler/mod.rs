//! Reconciliation orchestration over the logical and mounted ownership trees.

use std::cell::Cell;
use std::rc::Rc;

use rustc_hash::FxHashMap;

pub use super::*;

mod child;
mod diff_helpers;
mod logical_tree;
mod mounted_tree;
mod templated;
mod widget_dispatch;
mod wrappers;

pub use self::child::compute_lis;
use self::logical_tree::{LogicalNodeId, LogicalNodeKind, LogicalParentGuard, LogicalWrapperNode};
use self::mounted_tree::MountedTree;

fn output_is_empty(output: MountedOutput) -> bool {
    output.native.is_none() && output.logical.is_none()
}

#[derive(Default)]
struct ReconcilePass {
    forced_nodes: rustc_hash::FxHashSet<LogicalNodeId>,
    forced_controls: rustc_hash::FxHashSet<ControlId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LogicalSlotId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MountedOutput {
    slot: LogicalSlotId,
    native: Option<ControlId>,
    logical: Option<LogicalNodeId>,
}

impl MountedOutput {
    const fn empty(slot: LogicalSlotId) -> Self {
        Self {
            slot,
            native: None,
            logical: None,
        }
    }
}

struct HostContext {
    context_stack: Rc<ContextStack>,
    marshaller: Option<UiMarshaller>,
    host_id: HostId,
    inner_size: Rc<Cell<WindowSize>>,
    dpi: Rc<Cell<u32>>,
    request_rerender: Rc<dyn Fn()>,
}

impl HostContext {
    fn new() -> Self {
        Self {
            context_stack: Rc::new(ContextStack::new()),
            marshaller: None,
            host_id: HostId::next(),
            inner_size: Rc::new(Cell::new(WindowSize::default())),
            dpi: Rc::new(Cell::new(96_u32)),
            request_rerender: Rc::new(|| {}),
        }
    }
}

/// Diff/apply engine that drives a [`Backend`] from successive [`Element`] trees.
pub struct Reconciler<B: Backend> {
    pub backend: B,
    tree: MountedTree,
    pass: ReconcilePass,
    host: HostContext,
    stats: ReconcileStats,
    root_output: Option<MountedOutput>,
    next_slot_id: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReconcileStats {
    pub elements_skipped: u64,
    pub elements_diffed: u64,
    pub ui_elements_created: u64,
}

pub struct ComponentInstance {
    node_id: LogicalNodeId,
    parent: Option<LogicalNodeId>,
    native_root: Option<ControlId>,
    rendered_output: MountedOutput,
    pub render_cx: RenderCx,
    pub last_rendered: Element,
    pub last_obj: Rc<dyn ComponentObject>,
    pub read_contexts: rustc_hash::FxHashSet<ContextId>,
}

impl<B: Backend + 'static> Reconciler<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            tree: MountedTree::default(),
            pass: ReconcilePass::default(),
            host: HostContext::new(),
            stats: ReconcileStats::default(),
            root_output: None,
            next_slot_id: 0,
        }
    }

    pub fn set_marshaller(&mut self, marshaller: Option<UiMarshaller>) {
        self.host.marshaller = marshaller;
    }

    pub fn set_host_id(&mut self, host_id: HostId) {
        self.host.host_id = host_id;
    }

    #[cfg(feature = "test")]
    pub fn flush_deferred_unmounts(&mut self) {
        let outputs = std::mem::take(&mut self.tree.templated.deferred_unmounts);
        for output in outputs {
            self.unmount_output(output);
        }
    }

    #[cfg(feature = "test")]
    pub fn defer_templated_unmounts_for_test(&mut self, defer: bool) {
        self.tree.templated.defer_unmounts = defer;
    }

    pub fn context_stack_handle(&self) -> Rc<ContextStack> {
        Rc::clone(&self.host.context_stack)
    }

    fn is_node_state_dirty(&self, node_id: LogicalNodeId) -> bool {
        self.tree
            .logical
            .instance(node_id)
            .is_some_and(|inst| inst.render_cx.peek_state_dirty())
    }

    fn is_control_forced(&self, id: ControlId) -> bool {
        self.pass.forced_controls.contains(&id)
    }

    fn is_output_forced(&self, output: MountedOutput) -> bool {
        output.native.is_some_and(|id| self.is_control_forced(id))
            || output
                .logical
                .is_some_and(|id| self.pass.forced_nodes.contains(&id))
    }

    pub fn reset_stats(&mut self) {
        self.stats = ReconcileStats::default();
    }

    pub fn stats(&self) -> ReconcileStats {
        self.stats
    }

    #[cfg(feature = "test")]
    pub fn debug_forced_components_len(&self) -> usize {
        self.pass.forced_nodes.len()
    }

    #[cfg(feature = "test")]
    pub fn debug_logical_component_count(&self) -> usize {
        self.tree.logical.component_count()
    }

    #[cfg(feature = "test")]
    pub fn debug_logical_node_count(&self) -> usize {
        self.tree.logical.node_count()
    }

    fn allocate_logical_node_id(&mut self) -> LogicalNodeId {
        self.tree.logical.allocate_id()
    }

    fn allocate_slot_id(&mut self) -> LogicalSlotId {
        let id = LogicalSlotId(self.next_slot_id);
        self.next_slot_id = self
            .next_slot_id
            .checked_add(1)
            .expect("logical slot id overflow");
        id
    }

    fn enter_logical_parent(&self, node_id: LogicalNodeId) -> LogicalParentGuard {
        self.tree.logical.enter_parent(node_id)
    }

    fn add_forced_node_path(&mut self, node_id: LogicalNodeId) {
        let mut current = Some(node_id);
        while let Some(id) = current {
            self.pass.forced_nodes.insert(id);
            if let Some(native_root) = self.tree.logical.node_native_root(id) {
                let mut control = Some(native_root);
                while let Some(id) = control {
                    if !self.pass.forced_controls.insert(id) {
                        break;
                    }
                    control = self.tree.parent(id);
                }
            } else if let Some(owner) = self.tree.logical_owner(id) {
                let mut control = Some(owner);
                while let Some(id) = control {
                    if !self.pass.forced_controls.insert(id) {
                        break;
                    }
                    control = self.tree.parent(id);
                }
            }
            current = self.tree.logical.node_parent(id);
        }
    }

    fn add_forced_node_paths(&mut self, node_ids: impl IntoIterator<Item = LogicalNodeId>) {
        for node_id in node_ids {
            self.add_forced_node_path(node_id);
        }
    }

    #[cfg(feature = "test")]
    pub fn force_components_at_control_for_test(&mut self, id: ControlId) {
        let node_ids = self
            .tree
            .logical
            .projected_nodes(id)
            .map(|nodes| nodes.as_slice().to_vec())
            .unwrap_or_default();
        self.add_forced_node_paths(node_ids);
    }

    pub fn acquire_control(&mut self, kind: ControlKind) -> ControlId {
        self.stats.ui_elements_created += 1;
        let id = self.backend.create(kind);

        if let Some(stale) = self.tree.logical.take_projection(id) {
            stale.drain(|node_id| {
                if self.tree.logical.remove_component(node_id).is_none() {
                    self.tree.logical.remove_wrapper(node_id);
                }
            });
        }
        self.tree.templated.lists.remove(&id);
        self.tree.register(id, Some(kind));
        id
    }

    fn take_component_instance(&mut self, node_id: LogicalNodeId) -> Option<ComponentInstance> {
        self.tree.logical.take_component(node_id)
    }

    #[cfg(feature = "test")]
    pub fn debug_appeared_listener_count(&self) -> usize {
        self.tree.logical.appeared_listener_count()
    }

    #[cfg(feature = "test")]
    pub fn debug_disappeared_listener_count(&self) -> usize {
        self.tree.logical.disappeared_listener_count()
    }

    pub fn reconcile(
        &mut self,
        old: Option<&Element>,
        new: &Element,
        existing: Option<ControlId>,
        request_rerender: Rc<dyn Fn()>,
    ) -> Option<ControlId> {
        self.host.request_rerender = request_rerender;
        let output = match (old, self.root_output, existing) {
            (Some(old_el), Some(output), _) => {
                let seeded = self.force_state_dirty_components();
                let result = self.update_output(old_el, new, output);
                debug_assert!(
                    seeded
                        .iter()
                        .all(|node_id| !self.is_node_state_dirty(*node_id)),
                    "a state-dirty component was not re-rendered by the pass"
                );
                result
            }
            (Some(old_el), None, Some(id)) => {
                let seeded = self.force_state_dirty_components();
                let logical = self
                    .tree
                    .logical
                    .current_node(id, LogicalNodeKind::Component);
                let slot = self.allocate_slot_id();
                let result = self.update_output(
                    old_el,
                    new,
                    MountedOutput {
                        slot,
                        native: Some(id),
                        logical,
                    },
                );
                debug_assert!(
                    seeded
                        .iter()
                        .all(|node_id| !self.is_node_state_dirty(*node_id)),
                    "a state-dirty component was not re-rendered by the pass"
                );
                result
            }
            _ => self.mount_output(new),
        };
        self.root_output = (!output_is_empty(output)).then_some(output);
        #[cfg(debug_assertions)]
        self.assert_consistent_inner();
        output.native
    }

    /// Verifies the mounted logical and native ownership graphs.
    #[doc(hidden)]
    #[cfg(feature = "test")]
    pub fn assert_consistent(&self) {
        self.assert_consistent_inner();
    }

    #[cfg(any(debug_assertions, feature = "test"))]
    fn assert_consistent_inner(&self) {
        self.tree.logical.assert_consistent();
        self.tree.assert_consistent();

        if let Some(output) = self.root_output {
            assert!(
                output.native.is_some() || output.logical.is_some(),
                "mounted root output is empty"
            );
            if let Some(native) = output.native {
                assert!(
                    self.tree.contains_native(native),
                    "mounted root references missing native control {native:?}"
                );
            }
            if let Some(logical) = output.logical {
                assert!(
                    self.tree.logical.contains_node(logical),
                    "mounted root references missing logical node {logical:?}"
                );
                assert_eq!(
                    self.tree.logical.node_native_root(logical),
                    output.native,
                    "mounted root projection disagrees with its logical node"
                );
            }
        }
    }

    /// Forces dirty components to render even when unchanged parents can be skipped.
    fn force_state_dirty_components(&mut self) -> Vec<LogicalNodeId> {
        let dirty: Vec<LogicalNodeId> = self
            .tree
            .logical
            .components()
            .filter_map(|inst| inst.render_cx.peek_state_dirty().then_some(inst.node_id))
            .collect();
        if !dirty.is_empty() {
            self.add_forced_node_paths(dirty.iter().copied());
        }
        dirty
    }

    fn mount_output(&mut self, el: &Element) -> MountedOutput {
        let slot = self.allocate_slot_id();
        match el {
            Element::Component(ce) => {
                return self.mount_component_output(ce, slot);
            }
            Element::ErrorBoundary(eb) => {
                return self.mount_error_boundary_output_node(eb, slot);
            }
            Element::Provider(pe) => return self.mount_provider_output(pe, slot),
            Element::TemplatedList(tl) => {
                return MountedOutput {
                    slot,
                    native: Some(self.mount_templated_list(tl)),
                    logical: None,
                };
            }
            Element::Custom(c) => {
                return MountedOutput {
                    slot,
                    native: Some(self.mount_custom(c)),
                    logical: None,
                };
            }
            Element::Empty => return MountedOutput::empty(slot),
            _ => {}
        }
        let widget = el.as_widget().unwrap();
        let id = self.mount_widget(widget);
        if let Element::RichTextBlock(rt) = el
            && !rt.paragraphs.is_empty()
        {
            self.backend.set_rich_text_paragraphs(id, &rt.paragraphs);
        }
        MountedOutput {
            slot,
            native: Some(id),
            logical: None,
        }
    }

    pub fn mount(&mut self, el: &Element) -> Option<ControlId> {
        self.mount_output(el).native
    }

    fn update_output(
        &mut self,
        old: &Element,
        new: &Element,
        old_output: MountedOutput,
    ) -> MountedOutput {
        let forced = old_output
            .native
            .is_some_and(|id| self.is_control_forced(id))
            || old_output
                .logical
                .is_some_and(|id| self.pass.forced_nodes.contains(&id));
        if can_skip_update(old, new) && !forced {
            self.stats.elements_skipped += 1;
            return old_output;
        }
        self.stats.elements_diffed += 1;

        if !old.kind_matches(new) {
            self.unmount_output(old_output);
            return self.mount_output(new);
        }

        match (old, new) {
            (Element::Component(o), Element::Component(n)) => {
                return self.update_component_output(o, n, old_output);
            }
            (Element::ErrorBoundary(o), Element::ErrorBoundary(n)) => {
                return self.update_error_boundary_output(o, n, old_output);
            }
            (Element::Provider(o), Element::Provider(n)) => {
                return self.update_provider_output(o, n, old_output);
            }
            (Element::TemplatedList(o), Element::TemplatedList(n)) => {
                let id = old_output.native.unwrap();
                self.update_templated_list(o, n, id);
                return old_output;
            }
            (Element::Custom(o), Element::Custom(n)) => {
                let id = old_output.native.unwrap();
                return MountedOutput {
                    native: Some(self.update_custom(o, n, id)),
                    ..old_output
                };
            }
            (Element::Empty, Element::Empty) => return old_output,
            _ => {}
        }

        let id = old_output
            .native
            .expect("widget update requires a native output");
        let (Some(ow), Some(nw)) = (old.as_widget(), new.as_widget()) else {
            unreachable!("kind_matches guarantees same variant; non-widget variants handled above");
        };
        self.update_widget(ow, nw, id);
        if let (Element::RichTextBlock(o), Element::RichTextBlock(n)) = (old, new)
            && o.paragraphs != n.paragraphs
        {
            self.backend.set_rich_text_paragraphs(id, &n.paragraphs);
        }
        old_output
    }

    pub fn update(&mut self, old: &Element, new: &Element, id: ControlId) -> Option<ControlId> {
        let output = MountedOutput {
            slot: self.allocate_slot_id(),
            native: Some(id),
            logical: self.tree.logical.current_projection(id),
        };
        self.update_output(old, new, output).native
    }

    fn remove_logical_subtree(&mut self, root: LogicalNodeId) {
        if !self.tree.logical.contains_node(root) {
            return;
        }
        self.remove_logical_subtrees([root]);
    }

    fn remove_logical_subtrees(&mut self, roots: impl IntoIterator<Item = LogicalNodeId>) {
        let mut seen = rustc_hash::FxHashSet::default();
        let mut nodes: Vec<_> = roots
            .into_iter()
            .filter(|root| self.tree.logical.contains_node(*root))
            .filter(|root| seen.insert(*root))
            .collect();
        if nodes.is_empty() {
            return;
        }

        let mut children: FxHashMap<LogicalNodeId, Vec<LogicalNodeId>> = FxHashMap::default();
        for node in self.tree.logical.components() {
            if let Some(parent) = node.parent {
                children.entry(parent).or_default().push(node.node_id);
            }
        }
        for node in self.tree.logical.wrappers() {
            if let Some(parent) = node.parent {
                children.entry(parent).or_default().push(node.node_id);
            }
        }

        let mut index = 0;
        while index < nodes.len() {
            let parent = nodes[index];
            index += 1;
            if let Some(logical_children) = children.get(&parent) {
                nodes.extend(
                    logical_children
                        .iter()
                        .copied()
                        .filter(|child| seen.insert(*child)),
                );
            }
        }

        for node_id in &nodes {
            if let Some(inst) = self.tree.logical.instance(*node_id)
                && let Some(native) = inst.native_root
            {
                self.tree.logical.remove_projection(native, *node_id);
            }
            if let Some(node) = self.tree.logical.wrapper(*node_id)
                && let Some(native) = node.native_root
            {
                self.tree.logical.remove_projection(native, *node_id);
            }
        }

        for node_id in nodes.into_iter().rev() {
            if let Some(mut inst) = self.tree.logical.remove_component(node_id) {
                inst.render_cx.run_cleanups();
            } else {
                self.tree.logical.remove_wrapper(node_id);
            }
        }
    }

    fn unmount_output(&mut self, output: MountedOutput) {
        if let Some(native) = output.native {
            self.unmount_inner(native);
        }
        if let Some(logical) = output.logical {
            self.remove_logical_subtree(logical);
        }
    }

    pub fn unmount_root(&mut self) {
        if let Some(output) = self.root_output.take() {
            self.unmount_output(output);
        }
        #[cfg(debug_assertions)]
        self.assert_consistent_inner();
    }

    pub fn unmount(&mut self, id: ControlId) {
        if self
            .root_output
            .is_some_and(|output| output.native == Some(id))
        {
            self.root_output = None;
        }

        self.unmount_inner(id);
        #[cfg(debug_assertions)]
        self.assert_consistent_inner();
    }

    fn unmount_inner(&mut self, id: ControlId) {
        let mut nodes = vec![id];
        let mut next = 0;
        while next < nodes.len() {
            let node = nodes[next];
            next += 1;
            self.tree.extend_owned_children(node, &mut nodes);
        }

        let mut logical_roots = Vec::new();
        for node in &nodes {
            self.tree
                .extend_owned_logical_roots(*node, &mut logical_roots);
        }
        logical_roots.sort_unstable_by_key(|node| node.0);
        logical_roots.dedup();
        self.remove_logical_subtrees(logical_roots);

        for node in nodes.into_iter().rev() {
            if let Some(node_ids) = self.tree.logical.take_projection(node) {
                node_ids.drain(|node_id| {
                    if let Some(mut inst) = self.tree.logical.remove_component(node_id) {
                        inst.render_cx.run_cleanups();
                    } else {
                        self.tree.logical.remove_wrapper(node_id);
                    }
                });
            }

            self.tree.templated.lists.remove(&node);

            // Give external resources a chance to detach before native destroy.
            if let Some(lifecycle) = self.tree.take_before_unmount(node) {
                if let Some(reference) = lifecycle.reference {
                    reference.set_native(None);
                }
                if let Some(callback) = lifecycle.callback {
                    callback.invoke(self.backend.get_native_element(node));
                }
            }

            if let Some(handle) = self.tree.take_custom(node) {
                handle.before_destroy(node, &mut self.backend);
            }

            self.tree.remove_node(node);
            self.backend.destroy(node);
        }
    }

    fn append_output_tracked(&mut self, parent: ControlId, output: MountedOutput) {
        self.tree.append_logical_child(parent, output);
        if let Some(native) = output.native {
            self.tree.append_child(parent, native);
            self.backend.append_child(parent, native);
        }
    }

    fn insert_output_tracked(&mut self, parent: ControlId, index: usize, output: MountedOutput) {
        let index = self.tree.insert_logical_child(parent, index, output);
        if let Some(native) = output.native {
            let native_index = self.tree.native_index(parent, index);
            self.tree.insert_child(parent, native_index, native);
            self.backend.insert_child(parent, native_index, native);
        }
    }

    fn replace_output_tracked(
        &mut self,
        parent: ControlId,
        index: usize,
        output: MountedOutput,
    ) -> MountedOutput {
        let old = self
            .tree
            .logical_child(parent, index)
            .expect("logical child slot missing");
        let native_index = self.tree.native_index(parent, index);
        self.tree.replace_logical_child(parent, index, output);
        match (old.native, output.native) {
            (Some(old), Some(new)) if old != new => {
                self.tree.replace_child(parent, native_index, new);
                self.backend.replace_child(parent, native_index, new);
            }
            (Some(_), Some(_)) => {}
            (Some(_), None) => {
                self.tree.remove_child(parent, native_index);
                self.backend.remove_child(parent, native_index);
            }
            (None, Some(new)) => {
                self.tree.insert_child(parent, native_index, new);
                self.backend.insert_child(parent, native_index, new);
            }
            (None, None) => {}
        }
        old
    }

    fn remove_output_tracked(&mut self, parent: ControlId, index: usize) -> MountedOutput {
        let output = self
            .tree
            .logical_child(parent, index)
            .expect("logical child slot missing");
        let native_index = self.tree.native_index(parent, index);
        self.tree.remove_logical_child(parent, index);
        if output.native.is_some() {
            self.tree.remove_child(parent, native_index);
            self.backend.remove_child(parent, native_index);
        }
        output
    }

    fn move_output_tracked(&mut self, parent: ControlId, from: usize, to: usize) {
        if self
            .tree
            .logical_children(parent)
            .iter()
            .all(|output| output.native.is_some())
        {
            self.tree.move_logical_child(parent, from, to);
            if from != to {
                self.tree.move_child(parent, from, to);
                self.backend.move_child(parent, from, to);
            }
            return;
        }
        let output = self
            .tree
            .logical_child(parent, from)
            .expect("logical child slot missing");
        let from_native = output.native.map(|_| self.tree.native_index(parent, from));
        self.tree.move_logical_child(parent, from, to);
        if let Some(native) = from_native {
            let to_native = self.tree.native_index(parent, to);
            if native != to_native {
                self.tree.move_child(parent, native, to_native);
                self.backend.move_child(parent, native, to_native);
            }
        }
    }

    pub fn append_child_tracked(&mut self, parent: ControlId, child: ControlId) {
        self.tree.append_child(parent, child);
        self.backend.append_child(parent, child);
    }

    pub fn remove_child_tracked(&mut self, parent: ControlId, index: usize) {
        self.tree.remove_child(parent, index);
        self.backend.remove_child(parent, index);
    }

    pub fn replace_child_tracked(&mut self, parent: ControlId, index: usize, new: ControlId) {
        self.tree.replace_child(parent, index, new);
        self.backend.replace_child(parent, index, new);
    }

    pub fn move_child_tracked(&mut self, parent: ControlId, from: usize, to: usize) {
        self.tree.move_child(parent, from, to);
        self.backend.move_child(parent, from, to);
    }

    pub fn insert_child_tracked(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let index = self.tree.insert_child(parent, index, child);
        self.backend.insert_child(parent, index, child);
    }

    pub fn child_at(&self, parent: ControlId, i: usize) -> Option<ControlId> {
        self.tree.child(parent, i)
    }

    pub fn apply_modifiers(&mut self, id: ControlId, mods: &Modifiers) {
        if mods.is_empty() {
            return;
        }
        if let Some(v) = mods.margin {
            self.backend
                .set_prop(id, Prop::Margin, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.padding {
            self.backend
                .set_prop(id, Prop::Padding, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.width {
            self.backend.set_prop(id, Prop::Width, &PropValue::F64(v));
        }
        if let Some(v) = mods.height {
            self.backend.set_prop(id, Prop::Height, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_width {
            self.backend
                .set_prop(id, Prop::MinWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_width {
            self.backend
                .set_prop(id, Prop::MaxWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_height {
            self.backend
                .set_prop(id, Prop::MinHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_height {
            self.backend
                .set_prop(id, Prop::MaxHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.horizontal_alignment {
            self.backend
                .set_prop(id, Prop::HorizontalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.vertical_alignment {
            self.backend
                .set_prop(id, Prop::VerticalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.opacity {
            self.backend.set_prop(id, Prop::Opacity, &PropValue::F64(v));
        }
        if let Some(v) = &mods.background {
            self.backend
                .set_prop(id, Prop::Background, &PropValue::Color(*v));
        }
        if let Some(v) = &mods.foreground {
            self.backend
                .set_prop(id, Prop::Foreground, &PropValue::Color(*v));
        }
        if let Some(v) = &mods.font_family {
            self.backend
                .set_prop(id, Prop::FontFamily, &PropValue::Str(v.clone()));
        }
        if let Some(v) = mods.font_size {
            self.backend
                .set_prop(id, Prop::FontSize, &PropValue::F64(v));
        }

        if let Some(v) = mods.allow_drop {
            self.backend
                .set_prop(id, Prop::AllowDrop, &PropValue::Bool(v));
        }

        self.apply_theme_bindings_for(id, mods);
        self.apply_animations_for(id, mods);
        self.apply_accessibility_for(id, mods);
        self.apply_keyboard_accelerators_for(id, mods);
        self.apply_tooltip_for(id, mods);
        self.apply_pointer_handlers_for(id, mods);
        self.apply_drag_handlers_for(id, mods);

        if let Some(p) = mods.grid {
            self.apply_grid_placement(id, p);
        }

        if !mods.resources.is_empty() {
            self.backend.set_prop(
                id,
                Prop::Resources,
                &PropValue::Resources(mods.resources.clone()),
            );
        }
    }

    fn apply_tooltip_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(tt) = mods.tooltip.as_deref() else {
            return;
        };
        self.backend.set_tooltip(id, Some(tt));
    }

    fn apply_pointer_handlers_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(ph) = mods.pointer_handlers.as_deref() else {
            return;
        };
        if ph.is_empty() {
            return;
        }
        self.backend.set_pointer_handlers(id, Some(ph));
    }

    fn apply_drag_handlers_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(dh) = mods.drag_handlers.as_deref() else {
            return;
        };
        if dh.is_empty() {
            return;
        }
        self.backend.set_drag_handlers(id, Some(dh));
    }

    fn apply_accessibility_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(acc) = mods.accessibility.as_deref() else {
            return;
        };
        if acc.is_empty() {
            return;
        }
        self.backend.set_accessibility(id, acc);
    }

    fn apply_keyboard_accelerators_for(&mut self, id: ControlId, mods: &Modifiers) {
        if mods.keyboard_accelerators.is_empty() {
            return;
        }
        self.backend
            .set_keyboard_accelerators(id, &mods.keyboard_accelerators);
    }

    fn apply_animations_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(anim) = mods.animations.as_deref() else {
            return;
        };
        if anim.is_empty() {
            return;
        }
        if let Some(it) = anim.implicit_transitions
            && !it.is_empty()
        {
            self.backend.set_implicit_transitions(id, Some(it));
        }
        if let Some(la) = anim.layout_animation {
            self.backend.set_layout_animation(id, Some(la));
        }

        let enter = match anim.property_animation {
            None => anim.enter_transition,
            Some(_) => None,
        };
        if enter.is_some() || anim.exit_transition.is_some() {
            self.backend
                .set_element_transitions(id, enter, anim.exit_transition);
        }

        if let Some(p) = anim.property_animation {
            self.backend.run_property_animation(id, Some(p));
        }
    }

    fn diff_animations_for(
        &mut self,
        id: ControlId,
        old: Option<&AnimationModifiers>,
        new: Option<&AnimationModifiers>,
    ) {
        let old_it = old.and_then(|a| a.implicit_transitions);
        let new_it = new.and_then(|a| a.implicit_transitions);
        if old_it != new_it {
            self.backend
                .set_implicit_transitions(id, new_it.filter(|t| !t.is_empty()));
        }
        let old_la = old.and_then(|a| a.layout_animation);
        let new_la = new.and_then(|a| a.layout_animation);
        if old_la != new_la {
            self.backend.set_layout_animation(id, new_la);
        }

        let old_pa = old.and_then(|a| a.property_animation);
        let new_pa = new.and_then(|a| a.property_animation);
        if old_pa != new_pa {
            self.backend.run_property_animation(id, new_pa);
        }

        let old_enter = old.and_then(|a| match a.property_animation {
            None => a.enter_transition,
            Some(_) => None,
        });
        let new_enter = new.and_then(|a| match a.property_animation {
            None => a.enter_transition,
            Some(_) => None,
        });
        let old_exit = old.and_then(|a| a.exit_transition);
        let new_exit = new.and_then(|a| a.exit_transition);
        if old_enter != new_enter || old_exit != new_exit {
            self.backend
                .set_element_transitions(id, new_enter, new_exit);
        }
    }

    fn apply_theme_bindings_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(map) = mods.theme_bindings.as_deref() else {
            return;
        };
        if map.is_empty() {
            return;
        }
        let Some(kind) = self.tree.kind(id) else {
            return;
        };
        let bindings: Vec<(Prop, ThemeRef)> = map.iter().map(|(p, t)| (*p, t.clone())).collect();
        self.backend.set_theme_bindings(id, kind, &bindings);
    }

    pub fn diff_modifiers(&mut self, id: ControlId, old: &Modifiers, new: &Modifiers) {
        self.diff_opt_copy(
            id,
            Prop::Margin,
            old.margin,
            new.margin,
            PropValue::Thickness,
        );
        self.diff_opt_copy(
            id,
            Prop::Padding,
            old.padding,
            new.padding,
            PropValue::Thickness,
        );
        self.diff_opt_f64(id, Prop::Width, old.width, new.width);
        self.diff_opt_f64(id, Prop::Height, old.height, new.height);
        self.diff_opt_f64(id, Prop::MinWidth, old.min_width, new.min_width);
        self.diff_opt_f64(id, Prop::MaxWidth, old.max_width, new.max_width);
        self.diff_opt_f64(id, Prop::MinHeight, old.min_height, new.min_height);
        self.diff_opt_f64(id, Prop::MaxHeight, old.max_height, new.max_height);
        self.diff_opt_copy(
            id,
            Prop::HorizontalAlignment,
            old.horizontal_alignment,
            new.horizontal_alignment,
            |v: HorizontalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_copy(
            id,
            Prop::VerticalAlignment,
            old.vertical_alignment,
            new.vertical_alignment,
            |v: VerticalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_f64(id, Prop::Opacity, old.opacity, new.opacity);
        self.diff_opt_clone(
            id,
            Prop::Background,
            &old.background,
            &new.background,
            PropValue::Color,
        );
        self.diff_opt_clone(
            id,
            Prop::Foreground,
            &old.foreground,
            &new.foreground,
            PropValue::Color,
        );
        self.diff_opt_clone(
            id,
            Prop::FontFamily,
            &old.font_family,
            &new.font_family,
            PropValue::Str,
        );
        self.diff_opt_f64(id, Prop::FontSize, old.font_size, new.font_size);

        if old.theme_bindings != new.theme_bindings {
            let kind = self.tree.kind(id);
            if let Some(kind) = kind {
                let bindings: Vec<(Prop, ThemeRef)> = new
                    .theme_bindings
                    .as_deref()
                    .map(|m| m.iter().map(|(p, t)| (*p, t.clone())).collect())
                    .unwrap_or_default();
                self.backend.set_theme_bindings(id, kind, &bindings);
            }
        }

        let old_anim = old.animations.as_deref();
        let new_anim = new.animations.as_deref();
        if old_anim != new_anim {
            self.diff_animations_for(id, old_anim, new_anim);
        }

        let old_acc = old.accessibility.as_deref();
        let new_acc = new.accessibility.as_deref();
        if old_acc != new_acc {
            static EMPTY: AccessibilityModifiers = AccessibilityModifiers {
                automation_name: None,
                automation_id: None,
                help_text: None,
                live_setting: None,
                heading_level: None,
            };
            let new_acc = new_acc.unwrap_or(&EMPTY);
            self.backend.set_accessibility(id, new_acc);
        }

        let old_ka = &old.keyboard_accelerators;
        let new_ka = &new.keyboard_accelerators;
        if old_ka != new_ka {
            self.backend.set_keyboard_accelerators(id, new_ka);
        }

        // ToolTipService survives re-renders, so clear Some->None explicitly.
        let old_tt = old.tooltip.as_deref();
        let new_tt = new.tooltip.as_deref();
        if old_tt != new_tt {
            self.backend.set_tooltip(id, new_tt);
        }

        // Clear Some->None so event tokens are dropped.
        let old_ph = old.pointer_handlers.as_deref();
        let new_ph = new.pointer_handlers.as_deref();
        if old_ph != new_ph {
            let new_ph = new_ph.filter(|p| !p.is_empty());
            self.backend.set_pointer_handlers(id, new_ph);
        }

        if old.allow_drop != new.allow_drop {
            self.backend.set_prop(
                id,
                Prop::AllowDrop,
                &PropValue::Bool(new.allow_drop.unwrap_or(false)),
            );
        }

        let old_dh = old.drag_handlers.as_deref();
        let new_dh = new.drag_handlers.as_deref();
        if old_dh != new_dh {
            let new_dh = new_dh.filter(|d| !d.is_empty());
            self.backend.set_drag_handlers(id, new_dh);
        }

        // Emit all grid props on change so stale values are cleared.
        if old.grid != new.grid {
            self.apply_grid_placement_full(id, new.grid.unwrap_or_default());
        }

        if old.resources != new.resources {
            self.backend.set_prop(
                id,
                Prop::Resources,
                &PropValue::Resources(new.resources.clone()),
            );
        }
    }

    fn collect_affected_components(
        &self,
        root_id: ControlId,
        changed: &rustc_hash::FxHashSet<ContextId>,
    ) -> Vec<LogicalNodeId> {
        let mut affected = Vec::new();
        let mut stack = vec![root_id];
        let mut logical = Vec::new();
        while let Some(id) = stack.pop() {
            self.tree
                .logical
                .extend_context_subscribers(id, changed, &mut affected);
            self.tree.extend_owned_children(id, &mut stack);
            self.tree.extend_owned_logical_roots(id, &mut logical);
        }
        while let Some(node_id) = logical.pop() {
            if self.tree.logical.instance(node_id).is_some_and(|inst| {
                inst.read_contexts
                    .iter()
                    .any(|context| changed.contains(context))
            }) {
                affected.push(node_id);
            }
            logical.extend(
                self.tree
                    .logical
                    .components()
                    .filter(|node| node.parent == Some(node_id))
                    .map(|node| node.node_id),
            );
            logical.extend(
                self.tree
                    .logical
                    .wrappers()
                    .filter(|node| node.parent == Some(node_id))
                    .map(|node| node.node_id),
            );
        }
        affected
    }

    fn collect_affected_components_for_node(
        &self,
        root: LogicalNodeId,
        changed: &rustc_hash::FxHashSet<ContextId>,
    ) -> Vec<LogicalNodeId> {
        let mut affected: Vec<_> = self
            .tree
            .logical
            .components()
            .filter(|inst| {
                inst.read_contexts
                    .iter()
                    .any(|context| changed.contains(context))
                    && self.logical_is_descendant(inst.node_id, root)
            })
            .map(|inst| inst.node_id)
            .collect();

        if let Some(native_root) = self.tree.logical.node_native_root(root) {
            for node_id in self.collect_affected_components(native_root, changed) {
                if !affected.contains(&node_id) {
                    affected.push(node_id);
                }
            }
        }

        affected
    }

    fn logical_is_descendant(&self, node_id: LogicalNodeId, root: LogicalNodeId) -> bool {
        let mut current = Some(node_id);
        while let Some(node) = current {
            if node == root {
                return true;
            }
            current = self.tree.logical.node_parent(node);
        }
        false
    }

    fn force_all_context_subscribers(&mut self, changed: &rustc_hash::FxHashSet<ContextId>) {
        let affected: Vec<_> = self
            .tree
            .logical
            .components()
            .filter(|inst| inst.read_contexts.iter().any(|id| changed.contains(id)))
            .map(|inst| inst.node_id)
            .collect();
        self.add_forced_node_paths(affected);
    }

    pub fn notify_theme_changed(&mut self) {
        self.backend.on_theme_changed();
    }

    pub fn reconcile_children_positional(
        &mut self,
        parent: ControlId,
        old: &[Element],
        new: &[Element],
    ) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile_positional(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub fn reconcile_children(&mut self, parent: ControlId, old: &[Element], new: &[Element]) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub fn set_inner_size_cell(&mut self, cell: Rc<Cell<WindowSize>>) {
        self.host.inner_size = cell;
    }

    pub fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
        self.host.dpi = cell;
    }

    pub fn force_context_subscribers(
        &mut self,
        root_id: ControlId,
        context_ids: &rustc_hash::FxHashSet<ContextId>,
    ) {
        let affected = self.collect_affected_components(root_id, context_ids);
        if !affected.is_empty() {
            self.add_forced_node_paths(affected);
        }
    }

    pub fn force_context_subscribers_root(
        &mut self,
        context_ids: &rustc_hash::FxHashSet<ContextId>,
    ) {
        self.force_all_context_subscribers(context_ids);
    }

    pub fn clear_forced_components(&mut self) {
        self.pass.forced_nodes.clear();
        self.pass.forced_controls.clear();
    }
}

/// Borrowed child slice retained so logical empty slots stay addressable.
enum LiveChildren<'a> {
    Flat(&'a [Element]),
}

impl<'a> LiveChildren<'a> {
    fn from_slice(slice: &'a [Element]) -> Self {
        LiveChildren::Flat(slice)
    }

    fn as_ref(&self) -> LiveChildrenRef<'_> {
        match self {
            LiveChildren::Flat(s) => LiveChildrenRef::Flat(s),
        }
    }
}

pub enum LiveChildrenRef<'a> {
    Flat(&'a [Element]),
}

impl<'a> LiveChildrenRef<'a> {
    pub fn len(&self) -> usize {
        match self {
            LiveChildrenRef::Flat(s) => s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, i: usize) -> Option<&'a Element> {
        match self {
            LiveChildrenRef::Flat(s) => s.get(i),
        }
    }

    pub fn any_has_key(&self) -> bool {
        match self {
            LiveChildrenRef::Flat(s) => s.iter().any(|e| e.key().is_some()),
        }
    }
}

/// Retains all logical child positions, including empty output.
pub fn collect_live(slice: &[Element]) -> Vec<&Element> {
    slice.iter().collect()
}
