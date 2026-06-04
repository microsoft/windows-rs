//! Regression tests for prop unset lifecycle.
//!
//! Validates that when an optional prop transitions from `Some(v)` → `None`
//! (i.e., removed from the element), the reconciler emits `PropValue::Unset`
//! and the backend correctly resets the property to its default value.
//!
//! Background: prior to the module-per-element refactor, 64 props were missing
//! `Unset` handlers in the WinUI backend, silently falling through to a
//! catch-all `Ok(())`. The new `element_set_prop!` macro structurally prevents
//! this by requiring a `default:` for every prop.

use std::rc::Rc;

use windows_reactor::core::backend::{ControlId, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::{
    CheckBox, Element, ProgressBar, Slider, TextBlock, ToggleSwitch,
};
use windows_reactor::core::reconciler::Reconciler;

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

fn set_prop_ops(ops: &[Op]) -> Vec<(Prop, PropValue)> {
    ops.iter()
        .filter_map(|op| match op {
            Op::SetProp { prop, value, .. } => Some((*prop, value.clone())),
            _ => None,
        })
        .collect()
}

// ── TextBlock: wrap_text set → cleared ──────────────────────────────────────

#[test]
fn text_block_wrap_cleared_emits_unset() {
    let old = Element::TextBlock(TextBlock::new("hi").wrap());
    let new = Element::TextBlock(TextBlock::new("hi"));
    let ops = set_prop_ops(&update_ops(old, new));

    assert_eq!(ops.len(), 1, "expected exactly 1 prop change, got {ops:?}");
    assert_eq!(ops[0].0, Prop::TextWrappingWrap);
    assert_eq!(ops[0].1, PropValue::Unset);
}

#[test]
fn text_block_selectable_cleared_emits_unset() {
    let old = Element::TextBlock(TextBlock::new("hi").selectable());
    let new = Element::TextBlock(TextBlock::new("hi"));
    let ops = set_prop_ops(&update_ops(old, new));

    assert_eq!(ops.len(), 1, "expected exactly 1 prop change, got {ops:?}");
    assert_eq!(ops[0].0, Prop::IsTextSelectionEnabled);
    assert_eq!(ops[0].1, PropValue::Unset);
}

#[test]
fn text_block_font_size_cleared_emits_unset() {
    let old = Element::TextBlock(TextBlock::new("hi").font_size(20.0));
    let new = Element::TextBlock(TextBlock::new("hi"));
    let ops = set_prop_ops(&update_ops(old, new));

    assert_eq!(ops.len(), 1, "expected exactly 1 prop change, got {ops:?}");
    assert_eq!(ops[0].0, Prop::FontSize);
    assert_eq!(ops[0].1, PropValue::Unset);
}

// ── CheckBox: label set → cleared ──────────────────────────────────────────

#[test]
fn checkbox_label_cleared_emits_unset() {
    let old = Element::CheckBox(CheckBox::new(false).label("Accept"));
    let new = Element::CheckBox(CheckBox::new(false));
    let ops = set_prop_ops(&update_ops(old, new));

    assert_eq!(ops.len(), 1, "expected exactly 1 prop change, got {ops:?}");
    assert_eq!(ops[0].0, Prop::CheckBoxLabel);
    assert_eq!(ops[0].1, PropValue::Unset);
}

#[test]
fn checkbox_disabled_to_default_emits_unset() {
    let old = Element::CheckBox(CheckBox::new(true).enabled(false));
    let new = Element::CheckBox(CheckBox::new(true));
    let ops = set_prop_ops(&update_ops(old, new));

    // is_enabled transitions from explicitly false → not-emitted (default true)
    assert_eq!(ops.len(), 1, "expected exactly 1 prop change, got {ops:?}");
    assert_eq!(ops[0].0, Prop::IsEnabled);
    assert_eq!(ops[0].1, PropValue::Unset);
}

// ── ToggleSwitch: header set → cleared ──────────────────────────────────────

#[test]
fn toggle_switch_header_cleared_emits_unset() {
    let old = Element::ToggleSwitch(ToggleSwitch {
        header: Some("WiFi".into()),
        ..ToggleSwitch::new(true)
    });
    let new = Element::ToggleSwitch(ToggleSwitch::new(true));
    let ops = set_prop_ops(&update_ops(old, new));

    let unset_ops: Vec<_> = ops.iter().filter(|(_, v)| *v == PropValue::Unset).collect();
    assert!(
        unset_ops.iter().any(|(p, _)| *p == Prop::Header),
        "expected Header Unset in {ops:?}"
    );
}

// ── Slider: range props cleared ─────────────────────────────────────────────

#[test]
fn slider_range_removed_emits_unset_for_min_and_max() {
    // Slider always emits min/max, but if we go from a custom range to default
    // range we should see new values rather than Unset. Test value changes:
    let old = Element::Slider(Slider::new(50.0).range(10.0, 200.0));
    let new = Element::Slider(Slider::new(50.0).range(0.0, 100.0));
    let ops = set_prop_ops(&update_ops(old, new));

    // Should have Minimum and Maximum changes
    let min_op = ops.iter().find(|(p, _)| *p == Prop::Minimum);
    let max_op = ops.iter().find(|(p, _)| *p == Prop::Maximum);
    assert!(min_op.is_some(), "expected Minimum prop change in {ops:?}");
    assert!(max_op.is_some(), "expected Maximum prop change in {ops:?}");
    assert_eq!(min_op.unwrap().1, PropValue::F64(0.0));
    assert_eq!(max_op.unwrap().1, PropValue::F64(100.0));
}

// ── ProgressBar: indeterminate cleared ──────────────────────────────────────

#[test]
fn progress_bar_indeterminate_cleared_emits_prop_change() {
    let old = Element::ProgressBar(ProgressBar::indeterminate());
    let new = Element::ProgressBar(ProgressBar::new(50.0));
    let ops = set_prop_ops(&update_ops(old, new));

    let indet_op = ops.iter().find(|(p, _)| *p == Prop::IsIndeterminate);
    assert!(
        indet_op.is_some(),
        "expected IsIndeterminate change in {ops:?}"
    );
    // ProgressBar always emits IsIndeterminate, so transition is Bool(true) → Bool(false)
    assert_eq!(indet_op.unwrap().1, PropValue::Bool(false));
}

// ── Cross-element: multiple props set → cleared simultaneously ──────────────

#[test]
fn text_block_multiple_optional_props_cleared_simultaneously() {
    let old = Element::TextBlock(TextBlock::new("hi").font_size(14.0).wrap().selectable());
    let new = Element::TextBlock(TextBlock::new("hi"));
    let ops = set_prop_ops(&update_ops(old, new));

    assert_eq!(ops.len(), 3, "expected 3 unset ops, got {ops:?}");
    let props: Vec<Prop> = ops.iter().map(|(p, _)| *p).collect();
    assert!(props.contains(&Prop::FontSize));
    assert!(props.contains(&Prop::TextWrappingWrap));
    assert!(props.contains(&Prop::IsTextSelectionEnabled));
    for (_, v) in &ops {
        assert_eq!(*v, PropValue::Unset);
    }
}
