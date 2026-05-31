//! Card layout — composing `Border` and `Grid` into the standard WinUI
//! card pattern. Each card varies one `Border` knob (`corner_radius`,
//! `border_thickness`) so the visual difference makes the knob obvious.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let card = |title: &str, body: &str, radius: f64, stroke: f64| -> Element {
        border(
            vstack((
                text_block(title).font_size(16.0).bold(),
                text_block(body).font_size(13.0),
            ))
            .spacing(6.0),
        )
        .background(ThemeRef::CardBackground)
        .border_brush(Color::rgb(160, 160, 160))
        .border_thickness(Thickness::uniform(stroke))
        .corner_radius(radius)
        .padding(Thickness::uniform(16.0))
        .min_width(160.0)
        .max_width(240.0)
        .into()
    };

    grid((
        card("Sharp", "corner_radius(0.0)", 0.0, 1.0)
            .grid_row(0)
            .grid_column(0),
        card("Rounded", "corner_radius(8.0)", 8.0, 1.0)
            .grid_row(0)
            .grid_column(1),
        card("Pill", "corner_radius(24.0) + 4px stroke", 24.0, 4.0)
            .grid_row(0)
            .grid_column(2),
    ))
    .columns([
        GridLength::Star(1.0),
        GridLength::Star(1.0),
        GridLength::Star(1.0),
    ])
    .column_spacing(12.0)
    .padding(Thickness::uniform(24.0))
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
