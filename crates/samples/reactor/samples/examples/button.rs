#![windows_subsystem = "windows"]

use windows_reactor::{Button, ButtonEmphasis, Element, RenderCx, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let clicks = cx.use_state(|| 0_u32);
    let current = clicks.value();

    vstack(
        8.0,
        [
            Button::new(format!("Clicked {current} times"))
                .on_click(move || {
                    clicks.update(|value| *value += 1);
                })
                .build(),
            Button::new("Disabled").enabled(false).build(),
            Button::new("Accent (Primary Action)")
                .emphasis(ButtonEmphasis::Accent)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Button", app)
}
