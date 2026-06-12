use std::rc::Rc;

use windows_reactor::Reconciler;
use windows_reactor::{Button, Element, Orientation, StackPanel, TextBlock};
use windows_reactor::{ControlId, Op, RecordingBackend};

fn rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn make_stack(children: Vec<Element>) -> Element {
    Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children,
        ..StackPanel::default()
    })
}

fn mount_stack(children: Vec<Element>) -> (Reconciler<RecordingBackend>, ControlId) {
    let el = make_stack(children);
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &el, None, rr())
        .expect("stack mounts to an id");
    (r, id)
}

fn update_to(
    r: &mut Reconciler<RecordingBackend>,
    id: ControlId,
    old_children: Vec<Element>,
    new_children: Vec<Element>,
) {
    let old_el = make_stack(old_children);
    let new_el = make_stack(new_children);
    let _ = r.reconcile(Some(&old_el), &new_el, Some(id), rr());
}

#[test]
fn same_children_record_no_ops() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::TextBlock(TextBlock::new("b")),
    ]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
    );

    assert!(r.backend.ops.is_empty(), "ops: {:?}", r.backend.ops);

    assert!(r.debug_elements_skipped >= 1);
}

#[test]
fn same_children_but_stack_spacing_changed_still_skips_children() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::TextBlock(TextBlock::new("b")),
    ]);
    r.backend.clear_ops();

    let old = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        spacing: 4.0,
        children: vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
        ..StackPanel::default()
    });
    let new = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        spacing: 8.0,
        children: vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
        ..StackPanel::default()
    });
    let _ = r.reconcile(Some(&old), &new, Some(id), rr());

    assert_eq!(r.backend.ops.len(), 1, "{:?}", r.backend.ops);
    assert!(matches!(
        r.backend.ops[0],
        Op::SetProp {
            prop: windows_reactor::Prop::Spacing,
            ..
        }
    ));
    assert!(
        r.debug_elements_skipped >= 2,
        "expected each child to skip; got {}",
        r.debug_elements_skipped
    );
}

#[test]
fn appending_one_child_creates_and_appends() {
    let (mut r, id) = mount_stack(vec![Element::TextBlock(TextBlock::new("a"))]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![Element::TextBlock(TextBlock::new("a"))],
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
    );

    let ops = &r.backend.ops;
    assert_eq!(ops.len(), 5, "ops: {ops:?}");
    assert!(matches!(ops[0], Op::Create { .. }));
    // SetProp for IsTextSelectionEnabled, Text, TextWrapping
    assert!(matches!(ops[1], Op::SetProp { .. }));
    assert!(matches!(ops[2], Op::SetProp { .. }));
    assert!(matches!(ops[3], Op::SetProp { .. }));
    match &ops[4] {
        Op::AppendChild { parent, .. } => assert_eq!(*parent, id),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn removing_tail_child_destroys_and_removes() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::TextBlock(TextBlock::new("b")),
    ]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
        vec![Element::TextBlock(TextBlock::new("a"))],
    );

    let ops = &r.backend.ops;

    assert_eq!(ops.len(), 2, "ops: {ops:?}");
    assert!(matches!(ops[0], Op::Destroy { .. }));
    match &ops[1] {
        Op::RemoveChild { parent, index } => {
            assert_eq!(*parent, id);
            assert_eq!(*index, 1);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn replacing_middle_child_with_different_variant_destroys_and_creates() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::TextBlock(TextBlock::new("b")),
        Element::TextBlock(TextBlock::new("c")),
    ]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
            Element::TextBlock(TextBlock::new("c")),
        ],
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::Button(Button::new("b")),
            Element::TextBlock(TextBlock::new("c")),
        ],
    );

    let ops = &r.backend.ops;

    assert!(matches!(ops[0], Op::Destroy { .. }));
    assert!(matches!(
        ops[1],
        Op::Create {
            kind: windows_reactor::ControlKind::Button,
            ..
        }
    ));
    let replace = ops.iter().find_map(|o| match o {
        Op::ReplaceChild { parent, index, .. } => Some((*parent, *index)),
        _ => None,
    });
    assert_eq!(replace, Some((id, 1)));
}

#[test]
fn empty_elements_are_filtered_out_of_positional_diff() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::Empty,
        Element::TextBlock(TextBlock::new("b")),
    ]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::Empty,
            Element::TextBlock(TextBlock::new("b")),
        ],
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::Empty,
            Element::TextBlock(TextBlock::new("b")),
        ],
    );
    assert!(r.backend.ops.is_empty());
}

#[test]
fn remove_two_tail_children() {
    let (mut r, id) = mount_stack(vec![
        Element::TextBlock(TextBlock::new("a")),
        Element::TextBlock(TextBlock::new("b")),
        Element::TextBlock(TextBlock::new("c")),
    ]);
    r.backend.clear_ops();

    update_to(
        &mut r,
        id,
        vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
            Element::TextBlock(TextBlock::new("c")),
        ],
        vec![Element::TextBlock(TextBlock::new("a"))],
    );

    let ops = &r.backend.ops;

    let removes: Vec<_> = ops
        .iter()
        .filter_map(|o| match o {
            Op::RemoveChild { index, .. } => Some(*index),
            _ => None,
        })
        .collect();
    assert_eq!(removes, vec![2, 1], "removes should be high-to-low");
}
