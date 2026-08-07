//! Regression test that asserts every widget variant of `Element` round-trips
//! `grid_row` / `grid_column` (Grid attached properties) through
//! `Element::attached()` and the reconciler.
//!
//! Background: prior to this test, only `TextBlock`, `Button`, `CheckBox`,
//! `TextBox`, `Grid`, `ScrollViewer`, and `RichTextBlock` carried an
//! `attached: Option<AttachedProps>` field. The other 20 widget kinds
//! silently dropped `grid_row`/`grid_column`; `Element::attached_mut`
//! returned `None` and the chained setters were no-ops. PR #79 patched
//! `Button` because it was discovered visually in the tic-tac-toe sample;
//! this test ensures no other widget regresses to that state by
//! enumerating *every* `Element::*` widget variant.

use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::RichTextBlock;
use windows_reactor::{
    BackgroundExt, Border, Button, CanvasChildExt, CanvasPosition, CheckBox, Color, Element, Grid,
    GridChildExt, GridLength, PaddingExt, RelativePanelAlignment, RelativePanelChildExt,
    ScrollViewer, StackPanel, TextBlock, TextBox, TextStyleExt, VisualExt, list_view,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};
use windows_reactor::{Prop, PropValue};

/// One `Element` per widget variant. Use real, mountable instances so the
/// reconciler test below also exercises a successful mount.
fn placed<T: GridChildExt + Into<Element>>(widget: T) -> Element {
    widget
        .grid_row(2)
        .grid_column(3)
        .grid_row_span(4)
        .grid_column_span(5)
        .into()
}

fn one_of_every_widget() -> Vec<(&'static str, Element)> {
    vec![
        ("TextBlock", placed(TextBlock::new("t"))),
        ("Button", placed(Button::new("b"))),
        ("StackPanel", placed(StackPanel::vertical())),
        ("Border", placed(Border::new(Element::Empty))),
        ("CheckBox", placed(CheckBox::new(false))),
        ("TextBox", placed(TextBox::new("tf"))),
        (
            "Grid",
            placed(Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }),
        ),
        ("ScrollViewer", placed(ScrollViewer::new(Element::Empty))),
        ("ToggleSwitch", placed(ToggleSwitch::new(false))),
        ("Slider", placed(Slider::new(0.0))),
        ("RadioButton", placed(RadioButton::new("r"))),
        ("NumberBox", placed(NumberBox::new(0.0))),
        ("ProgressBar", placed(ProgressBar::new(50.0))),
        ("ProgressRing", placed(ProgressRing::indeterminate())),
        ("Expander", placed(Expander::new(Element::Empty))),
        ("HyperlinkButton", placed(HyperlinkButton::new("h"))),
        ("InfoBar", placed(InfoBar::new("i"))),
        ("InfoBadge", placed(InfoBadge::dot())),
        ("PersonPicture", placed(PersonPicture::new())),
        (
            "Shape",
            placed(Shape::rectangle().fill(Color::rgb(255, 0, 0))),
        ),
        ("Image", placed(Image::new_with_uri("ms-appx:///x.png"))),
        (
            "TabView",
            placed(TabView::new([TabItem::new("a", TextBlock::new("x"))])),
        ),
        (
            "NavigationView",
            placed(NavigationView::new(
                [NavViewItem::new("home")],
                Element::Empty,
            )),
        ),
        ("TitleBar", placed(TitleBar::new("title"))),
        (
            "Pivot",
            placed(Pivot::new([PivotItem::new("a", TextBlock::new("x"))])),
        ),
        ("BreadcrumbBar", placed(BreadcrumbBar::new(["root"]))),
        ("PasswordBox", placed(PasswordBox::new())),
        ("RadioButtons", placed(RadioButtons::new(["A", "B"]))),
        ("ComboBox", placed(ComboBox::new(["A", "B"]))),
        ("Canvas", placed(Canvas::new(()))),
        (
            "RichText",
            placed(RichTextBlock::single_paragraph(Vec::new())),
        ),
    ]
}

#[test]
fn every_widget_variant_round_trips_grid_placement() {
    for (name, element) in one_of_every_widget() {
        let p = element.modifiers().and_then(|m| m.grid).unwrap_or_else(|| {
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
    for (name, element) in one_of_every_widget() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &element, None, Rc::new(|| {}))
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

#[test]
fn templated_list_builder_retains_attached_layout_capabilities() {
    let element: Element = list_view(vec!["item"], |item, _| TextBlock::new(*item))
        .grid_row(2)
        .grid_column(3)
        .canvas_left(40.0)
        .relative_align_left()
        .opacity(0.5)
        .padding(6.0)
        .background(Color::rgb(1, 2, 3))
        .foreground(Color::rgb(4, 5, 6))
        .font_size(14.0)
        .into();
    let modifiers = element.modifiers().unwrap();
    let grid = modifiers.grid.unwrap();
    let attached = modifiers.attached.as_ref().unwrap();

    assert_eq!(grid.row, 2);
    assert_eq!(grid.column, 3);
    assert_eq!(modifiers.opacity, Some(0.5));
    assert_eq!(modifiers.padding, Some(6.0.into()));
    assert_eq!(modifiers.font_size, Some(14.0));
    assert!(modifiers.background.is_some());
    assert!(modifiers.foreground.is_some());
    assert_eq!(attached.get::<CanvasPosition>().unwrap().left, 40.0);
    assert!(
        attached
            .get::<RelativePanelAlignment>()
            .unwrap()
            .align_left_with_panel
    );
}
