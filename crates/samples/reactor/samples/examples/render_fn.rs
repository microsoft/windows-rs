#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, FontWeight, RenderCx, TextBlock, hstack, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_i32);
    let current = count.value();
    let decrement = count.clone();

    vstack(
        12.0,
        [
            TextBlock::new(format!("Count: {current}"))
                .font_size(24.0)
                .font_weight(FontWeight::BOLD)
                .build(),
            hstack(
                8.0,
                [
                    Button::new("-")
                        .on_click(move || {
                            decrement.update(|value| *value -= 1);
                        })
                        .build(),
                    Button::new("+")
                        .on_click(move || {
                            count.update(|value| *value += 1);
                        })
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RenderFn", app)
}
