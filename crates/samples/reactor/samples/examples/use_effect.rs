#![windows_subsystem = "windows"]

use windows_reactor::{Element, FontWeight, RenderCx, TextBlock, button, hstack, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_i32);
    let flag = cx.use_state(|| false);
    let current = count.value();
    let current_flag = flag.value();

    let last_seen = cx.use_ref(|| 0_i32);
    let last_seen_for_effect = last_seen.clone();
    cx.use_effect(current, move || {
        last_seen_for_effect.set(current);
    });

    let decrement = count.clone();
    let increment = count;
    vstack(
        8.0,
        [
            TextBlock::new(format!("count = {current}"))
                .font_size(24.0)
                .font_weight(FontWeight::BOLD)
                .build(),
            TextBlock::new(format!(
                "use_effect last observed: {}",
                last_seen.get().unwrap()
            ))
            .build(),
            hstack(
                8.0,
                [
                    button("-", move || {
                        decrement.update(|value| *value -= 1);
                    }),
                    button("+", move || {
                        increment.update(|value| *value += 1);
                    }),
                    button("toggle unrelated state", move || {
                        flag.update(|value| *value = !*value);
                    }),
                ],
            ),
            TextBlock::new(format!("unrelated flag = {current_flag}"))
                .font_size(12.0)
                .opacity(0.7)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseEffect", app)
}
