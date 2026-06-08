use windows_reactor::core::element::{
    Button, Color, Element, Modifiers, Orientation, StackPanel, TextBlock, Thickness,
    can_skip_update,
};

#[test]
fn identical_text_trees_skip() {
    let a = Element::TextBlock(TextBlock::new("same"));
    let b = Element::TextBlock(TextBlock::new("same"));
    assert!(can_skip_update(&a, &b));
}

#[test]
fn content_difference_prevents_skip() {
    let a = Element::TextBlock(TextBlock::new("a"));
    let b = Element::TextBlock(TextBlock::new("b"));
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn font_size_difference_prevents_skip() {
    let a = Element::TextBlock(TextBlock {
        text: "x".into(),
        font_size: Some(14.0),
        ..TextBlock::default()
    });
    let b = Element::TextBlock(TextBlock {
        text: "x".into(),
        font_size: Some(20.0),
        ..TextBlock::default()
    });
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn modifier_difference_prevents_skip() {
    let a = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            margin: Some(Thickness::uniform(5.0)),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let b = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            margin: Some(Thickness::uniform(10.0)),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn variant_kind_mismatch_prevents_skip() {
    let a = Element::TextBlock(TextBlock::new("x"));
    let b = Element::Button(Button::new("x"));
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn stack_with_matching_children_skips() {
    let a = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            Element::TextBlock(TextBlock::new("a")),
            Element::TextBlock(TextBlock::new("b")),
        ],
        ..StackPanel::default()
    });
    let b = a.clone();
    assert!(can_skip_update(&a, &b));
}

#[test]
fn stack_with_different_child_does_not_skip() {
    let a = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![Element::TextBlock(TextBlock::new("a"))],
        ..StackPanel::default()
    });
    let b = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![Element::TextBlock(TextBlock::new("B"))],
        ..StackPanel::default()
    });
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn button_with_same_click_handler_skips() {
    let cb = windows_reactor::core::callback::Callback::<()>::new(|()| {});
    let a = Element::Button(Button {
        content: "go".into(),
        is_enabled: true,
        on_click: Some(cb.clone()),
        ..Button::default()
    });
    let b = Element::Button(Button {
        content: "go".into(),
        is_enabled: true,
        on_click: Some(cb),
        ..Button::default()
    });
    assert!(can_skip_update(&a, &b));
}

#[test]
fn button_with_independently_constructed_handlers_does_not_skip() {
    let a = Element::Button(Button {
        content: "go".into(),
        is_enabled: true,
        on_click: Some(windows_reactor::core::callback::Callback::<()>::new(
            |()| {},
        )),
        ..Button::default()
    });
    let b = Element::Button(Button {
        content: "go".into(),
        is_enabled: true,
        on_click: Some(windows_reactor::core::callback::Callback::<()>::new(
            |()| {},
        )),
        ..Button::default()
    });
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn empty_matches_empty() {
    assert!(can_skip_update(&Element::Empty, &Element::Empty));
}

#[test]
fn text_attached_difference_prevents_skip() {
    // Changing only the grid placement on a TextBlock element must still trigger
    // an update so the new placement is applied to the underlying control.
    let a = Element::TextBlock(TextBlock::new("x")).grid_row(0);
    let b = Element::TextBlock(TextBlock::new("x")).grid_row(2);
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn foreground_brush_change_prevents_skip() {
    let a = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            foreground: Some(Color::rgb(255, 0, 0).into()),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let b = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            foreground: Some(Color::rgb(0, 0, 255).into()),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    assert!(!can_skip_update(&a, &b));
}

#[test]
fn theme_bindings_present_prevents_skip_even_when_otherwise_equal() {
    use windows_reactor::core::backend::Prop;
    use windows_reactor::core::theme::ThemeRef;

    let mut bindings = rustc_hash::FxHashMap::default();
    bindings.insert(Prop::Background, ThemeRef::Accent);

    let a = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            theme_bindings: Some(Box::new(bindings.clone())),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let b = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            theme_bindings: Some(Box::new(bindings)),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });

    assert_eq!(a, b, "elements should compare equal under PartialEq");

    assert!(
        !can_skip_update(&a, &b),
        "audit §7.2.1: theme_bindings must force the update path"
    );
}

#[test]
fn empty_theme_bindings_map_still_allows_skip() {
    let a = Element::TextBlock(TextBlock {
        text: "x".into(),
        modifiers: Modifiers {
            theme_bindings: Some(Box::default()),
            ..Modifiers::default()
        },
        ..TextBlock::default()
    });
    let b = a.clone();
    assert!(can_skip_update(&a, &b));
}

