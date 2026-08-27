use windows_core::Result;
use windows_reactor::*;

fn card(title: &str, body: &str, radius: f64, stroke: f64, column: i32) -> View {
    Border::new()
        .background(ThemeBrush::CardBackground)
        .border_brush(Color::rgb(160, 160, 160))
        .border_thickness(Thickness::uniform(stroke))
        .corner_radius(radius)
        .padding(Thickness::uniform(16.0))
        .min_width(160.0)
        .max_width(240.0)
        .grid_row(0)
        .grid_column(column)
        .content(
            StackPanel::new().spacing(6.0).children((
                TextBlock::new()
                    .text(title)
                    .font_size(16.0)
                    .font_weight(FontWeight::BOLD),
                TextBlock::new().text(body).font_size(13.0),
            )),
        )
}

fn view() -> View {
    Grid::new()
        .columns([GridLength::STAR, GridLength::STAR, GridLength::STAR])
        .column_spacing(12.0)
        .margin(Thickness::uniform(24.0))
        .children((
            card("Sharp", "corner_radius(0.0)", 0.0, 1.0, 0),
            card("Rounded", "corner_radius(8.0)", 8.0, 1.0, 1),
            card("Pill", "corner_radius(24.0) + 4px stroke", 24.0, 4.0, 2),
        ))
}

fn main() -> Result<()> {
    sample_reactor_controls::run("Card", view)
}
