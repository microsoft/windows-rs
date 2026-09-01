use super::scope::ScopeId;
use super::*;
use crate::reference::NativeElementRef;
use rustc_hash::FxHashMap as HashMap;
use std::any::TypeId;
use std::collections::BTreeMap;
use std::rc::Rc;

const PROVIDER_CHUNK_CAPACITY: usize = 256;
const PROVIDER_GROUP_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    Application,
    Window,
    Component,
    Fragment,
    Provider,
    Slot,
    NamedSlot(SlotId),
    Tooltip(TooltipPlacement),
    Flyout(FlyoutPlacement),
    Menu(OwnedMenuKind),
    CommandBarFlyout,
    TreeNodes,
    ContentDialog(bool),
    Native(MountedKind),
    VirtualCollection,
}

#[derive(Clone)]
struct Node {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    key: Option<Key>,
    data: NodeData,
}

#[derive(Clone)]
enum NodeData {
    Application,
    Window,
    Component(ComponentData),
    Fragment,
    Provider,
    Slot,
    NamedSlot(SlotId),
    Tooltip(TooltipPlacement),
    Flyout(FlyoutPlacement),
    Menu {
        kind: OwnedMenuKind,
        state: OwnedState<Vec<MenuItem>>,
    },
    CommandBarFlyout(OwnedState<(Vec<CommandBarCommand>, Vec<CommandBarCommand>)>),
    TreeNodes(Rc<Vec<TreeNode>>),
    ContentDialog(bool),
    Native(NativeData),
    Virtual(VirtualData),
}

impl NodeData {
    fn structural(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Application => Self::Application,
            NodeKind::Window => Self::Window,
            NodeKind::Fragment => Self::Fragment,
            NodeKind::Slot => Self::Slot,
            NodeKind::NamedSlot(slot) => Self::NamedSlot(slot),
            NodeKind::Tooltip(placement) => Self::Tooltip(placement),
            NodeKind::Flyout(placement) => Self::Flyout(placement),
            NodeKind::ContentDialog(open) => Self::ContentDialog(open),
            NodeKind::Component
            | NodeKind::Provider
            | NodeKind::Menu(_)
            | NodeKind::CommandBarFlyout
            | NodeKind::TreeNodes
            | NodeKind::Native(_)
            | NodeKind::VirtualCollection => panic!("node kind requires associated data: {kind:?}"),
        }
    }

    fn kind(&self) -> NodeKind {
        match self {
            Self::Application => NodeKind::Application,
            Self::Window => NodeKind::Window,
            Self::Component(_) => NodeKind::Component,
            Self::Fragment => NodeKind::Fragment,
            Self::Provider => NodeKind::Provider,
            Self::Slot => NodeKind::Slot,
            Self::NamedSlot(slot) => NodeKind::NamedSlot(*slot),
            Self::Tooltip(placement) => NodeKind::Tooltip(*placement),
            Self::Flyout(placement) => NodeKind::Flyout(*placement),
            Self::Menu { kind, .. } => NodeKind::Menu(*kind),
            Self::CommandBarFlyout(_) => NodeKind::CommandBarFlyout,
            Self::TreeNodes(_) => NodeKind::TreeNodes,
            Self::ContentDialog(open) => NodeKind::ContentDialog(*open),
            Self::Native(native) => NodeKind::Native(native.kind),
            Self::Virtual(_) => NodeKind::VirtualCollection,
        }
    }
}

#[derive(Clone)]
struct ComponentData {
    component_type: TypeId,
    scope: ScopeId,
}

#[derive(Clone)]
struct NativeData {
    kind: MountedKind,
    state: NativeState,
}

#[derive(Clone)]
enum VirtualData {
    #[cfg(test)]
    Bare {
        model: VirtualModel,
        realized: HashMap<RealizedContainer, RealizedRow>,
    },
    Items {
        model: VirtualModel,
        realized: HashMap<RealizedContainer, RealizedRow>,
        native: NativeState,
        items: Rc<VirtualItems>,
    },
}

impl VirtualData {
    fn model(&self) -> &VirtualModel {
        match self {
            #[cfg(test)]
            Self::Bare { model, .. } => model,
            Self::Items { model, .. } => model,
        }
    }

    fn model_mut(&mut self) -> &mut VirtualModel {
        match self {
            #[cfg(test)]
            Self::Bare { model, .. } => model,
            Self::Items { model, .. } => model,
        }
    }

    fn realized(&self) -> &HashMap<RealizedContainer, RealizedRow> {
        match self {
            #[cfg(test)]
            Self::Bare { realized, .. } => realized,
            Self::Items { realized, .. } => realized,
        }
    }

