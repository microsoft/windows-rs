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
    fn structural(kind: NodeKind) -> Result<Self, TreeError> {
        match kind {
            NodeKind::Application => Ok(Self::Application),
            NodeKind::Window => Ok(Self::Window),
            NodeKind::Fragment => Ok(Self::Fragment),
            NodeKind::Slot => Ok(Self::Slot),
            NodeKind::NamedSlot(slot) => Ok(Self::NamedSlot(slot)),
            NodeKind::Tooltip(placement) => Ok(Self::Tooltip(placement)),
            NodeKind::Flyout(placement) => Ok(Self::Flyout(placement)),
            NodeKind::ContentDialog(open) => Ok(Self::ContentDialog(open)),
            NodeKind::Component
            | NodeKind::Provider
            | NodeKind::Menu(_)
            | NodeKind::CommandBarFlyout
            | NodeKind::TreeNodes
            | NodeKind::Native(_)
            | NodeKind::VirtualCollection => Err(TreeError::IncompleteNode(kind)),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeError {
    Arena(ArenaError),
    IncompleteNode(NodeKind),
    KindMismatch {
        current: NodeKind,
        requested: NodeKind,
    },
    NotComponent,
    NotNative,
    NotVirtual,
    RealizedConflict(RealizedContainer),
    RootAlreadyExists,
    Virtual(VirtualModelError),
}

impl From<ArenaError> for TreeError {
    fn from(value: ArenaError) -> Self {
        Self::Arena(value)
    }
}

impl From<VirtualModelError> for TreeError {
    fn from(value: VirtualModelError) -> Self {
        Self::Virtual(value)
    }
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

    pub fn insert(&mut self, parent: Option<NodeId>, kind: NodeKind) -> Result<NodeId, TreeError> {
        self.insert_data(parent, None, NodeData::structural(kind)?)
    }

    fn insert_data(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        data: NodeData,
    ) -> Result<NodeId, TreeError> {
        if let Some(parent) = parent {
            self.arena.get(parent)?;
        } else if self.root.is_some() {
            return Err(TreeError::RootAlreadyExists);
        }

        let id = self.arena.insert(Node {
            parent,
            children: Vec::new(),
            key,
            data,
        })?;

        if let Some(parent) = parent {
            self.arena.get_mut(parent)?.children.push(id);
        } else {
            self.root = Some(id);
        }
        Ok(id)
    }

    pub fn insert_native(
        &mut self,
        parent: Option<NodeId>,
        kind: MountedKind,
        key: Option<Key>,
        desired: MountedProps,
        window_title_bar: Option<WindowTitleBarHeight>,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert_data(
            parent,
            key,
            NodeData::Native(NativeData {
                kind,
                state: NativeState::new(desired),
            }),
        )?;
        if let Some(height) = window_title_bar {
            Rc::make_mut(&mut self.window_title_bars).insert(id, height);
        }
        Ok(id)
    }

    pub fn insert_component(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        scope: ScopeId,
        component_type: TypeId,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert_data(
            parent,
            key,
            NodeData::Component(ComponentData {
                component_type,
                scope,
            }),
        )?;
        Rc::make_mut(&mut self.components).insert(scope, id);
        Ok(id)
    }

    pub fn insert_fragment(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
    ) -> Result<NodeId, TreeError> {
        self.insert_data(parent, key, NodeData::Fragment)
    }

    pub fn insert_provider(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        provision: ContextProvision,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert_data(parent, key, NodeData::Provider)?;
        self.providers.insert(id, provision);
        Ok(id)
    }

    pub fn insert_tooltip(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        placement: TooltipPlacement,
    ) -> Result<NodeId, TreeError> {
        self.insert_data(parent, key, NodeData::Tooltip(placement))
    }

    pub fn insert_flyout(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        placement: FlyoutPlacement,
    ) -> Result<NodeId, TreeError> {
        self.insert_data(parent, key, NodeData::Flyout(placement))
    }

    pub fn insert_content_dialog(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        open: bool,
    ) -> Result<NodeId, TreeError> {
        self.insert_data(parent, key, NodeData::ContentDialog(open))
    }

    pub fn insert_menu(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        kind: OwnedMenuKind,
        menu: Menu,
    ) -> Result<NodeId, TreeError> {
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
    ) -> Result<NodeId, TreeError> {
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
    ) -> Result<NodeId, TreeError> {
        self.insert_data(parent, key, NodeData::TreeNodes(nodes))
    }

    pub fn tree_nodes(&self, id: NodeId) -> Result<&Rc<Vec<TreeNode>>, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::TreeNodes(nodes) => Ok(nodes),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn update_tree_nodes(
        &mut self,
        id: NodeId,
        nodes: Rc<Vec<TreeNode>>,
    ) -> Result<(), TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::TreeNodes(current) => {
                *current = nodes;
                Ok(())
            }
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn owned_revision(&self, id: NodeId) -> Result<u32, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Menu { state, .. } => Ok(state.revision),
            NodeData::CommandBarFlyout(state) => Ok(state.revision),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn owned_callback(&self, id: NodeId) -> Result<&Callback<String>, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Menu { state, .. } => Ok(&state.callback),
            NodeData::CommandBarFlyout(state) => Ok(&state.callback),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn update_menu(&mut self, id: NodeId, menu: Menu) -> Result<u32, TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Menu { state, .. } => Ok(state.replace(menu.on_click, menu.items)),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn update_command_bar_flyout(
        &mut self,
        id: NodeId,
        flyout: CommandBarFlyout,
    ) -> Result<u32, TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::CommandBarFlyout(state) => {
                Ok(state.replace(flyout.on_click, (flyout.primary, flyout.secondary)))
            }
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn set_kind(&mut self, id: NodeId, kind: NodeKind) -> Result<(), TreeError> {
        let data = &mut self.arena.get_mut(id)?.data;
        match (data, kind) {
            (NodeData::Tooltip(current), NodeKind::Tooltip(placement)) => {
                *current = placement;
                Ok(())
            }
            (NodeData::Flyout(current), NodeKind::Flyout(placement)) => {
                *current = placement;
                Ok(())
            }
            (NodeData::Menu { kind: current, .. }, NodeKind::Menu(kind)) => {
                *current = kind;
                Ok(())
            }
            (NodeData::ContentDialog(current), NodeKind::ContentDialog(open)) => {
                *current = open;
                Ok(())
            }
            (data, requested) if data.kind() == requested => Ok(()),
            (data, requested) => {
                let current = data.kind();
                Err(TreeError::KindMismatch { current, requested })
            }
        }
    }

    pub fn owned_menu(&self, id: NodeId) -> Result<&[MenuItem], TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Menu { state, .. } => Ok(&state.content),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn owned_commands(
        &self,
        id: NodeId,
    ) -> Result<&(Vec<CommandBarCommand>, Vec<CommandBarCommand>), TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::CommandBarFlyout(state) => Ok(&state.content),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn set_content_dialog_open(&mut self, id: NodeId, open: bool) -> Result<(), TreeError> {
        let node = self.arena.get_mut(id)?;
        match &mut node.data {
            NodeData::ContentDialog(current) => {
                *current = open;
                Ok(())
            }
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn provision(&self, id: NodeId) -> Result<&ContextProvision, TreeError> {
        if !matches!(&self.arena.get(id)?.data, NodeData::Provider) {
            return Err(TreeError::NotComponent);
        }
        self.providers.get(id).ok_or(TreeError::NotComponent)
    }

    pub fn set_provision(
        &mut self,
        id: NodeId,
        provision: ContextProvision,
    ) -> Result<(), TreeError> {
        if !matches!(&self.arena.get(id)?.data, NodeData::Provider) {
            return Err(TreeError::NotComponent);
        }
        self.providers.insert(id, provision);
        Ok(())
    }

    pub fn set_tooltip_placement(
        &mut self,
        id: NodeId,
        placement: TooltipPlacement,
    ) -> Result<(), TreeError> {
        let node = self.arena.get_mut(id)?;
        match &mut node.data {
            NodeData::Tooltip(current) => {
                *current = placement;
                Ok(())
            }
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn tooltip_attachment(&self, id: NodeId) -> Result<Option<(NodeId, NodeId)>, TreeError> {
        let node = self.arena.get(id)?;
        if !matches!(&node.data, NodeData::Tooltip(_)) {
            return Err(TreeError::NotComponent);
        }
        Ok(self.owned_attachments.get(&id).copied())
    }

    pub fn set_tooltip_attachment(
        &mut self,
        id: NodeId,
        attachment: Option<(NodeId, NodeId)>,
    ) -> Result<(), TreeError> {
        let node = self.arena.get(id)?;
        if !matches!(&node.data, NodeData::Tooltip(_)) {
            return Err(TreeError::NotComponent);
        }
        if let Some(attachment) = attachment {
            Rc::make_mut(&mut self.owned_attachments).insert(id, attachment);
        } else {
            Rc::make_mut(&mut self.owned_attachments).remove(&id);
        }
        Ok(())
    }

    pub fn set_flyout_placement(
        &mut self,
        id: NodeId,
        placement: FlyoutPlacement,
    ) -> Result<(), TreeError> {
        let node = self.arena.get_mut(id)?;
        match &mut node.data {
            NodeData::Flyout(current) => {
                *current = placement;
                Ok(())
            }
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn flyout_attachment(&self, id: NodeId) -> Result<Option<(NodeId, NodeId)>, TreeError> {
        let node = self.arena.get(id)?;
        if !matches!(&node.data, NodeData::Flyout(_)) {
            return Err(TreeError::NotComponent);
        }
        Ok(self.owned_attachments.get(&id).copied())
    }

    pub fn set_flyout_attachment(
        &mut self,
        id: NodeId,
        attachment: Option<(NodeId, NodeId)>,
    ) -> Result<(), TreeError> {
        let node = self.arena.get(id)?;
        if !matches!(&node.data, NodeData::Flyout(_)) {
            return Err(TreeError::NotComponent);
        }
        if let Some(attachment) = attachment {
            Rc::make_mut(&mut self.owned_attachments).insert(id, attachment);
        } else {
            Rc::make_mut(&mut self.owned_attachments).remove(&id);
        }
        Ok(())
    }

    pub(crate) fn context_snapshot(&self, id: NodeId) -> Result<ContextSnapshot, TreeError> {
        let mut snapshot = ContextSnapshot::default();
        let mut current = self.parent(id)?;
        while let Some(node) = current {
            if self.kind(node)? == NodeKind::Provider {
                snapshot.insert(node, self.provision(node)?);
            }
            current = self.parent(node)?;
        }
        Ok(snapshot)
    }

    pub fn component_scope(&self, id: NodeId) -> Result<ScopeId, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Component(component) => Ok(component.scope),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn component_type(&self, id: NodeId) -> Result<TypeId, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Component(component) => Ok(component.component_type),
            _ => Err(TreeError::NotComponent),
        }
    }

    pub fn component_node(&self, scope: ScopeId) -> Result<Option<NodeId>, TreeError> {
        Ok(self.components.get(&scope).copied())
    }

    pub fn native(&self, id: NodeId) -> Result<&NativeState, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Native(native) => Ok(&native.state),
            NodeData::Virtual(VirtualData::Items { native, .. }) => Ok(native),
            _ => Err(TreeError::NotNative),
        }
    }

    pub fn native_mut(&mut self, id: NodeId) -> Result<&mut NativeState, TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Native(native) => Ok(&mut native.state),
            NodeData::Virtual(VirtualData::Items { native, .. }) => Ok(native),
            _ => Err(TreeError::NotNative),
        }
    }

    pub(crate) fn exit_transition(&self, id: NodeId) -> Option<ExitTransition> {
        self.exit_transitions.get(&id).copied()
    }

    pub(crate) fn set_exit_transition(
        &mut self,
        id: NodeId,
        transition: Option<ExitTransition>,
    ) -> Result<(), TreeError> {
        self.native(id)?;
        let transitions = Rc::make_mut(&mut self.exit_transitions);
        match transition {
            Some(transition) => {
                transitions.insert(id, transition);
            }
            None => {
                transitions.remove(&id);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn insert_virtual(
        &mut self,
        identity: WindowToken,
        parent: Option<NodeId>,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<NodeId, TreeError> {
        let id = self.arena.next_id()?;
        let model = VirtualModel::new(identity, id, keys)?;
        let inserted = self.insert_data(
            parent,
            None,
            NodeData::Virtual(VirtualData::Bare {
                model,
                realized: HashMap::default(),
            }),
        )?;
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
    ) -> Result<NodeId, TreeError> {
        let keys = (0..items.len()).map(|index| items.key(index).unwrap());
        let id = self.arena.next_id()?;
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
        )?;
        debug_assert_eq!(inserted, id);
        Ok(inserted)
    }

    pub fn virtual_items(&self, id: NodeId) -> Result<&VirtualItems, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Virtual(VirtualData::Items { items, .. }) => Ok(items.as_ref()),
            _ => Err(TreeError::NotVirtual),
        }
    }

    pub fn virtual_view_at(&self, id: NodeId, index: usize) -> Result<View, TreeError> {
        self.virtual_items(id)?
            .view(index)
            .ok_or_else(|| VirtualModelError::MissingIndex(index).into())
    }

    pub fn realized(
        &self,
        id: NodeId,
        container: RealizedContainer,
    ) -> Result<Option<RealizedRow>, TreeError> {
        let NodeData::Virtual(virtual_data) = &self.arena.get(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        Ok(virtual_data.realized().get(&container).copied())
    }

    pub fn realized_rows(
        &self,
        id: NodeId,
    ) -> Result<impl Iterator<Item = (RealizedContainer, RealizedRow)> + '_, TreeError> {
        let NodeData::Virtual(virtual_data) = &self.arena.get(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        Ok(virtual_data
            .realized()
            .iter()
            .map(|(container, row)| (*container, *row)))
    }

    pub fn realized_container(
        &self,
        id: NodeId,
        native_root: NodeId,
    ) -> Result<Option<RealizedContainer>, TreeError> {
        let NodeData::Virtual(virtual_data) = &self.arena.get(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        Ok(virtual_data.realized().iter().find_map(|(container, row)| {
            (row.native_root == Some(native_root)).then_some(*container)
        }))
    }

    pub fn realized_container_for_logical(
        &self,
        id: NodeId,
        logical_root: NodeId,
    ) -> Result<Option<RealizedContainer>, TreeError> {
        let NodeData::Virtual(virtual_data) = &self.arena.get(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        Ok(virtual_data
            .realized()
            .iter()
            .find_map(|(container, row)| (row.logical_root == logical_root).then_some(*container)))
    }

    pub fn set_realized(
        &mut self,
        id: NodeId,
        container: RealizedContainer,
        index: usize,
        logical_root: NodeId,
        native_root: Option<NodeId>,
    ) -> Result<(), TreeError> {
        self.arena.get(logical_root)?;
        if let Some(native_root) = native_root {
            self.arena.get(native_root)?;
        }
        let NodeData::Virtual(virtual_data) = &mut self.arena.get_mut(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        let realized = virtual_data.realized_mut();
        if realized.contains_key(&container)
            || realized.values().any(|row| {
                row.logical_root == logical_root
                    || native_root.is_some() && row.native_root == native_root
            })
        {
            return Err(TreeError::RealizedConflict(container));
        }
        realized.insert(
            container,
            RealizedRow {
                index,
                logical_root,
                native_root,
            },
        );
        Ok(())
    }

    pub fn update_realized(
        &mut self,
        id: NodeId,
        container: RealizedContainer,
        logical_root: NodeId,
        native_root: Option<NodeId>,
    ) -> Result<(), TreeError> {
        self.arena.get(logical_root)?;
        if let Some(native_root) = native_root {
            self.arena.get(native_root)?;
        }
        let NodeData::Virtual(virtual_data) = &mut self.arena.get_mut(id)?.data else {
            return Err(TreeError::NotVirtual);
        };
        let row = virtual_data
            .realized_mut()
            .get_mut(&container)
            .ok_or(TreeError::NotVirtual)?;
        *row = RealizedRow {
            index: row.index,
            logical_root,
            native_root,
        };
        Ok(())
    }

    pub fn update_virtual_items(
        &mut self,
        id: NodeId,
        items: VirtualItems,
    ) -> Result<(), TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Virtual(VirtualData::Items { items: current, .. }) => {
                *current = Rc::new(items);
                Ok(())
            }
            _ => Err(TreeError::NotVirtual),
        }
    }

    pub fn virtual_model(&self, id: NodeId) -> Result<&VirtualModel, TreeError> {
        match &self.arena.get(id)?.data {
            NodeData::Virtual(virtual_data) => Ok(virtual_data.model()),
            _ => Err(TreeError::NotVirtual),
        }
    }

    pub fn virtual_model_mut(&mut self, id: NodeId) -> Result<&mut VirtualModel, TreeError> {
        match &mut self.arena.get_mut(id)?.data {
            NodeData::Virtual(virtual_data) => Ok(virtual_data.model_mut()),
            _ => Err(TreeError::NotVirtual),
        }
    }

    pub fn parent(&self, id: NodeId) -> Result<Option<NodeId>, TreeError> {
        Ok(self.arena.get(id)?.parent)
    }

    pub fn is_descendant_of(&self, id: NodeId, ancestor: NodeId) -> Result<bool, TreeError> {
        let mut current = Some(id);
        while let Some(node) = current {
            if node == ancestor {
                return Ok(true);
            }
            current = self.parent(node)?;
        }
        Ok(false)
    }

    pub fn kind(&self, id: NodeId) -> Result<NodeKind, TreeError> {
        Ok(self.arena.get(id)?.data.kind())
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
    ) -> Result<(), TreeError> {
        self.native(title_bar)?;
        let title_bars = Rc::make_mut(&mut self.window_title_bars);
        if let Some(height) = height {
            title_bars.insert(title_bar, height);
        } else {
            title_bars.remove(&title_bar);
        }
        Ok(())
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

    pub fn children(&self, id: NodeId) -> Result<&[NodeId], TreeError> {
        Ok(&self.arena.get(id)?.children)
    }

    pub fn key(&self, id: NodeId) -> Result<Option<&Key>, TreeError> {
        Ok(self.arena.get(id)?.key.as_ref())
    }

    pub fn set_children(&mut self, id: NodeId, children: Vec<NodeId>) -> Result<(), TreeError> {
        if self.arena.get(id)?.children != children {
            self.arena.get_mut(id)?.children = children;
        }
        Ok(())
    }

    pub fn reparent(
        &mut self,
        id: NodeId,
        parent: NodeId,
        key: Option<Key>,
    ) -> Result<(), TreeError> {
        self.arena.get(parent)?;
        let previous = self.arena.get(id)?.parent;
        if let Some(previous) = previous {
            self.arena
                .get_mut(previous)?
                .children
                .retain(|child| *child != id);
        } else {
            self.root = None;
        }
        let node = self.arena.get_mut(id)?;
        node.parent = Some(parent);
        node.key = key;
        self.arena.get_mut(parent)?.children.push(id);
        Ok(())
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn depth(&self, id: NodeId) -> Result<usize, TreeError> {
        let mut depth = 0;
        let mut current = id;
        while let Some(parent) = self.arena.get(current)?.parent {
            depth += 1;
            current = parent;
        }
        Ok(depth)
    }

    pub fn retire_subtree(&mut self, id: NodeId) -> Result<Vec<(NodeId, NodeKind)>, TreeError> {
        let mut order = Vec::new();
        self.collect_postorder(id, &mut order)?;

        let parent = self.arena.get(id)?.parent;
        if let Some(parent) = parent {
            let parent = self.arena.get_mut(parent)?;
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
            let node = self.arena.remove(id)?;
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
        Ok(retired)
    }

    pub fn subtree_postorder(&self, id: NodeId) -> Result<Vec<NodeId>, TreeError> {
        let mut order = Vec::new();
        self.collect_postorder(id, &mut order)?;
        Ok(order)
    }

    fn collect_postorder(&self, id: NodeId, order: &mut Vec<NodeId>) -> Result<(), TreeError> {
        for child in self.arena.get(id)?.children.iter().copied() {
            self.collect_postorder(child, order)?;
        }
        order.push(id);
        Ok(())
    }
}

#[cfg(test)]
#[path = "engine_tests.rs"]
mod tests;
