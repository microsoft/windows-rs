//! Coverage for the W1–W5 widgets added per `docs/gaps.md`:
//!
//! * **W1**: `virtual_list` constructor and `scroll_templated_to_index`
//! * **W2**: `PasswordBox`
//! * **W3**: `RadioButtons`
//! * **W4**: `ComboBox`
//! * **W5**: `Canvas` element + `canvas_left` / `canvas_top` attached props
//!
//! All assertions are made against `RecordingBackend` so they execute on
//! any platform.

use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::ElementExt;
use windows_reactor::core::backend::{
    Backend, ControlKind, Event, Op, Prop, PropValue, RecordingBackend,
};
use windows_reactor::core::element::{
    Canvas, CanvasPosition, ComboBox, PasswordBox, PasswordRevealMode, RadioButtons,
};
use windows_reactor::core::element::{Element, TextBlock};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::templated_list::{list_view, virtual_list};

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

fn first_create(
    r: &Reconciler<RecordingBackend>,
) -> (ControlKind, windows_reactor::core::backend::ControlId) {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create { id, kind } => Some((*kind, *id)),
            _ => None,
        })
        .expect("no Create op")
}

// ── W2: PasswordBox ─────────────────────────────────────────────────────

#[test]
fn password_box_mounts_with_value_header_and_reveal_mode() {
    let el: Element = PasswordBox::new()
        .value("hunter2")
        .header("Secret")
        .placeholder_text("type something")
        .password_reveal_mode(PasswordRevealMode::Visible)
        .into();
    let r = mount(&el);
    let (kind, id) = first_create(&r);
    assert_eq!(kind, ControlKind::PasswordBox);

    let mut value_set = false;
    let mut reveal_set = false;
    let mut header_set = false;
    let mut placeholder_set = false;
    for op in &r.backend.ops {
        if let Op::SetProp {
            id: oid,
            prop,
            value,
        } = op
        {
            if *oid != id {
                continue;
            }
            match (prop, value) {
                (Prop::Value, PropValue::Str(s)) if s == "hunter2" => value_set = true,
                (
                    Prop::PasswordRevealMode,
                    PropValue::PasswordRevealMode(PasswordRevealMode::Visible),
                ) => {
                    reveal_set = true;
                }
                (Prop::Header, PropValue::Str(s)) if s == "Secret" => header_set = true,
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "type something" => {
                    placeholder_set = true;
                }
                _ => {}
            }
        }
    }
    assert!(value_set, "missing PasswordValue=hunter2");
    assert!(reveal_set, "missing PasswordRevealMode=Visible");
    assert!(header_set, "missing Header=Secret");
    assert!(placeholder_set, "missing Placeholder");
}

#[test]
fn password_box_attaches_password_changed_event() {
    let captured = Rc::new(Cell::new(None::<String>));
    let cap = Rc::clone(&captured);
    let el: Element = PasswordBox::new()
        .on_password_changed(move |s| cap.set(Some(s)))
        .into();
    let r = mount(&el);
    let (_, id) = first_create(&r);
    r.backend
        .fire_string(id, Event::PasswordChanged, "abc".into());
    assert_eq!(captured.take().as_deref(), Some("abc"));
}

// ── W3: RadioButtons ────────────────────────────────────────────────────

#[test]
fn radio_buttons_mounts_with_items_and_selection() {
    let el: Element = RadioButtons::new(["A", "B", "C"])
        .selected_index(1)
        .header("Pick")
        .max_columns(2)
        .into();
    let r = mount(&el);
    let (kind, id) = first_create(&r);
    assert_eq!(kind, ControlKind::RadioButtons);

    let mut items_ok = false;
    let mut sel_ok = false;
    let mut header_ok = false;
    let mut maxcols_ok = false;
    for op in &r.backend.ops {
        if let Op::SetProp {
            id: oid,
            prop,
            value,
        } = op
        {
            if *oid != id {
                continue;
            }
            match (prop, value) {
                (Prop::Items, PropValue::StrList(v)) if v == &["A", "B", "C"] => {
                    items_ok = true;
                }
                (Prop::SelectedIndex, PropValue::I32(1)) => sel_ok = true,
                (Prop::Header, PropValue::Str(s)) if s == "Pick" => header_ok = true,
                (Prop::MaxColumns, PropValue::I32(2)) => maxcols_ok = true,
                _ => {}
            }
        }
    }
    assert!(items_ok && sel_ok && header_ok && maxcols_ok);
}

#[test]
fn radio_buttons_fires_selection_changed() {
    let chosen = Rc::new(Cell::new(-1));
    let c = Rc::clone(&chosen);
    let el: Element = RadioButtons::new(["A", "B"])
        .on_selection_changed(move |i| c.set(i))
        .into();
    let r = mount(&el);
    let (_, id) = first_create(&r);
    r.backend.fire_i32(id, Event::SelectionChanged, 1);
    assert_eq!(chosen.get(), 1);
}

