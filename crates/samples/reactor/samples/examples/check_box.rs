#![windows_subsystem = "windows"]

use windows_reactor::{CheckBox, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let checked = cx.use_state(|| false);
    let current = checked.value();

    vstack(
        8.0,
        [
            CheckBox::new("I accept the terms", current, move |value| {
                checked.set(value);
            })
            .build(),
            TextBlock::new(if current {
                "Accepted"
            } else {
                "Not yet accepted"
            })
            .build(),
            CheckBox::display("Disabled (always on)", true).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("CheckBox", app)
}
