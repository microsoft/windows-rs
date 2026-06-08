use windows_reactor::core::element::{
    Button, Canvas, ComboBox, Element, Modifiers, PasswordBox, PasswordRevealMode, PersonPicture,
    RadioButtons, StackPanel, TextBlock, Thickness,
};

#[test]
fn thickness_uniform_applies_to_all_sides() {
    let t = Thickness::uniform(4.0);
    assert_eq!(t.left, 4.0);
    assert_eq!(t.top, 4.0);
    assert_eq!(t.right, 4.0);
    assert_eq!(t.bottom, 4.0);
}

#[test]
fn modifiers_default_is_empty() {
    assert!(Modifiers::default().is_empty());
}

#[test]
fn element_key_returns_none_for_empty() {
    assert!(Element::Empty.key().is_none());
}

#[test]
fn element_kind_matches_is_reflexive_and_symmetric() {
    let a = Element::TextBlock(TextBlock::new("x"));
    let b = Element::TextBlock(TextBlock::new("y"));
    let c = Element::Button(Button::new("z"));
    assert!(a.kind_matches(&b));
    assert!(b.kind_matches(&a));
    assert!(!a.kind_matches(&c));
}

#[test]
fn from_impls_convert_payload_to_element() {
    let e: Element = TextBlock::new("hi").into();
    assert!(matches!(e, Element::TextBlock(_)));
    let e: Element = Button::new("go").into();
    assert!(matches!(e, Element::Button(_)));
}

#[test]
fn kind_name_returns_variant_name() {
    assert_eq!(Element::Empty.kind_name(), "Empty");
    assert_eq!(
        Element::TextBlock(TextBlock::new("x")).kind_name(),
        "TextBlock"
    );
    assert_eq!(Element::Button(Button::new("y")).kind_name(), "Button");
}

#[test]
fn kind_name_distinguishes_all_variants() {
    let names = [
        Element::TextBlock(TextBlock::new("a")).kind_name(),
        Element::Button(Button::new("b")).kind_name(),
        Element::StackPanel(StackPanel::default()).kind_name(),
        Element::Empty.kind_name(),
    ];
    let mut sorted = names.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len());
}

#[test]
fn person_picture_default_has_no_text() {
    let p = PersonPicture::new();
    assert!(p.display_name.is_none());
    assert!(p.initials.is_none());
}

#[test]
fn person_picture_builder_sets_fields() {
    let p = PersonPicture::new()
        .display_name("Ada Lovelace")
        .initials("AL");
    assert_eq!(p.display_name.as_deref(), Some("Ada Lovelace"));
    assert_eq!(p.initials.as_deref(), Some("AL"));
}

#[test]
fn password_box_default_uses_peek_reveal_mode() {
    let p = PasswordBox::new();
    assert_eq!(p.password_reveal_mode, PasswordRevealMode::Peek);
    assert!(p.is_enabled);
}

#[test]
fn radio_buttons_default_no_selection() {
    let r = RadioButtons::new(["A", "B", "C"]);
    assert_eq!(r.items.len(), 3);
    assert_eq!(r.selected_index, -1);
}

#[test]
fn combo_box_collects_items_in_order() {
    let c = ComboBox::new(["Red", "Green", "Blue"]).selected_index(1);
    assert_eq!(c.items, vec!["Red", "Green", "Blue"]);
    assert_eq!(c.selected_index, 1);
}

#[test]
fn canvas_collects_children() {
    let c = Canvas::new([Element::Empty, Element::Empty]);
    assert_eq!(c.children.len(), 2);
}
