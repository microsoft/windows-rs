use super::*;

#[test]
fn exhausted_slots_are_retired() {
    let mut arena = Arena::default();
    let id = arena.insert(Node {
        parent: None,
        children: Vec::new(),
        kind: NodeKind::Logical,
        native_kind: None,
        mounted: None,
    });
    arena.slots[id.index() as usize].generation = u32::MAX;
    let exhausted = NodeId::new(id.index(), u32::MAX);

    arena.remove(exhausted).unwrap();
    let replacement = arena.insert(Node {
        parent: None,
        children: Vec::new(),
        kind: NodeKind::Logical,
        native_kind: None,
        mounted: None,
    });

    assert_ne!(replacement.index(), exhausted.index());
    assert!(!arena.contains(exhausted));
}

#[test]
fn revision_tracks_insertions_and_removals() {
    let mut arena = Arena::default();
    let initial = arena.revision();
    let id = arena.insert(Node {
        parent: None,
        children: Vec::new(),
        kind: NodeKind::Logical,
        native_kind: None,
        mounted: None,
    });
    let inserted = arena.revision();

    assert_ne!(inserted, initial);
    assert!(
        arena
            .remove(NodeId::new(id.index(), id.generation() + 1))
            .is_none()
    );
    assert_eq!(arena.revision(), inserted);
    arena.remove(id).unwrap();
    assert_ne!(arena.revision(), inserted);
}
