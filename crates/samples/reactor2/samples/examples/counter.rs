#![windows_subsystem = "windows"]

use windows_reactor2::{
    AutomationHeadingLevel, Button, Element, FontWeight, KeyboardAccelerator, RenderCx, TextBlock,
    TitleBar, VirtualKey, VirtualKeyModifiers, hstack, vstack,
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
                .heading_level(AutomationHeadingLevel::Level1)
                .automation_id("count-label")
                .build(),
            hstack(
                8.0,
                [
                    Button::new("-")
                        .on_click(move || {
                            decrement.update(|value| *value -= 1);
                        })
                        .automation_name("Decrement")
                        .automation_id("decrement-button")
                        .build(),
                    Button::new("+")
                        .on_click(move || {
                            increment.update(|value| *value += 1);
                        })
                        .automation_name("Increment")
                        .automation_id("increment-button")
                        .build(),
                    Button::new("reset (Ctrl+R)")
                        .on_click(move || {
                            reset.set(0);
                        })
                        .automation_id("reset-button")
                        .keyboard_accelerator(reset_accelerator)
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor2_samples::run_with_window(
        "Counter",
        |window| {
            window.title_bar(
                TitleBar::custom("windows_reactor \u{2014} counter")
                    .subtitle("Phase 1 demo".to_string()),
            )
        },
        app,
    )
}
