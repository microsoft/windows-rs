#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, CornerRadius, Element, FontWeight, Grid, GridChild, GridLength, RenderCx,
    TextBlock, Thickness, vstack,
};

fn card(title: &str, body: &str, radius: f64, stroke: f64) -> Element {
    Border::new(vstack(
        6.0,
        [
            TextBlock::new(title)
                .font_size(16.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .build(),
            TextBlock::new(body).font_size(13.0).build(),
        ],
    ))
    .background(Color::rgb(245, 245, 245))
    .border_brush(Color::rgb(120, 120, 120))
    .border_thickness(Thickness::uniform(stroke))
    .corner_radius(CornerRadius::uniform(radius))
    .padding(Thickness::uniform(16.0))
    .min_width(160.0)
    .max_width(240.0)
    .build()
}

fn app(_cx: &mut RenderCx<'_>) -> Element {
    Grid::new([
        GridChild::new(card("Sharp", "corner_radius(0.0)", 0.0, 1.0)).column(0),
        GridChild::new(card("Rounded", "corner_radius(8.0)", 8.0, 1.0)).column(1),
        GridChild::new(card(
            "Pill",
            "corner_radius(24.0) with a 4-pixel stroke",
            24.0,
            4.0,
        ))
        .column(2),
    ])
    .columns([GridLength::STAR; 3])
    .column_spacing(12.0)
    .margin(Thickness::uniform(24.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Card", app)
}
