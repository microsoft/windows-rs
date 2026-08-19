#![windows_subsystem = "windows"]

use windows_reactor::{Color, Element, RenderCx, TextBlock, TextBox, Thickness, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let text = cx.use_state(String::new);
    let current = text.value();
    let set_default = text.clone();
    let set_bordered = text.clone();

    vstack(
        8.0,
        [
            TextBlock::new("1. Default TextBox").build(),
            TextBox::new(current.clone(), move |value| {
                set_default.set(value);
            })
            .placeholder_text("Default style")
            .build(),
            TextBlock::new("2. Custom border (brush + thickness)").build(),
            TextBox::new(current.clone(), move |value| {
                set_bordered.set(value);
            })
            .placeholder_text("Thick blue border")
            .border_brush(Color::rgb(60, 120, 220))
            .border_thickness(Thickness::uniform(2.0))
            .build(),
            TextBlock::new("3. Borderless + transparent (chat/search bar)").build(),
            TextBox::new(current, move |value| {
                text.set(value);
            })
            .placeholder_text("Type a message...")
            .background(Color::argb(0, 0, 0, 0))
            .border_thickness(Thickness::uniform(0.0))
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TextBox border", app)
}
