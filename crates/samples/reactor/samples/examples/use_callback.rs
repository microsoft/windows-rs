#![windows_subsystem = "windows"]

use windows_reactor::{Button, Callback, Element, RenderCx, TextBlock, hstack, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let rerenders = cx.use_state(|| 0_u32);
    let current_rerenders = rerenders.value();
    let fires = cx.use_ref(|| 0_u32);
    let fires_for_callback = fires.clone();

    let on_fire: Callback<()> = cx.use_callback((), move |()| {
        fires_for_callback.with_mut(|value| *value += 1);
    });
    let fire_a = on_fire.clone();
    let fire_b = on_fire;

    vstack(
        8.0,
        [
            TextBlock::new(format!("callback fired {} time(s)", fires.get().unwrap()))
                .font_size(18.0)
                .build(),
            TextBlock::new(format!("forced rerenders = {current_rerenders}"))
                .font_size(12.0)
                .opacity(0.7)
                .build(),
            hstack(
                8.0,
                [
                    Button::new("Fire (A)")
                        .on_click(move || fire_a.call(()))
                        .build(),
                    Button::new("Fire (B)")
                        .on_click(move || fire_b.call(()))
                        .build(),
                    Button::new("Force rerender")
                        .on_click(move || {
                            rerenders.update(|value| *value += 1);
                        })
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseCallback", app)
}
