use super::*;
use crate::core::scope::ScopeArena;
#[cfg(target_pointer_width = "64")]
use std::mem::size_of;

fn identity() -> WindowToken {
    WindowToken::new(WindowId::allocate())
}
use std::collections::{HashMap, HashSet};

#[test]
fn observation_slot_preserves_and_advances_revisions() {
    let first = Callback::new(|_: ()| {});
    let second = Callback::new(|_: ()| {});
    let mut slot = ObservationSlot::default();

    assert!(slot.get().is_none());
    slot.set(Some(first.clone()));
    assert_eq!(slot.get(), Some((&first, 1)));
    slot.set(Some(first.clone()));
    assert_eq!(slot.get(), Some((&first, 1)));
    slot.set(Some(second.clone()));
    assert_eq!(slot.get(), Some((&second, 2)));
    slot.set(None);
    assert!(slot.get().is_none());
    assert_eq!(slot.revision, 2);
    slot.set(None);
    assert_eq!(slot.revision, 2);
    slot.set(Some(second.clone()));
    assert_eq!(slot.get(), Some((&second, 3)));
    slot.revision = u32::MAX;
    slot.set(Some(first.clone()));
    assert_eq!(slot.get(), Some((&first, 0)));
}

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
    assert_eq!(size_of::<Node>(), 304);
    assert_eq!(size_of::<NativeState>(), 72);
    assert_eq!(size_of::<MountedProps>(), 16);
    assert_eq!(size_of::<Element>(), 16);
}

#[test]
fn cloned_elements_share_generated_control_payloads() {
    let first = Element::from(TextBlock::new());
    let second = first.clone();
    let (Element::TextBlock(first), Element::TextBlock(second)) = (&first, &second) else {
        unreachable!()
    };

    assert!(Rc::ptr_eq(first, second));
}

