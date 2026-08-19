#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, RenderCx, SplitView, SplitViewDisplayMode, TextBlock, vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let current = open.value();
    let toggle = open.clone();
    let close = open;

    let content = vstack(
        12.0,
        [
            TextBlock::new(format!(
                "Pane is {}",
                if current { "open" } else { "closed" }
            ))
            .automation_id("pane-status")
            .build(),
            Button::new("Toggle Pane")
                .on_click(move || {
                    toggle.update(|value| *value = !*value);
                })
                .automation_id("toggle-pane")
                .build(),
        ],
    );
    let pane = vstack(
        8.0,
        [
            TextBlock::new("Pane Content").build(),
            TextBlock::new("Item A").build(),
            TextBlock::new("Item B").build(),
            TextBlock::new("Item C").build(),
        ],
    );

    SplitView::new(content, pane, move || {
        close.set(false);
    })
    .display_mode(SplitViewDisplayMode::Inline)
    .is_pane_open(current)
    .open_pane_length(200.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("SplitView", app)
}
