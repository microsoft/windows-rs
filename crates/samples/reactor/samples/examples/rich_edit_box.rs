#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, RichEditBox, StackPanel, TextBlock, Thickness};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let text = cx.use_state(String::new);
    let value = text.value();

    StackPanel::new([
        RichEditBox::new(value.clone(), {
            move |value| {
                text.set(value);
            }
        })
        .header("Rich Editor")
        .placeholder_text("Type rich text here...")
        .height(200.0)
        .automation_id("editor")
        .build(),
        TextBlock::new(format!("Plain text: {value}"))
            .automation_id("plain-text")
            .build(),
        RichEditBox::display("Read-only content.")
            .header("Read Only")
            .height(100.0)
            .automation_id("read-only-editor")
            .build(),
    ])
    .spacing(8.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RichEditBox", app)
}
