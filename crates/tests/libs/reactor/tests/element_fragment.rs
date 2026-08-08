//! Child-only fragment reconciliation tests.

use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::{Button, Element, TextBlock, fragment, vstack};
use windows_reactor::{Prop, PropValue};

fn noop() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn label(name: &str) -> Element {
    Element::TextBlock(TextBlock::new(name))
}

fn keyed_label(key: &str, name: &str) -> Element {
    Element::TextBlock(TextBlock {
        key: Some(key.into()),
        text: name.into(),
        ..Default::default()
    })
}

fn appends_for(ops: &[Op]) -> usize {
    ops.iter()
        .filter(|op| matches!(op, Op::AppendChild { .. } | Op::InsertChild { .. }))
        .count()
}

fn child_text_contents(
    r: &Reconciler<RecordingBackend>,
    parent: windows_reactor::ControlId,
) -> Vec<String> {
    r.backend
        .children_of(parent)
        .iter()
        .map(|cid| {
            r.backend
                .ops
                .iter()
                .filter_map(|op| match op {
                    Op::SetProp {
                        id,
                        prop: Prop::Text,
                        value: PropValue::Str(text),
                    } if id == cid => Some(text.clone()),
                    _ => None,
                })
                .next_back()
                .unwrap_or_default()
        })
        .collect()
}

#[test]
fn fragment_flattens_before_reconciliation() {
    let stack = vstack((label("a"), fragment((label("b"), label("c"))), label("d")));
    assert_eq!(stack.children.len(), 4);
    let element = stack.into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &element, None, noop()).unwrap();

    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c", "d"]);
}

#[test]
fn nested_fragments_flatten_recursively() {
    let stack = vstack((
        label("a"),
        fragment((label("b"), fragment((label("c"), label("d"))), label("e"))),
        label("f"),
    ));
    assert_eq!(stack.children.len(), 6);

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &stack.into(), None, noop()).unwrap();
    assert_eq!(
        child_text_contents(&r, parent),
        vec!["a", "b", "c", "d", "e", "f"],
    );
}

#[test]
fn empty_children_are_removed_during_construction() {
    let stack = vstack((
        fragment((label("a"), Element::Empty, label("b"))),
        Element::Empty,
        fragment((Element::Empty, Element::Empty)),
        label("c"),
    ));
    assert_eq!(stack.children.len(), 3);

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &stack.into(), None, noop()).unwrap();
    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c"]);
}

#[test]
fn fragment_structure_changes_preserve_keyed_controls() {
    let initial: Element = vstack((
        keyed_label("x", "X"),
        fragment((keyed_label("y", "Y"), keyed_label("z", "Z"))),
    ))
    .into();
    let updated: Element = vstack((
        fragment((keyed_label("y", "Y"),)),
        keyed_label("x", "X"),
        keyed_label("z", "Z"),
    ))
    .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &initial, None, noop()).unwrap();
    let mut initial_children = r.backend.children_of(parent).to_vec();
    let baseline = r.backend.ops.len();

    r.reconcile(Some(&initial), &updated, Some(parent), noop())
        .unwrap();

    let mut updated_children = r.backend.children_of(parent).to_vec();
    initial_children.sort_by_key(|id| id.get());
    updated_children.sort_by_key(|id| id.get());
    assert_eq!(initial_children, updated_children);
    assert!(
        !r.backend.ops[baseline..]
            .iter()
            .any(|op| matches!(op, Op::Destroy { .. }))
    );
    assert_eq!(child_text_contents(&r, parent), vec!["Y", "X", "Z"]);
}

#[test]
fn fragment_arity_changes_drive_positional_updates() {
    let initial: Element = vstack((label("a"), fragment((label("b"), label("c"))))).into();
    let updated: Element =
        vstack((label("a"), fragment((label("b"), label("c"), label("d"))))).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &initial, None, noop()).unwrap();
    r.reconcile(Some(&initial), &updated, Some(parent), noop())
        .unwrap();

    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c", "d"]);
}

#[test]
fn fragment_does_not_add_native_attach_operations() {
    let flat: Element = vstack((label("a"), label("b"), label("c"), label("d"))).into();
    let fragmented: Element =
        vstack((label("a"), fragment((label("b"), label("c"))), label("d"))).into();

    let mut flat_reconciler = Reconciler::new(RecordingBackend::new());
    flat_reconciler.reconcile(None, &flat, None, noop());
    let mut fragment_reconciler = Reconciler::new(RecordingBackend::new());
    fragment_reconciler.reconcile(None, &fragmented, None, noop());

    assert_eq!(
        appends_for(&flat_reconciler.backend.ops),
        appends_for(&fragment_reconciler.backend.ops),
    );
}

#[test]
fn fragment_accepts_mixed_widget_kinds() {
    let stack = vstack((
        label("intro"),
        fragment((Button::new("ok"), label("after"))),
    ));
    assert_eq!(stack.children.len(), 3);
}
