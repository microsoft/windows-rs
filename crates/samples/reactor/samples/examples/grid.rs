#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Grid", || {
        let cell = |label: &'static str, color: Color, row, column| {
            Border::new()
                .background(color)
                .padding(Thickness::uniform(12.0))
                .grid_row(row)
                .grid_column(column)
                .content(
                    TextBlock::new()
                        .text(label)
                        .font_weight(FontWeight::BOLD)
                        .foreground(Color::rgb(255, 255, 255)),
                )
        };

        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto])
            .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
            .row_spacing(6.0)
            .column_spacing(6.0)
            .max_width(360.0)
            .children((
                cell("0,0", Color::rgb(60, 100, 180), 0, 0),
                cell("0,1", Color::rgb(80, 140, 90), 0, 1),
                cell("1,0", Color::rgb(180, 90, 100), 1, 0),
                cell("1,1", Color::rgb(140, 90, 180), 1, 1),
            ))
    })
    .unwrap();
}
