use windows_reactor::Color;
use windows_reactor::{BrushBinding, ThemeRef, tokens};

#[test]
fn resource_key_matches_winui_resource_name() {
    assert_eq!(
        ThemeRef::Accent.resource_key(),
        "AccentFillColorDefaultBrush"
    );
    assert_eq!(
        ThemeRef::CardBackground.resource_key(),
        "CardBackgroundFillColorDefaultBrush"
    );
    assert_eq!(
        ThemeRef::DividerStroke.resource_key(),
        "DividerStrokeColorDefaultBrush"
    );
}

#[test]
fn custom_token_passes_through_key() {
    let t = ThemeRef::custom("MyAppCustomBrush");
    assert_eq!(t.resource_key(), "MyAppCustomBrush");
}

#[test]
fn brush_binding_from_brush() {
    let bb: BrushBinding = Color::rgb(1, 2, 3).into();
    assert!(matches!(bb, BrushBinding::Direct(_)));
}

#[test]
fn brush_binding_from_theme_ref() {
    let bb: BrushBinding = ThemeRef::Accent.into();
    assert!(matches!(bb, BrushBinding::Theme(ThemeRef::Accent)));
}

#[test]
fn theme_refs_hash_eq_for_dedup() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(ThemeRef::Accent);
    set.insert(ThemeRef::Accent);
    set.insert(ThemeRef::AccentSecondary);
    assert_eq!(set.len(), 2);
}

#[test]
fn tokens_module_re_exports_variants() {
    use tokens::*;
    assert_eq!(Accent, ThemeRef::Accent);
    assert_eq!(SecondaryText, ThemeRef::SecondaryText);
}