    fn realized_mut(&mut self) -> &mut HashMap<RealizedContainer, RealizedRow> {
        match self {
            #[cfg(test)]
            Self::Bare { realized, .. } => realized,
            Self::Items { realized, .. } => realized,
        }
    }
}

#[derive(Clone)]
struct OwnedState<T> {
    callback: Callback<String>,
    revision: u32,
    content: T,
}

impl<T> OwnedState<T> {
    fn replace(&mut self, callback: Callback<String>, content: T) -> u32 {
        self.revision = self.revision.checked_add(1).unwrap();
        self.callback = callback;
        self.content = content;
        self.revision
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealizedRow {
    pub index: usize,
    pub logical_root: NodeId,
    pub native_root: Option<NodeId>,
}

#[derive(Clone)]
pub struct NativeState {
    pub desired: MountedProps,
    pub reference: Option<NativeElementRef>,
    pub properties: BTreeMap<PropertyId, Option<PropertyValue>>,
    pub events: BTreeMap<EventId, EventState>,
}

impl NativeState {
    fn new(desired: MountedProps) -> Self {
        let mut events = BTreeMap::new();
        desired.visit_events(&mut |event, active| {
            if active {
                events.insert(
                    event,
                    EventState {
                        revision: 1,
                        active: true,
                    },
                );
            }
        });
        Self {
            desired,
            reference: None,
            properties: BTreeMap::new(),
            events,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EventState {
    pub revision: u32,
    pub active: bool,
}

#[derive(Clone)]
pub struct Tree {
    arena: Arena<Node>,
    components: Rc<HashMap<ScopeId, NodeId>>,
    exit_transitions: Rc<HashMap<NodeId, ExitTransition>>,
    providers: ProviderStore,
    root: Option<NodeId>,
    owned_attachments: Rc<HashMap<NodeId, (NodeId, NodeId)>>,
    window_declarations: Rc<HashMap<ScopeId, WindowDeclarations>>,
    window_title_bars: Rc<HashMap<NodeId, WindowTitleBarHeight>>,
}

#[derive(Clone, Default)]
struct WindowDeclarations {
    color_scheme: ObservationSlot<ColorScheme>,
    title: Option<Rc<str>>,
    visuals: Option<WindowVisuals>,
    window_size: ObservationSlot<WindowSize>,
}

#[derive(Clone, Default)]
struct ObservationSlot<T> {
    callback: Option<Callback<T>>,
    revision: u32,
}

impl<T> ObservationSlot<T> {
    fn get(&self) -> Option<(&Callback<T>, u32)> {
        self.callback
            .as_ref()
            .map(|callback| (callback, self.revision))
    }

    fn set(&mut self, callback: Option<Callback<T>>) {
        if callback.is_some() && callback.as_ref() != self.callback.as_ref() {
            self.revision = self.revision.wrapping_add(1);
        }
        self.callback = callback;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WindowTitleState {
    pub(crate) owner: ScopeId,
    pub(crate) title: Rc<str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WindowTitleBarState {
    pub(crate) title_bar: NodeId,
    pub(crate) height: WindowTitleBarHeight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WindowVisualsState {
    pub(crate) owner: ScopeId,
    pub(crate) visuals: WindowVisuals,
}

#[derive(Clone, Default)]
struct ProviderStore {
    groups: Rc<Vec<Rc<Vec<Rc<HashMap<NodeId, ContextProvision>>>>>>,
}

impl ProviderStore {
    fn get(&self, id: NodeId) -> Option<&ContextProvision> {
        let chunk = id.index() / PROVIDER_CHUNK_CAPACITY;
        self.groups
            .get(chunk / PROVIDER_GROUP_CAPACITY)?
            .get(chunk % PROVIDER_GROUP_CAPACITY)?
            .get(&id)
    }

    fn insert(&mut self, id: NodeId, provision: ContextProvision) {
        let chunk = id.index() / PROVIDER_CHUNK_CAPACITY;
        let group = chunk / PROVIDER_GROUP_CAPACITY;
        let groups = Rc::make_mut(&mut self.groups);
        while groups.len() <= group {
            groups.push(Rc::new(Vec::new()));
        }
        let chunks = Rc::make_mut(&mut groups[group]);
        while chunks.len() <= chunk % PROVIDER_GROUP_CAPACITY {
            chunks.push(Rc::new(HashMap::default()));
        }
        Rc::make_mut(&mut chunks[chunk % PROVIDER_GROUP_CAPACITY]).insert(id, provision);
    }

    fn remove(&mut self, id: NodeId) {
        let chunk = id.index() / PROVIDER_CHUNK_CAPACITY;
        let group = chunk / PROVIDER_GROUP_CAPACITY;
        let Some(chunks) = Rc::make_mut(&mut self.groups).get_mut(group) else {
            return;
        };
        if let Some(providers) = Rc::make_mut(chunks).get_mut(chunk % PROVIDER_GROUP_CAPACITY) {
            Rc::make_mut(providers).remove(&id);
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            components: Rc::new(HashMap::default()),
            exit_transitions: Rc::new(HashMap::default()),
            providers: ProviderStore::default(),
            root: None,
            owned_attachments: Rc::new(HashMap::default()),
            window_declarations: Rc::new(HashMap::default()),
            window_title_bars: Rc::new(HashMap::default()),
        }
    }

    // Avoid `unwrap` here: its caller tracking measurably slows these hot-path invariant lookups.
    #[inline(always)]
    fn node(&self, id: NodeId) -> &Node {
        match self.arena.get(id) {
            Some(node) => node,
            None => panic!("stale node"),
        }
    }

    #[inline(always)]
    fn node_mut(&mut self, id: NodeId) -> &mut Node {
        match self.arena.get_mut(id) {
            Some(node) => node,
            None => panic!("stale node"),
        }
    }

    pub fn insert(&mut self, parent: Option<NodeId>, kind: NodeKind) -> NodeId {
        self.insert_data(parent, None, NodeData::structural(kind))
    }

    fn insert_data(&mut self, parent: Option<NodeId>, key: Option<Key>, data: NodeData) -> NodeId {
        if let Some(parent) = parent {
            self.node(parent);
        } else if self.root.is_some() {
            panic!("tree already has a root");
        }

        let id = self.arena.insert(Node {
            parent,
            children: Vec::new(),
            key,
            data,
        });

        if let Some(parent) = parent {
            self.node_mut(parent).children.push(id);
        } else {
            self.root = Some(id);
        }
        id
    }

    pub fn insert_native(
        &mut self,
        parent: Option<NodeId>,
        kind: MountedKind,
        key: Option<Key>,
        desired: MountedProps,
        window_title_bar: Option<WindowTitleBarHeight>,
    ) -> NodeId {
        let id = self.insert_data(
            parent,
            key,
            NodeData::Native(NativeData {
                kind,
                state: NativeState::new(desired),
            }),
        );
        if let Some(height) = window_title_bar {
            Rc::make_mut(&mut self.window_title_bars).insert(id, height);
        }
        id
    }

    pub fn insert_component(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        scope: ScopeId,
        component_type: TypeId,
    ) -> NodeId {
        let id = self.insert_data(
            parent,
            key,
            NodeData::Component(ComponentData {
                component_type,
                scope,
            }),
        );
        Rc::make_mut(&mut self.components).insert(scope, id);
        id
    }

    pub fn insert_fragment(&mut self, parent: Option<NodeId>, key: Option<Key>) -> NodeId {
        self.insert_data(parent, key, NodeData::Fragment)
    }

    pub fn insert_provider(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        provision: ContextProvision,
    ) -> NodeId {
        let id = self.insert_data(parent, key, NodeData::Provider);
        self.providers.insert(id, provision);
        id
    }

    pub fn insert_tooltip(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        placement: TooltipPlacement,
    ) -> NodeId {
        self.insert_data(parent, key, NodeData::Tooltip(placement))
    }

    pub fn insert_flyout(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        placement: FlyoutPlacement,
    ) -> NodeId {
        self.insert_data(parent, key, NodeData::Flyout(placement))
    }

    pub fn insert_content_dialog(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        open: bool,
    ) -> NodeId {
        self.insert_data(parent, key, NodeData::ContentDialog(open))
    }

    pub fn insert_menu(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        kind: OwnedMenuKind,
        menu: Menu,
    ) -> NodeId {
        self.insert_data(
            parent,
            key,
            NodeData::Menu {
                kind,
                state: OwnedState {
                    callback: menu.on_click,
                    revision: 1,
                    content: menu.items,
                },
            },
        )
    }

    pub fn insert_command_bar_flyout(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        flyout: CommandBarFlyout,
    ) -> NodeId {
        self.insert_data(
            parent,
            key,
            NodeData::CommandBarFlyout(OwnedState {
                callback: flyout.on_click,
                revision: 1,
                content: (flyout.primary, flyout.secondary),
            }),
        )
    }

    pub fn insert_tree_nodes(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        nodes: Rc<Vec<TreeNode>>,
    ) -> NodeId {
        self.insert_data(parent, key, NodeData::TreeNodes(nodes))
    }

    pub fn tree_nodes(&self, id: NodeId) -> &Rc<Vec<TreeNode>> {
        match &self.node(id).data {
            NodeData::TreeNodes(nodes) => nodes,
            _ => panic!("node is not a tree view"),
        }
    }

    pub fn update_tree_nodes(&mut self, id: NodeId, nodes: Rc<Vec<TreeNode>>) {
        match &mut self.node_mut(id).data {
            NodeData::TreeNodes(current) => {
                *current = nodes;
            }
            _ => panic!("node is not a tree view"),
        }
    }

    pub fn owned_revision(&self, id: NodeId) -> u32 {
        match &self.node(id).data {
            NodeData::Menu { state, .. } => state.revision,
            NodeData::CommandBarFlyout(state) => state.revision,
            _ => panic!("node does not own commands"),
        }
    }

    pub fn owned_callback(&self, id: NodeId) -> &Callback<String> {
        match &self.node(id).data {
            NodeData::Menu { state, .. } => &state.callback,
            NodeData::CommandBarFlyout(state) => &state.callback,
            _ => panic!("node does not own commands"),
        }
    }

    pub fn update_menu(&mut self, id: NodeId, menu: Menu) -> u32 {
        match &mut self.node_mut(id).data {
            NodeData::Menu { state, .. } => state.replace(menu.on_click, menu.items),
            _ => panic!("node is not a menu"),
        }
    }

    pub fn update_command_bar_flyout(&mut self, id: NodeId, flyout: CommandBarFlyout) -> u32 {
        match &mut self.node_mut(id).data {
            NodeData::CommandBarFlyout(state) => {
                state.replace(flyout.on_click, (flyout.primary, flyout.secondary))
            }
            _ => panic!("node is not a command bar flyout"),
        }
    }

    pub fn set_kind(&mut self, id: NodeId, kind: NodeKind) {
        let data = &mut self.node_mut(id).data;
        match (data, kind) {
            (NodeData::Tooltip(current), NodeKind::Tooltip(placement)) => {
                *current = placement;
            }
            (NodeData::Flyout(current), NodeKind::Flyout(placement)) => {
                *current = placement;
            }
            (NodeData::Menu { kind: current, .. }, NodeKind::Menu(kind)) => {
                *current = kind;
            }
            (NodeData::ContentDialog(current), NodeKind::ContentDialog(open)) => {
                *current = open;
            }
            (data, requested) if data.kind() == requested => {}
            (data, requested) => {
                let current = data.kind();
                panic!("cannot change node kind from {current:?} to {requested:?}")
            }
        }
    }

    pub fn owned_menu(&self, id: NodeId) -> &[MenuItem] {
        match &self.node(id).data {
            NodeData::Menu { state, .. } => &state.content,
            _ => panic!("node is not a menu"),
        }
    }

    pub fn owned_commands(&self, id: NodeId) -> &(Vec<CommandBarCommand>, Vec<CommandBarCommand>) {
        match &self.node(id).data {
            NodeData::CommandBarFlyout(state) => &state.content,
            _ => panic!("node is not a command bar flyout"),
        }
    }

    pub fn set_content_dialog_open(&mut self, id: NodeId, open: bool) {
        let node = self.node_mut(id);
        match &mut node.data {
            NodeData::ContentDialog(current) => {
                *current = open;
            }
            _ => panic!("node is not a content dialog"),
        }
    }

    pub fn provision(&self, id: NodeId) -> &ContextProvision {
        assert!(
            matches!(&self.node(id).data, NodeData::Provider),
            "node is not a context provider"
        );
        self.providers.get(id).unwrap()
    }

    pub fn set_provision(&mut self, id: NodeId, provision: ContextProvision) {
        assert!(
            matches!(&self.node(id).data, NodeData::Provider),
            "node is not a context provider"
        );
        self.providers.insert(id, provision);
    }

    pub fn set_tooltip_placement(&mut self, id: NodeId, placement: TooltipPlacement) {
        let node = self.node_mut(id);
        match &mut node.data {
            NodeData::Tooltip(current) => {
                *current = placement;
            }
            _ => panic!("node is not a tooltip"),
        }
    }

    pub fn tooltip_attachment(&self, id: NodeId) -> Option<(NodeId, NodeId)> {
        let node = self.node(id);
        assert!(
            matches!(&node.data, NodeData::Tooltip(_)),
            "node is not a tooltip"
        );
        self.owned_attachments.get(&id).copied()
    }

    pub fn set_tooltip_attachment(&mut self, id: NodeId, attachment: Option<(NodeId, NodeId)>) {
        let node = self.node(id);
        assert!(
            matches!(&node.data, NodeData::Tooltip(_)),
            "node is not a tooltip"
        );
        if let Some(attachment) = attachment {
            Rc::make_mut(&mut self.owned_attachments).insert(id, attachment);
        } else {
            Rc::make_mut(&mut self.owned_attachments).remove(&id);
        }
    }

    pub fn set_flyout_placement(&mut self, id: NodeId, placement: FlyoutPlacement) {
        let node = self.node_mut(id);
        match &mut node.data {
            NodeData::Flyout(current) => {
                *current = placement;
            }
            _ => panic!("node is not a flyout"),
        }
    }

    pub fn flyout_attachment(&self, id: NodeId) -> Option<(NodeId, NodeId)> {
        let node = self.node(id);
        assert!(
            matches!(&node.data, NodeData::Flyout(_)),
            "node is not a flyout"
        );
        self.owned_attachments.get(&id).copied()
    }

    pub fn set_flyout_attachment(&mut self, id: NodeId, attachment: Option<(NodeId, NodeId)>) {
        let node = self.node(id);
        assert!(
            matches!(&node.data, NodeData::Flyout(_)),
            "node is not a flyout"
        );
        if let Some(attachment) = attachment {
            Rc::make_mut(&mut self.owned_attachments).insert(id, attachment);
        } else {
            Rc::make_mut(&mut self.owned_attachments).remove(&id);
        }
    }

    pub(crate) fn context_snapshot(&self, id: NodeId) -> ContextSnapshot {
        let mut snapshot = ContextSnapshot::default();
        let mut current = self.parent(id);
        while let Some(node) = current {
            if self.kind(node) == NodeKind::Provider {
                snapshot.insert(node, self.provision(node));
            }
            current = self.parent(node);
        }
        snapshot
    }

    pub fn component_scope(&self, id: NodeId) -> ScopeId {
        match &self.node(id).data {
            NodeData::Component(component) => component.scope,
            _ => panic!("node is not a component"),
        }
    }

    pub fn component_type(&self, id: NodeId) -> TypeId {
        match &self.node(id).data {
            NodeData::Component(component) => component.component_type,
            _ => panic!("node is not a component"),
        }
    }

    pub fn component_node(&self, scope: ScopeId) -> Option<NodeId> {
        self.components.get(&scope).copied()
    }

    pub(crate) fn try_kind(&self, id: NodeId) -> Option<NodeKind> {
        Some(self.arena.get(id)?.data.kind())
    }

    pub(crate) fn try_native(&self, id: NodeId) -> Option<&NativeState> {
        match &self.arena.get(id)?.data {
            NodeData::Native(native) => Some(&native.state),
            NodeData::Virtual(VirtualData::Items { native, .. }) => Some(native),
            _ => None,
        }
    }

    pub(crate) fn try_native_mut(&mut self, id: NodeId) -> Option<&mut NativeState> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Native(native) => Some(&mut native.state),
            NodeData::Virtual(VirtualData::Items { native, .. }) => Some(native),
            _ => None,
        }
    }

    pub fn native(&self, id: NodeId) -> &NativeState {
        match &self.node(id).data {
            NodeData::Native(native) => &native.state,
            NodeData::Virtual(VirtualData::Items { native, .. }) => native,
            _ => panic!("node is not native"),
        }
    }

    pub fn native_mut(&mut self, id: NodeId) -> &mut NativeState {
        match &mut self.node_mut(id).data {
            NodeData::Native(native) => &mut native.state,
            NodeData::Virtual(VirtualData::Items { native, .. }) => native,
            _ => panic!("node is not native"),
        }
    }

    pub(crate) fn exit_transition(&self, id: NodeId) -> Option<ExitTransition> {
        self.exit_transitions.get(&id).copied()
    }

    pub(crate) fn set_exit_transition(&mut self, id: NodeId, transition: Option<ExitTransition>) {
        self.native(id);
        let transitions = Rc::make_mut(&mut self.exit_transitions);
        match transition {
            Some(transition) => {
                transitions.insert(id, transition);
            }
            None => {
                transitions.remove(&id);
            }
        }
    }

    #[cfg(test)]
    pub fn insert_virtual(
        &mut self,
        identity: WindowToken,
        parent: Option<NodeId>,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<NodeId, DuplicateKeyError<Key>> {
        let id = self.arena.next_id();
        let model = VirtualModel::new(identity, id, keys)?;
        let inserted = self.insert_data(
            parent,
            None,
            NodeData::Virtual(VirtualData::Bare {
                model,
                realized: HashMap::default(),
            }),
        );
        debug_assert_eq!(inserted, id);
        Ok(inserted)
    }

    pub fn insert_virtual_items(
        &mut self,
        identity: WindowToken,
        parent: Option<NodeId>,
        key: Option<Key>,
        desired: MountedProps,
        items: VirtualItems,
    ) -> Result<NodeId, DuplicateKeyError<Key>> {
        let keys = (0..items.len()).map(|index| items.key(index).unwrap());
        let id = self.arena.next_id();
        let model = VirtualModel::new(identity, id, keys)?;
        let inserted = self.insert_data(
            parent,
            key,
            NodeData::Virtual(VirtualData::Items {
                model,
                realized: HashMap::default(),
                native: NativeState::new(desired),
                items: Rc::new(items),
            }),
        );
        debug_assert_eq!(inserted, id);
        Ok(inserted)
    }

    pub fn virtual_items(&self, id: NodeId) -> &VirtualItems {
        match &self.node(id).data {
            NodeData::Virtual(VirtualData::Items { items, .. }) => items.as_ref(),
            _ => panic!("node is not a virtual collection"),
        }
    }

    pub fn virtual_view_at(&self, id: NodeId, index: usize) -> View {
        self.virtual_items(id).view(index).unwrap()
    }

    pub fn realized(&self, id: NodeId, container: RealizedContainer) -> Option<RealizedRow> {
        let NodeData::Virtual(virtual_data) = &self.node(id).data else {
            panic!("node is not a virtual collection");
        };
        virtual_data.realized().get(&container).copied()
    }

    pub fn realized_rows(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (RealizedContainer, RealizedRow)> + '_ {
        let NodeData::Virtual(virtual_data) = &self.node(id).data else {
            panic!("node is not a virtual collection");
        };
        virtual_data
            .realized()
            .iter()
            .map(|(container, row)| (*container, *row))
    }

    pub fn realized_container(&self, id: NodeId, native_root: NodeId) -> Option<RealizedContainer> {
        let NodeData::Virtual(virtual_data) = &self.node(id).data else {
            panic!("node is not a virtual collection");
        };
        virtual_data.realized().iter().find_map(|(container, row)| {
            (row.native_root == Some(native_root)).then_some(*container)
        })
    }

    pub fn realized_container_for_logical(
        &self,
        id: NodeId,
        logical_root: NodeId,
    ) -> Option<RealizedContainer> {
        let NodeData::Virtual(virtual_data) = &self.node(id).data else {
            panic!("node is not a virtual collection");
        };
        virtual_data
            .realized()
            .iter()
            .find_map(|(container, row)| (row.logical_root == logical_root).then_some(*container))
    }

    pub fn set_realized(
        &mut self,
        id: NodeId,
        container: RealizedContainer,
        index: usize,
        logical_root: NodeId,
        native_root: Option<NodeId>,
    ) {
        self.node(logical_root);
        if let Some(native_root) = native_root {
            self.node(native_root);
        }
        let NodeData::Virtual(virtual_data) = &mut self.node_mut(id).data else {
            panic!("node is not a virtual collection");
        };
        let realized = virtual_data.realized_mut();
        if realized.contains_key(&container)
            || realized.values().any(|row| {
                row.logical_root == logical_root
                    || native_root.is_some() && row.native_root == native_root
            })
        {
            panic!("realized container already mapped: {container:?}");
        }
        realized.insert(
            container,
            RealizedRow {
                index,
                logical_root,
                native_root,
            },
        );
    }

    pub fn update_realized(
        &mut self,
        id: NodeId,
        container: RealizedContainer,
        logical_root: NodeId,
        native_root: Option<NodeId>,
    ) {
        self.node(logical_root);
        if let Some(native_root) = native_root {
            self.node(native_root);
        }
        let NodeData::Virtual(virtual_data) = &mut self.node_mut(id).data else {
            panic!("node is not a virtual collection");
        };
        let row = virtual_data.realized_mut().get_mut(&container).unwrap();
        *row = RealizedRow {
            index: row.index,
            logical_root,
            native_root,
        };
    }

    pub fn update_virtual_items(&mut self, id: NodeId, items: VirtualItems) {
        match &mut self.node_mut(id).data {
            NodeData::Virtual(VirtualData::Items { items: current, .. }) => {
                *current = Rc::new(items);
            }
            _ => panic!("node is not an item-backed virtual collection"),
        }
    }

    pub fn virtual_model(&self, id: NodeId) -> &VirtualModel {
        match &self.node(id).data {
            NodeData::Virtual(virtual_data) => virtual_data.model(),
            _ => panic!("node is not a virtual collection"),
        }
    }

    pub fn virtual_model_mut(&mut self, id: NodeId) -> &mut VirtualModel {
        match &mut self.node_mut(id).data {
            NodeData::Virtual(virtual_data) => virtual_data.model_mut(),
            _ => panic!("node is not a virtual collection"),
        }
    }

    pub(crate) fn try_virtual_model(&self, id: NodeId) -> Option<&VirtualModel> {
        match &self.arena.get(id)?.data {
            NodeData::Virtual(virtual_data) => Some(virtual_data.model()),
            _ => None,
        }
    }

    pub(crate) fn try_virtual_model_mut(&mut self, id: NodeId) -> Option<&mut VirtualModel> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Virtual(virtual_data) => Some(virtual_data.model_mut()),
            _ => None,
        }
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.node(id).parent
    }

    pub fn is_descendant_of(&self, id: NodeId, ancestor: NodeId) -> bool {
        let mut current = Some(id);
        while let Some(node) = current {
            if node == ancestor {
                return true;
            }
            current = self.parent(node);
        }
        false
    }

    pub fn kind(&self, id: NodeId) -> NodeKind {
        self.node(id).data.kind()
    }

    pub(crate) fn window_title(&self) -> Option<WindowTitleState> {
        let mut titles = self
            .window_declarations
            .iter()
            .filter_map(|(owner, declaration)| Some((*owner, declaration.title.as_ref()?)));
        let (owner, title) = titles.next()?;
        titles.next().is_none().then(|| WindowTitleState {
            owner,
            title: Rc::clone(title),
        })
    }

    pub(crate) fn validate_window_title(&self) -> Result<Option<WindowTitleState>, ()> {
        let count = self
            .window_declarations
            .values()
            .filter(|declaration| declaration.title.is_some())
            .count();
        (count <= 1).then(|| self.window_title()).ok_or(())
    }

    pub(crate) fn set_window_title(&mut self, owner: ScopeId, title: Option<Rc<str>>) {
        let declarations = Rc::make_mut(&mut self.window_declarations);
        let declaration = declarations.entry(owner).or_default();
        declaration.title = title;
    }

    pub(crate) fn window_title_bar(&self) -> Option<WindowTitleBarState> {
        let mut title_bars = self.window_title_bars.iter();
        let (&title_bar, &height) = title_bars.next()?;
        title_bars
            .next()
            .is_none()
            .then_some(WindowTitleBarState { title_bar, height })
    }

    pub(crate) fn validate_window_title_bar(&self) -> Result<Option<WindowTitleBarState>, ()> {
        (self.window_title_bars.len() <= 1)
            .then(|| self.window_title_bar())
            .ok_or(())
    }

    pub(crate) fn set_window_title_bar(
        &mut self,
        title_bar: NodeId,
        height: Option<WindowTitleBarHeight>,
    ) {
        self.native(title_bar);
        let title_bars = Rc::make_mut(&mut self.window_title_bars);
        if let Some(height) = height {
            title_bars.insert(title_bar, height);
        } else {
            title_bars.remove(&title_bar);
        }
    }

    pub(crate) fn node_window_title_bar(&self, title_bar: NodeId) -> Option<WindowTitleBarHeight> {
        self.window_title_bars.get(&title_bar).copied()
    }

    pub(crate) fn window_visuals(&self) -> Option<WindowVisualsState> {
        let mut declarations = self
            .window_declarations
            .iter()
            .filter_map(|(owner, declaration)| Some((*owner, declaration.visuals?)));
        let (owner, visuals) = declarations.next()?;
        declarations
            .next()
            .is_none()
            .then_some(WindowVisualsState { owner, visuals })
    }

    pub(crate) fn validate_window_visuals(&self) -> Result<Option<WindowVisualsState>, ()> {
        let count = self
            .window_declarations
            .values()
            .filter(|declaration| declaration.visuals.is_some())
            .count();
        (count <= 1).then(|| self.window_visuals()).ok_or(())
    }

    pub(crate) fn set_window_visuals(&mut self, owner: ScopeId, visuals: Option<WindowVisuals>) {
        let declarations = Rc::make_mut(&mut self.window_declarations);
        let declaration = declarations.entry(owner).or_default();
        declaration.visuals = visuals;
    }

    pub(crate) fn color_scheme_observation(
        &self,
    ) -> Option<(HostObservationId, Callback<ColorScheme>)> {
        let mut declarations =
            self.window_declarations
                .iter()
                .filter_map(|(owner, declaration)| {
                    let (callback, revision) = declaration.color_scheme.get()?;
                    Some((
                        HostObservationId {
                            owner: *owner,
                            revision,
                        },
                        callback.clone(),
                    ))
                });
        let observation = declarations.next()?;
        declarations.next().is_none().then_some(observation)
    }

    pub(crate) fn validate_color_scheme_observation(
        &self,
    ) -> Result<Option<(HostObservationId, Callback<ColorScheme>)>, ()> {
        let count = self
            .window_declarations
            .values()
            .filter(|declaration| declaration.color_scheme.get().is_some())
            .count();
        (count <= 1)
            .then(|| self.color_scheme_observation())
            .ok_or(())
    }

    pub(crate) fn set_color_scheme_observation(
        &mut self,
        owner: ScopeId,
        callback: Option<Callback<ColorScheme>>,
    ) {
        let declarations = Rc::make_mut(&mut self.window_declarations);
        let declaration = declarations.entry(owner).or_default();
        declaration.color_scheme.set(callback);
    }

    pub(crate) fn window_size_observation(
        &self,
    ) -> Option<(HostObservationId, Callback<WindowSize>)> {
        let mut declarations =
            self.window_declarations
                .iter()
                .filter_map(|(owner, declaration)| {
                    let (callback, revision) = declaration.window_size.get()?;
                    Some((
                        HostObservationId {
                            owner: *owner,
                            revision,
                        },
                        callback.clone(),
                    ))
                });
        let observation = declarations.next()?;
        declarations.next().is_none().then_some(observation)
    }

    pub(crate) fn validate_window_size_observation(
        &self,
    ) -> Result<Option<(HostObservationId, Callback<WindowSize>)>, ()> {
        let count = self
            .window_declarations
            .values()
            .filter(|declaration| declaration.window_size.get().is_some())
            .count();
        (count <= 1)
            .then(|| self.window_size_observation())
            .ok_or(())
    }

    pub(crate) fn set_window_size_observation(
        &mut self,
        owner: ScopeId,
        callback: Option<Callback<WindowSize>>,
    ) {
        let declarations = Rc::make_mut(&mut self.window_declarations);
        let declaration = declarations.entry(owner).or_default();
        declaration.window_size.set(callback);
    }

    pub(crate) fn remove_window_declarations(&mut self, owner: ScopeId) {
        Rc::make_mut(&mut self.window_declarations).remove(&owner);
    }

    pub fn children(&self, id: NodeId) -> &[NodeId] {
        &self.node(id).children
    }

    pub fn key(&self, id: NodeId) -> Option<&Key> {
        self.node(id).key.as_ref()
    }

    pub fn set_children(&mut self, id: NodeId, children: Vec<NodeId>) {
        if self.node(id).children != children {
            self.node_mut(id).children = children;
        }
    }

    pub fn reparent(&mut self, id: NodeId, parent: NodeId, key: Option<Key>) {
        self.node(parent);
        let previous = self.node(id).parent;
        if let Some(previous) = previous {
            self.node_mut(previous)
                .children
                .retain(|child| *child != id);
        } else {
            self.root = None;
        }
        let node = self.node_mut(id);
        node.parent = Some(parent);
        node.key = key;
        self.node_mut(parent).children.push(id);
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn depth(&self, id: NodeId) -> usize {
        let mut depth = 0;
        let mut current = id;
        while let Some(parent) = self.node(current).parent {
            depth += 1;
            current = parent;
        }
        depth
    }

    pub fn retire_subtree(&mut self, id: NodeId) -> Vec<(NodeId, NodeKind)> {
        let mut order = Vec::new();
        self.collect_postorder(id, &mut order);

        let parent = self.node(id).parent;
        if let Some(parent) = parent {
            let parent = self.node_mut(parent);
            parent.children.retain(|child| *child != id);
            if let NodeData::Virtual(virtual_data) = &mut parent.data {
                virtual_data
                    .realized_mut()
                    .retain(|_, row| row.logical_root != id);
            }
        } else {
            self.root = None;
        }

        let mut retired = Vec::with_capacity(order.len());
        for id in order {
            let Some(node) = self.arena.remove(id) else {
                panic!("stale node");
            };
            let kind = node.data.kind();
            Rc::make_mut(&mut self.exit_transitions).remove(&id);
            Rc::make_mut(&mut self.window_title_bars).remove(&id);
            if let NodeData::Component(component) = &node.data {
                Rc::make_mut(&mut self.components).remove(&component.scope);
            }
            if matches!(&node.data, NodeData::Provider) {
                self.providers.remove(id);
            }
            if matches!(&node.data, NodeData::Tooltip(_) | NodeData::Flyout(_)) {
                Rc::make_mut(&mut self.owned_attachments).remove(&id);
            }
            retired.push((id, kind));
        }
        retired
    }

    pub fn subtree_postorder(&self, id: NodeId) -> Vec<NodeId> {
        let mut order = Vec::new();
        self.collect_postorder(id, &mut order);
        order
    }

    fn collect_postorder(&self, id: NodeId, order: &mut Vec<NodeId>) {
        for child in self.node(id).children.iter().copied() {
            self.collect_postorder(child, order);
        }
        order.push(id);
    }
}

#[cfg(test)]
#[path = "engine_tests.rs"]
mod tests;
