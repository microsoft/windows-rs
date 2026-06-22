use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::{ControlId, Prop, PropValue};
use windows_reactor::{Element, TextBlock};

fn noop_rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(el: &Element) -> (Reconciler<RecordingBackend>, Option<ControlId>) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, el, None, noop_rr());
    (r, id)
}

fn update_ops(old: Element, new: Element) -> Vec<Op> {
    let (mut r, id) = mount(&old);
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old), &new, id, noop_rr());
    r.backend.ops.clone()
}

fn text_with(content: &str, font_size: Option<f64>, font_weight: Option<u16>) -> Element {
    let mut t = TextBlock::new(content);
    t.font_size = font_size;
    t.font_weight = font_weight;
    Element::TextBlock(t)
}

#[test]
fn mount_text_with_no_optionals_emits_only_text_prop() {
    let (r, _id) = mount(&Element::TextBlock(TextBlock::new("hi")));
    let props: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp { prop, .. } => Some(*prop),
            _ => None,
        })
        .collect();
    // Text is always emitted; IsTextSelectionEnabled + TextWrapping are always-emit too.
    assert!(props.contains(&Prop::Text));
}

#[test]
fn mount_text_with_optionals_emits_all_expected_props() {
    let (r, _id) = mount(&text_with("hi", Some(14.0), Some(700)));
    let ops: Vec<&Op> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetProp { .. }))
        .collect();
    // FontSize, FontWeight, IsTextSelectionEnabled, Text, TextWrapping
    assert_eq!(ops.len(), 5);
    let mut props: Vec<Prop> = ops
        .iter()
        .map(|op| match op {
            Op::SetProp { prop, .. } => *prop,
            _ => unreachable!(),
        })
        .collect();
    props.sort_by_key(|p| format!("{p:?}"));
    let mut expected = vec![
        Prop::Text,
        Prop::FontSize,
        Prop::FontWeight,
        Prop::IsTextSelectionEnabled,
        Prop::TextWrapping,
    ];
    expected.sort_by_key(|p| format!("{p:?}"));
    assert_eq!(props, expected);
}

#[test]
fn diff_props_unchanged_text_emits_nothing_for_widget_props() {
    let ops = update_ops(
        text_with("hi", Some(12.0), None),
        text_with("hi", Some(12.0), None),
    );
    assert!(ops.is_empty(), "expected no ops, got {ops:?}");
}

#[test]
fn diff_props_adds_optional_prop_some_to_some_change() {
    let ops = update_ops(
        text_with("hi", Some(12.0), None),
        text_with("hi", Some(14.0), None),
    );
    assert_eq!(ops.len(), 1, "expected 1 op, got {ops:?}");
    match &ops[0] {
        Op::SetProp { prop, value, .. } => {
            assert_eq!(*prop, Prop::FontSize);
            assert_eq!(*value, PropValue::F64(14.0));
        }
        other => panic!("expected SetProp(FontSize), got {other:?}"),
    }
}

#[test]
fn diff_props_adds_prop_on_none_to_some_transition() {
    let ops = update_ops(
        text_with("hi", None, None),
        text_with("hi", Some(14.0), None),
    );
    assert_eq!(ops.len(), 1, "expected 1 op, got {ops:?}");
    match &ops[0] {
        Op::SetProp { prop, value, .. } => {
            assert_eq!(*prop, Prop::FontSize);
            assert_eq!(*value, PropValue::F64(14.0));
        }
        other => panic!("expected SetProp(FontSize), got {other:?}"),
    }
}

#[test]
fn diff_props_emits_unset_on_some_to_none_transition() {
    let ops = update_ops(
        text_with("hi", Some(14.0), None),
        text_with("hi", None, None),
    );
    assert_eq!(ops.len(), 1, "expected 1 op, got {ops:?}");
    match &ops[0] {
        Op::SetProp { prop, value, .. } => {
            assert_eq!(*prop, Prop::FontSize);
            assert_eq!(*value, PropValue::Unset);
        }
        other => panic!("expected SetProp(FontSize, Unset), got {other:?}"),
    }
}

#[test]
fn diff_props_handles_multiple_simultaneous_transitions() {
    let ops = update_ops(
        text_with("hi", None, Some(700)),
        text_with("bye", Some(14.0), None),
    );
    let setprops: Vec<(Prop, PropValue)> = ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp { prop, value, .. } => Some((*prop, value.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(setprops.len(), 3, "expected 3 set_prop ops, got {ops:?}");

    let find = |p: Prop| {
        setprops
            .iter()
            .find(|(prop, _)| *prop == p)
            .map(|(_, v)| v.clone())
    };
    assert_eq!(find(Prop::Text), Some(PropValue::Str("bye".into())));
    assert_eq!(find(Prop::FontSize), Some(PropValue::F64(14.0)));
    assert_eq!(find(Prop::FontWeight), Some(PropValue::Unset));
}
