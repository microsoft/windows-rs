//! Tests for the `try_universal_prop` dispatch and related helpers that were
//! extracted from per-control match arms during the backend consolidation.
//! Covers: min/max sizing, canvas z_index, relative panel attached props,
//! and is_enabled unset.

use std::rc::Rc;

use windows_reactor::imp::Reconciler;
use windows_reactor::imp::{Op, RecordingBackend};
use windows_reactor::{Button, Element, Modifiers, StackPanel, TextBlock};
use windows_reactor::{Prop, PropValue};

fn rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, rr());
    r
}

fn props_for(r: &Reconciler<RecordingBackend>, prop: Prop) -> Vec<PropValue> {
    r.backend
        .ops
        .iter()
        .filter_map(|o| match o {
            Op::SetProp { prop: p, value, .. } if *p == prop => Some(value.clone()),
            _ => None,
        })
        .collect()
}

// ── Min/Max sizing ──────────────────────────────────────────────────────

#[test]
fn min_width_emits_set_prop() {
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            min_width: Some(100.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let r = mount(&el);
    let vals = props_for(&r, Prop::MinWidth);
    assert_eq!(vals, vec![PropValue::F64(100.0)]);
}

#[test]
fn max_width_emits_set_prop() {
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            max_width: Some(500.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let r = mount(&el);
    let vals = props_for(&r, Prop::MaxWidth);
    assert_eq!(vals, vec![PropValue::F64(500.0)]);
}

#[test]
fn min_height_emits_set_prop() {
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            min_height: Some(50.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let r = mount(&el);
    let vals = props_for(&r, Prop::MinHeight);
    assert_eq!(vals, vec![PropValue::F64(50.0)]);
}

#[test]
fn max_height_emits_set_prop() {
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            max_height: Some(800.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let r = mount(&el);
    let vals = props_for(&r, Prop::MaxHeight);
    assert_eq!(vals, vec![PropValue::F64(800.0)]);
}

#[test]
fn min_max_unset_on_diff() {
    let el1 = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            min_width: Some(100.0),
            max_height: Some(800.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let el2 = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers::default(),
        ..TextBlock::default()
    });
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el1, None, rr()).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&el1), &el2, Some(id), rr());

    let min_w = props_for(&r, Prop::MinWidth);
    let max_h = props_for(&r, Prop::MaxHeight);
    assert_eq!(min_w, vec![PropValue::Unset]);
    assert_eq!(max_h, vec![PropValue::Unset]);
}

// ── Canvas ZIndex ───────────────────────────────────────────────────────

#[test]
fn canvas_z_index_emits_set_prop() {
    use windows_reactor::Canvas;

    let child: Element = TextBlock::new("hi").into();
    let child = child.canvas_z_index(5);
    let el: Element = Canvas::new([child]).into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AttachedCanvasZIndex);
    assert_eq!(vals, vec![PropValue::I32(5)]);
}

// ── RelativePanel attached props ────────────────────────────────────────
// The relative_align_* methods store RelativePanelAlignment in the element's
// attached props. The reconciler dispatches them regardless of parent type.

#[test]
fn relative_panel_align_left_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_left();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignLeftWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

#[test]
fn relative_panel_align_right_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_right();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignRightWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

#[test]
fn relative_panel_align_top_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_top();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignTopWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

#[test]
fn relative_panel_align_bottom_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_bottom();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignBottomWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

#[test]
fn relative_panel_align_h_center_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_h_center();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignHCenterWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

#[test]
fn relative_panel_align_v_center_emits_set_prop() {
    let child: Element = TextBlock::new("hi").into();
    let child = child.relative_align_v_center();
    let el: Element = StackPanel {
        children: vec![child],
        ..StackPanel::vertical()
    }
    .into();
    let r = mount(&el);
    let vals = props_for(&r, Prop::AlignVCenterWithPanel);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}

// ── IsEnabled transition ─────────────────────────────────────────────────

#[test]
fn is_enabled_false_to_true_emits_set_prop() {
    let el1: Element = Button {
        is_enabled: false,
        ..Button::new("x")
    }
    .into();
    let el2: Element = Button::new("x").into();
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el1, None, rr()).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&el1), &el2, Some(id), rr());

    let vals = props_for(&r, Prop::IsEnabled);
    assert_eq!(vals, vec![PropValue::Bool(true)]);
}
