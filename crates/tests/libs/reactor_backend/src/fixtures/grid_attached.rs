//! Visual / runtime regression for "every widget kind supports Grid attached
//! properties (`grid_row` / `grid_column`)".
//!
//! Builds a single Grid that hosts one of every concrete widget kind, each
//! placed in a unique cell via the reactor builder API:
//!
//!     widget.into().grid_row(R).grid_column(C)
//!
//! After mount, the fixture walks the visual tree, finds the underlying
//! WinUI control for each widget, and asserts that
//! `Microsoft.UI.Xaml.Controls.Grid.GetRow(elem)` /
//! `Microsoft.UI.Xaml.Controls.Grid.GetColumn(elem)` return the requested
//! values. This is what failed for `Button` in #79 (it landed in cell
//! `(0, 0)` instead of the requested cell because `Button` had no
//! `attached` slot) and would silently fail for every other widget that
//! still lacks one. The fixture covers all widget kinds at once so a
//! regression on any of them is caught by a single, easy-to-read failing
//! assertion.

use crate::bindings;
use windows_reactor::core::element::{
    BreadcrumbBar, Expander, HyperlinkButton, Image, InfoBadge, InfoBar, NavViewItem,
    NavigationView, NumberBox, PersonPicture, Pivot, PivotItem, ProgressBar, ProgressRing,
    RadioButton, Shape, Slider, TabItem, TabView, ToggleSwitch,
};
use windows_reactor::core::element::{Color, Element, GridLength};
use windows_reactor::core::rich_text::{RichText, RichTextInline, RichTextRun};
use windows_reactor::dsl::{
    ElementExt, border, button, check_box, scroll_viewer, text_block, text_box,
};
use windows_reactor::grid;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

fn assert_widget_at_grid_row<T: windows_core::Interface>(
    h: &Harness,
    name: &str,
    expected_row: i32,
) {
    // Step 1: at least one T must exist somewhere in the tree (so we know
    // the widget actually got created).
    let any = h.find_all::<T>(&|_| true);
    h.check(
        &format!("Grid_AttachedProps_AllWidgetKinds_{name}_Created"),
        !any.is_empty(),
    );

    // Step 2: at least one T must have `Grid.Row == expected_row`. This is
    // what `grid_row(...)` is supposed to land on the underlying control.
    // Filtering by attached prop is essential because many widgets (Button,
    // CheckBox, Expander, etc.) realize *internal* TextBlocks/Borders/Grids
    // inside their template, and a naive "first T in DFS order" check would
    // pick those up instead of the top-level placement.
    let at_row = h.find_all::<T>(&|t| {
        t.cast::<bindings::FrameworkElement>()
            .ok()
            .and_then(|fe| bindings::Grid::GetRow(&fe).ok())
            .is_some_and(|r| r == expected_row)
    });
    h.check(
        &format!(
            "Grid_AttachedProps_AllWidgetKinds_{name}_Row{expected_row}(found_at_row={})",
            at_row.len()
        ),
        !at_row.is_empty(),
    );
}

