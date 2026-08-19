#![windows_subsystem = "windows"]

use windows_reactor::{DropDownButton, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let opened = cx.use_state(|| 0_u32);
    let closed = cx.use_state(|| 0_u32);
    let current_opened = opened.value();
    let current_closed = closed.value();

    vstack(
        8.0,
        [
            DropDownButton::new(
                "Options",
                vstack(
                    4.0,
                    [
                        TextBlock::new("First option").build(),
                        TextBlock::new("Second option").build(),
                    ],
                ),
            )
            .on_opened(move || {
                opened.update(|value| *value += 1);
            })
            .on_closed(move || {
                closed.update(|value| *value += 1);
            })
            .build(),
            TextBlock::new(format!(
                "Opened: {current_opened}; closed: {current_closed}"
            ))
            .automation_id("drop-down-status")
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("DropDownButton", app)
}
