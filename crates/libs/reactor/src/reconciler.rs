use std::cell::Cell;
use std::rc::Rc;

use rustc_hash::FxHashMap;

pub use super::*;

mod child;
mod diff_helpers;
mod templated;
mod widget_dispatch;
mod wrappers;

pub use self::child::compute_lis;
use self::templated::MountedTemplatedTree;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct LogicalNodeId(u64);

enum ProjectedNodes {
    Inline { nodes: [LogicalNodeId; 2], len: u8 },
    Heap(Vec<LogicalNodeId>),
}

impl Default for ProjectedNodes {
    fn default() -> Self {
        Self::Inline {
            nodes: [LogicalNodeId(0); 2],
            len: 0,
        }
    }
}

impl ProjectedNodes {
    fn as_slice(&self) -> &[LogicalNodeId] {
        match self {
            Self::Inline { nodes, len } => &nodes[..*len as usize],
            Self::Heap(nodes) => nodes,
        }
    }

    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    fn last(&self) -> Option<LogicalNodeId> {
        self.as_slice().last().copied()
    }

    fn push(&mut self, node_id: LogicalNodeId) {
        match self {
            Self::Inline { nodes, len } if *len < 2 => {
                nodes[*len as usize] = node_id;
                *len += 1;
            }
            Self::Inline { nodes, .. } => {
                *self = Self::Heap(vec![nodes[0], nodes[1], node_id]);
            }
            Self::Heap(nodes) => nodes.push(node_id),
        }
    }

    fn pop(&mut self) -> Option<LogicalNodeId> {
        match self {
            Self::Inline { nodes, len } => {
                if *len == 0 {
                    None
                } else {
                    *len -= 1;
                    Some(nodes[*len as usize])
                }
            }
            Self::Heap(nodes) => nodes.pop(),
        }
    }

    fn drain(self, mut f: impl FnMut(LogicalNodeId)) {
        match self {
            Self::Inline { nodes, len } => {
                for node_id in nodes.into_iter().take(len as usize) {
                    f(node_id);
                }
            }
            Self::Heap(nodes) => {
                for node_id in nodes {
                    f(node_id);
                }
            }
        }
    }
}

struct LogicalParentGuard {
    active: Rc<Cell<Option<LogicalNodeId>>>,
    previous: Option<LogicalNodeId>,
}

impl Drop for LogicalParentGuard {
    fn drop(&mut self) {
        self.active.set(self.previous);
    }
}

