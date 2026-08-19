#![windows_subsystem = "windows"]

use windows_reactor::{Element, FontWeight, RenderCx, TextBlock, button, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_i32);
    let current = count.value();

    vstack(
        0.0,
        [
            button("Click", move || {
                count.update(|value| *value += 1);
            }),
            TextBlock::new(format!("count = {current}"))
                .font_size(18.0)
                .font_weight(FontWeight::BOLD)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseState", app)
}
