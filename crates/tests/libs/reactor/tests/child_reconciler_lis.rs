use std::rc::Rc;

use windows_reactor::core::backend::{ControlId, Op, Prop, RecordingBackend};
use windows_reactor::core::element::{Element, StackPanel};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::dsl::{text_block, ElementExt};
use windows_reactor::vstack;

fn keyed_stack(items: impl IntoIterator<Item = &'static str>) -> StackPanel {
    let children: Vec<Element> = items
        .into_iter()
        .map(|k| text_block(k).with_key(k).into())
        .collect();
    let mut s = vstack(children);
    s.key = Some("root".to_string());
    s
}

fn mount_initial(items: &[&'static str]) -> (Reconciler<RecordingBackend>, Element, ControlId) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let initial: Element = keyed_stack(items.iter().copied()).into();
    let id = r
        .reconcile(None, &initial, None, Rc::new(|| {}))
        .expect("initial stack mounted");
    r.backend.clear_ops();
    (r, initial, id)
}

#[test]
fn reverse_five_keyed_items_uses_four_moves() {
    let (mut r, old, id) = mount_initial(&["a", "b", "c", "d", "e"]);
    let new: Element = keyed_stack(["e", "d", "c", "b", "a"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    let destroys = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();
    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    assert_eq!(destroys, 0, "reversal must not destroy anything");
    assert_eq!(creates, 0, "reversal must not create anything");
    assert_eq!(moves, 4, "reversal of 5 items = 4 moves (LIS=1)");
}

#[test]
fn swap_two_adjacent_keyed_items_uses_one_move() {
    let (mut r, old, id) = mount_initial(&["a", "b", "c", "d"]);
    let new: Element = keyed_stack(["a", "c", "b", "d"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    let destroys = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();
    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    assert_eq!(destroys, 0);
    assert_eq!(creates, 0);
    assert_eq!(moves, 1, "single swap = 1 move");
}

#[test]
fn insert_in_middle_emits_insert_no_moves() {
    let (mut r, old, id) = mount_initial(&["a", "b", "d", "e"]);
    let new: Element = keyed_stack(["a", "b", "c", "d", "e"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    let inserts = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::InsertChild { .. }))
        .count();
    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    let destroys = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();
    assert_eq!(creates, 1, "one new mount");
    assert_eq!(inserts, 1, "one insert_child into the middle");
    assert_eq!(moves, 0, "no existing items need to move");
    assert_eq!(destroys, 0);
}

#[test]
fn remove_from_middle_emits_remove_no_moves() {
    let (mut r, old, id) = mount_initial(&["a", "b", "c", "d", "e"]);
    let new: Element = keyed_stack(["a", "b", "d", "e"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let removes = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::RemoveChild { .. }))
        .count();

    let destroys = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();
    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    assert_eq!(removes, 1, "one remove_child");
    assert_eq!(destroys, 1, "removed TextBlock is destroyed");
    assert_eq!(moves, 0);
    assert_eq!(creates, 0);
}

#[test]
fn keyed_list_append_uses_insert_child() {
    let (mut r, old, id) = mount_initial(&["a", "b"]);
    let new: Element = keyed_stack(["a", "b", "c", "d"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    let inserts = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::InsertChild { .. }))
        .count();
    assert_eq!(creates, 2);

    assert_eq!(inserts, 2);
}

#[test]
fn unkeyed_list_falls_through_to_positional() {
    let mut r = Reconciler::new(RecordingBackend::new());
    let initial: Element = vstack((text_block("a"), text_block("b"), text_block("c"))).into();
    let id = r
        .reconcile(None, &initial, None, Rc::new(|| {}))
        .expect("mount");
    r.backend.clear_ops();

    let new: Element = vstack((text_block("c"), text_block("b"), text_block("a"))).into();
    r.reconcile(Some(&initial), &new, Some(id), Rc::new(|| {}));

    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    assert_eq!(moves, 0, "positional path emits no moves");
    let text_sets = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::SetProp { prop, .. } if matches!(prop, Prop::Text)))
        .count();
    assert_eq!(text_sets, 2, "two text-content updates");
}

#[test]
fn keyed_list_identical_emits_no_structural_ops() {
    let (mut r, old, id) = mount_initial(&["a", "b", "c"]);
    let new: Element = keyed_stack(["a", "b", "c"]).into();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let creates = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    let moves = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::MoveChild { .. }))
        .count();
    let removes = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::RemoveChild { .. }))
        .count();
    let inserts = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::InsertChild { .. }))
        .count();
    let destroys = r
        .backend
        .ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();

    assert_eq!(creates, 0);
    assert_eq!(moves, 0);
    assert_eq!(removes, 0);
    assert_eq!(inserts, 0);
    assert_eq!(destroys, 0);
}