// ── W4: ComboBox ────────────────────────────────────────────────────────

#[test]
fn combo_box_mounts_with_items_placeholder_and_selection() {
    let el: Element = ComboBox::new(["Red", "Green", "Blue"])
        .selected_index(2)
        .placeholder_text("color")
        .header("Pick a color")
        .into();
    let r = mount(&el);
    let (kind, id) = first_create(&r);
    assert_eq!(kind, ControlKind::ComboBox);

    let mut items_ok = false;
    let mut sel_ok = false;
    let mut placeholder_ok = false;
    let mut header_ok = false;
    for op in &r.backend.ops {
        if let Op::SetProp {
            id: oid,
            prop,
            value,
        } = op
        {
            if *oid != id {
                continue;
            }
            match (prop, value) {
                (Prop::Items, PropValue::StrList(v)) if v == &["Red", "Green", "Blue"] => {
                    items_ok = true;
                }
                (Prop::SelectedIndex, PropValue::I32(2)) => sel_ok = true,
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "color" => placeholder_ok = true,
                (Prop::Header, PropValue::Str(s)) if s == "Pick a color" => header_ok = true,
                _ => {}
            }
        }
    }
    assert!(items_ok && sel_ok && placeholder_ok && header_ok);
}

#[test]
fn combo_box_disabled_emits_is_enabled_false() {
    let el: Element = ComboBox::new(["x"]).enabled(false).into();
    let r = mount(&el);
    let saw = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::IsEnabled,
                value: PropValue::Bool(false),
                ..
            }
        )
    });
    assert!(saw);
}

#[test]
fn combo_box_fires_selection_changed() {
    let chosen = Rc::new(Cell::new(-1));
    let c = Rc::clone(&chosen);
    let el: Element = ComboBox::new(["A", "B"])
        .on_selection_changed(move |i| c.set(i))
        .into();
    let r = mount(&el);
    let (_, id) = first_create(&r);
    r.backend.fire_i32(id, Event::SelectionChanged, 1);
    assert_eq!(chosen.get(), 1);
}

// ── W5: Canvas + attached position ─────────────────────────────────────

#[test]
fn canvas_mounts_children() {
    let el: Element = Canvas::new([TextBlock::new("a"), TextBlock::new("b")]).into();
    let r = mount(&el);
    let canvas_id = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create {
                id,
                kind: ControlKind::Canvas,
            } => Some(*id),
            _ => None,
        })
        .expect("Canvas Create");
    let appends: usize = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::AppendChild { parent, .. } if *parent == canvas_id))
        .count();
    assert_eq!(appends, 2);
}

#[test]
fn canvas_position_round_trips_attached_props() {
    let placed: Element = TextBlock::new("x")
        .canvas_left(40.0)
        .canvas_top(80.0)
        .into();
    let attached = placed.attached().expect("attached");
    let p: &CanvasPosition = attached.get().expect("CanvasPosition");
    assert_eq!(p.left, 40.0);
    assert_eq!(p.top, 80.0);
}

#[test]
fn canvas_position_emits_attached_set_props_on_mount() {
    let el: Element = Canvas::new([TextBlock::new("x").canvas_left(40.0).canvas_top(80.0)]).into();
    let r = mount(&el);
    let mut left_ok = false;
    let mut top_ok = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::AttachedCanvasLeft, PropValue::F64(v)) if (*v - 40.0).abs() < 1e-9 => {
                    left_ok = true;
                }
                (Prop::AttachedCanvasTop, PropValue::F64(v)) if (*v - 80.0).abs() < 1e-9 => {
                    top_ok = true;
                }
                _ => {}
            }
        }
    }
    assert!(left_ok, "missing AttachedCanvasLeft=40.0");
    assert!(top_ok, "missing AttachedCanvasTop=80.0");
}

// ── W1: virtual_list alias + scroll_to_index plumbing ──────────────────

#[test]
fn virtual_list_alias_produces_templated_list() {
    let e = virtual_list(vec![1i32, 2, 3], |n, _| TextBlock::new(n.to_string())).build();
    matches!(&e, Element::TemplatedList(_));
    // Same shape as list_view so consumers can swap freely.
    let e2 = list_view(vec![1i32, 2, 3], |n, _| TextBlock::new(n.to_string())).build();
    assert_eq!(std::mem::discriminant(&e), std::mem::discriminant(&e2));
}

#[test]
fn scroll_templated_to_index_records_op_on_recording_backend() {
    let mut rb = RecordingBackend::new();
    let id = rb.create(ControlKind::ListView);
    rb.scroll_templated_to_index(id, 7);
    let saw = rb
        .ops
        .iter()
        .any(|op| matches!(op, Op::ScrollTemplatedToIndex { id: oid, index: 7 } if *oid == id));
    assert!(saw);
}
