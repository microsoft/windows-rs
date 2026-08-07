use windows_reactor::{BackgroundExt, ElementExt, LayoutExt, PaddingExt, TextStyleExt, VisualExt};
use windows_reactor::{Canvas, Color, Element, HorizontalAlignment, Thickness};
use windows_reactor::{border, button, text_block, vstack};

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

#[test]
fn styling_capabilities_match_native_support() {
    let control = button("go")
        .padding(4.0)
        .background(Color::rgb(1, 2, 3))
        .foreground(Color::rgb(4, 5, 6))
        .font_family("Segoe UI")
        .font_size(16.0);
    assert_eq!(control.modifiers.padding, Some(Thickness::uniform(4.0)));
    assert_eq!(control.modifiers.font_size, Some(16.0));

    let panel = vstack(()).padding(8.0).background(Color::rgb(7, 8, 9));
    assert_eq!(panel.modifiers.padding, Some(Thickness::uniform(8.0)));
    assert!(panel.modifiers.background.is_some());

    let text = text_block("label")
        .padding(2.0)
        .foreground(Color::rgb(10, 11, 12))
        .font_family("Consolas");
    assert_eq!(text.modifiers.padding, Some(Thickness::uniform(2.0)));
    assert_eq!(text.modifiers.font_family.as_deref(), Some("Consolas"));

    let border = border(text_block("content"))
        .padding(6.0)
        .background(Color::rgb(13, 14, 15));
    assert_eq!(border.modifiers.padding, Some(Thickness::uniform(6.0)));
    assert!(border.modifiers.background.is_some());

    let canvas = Canvas::new(std::iter::empty::<Element>()).background(Color::rgb(16, 17, 18));
    assert!(canvas.modifiers.background.is_some());
}
