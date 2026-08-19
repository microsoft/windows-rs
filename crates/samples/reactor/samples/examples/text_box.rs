#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TextBlock, TextBox, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let name = cx.use_state(String::new);
    let notes = cx.use_state(String::new);
    let current_name = name.value();
    let current_notes = notes.value();

    vstack(
        8.0,
        [
            TextBox::new(current_name.clone(), move |value| {
                name.set(value);
            })
            .header("Display name")
            .placeholder_text("Type your name...")
            .build(),
            TextBlock::new(format!(
                "Hello, {}!",
                if current_name.is_empty() {
                    "stranger"
                } else {
                    current_name.as_str()
                }
            ))
            .build(),
            TextBox::new(current_notes, move |value| {
                notes.set(value);
            })
            .header("Notes")
            .placeholder_text("Write something longer...")
            .multiline()
            .height(100.0)
            .build(),
            TextBox::display("read-only").header("Disabled").build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TextBox", app)
}
