//! Sample for the `SelectorBar` widget.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("Recent"));

    vstack((
        selector_bar(vec![
            selector_bar_item("Recent"),
            selector_bar_item("Shared").icon(Symbol::People),
            selector_bar_item("Favorites").icon(Symbol::Favorite),
        ])
        .on_selection_changed(move |text| set_selected.call(text)),
        text_block(format!("Selected: {selected}")),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("SelectorBar", app)
}
