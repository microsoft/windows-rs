use super::scope::ScopeId;
use super::*;
use crate::reference::NativeElementRef;
use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
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
    Native(MountedKind),
    VirtualCollection,
}

#[derive(Clone)]
struct Node {
    kind: NodeKind,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    component_type: Option<TypeId>,
    key: Option<Key>,
    native: Option<NativeState>,
    realized: HashMap<RealizedContainer, RealizedRow>,
    scope: Option<ScopeId>,
    virtual_items: Option<Rc<Vec<KeyedView>>>,
    virtual_model: Option<VirtualModel>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealizedRow {
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

#[derive(Clone, Copy)]
pub struct EventState {
    pub revision: u32,
    pub active: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeError {
    Arena(ArenaError),
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
    providers: ProviderStore,
    root: Option<NodeId>,
    window_title: Option<WindowTitleState>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WindowTitleState {
    pub(crate) owner: ScopeId,
    pub(crate) title: Rc<str>,
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
            chunks.push(Rc::new(HashMap::new()));
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
            components: Rc::new(HashMap::new()),
            providers: ProviderStore::default(),
            root: None,
            window_title: None,
        }
    }

    pub fn insert(&mut self, parent: Option<NodeId>, kind: NodeKind) -> Result<NodeId, TreeError> {
        if let Some(parent) = parent {
            self.arena.get(parent)?;
        } else if self.root.is_some() {
            return Err(TreeError::RootAlreadyExists);
        }

        let id = self.arena.insert(Node {
            kind,
            parent,
            children: Vec::new(),
            component_type: None,
            key: None,
            native: None,
            realized: HashMap::new(),
            scope: None,
            virtual_items: None,
            virtual_model: None,
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
    ) -> Result<NodeId, TreeError> {
        let id = self.insert(parent, NodeKind::Native(kind))?;
        let node = self.arena.get_mut(id)?;
        node.key = key;
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
        node.native = Some(NativeState {
            desired,
            reference: None,
            properties: BTreeMap::new(),
            events,
        });
        Ok(id)
    }

    pub fn insert_component(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        scope: ScopeId,
        component_type: TypeId,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert(parent, NodeKind::Component)?;
        let node = self.arena.get_mut(id)?;
        node.key = key;
        node.scope = Some(scope);
        node.component_type = Some(component_type);
        Rc::make_mut(&mut self.components).insert(scope, id);
        Ok(id)
    }

    pub fn insert_fragment(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert(parent, NodeKind::Fragment)?;
        self.arena.get_mut(id)?.key = key;
        Ok(id)
    }

    pub fn insert_provider(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        provision: ContextProvision,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert(parent, NodeKind::Provider)?;
        self.arena.get_mut(id)?.key = key;
        self.providers.insert(id, provision);
        Ok(id)
    }

    pub fn provision(&self, id: NodeId) -> Result<&ContextProvision, TreeError> {
        if self.arena.get(id)?.kind != NodeKind::Provider {
            return Err(TreeError::NotComponent);
        }
        self.providers.get(id).ok_or(TreeError::NotComponent)
    }

    pub fn set_provision(
        &mut self,
        id: NodeId,
        provision: ContextProvision,
    ) -> Result<(), TreeError> {
        if self.arena.get(id)?.kind != NodeKind::Provider {
            return Err(TreeError::NotComponent);
        }
        self.providers.insert(id, provision);
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
        let node = self.arena.get(id)?;
        if node.kind != NodeKind::Component {
            return Err(TreeError::NotComponent);
        }
        node.scope.ok_or(TreeError::NotComponent)
    }

    pub fn component_type(&self, id: NodeId) -> Result<TypeId, TreeError> {
        let node = self.arena.get(id)?;
        if node.kind != NodeKind::Component {
            return Err(TreeError::NotComponent);
        }

        node.component_type.ok_or(TreeError::NotComponent)
    }

    pub fn component_node(&self, scope: ScopeId) -> Result<Option<NodeId>, TreeError> {
        Ok(self.components.get(&scope).copied())
    }

    pub fn native(&self, id: NodeId) -> Result<&NativeState, TreeError> {
        self.arena
            .get(id)?
            .native
            .as_ref()
            .ok_or(TreeError::NotNative)
    }

    pub fn native_mut(&mut self, id: NodeId) -> Result<&mut NativeState, TreeError> {
        self.arena
            .get_mut(id)?
            .native
            .as_mut()
            .ok_or(TreeError::NotNative)
    }

    pub fn insert_virtual(
        &mut self,
        identity: WindowToken,
        parent: Option<NodeId>,
        keys: impl IntoIterator<Item = Key>,
    ) -> Result<NodeId, TreeError> {
        let id = self.insert(parent, NodeKind::VirtualCollection)?;
        let model = match VirtualModel::new(identity, id, keys) {
            Ok(model) => model,
            Err(error) => {
                self.retire_subtree(id)?;
                return Err(error.into());
            }
        };
        self.arena.get_mut(id)?.virtual_model = Some(model);
        Ok(id)
    }

    pub fn insert_virtual_items(
        &mut self,
        identity: WindowToken,
        parent: Option<NodeId>,
        key: Option<Key>,
        items: Rc<Vec<KeyedView>>,
    ) -> Result<NodeId, TreeError> {
        let keys = items.iter().map(|item| item.key().clone());
        let id = self.insert_virtual(identity, parent, keys)?;
        let node = self.arena.get_mut(id)?;
        node.key = key;
        node.virtual_items = Some(items);
        Ok(id)
    }

    pub fn virtual_items(&self, id: NodeId) -> Result<&[KeyedView], TreeError> {
        self.arena
            .get(id)?
            .virtual_items
            .as_deref()
            .map(Vec::as_slice)
            .ok_or(TreeError::NotVirtual)
    }

    pub fn virtual_item_at(&self, id: NodeId, index: usize) -> Result<&KeyedView, TreeError> {
        self.virtual_items(id)?
            .get(index)
            .ok_or_else(|| VirtualModelError::MissingIndex(index).into())
    }

    pub fn realized(
        &self,
        id: NodeId,
        container: RealizedContainer,
    ) -> Result<Option<RealizedRow>, TreeError> {
        Ok(self.arena.get(id)?.realized.get(&container).copied())
    }

    pub fn realized_rows(
        &self,
        id: NodeId,
    ) -> Result<impl Iterator<Item = (RealizedContainer, RealizedRow)> + '_, TreeError> {
        Ok(self
            .arena
            .get(id)?
            .realized
            .iter()
            .map(|(container, row)| (*container, *row)))
    }

    pub fn realized_container(
        &self,
        id: NodeId,
        native_root: NodeId,
    ) -> Result<Option<RealizedContainer>, TreeError> {
        Ok(self
            .arena
            .get(id)?
            .realized
            .iter()
            .find_map(|(container, row)| {
                (row.native_root == Some(native_root)).then_some(*container)
            }))
    }

    pub fn realized_container_for_logical(
        &self,
        id: NodeId,
        logical_root: NodeId,
    ) -> Result<Option<RealizedContainer>, TreeError> {
        Ok(self
            .arena
            .get(id)?
            .realized
            .iter()
            .find_map(|(container, row)| (row.logical_root == logical_root).then_some(*container)))
    }

    pub fn set_realized(
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
        let realized = &mut self.arena.get_mut(id)?.realized;
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
        let row = self
            .arena
            .get_mut(id)?
            .realized
            .get_mut(&container)
            .ok_or(TreeError::NotVirtual)?;
        *row = RealizedRow {
            logical_root,
            native_root,
        };
        Ok(())
    }

    pub fn update_virtual_items(
        &mut self,
        id: NodeId,
        items: Rc<Vec<KeyedView>>,
    ) -> Result<(), TreeError> {
        self.arena.get_mut(id)?.virtual_items = Some(items);
        Ok(())
    }

    pub fn virtual_model(&self, id: NodeId) -> Result<&VirtualModel, TreeError> {
        self.arena
            .get(id)?
            .virtual_model
            .as_ref()
            .ok_or(TreeError::NotVirtual)
    }

    pub fn virtual_model_mut(&mut self, id: NodeId) -> Result<&mut VirtualModel, TreeError> {
        self.arena
            .get_mut(id)?
            .virtual_model
            .as_mut()
            .ok_or(TreeError::NotVirtual)
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
        Ok(self.arena.get(id)?.kind)
    }

    pub(crate) fn window_title(&self) -> Option<&WindowTitleState> {
        self.window_title.as_ref()
    }

    pub(crate) fn set_window_title(&mut self, value: Option<WindowTitleState>) {
        self.window_title = value;
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
            parent.realized.retain(|_, row| row.logical_root != id);
        } else {
            self.root = None;
        }

        let mut retired = Vec::with_capacity(order.len());
        for id in order {
            let node = self.arena.remove(id)?;
            if let Some(scope) = node.scope {
                Rc::make_mut(&mut self.components).remove(&scope);
            }
            if node.kind == NodeKind::Provider {
                self.providers.remove(id);
            }
            retired.push((id, node.kind));
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
mod tests {
    use super::*;
    use crate::core::scope::ScopeArena;
    use std::mem::size_of;

    fn identity() -> WindowToken {
        WindowToken::new(WindowId::allocate())
    }
    use std::collections::{HashMap, HashSet};

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (self.0 >> 32) as usize
        }
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn generated_control_growth_preserves_core_layouts() {
        assert_eq!(size_of::<Node>(), 432);
        assert_eq!(size_of::<MountedProps>(), 72);
        assert_eq!(size_of::<Element>(), 88);
    }

    #[test]
    fn retires_children_before_parent() {
        let mut tree = Tree::new();
        let root = tree.insert(None, NodeKind::Application).unwrap();
        let window = tree.insert(Some(root), NodeKind::Window).unwrap();
        let component = tree.insert(Some(window), NodeKind::Component).unwrap();
        let slot = tree.insert(Some(component), NodeKind::Slot).unwrap();
        let native = tree
            .insert(Some(slot), NodeKind::Native(MountedKind::TextBlock))
            .unwrap();
        let collection = tree
            .insert(Some(window), NodeKind::VirtualCollection)
            .unwrap();

        assert_eq!(tree.parent(native), Ok(Some(slot)));
        assert_eq!(tree.children(root), Ok(&[window][..]));

        let retired = tree.retire_subtree(window).unwrap();

        assert_eq!(
            retired,
            vec![
                (native, NodeKind::Native(MountedKind::TextBlock)),
                (slot, NodeKind::Slot),
                (component, NodeKind::Component),
                (collection, NodeKind::VirtualCollection),
                (window, NodeKind::Window),
            ]
        );
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.children(root), Ok(&[][..]));
        assert_eq!(
            tree.parent(window),
            Err(TreeError::Arena(ArenaError::Stale(window)))
        );
    }

    #[test]
    fn candidate_tree_clones_component_identity_without_component_state() {
        struct State {
            value: u32,
        }

        let mut scopes = ScopeArena::new();
        let scope = scopes.reserve(State { value: 1 }).unwrap();
        scopes.publish(scope).unwrap();
        let mut tree = Tree::new();
        let root = tree.insert(None, NodeKind::Application).unwrap();
        let component = tree
            .insert_component(
                Some(root),
                Some(Key::from("child")),
                scope,
                TypeId::of::<State>(),
            )
            .unwrap();

        let candidate = tree.clone();
        scopes.get_mut(scope).unwrap().value = 2;

        assert_eq!(tree.component_scope(component), Ok(scope));
        assert_eq!(candidate.component_scope(component), Ok(scope));
        assert_eq!(
            candidate.component_type(component),
            Ok(TypeId::of::<State>())
        );
        assert_eq!(scopes.get(scope).unwrap().value, 2);
    }

    #[test]
    fn rejects_second_root() {
        let mut tree = Tree::new();
        tree.insert(None, NodeKind::Application).unwrap();

        assert_eq!(
            tree.insert(None, NodeKind::Application),
            Err(TreeError::RootAlreadyExists)
        );
    }

    #[test]
    fn virtual_model_uses_its_arena_identity_for_leases() {
        let mut tree = Tree::new();
        let application = tree.insert(None, NodeKind::Application).unwrap();
        let collection = tree
            .insert_virtual(identity(), Some(application), [Key::from("a")])
            .unwrap();

        let lease = tree
            .virtual_model_mut(collection)
            .unwrap()
            .realize(0, RealizedContainer(1))
            .unwrap();

        assert_eq!(lease.collection, collection);
        tree.retire_subtree(collection).unwrap();
        assert!(matches!(
            tree.virtual_model(collection),
            Err(TreeError::Arena(ArenaError::Stale(id))) if id == collection
        ));
    }

    #[test]
    fn realized_container_mapping_cannot_be_overwritten() {
        let mut tree = Tree::new();
        let application = tree.insert(None, NodeKind::Application).unwrap();
        let collection = tree
            .insert_virtual(identity(), Some(application), [Key::from("a")])
            .unwrap();
        let first = tree
            .insert(Some(collection), NodeKind::Native(MountedKind::TextBlock))
            .unwrap();
        let second = tree
            .insert(Some(collection), NodeKind::Native(MountedKind::Button))
            .unwrap();
        let container = RealizedContainer(1);

        tree.set_realized(collection, container, first, Some(first))
            .unwrap();

        assert_eq!(
            tree.set_realized(collection, container, second, Some(second)),
            Err(TreeError::RealizedConflict(container))
        );
    }

    #[test]
    fn detached_realized_row_remains_addressable_by_logical_root() {
        let mut tree = Tree::new();
        let collection = tree
            .insert_virtual(identity(), None, [Key::from("row")])
            .unwrap();
        let logical = tree.insert(Some(collection), NodeKind::Fragment).unwrap();
        let native = tree
            .insert(Some(logical), NodeKind::Native(MountedKind::TextBlock))
            .unwrap();
        let container = RealizedContainer(1);

        tree.set_realized(collection, container, logical, None)
            .unwrap();
        assert_eq!(
            tree.realized_container_for_logical(collection, logical),
            Ok(Some(container))
        );
        assert_eq!(tree.realized_container(collection, native), Ok(None));

        tree.update_realized(collection, container, logical, Some(native))
            .unwrap();
        assert_eq!(
            tree.realized_container(collection, native),
            Ok(Some(container))
        );
    }

    #[test]
    fn randomized_insert_and_retire_matches_tree_model() {
        let mut rng = Rng(0x5eed);
        let mut tree = Tree::new();
        let root = tree.insert(None, NodeKind::Application).unwrap();
        let mut live = vec![root];
        let mut parents = HashMap::from([(root, None)]);

        for _ in 0..5_000 {
            if live.len() == 1 || !rng.next().is_multiple_of(3) {
                let parent = live[rng.next() % live.len()];
                let id = tree.insert(Some(parent), NodeKind::Slot).unwrap();
                live.push(id);
                assert_eq!(parents.insert(id, Some(parent)), None);
            } else {
                let victim = live[1 + rng.next() % (live.len() - 1)];
                let retired = tree.retire_subtree(victim).unwrap();
                let retired_ids: HashSet<_> = retired.iter().map(|(id, _)| *id).collect();
                assert_eq!(retired.len(), retired_ids.len());

                let positions: HashMap<_, _> = retired
                    .iter()
                    .enumerate()
                    .map(|(position, (id, _))| (*id, position))
                    .collect();
                for id in retired_ids.iter().copied() {
                    if let Some(Some(parent)) = parents.get(&id)
                        && let Some(parent_position) = positions.get(parent)
                    {
                        assert!(positions[&id] < *parent_position);
                    }
                    assert_eq!(
                        tree.parent(id),
                        Err(TreeError::Arena(ArenaError::Stale(id)))
                    );
                }

                live.retain(|id| !retired_ids.contains(id));
                parents.retain(|id, _| !retired_ids.contains(id));
            }

            assert_eq!(tree.len(), live.len());
            assert_eq!(tree.parent(root), Ok(None));
        }
    }

    #[test]
    fn element_split_keeps_props_shallow_and_moves_children_once() {
        let parts =
            Element::from(StackPanel::new().native_child("text", TextBlock::new().text("hello")))
                .into_parts();

        assert_eq!(parts.kind, MountedKind::StackPanel);
        assert!(matches!(parts.props, MountedProps::StackPanel { .. }));
        let ElementStructure::Children(children) = parts.structure else {
            panic!("expected keyed children");
        };
        assert_eq!(children.len(), 1);
        assert!(matches!(children[0].element(), Element::TextBlock(_)));
    }
}
