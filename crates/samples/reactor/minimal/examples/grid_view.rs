//! Sample for the `grid_view` templated list.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let items: Vec<String> = ["Red", "Green", "Blue", "Yellow", "Magenta", "Cyan"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    grid_view(items, |s, _idx| {
        border(text_block(s.clone()).font_size(12.0).bold())
            .background(Color::rgb(220, 230, 245))
            .padding(Thickness::uniform(10.0))
            .width(110.0)
            .height(70.0)
    })
    .with_key_selector(|s| s.clone())
    .height(220.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new()
        .title("Sample")
        .eager_templated_realization(true)
        .render(app)
}
