//! Regression test: every widget variant round-trips accessibility modifiers
//! through `Element::accessibility()` and the reconciler emits
//! `Op::SetAccessibility` on mount.
//!
//! Accessibility is plumbed via `Modifiers::accessibility` (not per-widget
//! fields) since `AutomationProperties::Set*` applies uniformly to every
//! `FrameworkElement`. This test ensures:
//! 1. Builder methods record into `Modifiers::accessibility`.
//! 2. The reconciler emits `Op::SetAccessibility` on mount.
//! 3. Round-tripping is exact.

use std::rc::Rc;

use windows_reactor::ElementExt;
use windows_reactor::core::backend::{Op, RecordingBackend};
use windows_reactor::core::element::{
    AccessibilityModifiers, Border, Button, CheckBox, Color, Element, Grid, GridLength,
    HeadingLevel, LiveSetting, ScrollViewer, StackPanel, TextBlock, TextBox,
};
use windows_reactor::core::element::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::rich_text::RichText;

fn one_of_every_widget() -> Vec<(&'static str, Element)> {
    vec![
        ("TextBlock", TextBlock::new("t").into()),
        ("Button", Button::new("b").into()),
        ("StackPanel", StackPanel::vertical().into()),
        ("Border", Border::new(Element::Empty).into()),
        ("CheckBox", CheckBox::new(false).into()),
        ("TextBox", TextBox::new("tf").into()),
        (
            "Grid",
            Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }
            .into(),
        ),
        ("ScrollViewer", ScrollViewer::new(Element::Empty).into()),
        ("ToggleSwitch", ToggleSwitch::new(false).into()),
        ("Slider", Slider::new(0.0).into()),
        ("RadioButton", RadioButton::new("r").into()),
        ("NumberBox", NumberBox::new(0.0).into()),
        ("ProgressBar", ProgressBar::new(50.0).into()),
        ("ProgressRing", ProgressRing::indeterminate().into()),
        ("Expander", Expander::new(Element::Empty).into()),
        ("HyperlinkButton", HyperlinkButton::new("h").into()),
        ("InfoBar", InfoBar::new("i").into()),
        ("InfoBadge", InfoBadge::dot().into()),
        ("PersonPicture", PersonPicture::new().into()),
        (
            "Shape",
            Shape::rectangle().fill(Color::rgb(255, 0, 0)).into(),
        ),
        ("Image", Image::new_with_uri("ms-appx:///x.png").into()),
        (
            "TabView",
            TabView::new([TabItem::new("a", TextBlock::new("x"))]).into(),
        ),
        (
            "NavigationView",
            NavigationView::new([NavViewItem::new("home")], Element::Empty).into(),
        ),
        ("TitleBar", TitleBar::new("title").into()),
        (
            "Pivot",
            Pivot::new([PivotItem::new("a", TextBlock::new("x"))]).into(),
        ),
        ("BreadcrumbBar", BreadcrumbBar::new(["root"]).into()),
        ("PasswordBox", PasswordBox::new().into()),
        ("RadioButtons", RadioButtons::new(["A", "B"]).into()),
        ("ComboBox", ComboBox::new(["A", "B"]).into()),
        ("Canvas", Canvas::new(std::iter::empty::<Element>()).into()),
        ("RichText", RichText::single_paragraph(Vec::new()).into()),
    ]
}

fn populated() -> AccessibilityModifiers {
    AccessibilityModifiers {
        automation_name: Some("the name".into()),
        automation_id: Some("the-id".into()),
        help_text: Some("help".into()),
        live_setting: Some(LiveSetting::Polite),
        heading_level: Some(HeadingLevel::Level2),
    }
}

#[test]
fn every_widget_variant_round_trips_accessibility_modifiers() {
    for (name, el) in one_of_every_widget() {
        let labelled = el
            .automation_name("the name")
            .automation_id("the-id")
            .help_text("help")
            .accessibility_live_setting(LiveSetting::Polite)
            .heading_level(HeadingLevel::Level2);
        let acc = labelled.accessibility().unwrap_or_else(|| {
            panic!("{name}: accessibility builders did not record any modifiers")
        });
        assert_eq!(&populated(), acc, "{name}: round-trip mismatch");
    }
}

#[test]
fn every_widget_variant_emits_set_accessibility_on_mount() {
    for (name, el) in one_of_every_widget() {
        let labelled = el
            .automation_name("the name")
            .automation_id("the-id")
            .help_text("help")
            .accessibility_live_setting(LiveSetting::Polite)
            .heading_level(HeadingLevel::Level2);
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &labelled, None, Rc::new(|| {}))
            .unwrap_or_else(|| panic!("{name}: mount produced no control id"));

        let mut found = false;
        for op in &r.backend.ops {
            if let Op::SetAccessibility {
                id: oid,
                accessibility,
            } = op
            {
                if *oid != id {
                    continue;
                }
                assert_eq!(
                    &populated(),
                    accessibility,
                    "{name}: SetAccessibility payload mismatch"
                );
                found = true;
            }
        }
        assert!(found, "{name}: missing Op::SetAccessibility");
    }
}

#[test]
fn empty_accessibility_does_not_emit_set_accessibility_on_mount() {
    let el: Element = Button::new("b").into();
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, Rc::new(|| {}));
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetAccessibility { .. })),
        "no SetAccessibility op expected for widget without accessibility modifiers"
    );
}

#[test]
fn update_emits_set_accessibility_when_modifiers_change() {
    // Add → Change → Clear transitions.
    let plain: Element = Button::new("b").into();
    let labelled: Element = Button::new("b").automation_name("submit").into();
    let relabelled: Element = Button::new("b")
        .automation_name("submit")
        .help_text("ctrl+s")
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &plain, None, Rc::new(|| {}))
        .expect("mount");
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetAccessibility { .. })),
        "no op expected on initial mount without modifiers"
    );

    // Add a modifier.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&plain), &labelled, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetAccessibility { accessibility, .. } => Some(accessibility.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetAccessibility on add");
    assert_eq!(set_ops[0].automation_name.as_deref(), Some("submit"));

    // Change a modifier.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&labelled), &relabelled, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetAccessibility { accessibility, .. } => Some(accessibility.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetAccessibility on change");
    assert_eq!(set_ops[0].help_text.as_deref(), Some("ctrl+s"));
    assert_eq!(set_ops[0].automation_name.as_deref(), Some("submit"));

    // Clear all modifiers.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&relabelled), &plain, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetAccessibility { accessibility, .. } => Some(accessibility.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetAccessibility on clear");
    assert!(
        set_ops[0].is_empty(),
        "cleared modifiers should carry empty payload"
    );
}
