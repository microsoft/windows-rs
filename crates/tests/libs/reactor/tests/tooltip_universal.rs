//! Regression test that asserts every widget variant of `Element`
//! round-trips tooltip modifiers (`tooltip(text)`, `tooltip_with(...)`)
//! through `Element::modifiers().tooltip` and that the reconciler emits
//! `Op::SetTooltip` for them on mount, update, and clear.
//!
//! Background: roadmap item M1 (`Modifiers::tooltip`). Tooltips are
//! plumbed through `Modifiers::tooltip` rather than per-widget struct
//! fields, mirroring the `Modifiers::accessibility` strategy: at the
//! WinUI layer every backend `Handle` is a `FrameworkElement` (a
//! `DependencyObject`), so `ToolTipService::SetToolTip` applies
//! uniformly to every widget kind.

use std::rc::Rc;

use windows_reactor::core::backend::{Op, RecordingBackend};
use windows_reactor::core::element::{
    Border, Button, CheckBox, Color, Element, Grid, GridLength, ScrollViewer, StackPanel,
    TextBlock, TextBox, Tooltip, TooltipContent, TooltipPlacement,
};
use windows_reactor::core::element::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::rich_text::RichText;
use windows_reactor::dsl::ElementExt;

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
        ("Image", Image::new("ms-appx:///x.png").into()),
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

#[test]
fn every_widget_variant_round_trips_tooltip_text() {
    for (name, el) in one_of_every_widget() {
        let labelled = el.tooltip("the tip");
        let tt = labelled
            .modifiers()
            .and_then(|m| m.tooltip.as_deref())
            .unwrap_or_else(|| panic!("{name}: .tooltip(..) did not record a tooltip"));
        match &tt.content {
            TooltipContent::Text(s) => assert_eq!(s, "the tip", "{name}: payload mismatch"),
            _ => panic!("{name}: expected Text payload, got {:?}", tt.content),
        }
        assert_eq!(tt.placement, None, "{name}: no placement expected");
    }
}

#[test]
fn tooltip_with_placement_round_trips() {
    let el: Element = Button::new("b").into();
    let labelled = el.tooltip_with(Tooltip::text("rich").placement(TooltipPlacement::Right));
    let tt = labelled.modifiers().unwrap().tooltip.as_deref().unwrap();
    assert_eq!(tt.placement, Some(TooltipPlacement::Right));
    match &tt.content {
        TooltipContent::Text(s) => assert_eq!(s, "rich"),
        _ => panic!("expected Text payload"),
    }
}

#[test]
fn every_widget_variant_emits_set_tooltip_on_mount() {
    for (name, el) in one_of_every_widget() {
        let labelled = el.tooltip("hi");
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &labelled, None, Rc::new(|| {}))
            .unwrap_or_else(|| panic!("{name}: mount produced no control id"));

        let mut found = false;
        for op in &r.backend.ops {
            if let Op::SetTooltip { id: oid, tooltip } = op {
                if *oid != id {
                    continue;
                }
                let tt = tooltip
                    .as_ref()
                    .unwrap_or_else(|| panic!("{name}: mount SetTooltip was None"));
                match &tt.content {
                    TooltipContent::Text(s) => assert_eq!(s, "hi", "{name}: payload mismatch"),
                    _ => panic!("{name}: expected Text payload"),
                }
                found = true;
            }
        }
        assert!(found, "{name}: missing Op::SetTooltip on mount");
    }
}

#[test]
fn no_tooltip_does_not_emit_set_tooltip_on_mount() {
    let el: Element = Button::new("b").into();
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, Rc::new(|| {}));
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetTooltip { .. })),
        "no SetTooltip op expected for widget without tooltip modifier"
    );
}

#[test]
fn update_emits_set_tooltip_on_add_change_and_clear() {
    // Add → Change → Clear. Each transition must emit exactly one
    // `SetTooltip` op carrying the new tooltip (or `None` when cleared,
    // so the WinUI backend can reset the attached property — tooltips
    // are an attached property and otherwise survive re-renders).
    let plain: Element = Button::new("b").into();
    let labelled: Element = Button::new("b").tooltip("one").into();
    let relabelled: Element = Button::new("b").tooltip("two").into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &plain, None, Rc::new(|| {}))
        .expect("mount");
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetTooltip { .. })),
        "no op expected on initial mount without tooltip"
    );

    // Add a tooltip — update path should set it.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&plain), &labelled, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetTooltip { tooltip, .. } => Some(tooltip.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetTooltip on add");
    match &set_ops[0].as_ref().unwrap().content {
        TooltipContent::Text(s) => assert_eq!(s, "one"),
        _ => panic!("expected Text"),
    }

    // Change the tooltip — update path should re-set the new value.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&labelled), &relabelled, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetTooltip { tooltip, .. } => Some(tooltip.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetTooltip on change");
    match &set_ops[0].as_ref().unwrap().content {
        TooltipContent::Text(s) => assert_eq!(s, "two"),
        _ => panic!("expected Text"),
    }

    // Clear the tooltip — update path should re-set with `None` so the
    // backend can clear the attached property (it otherwise persists).
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&relabelled), &plain, Some(id), Rc::new(|| {}));
    let set_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetTooltip { tooltip, .. } => Some(tooltip.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(set_ops.len(), 1, "expected one SetTooltip on clear");
    assert!(
        set_ops[0].is_none(),
        "cleared tooltip should carry None payload"
    );
}