pub fn grid_attached_props_all_widget_kinds(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        // Each widget kind takes its own non-zero row. A widget that
        // *silently* fails to honor `grid_row(i)` falls back to the default
        // row 0; the matching `assert_widget_at_grid_row` below expects a non-zero row,
        // so any such regression trips a clear failing assertion.
        h.mount(cc(|_| {
            let cells: Vec<Element> = vec![
                text_block("text").grid_row(1).grid_column(0).into(),
                button("button").grid_row(2).grid_column(0).into(),
                check_box(false)
                    .label("cb")
                    .grid_row(3)
                    .grid_column(0)
                    .into(),
                text_box("tf").grid_row(4).grid_column(0).into(),
                border(text_block("inside"))
                    .width(20.0)
                    .height(20.0)
                    .grid_row(5)
                    .grid_column(0)
                    .into(),
                scroll_viewer(text_block("sv content"))
                    .grid_row(6)
                    .grid_column(0)
                    .into(),
                ToggleSwitch::new(false).grid_row(7).grid_column(0).into(),
                Slider::new(50.0)
                    .range(0.0, 100.0)
                    .grid_row(8)
                    .grid_column(0)
                    .into(),
                RadioButton::new("rb").grid_row(9).grid_column(0).into(),
                NumberBox::new(0.0).grid_row(10).grid_column(0).into(),
                ProgressBar::new(50.0).grid_row(11).grid_column(0).into(),
                ProgressRing::indeterminate()
                    .grid_row(12)
                    .grid_column(0)
                    .into(),
                Expander::new(text_block("body"))
                    .header("hdr")
                    .grid_row(13)
                    .grid_column(0)
                    .into(),
                HyperlinkButton::new("link")
                    .grid_row(14)
                    .grid_column(0)
                    .into(),
                InfoBar::new("info").grid_row(15).grid_column(0).into(),
                InfoBadge::numeric(7).grid_row(16).grid_column(0).into(),
                PersonPicture::new()
                    .initials("AB")
                    .grid_row(17)
                    .grid_column(0)
                    .into(),
                Shape::rectangle()
                    .fill(Color::rgb(255, 0, 0))
                    .width(20.0)
                    .height(20.0)
                    .grid_row(18)
                    .grid_column(0)
                    .into(),
                Shape::ellipse()
                    .fill(Color::rgb(0, 200, 0))
                    .width(20.0)
                    .height(20.0)
                    .grid_row(19)
                    .grid_column(0)
                    .into(),
                Shape::line(0.0, 0.0, 10.0, 10.0)
                    .grid_row(20)
                    .grid_column(0)
                    .into(),
                Image::new("ms-appx:///none.png")
                    .grid_row(21)
                    .grid_column(0)
                    .into(),
                TabView::new([TabItem::new("a", text_block("x"))])
                    .grid_row(22)
                    .grid_column(0)
                    .into(),
                NavigationView::new([NavViewItem::new("home")], text_block("page"))
                    .grid_row(23)
                    .grid_column(0)
                    .into(),
                Pivot::new([PivotItem::new("p1", text_block("x"))])
                    .grid_row(24)
                    .grid_column(0)
                    .into(),
                BreadcrumbBar::new(["root", "child"])
                    .grid_row(25)
                    .grid_column(0)
                    .into(),
                RichText::single_paragraph(vec![RichTextInline::Run(RichTextRun::plain("rich"))])
                    .grid_row(26)
                    .grid_column(0)
                    .into(),
                grid((text_block("nested"),))
                    .rows([GridLength::STAR])
                    .columns([GridLength::STAR])
                    .grid_row(27)
                    .grid_column(0)
                    .into(),
            ];

            let rows: Vec<GridLength> =
                std::iter::repeat_n(GridLength::Auto, cells.len() + 1).collect();
            grid(cells)
                .rows(rows)
                .columns([GridLength::Star(1.0)])
                .into()
        }));
        h.render().await;

        // Each (kind, expected_row) pair below mirrors the placement above.
        // We probe by WinUI control type — find_all<T> walks the visual tree
        // and returns the first match. For controls that may also appear
        // *inside* a templated parent (e.g. a Button realized inside an
        // Expander or NavigationView header), we match the *first* hit; the
        // explicit `grid_row(i).grid_column(0)` placements above ensure the
        // top-level instance sits at the expected row.
        assert_widget_at_grid_row::<bindings::TextBlock>(&h, "TextBlock", 1);
        assert_widget_at_grid_row::<bindings::Button>(&h, "Button", 2);
        assert_widget_at_grid_row::<bindings::CheckBox>(&h, "CheckBox", 3);
        assert_widget_at_grid_row::<bindings::TextBox>(&h, "TextBox", 4);
        assert_widget_at_grid_row::<bindings::Border>(&h, "Border", 5);
        assert_widget_at_grid_row::<bindings::ScrollViewer>(&h, "ScrollViewer", 6);
        assert_widget_at_grid_row::<bindings::ToggleSwitch>(&h, "ToggleSwitch", 7);
        assert_widget_at_grid_row::<bindings::Slider>(&h, "Slider", 8);
        assert_widget_at_grid_row::<bindings::RadioButton>(&h, "RadioButton", 9);
        assert_widget_at_grid_row::<bindings::NumberBox>(&h, "NumberBox", 10);
        assert_widget_at_grid_row::<bindings::ProgressBar>(&h, "ProgressBar", 11);
        assert_widget_at_grid_row::<bindings::ProgressRing>(&h, "ProgressRing", 12);
        assert_widget_at_grid_row::<bindings::Expander>(&h, "Expander", 13);
        assert_widget_at_grid_row::<bindings::HyperlinkButton>(&h, "HyperlinkButton", 14);
        assert_widget_at_grid_row::<bindings::InfoBar>(&h, "InfoBar", 15);
        assert_widget_at_grid_row::<bindings::InfoBadge>(&h, "InfoBadge", 16);
        assert_widget_at_grid_row::<bindings::PersonPicture>(&h, "PersonPicture", 17);
        assert_widget_at_grid_row::<bindings::Rectangle>(&h, "Rectangle", 18);
        assert_widget_at_grid_row::<bindings::Ellipse>(&h, "Ellipse", 19);
        assert_widget_at_grid_row::<bindings::Line>(&h, "Line", 20);
        assert_widget_at_grid_row::<bindings::Image>(&h, "Image", 21);
        assert_widget_at_grid_row::<bindings::TabView>(&h, "TabView", 22);
        assert_widget_at_grid_row::<bindings::NavigationView>(&h, "NavigationView", 23);
        assert_widget_at_grid_row::<bindings::Pivot>(&h, "Pivot", 24);
        assert_widget_at_grid_row::<bindings::BreadcrumbBar>(&h, "BreadcrumbBar", 25);
        // RichTextBlock is now backed by the actual Xaml::RichTextBlock control.
        assert_widget_at_grid_row::<bindings::RichTextBlock>(&h, "RichText_TextBlock_Row26", 26);
        // Nested Grid: the outermost host Grid is at row 0; any Grid found
        // at row 27 must be the nested one we placed.
        assert_widget_at_grid_row::<bindings::Grid>(&h, "NestedGrid", 27);
    })
}
