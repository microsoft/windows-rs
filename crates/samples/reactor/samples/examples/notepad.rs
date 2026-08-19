#![windows_subsystem = "windows"]

use windows_reactor::{Element, HorizontalAlignment, RenderCx, TextBox, VerticalAlignment};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let text = cx.use_state(String::new);
    TextBox::new(text.value(), move |value| {
        text.set(value);
    })
    .multiline()
    .placeholder_text("Start typing...")
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Notepad", app)
}
