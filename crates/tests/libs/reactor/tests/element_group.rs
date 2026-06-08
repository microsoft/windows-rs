//! Tests for `Element::Group` — fragment-style flattening into the parent's
//! child list during reconciliation.
//!
//! See [`docs/roadmap.md`](../../../../docs/roadmap.md) item 1a-i.

use std::rc::Rc;

use windows_reactor::core::backend::{Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::{
    Button, Element, GroupElement, Orientation, StackPanel, TextBlock, group,
};
use windows_reactor::core::reconciler::Reconciler;

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

fn appends_for(ops: &[Op]) -> Vec<(usize, usize)> {
    // Flatten append/insert into (parent_raw, child_raw) pairs in order.
    let mut out = Vec::new();
    for op in ops {
        match op {
            Op::AppendChild { parent, child } | Op::InsertChild { parent, child, .. } => {
                out.push((parent.get() as usize, child.get() as usize));
            }
            _ => {}
        }
    }
    out
}

fn child_text_contents(
    r: &Reconciler<RecordingBackend>,
    parent: windows_reactor::core::backend::ControlId,
) -> Vec<String> {
    // Walk children_of(parent) in order; for each child, find the most
    // recent SetProp(TextBlock) on that id and return its string value.
    let kids = r.backend.children_of(parent).to_vec();
    kids.into_iter()
        .map(|cid| {
            let mut last: Option<String> = None;
            for op in &r.backend.ops {
                if let Op::SetProp {
                    id,
                    prop: Prop::Text,
                    value: PropValue::Str(s),
                } = op
                    && *id == cid
                {
                    last = Some(s.clone());
                }
            }
            last.unwrap_or_default()
        })
        .collect()
}

#[test]
fn group_flattens_into_parent_child_list() {
    // <StackPanel> a, <Group>[b, c], d </StackPanel>  ⇒  StackPanel with children a, b, c, d
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            label("a"),
            Element::Group(GroupElement::new(vec![label("b"), label("c")])),
            label("d"),
        ],
        ..Default::default()
    };
    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r
        .reconcile(None, &Element::StackPanel(stack), None, noop())
        .expect("StackPanel mount produces an id");

    assert_eq!(
        child_text_contents(&r, parent),
        vec!["a", "b", "c", "d"],
        "Group should flatten into the parent's child list as if a/b/c/d were direct siblings",
    );
}

#[test]
fn nested_groups_flatten_recursively() {
    let inner = group(vec![label("c"), label("d")]);
    let outer = group(vec![label("b"), inner, label("e")]);
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![label("a"), outer, label("f")],
        ..Default::default()
    };
    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r
        .reconcile(None, &Element::StackPanel(stack), None, noop())
        .unwrap();

    assert_eq!(
        child_text_contents(&r, parent),
        vec!["a", "b", "c", "d", "e", "f"],
    );
}

#[test]
fn empty_inside_group_is_filtered() {
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            group(vec![label("a"), Element::Empty, label("b")]),
            Element::Empty,
            group(vec![Element::Empty, Element::Empty]),
            label("c"),
        ],
        ..Default::default()
    };
    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r
        .reconcile(None, &Element::StackPanel(stack), None, noop())
        .unwrap();

    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c"]);
}

#[test]
fn group_preserves_keys_across_structure_change() {
    // Initial:  StackPanel[ keyed("x"), Group[ keyed("y"), keyed("z") ] ]    → x, y, z
    // Update:   StackPanel[ Group[ keyed("y") ], keyed("x"), keyed("z") ]    → y, x, z
    //
    // The group structure changed, but the keys are preserved. Keyed
    // reconciliation should recognise the same three text controls and
    // reorder them — NOT destroy and re-create.
    let initial = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            keyed_label("x", "X"),
            group(vec![keyed_label("y", "Y"), keyed_label("z", "Z")]),
        ],
        ..Default::default()
    });
    let updated = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            group(vec![keyed_label("y", "Y")]),
            keyed_label("x", "X"),
            keyed_label("z", "Z"),
        ],
        ..Default::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &initial, None, noop()).unwrap();
    let initial_kids = r.backend.children_of(parent).to_vec();
    assert_eq!(initial_kids.len(), 3);

    // Snapshot ops, then apply the update.
    let baseline_op_count = r.backend.ops.len();
    let same = r
        .reconcile(Some(&initial), &updated, Some(parent), noop())
        .unwrap();
    assert_eq!(same, parent);

    let after_kids = r.backend.children_of(parent).to_vec();
    assert_eq!(after_kids.len(), 3, "no children added or removed");

    // Same ControlIds, just reordered — nothing was destroyed.
    let mut sorted_initial = initial_kids;
    sorted_initial.sort_by_key(|c| c.get());
    let mut sorted_after = after_kids;
    sorted_after.sort_by_key(|c| c.get());
    assert_eq!(
        sorted_initial, sorted_after,
        "control ids must be preserved"
    );

    // No Destroy ops emitted during the update.
    let new_ops = &r.backend.ops[baseline_op_count..];
    let destroyed: Vec<_> = new_ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .collect();
    assert!(
        destroyed.is_empty(),
        "structure-changing keyed reconciliation should not destroy controls; got {destroyed:?}"
    );

    // And the text content lines up with the new order.
    assert_eq!(child_text_contents(&r, parent), vec!["Y", "X", "Z"]);
}

