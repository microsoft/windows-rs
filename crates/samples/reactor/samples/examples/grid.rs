#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, Element, FontWeight, Grid, GridChild, GridLength, RenderCx, TextBlock, Thickness,
};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    let cell = |label: &str, color: Color| {
        Border::new(
            TextBlock::new(label)
                .font_weight(FontWeight::BOLD)
                .foreground(Color::rgb(255, 255, 255))
                .build(),
        )
        .background(color)
        .padding(Thickness::uniform(12.0))
        .build()
    };

    Grid::new([
        GridChild::new(cell("0,0", Color::rgb(60, 100, 180)))
            .row(0)
            .column(0),
        GridChild::new(cell("0,1", Color::rgb(80, 140, 90)))
            .row(0)
            .column(1),
        GridChild::new(cell("1,0", Color::rgb(180, 90, 100)))
            .row(1)
            .column(0),
        GridChild::new(cell("1,1", Color::rgb(140, 90, 180)))
            .row(1)
            .column(1),
    ])
    .rows([GridLength::Auto, GridLength::Auto])
    .columns([GridLength::STAR, GridLength::STAR])
    .row_spacing(6.0)
    .column_spacing(6.0)
    .max_width(360.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Grid", app)
}
