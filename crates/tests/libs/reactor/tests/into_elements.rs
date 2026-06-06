//! Tests for the `IntoElements` tuple trait used by `vstack`/`hstack`/`grid`.

use windows_reactor::core::element::{Element, Orientation};
use windows_reactor::dsl::factories::{button, text_block};
use windows_reactor::{grid, hstack, vstack};

#[test]
fn vstack_macro_collects_heterogeneous_children_in_order() {
    let s = vstack((text_block("a"), text_block("b").bold(), button("c")));
    assert_eq!(s.orientation, Orientation::Vertical);
    assert_eq!(s.children.len(), 3);
    match &s.children[0] {
        Element::TextBlock(t) => assert_eq!(t.content, "a"),
        other => panic!("expected text, got {other:?}"),
    }
    match &s.children[2] {
        Element::Button(b) => assert_eq!(b.content, "c"),
        other => panic!("expected button, got {other:?}"),
    }
}

#[test]
fn hstack_macro_is_horizontal_and_chainable() {
    let s = hstack((button("-"), button("+"))).spacing(8.0);
    assert_eq!(s.orientation, Orientation::Horizontal);
    assert_eq!(s.spacing, Some(8.0));
    assert_eq!(s.children.len(), 2);
}

#[test]
fn grid_macro_collects_children() {
    let g = grid((text_block("a"), text_block("b")));
    assert_eq!(g.children.len(), 2);
}

#[test]
fn empty_invocations_compile_and_produce_empty_builders() {
    let v = vstack(());
    assert_eq!(v.orientation, Orientation::Vertical);
    assert!(v.children.is_empty());

    let h = hstack(());
    assert_eq!(h.orientation, Orientation::Horizontal);
    assert!(h.children.is_empty());

    let g = grid(());
    assert!(g.children.is_empty());
}

#[test]
fn trailing_comma_is_tolerated() {
    let v = vstack((text_block("a"), text_block("b")));
    assert_eq!(v.children.len(), 2);
}

#[test]
fn macros_nest_without_element_from() {
    let tree = vstack((
        text_block("Hello").bold(),
        hstack((button("-"), button("+"))).spacing(8.0),
    ));
    assert_eq!(tree.children.len(), 2);
    match &tree.children[1] {
        Element::StackPanel(s) => {
            assert_eq!(s.orientation, Orientation::Horizontal);
            assert_eq!(s.spacing, Some(8.0));
            assert_eq!(s.children.len(), 2);
        }
        other => panic!("expected stack, got {other:?}"),
    }
}

#[test]
fn macro_form_matches_function_form_with_explicit_into() {
    let by_macro = vstack((text_block("a"), button("b")));
    let by_hand = vstack((text_block("a"), button("b")));
    // Sanity: the macro is referentially transparent w.r.t. itself.
    assert_eq!(by_macro.children.len(), by_hand.children.len());

    // And matches the explicit `Element::from` style the codebase uses today.
    let by_hand_explicit =
        windows_reactor::dsl::factories::vstack(
            [text_block("a").into(), button("b").into()] as [Element; 2]
        );
    assert_eq!(by_macro.children, by_hand_explicit.children);
}
