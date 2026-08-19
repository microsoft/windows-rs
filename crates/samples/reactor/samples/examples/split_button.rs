#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, SplitButton, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let clicks = cx.use_state(|| 0u32);
    let current = clicks.value();

    vstack(
        8.0,
        [
            SplitButton::new(format!("Primary action ({current})"))
                .on_click(move || {
                    clicks.set(current + 1);
                })
                .flyout(TextBlock::new(format!("Secondary action ({current})")).build())
                .build(),
            SplitButton::new("Disabled").enabled(false).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("SplitButton", app)
}