#[derive(Default)]
struct ReconcilePass {
    forced_nodes: rustc_hash::FxHashSet<LogicalNodeId>,
    forced_controls: rustc_hash::FxHashSet<ControlId>,
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

#[derive(Default)]
struct MountedTree {
    children: FxHashMap<ControlId, Vec<ControlId>>,
    nodes: FxHashMap<ControlId, MountedNativeNode>,
    headers: FxHashMap<ControlId, ControlId>,
    panes: FxHashMap<ControlId, ControlId>,
    custom: FxHashMap<ControlId, Box<dyn CustomElement>>,
    before_unmount: FxHashMap<ControlId, Callback<Option<windows_core::IInspectable>>>,
    templated: MountedTemplatedTree,
    logical: MountedLogicalTree,
}

struct MountedNativeNode {
    kind: Option<ControlKind>,
    parent: Option<ControlId>,
}

#[derive(Default)]
struct MountedLogicalTree {
    components: FxHashMap<LogicalNodeId, ComponentInstance>,
    wrappers: FxHashMap<LogicalNodeId, LogicalWrapperNode>,
    // Logical nodes projecting to a native root, innermost first.
    projections: FxHashMap<ControlId, ProjectedNodes>,
    active_parent: Rc<Cell<Option<LogicalNodeId>>>,
    next_id: u64,
    appeared_listener_count: usize,
    disappeared_listener_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogicalNodeKind {
    Component,
    Provider,
    ErrorBoundary,
}

struct LogicalWrapperNode {
    kind: LogicalNodeKind,
    node_id: LogicalNodeId,
    parent: Option<LogicalNodeId>,
    native_root: ControlId,
    fallback: bool,
}

impl MountedLogicalTree {
    fn allocate_id(&mut self) -> LogicalNodeId {
        let id = LogicalNodeId(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("logical component id overflow");
        id
    }

    fn enter_parent(&self, node_id: LogicalNodeId) -> LogicalParentGuard {
        let active = Rc::clone(&self.active_parent);
        let previous = active.replace(Some(node_id));
        LogicalParentGuard { active, previous }
    }

    fn active_parent(&self) -> Option<LogicalNodeId> {
        self.active_parent.get()
    }

    fn instance(&self, node_id: LogicalNodeId) -> Option<&ComponentInstance> {
        self.components.get(&node_id)
    }

    fn node_kind(&self, node_id: LogicalNodeId) -> Option<LogicalNodeKind> {
        if self.components.contains_key(&node_id) {
            Some(LogicalNodeKind::Component)
        } else {
            self.wrappers.get(&node_id).map(|node| node.kind)
        }
    }

    fn node_parent(&self, node_id: LogicalNodeId) -> Option<LogicalNodeId> {
        self.components
            .get(&node_id)
            .and_then(|node| node.parent)
            .or_else(|| self.wrappers.get(&node_id).and_then(|node| node.parent))
    }

    fn node_native_root(&self, node_id: LogicalNodeId) -> Option<ControlId> {
        self.components
            .get(&node_id)
            .map(|node| node.native_root)
            .or_else(|| self.wrappers.get(&node_id).map(|node| node.native_root))
    }

    #[cfg(debug_assertions)]
    fn contains_node(&self, node_id: LogicalNodeId) -> bool {
        self.components.contains_key(&node_id) || self.wrappers.contains_key(&node_id)
    }

    #[cfg(any(debug_assertions, feature = "test"))]
    fn node_count(&self) -> usize {
        self.components.len() + self.wrappers.len()
    }

    fn current_node(&self, id: ControlId, kind: LogicalNodeKind) -> Option<LogicalNodeId> {
        let node_id = self.projections.get(&id).and_then(ProjectedNodes::last)?;
        (self.node_kind(node_id) == Some(kind)).then_some(node_id)
    }

    fn register_component(&mut self, id: ControlId, inst: ComponentInstance) {
        if inst.last_obj.has_on_appeared() {
            self.appeared_listener_count += 1;
        }
        if inst.last_obj.has_on_disappeared() {
            self.disappeared_listener_count += 1;
        }
        let previous_root = inst.native_root;
        let node_id = inst.node_id;
        let previous = self.components.insert(
            node_id,
            ComponentInstance {
                native_root: id,
                ..inst
            },
        );
        debug_assert!(previous.is_none(), "logical component registered twice");
        self.register_projection(id, node_id, previous_root);
    }

    fn register_wrapper(&mut self, id: ControlId, mut node: LogicalWrapperNode) {
        let previous_root = node.native_root;
        node.native_root = id;
        let node_id = node.node_id;
        let previous = self.wrappers.insert(node_id, node);
        debug_assert!(previous.is_none(), "logical wrapper registered twice");
        self.register_projection(id, node_id, previous_root);
    }

    fn take_component(&mut self, id: ControlId) -> Option<ComponentInstance> {
        let node_id = self.pop_projection(id, LogicalNodeKind::Component)?;
        self.remove_component(node_id)
    }

    fn take_provider(&mut self, id: ControlId) -> Option<LogicalWrapperNode> {
        self.take_wrapper(id, LogicalNodeKind::Provider)
    }

    fn take_error_boundary(&mut self, id: ControlId) -> Option<LogicalWrapperNode> {
        self.take_wrapper(id, LogicalNodeKind::ErrorBoundary)
    }

    fn take_wrapper(&mut self, id: ControlId, kind: LogicalNodeKind) -> Option<LogicalWrapperNode> {
        let node_id = self.pop_projection(id, kind)?;
        self.wrappers.remove(&node_id)
    }

    fn pop_projection(
        &mut self,
        id: ControlId,
        expected: LogicalNodeKind,
    ) -> Option<LogicalNodeId> {
        let node_id = self.projections.get_mut(&id)?.pop()?;
        debug_assert_eq!(
            self.node_kind(node_id),
            Some(expected),
            "logical projection update order disagrees with element nesting"
        );
        Some(node_id)
    }

    fn take_projection(&mut self, id: ControlId) -> Option<ProjectedNodes> {
        self.projections.remove(&id)
    }

    #[cfg(feature = "test")]
    fn projected_nodes(&self, id: ControlId) -> Option<&ProjectedNodes> {
        self.projections.get(&id)
    }

    fn refresh_instance(
        &mut self,
        node_id: LogicalNodeId,
        obj: Rc<dyn ComponentObject>,
        parent: Option<LogicalNodeId>,
        native_root: ControlId,
    ) {
        let Some(inst) = self.components.get_mut(&node_id) else {
            return;
        };
        inst.last_obj = obj;
        inst.parent = parent;
        inst.native_root = native_root;
    }

    fn extend_context_subscribers(
        &self,
        id: ControlId,
        changed: &rustc_hash::FxHashSet<ContextId>,
        affected: &mut Vec<LogicalNodeId>,
    ) {
        let Some(node_ids) = self.projections.get(&id) else {
            return;
        };
        affected.extend(
            node_ids
                .as_slice()
                .iter()
                .filter(|node_id| {
                    self.instance(**node_id).is_some_and(|inst| {
                        inst.read_contexts
                            .iter()
                            .any(|context| changed.contains(context))
                    })
                })
                .copied(),
        );
    }

    fn remove_component(&mut self, node_id: LogicalNodeId) -> Option<ComponentInstance> {
        let inst = self.components.remove(&node_id)?;
        if inst.last_obj.has_on_appeared() {
            debug_assert!(
                self.appeared_listener_count > 0,
                "appeared_listener_count underflow: register/take are mismatched"
            );
            self.appeared_listener_count -= 1;
        }
        if inst.last_obj.has_on_disappeared() {
            debug_assert!(
                self.disappeared_listener_count > 0,
                "disappeared_listener_count underflow: register/take are mismatched"
            );
            self.disappeared_listener_count -= 1;
        }
        Some(inst)
    }

    fn remove_wrapper(&mut self, node_id: LogicalNodeId) {
        self.wrappers.remove(&node_id);
    }

    fn register_projection(
        &mut self,
        id: ControlId,
        node_id: LogicalNodeId,
        previous_root: ControlId,
    ) {
        self.projections.entry(id).or_default().push(node_id);
        self.remove_empty_projection(previous_root);
    }

    fn remove_empty_projection(&mut self, id: ControlId) {
        if self
            .projections
            .get(&id)
            .is_some_and(ProjectedNodes::is_empty)
        {
            self.projections.remove(&id);
        }
    }

    fn dispatch_appeared(&mut self, id: ControlId, context_stack: &Rc<ContextStack>) {
        if self.appeared_listener_count == 0 {
            return;
        }
        let Some(node_ids) = self.projections.get(&id) else {
            return;
        };
        for node_id in node_ids.as_slice().iter().rev() {
            if let Some(inst) = self.components.get_mut(node_id)
                && inst.last_obj.has_on_appeared()
            {
                inst.render_cx.set_context_stack(Rc::clone(context_stack));
                inst.last_obj.invoke_appeared(&mut inst.render_cx);
            }
        }
    }

    fn dispatch_disappeared(&mut self, id: ControlId, context_stack: &Rc<ContextStack>) {
        if self.disappeared_listener_count == 0 {
            return;
        }
        let Some(node_ids) = self.projections.get(&id) else {
            return;
        };
        for node_id in node_ids.as_slice() {
            if let Some(inst) = self.components.get_mut(node_id)
                && inst.last_obj.has_on_disappeared()
            {
                inst.render_cx.set_context_stack(Rc::clone(context_stack));
                inst.last_obj.invoke_disappeared(&mut inst.render_cx);
            }
        }
    }
}

impl MountedTree {
    fn register(&mut self, id: ControlId, kind: Option<ControlKind>) {
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.clear_parent(child, id);
            }
        }
        if let Some(header) = self.headers.remove(&id) {
            self.clear_parent(header, id);
        }
        if let Some(pane) = self.panes.remove(&id) {
            self.clear_parent(pane, id);
        }
        self.custom.remove(&id);
        self.before_unmount.remove(&id);
        self.nodes
            .insert(id, MountedNativeNode { kind, parent: None });
    }

