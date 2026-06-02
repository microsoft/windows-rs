//! Minimal sample for the `SplitView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (open, set_open) = cx.use_state(true);

    let toggle = {
        let set_open = set_open.clone();
        move || set_open.call(!open)
    };

    let on_closed = move || set_open.call(false);

    split_view(
        // Content area
        vstack((
            text_block(format!("Pane is {}", if open { "open" } else { "closed" })),
            button("Toggle Pane").on_click(toggle),
        ))
        .spacing(12.0),
    )
    .pane(
        vstack((
            text_block("Pane Content"),
            text_block("Item A"),
            text_block("Item B"),
            text_block("Item C"),
        ))
        .spacing(8.0),
    )
    .display_mode(SplitViewDisplayMode::Inline)
    .is_pane_open(open)
    .open_pane_length(200.0)
    .on_pane_closed(on_closed)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("SplitView Sample").render(app)
}