#[test]
fn group_with_changed_arity_drives_positional_add_remove() {
    // Positional (no keys): Group with 2 children → Group with 3 children
    // should add one new child to the parent.
    let initial = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![label("a"), group(vec![label("b"), label("c")])],
        ..Default::default()
    });
    let updated = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![label("a"), group(vec![label("b"), label("c"), label("d")])],
        ..Default::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &initial, None, noop()).unwrap();
    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c"]);

    r.reconcile(Some(&initial), &updated, Some(parent), noop())
        .unwrap();
    assert_eq!(child_text_contents(&r, parent), vec!["a", "b", "c", "d"]);
}

#[test]
fn group_at_top_level_panics_with_clear_message() {
    use std::panic::AssertUnwindSafe;
    // Root Group has no parent child list to flatten into — must panic.
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let mut r = Reconciler::new(RecordingBackend::new());
        r.reconcile(
            None,
            &Element::Group(GroupElement::new(vec![label("solo")])),
            None,
            noop(),
        )
    }));
    let payload = result.expect_err("mounting Element::Group at top level must panic");
    let msg: &str = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("");
    assert!(
        msg.contains("Group can only appear inside a multi-child container"),
        "panic message should explain the misuse; got: {msg}"
    );
}

#[test]
fn group_as_border_child_panics() {
    use std::panic::AssertUnwindSafe;
    // Border has a single-child slot, which is not a "child list" the
    // group can flatten into. Must panic with the same diagnostic.
    use windows_reactor::core::element::Border;
    let border = Border::new(group(vec![label("a"), label("b")]));
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let mut r = Reconciler::new(RecordingBackend::new());
        r.reconcile(None, &Element::Border(border), None, noop())
    }));
    assert!(result.is_err(), "Group as Border child must panic");
}

#[test]
fn group_constructor_helper_matches_struct_form() {
    let from_helper = group(vec![label("a"), label("b")]);
    let from_struct = Element::Group(GroupElement {
        key: None,
        children: vec![label("a"), label("b")],
    });
    assert_eq!(from_helper, from_struct);
}

#[test]
fn group_kind_name_is_group() {
    assert_eq!(group(vec![]).kind_name(), "Group");
}

#[test]
fn group_with_key_carries_key() {
    let g = GroupElement::default().with_key("k1");
    let e: Element = g.into();
    assert_eq!(e.key(), Some("k1"));
}

// Smoke check: the existence of a Group in a child list shouldn't
// inflate the number of (parent, child) attach ops compared with the
// equivalent flat list. Documents the "fragments are zero-cost at the
// backend level" property.
#[test]
fn group_does_not_emit_extra_attach_ops() {
    let flat = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![label("a"), label("b"), label("c"), label("d")],
        ..Default::default()
    });
    let with_group = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![label("a"), group(vec![label("b"), label("c")]), label("d")],
        ..Default::default()
    });

    let mut r1 = Reconciler::new(RecordingBackend::new());
    r1.reconcile(None, &flat, None, noop());
    let mut r2 = Reconciler::new(RecordingBackend::new());
    r2.reconcile(None, &with_group, None, noop());

    assert_eq!(
        appends_for(&r1.backend.ops).len(),
        appends_for(&r2.backend.ops).len(),
        "Group should not produce additional attach ops vs. the equivalent flat list",
    );
}

// Sanity: `Button` works in a Group too — Group is widget-agnostic.
#[test]
fn group_works_with_mixed_widget_kinds() {
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            label("intro"),
            group(vec![Element::Button(Button::new("ok")), label("after")]),
        ],
        ..Default::default()
    };
    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r
        .reconcile(None, &Element::StackPanel(stack), None, noop())
        .unwrap();
    assert_eq!(r.backend.children_of(parent).len(), 3);
}