#[test]
fn retires_children_before_parent() {
    struct Component;

    let mut scopes = ScopeArena::new();
    let scope = scopes.reserve(Component).unwrap();
    let mut tree = Tree::new();
    let root = tree.insert(None, NodeKind::Application).unwrap();
    let window = tree.insert(Some(root), NodeKind::Window).unwrap();
    let component = tree
        .insert_component(Some(window), None, scope, TypeId::of::<Component>())
        .unwrap();
    let slot = tree.insert(Some(component), NodeKind::Slot).unwrap();
    let parts = Element::from(TextBlock::new()).into_parts();
    let native = tree
        .insert_native(Some(slot), parts.kind, None, parts.props, None)
        .unwrap();
    let collection = tree.insert_virtual(identity(), Some(window), []).unwrap();

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
fn candidate_tree_clones_owned_node_payloads_on_write() {
    let mut tree = Tree::new();
    let root = tree.insert(None, NodeKind::Application).unwrap();
    let menu = Menu::new([MenuItem::item("old", "Old")], |_| {});
    let menu_callback = menu.on_click.clone();
    let menu_node = tree
        .insert_menu(Some(root), None, OwnedMenuKind::ButtonFlyout, menu)
        .unwrap();
    let flyout = CommandBarFlyout::new([CommandBarCommand::button("old", "Old")], [], |_| {});
    let flyout_callback = flyout.on_click.clone();
    let flyout_node = tree
        .insert_command_bar_flyout(Some(root), None, flyout)
        .unwrap();
    let tree_node = tree
        .insert_tree_nodes(Some(root), None, Rc::new(vec![TreeNode::new("old", "Old")]))
        .unwrap();

    let mut candidate = tree.clone();
    candidate
        .update_menu(menu_node, Menu::new([MenuItem::item("new", "New")], |_| {}))
        .unwrap();
    candidate
        .update_command_bar_flyout(
            flyout_node,
            CommandBarFlyout::new([CommandBarCommand::button("new", "New")], [], |_| {}),
        )
        .unwrap();
    candidate
        .update_tree_nodes(tree_node, Rc::new(vec![TreeNode::new("new", "New")]))
        .unwrap();

    assert_eq!(tree.owned_revision(menu_node), Ok(1));
    assert_eq!(tree.owned_revision(flyout_node), Ok(1));
    assert_eq!(tree.owned_callback(menu_node), Ok(&menu_callback));
    assert_eq!(tree.owned_callback(flyout_node), Ok(&flyout_callback));
    assert_eq!(
        tree.owned_menu(menu_node).unwrap()[0].key(),
        &Key::from("old")
    );
    assert_eq!(
        tree.owned_commands(flyout_node).unwrap().0[0].key(),
        &Key::from("old")
    );
    assert_eq!(tree.tree_nodes(tree_node).unwrap()[0].key, Key::from("old"));

    assert_eq!(candidate.owned_revision(menu_node), Ok(2));
    assert_eq!(candidate.owned_revision(flyout_node), Ok(2));
    assert_eq!(
        candidate.owned_menu(menu_node).unwrap()[0].key(),
        &Key::from("new")
    );
    assert_eq!(
        candidate.owned_commands(flyout_node).unwrap().0[0].key(),
        &Key::from("new")
    );
    assert_eq!(
        candidate.tree_nodes(tree_node).unwrap()[0].key,
        Key::from("new")
    );
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
fn generic_insert_rejects_payload_bearing_kinds() {
    let mut tree = Tree::new();

    assert_eq!(
        tree.insert(None, NodeKind::Component),
        Err(TreeError::IncompleteNode(NodeKind::Component))
    );
    assert_eq!(
        tree.insert(None, NodeKind::Native(MountedKind::Button)),
        Err(TreeError::IncompleteNode(NodeKind::Native(
            MountedKind::Button
        )))
    );
    assert_eq!(
        tree.insert(None, NodeKind::VirtualCollection),
        Err(TreeError::IncompleteNode(NodeKind::VirtualCollection))
    );
    assert_eq!(
        tree.insert(None, NodeKind::Provider),
        Err(TreeError::IncompleteNode(NodeKind::Provider))
    );
}

#[test]
fn set_kind_preserves_kind_specific_state() {
    struct Component;

    let mut scopes = ScopeArena::new();
    let scope = scopes.reserve(Component).unwrap();
    let mut tree = Tree::new();
    let root = tree.insert(None, NodeKind::Application).unwrap();
    let component = tree
        .insert_component(Some(root), None, scope, TypeId::of::<Component>())
        .unwrap();

    assert_eq!(tree.set_kind(component, NodeKind::Component), Ok(()));
    assert_eq!(
        tree.set_kind(component, NodeKind::Fragment),
        Err(TreeError::KindMismatch {
            current: NodeKind::Component,
            requested: NodeKind::Fragment,
        })
    );
    assert_eq!(tree.component_node(scope), Ok(Some(component)));
    assert_eq!(tree.component_scope(component), Ok(scope));
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
    let first_parts = Element::from(TextBlock::new()).into_parts();
    let first = tree
        .insert_native(
            Some(collection),
            first_parts.kind,
            None,
            first_parts.props,
            None,
        )
        .unwrap();
    let second_parts = Element::from(Button::new()).into_parts();
    let second = tree
        .insert_native(
            Some(collection),
            second_parts.kind,
            None,
            second_parts.props,
            None,
        )
        .unwrap();
    let container = RealizedContainer(1);

    tree.set_realized(collection, container, 0, first, Some(first))
        .unwrap();

    assert_eq!(
        tree.set_realized(collection, container, 0, second, Some(second)),
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
    let parts = Element::from(TextBlock::new()).into_parts();
    let native = tree
        .insert_native(Some(logical), parts.kind, None, parts.props, None)
        .unwrap();
    let container = RealizedContainer(1);

    tree.set_realized(collection, container, 0, logical, None)
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
