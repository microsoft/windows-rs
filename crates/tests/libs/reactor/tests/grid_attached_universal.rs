//! Regression test that asserts every widget variant of `Element` round-trips
//! `grid_row` / `grid_column` (Grid attached properties) through
//! `Element::attached()` and the reconciler.
//!
//! Background: prior to this test, only `TextBlock`, `Button`, `CheckBox`,
//! `TextBox`, `Grid`, `ScrollViewer`, and `RichText` carried an
//! `attached: Option<AttachedProps>` field. The other 20 widget kinds
//! silently dropped `grid_row`/`grid_column` — `Element::attached_mut`
//! returned `None` and the chained setters were no-ops. PR #79 patched
//! `Button` because it was discovered visually in the tic-tac-toe sample;
//! this test ensures no other widget regresses to that state by
//! enumerating *every* `Element::*` widget variant.

use std::rc::Rc;

use windows_reactor::Reconciler;
use windows_reactor::RichText;
use windows_reactor::{
    Border, Button, CheckBox, Color, Element, Grid, GridLength, ScrollViewer, StackPanel,
    TextBlock, TextBox,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};
use windows_reactor::{Op, RecordingBackend};
use windows_reactor::{Prop, PropValue};

/// One `Element` per widget variant. Use real, mountable instances so the
/// reconciler test below also exercises a successful mount.
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

#[test]
fn every_widget_variant_round_trips_grid_placement() {
    for (name, el) in one_of_every_widget() {
        let placed = el
            .grid_row(2)
            .grid_column(3)
            .grid_row_span(4)
            .grid_column_span(5);
        let p = placed.modifiers().and_then(|m| m.grid).unwrap_or_else(|| {
            panic!("{name}: .grid_row(...)/grid_column(...) did not record grid placement")
        });
        assert_eq!(p.row, 2, "{name}: row");
        assert_eq!(p.column, 3, "{name}: column");
        assert_eq!(p.row_span, 4, "{name}: row_span");
        assert_eq!(p.column_span, 5, "{name}: column_span");
    }
}

#[test]
fn every_widget_variant_emits_grid_attached_set_props_on_mount() {
    // Mount each widget with a non-zero placement and assert that the
    // reconciler emits AttachedGridRow / AttachedGridColumn / *Span set_prop
    // ops for it. This is the path that, in a real WinUI host, lands as
    // `Xaml::Grid::SetRow/SetColumn` calls on the underlying control.
    for (name, el) in one_of_every_widget() {
        let placed = el
            .grid_row(2)
            .grid_column(3)
            .grid_row_span(4)
            .grid_column_span(5);
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &placed, None, Rc::new(|| {}))
            .unwrap_or_else(|| panic!("{name}: mount produced no control id"));

        let mut saw_row = false;
        let mut saw_column = false;
        let mut saw_row_span = false;
        let mut saw_column_span = false;
        for op in &r.backend.ops {
            if let Op::SetProp {
                id: oid,
                prop,
                value,
            } = op
            {
                if *oid != id {
                    continue;
                }
                match (prop, value) {
                    (Prop::AttachedGridRow, PropValue::I32(2)) => saw_row = true,
                    (Prop::AttachedGridColumn, PropValue::I32(3)) => saw_column = true,
                    (Prop::AttachedGridRowSpan, PropValue::I32(4)) => saw_row_span = true,
                    (Prop::AttachedGridColumnSpan, PropValue::I32(5)) => saw_column_span = true,
                    _ => {}
                }
            }
        }
        assert!(saw_row, "{name}: missing AttachedGridRow=2");
        assert!(saw_column, "{name}: missing AttachedGridColumn=3");
        assert!(saw_row_span, "{name}: missing AttachedGridRowSpan=4");
        assert!(saw_column_span, "{name}: missing AttachedGridColumnSpan=5");
    }
}
