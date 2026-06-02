//! Minimal sample for grid background color.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    grid([TextBlock::new("Sample")])
        .background(Color::rgb(255, 0, 0))
        .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Sample").render(app)
}
