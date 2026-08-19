#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, TextBlock, fragment, hstack, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let number = cx.use_state(|| 3_i32);
    let hint = cx.use_state(|| false);
    let current_number = number.value();
    let current_hint = hint.value();
    let recomputes = cx.use_ref(|| 0_u32);
    let memo_recomputes = recomputes.clone();

    let factorial = cx.use_memo(current_number, move || {
        memo_recomputes.with_mut(|value| *value += 1);
        (1..=i64::from(current_number)).product::<i64>()
    });
    let decrement = number.clone();
    let increment = number;

    vstack(
        8.0,
        [
            TextBlock::new(format!("n = {current_number}, factorial(n) = {factorial}"))
                .font_size(18.0)
                .build(),
            TextBlock::new(format!(
                "memo factory ran {} time(s)",
                recomputes.get().unwrap()
            ))
            .font_size(12.0)
            .opacity(0.7)
            .build(),
            hstack(
                8.0,
                [
                    Button::new("-")
                        .on_click(move || {
                            decrement.update(|value| *value = (*value - 1).max(0));
                        })
                        .build(),
                    Button::new("+")
                        .on_click(move || {
                            increment.update(|value| *value = (*value + 1).min(20));
                        })
                        .build(),
                    Button::new("Toggle unrelated state")
                        .on_click(move || {
                            hint.update(|value| *value = !*value);
                        })
                        .build(),
                ],
            ),
            if current_hint {
                TextBlock::new("The memo is unchanged because its dependency did not change.")
                    .opacity(0.7)
                    .build()
            } else {
                fragment([])
            },
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseMemo", app)
}
