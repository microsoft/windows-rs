use windows_core::Interface as _;

use windows_reactor::core::element::{Element, GridLength};
use windows_reactor::dsl::{grid, text_block};

use crate::bindings::Grid as XamlGrid;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

pub fn grid_row_column_layout(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| {
            let mk = |label: &'static str, r: i32, c: i32| -> Element {
                let e: Element = text_block(label).into();
                e.grid_row(r).grid_column(c)
            };
            let cells: Vec<Element> = vec![
                mk("TopLeft", 0, 0),
                mk("TopRight", 0, 1),
                mk("BottomLeft", 1, 0),
                mk("BottomRight", 1, 1),
            ];
            grid(cells)
                .rows([GridLength::Auto, GridLength::Star(1.0)])
                .columns([GridLength::Pixel(200.0), GridLength::Star(1.0)])
                .into()
        }));
        h.render().await;

        h.check(
            "Grid_RowColumnLayout_AllCellsPresent",
            h.find_text("TopLeft").is_some()
                && h.find_text("TopRight").is_some()
                && h.find_text("BottomLeft").is_some()
                && h.find_text("BottomRight").is_some(),
        );

        let g = h.find_all::<XamlGrid>(&|g| {
            g.cast::<crate::bindings::IGrid>()
                .unwrap()
                .get_ColumnDefinitions()
                .map_or(0, |c| {
                    c.cast::<windows_collections::IVector<crate::bindings::ColumnDefinition>>()
                        .unwrap()
                        .Size()
                        .unwrap_or(0)
                })
                == 2
                && g.cast::<crate::bindings::IGrid>()
                    .unwrap()
                    .get_RowDefinitions()
                    .map_or(0, |r| {
                        r.cast::<windows_collections::IVector<crate::bindings::RowDefinition>>()
                            .unwrap()
                            .Size()
                            .unwrap_or(0)
                    })
                    == 2
        });
        h.check("Grid_RowColumnLayout_GridCreated", !g.is_empty());
    })
}