    fn kind(&self, id: ControlId) -> Option<ControlKind> {
        self.nodes.get(&id).and_then(|node| node.kind)
    }

    fn parent(&self, id: ControlId) -> Option<ControlId> {
        self.nodes.get(&id).and_then(|node| node.parent)
    }

    fn set_parent(&mut self, child: ControlId, parent: ControlId) {
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

    fn clear_parent(&mut self, child: ControlId, parent: ControlId) {
        if let Some(node) = self.nodes.get_mut(&child)
            && node.parent == Some(parent)
        {
            node.parent = None;
        }
    }

    fn set_header(&mut self, parent: ControlId, header: Option<ControlId>) {
        if let Some(old) = self.headers.remove(&parent) {
            self.clear_parent(old, parent);
        }
        if let Some(header) = header {
            self.set_parent(header, parent);
            self.headers.insert(parent, header);
        }
    }

    fn header(&self, parent: ControlId) -> Option<ControlId> {
        self.headers.get(&parent).copied()
    }

    fn set_pane(&mut self, parent: ControlId, pane: Option<ControlId>) {
        if let Some(old) = self.panes.remove(&parent) {
            self.clear_parent(old, parent);
        }
        if let Some(pane) = pane {
            self.set_parent(pane, parent);
            self.panes.insert(parent, pane);
        }
    }

    fn pane(&self, parent: ControlId) -> Option<ControlId> {
        self.panes.get(&parent).copied()
    }

    fn set_custom(&mut self, id: ControlId, handle: Box<dyn CustomElement>) {
        debug_assert!(self.nodes.contains_key(&id));
        self.custom.insert(id, handle);
    }

    fn take_custom(&mut self, id: ControlId) -> Option<Box<dyn CustomElement>> {
        self.custom.remove(&id)
    }

    fn set_before_unmount(
        &mut self,
        id: ControlId,
        callback: Option<Callback<Option<windows_core::IInspectable>>>,
    ) {
        debug_assert!(self.nodes.contains_key(&id));
        if let Some(callback) = callback {
            self.before_unmount.insert(id, callback);
        } else {
            self.before_unmount.remove(&id);
        }
    }

    fn take_before_unmount(
        &mut self,
        id: ControlId,
    ) -> Option<Callback<Option<windows_core::IInspectable>>> {
        self.before_unmount.remove(&id)
    }

    fn children(&self, parent: ControlId) -> &[ControlId] {
        self.children.get(&parent).map_or(&[], Vec::as_slice)
    }

    fn child(&self, parent: ControlId, index: usize) -> Option<ControlId> {
        self.children(parent).get(index).copied()
    }

    fn child_position(&self, parent: ControlId, child: ControlId) -> Option<usize> {
        self.children(parent).iter().position(|id| *id == child)
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        self.set_parent(child, parent);
        self.children.entry(parent).or_default().push(child);
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) -> Option<ControlId> {
        let removed = self
            .children
            .get_mut(&parent)
            .and_then(|list| (index < list.len()).then(|| list.remove(index)));
        if let Some(child) = removed {
            self.clear_parent(child, parent);
        }
        removed
    }

    fn replace_child(
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

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
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

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) -> usize {
        self.set_parent(child, parent);
        let list = self.children.entry(parent).or_default();
        let index = index.min(list.len());
        list.insert(index, child);
        index
    }

    fn remove_node(&mut self, id: ControlId) {
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.clear_parent(child, id);
            }
        }
        if let Some(header) = self.headers.remove(&id) {
            self.clear_parent(header, id);
        }
        if let Some(pane) = self.panes.remove(&id) {
            self.clear_parent(pane, id);
        }
        self.custom.remove(&id);
        self.before_unmount.remove(&id);
        self.nodes.remove(&id);
    }
}

