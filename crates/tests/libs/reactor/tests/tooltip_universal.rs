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

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::RichTextBlock;
use windows_reactor::TooltipExt;
use windows_reactor::{
    Border, Button, CheckBox, Color, Element, Grid, GridLength, ScrollViewer, StackPanel,
    TextBlock, TextBox, Tooltip, TooltipContent, TooltipPlacement,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};

fn tipped<T: TooltipExt + Into<Element>>(widget: T) -> Element {
    widget.tooltip("the tip").into()
}

fn one_of_every_widget() -> Vec<(&'static str, Element)> {
    vec![
        ("TextBlock", tipped(TextBlock::new("t"))),
        ("Button", tipped(Button::new("b"))),
        ("StackPanel", tipped(StackPanel::vertical())),
        ("Border", tipped(Border::new(Element::Empty))),
        ("CheckBox", tipped(CheckBox::new(false))),
        ("TextBox", tipped(TextBox::new("tf"))),
        (
            "Grid",
            tipped(Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }),
        ),
        ("ScrollViewer", tipped(ScrollViewer::new(Element::Empty))),
        ("ToggleSwitch", tipped(ToggleSwitch::new(false))),
        ("Slider", tipped(Slider::new(0.0))),
        ("RadioButton", tipped(RadioButton::new("r"))),
        ("NumberBox", tipped(NumberBox::new(0.0))),
        ("ProgressBar", tipped(ProgressBar::new(50.0))),
        ("ProgressRing", tipped(ProgressRing::indeterminate())),
        ("Expander", tipped(Expander::new(Element::Empty))),
        ("HyperlinkButton", tipped(HyperlinkButton::new("h"))),
        ("InfoBar", tipped(InfoBar::new("i"))),
        ("InfoBadge", tipped(InfoBadge::dot())),
        ("PersonPicture", tipped(PersonPicture::new())),
        (
            "Shape",
            tipped(Shape::rectangle().fill(Color::rgb(255, 0, 0))),
        ),
        ("Image", tipped(Image::new_with_uri("ms-appx:///x.png"))),
        (
            "TabView",
            tipped(TabView::new([TabItem::new("a", TextBlock::new("x"))])),
        ),
        (
            "NavigationView",
            tipped(NavigationView::new(
                [NavViewItem::new("home")],
                Element::Empty,
            )),
        ),
        ("TitleBar", tipped(TitleBar::new("title"))),
        (
            "Pivot",
            tipped(Pivot::new([PivotItem::new("a", TextBlock::new("x"))])),
        ),
        ("BreadcrumbBar", tipped(BreadcrumbBar::new(["root"]))),
        ("PasswordBox", tipped(PasswordBox::new())),
        ("RadioButtons", tipped(RadioButtons::new(["A", "B"]))),
        ("ComboBox", tipped(ComboBox::new(["A", "B"]))),
        ("Canvas", tipped(Canvas::new(()))),
        (
            "RichText",
            tipped(RichTextBlock::single_paragraph(Vec::new())),
        ),
    ]
}

#[test]
fn every_widget_variant_round_trips_tooltip_text() {
    for (name, element) in one_of_every_widget() {
        let tt = element
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
    let element: Element = Button::new("b")
        .tooltip_with(Tooltip::text("rich").placement(TooltipPlacement::Right))
        .into();
    let tt = element.modifiers().unwrap().tooltip.as_deref().unwrap();
    assert_eq!(tt.placement, Some(TooltipPlacement::Right));
    match &tt.content {
        TooltipContent::Text(s) => assert_eq!(s, "rich"),
        _ => panic!("expected Text payload"),
    }
}

#[test]
fn every_widget_variant_emits_set_tooltip_on_mount() {
    for (name, element) in one_of_every_widget() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &element, None, Rc::new(|| {}))
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
                    TooltipContent::Text(s) => assert_eq!(s, "the tip", "{name}: payload mismatch"),
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
    // Clearing must emit `None` because attached tooltips otherwise survive rerenders.
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
