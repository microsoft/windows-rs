#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, RepeatButton, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_i32);
    let current = count.value();
    let increment = count.clone();

    vstack(
        8.0,
        [
            TextBlock::new(format!("Count: {current}")).build(),
            RepeatButton::new("+1 (hold to repeat)")
                .on_click(move || {
                    increment.update(|value| *value += 1);
                })
                .delay(300)
                .interval(50)
                .build(),
            RepeatButton::new("-1 (hold to repeat)")
                .on_click(move || {
                    count.update(|value| *value -= 1);
                })
                .delay(300)
                .interval(50)
                .build(),
            RepeatButton::new("Disabled").enabled(false).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RepeatButton", app)
}
