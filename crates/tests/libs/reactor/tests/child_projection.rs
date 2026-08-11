use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    ContentDialog, ControlId, Element, KeyExt, Orientation, Reconciler, ReconcilerTestExt,
    StackPanel, TextBlock,
};

fn rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn text(key: &str) -> Element {
    TextBlock::new(key).with_key(key).into()
}

fn dialog(key: &str) -> Element {
    ContentDialog::new(key).with_key(key).into()
}

fn stack(children: Vec<Element>) -> Element {
    StackPanel {
        orientation: Orientation::Vertical,
        children,
        ..StackPanel::default()
    }
    .into()
}

fn mount(children: Vec<Element>) -> (Reconciler<RecordingBackend>, Element, ControlId) {
    let element = stack(children);
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let root = reconciler
        .reconcile(None, &element, None, rerender())
        .unwrap();
    (reconciler, element, root)
}

fn update(
    reconciler: &mut Reconciler<RecordingBackend>,
    root: ControlId,
    old: &Element,
    children: Vec<Element>,
) -> Element {
    let new = stack(children);
    assert_eq!(
        reconciler.reconcile(Some(old), &new, Some(root), rerender()),
        Some(root)
    );
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    new
}

#[test]
fn owned_only_children_are_not_projected_to_the_backend() {
    let (mut reconciler, old, root) =
        mount(vec![text("a"), Element::Empty, dialog("dialog"), text("b")]);
    let a = reconciler.child_at(root, 0).unwrap();
    let owned_only = reconciler.child_at(root, 1).unwrap();
    let b = reconciler.child_at(root, 2).unwrap();

    assert_eq!(
        reconciler.backend.children_of(root),
        &[a, b],
        "{:?}",
        reconciler.backend.ops
    );
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::AppendChild { child, .. } if *child == owned_only))
    );

    reconciler.backend.clear_ops();
    let _new = update(
        &mut reconciler,
        root,
        &old,
        vec![text("a"), Element::Empty, text("b")],
    );

    assert_eq!(reconciler.backend.children_of(root), &[a, b]);
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::RemoveChild { .. }))
    );
}

#[test]
fn replacing_across_projection_boundaries_uses_visual_indices() {
    let (mut reconciler, old, root) = mount(vec![text("a"), dialog("middle"), text("b")]);
    let a = reconciler.child_at(root, 0).unwrap();
    let b = reconciler.child_at(root, 2).unwrap();

    reconciler.backend.clear_ops();
    let visual = update(
        &mut reconciler,
        root,
        &old,
        vec![text("a"), text("middle"), text("b")],
    );
    let middle = reconciler.child_at(root, 1).unwrap();

    assert_eq!(
        reconciler.backend.children_of(root),
        &[a, middle, b],
        "{:?}",
        reconciler.backend.ops
    );
    assert!(reconciler.backend.ops.iter().any(
        |op| matches!(op, Op::InsertChild { parent, index: 1, child } if *parent == root && *child == middle)
    ));
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::ReplaceChild { .. }))
    );

    reconciler.backend.clear_ops();
    let _owned_only = update(
        &mut reconciler,
        root,
        &visual,
        vec![text("a"), dialog("middle"), text("b")],
    );

    assert_eq!(reconciler.backend.children_of(root), &[a, b]);
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::RemoveChild { parent, index: 1 } if *parent == root))
    );
}

#[test]
fn keyed_moves_ignore_owned_only_children() {
    let (mut reconciler, old, root) = mount(vec![text("a"), dialog("dialog"), text("b")]);
    let a = reconciler.child_at(root, 0).unwrap();
    let b = reconciler.child_at(root, 2).unwrap();

    reconciler.backend.clear_ops();
    let _new = update(
        &mut reconciler,
        root,
        &old,
        vec![text("b"), dialog("dialog"), text("a")],
    );

    assert_eq!(reconciler.backend.children_of(root), &[b, a]);
    let child_ops: Vec<_> = reconciler
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::AppendChild { .. }
                    | Op::InsertChild { .. }
                    | Op::RemoveChild { .. }
                    | Op::ReplaceChild { .. }
                    | Op::MoveChild { .. }
            )
        })
        .collect();
    assert_eq!(child_ops.len(), 1, "{child_ops:?}");
    assert!(matches!(
        child_ops[0],
        Op::MoveChild {
            parent,
            from: 1,
            to: 0
        } if *parent == root
    ));
}

#[test]
fn forward_moves_use_the_post_move_projection_index() {
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let root = reconciler.acquire_control(windows_reactor::ControlKind::StackPanel);
    let a = reconciler.acquire_control(windows_reactor::ControlKind::TextBlock);
    let b = reconciler.acquire_control(windows_reactor::ControlKind::TextBlock);
    let owned_only = reconciler.acquire_control(windows_reactor::ControlKind::ContentDialog);
    let c = reconciler.acquire_control(windows_reactor::ControlKind::TextBlock);
    for child in [a, b, owned_only, c] {
        reconciler.append_child_tracked(root, child);
    }
    reconciler.backend.clear_ops();

    reconciler.move_child_tracked(root, 0, 2);

    assert_eq!(reconciler.child_at(root, 0), Some(b));
    assert_eq!(reconciler.child_at(root, 1), Some(owned_only));
    assert_eq!(reconciler.child_at(root, 2), Some(a));
    assert_eq!(reconciler.child_at(root, 3), Some(c));
    assert_eq!(reconciler.backend.children_of(root), &[b, a, c]);
    assert!(matches!(
        reconciler.backend.ops.as_slice(),
        [Op::MoveChild {
            parent,
            from: 0,
            to: 1
        }] if *parent == root
    ));
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
}

#[test]
#[should_panic(expected = "mounted child destination index out of bounds")]
fn moves_reject_an_out_of_bounds_destination() {
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let root = reconciler.acquire_control(windows_reactor::ControlKind::StackPanel);
    let child = reconciler.acquire_control(windows_reactor::ControlKind::TextBlock);
    reconciler.append_child_tracked(root, child);

    reconciler.move_child_tracked(root, 0, 1);
}
