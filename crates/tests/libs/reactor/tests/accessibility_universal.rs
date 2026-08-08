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

use test_reactor::{Op, RecordingBackend};
use windows_reactor::AccessibilityExt;
use windows_reactor::Reconciler;
use windows_reactor::RichTextBlock;
use windows_reactor::{
    AccessibilityModifiers, AutomationHeadingLevel, AutomationLiveSetting, Border, Button,
    CheckBox, Color, Element, Grid, GridLength, ScrollViewer, StackPanel, TextBlock, TextBox,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};

fn labelled<T: AccessibilityExt + Into<Element>>(widget: T) -> Element {
    widget
        .automation_name("the name")
        .automation_id("the-id")
        .help_text("help")
        .accessibility_live_setting(AutomationLiveSetting::Polite)
        .heading_level(AutomationHeadingLevel::Level2)
        .into()
}

fn one_of_every_widget() -> Vec<(&'static str, Element)> {
    vec![
        ("TextBlock", labelled(TextBlock::new("t"))),
        ("Button", labelled(Button::new("b"))),
        ("StackPanel", labelled(StackPanel::vertical())),
        ("Border", labelled(Border::new(Element::Empty))),
        ("CheckBox", labelled(CheckBox::new(false))),
        ("TextBox", labelled(TextBox::new("tf"))),
        (
            "Grid",
            labelled(Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }),
        ),
        ("ScrollViewer", labelled(ScrollViewer::new(Element::Empty))),
        ("ToggleSwitch", labelled(ToggleSwitch::new(false))),
        ("Slider", labelled(Slider::new(0.0))),
        ("RadioButton", labelled(RadioButton::new("r"))),
        ("NumberBox", labelled(NumberBox::new(0.0))),
        ("ProgressBar", labelled(ProgressBar::new(50.0))),
        ("ProgressRing", labelled(ProgressRing::indeterminate())),
        ("Expander", labelled(Expander::new(Element::Empty))),
        ("HyperlinkButton", labelled(HyperlinkButton::new("h"))),
        ("InfoBar", labelled(InfoBar::new("i"))),
        ("InfoBadge", labelled(InfoBadge::dot())),
        ("PersonPicture", labelled(PersonPicture::new())),
        (
            "Shape",
            labelled(Shape::rectangle().fill(Color::rgb(255, 0, 0))),
        ),
        ("Image", labelled(Image::new_with_uri("ms-appx:///x.png"))),
        (
            "TabView",
            labelled(TabView::new([TabItem::new("a", TextBlock::new("x"))])),
        ),
        (
            "NavigationView",
            labelled(NavigationView::new(
                [NavViewItem::new("home")],
                Element::Empty,
            )),
        ),
        ("TitleBar", labelled(TitleBar::new("title"))),
        (
            "Pivot",
            labelled(Pivot::new([PivotItem::new("a", TextBlock::new("x"))])),
        ),
        ("BreadcrumbBar", labelled(BreadcrumbBar::new(["root"]))),
        ("PasswordBox", labelled(PasswordBox::new())),
        ("RadioButtons", labelled(RadioButtons::new(["A", "B"]))),
        ("ComboBox", labelled(ComboBox::new(["A", "B"]))),
        ("Canvas", labelled(Canvas::new(()))),
        (
            "RichText",
            labelled(RichTextBlock::single_paragraph(Vec::new())),
        ),
    ]
}

fn populated() -> AccessibilityModifiers {
    AccessibilityModifiers {
        automation_name: Some("the name".into()),
        automation_id: Some("the-id".into()),
        help_text: Some("help".into()),
        live_setting: Some(AutomationLiveSetting::Polite),
        heading_level: Some(AutomationHeadingLevel::Level2),
    }
}

#[test]
fn every_widget_variant_round_trips_accessibility_modifiers() {
    for (name, element) in one_of_every_widget() {
        let acc = element.accessibility().unwrap_or_else(|| {
            panic!("{name}: accessibility builders did not record any modifiers")
        });
        assert_eq!(&populated(), acc, "{name}: round-trip mismatch");
    }
}

#[test]
fn every_widget_variant_emits_set_accessibility_on_mount() {
    for (name, element) in one_of_every_widget() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &element, None, Rc::new(|| {}))
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
