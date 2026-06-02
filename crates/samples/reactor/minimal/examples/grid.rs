//! Minimal sample for the `Grid` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let cell = |label: &str, color: Color| -> Element {
        border(
            text_block(label)
                .bold()
                .foreground(Color::rgb(255, 255, 255)),
        )
        .background(color)
        .padding(Thickness::uniform(12.0))
        .into()
    };

    grid((
        cell("0,0", Color::rgb(60, 100, 180))
            .grid_row(0)
            .grid_column(0),
        cell("0,1", Color::rgb(80, 140, 90))
            .grid_row(0)
            .grid_column(1),
        cell("1,0", Color::rgb(180, 90, 100))
            .grid_row(1)
            .grid_column(0),
        cell("1,1", Color::rgb(140, 90, 180))
            .grid_row(1)
            .grid_column(1),
    ))
    .rows([GridLength::Auto, GridLength::Auto])
    .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
    .row_spacing(6.0)
    .column_spacing(6.0)
    .max_width(360.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Sample").render(app)
}
