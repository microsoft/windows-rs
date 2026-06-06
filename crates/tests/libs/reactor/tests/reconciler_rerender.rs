use std::rc::Rc;

use windows_reactor::core::backend::{ControlId, Event, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::callback::Callback;
use windows_reactor::core::element::ToggleSwitch;
use windows_reactor::core::element::{
    Border, Button, CheckBox, Color, Element, Modifiers, ScrollViewer, StackPanel, TextBlock,
    TextBox, Thickness,
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

fn update_ops(old: Element, new: Element) -> (Reconciler<RecordingBackend>, Vec<Op>) {
    let (mut r, id) = mount(&old);
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old), &new, id, noop_rr());
    let ops = r.backend.ops.clone();
    (r, ops)
}

#[test]
fn text_content_change_emits_single_set_prop() {
    let old = Element::TextBlock(TextBlock::new("hi"));
    let new = Element::TextBlock(TextBlock::new("bye"));
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1, "expected exactly one op, got {ops:?}");
    match &ops[0] {
        Op::SetProp { prop, value, .. } => {
            assert_eq!(*prop, Prop::Text);
            assert_eq!(*value, PropValue::Str("bye".into()));
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn identical_text_update_records_nothing_via_skip_path() {
    let old = Element::TextBlock(TextBlock::new("hi"));
    let new = Element::TextBlock(TextBlock::new("hi"));
    let (r, ops) = update_ops(old, new);

    assert!(ops.is_empty(), "expected skip path; got {ops:?}");
    assert_eq!(r.debug_elements_skipped, 1);
}

#[test]
fn kind_change_destroys_old_and_mounts_new() {
    let old = Element::TextBlock(TextBlock::new("hi"));
    let new = Element::Button(Button::new("hi"));
    let (_, ops) = update_ops(old, new);

    assert!(matches!(ops[0], Op::Destroy { .. }), "got {:?}", ops[0]);
    assert!(matches!(
        ops[1],
        Op::Create {
            kind: windows_reactor::core::backend::ControlKind::Button,
            ..
        }
    ));
    assert!(matches!(
        ops[2],
        Op::SetProp {
            prop: Prop::Content,
            ..
        }
    ));
}

#[test]
fn font_weight_unset_emits_unset_prop_value() {
    let old = Element::TextBlock(TextBlock {
        content: "x".into(),
        font_weight: Some(700),
        ..TextBlock::default()
    });
    let new = Element::TextBlock(TextBlock {
        content: "x".into(),
        font_weight: None,
        ..TextBlock::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1);
    assert!(matches!(
        ops[0],
        Op::SetProp {
            prop: Prop::FontWeight,
            value: PropValue::Unset,
            ..
        }
    ));
}

#[test]
fn button_label_change_does_not_reattach_unchanged_click_handler() {
    let cb = Callback::<()>::new(|()| {});
    let old = Element::Button(Button {
        content: "a".into(),
        is_enabled: true,
        on_click: Some(cb.clone()),
        ..Button::default()
    });
    let new = Element::Button(Button {
        content: "b".into(),
        is_enabled: true,
        on_click: Some(cb),
        ..Button::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(!ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
    assert!(ops.iter().any(|o| matches!(
        o,
        Op::SetProp {
            prop: Prop::Content,
            ..
        }
    )));
}

#[test]
fn button_click_handler_swap_re_attaches_event() {
    let cb_a = Callback::<()>::new(|()| {});
    let cb_b = Callback::<()>::new(|()| {});
    let old = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb_a),
        ..Button::default()
    });
    let new = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb_b),
        ..Button::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1, "ops: {ops:?}");
    assert!(matches!(ops[0], Op::AttachEvent { .. }));
}

#[test]
fn stack_orientation_change_emits_single_set_prop() {
    let old = Element::StackPanel(StackPanel {
        vertical: true,
        ..StackPanel::default()
    });
    let new = Element::StackPanel(StackPanel {
        vertical: false,
        ..StackPanel::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1);
    assert!(matches!(
        ops[0],
        Op::SetProp {
            prop: Prop::Orientation,
            value: PropValue::Vertical(false),
            ..
        }
    ));
}

#[test]
fn modifiers_diff_only_emits_for_changed_fields() {
    let old = Element::TextBlock(TextBlock {
        content: "x".into(),
        modifiers: Modifiers {
            margin: Some(Thickness::uniform(5.0)),
            opacity: Some(1.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let new = Element::TextBlock(TextBlock {
        content: "x".into(),
        modifiers: Modifiers {
            margin: Some(Thickness::uniform(10.0)),
            opacity: Some(1.0),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1, "expected only margin change");
    assert!(matches!(
        ops[0],
        Op::SetProp {
            prop: Prop::Margin,
            ..
        }
    ));
}

#[test]
fn modifier_some_to_none_emits_unset() {
    let old = Element::TextBlock(TextBlock {
        content: "x".into(),
        modifiers: Modifiers {
            background: Some(Color::rgb(255, 0, 0).into()),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let new = Element::TextBlock(TextBlock {
        content: "x".into(),
        modifiers: Modifiers::default(),
        ..TextBlock::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1);
    assert!(matches!(
        ops[0],
        Op::SetProp {
            prop: Prop::Background,
            value: PropValue::Unset,
            ..
        }
    ));
}

fn keyed_text(key: &str, content: &str) -> Element {
    Element::TextBlock(TextBlock {
        key: Some(key.into()),
        content: content.into(),
        ..TextBlock::default()
    })
}

fn scrollview_with(child: Element) -> Element {
    Element::ScrollViewer(ScrollViewer::new(child))
}

fn border_with(child: Element) -> Element {
    Element::Border(Border::new(child))
}

fn structural_ops(ops: &[Op]) -> Vec<&Op> {
    ops.iter()
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
        .collect()
}

#[test]
fn scrollview_child_key_change_uses_positional_replace() {
    let old = scrollview_with(keyed_text("a", "hi"));
    let new = scrollview_with(keyed_text("b", "bye"));
    let (_, ops) = update_ops(old, new);

    let structural = structural_ops(&ops);
    assert!(
        structural.is_empty(),
        "expected no structural ops on the positional path, got {structural:?}"
    );
    assert!(
        ops.iter().any(|op| matches!(
            op,
            Op::SetProp {
                prop: Prop::Text,
                value: PropValue::Str(s),
                ..
            } if s == "bye"
        )),
        "expected a SetProp(TextBlock, \"bye\") in-place update, got {ops:?}"
    );
}

#[test]
fn scrollview_child_swap_matches_border_child_swap() {
    let (_, sv_ops) = update_ops(
        scrollview_with(Element::TextBlock(TextBlock::new("a"))),
        scrollview_with(Element::Button(Button::new("b"))),
    );
    let (_, b_ops) = update_ops(
        border_with(Element::TextBlock(TextBlock::new("a"))),
        border_with(Element::Button(Button::new("b"))),
    );

    let sv_structural: Vec<_> = structural_ops(&sv_ops)
        .into_iter()
        .map(std::mem::discriminant)
        .collect();
    let b_structural: Vec<_> = structural_ops(&b_ops)
        .into_iter()
        .map(std::mem::discriminant)
        .collect();

    assert_eq!(
        sv_structural, b_structural,
        "ScrollViewer and Border should produce the same structural op kinds; \
         sv_ops={sv_ops:?} b_ops={b_ops:?}"
    );
}

#[test]
fn button_click_handler_some_to_none_emits_detach() {
    let cb = Callback::<()>::new(|()| {});
    let old = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb),
        ..Button::default()
    });
    let new = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: None,
        ..Button::default()
    });
    let (_, ops) = update_ops(old, new);

    assert_eq!(ops.len(), 1, "expected only a detach, got {ops:?}");
    match &ops[0] {
        Op::DetachEvent { event, .. } => assert_eq!(*event, Event::Click),
        other => panic!("expected DetachEvent, got {other:?}"),
    }
    assert!(!ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
}

#[test]
fn button_click_handler_none_to_some_emits_attach_only() {
    let cb = Callback::<()>::new(|()| {});
    let old = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: None,
        ..Button::default()
    });
    let new = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb),
        ..Button::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(!ops.iter().any(|o| matches!(o, Op::DetachEvent { .. })));
    assert!(ops.iter().any(|o| matches!(
        o,
        Op::AttachEvent {
            event: Event::Click,
            ..
        }
    )));
}

#[test]
fn checkbox_changed_handler_some_to_none_emits_detach() {
    let cb = Callback::<bool>::new(|_| {});
    let old = Element::CheckBox(CheckBox {
        is_checked: false,
        is_enabled: true,
        on_changed: Some(cb),
        ..CheckBox::default()
    });
    let new = Element::CheckBox(CheckBox {
        is_checked: false,
        is_enabled: true,
        on_changed: None,
        ..CheckBox::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(ops.iter().any(|o| matches!(
        o,
        Op::DetachEvent {
            event: Event::Checked,
            ..
        }
    )));
    assert!(!ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
}

#[test]
fn textfield_changed_handler_some_to_none_emits_detach() {
    let cb = Callback::<String>::new(|_| {});
    let old = Element::TextBox(TextBox {
        value: "hi".into(),
        is_enabled: true,
        on_changed: Some(cb),
        ..TextBox::default()
    });
    let new = Element::TextBox(TextBox {
        value: "hi".into(),
        is_enabled: true,
        on_changed: None,
        ..TextBox::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(ops.iter().any(|o| matches!(
        o,
        Op::DetachEvent {
            event: Event::TextChanged,
            ..
        }
    )));
    assert!(!ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
}

#[test]
fn toggle_switch_changed_handler_some_to_none_emits_detach() {
    let cb = Callback::<bool>::new(|_| {});
    let old = Element::ToggleSwitch(ToggleSwitch {
        is_on: false,
        is_enabled: true,
        on_changed: Some(cb),
        ..ToggleSwitch::default()
    });
    let new = Element::ToggleSwitch(ToggleSwitch {
        is_on: false,
        is_enabled: true,
        on_changed: None,
        ..ToggleSwitch::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(ops.iter().any(|o| matches!(
        o,
        Op::DetachEvent {
            event: Event::Toggled,
            ..
        }
    )));
    assert!(!ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
}

#[test]
fn handler_swap_emits_attach_without_detach() {
    let cb_a = Callback::<()>::new(|()| {});
    let cb_b = Callback::<()>::new(|()| {});
    let old = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb_a),
        ..Button::default()
    });
    let new = Element::Button(Button {
        content: "x".into(),
        is_enabled: true,
        on_click: Some(cb_b),
        ..Button::default()
    });
    let (_, ops) = update_ops(old, new);

    assert!(!ops.iter().any(|o| matches!(o, Op::DetachEvent { .. })));
    assert!(ops.iter().any(|o| matches!(o, Op::AttachEvent { .. })));
}
