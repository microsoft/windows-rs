#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, TabView, TabViewItem, TextBlock, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let alternate = cx.use_state(|| false);
    let last_close_key = cx.use_state(|| None::<u64>);
    let is_alternate = alternate.value();
    let current_close_key = last_close_key.value();
    let toggle = alternate;
    let update_close_key = last_close_key;
    let header = if is_alternate {
        "Renamed document"
    } else {
        "Document"
    };

    vstack(
        8.0,
        [
            Button::new("Rename tab")
                .on_click(move || {
                    toggle.set(!is_alternate);
                })
                .build(),
            TabView::display([TabViewItem::new(
                42,
                header,
                TextBlock::new("Close the tab to inspect its stable key.").build(),
            )])
            .on_close_requested(move |key| {
                update_close_key.set(Some(key));
            })
            .build(),
            TextBlock::new(format!(
                "configured key: 42; last close request: {}",
                current_close_key
                    .map_or_else(|| "<not requested>".to_string(), |key| key.to_string())
            ))
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TabView Item Key", app)
}
