use std::rc::Rc;

use windows_reactor::Reconciler;
use windows_reactor::{Button, Element, Orientation, StackPanel, TextBlock};
use windows_reactor::{ControlKind, Prop, PropValue};
use windows_reactor::{Op, RecordingBackend};

fn noop_request_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(
    el: Element,
) -> (
    Reconciler<RecordingBackend>,
    Option<windows_reactor::ControlId>,
) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el, None, noop_request_rerender());
    (r, id)
}

#[test]
fn mounting_text_records_create_and_set_text() {
    let (r, id) = mount(Element::TextBlock(TextBlock::new("hi")));
    let id = id.expect("TextBlock mount produces an id");

    let ops = &r.backend.ops;
    assert!(
        matches!(&ops[0], Op::Create { id: got, kind } if *got == id && *kind == ControlKind::TextBlock)
    );
    assert!(ops.iter().any(|op| matches!(op, Op::SetProp { prop: Prop::Text, value: PropValue::Str(s), .. } if s == "hi")));
}

#[test]
fn mounting_button_sets_content_and_enables_by_default() {
    let (r, id) = mount(Element::Button(Button::new("go")));
    let id = id.unwrap();

    let ops = &r.backend.ops;

    assert!(
        matches!(&ops[0], Op::Create { id: got, kind } if *got == id && *kind == ControlKind::Button)
    );
    assert!(ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            prop: Prop::Content,
            ..
        }
    )));
    assert!(ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            prop: Prop::IsEnabled,
            value: PropValue::Bool(true),
            ..
        }
    )));
}

#[test]
fn mounting_stack_appends_children_in_order() {
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::Button(Button::new("b")),
        ],
        ..StackPanel::default()
    };
    let (r, id) = mount(Element::StackPanel(stack));
    let id = id.unwrap();

    let ops = &r.backend.ops;

    assert_eq!(ops.len(), 12, "ops: {ops:#?}");

    let appends: Vec<_> = ops
        .iter()
        .filter_map(|o| match o {
            Op::AppendChild { parent, child } => Some((*parent, *child)),
            _ => None,
        })
        .collect();
    assert_eq!(appends.len(), 2);
    assert_eq!(appends[0].0, id);
    assert_eq!(appends[1].0, id);

    assert!(appends[0].1.get() < appends[1].1.get());
}

#[test]
fn mounting_empty_returns_none_and_records_nothing() {
    let (r, id) = mount(Element::Empty);
    assert!(id.is_none());
    assert!(r.backend.ops.is_empty());
}

#[test]
fn mounting_border_appends_single_child() {
    let border = windows_reactor::Border::new(Element::TextBlock(TextBlock::new("inside")));
    let (r, id) = mount(Element::Border(border));
    let id = id.unwrap();

    let appends: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|o| match o {
            Op::AppendChild { parent, child } => Some((*parent, *child)),
            _ => None,
        })
        .collect();
    assert_eq!(appends.len(), 1);
    assert_eq!(appends[0].0, id);
}

#[test]
fn nested_stacks_mount_in_tree_order() {
    let tree = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![Element::StackPanel(StackPanel {
            orientation: Orientation::Horizontal,
            children: vec![
                Element::TextBlock(TextBlock::new("a")),
                Element::TextBlock(TextBlock::new("b")),
            ],
            ..StackPanel::default()
        })],
        ..StackPanel::default()
    });
    let (r, _) = mount(tree);

    let kinds: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|o| match o {
            Op::Create { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            ControlKind::StackPanel,
            ControlKind::StackPanel,
            ControlKind::TextBlock,
            ControlKind::TextBlock
        ]
    );
}

#[test]
fn font_size_and_weight_are_set_when_present() {
    let t = TextBlock {
        text: "Heading".to_string(),
        font_size: Some(28.0),
        font_weight: Some(700),
        ..TextBlock::default()
    };
    let (r, _) = mount(Element::TextBlock(t));

    let set_props: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|o| match o {
            Op::SetProp { prop, value, .. } => Some((*prop, value.clone())),
            _ => None,
        })
        .collect();

    assert!(set_props.contains(&(Prop::Text, PropValue::Str("Heading".into()))));
    assert!(set_props.contains(&(Prop::FontSize, PropValue::F64(28.0))));
    assert!(set_props.contains(&(Prop::FontWeight, PropValue::U16(700))));
}

#[test]
fn disabled_button_emits_is_enabled_false() {
    let b = Button {
        content: "no".to_string(),
        is_enabled: false,
        ..Button::default()
    };
    let (r, _) = mount(Element::Button(b));

    assert!(r.backend.ops.iter().any(|o| matches!(
        o,
        Op::SetProp {
            prop: Prop::IsEnabled,
            value: PropValue::Bool(false),
            ..
        }
    )));
}
