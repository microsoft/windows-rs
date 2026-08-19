#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, FontWeight, RenderCx, TextBlock, TextBox, component, vstack,
};

fn counter() -> Element {
    component(|cx| {
        let count = cx.use_state(|| 0_i32);
        let current = count.value();
        vstack(
            8.0,
            [
                TextBlock::new(format!("count = {current}"))
                    .font_size(24.0)
                    .font_weight(FontWeight::BOLD)
                    .build(),
                Button::new("Increment")
                    .on_click(move || {
                        count.update(|value| *value += 1);
                    })
                    .build(),
            ],
        )
    })
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let name = cx.use_state(|| "world".to_string());
    let current = name.value();

    vstack(
        12.0,
        [
            TextBlock::new(format!("Hello, {current}!"))
                .font_size(20.0)
                .font_weight(FontWeight::BOLD)
                .build(),
            TextBox::new(current, move |value| {
                name.set(value);
            })
            .header("Your name")
            .placeholder_text("Type a name...")
            .build(),
            counter(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("FunctionComponent", app)
}
