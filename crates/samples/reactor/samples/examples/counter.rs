#![windows_subsystem = "windows"]

use windows_reactor::{
    AutomationHeadingLevel, Button, Element, FontWeight, KeyboardAccelerator, RenderCx, TextBlock,
    VirtualKey, VirtualKeyModifiers, hstack, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_i32);
    let current = count.value();

    let decrement = count.clone();
    let increment = count.clone();
    let reset = count.clone();
    let accelerator_reset = count;
    let reset_accelerator =
        KeyboardAccelerator::new(VirtualKey::R, VirtualKeyModifiers::CONTROL, move || {
            accelerator_reset.set(0);
        });

    vstack(
        12.0,
        [
            TextBlock::new(format!("Count: {current}"))
                .font_weight(FontWeight::BOLD)
                .font_size(28.0)
                .automation_id("count-label")
                .heading_level(AutomationHeadingLevel::Level1)
                .build(),
            hstack(
                8.0,
                [
                    Button::new("-")
                        .on_click(move || {
                            decrement.update(|value| *value -= 1);
                        })
                        .automation_id("decrement-button")
                        .automation_name("Decrement")
                        .build(),
                    Button::new("+")
                        .on_click(move || {
                            increment.update(|value| *value += 1);
                        })
                        .automation_id("increment-button")
                        .automation_name("Increment")
                        .build(),
                    Button::new("reset (Ctrl+R)")
                        .on_click(move || {
                            reset.set(0);
                        })
                        .keyboard_accelerator(reset_accelerator)
                        .automation_id("reset-button")
                        .automation_name("Reset")
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Counter", app)
}
