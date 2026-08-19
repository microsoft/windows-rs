#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, TextBlock, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let clicks = cx.use_state(|| 0_u32);
    let current_clicks = clicks.value();
    let render_count = cx.use_ref(|| 0_u64);
    render_count.with_mut(|value| *value += 1);
    let current_renders = render_count.get().unwrap();

    vstack(
        8.0,
        [
            TextBlock::new(format!("clicks (use_state) = {current_clicks}"))
                .font_size(18.0)
                .build(),
            TextBlock::new(format!("renders (use_ref) = {current_renders}"))
                .font_size(18.0)
                .build(),
            Button::new("Click me")
                .on_click(move || {
                    clicks.update(|value| *value += 1);
                })
                .build(),
            TextBlock::new(
                "The ref changes during every render but never schedules a render itself.",
            )
            .font_size(12.0)
            .opacity(0.7)
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseRef", app)
}
