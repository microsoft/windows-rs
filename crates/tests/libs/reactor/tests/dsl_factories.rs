use windows_reactor::core::element::{Element, Orientation};
use windows_reactor::dsl::factories::{border, button, hstack, text_block, vstack};

#[test]
fn text_block_is_bare() {
    let t = text_block("Hi");
    assert!(t.font_size.is_none());
    assert!(t.font_weight.is_none());
    assert!(t.modifiers.is_empty());
}

#[test]
fn button_default_enabled_no_click() {
    let b = button("go");
    assert!(b.is_enabled);
    assert!(b.on_click.is_none());
}

#[test]
fn button_on_click_installs_callback() {
    let b = button("go").on_click(|| {});
    assert!(b.on_click.is_some());
}

#[test]
fn button_disabled_clears_flag() {
    let b = button("go").enabled(false);
    assert!(!b.is_enabled);
}

#[test]
fn vstack_collects_children_in_order() {
    let s = vstack([text_block("a"), text_block("b")]);
    assert_eq!(s.orientation, Orientation::Vertical);
    assert_eq!(s.children.len(), 2);
    match &s.children[0] {
        Element::TextBlock(t) => assert_eq!(t.text, "a"),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn hstack_is_horizontal() {
    let s = hstack(());
    assert_eq!(s.orientation, Orientation::Horizontal);
}

#[test]
fn stack_spacing_sets_field() {
    let s = vstack(()).spacing(8.0);
    assert_eq!(s.spacing, 8.0);
}

#[test]
fn text_bold_sets_weight_700() {
    let t = text_block("a").bold();
    assert_eq!(t.font_weight, Some(700));
}

#[test]
fn border_wraps_child() {
    let b = border(text_block("inside"));
    match *b.child {
        Element::TextBlock(t) => assert_eq!(t.text, "inside"),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn nested_stacks_compose() {
    let tree = windows_reactor::vstack((
        text_block("Hello").bold().font_size(28.0),
        windows_reactor::hstack((button("-"), button("+"))).spacing(8.0),
    ));
    assert_eq!(tree.children.len(), 2);
    match &tree.children[1] {
        Element::StackPanel(s) => {
            assert_eq!(s.orientation, Orientation::Horizontal);
            assert_eq!(s.spacing, 8.0);
            assert_eq!(s.children.len(), 2);
        }
        other => panic!("unexpected {other:?}"),
    }
}
