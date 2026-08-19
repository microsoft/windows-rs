#![windows_subsystem = "windows"]

use windows_reactor::{DatePicker, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let date = cx.use_state(|| None);
    let current = date.value();
    let label = current.map_or_else(
        || "No date picked".to_string(),
        |value| format!("Picked: {value}"),
    );

    vstack(
        8.0,
        [
            DatePicker::new(current, move |value| {
                date.set(value);
            })
            .header("Pick a date")
            .build(),
            TextBlock::new(label).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("DatePicker", app)
}
