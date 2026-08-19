#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TextBlock, ToggleSwitch, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let on = cx.use_state(|| true);
    let current = on.value();

    vstack(
        8.0,
        [
            ToggleSwitch::new(current, move |value| {
                on.set(value);
            })
            .header("Notifications")
            .on_content("On")
            .off_content("Off")
            .build(),
            TextBlock::new(if current {
                "Notifications enabled"
            } else {
                "Notifications muted"
            })
            .build(),
            ToggleSwitch::display(true)
                .header("Disabled (always on)")
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ToggleSwitch", app)
}
