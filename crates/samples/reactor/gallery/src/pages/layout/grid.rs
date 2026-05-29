use crate::controls::*;
use windows_reactor::*;

pub fn grid_page(_: &(), cx: &mut RenderCx) -> Element {
    let (wide, set_wide) = cx.use_state(false);

    let basic_grid = {
        let mut g = grid(())
            .rows([GridLength::Auto, GridLength::Auto])
            .columns([
                GridLength::Star(1.0),
                GridLength::Star(1.0),
                GridLength::Star(1.0),
            ])
            .row_spacing(4.0)
            .column_spacing(4.0)
            .width(400.0);
        g.children = vec![
            text_block("1,1")
                .padding(Thickness::uniform(12.0))
                .grid_row(0)
                .grid_column(0)
                .into(),
            text_block("1,2")
                .padding(Thickness::uniform(12.0))
                .grid_row(0)
                .grid_column(1)
                .into(),
            text_block("1,3")
                .padding(Thickness::uniform(12.0))
                .grid_row(0)
                .grid_column(2)
                .into(),
            text_block("2,1")
                .padding(Thickness::uniform(12.0))
                .grid_row(1)
                .grid_column(0)
                .into(),
            text_block("2,2")
                .padding(Thickness::uniform(12.0))
                .grid_row(1)
                .grid_column(1)
                .into(),
            text_block("2,3")
                .padding(Thickness::uniform(12.0))
                .grid_row(1)
                .grid_column(2)
                .into(),
        ];
        g
    };

    let spanning_grid = {
        let mut g = grid(())
            .rows([GridLength::Auto, GridLength::Auto, GridLength::Auto])
            .columns([
                GridLength::Star(1.0),
                GridLength::Star(1.0),
                GridLength::Star(1.0),
            ])
            .row_spacing(4.0)
            .column_spacing(4.0)
            .width(400.0);
        g.children = vec![
            text_block("Header (spans 3 columns)")
                .bold()
                .padding(Thickness::uniform(12.0))
                .grid_row(0)
                .grid_column(0)
                .grid_column_span(3)
                .into(),
            text_block("Left")
                .padding(Thickness::uniform(12.0))
                .grid_row(1)
                .grid_column(0)
                .into(),
            text_block("Center (spans 2)")
                .padding(Thickness::uniform(12.0))
                .grid_row(1)
                .grid_column(1)
                .grid_column_span(2)
                .into(),
            text_block("Footer (spans 3)")
                .padding(Thickness::uniform(12.0))
                .grid_row(2)
                .grid_column(0)
                .grid_column_span(3)
                .into(),
        ];
        g
    };

    let mixed_grid = {
        let mut g = grid(())
            .rows([GridLength::Auto])
            .columns([
                GridLength::Pixel(100.0),
                GridLength::Star(1.0),
                GridLength::Star(2.0),
            ])
            .column_spacing(4.0)
            .width(400.0);
        g.children = vec![
            text_block("100px")
                .padding(Thickness::uniform(8.0))
                .grid_row(0)
                .grid_column(0)
                .into(),
            text_block("1*")
                .padding(Thickness::uniform(8.0))
                .grid_row(0)
                .grid_column(1)
                .into(),
            text_block("2*")
                .padding(Thickness::uniform(8.0))
                .grid_row(0)
                .grid_column(2)
                .into(),
        ];
        g
    };

    let switchable_grid = if wide {
        let mut g = grid(())
            .rows([GridLength::Auto])
            .columns([
                GridLength::Star(1.0),
                GridLength::Star(1.0),
                GridLength::Star(1.0),
            ])
            .column_spacing(8.0);
        g.children = vec![
            text_block("Col 1").grid_column(0).into(),
            text_block("Col 2").grid_column(1).into(),
            text_block("Col 3").grid_column(2).into(),
        ];
        g
    } else {
        let mut g = grid(())
            .rows([GridLength::Auto, GridLength::Auto])
            .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
            .row_spacing(8.0)
            .column_spacing(8.0);
        g.children = vec![
            text_block("Row 0, Col 0").grid_row(0).grid_column(0).into(),
            text_block("Row 0, Col 1").grid_row(0).grid_column(1).into(),
            text_block("Row 1, Col 0").grid_row(1).grid_column(0).into(),
            text_block("Row 1, Col 1").grid_row(1).grid_column(1).into(),
        ];
        g
    };

    page_content(
        "Grid",
        "Arranges children in rows and columns with star/pixel/auto sizing.",
        vec![
            sample_card(
                "Basic Grid (2×3)",
                basic_grid,
                r#"grid(()).rows([Auto, Auto]).columns([Star(1.0), Star(1.0), Star(1.0)])
// children placed with .grid_row(r).grid_column(c)"#,
            ),
            sample_card(
                "Column & Row Spanning",
                spanning_grid,
                r#"text_block("Header").grid_column_span(3)
text_block("Center").grid_column(1).grid_column_span(2)"#,
            ),
            sample_card(
                "Mixed Sizing (Pixel + Star)",
                mixed_grid,
                r#"grid(()).columns([Pixel(100.0), Star(1.0), Star(2.0)])
// 100px fixed, 1* proportional, 2* double proportional"#,
            ),
            sample_card(
                "Switchable Layout",
                vstack((
                    ToggleSwitch::new(wide)
                        .header("Wide layout (3 columns)")
                        .on_changed(move |v: bool| set_wide.call(v)),
                    switchable_grid,
                ))
                .spacing(12.0),
                r#"// Grid layout changes dynamically with state
if wide { grid with 3 columns } else { grid with 2×2 }"#,
            ),
        ],
    )
    .into()
}
