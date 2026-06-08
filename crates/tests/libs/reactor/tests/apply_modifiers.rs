use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::{
    Color, Element, HorizontalAlignment, Modifiers, TextBlock, Thickness,
};
use windows_reactor::core::reconciler::Reconciler;

fn rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount_text_with_modifiers(mods: Modifiers) -> (Reconciler<RecordingBackend>, Vec<Op>) {
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: mods,
        ..TextBlock::default()
    });
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, rr());
    let ops = r.backend.ops.clone();
    (r, ops)
}

#[test]
fn all_none_modifiers_emit_no_set_props_for_shared_fields() {
    let (_, ops) = mount_text_with_modifiers(Modifiers::default());

    let shared_sets = ops.iter().filter(|o| {
        matches!(
            o,
            Op::SetProp {
                prop: Prop::Margin,
                ..
            } | Op::SetProp {
                prop: Prop::Padding,
                ..
            } | Op::SetProp {
                prop: Prop::Width,
                ..
            } | Op::SetProp {
                prop: Prop::Height,
                ..
            } | Op::SetProp {
                prop: Prop::HorizontalAlignment,
                ..
            } | Op::SetProp {
                prop: Prop::VerticalAlignment,
                ..
            } | Op::SetProp {
                prop: Prop::Opacity,
                ..
            } | Op::SetProp {
                prop: Prop::Background,
                ..
            } | Op::SetProp {
                prop: Prop::Foreground,
                ..
            }
        )
    });
    assert_eq!(shared_sets.count(), 0);
}

#[test]
fn margin_some_emits_single_margin_set() {
    let mods = Modifiers {
        margin: Some(Thickness::uniform(10.0)),
        ..Modifiers::default()
    };
    let (_, ops) = mount_text_with_modifiers(mods);

    let margin_sets: Vec<_> = ops
        .iter()
        .filter_map(|o| match o {
            Op::SetProp {
                prop: Prop::Margin,
                value,
                ..
            } => Some(value.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(margin_sets.len(), 1);
    assert_eq!(
        margin_sets[0],
        PropValue::Thickness(Thickness::uniform(10.0))
    );
}

#[test]
fn diff_modifiers_identical_emits_no_ops() {
    let mods = Modifiers {
        margin: Some(Thickness::uniform(5.0)),
        width: Some(200.0),
        ..Modifiers::default()
    };
    let el = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: mods,
        ..TextBlock::default()
    });
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el, None, rr()).unwrap();
    r.backend.clear_ops();

    let old = el.clone();
    let new = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            margin: Some(Thickness::uniform(5.0)),
            width: Some(200.0),
            ..Modifiers::default()
        },
        font_size: Some(16.0),
        ..TextBlock::default()
    });
    let _ = r.reconcile(Some(&old), &new, Some(id), rr());

    assert_eq!(r.backend.ops.len(), 1, "ops: {:?}", r.backend.ops);
    assert!(matches!(
        r.backend.ops[0],
        Op::SetProp {
            prop: Prop::FontSize,
            ..
        }
    ));
}

#[test]
fn mount_emits_modifiers_after_create() {
    let mods = Modifiers {
        opacity: Some(0.5),
        background: Some(Color::rgb(255, 0, 0).into()),
        horizontal_alignment: Some(HorizontalAlignment::Center),
        ..Modifiers::default()
    };
    let (_, ops) = mount_text_with_modifiers(mods);

    assert!(matches!(
        ops[0],
        Op::Create {
            kind: ControlKind::TextBlock,
            ..
        }
    ));
    let modifier_props: Vec<_> = ops
        .iter()
        .filter_map(|o| match o {
            Op::SetProp { prop, .. }
                if matches!(
                    prop,
                    Prop::Opacity | Prop::Background | Prop::HorizontalAlignment
                ) =>
            {
                Some(*prop)
            }
            _ => None,
        })
        .collect();
    assert_eq!(modifier_props.len(), 3);

    assert_eq!(modifier_props[0], Prop::HorizontalAlignment);
    assert_eq!(modifier_props[1], Prop::Opacity);
    assert_eq!(modifier_props[2], Prop::Background);
}

