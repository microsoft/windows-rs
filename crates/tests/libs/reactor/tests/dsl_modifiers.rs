use windows_reactor::{Element, HorizontalAlignment, Thickness};
use windows_reactor::{ElementExt, LayoutExt, VisualExt};
use windows_reactor::{button, text_block, vstack};

#[test]
fn margin_chains_on_concrete_builder() {
    let t = text_block("hi").margin(Thickness::uniform(10.0));
    assert_eq!(t.modifiers.margin, Some(Thickness::uniform(10.0)));
}

#[test]
fn multiple_modifiers_set_independently() {
    let b = button("go")
        .margin(4.0)
        .width(100.0)
        .opacity(0.5)
        .horizontal_alignment(HorizontalAlignment::Center);
    assert_eq!(b.modifiers.margin, Some(Thickness::uniform(4.0)));
    assert_eq!(b.modifiers.width, Some(100.0));
    assert_eq!(b.modifiers.opacity, Some(0.5));
    assert_eq!(
        b.modifiers.horizontal_alignment,
        Some(HorizontalAlignment::Center)
    );

    assert!(b.modifiers.height.is_none());
    assert!(b.modifiers.padding.is_none());
}

#[test]
fn vstack_chains_spacing_and_margin() {
    let s = vstack(()).spacing(8.0).margin(10.0);
    assert_eq!(s.spacing, 8.0);
    assert_eq!(s.modifiers.margin, Some(Thickness::uniform(10.0)));
}

#[test]
fn with_key_sets_on_concrete_builder() {
    let t = text_block("row").with_key("row-1");
    assert_eq!(t.key.as_deref(), Some("row-1"));
}

#[test]
fn with_key_sets_on_element_blanket() {
    let e: Element = text_block("row").into();
    let e = e.with_key("row-x");
    assert_eq!(e.key(), Some("row-x"));
}