/// Diff/apply engine that drives a [`Backend`] from successive [`Element`] trees.
pub struct Reconciler<B: Backend> {
    pub backend: B,
    tree: MountedTree,
    pass: ReconcilePass,
    host: HostContext,
    stats: ReconcileStats,
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
    native_root: ControlId,
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
        let drained = std::mem::take(&mut self.tree.templated.deferred_unmounts);
        for cid in drained {
            self.unmount(cid);
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
        self.tree.logical.components.len()
    }

    #[cfg(feature = "test")]
    pub fn debug_logical_node_count(&self) -> usize {
        self.tree.logical.node_count()
    }

    fn allocate_logical_node_id(&mut self) -> LogicalNodeId {
        self.tree.logical.allocate_id()
    }

    fn enter_logical_parent(&self, node_id: LogicalNodeId) -> LogicalParentGuard {
        self.tree.logical.enter_parent(node_id)
    }

    fn current_component_node(&self, id: ControlId) -> Option<LogicalNodeId> {
        self.tree
            .logical
            .current_node(id, LogicalNodeKind::Component)
    }

    fn add_forced_node_path(&mut self, node_id: LogicalNodeId) {
        let mut current = Some(node_id);
        while let Some(id) = current {
            let Some(native_root) = self.tree.logical.node_native_root(id) else {
                break;
            };
            self.pass.forced_nodes.insert(id);
            let mut control = Some(native_root);
            while let Some(id) = control {
                if !self.pass.forced_controls.insert(id) {
                    break;
                }
                control = self.tree.parent(id);
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

    fn register_component_instance(&mut self, id: ControlId, inst: ComponentInstance) {
        self.tree.logical.register_component(id, inst);
    }

    fn take_component_instance(&mut self, id: ControlId) -> Option<ComponentInstance> {
        self.tree.logical.take_component(id)
    }

    fn remove_empty_component_projection(&mut self, id: ControlId) {
        self.tree.logical.remove_empty_projection(id);
    }

    #[cfg(feature = "test")]
    pub fn debug_appeared_listener_count(&self) -> usize {
        self.tree.logical.appeared_listener_count
    }

    #[cfg(feature = "test")]
    pub fn debug_disappeared_listener_count(&self) -> usize {
        self.tree.logical.disappeared_listener_count
    }

    pub fn reconcile(
        &mut self,
        old: Option<&Element>,
        new: &Element,
        existing: Option<ControlId>,
        request_rerender: Rc<dyn Fn()>,
    ) -> Option<ControlId> {
        self.host.request_rerender = request_rerender;
        let result = match (existing, old) {
            (None, _) => self.mount(new),
            (Some(id), Some(old_el)) => {
                let seeded = self.force_state_dirty_components();
                let result = self.update(old_el, new, id);
                debug_assert!(
                    seeded
                        .iter()
                        .all(|node_id| !self.is_node_state_dirty(*node_id)),
                    "a state-dirty component was not re-rendered by the pass"
                );
                result
            }
            (Some(_id), None) => self.mount(new),
        };
        #[cfg(debug_assertions)]
        {
            self.debug_assert_component_index();
            self.debug_assert_native_ownership();
        }
        result
    }

    #[cfg(debug_assertions)]
    fn debug_assert_component_index(&self) {
        let mut indexed = rustc_hash::FxHashSet::default();
        for (control_id, nodes) in &self.tree.logical.projections {
            debug_assert!(
                !nodes.is_empty(),
                "empty logical projection for {control_id:?}"
            );
            for node_id in nodes.as_slice() {
                debug_assert!(
                    indexed.insert(*node_id),
                    "logical node {node_id:?} is indexed more than once"
                );
                debug_assert!(
                    self.tree.logical.contains_node(*node_id),
                    "projected logical node has no instance"
                );
                debug_assert_eq!(
                    self.tree.logical.node_native_root(*node_id),
                    Some(*control_id),
                    "logical node native root disagrees with projection"
                );
            }
        }

        debug_assert_eq!(
            indexed.len(),
            self.tree.logical.node_count(),
            "logical projection index does not cover every node"
        );
        for (node_id, node) in &self.tree.logical.components {
            if let Some(parent) = node.parent {
                debug_assert!(
                    self.tree.logical.contains_node(parent),
                    "logical node parent is not mounted"
                );
            }
            debug_assert_eq!(*node_id, node.node_id);
        }
        for (node_id, node) in &self.tree.logical.wrappers {
            if let Some(parent) = node.parent {
                debug_assert!(
                    self.tree.logical.contains_node(parent),
                    "logical node parent is not mounted"
                );
            }
            debug_assert_eq!(*node_id, node.node_id);
        }
    }

    #[cfg(debug_assertions)]
    fn debug_assert_native_ownership(&self) {
        let mut owned = rustc_hash::FxHashSet::default();
        let mut record = |parent: ControlId, child: ControlId| {
            debug_assert!(
                owned.insert(child),
                "native control {child:?} has more than one owner"
            );
            debug_assert_eq!(
                self.tree.parent(child),
                Some(parent),
                "native control {child:?} disagrees with its owner"
            );
        };

        for (parent, children) in &self.tree.children {
            for child in children {
                record(*parent, *child);
            }
        }
        for (parent, header) in &self.tree.headers {
            record(*parent, *header);
        }
        for (parent, pane) in &self.tree.panes {
            record(*parent, *pane);
        }
        for (parent, state) in &self.tree.templated.lists {
            for row in state.rows.values() {
                record(*parent, row.content_id);
            }
        }

        for (id, node) in &self.tree.nodes {
            if node.parent.is_some() {
                debug_assert!(
                    owned.contains(id),
                    "native control {id:?} has a parent but is absent from its owner's children"
                );
            }
        }
        for id in self.tree.custom.keys() {
            debug_assert!(
                self.tree.nodes.contains_key(id),
                "custom handle {id:?} has no mounted native node"
            );
        }
        for id in self.tree.before_unmount.keys() {
            debug_assert!(
                self.tree.nodes.contains_key(id),
                "pre-unmount callback {id:?} has no mounted native node"
            );
        }
    }

    /// Forces dirty components to render even when unchanged parents can be skipped.
    fn force_state_dirty_components(&mut self) -> Vec<LogicalNodeId> {
        let dirty: Vec<LogicalNodeId> = self
            .tree
            .logical
            .components
            .iter()
            .filter_map(|(node_id, inst)| inst.render_cx.peek_state_dirty().then_some(*node_id))
            .collect();
        if !dirty.is_empty() {
            self.add_forced_node_paths(dirty.iter().copied());
        }
        dirty
    }

    pub fn mount(&mut self, el: &Element) -> Option<ControlId> {
        match el {
            Element::Component(ce) => return self.mount_component(ce),
            Element::ErrorBoundary(eb) => return self.mount_error_boundary(eb),
            Element::Provider(pe) => return self.mount_provider(pe),
            Element::TemplatedList(tl) => return Some(self.mount_templated_list(tl)),
            Element::Custom(c) => return Some(self.mount_custom(c)),
            Element::Empty => return None,
            _ => {}
        }
        let widget = el.as_widget().unwrap();
        let id = self.mount_widget(widget);
        if let Element::RichTextBlock(rt) = el
            && !rt.paragraphs.is_empty()
        {
            self.backend.set_rich_text_paragraphs(id, &rt.paragraphs);
        }
        Some(id)
    }

    pub fn update(&mut self, old: &Element, new: &Element, id: ControlId) -> Option<ControlId> {
        if can_skip_update(old, new) && !self.is_control_forced(id) {
            self.stats.elements_skipped += 1;
            return Some(id);
        }
        self.stats.elements_diffed += 1;

        if !old.kind_matches(new) {
            self.unmount(id);
            return self.mount(new);
        }

        match (old, new) {
            (Element::Component(o), Element::Component(n)) => {
                return self.update_component(o, n, id);
            }
            (Element::ErrorBoundary(o), Element::ErrorBoundary(n)) => {
                return self.update_error_boundary(o, n, id);
            }
            (Element::Provider(o), Element::Provider(n)) => return self.update_provider(o, n, id),
            (Element::TemplatedList(o), Element::TemplatedList(n)) => {
                self.update_templated_list(o, n, id);
                return Some(id);
            }
            (Element::Custom(o), Element::Custom(n)) => {
                return Some(self.update_custom(o, n, id));
            }
            (Element::Empty, Element::Empty) => return None,
            _ => {}
        }

        let (Some(ow), Some(nw)) = (old.as_widget(), new.as_widget()) else {
            unreachable!("kind_matches guarantees same variant; non-widget variants handled above");
        };
        self.update_widget(ow, nw, id);
        if let (Element::RichTextBlock(o), Element::RichTextBlock(n)) = (old, new)
            && o.paragraphs != n.paragraphs
        {
            self.backend.set_rich_text_paragraphs(id, &n.paragraphs);
        }
        Some(id)
    }

    pub fn unmount(&mut self, id: ControlId) {
        let mut nodes = vec![id];
        let mut next = 0;
        while next < nodes.len() {
            let node = nodes[next];
            next += 1;
            if let Some(pane) = self.tree.pane(node) {
                nodes.push(pane);
            }
            if let Some(header) = self.tree.header(node) {
                nodes.push(header);
            }
            if let Some(state) = self.tree.templated.lists.get(&node) {
                nodes.extend(state.rows.values().map(|row| row.content_id));
            }
            nodes.extend_from_slice(self.tree.children(node));
        }

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
            if let Some(cb) = self.tree.take_before_unmount(node) {
                cb.invoke(self.backend.get_native_element(node));
            }

            if let Some(handle) = self.tree.take_custom(node) {
                handle.before_destroy(node, &mut self.backend);
            }

            self.tree.remove_node(node);
            self.backend.destroy(node);
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
        while let Some(id) = stack.pop() {
            self.tree
                .logical
                .extend_context_subscribers(id, changed, &mut affected);
            for k in self.tree.children(id) {
                stack.push(*k);
            }
        }
        affected
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

    pub fn clear_forced_components(&mut self) {
        self.pass.forced_nodes.clear();
        self.pass.forced_controls.clear();
    }
}

/// Borrowed child slice with manually inserted empty elements removed.
enum LiveChildren<'a> {
    Flat(&'a [Element]),
    Filtered(Vec<&'a Element>),
}

impl<'a> LiveChildren<'a> {
    fn from_slice(slice: &'a [Element]) -> Self {
        let needs_filter = slice.iter().any(|e| matches!(e, Element::Empty));
        if needs_filter {
            LiveChildren::Filtered(collect_live(slice))
        } else {
            LiveChildren::Flat(slice)
        }
    }

    fn as_ref(&self) -> LiveChildrenRef<'_> {
        match self {
            LiveChildren::Flat(s) => LiveChildrenRef::Flat(s),
            LiveChildren::Filtered(v) => LiveChildrenRef::Filtered(v),
        }
    }
}

pub enum LiveChildrenRef<'a> {
    Flat(&'a [Element]),
    Filtered(&'a [&'a Element]),
}

impl<'a> LiveChildrenRef<'a> {
    pub fn len(&self) -> usize {
        match self {
            LiveChildrenRef::Flat(s) => s.len(),
            LiveChildrenRef::Filtered(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, i: usize) -> Option<&'a Element> {
        match self {
            LiveChildrenRef::Flat(s) => s.get(i),
            LiveChildrenRef::Filtered(v) => v.get(i).copied(),
        }
    }

    pub fn any_has_key(&self) -> bool {
        match self {
            LiveChildrenRef::Flat(s) => s.iter().any(|e| e.key().is_some()),
            LiveChildrenRef::Filtered(v) => v.iter().any(|e| e.key().is_some()),
        }
    }
}

/// Drops manually inserted empty children.
pub fn collect_live(slice: &[Element]) -> Vec<&Element> {
    let mut children = Vec::with_capacity(slice.len());
    children.extend(
        slice
            .iter()
            .filter(|element| !matches!(element, Element::Empty)),
    );
    children
}
