//! Sample for the `grid_view` templated list with drag-and-drop reordering.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (items, set_items) = cx.use_state(
        ["Red", "Green", "Blue", "Yellow", "Magenta", "Cyan"]
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>(),
    );

    let reorder_source = items.clone();

    grid_view(items, |s, _idx| {
        border(text_block(s.clone()).font_size(12.0).bold())
            .background(Color::rgb(220, 230, 245))
            .padding(Thickness::uniform(10.0))
            .width(110.0)
            .height(70.0)
    })
    .with_key_selector(|s| s.clone())
    .can_drag_items(true)
    .can_reorder_items(true)
    .allow_drop(true)
    .on_reorder(move |order: Vec<usize>| {
        let next: Vec<String> = order.iter().map(|i| reorder_source[*i].clone()).collect();
        set_items.call(next);
    })
    .height(220.0)
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Sample").render(app)
}
