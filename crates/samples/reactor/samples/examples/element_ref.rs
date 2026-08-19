#![windows_subsystem = "windows"]

use windows_reactor::{Border, Button, Element, RenderCx, TextBlock, TextBox, Thickness, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let input = cx.use_element_ref::<TextBox>();
    let input_for_focus = input.clone();
    let text = cx.use_state(|| String::from("Focus target"));
    let current_text = text.value();
    let text_for_change = text;
    let status = cx.use_state(|| "Not focused");
    let current_status = status.value();
    let status_for_click = status.clone();

    Border::new(vstack(
        8.0,
        [
            TextBlock::new(
                "The typed reference exists across renders, points at the TextBox only while \
                 mounted, and cannot be attached to a different widget type.",
            )
            .build(),
            TextBox::new(current_text, move |value| {
                text_for_change.set(value);
            })
            .reference(&input)
            .build(),
            Button::new("Focus TextBox")
                .on_click(move || {
                    status_for_click.set(if input_for_focus.focus() {
                        "Focus requested"
                    } else {
                        "Focus target is not mounted"
                    });
                })
                .build(),
            TextBlock::new(current_status).build(),
        ],
    ))
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Typed Element Reference", app)
}
