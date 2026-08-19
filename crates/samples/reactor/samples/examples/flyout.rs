#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, FlyoutPlacement, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_u32);
    let current = count.value();

    vstack(
        8.0,
        [
            Button::new("Show Flyout")
                .flyout(TextBlock::new("Hello from the flyout!").build())
                .build(),
            Button::new("Bottom Flyout")
                .flyout(TextBlock::new(format!("Clicked {current} times")).build())
                .flyout_placement(FlyoutPlacement::Bottom)
                .build(),
            Button::new("Increment")
                .on_click(move || {
                    count.set(current + 1);
                })
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Flyout", app)
}
